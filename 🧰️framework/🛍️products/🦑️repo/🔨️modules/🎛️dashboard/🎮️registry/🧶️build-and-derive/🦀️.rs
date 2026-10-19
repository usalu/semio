//! 🧶️ Build and derive of the registry: the build from facts, the tool and compound derivation and the settling of parameters and ownership. See [`super`].

use super::*;

// #region 🔖️Registry
impl Registry {
    /// 🧶 Derives every command from what the sources state. Invalid declarations become
    /// [`Registry::problems`] that name their file; everything valid stays runnable.
    pub fn build(root: &Path, facts: &Facts) -> Self {
        let mut problems = facts.problems.clone();
        let mut entries: Vec<Entry> = Vec::new();
        let (mut axes, mut axes_source) = (Vec::new(), String::new());
        let source = |project: &ProjectFacts| if project.manifest.is_empty() { format!("{} (project {})", graph_file(), project.name) } else { project.manifest.clone() };
        for project in &facts.projects {
            let file = source(project);
            if RESERVED_KINDS.contains(&project.name.as_str()) || project.name.contains(|c: char| c == ':' || c.is_whitespace()) { problems.push(Problem { file: file.clone(), at: "name".into(), message: format!("project name {:?} cannot be told from a command kind", project.name) }); continue; }
            let owner = owner_segments(&project.root);
            let keys: Vec<String> = owner.iter().map(|part| part.key.clone()).collect();
            for target in &project.targets {
                let at = format!("targets.{}.metadata.semio.dashboard", target.name);
                let declaration: TargetDeclaration = target.dashboard.as_ref().and_then(|value| decode(&file, &at, value, &mut problems)).unwrap_or_default();
                let mut scope = Scope { file: &file, at, problems: &mut problems };
                scope.verb(declaration.verb.as_ref());
                let requires = scope.members("requires", &declaration.requires);
                let ready = scope.ready(declaration.ready.as_ref());
                let own = scope.parameters(&declaration.parameters, Origin::Target);
                let verb = verb_of(declaration.verb.as_deref().filter(|verb| is_verb(verb)), &target.name);
                let mut path = owner.clone();
                path.push(segment(format!(":{}", target.name), target.name.as_str()));
                let qualifier = target.name.strip_prefix(verb.as_str()).map_or(target.name.as_str(), |rest| rest.trim_start_matches(['-', ':', '_'])).to_string();
                entries.push(Entry::new(Draft { id: format!("{}:{}", project.name, target.name), kind: Kind::Target, verb, path, owner: keys.clone(), subject: short_name(&project.name).into(), qualifier, long_running: target.continuous.unwrap_or(false) || ready.is_some(), source: &file, own, action: Action::Target { project: project.name.clone(), target: target.name.clone(), configurations: target.configurations.clone(), ready, requires } }));
                if let Some(entry) = entries.last_mut() { scope.presentation(entry, declaration.title.as_ref(), declaration.description.as_ref()); }
            }
            let Some(declaration) = project.dashboard.as_ref().and_then(|value| decode::<ProjectDeclaration>(&file, "metadata.semio.dashboard", value, &mut problems)) else { continue };
            let mut scope = Scope { file: &file, at: "metadata.semio.dashboard".into(), problems: &mut problems };
            if !declaration.parameters.is_empty() {
                if project.root == "." || project.root.is_empty() { axes = scope.parameters(&declaration.parameters, Origin::Axis); axes_source.clone_from(&file); } else { scope.report("`parameters` are the global axes and belong to the workspace root project"); }
            }
            Self::tools(&mut scope, &mut entries, &declaration.tools, &owner, &keys, &|id| format!("tool:{}/{id}", project.name), Kind::Tool);
            Self::compounds(&mut scope, &mut entries, &declaration.compounds, &owner, &keys, &|id| format!("compound:{}/{id}", project.name), Kind::Compound);
            for (index, group) in declaration.groups.iter().enumerate() {
                let mut scope = scope.within(format!("groups[{index}]"));
                let valid = scope.id(&group.id);
                let word = |text: &str| !text.is_empty() && !text.contains(|c: char| c.is_whitespace() || c == ',');
                if group.targets.is_empty() { scope.report("a group needs `targets`"); }
                for (position, name) in group.targets.iter().enumerate() { if !word(name) || group.targets[..position].contains(name) { scope.report(format!("target {name:?} must be one target name, listed once")); } }
                if group.projects.is_empty() { scope.report("a group needs `projects`"); }
                for (position, name) in group.projects.iter().enumerate() { if !word(name) || group.projects[..position].contains(name) { scope.report(format!("project {name:?} must be one project name, listed once")); } }
                if !valid || group.targets.is_empty() { continue; }
                let first = group.targets[0].as_str();
                let verb = verb_of(None, first);
                let mut path = owner.clone();
                path.push(segment(format!("group:{}", group.id), group.id.as_str()));
                let qualifier = if group.targets.len() > 1 { group.targets.join("+") } else { first.strip_prefix(verb.as_str()).map_or(first, |rest| rest.trim_start_matches(['-', ':', '_'])).to_string() };
                entries.push(Entry::new(Draft { id: format!("group:{}/{}", project.name, group.id), kind: Kind::Group, verb, path, owner: keys.clone(), subject: group.id.clone(), qualifier, long_running: false, source: &file, own: Vec::new(), action: Action::Group { targets: group.targets.clone(), projects: group.projects.clone() } }));
                if let Some(entry) = entries.last_mut() { scope.presentation(entry, group.title.as_ref(), group.description.as_ref()); }
            }
        }
        for playground in &facts.playgrounds {
            let path = vec![segment(segment_key(&playground.plugin), playground.plugin.as_str()), segment(segment_key(&playground.variant), playground.variant.as_str())];
            let aliases = playground.aliases.join(" ").to_lowercase();
            let mut entry = Entry::new(Draft { id: format!("playground:{}", playground.variant), kind: Kind::Playground, verb: "dev".into(), path, owner: vec![segment_key(&playground.plugin)], subject: playground.variant.clone(), qualifier: String::new(), long_running: true, source: PLAYGROUND_SOURCE, own: Vec::new(), action: Action::Playground(playground.clone()) });
            entry.haystack.push_str(&format!(" {aliases}"));
            entries.push(entry);
        }
        for ticket in &facts.tickets {
            let head = segment(ticket.id.as_str(), format!("{} [{}] {}", ticket.id, if ticket.open { "open" } else { "closed" }, elide_end(&ticket.title, LABEL_CELLS)));
            let actions = [("show", RepoAction::TicketShow { id: ticket.id.clone() }, false), ("files", RepoAction::TicketFiles { id: ticket.id.clone() }, false), if ticket.open { ("close", RepoAction::TicketClose { id: ticket.id.clone() }, true) } else { ("reopen", RepoAction::TicketReopen { id: ticket.id.clone(), prompt: String::new() }, true) }];
            for (key, action, mutating) in actions {
                let mut entry = Entry::new(Draft { id: format!("repo:{}", dashboard_repo_domain::action_key(&action)), kind: Kind::Repo, verb: "tickets".into(), path: vec![head.clone(), segment(key, key)], owner: Vec::new(), subject: ticket.id.clone(), qualifier: key.into(), long_running: false, source: "", own: Vec::new(), action: Action::Repo(action) });
                (entry.listed, entry.mutating) = (ticket.open || key == "reopen", mutating);
                if key == "reopen" { entry.title = Some(LocalizedText { en: format!("Reopen Ticket {}", ticket.id), de: format!("Ticket {} Wiederöffnen", ticket.id) }); entry.description = Some(LocalizedText { en: "Continue a closed ticket with a saved human reason.".into(), de: "Setze ein geschlossenes Ticket mit einem gespeicherten Grund fort.".into() }); }
                entries.push(entry);
            }
            let (Some(document), true) = (&ticket.commands, ticket.open) else { continue };
            let file = format!("{}/{TICKET_COMMANDS}", ticket.folder);
            let Some(declaration) = decode::<TicketDeclaration>(&file, "", document, &mut problems) else { continue };
            let mut scope = Scope { file: &file, at: String::new(), problems: &mut problems };
            let (owner, keys) = (vec![segment("tickets", "tickets"), segment(ticket.id.as_str(), ticket.id.as_str())], vec!["tickets".to_string(), ticket.id.clone()]);
            Self::tools(&mut scope, &mut entries, &declaration.tools, &owner, &keys, &|id| format!("ticket:{}/{id}", ticket.id), Kind::Ticket);
            Self::compounds(&mut scope, &mut entries, &declaration.compounds, &owner, &keys, &|id| format!("ticket:{}/{id}", ticket.id), Kind::Ticket);
        }
        let fixed = [("goals", "list", RepoAction::GoalsList), ("goals", "tree", RepoAction::GoalsTree), ("tree", "monorepo", RepoAction::TreeMonorepo), ("tree", "goal", RepoAction::TreeGoal), ("tree", "statute", RepoAction::TreeStatute), ("tree", "territory", RepoAction::TreeTerritory), ("statutes", "catalog", RepoAction::StatutesCatalog), ("draft", "start", RepoAction::DraftStart { title: String::new() }), ("draft", "discard", RepoAction::DraftDiscard { slug: String::new() })];
        let scopes = dashboard_repo_domain::ANALYZE_SCOPES.iter().map(|(scope, _)| ("analyze", *scope, RepoAction::Analyze { scope: (*scope).to_string(), policy: None }));
        for (verb, key, action) in fixed.into_iter().chain(scopes) {
            let mutating = action.mutates();
            let mut entry = Entry::new(Draft { id: format!("repo:{}", dashboard_repo_domain::action_key(&action)), kind: Kind::Repo, verb: verb.into(), path: vec![segment(key, key)], owner: Vec::new(), subject: key.into(), qualifier: String::new(), long_running: false, source: "", own: Vec::new(), action: Action::Repo(action) });
            entry.mutating = mutating;
            match key { "start" => { entry.title = Some(LocalizedText { en: "Start Draft".into(), de: "Entwurf Beginnen".into() }); }, "discard" => { entry.title = Some(LocalizedText { en: "Discard Draft".into(), de: "Entwurf Verwerfen".into() }); }, _ => {} }
            match key { "start" => entry.description = Some(LocalizedText { en: "Create an empty local draft for notes and unfinished work.".into(), de: "Erstelle einen leeren lokalen Entwurf für Notizen und laufende Arbeit.".into() }), "discard" => entry.description = Some(LocalizedText { en: "Remove the selected draft and its contents after confirmation.".into(), de: "Entferne den ausgewählten Entwurf samt Inhalt nach Bestätigung.".into() }), _ => {} }
            entries.push(entry);
        }
        entries.sort_by(|left, right| verb_rank(&left.verb).cmp(&verb_rank(&right.verb)).then_with(|| left.verb.cmp(&right.verb)).then_with(|| left.label.cmp(&right.label)).then_with(|| left.id.cmp(&right.id)));
        for entry in &mut entries { if matches!(entry.action, Action::Repo(_)) { for text in [&entry.title, &entry.description].into_iter().flatten() { entry.haystack.push_str(&format!(" {} {}", text.en.to_lowercase(), text.de.to_lowercase())); } } }
        let mut index = HashMap::with_capacity(entries.len());
        entries.retain(|entry| {
            let fresh = !index.contains_key(&entry.id);
            if fresh { index.insert(entry.id.clone(), index.len()); } else { problems.push(Problem { file: entry.source.clone(), at: entry.id.clone(), message: "command id is declared twice".into() }); }
            fresh
        });
        let mut definitions: Vec<String> = facts.projects.iter().filter(|project| !project.manifest.is_empty()).map(|project| project.manifest.clone()).chain(GRAPH_OWNERS.iter().map(|owner| (*owner).to_string())).collect();
        definitions.sort(); definitions.dedup();
        let mut source_projects = facts.projects.iter().filter(|project| source_path(&project.root)).map(|project| (project.name.clone(), project.root.clone())).collect::<Vec<_>>(); source_projects.sort(); source_projects.dedup();
        let mut registry = Self { root: root.to_path_buf(), source_projects, source_discovery: std::sync::Mutex::new(HashMap::new()), source_cancelled: std::sync::Mutex::new(HashSet::new()), entries, index, axes, axes_source, problems, definitions, basis: std::sync::Mutex::new(None) };
        registry.settle();
        registry
    }

