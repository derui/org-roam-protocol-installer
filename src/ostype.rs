use std::fmt::Display;

#[derive(Debug, Eq)]
pub enum OsType {
    Linux,
    Mac,
}

impl Display for OsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OsType::Linux => "Linux",
            OsType::Mac => "Mac",
        };

        write!(f, "{}", str)
    }
}

impl PartialEq for OsType {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (OsType::Linux, OsType::Linux) => true,
            (OsType::Mac, OsType::Mac) => true,
            _ => false,
        };
    }
}

impl OsType {
    pub fn from_string(str: &String) -> Option<OsType> {
        match str.as_str() {
            "linux" => Some(OsType::Linux),
            "macos" => Some(OsType::Mac),
            _ => None,
        }
    }
}

mod test {

    #[cfg(test)]
    mod ostype {
        use crate::ostype::OsType;

        #[test]
        fn linux_type() {
            let str = String::from("linux");

            let result = OsType::from_string(&str);

            assert_eq!(Some(OsType::Linux), result)
        }

        #[test]
        fn mac_type() {
            let str = String::from("macos");

            let result = OsType::from_string(&str);

            assert_eq!(Some(OsType::Mac), result)
        }

        #[test]
        fn unknown_type() {
            let str = String::from("unknown");

            let result = OsType::from_string(&str);

            assert_eq!(None, result)
        }
    }
}
