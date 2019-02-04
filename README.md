# voikko-rs

[![](http://meritbadge.herokuapp.com/voikko-rs)](https://crates.io/crates/voikko-rs)

Rust bindings for the [Voikko](https://voikko.puimula.org/) library.

## Requirements

* `libvoikko` version 4.1.1 or newer
* Unit tests currently expect the [`fi-x-morphoid` dictionary package](https://www.puimula.org/htp/testing/voikko-snapshot-v5/)
  to be installed.

## Status

The crate is feature-complete but not yet tested very well. Bugs may remain.

So far only tested on Ubuntu 18.04 x86_64 with `libvoikko`
version 4.1.1.

The version number is 1.0 as I do not expect major changes to the API.

### Things implemented

* Functions to list the available dictionaries and languages supported
  for spell-checking, hyphenation, and grammar-checking.
* Spell-checking
* Suggested spellings
* Hyphenation
* Tokenization
* Sentence delineation from input text
* Morphological analysis
* Grammar-checking

### Things missing

* Tests for the option setter functions
