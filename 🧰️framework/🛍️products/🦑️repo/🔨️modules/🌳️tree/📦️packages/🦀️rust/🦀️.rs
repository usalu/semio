//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🌳️ Repository tree projection: the monorepo tree assembled from already-loaded records, the
//! goal/ticket tree, the statute and territory trees, parent-id propagation, filtering,
//! deterministic sorting, in-memory search, text/markdown/Mermaid rendering and the
//! content-digest tree cache.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/tree`. Nothing in this crate touches the
//! filesystem, git or a process: every door to the outside world is a trait in
//! [`ports`](#ports) — [`TreeSource`] supplies the records, [`ArtifactIdentifier`] mints the
//! artifact ids parent-id propagation needs, [`EntityRenderer`] renders one entity line and
//! [`StatuteCatalog`] resolves statute metadata. The CLI satisfies them with the loader,
//! identity and statutes implementations; the language-agnostic tests satisfy them with the
//! committed fixtures, so no test needs a repository on disk.
//!
//! Ordering is deterministic everywhere. The Go original leans on Go map iteration in the
//! codebase and bundle assembly and then repairs the order with `sortTreeChildren`; this crate
//! keeps the same repair but builds through ordered maps, so the intermediate order is stable as
//! well.
//!
//! See `🧬️schema/🔣️.json` for the wire shapes and `🧪️tests/` for the language-agnostic cases.

//#endregion 🧲️Header

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// 🔣️ The JSON value a tree node's data map carries. Re-exported explicitly so a client never
/// has to name the encoder crate itself.
pub use serde_json::Value;

/// 🔏️ The digest and compression codec of the cache envelope, owned by 📜️statutes and shared
/// with this crate so one hand-rolled SHA-256 and one hand-rolled gzip serve both caches.
pub use semio_framework_repo_statutes::{gzip_decode, gzip_encode, sha256_hex};

pub use semio_framework_repo_model::{
    BreachPriority, BundleKind, FileKind, GoalNode, Statute, StatuteMeta, Technology, TechnologyKind, Territory, TicketNode, TreeFilter, TreeNode, TreeNodeKind,
};

//#region 🏷️Vocabulary

/// 🌿️ The connector drawn in front of a non-last child in the text tree.
pub const TEXT_BRANCH_CONNECTOR: &str = "├️─️─️ ";

/// 🌿️ The connector drawn in front of the last child in the text tree.
pub const TEXT_LAST_CONNECTOR: &str = "└️─️─️ ";

/// 🌿️ The indent carried down a subtree that still has following siblings.
pub const TEXT_PIPE_INDENT: &str = "│️   ";

/// 🌿️ The indent carried down the last subtree of a level.
pub const TEXT_BLANK_INDENT: &str = "    ";

/// 🟢️ The icon of a low-priority statute.
pub const PRIORITY_ICON_LOW: &str = "🟢️";

/// 🟡️ The icon of a medium-priority statute.
pub const PRIORITY_ICON_MEDIUM: &str = "🟡️";

/// 🔴️ The icon of a high-priority statute.
pub const PRIORITY_ICON_HIGH: &str = "🔴️";

/// 🎫️ The label of the synthetic goal that collects every ticket without a goal.
pub const NO_GOAL_LABEL: &str = "No Goal";

/// 📌️ The schema version of the tree cache envelope; a mismatch invalidates the cache.
pub const TREE_CACHE_SCHEMA_VERSION: i64 = 3;

//#endregion 🏷️Vocabulary

//#region 🔌️Ports

/// 🪪️ Mints the artifact id of one entity from its tree-node data, as `🪪️identity` defines it.
pub trait ArtifactIdentifier {
    /// 🆔️ Returns the artifact id of an entity of `entity_kind` described by `data`.
    fn artifact_id(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
}

/// 🎨️ Renders one entity as a single line, as the CLI renderers define it.
pub trait EntityRenderer {
    /// 🖥️ Renders the human, single-line form.
    fn human(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
    /// 📰️ Renders the markdown list-item form.
    fn markdown(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
    /// 🔗️ Renders the markdown link form used inside a goal tree line.
    fn markdown_link(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String;
}

/// 📜️ Resolves statute metadata, ids, uris and labels, as `📜️statutes` defines them.
pub trait StatuteCatalog {
    /// 🔖️ Returns the declared metadata of a statute.
    fn info(&self, statute: &Statute) -> Option<StatuteMeta>;
    /// 🆔️ Returns the artifact id of a statute.
    fn statute_id(&self, statute: &Statute) -> String;
    /// 🔗️ Returns the `repo://` uri of a statute.
    fn statute_uri(&self, statute: &Statute) -> String;
    /// 🏷️ Returns the display label of a statute path, without the priority icon.
    fn statute_label(&self, statute: &Statute) -> String;
    /// 🧱️ Returns the entity kind a statute applies to.
    fn entity_kind(&self, statute: &Statute) -> String;
    /// 🆔️ Returns the artifact id of a territory.
    fn territory_id(&self, territory: &Territory) -> String;
}

/// 🗂️ Supplies the already-loaded records the monorepo tree projects.
pub trait TreeSource {
    /// 🧪️ The technologies with their bundles, in load order.
    fn technologies(&self) -> Vec<TechnologyRecord>;
    /// 📁️ Every folder of the repository.
    fn folders(&self) -> Vec<FolderRecord>;
    /// 📄️ Every file of the repository.
    fn files(&self) -> Vec<FileRecord>;
    /// 🎯️ Every goal.
    fn goals(&self) -> Vec<GoalRecord>;
    /// 🎫️ Every ticket.
    fn tickets(&self) -> Vec<TicketRecord>;
    /// ✍️ Every draft.
    fn drafts(&self) -> Vec<DraftRecord>;
    /// 🛡️ Every policy.
    fn policies(&self) -> Vec<PolicyRecord>;
    /// 🧑️ Every contributor.
    fn contributors(&self) -> Vec<ContributorRecord>;
    /// 🔀️ Every checkpoint.
    fn checkpoints(&self) -> Vec<CheckpointRecord>;
    /// ⚪️ Every session.
    fn sessions(&self) -> Vec<SessionRecord>;
    /// 📑️ The parsed sections of one file, read only when sections are requested.
    fn sections(&self, file_path: &str) -> Vec<SectionRecord>;
}

/// 🪪️ Mints artifact ids through `📐️model`'s own id builder, the repository's one grammar.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultArtifactIdentifier;

impl ArtifactIdentifier for DefaultArtifactIdentifier {
    fn artifact_id(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::artifact_id(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }
}

/// 🎨️ Renders entities through `📐️model`'s own entity templates, without terminal colour.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultEntityRenderer;

impl EntityRenderer for DefaultEntityRenderer {
    fn human(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_human(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect(), false)
    }

    fn markdown(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_markdown(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }

    fn markdown_link(&self, entity_kind: &str, data: &BTreeMap<String, Value>) -> String {
        semio_framework_repo_model::entity::render_markdown_link(entity_kind, &data.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
    }
}

//#endregion 🔌️Ports

//#region 💿️Records

/// 🧪️ A technology with its bundles.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TechnologyRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub bundles: Vec<BundleRecord>,
}

/// 📦️ One bundle of a technology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub root: String,
    #[serde(default, rename = "sourceRoot")]
    pub source_root: String,
}

/// 📁️ One folder of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FolderRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    /// 🪜️ The identity of the folder this one sits in, empty at the repository root.
    #[serde(default)]
    pub parent_id: String,
}

/// 📄️ One file of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    /// 🗃️ The identity of the folder this file sits in, empty at the repository root.
    #[serde(default)]
    pub parent_id: String,
}

/// 📑️ One section of a file, with its definitions and nested sections.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectionRecord {
    pub id: String,
    pub path: String,
    pub name: String,
    pub uri: String,
    #[serde(default, rename = "startLine")]
    pub start_line: i64,
    #[serde(default, rename = "endLine")]
    pub end_line: i64,
    #[serde(default)]
    pub definitions: Vec<DefinitionRecord>,
    #[serde(default)]
    pub children: Vec<SectionRecord>,
}

/// 🏷️ One definition inside a section.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DefinitionRecord {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    #[serde(default, rename = "filePath")]
    pub file_path: String,
    #[serde(default, rename = "startLine")]
    pub start_line: i64,
    #[serde(default, rename = "endLine")]
    pub end_line: i64,
}

/// 🎯️ One goal.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalRecord {
    pub id: String,
    pub title: String,
    pub uri: String,
    #[serde(default, rename = "artifactId")]
    pub artifact_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "dueDate")]
    pub due_date: String,
    #[serde(default, rename = "createdAt")]
    pub created_at: String,
    #[serde(default)]
    pub description: String,
}

