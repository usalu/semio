//! 🚪️ Native artifact representation codecs.
use super::super::super::*;
use semio_framework_os_kernel::{os_spr as protocol,os_store as store};

impl store::ArtifactDsl for Snapshot {
 const EXTENSION:&'static str="snapshot-refusal";
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|store::TextError::from_value_error(error,store::TextSpan::at(1,1)))}
 fn print_dsl(&self)->String{semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(self))}
}
