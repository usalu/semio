//! 📁️ Folders: creating, moving and deleting folders.

use super::{clean_relative, fail, plan_agents_docs_path, refuse_sealed_move, Change, Message, MoveError, Plan, PlanEvent, Workspace};
use semio_framework_repo_events::FolderPayload;

//#region 📁️Folders

/// 📁️ Creates an empty folder.
pub fn plan_folder_create(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if workspace.exists(path) {
        return fail(format!("Folder already exists: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: path.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.create.starting", &payload));
    plan.changes.push(Change::CreateDirectory { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("folder.create.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📁️Created folder: {path}")));
    Ok(plan)
}

/// 🚚️ Moves a folder and rewrites the `AGENTS.md` headings that name it.
pub fn plan_folder_move(workspace: &Workspace, source: &str, target: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(source) {
        return fail(format!("Source folder not found: {source}"));
    }
    if workspace.exists(target) {
        return fail(format!("Target folder already exists: {target}"));
    }
    refuse_sealed_move(workspace, source)?;
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: target.to_string(), from: source.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.move.starting", &payload));
    plan.changes.push(Change::Move { from: clean_relative(source), to: clean_relative(target) });
    if let Some(change) = plan_agents_docs_path(workspace, source, target) {
        plan.changes.push(change);
    }
    plan.events.push(PlanEvent::new("folder.move.ended", &payload));
    plan.messages.push(Message::Success(format!("\n📁️Moved folder: {source} → {target}")));
    Ok(plan)
}

/// 🗑️ Deletes a folder and everything under it.
pub fn plan_folder_delete(workspace: &Workspace, path: &str) -> Result<Plan, MoveError> {
    if !workspace.exists(path) {
        return fail(format!("Folder not found: {path}"));
    }
    let mut plan = Plan::empty();
    let payload = FolderPayload { path: path.to_string(), ..FolderPayload::default() };
    plan.events.push(PlanEvent::new("folder.delete.starting", &payload));
    plan.changes.push(Change::Delete { path: clean_relative(path) });
    plan.events.push(PlanEvent::new("folder.delete.ended", &payload));
    plan.messages.push(Message::Success(format!("\n🗑️ Deleted folder: {path}")));
    Ok(plan)
}

//#endregion 📁️Folders
