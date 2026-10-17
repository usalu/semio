//! 💡️ PlyInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`, honestly derivable from
//! `elements` alone — PLY's own convention names the vertex-carrying element `"vertex"` with
//! `x`/`y`/`z` scalar properties, and the face-carrying element `"face"`).

use crate::PlySnapshot;
use framework_schema::ArtifactSchema;

use super::bounds::compute_ply_bounds;
//#region 🔖️Inference
/// 💡️ Everything inferable from a ply snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.inference")]
pub struct PlyInference {
    #[derived]
    pub bounds: PlyBounds,
}

impl protocol::Inference<PlySnapshot> for PlyInference {
    fn infer(snapshot: &PlySnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_ply_bounds(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `PlySnapshot::default()`'s `elements` ever stops being empty.
impl Default for PlyInference {
    fn default() -> Self {
        let snapshot = &PlySnapshot::default();

        Self { bounds: compute_ply_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<PlySnapshot> for PlyInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.ply.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.ply.inference.bounds", reads: &["elements"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.ply.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `ply_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ply_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.ply.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
    }
}
//#endregion 🔖️Descriptor

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::PlyBounds;
//#endregion 🔁️Re-exports

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts
