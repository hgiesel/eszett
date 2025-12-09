use std::convert::TryInto;
use std::fs;
use std::path::Path;
use anyhow::{anyhow, bail, Result};
use glob::glob;
use serde_yaml::Value;
use crate::lexeme_map::LexemeMap;

pub fn read_files(root: &Path) -> Result<LexemeMap> {
    let pattern = root.join("*.yaml");
    let pattern_str = pattern.to_str().ok_or_else(|| anyhow!("Invalid path"))?;

    let x = glob(pattern_str)?
        .map(|glob_result|
            match glob_result {
                Ok(path) => {
                    let data = fs::read_to_string(path)?;
                    let raw: &Value = &serde_yaml::from_str(data.as_str())?;
                    raw.try_into()
                },
                Err(error) => bail!(error)
            }
        )
        .collect::<Result<Vec<LexemeMap>>>()?;

    Ok(x.into())
}