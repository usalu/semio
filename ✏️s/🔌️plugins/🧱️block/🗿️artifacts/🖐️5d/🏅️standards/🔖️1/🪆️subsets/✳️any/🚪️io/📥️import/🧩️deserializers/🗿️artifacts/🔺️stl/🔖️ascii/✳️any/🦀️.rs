//! 🚪️ block5d ← stl — foreign `Deserializer<Block5dSnapshot>` that HONESTLY REFUSES.
//!
//! A `s.block.block5d` document is a part kind DEFINITION, not geometry or a raster. Its only geometry-bearing field is `representations[].mesh_url` — a URL pointing at an external mesh asset, never vertex/triangle data — and `grips` are anchor frames (angle/radius/position/direction), not a surface. Nothing in the schema an STL triangle soup could be built from, and nothing in an STL an identity/catalog/compatibility document could be built from.
//!
//! So this leaf returns a typed `IoError` naming the reason instead of an empty snapshot or an
//! invented solid. It stays REGISTERED on the `io_mechanism` channel at the weakest fidelity
//! (`IoFidelity::Lossy`, rank 0 — the router never prefers it over a real hop) so a caller that does
//! route here gets this reason back rather than a bare "no route" (see `📓️w3-io.md`).

use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf would read.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.stl@ascii/*` → `s.block.block5d@1/*` — always `Err`, see this file's module doc.
pub struct StlIntoBlock5d;

impl Deserializer<Block5dSnapshot> for StlIntoBlock5d {
    const FROM: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<Block5dSnapshot> {
        Err(IoError {
            message: "stl import not supported for a part kind definition: an STL solid carries triangles only — it has no kind identity, no handle/vortex/grip catalog and no compatibility rules to build a kind definition from".to_string(),
            diagnostics: Vec::new(),
        })
    }
}
