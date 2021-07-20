use std::error::Error;

use config::Config;

pub mod config;
pub mod ostype;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let installer = match config.target_os {
        ostype::OsType::Linux => Box::new(linux::new_installer()),
        _ => panic!(format!("not implemented for {}", config.target_os)),
    };

    installer.install()
}

type InstallerResult<T> = Result<T, Box<dyn Error>>;

pub trait RoamProtocolInstaller {
    fn install(&self) -> InstallerResult<()>;
}

mod linux {
    use std::io::Write;
    use std::{fs::File, io::ErrorKind};

    use super::InstallerResult;
    use super::RoamProtocolInstaller;

    const USER_DESKTOP_FILE: &'static str = "~/.local/share/applications/org-protocol.desktop";
    const DESKTOP_FILE_CONTENT: &'static str = r#"
[Desktop Entry]
Name=Org-Protocol
Exec=emacsclient %u
Icon=emacs-icon
Type=Application
Terminal=false
MimeType=x-scheme-handler/org-protocol
"#;

    pub fn new_installer() -> Box<dyn RoamProtocolInstaller> {
        Box::new(LinuxRoamProtocolInstaller::new())
    }

    struct LinuxRoamProtocolInstaller {}
    impl LinuxRoamProtocolInstaller {
        pub fn new() -> Self {
            LinuxRoamProtocolInstaller {}
        }

        fn open_desktop_file<'a>(&self) -> InstallerResult<File> {
            let f = match File::open(USER_DESKTOP_FILE) {
                Ok(file) => file,
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    File::create(self::USER_DESKTOP_FILE)?
                }
                Err(e) => Err(e)?,
            };
            Ok(f)
        }
    }

    impl RoamProtocolInstaller for LinuxRoamProtocolInstaller {
        fn install(&self) -> InstallerResult<()> {
            let mut f: File = self.open_desktop_file()?;

            f.write(DESKTOP_FILE_CONTENT.as_bytes())?;
            Ok(())
        }
    }
}
