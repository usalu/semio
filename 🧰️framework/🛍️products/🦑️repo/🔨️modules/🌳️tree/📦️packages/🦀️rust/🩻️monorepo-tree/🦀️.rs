//! 🩻️ Monorepo tree: assembly of the technology, bundle, folder, file, section and definition tree plus the goal, draft, policy, contributor, checkpoint and session branches.
//!
//! Holds the `🩻️MonorepoTree` region of the crate root; the root re-exports its items.

use super::*;

//#region 🩻️MonorepoTree

/// 🌳️ The options a monorepo tree build takes.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TreeBuildOptions {
    /// 📑️ Whether every file node carries its parsed section and definition subtree.
    #[serde(default, rename = "includeSections")]
    pub include_sections: bool,
}

/// 🏢️ Assembles the monorepo tree from the records a [`TreeSource`] supplies.
pub fn build_monorepo_tree(source: &dyn TreeSource, options: TreeBuildOptions) -> TreeNode {
    let mut root = tree_node(TreeNodeKind::Category, "", ".", "");
    root.children = Some(Vec::new());
    push_child(&mut root, build_codebase_node(source, options));
    push_child(&mut root, build_goals_node(source));
    push_child(&mut root, build_drafts_node(source));
    push_child(&mut root, build_policies_node(source));
    push_child(&mut root, build_contributors_node(source));
    push_child(&mut root, build_checkpoints_node(source));
    push_child(&mut root, build_sessions_node(source));
    root
}

/// 🏢️ Assembles the monorepo tree and stamps every node with its parent artifact id.
pub fn build_monorepo_tree_with_ids(source: &dyn TreeSource, options: TreeBuildOptions, identifier: &dyn ArtifactIdentifier) -> TreeNode {
    let mut root = build_monorepo_tree(source, options);
    propagate_parent_ids(&mut root, "", identifier);
    root
}

/// 📄️ Projects one file record, with its sections when they are requested.
fn build_file_node(source: &dyn TreeSource, file: &FileRecord, options: TreeBuildOptions) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::File, &file.id, &file.name, &file.uri);
    node.sub_kind = file.kind.clone();
    node.data = Some(data_of(vec![("path", text(&file.path)), ("name", text(&file.name)), ("kind", text(&file.kind)), ("parentId", text(&file.parent_id))]));
    if options.include_sections {
        for section in source.sections(&file.path) {
            push_child(&mut node, build_section_tree_node(&section));
        }
    }
    node
}

/// 📁️ Projects one folder record.
fn build_folder_node(folder: &FolderRecord) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Folder, &folder.id, &folder.name, &folder.uri);
    node.sub_kind = folder.kind.clone();
    node.data = Some(data_of(vec![("path", text(&folder.path)), ("name", text(&folder.name)), ("kind", text(&folder.kind)), ("parentId", text(&folder.parent_id))]));
    node
}

/// 📑️ Projects one section record with its definitions and nested sections.
pub fn build_section_tree_node(section: &SectionRecord) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Section, &section.id, &section.name, &section.uri);
    node.data = Some(data_of(vec![
        ("path", text(&section.path)),
        ("name", text(&section.name)),
        ("startLine", number(section.start_line)),
        ("endLine", number(section.end_line)),
    ]));
    for definition in &section.definitions {
        let mut definition_node = tree_node(TreeNodeKind::Definition, &definition.id, &definition.name, &definition.uri);
        definition_node.sub_kind = definition.kind.clone();
        definition_node.data = Some(data_of(vec![
            ("path", text(&definition.file_path)),
            ("name", text(&definition.name)),
            ("kind", text(&definition.kind)),
            ("startLine", number(definition.start_line)),
            ("endLine", number(definition.end_line)),
        ]));
        push_child(&mut node, definition_node);
    }
    for child in &section.children {
        push_child(&mut node, build_section_tree_node(child));
    }
    node
}

