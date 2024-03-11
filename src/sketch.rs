use regex::Regex;
use std::collections::HashMap;

pub struct Mal2eng {
    vowels: HashMap<&'static str, &'static str>,
    compounds: HashMap<&'static str, &'static str>,
    consonants: HashMap<&'static str, &'static str>,
    chill: HashMap<&'static str, &'static str>,
    modifiers: HashMap<&'static str, &'static str>,
}

impl Mal2eng {
    pub fn new() -> Mal2eng {
        let mut vowels_map = HashMap::new();
        for (key, val) in vec![
            ("അ", "a"),
            ("ആ", "aa"),
            ("ഇ", "i"),
            ("ഈ", "ee"),
            ("ഉ", "u"),
            ("ഊ", "oo"),
            ("ഋ", "ru"),
            ("എ", "e"),
            ("ഏ", "e"),
            ("ഐ", "ai"),
            ("ഒ", "o"),
            ("ഓ", "o"),
            ("ഔ", "au"),
        ] {
            vowels_map.insert(key, val);
        }
        let mut compounds_map = HashMap::new();
        for (key, val) in vec![
            ("ക്ക", "kk"),
            ("ഗ്ഗ", "gg"),
            ("ങ്ങ", "ng"),
            ("ച്ച", "cch"),
            ("ജ്ജ", "jj"),
            ("ഞ്ഞ", "nj"),
            ("ട്ട", "tt"),
            ("ണ്ണ", "nn"),
            ("ത്ത", "tth"),
            ("ദ്ദ", "ddh"),
            ("ദ്ധ", "ddh"),
            ("ന്ന", "nn"),
            ("ന്ത", "nth"),
            ("ങ്ക", "nk"),
            ("ണ്ട", "nd"),
            ("ബ്ബ", "bb"),
            ("പ്പ", "pp"),
            ("മ്മ", "mm"),
            ("യ്യ", "yy"),
            ("ല്ല", "ll"),
            ("വ്വ", "vv"),
            ("ശ്ശ", "sh"),
            ("സ്സ", "s"),
            ("ക്സ", "ks"),
            ("ഞ്ച", "nch"),
            ("ക്ഷ", "ksh"),
            ("മ്പ", "mp"),
            ("റ്റ", "tt"),
            ("ന്റ", "nt"),
            ("ന്ത്യ", "nthy"),
        ] {
            compounds_map.insert(key, val);
        }
        let mut consonants_map = HashMap::new();
        for (key, val) in vec![
            ("ക", "k"),
            ("ഖ", "kh"),
            ("ഗ", "g"),
            ("ഘ", "gh"),
            ("ങ", "ng"),
            ("ച", "ch"),
            ("ഛ", "chh"),
            ("ജ", "j"),
            ("ഝ", "jh"),
            ("ഞ", "nj"),
            ("ട", "t"),
            ("ഠ", "dt"),
            ("ഡ", "d"),
            ("ഢ", "dd"),
            ("ണ", "n"),
            ("ത", "th"),
            ("ഥ", "th"),
            ("ദ", "d"),
            ("ധ", "dh"),
            ("ന", "n"),
            ("പ", "p"),
            ("ഫ", "ph"),
            ("ബ", "b"),
            ("ഭ", "bh"),
            ("മ", "m"),
            ("യ", "y"),
            ("ര", "r"),
            ("ല", "l"),
            ("വ", "v"),
            ("ശ", "sh"),
            ("ഷ", "sh"),
            ("സ", "s"),
            ("ഹ", "h"),
            ("ള", "l"),
            ("ഴ", "zh"),
            ("റ", "r"),
        ] {
            consonants_map.insert(key, val);
        }
        let mut chill_map = HashMap::new();
        for (key, val) in vec![
            ("ൽ", "l"),
            ("ൾ", "l"),
            ("ൺ", "n"),
            ("ൻ", "n"),
            ("ർ", "r"),
            ("ൿ", "k"),
        ] {
            chill_map.insert(key, val);
        }
        let mut modifiers_map = HashMap::new();
        for (key, val) in vec![
            ("ു്", "u"),
            ("ാ", "aa"),
            ("ി", "i"),
            ("ീ", "ee"),
            ("ു", "u"),
            ("ൂ", "oo"),
            ("ൃ", "ru"),
            ("െ", "e"),
            ("േ", "e"),
            ("ൈ", "y"),
            ("ൊ", "o"),
            ("ോ", "o"),
            ("ൌ", "ou"),
            ("ൗ", "au"),
            ("ഃ", "a"),
        ] {
            modifiers_map.insert(key, val);
        }
        Mal2eng {
            vowels: vowels_map,
            compounds: compounds_map,
            consonants: consonants_map,
            chill: chill_map,
            modifiers: modifiers_map,
        }
    }

