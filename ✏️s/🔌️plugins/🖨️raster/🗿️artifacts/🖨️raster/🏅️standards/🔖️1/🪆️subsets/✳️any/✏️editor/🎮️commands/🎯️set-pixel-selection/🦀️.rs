//! 🎯️ Publish completed intrinsic selection coverage into session configuration.
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation,RasterPixelSelection};
use crate::{RasterMutation,RasterSnapshot};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all="camelCase")]
#[dsl(keyword="pixel-selection")]
pub struct SetPixelSelection {
    #[dsl(block)]
    pub selection:Option<RasterPixelSelection>,
    pub expected_image_key:Option<String>,
}
pub fn handle(payload:&SetPixelSelection,doc:&ArtifactView<'_,RasterSnapshot>,cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{
    if let Some(selection)=&payload.selection{
        if selection.target!=cfg.snapshot.paint_target{return Err(Fault::from("raster-selection-target-changed"));}
        selection.validate_current(doc.snapshot,payload.expected_image_key.as_deref())?;
    }
    Ok(Emit::config(vec![RasterConfigMutation::SetPixelSelection{selection:payload.selection.clone()}]))
}