/// 🎫️ One ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketRecord {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub started: String,
    #[serde(default)]
    pub finished: String,
}

/// ✍️ One draft.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DraftRecord {
    pub id: String,
    pub uri: String,
    #[serde(default, rename = "artifactId")]
    pub artifact_id: String,
}

/// 🛡️ One policy with its territories.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecord {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub groups: Vec<Territory>,
}

/// 🧑️ One contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorRecord {
    pub id: String,
    pub alias: String,
    pub uri: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub githubs: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub emails: Vec<String>,
}

/// 🔀️ One checkpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointRecord {
    pub id: String,
    pub sha: String,
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default, rename = "authorId")]
    pub author_id: String,
}

/// ⚪️ One agent session.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SessionRecord {
    pub id: String,
    pub uuid: String,
    pub uri: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default, rename = "kindEmoji")]
    pub kind_emoji: String,
    #[serde(default)]
    pub client: String,
    #[serde(default, rename = "startedAt")]
    pub started_at: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub checkpoint: String,
}

/// 🗄️ A [`TreeSource`] backed entirely by already-decoded records, the shape the fixtures carry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MemoryTreeSource {
    #[serde(default)]
    pub technologies: Vec<TechnologyRecord>,
    #[serde(default)]
    pub folders: Vec<FolderRecord>,
    #[serde(default)]
    pub files: Vec<FileRecord>,
    #[serde(default)]
    pub goals: Vec<GoalRecord>,
    #[serde(default)]
    pub tickets: Vec<TicketRecord>,
    #[serde(default)]
    pub drafts: Vec<DraftRecord>,
    #[serde(default)]
    pub policies: Vec<PolicyRecord>,
    #[serde(default)]
    pub contributors: Vec<ContributorRecord>,
    #[serde(default)]
    pub checkpoints: Vec<CheckpointRecord>,
    #[serde(default)]
    pub sessions: Vec<SessionRecord>,
    #[serde(default)]
    pub sections: BTreeMap<String, Vec<SectionRecord>>,
}

impl MemoryTreeSource {
    /// 📥️ Decodes a record set from its JSON document.
    pub fn from_json(document: &str) -> Result<MemoryTreeSource, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }
}

impl TreeSource for MemoryTreeSource {
    fn technologies(&self) -> Vec<TechnologyRecord> {
        self.technologies.clone()
    }
    fn folders(&self) -> Vec<FolderRecord> {
        self.folders.clone()
    }
    fn files(&self) -> Vec<FileRecord> {
        self.files.clone()
    }
    fn goals(&self) -> Vec<GoalRecord> {
        self.goals.clone()
    }
    fn tickets(&self) -> Vec<TicketRecord> {
        self.tickets.clone()
    }
    fn drafts(&self) -> Vec<DraftRecord> {
        self.drafts.clone()
    }
    fn policies(&self) -> Vec<PolicyRecord> {
        self.policies.clone()
    }
    fn contributors(&self) -> Vec<ContributorRecord> {
        self.contributors.clone()
    }
    fn checkpoints(&self) -> Vec<CheckpointRecord> {
        self.checkpoints.clone()
    }
    fn sessions(&self) -> Vec<SessionRecord> {
        self.sessions.clone()
    }
    fn sections(&self, file_path: &str) -> Vec<SectionRecord> {
        self.sections.get(file_path).cloned().unwrap_or_default()
    }
}

/// 🗄️ A [`StatuteCatalog`] backed by declared entries, the shape the fixtures carry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MemoryStatuteCatalog {
    #[serde(default)]
    pub statutes: Vec<StatuteCatalogEntry>,
    #[serde(default)]
    pub territories: BTreeMap<String, String>,
}

/// 📜️ One declared statute of a [`MemoryStatuteCatalog`].
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatuteCatalogEntry {
    pub id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub uri: String,
    pub label: String,
    #[serde(rename = "entityKind")]
    pub entity_kind: String,
    pub meta: StatuteMetaRecord,
}

/// 🔖️ The declared metadata of one catalog entry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StatuteMetaRecord {
    #[serde(default, rename = "policyId")]
    pub policy_id: String,
    pub priority: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub solution: String,
    #[serde(default)]
    pub autofixable: bool,
}

impl MemoryStatuteCatalog {
    /// 📥️ Decodes a catalog from its JSON document.
    pub fn from_json(document: &str) -> Result<MemoryStatuteCatalog, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🔎️ Returns the declared entry of a statute.
    fn entry(&self, statute: &Statute) -> Option<&StatuteCatalogEntry> {
        self.statutes.iter().find(|entry| entry.id == statute.0)
    }
}

impl StatuteCatalog for MemoryStatuteCatalog {
    fn info(&self, statute: &Statute) -> Option<StatuteMeta> {
        self.entry(statute).map(|entry| StatuteMeta {
            kind: statute.clone(),
            policy_id: entry.meta.policy_id.clone(),
            priority: parse_breach_priority(&entry.meta.priority),
            reason: entry.meta.reason.clone(),
            solution: entry.meta.solution.clone(),
            autofixable: entry.meta.autofixable,
        })
    }
    fn statute_id(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.artifact_id.clone()).unwrap_or_default()
    }
    fn statute_uri(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.uri.clone()).unwrap_or_default()
    }
    fn statute_label(&self, statute: &Statute) -> String {
        self.entry(statute).map_or_else(|| statute.0.clone(), |entry| entry.label.clone())
    }
    fn entity_kind(&self, statute: &Statute) -> String {
        self.entry(statute).map(|entry| entry.entity_kind.clone()).unwrap_or_default()
    }
    fn territory_id(&self, territory: &Territory) -> String {
        self.territories.get(&territory.name).cloned().unwrap_or_else(|| territory.name.clone())
    }
}

/// 🌿️ A compact tree-node literal: every field defaults, so a fixture states only what matters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeNodeSpec {
    pub kind: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub uri: String,
    #[serde(default, rename = "subKind")]
    pub sub_kind: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub contributor: String,
    #[serde(default)]
    pub data: Option<BTreeMap<String, Value>>,
    #[serde(default)]
    pub children: Vec<TreeNodeSpec>,
}

impl TreeNodeSpec {
    /// 📥️ Decodes a node literal from its JSON document.
    pub fn from_json(document: &str) -> Result<TreeNodeSpec, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🌿️ Expands the literal into a real tree node.
    pub fn to_tree_node(&self) -> TreeNode {
        let mut node = tree_node(parse_tree_node_kind(&self.kind), &self.id, &self.label, &self.uri);
        node.sub_kind = self.sub_kind.clone();
        node.description = self.description.clone();
        node.summary = self.summary.clone();
        node.year = self.year;
        node.month = self.month;
        node.day = self.day;
        node.status = self.status.clone();
        node.contributor = self.contributor.clone();
        node.data = self.data.clone();
        if !self.children.is_empty() {
            node.children = Some(self.children.iter().map(TreeNodeSpec::to_tree_node).collect());
        }
        node
    }
}

/// 🧹️ A compact tree-filter literal: every field defaults, so a fixture states only what matters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeFilterSpec {
    #[serde(default)]
    pub query: String,
    #[serde(default, rename = "onlyKinds")]
    pub only_kinds: Vec<String>,
    #[serde(default, rename = "excludeKinds")]
    pub exclude_kinds: Vec<String>,
    #[serde(default, rename = "onlySubKinds")]
    pub only_sub_kinds: BTreeMap<String, Vec<String>>,
    #[serde(default, rename = "excludeSubKinds")]
    pub exclude_sub_kinds: BTreeMap<String, Vec<String>>,
    #[serde(default, rename = "onlyYears")]
    pub only_years: Vec<i64>,
    #[serde(default, rename = "excludeYears")]
    pub exclude_years: Vec<i64>,
    #[serde(default, rename = "onlyMonths")]
    pub only_months: Vec<i64>,
    #[serde(default, rename = "excludeMonths")]
    pub exclude_months: Vec<i64>,
    #[serde(default, rename = "onlyDays")]
    pub only_days: Vec<i64>,
    #[serde(default, rename = "excludeDays")]
    pub exclude_days: Vec<i64>,
    #[serde(default, rename = "onlyStatus")]
    pub only_status: String,
    #[serde(default, rename = "onlyContributors")]
    pub only_contributors: Vec<String>,
    #[serde(default, rename = "excludeContributors")]
    pub exclude_contributors: Vec<String>,
    #[serde(default, rename = "onlyPolicies")]
    pub only_policies: Vec<String>,
    #[serde(default, rename = "excludePolicies")]
    pub exclude_policies: Vec<String>,
}

