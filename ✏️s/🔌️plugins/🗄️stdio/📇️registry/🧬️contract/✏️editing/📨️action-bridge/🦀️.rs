//! 📨️ Action definitions, argument parsing and the bridge from host actions to edit events.

use super::*;

pub(super) fn edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

pub(super) fn snapshot_edit_fault(error: SnapshotEditError) -> Fault {
    let mut fault = Fault::new(FaultOrigin::App, FaultCode::new(error.code), error.message);
    if !error.path.is_empty() {
        fault = fault.with_param("path", error.path);
    }
    fault.span = error.span;
    fault
}

pub(super) fn argument<'a>(args: Option<&'a DslValue>, key: &str) -> Result<&'a DslValue, Fault> {
    let Some(DslValue::Object(entries)) = args else {
        return Err(edit_fault("snapshot-edit.arguments-required", "snapshot edit arguments must be an object"));
    };
    let mut matches = entries.iter().filter(|(candidate, _)| candidate == key);
    let value = matches.next().map(|(_, value)| value).ok_or_else(|| edit_fault("snapshot-edit.argument-missing", format!("snapshot edit argument '{key}' is required")))?;
    if matches.next().is_some() {
        return Err(edit_fault("snapshot-edit.argument-duplicate", format!("snapshot edit argument '{key}' occurs more than once")));
    }
    Ok(value)
}

fn text_argument(args: Option<&DslValue>, key: &str) -> Result<String, Fault> {
    match argument(args, key)? {
        DslValue::String(value) => Ok(value.clone()),
        _ => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{key}' must be text"))),
    }
}

fn optional_argument<'a>(args: Option<&'a DslValue>, key: &str) -> Result<Option<&'a DslValue>, Fault> {
    let Some(DslValue::Object(entries)) = args else {
        return Err(edit_fault("snapshot-edit.arguments-required", "snapshot edit arguments must be an object"));
    };
    let mut matches = entries.iter().filter(|(candidate, _)| candidate == key);
    let value = matches.next().map(|(_, value)| value);
    if matches.next().is_some() {
        return Err(edit_fault("snapshot-edit.argument-duplicate", format!("snapshot edit argument '{key}' occurs more than once")));
    }
    Ok(value)
}

fn pointer_argument(args: Option<&DslValue>, direct: &str, chunks: &str) -> Result<String, Fault> {
    match (optional_argument(args, direct)?, optional_argument(args, chunks)?) {
        (Some(DslValue::String(path)), None) => Ok(path.clone()),
        (None, Some(DslValue::Array(parts))) => {
            let mut bytes = 0usize;
            for part in parts {
                let DslValue::String(part) = part else {
                    return Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{chunks}' must contain only text")));
                };
                bytes =
                    bytes.checked_add(part.len()).filter(|bytes| *bytes <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES).ok_or_else(|| edit_fault("snapshot-edit.path-too-large", format!("snapshot edit argument '{chunks}' exceeds the bounded path extent")))?;
            }
            let mut path = String::with_capacity(bytes);
            for part in parts {
                let DslValue::String(part) = part else { unreachable!("validated path chunk") };
                path.push_str(part);
            }
            Ok(path)
        }
        (Some(_), None) => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{direct}' must be text"))),
        (None, Some(_)) => Err(edit_fault("snapshot-edit.argument-type", format!("snapshot edit argument '{chunks}' must be a text list"))),
        (Some(_), Some(_)) => Err(edit_fault("snapshot-edit.path-shape", format!("snapshot edit arguments must contain exactly one of '{direct}' or '{chunks}'"))),
        (None, None) => Err(edit_fault("snapshot-edit.argument-missing", format!("snapshot edit argument '{direct}' or '{chunks}' is required"))),
    }
}

pub fn snapshot_edit_event_from_action(action: &str, args: Option<&DslValue>) -> Result<Option<SnapshotEditEvent>, Fault> {
    let json_value = || -> Result<DslValue, Fault> {
        let value = argument(args, "value")?;
        if matches!(argument(args, "valueEncoding"), Ok(DslValue::String(encoding)) if encoding == "json") {
            let DslValue::String(source) = value else { return Err(edit_fault("snapshot-edit.argument-type", "a JSON-encoded control value must be text")) };
            return parse_snapshot_json(source).map_err(snapshot_edit_fault);
        }
        Ok(value.clone())
    };
    let event = match action {
        SET_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::SetValue { path: pointer_argument(args, "path", "pathChunks")?, value: json_value()? },
        INSERT_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::InsertValue { path: pointer_argument(args, "path", "pathChunks")?, value: json_value()? },
        REMOVE_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::RemoveValue { path: pointer_argument(args, "path", "pathChunks")? },
        MOVE_SNAPSHOT_VALUE_ACTION_ID => SnapshotEditEvent::MoveValue { from: pointer_argument(args, "from", "fromChunks")?, path: pointer_argument(args, "path", "pathChunks")? },
        RENAME_SNAPSHOT_KEY_ACTION_ID => SnapshotEditEvent::RenameKey { path: pointer_argument(args, "path", "pathChunks")?, key: text_argument(args, "value")? },
        REPLACE_SNAPSHOT_SOURCE_ACTION_ID => SnapshotEditEvent::ReplaceSource { source: text_argument(args, "value")? },
        _ => return Ok(None),
    };
    Ok(Some(event))
}

