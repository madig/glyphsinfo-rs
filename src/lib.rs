use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod data;

lazy_static! {
    static ref GLYPH_DATA: GlyphData = GlyphData::default();
}

#[derive(Debug)]
pub struct GlyphData {
    by_name: HashMap<&'static str, &'static Record>,
    by_production_name: HashMap<&'static str, &'static Record>,
    by_alternative_name: HashMap<&'static str, &'static Record>,
    by_unicode: HashMap<char, &'static Record>,
}

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

impl Default for GlyphData {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphData {
    pub fn new() -> Self {
        let mut by_name = HashMap::new();
        let mut by_production_name = HashMap::new();
        let mut by_alternative_name = HashMap::new();
        let mut by_unicode = HashMap::new();

        for (name, record) in data::GLYPHSINFO_RAW.iter() {
            by_name.insert(*name, record);
            if let Some(production_name) = record.production_name {
                by_production_name.insert(production_name, record);
            }
            for alternative_name in record.alterative_names.iter() {
                by_alternative_name.insert(*alternative_name, record);
            }
            if let Some(unicode) = record.unicode {
                by_unicode.insert(unicode, record);
            }
        }

        Self {
            by_name,
            by_production_name,
            by_alternative_name,
            by_unicode,
        }
    }

    pub fn record_for_name(&self, name: &str) -> Option<&Record> {
        self.by_name.get(name).copied()
    }

    pub fn record_for_production_name(&self, name: &str) -> Option<&Record> {
        self.by_production_name.get(name).copied()
    }

    pub fn record_for_alternative_name(&self, name: &str) -> Option<&Record> {
        self.by_alternative_name.get(name).copied()
    }

    pub fn record_for_unicode(&self, unicode_value: char) -> Option<&'static Record> {
        self.by_unicode.get(&unicode_value).copied()
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
            script: Some(Script::Khmer),
            description: "KHMER SYMBOL LEK ATTAK PRAM-MUOY",
            production_name: Some("uni17F6"),
            alterative_names: &["pramMuoyLekattak-khmer"],
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
