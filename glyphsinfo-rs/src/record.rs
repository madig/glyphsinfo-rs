use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub unicode: Option<char>,
    pub category: Category,
    pub sub_category: Option<SubCategory>,
    pub case: Option<Case>,
    pub script: Option<Script>,
    pub direction: Option<Direction>,
    pub description: Option<String>,
    pub production_name: Option<String>,
    pub alterative_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum Case {
    Lower,
    Minor,
    SmallCaps,
    Upper,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom, // XXX: The docs say something about VerticalLTR and VerticalRTL instead
    BottomToTop, // XXX: The docs say something about VerticalLTR and VerticalRTL instead
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Letter" => Ok(Category::Letter),
            "Mark" => Ok(Category::Mark),
            "Number" => Ok(Category::Number),
            "Other" => Ok(Category::Other),
            "Punctuation" => Ok(Category::Punctuation),
            "Separator" => Ok(Category::Separator),
            "Symbol" => Ok(Category::Symbol),
            _ => Err("Unknown category"),
        }
    }
}

impl FromStr for SubCategory {
    type Err = &'static str;

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
            _ => Err("Unknown subcategory"),
        }
    }
}

impl FromStr for Case {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lower" => Ok(Case::Lower),
            "minor" => Ok(Case::Minor),
            "smallCaps" => Ok(Case::SmallCaps),
            "upper" => Ok(Case::Upper),
            _ => Err("Unknown case"),
        }
    }
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LTR" => Ok(Direction::LeftToRight),
            "RTL" => Ok(Direction::RightToLeft),
            "TTB" => Ok(Direction::TopToBottom),
            "BTT" => Ok(Direction::BottomToTop),
            _ => Err("Unknown direction"),
        }
    }
}

impl FromStr for Script {
    type Err = &'static str;

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
            _ => Err("Unknown script"),
        }
    }
}
