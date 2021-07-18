pub mod ostype;

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
