//! 🚪️ block5d → zip — foreign `Serializer<Block5dSnapshot>` on the framework's `io_mechanism`
//! channel: a REAL zip 2.0 container (stdio's own `encode_zip`, not a renamed text blob) holding two
//! members — `snapshot.block5d.semio` with this subset's authoritative `.semio` DSL snapshot text, and
//! `snapshot.json` with its rfc8259 rendition for readers that cannot parse `.semio`. Both members are
//! lossless and the sibling `📥️import` leaf reads the DSL member back first, so the hop is
//! `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/05/BLOCK-PLUGIN-END-TO-END, W3): this file used to hand back
//! `print_dsl(...).into_bytes()` — plain DSL text mislabelled as a zip archive, which no zip reader
//! could open.

use crate::artifacts::block5d::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::artifacts::block5d::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::artifacts::block5d::Block5dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::zip::io::encode_zip;
use semio_s_plugin_stdio::artifacts::zip::schema::snapshot::ZipEntry;
use semio_s_plugin_stdio::artifacts::zip::ZipSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 📄️ The container member carrying the authoritative `.semio` DSL snapshot text.
pub const ZIP_DSL_ENTRY: &str = "snapshot.block5d.semio";
/// 📄️ The container member carrying the rfc8259 rendition.
pub const ZIP_JSON_ENTRY: &str = "snapshot.json";

/// 🎒️ Builds the container this leaf writes — shared with the sibling `📥️import` leaf's tests.
pub fn archive_of(from: &Block5dSnapshot) -> ZipSnapshot {
    ZipSnapshot { entries: vec![ZipEntry { name: ZIP_DSL_ENTRY.to_string(), data: dsl_text(from).into_bytes() }, ZipEntry { name: ZIP_JSON_ENTRY.to_string(), data: json_text(from).into_bytes() }], ..ZipSnapshot::default() }
}

/// 🧵️ `s.block.block5d@1/*` → `s.stdio.zip@2.0/*`.
pub struct Block5dIntoZip;

impl Serializer<Block5dSnapshot> for Block5dIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Block5dSnapshot) -> IoResult<IoPayload> {
        let bytes = encode_zip(&archive_of(from)).map_err(|error| IoError { message: format!("block5d→zip: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