    pub(super) fn tools(scope: &mut Scope<'_>, entries: &mut Vec<Entry>, tools: &[ToolDeclaration], owner: &[Segment], keys: &[String], id_of: &dyn Fn(&str) -> String, kind: Kind) {
        for (index, tool) in tools.iter().enumerate() {
            let mut scope = scope.within(format!("tools[{index}]"));
            let valid = scope.id(&tool.id);
            scope.verb(tool.verb.as_ref());
            if tool.command.is_empty() || tool.command[0].is_empty() { scope.report("a tool needs a `command`"); }
            for word in &tool.command { scope.text("command word", word); }
            if !is_relative(&tool.cwd) { scope.report(format!("cwd {:?} must be workspace-relative", tool.cwd)); }
            let requires = scope.members("requires", &tool.requires);
            let env = scope.env(&tool.env);
            let ready = scope.ready(tool.ready.as_ref());
            let own = scope.parameters(&tool.parameters, Origin::Tool);
            scope.mcp(tool.mcp.as_ref(), &tool.parameters);
            if !valid || tool.command.first().is_none_or(String::is_empty) { continue; }
            let mut path = owner.to_vec();
            path.push(segment(format!("tool:{}", tool.id), tool.id.as_str()));
            entries.push(Entry::new(Draft { id: id_of(&tool.id), kind, verb: verb_of(tool.verb.as_deref().filter(|verb| is_verb(verb)), &tool.id), path, owner: keys.to_vec(), subject: tool.id.clone(), qualifier: String::new(), long_running: tool.continuous || ready.is_some(), source: scope.file, own, action: Action::Tool { command: tool.command.clone(), cwd: tool.cwd.clone(), env, ready, requires } }));
            if let Some(entry) = entries.last_mut() { entry.mcp.clone_from(&tool.mcp); scope.presentation(entry, tool.title.as_ref(), tool.description.as_ref()); }
        }
    }

