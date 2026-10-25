//! 📙️ The ticket, goal, draft, todo and statute aggregates.

use super::*;

//#region 🎫️Ticket Aggregate

/// 🧷️ A client-native plan or spec file archived into the ticket folder on close.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketPlan {
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(rename = "local", default, skip_serializing_if = "String::is_empty")]
    pub local: String,
}

/// 🎫️ One step of an agent's plan on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketAgentPlanStep {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "status", default, skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(rename = "ideated", default, skip_serializing_if = "String::is_empty")]
    pub ideated: String,
    #[serde(rename = "started", default, skip_serializing_if = "String::is_empty")]
    pub started: String,
    #[serde(rename = "completed", default, skip_serializing_if = "String::is_empty")]
    pub completed: String,
    #[serde(rename = "abandoned", default, skip_serializing_if = "String::is_empty")]
    pub abandoned: String,
}

/// 💠️ The plan an agent declared on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketAgentPlan {
    #[serde(rename = "steps", default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<TicketAgentPlanStep>,
}

/// 🔳️ One agent session recorded on a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketAgent {
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "contributor", default, skip_serializing_if = "String::is_empty")]
    pub contributor: String,
    #[serde(rename = "system", default, skip_serializing_if = "String::is_empty")]
    pub system: String,
    #[serde(rename = "client", default, skip_serializing_if = "String::is_empty")]
    pub client: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "transcript", default, skip_serializing_if = "String::is_empty")]
    pub transcript: String,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketAgentPlan>,
}

/// 🐙️ The management-provider link of a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketManagementData {
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
}

/// 🔖️ A ticket: one unit of work with a lifecycle and a contribution record.
///
/// The Go twin carries a hand-written `UnmarshalJSON` that additionally rewrites goal and
/// contributor compose-ids and absorbs pre-schema field spellings. Both belong to `🪪️identity`
/// and `🎫️tickets`; this crate is the pure shape.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ticket {
    #[serde(skip)]
    pub year: i64,
    #[serde(skip)]
    pub month: i64,
    #[serde(skip)]
    pub day: i64,
    #[serde(skip)]
    pub slug: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "emoji", default, skip_serializing_if = "String::is_empty")]
    pub emoji: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<TicketManagementData>,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "plan", default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<TicketPlan>,
    #[serde(rename = "sessions", default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<String>,
    #[serde(rename = "interactions", default, skip_serializing_if = "Vec::is_empty")]
    pub interactions: Vec<Interaction>,
    #[serde(rename = "agents", default, skip_serializing_if = "Vec::is_empty")]
    pub agents: Vec<TicketAgent>,
    #[serde(skip)]
    pub folder_path: String,
    #[serde(skip)]
    pub json_path: String,
    #[serde(skip)]
    pub important_path: String,
}

/// ◼ The persisted subset of a ticket, without any derived field.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketData {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<TicketManagementData>,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
}

/// 🟪️ Creation and completion timestamps of a ticket.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketDate {
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "finished", default, skip_serializing_if = "Option::is_none")]
    pub finished: Option<String>,
}

/// ◾ A file a ticket touched, with its line metrics.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFile {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// ◽ A file a ticket renamed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileRenamed {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 🗃️ The four change classes of one entity level.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDiffSet {
    #[serde(rename = "deleted")]
    pub deleted: Option<Vec<TicketFile>>,
    #[serde(rename = "renamed")]
    pub renamed: Option<Vec<TicketFileRenamed>>,
    #[serde(rename = "modified")]
    pub modified: Option<Vec<TicketFile>>,
    #[serde(rename = "added")]
    pub added: Option<Vec<TicketFile>>,
}

/// ◻ The semantic diff of a ticket across every entity level.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDiffs {
    #[serde(rename = "bundles", default)]
    pub bundles: TicketDiffSet,
    #[serde(rename = "folders", default)]
    pub folders: TicketDiffSet,
    #[serde(rename = "files", default)]
    pub files: TicketDiffSet,
    #[serde(rename = "sections", default)]
    pub sections: TicketDiffSet,
    #[serde(rename = "definitions", default)]
    pub definitions: TicketDiffSet,
}

/// 📑️ The metrics a ticket accumulated inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketSectionMetrics {
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// ▫️ A named section a ticket touched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSection {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 📍️ The metrics a ticket accumulated inside one file, keyed by section name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileMetricsEntry {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
    #[serde(rename = "sections", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sections: BTreeMap<String, TicketSectionMetrics>,
}

/// 📏️ Section metrics of one file, keyed by section name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketFileMetrics {
    #[serde(rename = "sections")]
    pub sections: Option<BTreeMap<String, TicketSectionMetrics>>,
}

/// 📦️ File metrics of one bundle, keyed by file path.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketBundleMetrics {
    #[serde(rename = "files")]
    pub files: Option<BTreeMap<String, TicketFileMetrics>>,
}

