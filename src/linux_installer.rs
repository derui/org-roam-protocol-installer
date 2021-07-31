use std::fs::remove_file;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

use crate::config::LinuxConfig;

use super::InstallerResult;
use super::RoamProtocolInstaller;

const DESKTOP_FILE_CONTENT: &'static str = r#"
[Desktop Entry]
Name=Org-Protocol
Exec=emacsclient %u
Icon=emacs-icon
Type=Application
Terminal=false
MimeType=x-scheme-handler/org-protocol
"#;

pub fn new(config: LinuxConfig) -> Box<dyn RoamProtocolInstaller> {
    Box::new(LinuxRoamProtocolInstaller::new(config))
}

struct LinuxRoamProtocolInstaller {
    config: LinuxConfig,
}
impl LinuxRoamProtocolInstaller {
    pub fn new(config: LinuxConfig) -> LinuxRoamProtocolInstaller {
        LinuxRoamProtocolInstaller { config }
    }

    fn get_desktop_file_path(&self) -> String {
        return self.config.get_desktop_file_path().unwrap_or(String::new());
    }

    fn open_desktop_file<'a>(&self) -> InstallerResult<File> {
        let desktop_file_path = self.get_desktop_file_path();

        let f = File::create(&desktop_file_path)?;
        Ok(f)
    }

    fn install_mime_for_xdg(&self) {
        process::Command::new("xdg-mime")
            .args(&[
                "default",
                "org-protocol.desktop",
                "x-scheme-handler/org-protocol",
            ])
            .output()
            .expect("Can not execute xdg-mime");
    }
}

impl RoamProtocolInstaller for LinuxRoamProtocolInstaller {
    fn install(&mut self) -> InstallerResult<()> {
        println!("Install desktop file...");
        let mut f: File = self.open_desktop_file()?;

        f.write(DESKTOP_FILE_CONTENT.as_bytes())?;

        println!("Install xdg-mime to this environment...");
        self.install_mime_for_xdg();
        Ok(())
    }

    fn uninstall(&mut self) -> InstallerResult<()> {
        println!("Remove desktop file...");
        let path = PathBuf::from(self.get_desktop_file_path());
        let path = path.as_path();

        if path.exists() {
            remove_file(path)?
        }
        Ok(())
    }
}
