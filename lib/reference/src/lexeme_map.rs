use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use anyhow::{Error, anyhow};
use serde_json::to_string;
use serde_yaml::Value;
use crate::lexeme_meta::LexemeMeta;

#[derive(Debug, Clone)]
pub struct LexemeMap { pub map: BTreeMap<String, Vec<LexemeMeta>> }

pub const EMPTY_LEXEMES: LexemeMap = LexemeMap {
    map: BTreeMap::new(),
};

impl TryFrom<&Value> for LexemeMap {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::<String, Vec<LexemeMeta>>::new();

        let mapping = value
            .as_mapping()
            .ok_or_else(|| anyhow!("Value is not a mapping"))?;

        for (k, v) in mapping {
            let key = k.as_str()
                .ok_or_else(|| anyhow!("Key is not a string"))?
                .to_string();


            let items = match v {
                Value::Null => Vec::new(),
                Value::Sequence(seq) => seq
                    .into_iter()
                    .map(|v| v.try_into())
                    .collect::<anyhow::Result<_>>()
                    .expect(format!("Could not convert value to vec: {}", key).as_str()),
                _ => Vec::new(),
            };

            map.insert(key, items);
        }

        Ok(LexemeMap { map })
    }
}

impl From<Vec<LexemeMap>> for LexemeMap {
    fn from(value: Vec<LexemeMap>) -> Self {
        let mut merged = BTreeMap::new();

        for map in value {
            for (key, vec) in map.map {
                merged.entry(key)
                    .and_modify(|existing_vec: &mut Vec<LexemeMeta>| existing_vec.extend(vec.clone()))
                    .or_insert(vec);
            }
        }

        LexemeMap { map: merged }
    }
}


impl fmt::Display for LexemeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (lemma, lexemes) in &self.map {
            writeln!(f, "{}:", to_string(lemma).expect("Lemma string conversion failed: {}"))?;

            for lexeme in lexemes {
                writeln!(f, "- [{}, {}{}]", lexeme.part_of_speech, format!("[{}]", lexeme.indicators.iter()
                    .map(|s| to_string(s).expect("Indicator string conversion failed"))
                    .collect::<Vec<_>>()
                    .join(", ")
                ), if let Some(value) = &lexeme.comment {
                    format!(", {}", to_string(&value).expect("Comment string conversion failed"))
                } else {
                    String::from("")
                })?
            }
        }
        Ok(())
    }
}