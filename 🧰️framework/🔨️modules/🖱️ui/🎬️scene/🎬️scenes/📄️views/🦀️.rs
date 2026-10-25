//! 📄️ The document-style scene payloads (text editor, table, block list, diff view, event feed, graph timeline, virtual file system, icon render) with the draft text splice.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_doc::{scene_lane_hash, scene_pack_wire, value_decode, value_decode_default, value_decode_option, value_push, value_push_option, SceneDoc, SceneLanePayload, SceneLaneRef};
use semio_framework_ui_scene_pack::{from_bytes, to_bytes, PackError};
use serde::{Deserialize, Serialize};
#[path = "../../✂️text-splice/🦀️.rs"]
mod text_splice;
pub use text_splice::{
    apply_draft_changes, compose_draft_step, rebase_text_edits, write_draft_changes_json_into, AppliedTextSplice, DraftChange, DraftChangesJsonCursor, LocatedTextSplice, TextSplice, TextSpliceComposition, DRAFT_SPLICES_ARGUMENT,
    TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS,
};

//#region 🔖️TextEditorScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditorScene {
    pub buffer: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_carets_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_spans_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newline_gates_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_json: Option<String>,
}

scene_pack_wire!(TextEditorScenePack, TextEditorScene {
    buffer: String,
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
    language: Option<String>,
    selection_json: Option<String>,
    tokens_json: Option<String>,
    diagnostics_json: Option<String>,
    completions_json: Option<String>,
    overlays_json: Option<String>,
    occurrences_json: Option<String>,
    placeholders_json: Option<String>,
    extra_carets_json: Option<String>,
    selectable_spans_json: Option<String>,
    settings_json: Option<String>,
    camera_json: Option<String>,
    hover_json: Option<String>,
    newline_gates_json: Option<String>,
    rename_json: Option<String>,
});

impl SceneDoc for TextEditorScene {
    const SCHEMA: &'static str = "text-editor@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let payload = std::mem::take(&mut spine.buffer);
        spine.lanes = vec![SceneLaneRef { lane: "buffer".into(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) }];
        (spine, vec![SceneLanePayload { key: TEXT_EDITOR_BUFFER_LANE_KEY, payload }])
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        if key != TEXT_EDITOR_BUFFER_LANE_KEY {
            return false;
        }
        self.buffer = payload;
        true
    }

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(&TextEditorScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<TextEditorScenePack>(bytes).map(Into::into)
    }
}

/// 🚚️ The document buffer travels in paged carriers independently of the bounded scene header.
pub const TEXT_EDITOR_BUFFER_LANE_KEY: &str = "framework.scene.text.buffer";

impl TextEditorScene {
    /** ✍️ Builds a text-editor scene with optional extensions unset. */
    pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
        Self {
            buffer,
            lanes: Vec::new(),
            language,
            selection_json,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        }
    }
}

impl ToValue for TextEditorScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "buffer", &self.buffer);
        if !self.lanes.is_empty() {
            value_push(&mut entries, "lanes", &self.lanes);
        }
        value_push_option(&mut entries, "language", &self.language);
        value_push_option(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "tokensJson", &self.tokens_json);
        value_push_option(&mut entries, "diagnosticsJson", &self.diagnostics_json);
        value_push_option(&mut entries, "completionsJson", &self.completions_json);
        value_push_option(&mut entries, "overlaysJson", &self.overlays_json);
        value_push_option(&mut entries, "occurrencesJson", &self.occurrences_json);
        value_push_option(&mut entries, "placeholdersJson", &self.placeholders_json);
        value_push_option(&mut entries, "extraCaretsJson", &self.extra_carets_json);
        value_push_option(&mut entries, "selectableSpansJson", &self.selectable_spans_json);
        value_push_option(&mut entries, "settingsJson", &self.settings_json);
        value_push_option(&mut entries, "cameraJson", &self.camera_json);
        value_push_option(&mut entries, "hoverJson", &self.hover_json);
        value_push_option(&mut entries, "newlineGatesJson", &self.newline_gates_json);
        value_push_option(&mut entries, "renameJson", &self.rename_json);
        DslValue::Object(entries)
    }
}

