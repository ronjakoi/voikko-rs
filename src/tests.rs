
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_init() {
        let V = voikko::Voikko::new("fi-x-morphoid", Some("/etc/voikko")).unwrap();
        //libvoikko::init_test();
    }

    #[test]
    fn test_version() {
        let version = voikko::version();
        println!("version: {}", version);
    }
}
