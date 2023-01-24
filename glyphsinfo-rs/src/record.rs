use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubCategory {
    Arrow,
    Compatibility,
    Currency,
    Dash,
    #[serde(rename = "Decimal Digit", alias = "Decimal digit")]
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
    #[serde(rename = "Spacing Combining")]
    SpacingCombining,
    Superscript,
    Syllable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Case {
    Lower,
    Minor,
    SmallCaps,
    Upper,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
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
    #[serde(rename = "Black Letter")]
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
    #[serde(rename = "nyiakeng puachue hmong")]
    NyiakengPuachueHmong,
    Oriya,
    Osage,
    Osmanya,
    #[serde(rename = "pahawh hmong")]
    PahawhHmong,
    #[serde(rename = "phaistosDisc")]
    PhaistosDisc,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum Direction {
    #[serde(rename = "LTR")]
    LeftToRight,
    #[serde(rename = "RTL")]
    RightToLeft,
}