impl TreeFilterSpec {
    /// 📥️ Decodes a filter literal from its JSON document.
    pub fn from_json(document: &str) -> Result<TreeFilterSpec, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }

    /// 🧹️ Expands the literal into a real tree filter.
    pub fn to_filter(&self) -> TreeFilter {
        fn kind_set(kinds: &[String]) -> BTreeMap<TreeNodeKind, bool> {
            kinds.iter().map(|kind| (parse_tree_node_kind(kind), true)).collect()
        }
        fn sub_kind_map(entries: &BTreeMap<String, Vec<String>>) -> BTreeMap<TreeNodeKind, Vec<String>> {
            entries.iter().map(|(kind, values)| (parse_tree_node_kind(kind), values.clone())).collect()
        }
        TreeFilter {
            query: self.query.clone(),
            only_kinds: Some(kind_set(&self.only_kinds)),
            exclude_kinds: Some(kind_set(&self.exclude_kinds)),
            only_sub_kinds: Some(sub_kind_map(&self.only_sub_kinds)),
            exclude_sub_kinds: Some(sub_kind_map(&self.exclude_sub_kinds)),
            only_years: Some(self.only_years.clone()),
            exclude_years: Some(self.exclude_years.clone()),
            only_months: Some(self.only_months.clone()),
            exclude_months: Some(self.exclude_months.clone()),
            only_days: Some(self.only_days.clone()),
            exclude_days: Some(self.exclude_days.clone()),
            only_status: self.only_status.clone(),
            only_contributors: Some(self.only_contributors.clone()),
            exclude_contributors: Some(self.exclude_contributors.clone()),
            only_policies: Some(self.only_policies.clone()),
            exclude_policies: Some(self.exclude_policies.clone()),
        }
    }
}

/// 📥️ Decodes a territory forest from its JSON document.
pub fn decode_territories(document: &str) -> Result<Vec<Territory>, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 📥️ Decodes a statute list from its JSON document.
pub fn decode_statutes(document: &str) -> Result<Vec<Statute>, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 🔤️ Decodes a tree node kind slug, defaulting to the structural category kind.
pub fn parse_tree_node_kind(value: &str) -> TreeNodeKind {
    match value {
        "technology" => TreeNodeKind::Technology,
        "bundle" => TreeNodeKind::Bundle,
        "folder" => TreeNodeKind::Folder,
        "file" => TreeNodeKind::File,
        "section" => TreeNodeKind::Section,
        "definition" => TreeNodeKind::Definition,
        "goal" => TreeNodeKind::Goal,
        "ticket" => TreeNodeKind::Ticket,
        "draft" => TreeNodeKind::Draft,
        "todo" => TreeNodeKind::Todo,
        "policy" => TreeNodeKind::Policy,
        "breach" => TreeNodeKind::Breach,
        "contributor" => TreeNodeKind::Contributor,
        "checkpoint" => TreeNodeKind::Checkpoint,
        "session" => TreeNodeKind::Session,
        "statute" => TreeNodeKind::Statute,
        _ => TreeNodeKind::Category,
    }
}

/// 🔤️ Decodes a priority slug, defaulting to the low priority the icon table also defaults to.
pub fn parse_breach_priority(value: &str) -> BreachPriority {
    match value {
        "high" => BreachPriority::High,
        "medium" => BreachPriority::Medium,
        _ => BreachPriority::Low,
    }
}

//#endregion 💿️Records

//#region 🌿️NodeHelpers

/// 🌿️ Builds an empty node of one kind; every projection fills the fields it owns.
pub fn tree_node(kind: TreeNodeKind, id: &str, label: &str, uri: &str) -> TreeNode {
    TreeNode {
        kind,
        id: id.to_string(),
        label: label.to_string(),
        uri: uri.to_string(),
        sub_kind: String::new(),
        description: String::new(),
        summary: String::new(),
        year: 0,
        month: 0,
        day: 0,
        status: String::new(),
        contributor: String::new(),
        data: None,
        children: None,
    }
}

/// 🧒️ Returns the children of a node as a slice, treating the absent list as empty.
pub fn children_of(node: &TreeNode) -> &[TreeNode] {
    node.children.as_deref().unwrap_or(&[])
}

/// ➕️ Appends one child, creating the child list on first use.
pub fn push_child(node: &mut TreeNode, child: TreeNode) {
    node.children.get_or_insert_with(Vec::new).push(child);
}

/// 🗃️ Builds a data map from string pairs.
fn data_of(entries: Vec<(&str, Value)>) -> BTreeMap<String, Value> {
    entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect()
}

/// 🔤️ Wraps a string as a JSON value.
fn text(value: &str) -> Value {
    Value::String(value.to_string())
}

/// 🔢️ Wraps a number as a JSON value, matching the Go encoder that widens every count to float64.
fn number(value: i64) -> Value {
    Value::from(value as f64)
}

/// 📃️ Wraps a string list as a JSON value.
fn list(values: &[String]) -> Value {
    Value::Array(values.iter().map(|value| text(value)).collect())
}

/// 📁️ Returns the parent path of a slash-separated path, matching Go's `filepath.Dir` on the
/// normalised paths the record set carries: no separator yields `.`.
pub fn parent_path(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => path[..index].to_string(),
        None => ".".to_string(),
    }
}

/// 📄️ Returns the last segment of a slash-separated path.
pub fn base_name(path: &str) -> String {
    match path.rfind('/') {
        Some(index) => path[index + 1..].to_string(),
        None => path.to_string(),
    }
}

/// 🪜️ Nests items under the item carrying their parent id, preserving input order at every level.
/// An item whose parent is absent from the set, or that is its own parent, becomes a root.
fn nest_ordered<T>(items: Vec<(String, String, T)>, attach: &dyn Fn(&mut T, T)) -> Vec<T> {
    let positions: BTreeMap<String, usize> = items.iter().enumerate().map(|(index, (id, _, _))| (id.clone(), index)).collect();
    let mut roots: Vec<usize> = Vec::new();
    let mut children: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, (id, parent, _)) in items.iter().enumerate() {
        match positions.get(parent) {
            Some(&parent_index) if parent != id && parent_index != index => children.entry(parent_index).or_default().push(index),
            _ => roots.push(index),
        }
    }
    let mut slots: Vec<Option<T>> = items.into_iter().map(|(_, _, value)| Some(value)).collect();
    roots.into_iter().filter_map(|index| assemble_nested(index, &mut slots, &children, attach)).collect()
}

/// 🪜️ Assembles one item of [`nest_ordered`] together with everything below it.
fn assemble_nested<T>(index: usize, slots: &mut Vec<Option<T>>, children: &BTreeMap<usize, Vec<usize>>, attach: &dyn Fn(&mut T, T)) -> Option<T> {
    let mut node = slots[index].take()?;
    if let Some(kids) = children.get(&index).cloned() {
        for kid in kids {
            if let Some(child) = assemble_nested(kid, slots, children, attach) {
                attach(&mut node, child);
            }
        }
    }
    Some(node)
}

/// 🎨️ Returns the icon of a priority.
pub fn priority_icon(priority: BreachPriority) -> &'static str {
    match priority {
        BreachPriority::High => PRIORITY_ICON_HIGH,
        BreachPriority::Medium => PRIORITY_ICON_MEDIUM,
        _ => PRIORITY_ICON_LOW,
    }
}

//#endregion 🌿️NodeHelpers

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

//#region 🎯️GoalTree

/// 🎯️ Builds the goal/ticket tree: subgoals nested by id path, tickets nested by parent.
pub fn build_goal_tree(goals: &[GoalRecord], tickets: &[TicketRecord]) -> Vec<GoalNode> {
    let mut nodes: BTreeMap<String, GoalNode> = BTreeMap::new();
    for goal in goals {
        nodes.insert(
            goal.id.clone(),
            GoalNode {
                id: goal.id.clone(),
                title: goal.title.clone(),
                status: goal.status.clone(),
                due_date: goal.due_date.clone(),
                created_at: goal.created_at.clone(),
                description: goal.description.clone(),
                children: None,
                tickets: None,
            },
        );
    }

    let mut orphans: Vec<TicketNode> = Vec::new();
    for ticket in tickets {
        let node = TicketNode {
            id: ticket.id.clone(),
            slug: ticket.slug.clone(),
            status: ticket.status.clone(),
            title: ticket.title.clone(),
            uri: ticket_uri(ticket),
            goal_id: ticket.goal.clone(),
            parent_id: ticket.parent.clone(),
            children: None,
            created: ticket.started.clone(),
            finished: ticket.finished.clone(),
            description: ticket.description.clone(),
            summary: ticket.summary.clone(),
        };
        match nodes.get_mut(&ticket.goal) {
            Some(goal) if !ticket.goal.is_empty() => goal.tickets.get_or_insert_with(Vec::new).push(node),
            _ => orphans.push(node),
        }
    }

    for node in nodes.values_mut() {
        let nested = nest_tickets(node.tickets.take().unwrap_or_default());
        node.tickets = if nested.is_empty() { None } else { Some(nested) };
    }

    let items: Vec<(String, String, GoalNode)> = goals
        .iter()
        .filter_map(|goal| nodes.remove(&goal.id).map(|node| (goal.id.clone(), goal_parent_id(&goal.id), node)))
        .collect();
    let mut result = nest_ordered(items, &|parent: &mut GoalNode, child: GoalNode| parent.children.get_or_insert_with(Vec::new).push(child));
    sort_goal_nodes(&mut result);
    if !orphans.is_empty() {
        let nested = nest_tickets(orphans);
        result.push(GoalNode {
            id: String::new(),
            title: NO_GOAL_LABEL.to_string(),
            status: String::new(),
            due_date: String::new(),
            created_at: String::new(),
            description: String::new(),
            children: None,
            tickets: Some(nested),
        });
    }
    result
}

