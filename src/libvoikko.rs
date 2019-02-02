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

use libc::{c_int, c_char, size_t};
use std::ffi;

#[repr(C)] pub struct VoikkoHandle { _private: [u8; 0] }
#[repr(C)] pub struct VoikkoGrammarError { _private: [u8; 0] }
#[repr(C)] pub struct voikko_dict { _private: [u8; 0] }
#[repr(C)] pub struct voikko_mor_analysis { _private: [u8; 0] }

#[repr(C)]
#[allow(non_camel_case_types)]
enum voikko_token_type {
    TOKEN_NONE,
    TOKEN_WORD,
    TOKEN_PUNCTUATION,
    TOKEN_WHITESPACE,
    TOKEN_UNKNOWN
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum voikko_sentence_type {
    SENTENCE_NONE,
    SENTENCE_NO_START,
    SENTENCE_PROBABLE,
    SENTENCE_POSSIBLE
}

#[link(name = "voikko")]
#[allow(non_snake_case)]
// https://github.com/voikko/corevoikko/blob/rel-libvoikko-4.1.1/libvoikko/src/voikko.h
extern "C" {
    fn voikkoInit(error: *const *const c_char, langcode: *const c_char,
                  path: *const c_char) -> *mut VoikkoHandle;

    fn voikkoTerminate(handle: *mut VoikkoHandle);

    fn voikkoSetBooleanOption(handle: *mut VoikkoHandle, option: c_int,
                              value: c_int) -> c_int;

    fn voikkoSetIntegerOption(handle: *mut VoikkoHandle, option: c_int,
                              value: c_int) -> c_int;

    fn voikkoSpellCstr(handle: *mut VoikkoHandle, word: *const c_char) -> c_int;

    fn voikkoSuggestCstr(handle: *mut VoikkoHandle, word: *const c_char) -> *mut *mut c_char;

    fn voikkoHyphenateCstr(handle: *mut VoikkoHandle, word: *const c_char) -> *mut c_char;

    //fn voikkoInsertHyphensCstr(handle: *mut VoikkoHandle, word: *const c_char, hyphen: *const c_char,
    //                           allowContextChanges: c_int) -> *mut c_char;

    fn voikkoFreeCstrArray(cstrArray: *mut *mut c_char);

    fn voikkoFreeCstr(cstr: *mut c_char);

    fn voikkoNextTokenCstr(handle: *mut VoikkoHandle, text: *const c_char, textlen: size_t,
                           tokenlen: *mut size_t) -> voikko_token_type;

    fn voikkoNextSentenceStartCstr(handle: *mut VoikkoHandle, text: *const c_char, textlen: size_t,
                                   tokenlen: *mut size_t) -> voikko_sentence_type;

    fn voikkoNextGrammarErrorCstr(handle: *mut VoikkoHandle, text: *const c_char, textlen: size_t,
                                  startpos: size_t, skiperrors: c_int) -> VoikkoGrammarError;

    fn voikkoGetGrammarErrorCode(error: *const VoikkoGrammarError) -> c_int;

    fn voikkoGetGrammarErrorStartPos(error: *const VoikkoGrammarError) -> size_t;

    fn voikkoGetGrammarErrorLength(error: *const VoikkoGrammarError) -> size_t;

    fn voikkoGetGrammarErrorSuggestions(error: *const VoikkoGrammarError) -> *const *const c_char;

    fn voikkoFreeGrammarError(error: *mut VoikkoGrammarError);

    fn voikkoGetGrammarErrorShortDescription(error: *mut VoikkoGrammarError,
                                             language: *const c_char) -> *mut c_char;

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

    fn voikkoAnalyzeWordCstr(handle: *mut VoikkoHandle, word: *const c_char)
        -> *mut *mut voikko_mor_analysis;

    fn voikko_free_mor_analysis(analysis: *mut *mut voikko_mor_analysis);

    fn voikko_mor_analysis_keys(analysis: *const voikko_mor_analysis) -> *const *const c_char;

    fn voikko_mor_analysis_value_cstr(analysis: *const voikko_mor_analysis, key: *const c_char)
        -> *mut c_char;

    fn voikko_free_mor_analysis_value_cstr(analysis_value: *mut c_char);
}

pub fn init(language: &str, path: Option<&str>) -> Result<*mut VoikkoHandle, String> {
    let path_ptr = match path {
        Some(x) => ffi::CString::new(x).expect("CString::new failed").as_ptr(),
        None    => std::ptr::null() as *const c_char
    };
    let handle_ptr;
    let error_ptr = ffi::CString::new("").unwrap().as_ptr() as *const *const c_char;
    unsafe {
        let lang = ffi::CString::new(language).unwrap();
        let lang_ptr = lang.as_ptr() as *const c_char;
        handle_ptr = voikkoInit(error_ptr, lang_ptr, path_ptr);
    }

    if handle_ptr.is_null() {
        let error = unsafe {
            ffi::CStr::from_ptr(*error_ptr).to_str().unwrap_or_default().to_string()
        };
        Err(error)
    } else {
        Ok(handle_ptr)
    }
}

pub fn terminate(handle: *mut VoikkoHandle) {
    unsafe {
        voikkoTerminate(handle);
    }
}

pub fn version() -> String {
    let ver;
    unsafe {
        let version_ptr = voikkoGetVersion();
        ver = ffi::CStr::from_ptr(version_ptr).to_str().unwrap().to_string();
    }
    return ver
}

pub fn spell(handle: *mut VoikkoHandle, word: &str) -> isize {
    let res = unsafe {
        voikkoSpellCstr(handle, ffi::CString::new(word).unwrap().as_ptr())
    };
    res as isize
}

pub fn suggest(handle: *mut VoikkoHandle, word: &str) -> Vec<String> {
    let ptr = unsafe {
        voikkoSuggestCstr(handle, ffi::CString::new(word).unwrap().as_ptr())
    };
    let mut vector = Vec::new();
    if ptr.is_null() {
        return vector;
    } else {
        unsafe {
            let mut i = 0;
            while !(*ptr.offset(i)).is_null() {
                let sug_str = ffi::CStr::from_ptr(*ptr.offset(i)).to_str().unwrap();
                vector.push(String::from(sug_str));
                i += 1;
            }

            voikkoFreeCstrArray(ptr);
        }
        return vector;
    }
}

pub fn hyphenate(handle: *mut VoikkoHandle, word: &str) -> Result<String, bool> {
    let ptr = unsafe {
        voikkoHyphenateCstr(handle, ffi::CString::new(word).unwrap().as_ptr())
    };
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

/* This function uses functionality from the libvoikko 4.2.0 API, but
 * as Ubuntu 18.04 only has 4.1.1, I have not tested it. */
/*pub fn insert_hyphens(handle: *mut VoikkoHandle, word: &str, hyphen: &str, allow_context_changes: bool) -> Result<String, bool> {
    let ptr = unsafe {
        voikkoInsertHyphensCstr(handle,
                                ffi::CString::new(word).unwrap().as_ptr(),
                                ffi::CString::new(hyphen).unwrap().as_ptr(),
                                allow_context_changes as c_int)
    };
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
}*/
