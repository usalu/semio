//! 🏷️ Localized labels and the scalar, enum, template, variant and text controls.

use super::*;

#[derive(Clone, Copy)]
pub(super) struct Labels {
    pub(super) locale: Locale,
    pub(super) details: &'static str,
    pub(super) source: &'static str,
    pub(super) apply: &'static str,
    pub(super) discard: &'static str,
    pub(super) conflict: &'static str,
    pub(super) applying: &'static str,
    pub(super) cancel: &'static str,
    pub(super) failed: &'static str,
    pub(super) syntax_error: &'static str,
    pub(super) validation_error: &'static str,
    pub(super) path: &'static str,
    pub(super) line: &'static str,
    pub(super) column: &'static str,
    pub(super) add: &'static str,
    pub(super) add_text: &'static str,
    pub(super) add_number: &'static str,
    pub(super) add_true: &'static str,
    pub(super) add_false: &'static str,
    pub(super) add_object: &'static str,
    pub(super) add_list: &'static str,
    pub(super) add_null: &'static str,
    pub(super) create: &'static str,
    pub(super) choose: &'static str,
    pub(super) switch_to: &'static str,
    pub(super) remove: &'static str,
    pub(super) move_up: &'static str,
    pub(super) move_down: &'static str,
    pub(super) key: &'static str,
    pub(super) item: &'static str,
    pub(super) value: &'static str,
}

pub(super) fn labels(locale: Locale) -> Labels {
    if locale == Locale::De {
        Labels {
            locale,
            details: "Details",
            source: "Quelltext",
            apply: "Anwenden",
            discard: "Verwerfen",
            conflict: "Das Dokument wurde geändert. Kopieren Sie Ihren Entwurf oder verwerfen Sie ihn vor dem Anwenden.",
            applying: "Entwurf wird vorbereitet",
            cancel: "Abbrechen",
            failed: "Entwurf konnte nicht angewendet werden",
            syntax_error: "Der Quelltext enthält ungültige Syntax.",
            validation_error: "Der Entwurf entspricht nicht dem Dokumentformat.",
            path: "Pfad",
            line: "Zeile",
            column: "Spalte",
            add: "Hinzufügen",
            add_text: "Text hinzufügen",
            add_number: "Zahl hinzufügen",
            add_true: "Wahr hinzufügen",
            add_false: "Falsch hinzufügen",
            add_object: "Objekt hinzufügen",
            add_list: "Liste hinzufügen",
            add_null: "Leerwert hinzufügen",
            create: "Gültigen Eintrag erstellen",
            choose: "Auswählen",
            switch_to: "Wechseln zu",
            remove: "Entfernen",
            move_up: "Nach oben",
            move_down: "Nach unten",
            key: "Schlüssel",
            item: "Eintrag",
            value: "Wert",
        }
    } else {
        Labels {
            locale,
            details: "Details",
            source: "Source",
            apply: "Apply",
            discard: "Discard",
            conflict: "The document changed. Copy your draft or discard it before applying.",
            applying: "Preparing draft",
            cancel: "Cancel",
            failed: "The draft could not be applied",
            syntax_error: "The source contains invalid syntax.",
            validation_error: "The draft does not match the document format.",
            path: "Path",
            line: "Line",
            column: "Column",
            add: "Add",
            add_text: "Add text",
            add_number: "Add number",
            add_true: "Add true",
            add_false: "Add false",
            add_object: "Add object",
            add_list: "Add list",
            add_null: "Add null",
            create: "Create valid item",
            choose: "Choose",
            switch_to: "Switch to",
            remove: "Remove",
            move_up: "Move up",
            move_down: "Move down",
            key: "Key",
            item: "Item",
            value: "Value",
        }
    }
}

pub(super) fn error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "snapshot details UI admission failed")
}

/// 🎟️ A refusal of the process-wide `UiValue` arena is the framework's capacity refusal, so the tree window
/// building this row ends there with a shorter run instead of faulting the whole render — see
/// `semio_framework_plugin::tree_window_indexed_section` and law `🧪️tests/🎟️details-arena-headroom`.
fn arena(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("snapshot details {stage} admission failed: the UiValue arena has no free credit"))
}

pub(super) fn text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| error("ui.snapshot-details.text"))
}

pub(super) fn label(value: &str) -> UiAssemblyResult<ui::Label> {
    Ok(ui::Label(UiText::clipped(value)))
}

pub(super) fn ui_args(entries: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut map = UiMapBuilder::try_new().ok_or_else(|| arena("argument map"))?;
    for (key, value) in entries {
        map.try_insert(key.to_string(), value).map_err(|_| arena("argument entry"))?;
    }
    Ok(UiValue::Map(map.finish()))
}

pub(super) fn ui_text_value(value: &str) -> UiAssemblyResult<UiValue> {
    Ok(UiValue::Text(text(value)?))
}

pub(super) fn action(controller_id: &str, name: &str) -> UiAssemblyResult<ActionId> {
    ActionId::try_v1(controller_id, name).ok_or_else(|| error("ui.snapshot-details.action-id"))
}

