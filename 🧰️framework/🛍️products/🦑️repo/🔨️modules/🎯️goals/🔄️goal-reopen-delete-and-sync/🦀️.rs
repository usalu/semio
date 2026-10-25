//! 🔄️ Goal reopen, delete and sync: the reopen and delete operations, the management synchronisation, retitle, relocate and the event emission. See [`super`].

use super::*;

//#region 🔓️Lifecycle
impl<'a> Goals<'a> {
    /// 🔓️ Reopens a goal, refusing a goal that is already open, and replaces the interaction
    /// members (prompt, llm, client, effort) with the ones the reopen carries.
    pub fn reopen(&self, input: &GoalReopenInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if goal.status == GoalStatus::Open {
            return Err(GoalError::AlreadyInState("open"));
        }
        goal.status = GoalStatus::Open;
        if let Some(title) = &input.title {
            self.retitle(&mut goal, title)?;
        }
        if let Some(description) = &input.description {
            goal.description = description.clone();
        }
        if let Some(due_date) = &input.due_date {
            goal.dates.due = due_date.clone();
        }
        if let Some(parent) = &input.parent {
            goal.parent = parent.clone();
        }
        goal.prompt = input.prompt.clone();
        goal.llm = input.llm.clone();
        goal.client = input.client.clone();
        if !input.effort.is_empty() {
            goal.effort = model::resolve_allowed_effort(&input.effort)?;
        }
        if !input.no_management {
            self.sync_management(&goal, "open", &goal.dates.due.clone())?;
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_REOPEN_ENDED,
            &semio_framework_repo_events::GoalReopenPayload {
                id: goal.id.clone(),
                prompt: input.prompt.clone(),
                client: input.client.clone(),
                llm: input.llm.clone(),
                effort: goal.effort.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    /// 🗑️ Deletes a goal and everything below it, after retiring its milestone or issue.
    pub fn delete(&self, input: &GoalDeleteInput) -> Result<bool, GoalError> {
        let goal = self.read(&input.id)?;
        if !input.no_management {
            if let Some(data) = goal.management.clone() {
                if is_root_goal(&goal.id) && !data.milestone.is_empty() {
                    if let Ok(number) = parse_milestone_number(&data.milestone) {
                        self.management.delete_milestone(number)?;
                    }
                } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
                    let number = data.issue.rsplit('/').next().unwrap_or_default().to_string();
                    self.management.delete_issue(&number)?;
                }
            }
        }
        self.store.remove(&goal.id)?;
        Ok(true)
    }

    pub(super) fn sync_management(&self, goal: &Goal, state: &str, due: &str) -> Result<(), GoalError> {
        let Some(data) = goal.management.clone() else { return Ok(()) };
        if is_root_goal(&goal.id) && !data.milestone.is_empty() {
            if let Ok(number) = parse_milestone_number(&data.milestone) {
                self.management.update_milestone(number, &goal.title, &goal.description, state, due)?;
            }
        } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
            self.management.update_goal_issue(&data.issue, &goal.title, &goal.description)?;
        }
        Ok(())
    }

    pub(super) fn retitle(&self, goal: &mut Goal, title: &str) -> Result<(), GoalError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(GoalError::Title("goal title is required"));
        }
        let slug = identity::slugify(title);
        if title == slug {
            return Err(GoalError::Title("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug"));
        }
        if title == slug.to_lowercase() {
            return Err(GoalError::Title("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug"));
        }
        let target = compose_goal_id(&goal.parent, title);
        self.relocate(goal, &target)?;
        goal.title = title.to_string();
        Ok(())
    }

    pub(super) fn relocate(&self, goal: &mut Goal, target: &str) -> Result<(), GoalError> {
        if target == goal.id {
            return Ok(());
        }
        if self.store.exists(target) {
            return Err(GoalError::AlreadyExists(target.to_string()));
        }
        self.store.rename(&goal.id, target)?;
        goal.id = target.to_string();
        Ok(())
    }

    pub(super) fn emit<T: serde::Serialize>(&self, kind: &str, payload: &T) {
        let Ok(value) = serde_json::to_value(payload) else { return };
        self.emitter.emit(kind, "repo-cli", &value);
    }
}
//#endregion 🔓️Lifecycle
