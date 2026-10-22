//! 🔌️ The `RepoContext` port of the production context: one trait impl (Rust admits exactly one) answering the queries, the goal, todo, draft, ticket, contributor and structure commands and the management sync. See [`super`].

use super::*;

//#region 🗄️FsRepoContext
impl RepoContext for FsRepoContext {
    fn root_dir(&self) -> String {
        self.slash_root()
    }

    fn technologies(&self) -> Vec<Technology> {
        self.codebase.technologies()
    }

    fn bundles(&self) -> Vec<Bundle> {
        self.codebase.bundles()
    }

    fn checkpoints(&self, limit: Option<i64>) -> Result<Vec<Checkpoint>, ContextError> {
        Ok(semio_framework_repo_contributors::list_checkpoints(&semio_framework_repo_contributors::GitCheckpoints::new(&self.root), limit.map(|limit| limit.max(0) as usize)))
    }

    fn folders(&self) -> Vec<Folder> {
        self.considered()
            .build_folders()
            .into_iter()
            .map(|folder| Folder {
                kind: self.codebase.derive_folder_kind(&folder.path),
                generated: semio_framework_repo_codebase::is_generated_folder(&folder.path),
                id: folder.id,
                path: folder.path,
                uri: folder.uri,
                name: folder.name,
                parent_id: folder.parent_id,
                bundle_id: folder.bundle_id,
                emoji: String::new(),
                ignored: false,
            })
            .collect()
    }

    fn files(&self) -> Vec<File> {
        self.considered()
            .build_files()
            .into_iter()
            .map(|file| {
                let mut record = self.file_record(&file.path);
                record.id = file.id;
                record.uri = file.uri;
                record.folder_id = file.folder_id;
                record.bundle_id = file.bundle_id;
                record
            })
            .collect()
    }

    fn sections(&self) -> Vec<Section> {
        let context = self.considered();
        let mut result = Vec::new();
        for file in &context.files {
            let Some(content) = self.read(file) else { continue };
            let file_id = self.codebase.build_file_id(file);
            for mut section in semio_framework_repo_languages::parse_sections(&content, file) {
                section.file_path = file.clone();
                section.id = semio_framework_repo_model::build_section_id(&file_id, &semio_framework_repo_languages::normalize_section_path(&section.path));
                result.push(section);
            }
        }
        result
    }

    fn definitions(&self) -> Vec<Definition> {
        let context = self.considered();
        let mut result = Vec::new();
        for file in &context.files {
            let Some(content) = self.read(file) else { continue };
            let file_id = self.codebase.build_file_id(file);
            for mut definition in semio_framework_repo_codebase::file_definitions(&content, file) {
                definition.id = semio_framework_repo_codebase::definition_id(&file_id, &definition);
                result.push(definition);
            }
        }
        result
    }

    fn contributors(&self) -> Result<Vec<Contributor>, ContextError> {
        Ok(semio_framework_repo_contributors::list_contributors(&semio_framework_repo_contributors::FsContributorStore::new(&self.root)))
    }

    fn goals(&self) -> Result<Vec<Goal>, ContextError> {
        self.with_goals(true, |goals| goals.list()).map_err(|error| ContextError::new(error.to_string()))
    }

    fn tickets(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>, status: Option<TicketStatus>) -> Result<Vec<Ticket>, ContextError> {
        let tickets = self.with_tickets(McpClientKind::Generic, |service| service.list(year, month, day)).map_err(|error| ContextError::new(error.message))?;
        Ok(tickets.into_iter().filter(|ticket| status.is_none_or(|status| ticket.status == status)).collect())
    }

    fn policies(&self) -> Vec<Policy> {
        semio_framework_repo_statutes::policies().to_vec()
    }

    fn drafts(&self) -> Result<Vec<Draft>, ContextError> {
        Ok(semio_framework_repo_todos::list_drafts(&FsDraftStore::new(&self.root)))
    }

    fn todos(&self, filter: Option<&FilterInput>) -> Result<Vec<Todo>, ContextError> {
        let all = self.with_todos(|todos| todos.list());
        let Some(search) = filter.and_then(|filter| filter.filter.as_ref()).map(|search| search.to_lowercase()) else { return Ok(all) };
        Ok(all.into_iter().filter(|todo| todo.name.to_lowercase().contains(&search) || todo.description.to_lowercase().contains(&search)).collect())
    }

    fn statutes(&self) -> Vec<StatuteMeta> {
        semio_framework_repo_statutes::statutes().to_vec()
    }

