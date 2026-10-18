//! 🗂️ The tree source and the list-verb stream filters: the filesystem tree source, the model ids and the stream options.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_event_stream::Format;
use semio_framework_repo_codebase::Codebase;
use semio_framework_repo_graphql::RepoContext;
use semio_framework_repo_identity::{emoji_text, entity as identity_entity, flat};
use semio_framework_repo_model::{TreeFilter, TreeNode, TreeNodeKind};
use semio_framework_repo_model_entity::entity;
use serde_json::{Map, Value as Json};
use std::collections::BTreeMap;

//#region 🧭️RepoCli
//#region 🗂️TreeSource

/// 🧹️ One scope as the record paths spell it: forward slashes, no outer separators, `.` empty.
pub fn normalize_tree_scope(scope: &str) -> String {
    let scope = scope.trim().replace('\\', "/");
    let scope = scope.trim_matches('/');
    if scope == "." {
        String::new()
    } else {
        scope.to_string()
    }
}

/// 🎯️ Whether a record path is the scope itself or lies below it.
pub fn path_within_scope(path: &str, scope: &str) -> bool {
    scope.is_empty() || path == scope || path.starts_with(&format!("{scope}/"))
}

/// 🗂️ The tree source the tree verbs project from: every record read out of the repository
/// through the repository context, twin of the Go `FsTreeSource`.
#[derive(Default)]
pub struct FsTreeSource {
    pub root: String,
    pub include_sections: bool,
    pub technologies: Vec<semio_framework_repo_tree::TechnologyRecord>,
    pub folders: Vec<semio_framework_repo_tree::FolderRecord>,
    pub files: Vec<semio_framework_repo_tree::FileRecord>,
    pub goals: Vec<semio_framework_repo_tree::GoalRecord>,
    pub tickets: Vec<semio_framework_repo_tree::TicketRecord>,
    pub drafts: Vec<semio_framework_repo_tree::DraftRecord>,
    pub policies: Vec<semio_framework_repo_tree::PolicyRecord>,
    pub contributors: Vec<semio_framework_repo_tree::ContributorRecord>,
    pub checkpoints: Vec<semio_framework_repo_tree::CheckpointRecord>,
}

