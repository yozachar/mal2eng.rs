# mal2eng

An algorithm that transliterates Malayalam script to Roman / Latin characters (aka. 'Manglish') with reasonable phonemic fairness.

## Usage

```console
$ cargo add mal2eng
    Updating crates.io index
      Adding mal2eng v0.1.1 to dependencies.
    Updating crates.io index
```

```rs
use mal2eng::CharacterMap;

fn main() {
    let m2e = CharacterMap::init();
    println!("{}",
        m2e.transliterate(
            "വ്യാഴത്തിന്റെ കാന്തികക്ഷേത്രം സൗരവാതത്തെ ചെറുക്കുന്ന മേഖലയാണ്‌ വ്യാഴത്തിന്റെ കാന്തമണ്ഡലം. സൂര്യനിലേക്കുള്ള ദിശയിൽ ഏതാണ്ട് എഴുപത് ലക്ഷം കിലോമീറ്ററും വിപരീത ദിശയിൽ ശനിയുടെ പരിക്രമണപഥം വരെയും ഇത് വ്യാപിച്ചുകിടക്കുന്നു. സൗരയൂഥത്തിലെ ഗ്രഹങ്ങളുടെ കാന്തമണ്ഡലങ്ങളിൽ വച്ച് ഏറ്റവും ശക്തിയേറിയതാണ്‌ വ്യാഴത്തിന്റേത്. സൗരമണ്ഡലം കഴിഞ്ഞാൽ സൗരയൂഥത്തിലെ ഏറ്റവും വലിയ ഘടനയും ഇതുതന്നെ. ഭൂമിയുടെ കാന്തമണ്ഡലത്തെക്കാൾ വീതിയേറിയതും പരന്നതുമായ വ്യാഴത്തിന്റെ കാന്തമണ്ഡലത്തിന്റെ ശക്തി ഭൂമിയൂടേതിന്റെ പത്തിരട്ടിയോളവും വ്യാപ്തം 18000 ഇരട്ടിയോളവുമാണ്‌.",
        true // flag for optional sentence capitalization
        )
    );
}
```

```console
$ cargo run
Vyaazhatthinte kaanthikakshethram sauravaathatthe cherukkunna mekhalayaanu vyaazhatthinte kaanthamandalam. Sooryanilekkulla dishayil ethaandu ezhupathu laksham kilomeettarum vipareetha dishayil shaniyute parikramanapatham vareyum ithu vyaapicchukitakkunnu. Saurayoothatthile grahangalute kaanthamandalangalil vacchu ettavum shakthiyeriyathaanu vyaazhatthintethu. Sauramandalam kazhinjaal saurayoothatthile ettavum valiya ghatanayum ithuthanne. Bhoomiyute kaanthamandalatthekkaal veethiyeriyathum parannathumaaya vyaazhatthinte kaanthamandalatthinte shakthi bhoomiyootethinte patthirattiyolavum vyaaptham 18000 irattiyolavumaanu.
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Rust port of: <https://github.com/knadh/ml2en/>