/// 📁️ Nests every folder under its parent folder and returns the folders with no parent.
pub fn build_folder_roots(folders: &[FolderRecord]) -> Vec<TreeNode> {
    let items: Vec<(String, String, TreeNode)> = folders.iter().map(|folder| (folder.path.clone(), parent_path(&folder.path), build_folder_node(folder))).collect();
    nest_ordered(items, &push_child)
}

/// 📄️ Attaches every file node under the folder node of its directory, or under `root`.
pub fn attach_files_to_folders(root: &mut TreeNode, files: &[FileRecord], file_nodes: &BTreeMap<String, TreeNode>) {
    for file in files {
        let Some(node) = file_nodes.get(&file.path) else { continue };
        let folder_path = parent_path(&file.path);
        if !attach_into_folder(root, &folder_path, node.clone()) {
            push_child(root, node.clone());
        }
    }
}

/// 🔎️ Attaches a node under the folder node carrying `folder_path`, reporting whether it landed.
fn attach_into_folder(node: &mut TreeNode, folder_path: &str, file_node: TreeNode) -> bool {
    if node.kind == TreeNodeKind::Folder {
        let matches = node
            .data
            .as_ref()
            .and_then(|data| data.get("path"))
            .and_then(Value::as_str)
            .is_some_and(|path| !path.is_empty() && path == folder_path);
        if matches {
            push_child(node, file_node);
            return true;
        }
    }
    let Some(children) = node.children.as_mut() else { return false };
    for child in children.iter_mut() {
        if attach_into_folder(child, folder_path, file_node.clone()) {
            return true;
        }
    }
    false
}

/// 🗂️ Projects the codebase category: the folder/file hierarchy plus the technology/bundle view.
fn build_codebase_node(source: &dyn TreeSource, options: TreeBuildOptions) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "codebase", &format!("{}Codebase", semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::collection("codebase"))), "repo://codebase");
    let folders = source.folders();
    let files = source.files();
    let file_nodes: BTreeMap<String, TreeNode> = files.iter().map(|file| (file.path.clone(), build_file_node(source, file, options))).collect();
    let folder_nodes: BTreeMap<String, TreeNode> = folders.iter().map(|folder| (folder.path.clone(), build_folder_node(folder))).collect();

    for folder_root in build_folder_roots(&folders) {
        push_child(&mut node, folder_root);
    }
    attach_files_to_folders(&mut node, &files, &file_nodes);

    let mut technologies = source.technologies();
    technologies.sort_by(|left, right| left.name.cmp(&right.name));
    for technology in &technologies {
        let mut technology_node = tree_node(TreeNodeKind::Technology, &technology.id, &technology.name, &technology.uri);
        technology_node.sub_kind = technology.kind.clone();
        technology_node.data = Some(data_of(vec![("name", text(&technology.name)), ("kind", text(&technology.kind)), ("emoji", text(&technology.emoji))]));
        let mut bundles = technology.bundles.clone();
        bundles.sort_by(|left, right| left.name.cmp(&right.name));
        for bundle in &bundles {
            push_child(&mut technology_node, build_bundle_node(bundle, &folder_nodes, &file_nodes));
        }
        push_child(&mut node, technology_node);
    }
    sort_tree_children(&mut node);
    node
}

