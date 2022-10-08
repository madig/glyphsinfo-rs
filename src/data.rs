use super::Record;

/// Raw GlyphData.xml records. MIT-Licensed, see https://github.com/schriftgestalt/GlyphsInfo.
pub static GLYPHSINFO_RAW: [(&str, Record); 1] = [(
    "lekattakpramMuoy-khmer",
    Record {
        unicode: Some('\u{17f6}'),
        category: "Number",
        sub_category: Some("Decimal Digit"),
        case: None,
        script: Some("khmer"),
        description: "KHMER SYMBOL LEK ATTAK PRAM-MUOY",
        production_name: Some("uni17F6"),
        alterative_names: &["pramMuoyLekattak-khmer"],
    },
)];
