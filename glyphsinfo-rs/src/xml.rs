use serde::{Deserialize, Deserializer, Serialize};

use crate::{Case, Category, Direction, Record, Script, SubCategory};

#[derive(Deserialize)]
#[serde(rename = "glyphData")]
pub struct XmlGlyphData {
    #[serde(rename = "$value", default)]
    pub glyph: Vec<XmlRecord>,
}

/// A record as it is in the upstream GlyphData.xml file.
#[derive(Debug, Serialize, Deserialize)]
pub struct XmlRecord {
    #[serde(rename = "@unicode", deserialize_with = "string_to_char", default)]
    pub unicode: Option<char>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@category")]
    pub category: Category,
    #[serde(rename = "@subCategory")]
    pub sub_category: Option<SubCategory>,
    #[serde(rename = "@case")]
    pub case: Option<Case>,
    #[serde(rename = "@script")]
    pub script: Option<Script>,
    #[serde(rename = "@direction")]
    pub direction: Option<Direction>,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@production")]
    pub production_name: Option<String>,
    #[serde(rename = "@altNames", default)]
    pub alterative_names: Vec<String>,
}

impl XmlRecord {
    pub fn into_record(self) -> (String, Record) {
        (
            self.name,
            Record {
                unicode: self.unicode,
                category: self.category,
                sub_category: self.sub_category,
                case: self.case,
                script: self.script,
                description: self.description,
                direction: self.direction,
                production_name: self.production_name,
                alterative_names: self.alterative_names,
            },
        )
    }
}

fn string_to_char<'de, D>(deserializer: D) -> Result<Option<char>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    let char = char::try_from(u32::from_str_radix(value, 16).map_err(serde::de::Error::custom)?)
        .map_err(serde::de::Error::custom)?;
    Ok(Some(char))
}
