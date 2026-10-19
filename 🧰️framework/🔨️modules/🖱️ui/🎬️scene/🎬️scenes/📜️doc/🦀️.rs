//! 📜️ The scene payload contract: the `SceneDoc` trait every product scene implements, the lane carriers and references, the pack wire macro and the shared value-codec primitives.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.
//!
//! The 15 product scene payloads — one per [`ui_contract::SurfaceKind`] wire tag — relocated
//! verbatim from `ui_wgpu`'s `🎯️targets/🧊️wgpu/🦀️component.rs` (ticket
//! 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet `scene-surface`). `ui_wgpu` now
//! re-exports these types instead of defining them, so every existing `ui_wgpu::wgpu::WorldXScene`
//! reference keeps compiling unchanged.
//!
//! Every field here is plain wasm-safe data (`String`/`f64`/`bool`/`Option`/`Vec`/nested plain
//! records) — scene lanes use first-party value/JSON codecs alongside `ui_contract` and `serde`. Two fields could not
//! move byte-identical for that reason: [`TableScene::drop_action_json`] (was
//! `Option<ActionDescriptor>`, a `ui_wgpu`-only type pulling `semio_framework_value::DslValue`/`Label`/`IconName`) and
//! [`NodeGraphOperatorChannelRecord::default_json`] (was `Option<serde_json::Value>`, which would
//! have been this crate's first dependency beyond `ui_contract`/`serde`). Every sibling field on
//! these 15 structs already uses the `_json: String` opaque-payload convention for exactly this
//! class of renderer-specific/arbitrary-shaped data (`selection_json`, `sort_json`, `camera_json`,
//! ...) — both renamed fields just apply that same established convention to their one outlier,
//! rather than inventing a shim or a new dependency.
//!
//! 🚫️async: E6 sync payload construction — the `.base()` constructors below are plain sync `fn` by
//! decree (ticket ruling E6), not the `pub async fn` they arrived as in `ui_wgpu`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_pack::{from_bytes, to_bytes, PackError};
use serde::{Deserialize, Serialize};

//#region 🔖️SceneDoc
/// 🆔️ The version axis every renderer/decoder gates on: `T::SCHEMA` is stamped into
/// `SurfaceProps::doc_schema` by [`crate::pack::encode`] and checked back by
/// [`crate::pack::decode`], which refuses to decode bytes stamped with any other schema.
pub trait SceneDoc: Clone + Serialize + serde::de::DeserializeOwned {
    /// 🏷️ `"<surface-kind-wire-tag>@<version>"` — the wire tag half agrees verbatim with the
    /// matching [`ui_contract::SurfaceKind`] variant's own `#[serde(rename = ...)]`, every kind
    /// kebab-case: the last camelCase holdout, `virtualFileSystem`, is now `virtual-file-system`
    /// in the contract, in this crate's schemas AND in the wgpu target's own `SurfaceKind`.
    const SCHEMA: &'static str;

    /// 🧳️ Encodes the stable opaque scene payload independently of its human-readable JSON shape.
    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(self)
    }

    /// 🧳️ Decodes the stable opaque scene payload independently of its human-readable JSON shape.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes(bytes)
    }

    /// 🚚️ Splits this doc into the SPINE that still rides inside the fixed-capacity
    /// `SurfaceProps.doc` and the payload lanes that ride outside it, each as its own retained,
    /// individually paged text carrier keyed by [`SceneLanePayload::key`].
    ///
    /// `SurfaceDoc.bytes` is `ui_contract::UiFixedBytes` — a hard `UI_FIXED_BYTES` (32 KiB) ceiling
    /// that cannot page — so any scene whose payload is proportional to its document outgrows one
    /// doc as soon as the document is large enough (measured: a 57 281-byte world-3d scene refused
    /// admission outright at `scene-surface.encode`). A doc that returns lanes here keeps only its
    /// per-frame spine inside that ceiling; everything else is bounded by the number of carrier
    /// pages, not by 32 KiB, and re-publishes per lane instead of per scene.
    ///
    /// The default is "no lanes" — a scene whose payload is a bounded per-frame descriptor
    /// (`Canvas2dScene`, `IconRenderScene`, …) has nothing to gain and stays one atomic doc.
    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>)
    where
        Self: Sized,
    {
        (self.clone(), Vec::new())
    }

    /// 🚚️ Re-attaches one lane's payload to a spine decoded from the surface doc — the exact inverse
    /// of [`SceneDoc::split_lanes`]. Returns `false` for a key this doc does not own.
    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let _ = (key, payload);
        false
    }
}

/// 🚚️ One scene lane on its way out of a [`SceneDoc`]: the reserved carrier key its retained subtree
/// is rooted at, and the payload text that subtree's depth-first leaf concatenation reproduces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneLanePayload {
    pub key: &'static str,
    pub payload: String,
}