    pub(super) fn compounds(scope: &mut Scope<'_>, entries: &mut Vec<Entry>, compounds: &[CompoundDeclaration], owner: &[Segment], keys: &[String], id_of: &dyn Fn(&str) -> String, kind: Kind) {
        for (index, compound) in compounds.iter().enumerate() {
            let mut scope = scope.within(format!("compounds[{index}]"));
            let valid = scope.id(&compound.id);
            scope.verb(compound.verb.as_ref());
            if compound.members.is_empty() { scope.report("a compound needs `members`"); }
            let members = scope.members("members", &compound.members);
            if !valid || members.is_empty() { continue; }
            let mut path = owner.to_vec();
            path.push(segment(format!("compound:{}", compound.id), compound.id.as_str()));
            let verb = compound.verb.clone().filter(|verb| is_verb(verb)).unwrap_or_else(|| match verb_of(None, &compound.id) { verb if verb == FALLBACK_VERB => String::new(), verb => verb });
            entries.push(Entry::new(Draft { id: id_of(&compound.id), kind, verb, path, owner: keys.to_vec(), subject: compound.id.clone(), qualifier: String::new(), long_running: false, source: scope.file, own: Vec::new(), action: Action::Compound { members, stop: compound.stop.unwrap_or_default() } }));
                if let Some(entry) = entries.last_mut() { scope.presentation(entry, compound.title.as_ref(), compound.description.as_ref()); }
        }
    }