/// 🔗️ Builds the `repo://` uri of a ticket, matching the Go builder.
pub fn ticket_uri(ticket: &TicketRecord) -> String {
    if !ticket.uri.is_empty() {
        return ticket.uri.clone();
    }
    let emoji = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("ticket"));
    format!("repo://ticket/{emoji}{}", semio_framework_repo_identity::flat(&ticket.slug))
}

/// 🪜️ Nests tickets under their parent ticket, returning the ones with no parent in the set.
fn nest_tickets(tickets: Vec<TicketNode>) -> Vec<TicketNode> {
    let items: Vec<(String, String, TicketNode)> = tickets.into_iter().map(|ticket| (ticket.id.clone(), ticket.parent_id.clone(), ticket)).collect();
    nest_ordered(items, &|parent: &mut TicketNode, child: TicketNode| parent.children.get_or_insert_with(Vec::new).push(child))
}

/// 📶️ Sorts goals by due date, blank dates last, ties broken by id, recursively.
pub fn sort_goal_nodes(goals: &mut [GoalNode]) {
    goals.sort_by(|left, right| {
        if left.due_date != right.due_date {
            if left.due_date.is_empty() {
                return std::cmp::Ordering::Greater;
            }
            if right.due_date.is_empty() {
                return std::cmp::Ordering::Less;
            }
            return left.due_date.cmp(&right.due_date);
        }
        left.id.cmp(&right.id)
    });
    for goal in goals.iter_mut() {
        if let Some(children) = goal.children.as_mut() {
            sort_goal_nodes(children);
        }
    }
}

/// 🧮️ Counts the open subgoals below a goal.
pub fn count_open_subgoals(goal: &GoalNode) -> usize {
    let mut count = 0;
    for child in goal.children.as_deref().unwrap_or(&[]) {
        if child.status == "open" {
            count += 1;
        }
        count += count_open_subgoals(child);
    }
    count
}

/// 🧮️ Counts the open tickets below a goal, including the tickets of every subgoal.
pub fn count_open_tickets(goal: &GoalNode) -> usize {
    fn walk(tickets: &[TicketNode], count: &mut usize) {
        for ticket in tickets {
            if ticket.status == "open" {
                *count += 1;
            }
            walk(ticket.children.as_deref().unwrap_or(&[]), count);
        }
    }
    let mut count = 0;
    walk(goal.tickets.as_deref().unwrap_or(&[]), &mut count);
    for child in goal.children.as_deref().unwrap_or(&[]) {
        count += count_open_tickets(child);
    }
    count
}

/// 💿️ The data map a goal line renders from.
pub fn goal_node_data(goal: &GoalNode) -> BTreeMap<String, Value> {
    data_of(vec![
        ("id", text(&goal.id)),
        ("title", text(&goal.title)),
        ("status", text(&goal.status)),
        ("dueDate", text(&goal.due_date)),
        ("createdAt", text(&goal.created_at)),
        ("description", text(&goal.description)),
    ])
}

/// 🌿️ The data map a ticket line renders from.
pub fn ticket_node_data(ticket: &TicketNode) -> BTreeMap<String, Value> {
    data_of(vec![
        ("slug", text(&ticket.slug)),
        ("title", text(&ticket.title)),
        ("status", text(&ticket.status)),
        ("started", text(&ticket.created)),
        ("finished", text(&ticket.finished)),
        ("prompt", text(&ticket.description)),
        ("summary", text(&ticket.summary)),
    ])
}

/// 🖨️ The output format of a goal tree rendering.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeRenderFormat {
    /// 🖥️ The indented text form with box-drawing connectors.
    #[serde(rename = "text")]
    Text,
    /// 📰️ The markdown list form.
    #[serde(rename = "md")]
    Markdown,
}

/// ⛳️ Renders a goal tree, one line per goal and ticket.
pub fn render_goal_tree_nodes(roots: &[GoalNode], format: TreeRenderFormat, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    let last = roots.len().saturating_sub(1);
    for (index, root) in roots.iter().enumerate() {
        render_goal_node(&mut out, root, "", index == last, true, format, renderer);
    }
    out
}

/// 🎯️ Renders one goal and everything below it.
fn render_goal_node(out: &mut String, goal: &GoalNode, prefix: &str, is_last: bool, is_root: bool, format: TreeRenderFormat, renderer: &dyn EntityRenderer) {
    let data = goal_node_data(goal);
    let line = match format {
        TreeRenderFormat::Markdown => renderer.markdown_link("goal", &data),
        TreeRenderFormat::Text => renderer.human("goal", &data),
    };
    let children = goal.children.as_deref().unwrap_or(&[]);
    let tickets = goal.tickets.as_deref().unwrap_or(&[]);
    let total = children.len() + tickets.len();
    let new_prefix = write_tree_line(out, prefix, &line, is_last, is_root, format);
    for (index, child) in children.iter().enumerate() {
        render_goal_node(out, child, &new_prefix, index == total - 1, false, format, renderer);
    }
    for (index, ticket) in tickets.iter().enumerate() {
        render_ticket_node(out, ticket, &new_prefix, children.len() + index == total - 1, false, format, renderer);
    }
}

/// 🎫️ Renders one ticket and everything below it.
fn render_ticket_node(out: &mut String, ticket: &TicketNode, prefix: &str, is_last: bool, is_root: bool, format: TreeRenderFormat, renderer: &dyn EntityRenderer) {
    let data = ticket_node_data(ticket);
    let line = match format {
        TreeRenderFormat::Markdown => renderer.markdown_link("ticket", &data),
        TreeRenderFormat::Text => renderer.human("ticket", &data),
    };
    let children = ticket.children.as_deref().unwrap_or(&[]);
    let new_prefix = write_tree_line(out, prefix, &line, is_last, is_root, format);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_ticket_node(out, child, &new_prefix, index == last, false, format, renderer);
    }
}

/// ✏️ Writes one rendered line and returns the prefix its children carry.
fn write_tree_line(out: &mut String, prefix: &str, line: &str, is_last: bool, is_root: bool, format: TreeRenderFormat) -> String {
    match format {
        TreeRenderFormat::Text => {
            let connector = if is_root {
                ""
            } else if is_last {
                TEXT_LAST_CONNECTOR
            } else {
                TEXT_BRANCH_CONNECTOR
            };
            out.push_str(prefix);
            out.push_str(connector);
            out.push_str(line);
            out.push('\n');
            if is_root {
                prefix.to_string()
            } else if is_last {
                format!("{prefix}{TEXT_BLANK_INDENT}")
            } else {
                format!("{prefix}{TEXT_PIPE_INDENT}")
            }
        }
        TreeRenderFormat::Markdown => {
            out.push_str(prefix);
            out.push_str("- ");
            out.push_str(line);
            out.push('\n');
            format!("{prefix}  ")
        }
    }
}

//#endregion 🎯️GoalTree

//#region 📜️StatuteTree

/// 📜️ The catalog of the law this repository actually declares, resolved through `📜️statutes` —
/// the production [`StatuteCatalog`] every consumer of the statute and territory trees shares.
pub struct DeclaredStatuteCatalog;

impl StatuteCatalog for DeclaredStatuteCatalog {
    fn info(&self, statute: &Statute) -> Option<StatuteMeta> {
        semio_framework_repo_statutes::statutes().iter().find(|meta| meta.kind == *statute).cloned()
    }

    fn statute_id(&self, statute: &Statute) -> String {
        format!("📜️{}", statute.0.replace(['/', '.'], ""))
    }

