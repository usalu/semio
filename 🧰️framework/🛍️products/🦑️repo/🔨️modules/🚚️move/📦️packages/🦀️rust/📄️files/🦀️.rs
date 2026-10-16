//! 📄️ Files: creating, moving and deleting files.

use super::{clean_relative, fail, plan_agents_docs_path, refuse_sealed_move, Change, Message, MoveError, Plan, PlanEvent, Workspace};
use semio_framework_repo_events::FilePayload;

//#region 📄️Files

/// 📄️ Creates a file holding `header`, the caller's rendered file header.
pub fn plan_file_create(workspace: &Workspace, path: &str, header: &str) -> Result<Plan, MoveError> {
    if workspace.exists(path) {
        return fail(format!("File already exists: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FilePayload { path: path.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.create.starting", &payload));
    plan.changes.push(Change::Write { path: clean_relative(path), content: header.to_string() });
    plan.events.push(PlanEvent::new("file.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📄️Created file: {path}")));
    Ok(plan)
}

/// 🚚️ Moves a file and rewrites the `AGENTS.md` headings that name it.
pub fn plan_file_move(workspace: &Workspace, source: &str, target: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(source) {
        return fail(format!("Source file not found: {source}"));
    }
    if workspace.exists(target) {
        return fail(format!("Target file already exists: {target}"));
    }
    refuse_sealed_move(workspace, source)?;
    let mut plan = Plan::empty();
    let payload = FilePayload { path: target.to_string(), from: source.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.move.starting", &payload));
    plan.changes.push(Change::Move { from: clean_relative(source), to: clean_relative(target) });
    if let Some(change) = plan_agents_docs_path(workspace, source, target) {
        plan.changes.push(change);
    }
    plan.events.push(PlanEvent::new("file.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📄️Moved file: {source} → {target}")));
    Ok(plan)
}

/// 🗑️ Deletes a file.
pub fn plan_file_delete(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(path) {
        return fail(format!("File not found: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FilePayload { path: path.to_string(), ..FilePayload::default() };
    plan.events.push(PlanEvent::new("file.delete.starting", &payload));
    plan.changes.push(Change::Delete { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("file.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted file: {path}")));
    Ok(plan)
}

//#endregion 📄️Files