/// 📦️ Projects one bundle: every folder and file below its source root.
fn build_bundle_node(bundle: &BundleRecord, folder_nodes: &BTreeMap<String, TreeNode>, file_nodes: &BTreeMap<String, TreeNode>) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Bundle, &bundle.id, &bundle.name, &bundle.uri);
    node.sub_kind = bundle.kind.clone();
    node.data = Some(data_of(vec![
        ("name", text(&bundle.name)),
        ("root", text(&bundle.root)),
        ("kind", text(&bundle.kind)),
        ("emoji", text(&bundle.emoji)),
    ]));
    let bundle_root = if bundle.source_root.is_empty() { bundle.root.clone() } else { bundle.source_root.clone() };
    let mut owned: BTreeMap<String, TreeNode> = folder_nodes
        .iter()
        .filter(|(path, _)| is_under(path, &bundle_root))
        .map(|(path, folder_node)| (path.clone(), folder_node.clone()))
        .collect();
    for (path, file_node) in file_nodes {
        if !is_under(path, &bundle_root) {
            continue;
        }
        match owned.get_mut(&parent_path(path)) {
            Some(folder_node) => push_child(folder_node, file_node.clone()),
            None => push_child(&mut node, file_node.clone()),
        }
    }
    let items: Vec<(String, String, TreeNode)> = owned.into_iter().map(|(path, folder_node)| (path.clone(), parent_path(&path), folder_node)).collect();
    for folder_root in nest_ordered(items, &push_child) {
        push_child(&mut node, folder_root);
    }
    sort_tree_children(&mut node);
    node
}

/// 📏️ Whether a path is the given root or lies below it.
fn is_under(path: &str, root: &str) -> bool {
    path == root || path.starts_with(&format!("{root}/"))
}

/// 🎯️ Projects the goals category, nesting subgoals and attaching tickets to their goal.
fn build_goals_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "goals", "🎯️Goals", "repo://goals");
    let mut goals = source.goals();
    goals.sort_by(|left, right| left.id.cmp(&right.id));
    let mut goal_nodes: BTreeMap<String, TreeNode> = BTreeMap::new();
    for goal in &goals {
        let mut goal_node = tree_node(TreeNodeKind::Goal, &goal.artifact_id_or_id(), &goal.title, &goal.uri);
        goal_node.sub_kind = goal.status.clone();
        goal_node.description = goal.description.clone();
        goal_node.status = goal.status.clone();
        goal_node.data = Some(data_of(vec![
            ("id", text(&goal.id)),
            ("title", text(&goal.title)),
            ("status", text(&goal.status)),
            ("dueDate", text(&goal.due_date)),
            ("createdAt", text("")),
            ("description", text(&goal.description)),
        ]));
        goal_nodes.insert(goal.id.clone(), goal_node);
    }

    let mut goal_tickets: BTreeMap<String, Vec<TreeNode>> = BTreeMap::new();
    let mut loose_tickets: Vec<TreeNode> = Vec::new();
    for ticket in source.tickets() {
        let ticket_node = build_ticket_tree_node(&ticket);
        if !ticket.goal.is_empty() && goal_nodes.contains_key(&ticket.goal) {
            goal_tickets.entry(ticket.goal.clone()).or_default().push(ticket_node);
        } else {
            loose_tickets.push(ticket_node);
        }
    }

    let mut roots: Vec<String> = Vec::new();
    let mut subgoals: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for goal in &goals {
        let parent = goal_parent_id(&goal.id);
        if !parent.is_empty() && parent != goal.id && goal_nodes.contains_key(&parent) {
            subgoals.entry(parent).or_default().push(goal.id.clone());
        } else {
            roots.push(goal.id.clone());
        }
    }
    let mut children: Vec<TreeNode> = roots
        .into_iter()
        .filter_map(|id| assemble_goal_node(&id, &mut goal_nodes, &subgoals, &mut goal_tickets))
        .collect();
    children.extend(loose_tickets);
    node.children = Some(children);
    node
}

/// 🎯️ Assembles one goal node: its subgoals first, then the tickets that name it.
fn assemble_goal_node(
    id: &str,
    goal_nodes: &mut BTreeMap<String, TreeNode>,
    subgoals: &BTreeMap<String, Vec<String>>,
    goal_tickets: &mut BTreeMap<String, Vec<TreeNode>>,
) -> Option<TreeNode> {
    let mut node = goal_nodes.remove(id)?;
    for child_id in subgoals.get(id).cloned().unwrap_or_default() {
        if let Some(child) = assemble_goal_node(&child_id, goal_nodes, subgoals, goal_tickets) {
            push_child(&mut node, child);
        }
    }
    for ticket in goal_tickets.remove(id).unwrap_or_default() {
        push_child(&mut node, ticket);
    }
    Some(node)
}

