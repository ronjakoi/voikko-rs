# voikko-rs

Rust bindings for the [Voikko](https://voikko.puimula.org/) library.

## Requirements

* `libvoikko` version 4.1.1 or newer
* Unit tests currently expect the [`fi-x-morphoid` dictionary package](https://www.puimula.org/htp/testing/voikko-snapshot-v5/)
  to be installed.

## Status

This crate is still under development and not yet feature-complete.
It is also not yet up on [crates.io](https://crates.io/).

So far only tested on Ubuntu 18.04 on x86_64 with `libvoikko`
version 4.1.1.

### Things implemented

* Functions to list the available dictionaries and languages supported
  for spell-checking, hyphenation, and grammar-checking.
* Spell-checking
* Suggested spellings
* Hyphenation
* Tokenization
* Sentence delineation from input text
* Morphological analysis

### Things missing

* Grammar-checking
* Tests and documentation for the option setter functions
