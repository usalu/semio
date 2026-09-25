//! 🧬️ The owned interpreter's [`ComponentDocumentCodec`]: one compiled component answering the
//! `world actor` `codec` interface (ticket 26/09/18 slice TC3b) for the document schema it owns, so a
//! native host that links no Rust codec for that kind — a wgpu shell that mounted the component —
//! identifies and validates the kind's documents the way the hub's trusted catalog does.

use super::{CompiledHandle, GuestRuntimes};
use semio_framework_os_kernel::os_store::{ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, ComponentDocumentGenesis, VcsError};
use std::sync::{Arc, OnceLock};

/// ⛽️ One guest codec call's bound: the fuel cap is the runaway bound and the wall bounds the gap
/// between fuel observations, never the call's total (`OwnedDeadline::NoFuelProgress`) — the same
/// shape and values the hub's trusted catalog arms for the same four exports.
pub const GUEST_CODEC_BUDGET: semio_framework::kernel::Budget = semio_framework::kernel::Budget { fuel: 4_000_000_000, deadline_ms: 30_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };

/// 🧬️ A compiled component and the schema it owns. The fingerprint is asked of the component once
/// and kept: it is a pure function of the component and the schema.
pub struct OwnedComponentDocumentCodec {
    runtime: Arc<GuestRuntimes>,
    compiled: CompiledHandle,
    schema: String,
    pack_schema_hash: OnceLock<[u8; 32]>,
}

impl OwnedComponentDocumentCodec {
    /// 🧬️ Binds `compiled` to the `schema` it owns; refused unless `runtime` is the owned
    /// interpreter, the only runtime whose codec calls run outside a live actor.
    pub fn try_new(runtime: Arc<GuestRuntimes>, compiled: CompiledHandle, schema: String) -> Result<Self, super::PluginHostError> {
        if !matches!(runtime.as_ref(), GuestRuntimes::Owned(_)) {
            return Err(super::PluginHostError::Plugin(format!("component codec for {schema:?} needs the owned interpreter")));
        }
        if schema.is_empty() {
            return Err(super::PluginHostError::Plugin("component codec needs a document schema".into()));
        }
        Ok(Self { runtime, compiled, schema, pack_schema_hash: OnceLock::new() })
    }

    fn owned(&self) -> &super::OwnedRuntime {
        match self.runtime.as_ref() {
            GuestRuntimes::Owned(runtime) => runtime,
            _ => unreachable!("try_new admits only the owned interpreter"),
        }
    }
}

impl ComponentDocumentCodec for OwnedComponentDocumentCodec {
    fn schema(&self) -> &str {
        &self.schema
    }

    fn pack_schema_hash(&self) -> ComponentDocumentCodecFuture<'_, [u8; 32]> {
        Box::pin(async move {
            if let Some(hash) = self.pack_schema_hash.get() {
                return Ok(*hash);
            }
            let hash = self.owned().codec_pack_schema_hash(&self.compiled, &self.schema, GUEST_CODEC_BUDGET).await.map_err(|fault| VcsError::ValidationFailed(format!("component codec.pack-schema-hash({}): {fault}", self.schema)))?;
            Ok(*self.pack_schema_hash.get_or_init(|| hash))
        })
    }

    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles> {
        Box::pin(async move {
            let mirror = self.owned().codec_print_mirror(&self.compiled, &self.schema, pack, spr, GUEST_CODEC_BUDGET).await.map_err(|fault| VcsError::Deserialize(format!("component codec.print-mirror({}): {fault}", self.schema)))?;
            Ok(ArtifactTextFiles { dsl: mirror.dsl, ops: mirror.ops })
        })
    }

    fn genesis<'a>(&'a self, document_id: &'a str) -> ComponentDocumentCodecFuture<'a, ComponentDocumentGenesis> {
        Box::pin(async move {
            let pair = self.owned().codec_genesis(&self.compiled, &self.schema, document_id, GUEST_CODEC_BUDGET).await.map_err(|fault| VcsError::ValidationFailed(format!("component codec.genesis({}): {fault}", self.schema)))?;
            Ok(ComponentDocumentGenesis { pack: pair.pack, spr: pair.spr })
        })
    }
}
