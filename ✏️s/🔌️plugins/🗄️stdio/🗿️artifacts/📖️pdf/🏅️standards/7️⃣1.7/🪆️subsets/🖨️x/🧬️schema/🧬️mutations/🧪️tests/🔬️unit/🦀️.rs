use semio_framework_test_fixtures::fixture_path;

#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
use crate::standards::v1_7::subsets::x::schema::mutations::*;
use crate::standards::v1_7::subsets::x::io::{binary::mutations as binary, text::mutations as text};

#[test]
fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
    let mutation_root = fixture_path!("../../../../../../../../🖨️x/🧬️schema/🧬️mutations");
    let manifest = std::fs::read_to_string(mutation_root.join("../../🔮️oracles/🔣️.json")).expect("language-neutral oracle catalog");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    let derived: Vec<&str> = pdf_x_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
    assert_eq!(declared, derived);
    assert_eq!(text::TEXT_OPCODE_REGISTRY.iter().map(|(_, kind)| *kind).collect::<Vec<_>>(), derived);
    assert_eq!(binary::BINARY_TAG_REGISTRY.iter().map(|(_, _, tag)| *tag).collect::<Vec<_>>(), (0..derived.len() as u8).collect::<Vec<_>>());
}
