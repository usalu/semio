//! 📄️ Owns the original picker callback without copying the encoded image.
use super::{ArtifactView, ConfigView, Emit, Fault, RasterConfig, RasterConfigMutation, RasterMutation, RasterSnapshot, base64_offset};
use semio_framework_value::DslValue;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-png-file")]
pub struct ImportPngFile {
    pub payload: String,
}

impl ImportPngFile {
    /// 📦️ Moves only a validated PNG data URL out of the staged file action.
    pub fn receive(args: &mut Option<DslValue>) -> Result<Self, Fault> {
        let refused = || Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("raster.import.png-file"), "The chosen file is missing or exceeds the PNG import limit.");
        let Some(DslValue::Object(fields)) = args else { return Err(refused()) };
        let mut payloads = fields.iter_mut().filter(|(name, _)| name == "payload");
        let Some((_, DslValue::String(payload))) = payloads.next() else { return Err(refused()) };
        if payloads.next().is_some() || base64_offset(payload).is_none() {
            return Err(refused());
        }
        Ok(Self { payload: std::mem::take(payload) })
    }
}

pub fn handle(_: &ImportPngFile, _: &ArtifactView<'_, RasterSnapshot>, _: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Err(Fault::from("raster.import.requires-retained-work"))
}
