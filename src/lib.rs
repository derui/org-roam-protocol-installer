pub mod ostype;

#[derive(Debug, Eq)]
pub struct Config {
    target_os: ostype::OsType,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let target_os = ostype::OsType::from_string(&args[1]);

        return match target_os {
            Some(v) => Ok(Config { target_os: v }),
            None => Err("Can not detect OS type"),
        };
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        if self.target_os == other.target_os {
            return true;
        } else {
            return false;
        }
    }
}

mod test {
    #[cfg(test)]
    mod config {
        use crate::ostype;
        use crate::Config;

        #[test]
        fn get_valid_config() {
            // arrange
            let args = vec![String::from(""), String::from("linux")];

            // do
            let actual = Config::new(&args);

            // verify
            assert_eq!(
                actual,
                Ok(Config {
                    target_os: ostype::OsType::Linux
                })
            )
        }

        #[test]
        fn get_error_if_invalid_os() {
            // arrange
            let args = vec![String::from(""), String::from("invalid")];

            // do
            let actual = Config::new(&args);

            // verify
            assert_eq!(actual, Err("Can not detect OS type"))
        }
    }
}
