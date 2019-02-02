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
        /// Initializes Voikko and returns a Voikko struct or an error string.
        ///
        /// # Arguments
        /// * `language` - BCP 47 language tag for the language to be used.
        ///                Private use subtags can be used to specify the dictionary variant.
        /// * `path` - Path to a directory from which dictionary files should be searched first before
        ///            looking into the standard dictionary locations. If `None`, no additional search path
        ///            will be used.
        pub fn new(language: &str, path: Option<&str>) -> Result<Voikko, String> {
            let v = libvoikko::init(language, path);

            match v {
                Ok(handle) => Ok(Voikko { handle: handle }),
                Err(error) => Err(error),
            }
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(self.handle);
        }
    }
}
