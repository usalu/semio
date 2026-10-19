//! 📄️ Scene document and host sync: the editor scene range, token, diagnostic, placeholder, span, settings and document records, their field parsers and the editor host synchronisation. See [`super`].

use super::*;

//#region ✍️EditorSceneDocument
/// 📏️ The bytes one decoded editor scene may own — the authority React's twin grants the same decode
/// (`session.synchronizeScene(pack, 0, 16_777_216, 16_777_216, …)`, `🧱️elements/✏️TextEditor/🟦️.tsx`).
pub const TEXT_EDITOR_SCENE_OWNED_BYTES: usize = 16_777_216;

/// ↔️ One byte range of the canonical scene: the selection, an occurrence.
#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
struct EditorSceneRange {
    pub(in super::super) start: usize,
    pub(in super::super) end: usize,
}

/// 🎨️ One semantic token of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneToken {
    pub(in super::super) start: usize,
    pub(in super::super) end: usize,
    class: String,
}

/// 🩺️ One diagnostic of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneDiagnostic {
    pub(in super::super) start: usize,
    pub(in super::super) end: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    severity: Option<String>,
    message: String,
}

/// 🪧️ One placeholder of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorScenePlaceholder {
    pub(super) offset: usize,
    pub(in super::super) label: String,
}

/// 🎯️ One selectable span of the canonical scene.
#[derive(serde::Deserialize, serde::Serialize)]
struct EditorSceneSpan {
    pub(in super::super) start: usize,
    pub(in super::super) end: usize,
    pub(in super::super) kind: String,
}

/// ⚙️ The editor ENGINE's settings — the four members the canonical scene declares. The contract's `settingsJson` also
/// carries the host's own members (`readOnly`, `commit`, the explicit-draft labels, read by
/// [`text_editor_explicit_draft_settings`]); the canonical scene refuses a member it does not declare, so only these four
/// cross, and an absent one takes the engine's default.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    font_px: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    show_line_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tab_size: Option<usize>,
}

/// 🔦️ The hover and selection occurrence lists of the canonical scene.
#[derive(serde::Serialize)]
struct EditorSceneOccurrences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) hover: Option<Vec<EditorSceneRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) selection: Option<Vec<EditorSceneRange>>,
}

/// 📷️ The canonical scene's camera: one scrolling axis.
#[derive(serde::Serialize)]
struct EditorSceneCamera {
    pub(in super::super) y: f64,
}

/// 🚧️ The canonical scene's overlays.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneOverlays {
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_line_y: Option<f64>,
}

/// 🐁️ The canonical scene's hover: cleared, or one range.
#[derive(serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum EditorSceneHover {
    Clear,
    Range { start: usize, end: usize },
}

/// 🧾️ The editor engine's canonical scene document (`framework.editor.canonical-scene/v1`) as this host writes it. A
/// member left out leaves the engine's own value unchanged.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) selection: Option<EditorSceneRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tokens: Option<Vec<EditorSceneToken>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics: Option<Vec<EditorSceneDiagnostic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholders: Option<Vec<EditorScenePlaceholder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) occurrences: Option<EditorSceneOccurrences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_carets: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectable_spans: Option<Vec<EditorSceneSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) settings: Option<EditorSceneSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) camera: Option<EditorSceneCamera>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlays: Option<EditorSceneOverlays>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in super::super) hover: Option<EditorSceneHover>,
}

/// 🧷️ The contract's `selectionJson`: a caret alone is a collapsed range.
#[derive(serde::Deserialize)]
struct EditorSceneSelectionField {
    #[serde(default)]
    pub(in super::super) start: usize,
    #[serde(default)]
    pub(in super::super) end: Option<usize>,
}

/// 🗂️ The contract's `occurrencesJson`: each list is itself an encoded scene field.
#[derive(serde::Deserialize)]
struct EditorSceneOccurrenceFields {
    #[serde(default)]
    pub(in super::super) hover: Option<String>,
    #[serde(default)]
    pub(in super::super) selection: Option<String>,
}

/// 🎥️ The contract's `cameraJson`: a whole camera, of which the editor scrolls one axis.
#[derive(serde::Deserialize)]
struct EditorSceneCameraField {
    #[serde(default)]
    pub(in super::super) y: f64,
}

/// 🧱️ The contract's `overlaysJson`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditorSceneOverlaysField {
    #[serde(default)]
    dead_line_y: Option<f64>,
}

/// 🫥️ The contract's `hoverJson`: a range hovers, anything short of one clears.
#[derive(serde::Deserialize)]
struct EditorSceneHoverField {
    #[serde(default)]
    pub(in super::super) start: Option<usize>,
    #[serde(default)]
    pub(in super::super) end: Option<usize>,
}

/// 🔎️ One `*Json` member of the contract's scene in its typed form: the encoded field expanded
/// (`semio_framework_os_kernel::os_pack::pack_rt::scene_field_json_text`, React's `parseSceneJsonField`) and read; a JSON `null` is no value.
fn text_editor_scene_member<T: serde::de::DeserializeOwned>(member: &str, raw: &str) -> Result<Option<T>, String> {
    let json = semio_framework_os_kernel::os_pack::pack_rt::scene_field_json_text(raw).map_err(|error| format!("{member}: {error}"))?;
    serde_json::from_str::<Option<T>>(&json).map_err(|error| format!("{member}: {error}"))
}