pub(super) fn bind<B: HasBase>(builder: B, trigger: ui::Trigger, controller_id: &str, action_id: &str, args: UiValue) -> UiAssemblyResult<B> {
    builder.try_on_with(trigger, action(controller_id, action_id)?, args).map_err(|_| error("ui.snapshot-details.action-binding"))
}

pub(super) fn button(id: &str, title: &str, icon: &str, controller_id: &str, action_id: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let builder = ui::button(label(title)?).icon(text(icon)?).try_id(id).map_err(|_| error("ui.snapshot-details.button-id"))?;
    bind(builder, ui::Trigger::Activate, controller_id, action_id, args)?.try_build().map_err(|_| error("ui.snapshot-details.button"))
}

pub(super) fn control_group(id: &str, controls: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<BuiltNode> {
    let mut builder = ui::row().try_id(id).map_err(|_| error("ui.snapshot-details.controls-id"))?;
    for control in controls {
        builder = builder.try_child(control?).map_err(|_| error("ui.snapshot-details.control"))?;
    }
    builder.try_build().map_err(|_| error("ui.snapshot-details.controls"))
}

pub(super) fn pointer(path: &[SnapshotDetailPathSegment]) -> String {
    let mut pointer = String::new();
    for segment in path {
        pointer.push('/');
        match segment {
            SnapshotDetailPathSegment::Key(key) => pointer.push_str(&key.replace('~', "~0").replace('/', "~1")),
            SnapshotDetailPathSegment::Index(index) => pointer.push_str(&index.to_string()),
        }
    }
    pointer
}

pub(super) fn path_id(path: &[SnapshotDetailPathSegment]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in pointer(path).bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sd-{hash:016x}")
}

pub(super) fn static_value_args(path: &str, value_json: &str) -> UiAssemblyResult<UiValue> {
    let value = match value_json {
        "null" => UiValue::Null,
        "true" => UiValue::Bool(true),
        "false" => UiValue::Bool(false),
        "0" => UiValue::Number(0.0),
        "\"\"" => ui_text_value("")?,
        "[]" => UiValue::List(Default::default()),
        "{}" => UiValue::Map(Default::default()),
        _ => return Err(error("ui.snapshot-details.static-value")),
    };
    let path = pointer_argument(path, "path", "pathChunks")?;
    ui_args([path, ("value", value)])
}

pub(super) fn path_args(path: &str) -> UiAssemblyResult<UiValue> {
    ui_args([pointer_argument(path, "path", "pathChunks")?])
}

fn json_encoded_path_args(path: &str) -> UiAssemblyResult<UiValue> {
    ui_args([pointer_argument(path, "path", "pathChunks")?, ("valueEncoding", ui_text_value("json")?)])
}

pub(super) fn pointer_argument(path: &str, direct: &'static str, chunks: &'static str) -> UiAssemblyResult<(&'static str, UiValue)> {
    if let Some(path) = UiText::try_from_str(path) {
        return Ok((direct, UiValue::Text(path)));
    }
    let mut values = UiListBuilder::try_new().ok_or_else(|| arena("path chunk list"))?;
    let mut start = 0;
    while start < path.len() {
        let mut end = (start + ui_contract::UI_TEXT_MAX_BYTES).min(path.len());
        while !path.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return Err(error("ui.snapshot-details.path-chunk-boundary"));
        }
        values.push(UiValue::Text(UiText::try_from_str(&path[start..end]).ok_or_else(|| error("ui.snapshot-details.path-chunk"))?)).map_err(|_| arena("path chunk"))?;
        start = end;
    }
    Ok((chunks, UiValue::List(values.finish())))
}

pub(super) fn path_is_bindable(path: &str) -> bool {
    if UiText::try_from_str(path).is_some() {
        return true;
    }
    let mut start = 0;
    let mut count = 0;
    while start < path.len() {
        let mut end = (start + ui_contract::UI_TEXT_MAX_BYTES).min(path.len());
        while !path.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return false;
        }
        count += 1;
        if count > ui_contract::UI_VALUE_MAX_ITEMS {
            return false;
        }
        start = end;
    }
    true
}

pub(super) fn read_only_value(id: &str, value: SnapshotDetailValue) -> UiAssemblyResult<BuiltNode> {
    let value = match value {
        SnapshotDetailValue::Null => "null".to_string(),
        SnapshotDetailValue::Bool(value) => value.to_string(),
        SnapshotDetailValue::Number(Number::UInt(value)) => value.to_string(),
        SnapshotDetailValue::Number(Number::Int(value)) => value.to_string(),
        SnapshotDetailValue::Number(Number::Float(value)) => value.to_string(),
        SnapshotDetailValue::String(value) => value,
        SnapshotDetailValue::Bytes { len } => format!("{len} B"),
        SnapshotDetailValue::Array => "[]".to_string(),
        SnapshotDetailValue::Object => "{}".to_string(),
    };
    readonly_long_text(id, value)
}

