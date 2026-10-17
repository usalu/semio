//! 🧲️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-commands` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin, NOTE_SHELL_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_operation::CANCEL_TYPED_OPERATION_ACTION_ID;
use semio_framework_plugin_job::reserved_jobs::{CLIPBOARD_ACTION_IDS, is_framework_reserved_action_id};
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use std::future::Future;
use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_ui_locale::LocalizedLabel;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

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

#[doc(hidden)]
pub fn framework_reserved_action_kind(action: &str) -> Option<ActionKind> {
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