    fn statute_uri(&self, statute: &Statute) -> String {
        format!("repo://statute/{}", statute.0)
    }

    fn statute_label(&self, statute: &Statute) -> String {
        statute.0.rsplit('/').next().unwrap_or(&statute.0).to_string()
    }

    fn entity_kind(&self, statute: &Statute) -> String {
        statute.0.split('/').next().unwrap_or_default().to_string()
    }

    fn territory_id(&self, territory: &Territory) -> String {
        format!("🗺️{}", territory.name.replace([' ', '/'], ""))
    }
}

/// 📜️ Every declared statute, identifier ascending, so the projection never depends on the order a
/// map happened to iterate in.
pub fn declared_statutes() -> Vec<Statute> {
    let mut kinds: Vec<Statute> = semio_framework_repo_statutes::statutes().iter().map(|meta| meta.kind.clone()).collect();
    kinds.sort_by(|left, right| left.0.cmp(&right.0));
    kinds
}

/// 🗺️ Every territory a declared policy carries at its top level, name ascending, each name claimed
/// by the first policy that declares it.
pub fn declared_territories() -> Vec<Territory> {
    let mut seen: BTreeMap<String, Territory> = BTreeMap::new();
    for policy in semio_framework_repo_statutes::policies() {
        for territory in policy.groups.iter().flatten() {
            seen.entry(territory.name.clone()).or_insert_with(|| territory.clone());
        }
    }
    seen.into_values().collect()
}

/// 📜️ Builds the statute tree: one category per path segment, the leaf carrying its metadata.
pub fn build_statute_tree(statutes: &[Statute], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    #[derive(Default)]
    struct Entry {
        node: Option<TreeNode>,
        order: Vec<String>,
        children: BTreeMap<String, Entry>,
    }

    fn insert(entry: &mut Entry, parts: &[String], full: &Statute, catalog: &dyn StatuteCatalog, depth: usize) {
        let Some(part) = parts.get(depth) else { return };
        if !entry.children.contains_key(part) {
            entry.order.push(part.clone());
            let is_leaf = depth == parts.len() - 1;
            let node = if is_leaf {
                let meta = catalog.info(full);
                let priority = meta.as_ref().map_or(BreachPriority::Low, |meta| meta.priority);
                let mut node = tree_node(
                    TreeNodeKind::Statute,
                    &full.0,
                    &format!("{}{part}", priority_icon(priority)),
                    &catalog.statute_uri(full),
                );
                if let Some(meta) = meta {
                    node.description = meta.reason.clone();
                    if meta.autofixable {
                        node.sub_kind = "autofixable".to_string();
                    }
                    node.data = Some(data_of(vec![
                        ("id", text(&full.0)),
                        ("priority", text(meta.priority.as_str())),
                        ("autofixable", Value::Bool(meta.autofixable)),
                        ("reason", text(&meta.reason)),
                        ("solution", text(&meta.solution)),
                    ]));
                }
                node
            } else {
                let prefix = parts[..=depth].join("/");
                tree_node(
                    TreeNodeKind::Category,
                    &format!("breachCategory:{prefix}"),
                    part,
                    &format!("repo://statute/{}", semio_framework_repo_identity::flat(&prefix)),
                )
            };
            entry.children.insert(part.clone(), Entry { node: Some(node), order: Vec::new(), children: BTreeMap::new() });
        }
        let child = entry.children.get_mut(part).expect("child was just inserted");
        insert(child, parts, full, catalog, depth + 1);
    }

    fn collect(entry: &mut Entry, ordered: bool) -> Vec<TreeNode> {
        let keys: Vec<String> = if ordered { entry.order.clone() } else { entry.children.keys().cloned().collect() };
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut result = Vec::new();
        for key in keys {
            if !seen.insert(key.clone()) {
                continue;
            }
            let Some(mut child) = entry.children.remove(&key) else { continue };
            let mut node = child.node.take().expect("every entry carries a node");
            let grandchildren = collect(&mut child, false);
            if !grandchildren.is_empty() {
                node.children = Some(grandchildren);
            }
            result.push(node);
        }
        result
    }

    let mut root = Entry::default();
    for statute in statutes {
        let parts: Vec<String> = statute.0.split('/').map(str::to_string).collect();
        insert(&mut root, &parts, statute, catalog, 0);
    }
    collect(&mut root, true)
}

/// 🔷️ Builds the territory tree: one category per territory, its statutes and nested territories.
pub fn build_territory_tree(territories: &[Territory], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    let territory_emoji = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("territory"));
    let mut result = Vec::new();
    for territory in territories {
        let mut node = tree_node(
            TreeNodeKind::Category,
            &format!("{territory_emoji}{}", territory.name),
            &territory.name,
            &format!("repo://territory/{}", catalog.territory_id(territory)),
        );
        node.description = territory.description.clone();
        node.sub_kind = "territory".to_string();
        node.data = Some(data_of(vec![
            ("name", text(&territory.name)),
            ("description", text(&territory.description)),
            ("scopes", list(&territory.scopes)),
        ]));
        for statute in &territory.kinds {
            push_child(&mut node, statute_leaf_node(statute, catalog));
        }
        for child in build_territory_tree(&territory.groups, catalog) {
            push_child(&mut node, child);
        }
        result.push(node);
    }
    result
}

/// 📜️ Builds the leaf node of one statute, labelled with its priority icon.
pub fn statute_leaf_node(statute: &Statute, catalog: &dyn StatuteCatalog) -> TreeNode {
    let meta = catalog.info(statute);
    let priority = meta.as_ref().map_or(BreachPriority::Low, |meta| meta.priority);
    let mut node = tree_node(
        TreeNodeKind::Statute,
        &catalog.statute_id(statute),
        &format!("{}{}", priority_icon(priority), catalog.statute_label(statute)),
        &catalog.statute_uri(statute),
    );
    if let Some(meta) = meta {
        node.description = meta.reason.clone();
        if meta.autofixable {
            node.sub_kind = "autofixable".to_string();
        }
        node.data = Some(data_of(vec![
            ("id", text(&statute.0)),
            ("priority", text(meta.priority.as_str())),
            ("autofixable", Value::Bool(meta.autofixable)),
            ("reason", text(&meta.reason)),
            ("solution", text(&meta.solution)),
        ]));
    }
    node
}

/// 🧱️ Groups the statutes of a policy's territories by the entity kind they apply to.
pub fn build_policy_entity_kind_tree(territories: &[Territory], catalog: &dyn StatuteCatalog) -> Vec<TreeNode> {
    fn collect(entries: &[Territory], catalog: &dyn StatuteCatalog, grouped: &mut BTreeMap<String, BTreeSet<String>>) {
        for entry in entries {
            for statute in &entry.kinds {
                grouped.entry(catalog.entity_kind(statute)).or_default().insert(statute.0.clone());
            }
            collect(&entry.groups, catalog, grouped);
        }
    }
    let mut grouped: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    collect(territories, catalog, &mut grouped);
    let mut result = Vec::new();
    for (entity_kind, statutes) in grouped {
        let mut node = tree_node(
            TreeNodeKind::Category,
            &format!("entitykind:{entity_kind}"),
            &entity_kind,
            &format!("repo://entitykind/{entity_kind}"),
        );
        node.sub_kind = "entitykind".to_string();
        node.data = Some(data_of(vec![("kind", text(&entity_kind))]));
        for statute in statutes {
            push_child(&mut node, statute_leaf_node(&Statute(statute), catalog));
        }
        result.push(node);
    }
    result
}

//#endregion 📜️StatuteTree

//#region 🔀️Sorting