pub(super) fn input_draft_target(controller_id: &str, action_id: &str, address: &str) -> Option<UiText> {
    UiText::try_from_str(&format!("stdio-input.v1/{}/{}/{}/{}/{}/{}", controller_id.len(), controller_id, action_id.len(), action_id, address.len(), address))
}

pub(super) fn set_input(id: &str, path: &str, kind: ui::InputKind, value: &str, title: &str, labels: Labels, controller_id: &str, json_encoded: bool, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let args = if json_encoded { json_encoded_path_args(path)? } else { path_args(path)? };
    let Some(draft_target) = input_draft_target(controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, path) else {
        return action_text_draft(id, value.into(), json_encoded.then(|| "json".into()), SET_SNAPSHOT_VALUE_ACTION_ID, Some(args), labels, publication_revision);
    };
    let mut builder = ui::input(kind).value(text(value)?).commit(text("blur")?).draft_target(draft_target).try_label(title).map_err(|_| error("ui.snapshot-details.input-label"))?.try_id(id).map_err(|_| error("ui.snapshot-details.input-id"))?;
    if let Some(publication_revision) = publication_revision {
        builder = builder.publication_revision(publication_revision);
    }
    bind(builder, ui::Trigger::Commit, controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, args)?.try_build().map_err(|_| error("ui.snapshot-details.input"))
}

fn readonly_long_text(id: &str, value: String) -> UiAssemblyResult<BuiltNode> {
    let surface = TextWindowKit::render_read_only(id, &TextView { text: value, language: None })?;
    ui::column().try_id(id).map_err(|_| error("ui.snapshot-details.long-value-id"))?.try_child(surface).map_err(|_| error("ui.snapshot-details.long-value"))?.try_build().map_err(|_| error("ui.snapshot-details.long-value"))
}

pub(super) fn text_draft(id: &str, value: String, path: Option<&str>, labels: Labels, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let language = path.is_none().then(|| "json".to_string());
    let (action_id, arguments) = match path {
        Some(path) => (SET_SNAPSHOT_VALUE_ACTION_ID, Some(path_args(path)?)),
        None => (REPLACE_SNAPSHOT_SOURCE_ACTION_ID, None),
    };
    action_text_draft(id, value, language, action_id, arguments, labels, publication_revision)
}

pub(super) fn action_text_draft(id: &str, value: String, language: Option<String>, action_id: &str, arguments: Option<UiValue>, labels: Labels, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let publication_revision = publication_revision.ok_or_else(|| error("ui.text-draft.publication-revision"))?;
    let surface = TextWindowKit::render_draft(&TextDraftView {
        surface_id: id.into(),
        text: value,
        language,
        action_id: action_id.into(),
        argument: "value".into(),
        arguments,
        publication_revision,
        apply_label: labels.apply.into(),
        discard_label: labels.discard.into(),
        conflict_label: labels.conflict.into(),
        applying_label: labels.applying.into(),
        cancel_label: labels.cancel.into(),
        failed_label: labels.failed.into(),
        syntax_error_label: labels.syntax_error.into(),
        validation_error_label: labels.validation_error.into(),
        path_label: labels.path.into(),
        line_label: labels.line.into(),
        column_label: labels.column.into(),
    })?;
    ui::column().grow(true).try_id(id).map_err(|_| error("ui.snapshot-details.draft-id"))?.try_child(surface).map_err(|_| error("ui.snapshot-details.draft"))?.try_build().map_err(|_| error("ui.snapshot-details.draft"))
}

pub(super) fn template_control(id: &str, title: &str, path: &str, template: DslValue, insert: bool, labels: Labels, controller_id: &str, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let action_id = if insert { INSERT_SNAPSHOT_VALUE_ACTION_ID } else { SET_SNAPSHOT_VALUE_ACTION_ID };
    let source = snapshot_edit_source(&template);
    if let Some(source) = UiText::try_from_str(&source) {
        return button(id, title, "plus", controller_id, action_id, ui_args([pointer_argument(path, "path", "pathChunks")?, ("value", UiValue::Text(source)), ("valueEncoding", ui_text_value("json")?)])?);
    }
    action_text_draft(id, source, Some("json".into()), action_id, Some(json_encoded_path_args(path)?), labels, publication_revision)
}

pub(super) fn variant_control(id: &str, title: &str, path: &str, template: DslValue, labels: Labels, controller_id: &str, publication_revision: Option<ui_contract::UiPublicationRevision>) -> UiAssemblyResult<BuiltNode> {
    let source = snapshot_edit_source(&template);
    if let Some(source) = UiText::try_from_str(&source) {
        return button(id, title, "replace", controller_id, SET_SNAPSHOT_VALUE_ACTION_ID, ui_args([pointer_argument(path, "path", "pathChunks")?, ("value", UiValue::Text(source)), ("valueEncoding", ui_text_value("json")?)])?);
    }
    action_text_draft(id, source, None, SET_SNAPSHOT_VALUE_ACTION_ID, Some(json_encoded_path_args(path)?), labels, publication_revision)
}
