//! 🎭️ Set local mask coverage for alpha painting.
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, SetMaskValueEdit};
use crate::{RasterMutation,RasterSnapshot};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword="mask-value")]
pub struct SetMaskValue {pub value:u32}
pub fn handle(payload:&SetMaskValue,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{
    if payload.value>255{return Err(Fault::from("raster-mask-value-invalid"));}
    Ok(Emit::config(vec![RasterConfigMutation::SetMaskValue(SetMaskValueEdit {value:payload.value})]))
}
