//! 🎒️ Shared archive edit targeting and mutation rules for every ZIP dialect.

use crate::schema::mutations::set_archive_comment;
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_plugin::app::{TextDraftView, TextWindowKit};
use semio_framework_plugin::tree_window_indexed_section;
use semio_framework_plugin::tree_window_item;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_ui_contract as ui;
use semio_framework_ui_locale::Locale;

#[path = "🧵️retained/🦀️.rs"]
pub mod retained;

pub const COMMENT_NODE_ID: &str = "comment";
pub const ENTRY_NODE_PREFIX: &str = "entry:";
pub const MAXIMUM_TEXT_BYTES: usize = u16::MAX as usize;

/// 🏷️ Names remain addressed across entry reordering; a concurrent rename invalidates the old target.
pub fn entry_node_id(name: &str) -> String {
    format!("{ENTRY_NODE_PREFIX}{}", semio_framework_hash::hash_bytes(name.as_bytes()))
}

fn fault(code: &'static str, message: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 📥️ Reads the exact schema-owned archive draft payload.
pub fn edit_arguments(args: Option<&semio_framework_value::DslValue>) -> Result<(String, String, String), Fault> {
    if let Some(semio_framework_value::DslValue::Object(fields)) = args {
        if fields.iter().any(|(key, _)| !["nodeId", "value", "revision", "windowId"].contains(&key.as_str())) {
            return Err(fault("stdio.zip.argument-unknown", "the archive edit contains an unknown argument"));
        }
    }
    let value = semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?;
    validate_text(&value)?;
    Ok((semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "nodeId")?, value, semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?))
}

fn validate_text(value: &str) -> Result<(), Fault> {
    if value.len() > MAXIMUM_TEXT_BYTES {
        return Err(fault("stdio.zip.text-too-large", "archive names and comments cannot exceed 65,535 UTF-8 bytes"));
    }
    Ok(())
}

/// ✏️ Publishes one reversible edit only when the exact archive target remains unambiguous.
pub fn edit_node(snapshot: &ZipSnapshot, node_id: &str, value: &str, revision: &str) -> Result<Emit<ZipMutation>, Fault> {
    validate_text(value)?;
    if node_id == COMMENT_NODE_ID {
        validate_text(&snapshot.comment)?;
        require_revision(&snapshot.comment, revision)?;
        return Ok(if snapshot.comment == value {
            Emit::default()
        } else {
            Emit {
                artifact_mutations: vec![ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment {
                    comment: value.into(),
                    comment_utf8: crate::standards::v2_0::subsets::base::io::archive_comment_utf8_after_edit(snapshot.comment_utf8, value),
                })],
                ..Default::default()
            }
        });
    }
    let mut cursor = retained::ArchiveTextCursor::default();
    cursor.bind(snapshot, node_id, value, revision)?;
    loop {
        if let Some(emit) = cursor.advance_bound(snapshot, node_id, value, revision)? {
            return Ok(emit);
        }
    }
}

/// 🔐️ The token an agent's omitted `revision` is admitted against: the addressed field's saved text, as its draft binding
/// carries it (`draft`); a missing or ambiguous entry is refused exactly as the edit itself would refuse it.
pub fn agent_target_revision(snapshot: &ZipSnapshot, args: &semio_framework_value::DslValue) -> Result<Option<String>, Fault> {
    let node_id = semio_s_artifact_stdio_contract::window_kit_required_text_argument(Some(args), "nodeId")?;
    if node_id == COMMENT_NODE_ID {
        return Ok(Some(text_revision(&snapshot.comment)));
    }
    let mut named = snapshot.entries.iter().filter(|entry| entry_node_id(&entry.name) == node_id);
    match (named.next(), named.next()) {
        (Some(entry), None) => Ok(Some(text_revision(&entry.name))),
        (Some(_), Some(_)) => Err(fault("stdio.zip.target-ambiguous", "entries with identical names must be disambiguated before renaming")),
        (None, _) => Err(fault("stdio.zip.target-missing", "the archive entry changed or no longer exists")),
    }
}

