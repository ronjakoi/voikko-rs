mod tests;
mod libvoikko;

mod voikko {

    use super::*;

    pub fn version() -> String {
        libvoikko::version()
    }

    pub struct Voikko {
        handle: libvoikko::VoikkoHandle,
    }

    impl Voikko {
        pub fn new(language: &str, path: Option<String>) -> Result<Self, String> {
            let v = libvoikko::init(language, path);

            match v {
                Ok(handle) => Ok(Voikko { handle: handle }),
                Err(error) => Err(error),
            }
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(&mut self.handle);
        }
    }
}
