//! 🧲️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-commands` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use super::*;

/// 📚️ Turns a registered example body into the app snapshot. A `{` body is the JSON a deferred
/// producer emits; anything else is the subset's own DSL.
pub(crate) fn catalogue_snapshot_from_document<S>(body: &str) -> Result<S, Fault>
where
    S: pack::snapshot::ArtifactDsl + semio_framework_value::FromValue,
{
    if body.trim_start().starts_with('{') {
        semio_framework_pack_json::from_json_str(body, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.example.unreadable"), error.to_string()))
    } else {
        <S as pack::snapshot::ArtifactDsl>::parse_dsl(body).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.example.unreadable"), error.to_string()))
    }
}

pub(crate) fn framework_reserved_action_kind(action: &str) -> Option<ActionKind> {
    if !is_framework_reserved_action_id(action) {
        return None;
    }
    if CLIPBOARD_ACTION_IDS.contains(&action) {
        return Some(ActionKind::Clipboard);
    }
    if INTERACTION_ACTION_IDS.contains(&action) || action == semio_framework::HOST_EVENT_ACTION_ID {
        return Some(ActionKind::Interaction);
    }
    if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID || action == CANCEL_TYPED_OPERATION_ACTION_ID {
        return Some(ActionKind::View);
    }
    if action == NOTE_SHELL_COMMAND_ACTION_ID || matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
        return Some(ActionKind::Shell);
    }
    Some(ActionKind::History)
}

/// 🏷️ A noted shell command's history label in every locale: a full `LocalizedLabel`, an `{en, de}` pair, or plain
/// text (locale-invariant data, e.g. a file name); the optional `detail` is appended in every locale. Without a
/// label the command id stands, as data.
pub(crate) fn note_shell_command_label(label: Option<&DslValue>, command_id: &str, detail: Option<&DslValue>) -> LocalizedLabel {
    let base = match label {
        Some(DslValue::String(text)) => LocalizedLabel::data(text.clone()),
        Some(value @ DslValue::Object(_)) => match (value.get("en").and_then(DslValue::as_str), value.get("de").and_then(DslValue::as_str)) {
            (Some(en), Some(de)) => LocalizedLabel::native(en, de),
            _ => <LocalizedLabel as semio_framework_value::FromValue>::from_value(value.clone()).unwrap_or_else(|_| LocalizedLabel::data(command_id)),
        },
        _ => LocalizedLabel::data(command_id),
    };
    match detail.and_then(DslValue::as_str) {
        Some(detail) => LocalizedLabel::from_fn(|terminology, locale| format!("{} - {detail}", base.resolve(terminology, locale))),
        None => base,
    }
}

/// 🎞️ The document-store commands an emission's artifact lane dispatches (design §15): a streamed tick appends to its
/// transaction's open edit (opening it), a commit closes the open edit of its transaction (appending its last operations
/// first) or, with none open, records its operations as one edit carrying the ref, an abort reverts the open edit, and an
/// emission outside a transaction applies as one plain edit. An empty list publishes nothing.
pub(crate) fn artifact_lane_commands<M>(mutations: Vec<M>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::os_store::store_schema::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {
    let open = open.zip(transaction.as_ref()).is_some_and(|(open, transaction)| open.transaction.id == transaction.id);
    let append = |mutations: Vec<M>, transaction: protocol::TransactionRef| (!mutations.is_empty()).then(|| ArtifactCommand::AppendTransaction { mutations, transaction });
    match (transaction, phase) {
        (Some(transaction), TransactionPhase::Stream) => append(mutations, transaction).into_iter().collect(),
        (Some(transaction), TransactionPhase::Commit) if open => {
            let transaction_id = transaction.id.clone();
            append(mutations, transaction).into_iter().chain([ArtifactCommand::CommitTransaction { transaction_id }]).collect()
        }
        (Some(transaction), TransactionPhase::Abort) => open.then(|| ArtifactCommand::AbortTransaction { transaction_id: transaction.id }).into_iter().collect(),
        (_, _) if mutations.is_empty() => Vec::new(),
        (transaction, _) => vec![ArtifactCommand::Apply { mutations, transaction }],
    }
}
