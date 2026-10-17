//! 🔺️ GltfDiff — handcrafted sparse per-field diff for the fully typed glTF 2.0 document (ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4). **DELETES the prior
//! `snapshot: Option<GltfSnapshot>` full-replace slot wholesale.** Index-keyed collection triples
//! for every one of the 14 top-level arrays (`scenes`/`nodes`/`meshes`/`accessors`/`bufferViews`/
//! `buffers`/`buffer` bytes/`materials`/`textures`/`images`/`samplers`/`skins`/`animations`/
//! `cameras`) plus sparse scalar slots for `asset`/`scene`/`extensionsUsed`/`extensionsRequired`/
//! `extensions`/`extras`/`sourceForm`. STRONG entities (scenes, nodes, meshes, accessors,
//! materials, buffers -- the recipe's explicitly prioritized highest-value arrays) get their own
//! per-field diff struct via the local [`ItemDiff`] trait; the remaining WEAK entities (bufferView,
//! buffer bytes, texture, image, sampler, skin, animation, camera -- real, undifferentiated glTF
//! objects whose "diff" is legitimately their whole new value per the recipe's strong/weak split)
//! reuse the SAME generic [`GltfCollectionDiff<T, D>`] wrapper via the blanket `ItemDiff<T> for T`
//! impl below (`D = T`). This is the general form of gif 89a's hand-duplicated
//! frames/comments/appExtensions triples -- one real generic collection algebra, instantiated per
//! entity, not a shortcut around per-entity semantics.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>, Vec<(usize, D)>);

use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessor, GltfAlphaMode, GltfAnimation, GltfAnimationChannel, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfCamera, GltfCameraProjection, GltfImage, GltfJson, GltfMaterial, GltfMesh, GltfMorphTarget, GltfNode, GltfNormalTextureInfo, GltfOcclusionTextureInfo, GltfOrthographic, GltfPbrMetallicRoughness, GltfPrimitive, GltfSampler, GltfScene, GltfSkin, GltfSourceForm, GltfSparseAccessor, GltfTexture, GltfTextureInfo};
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
#[cfg(test)] use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfPerspective, GltfSparseIndices, GltfSparseValues};
//#region 🔖️JsonPresence
/// 🈳️ Wire form for a diff slot holding `Option<Option<GltfJson>>`. Three states must stay
/// DISTINCT on the wire: "unchanged" (outer `None` — the key is absent, `skip_serializing_if`),
/// "cleared" (`Some(None)`) and "set to a JSON value" (`Some(Some(v))`). A bare `null` cannot
/// carry that, because [`GltfJson`] has its own `Null` variant: glTF 2.0 §3.2's `extras`/
/// `extensions` may legitimately hold JSON `null`, so `Some(Some(GltfJson::Null))` and
/// `Some(None)` both printed `null` and `Option`'s own `FromValue` read either one back as the
/// outer `None` — the change vanished from every JSON wire round trip (`GltfDiff` -> text ->
/// `GltfDiff`), taking the inverse mutation with it. The presence tag is the same `state`/`value`
/// vocabulary `GltfDataPresence` (the change-extras mutation payload) already spells.
/// https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
pub mod json_presence {
    use super::GltfJson;

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn to_value(value: &Option<Option<GltfJson>>) -> semio_framework_value::DslValue {
        match value {
            None => semio_framework_value::DslValue::Null,
            Some(None) => semio_framework_value::DslValue::object([("state".to_string(), semio_framework_value::DslValue::String("absent".to_string()))]),
            Some(Some(inner)) => semio_framework_value::DslValue::object([("state".to_string(), semio_framework_value::DslValue::String("present".to_string())), ("value".to_string(), semio_framework_value::ToValue::to_value(inner))]),
        }
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<Option<Option<GltfJson>>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = &value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a presence object {state,value}"));
        };
        match entries.iter().find(|(key, _)| key == "state").map(|(_, state)| state) {
            Some(semio_framework_value::DslValue::String(state)) if state == "absent" => Ok(Some(None)),
            Some(semio_framework_value::DslValue::String(state)) if state == "present" => {
                let inner = entries.iter().find(|(key, _)| key == "value").map(|(_, inner)| inner.clone()).unwrap_or(semio_framework_value::DslValue::Null);
                Ok(Some(Some(<GltfJson as semio_framework_value::FromValue>::from_value(inner)?)))
            }
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "presence object needs state = \"absent\" | \"present\"")),
        }
    }
}
//#endregion 🔖️JsonPresence

