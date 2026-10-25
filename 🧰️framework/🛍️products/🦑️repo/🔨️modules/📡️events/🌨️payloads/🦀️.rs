//! 🌨️ Payloads: one typed payload record per event kind.

use serde::{Deserialize, Serialize};

//#region 🌨️Payloads

fn is_zero(value: &i64) -> bool {
    *value == 0
}

/// 📦️ Common ticket identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
}

/// 🎫️ Payload for `ticket.open`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketOpenPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub title: String,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    pub goal: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
}

/// 📪️ Payload for `ticket.close`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketClosePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🔓️ Payload for `ticket.reopen`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketReopenPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// ♻️ Payload for `ticket.change`. A field absent from the wire is a field left untouched.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// ⛳️ Common goal identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalPayload {
    pub id: String,
}

/// 🎯️ Payload for `goal.open`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalOpenPayload {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🏁️ Payload for `goal.close`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalClosePayload {
    pub id: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🔄️ Payload for `goal.reopen`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalReopenPayload {
    pub id: String,
    pub prompt: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📐️ Payload for `goal.change`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 👥️ Contributor identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContributorPayload {
    pub github: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 💾️ Payload for `checkpoint` (a GitHub push).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckpointPayload {
    pub author: String,
    pub github: String,
    pub sha: String,
    pub message: String,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technologies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files_changed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
}

/// ✅️ Todo identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TodoPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🆕️ Payload for `todo.create`.
pub type TodoCreatePayload = TodoPayload;

/// 🗑️ Payload for `todo.delete`.
pub type TodoDeletePayload = TodoPayload;

/// ✏️ Payload for `todo.change`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TodoChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 💼️ A single item a contributor is working on.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkItem {
    pub kind: String,
    pub id: String,
}

/// 🤝️ Every work item of one contributor.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContributorWork {
    pub github: String,
    #[serde(default)]
    pub tickets: Option<Vec<String>>,
    #[serde(default)]
    pub goals: Option<Vec<String>>,
    #[serde(default)]
    pub todos: Option<Vec<String>>,
    #[serde(default)]
    pub technologies: Option<Vec<String>>,
    #[serde(default)]
    pub bundles: Option<Vec<String>>,
    #[serde(default)]
    pub folders: Option<Vec<String>>,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    pub sections: Option<Vec<String>>,
    #[serde(default)]
    pub definitions: Option<Vec<String>>,
}

/// 📝️ Draft identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DraftPayload {
    pub slug: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📄️ File operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FilePayload {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📁️ Folder operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FolderPayload {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📑️ Section operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SectionPayload {
    pub file: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub old_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🧬️ Integrate operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IntegratePayload {
    pub source: String,
    pub target_file: String,
    pub target_section: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🧲️ Extract operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExtractPayload {
    pub source_file: String,
    pub source_section: String,
    pub target_file: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

//#endregion 🌨️Payloads
