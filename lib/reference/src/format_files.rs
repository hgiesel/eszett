use std::io::Write;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use icu::locale::locale;
use crate::lexeme_map::LexemeMap;
use anyhow::Result;
use icu::collator::{Collator, CollatorPreferences};
use icu::collator::options::{CollatorOptions, Strength};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum LatinCharacterClass {
    A, B, C, D, E, F, G,
    H, I, J, K, L, M, N,
    O, P, Q, R, S, T, U,
    V, W, X, Y, Z, Hash,
}

pub fn categorize(all: LexemeMap) -> Result<BTreeMap<LatinCharacterClass, LexemeMap>> {
    let mut map: BTreeMap<LatinCharacterClass, LexemeMap> = BTreeMap::new();

    let prefs: CollatorPreferences = locale!("en").into();
    let mut options = CollatorOptions::default();
    options.strength = Some(Strength::Tertiary);
    let coll = Collator::try_new(prefs, options)?;

    for (lemma, lexemes) in &all.map {
        let first = lemma.graphemes(true).next().unwrap();

        if coll.compare(first, "a").is_eq() {
            let found = map.entry(LatinCharacterClass::A).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());

        } else if coll.compare(first, "b").is_eq() {
            let found = map.entry(LatinCharacterClass::B).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());

        } else if coll.compare(first, "c").is_eq() {
            let found = map.entry(LatinCharacterClass::C).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());

        } else if coll.compare(first, "d").is_eq() {
            let found = map.entry(LatinCharacterClass::D).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());

        } else if coll.compare(first, "e").is_eq() {
            let found = map.entry(LatinCharacterClass::E).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());

        } else {
            let found = map.entry(LatinCharacterClass::Hash).or_insert(LexemeMap {
                map: BTreeMap::new(),
            });
            found.map.insert(lemma.clone(), lexemes.clone());
        }
    }

    Ok(map)
}

pub fn format_files(path: &Path, map: LexemeMap) -> Result<()> {
    let categories = categorize(map)?;

    for (key, map) in categories {
        let output_path = path.join(format!("data/en/{}.yaml", match key {
            LatinCharacterClass::A => "a",
            LatinCharacterClass::B => "b",
            LatinCharacterClass::C => "c",
            LatinCharacterClass::D => "d",
            LatinCharacterClass::E => "e",
            _ => "#",
        }));

        let mut file = File::create(output_path)?;
        write!(file, "{}", map)?;
    }

    Ok(())
}