    fn interactions(&self) -> Result<Vec<InteractionResource>, ContextError> {
        let tickets = self.tickets(None, None, None, None)?;
        Ok(tickets
            .into_iter()
            .flat_map(|ticket| {
                let id = emoji_text(entity("ticket")) + &flat(&ticket.slug);
                let goal = ticket.goal.clone();
                ticket.interactions.into_iter().map(move |interaction| InteractionResource { interaction, source_kind: "ticket".to_string(), source_id: id.clone(), goal_id: goal.clone(), ticket_id: id.clone() })
            })
            .collect())
    }

    fn analyze(&self, scope: Option<&str>) -> Result<AnalyzeResult, ContextError> {
        let scope = AnalyzeScope::parse(scope.unwrap_or("compose"));
        let statutes = semio_framework_repo_statutes::statutes();
        let breachs: Vec<Breach> = self.cached_breachs()?.into_iter().filter(|breach| scope.matches(breach)).collect();
        let mut by_priority = PriorityCount::default();
        let mut autofixable = 0i64;
        for breach in &breachs {
            let meta = statutes.iter().find(|meta| meta.kind == breach.kind);
            match breach.lint_priority.or(meta.map(|meta| meta.priority)).unwrap_or(BreachPriority::Low) {
                BreachPriority::High => by_priority.high += 1,
                BreachPriority::Medium => by_priority.medium += 1,
                BreachPriority::Low => by_priority.low += 1,
            }
            if breach.lint_autofixable.or(meta.map(|meta| meta.autofixable)).unwrap_or(false) {
                autofixable += 1;
            }
        }
        Ok(AnalyzeResult { metrics: Some(AnalyzeMetrics { total: breachs.len() as i64, by_priority: Some(by_priority), autofixable }), breachs: Some(breachs) })
    }
    fn fix(&self, scope: Option<&str>) -> Result<FixResult, ContextError> {
        self.autofix(scope)
    }

