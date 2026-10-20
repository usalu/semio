//! 🧪️ The GIS Map binding decodes the canonical proposal preview, refuses every substitution by kind, and answers the default Map's fixed-three members.

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
}

#[test]
fn the_gis_map_preview_decodes_the_canonical_proposal_into_its_exact_region() {
    let fixture = fixture();
    let job_id = fixture["sampleJobId"].as_str().expect("sample job");
    let preview = GisMapInferenceBindingV1.preview(job_id, fixture["proposalCanonical"].as_str().expect("canonical proposal").as_bytes()).expect("canonical preview");
    assert_eq!(serde_json::json!(preview.region_id), fixture["preview"]["regionId"]);
    assert_eq!(serde_json::json!(preview.ring), fixture["preview"]["ring"]);
}

#[test]
fn the_gis_map_preview_refuses_each_substitution_by_kind() {
    let fixture = fixture();
    let job_id = fixture["sampleJobId"].as_str().expect("sample job");
    let proposal = || serde_json::from_str::<serde_json::Value>(fixture["proposalCanonical"].as_str().expect("proposal")).expect("proposal value");
    assert_eq!(gis_map_region_preview(job_id, b"{\"DeleteRegion\":{\"id\":\"inference-11111111111111111111111111111111\"}}").err(), Some(MapBindingRefusalV1::Composition));
    for (name, value, pointer, expected) in [
        ("wrong-region-id", serde_json::json!("substituted"), "/CreateRegion/item/id", MapBindingRefusalV1::Composition),
        ("wrong-kind", serde_json::json!("route"), "/CreateRegion/item/data/kind", MapBindingRefusalV1::Composition),
        ("short-ring", serde_json::json!([[7, 46], [9, 46], [9, 48], [7, 46]]), "/CreateRegion/item/data/ring", MapBindingRefusalV1::Bounds),
        ("reordered-ring", serde_json::json!([[7, 46], [7, 48], [9, 48], [9, 46], [7, 46]]), "/CreateRegion/item/data/ring", MapBindingRefusalV1::Composition),
        ("out-of-range-ring", serde_json::json!([[-181, 46], [9, 46], [9, 48], [-181, 48], [-181, 46]]), "/CreateRegion/item/data/ring", MapBindingRefusalV1::Composition),
    ] {
        let mut candidate = proposal();
        *candidate.pointer_mut(pointer).unwrap_or_else(|| panic!("{name} pointer")) = value;
        assert_eq!(gis_map_region_preview(job_id, &serde_json::to_vec(&candidate).expect("candidate bytes")).err(), Some(expected), "{name}");
    }
    assert_eq!(gis_map_region_preview(job_id, b"{").err(), Some(MapBindingRefusalV1::Undecodable));
}

#[test]
fn the_default_gis_map_owns_exactly_the_fixed_three_commit_members() {
    let pack = semio_s_artifact_gis_gismap::schema::default_document().encode_pack();
    let members = GisMapInferenceBindingV1.composed_children(&pack).expect("default Map members");
    assert_eq!(members.as_slice(), GisMapInferenceBindingV1.commit_members());
    assert_eq!(GisMapInferenceBindingV1.commit_members(), <GisMapInferenceBindingV1 as ComposedMapCommitBindingV1>::COMMIT_MEMBERS);
    let parent = <GisMapInferenceBindingV1 as ComposedMapCommitBindingV1>::decode_parent(&pack).expect("default Map parent");
    let (drawing, value) = <GisMapInferenceBindingV1 as ComposedMapCommitBindingV1>::derived_members(&parent);
    assert_eq!(drawing, semio_s_artifact_gis_gismap::schema::gis_map_snapshot_to_drawing(&parent));
    assert_eq!(value, semio_s_artifact_gis_gismap::gis_map_value_from_descriptor_json(&semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json(&parent)));
    assert_eq!(GisMapInferenceBindingV1.composed_children(&[0xff]), Err(MapBindingRefusalV1::Undecodable));
}
