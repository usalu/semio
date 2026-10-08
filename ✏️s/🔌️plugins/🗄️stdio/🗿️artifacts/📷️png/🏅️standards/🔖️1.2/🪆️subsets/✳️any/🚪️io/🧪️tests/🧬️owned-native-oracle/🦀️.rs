use crate::PngSnapshot;

#[test]
fn owned_fixture_logical_carriers_roundtrip_and_demo_assets_match() {
 use store::{ArtifactDsl,ArtifactPack};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {let snapshot=semio_framework_pack_json::from_json_str::<PngSnapshot>(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let text=snapshot.print_dsl();let binary=snapshot.encode_pack_with(&Default::default()).unwrap();assert_eq!(PngSnapshot::parse_dsl(&text).unwrap(),snapshot);assert_eq!(PngSnapshot::decode_pack_with(&binary,&Default::default()).unwrap(),snapshot);}
 let demo=crate::schema::demo_png_snapshot();assert_eq!(demo.print_dsl(),include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio"));
}

#[test]
fn owned_fixture_publication_reports_canonical_logical_carriers() {
 owned_fixture_logical_carriers_roundtrip_and_demo_assets_match();
 let factories=crate::native_codecs();assert_eq!(factories.len(),1);
 let factory=&factories[0];let codec=(factory.codec)();let kind=(factory.kind)();
 let compiled=include_bytes!("../../💾️binary/📸️snapshot/📡️.protocol.semio");
 let current=std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
 assert_eq!(current.as_slice(),compiled,"the live native receipt requires the current compiled protocol");
 let digest=semio_framework_hash::Sha256::digest(compiled);
 assert_eq!(codec.pack_schema_hash,digest);
 assert_eq!(kind.id,crate::PNG_ARTIFACT_SCHEMA_ID);
 assert_eq!(codec.schema,crate::STDIO_PNG_DOCUMENT_SCHEMA);
 assert_eq!(codec.extension,"png");
 let hex=|bytes:&[u8]|bytes.iter().map(|byte|format!("{byte:02x}")).collect::<String>();
 let receipt=serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});
 eprintln!("[DEBUG] native-codec-publication={receipt}");
}
