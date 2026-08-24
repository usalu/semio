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
//! reject the surrounding [`crate::UiPatch`] or panic reconciliation.** Neither `🦀️limits.rs`'s
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

use serde::{Deserialize, Serialize};

//#region 🔖️Surface

/// 🖼️ The 15 embeddable product surface kinds. Ported from the wgpu target's `SurfaceKind`, with its
/// one real wire inconsistency FIXED rather than preserved: `VirtualFileSystem` was
/// `"virtualFileSystem"` (camelCase) where every sibling is kebab-case. This program has no back-compat
/// obligation (greenfield, no users, no legacy support — root `CLAUDE.md`), so the rename is made here
/// deliberately rather than carried forward as debt for "a later packet to make on purpose".
///
/// **Rename: `"virtualFileSystem"` → `"virtual-file-system"`.**
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceKind {
    /// 🖌️ The plainest surface a host can always satisfy, so a `SurfaceProps` built from defaults
    /// names something renderable rather than a kind no backend registered.
    #[default]
    #[serde(rename = "canvas-2d")]
    Canvas2d,
    #[serde(rename = "world-3d")]
    World3d,
    #[serde(rename = "node-graph")]
    NodeGraph,
    #[serde(rename = "text-editor")]
    TextEditor,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "paint-2d")]
    Paint2d,
    #[serde(rename = "virtual-file-system")]
    VirtualFileSystem,
    #[serde(rename = "tiled-map")]
    TiledMap,
    #[serde(rename = "board-2d")]
    Board2d,
    #[serde(rename = "icon-render")]
    IconRender,
    #[serde(rename = "ink-canvas")]
    InkCanvas,
    #[serde(rename = "graph-timeline")]
    GraphTimeline,
    #[serde(rename = "block-list")]
    BlockList,
    #[serde(rename = "diff-view")]
    DiffView,
    #[serde(rename = "event-feed")]
    EventFeed,
}

/// 📦️ An opaque, pack-encoded payload. The contract never parses it — `doc_schema` on the owning
/// [`SurfaceProps`] names the version-specific shape (e.g. `"world3d@1"`) that some other layer (the
/// `🎬️scene` crate) knows how to decode.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDoc {
    pub bytes: crate::UiFixedBytes,
}

/// 🗺️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
/// `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
/// scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️component.rs` in a
/// later packet, never into this dependency-free contract crate. See this file's own module doc for
/// the exact reasoning behind each field (and each field the scaffold this replaces used to carry but
/// no longer does).
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
mod tests {
    use super::*;

    //#region 🔖️Rename
    #[test]
    fn surface_kind_wire_names_are_all_kebab_case() {
        assert_eq!(serde_json::to_string(&SurfaceKind::World3d).expect("serialize"), "\"world-3d\"");
        assert_eq!(serde_json::to_string(&SurfaceKind::VirtualFileSystem).expect("serialize"), "\"virtual-file-system\"", "the wire-name inconsistency this packet fixes: was camelCase \"virtualFileSystem\"");
    }
    //#endregion 🔖️Rename

