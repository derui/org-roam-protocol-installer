use std::fmt::{Debug, Display};
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use dirs::home_dir;

#[derive(Debug, Eq)]
pub struct Config {
    pub linux_config: Option<LinuxConfig>,
}

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
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
                Arg::with_name("desktop-file-name")
                    .short("f")
                    .default_value("org-protocol.desktop")
                    .help("Name of desktop file for org-protocol"),
            ),
    );
}

#[derive(Debug, Eq)]
pub struct LinuxConfig {
    pub desktop_entry_directory: PathBuf,
    pub desktop_file_name: String,
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

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config{{linux_config: {:?}}}", self.linux_config)
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

            Ok(Config {
                linux_config: Some(LinuxConfig {
                    desktop_entry_directory: desktop_entry_dir,
                    desktop_file_name: String::from(matches.value_of("desktop-file-name").unwrap()),
                }),
            })
        } else {
            Err("Can not detect OS type")
        };
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        if self.linux_config == other.linux_config {
            return true;
        } else {
            return false;
        }
    }
}

impl PartialEq for LinuxConfig {
    fn eq(&self, other: &Self) -> bool {
        let same_desktop_entry_directory =
            self.desktop_entry_directory == other.desktop_entry_directory;
        let same_desktop_file_name = self.desktop_file_name == other.desktop_file_name;
        return same_desktop_entry_directory && same_desktop_file_name;
    }
}

mod test {
    #[cfg(test)]
    mod config {
        use std::path::PathBuf;

        use crate::config::{get_app, Config, LinuxConfig};

        #[test]
        fn get_valid_config() {
            // arrange
            let args = vec![String::from(""), String::from("linux")];
            let matches = get_app().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            let mut home = dirs::home_dir().unwrap();
            home.push(".local/share/applications");

            assert_eq!(
                actual,
                Ok(Config {
                    linux_config: Some(LinuxConfig {
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
            let matches = get_app().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            assert_eq!(
                actual,
                Ok(Config {
                    linux_config: Some(LinuxConfig {
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
            let matches = get_app().get_matches_from(args);

            // do
            let actual = Config::new(&matches);

            // verify
            assert_eq!(actual, Err("Can not detect OS type"))
        }
    }
}
