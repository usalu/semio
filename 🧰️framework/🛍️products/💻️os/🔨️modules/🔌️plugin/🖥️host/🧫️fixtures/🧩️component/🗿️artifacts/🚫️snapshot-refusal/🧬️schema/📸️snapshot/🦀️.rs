//! 🚫️ Complete typed document owned by the selected compiled refusal contributors.
use semio_framework_os_kernel::os_store as store;
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord)]
#[value(deny_unknown_fields)]
#[dsl(id="fixture.neutral-host-fixture.snapshot-refusal",layout="lines")]
pub struct Snapshot {pub value:i32}
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;
impl semio_framework_schema_composition::ArtifactCompositionFields for Snapshot {
 fn visit_child_refs<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,_visitor:&mut V)->Result<(),V::Error>{Ok(())}
}
impl store::ArtifactDsl for Snapshot {
 const EXTENSION:&'static str="snapshot-refusal";
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|store::TextError::from_value_error(error,store::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(self))}
}
impl store::ArtifactPack for Snapshot {
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(record,_)=store::pack_rt::decode_document(bytes,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
}
