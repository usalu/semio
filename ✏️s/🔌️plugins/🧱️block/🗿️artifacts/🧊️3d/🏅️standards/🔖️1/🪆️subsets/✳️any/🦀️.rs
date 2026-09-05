//! 🪆️ Subset root for `s.block.block3d@1/*` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
//! `descriptor-prep`, following `🔱️trinity`'s `fleet-trinity-recipe`). Exports
//! `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::block3d`/`crate::viewer::block3d` stay mounted at the
//! plugin's top-level `editor`/`viewer` modules, not here.
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🖍️draw`/`🔱️trinity` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/05/BLOCK-PLUGIN-END-TO-END, W3 hand-authored the twelve typed
//! `Serializer<Block3dSnapshot>`/`Deserializer<Block3dSnapshot>` leaves that gap called for and relocated the
//! declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use crate::artifacts::block3d::standards::v1::subsets::any::{io, schema};
use crate::artifacts::block3d::BLOCK3D_DIALECT;
use crate::editor::block3d as editor;
use crate::viewer::block3d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::art_3d_hexagonal_cut_concrete_forest_left::source(), crate::examples::art_3d_nakagin_capsule::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::block3d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::BlockApps> {
    SubsetDeclaration {
        dialect: BLOCK3D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::block3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Block3dViewer, crate::BlockApps>(viewer::create_block3d_viewer()),
        editor: editor_surface::<editor::Block3dPlayApp, crate::BlockApps>(editor::create_block3d_app()),
        examples: examples(),
    }
}
