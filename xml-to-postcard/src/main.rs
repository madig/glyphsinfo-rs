use std::path::PathBuf;

use postcard::to_allocvec;
use quick_xml::{events::Event, reader::Reader};

use glyphsinfo_rs::*;

fn main() {
    let args: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if args.len() < 2 {
        eprintln!("Must have one or more XML file paths to read and at least one output path.");
        std::process::exit(1);
    }

    let xml_files = &args[..args.len() - 1];
    let output_file = args.last().unwrap();

    let mut raw_records = Vec::new();
    for xml_file in xml_files {
        let mut reader = Reader::from_file(xml_file).unwrap();
        let mut buffer = Vec::new();
        reader.trim_text(true);
        loop {
            match reader.read_event_into(&mut buffer) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Empty(e)) => match e.name().as_ref() {
                    b"glyph" => {
                        let raw_record: xml::XmlRecord = e.try_into().unwrap_or_else(|e| {
                            panic!(
                                "Could not parse glyph record in file {} at position {}: {:?}",
                                xml_file.display(),
                                reader.buffer_position(),
                                e
                            )
                        });
                        raw_records.push(raw_record);
                    }
                    _ => panic!(
                        "Unexpected element at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ),
                },
                _ => (),
            }
        }
    }

    let bytes = to_allocvec(&raw_records).unwrap();
    std::fs::write(output_file, bytes).unwrap();
}