impl FromValue for TextEditorScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            buffer: value_decode(&entries, "buffer")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
            language: value_decode_option(&entries, "language")?,
            selection_json: value_decode_option(&entries, "selectionJson")?,
            tokens_json: value_decode_option(&entries, "tokensJson")?,
            diagnostics_json: value_decode_option(&entries, "diagnosticsJson")?,
            completions_json: value_decode_option(&entries, "completionsJson")?,
            overlays_json: value_decode_option(&entries, "overlaysJson")?,
            occurrences_json: value_decode_option(&entries, "occurrencesJson")?,
            placeholders_json: value_decode_option(&entries, "placeholdersJson")?,
            extra_carets_json: value_decode_option(&entries, "extraCaretsJson")?,
            selectable_spans_json: value_decode_option(&entries, "selectableSpansJson")?,
            settings_json: value_decode_option(&entries, "settingsJson")?,
            camera_json: value_decode_option(&entries, "cameraJson")?,
            hover_json: value_decode_option(&entries, "hoverJson")?,
            newline_gates_json: value_decode_option(&entries, "newlineGatesJson")?,
            rename_json: value_decode_option(&entries, "renameJson")?,
        })
    }
}
//#endregion 🔖️TextEditorScene

//#region 🔖️TableScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableScene {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
    pub columns_json: String,
    pub rows_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_drag_mime: Option<String>,
    /// 🪟️ Opaque pack-encoded `ActionDescriptor` JSON — was a live `ui_wgpu::wgpu::ActionDescriptor`
    /// before this move; see this module's own header for why that type couldn't come along.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_action_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_json: Option<String>,
    /// 🏷️ The framework interaction domain a row pick selects in — `None` keeps row clicks plugin-private.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 🎯️ `domain_id`'s granularity id a row is picked under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_granularity_id: Option<String>,
}

impl SceneDoc for TableScene {
    const SCHEMA: &'static str = "table@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let columns = std::mem::take(&mut spine.columns_json);
        let rows = std::mem::take(&mut spine.rows_json);
        spine.lanes = vec![
            SceneLaneRef { lane: "columns".into(), bytes: columns.len() as u32, hash: scene_lane_hash(&columns) },
            SceneLaneRef { lane: "rows".into(), bytes: rows.len() as u32, hash: scene_lane_hash(&rows) },
        ];
        (spine, vec![SceneLanePayload { key: TABLE_COLUMNS_LANE_KEY, payload: columns }, SceneLanePayload { key: TABLE_ROWS_LANE_KEY, payload: rows }])
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        match key {
            TABLE_COLUMNS_LANE_KEY => self.columns_json = payload,
            TABLE_ROWS_LANE_KEY => self.rows_json = payload,
            _ => return false,
        }
        true
    }
}

pub const TABLE_COLUMNS_LANE_KEY: &str = "framework.scene.table.columns";
pub const TABLE_ROWS_LANE_KEY: &str = "framework.scene.table.rows";

impl TableScene {
    /** 📋️ Builds a table scene with optional extensions (selection/drag/sort/domain) unset. */
    pub fn base(columns_json: impl Into<String>, rows_json: impl Into<String>) -> Self {
        Self { lanes: Vec::new(), columns_json: columns_json.into(), rows_json: rows_json.into(), selection_json: None, row_drag_mime: None, drop_action_json: None, sort_json: None, domain_id: None, domain_granularity_id: None }
    }
}

impl ToValue for TableScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        if !self.lanes.is_empty() { value_push(&mut entries, "lanes", &self.lanes); }
        value_push(&mut entries, "columnsJson", &self.columns_json);
        value_push(&mut entries, "rowsJson", &self.rows_json);
        value_push_option(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "rowDragMime", &self.row_drag_mime);
        value_push_option(&mut entries, "dropActionJson", &self.drop_action_json);
        value_push_option(&mut entries, "sortJson", &self.sort_json);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        value_push_option(&mut entries, "domainGranularityId", &self.domain_granularity_id);
        DslValue::Object(entries)
    }
}

impl FromValue for TableScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
            columns_json: value_decode(&entries, "columnsJson")?,
            rows_json: value_decode(&entries, "rowsJson")?,
            selection_json: value_decode_option(&entries, "selectionJson")?,
            row_drag_mime: value_decode_option(&entries, "rowDragMime")?,
            drop_action_json: value_decode_option(&entries, "dropActionJson")?,
            sort_json: value_decode_option(&entries, "sortJson")?,
            domain_id: value_decode_option(&entries, "domainId")?,
            domain_granularity_id: value_decode_option(&entries, "domainGranularityId")?,
        })
    }
}
//#endregion 🔖️TableScene

