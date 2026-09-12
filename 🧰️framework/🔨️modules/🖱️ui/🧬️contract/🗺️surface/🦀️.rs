//! @emoji 🗺️ `SurfaceProps` — embedded product surfaces with an opaque pack-encoded payload.
//!
//! Final shape (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` 📌️important.md, packet
//! `ui-w4-core`, item 4 — replaces this file's own former `⚠️ SCAFFOLD` header and the placement-
//! carrying shape `contract-doc`/`contract-layout` left behind wholesale, per that ruling):
//!
//! ```text
//! pub struct SurfaceProps {
//!     pub kind: SurfaceKind,
//!     pub doc_schema: UiText,      // "<kind>@<version>", e.g. "world3d@1"
//!     pub doc: SurfaceDoc,         // opaque pack-encoded bytes; this crate NEVER parses them
//!     pub bindings: Vec<ActionBinding>,
//! }
//! ```
//!
//! `SurfaceProps` names WHAT is embedded (`kind` plus a `doc_schema`-tagged opaque payload) and WHICH
//! surface-level intents it can fire (`bindings`) — never WHERE it sits or which product instance owns
//! it. The scaffold's `surface_id`/`controller_id`/`pane_id`/`binding_id`/`domain_id`/
//! `domain_granularity_id` fields are gone: placement is `semio-framework-ui-render`'s own
//! `SurfacePlacement`/`AnySurface` concern (that crate's `🦀️surface.rs` already imports only
//! [`SurfaceId`]/[`SurfaceKind`] from this crate, never those dropped fields), and a per-instance
//! identity (which pane, which binding, which domain granularity) is exactly the kind of
//! product-specific concern `doc`'s own opaque, schema-tagged bytes now carry — decoded only by the
//! `🎬️scene` crate that actually knows `doc_schema`'s shape, never by this dependency-free contract.
//!
//! `doc_schema` is `"<kind>@<version>"` (e.g. `"world3d@1"`) — the axis a renderer gates its own
//! per-kind decode logic on. This crate NEVER parses `doc.bytes`: [`parse_doc_schema`] only splits the
//! schema STRING itself, never touches the payload, and never panics — a malformed or unrecognised
//! schema is a typed [`SurfaceSchemaFault`], not a crash.
//!
//! **The contract-side rule this file exists to guarantee: an unrecognised `doc_schema` must never
//! reject the surrounding [`crate::UiPatch`] or panic reconciliation.** Neither `🛡️limits.rs`'s
//! `component_text_bytes` nor its `component_is_finite` special-cases `Component::Surface` in any way
//! that could reject on an unrecognised schema (both fall through their catch-all arm for it), and
//! `validate_snapshot`/`apply_patch` impose zero constraint on `doc_schema` content — see this file's
//! own tests. The renderer that DOES recognise schemas (a sibling crate, out of this packet's scope) is
//! the one that actually renders a placeholder and logs the fault; this crate's only job is to make
//! sure nothing here stops it from doing so.
//!
//! `doc`'s bytes diff as an opaque blob: [`SurfaceProps`] derives plain structural [`PartialEq`], so a
//! changed byte anywhere inside `doc.bytes` makes the whole `SurfaceProps` unequal to what it was —
//! exactly one `component` change as far as `semio-framework-ui-runtime`'s `SurfaceReconciler` is
//! concerned (`Component::Surface` is diffed as part of the same `component` field group every other
//! `Component` variant is), which folds into a single `SetComponent`/`Upsert` op. No separate
//! `SetSurface` op exists or is needed — a changed scene is exactly one op.
//!
//! **Signature the `🎬️scene` crate must build its typed `encode`/`decode` helpers against** (that crate
//! depends on this one, never the reverse):
//!
//! ```text
//! fn encode<T: Serialize>(kind: SurfaceKind, version: u32, value: &T) -> SurfaceProps;
//! fn decode<T: DeserializeOwned>(props: &SurfaceProps) -> Result<T, DecodeFault>;
//! ```
//!
//! where `encode` is expected to set `doc_schema` to `format!("{kind_slug}@{version}")` — the exact
//! string [`parse_doc_schema`] splits back apart — and `decode` is expected to call
//! [`parse_doc_schema`] first and treat a [`SurfaceSchemaFault`] (or a recognised-but-unimplemented
//! kind/version pair) as its own `DecodeFault::UnknownSchema`-shaped case, never a panic. The scene
//! crate owns `DecodeFault`, the per-kind `kind_slug` strings, and the actual pack encode/decode of
//! `T`; this crate defines only the opaque envelope and the schema-string convention.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. `SurfaceProps` is the
// deliberate exception — `bindings: UiNodeBindings` embeds `crate::ActionBinding`, which embeds
// `UiValue`, the DslValue-free exception (see its docstring in `🎬️action.rs`).
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Surface

