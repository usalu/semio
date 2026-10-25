//! 🧾️ Projection records of the coordinator: the ticket, scope and warning records and the event kind names. See [`super`].

use super::*;

//#region 🧠️Projection
/// 🎫️ A tracked work item with its lifecycle status.
#[derive(Debug, Clone, Default)]
pub struct Ticket {
    pub id: String,
    pub status: String,
    pub title: String,
    pub emoji: String,
    pub prompt: String,
    pub summary: String,
    pub llm: String,
    pub client: String,
    pub author: String,
    pub github_issue: String,
    pub created_at: String,
    pub closed_at: Option<String>,
}

impl Ticket {
    pub(super) fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("status".to_string(), Value::String(self.status.clone()));
        object.insert("title".to_string(), Value::String(self.title.clone()));
        object.insert("emoji".to_string(), Value::String(self.emoji.clone()));
        object.insert("prompt".to_string(), Value::String(self.prompt.clone()));
        object.insert("summary".to_string(), Value::String(self.summary.clone()));
        object.insert("llm".to_string(), Value::String(self.llm.clone()));
        object.insert("client".to_string(), Value::String(self.client.clone()));
        object.insert("author".to_string(), Value::String(self.author.clone()));
        object.insert("github_issue".to_string(), Value::String(self.github_issue.clone()));
        object.insert("created_at".to_string(), Value::String(self.created_at.clone()));
        object.insert("closed_at".to_string(), self.closed_at.clone().map_or(Value::Null, Value::String));
        Value::Object(object)
    }

    pub(super) fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            status: text("status"),
            title: text("title"),
            emoji: text("emoji"),
            prompt: text("prompt"),
            summary: text("summary"),
            llm: text("llm"),
            client: text("client"),
            author: text("author"),
            github_issue: text("github_issue"),
            created_at: text("created_at"),
            closed_at: value.get("closed_at").and_then(Value::as_str).map(str::to_string),
        }
    }
}

/// 📖️ A code region — a file, a section or a definition — with its line range.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub id: String,
    pub kind: String,
    pub file_path: String,
    pub section_path: String,
    pub definition: String,
    pub start_line: i64,
    pub end_line: i64,
    pub updated_at: String,
}

impl Scope {
    pub(super) fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("kind".to_string(), Value::String(self.kind.clone()));
        object.insert("file_path".to_string(), Value::String(self.file_path.clone()));
        object.insert("section_path".to_string(), Value::String(self.section_path.clone()));
        object.insert("definition_name".to_string(), Value::String(self.definition.clone()));
        object.insert("start_line".to_string(), Value::from(self.start_line));
        object.insert("end_line".to_string(), Value::from(self.end_line));
        object.insert("updated_at".to_string(), Value::String(self.updated_at.clone()));
        Value::Object(object)
    }

    pub(super) fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            kind: text("kind"),
            file_path: text("file_path"),
            section_path: text("section_path"),
            definition: text("definition_name"),
            start_line: value.get("start_line").and_then(Value::as_i64).unwrap_or_default(),
            end_line: value.get("end_line").and_then(Value::as_i64).unwrap_or_default(),
            updated_at: text("updated_at"),
        }
    }
}

/// 🔭️ A detected issue, such as a scope conflict between tickets.
#[derive(Debug, Clone, Default)]
pub struct Warning {
    pub id: String,
    pub kind: String,
    pub severity: String,
    pub message: String,
    pub ticket_id: String,
    pub scope_id: String,
    pub created_at: String,
}

impl Warning {
    pub(super) fn to_value(&self) -> Value {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(self.id.clone()));
        object.insert("kind".to_string(), Value::String(self.kind.clone()));
        object.insert("severity".to_string(), Value::String(self.severity.clone()));
        object.insert("message".to_string(), Value::String(self.message.clone()));
        object.insert("ticket_id".to_string(), Value::String(self.ticket_id.clone()));
        object.insert("scope_id".to_string(), Value::String(self.scope_id.clone()));
        object.insert("created_at".to_string(), Value::String(self.created_at.clone()));
        object.insert("acknowledged_at".to_string(), Value::Null);
        object.insert("ack_by".to_string(), Value::String(String::new()));
        Value::Object(object)
    }

    pub(super) fn from_value(value: &Value) -> Self {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
        Self {
            id: text("id"),
            kind: text("kind"),
            severity: text("severity"),
            message: text("message"),
            ticket_id: text("ticket_id"),
            scope_id: text("scope_id"),
            created_at: text("created_at"),
        }
    }
}

pub(super) const COORDINATOR_STREAM: &str = "coordinator";

pub(super) const EVENT_PUBLISHED: &str = "event.published";

pub(super) const EVENT_TICKET_RECORDED: &str = "ticket.recorded";

pub(super) const EVENT_SCOPES_RECORDED: &str = "scopes.recorded";

pub(super) const EVENT_CLAIM_RECORDED: &str = "claim.recorded";

pub(super) const EVENT_WARNINGS_RECORDED: &str = "warnings.recorded";

pub(super) const EVENT_CONTRIBUTOR_RECORDED: &str = "contributor.recorded";

pub(super) const EVENT_CONTRIBUTOR_RELEASED: &str = "contributor.released";

pub(super) const EVENT_CHECKPOINT_RECORDED: &str = "checkpoint.recorded";
//#endregion 🧠️Projection
