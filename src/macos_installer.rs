use std::fs::remove_file;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use quick_xml::events::BytesEnd;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use tempfile::Builder;

use crate::config::MacOSConfig;

use super::InstallerResult;
use super::RoamProtocolInstaller;

fn make_org_protocol_script(path: &Path) -> String {
    let script = format!(
        r#"
on open location this_URL
    set EC to "{} --no-wait "
    set filePath to quoted form of this_URL
    do shell script EC & filePath
    tell application "Emacs" to activate
end open location
"#,
        path.to_str().unwrap()
    );

    script
}

const PLIST_ELEMENTS: &str = r#"<key>CFBundleURLTypes</key>
<array>
  <dict>
    <key>CFBundleURLName</key>
    <string>org-protocol handler</string>
    <key>CFBundleURLSchemes</key>
    <array>
      <string>org-protocol</string>
    </array>
  </dict>
</array>
"#;

pub fn new(config: MacOSConfig) -> Box<dyn RoamProtocolInstaller> {
    Box::new(MacOSRoamProtocolInstaller::new(config))
}

struct MacOSRoamProtocolInstaller {
    config: MacOSConfig,
}

impl MacOSRoamProtocolInstaller {
    pub fn new(config: MacOSConfig) -> Self {
        MacOSRoamProtocolInstaller { config }
    }

    fn write_protocol_script(&self, writer: &mut dyn Write) -> InstallerResult<()> {
        let script = make_org_protocol_script(self.config.emacsclient_path.as_path());
        writer.write_all(script.as_bytes())?;

        Ok(())
    }

    fn compile_client_script(&self, path: &Path) -> InstallerResult<()> {
        let mut child = Command::new("osacompile")
            .args(&[
                "-o",
                "/Applications/OrgProtocolClient.app",
                path.to_str().unwrap(),
            ])
            .spawn()?;
        child.wait()?;

        Ok(())
    }

    fn rewrite_plist(&self, original: &mut dyn BufRead) -> InstallerResult<Vec<u8>> {
        let mut reader = Reader::from_reader(original);
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::End(ref e)) if e.name() == b"dict" => {
                    self.write_url_association_info(&mut writer)?;
                    writer.write_event(Event::End(BytesEnd::borrowed(b"dict")))
                }
                Ok(Event::Eof) => break,
                Ok(e) => writer.write_event(&e),
                Err(e) => Err(e),
            }?;
            buf.clear();
        }

        Ok(writer.into_inner().into_inner())
    }

    fn write_url_association_info<W>(&self, writer: &mut Writer<W>) -> InstallerResult<()>
    where
        W: Write,
    {
        let mut reader = Reader::from_str(PLIST_ELEMENTS);
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(e) => writer.write_event(&e),
                Err(e) => Err(e),
            }?;
            buf.clear()
        }
        Ok(())
    }
}

impl RoamProtocolInstaller for MacOSRoamProtocolInstaller {
    fn install(&mut self) -> InstallerResult<()> {
        println!("Building client application via Script Editor...");
        let mut script_temp_file = Builder::new()
            .prefix("org-protocol-script")
            .suffix(".scpt")
            .rand_bytes(8)
            .tempfile()?;
        self.write_protocol_script(script_temp_file.as_file_mut())?;

        self.compile_client_script(script_temp_file.path())?;

        println!("Editing plist to associate URL to application...");
        let path = Path::new("/Applications/OrgProtocolClient.app/Contents/Info.plist");
        let buf;
        {
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            buf = self.rewrite_plist(&mut reader)?;
        };
        let mut file = File::create(path)?;
        file.write_all(&buf)?;

        println!("Need associating URL to created application.");
        println!("Please run application located /Applications/OrgRoamProtocol.app by hand.");
        Ok(())
    }

    fn uninstall(&mut self) -> InstallerResult<()> {
        let path = Path::new("/Applications/OrgProtocolClient.app");
        if path.exists() {
            remove_file(&path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{
        io::{Cursor, Read, Seek, SeekFrom},
        path::PathBuf,
    };

    use super::*;

    #[test]
    fn write_protocol_script() {
        // arrange
        let installer = MacOSRoamProtocolInstaller::new(MacOSConfig {
            emacsclient_path: PathBuf::from("foo"),
        });

        // do
        let mut cursor = Cursor::new(Vec::new());
        let _ = installer.write_protocol_script(&mut cursor);

        // verify
        let mut buf = String::new();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, make_org_protocol_script(Path::new("foo")))
    }

    #[test]
    fn write_url_association_into_plist() {
        // arrange
        let installer = MacOSRoamProtocolInstaller::new(MacOSConfig {
            emacsclient_path: PathBuf::from("foo"),
        });
        let mut reader = BufReader::new(Cursor::new(Vec::from(
            r#"<?xml version="1.0" encoding="utf-8"?><plist version="1.0"><dict></dict></plist>"#
                .as_bytes(),
        )));

        // do
        let vec = installer.rewrite_plist(&mut reader);

        // verify
        let plist_element = String::from(PLIST_ELEMENTS)
            .lines()
            .collect::<Vec<_>>()
            .join("");
        let expect = format!(
            r#"<?xml version="1.0" encoding="utf-8"?><plist version="1.0"><dict>{}</dict></plist>"#,
            plist_element
        );
        assert_eq!(
            String::from_utf8(vec.unwrap()).unwrap().replace("\n", ""),
            expect
        );
    }

    #[test]
    fn contains_emacsclient_path() {
        // arrange
        let path = Path::new("foo/bar");

        // do
        let ret = make_org_protocol_script(path);

        // verify
        assert_eq!(ret.contains("foo/bar"), true)
    }
}