/// 🎫️ Projects one ticket record as a monorepo tree node.
fn build_ticket_tree_node(ticket: &TicketRecord) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Ticket, &ticket.id, &ticket.title, &ticket.uri);
    node.description = ticket.description.clone();
    node.year = ticket.year;
    node.month = ticket.month;
    node.day = ticket.day;
    node.status = ticket.status.clone();
    node.data = Some(data_of(vec![
        ("year", number(ticket.year)),
        ("month", number(ticket.month)),
        ("day", number(ticket.day)),
        ("slug", text(&ticket.slug)),
        ("title", text(&ticket.title)),
        ("status", text(&ticket.status)),
        ("started", text(&ticket.started)),
        ("finished", text(&ticket.finished)),
        ("prompt", text(&ticket.description)),
        ("summary", text(&ticket.summary)),
        ("goalId", text(&ticket.goal)),
    ]));
    node
}

/// 🧬️ Returns the parent goal id of a slash-separated goal id.
pub fn goal_parent_id(id: &str) -> String {
    match id.rfind('/') {
        Some(index) => id[..index].to_string(),
        None => String::new(),
    }
}

impl GoalRecord {
    /// 🆔️ Returns the declared artifact id, falling back to the goal id.
    fn artifact_id_or_id(&self) -> String {
        if self.artifact_id.is_empty() {
            self.id.clone()
        } else {
            self.artifact_id.clone()
        }
    }
}

impl DraftRecord {
    /// 🆔️ Returns the declared artifact id, falling back to the draft id.
    fn artifact_id_or_id(&self) -> String {
        if self.artifact_id.is_empty() {
            self.id.clone()
        } else {
            self.artifact_id.clone()
        }
    }
}

/// ✍️ Projects the drafts category.
fn build_drafts_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "drafts", "✍️Drafts", "repo://drafts");
    for draft in source.drafts() {
        let mut draft_node = tree_node(TreeNodeKind::Draft, &draft.artifact_id_or_id(), &draft.id, &draft.uri);
        draft_node.data = Some(data_of(vec![("id", text(&draft.id)), ("slug", text(&draft.id))]));
        push_child(&mut node, draft_node);
    }
    node
}

/// 🛡️ Projects the policies category with the entity-kind statute tree of every policy.
fn build_policies_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "policies", "🛡️Policies", "repo://policies");
    let mut policies = source.policies();
    policies.sort_by(|left, right| left.id.cmp(&right.id));
    let policy_emoji = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("policy"));
    for policy in &policies {
        let flat = semio_framework_repo_identity::flat(policy.id.trim_start_matches('/'));
        let mut policy_node = tree_node(
            TreeNodeKind::Policy,
            &format!("{policy_emoji}/{}", policy.id),
            &policy.name,
            &format!("repo://policy/{policy_emoji}{flat}"),
        );
        policy_node.description = policy.description.clone();
        policy_node.data = Some(data_of(vec![("id", text(&policy.id)), ("name", text(&policy.name)), ("description", text(&policy.description))]));
        push_child(&mut node, policy_node);
    }
    node
}

