//! 🎟️ The tree aggregate and the input types of the ticket, goal, todo and contributor commands.

use super::*;

//#region 🌳️Tree Aggregate

/// 🏷️ Every entity kind the monorepo tree can address.
pub const ENTITY_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "minute", "second", "technology", "bundle", "folder",
    "file", "line", "range", "section", "definition", "goal", "ticket", "draft", "todo", "policy",
    "breach", "contributor", "checkpoint", "interaction", "session",
];

/// 🎁️ The entity kinds that are artifacts, in containment order.
pub const ARTIFACT_KINDS: &[&str] =
    &["repo", "technology", "bundle", "folder", "file", "section", "definition"];

/// 💿️ The entity kinds a checkpoint diff can report on.
pub const DIFFABLE_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "technology", "bundle", "folder", "file", "section",
    "definition", "goal", "ticket", "contributor", "checkpoint", "interaction", "session",
];

/// 📄️ The entity kinds that can be related to a file.
pub const RELATED_TO_FILE_KINDS: &[&str] = &[
    "root", "year", "month", "day", "hour", "minute", "second", "technology", "bundle", "folder",
    "goal", "ticket", "draft", "todo", "policy", "breach", "contributor", "checkpoint",
    "interaction", "session",
];

/// 🌳️ The entity level of a monorepo tree node.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreeNodeKind {
    #[serde(rename = "technology")]
    Technology,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "goal")]
    Goal,
    #[serde(rename = "ticket")]
    Ticket,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "breach")]
    Breach,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "checkpoint")]
    Checkpoint,
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "statute")]
    Statute,
    #[serde(rename = "category")]
    Category,
}

impl TreeNodeKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            TreeNodeKind::Technology => "technology",
            TreeNodeKind::Bundle => "bundle",
            TreeNodeKind::Folder => "folder",
            TreeNodeKind::File => "file",
            TreeNodeKind::Section => "section",
            TreeNodeKind::Definition => "definition",
            TreeNodeKind::Goal => "goal",
            TreeNodeKind::Ticket => "ticket",
            TreeNodeKind::Draft => "draft",
            TreeNodeKind::Todo => "todo",
            TreeNodeKind::Policy => "policy",
            TreeNodeKind::Breach => "breach",
            TreeNodeKind::Contributor => "contributor",
            TreeNodeKind::Checkpoint => "checkpoint",
            TreeNodeKind::Session => "session",
            TreeNodeKind::Statute => "statute",
            TreeNodeKind::Category => "category",
        }
    }
}

impl fmt::Display for TreeNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🌿️ One node of the monorepo tree. The Go twin declares no field tags, so the wire keys are
/// the Go field names themselves.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TreeNode {
    #[serde(rename = "Kind")]
    pub kind: TreeNodeKind,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "SubKind")]
    pub sub_kind: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Summary")]
    pub summary: String,
    #[serde(rename = "Year")]
    pub year: i64,
    #[serde(rename = "Month")]
    pub month: i64,
    #[serde(rename = "Day")]
    pub day: i64,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Contributor")]
    pub contributor: String,
    #[serde(rename = "Data")]
    pub data: Option<BTreeMap<String, Value>>,
    #[serde(rename = "Children")]
    pub children: Option<Vec<TreeNode>>,
}

/// 🧹️ The criteria a monorepo tree query filters by.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TreeFilter {
    #[serde(rename = "Query")]
    pub query: String,
    #[serde(rename = "OnlyKinds")]
    pub only_kinds: Option<BTreeMap<TreeNodeKind, bool>>,
    #[serde(rename = "ExcludeKinds")]
    pub exclude_kinds: Option<BTreeMap<TreeNodeKind, bool>>,
    #[serde(rename = "OnlySubKinds")]
    pub only_sub_kinds: Option<BTreeMap<TreeNodeKind, Vec<String>>>,
    #[serde(rename = "ExcludeSubKinds")]
    pub exclude_sub_kinds: Option<BTreeMap<TreeNodeKind, Vec<String>>>,
    #[serde(rename = "OnlyYears")]
    pub only_years: Option<Vec<i64>>,
    #[serde(rename = "ExcludeYears")]
    pub exclude_years: Option<Vec<i64>>,
    #[serde(rename = "OnlyMonths")]
    pub only_months: Option<Vec<i64>>,
    #[serde(rename = "ExcludeMonths")]
    pub exclude_months: Option<Vec<i64>>,
    #[serde(rename = "OnlyDays")]
    pub only_days: Option<Vec<i64>>,
    #[serde(rename = "ExcludeDays")]
    pub exclude_days: Option<Vec<i64>>,
    #[serde(rename = "OnlyStatus")]
    pub only_status: String,
    #[serde(rename = "OnlyContributors")]
    pub only_contributors: Option<Vec<String>>,
    #[serde(rename = "ExcludeContributors")]
    pub exclude_contributors: Option<Vec<String>>,
    #[serde(rename = "OnlyPolicies")]
    pub only_policies: Option<Vec<String>>,
    #[serde(rename = "ExcludePolicies")]
    pub exclude_policies: Option<Vec<String>>,
}

