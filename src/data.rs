use super::{Category, Record, Script, SubCategory};

/// Raw GlyphData.xml records. MIT-Licensed, see https://github.com/schriftgestalt/GlyphsInfo.
pub static GLYPHSINFO_RAW: [(&str, Record); 1] = [(
    "lekattakpramMuoy-khmer",
    Record {
        unicode: Some('\u{17f6}'),
        category: Category::Number,
        sub_category: Some(SubCategory::DecimalDigit),
        script: Some(Script::Khmer),
        description: "KHMER SYMBOL LEK ATTAK PRAM-MUOY",
        production_name: Some("uni17F6"),
        alterative_names: &["pramMuoyLekattak-khmer"],
    },
)];
