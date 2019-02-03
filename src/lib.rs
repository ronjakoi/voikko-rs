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

pub mod voikko {

    use crate::libvoikko;
    use unicode_segmentation::UnicodeSegmentation;
    use std::collections::HashMap;

    /// Returns the version number of libvoikko.
    pub fn version<'a>() -> &'a str {
        libvoikko::version()
    }

    /// Struct containing information about an available dictionary.
    /// Fields are `language`, `script`, `variant` and `description`.
    #[derive(Debug, PartialEq, Eq)]
    pub struct Dictionary {
        pub language: String,
        pub script: String,
        pub variant: String,
        pub description: String,
    }

    impl Dictionary {
        /// Construct new Dictionary struct.
        ///
        /// # Arguments
        ///
        /// * `language`
        /// * `script`
        /// * `variant`
        /// * `description`
        pub fn new(language: &str, script: &str, variant: &str, description: &str) -> Dictionary {
            Dictionary {
                language: String::from(language),
                script: String::from(script),
                variant: String::from(variant),
                description: String::from(description),
            }
        }
    }

    pub type Analysis = HashMap<String, String>;

    /// Get a list of available dictionaries. Returns a vector of Dictionary structs.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_dicts(path: &str) -> Vec<Dictionary> {
        libvoikko::list_dicts(path)
    }

    /// Return a list of language codes representing the languages for which at least one
    /// dictionary is available for spell checking. The codes conform to those specified
    /// in BCP 47. Typically the returned codes consist of only BCP 47 language subtags.
    /// They may also include tags in format Language-Script, Language-Region, or
    /// Language-Script-Region if such variants are widely used for a particular language.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_supported_spelling_languages(path: &str) -> Vec<String> {
        libvoikko::list_supported_spelling_languages(path)
    }

    /// Same as list_supported_spelling_languages() but for hyphenation.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_supported_hyphenation_languages(path: &str) -> Vec<String> {
        libvoikko::list_supported_hyphenation_languages(path)
    }

    /// Same as list_supported_spelling_languages() but for grammar checking.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_supported_grammar_checking_languages(path: &str) -> Vec<String> {
        libvoikko::list_supported_grammar_checking_languages(path)
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

    #[derive(Debug, PartialEq, Eq)]
    pub enum TokenType {
        None,
        Word,
        Punctuation,
        Whitespace,
        Unknown
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Token {
        pub token_text: String,
        pub token_type: TokenType,
    }

    impl Token {
        pub fn new(token_text: &str, token_type: TokenType) -> Token {
            Token { token_text: String::from(token_text),
                    token_type: token_type }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum SentenceType {
        None,
        NoStart,
        Probable,
        Possible,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Sentence {
        text: String,
        next_start_type: SentenceType,
    }

    impl Sentence {
        pub fn new(sentence_text: &str, sentence_type: SentenceType) -> Sentence {
            Sentence { text: String::from(sentence_text),
                       next_start_type: sentence_type}
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

        /// Find sentences in a text string. Returns a vector of Sentence structs.
        ///
        /// # Arguments
        ///
        /// * `text` - Text to find sentences in.
        pub fn sentences(&self, text: &str) -> Vec<Sentence> {
            let mut sentlist = Vec::new();
            let mut offset = 0;
            let mut next_start_type = SentenceType::NoStart;
            while offset < text.chars().count() && next_start_type != SentenceType::None {
                // sent_len is in UTF-8 characters, not bytes
                let next_text = text
                                .chars()
                                .skip(offset)
                                .collect::<String>();
                let (raw_sent, sent_len) =
                    libvoikko::next_sentence(self.handle, next_text.as_str());
                next_start_type = match raw_sent {
                    libvoikko::voikko_sentence_type::SENTENCE_NO_START => SentenceType::NoStart,
                    libvoikko::voikko_sentence_type::SENTENCE_POSSIBLE => SentenceType::Possible,
                    libvoikko::voikko_sentence_type::SENTENCE_PROBABLE => SentenceType::Probable,
                    _ => SentenceType::None,
                };
                // construct new Sentence object with text slice and sentence type
                let token = Sentence::new(text
                                          .chars()
                                          .skip(offset)
                                          .take(sent_len)
                                          .collect::<String>()
                                          .as_str()
                                          ,
                                          next_start_type);
                sentlist.push(token);
                offset += sent_len;
            }
            sentlist
        }

        /// Analyzes the morphology of given word.
        ///
        /// Returns a vector of Analysis structs (std::collections::HashMap) or an empty vector if
        /// analysis fails.
        ///
        /// # Arguments
        ///
        /// * `word` - word to analyze
        // https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/doc/morphological-analysis.txt
        pub fn analyze(&self, word: &str) -> Vec<Analysis> {
            libvoikko::analyze_word(self.handle, word)
        }

        // Values of option constants documented in
        // https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/src/voikko_defines.h

        // Boolean options

        pub fn set_opt_ignore_dot(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 0, value)
        }

        pub fn set_opt_ignore_numbers(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 1, value)
        }

        pub fn set_opt_ignore_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 3, value)
        }

        pub fn set_opt_accept_first_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 6, value)
        }

        pub fn set_opt_accept_all_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 7, value)
        }

        pub fn set_opt_no_ugly_hyphenation(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 4, value)
        }

        pub fn set_opt_ocr_suggestions(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 8, value)
        }

        pub fn set_opt_ignore_nonwords(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 10, value)
        }

        pub fn set_opt_accept_extra_hyphens(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 11, value)
        }

        pub fn set_opt_accept_missing_hyphens(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 12, value)
        }

        pub fn set_opt_accept_titles_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 13, value)
        }

        pub fn set_opt_accept_unfinished_paragrahs_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 14, value)
        }

        pub fn set_opt_hyphenate_unknown_words(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 15, value)
        }

        pub fn set_opt_accept_bulleted_lists_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 16, value)
        }

        // Integer options

        pub fn set_min_hyphenated_word_length(&self, value: isize) -> bool {
            libvoikko::set_int_option(self.handle, 9, value)
        }

        pub fn set_speller_cache_size(&self, value: isize) -> bool {
            libvoikko::set_int_option(self.handle, 17, value)
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(self.handle);
        }
    }
}
