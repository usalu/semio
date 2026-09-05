//! 🚪️ block2d ← obj — foreign `Deserializer<Block2dSnapshot>` that HONESTLY REFUSES.
//!
//! A `s.block.block2d` document is a node kind DEFINITION, not geometry or a raster. Identical to the sibling `🔺️stl` leaf's reasoning: `representations[].mesh_url` is a reference, `handles` are anchor frames, and an OBJ mesh carries no identity/catalog/compatibility data.
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
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.obj@3.0/*` → `s.block.block2d@1/*` — always `Err`, see this file's module doc.
pub struct ObjIntoBlock2d;

impl Deserializer<Block2dSnapshot> for ObjIntoBlock2d {
    const FROM: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<Block2dSnapshot> {
        Err(IoError {
            message: "obj import not supported for a node kind definition: an OBJ mesh carries vertices and faces only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from".to_string(),
            diagnostics: Vec::new(),
        })
    }
}
