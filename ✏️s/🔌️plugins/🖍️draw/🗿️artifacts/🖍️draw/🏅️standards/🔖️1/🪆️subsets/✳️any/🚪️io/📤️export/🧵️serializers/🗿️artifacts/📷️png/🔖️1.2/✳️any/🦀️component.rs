//! 🚪️ draw -> png — foreign `Serializer<DrawSnapshot>` (design.md §3). Honest not-yet-implemented
//! stub: the pre-migration free function this replaces printed the artifact's OWN `.draw` DSL text
//! and mislabeled it as PNG bytes — a real correctness bug. Fixed here by refusing honestly instead
//! of perpetuating the mislabeled payload; real raster rendering is out of scope for this cutover.

use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct DrawIntoPng;

impl Serializer<DrawSnapshot> for DrawIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(_from: &DrawSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "DrawIntoPng: PNG export is not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