impl FsTreeSource {
    /// 🎯️ Reads the records of one repository-relative subtree; an empty scope is the whole
    /// repository and also loads the aggregates that have no place in the codebase.
    pub fn load(context: &dyn RepoContext, include_sections: bool, scope: &str) -> FsTreeSource {
        let scope = normalize_tree_scope(scope);
        let mut source = FsTreeSource { root: context.root_dir(), include_sections, ..FsTreeSource::default() };
        let bundles = context.bundles();
        for technology in context.technologies() {
            let id = model_technology_id(&technology);
            let records: Vec<semio_framework_repo_tree::BundleRecord> = bundles
                .iter()
                .filter(|bundle| bundle.technology_name == technology.name && path_within_scope(&bundle.root, &scope))
                .map(|bundle| {
                    let id = model_bundle_id(bundle);
                    semio_framework_repo_tree::BundleRecord { uri: format!("repo://bundle/{id}"), id, name: bundle.name.clone(), kind: bundle.kind.as_str().to_string(), emoji: bundle.emoji.clone(), root: bundle.root.clone(), source_root: bundle.source_root.clone() }
                })
                .collect();
            if !scope.is_empty() && records.is_empty() {
                continue;
            }
            source.technologies.push(semio_framework_repo_tree::TechnologyRecord { uri: format!("repo://technology/{id}"), id, name: technology.name.clone(), kind: technology.kind.as_str().to_string(), emoji: technology.emoji.clone(), bundles: records });
        }
        for folder in context.folders().into_iter().filter(|folder| path_within_scope(&folder.path, &scope)) {
            source.folders.push(semio_framework_repo_tree::FolderRecord { id: folder.id, path: folder.path, name: folder.name, uri: folder.uri, kind: folder.kind.as_str().to_string(), parent_id: folder.parent_id.unwrap_or_default() });
        }
        for file in context.files().into_iter().filter(|file| path_within_scope(&file.path, &scope)) {
            source.files.push(semio_framework_repo_tree::FileRecord { id: file.id, path: file.path, name: file.name, uri: file.uri, kind: file.kind, parent_id: file.folder_id.unwrap_or_default() });
        }
        if !scope.is_empty() {
            return source;
        }
        for goal in context.goals().unwrap_or_default() {
            source.goals.push(semio_framework_repo_tree::GoalRecord { uri: format!("repo://goal/{}", goal.id), id: goal.id, title: goal.title, status: goal.status.as_str().to_string(), due_date: goal.dates.due, description: goal.description, ..Default::default() });
        }
        for ticket in context.tickets(None, None, None, None).unwrap_or_default() {
            source.tickets.push(semio_framework_repo_tree::TicketRecord {
                id: format!("{}/{}/{}/{}", ticket.year, ticket.month, ticket.day, ticket.slug),
                uri: format!("repo://ticket/{}", ticket.slug),
                slug: ticket.slug,
                title: ticket.title,
                description: ticket.description,
                summary: ticket.summary,
                goal: ticket.goal,
                parent: ticket.parent,
                year: ticket.year,
                month: ticket.month,
                day: ticket.day,
                status: ticket.status.as_str().to_string(),
                ..Default::default()
            });
        }
        for draft in context.drafts().unwrap_or_default() {
            source.drafts.push(semio_framework_repo_tree::DraftRecord { uri: format!("repo://draft/{}", draft.id), id: draft.id, ..Default::default() });
        }
        for policy in context.policies() {
            source.policies.push(semio_framework_repo_tree::PolicyRecord { id: policy.id, name: policy.name, description: policy.description.unwrap_or_default(), groups: policy.groups.unwrap_or_default() });
        }
        for contributor in context.contributors().unwrap_or_default() {
            source.contributors.push(semio_framework_repo_tree::ContributorRecord {
                id: contributor.alias.clone(),
                uri: format!("repo://contributor/{}", contributor.alias),
                alias: contributor.alias,
                name: contributor.name,
                github: contributor.github,
                email: contributor.emails.first().cloned().unwrap_or_default(),
                emails: contributor.emails,
                ..Default::default()
            });
        }
        for checkpoint in context.checkpoints(Some(100)).unwrap_or_default() {
            source.checkpoints.push(semio_framework_repo_tree::CheckpointRecord { id: checkpoint.id, uri: format!("repo://checkpoint/{}", checkpoint.sha), sha: checkpoint.sha, title: checkpoint.title, author_id: checkpoint.author_id.unwrap_or_default(), ..Default::default() });
        }
        source
    }
}

/// 📑️ One parsed section as the record the tree module reads.
fn section_record(section: &semio_framework_repo_model::Section, file_path: &str) -> semio_framework_repo_tree::SectionRecord {
    semio_framework_repo_tree::SectionRecord {
        id: section.id.clone(),
        path: if section.path.is_empty() { format!("{file_path}#{}", section.name) } else { section.path.clone() },
        name: section.name.clone(),
        start_line: section.start_line,
        end_line: section.end_line,
        definitions: section
            .definitions
            .iter()
            .map(|definition| semio_framework_repo_tree::DefinitionRecord { id: definition.id.clone(), name: definition.name.clone(), kind: definition.kind.as_str().to_string(), file_path: file_path.to_string(), start_line: definition.start_line, end_line: definition.end_line, ..Default::default() })
            .collect(),
        children: section.children.iter().map(|child| section_record(child, file_path)).collect(),
        ..Default::default()
    }
}

impl semio_framework_repo_tree::TreeSource for FsTreeSource {
    fn technologies(&self) -> Vec<semio_framework_repo_tree::TechnologyRecord> {
        self.technologies.clone()
    }
    fn folders(&self) -> Vec<semio_framework_repo_tree::FolderRecord> {
        self.folders.clone()
    }
    fn files(&self) -> Vec<semio_framework_repo_tree::FileRecord> {
        self.files.clone()
    }
    fn goals(&self) -> Vec<semio_framework_repo_tree::GoalRecord> {
        self.goals.clone()
    }
    fn tickets(&self) -> Vec<semio_framework_repo_tree::TicketRecord> {
        self.tickets.clone()
    }
    fn drafts(&self) -> Vec<semio_framework_repo_tree::DraftRecord> {
        self.drafts.clone()
    }
    fn policies(&self) -> Vec<semio_framework_repo_tree::PolicyRecord> {
        self.policies.clone()
    }
    fn contributors(&self) -> Vec<semio_framework_repo_tree::ContributorRecord> {
        self.contributors.clone()
    }
    fn checkpoints(&self) -> Vec<semio_framework_repo_tree::CheckpointRecord> {
        self.checkpoints.clone()
    }
    fn sessions(&self) -> Vec<semio_framework_repo_tree::SessionRecord> {
        Vec::new()
    }
    fn sections(&self, file_path: &str) -> Vec<semio_framework_repo_tree::SectionRecord> {
        if !self.include_sections {
            return Vec::new();
        }
        let Ok(content) = std::fs::read_to_string(std::path::Path::new(&self.root).join(file_path)) else { return Vec::new() };
        semio_framework_repo_languages::parse_sections(&content, file_path).iter().map(|section| section_record(section, file_path)).collect()
    }
}