impl TreeFilter {
    /// 👁️ Whether the filter restricts the visible kinds to an explicit allow list.
    pub fn has_only_kinds(&self) -> bool {
        self.only_kinds.as_ref().is_some_and(|kinds| !kinds.is_empty())
    }

    /// 🔷️ Whether a kind survives the filter. A category node is structural and always shown.
    pub fn is_kind_visible(&self, kind: TreeNodeKind) -> bool {
        if kind == TreeNodeKind::Category {
            return true;
        }
        if self.has_only_kinds() {
            return self.only_kinds.as_ref().is_some_and(|kinds| *kinds.get(&kind).unwrap_or(&false));
        }
        !self.exclude_kinds.as_ref().is_some_and(|kinds| *kinds.get(&kind).unwrap_or(&false))
    }

    /// 📥️ Whether a sub kind survives the filter. A blank sub kind always survives.
    pub fn matches_sub_kind(&self, kind: TreeNodeKind, sub_kind: &str) -> bool {
        if sub_kind.is_empty() {
            return true;
        }
        if let Some(only) = self.only_sub_kinds.as_ref().and_then(|map| map.get(&kind)) {
            if !only.is_empty() {
                return only.iter().any(|candidate| eq_fold(candidate, sub_kind));
            }
        }
        if let Some(exclude) = self.exclude_sub_kinds.as_ref().and_then(|map| map.get(&kind)) {
            if exclude.iter().any(|candidate| eq_fold(candidate, sub_kind)) {
                return false;
            }
        }
        true
    }

    /// 🎯️ Whether a year/month/day triple survives the date filters. A zero month or day is
    /// only tested against the exclusion lists.
    pub fn matches_date(&self, year: i64, month: i64, day: i64) -> bool {
        fn survives(only: Option<&Vec<i64>>, exclude: Option<&Vec<i64>>, value: i64, test_only: bool) -> bool {
            if test_only {
                if let Some(only) = only {
                    if !only.is_empty() && !only.contains(&value) {
                        return false;
                    }
                }
            }
            !exclude.is_some_and(|exclude| exclude.contains(&value))
        }
        survives(self.only_years.as_ref(), self.exclude_years.as_ref(), year, true)
            && survives(self.only_months.as_ref(), self.exclude_months.as_ref(), month, month > 0)
            && survives(self.only_days.as_ref(), self.exclude_days.as_ref(), day, day > 0)
    }

    /// 🔶️ Whether a status survives the filter. A blank filter status accepts everything.
    pub fn matches_status(&self, status: &str) -> bool {
        if self.only_status.is_empty() {
            return true;
        }
        eq_fold(&self.only_status, status)
    }

    /// 🤝️ Whether a contributor survives the filter.
    pub fn matches_contributor(&self, contributor: &str) -> bool {
        if let Some(only) = self.only_contributors.as_ref() {
            if !only.is_empty() {
                return only.iter().any(|candidate| eq_fold(candidate, contributor));
            }
        }
        !self
            .exclude_contributors
            .as_ref()
            .is_some_and(|exclude| exclude.iter().any(|candidate| eq_fold(candidate, contributor)))
    }
}

/// 🎫️ A ticket node of the goal/ticket tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Slug")]
    pub slug: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "GoalID")]
    pub goal_id: String,
    #[serde(rename = "ParentID")]
    pub parent_id: String,
    #[serde(rename = "Children")]
    pub children: Option<Vec<TicketNode>>,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "Finished")]
    pub finished: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Summary")]
    pub summary: String,
}

