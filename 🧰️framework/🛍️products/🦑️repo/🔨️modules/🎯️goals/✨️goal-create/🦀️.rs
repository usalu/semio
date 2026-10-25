//! ✨️ Goal create: the creation of a goal and the opening of its management milestone or issue. See [`super`].

use super::*;

//#region 🔓️Lifecycle
impl<'a> Goals<'a> {
    /// 🆕️ Opens a goal: validates the inputs, resolves the slug vocabularies, refuses a duplicate
    /// identifier, asks the management provider for the milestone or issue the depth calls for,
    /// persists the document and emits `goal.open.ended`.
    pub fn create(&self, input: &GoalCreateInput) -> Result<Goal, GoalError> {
        if input.title.is_empty() {
            return Err(GoalError::Missing("title"));
        }
        if input.description.is_empty() {
            return Err(GoalError::Missing("description"));
        }
        if input.prompt.is_empty() {
            return Err(GoalError::Missing("prompt"));
        }
        if input.due_date.is_empty() {
            return Err(GoalError::Missing("due date"));
        }
        if input.llm.is_empty() {
            return Err(GoalError::Missing("llm"));
        }
        if input.client.is_empty() {
            return Err(GoalError::Missing("client"));
        }
        let llm = model::resolve_allowed_llm(&input.llm)?;
        let effort = if input.effort.is_empty() { String::new() } else { model::resolve_allowed_effort(&input.effort)? };
        let client = model::resolve_allowed_client(&input.client)?;
        let id = compose_goal_id(&input.parent, &input.title);
        if self.store.exists(&id) {
            return Err(GoalError::AlreadyExists(id));
        }
        let mut goal = Goal {
            title: input.title.clone(),
            description: input.description.clone(),
            prompt: input.prompt.clone(),
            status: GoalStatus::Open,
            summary: String::new(),
            due_date: String::new(),
            dates: GoalDates { due: input.due_date.clone() },
            client,
            llm: llm.clone(),
            effort: effort.clone(),
            parent: input.parent.clone(),
            management: None,
            id: id.clone(),
            path: String::new(),
        };
        if !input.no_management {
            goal.management = Some(self.open_management(&id, input)?);
        }
        self.store.write(&id, &encode_goal(&goal)?)?;
        self.emit(
            semio_framework_repo_events::GOAL_OPEN_ENDED,
            &semio_framework_repo_events::GoalOpenPayload {
                id: goal.id.clone(),
                title: goal.title.clone(),
                description: goal.description.clone(),
                llm,
                effort,
                parent: goal.parent.clone(),
                author: self.author.clone(),
            },
        );
        Ok(goal)
    }

    fn open_management(&self, id: &str, input: &GoalCreateInput) -> Result<GoalManagementData, GoalError> {
        if is_root_goal(id) {
            if !input.milestone.is_empty() {
                return Ok(GoalManagementData { milestone: input.milestone.clone(), issue: String::new() });
            }
            let number = self.management.create_milestone(&input.title, &input.description)?;
            if !input.due_date.is_empty() {
                let _ = self.management.update_milestone(number, "", "", "", &input.due_date);
            }
            return Ok(GoalManagementData { milestone: format!("{}/milestone/{number}", self.management.repo_url()), issue: String::new() });
        }
        if is_first_gen_goal(id) {
            let milestone = self.read(&root_goal_id(id)).ok().and_then(|root| root.management).map(|data| data.milestone).filter(|value| !value.is_empty());
            let milestone = milestone.and_then(|value| parse_milestone_number(&value).ok());
            let issue = self.management.create_goal_issue(&input.title, &input.description, milestone)?;
            return Ok(GoalManagementData { milestone: String::new(), issue });
        }
        let issue = self.management.create_goal_issue(&input.title, &input.description, None)?;
        if let Ok(parent) = self.read(&parent_goal_id(id)) {
            if let Some(data) = parent.management {
                if !data.issue.is_empty() {
                    let _ = self.management.add_sub_issue(&data.issue, &issue);
                }
            }
        }
        Ok(GoalManagementData { milestone: String::new(), issue })
    }
}
//#endregion 🔓️Lifecycle