    /// ⛓️ Settles what depends on other commands: a compound's verb and duration follow its
    /// members, every reference and token must name something that exists.
    fn settle(&mut self) {
        let mut problems = Vec::new();
        let mut names: Vec<&str> = if self.entries.iter().any(|entry| matches!(entry.action, Action::Group { .. })) { self.entries.iter().filter_map(|entry| match &entry.action { Action::Target { project, .. } => Some(project.as_str()), _ => None }).collect() } else { Vec::new() };
        names.sort_unstable(); names.dedup();
        let names: Vec<String> = names.into_iter().map(str::to_string).collect();
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        for position in 0..self.entries.len() {
            let entry = &self.entries[position];
            let mut report = |at: String, message: String| problems.push(Problem { file: entry.source.clone(), at, message });
            let mut update = None;
            match &entry.action {
                Action::Compound { members, .. } => {
                    let found: Vec<Option<&Entry>> = members.iter().map(|member| self.member(&member.run).map(|(entry, _)| entry)).collect();
                    for (member, found) in members.iter().zip(&found) {
                        match found {
                            None => report(entry.id.clone(), format!("member {:?} names no command", member.run)),
                            Some(target) if matches!(target.action, Action::Compound { .. }) => report(entry.id.clone(), format!("member {:?} is a compound; compounds do not nest", member.run)),
                            Some(target) => { let accepted = self.parameters(target); for (key, _) in &member.pins { if !accepted.iter().any(|parameter| &parameter.id == key) { report(entry.id.clone(), format!("member {:?} has no parameter {key:?}", member.run)); } } }
                        }
                        for (_, value) in &member.env { for token in tokens(value) { if token != "workspace" { report(entry.id.clone(), format!("member {:?} environment token `{{{token}}}` is not `{{workspace}}`", member.run)); } } }
                    }
                    let verb = if entry.verb.is_empty() { found.iter().flatten().next().map_or_else(|| FALLBACK_VERB.to_string(), |first| first.verb.clone()) } else { entry.verb.clone() };
                    update = Some((verb, found.iter().flatten().any(|member| member.long_running)));
                }
                Action::Group { targets, projects } => {
                    let excluded = |name: &str| projects.iter().filter_map(|pattern| pattern.strip_prefix('!')).any(|pattern| glob(pattern, name));
                    let mut continuous = false;
                    for pattern in projects {
                        let matched: Vec<&str> = names.iter().copied().filter(|name| glob(pattern.trim_start_matches('!'), name)).collect();
                        if matched.is_empty() { report(entry.id.clone(), format!("project {pattern:?} names no project")); continue; }
                        if pattern.starts_with('!') { continue; }
                        let members: Vec<&Entry> = matched.iter().filter(|name| !excluded(name)).flat_map(|name| targets.iter().filter_map(|target| self.find(&format!("{name}:{target}"))).collect::<Vec<_>>()).collect();
                        if members.is_empty() { report(entry.id.clone(), format!("project {pattern:?} has no target {}", targets.join(" or "))); }
                        continuous |= members.iter().any(|member| member.long_running);
                    }
                    update = Some((entry.verb.clone(), continuous));
                }
                Action::Target { ready, requires, .. } | Action::Tool { ready, requires, .. } => {
                    for need in requires {
                        match self.member(&need.run) {
                            None => report(entry.id.clone(), format!("requires {:?} names no command", need.run)),
                            Some((target, _)) => { let accepted = self.parameters(target); for (key, _) in &need.pins { if !accepted.iter().any(|parameter| &parameter.id == key) { report(entry.id.clone(), format!("requires {:?} has no parameter {key:?}", need.run)); } } }
                        }
                        for (_, value) in &need.env { for token in tokens(value) { if token != "workspace" { report(entry.id.clone(), format!("requires {:?} environment token `{{{token}}}` is not `{{workspace}}`", need.run)); } } }
                    }
                    let accepted = self.parameters(entry);
                    let known = |name: &str| name == "workspace" || accepted.iter().any(|parameter| parameter.id == name);
                    let mut texts: Vec<&String> = accepted.iter().flat_map(|parameter| parameter.values.iter().map(|(_, effect)| effect).chain([&parameter.effect])).flat_map(|effect| effect.env.iter().map(|(_, value)| value).chain(&effect.args)).collect();
                    if let Action::Tool { command, cwd, env, .. } = &entry.action { texts.extend(command.iter().chain([cwd]).chain(env.iter().map(|(_, value)| value))); }
                    for text in texts { for token in tokens(text) { if token == "port" { if ready.is_none() { report(entry.id.clone(), "`{port}` needs a `ready` declaration".into()); } } else if !known(token) { report(entry.id.clone(), format!("token `{{{token}}}` names no parameter of this command")); } } }
                    for default in accepted.iter().filter_map(|parameter| parameter.default.as_ref()) { for token in tokens(default) { if !known(token) { report(entry.id.clone(), format!("token `{{{token}}}` in a default names no parameter of this command")); } } }
                    for parameter in entry.own.iter().filter(|parameter| parameter.kind == ParameterKind::Text && parameter.destination == Destination::Token) {
                        let mut places: Vec<&String> = accepted.iter().flat_map(|other| other.values.iter().map(|(_, effect)| effect).chain([&other.effect])).flat_map(|effect| effect.env.iter().map(|(_, value)| value).chain(&effect.args)).chain(accepted.iter().filter_map(|other| other.default.as_ref())).collect();
                        if let Action::Tool { command, cwd, env, .. } = &entry.action { places.extend(command.iter().chain([cwd]).chain(env.iter().map(|(_, value)| value))); }
                        let named = ready.as_ref().is_some_and(|ready| ready.port_env.as_deref() == Some(parameter.id.as_str()));
                        if !named && !places.iter().any(|text| tokens(text).contains(&parameter.id.as_str())) { report(entry.id.clone(), format!("text parameter {:?} goes nowhere: declare `valueEnv`, `valueFlag`, `valuePositional` or use `{{{}}}`", parameter.id, parameter.id)); }
                    }
                    if let Some(ReadySpec { port: None, port_env: Some(name), .. }) = ready {
                        let produced = accepted.iter().any(|parameter| &parameter.id == name || parameter.destination == Destination::Env(name.clone()) || parameter.values.iter().map(|(_, effect)| effect).chain([&parameter.effect]).any(|effect| effect.env.iter().any(|(key, _)| key == name)));
                        let stated = matches!(&entry.action, Action::Tool { env, .. } if env.iter().any(|(key, _)| key == name));
                        if !produced && !stated { report(entry.id.clone(), format!("ready.portEnv {name:?} names no parameter and no declared variable, and no `port` is given")); }
                    }
                }
                _ => {}
            }
            if let Some((verb, long_running)) = update {
                let entry = &mut self.entries[position];
                if entry.verb != verb { entry.verb = verb; entry.relabel(); }
                entry.long_running = long_running;
            }
        }
        for axis in &self.axes { if PLAYGROUND_PARAMETERS.contains(&axis.id.as_str()) || axis.id == CONFIGURATION { problems.push(Problem { file: self.axes_source.clone(), at: format!("metadata.semio.dashboard.parameters.{}", axis.id), message: "the id is a built-in parameter".into() }); } }
        self.problems.extend(problems);
        let order = |entry: &Entry| (verb_rank(&entry.verb), entry.verb.clone(), entry.label.clone(), entry.id.clone());
        if !self.entries.is_sorted_by_key(order) { self.entries.sort_by_key(order); self.index = self.entries.iter().enumerate().map(|(position, entry)| (entry.id.clone(), position)).collect(); }
    }
}
// #endregion 🔖️Registry
