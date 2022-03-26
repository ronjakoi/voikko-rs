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

use crate::voikko;
use libc::{c_char, c_int, size_t};
use std::ffi;

#[repr(C)]
pub struct VoikkoHandle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct VoikkoGrammarError {
    _private: [u8; 0],
}
#[repr(C)]
pub struct voikko_dict {
    _private: [u8; 0],
}
#[repr(C)]
pub struct voikko_mor_analysis {
    _private: [u8; 0],
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum voikko_token_type {
    TOKEN_NONE,
    TOKEN_WORD,
    TOKEN_PUNCTUATION,
    TOKEN_WHITESPACE,
    TOKEN_UNKNOWN,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum voikko_sentence_type {
    SENTENCE_NONE,
    SENTENCE_NO_START,
    SENTENCE_PROBABLE,
    SENTENCE_POSSIBLE,
}

#[link(name = "voikko")]
#[allow(non_snake_case)]
// https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/src/voikko.h
extern "C" {
    fn voikkoInit(
        error: *mut *const c_char,
        langcode: *const c_char,
        path: *const c_char,
    ) -> *mut VoikkoHandle;

    fn voikkoTerminate(handle: *mut VoikkoHandle);

    fn voikkoSetBooleanOption(handle: *mut VoikkoHandle, option: c_int, value: c_int) -> c_int;

    fn voikkoSetIntegerOption(handle: *mut VoikkoHandle, option: c_int, value: c_int) -> c_int;

    fn voikkoSpellCstr(handle: *mut VoikkoHandle, word: *const c_char) -> c_int;

    fn voikkoSuggestCstr(handle: *mut VoikkoHandle, word: *const c_char) -> *mut *mut c_char;

    fn voikkoHyphenateCstr(handle: *mut VoikkoHandle, word: *const c_char) -> *mut c_char;

    fn voikkoInsertHyphensCstr(handle: *mut VoikkoHandle, word: *const c_char, hyphen: *const c_char,
                               allowContextChanges: c_int) -> *mut c_char;

    fn voikkoFreeCstrArray(cstrArray: *mut *mut c_char);

    fn voikkoFreeCstr(cstr: *mut c_char);

    fn voikkoNextTokenCstr(
        handle: *mut VoikkoHandle,
        text: *const c_char,
        textlen: size_t,
        tokenlen: *mut size_t,
    ) -> voikko_token_type;

    fn voikkoNextSentenceStartCstr(
        handle: *mut VoikkoHandle,
        text: *const c_char,
        textlen: size_t,
        textlen: *mut size_t,
    ) -> voikko_sentence_type;

    fn voikkoNextGrammarErrorCstr(
        handle: *mut VoikkoHandle,
        text: *const c_char,
        textlen: size_t,
        startpos: size_t,
        skiperrors: c_int,
    ) -> *mut VoikkoGrammarError;

    fn voikkoGetGrammarErrorCode(error: *const VoikkoGrammarError) -> c_int;

    fn voikkoGetGrammarErrorStartPos(error: *const VoikkoGrammarError) -> size_t;

    fn voikkoGetGrammarErrorLength(error: *const VoikkoGrammarError) -> size_t;

    fn voikkoGetGrammarErrorSuggestions(error: *const VoikkoGrammarError) -> *const *const c_char;

    fn voikkoFreeGrammarError(error: *mut VoikkoGrammarError);

    fn voikkoGetGrammarErrorShortDescription(
        error: *mut VoikkoGrammarError,
        language: *const c_char,
    ) -> *mut c_char;

    fn voikkoFreeErrorMessageCstr(message: *mut c_char);

    fn voikko_list_dicts(path: *const c_char) -> *mut *mut voikko_dict;

    fn voikko_free_dicts(dicts: *mut *mut voikko_dict);

    fn voikko_dict_language(dict: *const voikko_dict) -> *const c_char;

    fn voikko_dict_script(dict: *const voikko_dict) -> *const c_char;

    fn voikko_dict_variant(dict: *const voikko_dict) -> *const c_char;

    fn voikko_dict_description(dict: *const voikko_dict) -> *const c_char;

    fn voikkoListSupportedSpellingLanguages(path: *const c_char) -> *const *const c_char;

    fn voikkoListSupportedHyphenationLanguages(path: *const c_char) -> *const *const c_char;

    fn voikkoListSupportedGrammarCheckingLanguages(path: *const c_char) -> *const *const c_char;

    fn voikkoGetVersion() -> *const c_char;

    fn voikkoAnalyzeWordCstr(
        handle: *mut VoikkoHandle,
        word: *const c_char,
    ) -> *mut *mut voikko_mor_analysis;

    fn voikko_free_mor_analysis(analysis: *mut *mut voikko_mor_analysis);

    fn voikko_mor_analysis_keys(analysis: *const voikko_mor_analysis) -> *const *const c_char;

    fn voikko_mor_analysis_value_cstr(
        analysis: *const voikko_mor_analysis,
        key: *const c_char,
    ) -> *mut c_char;

    fn voikko_free_mor_analysis_value_cstr(analysis_value: *mut c_char);
}

pub fn init(language: &str, path: Option<&str>) -> Result<*mut VoikkoHandle, voikko::InitError> {
    let path_ptr = match path {
        Some(x) => {
            let tmp_cstring = ffi::CString::new(x)?;
            tmp_cstring.as_ptr()
        },
        None => std::ptr::null() as *const c_char,
    };
    let handle_ptr;
    let error_ptr: *mut *const c_char = &mut std::ptr::null();
    unsafe {
        let lang = ffi::CString::new(language)?;
        let lang_ptr = lang.as_ptr() as *const c_char;
        handle_ptr = voikkoInit(error_ptr, lang_ptr, path_ptr);
    }

    if handle_ptr.is_null() {
        let error = unsafe { ffi::CStr::from_ptr(*error_ptr).to_str().unwrap_or_default() };
        Err(voikko::InitError::new(error))
    } else {
        Ok(handle_ptr)
    }
}

pub fn terminate(handle: *mut VoikkoHandle) {
    unsafe {
        voikkoTerminate(handle);
    }
}

pub fn version<'a>() -> &'a str {
    unsafe {
        let version_ptr = voikkoGetVersion();
        ffi::CStr::from_ptr(version_ptr).to_str().unwrap()
    }
}

pub fn spell(handle: *mut VoikkoHandle, word: &str) -> Result<isize, ffi::NulError> {
    let word_cstring = ffi::CString::new(word)?;
    let res = unsafe { voikkoSpellCstr(handle, word_cstring.as_ptr()) };
    Ok(res as isize)
}

pub fn suggest(handle: *mut VoikkoHandle, word: &str) -> Result<Vec<String>, ffi::NulError> {
    let word_cstring = ffi::CString::new(word)?;
    let ptr = unsafe { voikkoSuggestCstr(handle, word_cstring.as_ptr()) }
        as *mut *mut c_char;
    Ok(get_string_vec(ptr, true))
}

pub fn hyphens(handle: *mut VoikkoHandle, word: &str) -> Result<String, bool> {
    let word_cstring = ffi::CString::new(word);
    match word_cstring {
        Err(_) => Err(false),
        Ok(wcst) => {
            let ptr = unsafe { voikkoHyphenateCstr(handle, wcst.as_ptr()) };
            if ptr.is_null() {
                Err(false)
            } else {
                let cstr = unsafe { ffi::CStr::from_ptr(ptr).to_str().unwrap() };
                let ret = cstr.to_string();
                unsafe {
                    voikkoFreeCstr(ptr);
                }
                Ok(ret)
            }
        }
    }
}

pub fn insert_hyphens(handle: *mut VoikkoHandle, word: &str, hyphen: &str, allow_context_changes: bool) -> Result<String, voikko::HyphenateError> {
    let word_cstring = ffi::CString::new(word)?;
    let hyphen_cstring = ffi::CString::new(hyphen)?;
    let ptr = unsafe {
        voikkoInsertHyphensCstr(handle,
                                word_cstring.as_ptr(),
                                hyphen_cstring.as_ptr(),
                                allow_context_changes as c_int)
    };
    if ptr.is_null() {
        Err(voikko::HyphenateError::new("Error hyphenating string: null pointer from libvoikko"))
    } else {
        let cstr = unsafe { ffi::CStr::from_ptr(ptr).to_str()? };
        let ret = cstr.to_string();
        unsafe {
            voikkoFreeCstr(ptr);
        }
        Ok(ret)
    }
}

pub fn next_token(handle: *mut VoikkoHandle, text: &str) -> (voikko_token_type, usize) {
    let mut tokenlen = 0;
    let tokenlen_ptr: *mut size_t = &mut tokenlen;
    let token;
    unsafe {
        let text_cstr = ffi::CString::new(text).unwrap();
        let text_ptr = text_cstr.as_ptr();
        token = voikkoNextTokenCstr(handle, text_ptr, text.len(), tokenlen_ptr);
        tokenlen = std::ptr::read_unaligned(tokenlen_ptr) as usize;
    }

    (token, tokenlen)
}

// 'text' is a pointer to the start of our buffer, in terms of bytes.
// however, the return value 'sentlen' is a unicode character count. tricky.
pub fn next_sentence(handle: *mut VoikkoHandle, text: &str) -> (voikko_sentence_type, usize) {
    let mut sentlen = 0;
    let sentlen_ptr: *mut size_t = &mut sentlen;
    let sentence;
    unsafe {
        let text_cstr = ffi::CString::new(text).unwrap();
        let text_ptr = text_cstr.as_ptr();
        sentence = voikkoNextSentenceStartCstr(handle, text_ptr, text.len(), sentlen_ptr);
        sentlen = std::ptr::read_unaligned(sentlen_ptr) as usize;
    }

    (sentence, sentlen)
}

pub fn list_dicts(path: &str) -> Result<Vec<voikko::Dictionary>, ffi::NulError> {
    let mut vect = Vec::new();
    let path_cstring = ffi::CString::new(path)?;
    let ptr = unsafe { voikko_list_dicts(path_cstring.as_ptr()) };
    if ptr.is_null() {
        Ok(vect)
    } else {
        unsafe {
            let mut i = 0;
            while !(*ptr.offset(i)).is_null() {
                let lang_ptr = voikko_dict_language(*ptr.offset(i));
                let script_ptr = voikko_dict_script(*ptr.offset(i));
                let variant_ptr = voikko_dict_variant(*ptr.offset(i));
                let desc_ptr = voikko_dict_description(*ptr.offset(i));
                let lang_str = ffi::CStr::from_ptr(lang_ptr).to_str().unwrap();
                let script_str = ffi::CStr::from_ptr(script_ptr).to_str().unwrap();
                let variant_str = ffi::CStr::from_ptr(variant_ptr).to_str().unwrap();
                let desc_str = ffi::CStr::from_ptr(desc_ptr).to_str().unwrap();
                vect.push(voikko::Dictionary::new(
                    lang_str,
                    script_str,
                    variant_str,
                    desc_str,
                ));
                i += 1;
            }
            voikko_free_dicts(ptr);
        }
        Ok(vect)
    }
}

// Get vector of Strings from double pointer to c_char.
// Also free memory reserved by the pointer.
fn get_string_vec(ptr: *mut *mut c_char, free_memory: bool) -> Vec<String> {
    let mut vect = Vec::new();
    if ptr.is_null() {
        vect
    } else {
        unsafe {
            let mut i = 0;
            while !(*ptr.offset(i)).is_null() {
                vect.push(String::from(
                    ffi::CStr::from_ptr(*ptr.offset(i)).to_str().unwrap(),
                ));
                i += 1;
            }
            if free_memory {
                voikkoFreeCstrArray(ptr);
            }
        }
        vect
    }
}

pub fn list_supported_spelling_languages(path: &str) -> Result<Vec<String>, ffi::NulError> {
    let path_cstring = ffi::CString::new(path)?;
    let ptr =
        unsafe { voikkoListSupportedSpellingLanguages(path_cstring.as_ptr()) }
            as *mut *mut c_char;
    Ok(get_string_vec(ptr, true))
}

pub fn list_supported_hyphenation_languages(path: &str) -> Result<Vec<String>, ffi::NulError> {
    let path_cstring = ffi::CString::new(path)?;
    let ptr = unsafe {
        voikkoListSupportedHyphenationLanguages(path_cstring.as_ptr())
    } as *mut *mut c_char;
    Ok(get_string_vec(ptr, true))
}

pub fn list_supported_grammar_checking_languages(path: &str) -> Result<Vec<String>, ffi::NulError> {
    let path_cstring = ffi::CString::new(path)?;
    let ptr = unsafe {
        voikkoListSupportedGrammarCheckingLanguages(path_cstring.as_ptr())
    } as *mut *mut c_char;
    Ok(get_string_vec(ptr, true))
}

pub fn analyze_word(handle: *mut VoikkoHandle, word: &str) -> Result<Vec<voikko::Analysis>, ffi::NulError> {
    let mut vect = Vec::new();
    let word_cstring = ffi::CString::new(word)?;
    unsafe {
        // NULL-pointer terminated list of analyses
        let analysis_list_ptr =
            voikkoAnalyzeWordCstr(handle, word_cstring.as_ptr());
        if analysis_list_ptr.is_null() {
            Ok(vect)
        } else {
            // loop through list until NULL pointer
            let mut i = 0;
            while !(*analysis_list_ptr.offset(i)).is_null() {
                let mut analysis = voikko::Analysis::new();
                // get all key-value pairs for this analysis
                let keys_ptr = voikko_mor_analysis_keys(*analysis_list_ptr.offset(i));
                let keys = get_string_vec(keys_ptr as *mut *mut c_char, false);
                for key in keys {
                    let key_cstring = ffi::CString::new(key.as_str())?;
                    let value_ptr = voikko_mor_analysis_value_cstr(
                        *analysis_list_ptr.offset(i),
                        key_cstring.as_ptr(),
                    );
                    let value = ffi::CStr::from_ptr(value_ptr).to_str().unwrap_or_default();
                    // insert key-value pair
                    analysis.insert(key, String::from(value));
                    voikko_free_mor_analysis_value_cstr(value_ptr);
                }
                // add this analysis to the return vector
                vect.push(analysis);
                i += 1;
            }
            voikko_free_mor_analysis(analysis_list_ptr);
            Ok(vect)
        }
    }
}

pub fn get_grammar_errors(
    handle: *mut VoikkoHandle,
    text: &str,
    desc_lang: &str,
) -> Result<Vec<voikko::GrammarError>, ffi::NulError> {
    let mut vect: Vec<voikko::GrammarError> = Vec::new();
    unsafe {
        let mut offset = 0;
        loop {
            let input_text_cstr = ffi::CString::new(text).unwrap();
            let input_text_ptr = input_text_cstr.as_ptr() as *const c_char;
            // get pointer to a grammar error C struct. it will be a null pointer if no (more) grammar errors found.
            // this is not documented in libvoikko.h but I checked the C++ function implementation.
            //
            // arguments are:
            // * pointer to VoikkoHandle
            // * pointer to the beginning of the input text buffer
            // * length of the buffer in bytes
            // * offset in characters: which position to start searching from
            // * how many errors to skip from beginning
            let grammar_error_ptr =
                voikkoNextGrammarErrorCstr(handle, input_text_ptr, text.len(), offset, 0);
            if grammar_error_ptr.is_null() {
                voikkoFreeGrammarError(grammar_error_ptr);
                break;
            }

            // start asking things about the error struct
            let error_code = voikkoGetGrammarErrorCode(grammar_error_ptr);
            let start_pos = voikkoGetGrammarErrorStartPos(grammar_error_ptr);
            let error_length = voikkoGetGrammarErrorLength(grammar_error_ptr);
            let suggestions_ptr = voikkoGetGrammarErrorSuggestions(grammar_error_ptr);
            let suggestions = get_string_vec(suggestions_ptr as *mut *mut c_char, false);
            let desc_cstring = ffi::CString::new(desc_lang)?;
            let desc_ptr = voikkoGetGrammarErrorShortDescription(
                grammar_error_ptr,
                desc_cstring.as_ptr(),
            );
            let desc_str = ffi::CStr::from_ptr(desc_ptr).to_str().unwrap();
            // push a new Rust-side GrammarError struct into the vector
            vect.push(voikko::GrammarError {
                code: error_code,
                start_pos,
                length: error_length,
                suggestions,
                description: desc_str.to_string(),
            });

            // free some memory
            voikkoFreeErrorMessageCstr(desc_ptr);
            voikkoFreeGrammarError(grammar_error_ptr);

            // increment offset for next loop
            offset += start_pos + error_length;
        }
    }
    Ok(vect)
}

pub fn set_bool_option(handle: *mut VoikkoHandle, option: isize, value: bool) -> bool {
    let res = unsafe { voikkoSetBooleanOption(handle, option as c_int, value as c_int) };
    match res {
        0 => false,
        _ => true,
    }
}

pub fn set_int_option(handle: *mut VoikkoHandle, option: isize, value: isize) -> bool {
    let res = unsafe { voikkoSetIntegerOption(handle, option as c_int, value as c_int) };
    match res {
        0 => false,
        _ => true,
    }
}