/// 🖼️ The 15 embeddable product surface kinds. Ported from the wgpu target's `SurfaceKind`, with its
/// one real wire inconsistency FIXED rather than preserved: `VirtualFileSystem` was
/// `"virtualFileSystem"` (camelCase) where every sibling is kebab-case. This program has no back-compat
/// obligation (greenfield, no users, no legacy support — root `CLAUDE.md`), so the rename is made here
/// deliberately rather than carried forward as debt for "a later packet to make on purpose".
///
/// **Rename: `"virtualFileSystem"` → `"virtual-file-system"`.**
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::protocol::value")]
pub enum SurfaceKind {
    /// 🖌️ The plainest surface a host can always satisfy, so a `SurfaceProps` built from defaults
    /// names something renderable rather than a kind no backend registered.
    #[default]
    #[serde(rename = "canvas-2d")]
    #[value(rename = "canvas-2d")]
    Canvas2d,
    #[serde(rename = "world-3d")]
    #[value(rename = "world-3d")]
    World3d,
    #[serde(rename = "node-graph")]
    #[value(rename = "node-graph")]
    NodeGraph,
    #[serde(rename = "text-editor")]
    #[value(rename = "text-editor")]
    TextEditor,
    #[serde(rename = "table")]
    #[value(rename = "table")]
    Table,
    #[serde(rename = "paint-2d")]
    #[value(rename = "paint-2d")]
    Paint2d,
    #[serde(rename = "virtual-file-system")]
    #[value(rename = "virtual-file-system")]
    VirtualFileSystem,
    #[serde(rename = "tiled-map")]
    #[value(rename = "tiled-map")]
    TiledMap,
    #[serde(rename = "board-2d")]
    #[value(rename = "board-2d")]
    Board2d,
    #[serde(rename = "icon-render")]
    #[value(rename = "icon-render")]
    IconRender,
    #[serde(rename = "ink-canvas")]
    #[value(rename = "ink-canvas")]
    InkCanvas,
    #[serde(rename = "graph-timeline")]
    #[value(rename = "graph-timeline")]
    GraphTimeline,
    #[serde(rename = "block-list")]
    #[value(rename = "block-list")]
    BlockList,
    #[serde(rename = "diff-view")]
    #[value(rename = "diff-view")]
    DiffView,
    #[serde(rename = "event-feed")]
    #[value(rename = "event-feed")]
    EventFeed,
}

/// 📦️ An opaque, pack-encoded payload. The contract never parses it — `doc_schema` on the owning
/// [`SurfaceProps`] names the version-specific shape (e.g. `"world3d@1"`) that some other layer (the
/// `🎬️scene` crate) knows how to decode.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::protocol::value")]
pub struct SurfaceDoc {
    pub bytes: crate::UiFixedBytes,
}

/// 🗺️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
/// `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
/// scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️.rs` in a
/// later packet, never into this dependency-free contract crate. See this file's own module doc for
/// the exact reasoning behind each field (and each field the scaffold this replaces used to carry but
/// no longer does).
// 🌱️ No `ToValue`/`FromValue` here: `bindings: UiNodeBindings` embeds `crate::ActionBinding`, the
// deliberate DslValue-free exception — see its own note above.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceProps {
    pub kind: SurfaceKind,
    /// 🏷️ `"<kind>@<version>"`, e.g. `"world3d@1"` — the axis a renderer gates its own per-kind decode
    /// logic on. Never validated against `kind` by this crate (see [`parse_doc_schema`]); a mismatch
    /// between the two is a `🎬️scene`-crate-level authoring bug, not a contract violation.
    pub doc_schema: crate::UiText,
    pub doc: SurfaceDoc,
    /// 🔗️ Surface-level intents — bindings that fire against the surface itself (e.g. a "focus"/
    /// "reset view" action a host chrome offers around the embedded content), as opposed to intents the
    /// embedded content's own scene graph interprets internally via `doc`'s opaque bytes.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub bindings: crate::UiNodeBindings,
}

impl SurfaceProps {
    pub fn credited_clone(&self) -> Option<Self> {
        let mut bindings = crate::UiNodeBindings::default();
        for binding in self.bindings.iter() {
            bindings.try_push(binding.credited_clone()?).ok()?;
        }
        Some(Self { kind: self.kind, doc_schema: self.doc_schema.clone(), doc: self.doc.clone(), bindings })
    }
}

/// 🧩️ One `doc_schema` string, split into its `kind`/`version` halves — never the payload itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceSchema<'a> {
    pub kind: &'a str,
    pub version: u32,
}

/// 🚧️ Why a `doc_schema` string could not be parsed into a [`SurfaceSchema`] — NEVER a panic, always
/// this typed fault. A renderer that cannot resolve a `doc_schema` — either it fails to parse, per this
/// type, or it parses cleanly but names a `kind`/`version` pair the renderer does not implement — MUST
/// render a placeholder and log the fault; it must never panic and must never drop the surrounding
/// [`crate::UiPatch`]. Dropping one surface's content is not license to reject the whole document a
/// `SetComponent`/`Upsert` arrived in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceSchemaFault {
    /// 🕳️ `doc_schema` was the empty string.
    Empty,
    /// 🔗️ No `@` separator — the string is not `"<kind>@<version>"` shaped at all.
    MissingVersionSeparator,
    /// 🏷️ An `@` was found but the kind half in front of it was empty.
    EmptyKind,
    /// 🔢️ The version half after `@` did not parse as a `u32`.
    InvalidVersion,
}

/// 🧩️ Splits `doc_schema` into a [`SurfaceSchema`], or a [`SurfaceSchemaFault`] describing exactly why
/// it could not — never panics on any input, including empty strings, strings with no `@`, or a
/// non-numeric version half. This crate calls this function nowhere itself (validation never rejects on
/// `doc_schema` content — see this file's own module doc); it exists for renderers to call so their own
/// "unknown schema → placeholder + logged fault, never a panic" behaviour has a shared, tested
/// building block instead of each renderer hand-rolling its own `split_once('@')`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn parse_doc_schema(doc_schema: &str) -> Result<SurfaceSchema<'_>, SurfaceSchemaFault> {
    if doc_schema.is_empty() {
        return Err(SurfaceSchemaFault::Empty);
    }
    let (kind, version) = doc_schema.split_once('@').ok_or(SurfaceSchemaFault::MissingVersionSeparator)?;
    if kind.is_empty() {
        return Err(SurfaceSchemaFault::EmptyKind);
    }
    let version = version.parse::<u32>().map_err(|_| SurfaceSchemaFault::InvalidVersion)?;
    Ok(SurfaceSchema { kind, version })
}
//#endregion 🔖️Surface

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️surface-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
