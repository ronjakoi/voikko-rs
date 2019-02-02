/*  voikko-rs - libvoikko bindings for the Rust programming language
    Copyright (C) 2019 Ronja Koistinen

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

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

    #[derive(Debug, PartialEq, Eq)]
    pub enum SpellReturn {
        SpellFailed,
        SpellOk,
        InternalError,
        CharsetConversionFailed,
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

        /// Check the spelling of a UTF-8 character string.
        ///
        /// # Arguments
        ///
        /// * `word` - word to check
        pub fn spell(&self, word: &str) -> SpellReturn {
            let ret = libvoikko::spell(self.handle, word);
            match ret {
                0 => SpellReturn::SpellFailed,
                1 => SpellReturn::SpellOk,
                3 => SpellReturn::CharsetConversionFailed,
                _ => SpellReturn::InternalError,
            }
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(self.handle);
        }
    }
}
