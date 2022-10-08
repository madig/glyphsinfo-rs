use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod data;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub unicode: Option<char>,
    pub category: &'static str,
    pub sub_category: Option<&'static str>,
    pub case: Option<&'static str>,
    pub script: Option<&'static str>,
    pub description: &'static str,
    pub production_name: Option<&'static str>,
    pub alterative_names: &'static [&'static str],
}

lazy_static! {
    static ref BY_NAME: HashMap<&'static str, &'static Record> = {
        HashMap::from_iter(
            data::GLYPHSINFO_RAW
                .iter()
                .map(|(name, record)| (*name, record)),
        )
    };
    static ref BY_PRODUCTION_NAME: HashMap<&'static str, &'static Record> = {
        HashMap::from_iter(
            data::GLYPHSINFO_RAW
                .iter()
                .filter_map(|(_, record)| record.production_name.map(|name| (name, record)))
                .map(|(name, record)| (name, record)),
        )
    };
    static ref BY_ALTERNATIVE_NAME: HashMap<&'static str, &'static Record> = {
        HashMap::from_iter(
            data::GLYPHSINFO_RAW
                .iter()
                .flat_map(|(_, record)| record.alterative_names.iter().zip(std::iter::once(record)))
                .map(|(name, record)| (*name, record)),
        )
    };
    static ref BY_UNICODE: HashMap<char, &'static Record> = {
        HashMap::from_iter(
            data::GLYPHSINFO_RAW
                .iter()
                .filter_map(|(_, record)| record.unicode.map(|uv| (uv, record))),
        )
    };
}

pub fn record_for_name(name: &str) -> Option<&Record> {
    BY_NAME.get(name).copied()
}

pub fn record_for_production_name(name: &str) -> Option<&Record> {
    BY_PRODUCTION_NAME.get(name).copied()
}

pub fn record_for_alternative_name(name: &str) -> Option<&Record> {
    BY_ALTERNATIVE_NAME.get(name).copied()
}

pub fn record_for_unicode(unicode_value: char) -> Option<&'static Record> {
    BY_UNICODE.get(&unicode_value).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_name() {
        let expected = Record {
            unicode: Some('\u{17F6}'),
            category: "Number",
            sub_category: Some("Decimal Digit"),
            case: None,
            script: Some("khmer"),
            description: "KHMER SYMBOL LEK ATTAK PRAM-MUOY",
            production_name: Some("uni17F6"),
            alterative_names: &["pramMuoyLekattak-khmer"],
        };

        let record = record_for_name("lekattakpramMuoy-khmer").unwrap();
        assert_eq!(*record, expected);
        let record = record_for_production_name("uni17F6").unwrap();
        assert_eq!(*record, expected);
        let record = record_for_alternative_name("pramMuoyLekattak-khmer").unwrap();
        assert_eq!(*record, expected);
        let record = record_for_unicode('\u{17F6}').unwrap();
        assert_eq!(*record, expected);
    }
}
