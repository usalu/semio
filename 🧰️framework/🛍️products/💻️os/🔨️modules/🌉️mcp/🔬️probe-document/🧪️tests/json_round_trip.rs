//! 🔁️ The probe's owner-less JSON print and parse round-trip one tree and agree with serde_json.

use semio_framework_value::{DslValue,Number};
use semio_os_kernel::ArtifactPack;
use semio_os_mcp_probe_document::probe_document::ProbeSnapshot;

fn tree()->DslValue{DslValue::Object(vec![
 ("name".into(),DslValue::String("probe".into())),
 ("count".into(),DslValue::Number(Number::UInt(3))),
 ("items".into(),DslValue::Array(vec![DslValue::Bool(true),DslValue::Null,DslValue::String("x".into())])),
])}

#[test]
fn print_matches_serde_json_and_parse_restores_the_tree(){
 let snapshot=ProbeSnapshot(tree());
 let bytes=snapshot.encode_pack_with(&Default::default()).expect("probe prints through owner-less retirement turns");
 let printed:serde_json::Value=serde_json::from_slice(&bytes).expect("printed text is JSON");
 assert_eq!(printed,serde_json::json!({"name":"probe","count":3,"items":[true,null,"x"]}));
 let restored=ProbeSnapshot::decode_pack_with(&bytes,&Default::default()).expect("probe parses through owner-less retirement turns");
 assert_eq!(restored.0,tree());
}
