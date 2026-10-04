//! 🎯️ Choose whether brush and eraser edit pixels or mask coverage.
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation};
use crate::{RasterMutation,RasterSnapshot};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="paint-target")]
pub struct SetPaintTarget {pub value:String}
pub fn handle(payload:&SetPaintTarget,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{
    if !matches!(payload.value.as_str(),"pixels"|"mask"){return Err(Fault::from("raster-paint-target-invalid"));}
    Ok(Emit::config(vec![RasterConfigMutation::SetPaintTarget{value:payload.value.clone()}]))
}
