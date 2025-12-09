use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use icu::locale::locale;
use crate::lexeme_map::{LexemeMap, EMPTY_LEXEMES};
use anyhow::Result;
use icu::collator::{Collator, CollatorPreferences};
use icu::collator::options::CollatorOptions;
use icu::collator::preferences::{CollationCaseFirst, CollationNumericOrdering, CollationType};
use icu::locale::preferences::LocalePreferences;
use icu::locale::subtags::language;
use base::language::Language;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
enum LatinCharacterClass {
    A, B, C, D, E, F, G,
    H, I, J, K, L, M, N,
    O, P, Q, R, S, T, U,
    V, W, X, Y, Z, Hash,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
enum LatinCharacterClassSpanish {
    A, B, C, D, E, F, G,
    H, I, J, K, L, M, N, Nn,
    O, P, Q, R, S, T, U,
    V, W, X, Y, Z, Hash,
}

struct LocaleKey {
    pub sort_key: Vec<u8>, // stable ordering
    pub original: String,
}

pub fn categorize<LocaleKey>() {
    // let ag = Collator::try_new(
    //
    // );
    let collator = &locale!("sv");//, CollatorOptions::new()).unwrap();

    let mut map: BTreeMap<LocaleKey, i32> = BTreeMap::new();
    // println!("{}", map.len());


    for word in ["zebra", "åland", "örebro", "apple"] {

        // map.insert(key, 1);
    };

    // for k in map.keys() {
    //     println!("{}", k.original);
    // }
}


pub fn format_files(path: &Path, map: LexemeMap) -> Result<()> {
    // let prefs = CollatorPreferences {
    //     locale_preferences: LocalePreferences::default(),
    //     collation_type: Some(CollationType::Eor),
    //     case_first: Some(CollationCaseFirst::Lower),
    //     numeric_ordering: Some(CollationNumericOrdering::False),
    // };
    // prefs.locale_preferences = LocalePreferences::default();
    // prefs.locale_preferences
    //
    // let options = CollatorOptions::default().
    //
    //
    // let x = Collator::try_new(prefs, &locale!("en"))?;
    // let collator = Collator::new(&locale!("fr"), CollatorOptions::new())
    //     .expect("collator");
    // //, CollatorOptions::newk()).unwrap();
    //
    // println!("{:#?}", collator);

    // for key in map.map.keys() {
    //     key
    //
    //
    // }


    categorize::<Language>();
    Ok(())
}
