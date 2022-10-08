use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub unicode: Option<char>,
    pub category: Category,
    pub sub_category: Option<SubCategory>,
    pub case: Option<Case>,
    pub script: Option<Script>,
    pub description: String,
    pub production_name: Option<String>,
    pub alterative_names: Vec<String>,
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
    Emoji,
    Enclosing,
    Format,
    Fraction,
    Geometry,
    Halfform,
    Jamo,
    Letter,
    Ligature,
    Math,
    Matra,
    Modifier,
    Nonspacing,
    Number,
    Parenthesis,
    Quote,
    Radical,
    Small,
    Space,
    Spacing,
    SpacingCombining,
    Superscript,
    Syllable,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Case {
    Lower,
    Minor,
    SmallCaps,
    Upper,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Script {
    Adlam,
    Alchemical,
    Arabic,
    Armenian,
    Avestan,
    Balinese,
    Bamum,
    Batak,
    Bengali,
    BlackLetter,
    Bopomofo,
    Braille,
    Buginese,
    Canadian,
    Chakma,
    Cham,
    Cherokee,
    Chorasmian,
    Cyrillic,
    Dentistry,
    Deseret,
    Devanagari,
    Divesakuru,
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
    Khojki,
    Lao,
    Latin,
    Lepcha,
    Lue,
    Mahjong,
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
    Runic,
    Samaritan,
    Shavian,
    Sinhala,
    Syriac,
    Tamil,
    Telugu,
    Thaana,
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
        let mut records = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_production_name = HashMap::new();
        let mut by_alternative_name = HashMap::new();
        let mut by_unicode = HashMap::new();

        for xml_bytes in [&XML_BASE[..], &XML_IDEOGRAPHS[..]] {
            let mut reader = Reader::from_reader(xml_bytes);
            reader.trim_text(true);
            loop {
                match reader.read_event() {
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    Ok(Event::Empty(e)) => match e.name().as_ref() {
                        b"glyph" => {
                            if let Some((name, record)) = record_from_element(&e) {
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

fn record_from_element<'a>(element: &quick_xml::events::BytesStart) -> Option<(String, Record)> {
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
                b"altNames" => alterative_names.extend(value.split(',').map(|s| s.trim().into())),
                _ => (),
            }
        }
    }

    Some((
        name,
        Record {
            unicode,
            category: category.unwrap(),
            sub_category,
            case,
            script,
            description,
            production_name,
            alterative_names,
        },
    ))
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Letter" => Ok(Category::Letter),
            "Mark" => Ok(Category::Mark),
            "Number" => Ok(Category::Number),
            "Other" => Ok(Category::Other),
            "Punctuation" => Ok(Category::Punctuation),
            "Separator" => Ok(Category::Separator),
            "Symbol" => Ok(Category::Symbol),
            _ => Err("Unknown category".into()),
        }
    }
}

impl FromStr for SubCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Arrow" => Ok(SubCategory::Arrow),
            "Compatibility" => Ok(SubCategory::Compatibility),
            "Currency" => Ok(SubCategory::Currency),
            "Dash" => Ok(SubCategory::Dash),
            "Decimal Digit" | "Decimal digit" => Ok(SubCategory::DecimalDigit),
            "Emoji" => Ok(SubCategory::Emoji),
            "Enclosing" => Ok(SubCategory::Enclosing),
            "Format" => Ok(SubCategory::Format),
            "Fraction" => Ok(SubCategory::Fraction),
            "Geometry" => Ok(SubCategory::Geometry),
            "Halfform" => Ok(SubCategory::Halfform),
            "Jamo" => Ok(SubCategory::Jamo),
            "Letter" => Ok(SubCategory::Letter),
            "Ligature" => Ok(SubCategory::Ligature),
            "Math" => Ok(SubCategory::Math),
            "Matra" => Ok(SubCategory::Matra),
            "Modifier" => Ok(SubCategory::Modifier),
            "Nonspacing" => Ok(SubCategory::Nonspacing),
            "Number" => Ok(SubCategory::Number),
            "Parenthesis" => Ok(SubCategory::Parenthesis),
            "Quote" => Ok(SubCategory::Quote),
            "Radical" => Ok(SubCategory::Radical),
            "Small" => Ok(SubCategory::Small),
            "Space" => Ok(SubCategory::Space),
            "Spacing Combining" => Ok(SubCategory::SpacingCombining),
            "Spacing" => Ok(SubCategory::Spacing),
            "Superscript" => Ok(SubCategory::Superscript),
            "Syllable" => Ok(SubCategory::Syllable),
            _ => Err("Unknown subcategory".into()),
        }
    }
}

