//! 🆔️ Semantic surface, node and revision identity with monotonic allocation.
use serde::{Deserialize,Serialize};
use semio_framework_value_derive::{FromValue,ToValue};
use semio_framework_ui_contract_text::UiText;
//#region 🆔️Ids
/// 🪧️ A render surface address — today's dotted strings, e.g. `"note.play.navigator"`.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct SurfaceId(pub UiText);

impl AsRef<str> for SurfaceId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for SurfaceId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        UiText::try_from_string(value).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for SurfaceId {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        UiText::try_from_str(value).map(Self).ok_or(value)
    }
}

/// 🔢️ A node's identity within one [`SurfaceId`] — monotonic per surface, never reused, so a stale
/// reference to a removed node is always distinguishable from a fresh node at the same tree position.
///
/// The TypeScript type is pinned to `number`, not a Rust-centric `bigint` projection for `u64`: serde writes
/// this as a plain JSON number, so `JSON.parse` hands JavaScript a `number` at runtime and a `bigint`
/// declaration would be a type that never actually occurs. Ids are per-surface and monotonic, so the
/// 2^53 exact-integer ceiling is unreachable in practice — a surface would have to mint nine
/// quadrillion nodes to reach it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct UiNodeId(pub u64);

/// 🔢️ A snapshot's wire revision — advances by one per accepted [`UiPatch`]; a patch whose
/// `base_revision` does not match the receiver's current revision is rejected whole.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct UiRevision(pub u64);

impl UiRevision {
    /// ⏭️ The next revision after this one.
    pub fn try_next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

/// 🏭️ Per-surface monotonic [`UiNodeId`] source — the only legitimate way to mint one. Never resets
/// and never yields the same id twice, so an id is a stable identity for the lifetime of its surface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiNodeIdAllocator(u64);

impl UiNodeIdAllocator {
    /// 🏭️ Mints the next unused [`UiNodeId`] for this surface.
    pub fn try_allocate(&mut self) -> Option<UiNodeId> {
        let id = UiNodeId(self.0);
        self.0 = self.0.checked_add(1)?;
        Some(id)
    }
}
//#endregion 🆔️Ids
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
