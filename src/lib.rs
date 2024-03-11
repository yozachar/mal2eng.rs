/* --------------------------------------------------------------------------------
Port of: https://github.com/knadh/ml2en

(Phonemic) Romanization of Malayalam script
Transliterates Malayalam script to Roman characters (aka. 'Manglish')
Implements some heuristics try to retain a certain level phonemic fairness.

This work is licensed under Apache License 2.0
Author: Jovial Joe Jayarson
-------------------------------------------------------------------------------- */

mod sketch;

use sketch::Mal2eng;

pub fn transliterate() -> String {
    let mal2eng = Mal2eng::new();
    mal2eng.render("മലയാളം".to_string())
}

#[cfg(test)]
mod tests {
    use crate::transliterate;

    #[test]
    fn test_transliterate() {
        let result = transliterate();
        assert_eq!(result, "malayaalam");
    }
}
