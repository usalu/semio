
use super::*;

#[test]
fn derived_catalog_matches_the_language_neutral_oracle_catalog() {
    let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🧬️mutations");
    let manifest = std::fs::read_to_string(mutation_root.join("../../🔣️oracle.json")).expect("language-neutral oracle catalog");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    let derived: Vec<&str> = pdf_a_mutation_kinds().iter().map(|descriptor| descriptor.kind).collect();
    assert_eq!(declared, derived);
}
