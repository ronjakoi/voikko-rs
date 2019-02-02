mod tests;
mod libvoikko;

mod voikko {

    use super::*;

    pub fn version() -> String {
        libvoikko::version()
    }

    pub struct Voikko {
        handle: *mut libvoikko::VoikkoHandle,
    }

    impl Voikko {
        pub fn new(language: &str, path: Option<&str>) -> Result<Self, String> {
            let v = libvoikko::init(language, path);

            match v {
                Ok(handle) => Ok(Voikko { handle: handle }),
                Err(error) => Err(error),
            }
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            println!("going to drop Voikko");
            libvoikko::terminate(self.handle);
        }
    }
}