/// 🔐️ Guards an archive draft against changes to its persisted text.
pub fn text_revision(value: &str) -> String {
    semio_framework_hash::hash_bytes(value.as_bytes())
}

fn require_revision(value: &str, revision: &str) -> Result<(), Fault> {
    if text_revision(value) == revision {
        Ok(())
    } else {
        Err(fault("stdio.zip.draft-conflict", "the saved archive text changed while this local draft was open"))
    }
}

fn ui_error() -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.zip.edit-controls", "archive edit controls exceed the UI admission bounds")
}

fn draft(surface_id: &str, node_id: &str, value: &str, locale: Locale, publication_revision: ui::UiPublicationRevision) -> UiAssemblyResult<BuiltNode> {
    let mut arguments = UiMapBuilder::try_new().ok_or_else(ui_error)?;
    for (key, value) in [("nodeId", node_id.to_string()), ("revision", text_revision(value))] {
        arguments.try_insert(key.into(), UiValue::Text(UiText::try_from_str(&value).ok_or_else(ui_error)?)).map_err(|_| ui_error())?;
    }
    let (apply, discard, conflict, applying, cancel, failed, syntax_error, validation_error, path, line, column) = match locale {
        Locale::En => ("Apply", "Discard", "The saved archive text changed. Reconcile before applying.", "Applying…", "Cancel", "The edit failed. Your draft is preserved.", "The source contains invalid syntax.", "The draft does not match the document format.", "Path", "Line", "Column"),
        Locale::De => ("Anwenden", "Verwerfen", "Der gespeicherte Archivtext hat sich geändert. Bitte neu abgleichen.", "Wird angewendet …", "Abbrechen", "Änderung fehlgeschlagen. Der Entwurf bleibt erhalten.", "Der Quelltext enthält ungültige Syntax.", "Der Entwurf entspricht nicht dem Dokumentformat.", "Pfad", "Zeile", "Spalte"),
    };
    TextWindowKit::render_draft(&TextDraftView {
        surface_id: surface_id.into(),
        text: value.into(),
        language: None,
        action_id: "set-node".into(),
        argument: "value".into(),
        arguments: Some(UiValue::Map(arguments.finish())),
        publication_revision,
        apply_label: apply.into(),
        discard_label: discard.into(),
        conflict_label: conflict.into(),
        applying_label: applying.into(),
        cancel_label: cancel.into(),
        failed_label: failed.into(),
        syntax_error_label: syntax_error.into(),
        validation_error_label: validation_error.into(),
        path_label: path.into(),
        line_label: line.into(),
        column_label: column.into(),
    })
}

/// 🪟️ Presents prefilled comment and entry-name drafts only for the visible archive rows.
pub fn render(snapshot: &ZipSnapshot, windows: &TreeWindows<'_>, locale: Locale, publication_revision: ui::UiPublicationRevision) -> UiAssemblyResult<BuiltNode> {
    tree_window_indexed_section(windows, "archive-fields", ui::Label::default(), true, snapshot.entries.len().saturating_add(1), |index| {
        let (node_id, title, value) = if index == 0 {
            (
                COMMENT_NODE_ID.into(),
                match locale {
                    Locale::En => "Archive comment",
                    Locale::De => "Archivkommentar",
                }
                .into(),
                snapshot.comment.as_str(),
            )
        } else {
            let entry = &snapshot.entries[index - 1];
            (entry_node_id(&entry.name), entry.name.clone(), entry.name.as_str())
        };
        let key = format!("archive-field-{index}");
        let item = ui::tree_item(ui::Label(UiText::clipped(&title))).try_id(&key).map_err(|_| ui_error())?;
        tree_window_item(windows, item, &key, index == 0, &[value], |value| draft(&format!("archive-draft-{index}"), &node_id, value, locale, publication_revision))
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