//#endregion 🗂️TreeSource

//#region 🪪️ModelIds

/// 📜️ A technology's artifact id, twin of `Technology.GetID`.
pub fn model_technology_id(technology: &semio_framework_repo_model::Technology) -> String {
    let emoji = if technology.emoji.is_empty() { technology.kind.as_str().to_string() } else { technology.emoji.clone() };
    emoji_text(&emoji) + &flat(&technology.name)
}

/// 🔵️ A bundle's artifact id, twin of `Bundle.GetID`: the technology segment resolves its emoji
/// from the technologies of the repository the process runs in.
pub fn model_bundle_id(bundle: &semio_framework_repo_model::Bundle) -> String {
    let emoji = if bundle.emoji.is_empty() {
        match bundle.kind.as_str() {
            "schema" => emoji_text(identity_entity("bundle-schema")),
            "binary" => emoji_text(identity_entity("bundle-binary")),
            "ui" => emoji_text(identity_entity("bundle-ui")),
            "site" => emoji_text(identity_entity("bundle-site")),
            "assets" => emoji_text(identity_entity("bundle-assets")),
            "library" => emoji_text(identity_entity("bundle-library")),
            "example" => emoji_text(identity_entity("bundle-example")),
            "repo" => emoji_text(identity_entity("bundle-repo")),
            _ => String::new(),
        }
    } else {
        emoji_text(&bundle.emoji)
    };
    let (technology_code, bundle_code) = match bundle.name.split_once('/') {
        Some((head, tail)) => (head.to_string(), tail.to_string()),
        None => (bundle.name.clone(), bundle.name.clone()),
    };
    let technology_emoji = process_codebase()
        .technologies()
        .into_iter()
        .find(|technology| technology.name == technology_code)
        .map(|technology| technology.emoji)
        .filter(|emoji| !emoji.is_empty())
        .unwrap_or_else(|| semio_framework_repo_model::derive_technology_kind(&technology_code).as_str().to_string());
    emoji_text(&technology_emoji) + &flat(&technology_code) + &emoji + &flat(&bundle_code)
}

/// 🗂️ The codebase of the repository the process runs in, which Go's global registries read.
fn process_codebase() -> Codebase {
    Codebase::new(semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))))
}

/// 💗️ A section's artifact id, twin of `Section.GetID`.
pub fn model_section_id(codebase: &Codebase, section: &semio_framework_repo_model::Section) -> String {
    if section.file_path.is_empty() {
        let emoji = if section.emoji.is_empty() { identity_entity("section").to_string() } else { section.emoji.clone() };
        return emoji_text(&emoji) + &flat(&section.name);
    }
    let file_id = codebase.build_file_id(&section.file_path);
    if !section.path.is_empty() {
        return semio_framework_repo_model::build_section_id(&file_id, &section.path.replace('#', "/").split('/').map(str::to_string).collect::<Vec<String>>());
    }
    semio_framework_repo_model::build_section_id(&file_id, &[format!("{}{}", section.emoji, section.name)])
}

/// 💕️ A definition's artifact id, twin of `Definition.GetID`.
pub fn model_definition_id(codebase: &Codebase, definition: &semio_framework_repo_model::Definition) -> String {
    if definition.file_path.is_empty() {
        let mut data = Map::new();
        data.insert("kind".to_string(), Json::String(definition.kind.as_str().to_string()));
        return entity::artifact_id("definition", &data);
    }
    let parts: Vec<String> = if definition.section_path.is_empty() { Vec::new() } else { definition.section_path.replace('#', "/").split('/').map(str::to_string).collect() };
    semio_framework_repo_model::build_definition_id(&codebase.build_file_id(&definition.file_path), &parts, &definition.name, definition.kind)
}

