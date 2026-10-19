//! 📥️ Items of `store` that belong to `os-store-config` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_os_store_space_schema::{ArtifactBackboneRef, ArtifactEnvelope};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_os_store_document::create_document_envelope;
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️MergeHelpers

pub type ConfigEnvelope<C, ConfigMutation> = ArtifactEnvelope<C, ConfigMutation>;
//#endregion 🔖️MergeHelpers
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️Config
/// 🎚️ A config store's envelope: a document envelope whose history holds undo and redo only
/// ([`semio_framework_os_spr::HistoryShape::Config`]).
pub async fn create_config_envelope<C, ConfigMutation>(schema: &str, id: &str, initial_snapshot: C, backbone: Option<ArtifactBackboneRef>) -> ConfigEnvelope<C, ConfigMutation>
where
    C: Clone + pack::snapshot::ArtifactPack,
{
    let mut envelope = create_document_envelope(schema, id, initial_snapshot, backbone, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.history_shape = semio_framework_os_spr::HistoryShape::Config;
    envelope
}

/// 🧮️ Config snapshots use the same DSL law as documents — `ConfigRecord` marks config types.
pub trait ConfigRecord: ArtifactDsl {}
//#endregion 🔖️Config
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