/// 📥️ [`text_editor_scene_member`] for a member that may be absent; one it cannot read is named in `faults` and left out.
fn text_editor_scene_field<T: serde::de::DeserializeOwned>(member: &str, raw: Option<&str>, faults: &mut Vec<String>) -> Option<T> {
    match text_editor_scene_member(member, raw?) {
        Ok(value) => value,
        Err(fault) => {
            faults.push(fault);
            None
        }
    }
}

/// 🏗️ Builds the editor engine's canonical scene document from the UI contract's text-editor scene — the ONE place this
/// host turns the contract's `*Json` string members into the typed members `EditorHost::synchronize_scene` takes, the
/// wgpu twin of React's `textEditorScenePacket` (`🧱️elements/✏️TextEditor/🟦️.tsx`).
///
/// `carries_text` is whether the scene's buffer and selection are news to the host: a scene that only echoes the host's
/// own edit leaves both out, so the local draft and its caret stay (React's `sceneWithoutEchoedTextV1`). A member this
/// host cannot read is left out — unchanged in the engine, by the document's own rule — and named in the second answer;
/// it is never guessed.
pub fn text_editor_scene_document(editor: &semio_framework_ui_declaration::ui::TextEditorScene, carries_text: bool) -> (String, Vec<String>) {
    let mut faults = Vec::new();
    let selection = if carries_text { text_editor_scene_field::<EditorSceneSelectionField>("selectionJson", editor.selection_json.as_deref(), &mut faults) } else { None };
    let occurrences = text_editor_scene_field::<EditorSceneOccurrenceFields>("occurrencesJson", editor.occurrences_json.as_deref(), &mut faults);
    let hover = editor.hover_json.as_deref().and_then(|raw| match text_editor_scene_member::<EditorSceneHoverField>("hoverJson", raw) {
        Ok(Some(EditorSceneHoverField { start: Some(start), end: Some(end) })) => Some(EditorSceneHover::Range { start, end }),
        Ok(_) => Some(EditorSceneHover::Clear),
        Err(fault) => {
            faults.push(fault);
            None
        }
    });
    let document = EditorSceneDocument {
        buffer: carries_text.then(|| editor.buffer.clone()),
        selection: selection.map(|field| EditorSceneRange { start: field.start, end: field.end.unwrap_or(field.start) }),
        tokens: text_editor_scene_field("tokensJson", editor.tokens_json.as_deref(), &mut faults),
        diagnostics: text_editor_scene_field("diagnosticsJson", editor.diagnostics_json.as_deref(), &mut faults),
        placeholders: text_editor_scene_field("placeholdersJson", editor.placeholders_json.as_deref(), &mut faults),
        occurrences: occurrences.map(|fields| EditorSceneOccurrences {
            hover: text_editor_scene_field("occurrencesJson.hover", fields.hover.as_deref(), &mut faults),
            selection: text_editor_scene_field("occurrencesJson.selection", fields.selection.as_deref(), &mut faults),
        }),
        extra_carets: text_editor_scene_field("extraCaretsJson", editor.extra_carets_json.as_deref(), &mut faults),
        selectable_spans: text_editor_scene_field("selectableSpansJson", editor.selectable_spans_json.as_deref(), &mut faults),
        settings: text_editor_scene_field("settingsJson", editor.settings_json.as_deref(), &mut faults),
        camera: text_editor_scene_field::<EditorSceneCameraField>("cameraJson", editor.camera_json.as_deref(), &mut faults).map(|field| EditorSceneCamera { y: field.y }),
        overlays: text_editor_scene_field::<EditorSceneOverlaysField>("overlaysJson", editor.overlays_json.as_deref(), &mut faults).map(|field| EditorSceneOverlays { dead_line_y: field.dead_line_y }),
        hover,
    };
    (serde_json::to_string(&document).unwrap_or_else(|_| "{}".to_string()), faults)
}

/// 🎬️ Commits one contract scene into its editor host: the canonical document ([`text_editor_scene_document`]) decoded
/// under a bounded authority ([`TEXT_EDITOR_SCENE_OWNED_BYTES`]) and handed to `EditorHost::synchronize_scene`. A member
/// the builder left out is told on the console; a document the engine refuses is the caller's `Err` — never silence.
pub fn synchronize_text_editor_host(host: &mut EditorHost, editor: &semio_framework_ui_declaration::ui::TextEditorScene, carries_text: bool) -> Result<(), String> {
    let (document, faults) = text_editor_scene_document(editor, carries_text);
    for fault in &faults {
        engine_canvas_fault_log(&format!("[text-editor] scene member left unchanged — {fault}"));
    }
    let mut admitted = |_| true;
    let mut control = semio_framework_value::NativeDecodeControl::new(TEXT_EDITOR_SCENE_OWNED_BYTES, &mut admitted);
    let scene = framework_editor::scene::from_json(&document, &mut control).map_err(|error| error.to_string())?;
    host.synchronize_scene(scene);
    Ok(())
}
//#endregion ✍️EditorSceneDocument