/// 📦️ Every snapshot entity of a repository in taxonomy order, twin of `repoExportSource`.
pub fn export_entities(context: &dyn RepoContext) -> Result<Vec<semio_framework_repo_events::ExportEntity>, String> {
    let codebase = process_codebase();
    let payload = |value: Result<semio_framework_repo_events::Payload, semio_framework_repo_events::StoreError>| value.map_err(|error| error.to_string());
    let mut entities = Vec::new();
    for value in context.technologies() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "technology".to_string(), id: model_technology_id(&value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    for value in context.bundles() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "bundle".to_string(), id: model_bundle_id(&value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    for value in context.folders() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "folder".to_string(), id: codebase.build_folder_id(&value.path), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    for value in context.files() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "file".to_string(), id: codebase.build_file_id(&value.path), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    for value in context.sections() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "section".to_string(), id: model_section_id(&codebase, &value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    for value in context.definitions() {
        entities.push(semio_framework_repo_events::ExportEntity { kind: "definition".to_string(), id: model_definition_id(&codebase, &value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
    }
    Ok(entities)
}

//#endregion 🪪️ModelIds

//#region 🔎️StreamOptions

/// 🧹️ The stream filters a list verb applies, twin of `model.StreamOptions`.
#[derive(Clone, Debug, Default)]
pub struct StreamOptions {
    pub exclude_kinds: Vec<String>,
    pub include_kinds: Vec<String>,
    pub exclude_bundle_kinds: Vec<String>,
    pub include_bundle_kinds: Vec<String>,
    pub exclude_definition_kinds: Vec<String>,
    pub include_definition_kinds: Vec<String>,
    pub filter: String,
    pub query: String,
    pub match_case: bool,
    pub match_whole_word: bool,
}

impl StreamOptions {
    /// 🧹️ Reads the options the stream flags carry, twin of `getStreamOptions`.
    pub fn of(invocation: &Invocation<'_>) -> StreamOptions {
        let pick = |names: &[(&str, &str)], prefix: &str| names.iter().filter(|(flag, _)| invocation.flag(&format!("{prefix}-{flag}"))).map(|(_, kind)| (*kind).to_string()).collect::<Vec<String>>();
        let files = [("code", "code"), ("script", "script"), ("config", "config"), ("lab", "lab"), ("docs", "docs"), ("resource", "resource"), ("template", "template"), ("license", "license")];
        let bundles = [("library", "library"), ("schema", "schema"), ("binary", "binary"), ("client", "ui"), ("site", "site"), ("assets", "assets")];
        let definitions = [("implementation", "implementation"), ("interface", "interface"), ("constant", "constant")];
        StreamOptions {
            exclude_kinds: pick(&files, "no"),
            include_kinds: pick(&files, "only"),
            exclude_bundle_kinds: pick(&bundles, "no"),
            include_bundle_kinds: pick(&bundles, "only"),
            exclude_definition_kinds: pick(&definitions, "no"),
            include_definition_kinds: pick(&definitions, "only"),
            filter: invocation.text("filter"),
            query: invocation.text("query"),
            match_case: invocation.flag("match-case"),
            match_whole_word: invocation.flag("match-whole-word"),
        }
    }

    /// 🟤️ Whether a name passes the literal filter, twin of `MatchesFilter`.
    pub fn matches_filter(&self, name: &str) -> bool {
        if self.filter.is_empty() {
            return true;
        }
        let (target, pattern) = if self.match_case { (name.to_string(), self.filter.clone()) } else { (name.to_lowercase(), self.filter.to_lowercase()) };
        if self.match_whole_word {
            return target.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_')).any(|word| word == pattern);
        }
        target.contains(&pattern)
    }

    /// 🔍️ Whether a text passes the fuzzy query, twin of `MatchesQuery`.
    pub fn matches_query(&self, text: &str) -> bool {
        if self.query.is_empty() || text.to_lowercase().contains(&self.query.to_lowercase()) {
            return true;
        }
        let queries: Vec<semio_framework_repo_search::MatchQuery> = self.query.split_whitespace().map(|term| semio_framework_repo_search::MatchQuery::new(&term.to_lowercase()).with_fuzziness(2)).collect();
        semio_framework_repo_search::score(&text.to_lowercase(), &queries).is_some()
    }

    /// 🔵️ Whether a bundle kind passes the include and exclude lists.
    pub fn includes_bundle_kind(&self, kind: &str) -> bool {
        (self.include_bundle_kinds.is_empty() || self.include_bundle_kinds.iter().any(|wanted| wanted == kind)) && !self.exclude_bundle_kinds.iter().any(|unwanted| unwanted == kind)
    }

    /// 💕️ Whether a definition kind passes the include and exclude lists.
    pub fn includes_definition_kind(&self, kind: &str) -> bool {
        (self.include_definition_kinds.is_empty() || self.include_definition_kinds.iter().any(|wanted| wanted == kind)) && !self.exclude_definition_kinds.iter().any(|unwanted| unwanted == kind)
    }
}

/// 🔲️ The status filter of a list verb, twin of `getStatusFilter`.
pub fn status_filter(invocation: &Invocation<'_>) -> Option<String> {
    if invocation.flag("open") {
        return Some("open".to_string());
    }
    if invocation.flag("closed") {
        return Some("closed".to_string());
    }
    Some(invocation.text("status")).filter(|status| !status.is_empty())
}

/// 🧹️ The monorepo-tree filter the tree flags carry, twin of `buildTreeFilterFromFlags`.
pub fn tree_filter(invocation: &Invocation<'_>) -> TreeFilter {
    let kinds = [
        ("technology", TreeNodeKind::Technology),
        ("bundle", TreeNodeKind::Bundle),
        ("folder", TreeNodeKind::Folder),
        ("file", TreeNodeKind::File),
        ("section", TreeNodeKind::Section),
        ("definition", TreeNodeKind::Definition),
        ("goal", TreeNodeKind::Goal),
        ("ticket", TreeNodeKind::Ticket),
        ("draft", TreeNodeKind::Draft),
        ("policy", TreeNodeKind::Policy),
        ("contributor", TreeNodeKind::Contributor),
        ("checkpoint", TreeNodeKind::Checkpoint),
        ("statute", TreeNodeKind::Statute),
        ("todo", TreeNodeKind::Todo),
        ("breach", TreeNodeKind::Breach),
    ];
    let sub_kinds = [
        ("library", TreeNodeKind::Bundle, "library"),
        ("schema", TreeNodeKind::Bundle, "schema"),
        ("binary", TreeNodeKind::Bundle, "binary"),
        ("client", TreeNodeKind::Bundle, "ui"),
        ("site", TreeNodeKind::Bundle, "site"),
        ("assets", TreeNodeKind::Bundle, "assets"),
        ("organization", TreeNodeKind::Folder, "organization"),
        ("code", TreeNodeKind::File, "code"),
        ("script", TreeNodeKind::File, "script"),
        ("config", TreeNodeKind::File, "config"),
        ("lab", TreeNodeKind::File, "lab"),
        ("docs", TreeNodeKind::File, "docs"),
        ("resource", TreeNodeKind::File, "resource"),
        ("template", TreeNodeKind::File, "template"),
        ("license", TreeNodeKind::File, "license"),
        ("implementation", TreeNodeKind::Definition, "implementation"),
        ("interface", TreeNodeKind::Definition, "interface"),
        ("constant", TreeNodeKind::Definition, "constant"),
    ];
    let mut only_kinds = BTreeMap::new();
    let mut exclude_kinds = BTreeMap::new();
    for (name, kind) in kinds {
        if invocation.flag(&format!("only-{name}")) {
            only_kinds.insert(kind, true);
        }
        if invocation.flag(&format!("no-{name}")) {
            exclude_kinds.insert(kind, true);
        }
    }
    let mut only_sub_kinds: BTreeMap<TreeNodeKind, Vec<String>> = BTreeMap::new();
    let mut exclude_sub_kinds: BTreeMap<TreeNodeKind, Vec<String>> = BTreeMap::new();
    for (name, kind, value) in sub_kinds {
        if invocation.flag(&format!("only-{name}")) {
            only_sub_kinds.entry(kind).or_default().push(value.to_string());
        }
        if invocation.flag(&format!("no-{name}")) {
            exclude_sub_kinds.entry(kind).or_default().push(value.to_string());
        }
    }
    let mut status = String::new();
    if invocation.flag("only-open") || invocation.flag("open") {
        status = "open".to_string();
    }
    if invocation.flag("only-closed") || invocation.flag("closed") {
        status = "closed".to_string();
    }
    let ints = |name: &str| Some(invocation.ints(name)).filter(|items| !items.is_empty());
    let texts = |name: &str| Some(invocation.texts(name)).filter(|items| !items.is_empty());
    TreeFilter {
        query: invocation.text("query"),
        only_kinds: Some(only_kinds),
        exclude_kinds: Some(exclude_kinds),
        only_sub_kinds: Some(only_sub_kinds),
        exclude_sub_kinds: Some(exclude_sub_kinds),
        only_years: ints("only-year"),
        exclude_years: ints("no-year"),
        only_months: ints("only-month"),
        exclude_months: ints("no-month"),
        only_days: ints("only-day"),
        exclude_days: ints("no-day"),
        only_status: status,
        only_contributors: texts("only-contributor-name"),
        exclude_contributors: texts("no-contributor-name"),
        only_policies: texts("only-policy-name"),
        exclude_policies: texts("no-policy-name"),
    }
}

/// 🌳️ Every non-category node of a tree, depth first.
pub fn flatten_tree(node: &TreeNode, out: &mut Vec<TreeNode>) {
    if node.kind != TreeNodeKind::Category {
        out.push(node.clone());
    }
    for child in node.children.as_deref().unwrap_or_default() {
        flatten_tree(child, out);
    }
}

/// 🌳️ The monorepo tree a search or list verb answers from.
pub fn searched_tree(invocation: &Invocation<'_>, positional: usize) -> (TreeNode, TreeFilter) {
    let mut filter = tree_filter(invocation);
    if invocation.args().len() > positional {
        filter.query = invocation.arg(positional);
    }
    let include_sections = filter.only_kinds.as_ref().is_some_and(|kinds| kinds.contains_key(&TreeNodeKind::Section) || kinds.contains_key(&TreeNodeKind::Definition));
    let context = invocation.context();
    let source = FsTreeSource::load(&context, include_sections, "");
    let mut node = semio_framework_repo_tree::build_monorepo_tree(&source, semio_framework_repo_tree::TreeBuildOptions { include_sections, ..Default::default() });
    if !filter.query.is_empty() {
        node = semio_framework_repo_tree::search_tree_in_memory(&node, &filter.query);
    }
    (semio_framework_repo_tree::filter_monorepo_tree(&node, Some(&filter)), filter)
}

/// 🖨️ One tree node in the invocation's format, exactly as `search` renders one.
pub fn write_tree_node(invocation: &mut Invocation<'_>, node: &TreeNode) -> Result<(), CliError> {
    let text = match invocation.config.format {
        Format::Json => format!("{}\n", serde_json::to_string(node).map_err(|error| error.to_string())?),
        Format::Markdown => semio_framework_repo_tree::render_monorepo_tree_markdown(node, &semio_framework_repo_tree::DefaultEntityRenderer),
        Format::Text => semio_framework_repo_tree::render_monorepo_tree(node, &semio_framework_repo_tree::DefaultEntityRenderer),
    };
    invocation.print(&text);
    Ok(())
}

/// 🌲️ A forest under the anonymous category root the monorepo tree carries.
pub fn write_tree_forest(invocation: &mut Invocation<'_>, roots: Vec<TreeNode>) -> Result<(), CliError> {
    let mut node = semio_framework_repo_tree::tree_node(TreeNodeKind::Category, "", ".", "");
    node.children = Some(roots);
    write_tree_node(invocation, &node)
}

/// 🌿️ A box-drawn text tree as log lines.
pub fn text_tree_lines(nodes: &[TreeNode], prefix: &str, lines: &mut Vec<String>) {
    let last = nodes.len().saturating_sub(1);
    for (index, node) in nodes.iter().enumerate() {
        let (connector, child_prefix) = if index == last { ("└️─️─️ ", format!("{prefix}    ")) } else { ("├️─️─️ ", format!("{prefix}│️   ")) };
        let label = if node.description.is_empty() { node.label.clone() } else { format!("{} - {}", node.label, node.description) };
        lines.push(format!("{prefix}{connector}{label}"));
        text_tree_lines(node.children.as_deref().unwrap_or_default(), &child_prefix, lines);
    }
}

//#endregion 🔎️StreamOptions
//#endregion 🧭️RepoCli