    fn goal_create(&self, input: GoalCreateInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.create(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_change(&self, input: GoalChangeInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.change(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_close(&self, input: GoalCloseInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.close(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_reopen(&self, input: GoalReopenInput) -> Result<Goal, ContextError> {
        self.with_goals(input.no_management, |goals| goals.reopen(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn goal_delete(&self, input: GoalDeleteInput) -> Result<bool, ContextError> {
        self.with_goals(input.no_management, |goals| goals.delete(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_create(&self, input: TodoCreateInput) -> Result<Todo, ContextError> {
        self.with_todos(|todos| todos.create(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_change(&self, input: TodoChangeInput) -> Result<Todo, ContextError> {
        self.with_todos(|todos| todos.change(&input)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn todo_delete(&self, id: &str) -> Result<bool, ContextError> {
        self.with_todos(|todos| todos.delete(id)).map_err(|error| ContextError::new(error.to_string()))
    }

    fn draft_create(&self, input: DraftCreateInput) -> Result<Draft, ContextError> {
        let files = input
            .files
            .iter()
            .map(|path| std::fs::read_to_string(self.root.join(path)).map(|content| (path.clone(), content)).map_err(|error| ContextError::new(format!("{path}: {error}"))))
            .collect::<Result<Vec<(String, String)>, ContextError>>()?;
        let draft = semio_framework_repo_todos::create_draft(&FsDraftStore::new(&self.root), &input.title, &files).map_err(|error| ContextError::new(error.to_string()))?;
        semio_framework_repo_events::emit(semio_framework_repo_events::DRAFT_CREATE_ENDED, "repo-cli", &json!({ "slug": draft.id, "title": input.title }));
        Ok(draft)
    }

    fn draft_delete(&self, id: &str) -> Result<bool, ContextError> {
        semio_framework_repo_todos::delete_draft(&FsDraftStore::new(&self.root), id).map_err(|error| ContextError::new(error.to_string()))?;
        semio_framework_repo_events::emit(semio_framework_repo_events::DRAFT_DELETE_ENDED, "repo-cli", &json!({ "slug": id }));
        Ok(true)
    }
    fn ticket_open(&self, input: TicketOpenInput) -> Result<Ticket, ContextError> {
        let client = semio_framework_repo_model::resolve_allowed_client(&input.client).map_err(|error| ContextError::new(error.to_string()))?;
        let kind = semio_framework_repo_providers::mcp_kind_from_resolved_client(&client);
        let request = TicketOpenRequest {
            emoji: input.emoji,
            title: input.title,
            prompt: input.prompt,
            llm: input.llm,
            effort: input.effort,
            client: input.client,
            goal: input.goal,
            parent: input.parent,
            no_issue: input.no_issue,
            no_management: input.no_management,
            issue: input.issue,
            session: String::new(),
            plan_id: input.plan_id,
            spec_id: input.spec_id,
            draft: input.draft,
        };
        let outcome = self.with_tickets(kind, |service| service.open(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_close(&self, input: TicketCloseInput) -> Result<Ticket, ContextError> {
        if input.all {
            let mut last = None;
            for ticket in self.tickets(None, None, None, Some(TicketStatus::Open))? {
                let id = TicketId::new(ticket.year, ticket.month, ticket.day, ticket.slug.clone());
                println!("Closing ticket {}...", ticket.slug);
                let request = TicketCloseRequest { id: id.id(), summary: "Bulk close".to_string(), files: Vec::new(), no_management: input.no_management, bulk: true };
                match self.with_tickets(McpClientKind::Generic, |service| service.close(&request)) {
                    Ok(_) => last = self.ticket_by_id(&id.id()).ok(),
                    Err(error) => eprintln!("Warning: Failed to close ticket {}: {}", ticket.slug, error.message),
                }
            }
            if !input.no_management {
                let provider = semio_framework_repo_providers::system_management_provider();
                match ManagementProvider::list_open_issues_with_label(&provider, "ticket") {
                    Ok(issues) => {
                        for issue in issues {
                            println!("Closing GitHub issue {issue}...");
                            if let Err(error) = ManagementProvider::close_issue(&provider, &issue) {
                                eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);
                            }
                        }
                    }
                    Err(error) => eprintln!("Warning: Failed to list GitHub issues with 'ticket' label: {}", error.message),
                }
            }
            return last.ok_or_else(|| ContextError::new("no open tickets"));
        }
        let id = self.retitle(&TicketId::new(input.year, input.month, input.day, input.slug.clone()), input.title.as_ref(), input.no_management)?;
        let request = TicketCloseRequest { id: id.id(), summary: input.summary, files: input.files.unwrap_or_default(), no_management: input.no_management, bulk: false };
        let outcome = self.with_tickets(McpClientKind::Generic, |service| service.close(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_reopen(&self, input: TicketReopenInput) -> Result<Ticket, ContextError> {
        let client = semio_framework_repo_model::resolve_allowed_client(&input.client).map_err(|error| ContextError::new(error.to_string()))?;
        if !input.llm.trim().is_empty() { semio_framework_repo_model::resolve_allowed_llm(&input.llm).map_err(|error| ContextError::new(error.to_string()))?; }
        if !input.effort.trim().is_empty() { semio_framework_repo_model::resolve_allowed_effort(&input.effort).map_err(|error| ContextError::new(error.to_string()))?; }
        let id = self.retitle(&TicketId::new(input.year, input.month, input.day, input.slug.clone()), input.title.as_ref(), input.no_management)?;
        let kind = semio_framework_repo_providers::mcp_kind_from_resolved_client(&client);
        let request = TicketReopenRequest {
            id: id.id(),
            prompt: input.prompt,
            llm: input.llm,
            effort: input.effort,
            client: input.client,
            goal: input.goal,
            parent: input.parent,
            no_management: input.no_management,
            session: String::new(),
            plan_id: input.plan_id,
            spec_id: input.spec_id,
            draft: input.draft,
        };
        let outcome = self.with_tickets(kind, |service| service.reopen(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings {
            eprintln!("Warning: {warning}");
        }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_change(&self, input: TicketChangeInput) -> Result<Ticket, ContextError> {
        let request = TicketChangeRequest { id: TicketId::new(input.year, input.month, input.day, input.slug).id(), title: input.title, description: input.prompt, llm: input.llm, effort: input.effort, client: input.client, goal: input.goal, parent: input.parent, no_management: input.no_management };
        let outcome = self.with_tickets(McpClientKind::Generic, |service| service.change(&request)).map_err(|error| ContextError::new(error.message))?;
        for warning in &outcome.warnings { eprintln!("Warning: {warning}"); }
        self.ticket_by_id(&outcome.id)
    }

    fn ticket_delete(&self, input: TicketDeleteInput) -> Result<bool, ContextError> {
        let id = TicketId::new(input.year, input.month, input.day, input.slug);
        self.with_tickets(McpClientKind::Generic, |service| service.delete(&id, input.no_management)).map_err(|error| ContextError::new(error.message))?;
        Ok(true)
    }
    fn contributor_add(&self, input: ContributorAddInput) -> Result<Contributor, ContextError> {
        let mut contributor = match semio_framework_repo_contributors::load_contributor(&self.root, &input.github) {
            Ok(contributor) => contributor,
            Err(_) => semio_framework_repo_contributors::create_contributor(&self.root, &input.github).map_err(ContextError::new)?,
        };
        let mut changed = false;
        let mut fill = |slot: &mut String, value: &Option<String>| {
            if let Some(value) = value.as_ref().filter(|value| !value.is_empty() && slot.is_empty()) {
                *slot = value.clone();
                changed = true;
            }
        };
        fill(&mut contributor.name, &input.name);
        fill(&mut contributor.email, &input.email);
        fill(&mut contributor.fingerprint, &input.fingerprint);
        for (list, values) in [(&mut contributor.names, &input.names), (&mut contributor.emails, &input.emails), (&mut contributor.fingerprints, &input.fingerprints)] {
            for value in values {
                if !list.contains(value) {
                    list.push(value.clone());
                    changed = true;
                }
            }
        }
        if changed {
            semio_framework_repo_contributors::save_contributor(&self.root, &contributor).map_err(ContextError::new)?;
            semio_framework_repo_events::emit(semio_framework_repo_events::CONTRIBUTOR_ADD_ENDED, "repo-cli", &json!({ "github": contributor.github, "author": self.author() }));
        }
        Ok(contributor)
    }

    fn contributor_remove(&self, github: &str) -> Result<(), ContextError> {
        semio_framework_repo_contributors::remove_contributor(&self.root, github).map_err(ContextError::new)?;
        semio_framework_repo_events::emit(semio_framework_repo_events::CONTRIBUTOR_REMOVE_ENDED, "repo-cli", &json!({ "github": github, "author": self.author() }));
        Ok(())
    }

    fn folder_create(&self, path: &str) -> Result<Folder, ContextError> {
        let workspace = self.workspace(&[], &[]);
        self.apply(semio_framework_repo_move::plan_folder_create(&workspace, path))?;
        Ok(Folder {
            id: self.codebase.build_folder_id(path),
            path: path.to_string(),
            uri: self.codebase.folder_uri(path),
            name: semio_framework_repo_identity::base_of(path),
            parent_id: None,
            bundle_id: None,
            kind: self.codebase.derive_folder_kind(path),
            emoji: String::new(),
            ignored: false,
            generated: semio_framework_repo_codebase::is_generated_folder(path),
        })
    }

    fn folder_move(&self, source: &str, destination: &str) -> Result<Folder, ContextError> {
        let workspace = self.workspace(&[], &[source, destination]);
        self.apply(semio_framework_repo_move::plan_folder_move(&workspace, source, destination))?;
        let mut folder = self.folder_create_record(destination);
        folder.id = self.codebase.build_folder_id(destination);
        Ok(folder)
    }

    fn folder_delete(&self, path: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[], &[path]);
        self.apply(semio_framework_repo_move::plan_folder_delete(&workspace, path))?;
        Ok(())
    }

    fn file_create(&self, path: &str) -> Result<File, ContextError> {
        let workspace = self.workspace(&[path], &[]);
        let header = self.file_header(path);
        self.apply(semio_framework_repo_move::plan_file_create(&workspace, path, &header))?;
        Ok(self.file_record(path))
    }

    fn file_move(&self, source: &str, destination: &str) -> Result<File, ContextError> {
        let workspace = self.workspace(&[source, destination], &[]);
        self.apply(semio_framework_repo_move::plan_file_move(&workspace, source, destination))?;
        Ok(self.file_record(destination))
    }

    fn file_delete(&self, path: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[path], &[]);
        self.apply(semio_framework_repo_move::plan_file_delete(&workspace, path))?;
        Ok(())
    }

    fn section_create(&self, file: &str, name: &str, parent: Option<&str>) -> Result<Section, ContextError> {
        let path = match parent.filter(|parent| !parent.is_empty()) {
            Some(parent) => format!("{parent}/{name}"),
            None => name.to_string(),
        };
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_create(&workspace, file, &path))?;
        let id = semio_framework_repo_model::build_section_id(&self.codebase.build_file_id(file), &path.split('/').map(str::to_string).collect::<Vec<String>>());
        Ok(section_record(id, name, &path, file))
    }

    fn section_move(&self, file: &str, old_name: &str, new_name: &str) -> Result<Section, ContextError> {
        self.move_section(file, old_name, new_name)?;
        let id = semio_framework_repo_model::build_section_id(&self.codebase.build_file_id(file), &new_name.split('/').map(str::to_string).collect::<Vec<String>>());
        Ok(section_record(id, new_name, new_name, file))
    }

    fn section_delete(&self, file: &str, name: &str) -> Result<(), ContextError> {
        let workspace = self.workspace(&[file], &[]);
        self.apply(semio_framework_repo_move::plan_section_delete(&workspace, file, name))?;
        Ok(())
    }

    fn integrate(&self, source: Option<&str>, target_section: Option<&str>, target_file: Option<&str>, target_parent: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.integrate_file(source.unwrap_or_default(), target_section.unwrap_or_default(), target, target_parent.unwrap_or_default())?;
        Ok(self.file_record(target))
    }

    fn extract(&self, source_file: Option<&str>, source_section: Option<&str>, target_file: Option<&str>) -> Result<File, ContextError> {
        let target = target_file.unwrap_or_default();
        self.extract_section(source_file.unwrap_or_default(), source_section.unwrap_or_default(), target)?;
        Ok(self.file_record(target))
    }
    fn sync_management(&self) -> Result<bool, ContextError> {
        println!("Syncing local tickets and goals with GitHub...");
        let provider = semio_framework_repo_providers::system_management_provider();
        let mut valid: Vec<String> = vec!["repo".to_string()];
        for technology in self.codebase.technologies() {
            valid.push(format!("@{}", technology.name.trim_start_matches('@')));
            for bundle in technology.bundles.unwrap_or_default() {
                valid.push(semio_framework_repo_codebase::normalize_bundle_label(&bundle.name));
            }
        }
        valid.sort();
        valid.dedup();
        if let Err(error) = ManagementProvider::sync_repo_label_catalog(&provider, &valid) {
            eprintln!("Warning: Failed to sync GitHub label catalog: {}", error.message);
        }
        match self.goals() {
            Err(error) => eprintln!("Warning: Failed to list goals: {error}"),
            Ok(mut goals) => {
                goals.sort_by_key(|goal| semio_framework_repo_goals::goal_depth(&goal.id));
                for mut goal in goals {
                    self.sync_goal(&provider, &mut goal);
                }
            }
        }
        for ticket in self.tickets(None, None, None, None)? {
            let Some(issue) = ticket.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty()) else { continue };
            let remote = match ManagementProvider::get_issue_details(&provider, &issue) {
                Ok(Some(remote)) => remote,
                Ok(None) => continue,
                Err(error) => {
                    eprintln!("Warning: Failed to get GitHub issue {issue}: {}", error.message);
                    continue;
                }
            };
            if ticket.status == TicketStatus::Closed && remote.state.eq_ignore_ascii_case("open") {
                println!("Closing GitHub issue {issue} (Ticket is closed locally)");
                if let Err(error) = ManagementProvider::close_issue(&provider, &issue) {
                    eprintln!("Warning: Failed to close GitHub issue {issue}: {}", error.message);
                }
            }
            if !ticket.goal.is_empty() {
                if let Ok(mut root_goal) = self.with_goals(true, |goals| goals.read(&semio_framework_repo_goals::root_goal_id(&ticket.goal))) {
                    match self.ensure_goal_milestone(&provider, &mut root_goal) {
                        Err(error) => eprintln!("Warning: Failed to resolve goal milestone for issue {issue}: {error}"),
                        Ok(Some(milestone)) if !milestone.title.is_empty() && remote.milestone.as_ref().is_none_or(|current| current.title != milestone.title) => {
                            println!("Updating milestone for issue {issue} to {}...", milestone.title);
                            if let Err(error) = ManagementProvider::update_issue_milestone(&provider, &issue, &milestone.title) {
                                eprintln!("Warning: Failed to update milestone for GitHub issue {issue}: {}", error.message);
                            }
                        }
                        Ok(_) => {}
                    }
                }
            }
            prune_labels(&provider, &issue, &remote.labels, &valid);
        }
        match ManagementProvider::list_issues_for_label_sync(&provider) {
            Err(error) => eprintln!("Warning: Failed to list GitHub issues for label sync: {}", error.message),
            Ok(issues) => {
                for issue in issues {
                    prune_labels(&provider, &issue.url, &issue.labels, &valid);
                }
            }
        }
        println!("GitHub sync completed.");
        Ok(true)
    }
}
//#endregion 🗄️FsRepoContext
