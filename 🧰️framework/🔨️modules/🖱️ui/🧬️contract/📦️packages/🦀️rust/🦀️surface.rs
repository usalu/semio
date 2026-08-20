//! @emoji 🗺️ `SurfaceProps` — embedded product surfaces with an opaque pack-encoded payload.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

use serde::{Deserialize, Serialize};

//#region 🔖️Surface

/// 🆔️ Identifies one embedded product surface (a viewport instance a plugin owns) within a document.
///
pub use crate::SurfaceId;

/// 🖼️ The 15 embeddable product surface kinds. Ported VERBATIM from the wgpu target's `SurfaceKind`,
/// same serde renames — including the one real inconsistency already in the wire format
/// (`virtualFileSystem` is camelCase where every sibling is kebab-case). Preserved rather than
/// "cleaned up" because the packet brief demands a byte-identical port; a rename is a breaking wire
/// change for a later packet to make deliberately, not a silent side effect of this one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
    #[serde(rename = "virtualFileSystem")]
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
/// product's own scene crate) knows how to decode.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct SurfaceDoc {
    pub bytes: Vec<u8>,
}

/// 🗺️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
/// `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
/// scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️component.rs` in a
/// later packet, never into this dependency-free contract crate.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct SurfaceProps {
    pub surface_id: SurfaceId,
    pub controller_id: String,
    pub kind: SurfaceKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pane_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_id: Option<String>,
    pub doc_schema: String,
    pub doc: SurfaceDoc,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_granularity_id: Option<String>,
}
//#endregion 🔖️Surface

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_kind_verbatim_renames() {
        assert_eq!(serde_json::to_string(&SurfaceKind::World3d).expect("serialize"), "\"world-3d\"");
        assert_eq!(serde_json::to_string(&SurfaceKind::VirtualFileSystem).expect("serialize"), "\"virtualFileSystem\"");
    }

    #[test]
    fn surface_props_with_non_empty_doc_roundtrips() {
        let props = SurfaceProps {
            surface_id: SurfaceId("surf-1".into()),
            controller_id: "ctrl".into(),
            kind: SurfaceKind::World3d,
            pane_id: Some("pane-1".into()),
            binding_id: None,
            doc_schema: "world3d@1".into(),
            doc: SurfaceDoc { bytes: vec![1, 2, 3, 4, 5] },
            domain_id: Some("domain-1".into()),
            domain_granularity_id: None,
        };
        let json = serde_json::to_string(&props).expect("serialize");
        let back: SurfaceProps = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(props, back);
        assert!(!back.doc.bytes.is_empty());
    }
}
