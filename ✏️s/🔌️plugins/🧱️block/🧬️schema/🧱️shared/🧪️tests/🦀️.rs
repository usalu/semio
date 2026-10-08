//! 🧱️ Plugin-owned native records agree with the portable JSON corpus and independent Serde.
use semio_s_plugin_block::*;
fn roundtrip<T: serde::de::DeserializeOwned + serde::Serialize + semio_framework_value::ToValue + semio_framework_value::FromValue>(input:serde_json::Value)->serde_json::Value{
 let value:T=serde_json::from_value(input.clone()).expect("native shared record");
 let mut encode_calls=0usize;let mut encode_progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{encode_calls+=1;event.owned_bytes<=65536&&(event.total==0||event.completed<=event.total)};
 let mut encode=semio_framework_value::NativeEncodeControl::new(65536,&mut encode_progress);
 let owned=value.to_value_controlled(&mut encode).expect("controlled shared record projection");drop(encode);assert!(encode_calls>0);assert_eq!(serde_json::Value::from(&owned),input);
 let mut decode_calls=0usize;let mut decode_progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{decode_calls+=1;event.owned_bytes<=65536&&(event.total==0||event.completed<=event.total)};
 let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();let mut decode=semio_framework_value::NativeDecodeControl::new(65536,&mut decode_progress);decode.install_retirement_recipient(&mut recipient).expect("explicit shared record retirement recipient");
 let actual=T::from_value_controlled(&owned,&mut decode).expect("controlled shared record construction");drop(decode);assert!(decode_calls>0);assert!(semio_framework_value::ErasedSnapshotRetirement::terminal_is_empty(&recipient));
 serde_json::to_value(actual).expect("independent shared record output")
}
#[test]
fn shared_plugin_records_match_the_portable_schema_vectors(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("shared corpus");assert_eq!(fixture["schemaVersion"],1);let rows=fixture["cases"].as_array().expect("eight records");assert_eq!(rows.len(),8);for row in rows{let input=row["input"].clone();let actual=match row["type"].as_str().expect("type"){"BlockKindIdentity"=>roundtrip::<BlockKindIdentity>(input.clone()),"BlockAttribute"=>roundtrip::<BlockAttribute>(input.clone()),"BlockAuthor"=>roundtrip::<BlockAuthor>(input.clone()),"BlockCompatibilityRule"=>roundtrip::<BlockCompatibilityRule>(input.clone()),"BlockRepresentation"=>roundtrip::<BlockRepresentation>(input.clone()),"BlockCamera2d"=>roundtrip::<BlockCamera2d>(input.clone()),"BlockCamera3d"=>roundtrip::<BlockCamera3d>(input.clone()),"BlockMeta"=>roundtrip::<BlockMeta>(input.clone()),_=>panic!("unknown shared type")};assert_eq!(actual,input,"{}",row["type"]);eprintln!("[DEBUG] Native Block plugin shared schema {}",row["type"]);}}
#[test]
fn shared_retirement_is_owned_by_the_plugin(){fn owner<T:semio_framework_value::retirement::RetireOwned>(){} owner::<BlockKindIdentity>();owner::<BlockAttribute>();owner::<BlockAuthor>();owner::<BlockCompatibilityRule>();owner::<BlockRepresentation>();owner::<BlockCamera2d>();owner::<BlockCamera3d>();owner::<BlockMeta>();}

fn representation(tags: &[&str], attributes: &[(&str, &str)]) -> BlockRepresentation {
    BlockRepresentation {
        id: "r0".into(),
        name: "mesh".into(),
        mesh_url: None,
        tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
        lod: None,
        description: String::new(),
        attributes: attributes.iter().map(|(key, value)| BlockAttribute { key: (*key).into(), value: (*value).into(), definition: None }).collect(),
    }
}

#[test]
fn a_representation_patch_edits_its_tag_and_attribute_sets_in_place() {
    let base = representation(&["a", "b", "c"], &[("k", "1")]);
    let patch = BlockRepresentationPatch {
        name: Some("renamed".into()),
        tags_removed: vec!["b".into()],
        tags_added: vec!["d".into()],
        attributes_added: vec![BlockAttribute { key: "j".into(), value: "2".into(), definition: None }],
        ..Default::default()
    };
    let after = patch.patched(&base).expect("the patch fits the row");
    assert_eq!(after.name, "renamed");
    assert_eq!(after.tags, ["a", "c", "d"]);
    assert_eq!(after.attributes.len(), 2);
    let inverse = patch.inverse(&base);
    assert_eq!(inverse.patched(&after).expect("the inverse fits the patched row"), base, "the inverse restores values AND positions");
    assert_eq!(BlockRepresentationPatch::between(&base, &after).patched(&base).expect("between fits"), after);
}