/// 🕳️ Tri-state decode of every non-JSON `Option<Option<T>>` slot (the shape the value derive's docs call
/// `deserialize_double_option`): the key is skipped when unchanged (`None`), and a PRESENT `null` is the clear
/// `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into — so a restore diff that
/// clears `scale`/`mesh`/`name` survives every wire round trip. JSON slots use [`json_presence`] instead, since a
/// glTF JSON value may itself be `null`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

// 🧬️ `GltfDocument` is only reached through `mod tests`' `use super::*;` glob (its non-test uses
// below are all inside `#[cfg(test)]`), so — like the reactor/puzzle wasm-only imports elsewhere in
// this ticket — it must be gated to its actual consumer or it warns unused on the plain `lib` build.
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot::GltfDocument;
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use protocol::MutationDiff;

#[macro_use]
#[path = "🧮️collection-algebra/🦀️.rs"]
mod collection_algebra;
pub use collection_algebra::*;

#[path = "📐️structural-element-diffs/🦀️.rs"]
mod structural_element_diffs;
pub use structural_element_diffs::*;

#[path = "🎨️resource-element-diffs/🦀️.rs"]
mod resource_element_diffs;
pub use resource_element_diffs::*;

//#region 🔖️HandcraftedDiffCodec
// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `GltfDiff` — CONFIRMED (not just per the
// recon sweep's guess) by two independent real `cargo check -p semio-s-plugin-stdio --lib`
// failures with `#[derive(dsl::DslDiff)]` temporarily added to this struct (captured verbatim in
// `f6-gltf-recon-check1.txt`/`f6-gltf-diff-derive-check1.txt` in the ticket folder, then reverted
// — 77 `E0277` errors): (1) every one of the 14 top-level arrays is typed through the GENERIC
// [`GltfCollectionDiff<T, D>`] wrapper (e.g. `GltfCollectionDiff<GltfScene, GltfSceneDiff>`,
// `GltfCollectionDiff<GltfCamera, GltfCamera>`) — `DslField` has no blanket impl for ANY
// user-defined generic struct (only `Vec<T>`/`BTreeMap<String,T>`/`[T;N]` have such blanket impls
// in the `dsl` crate), so the derive fails on EVERY collection field regardless of enum/tri-state
// content, a blocker beyond both 3a and 3b from `f6-recon-report.md` §3; (2) `Option<GltfJson>` —
// `GltfJson` is a real data-carrying enum (`Null`/`Bool`/`Number`/`String`/`Array`/`Object`), the
// artifact's OWN local JSON value type for `extras`/`extensions` (F4's `GltfJson`, confirmed here
// to in fact be diff-reachable via 20+ `Option<Option<GltfJson>>` fields, resolving the recon's
// open question at classification row #23: 0 enums does NOT hold) — same 3a shape as `SvgNodeDiff`/
// `XmlNode`. `GltfCameraProjection` (`Perspective`/`Orthographic`, inside `GltfCamera`, itself a
// WEAK-collection item type) is a second data-carrying enum in the tree, reachable via the
// `cameras` field. Every `Option<Option<T>>` tri-state field (42 per the recon sweep, e.g.
// `GltfNodeDiff::mesh`/`matrix`/`translation`, `GltfAccessorDiff::sparse`, `GltfMaterialDiff::
// pbr_metallic_roughness`) hits 3b independently on top. `#[derive(dsl::DslOps)]` on
// `GltfMutation` (🧬️mutations/component.rs) fails the same way for the identical structural
// reason (33 `E0277` errors, `f6-gltf-mutation-derive-check1.txt`): `SetSnapshot{snapshot:
// GltfSnapshot}` (since removed) recursively requires `DslField` on `GltfAsset`/`GltfScene`/`GltfNode`/.../
// `GltfSnapshot` itself, none of which are `DslRecord`-derived, and even if they all were, the
// `GltfJson`/`GltfCameraProjection` enums nested inside would still block it (3a).
//
// Grammar follows the same style as `GifDiff`/`SvgDiff`'s hand-rolled codecs (bracket-depth-aware
// `split_top_level`, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, tag-prefix for
// data-carrying enums) — this file re-derives its own copies of the small primitives (no shared
// "hand-roll helpers" module exists yet, per `f6-recon-report.md` §5's "known duplication" note).
// Given the sheer breadth of this artifact's fully-typed 2.0 model (by far the largest hand-roll
// in the F6 program per the recon's own sizing), the value codecs below are grouped by field
// GROUP (asset/scene/node; mesh/accessor/material; buffer family; texture/image/sampler/skin;
// animation; camera) rather than one monolithic function, per the recon's own suggested structure.
//#region 🔖️Primitives

