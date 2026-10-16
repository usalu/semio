//! ⏳️ Local operation status and exact cancellation controls shared by artifact editors.
use crate::app_views::CANCEL_TYPED_OPERATION_ACTION_ID;
use crate::app_views::ArtifactOperationProgress;
use super::*;

pub fn decode_operation_cancellation(value: &DslValue) -> Option<(u64, u64)> {
    let DslValue::Object(fields) = value else { return None };
    if fields.len() != 2 {
        return None;
    }
    let read = |key: &str| {
        let text = fields.iter().find(|(name, _)| name == key)?.1.as_str()?;
        if text.len() != 16 || !text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
            return None;
        }
        u64::from_str_radix(text, 16).ok()
    };
    Some((read("operationId")?, read("generation")?))
}

pub fn operation_progress_text(locale: Locale, completed: u64, cancelling: bool) -> String {
    let (state, units) = match locale {
        Locale::En => (if cancelling { "Cancelling" } else { "Working" }, "Completed units"),
        Locale::De => (if cancelling { "Wird abgebrochen" } else { "Wird ausgeführt" }, "Abgeschlossene Schritte"),
    };
    format!("{state} · {units}: {completed}")
}

/// 🎯️ The scope an operation-progress change dirties: the app's declared scope (read only when something changed), never
/// wider, and nothing when it declares none.
pub fn operation_progress_dirty_scope(changed: bool, declared: impl FnOnce() -> UiDirtyScope) -> Option<UiDirtyScope> {
    changed.then(declared).filter(|scope| *scope != UiDirtyScope::None)
}

pub use semio_framework_plugin_decl_app::operation_progress::cancellation_result_lane;

pub fn operation_cancellation_target_admitted(present: bool, terminal: bool, cancellable: bool) -> bool {
    present && !terminal && cancellable
}

/// 📶️ A localized, keyboard-accessible control group observing the operation's current lease.
pub fn operation_progress_controls(status: &[ArtifactOperationProgress], controller: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    use semio_framework_ui_contract::Label as UiLabel;
    let mut groups = BuiltChildren::default();
    for operation in status {
        let key = format!("framework.operation.{:016x}", operation.operation_id);
        let label = operation.label.resolve(Terminology::Native, locale).to_string();
        let caption = operation_progress_text(locale, operation.completed_units, operation.cancelling);
        let text = text(UiLabel::try_from(caption.as_str()).map_err(|_| ui_assembly_error("operation-progress.caption"))?)
            .live(Liveness::Polite)
            .try_id(format!("{key}.status"))
            .map_err(|_| ui_assembly_error("operation-progress.status-id"))?
            .try_build()
            .map_err(|_| ui_assembly_error("operation-progress.status"))?;
        let progress = progress(operation.completed_units as f64, UiLabel::try_from(caption.as_str()).map_err(|_| ui_assembly_error("operation-progress.value"))?)
            .try_id(format!("{key}.progress"))
            .map_err(|_| ui_assembly_error("operation-progress.progress-id"))?
            .try_build()
            .map_err(|_| ui_assembly_error("operation-progress.progress"))?;
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("operation-progress.args"))?;
        for (name, value) in [("operationId", operation.operation_id), ("generation", operation.generation)] {
            args.push(name.to_owned(), UiValue::Text(UiText::try_from_str(&format!("{value:016x}")).ok_or_else(|| ui_assembly_error("operation-progress.identity"))?)).map_err(|_| ui_assembly_error("operation-progress.args"))?;
        }
        let cancel = button(
            UiLabel::try_from(match locale {
                Locale::En => "Cancel",
                Locale::De => "Abbrechen",
            })
            .map_err(|_| ui_assembly_error("operation-progress.cancel-label"))?,
        )
        .disabled(operation.cancelling)
        .try_id(format!("{key}.cancel"))
        .map_err(|_| ui_assembly_error("operation-progress.cancel-id"))?
        .try_on_with(Trigger::Activate, ActionId::try_v1(controller, CANCEL_TYPED_OPERATION_ACTION_ID).ok_or_else(|| ui_assembly_error("operation-progress.action"))?, UiValue::Map(args.finish()))
        .map_err(|_| ui_assembly_error("operation-progress.binding"))?
        .try_build()
        .map_err(|_| ui_assembly_error("operation-progress.cancel"))?;
        let group = column()
            .try_id(key)
            .map_err(|_| ui_assembly_error("operation-progress.group-id"))?
            .try_label(label)
            .map_err(|_| ui_assembly_error("operation-progress.group-label"))?
            .try_children([text, progress, cancel])
            .map_err(|_| ui_assembly_error("operation-progress.children"))?
            .try_build()
            .map_err(|_| ui_assembly_error("operation-progress.group"))?;
        groups.try_push(group).map_err(|_| ui_assembly_error("operation-progress.groups"))?;
    }
    column()
        .try_id("framework.operations")
        .map_err(|_| ui_assembly_error("operation-progress.id"))?
        .try_children(groups)
        .map_err(|_| ui_assembly_error("operation-progress.groups"))?
        .try_build()
        .map_err(|_| ui_assembly_error("operation-progress.root"))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
