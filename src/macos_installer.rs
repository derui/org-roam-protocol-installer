use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::u8;

use tempfile::NamedTempFile;

use crate::config::MacOSConfig;

use super::InstallerResult;
use super::RoamProtocolInstaller;

const ORG_PROTOCOL_SCRIPT: &'static str = r#"
on open location this_URL
    set EC to "{} --no-wait "
    set filePath to quoted form of this_URL
    do shell script EC & filePath
    tell application "Emacs" to activate
end open location
"#;

pub fn new(config: MacOSConfig) -> Box<dyn RoamProtocolInstaller> {
    Box::new(MacOSRoamProtocolInstaller::new(config))
}

fn make_application_making_script(path: &Path) -> String {
    let path = path.to_str().unwrap();
    let script = format!(
        "tell application \"Script Editor\"
  set file to open {}
  tell file to save as \"application\" in \"/Application/OrgProtocolClient.app\"
  quit
end tell
",
        path
    );

    return String::from(script);
}

trait WriteTempfile {
    fn write_tempfile(&self, buf: &[u8]) -> InstallerResult<PathBuf>;
}

struct WriteTempfileImpl {}
impl WriteTempfile for WriteTempfileImpl {
    fn write_tempfile(&self, buf: &[u8]) -> InstallerResult<PathBuf> {
        let mut file = NamedTempFile::new()?;

        file.write_all(buf)?;
        file.flush()?;

        Ok(file.path().to_path_buf())
    }
}

trait ExecuteScript {
    fn execute(&mut self, path: &PathBuf) -> InstallerResult<()>;
}

struct ExecuteScriptImpl {}
impl ExecuteScript for ExecuteScriptImpl {
    fn execute(&mut self, path: &PathBuf) -> InstallerResult<()> {
        let mut child = Command::new("osascript")
            .arg(path.as_os_str().to_str().unwrap())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

struct MacOSRoamProtocolInstaller {
    config: MacOSConfig,
}

impl MacOSRoamProtocolInstaller {
    pub fn new(config: MacOSConfig) -> Self {
        MacOSRoamProtocolInstaller { config }
    }

    fn write_protocol_script(&self, writer: &mut dyn Write) -> InstallerResult<()> {
        writer.write_all(ORG_PROTOCOL_SCRIPT.as_bytes())?;

        Ok(())
    }

    fn write_application_script(&self, writer: &mut dyn Write, path: &Path) -> InstallerResult<()> {
        let script = make_application_making_script(path);
        writer.write_all(script.as_bytes())?;

        Ok(())
    }

    fn execute_osascript(&self, path: &Path) -> InstallerResult<()> {
        let mut child = Command::new("osascript")
            .arg(path.as_os_str().to_str().unwrap())
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}

impl RoamProtocolInstaller for MacOSRoamProtocolInstaller {
    fn install(&mut self) -> InstallerResult<()> {
        let mut script_temp_file = NamedTempFile::new()?;
        self.write_protocol_script(script_temp_file.as_file_mut())?;
        let mut application_temp_file = NamedTempFile::new()?;
        self.write_application_script(
            application_temp_file.as_file_mut(),
            script_temp_file.path(),
        )?;

        self.execute_osascript(application_temp_file.path())
    }

    fn uninstall(&mut self) -> InstallerResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::{Cursor, Read, Seek, SeekFrom};

    use crate::linux_installer::new;

    use super::*;

    struct WriteTempfileStub {
        pub path: PathBuf,
    }
    impl WriteTempfile for WriteTempfileStub {
        fn write_tempfile(&self, _: &[u8]) -> InstallerResult<PathBuf> {
            Ok(self.path.clone())
        }
    }

    struct ExecuteScriptStub<'a> {
        pub buf: &'a mut PathBuf,
    }
    impl ExecuteScriptStub<'_> {
        fn update_buf(&mut self, path: &PathBuf) {
            self.buf.clear();
            self.buf.push(path);
        }
    }
    impl ExecuteScript for ExecuteScriptStub<'_> {
        fn execute(&mut self, path: &PathBuf) -> InstallerResult<()> {
            self.update_buf(path);
            Ok(())
        }
    }

    #[test]
    fn write_protocol_script() {
        // arrange
        let installer = MacOSRoamProtocolInstaller::new(MacOSConfig {
            mode: crate::execution_mode::ExecutionMode::Install,
        });

        // do
        let mut cursor = Cursor::new(Vec::new());
        let _ = installer.write_protocol_script(&mut cursor);

        // verify
        let mut buf = String::new();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, ORG_PROTOCOL_SCRIPT)
    }

    #[test]
    fn write_application_script() {
        // arrange
        let installer = MacOSRoamProtocolInstaller::new(MacOSConfig {
            mode: crate::execution_mode::ExecutionMode::Install,
        });

        // do
        let mut cursor = Cursor::new(Vec::new());
        let path = Path::new("/test");
        let _ = installer.write_application_script(&mut cursor, &path);

        // verify
        let mut buf = String::new();
        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, make_application_making_script(&path))
    }
}
