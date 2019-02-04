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
        let _v = Voikko::new("fi-x-morphoid", None).unwrap();
    }

    #[test]
    fn test_version() {
        let version = version();
        assert!(version.starts_with("4."));
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
        let hyph = v.hyphens("suihkumoottorimekaanikko");
        assert_eq!(hyph, Ok("    - -   - - - -  -  - ".to_string()));
    }

    #[test]
    fn test_insert_hyphens() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let hyph = v.hyphenate("suihkumoottorimekaanikko", "-");
        let hyph2 = v.hyphenate("rei'ittää", "-");
        let hyph3 = v.hyphenate("kuorma-auto", "-");
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

    fn test_sentences() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let text = "Järvenpää kuuluu Uudenmaan maakuntaan. Sen naapurikunnat ovat Mäntsälä koillisessa, \
                    Sipoo idässä ja Tuusula etelässä, lännessä sekä pohjoisessa.";
        let sentences = v.sentences(text);
        assert_eq!(
            sentences[0],
            Sentence::new(
                "Järvenpää kuuluu Uudenmaan maakuntaan. ",
                SentenceType::Probable
            )
        );
        assert_eq!(sentences[1], Sentence::new("Sen naapurikunnat ovat Mäntsälä koillisessa, Sipoo idässä ja Tuusula etelässä, lännessä sekä pohjoisessa.",
                                                SentenceType::None));
    }

    #[test]
    fn test_dictionaries() {
        let dicts = list_dicts("");
        assert_eq!(dicts[0].language, "fi");
    }

    #[test]
    fn test_spelling_languages() {
        let langs = list_supported_spelling_languages("");
        assert!(langs.into_iter().any(|x| x.starts_with("fi")));
    }

    #[test]
    fn test_hyphenation_languages() {
        let langs = list_supported_hyphenation_languages("");
        assert!(langs.into_iter().any(|x| x.starts_with("fi")));
    }

    #[test]
    fn test_gc_languages() {
        let langs = list_supported_grammar_checking_languages("");
        assert!(langs.into_iter().any(|x| x.starts_with("fi")));
    }

    #[test]
    fn test_analyze() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let analyses = v.analyze("kaljakori");
        let mut comparison = Analysis::new();
        comparison.insert("CLASS".to_string(), "nimisana".to_string());
        comparison.insert("FSTOUTPUT".to_string(),
            "[Ln][Xs]504403[X][Xp]kalja[X]kalj[Sn][Ny]a[Bh][Bc][Ln][Xs]506023[X][Xp]kori[X]kor[Sn][Ny]i".to_string());
        comparison.insert("STRUCTURE".to_string(), "=ppppp=pppp".to_string());
        comparison.insert(
            "WORDIDS".to_string(),
            "+kalja(w504403)+kori(w506023)".to_string(),
        );
        comparison.insert("SIJAMUOTO".to_string(), "nimento".to_string());
        comparison.insert("BASEFORM".to_string(), "kaljakori".to_string());
        comparison.insert(
            "WORDBASES".to_string(),
            "+kalja(kalja)+kori(kori)".to_string(),
        );
        comparison.insert("NUMBER".to_string(), "singular".to_string());
        assert_eq!(analyses[0], comparison);
    }

    #[test]
    fn test_gc() {
        let v = Voikko::new("fi-x-morphoid", None).unwrap();
        let errors = v.grammar_errors(
            "Johanneksen leipäpuu pitää pitää leivottu juureen",
            "en",
        );
        assert_eq!(
            errors[0],
            GrammarError {
                code: 8,
                start_pos: 21,
                length: 11,
                suggestions: vec!["pitää".to_string()],
                description: "Remove duplicate word.".to_string()
            }
        );
        assert_eq!(
            errors[1],
            GrammarError {
                code: 9,
                start_pos: 42,
                length: 7,
                suggestions: vec![],
                description: "Terminating punctuation is missing.".to_string()
            }
        );
    }
}
