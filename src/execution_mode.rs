#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Install,
    Uninstall,
}

impl ExecutionMode {
    pub fn from(str: &str) -> Option<ExecutionMode> {
        return match str {
            "install" => Some(ExecutionMode::Install),
            "uninstall" => Some(ExecutionMode::Uninstall),
            _ => None,
        };
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn convert_install_from_string() {
        // arrange
        // do
        let actual = ExecutionMode::from("install");

        // verify
        assert_eq!(actual, Some(ExecutionMode::Install))
    }

    #[test]
    fn convert_uninstall_from_string() {
        // arrange
        // do
        let actual = ExecutionMode::from("uninstall");

        // verify
        assert_eq!(actual, Some(ExecutionMode::Uninstall))
    }

    #[test]
    fn return_error_if_can_not_convert() {
        // arrange
        // do
        let actual = ExecutionMode::from("unknown");

        // verify
        assert_eq!(actual, None)
    }
}
