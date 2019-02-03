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

    use crate::libvoikko;
    use unicode_segmentation::UnicodeSegmentation;

    /// Returns the version number of libvoikko.
    pub fn version<'a>() -> &'a str {
        libvoikko::version()
    }

    pub struct Voikko {
        handle: *mut libvoikko::VoikkoHandle,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    pub enum SpellReturn {
        SpellFailed,
        SpellOk,
        InternalError,
        CharsetConversionFailed,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    pub enum TokenType {
        None,
        Word,
        Punctuation,
        Whitespace,
        Unknown
    }

    #[derive(Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    pub struct Token {
        token_text: String,
        token_type: TokenType,
    }

    impl Token {
        pub fn new(token_text: &str, token_type: TokenType) -> Token {
            Token { token_text: String::from(token_text),
                    token_type: token_type }
        }
    }

    impl Voikko {
        /// Initializes Voikko and returns a Voikko struct.
        ///
        /// # Arguments
        ///
        /// * `language` - BCP 47 language tag for the language to be used.
        ///                Private use subtags can be used to specify the dictionary variant.
        /// * `path` - Path to a directory from which dictionary files should be searched first before
        ///            looking into the standard dictionary locations. If `None`, no additional search path
        ///            will be used.
        ///
        /// # Errors
        ///
        /// Returns an error string if init fails.
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

        /// Finds suggested correct spellings for given UTF-8 encoded word.
        /// Returns a vector of strings - an empty vector, if no suggestions.
        ///
        /// # Arguments
        ///
        /// * `word` - word to find suggestions for
        pub fn suggest(&self, word: &str) -> Vec<String> {
            libvoikko::suggest(self.handle, word)
        }

        /// Hyphenates the given word in UTF-8 encoding.
        /// Returns a string containing the hyphenation using the following notation:
        /// * `' '` = no hyphenation at this character,
        /// * `'-'` = hyphenation point (character at this position
        ///        is preserved in the hyphenated form),
        /// * `'='` = hyphenation point (character at this position
        ///        is replaced by the hyphen.)
        ///
        /// # Arguments
        ///
        /// * `word` - word to hyphenate
        ///
        /// # Errors
        ///
        /// Returns an error string on error.
        pub fn hyphenate(&self, word: &str) -> Result<String, bool> {
            libvoikko::hyphenate(self.handle, word)
        }

        /// Hyphenates the given word in UTF-8 encoding.
        /// Returns a string where hyphens are inserted in all hyphenation points.
        ///
        /// # Arguments
        ///
        /// * `word` - word to hyphenate
        /// * `hyphen` - string to insert at hyphenation points
        pub fn insert_hyphens(&self, word: &str, hyphen: &str) -> Result<String, bool> {
            let hyphens = self.hyphenate(word);
            match hyphens {
                Err(_) => Err(false),
                Ok(hyph) => {
                    Ok(word.graphemes(true)
                       .zip(hyph.graphemes(true))
                       .map(|(w, h)| match h {
                           " " => String::from(w),
                           "-" => format!("{}{}", hyphen, w),
                           "=" => String::from(hyphen),
                           _   => String::from(w),
                       })
                       .collect::<String>()
                      )
                }
            }
        }

        /// Tokenize a text string. Returns a vector of Token structs.
        ///
        /// # Arguments
        ///
        /// * `text` - Text to find tokens in.
        pub fn tokens(&self, text: &str) -> Vec<Token> {
            let mut tokenlist = Vec::new();
            let mut offset = 0;
            while offset < text.len() {
                let (raw_token, token_len) = libvoikko::next_token(self.handle, &text[offset..]);
                let token_type = match raw_token {
                    libvoikko::voikko_token_type::TOKEN_NONE => TokenType::None,
                    libvoikko::voikko_token_type::TOKEN_PUNCTUATION => TokenType::Punctuation,
                    libvoikko::voikko_token_type::TOKEN_WHITESPACE => TokenType::Whitespace,
                    libvoikko::voikko_token_type::TOKEN_WORD => TokenType::Word,
                    _ => TokenType::Unknown,
                };
                if token_type == TokenType::None {
                    break;
                }
                let token = Token::new(&text[offset..offset+token_len],
                                       token_type);
                tokenlist.push(token);
                offset += token_len;
            }
            tokenlist
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(self.handle);
        }
    }
}
