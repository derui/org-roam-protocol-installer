use std::error::Error;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(config) = config.linux_config {
        return linux_installer::new(config).install();
    } else {
        panic!("not implemented");
    };
}

type InstallerResult<T> = Result<T, Box<dyn Error>>;

pub trait RoamProtocolInstaller {
    fn install(&self) -> InstallerResult<()>;
}

mod linux_installer {
    use std::io::Write;
    use std::process;
    use std::{fs::File, io::ErrorKind};

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
        pub fn new(config: LinuxConfig) -> Self {
            LinuxRoamProtocolInstaller { config }
        }

        fn open_desktop_file<'a>(&self) -> InstallerResult<File> {
            let desktop_file_path = self.config.get_desktop_file_path().unwrap_or(String::new());

            let f = match File::open(&desktop_file_path) {
                Ok(file) => file,
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    File::create(&desktop_file_path)?
                }
                Err(e) => Err(e)?,
            };
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
        fn install(&self) -> InstallerResult<()> {
            let mut f: File = self.open_desktop_file()?;

            f.write(DESKTOP_FILE_CONTENT.as_bytes())?;
            self.install_mime_for_xdg();
            Ok(())
        }
    }
}
