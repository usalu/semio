//! 🎯️ Goal and file support of the production context: the goal retitle and milestone support, the file record, the file header and the workspace loading. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
impl FsRepoContext {
    /// 🏷️ Retitles a ticket when a non-blank title is given, following the folder the new slug names.
    pub fn retitle(&self, id: &TicketId, title: Option<&String>, no_management: bool) -> Result<TicketId, ContextError> {
        let Some(title) = title.filter(|title| !title.trim().is_empty()) else { return Ok(id.clone()) };
        let before = self.ticket(id.year, id.month, id.day, &id.slug)?;
        let outcome = self
            .with_tickets(McpClientKind::Generic, |service| service.change(&TicketChangeRequest { id: id.id(), title: Some(title.clone()), no_management, ..Default::default() }))
            .map_err(|error| ContextError::new(error.message))?;
        if let Some(issue) = before.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty() && !no_management) {
            if let Err(error) = ManagementProvider::update_issue_title(&semio_framework_repo_providers::system_management_provider(), &issue, title) {
                eprintln!("Warning: Failed to update GitHub issue title: {}", error.message);
            }
        }
        TicketId::parse(&outcome.id).map_err(|error| ContextError::new(error.message))
    }

    pub(super) fn ensure_goal_milestone(&self, provider: &ManagementProviders, goal: &mut Goal) -> Result<Option<ManagementMilestone>, ContextError> {
        if goal.title.trim().is_empty() || !semio_framework_repo_goals::is_root_goal(&goal.id) {
            return Ok(None);
        }
        let known = goal.management.as_ref().map(|management| management.milestone.clone()).unwrap_or_default();
        if let Some(number) = parse_milestone_number(&known).filter(|number| *number > 0) {
            if let Ok(Some(milestone)) = ManagementProvider::get_milestone(provider, number) {
                return Ok(Some(milestone));
            }
        }
        let found = ManagementProvider::find_milestone_by_title(provider, &goal.title).map_err(|error| ContextError::new(error.message))?;
        let milestone = match found {
            Some(milestone) => milestone,
            None => {
                let number = ManagementProvider::create_milestone(provider, &goal.title, &goal.description).map_err(|error| ContextError::new(error.message))?;
                if !goal.dates.due.is_empty() {
                    let _ = ManagementProvider::update_milestone(provider, number, "", "", "", &goal.dates.due);
                }
                ManagementProvider::get_milestone(provider, number).ok().flatten().unwrap_or(ManagementMilestone { number, title: goal.title.clone(), description: goal.description.clone(), ..Default::default() })
            }
        };
        let repo_url = VersionControlProvider::repo_url(&semio_framework_repo_providers::system_version_control_provider()).map_err(|error| ContextError::new(error.message))?;
        let link = format!("{repo_url}/milestone/{}", milestone.number);
        let management = goal.management.get_or_insert_with(GoalManagementData::default);
        if management.milestone != link {
            management.milestone = link;
            self.save_goal(goal)?;
        }
        Ok(Some(milestone))
    }

    pub(super) fn save_goal(&self, goal: &Goal) -> Result<(), ContextError> {
        let document = semio_framework_repo_goals::encode_goal(goal).map_err(|error| ContextError::new(error.to_string()))?;
        GoalStore::write(&FsGoalStore::new(&self.root), &goal.id, &document).map_err(|error| ContextError::new(error.to_string()))
    }

    pub(super) fn file_record(&self, path: &str) -> File {
        let name = semio_framework_repo_identity::base_of(path);
        File {
            id: self.codebase.build_file_id(path),
            path: path.to_string(),
            uri: self.codebase.file_uri(path),
            name: name.clone(),
            extension: semio_framework_repo_identity::ext_of(&name),
            folder_id: None,
            bundle_id: None,
            kind: semio_framework_repo_model::derive_file_kind(&name).as_str().to_string(),
            emoji: String::new(),
            ignored: false,
            generated: false,
        }
    }

    pub(super) fn apply(&self, plan: Result<semio_framework_repo_move::Plan, semio_framework_repo_move::MoveError>) -> Result<Vec<String>, ContextError> {
        let plan = plan.map_err(|error| ContextError::new(error.0))?;
        let (starting, ended): (Vec<&semio_framework_repo_move::PlanEvent>, Vec<&semio_framework_repo_move::PlanEvent>) = plan.events.iter().partition(|event| event.kind.ends_with(".starting"));
        for event in starting {
            semio_framework_repo_events::emit(&event.kind, "repo-cli", &event.payload);
        }
        let mut executor = semio_framework_repo_move::FileSystemExecutor::new(&self.root);
        semio_framework_repo_move::execute(&plan, &mut executor).map_err(|error| ContextError::new(error.0))?;
        for event in ended {
            semio_framework_repo_events::emit(&event.kind, "repo-cli", &event.payload);
        }
        Ok(plan.lines())
    }

    /// 📄️ The header region a new file opens with, twin of `GenerateFileHeader`.
    pub fn file_header(&self, path: &str) -> String {
        let Some(language) = semio_framework_repo_languages::language_for_path(path) else { return String::new() };
        let year = semio_framework_repo_tickets::Clock::stamp(&TicketSystemClock).chars().take(4).collect::<String>();
        let header = semio_framework_repo_languages::Header {
            file_id: self.codebase.build_file_id(path),
            file_uri: self.codebase.file_uri(path),
            summary: String::new(),
            contributors: format!("{year} {}", semio_framework_repo_workspace::git_author(&self.root)),
            license: semio_framework_repo_workspace::AGPL_LICENSE_TEXT.to_string(),
            requirements: String::new(),
        };
        semio_framework_repo_languages::format_header(language, &header)
    }

    /// 🗂️ The in-memory workspace a move plan reads: the named files, and every considered file
    /// below the named directories — structurally excluded and ignored paths never enter it.
    pub fn workspace(&self, files: &[&str], directories: &[&str]) -> semio_framework_repo_move::Workspace {
        let mut workspace = semio_framework_repo_move::Workspace::new();
        for directory in directories.iter().filter(|directory| !directory.is_empty()) {
            let absolute = self.root.join(directory);
            if absolute.is_dir() {
                if *directory != "." {
                    workspace.insert_directory(directory);
                }
                self.load_directory(&mut workspace, if *directory == "." { "" } else { directory });
            } else if let Some(content) = self.read(directory) {
                workspace.insert_file(directory, &content);
            }
        }
        for file in files.iter().filter(|file| !file.is_empty()) {
            if let Some(content) = self.read(file) {
                workspace.insert_file(file, &content);
            }
        }
        if let Some(agents) = self.read("AGENTS.md") {
            workspace.insert_file("AGENTS.md", &agents);
        }
        workspace
    }

    fn load_directory(&self, workspace: &mut semio_framework_repo_move::Workspace, directory: &str) {
        let Ok(entries) = std::fs::read_dir(self.root.join(directory)) else { return };
        let mut entries: Vec<std::fs::DirEntry> = entries.flatten().collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            let relative = if directory.is_empty() { name } else { format!("{directory}/{name}") };
            if self.codebase.is_repo_excluded_path(&relative) || self.codebase.is_gitignored(&relative) {
                continue;
            }
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                workspace.insert_directory(&relative);
                self.load_directory(workspace, &relative);
            } else if kind.is_file() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    workspace.insert_file(&relative, &content);
                }
            }
        }
    }
}
//#endregion 🗄️FsRepoContext