#[macro_export]
macro_rules! scene_pack_wire {
    ($wire:ident, $scene:ident { $($(#[$attribute:meta])* $field:ident: $ty:ty),+ $(,)? }) => {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct $wire {
            $($(#[$attribute])* $field: $ty),+
        }

        impl From<&$scene> for $wire {
            fn from(scene: &$scene) -> Self {
                Self { $($field: scene.$field.clone()),+ }
            }
        }

        impl From<$wire> for $scene {
            fn from(wire: $wire) -> Self {
                Self { $($field: wire.$field),+ }
            }
        }
    };
}
//#endregion 🔖️SceneDoc

//#region 🔖️ValueCodecHelpers
/// 🧰️ Shared object-entry primitives every hand-written `ToValue`/`FromValue` impl below builds on
/// — `#[derive(ToValue, FromValue)]` hardcodes `::semio_framework_os_kernel::…` paths (see this
/// crate's `Cargo.toml` docstring), which this wasm-safe, os-kernel-free crate must never depend on,
/// so every scene/record type is encoded/decoded by hand against `protocol::value::` directly.
/// Mirrors serde semantics exactly: an `Option<T>` field decodes to `None` when its key is absent
/// (matching serde's implicit optionality for `Option` fields, independent of whether
/// `#[serde(default)]` is spelled out), a field with an explicit `#[serde(default = ...)]` falls
/// back to that default when absent, and every other field is required.
pub fn value_field(entries: &[(String, DslValue)], key: &str) -> Option<DslValue> {
    entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

pub fn value_required(entries: &[(String, DslValue)], key: &str) -> Result<DslValue, ValueError> {
    value_field(entries, key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")))
}

/// 🌱️ Decodes a required (non-defaulted, non-`Option`) field.
pub fn value_decode<T: FromValue>(entries: &[(String, DslValue)], key: &str) -> Result<T, ValueError> {
    T::from_value(value_required(entries, key)?).map_err(|error| error.under(key))
}

/// 🌱️ Decodes an `Option<T>` field — a missing key or an explicit `null` both decode to `None`.
pub fn value_decode_option<T: FromValue>(entries: &[(String, DslValue)], key: &str) -> Result<Option<T>, ValueError> {
    match value_field(entries, key) {
        None | Some(DslValue::Null) => Ok(None),
        Some(value) => T::from_value(value).map(Some).map_err(|error| error.under(key)),
    }
}

/// 🌱️ Decodes a field with a `#[serde(default = ...)]` fallback — a missing key calls `default`.
pub fn value_decode_default<T: FromValue>(entries: &[(String, DslValue)], key: &str, default: impl FnOnce() -> T) -> Result<T, ValueError> {
    match value_field(entries, key) {
        None => Ok(default()),
        Some(value) => T::from_value(value).map_err(|error| error.under(key)),
    }
}

/// 🌱️ Pushes a field unconditionally — the twin of a plain (non-`skip_serializing_if`) struct field.
pub fn value_push<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &T) {
    entries.push((key.to_string(), value.to_value()));
}

/// 🌱️ Pushes an `Option<T>` field only when `Some` — mirrors `#[serde(skip_serializing_if =
/// "Option::is_none")]`.
pub fn value_push_option<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &Option<T>) {
    if let Some(value) = value {
        entries.push((key.to_string(), value.to_value()));
    }
}

/// 🌱️ Pushes a `Vec<T>` field only when non-empty — mirrors `#[serde(skip_serializing_if =
/// "Vec::is_empty")]`.
pub fn value_push_if_nonempty<T: ToValue>(entries: &mut Vec<(String, DslValue)>, key: &str, value: &[T]) {
    if !value.is_empty() {
        entries.push((key.to_string(), DslValue::Array(value.iter().map(ToValue::to_value).collect())));
    }
}
//#endregion 🔖️ValueCodecHelpers

//#region 🔖️SceneLane
/// 🚚️ One entry of a scene spine's lane manifest ([`World3dScene::lanes`], [`Canvas2dScene::lanes`]) — see
/// [`World3dScene::lanes`]' doc.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLaneRef {
    pub lane: String,
    pub bytes: u32,
    pub hash: String,
}

impl ToValue for SceneLaneRef {
    fn to_value(&self) -> DslValue {
        DslValue::object([("lane".to_string(), self.lane.to_value()), ("bytes".to_string(), self.bytes.to_value()), ("hash".to_string(), self.hash.to_value())])
    }
}

impl FromValue for SceneLaneRef {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self { lane: value_decode(&entries, "lane")?, bytes: value_decode(&entries, "bytes")?, hash: value_decode(&entries, "hash")? })
    }
}

/// 🔢️ FNV-1a/64 of a lane payload, lowercase hex — the ONE thing that makes a scene spine differ
/// when a lane's content differs, and stay byte-identical when it does not. Hand-rolled rather than
/// pulled from a hashing crate because this crate depends on nothing beyond `ui_contract`/`serde`,
/// and because the digest is part of a wire payload two languages have to agree on.
// 🚫️async: E6 sync payload construction — see this module's own header.
pub fn scene_lane_hash(payload: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in payload.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub fn deserialize_non_empty_interaction_domain_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty() {
        return Err(serde::de::Error::custom("interaction domain fields must be non-empty"));
    }
    Ok(value)
}
//#endregion 🔖️SceneLane

#[cfg(test)]
#[path = "../../🧪️tests/🔬️doc-unit/🦀️.rs"]
mod tests;