/// 🧑️ Projects the contributors category.
fn build_contributors_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "contributors", "🧑️‍💻️Contributors", "repo://contributors");
    let mut contributors = source.contributors();
    contributors.sort_by(|left, right| left.alias.cmp(&right.alias));
    for contributor in &contributors {
        let label = if contributor.name.is_empty() {
            contributor.alias.clone()
        } else {
            format!("{} ({})", contributor.name, contributor.alias)
        };
        let mut contributor_node = tree_node(TreeNodeKind::Contributor, &contributor.id, &label, &contributor.uri);
        contributor_node.contributor = contributor.alias.clone();
        contributor_node.data = Some(data_of(vec![
            ("alias", text(&contributor.alias)),
            ("aliases", list(&contributor.aliases)),
            ("github", text(&contributor.github)),
            ("githubs", list(&contributor.githubs)),
            ("name", text(&contributor.name)),
            ("names", list(&contributor.names)),
            ("email", text(&contributor.email)),
            ("emails", list(&contributor.emails)),
        ]));
        push_child(&mut node, contributor_node);
    }
    node
}

/// 🔀️ Projects the checkpoints category.
fn build_checkpoints_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "checkpoints", "🔀️Checkpoints", "repo://checkpoints");
    for checkpoint in source.checkpoints() {
        let short: String = checkpoint.sha.chars().take(8).collect();
        let mut checkpoint_node = tree_node(TreeNodeKind::Checkpoint, &checkpoint.id, &format!("{short} {}", checkpoint.title), &checkpoint.uri);
        checkpoint_node.year = checkpoint.year;
        checkpoint_node.month = checkpoint.month;
        checkpoint_node.day = checkpoint.day;
        checkpoint_node.data = Some(data_of(vec![
            ("sha", text(&checkpoint.sha)),
            ("message", text(&checkpoint.title)),
            ("authorId", text(&checkpoint.author_id)),
        ]));
        push_child(&mut node, checkpoint_node);
    }
    node
}

/// ⚪️ Projects the sessions category, newest first.
fn build_sessions_node(source: &dyn TreeSource) -> TreeNode {
    let mut node = tree_node(TreeNodeKind::Category, "sessions", "⚪️Sessions", "repo://sessions");
    let mut sessions = source.sessions();
    sessions.sort_by(|left, right| right.started_at.cmp(&left.started_at));
    for session in &sessions {
        let mut label = format!("{} {}", session.kind_emoji, session.uuid);
        if !session.client.is_empty() {
            label.push_str(&format!(" ({})", session.client));
        }
        let mut session_node = tree_node(TreeNodeKind::Session, &session.id, &label, &session.uri);
        session_node.sub_kind = session.kind.clone();
        session_node.year = session.year;
        session_node.month = session.month;
        session_node.day = session.day;
        session_node.status = session.kind.clone();
        session_node.data = Some(data_of(vec![
            ("uuid", text(&session.uuid)),
            ("kind", text(&session.kind)),
            ("client", text(&session.client)),
            ("startedAt", text(&session.started_at)),
            ("year", number(session.year)),
            ("month", number(session.month)),
            ("day", number(session.day)),
            ("checkpoint", text(&session.checkpoint)),
        ]));
        push_child(&mut node, session_node);
    }
    node
}

/// 🧬️ Stamps every entity node with the artifact id of its nearest entity ancestor.
pub fn propagate_parent_ids(node: &mut TreeNode, parent_artifact_id: &str, identifier: &dyn ArtifactIdentifier) {
    let data = node.data.get_or_insert_with(BTreeMap::new);
    let entity_kind = tree_node_kind_to_entity_kind(node.kind);
    let current = if entity_kind.is_empty() {
        parent_artifact_id.to_string()
    } else {
        data.insert("parentId".to_string(), text(parent_artifact_id));
        identifier.artifact_id(entity_kind, data)
    };
    let Some(children) = node.children.as_mut() else { return };
    for child in children.iter_mut() {
        propagate_parent_ids(child, &current, identifier);
    }
}

/// 🌿️ Maps a tree node kind to the entity kind the renderers and id builders speak.
pub fn tree_node_kind_to_entity_kind(kind: TreeNodeKind) -> &'static str {
    match kind {
        TreeNodeKind::Category => "",
        TreeNodeKind::Statute => "",
        other => other.as_str(),
    }
}

//#endregion 🩻️MonorepoTree
