//! 🚪️ block5d → stl — foreign `Serializer<Block5dSnapshot>` that HONESTLY REFUSES.
//!
//! A `s.block.block5d` document is a part kind DEFINITION, not geometry or a raster. Its only geometry-bearing field is `representations[].mesh_url` — a URL pointing at an external mesh asset, never vertex/triangle data — and `grips` are anchor frames (angle/radius/position/direction), not a surface. Nothing in the schema an STL triangle soup could be built from, and nothing in an STL an identity/catalog/compatibility document could be built from.
//!
//! So this leaf returns a typed `IoError` naming the reason instead of an empty snapshot or an
//! invented solid. It stays REGISTERED on the `io_mechanism` channel at the weakest fidelity
//! (`IoFidelity::Lossy`, rank 0 — the router never prefers it over a real hop) so a caller that does
//! route here gets this reason back rather than a bare "no route" (see `📓️w3-io.md`).

use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf would write.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🧵️ `s.block.block5d@1/*` → `s.stdio.stl@ascii/*` — always `Err`, see this file's module doc.
pub struct Block5dIntoStl;

impl Serializer<Block5dSnapshot> for Block5dIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(_from: &Block5dSnapshot) -> IoResult<IoPayload> {
        Err(IoError {
            message: "stl export not supported for a part kind definition: the schema carries no triangle geometry — `representations[].mesh_url` is a URL pointing at an external mesh asset, never vertex data".to_string(),
            diagnostics: Vec::new(),
        })
    }
}
