use std::collections::HashMap;

use lazy_static::lazy_static;
use quick_xml::{events::Event, reader::Reader};

use record::*;
use xml::XmlRecord;

pub mod record;
pub mod xml;

lazy_static! {
    static ref GLYPH_DATA: GlyphData = GlyphData::default();
}

static XML_BASE: &[u8; 4759770] = include_bytes!("../../GlyphsInfo/GlyphData.xml");
static XML_IDEOGRAPHS: &[u8; 2219401] = include_bytes!("../../GlyphsInfo/GlyphData_Ideographs.xml");

#[derive(Debug)]
pub struct GlyphData {
    records: Vec<Record>,
    by_name: HashMap<String, usize>,
    by_production_name: HashMap<String, usize>,
    by_alternative_name: HashMap<String, usize>,
    by_unicode: HashMap<char, usize>,
}

impl Default for GlyphData {
    fn default() -> Self {
        Self::from_xmls(&[&XML_BASE[..], &XML_IDEOGRAPHS[..]])
    }
}

impl GlyphData {
    pub fn from_xmls(xmls: &[&[u8]]) -> Self {
        let mut records = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_production_name = HashMap::new();
        let mut by_alternative_name = HashMap::new();
        let mut by_unicode = HashMap::new();

        for xml_bytes in xmls {
            let mut reader = Reader::from_reader(*xml_bytes);
            reader.trim_text(true);
            loop {
                match reader.read_event() {
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    Ok(Event::Empty(e)) => match e.name().as_ref() {
                        b"glyph" => {
                            let raw_record: Option<xml::XmlRecord> = e.try_into().ok();
                            if let Some(raw_record) = raw_record {
                                let (name, record) = split_xml_record(raw_record);

                                let record_index = records.len();
                                by_name.insert(name, record_index);
                                if let Some(production_name) = &record.production_name {
                                    by_production_name.insert(production_name.into(), record_index);
                                }
                                for alternative_name in record.alterative_names.iter() {
                                    by_alternative_name
                                        .insert(alternative_name.into(), record_index);
                                }
                                if let Some(unicode) = record.unicode {
                                    by_unicode.insert(unicode, record_index);
                                }
                                records.push(record);
                            };
                        }
                        _ => (),
                    },
                    _ => (),
                }
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

    pub fn from_postcard(content: &[u8]) -> Self {
        let raw_records: Vec<XmlRecord> = postcard::from_bytes(content).unwrap();
        let mut records = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_production_name = HashMap::new();
        let mut by_alternative_name = HashMap::new();
        let mut by_unicode = HashMap::new();

        for (record_index, raw_record) in raw_records.into_iter().enumerate() {
            let (name, record) = split_xml_record(raw_record);
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

        Self {
            records,
            by_name,
            by_production_name,
            by_alternative_name,
            by_unicode,
        }
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn record_for_name(&self, name: &str) -> Option<&Record> {
        self.by_name.get(name).and_then(|i| Some(&self.records[*i]))
    }

    pub fn record_for_production_name(&self, name: &str) -> Option<&Record> {
        self.by_production_name
            .get(name)
            .and_then(|i| Some(&self.records[*i]))
    }

    pub fn record_for_alternative_name(&self, name: &str) -> Option<&Record> {
        self.by_alternative_name
            .get(name)
            .and_then(|i| Some(&self.records[*i]))
    }

    pub fn record_for_unicode(&self, unicode_value: char) -> Option<&Record> {
        self.by_unicode
            .get(&unicode_value)
            .and_then(|i| Some(&self.records[*i]))
    }
}

fn split_xml_record(raw_record: XmlRecord) -> (String, Record) {
    (
        raw_record.name,
        Record {
            unicode: raw_record.unicode,
            category: raw_record.category,
            sub_category: raw_record.sub_category,
            case: raw_record.case,
            script: raw_record.script,
            description: raw_record.description,
            production_name: raw_record.production_name,
            alterative_names: raw_record.alterative_names,
        },
    )
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
            description: "KHMER SYMBOL LEK ATTAK PRAM-MUOY".into(),
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