/// 📶️ Sorts every child list: folders before everything else, then by label, recursively.
pub fn sort_tree_children(node: &mut TreeNode) {
    if let Some(children) = node.children.as_mut() {
        children.sort_by(|left, right| {
            if left.kind != right.kind {
                let left_folder = left.kind == TreeNodeKind::Folder;
                let right_folder = right.kind == TreeNodeKind::Folder;
                if left_folder != right_folder {
                    return if left_folder { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
                }
            }
            left.label.cmp(&right.label)
        });
        for child in children.iter_mut() {
            sort_tree_children(child);
        }
    }
}

//#endregion 🔀️Sorting

//#region 🧹️Filtering

/// 🔍️ Filters a monorepo tree, keeping its structure and collapsing hidden entity levels.
pub fn filter_monorepo_tree(root: &TreeNode, filter: Option<&TreeFilter>) -> TreeNode {
    let Some(filter) = filter else { return root.clone() };
    let Some(mut filtered) = filter_node(root, filter) else {
        let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
        empty.children = Some(Vec::new());
        return empty;
    };
    if filter.has_only_kinds() || filter.exclude_kinds.as_ref().is_some_and(|kinds| !kinds.is_empty()) {
        collapse_filtered_kinds(&mut filtered, filter);
    }
    filtered
}

/// 🧹️ Keeps a node when it survives every criterion, otherwise keeps only its surviving subtree.
pub fn filter_node(node: &TreeNode, filter: &TreeFilter) -> Option<TreeNode> {
    if node.kind != TreeNodeKind::Category {
        if !filter.is_kind_visible(node.kind) {
            let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| filter_node(child, filter)).collect();
            if kept.is_empty() {
                return None;
            }
            let mut copy = node.clone();
            copy.children = Some(kept);
            return Some(copy);
        }
        if !filter.matches_sub_kind(node.kind, &node.sub_kind) {
            return None;
        }
        if node.year > 0 && !filter.matches_date(node.year, node.month, node.day) {
            return None;
        }
        if !node.status.is_empty() && !filter.matches_status(&node.status) {
            return None;
        }
        if !node.contributor.is_empty() && !filter.matches_contributor(&node.contributor) {
            return None;
        }
    }
    let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| filter_node(child, filter)).collect();
    if node.kind == TreeNodeKind::Category && kept.is_empty() {
        return None;
    }
    let mut copy = node.clone();
    copy.children = if kept.is_empty() { None } else { Some(kept) };
    Some(copy)
}

/// 🏷️ Lifts the children of every hidden entity node into its parent, depth first.
pub fn collapse_filtered_kinds(node: &mut TreeNode, filter: &TreeFilter) {
    let Some(children) = node.children.take() else { return };
    let mut result: Vec<TreeNode> = Vec::new();
    for mut child in children {
        collapse_filtered_kinds(&mut child, filter);
        if child.kind != TreeNodeKind::Category && !filter.is_kind_visible(child.kind) {
            result.extend(child.children.take().unwrap_or_default());
        } else {
            result.push(child);
        }
    }
    node.children = Some(result);
}

//#endregion 🧹️Filtering

//#region 🔎️Search

/// 🔤️ Flattens a node's data map into the searchable text the Go twin builds.
pub fn serialize_tree_node_data(data: Option<&BTreeMap<String, Value>>) -> String {
    let Some(data) = data else { return String::new() };
    if data.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = data
        .values()
        .map(|value| match value {
            Value::String(inner) => inner.clone(),
            Value::Array(items) => items
                .iter()
                .map(|item| match item {
                    Value::String(inner) => inner.clone(),
                    other => other.to_string(),
                })
                .collect::<Vec<String>>()
                .join(" "),
            Value::Bool(inner) => inner.to_string(),
            Value::Number(inner) => inner.to_string(),
            Value::Null => "null".to_string(),
            other => other.to_string(),
        })
        .collect();
    parts.join(" ")
}

/// 📄️ Builds the search document of one node, including its non-category children.
pub fn build_search_document_text(node: &TreeNode) -> String {
    let mut parts = vec![
        node.label.clone(),
        node.id.clone(),
        node.sub_kind.clone(),
        node.description.clone(),
        node.uri.clone(),
        node.status.clone(),
        node.contributor.clone(),
        node.kind.as_str().to_string(),
        serialize_tree_node_data(node.data.as_ref()),
    ];
    for child in children_of(node) {
        if child.kind == TreeNodeKind::Category {
            continue;
        }
        parts.push(child.label.clone());
        parts.push(child.id.clone());
        parts.push(child.description.clone());
        parts.push(serialize_tree_node_data(child.data.as_ref()));
    }
    parts.join(" ")
}

/// 🔶️ The Levenshtein distance between two byte strings.
pub fn levenshtein(left: &str, right: &str) -> usize {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.is_empty() {
        return right.len();
    }
    if right.is_empty() {
        return left.len();
    }
    let mut previous: Vec<usize> = (0..=right.len()).collect();
    let mut current: Vec<usize> = vec![0; right.len() + 1];
    for row in 1..=left.len() {
        current[0] = row;
        for column in 1..=right.len() {
            let cost = usize::from(left[row - 1] != right[column - 1]);
            current[column] = (current[column - 1] + 1).min(previous[column] + 1).min(previous[column - 1] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right.len()]
}

/// 🔹️ Whether a text contains a term literally or within the tolerated edit distance.
pub fn fuzzy_contains(text: &str, term: &str) -> bool {
    if text.contains(term) {
        return true;
    }
    let max_distance = if term.len() > 5 { 2 } else { 1 };
    for word in text.split_whitespace() {
        if levenshtein(word, term) <= max_distance {
            return true;
        }
        if word.len() >= term.len() {
            let mut start = 0;
            while start <= word.len() - term.len() + max_distance && start < word.len() {
                let end = (start + term.len() + max_distance).min(word.len());
                if word.is_char_boundary(start) && word.is_char_boundary(end) && levenshtein(&word[start..end], term) <= max_distance {
                    return true;
                }
                start += 1;
            }
        }
    }
    false
}

/// 🔎️ Prunes a tree to the nodes whose search document matches every query term.
pub fn search_tree_in_memory(root: &TreeNode, query: &str) -> TreeNode {
    let terms: Vec<String> = query.to_lowercase().split_whitespace().map(str::to_string).collect();
    if terms.is_empty() {
        return root.clone();
    }
    let mut matched: BTreeSet<String> = BTreeSet::new();
    collect_matches(root, &terms, &mut matched);
    if matched.is_empty() {
        let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
        empty.children = Some(Vec::new());
        return empty;
    }
    match prune_unmatched(root, &matched, false) {
        Some(pruned) => pruned,
        None => {
            let mut empty = tree_node(TreeNodeKind::Category, "", ".", "");
            empty.children = Some(Vec::new());
            empty
        }
    }
}

/// 🎯️ Records the document ids of every matching non-category node.
fn collect_matches(node: &TreeNode, terms: &[String], matched: &mut BTreeSet<String>) {
    if node.kind != TreeNodeKind::Category {
        let document = build_search_document_text(node).to_lowercase();
        if terms.iter().all(|term| fuzzy_contains(&document, term)) {
            matched.insert(document_id(node));
        }
    }
    for child in children_of(node) {
        collect_matches(child, terms, matched);
    }
}

/// 🆔️ The id a node is matched under: its id, or its label when it has none.
fn document_id(node: &TreeNode) -> String {
    if node.id.is_empty() {
        node.label.clone()
    } else {
        node.id.clone()
    }
}

/// 🎯️ Keeps every matched node, its ancestors and, once matched, its whole subtree.
fn prune_unmatched(node: &TreeNode, matched: &BTreeSet<String>, ancestor_matched: bool) -> Option<TreeNode> {
    let self_matched = matched.contains(&document_id(node));
    let keep_descendants = ancestor_matched || self_matched;
    let kept: Vec<TreeNode> = children_of(node).iter().filter_map(|child| prune_unmatched(child, matched, keep_descendants)).collect();
    if keep_descendants || !kept.is_empty() || node.kind == TreeNodeKind::Category {
        if node.kind == TreeNodeKind::Category && kept.is_empty() && !self_matched && !ancestor_matched {
            return None;
        }
        let mut copy = node.clone();
        copy.children = if kept.is_empty() { None } else { Some(kept) };
        return Some(copy);
    }
    None
}

//#endregion 🔎️Search

//#region 🎨️Rendering

/// 🎨️ Renders a monorepo tree as indented text, one line per node.
pub fn render_monorepo_tree(root: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    let children = children_of(root);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_tree_node_text(&mut out, child, "", index == last, true, renderer);
    }
    out
}

/// 📰️ Renders a monorepo tree as a markdown list.
pub fn render_monorepo_tree_markdown(root: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    for child in children_of(root) {
        render_tree_node_markdown(&mut out, child, "", renderer);
    }
    out
}

/// 🧾️ The pre-order outline of a tree: one `<depth>|<kind>|<id>|<label>` row per node, the
/// implementation-neutral projection every structural assertion compares.
pub fn tree_outline(root: &TreeNode) -> Vec<String> {
    fn walk(node: &TreeNode, depth: usize, rows: &mut Vec<String>) {
        rows.push(format!("{depth}|{}|{}|{}", node.kind.as_str(), node.id, node.label));
        for child in children_of(node) {
            walk(child, depth + 1, rows);
        }
    }
    let mut rows = Vec::new();
    walk(root, 0, &mut rows);
    rows
}

/// 🔺️ Renders one node and its subtree as text.
fn render_tree_node_text(out: &mut String, node: &TreeNode, prefix: &str, is_last: bool, is_root: bool, renderer: &dyn EntityRenderer) {
    let connector = if is_root {
        ""
    } else if is_last {
        TEXT_LAST_CONNECTOR
    } else {
        TEXT_BRANCH_CONNECTOR
    };
    let label = tree_node_text_label(node, renderer);
    out.push_str(prefix);
    out.push_str(connector);
    out.push_str(&label);
    out.push('\n');
    let new_prefix = if is_root {
        prefix.to_string()
    } else if is_last {
        format!("{prefix}{TEXT_BLANK_INDENT}")
    } else {
        format!("{prefix}{TEXT_PIPE_INDENT}")
    };
    let children = children_of(node);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_tree_node_text(out, child, &new_prefix, index == last, false, renderer);
    }
}

