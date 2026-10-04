//! ⏳️ Local operation status and exact cancellation controls shared by artifact editors.
use super::*;

pub const CANCEL_TYPED_OPERATION_ACTION_ID: &str = "cancelTypedOperation";

#[derive(Clone, Debug)]
pub struct ArtifactOperationProgress {
    pub operation_id: u64,
    pub generation: u64,
    pub label: LocalizedLabel,
    pub completed_units: u64,
    pub cancelling: bool,
}

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

pub fn cancellation_action_definition() -> ActionDefinition {
    ActionDefinition::resumable_framework(CANCEL_TYPED_OPERATION_ACTION_ID, LocalizedLabel::native("Cancel Operation", "Vorgang abbrechen"), ActionKind::View, "square").with_in_palette(false)
}

/// 🎯️ The scope an operation-progress change dirties: the app's declared scope (read only when something changed), never
/// wider, and nothing when it declares none.
pub fn operation_progress_dirty_scope(changed: bool, declared: impl FnOnce() -> UiDirtyScope) -> Option<UiDirtyScope> {
    changed.then(declared).filter(|scope| *scope != UiDirtyScope::None)
}

pub fn cancellation_result_lane(user_requested: bool, worker_fault: bool) -> TypedOperationResultLane {
    if user_requested && !worker_fault {
        TypedOperationResultLane::Terminal
    } else {
        TypedOperationResultLane::Fault
    }
}

pub fn operation_cancellation_target_admitted(present: bool, terminal: bool, cancellable: bool) -> bool {
    present && !terminal && cancellable
}

/// 📶️ A localized, keyboard-accessible control group observing the operation's current lease.
pub fn operation_progress_controls(status: &[ArtifactOperationProgress], controller: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    use semio_framework_ui_contract::{self as ui, Label as UiLabel};
    let mut groups = BuiltChildren::default();
    for operation in status {
        let key = format!("framework.operation.{:016x}", operation.operation_id);
        let label = operation.label.resolve(Terminology::Native, locale).to_string();
        let caption = operation_progress_text(locale, operation.completed_units, operation.cancelling);
        let text = ui::text(UiLabel::try_from(caption.as_str()).map_err(|_| ui_assembly_error("operation-progress.caption"))?)
            .live(Liveness::Polite)
            .try_id(format!("{key}.status"))
            .map_err(|_| ui_assembly_error("operation-progress.status-id"))?
            .try_build()
            .map_err(|_| ui_assembly_error("operation-progress.status"))?;
        let progress = ui::progress(operation.completed_units as f64, UiLabel::try_from(caption.as_str()).map_err(|_| ui_assembly_error("operation-progress.value"))?)
            .try_id(format!("{key}.progress"))
            .map_err(|_| ui_assembly_error("operation-progress.progress-id"))?
            .try_build()
            .map_err(|_| ui_assembly_error("operation-progress.progress"))?;
        let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("operation-progress.args"))?;
        for (name, value) in [("operationId", operation.operation_id), ("generation", operation.generation)] {
            args.push(name.to_owned(), UiValue::Text(UiText::try_from_str(&format!("{value:016x}")).ok_or_else(|| ui_assembly_error("operation-progress.identity"))?)).map_err(|_| ui_assembly_error("operation-progress.args"))?;
        }
        let cancel = ui::button(
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
        let group = ui::column()
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
    ui::column()
        .try_id("framework.operations")
        .map_err(|_| ui_assembly_error("operation-progress.id"))?
        .try_children(groups)
        .map_err(|_| ui_assembly_error("operation-progress.groups"))?
        .try_build()
        .map_err(|_| ui_assembly_error("operation-progress.root"))
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    pub(super) fn operation_progress_snapshot(&self) -> Vec<ArtifactOperationProgress> {
        let mut result = Vec::new();
        self.tool_operations.each_id(|id| {
            let Some(operation) = self.tool_operations.get(id) else { return };
            let Some(completed_units) = operation.progress.filter(|_| !operation.terminal_seen) else { return };
            let Some(definition) = self.registry.get(&operation.verb) else { return };
            result.push(ArtifactOperationProgress {
                operation_id: operation.operation.operation.0,
                generation: operation.operation.generation.0,
                label: definition.label.clone(),
                completed_units,
                cancelling: operation.cancellation_lease.as_ref().is_some_and(|lease| lease.cancel_token().is_cancelled_now()),
            });
        });
        result
    }

    /// ⏳️ The app's declared operation-progress scope ([`ArtifactApp::operation_progress_scope`]) once an operation's
    /// progress changed or it retired; `None` when nothing changed or the app renders no progress.
    pub(super) fn take_operation_progress_scope(&mut self) -> Option<UiDirtyScope> {
        let mut changed = std::mem::take(&mut self.operation_progress_retired);
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            if let Some((_, operation)) = self.tool_operations.entry_mut(index) {
                changed |= std::mem::take(&mut operation.progress_pending);
            }
        }
        operation_progress_dirty_scope(changed, A::operation_progress_scope)
    }

    pub(super) async fn dispatch_operation_cancellation(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let (id, generation) = args.and_then(decode_operation_cancellation).ok_or_else(|| plugin_sdk_fault("operation cancellation requires exact operationId and generation"))?;
        let operation = self.tool_operations.get_mut(id).ok_or_else(|| plugin_sdk_fault("operation cancellation target is no longer live"))?;
        if operation.meta.instance_id != meta.instance_id || operation.meta.actor != meta.actor || operation.operation.generation.0 != generation {
            return Err(plugin_sdk_fault("operation cancellation authority does not match"));
        }
        if !operation_cancellation_target_admitted(true, operation.terminal_seen, operation.cancellation_lease.is_some()) {
            return Err(plugin_sdk_fault("operation cancellation target is no longer cancellable"));
        }
        operation.user_cancel_requested = true;
        operation.cancellation_lease.as_ref().expect("admitted cancellation has a lease").cancel();
        operation.progress_pending = true;
        Ok(Self::empty_result(CANCEL_TYPED_OPERATION_ACTION_ID, meta, Vec::new(), Vec::new(), A::operation_progress_scope()).await)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
