use std::error::Error;

use config::Config;

pub mod config;
pub mod execution_mode;
pub mod linux_installer;
pub mod macos_installer;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    return if let Some(config) = config.linux_config {
        match config.mode {
            execution_mode::ExecutionMode::Install => {
                linux_installer::new(config).install()?;
                println!("Installation process finished successfully.\n");
                println!("You should do something to use org-roam-protocol");
                println!("  1: Enable org-roam-protocol in your Emacs's init file.");
                println!("    (require 'org-roam-protocol)");
                println!("  2: Create the bookmarklet in your browser written at https://www.orgroam.com/manual.html#The-roam_002dref-protocol");
            }
            execution_mode::ExecutionMode::Uninstall => {
                linux_installer::new(config).uninstall()?;
                println!("Uninstall process finished successfully");
            }
        }

        Ok(())
    } else {
        panic!("not implemented");
    };
}

type InstallerResult<T> = Result<T, Box<dyn Error>>;

pub trait RoamProtocolInstaller {
    fn install(&mut self) -> InstallerResult<()>;
    fn uninstall(&mut self) -> InstallerResult<()>;
}
