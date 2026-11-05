//! 🧱️ Independent neutral material arithmetic qualifies the actual guest JSON output.
#[path="../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧱️material/🦀️.rs"]
mod oracle;
use semio_s_artifact_puzzle_2d::Puzzle2dSnapshot;
use semio_os_kernel::os_store::serialization::ArtifactSqliteSnapshotHost as _;
use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotEncoding};
use semio_framework_value::ToValue;
use semio_framework::io_schema::IoPayload;
use semio_framework_actor::OriginalJobSource;
use serde_json::Value;

struct Source(Option<Puzzle2dSnapshot>);
impl Drop for Source{fn drop(&mut self){if let Some(source)=self.0.take(){source.retire_sqlite_snapshot();}}}
pub struct Material{pub expected:Value,pub input:Vec<u8>,maximum_result_bytes:usize}
pub fn prepare(law:&Value)->Material{
 let(document,expected)=oracle::material(&law["material"]);
 let source=Source(Some(semio_framework_pack_json::from_json_str::<Puzzle2dSnapshot>(&serde_json::to_string(&document).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored neutral source vocabulary")));
 let native=original_owners::with_encoding(|owner|source.0.as_ref().unwrap().encode_native_snapshot(NativeSnapshotEncoding::Binary,owner.native())).expect("original native material carrier");
 let input=semio_framework_value::DslValue::Object(vec![("source".into(),semio_framework_value::DslValue::String(law["route"]["from"].as_str().unwrap().into())),("target".into(),semio_framework_value::DslValue::String(law["route"]["into"].as_str().unwrap().into())),("payload".into(),native.to_value())]);
 let input=semio_framework_pack_json::to_json_string(&input).into_bytes();assert!(input.len()<=law["limits"]["requestBytes"].as_u64().unwrap()as usize,"actual serialized request exceeds the closed case envelope");
 Material{expected,input,maximum_result_bytes:law["limits"]["resultBytes"].as_u64().unwrap()as usize}
}

impl Material{
 pub fn source(&self,job:u64)->OriginalJobSource{OriginalJobSource{job,kind:"semio.io-run".into(),input:Some(self.input.clone()),checkpoint:None}}
 pub fn verify(&self,bytes:&[u8]){
  let payload:IoPayload=semio_framework_pack_json::from_json_str(std::str::from_utf8(bytes).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual public job IO payload");let IoPayload::Text(text)=payload else{panic!("actual public JSON job retains its declared text carrier")};let bytes=text.into_bytes();assert!(bytes.len()<=self.maximum_result_bytes,"actual foreign file exceeds the closed case envelope");let actual:Value=serde_json::from_slice(&bytes).expect("independent third-party actual material JSON");assert_eq!(actual,self.expected,"complete large board material independently predicted from the neutral recipe");
 }
}
