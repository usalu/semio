//! 📤️ Original atlas and raster upload staging with independent credits and retirement.
use semio_framework_ui_scene_mesh::Mesh3dLease;
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicU8, Ordering};
use std::sync::{LazyLock, Mutex};
#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;
use retirement::*;

//#region 🧩️PagedRasterProducer
pub use semio_framework_ui_raster_owner::RASTER_CONTENT_PAGE_BYTES as PREPARED_RASTER_PAGE_BYTES;

pub use semio_framework_ui_raster_owner::RasterContentIdentity;

#[path = "🧾️raster-ledger-and-atlas-permits/🦀️.rs"]
mod raster_ledger_and_atlas_permits;
use raster_ledger_and_atlas_permits::{
    atlas_add, atlas_cap, atlas_gate, atlas_progress, atlas_refused, PreparedAtlasAbandonment,
    PreparedAtlasPage, PreparedAtlasPermit, PreparedRasterCredit, PreparedRasterPage,
    PREPARED_ATLAS_ABANDONMENT_OWNER, PREPARED_ATLAS_ABANDONMENT_STATE,
    PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES, PREPARED_RASTER_KEY_BYTES, PREPARED_RASTER_LEDGER,
};
pub use raster_ledger_and_atlas_permits::{
    PreparedAtlasAuthority, PreparedAtlasPages, PreparedRasterGeneration, PreparedRasterPages,
    PREPARED_ATLAS_PAGE_BYTES, PREPARED_ATLAS_PAGE_CAPACITY, PREPARED_RASTER_ITEM_BYTES,
};
#[cfg(test)]
use raster_ledger_and_atlas_permits::{
    PreparedRasterLedger, PREPARED_ATLAS_PROCESS_ITEMS, PREPARED_ATLAS_PROCESS_PERMITS,
    PREPARED_RASTER_PRODUCER_BYTES, PREPARED_RASTER_PRODUCER_CAPACITY,
    PREPARED_RASTER_PRODUCER_ITEMS,
};

#[path = "🗒️atlas-pages-and-raster-pages/🦀️.rs"]
mod atlas_pages_and_raster_pages;

#[path = "🎞️raster-reservation-and-producer/🦀️.rs"]
mod raster_reservation_and_producer;
pub use raster_reservation_and_producer::{
    PreparedRasterProducer, PreparedRasterProducerStep, PreparedRasterRejected,
    PreparedRasterReservation,
};

#[path = "🛫️render-upload-and-eviction/🦀️.rs"]
mod render_upload_and_eviction;
pub use render_upload_and_eviction::{PreparedRenderEviction, PreparedRenderUpload, RenderDirective};

//#endregion 🧩️PagedRasterProducer

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../🖼️atlas/🧪️tests/🦀️.rs"]
mod atlas_original_owner_tests;

#[cfg(test)]
#[global_allocator]
static ALLOCATOR: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
pub fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    assert!(!heap.overflowed);
    (result, heap.requested_bytes, heap.released_bytes)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/♻️abandoned/🦀️.rs"]
mod abandoned_original_owner_tests;