pub fn snapshot_editing_command_from_action<C>(action: &str, args: Option<&DslValue>, native: impl FnOnce(&str, Option<&DslValue>) -> Result<C, Fault>) -> Result<SnapshotEditingCommand<C>, Fault> {
    match snapshot_edit_event_from_action(action, args)? {
        Some(event) => Ok(SnapshotEditingCommand::Edit(event)),
        None => native(action, args).map(SnapshotEditingCommand::Native),
    }
}

pub fn snapshot_editing_command_id<C>(command: &SnapshotEditingCommand<C>, native: impl FnOnce(&C) -> &'static str) -> &'static str {
    match command {
        SnapshotEditingCommand::Native(command) => native(command),
        SnapshotEditingCommand::Edit(event) => event.action_id(),
    }
}

pub fn snapshot_edit_actions() -> Vec<ActionDefinition> {
    let path = || ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")).min_length(0);
    let path_chunks = || ActionArgDef::text_list("pathChunks", LocalizedLabel::native("Path Chunks", "Pfadsegmente"));
    let value = || ActionArgDef::any("value", LocalizedLabel::native("Value", "Wert")).required();
    let encoding = || ActionArgDef::text("valueEncoding", LocalizedLabel::native("Value Encoding", "Wertkodierung"));
    let mutation = |id, en, de, args| ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args);
    vec![
        mutation(SET_SNAPSHOT_VALUE_ACTION_ID, "Set Value", "Wert setzen", vec![path(), path_chunks(), value(), encoding()]),
        mutation(INSERT_SNAPSHOT_VALUE_ACTION_ID, "Insert Value", "Wert einfügen", vec![path(), path_chunks(), value(), encoding()]),
        mutation(REMOVE_SNAPSHOT_VALUE_ACTION_ID, "Remove Value", "Wert entfernen", vec![path(), path_chunks()]),
        mutation(
            MOVE_SNAPSHOT_VALUE_ACTION_ID,
            "Move Value",
            "Wert verschieben",
            vec![ActionArgDef::text("from", LocalizedLabel::native("From", "Von")).min_length(0), ActionArgDef::text_list("fromChunks", LocalizedLabel::native("From Chunks", "Von-Segmente")), path(), path_chunks()],
        ),
        mutation(RENAME_SNAPSHOT_KEY_ACTION_ID, "Rename Key", "Schlüssel umbenennen", vec![path(), path_chunks(), ActionArgDef::text("value", LocalizedLabel::native("Key", "Schlüssel")).min_length(0).required()]),
        mutation(REPLACE_SNAPSHOT_SOURCE_ACTION_ID, "Replace Source", "Quelltext ersetzen", vec![ActionArgDef::text("value", LocalizedLabel::native("Source", "Quelltext")).required()]),
    ]
}

/// 🧰 Adds the complete snapshot-edit action catalog to an artifact editor manifest.
pub fn snapshot_edit_actions_with(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    snapshot_edit_actions().into_iter().fold(builder, |builder, action| {
        let id = action.id.clone();
        builder.action_with(action).action_interactive_job(id, InteractiveJobClassification::Migrated)
    })
}

pub(super) const SNAPSHOT_EDIT_PAYLOAD_SCHEMA: &str = "semio.stdio.snapshot-edit-command.v1";
pub const SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES: usize = 16 * 1_024 * 1_024;
pub const SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES: usize = 65_536;

pub fn snapshot_edit_source_is_admitted(source: &str) -> bool {
    <SnapshotEditEvent as OpBinary>::encode_op(&SnapshotEditEvent::ReplaceSource { source: source.to_owned() }).is_ok_and(|encoded| encoded.len() <= SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES)
}
pub(super) const SNAPSHOT_EDIT_WORK_CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);
pub(super) const SNAPSHOT_EDIT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: SET_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: INSERT_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: REMOVE_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: MOVE_SNAPSHOT_VALUE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: RENAME_SNAPSHOT_KEY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: REPLACE_SNAPSHOT_SOURCE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];
