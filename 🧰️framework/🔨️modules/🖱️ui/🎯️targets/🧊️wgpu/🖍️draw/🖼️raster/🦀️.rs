//! 🖼️ Actual bounded raster reservations, resource tables and exact refusal handbacks.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
use prepared::{PreparedRasterPages,RasterContentIdentity};
use semio_framework_ui_render_prepared_shaders as shaders;

#[path = "🔑️texture-keys-ledger-and-registry/🦀️.rs"]
mod texture_keys_ledger_and_registry;
pub use texture_keys_ledger_and_registry::{
    RasterKeepSetV1, RasterResidencyLedger, RasterTexture, RasterTextureCleanupStep,
    RasterTextureWitness, RASTER_TEXTURE_ITEM_BYTE_CAPACITY, RASTER_TEXTURE_KEY_BYTES,
    RASTER_TEXTURE_TABLE_BYTE_CAPACITY, RASTER_TEXTURE_TABLE_CAPACITY,
};
#[cfg(test)]
use texture_keys_ledger_and_registry::RASTER_TEXTURE_PROBE_CAPACITY;
use texture_keys_ledger_and_registry::{
    raster_admission_fits, raster_content_is_reusable, raster_mip_bytes, raster_texture_bytes,
    raster_witness_is_stale, take_unowned_live_entry, FixedRasterTextureRegistry,
    RasterTextureEntry, RasterTextureKey,
};

#[path = "🧾️reservations-retirement-and-witnesses/🦀️.rs"]
mod reservations_retirement_and_witnesses;
pub use reservations_retirement_and_witnesses::{
    RasterOperationWitnessLedger, RasterTextureAdmission, RasterTextureCancelFault,
    RasterTextureStageFault, RasterWitnessAdmission,
};
use reservations_retirement_and_witnesses::{
    claim_raster_stage_tuple, RasterTextureReservation, RasterTextureReservationCloseCursor,
    RasterTextureReservationRetirement, RasterTextureStageClaim, RasterTextureStageClaimRetirement,
};
#[cfg(test)]
use reservations_retirement_and_witnesses::RasterTextureWitnessSlot;

#[path = "📤️upload-and-retirement-owners/🦀️.rs"]
mod upload_and_retirement_owners;
use upload_and_retirement_owners::{
    RasterTextureRetirementCursor, RasterTextureRetirementMode, RasterTextureRetirementOwner,
    RasterTextureUploadCloseCursor, RasterTextureUploadCursor,
};
pub use upload_and_retirement_owners::{RasterTextureTable, RasterUploadPixels};

#[path = "🏗️texture-table-admission/🦀️.rs"]
mod texture_table_admission;

#[path = "🔁️texture-table-ensure-and-retirement/🦀️.rs"]
mod texture_table_ensure_and_retirement;

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="🧪️tests/🖼️raster-residency/🦀️.rs"]
mod raster_residency_tests;

#[cfg(test)]
#[path="🧪️tests/🖼️raster-witness-lifecycle/🦀️.rs"]
mod raster_witness_lifecycle_tests;
