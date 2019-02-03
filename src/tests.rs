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

#[cfg(test)]
mod tests {
    use crate::voikko::*;

    #[test]
    fn test_init() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
    }

    #[test]
    fn test_version() {
        let version = version();
        println!("version: {}", version);
    }

    #[test]
    fn test_spell() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let test0 = v.spell("kuningas");
        let test1 = v.spell("adfasdf");
        assert_eq!(test0, SpellReturn::SpellOk);
        assert_eq!(test1, SpellReturn::SpellFailed);
    }

    #[test]
    fn test_suggest() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let sug = v.suggest("kisse");
        assert_eq!(sug, vec!["kissa", "kusse", "Kessi"]);
    }

    #[test]
    fn test_hyphenate() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let hyph = v.hyphenate("suihkumoottorimekaanikko");
        assert_eq!(hyph, Ok("    - -   - - - -  -  - ".to_string()));
    }

    #[test]
    fn test_insert_hyphens() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let hyph = v.insert_hyphens("suihkumoottorimekaanikko", "-");
        let hyph2 = v.insert_hyphens("rei'ittää", "-");
        let hyph3 = v.insert_hyphens("kuorma-auto", "-");
        assert_eq!(hyph, Ok("suih-ku-moot-to-ri-me-kaa-nik-ko".to_string()));
        assert_eq!(hyph2, Ok("rei-it-tää".to_string()));
        assert_eq!(hyph3, Ok("kuor-ma-au-to".to_string()));
    }

    #[test]
    fn test_tokens() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let tokens = v.tokens("juhannuksen vietto.");
        assert_eq!(tokens[0], Token::new("juhannuksen", TokenType::Word));
        assert_eq!(tokens[1], Token::new(" ", TokenType::Whitespace));
        assert_eq!(tokens[2], Token::new("vietto", TokenType::Word));
        assert_eq!(tokens[3], Token::new(".", TokenType::Punctuation));
    }

    #[test]

    fn test_sentences() { // sentences() doesn't seem to work very reliably
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let text = "Järvenpää kuuluu Uudenmaan maakuntaan. Sen naapurikunnat ovat Mäntsälä koillisessa, \
                    Sipoo idässä ja Tuusula etelässä, lännessä sekä pohjoisessa.";
        let sentences = v.sentences(text);
        assert!(sentences.len() >= 1); // basic sanity check
    }
}