    //#region 🔖️SurfaceProps
    #[test]
    fn surface_props_round_trips_with_bindings_and_non_empty_doc() {
        let props = SurfaceProps {
            kind: SurfaceKind::World3d,
            doc_schema: "world3d@1".into(),
            doc: SurfaceDoc { bytes: vec![1, 2, 3, 4, 5] },
            bindings: vec![crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::v1("scope", "reset-view"), args: None, capability: None }],
        };
        let json = serde_json::to_string(&props).expect("serialize");
        let back: SurfaceProps = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(props, back);
        assert!(!back.doc.bytes.is_empty());
        assert_eq!(back.bindings.len(), 1);
    }

    #[test]
    fn surface_props_omits_empty_bindings_on_the_wire() {
        let props = SurfaceProps { kind: SurfaceKind::Canvas2d, doc_schema: "canvas2d@1".into(), doc: SurfaceDoc::default(), bindings: Vec::new() };
        let json = serde_json::to_value(&props).expect("serialize");
        assert!(json.get("bindings").is_none());
    }

    /// 🧬️ The property this packet's whole "opaque blob" rule rests on: two `SurfaceProps` differing
    /// only in one byte of `doc.bytes` are unequal via plain derived `PartialEq` — the reconciler needs
    /// nothing schema-aware to notice a changed scene, structural equality already carries it.
    #[test]
    fn differing_only_in_one_doc_byte_makes_surface_props_unequal() {
        let base = SurfaceProps { kind: SurfaceKind::World3d, doc_schema: "world3d@1".into(), doc: SurfaceDoc { bytes: vec![1, 2, 3] }, bindings: Vec::new() };
        let mut changed = base.clone();
        changed.doc.bytes[1] = 99;
        assert_ne!(base, changed, "a one-byte doc change must make the whole SurfaceProps compare unequal");
        assert_eq!(base, base.clone(), "an identical doc must compare equal");
    }
    //#endregion 🔖️SurfaceProps

    //#region 🔖️SchemaParsing
    #[test]
    fn parse_doc_schema_splits_kind_and_version() {
        assert_eq!(parse_doc_schema("world3d@1"), Ok(SurfaceSchema { kind: "world3d", version: 1 }));
        assert_eq!(parse_doc_schema("node-graph@42"), Ok(SurfaceSchema { kind: "node-graph", version: 42 }));
    }

    /// 🚧️ Every malformed shape returns a typed fault, never a panic — exercised explicitly rather than
    /// merely trusted, since this is the exact guarantee an unknown/malformed `doc_schema` depends on to
    /// avoid taking down reconciliation.
    #[test]
    fn parse_doc_schema_never_panics_and_returns_a_typed_fault_for_every_malformed_shape() {
        assert_eq!(parse_doc_schema(""), Err(SurfaceSchemaFault::Empty));
        assert_eq!(parse_doc_schema("world3d"), Err(SurfaceSchemaFault::MissingVersionSeparator));
        assert_eq!(parse_doc_schema("@1"), Err(SurfaceSchemaFault::EmptyKind));
        assert_eq!(parse_doc_schema("world3d@not-a-number"), Err(SurfaceSchemaFault::InvalidVersion));
        assert_eq!(parse_doc_schema("world3d@"), Err(SurfaceSchemaFault::InvalidVersion));
        assert_eq!(parse_doc_schema("@"), Err(SurfaceSchemaFault::EmptyKind));
        assert_eq!(parse_doc_schema("a@b@1"), Err(SurfaceSchemaFault::InvalidVersion), "split_once takes only the FIRST '@'; \"b@1\" is what gets parsed as the version half and fails to parse as u32");
    }
    //#endregion 🔖️SchemaParsing

    //#region 🔖️UnknownSchemaNeverRejects
    /// 🛡️ The contract-side half of "an unknown `doc_schema` must never panic or drop the surrounding
    /// patch": a `Component::Surface` carrying a schema no renderer recognises still passes
    /// `validate_snapshot` cleanly and applies through `apply_patch` like any other component change —
    /// this crate never validates `doc_schema` against a known set, because it can never own that set
    /// (every product crate that embeds a surface adds its own kinds).
    #[test]
    fn validate_snapshot_never_rejects_an_unrecognised_doc_schema() {
        let surface = crate::Component::Surface(SurfaceProps { kind: SurfaceKind::World3d, doc_schema: "totally-unknown-schema-nobody-registered@999".into(), doc: SurfaceDoc { bytes: vec![9, 9, 9] }, bindings: Vec::new() });
        let record = crate::UiNodeRecord {
            id: crate::UiNodeId(0),
            key: "root".into(),
            component: surface,
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Vec::new(),
            menu: None,
            children: Vec::new(),
        };
        let snapshot = crate::UiSnapshot { surface: crate::SurfaceId::from("s"), revision: crate::UiRevision(0), root: crate::UiNodeId(0), nodes: vec![record], layout_epoch: 0 };
        assert_eq!(crate::validate_snapshot(&snapshot, &crate::UiDocumentLimits::default()), Ok(()), "an unrecognised doc_schema must never be a validation violation");
    }

    #[test]
    fn apply_patch_accepts_a_set_component_carrying_an_unrecognised_doc_schema() {
        let mut state = crate::UiSnapshotState::new(crate::SurfaceId::from("s"));
        state.root = Some(crate::UiNodeId(0));
        state.nodes.insert(
            crate::UiNodeId(0),
            crate::UiNodeRecord {
                id: crate::UiNodeId(0),
                key: "root".into(),
                component: crate::Component::Separator(crate::SeparatorProps {}),
                layout: Default::default(),
                style: Default::default(),
                activity: Default::default(),
                disabled: false,
                transition: None,
                accessibility: Default::default(),
                bindings: Vec::new(),
                menu: None,
                children: Vec::new(),
            },
        );
        let limits = crate::UiDocumentLimits::default();

        let surface = crate::Component::Surface(SurfaceProps { kind: SurfaceKind::Canvas2d, doc_schema: "nonsense@not-a-version".into(), doc: SurfaceDoc::default(), bindings: Vec::new() });
        let patch = crate::UiPatch { surface: state.surface.clone(), base_revision: state.revision, revision: state.revision.next(), ops: vec![crate::UiPatchOp::SetComponent { id: crate::UiNodeId(0), component: surface }] };

        crate::apply_patch(&mut state, &patch, &limits).expect("a patch carrying an unrecognised/unparseable doc_schema must still apply — it is not this crate's job to reject it");
    }
    //#endregion 🔖️UnknownSchemaNeverRejects
}
//#endregion 🧪️Tests
