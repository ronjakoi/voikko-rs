/*  voikko-rs - libvoikko bindings for the Rust programming language
    Copyright (C) 2019-2022 Ronja Koistinen

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
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::similar_names)]

//! This module provides Rust bindings for libvoikko.
//!
//! Libvoikko provides spell checking, hyphenation, grammar checking and
//! morphological analysis for the Finnish language.
//!
//! voikko-rs requires libvoikko (version 4.1.1 or greater)
//! to be installed on your system.
//!
mod libvoikko;
mod tests;

/// This module contains the functions, types and structs of the crate.
pub mod voikko {

    use crate::libvoikko;
    use std::collections::HashMap;
    use std::error;
    use unicode_segmentation::UnicodeSegmentation;

    /// Returns the version number of libvoikko.
    pub fn version<'a>() -> &'a str {
        libvoikko::version()
    }

    /// Information about an available dictionary
    ///
    /// Contains the language, script, variant and human readable description
    /// of the dictionary.
    #[allow(missing_docs)]
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
        #[must_use]
        pub fn new(language: &str, script: &str, variant: &str, description: &str) -> Dictionary {
            Dictionary {
                language: String::from(language),
                script: String::from(script),
                variant: String::from(variant),
                description: String::from(description),
            }
        }
    }

    /// A morphological analysis item
    pub type Analysis = HashMap<String, String>;

    /// Get a list of available dictionaries. Returns a vector of Dictionary structs.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_dicts(path: &str) -> Vec<Dictionary> {
        libvoikko::list_dicts(path).unwrap_or_else(|_| vec![])
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
        libvoikko::list_supported_spelling_languages(path).unwrap_or_else(|_| vec![])
    }

    /// Same as `list_supported_spelling_languages()` but for hyphenation.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_supported_hyphenation_languages(path: &str) -> Vec<String> {
        libvoikko::list_supported_hyphenation_languages(path).unwrap_or_else(|_| vec![])
    }

    /// Same as `list_supported_spelling_languages()` but for grammar checking.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a directory from which dictionary files should be searched
    ///            first before looking into the standard dictionary locations.
    ///            Pass an empty string in order to only look in standard locations.
    pub fn list_supported_grammar_checking_languages(path: &str) -> Vec<String> {
        libvoikko::list_supported_grammar_checking_languages(path).unwrap_or_else(|_| vec![])
    }

    /// A Voikko instance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate voikko_rs; // in Rust 2015
    /// use voikko_rs::voikko;
    ///
    /// fn main() {
    ///     let v = voikko::Voikko::new("fi-x-morphoid", None).unwrap();
    ///     assert_eq!(v.hyphenate("kunnallispolitiikka", "-"),
    ///                Ok(String::from("kun-nal-lis-po-li-tiik-ka")));
    /// }
    /// ```
    pub struct Voikko {
        handle: *mut libvoikko::VoikkoHandle,
    }

    /// A spell check return value
    #[derive(Debug, PartialEq, Eq)]
    pub enum SpellReturn {
        /// Incorrect spelling
        SpellFailed,
        /// Correct spelling
        SpellOk,
        /// Internal error from libvoikko
        InternalError,
        /// libvoikko failed to convert character sets
        CharsetConversionFailed,
    }

    /// Type of token returned by [`analyze()`]
    #[derive(Debug, PartialEq, Eq)]
    #[allow(missing_docs)]
    pub enum TokenType {
        None,
        Word,
        Punctuation,
        Whitespace,
        Unknown,
    }

    /// Tokenization unit
    #[derive(Debug, PartialEq, Eq)]
    pub struct Token {
        /// Text of the token
        pub token_text: String,
        /// Type of the token
        pub token_type: TokenType,
    }

    #[allow(missing_docs)]
    impl Token {
        pub fn new(token_text: &str, token_type: TokenType) -> Token {
            Token {
                token_text: String::from(token_text),
                token_type,
            }
        }
    }

    /// Type of a following sentence
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum SentenceType {
        /// End of text reached or error.
        None,
        /// This is not a start of a new sentence.
        NoStart,
        /// This may be a start of a new sentence.
        Probable,
        /// This is a probable start of a new sentence.
        Possible,
    }

    /// A sentence
    #[derive(Debug, PartialEq, Eq)]
    pub struct Sentence {
        /// Text of the sentence
        text: String,
        /// The type of the next sentence
        next_start_type: SentenceType,
    }

    #[allow(missing_docs)]
    impl Sentence {
        pub fn new(sentence_text: &str, sentence_type: SentenceType) -> Sentence {
            Sentence {
                text: String::from(sentence_text),
                next_start_type: sentence_type,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    /// Grammar error
    pub struct GrammarError {
        /// Error code
        pub code: i32,
        /// Start position of the error in characters
        pub start_pos: usize,
        /// Length of the error in characters
        pub length: usize,
        /// A list of suggestions for correcting the grammar error
        pub suggestions: Vec<String>,
        /// A localized short description of the grammar error
        pub description: String,
    }

    #[derive(Debug)]
    /// Error in initializing libvoikko
    pub struct InitError {
        message: String,
    }

    #[allow(missing_docs)]
    impl InitError {
        pub fn new(message: &str) -> InitError {
            InitError {
                message: String::from(message),
            }
        }
    }

    impl std::fmt::Display for InitError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl error::Error for InitError {
        fn description(&self) -> &str {
            self.message.as_str()
        }
    }

    impl std::convert::From<std::ffi::NulError> for InitError {
        fn from(error: std::ffi::NulError) -> Self {
            InitError {
                message: format!("{}", error)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    /// Error hyphenating a string
    pub struct HyphenateError {
        message: String,
    }

    #[allow(missing_docs)]
    impl HyphenateError {
        pub fn new(message: &str) -> Self {
            HyphenateError {
                message: String::from(message),
            }
        }
    }

    impl std::fmt::Display for HyphenateError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl error::Error for HyphenateError {
        fn description(&self) -> &str {
            self.message.as_str()
        }
    }

    impl std::convert::From<std::ffi::NulError> for HyphenateError {
        fn from(error: std::ffi::NulError) -> Self {
            HyphenateError {
                message: format!("{}", error)
            }
        }
    }

    impl std::convert::From<std::str::Utf8Error> for HyphenateError {
        fn from(error: std::str::Utf8Error) -> Self {
            HyphenateError {
                message: format!("{}", error)
            }
        }
    }

    impl Voikko {
        /// Initializes Voikko and returns a `Result<Voikko, InitError>`
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
        /// Returns an `InitError` result if init fails.
        pub fn new(language: &str, path: Option<&str>) -> Result<Voikko, InitError> {
            let v = libvoikko::init(language, path);

            match v {
                Ok(handle) => Ok(Voikko { handle }),
                Err(error) => Err(error),
            }
        }

        /// Check the spelling of a UTF-8 character string.
        ///
        /// # Arguments
        ///
        /// * `word` - word to check
        #[must_use]
        pub fn spell(&self, word: &str) -> SpellReturn {
            let ret = libvoikko::spell(self.handle, word);
            match ret {
                Ok(code) => match code {
                    0 => SpellReturn::SpellFailed,
                    1 => SpellReturn::SpellOk,
                    3 => SpellReturn::CharsetConversionFailed,
                    _ => SpellReturn::InternalError,
                },
                Err(_) => SpellReturn::SpellFailed,
            }

        }

        /// Finds suggested correct spellings for given UTF-8 encoded word.
        /// Returns a vector of strings - an empty vector, if no suggestions.
        ///
        /// # Arguments
        ///
        /// * `word` - word to find suggestions for
        #[must_use]
        pub fn suggest(&self, word: &str) -> Vec<String> {
            libvoikko::suggest(self.handle, word).unwrap_or_else(|_| vec![])
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
        /// Returns an error result on error.
        pub fn hyphens(&self, word: &str) -> Result<String, bool> {
            libvoikko::hyphens(self.handle, word)
        }

        /// Hyphenates the given word in UTF-8 encoding.
        /// Returns a string where caller-supplied characters are inserted in all hyphenation points.
        ///
        /// # Arguments
        ///
        /// * `word` - word to hyphenate
        /// * `hyphen` - string to insert at hyphenation points
        ///
        /// # Errors
        ///
        /// Returns an error result on error.
        pub fn hyphenate(&self, word: &str, hyphen: &str) -> Result<String, bool> {
            let hyphens = self.hyphens(word);
            match hyphens {
                Err(_) => Err(false),
                Ok(hyph) => Ok(word
                    .graphemes(true)
                    .zip(hyph.graphemes(true))
                    .map(|(w, h)| match h {
                        // " " => String::from(w),
                        "-" => format!("{}{}", hyphen, w),
                        "=" => String::from(hyphen),
                        _ => String::from(w),
                    })
                    .collect::<String>()),
            }
        }

        /// Hyphenates the given word in UTF-8 encoding.
        /// Returns a string where caller-supplied characters are inserted in all hyphenation points.
        /// **Requires libvoikko version 4.2.0 or greater.**
        ///
        /// # Arguments
        ///
        /// * `word` - word to hyphenate
        /// * `character` - string to insert at hyphenation points
        /// * `allow_context_changes` - boolean parameter controlling whether to insert hyphens even if they alter the word
        ///
        /// # Examples
        ///
        /// ```
        /// # use voikko_rs::voikko;
        /// # let v = voikko::Voikko::new("fi-x-morphoid", None).unwrap();
        /// // Voikko initialized on the variable v
        /// let hyphenated1 = v.hyphenate_new("rei'ittää", "-", true);
        /// assert_eq!(hyphenated1, Ok(String::from("rei-it-tää")));
        /// let hyphenated2 = v.hyphenate_new("rei'ittää", "-", false);
        /// assert_eq!(hyphenated2, Ok(String::from("rei'it-tää")));
        ///
        /// ```
        ///
        /// # Errors
        ///
        /// Is Err if libvoikko returns a null pointer, i.e. it fails to hyphenate.
        pub fn hyphenate_new(&self, word: &str, character: &str, allow_context_changes: bool) -> Result<String, HyphenateError> {
            libvoikko::insert_hyphens(self.handle, word, character, allow_context_changes)
        }

        /// Tokenize a text string. Returns a vector of Token structs.
        ///
        /// # Arguments
        ///
        /// * `text` - Text to find tokens in.
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[must_use]
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
                let token_text: String = text[offset..].chars().take(token_len).collect();
                let token = Token::new(&token_text, token_type);
                tokenlist.push(token);
                offset += token_text.as_bytes().len();
            }
            tokenlist
        }

        /// Find sentences in a text string. Returns a vector of Sentence structs.
        ///
        /// # Arguments
        ///
        /// * `text` - Text to find sentences in.
        #[allow(clippy::match_wildcard_for_single_variants)]
        #[must_use]
        pub fn sentences(&self, text: &str) -> Vec<Sentence> {
            let mut sentlist = Vec::new();
            let mut offset = 0;
            let mut next_start_type = SentenceType::NoStart;
            while offset < text.chars().count() && next_start_type != SentenceType::None {
                // sent_len is in UTF-8 characters, not bytes
                let next_text = text.chars().skip(offset).collect::<String>();
                let (raw_sent, sent_len) =
                    libvoikko::next_sentence(self.handle, next_text.as_str());
                next_start_type = match raw_sent {
                    libvoikko::voikko_sentence_type::SENTENCE_NO_START => SentenceType::NoStart,
                    libvoikko::voikko_sentence_type::SENTENCE_POSSIBLE => SentenceType::Possible,
                    libvoikko::voikko_sentence_type::SENTENCE_PROBABLE => SentenceType::Probable,
                    _ => SentenceType::None,
                };
                // construct new Sentence object with text slice and sentence type
                let token = Sentence::new(
                    text.chars()
                        .skip(offset)
                        .take(sent_len)
                        .collect::<String>()
                        .as_str(),
                    next_start_type,
                );
                sentlist.push(token);
                offset += sent_len;
            }
            sentlist
        }

        /// Analyzes the morphology of given word.
        ///
        /// Returns a vector of Analysis structs (`std::collections::HashMap`) or an empty vector if
        /// analysis fails.
        ///
        /// # Arguments
        ///
        /// * `word` - word to analyze
        // https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/doc/morphological-analysis.txt
        #[must_use]
        pub fn analyze(&self, word: &str) -> Vec<Analysis> {
            libvoikko::analyze_word(self.handle, word).unwrap_or_else(|_| vec![])
        }

        /// Find all grammar errors in given text.
        ///
        /// Returns a vector of `GrammarError` structs or an empty vector if no errors found.
        ///
        /// # Arguments
        ///
        /// * `text` - Text to find grammar errors in. The text should usually begin at the start of
        ///            a paragraph or sentence.
        /// * `desc_lang` - ISO language code for the language in which to recieve error descriptions.
        #[must_use]
        pub fn grammar_errors(&self, text: &str, desc_lang: &str) -> Vec<GrammarError> {
            libvoikko::get_grammar_errors(self.handle, text, desc_lang).unwrap_or_else(|_| vec![])
        }

        // Values of option constants documented in
        // https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/src/voikko_defines.h

        // Boolean options

        /// Ignore dot at the end of the word (needed for use in some word processors).
        /// If this option is set and input word ends with a dot, spell checking and
        /// hyphenation functions try to analyze the word without the dot if no results
        /// can be obtained for the original form. Also with this option, string tokenizer
        /// will consider trailing dot of a word to be a part of that word.
        ///
        /// Default: false
        pub fn set_opt_ignore_dot(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 0, value)
        }

        /// (Spell checking only) Ignore words containing numbers
        ///
        /// Default: false
        pub fn set_opt_ignore_numbers(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 1, value)
        }

        /// Accept words that are written completely in uppercase letters without checking
        /// them at all.
        ///
        /// Default: false
        pub fn set_opt_ignore_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 3, value)
        }

        /// Accept words even when the first letter is in uppercase (start of sentence etc.)
        ///
        /// Default: true
        pub fn set_opt_accept_first_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 6, value)
        }

        /// Accept words even when all of the letters are in uppercase. Note that this is
        /// not the same as `set_opt_ignore_uppercase(true)`: with this option the word is still
        /// checked, only case differences are ignored.
        ///
        /// Default: true
        pub fn set_opt_accept_all_uppercase(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 7, value)
        }

        /// Do not insert hyphenation positions that are considered to be ugly but correct
        ///
        /// Default: false
        pub fn set_opt_no_ugly_hyphenation(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 4, value)
        }

        /// Use suggestions optimized for optical character recognition software.
        /// By default suggestions are optimized for typing errors.
        ///
        /// Default: false
        pub fn set_opt_ocr_suggestions(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 8, value)
        }

        /// (Spell checking only): Ignore non-words such as URLs and email addresses.
        ///
        /// Default: true
        pub fn set_opt_ignore_nonwords(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 10, value)
        }

        /// (Spell checking only): Allow some extra hyphens in words. This option relaxes
        /// hyphen checking rules to work around some unresolved issues in the underlying
        /// morphology, but it may cause some incorrect words to be accepted. The exact
        /// behavior (if any) of this option is not specified.
        ///
        /// Default: false */
        pub fn set_opt_accept_extra_hyphens(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 11, value)
        }

        /// (Spell checking only): Accept missing hyphens at the start and end of the word.
        /// Some application programs do not consider hyphens to be word characters. This
        /// is a reasonable assumption for many languages but not for Finnish. If the
        /// application cannot be fixed to use a proper tokenisation algorithm for Finnish,
        /// this option may be used to tell libvoikko to work around this defect.
        ///
        /// Default: false
        pub fn set_opt_accept_missing_hyphens(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 12, value)
        }

        /// (Grammar checking only): Accept incomplete sentences that could occur in
        /// titles or headings. Set this option to true if your application is not able
        /// to differentiate titles from normal text paragraphs, or if you know that
        /// you are checking title text.
        ///
        /// Default: false
        pub fn set_opt_accept_titles_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 13, value)
        }

        /// (Grammar checking only): Accept incomplete sentences at the end of the
        /// paragraph. These may exist when text is still being written.
        ///
        /// Default: false
        pub fn set_opt_accept_unfinished_paragraphs_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 14, value)
        }

        /// (Hyphenation only): Hyphenate unknown words.
        ///
        /// Default: true
        pub fn set_opt_hyphenate_unknown_words(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 15, value)
        }

        /// (Grammar checking only): Accept paragraphs if they would be valid within
        /// bulleted lists.
        ///
        /// Default: false
        pub fn set_opt_accept_bulleted_lists_in_gc(&self, value: bool) -> bool {
            libvoikko::set_bool_option(self.handle, 16, value)
        }

        // Integer options

        /// The minimum length for words that may be hyphenated. This limit is also enforced on
        /// individual parts of compound words.
        ///
        /// Default: 2
        pub fn set_min_hyphenated_word_length(&self, value: i32) -> bool {
            libvoikko::set_int_option(self.handle, 9, value)
        }

        /// Size of the spell checker cache. This can be -1 (no cache) or
        /// >= 0 ( size in bytes = `2^cache_size * (6544*sizeof(wchar_t) + 1008)` ).
        ///
        /// Default: 0
        pub fn set_speller_cache_size(&self, value: i32) -> bool {
            libvoikko::set_int_option(self.handle, 17, value)
        }
    }

    impl Drop for Voikko {
        fn drop(&mut self) {
            libvoikko::terminate(self.handle);
        }
    }
}
