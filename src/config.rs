use std::fmt::{Debug, Display};
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use dirs::home_dir;

use crate::execution_mode::ExecutionMode;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub linux_config: Option<LinuxConfig>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config{{linux_config: {:?}}}", self.linux_config)
    }
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Result<Config, &'static str> {
        return if let Some(matches) = matches.subcommand_matches("linux") {
            let mut home = home_dir().unwrap();
            let desktop_entry_dir = match matches.value_of("desktop-entry-directory") {
                Some(str) if str.len() > 0 => PathBuf::from(str),
                Some(_) | None => {
                    home.push(".local/share/applications");
                    home
                }
            };
            let mode = ExecutionMode::from(matches.value_of("mode").unwrap())
                .unwrap_or(ExecutionMode::Install);

            Ok(Config {
                linux_config: Some(LinuxConfig {
                    mode,
                    desktop_entry_directory: desktop_entry_dir,
                    desktop_file_name: String::from(matches.value_of("desktop-file-name").unwrap()),
                }),
            })
        } else {
            Err("Can not detect OS type")
        };
    }
}

pub fn application_definition<'a, 'b>() -> App<'a, 'b> {
    return App::new("org-roam-protocol-installer").subcommand(
        SubCommand::with_name("linux")
            .about("Install for linux")
            .arg(
                Arg::with_name("desktop-entry-directory")
                    .short("d")
                    .default_value("")
                    .help("A full path of directory to save desktop entry"),
            )
            .arg(
                Arg::with_name("mode")
                    .default_value("install")
                    .possible_values(&["install", "uninstall"])
                    .help("A full path of directory to save desktop entry"),
            )
            .arg(
                Arg::with_name("desktop-file-name")
                    .short("f")
                    .default_value("org-protocol.desktop")
                    .help("Name of desktop file for org-protocol"),
            ),
    );
}

// configuration for linux
#[derive(Debug, PartialEq, Eq)]
pub struct LinuxConfig {
    pub desktop_entry_directory: PathBuf,
    pub desktop_file_name: String,
    pub mode: ExecutionMode,
}

impl LinuxConfig {
    pub fn get_desktop_file_path(&self) -> Option<String> {
        let mut buf = self.desktop_entry_directory.clone();
        buf.push(self.desktop_file_name.clone());

        if let Some(path) = buf.to_str() {
            return Some(String::from(path));
        } else {
            return None;
        }
    }
}

impl Display for LinuxConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LinuxConfig{{desktop_entry_directory: {}, desktop_file_name: {}}}",
            self.desktop_entry_directory.to_str().unwrap(),
            self.desktop_file_name
        )
    }
}

// configuration for macOS
#[derive(Debug, PartialEq, Eq)]
pub struct MacOSConfig {
    pub mode: ExecutionMode,
}

impl Display for MacOSConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MacOSConfig{{}}",)
    }
}

mod test {
    #[cfg(test)]
    mod config {
        use std::path::PathBuf;

        use crate::config::{application_definition, Config, LinuxConfig};

        #[test]
        fn get_valid_config() {
            // arrange
            let args = vec![String::from(""), String::from("linux")];
            let matches = application_definition().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            let mut home = dirs::home_dir().unwrap();
            home.push(".local/share/applications");

            assert_eq!(
                actual,
                Ok(Config {
                    linux_config: Some(LinuxConfig {
                        mode: crate::execution_mode::ExecutionMode::Install,
                        desktop_entry_directory: home,
                        desktop_file_name: String::from("org-protocol.desktop")
                    })
                })
            )
        }

        mod linux_config {
            use std::path::PathBuf;

            use crate::config::LinuxConfig;

            #[test]
            fn get_desktop_file() {
                // arrange
                let config = LinuxConfig {
                    mode: crate::execution_mode::ExecutionMode::Install,
                    desktop_entry_directory: PathBuf::from("directory"),
                    desktop_file_name: String::from("file.desktop"),
                };

                // do
                let actual = config.get_desktop_file_path();

                // verify
                let mut buf = PathBuf::from("directory");
                buf.push("file.desktop");
                assert_eq!(actual, buf.to_str().map(|v| { String::from(v) }))
            }
        }

        #[test]
        fn change_linux_path() {
            // arrange
            let args = vec![
                String::from(""),
                String::from("linux"),
                String::from("-d"),
                String::from("directory"),
                String::from("-f"),
                String::from("file.desktop"),
            ];
            let matches = application_definition().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            assert_eq!(
                actual,
                Ok(Config {
                    linux_config: Some(LinuxConfig {
                        mode: crate::execution_mode::ExecutionMode::Install,
                        desktop_entry_directory: PathBuf::from("directory"),
                        desktop_file_name: String::from("file.desktop")
                    })
                })
            )
        }

        #[test]
        fn get_error_if_invalid_os() {
            // arrange
            let args = vec![String::from("")];
            let matches = application_definition().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            assert_eq!(actual, Err("Can not detect OS type"))
        }
    }
}
