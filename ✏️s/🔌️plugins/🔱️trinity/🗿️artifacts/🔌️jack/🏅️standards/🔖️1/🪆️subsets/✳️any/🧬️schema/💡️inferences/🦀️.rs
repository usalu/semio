//! 💡️ Jack inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors puzzle3d's own `💡️inferences/` (the pattern's exemplar): this file is the family-root
//! assembly (never mod's/includes the slug dirs directly — `🦀️.rs` is the sole mounting
//! mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `🧭topology/`, honestly derivable from this directed property port graph's own
//! `nodes`/`edges` — the closest fit from the family root's "workflow/dag-shaped" category, this
//! artifact being a directed node/edge graph with a `root_node_id`, not the "positioned" category
//! (its `x`/`y` are already explicit fields, not something to re-derive); and `🎛️flat-position/`,
//! ported from the former `Graph::recompute_derived` — each node's flattened `(u, v)` position is
//! honestly re-derivable from `nodes`/`edges`/`root_node_id` alone, so it moved out of the manifest's
//! former `flatPosition` `"derived"` node property into this family instead).

use crate::standards::v1::subsets::any::schema::content::{jack_content_for_handle, jack_content_identity};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack_graph_query::{infer_jack_flat_position, infer_jack_topology, JackGraphQuery};

//#region 🔖️Inference
/// 💡️ Everything inferable from a jack snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by `🧭topology/`; `flat_position`, backed by
/// `🎛️flat-position/`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.jack.inference")]
pub struct JackInference {
    #[derived]
    pub topology: JackTopology,
    #[derived]
    pub flat_position: JackFlatPosition,
}

impl JackInference {
    /// 💡️ Reads both inferences from the exact graph retained by the artifact composition.
    pub fn infer_from_graph(snapshot: &JackSnapshot, graph: &dyn JackGraphQuery) -> Result<Self, semio_framework_value::ValueError> {
        let selected = jack_content_identity(&snapshot.content);
        Ok(Self {
            topology: infer_jack_topology(selected, graph)?,
            flat_position: infer_jack_flat_position(selected, snapshot.root_node_id.as_deref(), graph)?,
        })
    }
}

impl protocol::Inference<JackSnapshot> for JackInference {
    fn infer(snapshot: &JackSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        let owner = jack_content_for_handle(&snapshot.content)?;
        Self::infer_from_graph(snapshot, owner.read())
    }
}

impl protocol::InferenceSpec<JackSnapshot> for JackInference {
    fn inference_schema_id() -> &'static str {
        "s.trinity.jack.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.trinity.jack.inference.topology", reads: &["content"] },
            protocol::InferenceFieldSpec { id: "s.trinity.jack.inference.flatPosition", reads: &["content", "root_node_id"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.trinity.jack.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `jack_artifact_schema_descriptor`'s registration.
pub fn jack_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.trinity.jack.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use super::flat_position::JackFlatPosition;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::topology::JackTopology;
//#endregion 🔁️Re-exports
