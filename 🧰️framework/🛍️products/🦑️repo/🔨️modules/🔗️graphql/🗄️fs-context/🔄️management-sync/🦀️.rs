//! 🔄️ Management sync of the production context: the management synchronisation, the section record and label pruning and the goal sync. See [`super`].

use super::*;

//#region 🗄️FsRepoContext

/// 💗️ The section record a section mutation answers with.
pub(super) fn section_record(id: String, name: &str, path: &str, file: &str) -> Section {
    Section { id, name: name.to_string(), path: path.to_string(), file_path: file.to_string(), emoji: String::new(), start_line: 0, end_line: 0, start_index: 0, end_index: 0, children: Vec::new(), definitions: Vec::new() }
}

/// 🏷️ Removes every technology label an issue carries that the catalog no longer declares.
fn prune_labels(provider: &ManagementProviders, issue: &str, labels: &[ManagementLabel], valid: &[String]) {
    let stale: Vec<String> = labels.iter().map(|label| label.name.clone()).filter(|name| name.starts_with('@') && !valid.contains(name)).collect();
    if stale.is_empty() {
        return;
    }
    println!("Removing invalid technology labels from issue {issue}: {stale:?}");
    if let Err(error) = ManagementProvider::remove_labels(provider, issue, &stale) {
        eprintln!("Warning: Failed to remove labels from GitHub issue {issue}: {}", error.message);
    }
}

impl FsRepoContext {
    pub(super) fn folder_create_record(&self, path: &str) -> Folder {
        Folder {
            id: String::new(),
            path: path.to_string(),
            uri: self.codebase.folder_uri(path),
            name: semio_framework_repo_identity::base_of(path),
            parent_id: None,
            bundle_id: None,
            kind: self.codebase.derive_folder_kind(path),
            emoji: String::new(),
            ignored: false,
            generated: semio_framework_repo_codebase::is_generated_folder(path),
        }
    }

    fn sync_goal(&self, provider: &ManagementProviders, goal: &mut Goal) {
        if semio_framework_repo_goals::is_root_goal(&goal.id) {
            if let Some(management) = goal.management.as_mut().filter(|management| !management.issue.is_empty()) {
                println!("Migrating root goal {}: removing issue reference", goal.id);
                management.issue.clear();
            }
            match self.ensure_goal_milestone(provider, goal) {
                Err(error) => eprintln!("Warning: Failed to ensure milestone for root goal {}: {error}", goal.id),
                Ok(Some(milestone)) => {
                    let state = if goal.status == GoalStatus::Closed { "closed" } else { "open" };
                    if let Err(error) = ManagementProvider::update_milestone(provider, milestone.number, &goal.title, &goal.description, state, &goal.dates.due) {
                        eprintln!("Warning: Failed to update milestone for root goal {}: {}", goal.id, error.message);
                    }
                }
                Ok(None) => {}
            }
            return;
        }
        if let Some(management) = goal.management.as_mut().filter(|management| !management.milestone.is_empty()) {
            println!("Migrating child goal {}: removing milestone", goal.id);
            if let Some(number) = parse_milestone_number(&management.milestone) {
                if let Err(error) = ManagementProvider::delete_milestone(provider, number) {
                    eprintln!("Warning: Failed to delete milestone {number} for child goal {}: {}", goal.id, error.message);
                }
            }
            management.milestone.clear();
        }
        let issue = goal.management.as_ref().map(|management| management.issue.clone()).unwrap_or_default();
        if issue.is_empty() {
            let milestone = if semio_framework_repo_goals::is_first_gen_goal(&goal.id) {
                self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&goal.id))).ok().and_then(|root| root.management).and_then(|management| parse_milestone_number(&management.milestone))
            } else {
                None
            };
            let created = match ManagementProvider::create_goal_issue(provider, &goal.title, &goal.description, milestone) {
                Ok(created) => created,
                Err(error) => {
                    eprintln!("Warning: Failed to create issue for child goal {}: {}", goal.id, error.message);
                    return;
                }
            };
            goal.management.get_or_insert_with(GoalManagementData::default).issue = created.clone();
            println!("Created issue for child goal {}: {created}", goal.id);
            if goal.status == GoalStatus::Closed {
                let _ = ManagementProvider::close_issue(provider, &created);
            }
            if semio_framework_repo_goals::is_deeper_goal(&goal.id) {
                let parent_id = semio_framework_repo_goals::parent_goal_id(&goal.id);
                if let Some(parent_issue) = self.with_goals(true, |goals| goals.read(&parent_id)).ok().and_then(|parent| parent.management).map(|management| management.issue).filter(|issue| !issue.is_empty()) {
                    match ManagementProvider::add_sub_issue(provider, &parent_issue, &created) {
                        Ok(()) => println!("Linked sub-issue {} to parent {parent_id}", goal.id),
                        Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),
                    }
                }
            }
            if let Err(error) = self.save_goal(goal) {
                eprintln!("Warning: Failed to save goal {}: {error}", goal.id);
            }
            return;
        }
        let remote = match ManagementProvider::get_issue_details(provider, &issue) {
            Ok(Some(remote)) => remote,
            Ok(None) => return,
            Err(error) => {
                eprintln!("Warning: Failed to get issue for goal {}: {}", goal.id, error.message);
                return;
            }
        };
        if goal.status == GoalStatus::Closed && remote.state.eq_ignore_ascii_case("open") {
            let _ = ManagementProvider::close_issue(provider, &issue);
        } else if goal.status == GoalStatus::Open && remote.state.eq_ignore_ascii_case("closed") {
            let _ = ManagementProvider::reopen_issue(provider, &issue);
        }
        if semio_framework_repo_goals::is_first_gen_goal(&goal.id) {
            if let Ok(root_goal) = self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&goal.id))) {
                let has_milestone = root_goal.management.as_ref().is_some_and(|management| parse_milestone_number(&management.milestone).is_some());
                if has_milestone && remote.milestone.as_ref().is_none_or(|current| current.title != root_goal.title) {
                    let _ = ManagementProvider::update_issue_milestone(provider, &issue, &root_goal.title);
                }
            }
        } else if semio_framework_repo_goals::is_deeper_goal(&goal.id) {
            if remote.milestone.is_some() {
                if let Err(error) = ManagementProvider::clear_issue_milestone(provider, &issue) {
                    eprintln!("Warning: Failed to clear milestone for goal {}: {}", goal.id, error.message);
                }
            }
            let parent_id = semio_framework_repo_goals::parent_goal_id(&goal.id);
            if let Some(parent_issue) = self.with_goals(true, |goals| goals.read(&parent_id)).ok().and_then(|parent| parent.management).map(|management| management.issue).filter(|issue| !issue.is_empty()) {
                match ManagementProvider::get_issue_parent_url(provider, &issue) {
                    Err(error) => eprintln!("Warning: Failed to resolve parent for goal {}: {}", goal.id, error.message),
                    Ok(current) if current != parent_issue => match ManagementProvider::add_sub_issue(provider, &parent_issue, &issue) {
                        Ok(()) => println!("Linked sub-issue {} to parent {parent_id}", goal.id),
                        Err(error) => eprintln!("Warning: Failed to link sub-issue {} to parent {parent_id}: {}", goal.id, error.message),
                    },
                    Ok(_) => {}
                }
            }
        }
        if !remote.labels.iter().any(|label| label.name == "goal") {
            if let Err(error) = ManagementProvider::add_labels(provider, &issue, &["goal".to_string()]) {
                eprintln!("Warning: Failed to add goal label for {}: {}", goal.id, error.message);
            }
        }
    }
}
//#endregion 🗄️FsRepoContext
