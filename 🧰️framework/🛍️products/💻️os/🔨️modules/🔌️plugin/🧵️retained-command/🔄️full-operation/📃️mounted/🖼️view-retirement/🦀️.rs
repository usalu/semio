//! ♻️ Final retirement for the bounded plain-text action view.
use semio_framework::{TreeWindowRequest, ViewModel, ViewSessionIdentity, ViewWindowInstance};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError};
use std::sync::Arc;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

pub(super) fn compact_demands(view: &Option<Arc<ViewModel>>) -> Option<(RetirementDemand, u64)> {
    let owner = view.as_ref()?;
    if Arc::strong_count(owner) != 1 || Arc::weak_count(owner) != 0 { return None; }
    let ViewModel {
        active_mode_id, active_tool_id, active_utility_by_window_id, active_utility_id,
        active_window_kind_id, extension_input_json, focused_window_id, locale: _, panel_json,
        session_identity, terminology: _, tool_run_trace_cursor_by_window_id,
        tree_viewport_rows: _, tree_windows, window_id, window_instances,
    } = &**owner;
    let rows = tree_windows.len().checked_add(window_instances.len())?;
    if rows > 16 || !active_utility_by_window_id.is_empty() || !tool_run_trace_cursor_by_window_id.is_empty() { return None; }

    let mut release = semio_framework_value::shared_retirement_allocation_bytes::<ViewModel>();
    let fields = [active_mode_id, active_tool_id, active_utility_id, active_window_kind_id, extension_input_json, focused_window_id, panel_json, window_id];
    for text in fields.into_iter().flatten() { release = release.checked_add(text.capacity())?; }
    if let Some(ViewSessionIdentity { user_id, display_name }) = session_identity {
        release = release.checked_add(user_id.capacity())?.checked_add(display_name.capacity())?;
    }
    release = release.checked_add(tree_windows.capacity().checked_mul(size_of::<TreeWindowRequest>())?)?;
    release = release.checked_add(window_instances.capacity().checked_mul(size_of::<ViewWindowInstance>())?)?;
    for TreeWindowRequest { body_key, node_key, open: _, offset: _, rows: _ } in tree_windows {
        release = release.checked_add(body_key.capacity())?.checked_add(node_key.capacity())?;
    }
    for ViewWindowInstance { id, window_kind_id } in window_instances {
        release = release.checked_add(id.capacity())?.checked_add(window_kind_id.capacity())?;
    }
    let copied_bytes = size_of::<Option<Arc<ViewModel>>>();
    Some((RetirementDemand { copy_bytes: copied_bytes, release_bytes: release, depth: 1, ..Default::default() }, 16 + 2 * rows as u64))
}

pub(super) fn compact_close(
    view: &mut Option<Arc<ViewModel>>,
    demand: RetirementDemand,
    work: u64,
    effect: &mut RetainedAllocationEffect<'_>,
) -> Result<RetainedCloneProgress, ValueError> {
    let grant = effect.grant();
    if grant.maximum_items == 0 || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand)
        || view.as_mut().and_then(Arc::get_mut).is_none() || !effect.performed(work)? {
        return Ok(RetainedCloneProgress::default());
    }
    drop(view.take());
    Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })
}