    fn replace_modified_glyphs(&self, glyphs: HashMap<&str, &str>, given_text: String) -> String {
        let gk = glyphs.keys().map(|&k| k).collect::<Vec<_>>().join("|");
        let mk = self
            .modifiers
            .keys()
            .map(|&k| k)
            .collect::<Vec<_>>()
            .join("|");
        let exp = regex::Regex::new(&format!("({gk})({mk})")).unwrap();
        let mut modified_text = given_text.to_owned();
        for mch in exp.captures_iter(&given_text) {
            let matched_str = mch.get(0).unwrap().as_str();
            let matched_glyph = glyphs.get(mch.get(1).unwrap().as_str()).unwrap();
            let matched_modifier = self.modifiers.get(mch.get(2).unwrap().as_str()).unwrap();
            modified_text =
                modified_text.replace(matched_str, &format!("{matched_glyph}{matched_modifier}"))
        }
        modified_text
    }

    pub fn render(&self, given_text: String) -> String {
        let mut modified_text = given_text.to_owned();
        // replace zero width non joiners
        // \u{200C} == \xE2\x80\x8C
        modified_text = Regex::new("\u{200C}")
            .unwrap()
            .replace_all(&modified_text, "")
            .to_string();
        modified_text = self.replace_modified_glyphs(self.compounds.clone(), modified_text);
        modified_text = self.replace_modified_glyphs(self.vowels.clone(), modified_text);
        modified_text = self.replace_modified_glyphs(self.consonants.clone(), modified_text);
        println!("{}", modified_text);
        // replace unmodified compounds
        for (key, val) in self.compounds.clone().into_iter() {
            // compounds ending in chandrakkala but not at the end of the word
            modified_text = Regex::new(&(key.to_string() + "്([\\w])"))
                .unwrap()
                .replace_all(&modified_text, &(val.to_string() + "$1"))
                .to_string();
            // compounds ending in chandrakkala have +'u' pronunciation
            modified_text =
                modified_text.replace(&(key.to_string() + "്"), &(val.to_string() + "u"));
            // compounds not ending in chandrakkala have +'a' pronunciation
            modified_text = modified_text.replace(key, &(val.to_string() + "a"));
        }
        println!("{}", modified_text);
        // glyphs not ending in chandrakkala have +'a' pronunciation
        for (key, val) in self.consonants.clone().into_iter() {
            modified_text = Regex::new(&(key.to_string() + "[^്]"))
                .unwrap()
                .replace_all(&modified_text, &(val.to_string() + "a"))
                .to_string();
        }
        println!("{}", modified_text);
        // glyphs ending in chandrakkala not at the end of a word
        for (key, val) in self.consonants.clone().into_iter() {
            modified_text = Regex::new(&(key.to_string() + "്[^[:space:]\\)\\.;,\"'\\/\\\\%!]"))
                .unwrap()
                .replace_all(&modified_text, val)
                .to_string();
        }
        println!("{}", modified_text);
        // remaining glyphs ending in chandrakkala will be at end of words and have a +'u' pronunciation
        for (key, val) in self.consonants.clone().into_iter() {
            modified_text =
                modified_text.replace(&(key.to_string() + "്"), &(val.to_string() + "u"));
        }
        // remaining consonants
        for (key, val) in self.consonants.clone().into_iter() {
            modified_text = modified_text.replace(key, val);
        }
        // remaining vowels
        for (key, val) in self.vowels.clone().into_iter() {
            modified_text = modified_text.replace(key, val);
        }
        // chill glyphs
        for (key, val) in self.chill.clone().into_iter() {
            modified_text = modified_text.replace(key, val);
        }
        // anusvaram 'am' at the end
        modified_text = modified_text.replace("ം", "m");
        // replace any stray modifiers that may have been left out
        for (key, val) in self.modifiers.clone().into_iter() {
            modified_text = modified_text.replace(key, val);
        }
        let chunks_regex = Regex::new("([.!?] *)").unwrap();
        modified_text = chunks_regex
            .split(&modified_text)
            .collect::<Vec<_>>()
            .join("");
        modified_text
    }
}