//#endregion 🔖️Primitives

//#region 🔖️ScalarCodecs

//#endregion 🔖️ScalarCodecs

//#region 🔖️GltfJsonCodec

//#endregion 🔖️GltfJsonCodec

//#region 🔖️UnitEnumCodecs

//#endregion 🔖️UnitEnumCodecs

//#region 🔖️AssetSceneNodeGroupCodecs

//#endregion 🔖️AssetSceneNodeGroupCodecs

//#region 🔖️MeshAccessorMaterialGroupCodecs

//#endregion 🔖️MeshAccessorMaterialGroupCodecs

//#region 🔖️BufferGroupCodecs

//#endregion 🔖️BufferGroupCodecs

//#region 🔖️TextureImageSamplerSkinGroupCodecs

//#endregion 🔖️TextureImageSamplerSkinGroupCodecs

//#region 🔖️AnimationGroupCodecs

//#endregion 🔖️AnimationGroupCodecs

//#region 🔖️CameraGroupCodecs

//#endregion 🔖️CameraGroupCodecs

//#region 🔖️GenericCollectionCodec

//#endregion 🔖️GenericCollectionCodec

//#region 🔖️RealBinaryPrimitives

//#endregion 🔖️RealBinaryPrimitives

//#region 🔖️RealBinaryJsonCodec

//#endregion 🔖️RealBinaryJsonCodec

//#region 🔖️RealBinaryUnitEnumCodecs

//#endregion 🔖️RealBinaryUnitEnumCodecs

//#region 🔖️RealBinaryAssetSceneNodeGroupCodecs

//#endregion 🔖️RealBinaryAssetSceneNodeGroupCodecs

//#region 🔖️RealBinaryMeshAccessorMaterialGroupCodecs

//#endregion 🔖️RealBinaryMeshAccessorMaterialGroupCodecs

//#region 🔖️RealBinaryBufferGroupCodecs

//#endregion 🔖️RealBinaryBufferGroupCodecs

//#region 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs

//#endregion 🔖️RealBinaryTextureImageSamplerSkinGroupCodecs

//#region 🔖️RealBinaryAnimationGroupCodecs

//#endregion 🔖️RealBinaryAnimationGroupCodecs

//#region 🔖️RealBinaryCameraGroupCodecs

//#endregion 🔖️RealBinaryCameraGroupCodecs

//#region 🔖️RealBinaryGenericCollectionCodec

//#endregion 🔖️RealBinaryGenericCollectionCodec

//#region 🔖️TopLevel

//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🧪️HandcraftedDiffCodecTests

//#endregion 🧪️HandcraftedDiffCodecTests
