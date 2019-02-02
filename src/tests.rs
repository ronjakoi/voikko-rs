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
}
