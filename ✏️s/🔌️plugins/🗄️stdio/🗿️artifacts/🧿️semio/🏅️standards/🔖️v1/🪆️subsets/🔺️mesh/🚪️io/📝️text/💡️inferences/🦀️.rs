//! 📖️ s.stdio.semio.mesh.inference — the normative handcrafted text grammar for this facet. Inference
//! values are never authored via DSL text (they are always computed from a snapshot, never a
//! source of truth), so — unlike `📸️snapshot/📝️text`'s live `parse_dsl`/`print_dsl` pair — this
//! leaf declares the wire grammar only, matching the generic header/payload scaffold shape every
//! other representation leaf in this tree already uses for its own facet.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SemioMeshInferenceText = String;
//#endregion 🚚️Carrier

mod semantic_cache_codec {
use crate::standards::v1::subsets::mesh::schema::inferences::aabb::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive};
/// 🌉 Hand-written, not dual-derived: `SemioAabb`'s own field type `SemioPoint3` has already
/// dropped `serde` entirely in favor of `ToValue`/`FromValue` — a `#[derive(Serialize,
/// Deserialize)]` here would need it to grow serde back. `store::InferredField::Value` still
/// bounds on `Serialize + DeserializeOwned` (a genuine byte-cache codec, not a stale requirement),
/// so this bridges through the value this type's own `ToValue`/`FromValue` already compute — same
/// bridge shape as `🎛flattened-scene`'s `FlattenedNode` (see its docstring for the full rationale)
/// and, mirrored, the store's `🌉️SerdeValueBridge` (`🏪️store/🧬️schema/🧬️mutations/🦀️.rs`).
impl serde::Serialize for SemioAabb {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde_json::Value::from(&<Self as semio_framework_value::ToValue>::to_value(self)).serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for SemioAabb {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = serde_json::Value::deserialize(deserializer)?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(json)).map_err(serde::de::Error::custom)
    }
}
}