/// ⛳️ A goal node of the goal/ticket tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GoalNode {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "DueDate")]
    pub due_date: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Children")]
    pub children: Option<Vec<GoalNode>>,
    #[serde(rename = "Tickets")]
    pub tickets: Option<Vec<TicketNode>>,
}

//#endregion 🌳️Tree Aggregate

//#region 📥️Input Types

/// 📋️ The updated/created/removed file lists a mutation reports.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileListInput {
    #[serde(rename = "updated", default, skip_serializing_if = "Vec::is_empty")]
    pub updated: Vec<String>,
    #[serde(rename = "created", default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<String>,
    #[serde(rename = "removed", default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
}

/// 📂️ The arguments of a ticket-open mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketOpenInput {
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "noIssue", default, skip_serializing_if = "is_false")]
    pub no_issue: bool,
    #[serde(rename = "draft", default, skip_serializing_if = "String::is_empty")]
    pub draft: String,
    #[serde(rename = "goal")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "issue", default, skip_serializing_if = "String::is_empty")]
    pub issue: String,
    #[serde(rename = "planId", default, skip_serializing_if = "String::is_empty")]
    pub plan_id: String,
    #[serde(rename = "specId", default, skip_serializing_if = "String::is_empty")]
    pub spec_id: String,
}

/// 🆕️ The arguments of a draft-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DraftCreateInput {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
}

/// 📝️ The arguments of a goal-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalCreateInput {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "dueDate")]
    pub due_date: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "milestone", default, skip_serializing_if = "String::is_empty")]
    pub milestone: String,
}

/// ♻️ The arguments of a goal-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalChangeInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "llm", default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(rename = "effort", default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 📪️ The arguments of a goal-close mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalCloseInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🔓️ The arguments of a goal-reopen mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalReopenInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "llm")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🗑️ The arguments of a goal-delete mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GoalDeleteInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// ✂️ The arguments of a ticket-delete mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDeleteInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 🔒️ The arguments of a ticket-close mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketCloseInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "all", default, skip_serializing_if = "is_false")]
    pub all: bool,
}

/// 📬️ The arguments of a ticket-reopen mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketReopenInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "draft", default, skip_serializing_if = "String::is_empty")]
    pub draft: String,
    #[serde(rename = "goal", default, skip_serializing_if = "String::is_empty")]
    pub goal: String,
    #[serde(rename = "parent", default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
    #[serde(rename = "planId", default, skip_serializing_if = "String::is_empty")]
    pub plan_id: String,
    #[serde(rename = "specId", default, skip_serializing_if = "String::is_empty")]
    pub spec_id: String,
}

/// 🔹️ The arguments of a ticket-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketChangeInput {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "prompt", default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(rename = "llm", default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(rename = "effort", default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(rename = "client", default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(rename = "goal", default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(rename = "parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "noManagement", default, skip_serializing_if = "is_false")]
    pub no_management: bool,
}

/// 👥️ The arguments of a contributor-add mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorAddInput {
    #[serde(rename = "github")]
    pub github: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "names", default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "fingerprint", default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "fingerprints", default, skip_serializing_if = "Vec::is_empty")]
    pub fingerprints: Vec<String>,
}

/// ⌨️ The arguments of a todo-create mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TodoCreateInput {
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
}

/// 🔄️ The arguments of a todo-change mutation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TodoChangeInput {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// 🌊️ The criteria a stream or list query filters by.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FilterInput {
    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "regex", default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<bool>,
    #[serde(rename = "matchCase", default, skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
    #[serde(rename = "matchWholeWord", default, skip_serializing_if = "Option::is_none")]
    pub match_whole_word: Option<bool>,
    #[serde(rename = "showIgnored", default, skip_serializing_if = "Option::is_none")]
    pub show_ignored: Option<bool>,
    #[serde(rename = "showGenerated", default, skip_serializing_if = "Option::is_none")]
    pub show_generated: Option<bool>,
    #[serde(rename = "excludeKinds", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_kinds: Vec<String>,
    #[serde(rename = "includeKinds", default, skip_serializing_if = "Vec::is_empty")]
    pub include_kinds: Vec<String>,
}

//#endregion 📥️Input Types