impl FromStr for Case {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lower" => Ok(Case::Lower),
            "minor" => Ok(Case::Minor),
            "smallCaps" => Ok(Case::SmallCaps),
            "upper" => Ok(Case::Upper),
            _ => Err("Unknown case".into()),
        }
    }
}

impl FromStr for Script {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adlam" => Ok(Script::Adlam),
            "alchemical" => Ok(Script::Alchemical),
            "arabic" => Ok(Script::Arabic),
            "armenian" => Ok(Script::Armenian),
            "avestan" => Ok(Script::Avestan),
            "balinese" => Ok(Script::Balinese),
            "bamum" => Ok(Script::Bamum),
            "batak" => Ok(Script::Batak),
            "bengali" => Ok(Script::Bengali),
            "Black Letter" => Ok(Script::BlackLetter),
            "bopomofo" => Ok(Script::Bopomofo),
            "braille" => Ok(Script::Braille),
            "buginese" => Ok(Script::Buginese),
            "canadian" => Ok(Script::Canadian),
            "chakma" => Ok(Script::Chakma),
            "cham" => Ok(Script::Cham),
            "cherokee" => Ok(Script::Cherokee),
            "chorasmian" => Ok(Script::Chorasmian),
            "cyrillic" => Ok(Script::Cyrillic),
            "dentistry" => Ok(Script::Dentistry),
            "deseret" => Ok(Script::Deseret),
            "devanagari" => Ok(Script::Devanagari),
            "divesakuru" => Ok(Script::Divesakuru),
            "elbasan" => Ok(Script::Elbasan),
            "elymaic" => Ok(Script::Elymaic),
            "ethiopic" => Ok(Script::Ethiopic),
            "georgian" => Ok(Script::Georgian),
            "glagolitic" => Ok(Script::Glagolitic),
            "gothic" => Ok(Script::Gothic),
            "greek" => Ok(Script::Greek),
            "gujarati" => Ok(Script::Gujarati),
            "gurmukhi" => Ok(Script::Gurmukhi),
            "han" => Ok(Script::Han),
            "hangul" => Ok(Script::Hangul),
            "hebrew" => Ok(Script::Hebrew),
            "javanese" => Ok(Script::Javanese),
            "kana" => Ok(Script::Kana),
            "kannada" => Ok(Script::Kannada),
            "kayahli" => Ok(Script::Kayahli),
            "khmer" => Ok(Script::Khmer),
            "khojki" => Ok(Script::Khojki),
            "lao" => Ok(Script::Lao),
            "latin" => Ok(Script::Latin),
            "lepcha" => Ok(Script::Lepcha),
            "lue" => Ok(Script::Lue),
            "mahjong" => Ok(Script::Mahjong),
            "malayalam" => Ok(Script::Malayalam),
            "mandaic" => Ok(Script::Mandaic),
            "math" => Ok(Script::Math),
            "mongolian" => Ok(Script::Mongolian),
            "musical" => Ok(Script::Musical),
            "myanmar" => Ok(Script::Myanmar),
            "nko" => Ok(Script::Nko),
            "nyiakeng puachue hmong" => Ok(Script::NyiakengPuachueHmong),
            "oriya" => Ok(Script::Oriya),
            "osage" => Ok(Script::Osage),
            "osmanya" => Ok(Script::Osmanya),
            "pahawh hmong" => Ok(Script::PahawhHmong),
            "phaistosDisc" => Ok(Script::Phaistosdisc),
            "rovas" => Ok(Script::Rovas),
            "runic" => Ok(Script::Runic),
            "samaritan" => Ok(Script::Samaritan),
            "shavian" => Ok(Script::Shavian),
            "sinhala" => Ok(Script::Sinhala),
            "syriac" => Ok(Script::Syriac),
            "tamil" => Ok(Script::Tamil),
            "telugu" => Ok(Script::Telugu),
            "thaana" => Ok(Script::Thaana),
            "thai" => Ok(Script::Thai),
            "tham" => Ok(Script::Tham),
            "tibet" => Ok(Script::Tibet),
            "tifinagh" => Ok(Script::Tifinagh),
            "yi" => Ok(Script::Yi),
            _ => Err("Unknown script".into()),
        }
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
