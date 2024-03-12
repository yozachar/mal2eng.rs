use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::collections::HashMap;

pub struct CharacterMap {
    vowels: HashMap<&'static str, &'static str>,
    compounds: HashMap<&'static str, &'static str>,
    consonants: HashMap<&'static str, &'static str>,
    chill: HashMap<&'static str, &'static str>,
    modifiers: HashMap<&'static str, &'static str>,
}

impl CharacterMap {
    /// # CharacterMap::init()
    /// Initializes Malayalam to English transliterator
    ///
    /// Usage:
    /// ```rust
    /// use mal2eng::CharacterMap;
    ///
    /// fn main() {
    ///     let m2e = CharacterMap::init();
    ///     // ... rest of the code ...
    ///     // ... refer `m2e.transliterate` ...
    /// }
    /// ````
    pub fn init() -> CharacterMap {
        let mut vowels_map = HashMap::new();
        for (key, val) in [
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
        for (key, val) in [
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
        for (key, val) in [
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
        for (key, val) in [
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
        for (key, val) in [
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
        CharacterMap {
            vowels: vowels_map,
            compounds: compounds_map,
            consonants: consonants_map,
            chill: chill_map,
            modifiers: modifiers_map,
        }
    }

    /// # replace_modified_glyphs
    fn replace_modified_glyphs(&self, glyphs: &HashMap<&str, &str>, given_text: String) -> String {
        let gk = glyphs.keys().map(|&k| k).collect::<Vec<_>>().join("|");
        let mk = self
            .modifiers
            .keys()
            .map(|&k| k)
            .collect::<Vec<_>>()
            .join("|");
        let exp = regex::Regex::new(&format!("({gk})({mk})"))
            .expect(&format!("E@{}: Unable to build regex", line!()));

        let mut modified_text = given_text.to_owned();

        for cap in exp.captures_iter(&given_text) {
            let matched_str = cap.get(0).unwrap().as_str();
            let matched_glyph = glyphs.get(cap.get(1).unwrap().as_str()).unwrap();
            let matched_modifier = self.modifiers.get(cap.get(2).unwrap().as_str()).unwrap();
            modified_text =
                modified_text.replace(matched_str, &format!("{matched_glyph}{matched_modifier}"))
        }

        modified_text
    }

    /// # render
    fn render(&self, given_text: String, caps: bool) -> String {
        let mut modified_text = given_text.to_owned();

        // replace zero width non joiners
        // \u{200C} == \xE2\x80\x8C
        modified_text = Regex::new("\u{200c}")
            .expect(&format!("E@{}: Unable to build regex", line!()))
            .replace_all(&modified_text, "")
            .to_string();

        modified_text = self.replace_modified_glyphs(&self.compounds, modified_text);
        modified_text = self.replace_modified_glyphs(&self.vowels, modified_text);
        modified_text = self.replace_modified_glyphs(&self.consonants, modified_text);

        // replace unmodified compounds
        for (key, val) in &self.compounds {
            // compounds ending in chandrakkala but not at the end of the word
            modified_text = Regex::new(&format!("{key}്([\\w])"))
                .expect(&format!("E@{}: Unable to build regex", line!()))
                .replace_all(&modified_text, format!("{val}$1"))
                .to_string();
            // compounds ending in chandrakkala have +'u' pronunciation
            modified_text = modified_text.replace(&format!("{key}്"), &format!("{val}u"));
            // compounds not ending in chandrakkala have +'a' pronunciation
            modified_text = modified_text.replace(key, &format!("{val}a"));
        }

        // glyphs not ending in chandrakkala have +'a' pronunciation
        for (key, val) in &self.consonants {
            modified_text = FancyRegex::new(&format!("{key}(?!്)"))
                .expect(&format!("E@{}: Unable to build regex", line!()))
                .replace_all(&modified_text, format!("{val}a"))
                .to_string();
        }

        // glyphs ending in chandrakkala not at the end of a word
        for (key, val) in &self.consonants {
            modified_text = FancyRegex::new(&format!("{key}്(?![\\s\\)\\.;,\"'\\/\\\\%\\!])"))
                .expect(&format!("E@{}: Unable to build regex", line!()))
                .replace_all(&modified_text, format!("{val}"))
                .to_string();
        }
        // println!("{modified_text}");

        // remaining glyphs ending in chandrakkala will be at end of words and have a +'u' pronunciation
        for (key, val) in &self.consonants {
            modified_text = modified_text.replace(&format!("{key}്"), &format!("{val}u"));
        }

        // remaining consonants
        for (key, val) in &self.consonants {
            modified_text = modified_text.replace(key, val);
        }

        // remaining vowels
        for (key, val) in &self.vowels {
            modified_text = modified_text.replace(key, val);
        }

        // chill glyphs
        for (key, val) in &self.chill {
            modified_text = modified_text.replace(key, val);
        }

        // anusvaram 'am' at the end
        modified_text = modified_text.replace("ം", "m");

        // replace any stray modifiers that may have been left out
        for (key, val) in &self.modifiers {
            modified_text = modified_text.replace(key, val);
        }

        if !caps {
            return modified_text;
        }

        // capitalize first letter of modified_text for better aesthetics
        modified_text
            .split_inclusive(&['.', '!', '?'])
            .map(|s| capitalize(s.trim()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// # transliterate
    /// Transliterate Malayalam to English
    ///
    /// Usage:
    /// ```rust
    /// use mal2eng::CharacterMap;
    ///
    /// fn main() {
    ///     let m2e = CharacterMap::init();
    ///     let res = m2e.transliterate("മലയാളത്തിലെ നിങ്ങളുടെ വാക്കുകൾ", true);
    ///     println!("{}", res);
    /// }
    /// ````
    pub fn transliterate(&self, text: &str, capitalize: bool) -> String {
        self.render(text.to_string(), capitalize)
    }
}

fn capitalize(sentence: &str) -> String {
    let mut characters = sentence.chars();
    match characters.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + characters.as_str(),
    }
}
