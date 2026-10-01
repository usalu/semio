//! 🧫️ Neutral document snapshot carried by the real host component fixture.
use semio_framework_os_kernel::{os_store as store,os_pack as pack,os_dsl as dsl};
use semio_framework_value_derive::{ToValue,FromValue};
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue,dsl::DslRecord)]
#[value(deny_unknown_fields)]
#[dsl(id="fixture.neutral-host-fixture.counter",layout="lines")]
pub struct Snapshot {pub count:i32}
impl semio_framework_schema::ArtifactCompositionFields for Snapshot {
    fn visit_child_refs<'a,V:semio_framework_schema::ChildRefVisitor<'a>>(&'a self,_visitor:&mut V)->Result<(),V::Error>{Ok(())}
}
impl store::ArtifactDsl for Snapshot {
    const EXTENSION:&'static str="neutral-host-fixture";
    fn parse_dsl(text:&str)->Result<Self,store::TextError>{if text.trim().is_empty(){return Ok(Self::default());}pack::json::from_json_str(text).map_err(|error|store::TextError::new(error.to_string(),store::TextSpan::at(1,1)))}
    fn print_dsl(&self)->String{pack::json::to_json_string(&semio_framework_os_kernel::ToValue::to_value(self))}
}
impl store::ArtifactPack for Snapshot {
    fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{store::pack_rt::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options)}
    fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(record,_)=store::pack_rt::decode_document(bytes,&Self::__dsl_spec(),options)?;Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)}
    fn record_spec()->Option<dsl::RecordSpec>{Some(Self::__dsl_spec())}
}