/// ⬛️ Bundle metrics of a ticket, keyed by bundle id.
pub type TicketBundles = BTreeMap<String, TicketBundleMetrics>;

/// 📊️ The definitions and metrics a ticket touched inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSectionContrib {
    #[serde(rename = "sectionId")]
    pub section_id: String,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<String>>,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🗂️ The sections a ticket touched inside one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileContrib {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<TicketSectionContrib>>,
}

/// 🎁️ The files a ticket touched inside one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketBundleContrib {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "files")]
    pub files: Option<Vec<TicketFileContrib>>,
}

//#endregion 🎫️Ticket Aggregate

//#region 🎯️Goal Aggregate

/// 🟠️ The dates a goal declares.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalDates {
    #[serde(rename = "due", default, skip_serializing_if = "String::is_empty")]
    pub due: String,
}

/// 🟡️ The management-provider link of a goal.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalManagementData {
    #[serde(rename = "milestone", default, skip_serializing_if = "String::is_empty")]
    pub milestone: String,
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
}

/// 🔵️ A goal: the long-lived intent that tickets are opened against.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Goal {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "status")]
    pub status: GoalStatus,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "dueDate", default, skip_serializing_if = "String::is_empty")]
    pub due_date: String,
    #[serde(rename = "dates", default)]
    pub dates: GoalDates,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub management: Option<GoalManagementData>,
    #[serde(skip)]
    pub id: String,
    #[serde(skip)]
    pub path: String,
}

//#endregion 🎯️Goal Aggregate

//#region 📝️Draft Aggregate

/// 💿️ A draft: a named set of files staged before a ticket exists.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Draft {
    #[serde(rename = "id")]
    pub id: String,
}

//#endregion 📝️Draft Aggregate

//#region ✅️Todo Aggregate

/// 🔷️ A position inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Location {
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "line")]
    pub line: i64,
    #[serde(rename = "column")]
    pub column: i64,
}

/// ✅️ A todo recorded against an artifact.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Todo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "location", default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

//#endregion ✅️Todo Aggregate

//#region 👮️Statute Aggregate

/// 🟢️ The slash-separated identifier of a statute, for example `code/section/empty`. The
/// closed vocabulary and its metadata belong to `📜️statutes`; the model only carries the id
/// so that every artifact type can reference a statute without depending on that module.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Statute(pub String);

impl fmt::Display for Statute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for Statute {
    fn from(value: &str) -> Self {
        Statute(value.to_string())
    }
}

/// 🟣️ A named grouping of statutes and nested territories.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Territory {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "scopes", default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "groups", default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Territory>,
    #[serde(rename = "kinds", default, skip_serializing_if = "Vec::is_empty")]
    pub kinds: Vec<Statute>,
}

impl Territory {
    /// 🟤️ Every statute of this territory and of every nested territory, depth first.
    pub fn all_kinds(&self) -> Vec<Statute> {
        let mut result = self.kinds.clone();
        for child in &self.groups {
            result.extend(child.all_kinds());
        }
        result
    }
}

/// 📜️ The declared metadata of one statute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StatuteMeta {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "solution")]
    pub solution: String,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
}

/// 🔏️ A policy: a scoped set of territories and statutes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Policy {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "groups")]
    pub groups: Option<Vec<Territory>>,
    #[serde(rename = "statutes")]
    pub statutes: Option<Vec<StatuteMeta>>,
}

/// 🔶️ One detected violation of a statute.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Breach {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "line", default, skip_serializing_if = "is_zero")]
    pub line: i64,
    #[serde(rename = "column", default, skip_serializing_if = "is_zero")]
    pub column: i64,
    #[serde(rename = "excerpt", default, skip_serializing_if = "String::is_empty")]
    pub excerpt: String,
    #[serde(rename = "priority", default, skip_serializing_if = "Option::is_none")]
    pub lint_priority: Option<BreachPriority>,
    #[serde(rename = "autofixable", default, skip_serializing_if = "Option::is_none")]
    pub lint_autofixable: Option<bool>,
    #[serde(rename = "reason", default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
    #[serde(rename = "solution", default, skip_serializing_if = "String::is_empty")]
    pub solution: String,
}

/// ▶️ The outcome of an analyze run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnalyzeResult {
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<Breach>>,
    #[serde(rename = "metrics")]
    pub metrics: Option<AnalyzeMetrics>,
}

/// 🔧️ The outcome of a fix run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FixResult {
    #[serde(rename = "fixed")]
    pub fixed: i64,
    #[serde(rename = "remaining")]
    pub remaining: i64,
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<Breach>>,
}

/// 🔂️ A single semantic change to one entity between two checkpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SemanticChange {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "Status")]
    pub status: SemanticChangeType,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "FromPath")]
    pub from_path: String,
    #[serde(rename = "ToPath")]
    pub to_path: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
}

//#endregion 👮️Statute Aggregate