//#region 🔖️IconRenderScene
/** 🖼️ Icon-render scene: client-side render request for a shot preview. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconRenderScene {
    pub request_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_json: Option<String>,
}

impl SceneDoc for IconRenderScene {
    const SCHEMA: &'static str = "icon-render@1";
}

impl ToValue for IconRenderScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "requestJson", &self.request_json);
        value_push_option(&mut entries, "footer", &self.footer);
        value_push_option(&mut entries, "frameJson", &self.frame_json);
        DslValue::Object(entries)
    }
}

impl FromValue for IconRenderScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            request_json: value_decode(&entries, "requestJson")?,
            footer: value_decode_option(&entries, "footer")?,
            frame_json: value_decode_option(&entries, "frameJson")?,
        })
    }
}
//#endregion 🔖️IconRenderScene

//#region 🔖️VirtualFileSystemScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualFileSystemScene {
    pub schema_json: String,
    pub rows_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_row_ids_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_row_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_drop_enabled: Option<bool>,
}

impl SceneDoc for VirtualFileSystemScene {
    const SCHEMA: &'static str = "virtual-file-system@1";
}

impl ToValue for VirtualFileSystemScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "schemaJson", &self.schema_json);
        value_push(&mut entries, "rowsJson", &self.rows_json);
        value_push_option(&mut entries, "selectedRowIdsJson", &self.selected_row_ids_json);
        value_push_option(&mut entries, "hoveredRowId", &self.hovered_row_id);
        value_push_option(&mut entries, "emptyMessage", &self.empty_message);
        value_push_option(&mut entries, "dragDropEnabled", &self.drag_drop_enabled);
        DslValue::Object(entries)
    }
}

impl FromValue for VirtualFileSystemScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            schema_json: value_decode(&entries, "schemaJson")?,
            rows_json: value_decode(&entries, "rowsJson")?,
            selected_row_ids_json: value_decode_option(&entries, "selectedRowIdsJson")?,
            hovered_row_id: value_decode_option(&entries, "hoveredRowId")?,
            empty_message: value_decode_option(&entries, "emptyMessage")?,
            drag_drop_enabled: value_decode_option(&entries, "dragDropEnabled")?,
        })
    }
}
//#endregion 🔖️VirtualFileSystemScene

//#region 🔖️GraphTimelineScene
/** 🗄️ A checkpoint ancestor-graph history view. `columns_json` is a `HistoryColumn[]` array,
 * newest checkpoint first. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphTimelineScene {
    pub columns_json: String,
}

impl SceneDoc for GraphTimelineScene {
    const SCHEMA: &'static str = "graph-timeline@1";
}

impl ToValue for GraphTimelineScene {
    fn to_value(&self) -> DslValue {
        DslValue::object([("columnsJson".to_string(), self.columns_json.to_value())])
    }
}

impl FromValue for GraphTimelineScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self { columns_json: value_decode(&entries, "columnsJson")? })
    }
}
//#endregion 🔖️GraphTimelineScene

//#region 🔖️DiffViewScene
/** 🆚️ A before/after text comparison. `mode` picks the renderer's layout (`"unified"` inline
 * hunks or `"split"` side-by-side panes); `language` is an optional syntax-highlighting hint. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffViewScene {
    pub before: String,
    pub after: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for DiffViewScene {
    const SCHEMA: &'static str = "diff-view@1";
}

impl ToValue for DiffViewScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "before", &self.before);
        value_push(&mut entries, "after", &self.after);
        value_push_option(&mut entries, "language", &self.language);
        value_push_option(&mut entries, "mode", &self.mode);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for DiffViewScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            before: value_decode(&entries, "before")?,
            after: value_decode(&entries, "after")?,
            language: value_decode_option(&entries, "language")?,
            mode: value_decode_option(&entries, "mode")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️DiffViewScene

//#region 🔖️EventFeedScene
/** 📰️ A chronological feed of host-authored events. `entries_json` is a
 * `{id, timestampMs, iconId, title, detail?, tone?}[]` array; `activate_action` (if set) is the
 * action name fired with the clicked entry's `id` when an entry is activated. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFeedScene {
    pub entries_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl SceneDoc for EventFeedScene {
    const SCHEMA: &'static str = "event-feed@1";
}

impl ToValue for EventFeedScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "entriesJson", &self.entries_json);
        value_push_option(&mut entries, "follow", &self.follow);
        value_push_option(&mut entries, "activateAction", &self.activate_action);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for EventFeedScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            entries_json: value_decode(&entries, "entriesJson")?,
            follow: value_decode_option(&entries, "follow")?,
            activate_action: value_decode_option(&entries, "activateAction")?,
            domain_id: value_decode_option(&entries, "domainId")?,
        })
    }
}
//#endregion 🔖️EventFeedScene

//#region 🔖️BlockListScene
#[path = "../../🧬️schema/🧩️block-list/🦀️.rs"]
mod block_list;
pub use block_list::{BlockListBlock, BlockListPaletteEntry, BlockListScene, BlockListSelectionTarget, BlockListStep};
//#endregion 🔖️BlockListScene

#[cfg(test)]
#[path = "../../🧪️tests/🔬️views-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️views-value-round-trip/🦀️.rs"]
mod value_round_trip_tests;
