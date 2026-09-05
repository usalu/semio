//! 🚪️ block3d ← stl — foreign `Deserializer<Block3dSnapshot>` that HONESTLY REFUSES.
//!
//! A `s.block.block3d` document is an object kind DEFINITION, not geometry or a raster. Its only geometry-bearing field is `representations[].mesh_url` — a URL pointing at an external mesh asset, never vertex/triangle data — and `vortices` are anchor frames (angle/radius/position/direction), not a surface. Nothing in the schema an STL triangle soup could be built from, and nothing in an STL an identity/catalog/compatibility document could be built from.
//!
//! So this leaf returns a typed `IoError` naming the reason instead of an empty snapshot or an
//! invented solid. It stays REGISTERED on the `io_mechanism` channel at the weakest fidelity
//! (`IoFidelity::Lossy`, rank 0 — the router never prefers it over a real hop) so a caller that does
//! route here gets this reason back rather than a bare "no route" (see `📓️w3-io.md`).

use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf would read.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.stl@ascii/*` → `s.block.block3d@1/*` — always `Err`, see this file's module doc.
pub struct StlIntoBlock3d;

impl Deserializer<Block3dSnapshot> for StlIntoBlock3d {
    const FROM: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<Block3dSnapshot> {
        Err(IoError {
            message: "stl import not supported for an object kind definition: an STL solid carries triangles only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from".to_string(),
            diagnostics: Vec::new(),
        })
    }
}
