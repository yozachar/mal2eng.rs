/* --------------------------------------------------------------------------------
Port of: https://github.com/knadh/ml2en

(Phonemic) Romanization of Malayalam script
Transliterates Malayalam script to Roman characters (aka. 'Manglish')
Implements some heuristics try to retain a certain level phonemic fairness.

This work is licensed under Apache License 2.0
Author: Jovial Joe Jayarson
-------------------------------------------------------------------------------- */

mod sketch;

pub use crate::sketch::CharacterMap;

#[cfg(test)]
mod tests {
    use crate::CharacterMap;
    #[test]
    fn test_transliterate() {
        let m2e = CharacterMap::init();
        assert_eq!(
            m2e.transliterate(
                "ഒരു ചെറിയ ചിപ്പിൽ ഇലക്ട്രോണിക് സിഗ്നലുകളുടെ പ്രവാഹം നിയന്ത്രിക്കാനും വർദ്ധിപ്പിക്കാനും ഉപയോഗിക്കുന്ന ഒരു ഉപകരണമാണ് ട്രാൻസിസ്റ്റർ. വാക്വേറ്റ്യൂബുകളെ പോലെ വലുതായിരുന്ന പഴയ ഇലക്ട്രോണിക് ഉപകരണങ്ങളെ മാറ്റിസ്ഥാപിക്കാൻ സഹായിച്ച ഈ കണ്ടുപിടിത്തം റേഡിയോ, ടെലിവിഷൻ, കമ്പ്യൂട്ടർ തുടങ്ങിയ നിരവധി ഇലക്ട്രോണിക് ഉപകരണങ്ങളുടെ വികസനത്തിന്‌ വഴിയൊരുക്കി.", false
            ),
            "oru cheriya chippil ilaktroniku signalukalute pravaaham niyanthrikkaanum varddhippikkaanum upayogikkunna oru upakaranamaanu traansisttar. vaakvettyoobukale pole valuthaayirunna pazhaya ilaktroniku upakaranangale maattisthaapikkaan sahaayiccha ee kandupitittham rediyo, telivishan, kampyoottar thutangiya niravadhi ilaktroniku upakaranangalute vikasanatthinu vazhiyorukki."
        );

        assert_eq!(
            m2e.transliterate(
                "ക്വാണ്ടം ഭൗതികശാസ്ത്രം അതിന്റെ പ്രധാന അവശ്യങ്ങൾ സമ്പരിചയപ്പെടുത്തുന്ന ഒരു സാങ്കേതികതാതീത ശാസ്ത്രമാണ്. പരമാണുകളുടെ വൈവിദ്ധ്യം, തമ്മിലുള്ള ബന്ധങ്ങൾ എന്നിവയെക്കുറിച്ച് പഠിക്കുന്നു. ഇതിന്റെ പ്രധാന ധാരണകൾ പ്രകൃതിയുടെ അവസ്ഥയുടെ കേന്ദ്രസ്ഥാനത്തിലുള്ളവയാണ്. പരമാണുകളുടെ ചരിത്രം, അവയുടെ പ്രകൃതിയും ഗതാഗതവും എന്നിവയെക്കുറിച്ച് പഠിക്കുന്നു. അവയുടെ പ്രക്രിയകൾ എങ്ങനെ നടക്കുന്നുവെന്ന് പരിശോധിക്കുന്നു. ക്വാണ്ടം ഭൗതികശാസ്ത്രത്തിൽ, നിറഞ്ഞ അവകാശം നിർത്തി പോലീസ് ശാസ്ത്രത്തിന്റെ മാനദണ്ഡങ്ങൾ ലളിതമായ രീതിയിൽ നിർവചിക്കുന്നു. പ്രദത്ത ഭൌതികശാസ്ത്രജ്ഞർ ക്വാണ്ടം ഭൗതികശാസ്ത്രം പ്രയോഗിക്കുന്നുണ്ടായിരിക്കുന്ന സംശയങ്ങൾക്ക് അവസരമായ സൂചനകൾ നൽകുന്നു. ഇത് വലിയ ഉലേഖം നൽകുന്ന വിഭവങ്ങൾ ആയിരിക്കാം. ഇത് പരമാണുകളും അവയുടെ സംഘടനയും ക്വാണ്ടം ഭൗതികശാസ്ത്രത്തിൽ പ്രധാനമാണ്. അതിനാൽ, പരമാണുകളുടെ പ്രക്രിയകൾ പരിശോധിക്കാൻ പ്രധാനപ്പെട്ട ക്രിയാത്മകത നൽകുന്നു. ഒരു പ്രധാന പ്രത്യാശയാണ് അത് നമ്പർവദ്ധിച്ചുകൂട്ടുക.", true
            ),
            "Kvaandam bhauthikashaasthram athinte pradhaana avashyangal samparichayappetutthunna oru saankethikathaatheetha shaasthramaanu. Paramaanukalute vyviddhyam, thammilulla bandhangal ennivayekkuricchu padtikkunnu. Ithinte pradhaana dhaaranakal prakruthiyute avasthayute kendrasthaanatthilullavayaanu. Paramaanukalute charithram, avayute prakruthiyum gathaagathavum ennivayekkuricchu padtikkunnu. Avayute prakriyakal engane natakkunnuvennu parishodhikkunnu. Kvaandam bhauthikashaasthratthil, niranja avakaasham nirtthi poleesu shaasthratthinte maanadandangal lalithamaaya reethiyil nirvachikkunnu. Pradattha bhouthikashaasthrajnjar kvaandam bhauthikashaasthram prayogikkunnundaayirikkunna samshayangalkku avasaramaaya soochanakal nalkunnu. Ithu valiya ulekham nalkunna vibhavangal aayirikkaam. Ithu paramaanukalum avayute samghatanayum kvaandam bhauthikashaasthratthil pradhaanamaanu. Athinaal, paramaanukalute prakriyakal parishodhikkaan pradhaanappetta kriyaathmakatha nalkunnu. Oru pradhaana prathyaashayaanu athu namparvaddhicchukoottuka."
        );

        assert_eq!(
            m2e.transliterate(
                "വ്യാഴത്തിന്റെ കാന്തികക്ഷേത്രം സൗരവാതത്തെ ചെറുക്കുന്ന മേഖലയാണ്‌ വ്യാഴത്തിന്റെ കാന്തമണ്ഡലം. സൂര്യനിലേക്കുള്ള ദിശയിൽ ഏതാണ്ട് എഴുപത് ലക്ഷം കിലോമീറ്ററും വിപരീത ദിശയിൽ ശനിയുടെ പരിക്രമണപഥം വരെയും ഇത് വ്യാപിച്ചുകിടക്കുന്നു. സൗരയൂഥത്തിലെ ഗ്രഹങ്ങളുടെ കാന്തമണ്ഡലങ്ങളിൽ വച്ച് ഏറ്റവും ശക്തിയേറിയതാണ്‌ വ്യാഴത്തിന്റേത്. സൗരമണ്ഡലം കഴിഞ്ഞാൽ സൗരയൂഥത്തിലെ ഏറ്റവും വലിയ ഘടനയും ഇതുതന്നെ. ഭൂമിയുടെ കാന്തമണ്ഡലത്തെക്കാൾ വീതിയേറിയതും പരന്നതുമായ വ്യാഴത്തിന്റെ കാന്തമണ്ഡലത്തിന്റെ ശക്തി ഭൂമിയൂടേതിന്റെ പത്തിരട്ടിയോളവും വ്യാപ്തം 18000 ഇരട്ടിയോളവുമാണ്‌.", false
            ),
            "vyaazhatthinte kaanthikakshethram sauravaathatthe cherukkunna mekhalayaanu vyaazhatthinte kaanthamandalam. sooryanilekkulla dishayil ethaandu ezhupathu laksham kilomeettarum vipareetha dishayil shaniyute parikramanapatham vareyum ithu vyaapicchukitakkunnu. saurayoothatthile grahangalute kaanthamandalangalil vacchu ettavum shakthiyeriyathaanu vyaazhatthintethu. sauramandalam kazhinjaal saurayoothatthile ettavum valiya ghatanayum ithuthanne. bhoomiyute kaanthamandalatthekkaal veethiyeriyathum parannathumaaya vyaazhatthinte kaanthamandalatthinte shakthi bhoomiyootethinte patthirattiyolavum vyaaptham 18000 irattiyolavumaanu."
        );

        assert_eq!(m2e.transliterate("മലയാളം", true), "Malayaalam");
    }
}

// #[macro_export]
// macro_rules! transliterate {
//     ($text:expr) => {
//         m2e::init().render($text, true)
//     };
//     ($text:expr, $caps:literal) => {
//         m2e::init().render($text, $caps)
//     };
// }
