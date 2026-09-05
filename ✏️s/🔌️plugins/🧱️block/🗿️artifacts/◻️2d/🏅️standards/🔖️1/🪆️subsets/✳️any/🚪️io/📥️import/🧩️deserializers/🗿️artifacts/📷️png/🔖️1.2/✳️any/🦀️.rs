//! 🚪️ block2d ← png — foreign `Deserializer<Block2dSnapshot>` that HONESTLY REFUSES.
//!
//! A `s.block.block2d` document is a node kind DEFINITION, not geometry or a raster. A block document has no pixel field at all, and this plugin ships no rasterizer (the `👁️viewer` renders through the framework's window kits, not into a buffer this leaf can reach). Painting a blank canvas — the shape `🗒️note`'s own png leaf settled for — would silently claim an export that did not happen.
//!
//! So this leaf returns a typed `IoError` naming the reason instead of an empty snapshot or an
//! invented solid. It stays REGISTERED on the `io_mechanism` channel at the weakest fidelity
//! (`IoFidelity::Lossy`, rank 0 — the router never prefers it over a real hop) so a caller that does
//! route here gets this reason back rather than a bare "no route" (see `📓️w3-io.md`).

use crate::artifacts::block2d::Block2dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf would read.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.png@1.2/*` → `s.block.block2d@1/*` — always `Err`, see this file's module doc.
pub struct PngIntoBlock2d;

impl Deserializer<Block2dSnapshot> for PngIntoBlock2d {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<Block2dSnapshot> {
        Err(IoError {
            message: "png import not supported for a node kind definition: a PNG carries pixels only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from".to_string(),
            diagnostics: Vec::new(),
        })
    }
}
