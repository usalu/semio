//! 🔮️ Mutation oracle for this subset — the kinds of `GltfMutation` (`../🧬️schema/🧬️mutations/
//! 🦀️.rs`) performed by an independent GLB container codec plus generic JSON manipulation, so the
//! subject's own mutation has a second producer to be compared against instead of being checked
//! against its own reading. It is a `cross-semio-implementation`: the JSON tokenizer is third-party,
//! the glTF semantics below are this repository's own second reading of the specification.
//!
//! **Why `json` (json-rust), not the `gltf` crate (1.4.1, MIT):** `gltf` is a credible, actively
//! maintained reader and IS already production-reachable in this repository — but confirmed
//! genuinely independent of this subset's own codec first. `crate::schema::
//! snapshot::🦀️.rs`'s `GltfSnapshot`/`GltfDocument`/`GltfJson` never names `gltf::`
//! anywhere (no `impl From<gltf::…>`, no import), and `decode_glb`/`encode_glb`/`parse_gltf_document`
//! (`../🚪️io/🦀️.rs`) are hand-rolled over `serde_json` alone. Every real `gltf::` call site
//! in this repository lives in `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`
//! (`mesh_to_glb`/`mesh_from_glb`/`GlbExporter`/`GlbImporter`, byte-in/byte-out, no `gltf::` type
//! crosses that boundary) — reached from `semio-s-plugin-stdio` only through the unrelated BREP/DWG
//! mesh-IO codecs, never from this artifact's own tree. That is a small, nameable production surface
//! exactly like the `image`/`png` `productionDebt` precedent, so registering `gltf` here WOULD have
//! been legitimate. It was not registered anyway: linking it needs a `Cargo.toml` edit this ticket
//! must not make itself, and `json` 0.12 is already linked (`oracles = […, "dep:json", …]`,
//! `../../../../../../🔮️oracles/📦️packages/🦀️rust/Cargo.toml`) and already proven independent for
//! `stdio.json`'s own oracle (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/
//! 🪆️subsets/♾️any/🦀️oracle.rs`) — it appears nowhere in this repository's production
//! dependency graph. This subset's own production codec also uses `serde_json` (see above), which
//! rules `serde_json` out as an oracle for the same reason it was ruled out for `stdio.json`.
//!
//! `json` is domain-BLIND (no glTF schema awareness at all, unlike `gltf`), so every mutation's
//! actual semantics — index bounds, cycle rejection, duplicate-root rejection, alphaMode enum
//! validity, every top-level family's reference remapping — are reimplemented from scratch below, independently of
//! `../🧬️schema/🧬️mutations/*/🧬️operation/🦀️.rs`, operating on a hand-parsed GLB container
//! and a plain `json::JsonValue` document tree rather than this subset's own `GltfSnapshot`.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — `GLTF_MUTATION_LEAF_DESCRIPTORS`, the real vocabulary.

use semio_repo_test_host::Json;

#[path = "🧰️glb-container-and-json-helpers/🦀️.rs"]
mod glb_container_and_json_helpers;
use glb_container_and_json_helpers::*;

#[path = "🎬️scene-node-and-collection-kinds/🦀️.rs"]
mod scene_node_and_collection_kinds;
pub use scene_node_and_collection_kinds::*;

#[path = "🎨️asset-and-material-resource-kinds/🦀️.rs"]
mod asset_and_material_resource_kinds;
pub use asset_and_material_resource_kinds::*;

#[path = "🏗️structure-kinds/🦀️.rs"]
mod structure_kinds;
pub use structure_kinds::*;

#[path = "🧭️dispatch-and-projection/🦀️.rs"]
mod dispatch_and_projection;
pub use dispatch_and_projection::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
