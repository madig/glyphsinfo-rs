use std::collections::HashMap;

use lazy_static::lazy_static;

use record::*;
use serde::{Deserialize, Serialize};
use xml::XmlGlyphData;

pub mod record;
pub mod xml;

// TODO: Try https://crates.io/crates/databake?
lazy_static! {
    static ref GLYPH_DATA: GlyphData = GlyphData::default();
}

static GLYPHDATA_POSTCARD_DATA: &[u8; 3494378] = include_bytes!("data/glyphdata.postcard");

#[derive(Debug, Deserialize, Serialize)]
pub struct GlyphData {
    records: Vec<Record>,
    by_name: HashMap<String, usize>,
    by_production_name: HashMap<String, usize>,
    by_alternative_name: HashMap<String, usize>,
    by_unicode: HashMap<char, usize>,
}

impl Default for GlyphData {
    fn default() -> Self {
        Self::from_postcard(GLYPHDATA_POSTCARD_DATA)
    }
}

impl GlyphData {
    pub fn from_postcard(content: &[u8]) -> Self {
        postcard::from_bytes(content).unwrap()
    }

    pub fn from_xml(xml_strs: &[&str]) -> Self {
        let mut records = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_production_name = HashMap::new();
        let mut by_alternative_name = HashMap::new();
        let mut by_unicode = HashMap::new();

        for xml_str in xml_strs {
            let glyph_data: XmlGlyphData = quick_xml::de::from_str(*xml_str).unwrap();
            for (record_index, xml_record) in glyph_data.glyph.into_iter().enumerate() {
                let (name, record) = xml_record.into_record();
                if by_name.contains_key(&name) {
                    panic!("Duplicate record for {name} found.")
                };
                by_name.insert(name, record_index);
                if let Some(production_name) = &record.production_name {
                    by_production_name.insert(production_name.into(), record_index);
                }
                for alternative_name in record.alterative_names.iter() {
                    by_alternative_name.insert(alternative_name.into(), record_index);
                }
                if let Some(unicode) = record.unicode {
                    by_unicode.insert(unicode, record_index);
                }
                records.push(record);
            }
        }

        Self {
            records,
            by_name,
            by_production_name,
            by_alternative_name,
            by_unicode,
        }
    }

    pub fn record_for_name(&self, name: &str) -> Option<&Record> {
        self.by_name.get(name).map(|i| &self.records[*i])
    }

    pub fn record_for_production_name(&self, name: &str) -> Option<&Record> {
        self.by_production_name.get(name).map(|i| &self.records[*i])
    }

    pub fn record_for_alternative_name(&self, name: &str) -> Option<&Record> {
        self.by_alternative_name
            .get(name)
            .map(|i| &self.records[*i])
    }

    pub fn record_for_unicode(&self, unicode_value: char) -> Option<&Record> {
        self.by_unicode
            .get(&unicode_value)
            .map(|i| &self.records[*i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_name() {
        let expected = Record {
            unicode: Some('\u{17F6}'),
            category: Category::Number,
            sub_category: Some(SubCategory::DecimalDigit),
            case: None,
            script: Some(Script::Khmer),
            direction: None,
            description: Some("KHMER SYMBOL LEK ATTAK PRAM-MUOY".into()),
            production_name: Some("uni17F6".into()),
            alterative_names: vec!["pramMuoyLekattak-khmer".into()],
        };

        let record = GLYPH_DATA
            .record_for_name("lekattakpramMuoy-khmer")
            .unwrap();
        assert_eq!(*record, expected);
        let record = GLYPH_DATA.record_for_production_name("uni17F6").unwrap();
        assert_eq!(*record, expected);
        let record = GLYPH_DATA
            .record_for_alternative_name("pramMuoyLekattak-khmer")
            .unwrap();
        assert_eq!(*record, expected);
        let record = GLYPH_DATA.record_for_unicode('\u{17F6}').unwrap();
        assert_eq!(*record, expected);
    }
}
