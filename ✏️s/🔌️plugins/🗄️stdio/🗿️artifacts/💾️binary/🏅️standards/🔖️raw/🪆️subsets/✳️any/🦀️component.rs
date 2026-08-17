//! 🪆️ Subset root for `s.stdio.binary@raw/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM, W2-P pilot — first real subset migration onto the new declaration tree, design.md
//! §1/§2). Exports `subset() -> SubsetDeclaration`. Reuses the existing `🧬️schema`/`🚪️io`/
//! `👁️viewer`/`✏️editor`/`📚️examples` children unchanged (mounted elsewhere, via `📦️glue.rs`) —
//! this file only ASSEMBLES their declaration-shaped surface; it does not remount them (the
//! `#[path]` mounts stay `📦️glue.rs`'s job per design.md §1's "owner mounts children" rule, and
//! `📦️glue.rs` is outside this file's own directory anyway).

use crate::artifacts::binary::standards::v_raw::subsets::any::{io, schema};
use crate::editor::binary as editor;
use crate::viewer::binary as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, ExampleSource, StandardId, SubsetId};
use std::sync::OnceLock;

/// 🎯️ `s.stdio.binary@raw/*` — `CARRIER_BINARY` in `semio_framework::io_schema`.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::binary::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::binary_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard raw / subset any`'s complete declaration — the carrier pilot: `io.entries` is
/// empty by the carrier law (see `🚪️io/🦀️component.rs`'s `io()` doc comment), `examples` carries
/// the one `demo` example, `inferences` carries the `extent` inference descriptor.
pub fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::binary_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::BinaryViewer>(viewer::create_binary_viewer()),
        editor: editor_surface::<editor::BinaryEditor>(editor::create_binary_editor()),
        examples: examples(),
    }
}
