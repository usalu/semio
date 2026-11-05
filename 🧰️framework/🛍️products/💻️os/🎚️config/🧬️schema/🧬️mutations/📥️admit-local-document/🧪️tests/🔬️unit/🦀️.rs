use semio_framework_os_kernel::ColdMutation;
use semio_framework_os_kernel::ColdMutationDiff;
use semio_framework_os_kernel::ColdDiffAlgebra;
use crate::io::text::snapshot::{decode_local_catalog_json, encode_local_catalog_json};
use super::*;
use super::super::{apply_local_catalog_config_mutation, apply_local_catalog_config_mutation_reporting};
use protocol::{Mutation};

fn document(document_id: &str, name: &str) -> LocalDocument {
    LocalDocument { document_id: document_id.to_string(), schema: "s.space".to_string(), name: name.to_string(), storage: LocalDocumentStorage::Folder, target: format!("/data/os/local-documents/{document_id}"), admitted_at_ms: 7, surface: None }
}

#[test]
fn admit_serializes_like_the_typescript_projection() {
    let json = serde_json::to_value(admit_local_document(document("studio-a", "A"))).expect("admit encodes");
    assert_eq!(json["mutation"], "admitLocalDocument");
    assert_eq!(json["documentId"], "studio-a");
    assert_eq!(json["storage"], "folder");
    assert_eq!(json["admittedAtMs"], 7);
}

#[test]
fn admitting_a_new_document_inverts_to_its_retirement() {
    let base = LocalCatalog::default();
    assert_eq!(admit_local_document(document("studio-a", "A")).inverse(&base).expect("valid retained mutation inverse fixture"), vec![super::super::retire_local_document("studio-a")]);
}

#[test]
fn readmitting_a_listed_document_replaces_it_in_place_and_inverts_to_the_prior_entry() {
    let base = LocalCatalog { documents: vec![document("studio-a", "A"), document("studio-b", "B")] };
    let renamed = admit_local_document(document("studio-a", "A renamed"));
    let mut snapshot = base.clone();
    apply_local_catalog_config_mutation(&mut snapshot, &renamed).expect("admit applies");
    assert_eq!(snapshot.documents.iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), vec!["A renamed", "B"], "one entry per id, ordered by id");
    assert_eq!(renamed.inverse(&base).expect("valid retained mutation inverse fixture"), vec![admit_local_document(document("studio-a", "A"))]);
}

#[test]
fn admitting_an_identical_entry_is_a_warned_no_op() {
    let base = LocalCatalog { documents: vec![document("studio-a", "A")] };
    let mut snapshot = base.clone();
    let raised = apply_local_catalog_config_mutation_reporting(&mut snapshot, &admit_local_document(document("studio-a", "A")));
    assert_eq!(snapshot, base);
    assert_eq!(raised.len(), 1);
    assert_eq!(raised[0].0, "mutation.no-op");
}

#[test]
fn the_catalog_round_trips_through_its_json_projection() {
    let catalog = LocalCatalog { documents: vec![document("studio-a", "A")] };
    assert_eq!(decode_local_catalog_json(&encode_local_catalog_json(&catalog)).expect("catalog decodes"), catalog);
    assert_eq!(decode_local_catalog_json("{\"documents\":[]}").expect("empty catalog decodes"), LocalCatalog::default());
}

#[test]
fn selected_surface_neutral_vectors_preserve_subset_through_admission_diff_inverse_and_codecs() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪟️surface/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let base: LocalCatalog = serde_json::from_value(row["before"].clone()).unwrap();
        let mutation: LocalCatalogConfigMutation = serde_json::from_value(row["mutation"].clone()).unwrap();
        let expected: LocalCatalog = serde_json::from_value(row["after"].clone()).unwrap();
        let mut actual = base.clone();
        apply_local_catalog_config_mutation(&mut actual, &mutation).unwrap();
        assert_eq!(actual, expected, "{}", row["name"]);
        assert_eq!(serde_json::to_value(&actual).unwrap(), row["after"]);
        assert_eq!(decode_local_catalog_json(&encode_local_catalog_json(&actual)).unwrap(), expected);
        let value = semio_framework_value::ToValue::to_value(&actual);
        assert_eq!(<LocalCatalog as semio_framework_value::FromValue>::from_value(value.clone()).unwrap(), expected);
        let mut progress = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(65_536, &mut progress);
        assert_eq!(<LocalCatalog as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).unwrap(), expected);
        for inverse in mutation.inverse(&base).unwrap().into_iter().rev() {
            apply_local_catalog_config_mutation(&mut actual, &inverse).unwrap();
        }
        assert_eq!(actual, base);
    }
    for surface in law["invalidSurfaces"].as_array().unwrap() {
        let mut invalid = law["cases"][0]["after"].clone();
        invalid["documents"][0]["surface"] = surface.clone();
        assert!(serde_json::from_value::<LocalCatalog>(invalid.clone()).is_err());
        let value = semio_framework_value::DslValue::from(&invalid);
        assert!(<LocalCatalog as semio_framework_value::FromValue>::from_value(value.clone()).is_err());
        let mut progress = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(65_536, &mut progress);
        assert!(<LocalCatalog as semio_framework_value::FromValue>::from_value_controlled(&value, &mut control).is_err());
    }
}
