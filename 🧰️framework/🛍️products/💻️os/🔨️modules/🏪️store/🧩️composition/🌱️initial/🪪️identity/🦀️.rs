//! 🌱️ Pure bounded initial-child identity; this digest grants no creation or publication authority.

use crate::os_directory::schema::DocumentScope;
use crate::os_io::{ArtifactDialect, ArtifactRef};

const INITIAL_CHILD_DOMAIN: &[u8] = b"semio.initial-child.v1\0";
const INITIAL_CHILD_FIELD_BYTES: usize = 256;
const INITIAL_CHILD_ORDINAL_LIMIT: u32 = 64;

/// 🪪️ Hashes agreeing typed scope/parent coordinates, slot, child dialect and ordinal without mutable content.
pub(super) fn initial_child_identity(scope: &DocumentScope, parent: &ArtifactRef, slot: &str, child: &ArtifactDialect, ordinal: u32) -> Result<String, &'static str> {
    if scope.document_id.len() > INITIAL_CHILD_FIELD_BYTES || parent.artifact_id.len() > INITIAL_CHILD_FIELD_BYTES {
        return Err("initial-child-coordinate");
    }
    if scope.document_id != parent.artifact_id {
        return Err("initial-child-document");
    }
    let fields = [
        scope.space_id.as_str(),
        parent.artifact_id.as_str(),
        parent.dialect.artifact_kind.as_str(),
        parent.dialect.standard.as_str(),
        parent.dialect.subset.as_str(),
        slot,
        child.artifact_kind.as_str(),
        child.standard.as_str(),
        child.subset.as_str(),
    ];
    if ordinal >= INITIAL_CHILD_ORDINAL_LIMIT {
        return Err("initial-child-ordinal");
    }
    if fields.iter().any(|field| field.is_empty() || field.len() > INITIAL_CHILD_FIELD_BYTES || field.chars().any(|value| matches!(value, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}'))) {
        return Err("initial-child-coordinate");
    }
    let mut hash = semio_framework_hash::Hasher::new();
    hash.update(INITIAL_CHILD_DOMAIN);
    for field in fields {
        hash.update(&(field.len() as u32).to_le_bytes());
        hash.update(field.as_bytes());
    }
    hash.update(&ordinal.to_le_bytes());
    Ok(format!("initial-child-{}", hash.finalize().to_hex()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coordinate(fields: [&str; 9]) -> (DocumentScope, ArtifactRef, ArtifactDialect) {
        let parent = ArtifactRef { artifact_id: fields[1].into(), dialect: ArtifactDialect { artifact_kind: fields[2].into(), standard: fields[3].into(), subset: fields[4].into() } };
        let child = ArtifactDialect { artifact_kind: fields[6].into(), standard: fields[7].into(), subset: fields[8].into() };
        (DocumentScope::new(fields[0], fields[1]), parent, child)
    }

    fn derive(fields: [&str; 9], ordinal: u32) -> Result<String, &'static str> {
        let (scope, parent, child) = coordinate(fields);
        initial_child_identity(&scope, &parent, fields[5], &child, ordinal)
    }

    fn oracle(fields: [&str; 9], ordinal: u32) -> (String, usize) {
        let mut wire = Vec::from(b"semio.initial-child.v1\0");
        for field in fields {
            wire.extend_from_slice(&(field.len() as u32).to_le_bytes());
            wire.extend_from_slice(field.as_bytes());
        }
        wire.extend_from_slice(&ordinal.to_le_bytes());
        (format!("initial-child-{}", blake3::hash(&wire).to_hex()), wire.len())
    }

    #[test]
    fn initial_child_identity_matches_neutral_coordinates_and_blake3() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
        assert_eq!(fixture["authority"], "none");
        assert_eq!(fixture["maximumFieldBytes"], INITIAL_CHILD_FIELD_BYTES);
        assert_eq!(fixture["maximumChildren"], INITIAL_CHILD_ORDINAL_LIMIT);
        let rows = fixture["cases"].as_array().unwrap();
        let first_space = rows.iter().find(|row| row["id"] == "document-one").unwrap();
        let second_space = rows.iter().find(|row| row["id"] == "different-space-same-document").unwrap();
        assert_eq!(first_space["values"].as_array().unwrap()[1..], second_space["values"].as_array().unwrap()[1..]);
        assert_ne!(first_space["values"][0], second_space["values"][0]);
        assert_ne!(first_space["expectedId"], second_space["expectedId"]);
        for row in rows {
            let fields = std::array::from_fn(|index| row["values"][index].as_str().unwrap());
            let ordinal = row["ordinal"].as_u64().unwrap() as u32;
            let actual = derive(fields, ordinal).unwrap();
            let independent = oracle(fields, ordinal);
            assert_eq!(actual, row["expectedId"].as_str().unwrap());
            assert_eq!(actual, independent.0);
            assert_eq!(independent.1, row["wireBytes"].as_u64().unwrap() as usize);
            for index in 0..9 {
                let altered = format!("{}-other", fields[index]);
                let mut changed = fields;
                changed[index] = &altered;
                let changed_id = derive(changed, ordinal).unwrap();
                assert_ne!(changed_id, actual);
                assert_eq!(changed_id, oracle(changed, ordinal).0);
            }
        }
        let base = std::array::from_fn(|index| rows[0]["values"][index].as_str().unwrap());
        for row in fixture["scopeAgreementCases"].as_array().unwrap() {
            let (mut scope, parent, child) = coordinate(base);
            scope.document_id = row["scopeDocumentId"].as_str().unwrap().into();
            let result = initial_child_identity(&scope, &parent, base[5], &child, 0);
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap());
            if result.is_err() {
                assert_eq!(result, Err("initial-child-document"));
            }
        }
        let mut denied = 0;
        for index in 0..9 {
            for rejected in fixture["rejectedFields"].as_array().unwrap() {
                let value = rejected["unit"].as_str().unwrap().repeat(rejected["repeat"].as_u64().unwrap() as usize);
                let mut fields = base;
                fields[index] = &value;
                assert_eq!(derive(fields, 0), Err("initial-child-coordinate"));
                denied += 1;
            }
        }
        for ordinal in [INITIAL_CHILD_ORDINAL_LIMIT, u32::MAX] {
            assert_eq!(derive(base, ordinal), Err("initial-child-ordinal"));
            denied += 1;
        }
        for value in ["x".repeat(256), "🌊".repeat(64)] {
            let fields = [value.as_str(); 9];
            assert_eq!(derive(fields, 63).unwrap(), oracle(fields, 63).0);
        }
        eprintln!("[DEBUG] initial-child identity vectors={} coordinate substitutions={} denials={} scope-agreement=3 full-width frames=2 independent=blake3 authority=none", rows.len(), rows.len() * 9, denied);
    }
}