/// 🏷️ The text label of one node: a link for a category, the entity line for everything else.
pub fn tree_node_text_label(node: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    if node.kind == TreeNodeKind::Category {
        if node.uri.is_empty() {
            return node.label.clone();
        }
        return format!("[{}]({})", node.label, node.uri);
    }
    let Some(data) = node.data.as_ref() else { return node.label.clone() };
    let entity_kind = entity_kind_of(node);
    let mut blanked = data.clone();
    blanked.insert("parentId".to_string(), text(""));
    let rendered = renderer.human(&entity_kind, &blanked);
    if rendered.is_empty() {
        node.label.clone()
    } else {
        rendered
    }
}

/// 🔻️ Renders one node and its subtree as markdown.
fn render_tree_node_markdown(out: &mut String, node: &TreeNode, indent: &str, renderer: &dyn EntityRenderer) {
    let line = match (node.kind, node.data.as_ref()) {
        (TreeNodeKind::Category, _) | (_, None) => {
            if node.uri.is_empty() {
                format!("{indent}- {}", node.label)
            } else {
                format!("{indent}- [{}]({})", node.label, node.uri)
            }
        }
        (_, Some(data)) => format!("{indent}{}", renderer.markdown(&entity_kind_of(node), data)),
    };
    out.push_str(&line);
    out.push('\n');
    let child_indent = format!("{indent}  ");
    for child in children_of(node) {
        render_tree_node_markdown(out, child, &child_indent, renderer);
    }
}

/// 🧱️ The entity kind a node renders as, falling back to its own kind slug.
fn entity_kind_of(node: &TreeNode) -> String {
    let entity_kind = tree_node_kind_to_entity_kind(node.kind);
    if entity_kind.is_empty() {
        node.kind.as_str().to_string()
    } else {
        entity_kind.to_string()
    }
}

//#endregion 🎨️Rendering

//#region 🧜️Mermaid

/// 🧜️ One node of a Mermaid `treemap-beta` diagram: a labelled group, or a labelled leaf value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MermaidNode {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MermaidNode>,
}

/// 🧜️ A Mermaid `treemap-beta` diagram: a title and its node forest.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MermaidTreemap {
    pub title: String,
    #[serde(default)]
    pub nodes: Vec<MermaidNode>,
}

impl MermaidTreemap {
    /// 📥️ Decodes a treemap from its JSON document.
    pub fn from_json(document: &str) -> Result<MermaidTreemap, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }
}

/// 🔤️ Escapes a Mermaid label: a double quote would close the quoted label, so it becomes an
/// apostrophe, exactly as the Go twin does.
pub fn mermaid_escape_label(value: &str) -> String {
    value.replace('"', "'")
}

/// 🧜️ Renders a `treemap-beta` diagram: the header, the quoted title, then four spaces of indent
/// per level, a quoted label and, for a leaf, `: <value>`.
pub fn render_mermaid_treemap(treemap: &MermaidTreemap) -> String {
    let mut out = String::from("treemap-beta\n");
    out.push_str(&format!("\"{}\"\n", mermaid_escape_label(&treemap.title)));
    for node in &treemap.nodes {
        render_mermaid_node(&mut out, node, 1);
    }
    out
}

/// 🧜️ Renders one treemap node at a depth.
fn render_mermaid_node(out: &mut String, node: &MermaidNode, depth: usize) {
    let indent = TEXT_BLANK_INDENT.repeat(depth);
    match node.value {
        Some(value) if node.children.is_empty() => out.push_str(&format!("{indent}\"{}\": {value}\n", mermaid_escape_label(&node.label))),
        _ => out.push_str(&format!("{indent}\"{}\"\n", mermaid_escape_label(&node.label))),
    }
    for child in &node.children {
        render_mermaid_node(out, child, depth + 1);
    }
}

/// 🌳️ Projects a monorepo tree into a treemap: every node becomes a group, and a node carrying a
/// numeric `value` in its data becomes a leaf of that weight.
pub fn mermaid_treemap_from_tree(root: &TreeNode, title: &str, weight_key: &str) -> MermaidTreemap {
    MermaidTreemap {
        title: title.to_string(),
        nodes: children_of(root).iter().map(|child| mermaid_node_from_tree(child, weight_key)).collect(),
    }
}

/// 🌿️ Projects one tree node into a treemap node.
fn mermaid_node_from_tree(node: &TreeNode, weight_key: &str) -> MermaidNode {
    MermaidNode {
        label: node.label.clone(),
        value: node.data.as_ref().and_then(|data| data.get(weight_key)).and_then(Value::as_i64),
        children: children_of(node).iter().map(|child| mermaid_node_from_tree(child, weight_key)).collect(),
    }
}

//#endregion 🧜️Mermaid

//#region 🔢️MermaidLoc
// 🔢️ The three LOC treemaps the `mermaid` verb renders. Every one is a pure function over already
// gathered rows, so the walk, the line counting and the `git blame` stay outside this crate.

/// 👤️ The emoji a technology kind carries in a LOC treemap, raw and without a presentation
/// selector — the Go twin writes the constant, not its text form.
fn mermaid_technology_emoji(kind: TechnologyKind) -> &'static str {
    match kind {
        TechnologyKind::Infrastructure => "🧰️",
        TechnologyKind::Research => "🔬️",
        _ => "👤️",
    }
}

/// 📦️ The emoji a bundle kind carries in a LOC treemap.
fn mermaid_bundle_emoji(kind: BundleKind) -> &'static str {
    match kind {
        BundleKind::Schema => "🛂️",
        BundleKind::Binary => "⌨️",
        BundleKind::Ui => "🖱️",
        BundleKind::Site => "🌐️",
        BundleKind::Assets => "🏪️",
        _ => "📚️",
    }
}

/// 📄️ The emoji a file kind carries in a LOC treemap.
fn mermaid_file_emoji(kind: FileKind) -> &'static str {
    match kind {
        FileKind::Lab => "🥼️",
        FileKind::Script => "📜️",
        FileKind::Docs => "📃️",
        FileKind::Config => "⚙️",
        FileKind::Resource => "💾️",
        FileKind::Template => "📋️",
        FileKind::License => "⚖️",
        _ => "💻️",
    }
}

/// 🗃️ The emoji an organization folder carries in a LOC treemap.
const MERMAID_FOLDER_EMOJI: &str = "🗃️";

/// 📄️ One counted file, already attributed to the bundle that owns it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MermaidLocFile {
    /// 📦️ `technology/bundle` name of the owning bundle.
    pub bundle_name: String,
    /// 📁️ Bundle-relative folder, empty for a file at the bundle root.
    pub folder: String,
    /// 🏷️ The file's own name, as the treemap labels it.
    pub name: String,
    /// 🏷️ Its kind, which decides the emoji.
    pub kind: FileKind,
    /// 🔢️ Its physical line count; a file counted as zero is dropped by the caller.
    pub loc: i64,
}

