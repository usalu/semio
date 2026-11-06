//! 🗺️ `SurfaceProps` — embedded product surfaces with an opaque pack-encoded payload. [Design notes](README.md#source-notes-1).

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. `SurfaceProps` is the
// deliberate exception — `bindings: UiNodeBindings` embeds `crate::ActionBinding`, which embeds
// `UiValue`, the DslValue-free exception (see its docstring in `🎬️action.rs`).
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Surface

/// 🖼️ The 15 embeddable product surface kinds, every tag kebab-case. `VirtualFileSystem` used to be
/// the one camelCase holdout (`"virtualFileSystem"`); the rename landed here first and reached the
/// wgpu target's own `SurfaceKind` (`🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`) and the schema
/// projection afterwards, so there is now exactly ONE spelling repo-wide — the law
/// `surface_kind_wire_tags_agree_between_contract_and_wgpu_target` pins it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::semio_framework_value")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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
#[value(crate = "::semio_framework_value")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct SurfaceDoc {
    pub bytes: crate::UiFixedBytes,
}

/// 🛍️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
/// `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
/// scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️.rs` in a
/// later packet, never into this dependency-free contract crate. See this file's own module doc for
/// the exact reasoning behind each field (and each field the scaffold this replaces used to carry but
/// no longer does).
// 🌱️ No `ToValue`/`FromValue` here: `bindings: UiNodeBindings` embeds `crate::ActionBinding`, the
// deliberate DslValue-free exception — see its own note above.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
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
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
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

/// 🚧️ Why a `doc_schema` string could not be parsed into a [`SurfaceSchema`] — NEVER a panic, always [Design notes](README.md#source-notes-187).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceSchemaFault {
    /// 🕳️ `doc_schema` was the empty string.
    Empty,
    /// ✂️ No `@` separator — the string is not `"<kind>@<version>"` shaped at all.
    MissingVersionSeparator,
    /// 🪫️ An `@` was found but the kind half in front of it was empty.
    EmptyKind,
    /// 🔢️ The version half after `@` did not parse as a `u32`.
    InvalidVersion,
}

/// 🔪️ Splits `doc_schema` into a [`SurfaceSchema`], or a [`SurfaceSchemaFault`] describing exactly why [Design notes](README.md#source-notes-205).
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

//#endregion 🧪️Tests
