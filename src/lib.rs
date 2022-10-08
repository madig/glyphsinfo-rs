use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod data;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub unicode: Option<char>,
    pub category: Category,
    pub sub_category: Option<SubCategory>,
    // pub case: Option<&'static str>, // Nothing in the data?!
    pub script: Option<Script>,
    pub description: &'static str,
    pub production_name: Option<&'static str>,
    pub alterative_names: &'static [&'static str],
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Category {
    Letter,
    Mark,
    Number,
    Other,
    Punctuation,
    Separator,
    Symbol,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubCategory {
    Arrow,
    Compatibility,
    Currency,
    Dash,
    DecimalDigit,
    Enclosing,
    Format,
    Fraction,
    Geometry,
    Halfform,
    Jamo,
    Letter,
    Ligature,
    Lowercase,
    Math,
    Matra,
    Modifier,
    Nonspace,
    Nonspacing,
    Number,
    Parenthesis,
    Quote,
    Radical,
    Small,
    Space,
    SpacingCombining,
    Spacing,
    Superscript,
    Syllable,
    Uppercase,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Script {
    Adlam,
    Alchemical,
    Arabic,
    Armenian,
    Avestan,
    Bamum,
    Bengali,
    BlackLetter,
    Bopomofo,
    Braille,
    Buginese,
    Canadian,
    Chakma,
    Cham,
    Cherokee,
    Cyrillic,
    Dentistry,
    Deseret,
    Devanagari,
    Elbasan,
    Elymaic,
    Ethiopic,
    Georgian,
    Glagolitic,
    Gothic,
    Greek,
    Gujarati,
    Gurmukhi,
    Han,
    Hangul,
    Hebrew,
    Javanese,
    Kana,
    Kannada,
    Kayahli,
    Khmer,
    Lao,
    Latin,
    Lepcha,
    Malayalam,
    Mandaic,
    Math,
    Mongolian,
    Musical,
    Myanmar,
    Nko,
    NyiakengPuachueHmong,
    Oriya,
    Osage,
    Osmanya,
    PahawhHmong,
    Phaistosdisc,
    Rovas,
    Samaritan,
    Shavian,
    Sinhala,
    Syriac,
    Tamil,
    Telugu,
    Thai,
    Tham,
    Tibet,
    Tifinagh,
    Yi,
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
            category: Category::Number,
            sub_category: Some(SubCategory::DecimalDigit),
            script: Some(Script::Khmer),
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
