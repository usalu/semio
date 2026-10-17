//! 🧰️ The one bound every raster retained-command work shares: any artifact app whose command channel is the closed `RasterCommand`.
//!
//! 🔗 Work types name no concrete app, so the commands layer needs nothing from the app that dispatches them.
use crate::config::{RasterConfig, RasterConfigMutation};
use crate::RasterCommand;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_framework_plugin::{ArtifactApp, NoDraftMutation};

/// 🧰️ Any artifact app that dispatches the closed raster command channel over the raster document, mutation and session config.
pub trait RasterWorkApp: ArtifactApp<Command = RasterCommand, Snapshot = RasterSnapshot, Mutation = RasterMutation, Config = RasterConfig, ConfigMutation = RasterConfigMutation, DraftMutation = NoDraftMutation> {}

impl<A> RasterWorkApp for A where A: ArtifactApp<Command = RasterCommand, Snapshot = RasterSnapshot, Mutation = RasterMutation, Config = RasterConfig, ConfigMutation = RasterConfigMutation, DraftMutation = NoDraftMutation> {}
