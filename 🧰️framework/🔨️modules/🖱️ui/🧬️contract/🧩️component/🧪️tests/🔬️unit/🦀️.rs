//! 🧪️ Canonical component serde remains owned with its actual algebra.
use crate::*;
#[test]
fn canonical_input_publication_revision_preserves_u64_max(){let original=UiPublicationRevision(u64::MAX);let wire=serde_json::to_string(&original).unwrap();assert_eq!(wire,"18446744073709551615");let back:UiPublicationRevision=serde_json::from_str(&wire).unwrap();assert_eq!(original,back);}
#[test]
fn canonical_component_variant_round_trips(){let wire=r#"{"type":"button","icon":"save","label":"Speichern"}"#;let value:Component=serde_json::from_str(wire).unwrap();assert_eq!(serde_json::to_string(&value).unwrap(),wire);}
