//! 📦️ Neutral durable blob identity and original controlled retirement laws.
use semio_framework_os_store_blob_identity::BlobRef;
use semio_framework_value::{FromValue,ToValue};

#[test]
fn neutral_blob_identity_agrees_with_independent_serde_including_full_u64() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let input=row["json"].as_str().unwrap();
        let oracle=serde_json::from_str::<BlobRef>(input).ok();
        let actual=serde_json::from_str::<semio_framework_value::DslValue>(input).ok().and_then(|value|BlobRef::from_value(value).ok());
        assert_eq!(oracle.is_some(),row["accepted"].as_bool().unwrap(),"{input}");
        assert_eq!(actual,oracle,"{input}");
        if let Some(blob)=actual {
            let expected:serde_json::Value=serde_json::from_str(input).unwrap();
            assert_eq!(serde_json::to_value(blob.to_value()).unwrap(),expected);
            assert_eq!(serde_json::to_value(&blob).unwrap(),expected);
            assert_eq!(BlobRef::from_value(blob.to_value()).unwrap(),blob);
        }
    }
    eprintln!("[DEBUG] Exact blob fields, full u64, Unicode and refusal corpus agree with independent Serde");
}

#[test]
fn canonical_blob_owner_retains_original_strings_until_funded_retirement() {
    use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonNode};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    let original=BlobRef{hash:"original-blob-hash".into(),size:u64::MAX,media_type:"application/octet-stream".into()};
    assert!(matches!(original.canonical_tree_node().unwrap(),ArtifactCanonicalJsonNode::Object(3)));
    let pointer=original.hash.as_ptr();
    let mut owner=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("canonical blob owner refused"));
    assert_eq!(owner.original().unwrap().hash.as_ptr(),pointer);
    let denied=owner.step(RetainedCloneGrant::default());
    if let Ok(step)=denied {assert_eq!(step.progress(),Default::default());}
    assert_eq!(owner.original().unwrap().hash.as_ptr(),pointer);
    let mut released=0;
    for _ in 0..512 {
        if owner.terminal_is_empty() {break;}
        let copy=owner.next_copy_byte_demand().unwrap().max(1);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let step=owner.step(grant).unwrap();
        assert!(step.progress().fits(grant));
        released+=step.progress().released_bytes;
    }
    assert!(owner.terminal_is_empty());
    assert!(released>0);
    eprintln!("[DEBUG] Original blob identity pointers survive denied work and retire through funded canonical owner");
}
