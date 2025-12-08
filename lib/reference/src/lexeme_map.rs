use std::collections::BTreeMap;
use std::convert::TryFrom;
use anyhow::{Error, anyhow};
use serde_yaml::Value;
use crate::lexeme_meta::LexemeMeta;

#[derive(Debug)]
pub struct LexemeMap(pub BTreeMap<String, Vec<LexemeMeta>>);

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
                    .collect::<anyhow::Result<_>>()?,
                _ => Vec::new(),
            };

            map.insert(key, items);
        }

        Ok(LexemeMap(map))
    }
}

impl From<Vec<LexemeMap>> for LexemeMap {
    fn from(value: Vec<LexemeMap>) -> Self {
        let mut merged = BTreeMap::new();

        for map in value {
            for (key, vec) in map.0 {
                merged.entry(key)
                    .and_modify(|existing_vec: &mut Vec<LexemeMeta>| existing_vec.extend(vec.clone()))
                    .or_insert(vec);
            }
        }

        LexemeMap(merged)
    }
}