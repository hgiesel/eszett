use std::str::FromStr;
use anyhow::{anyhow, bail, Error, Result};
use serde_yaml::Value;
use dto::part_of_speech::PartOfSpeechDto;

#[derive(Debug, Clone)]
pub struct LexemeMeta {
    pub part_of_speech: PartOfSpeechDto,
    pub indicators: Vec<String>,
    pub comment: Option<String>,
}

impl TryFrom<&Value> for LexemeMeta {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let seq = value.as_sequence().ok_or_else(|| anyhow!("Expected a sequence"))?;

        let mut part_of_speech: Option<PartOfSpeechDto> = None;
        let mut indicators: Option<Vec<String>> = None;
        let mut comment: Option<String> = None;

        for (index, value) in seq.into_iter().enumerate() {
            match index {
                0 => {
                    let string = value
                        .as_str()
                        .ok_or_else(|| anyhow!("Expected a string as part-of-speech"))?;
                    part_of_speech = Some(string.parse::<PartOfSpeechDto>()?);
                },
                1 => {
                    let ind = value
                        .as_sequence()
                        .ok_or_else(|| anyhow!("Expected a string as part-of-speech"))?;
                    let indic = ind
                        .into_iter()
                        .map(|x| x.as_str().ok_or_else(|| anyhow!("foo")))
                        .collect::<Result<Vec<_>>>()?
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect();
                    indicators = Some(indic)
                }
                2 => {
                    comment = Some(value.as_str().ok_or_else(|| anyhow!("Expected a string as part-of-speech"))?.to_string());
                }
                _ => {
                    bail!("Too many values in meta array");
                }
            }
        }

        if let Some(p) = part_of_speech && let Some(i) = indicators {
            Ok(LexemeMeta {
                part_of_speech: p,
                indicators: i,
                comment,
            })
        } else {
            bail!("Invalid values lexeme meta")
        }
    }
}
