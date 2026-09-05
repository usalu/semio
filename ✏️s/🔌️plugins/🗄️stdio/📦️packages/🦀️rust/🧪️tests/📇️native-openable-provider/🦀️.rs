use std::collections::BTreeSet;

use semio_s_plugin_stdio::registry::native_codec_factory_receipts;
use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};

#[test]
fn native_composition_and_validation_claims_are_disjoint_but_each_exclusive() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️registry/🧪️fixtures/🧾️claim-authority/🔣️.json"))).unwrap();
    assert_eq!(ArtifactIdentityNamespace::validated_dialect().as_str(), "validated-dialect");
    let rows = fixture["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 8);
    for row in rows {
        let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.stdio.xml").unwrap());
        for (index, claim) in row["claims"].as_array().unwrap().iter().enumerate() {
            let category = claim["category"].as_str().unwrap();
            let authority = if category == "codec" {
                let encoded = ArtifactIdentityClaim::codec_extension(claim["codecSchema"].as_str().unwrap(), claim["extension"].as_str().unwrap()).unwrap();
                assert_eq!(encoded.value(), claim["value"].as_str().unwrap());
                encoded
            } else {
                ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(claim["namespace"].as_str().unwrap()).unwrap(), claim["value"].as_str().unwrap()).unwrap()
            };
            let identity = if category == "codec" { format!("s.stdio.xml.standard.v1.codec.claim-{index}.v1") } else { format!("s.stdio.xml.{category}.claim-{index}.v1") };
            let capability = ArtifactCapability::new(ArtifactIdentity::parse(&identity).unwrap(), ArtifactCapabilityKind::parse(category).unwrap())
                .descriptor(b"claim authority fixture".to_vec()).unwrap()
                .claim(authority).unwrap();
            definition = definition.capability(capability).unwrap();
        }
        let code = definition.validate().err().map(|error| error.code().to_owned()).unwrap_or_else(|| "accepted".into());
        assert_eq!(code, row["code"].as_str().unwrap(), "{}", row["id"]);
    }
}

#[test]
fn artifact_owned_native_codec_receipts_form_one_complete_static_bijection() {
    let receipts = native_codec_factory_receipts().expect("artifact-owned native codec receipts");
    assert_eq!(receipts.len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.descriptor_codec_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), 26);
    assert!(receipts.iter().all(|receipt| receipt.pack_schema_hash != [0; 32] && receipt.instantiate().is_ok()));
}