/// 🧜️ The `loc-by-technologies-bundles-folders-files` treemap.
///
/// Twin of `MermaidLocByTechnologiesBundlesFoldersFiles` in `📦️packages/🐹️go/🐹️.go`: technologies,
/// bundles and folders in name order, an empty branch dropped, a bundle-root file promoted to the
/// folder level, and the bundle label being the segment after the technology.
pub fn mermaid_loc_by_technologies_bundles_folders_files(technologies: &[Technology], files: &[MermaidLocFile]) -> String {
    let mut rows: BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<MermaidLocFile>>>> = BTreeMap::new();
    let mut technology_of: BTreeMap<String, (String, TechnologyKind)> = BTreeMap::new();
    let mut bundle_kind: BTreeMap<String, BundleKind> = BTreeMap::new();
    for technology in technologies {
        rows.entry(technology.name.clone()).or_default();
        technology_of.insert(technology.name.clone(), (technology.name.clone(), technology.kind));
        for bundle in technology.bundles.as_deref().unwrap_or_default() {
            rows.entry(technology.name.clone()).or_default().entry(bundle.name.clone()).or_default();
            bundle_kind.insert(bundle.name.clone(), bundle.kind);
        }
    }
    for file in files {
        let technology_name = file.bundle_name.split('/').next().unwrap_or_default().to_string();
        let Some(bundles) = rows.get_mut(&technology_name) else { continue };
        let Some(folders) = bundles.get_mut(&file.bundle_name) else { continue };
        folders.entry(file.folder.clone()).or_default().push(file.clone());
    }
    let mut out = String::from("treemap-beta\n\"Lines of Code\"\n");
    for (technology_name, bundles) in &rows {
        let technology_loc: i64 = bundles.values().flat_map(|folders| folders.values()).flatten().map(|file| file.loc).sum();
        if technology_loc == 0 {
            continue;
        }
        let kind = technology_of.get(technology_name).map_or(TechnologyKind::User, |entry| entry.1);
        out.push_str(&format!("    \"{}{}\"\n", mermaid_technology_emoji(kind), mermaid_escape_label(&semio_framework_repo_identity::flat(technology_name))));
        for (bundle_name, folders) in bundles {
            let bundle_loc: i64 = folders.values().flatten().map(|file| file.loc).sum();
            if bundle_loc == 0 {
                continue;
            }
            let label = bundle_name.split_once('/').map_or_else(|| bundle_name.clone(), |(_, tail)| tail.to_string());
            let kind = bundle_kind.get(bundle_name).copied().unwrap_or(BundleKind::Library);
            out.push_str(&format!("        \"{}{}\"\n", mermaid_bundle_emoji(kind), mermaid_escape_label(&semio_framework_repo_identity::flat(&label))));
            for (folder_name, folder_files) in folders {
                let folder_loc: i64 = folder_files.iter().map(|file| file.loc).sum();
                if folder_loc == 0 {
                    continue;
                }
                if folder_name.is_empty() {
                    for file in folder_files {
                        out.push_str(&format!("            \"{}{}\": {}\n", mermaid_file_emoji(file.kind), mermaid_escape_label(&file.name), file.loc));
                    }
                    continue;
                }
                let leaf = folder_name.rsplit('/').next().unwrap_or(folder_name);
                out.push_str(&format!("            \"{MERMAID_FOLDER_EMOJI}{}\"\n", mermaid_escape_label(leaf)));
                let mut sorted = folder_files.clone();
                sorted.sort_by(|left, right| left.name.cmp(&right.name));
                for file in sorted {
                    out.push_str(&format!("                \"{}{}\": {}\n", mermaid_file_emoji(file.kind), mermaid_escape_label(&file.name), file.loc));
                }
            }
        }
    }
    out
}

/// 🧜️ A flat `label: value` treemap under a title, ordered by descending value then by label.
///
/// Twin of the tail of `MermaidLocByContributors` and `MermaidLocByLanguage`, which differ only in
/// their title and in how their rows were gathered.
pub fn mermaid_loc_flat(title: &str, rows: &BTreeMap<String, i64>) -> String {
    let mut ordered: Vec<(&String, &i64)> = rows.iter().collect();
    ordered.sort_by(|left, right| right.1.cmp(left.1).then_with(|| left.0.cmp(right.0)));
    let mut out = format!("treemap-beta\n\"{title}\"\n");
    for (label, value) in ordered {
        out.push_str(&format!("    \"{}\": {value}\n", mermaid_escape_label(label)));
    }
    out
}

/// 🧜️ The `loc-by-contributors` treemap title.
pub const MERMAID_TITLE_BY_CONTRIBUTORS: &str = "Lines of Code by Contributor";

/// 🧜️ The `loc-by-language` treemap title.
pub const MERMAID_TITLE_BY_LANGUAGE: &str = "Lines of Code by Language";

//#endregion 🔢️MermaidLoc

//#region 📌️Cache

/// 💿️ The metadata a cached tree carries; a build is reused only when all of it still matches.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeCacheMeta {
    #[serde(rename = "SchemaVersion")]
    pub schema_version: i64,
    #[serde(rename = "Fingerprint")]
    pub fingerprint: String,
    #[serde(rename = "IncludeSections")]
    pub include_sections: bool,
    #[serde(rename = "ContentDigest", default, skip_serializing_if = "String::is_empty")]
    pub content_digest: String,
}

impl TreeCacheMeta {
    /// 🆕️ Builds the metadata of a freshly computed tree.
    pub fn of(tree: &TreeNode, fingerprint: &str, include_sections: bool) -> TreeCacheMeta {
        TreeCacheMeta {
            schema_version: TREE_CACHE_SCHEMA_VERSION,
            fingerprint: fingerprint.to_string(),
            include_sections,
            content_digest: tree_content_digest(tree),
        }
    }
}

/// ♻️ Whether a cached tree may be reused for a fingerprint and section choice.
pub fn tree_cache_is_valid(meta: &TreeCacheMeta, fingerprint: &str, include_sections: bool) -> bool {
    meta.schema_version == TREE_CACHE_SCHEMA_VERSION && meta.fingerprint == fingerprint && meta.include_sections == include_sections
}

/// 🔏️ The content digest of a tree: the SHA-256 of its canonical JSON encoding.
pub fn tree_content_digest(tree: &TreeNode) -> String {
    sha256_hex(canonical_tree_json(tree).as_bytes())
}

/// 🔣️ The canonical JSON encoding of a tree, with every map key in sorted order.
pub fn canonical_tree_json(tree: &TreeNode) -> String {
    serde_json::to_string(tree).unwrap_or_default()
}

/// 🗜️ The gzip-wrapped canonical JSON of a tree: the payload half of the cache envelope, the
/// same shape `github.com/usalu/semio/repo/tree` writes to `tree.json.gz`.
pub fn encode_tree_cache(tree: &TreeNode) -> Vec<u8> {
    gzip_encode(canonical_tree_json(tree).as_bytes())
}

/// 🗜️ The tree read back out of a gzip-wrapped cache payload.
pub fn decode_tree_cache(payload: &[u8]) -> Result<TreeNode, String> {
    let plain = gzip_decode(payload)?;
    let text = String::from_utf8(plain).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

//#endregion 📌️Cache

//#region 🔣️Encoding

/// 🔣️ Encodes any tree value as JSON, the shape both implementations project through.
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}

//#endregion 🔣️Encoding

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(kind: TreeNodeKind, id: &str, label: &str) -> TreeNode {
        tree_node(kind, id, label, "")
    }

    #[test]
    fn sorts_folders_before_files_then_by_label() {
        let mut root = tree_node(TreeNodeKind::Category, "", "root", "");
        root.children = Some(vec![
            leaf(TreeNodeKind::File, "", "z.ts"),
            leaf(TreeNodeKind::File, "", "a.ts"),
            leaf(TreeNodeKind::Folder, "", "src"),
        ]);
        sort_tree_children(&mut root);
        let labels: Vec<String> = children_of(&root).iter().map(|node| node.label.clone()).collect();
        assert_eq!(labels, vec!["src", "a.ts", "z.ts"]);
    }

    #[test]
    fn digest_is_stable_and_content_addressed() {
        let mut left = tree_node(TreeNodeKind::Category, "", "root", "");
        left.children = Some(vec![leaf(TreeNodeKind::File, "a", "a.ts")]);
        let mut right = left.clone();
        assert_eq!(tree_content_digest(&left), tree_content_digest(&right));
        right.children = Some(vec![leaf(TreeNodeKind::File, "b", "b.ts")]);
        assert_ne!(tree_content_digest(&left), tree_content_digest(&right));
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn mermaid_treemap_indents_by_four_spaces_per_level() {
        let treemap = MermaidTreemap {
            title: "Lines of Code".to_string(),
            nodes: vec![MermaidNode {
                label: "🛠️repo".to_string(),
                value: None,
                children: vec![MermaidNode { label: "\"cli\"".to_string(), value: Some(12), children: Vec::new() }],
            }],
        };
        assert_eq!(render_mermaid_treemap(&treemap), "treemap-beta\n\"Lines of Code\"\n    \"🛠️repo\"\n        \"'cli'\": 12\n");
    }

    #[test]
    fn cache_envelope_round_trips_through_the_shared_gzip_codec() {
        let mut tree = leaf(TreeNodeKind::Category, "root", "Root");
        tree.children = Some(vec![leaf(TreeNodeKind::Folder, "f", "folder"), leaf(TreeNodeKind::File, "a", "a.ts")]);
        let payload = encode_tree_cache(&tree);
        assert_eq!(&payload[..3], &[0x1f, 0x8b, 0x08]);
        assert_eq!(decode_tree_cache(&payload).unwrap(), tree);
        assert!(decode_tree_cache(b"not gzip at all!!!!").is_err());
    }
}

//#endregion 🧪️Tests
