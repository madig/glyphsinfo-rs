use quick_xml::events::BytesStart;
use serde::{Deserialize, Serialize};

use crate::{Case, Category, Script, SubCategory};

/// A record as it is in the upstream GlyphData.xml file.
#[derive(Debug, Serialize, Deserialize)]
pub struct XmlRecord {
    pub unicode: Option<char>,
    pub name: String,
    pub category: Category,
    pub sub_category: Option<SubCategory>,
    pub case: Option<Case>,
    pub script: Option<Script>,
    pub description: String,
    pub production_name: Option<String>,
    pub alterative_names: Vec<String>,
}

impl TryFrom<BytesStart<'_>> for XmlRecord {
    type Error = &'static str;

    fn try_from(element: BytesStart) -> Result<Self, Self::Error> {
        let mut unicode = None;
        let mut name = String::new();
        let mut category: Option<Category> = None;
        let mut sub_category = None;
        let mut case = None;
        let mut script = None;
        let mut description = String::new();
        let mut production_name = None;
        let mut alterative_names = vec![];

        for attribute in element.attributes().filter_map(|a| a.ok()) {
            if let Some(value) = attribute.unescape_value().ok() {
                match attribute.key.into_inner() {
                    b"unicode" => {
                        if let Some(c) = u32::from_str_radix(&value, 16)
                            .ok()
                            .map(|i| char::try_from(i).ok())
                            .flatten()
                        {
                            unicode = Some(c);
                        }
                    }
                    b"name" => {
                        name.push_str(&value);
                    }
                    b"category" => {
                        category = Some(value.parse().unwrap());
                    }
                    b"subCategory" => sub_category = Some(value.parse().unwrap()),
                    b"case" => case = Some(value.parse().unwrap()),
                    b"script" => script = Some(value.parse().unwrap()),
                    b"description" => description.push_str(&value),
                    b"production" => production_name = Some(value.into()),
                    b"altNames" => {
                        alterative_names.extend(value.split(',').map(|s| s.trim().into()))
                    }
                    _ => (),
                }
            }
        }

        if name.is_empty() || category.is_none() || description.is_empty() {
            return Err("Invalid XML glyph record.");
        }

        Ok(XmlRecord {
            unicode,
            name,
            category: category.unwrap(),
            sub_category,
            case,
            script,
            description,
            production_name,
            alterative_names,
        })
    }
}
