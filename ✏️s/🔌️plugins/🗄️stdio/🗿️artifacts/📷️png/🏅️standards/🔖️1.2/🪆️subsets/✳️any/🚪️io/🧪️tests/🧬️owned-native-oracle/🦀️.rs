use crate::PngSnapshot;

#[test]
fn owned_fixture_logical_carriers_roundtrip_and_demo_assets_match() {
 use store::{ArtifactDsl,ArtifactPack};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {let snapshot=semio_framework_pack_json::from_json_str::<PngSnapshot>(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let text=snapshot.print_dsl();let binary=snapshot.encode_pack_with(&Default::default()).unwrap();assert_eq!(PngSnapshot::parse_dsl(&text).unwrap(),snapshot);assert_eq!(PngSnapshot::decode_pack_with(&binary,&Default::default()).unwrap(),snapshot);}
 let demo=crate::schema::demo_png_snapshot();assert_eq!(demo.print_dsl(),include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio"));
}
