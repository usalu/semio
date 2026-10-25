//! ✏️ Goal change and close: the change and close operations of the goal aggregate. See [`super`].

use super::*;

//#region 🔓️Lifecycle
impl<'a> Goals<'a> {
    /// ♻️ Changes a goal: retitling moves the document, re-parenting moves it too, and the
    /// management provider is told about the new title, body, state and due date.
    pub fn change(&self, input: &GoalChangeInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if let Some(title) = &input.title {
            self.retitle(&mut goal, title)?;
        }
        if let Some(description) = &input.description {
            goal.description = description.clone();
        }
        if let Some(due_date) = &input.due_date {
            goal.dates.due = due_date.clone();
        }
        if let Some(llm) = &input.llm {
            goal.llm = model::resolve_allowed_llm(llm)?;
        }
        if let Some(effort) = &input.effort {
            goal.effort = model::resolve_allowed_effort(effort)?;
        }
        if let Some(parent) = &input.parent {
            let slug = goal.id.rsplit('/').next().unwrap_or(&goal.id).to_string();
            let target = if parent.is_empty() { slug } else { format!("{parent}/{slug}") };
            self.relocate(&mut goal, &target)?;
            goal.parent = parent.clone();
        }
        if !input.no_management {
            self.sync_management(&goal, goal.status.as_str(), &goal.dates.due.clone())?;
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_CHANGE_ENDED,
            &semio_framework_repo_events::GoalChangePayload {
                id: goal.id.clone(),
                title: input.title.clone(),
                description: input.description.clone(),
                llm: input.llm.clone(),
                effort: input.effort.clone(),
                parent: input.parent.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    /// 📪️ Closes a goal, refusing a goal that is already closed.
    pub fn close(&self, input: &GoalCloseInput) -> Result<Goal, GoalError> {
        let mut goal = self.read(&input.id)?;
        if goal.status == GoalStatus::Closed {
            return Err(GoalError::AlreadyInState("closed"));
        }
        goal.status = GoalStatus::Closed;
        goal.summary = input.summary.clone();
        if !input.no_management {
            if let Some(data) = goal.management.clone() {
                if is_root_goal(&goal.id) && !data.milestone.is_empty() {
                    if let Ok(number) = parse_milestone_number(&data.milestone) {
                        self.management.update_milestone(number, &goal.title, &goal.description, "closed", "")?;
                    }
                } else if !is_root_goal(&goal.id) && !data.issue.is_empty() {
                    self.management.close_issue(&data.issue)?;
                }
            }
        }
        self.store.write(&goal.id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_CLOSE_ENDED,
            &semio_framework_repo_events::GoalClosePayload { id: goal.id.clone(), summary: goal.summary.clone(), author: self.author.clone() },
        );
        Ok(goal)
    }
}
//#endregion 🔓️Lifecycle
