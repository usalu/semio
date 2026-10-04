//! 📤️ Export the visible layer composition through a cancellable download job.
use crate::{RasterSnapshot,RasterMutation};
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="export-png")]
pub struct ExportPng {}

pub fn handle(_: &ExportPng,_: &ArtifactView<'_,RasterSnapshot>,_: &ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {Err(Fault::from("raster.export-job-required"))}
