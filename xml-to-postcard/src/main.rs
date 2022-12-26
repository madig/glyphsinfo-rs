use std::path::PathBuf;

use postcard::to_allocvec;

use glyphsinfo_rs::GlyphData;

fn main() {
    let args: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if args.len() < 2 {
        eprintln!("Must have one or more XML file paths to read and at least one output path.");
        std::process::exit(1);
    }

    let xml_files = &args[..args.len() - 1];
    let output_file = args.last().unwrap();

    let xml_contents: Vec<String> = xml_files
        .into_iter()
        .map(|xml_file| std::fs::read_to_string(xml_file).unwrap())
        .collect();

    let xmls: Vec<&str> = xml_contents.iter().map(|s| s.as_ref()).collect();
    let glyph_data = GlyphData::from_xml(&xmls);

    let bytes = to_allocvec(&glyph_data).unwrap();
    std::fs::write(output_file, bytes).unwrap();
}
