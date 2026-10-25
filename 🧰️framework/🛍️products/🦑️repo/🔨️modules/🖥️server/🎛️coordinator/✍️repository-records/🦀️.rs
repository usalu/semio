//! ✍️ Repository records of the coordinator: the commands that record published events, tickets, scopes, claims, warnings, contributor work and checkpoints. See [`super`].

use super::*;

//#region 📚️Repository
impl Repository {
    /// 📨️ Records one published in-process event.
    pub fn record_published_event(&self, event: &Value) -> Result<(), StoreError> {
        self.execute(EVENT_PUBLISHED, event.clone())
    }

    /// 🎫️ Records the current state of one ticket.
    pub fn record_ticket(&self, ticket: &Ticket) -> Result<(), StoreError> {
        self.execute(EVENT_TICKET_RECORDED, ticket.to_value())
    }

    /// 📖️ Records the scopes of one file.
    pub fn record_scopes(&self, file_path: &str, scopes: &[Scope]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("file_path".to_string(), Value::String(file_path.to_string()));
        object.insert("scopes".to_string(), Value::Array(scopes.iter().map(Scope::to_value).collect()));
        self.execute(EVENT_SCOPES_RECORDED, Value::Object(object))
    }

    /// 🗡️ Records one ticket's claim on one scope.
    pub fn record_claim(&self, ticket_id: &str, scope_id: &str, claim_type: &str, at: &str) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("ticket_id".to_string(), Value::String(ticket_id.to_string()));
        object.insert("scope_id".to_string(), Value::String(scope_id.to_string()));
        object.insert("claim_type".to_string(), Value::String(claim_type.to_string()));
        object.insert("at".to_string(), Value::String(at.to_string()));
        self.execute(EVENT_CLAIM_RECORDED, Value::Object(object))
    }

    /// ⚠️ Replaces every conflict warning with the current set.
    pub fn record_warnings(&self, warnings: &[Warning]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("warnings".to_string(), Value::Array(warnings.iter().map(Warning::to_value).collect()));
        self.execute(EVENT_WARNINGS_RECORDED, Value::Object(object))
    }

    /// 🧑️ Records that a contributor is working on one item.
    pub fn record_contributor_work(&self, github: &str, kind: &str, item_id: &str) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("github".to_string(), Value::String(github.to_string()));
        object.insert("kind".to_string(), Value::String(kind.to_string()));
        object.insert("item_id".to_string(), Value::String(item_id.to_string()));
        self.execute(EVENT_CONTRIBUTOR_RECORDED, Value::Object(object))
    }

    /// 🏁️ Records a checkpoint, releasing every file it touched.
    pub fn record_checkpoint(&self, github: &str, files: &[String]) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("github".to_string(), Value::String(github.to_string()));
        object.insert("files".to_string(), Value::Array(files.iter().map(|file| Value::String(file.clone())).collect()));
        self.execute(EVENT_CHECKPOINT_RECORDED, Value::Object(object))
    }
}
//#endregion 📚️Repository
