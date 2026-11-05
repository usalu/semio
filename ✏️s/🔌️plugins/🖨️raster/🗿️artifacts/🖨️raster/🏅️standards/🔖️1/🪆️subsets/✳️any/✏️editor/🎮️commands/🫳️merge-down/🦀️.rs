//! 🫳️ One small intent selects adjacent normal-blend siblings for the original retained bake owner.
use crate::config::{RasterConfig,RasterConfigMutation};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[dsl(keyword="merge-down")]
#[value(rename_all="camelCase")]
pub struct MergeDown{pub layer_id:String}
pub fn handle(_payload:&MergeDown,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{Err(Fault::from("raster.merge-requires-retained-work"))}
