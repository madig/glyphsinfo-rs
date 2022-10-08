from __future__ import annotations

import argparse
import xml.etree.ElementTree
from dataclasses import dataclass
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument(
    "glyphdata_xml", nargs="+", type=Path, help="The GlyphData.xml files to use."
)
parser.add_argument("output", type=Path, help="The Rust source file to write.")
parsed_args = parser.parse_args()

TEMPLATE = """
use super::Record;

/// Raw GlyphData.xml records. MIT-Licensed, see https://github.com/schriftgestalt/GlyphsInfo.
pub static GLYPHSINFO_RAW: [(&str, Record); {records_length}] = [
    {records}
];
"""


@dataclass
class Record:
    unicode: int | None
    category: str
    sub_category: str | None
    case: str | None
    script: str | None
    description: str
    production_name: str | None
    alternative_names: list[str]

    def to_rust(self) -> str:
        if isinstance(self.unicode, int):
            unicode = f"Some('\\u{{{hex(self.unicode)[2:]}}}')"
        else:
            unicode = "None"
        category = f'"{self.category}"'
        if isinstance(self.sub_category, str):
            sub_category = f'Some("{self.sub_category}")'
        else:
            sub_category = "None"
        if isinstance(self.case, str):
            case = f'Some("{self.case}")'
        else:
            case = "None"
        if isinstance(self.script, str):
            script = f'Some("{self.script}")'
        else:
            script = "None"
        description = f'"{self.description}"'
        if isinstance(self.production_name, str):
            production_name = f'Some("{self.production_name}")'
        else:
            production_name = "None"
        alternative_names_quoted = [f'"{n}"' for n in self.alternative_names]
        alternative_names = f"&[{', '.join(alternative_names_quoted)}]"

        return f"""
    Record {{
        unicode: {unicode},
        category: {category},
        sub_category: {sub_category},
        case: {case},
        script: {script},
        description: {description},
        production_name: {production_name},
        alterative_names: {alternative_names},
    }},
        """


records: dict[str, Record] = {}
for glyphdata_file in parsed_args.glyphdata_xml:
    glyph_data = xml.etree.ElementTree.parse(glyphdata_file).getroot()
    for glyph in glyph_data:
        attribs = glyph.attrib
        name = attribs["name"]
        if name in records:
            raise Exception(f"Glyph {name} has more than one entry")
        ### TESTING ###
        if name != "lekattakpramMuoy-khmer":
            continue
        ### TESTING ###
        records[name] = Record(
            unicode=int(attribs["unicode"], 16) if "unicode" in attribs else None,
            category=attribs["category"],
            sub_category=attribs.get("subCategory"),
            case=attribs.get("case"),
            script=attribs.get("script"),
            description=attribs.get("description"),
            production_name=attribs.get("production"),
            alternative_names=(
                [n.strip() for n in attribs["altNames"].split(",")]
                if "altNames" in attribs
                else []
            ),
        )

records_rust: list[str] = []
for name, record in records.items():
    records_rust.append(f'("{name}", {record.to_rust()}),')

data_file = TEMPLATE.format(
    records_length=len(records), records="\n".join(records_rust)
)
parsed_args.output.write_text(data_file)