#[test]
fn a_representation_patch_refuses_entries_that_do_not_fit() {
    let base = representation(&["a"], &[]);
    let removes_a_ghost = BlockRepresentationPatch { tags_removed: vec!["ghost".into()], ..Default::default() };
    assert!(removes_a_ghost.patched(&base).is_err());
    let adds_a_duplicate = BlockRepresentationPatch { tags_added: vec!["a".into()], ..Default::default() };
    assert!(adds_a_duplicate.patched(&base).is_err());
}

#[test]
fn tag_patches_coalesce_per_entry() {
    let base = representation(&["a", "b"], &[]);
    let add = |tag: &str| BlockRepresentationPatch { tags_added: vec![tag.into()], ..Default::default() };
    let remove = |tag: &str| BlockRepresentationPatch { tags_removed: vec![tag.into()], ..Default::default() };
    let mut cancelled = add("x");
    cancelled.absorb(remove("x"));
    assert!(cancelled.is_empty(), "an entry added then removed leaves nothing");
    let mut replaced = remove("a");
    replaced.absorb(add("a"));
    assert_eq!(replaced.patched(&base).expect("replace fits").tags, ["b", "a"]);
    let mut dropped = remove("a");
    dropped.absorb(remove("b"));
    assert_eq!(dropped.patched(&base).expect("both removals fit").tags, Vec::<String>::new());
}

#[test]
fn a_kind_identity_patch_sets_and_clears_optional_fields() {
    let base = BlockKindIdentity { id: "k".into(), name: "n".into(), label: "l".into(), variant: Some("v".into()), description: String::new(), icon: None, unit: None };
    let patch = BlockKindIdentityPatch { variant: Some(BlockOptionalText { value: None }), icon: Some(BlockOptionalText { value: Some("i".into()) }), ..Default::default() };
    let after = patch.patched(&base).expect("identity patches always fit");
    assert_eq!((after.variant.clone(), after.icon.clone()), (None, Some("i".into())));
    assert_eq!(patch.inverse(&base).patched(&after).expect("inverse fits"), base);
    let mut merged = BlockKindIdentityPatch { name: Some("x".into()), label: Some("y".into()), ..Default::default() };
    merged.absorb(BlockKindIdentityPatch { name: Some("z".into()), ..Default::default() });
    assert_eq!((merged.name.as_deref(), merged.label.as_deref()), (Some("z"), Some("y")));
}

#[test]
fn id_keyed_rows_cancel_replace_and_restore_order() {
    let author = |id: &str| BlockAuthor { id: id.into(), name: id.to_uppercase(), email: None };
    let base = vec![author("a"), author("b"), author("c")];
    let mut created_then_deleted = BlockAuthorsDelta { added: vec![author("x")], ..Default::default() };
    created_then_deleted.absorb(BlockAuthorsDelta { removed: vec!["x".into()], ..Default::default() });
    assert!(created_then_deleted.is_empty());
    let removed_middle = BlockAuthorsDelta { removed: vec!["b".into()], ..Default::default() };
    let after = removed_middle.apply(&base).expect("removal fits");
    assert_eq!(removed_middle.inverse(&base).apply(&after).expect("inverse fits"), base, "the inverse puts the row back into its slot through a full order");
    let target = vec![author("c"), author("a"), author("d")];
    assert_eq!(BlockAuthorsDelta::between(&base, &target).apply(&base).expect("between fits"), target);
}

#[test]
fn inserting_at_an_index_reorders_only_inside_the_list() {
    assert_eq!(crate::block_insert_order(["a", "b", "c"], "x", None), None);
    assert_eq!(crate::block_insert_order(["a", "b", "c"], "x", Some(3)), None);
    assert_eq!(crate::block_insert_order(["a", "b", "c"], "x", Some(9)), None);
    assert_eq!(crate::block_insert_order(["a", "b", "c"], "x", Some(1)), Some(vec!["a".to_string(), "x".into(), "b".into(), "c".into()]));
    let author = |id: &str| BlockAuthor { id: id.into(), name: id.to_uppercase(), email: None };
    let base = vec![author("a"), author("b"), author("c")];
    let without_middle = vec![author("a"), author("c")];
    let delete_middle = BlockAuthorsDelta { removed: vec!["b".into()], ..Default::default() };
    let recreate_middle = BlockAuthorsDelta { added: vec![author("b")], reordered: crate::block_insert_order(["a", "c"], "b", Some(1)), ..Default::default() };
    assert_eq!(recreate_middle.apply(&without_middle).expect("insertion fits"), base);
    assert_eq!(delete_middle.inverse(&base), recreate_middle, "the inverse of deleting a middle row is its insertion at the original index");
}
