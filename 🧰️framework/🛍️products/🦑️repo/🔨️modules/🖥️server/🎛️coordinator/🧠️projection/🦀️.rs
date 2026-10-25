//! 🧠️ Projection of the coordinator: the replayed projection and its application of events. See [`super`].

use super::*;

//#region 🧠️Projection
/// 📊️ Everything a coordinator query answers, rebuilt from the log alone.
#[derive(Debug, Default, Clone)]
pub struct Projection {
    pub sequence: u64,
    pub tickets: std::collections::BTreeMap<String, Ticket>,
    pub scopes: std::collections::BTreeMap<String, std::collections::BTreeMap<String, Scope>>,
    pub claims: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    pub warnings: std::collections::BTreeMap<String, Warning>,
    pub contributors: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
}

impl Projection {
    pub(super) fn apply(&mut self, event: &EventEnvelope) -> Result<(), StoreError> {
        if event.stream != COORDINATOR_STREAM {
            return Ok(());
        }
        match event.kind.as_str() {
            EVENT_PUBLISHED => {}
            EVENT_TICKET_RECORDED => {
                let ticket = Ticket::from_value(&event.payload);
                self.tickets.insert(ticket.id.clone(), ticket);
            }
            EVENT_SCOPES_RECORDED => {
                let file_path = event.payload.get("file_path").and_then(Value::as_str).unwrap_or_default().to_string();
                let mut by_id = std::collections::BTreeMap::new();
                if let Some(items) = event.payload.get("scopes").and_then(Value::as_array) {
                    for item in items {
                        let scope = Scope::from_value(item);
                        by_id.insert(scope.id.clone(), scope);
                    }
                }
                self.scopes.insert(file_path, by_id);
            }
            EVENT_CLAIM_RECORDED => {
                let ticket = event.payload.get("ticket_id").and_then(Value::as_str).unwrap_or_default().to_string();
                let scope = event.payload.get("scope_id").and_then(Value::as_str).unwrap_or_default().to_string();
                let kind = event.payload.get("claim_type").and_then(Value::as_str).unwrap_or_default().to_string();
                self.claims.entry(ticket).or_default().insert(scope, kind);
            }
            EVENT_WARNINGS_RECORDED => {
                self.warnings.retain(|_, warning| warning.kind != "conflict");
                if let Some(items) = event.payload.get("warnings").and_then(Value::as_array) {
                    for item in items {
                        let warning = Warning::from_value(item);
                        self.warnings.insert(warning.id.clone(), warning);
                    }
                }
            }
            EVENT_CONTRIBUTOR_RECORDED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                let kind = event.payload.get("kind").and_then(Value::as_str).unwrap_or_default();
                let item = event.payload.get("item_id").and_then(Value::as_str).unwrap_or_default();
                self.contributors.entry(github).or_default().insert(contributor_key(kind, item));
            }
            EVENT_CONTRIBUTOR_RELEASED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(items) = event.payload.get("items").and_then(Value::as_array) {
                    for item in items {
                        let kind = item.get("kind").and_then(Value::as_str).unwrap_or_default();
                        let id = item.get("id").and_then(Value::as_str).unwrap_or_default();
                        self.contributors.entry(github.clone()).or_default().remove(&contributor_key(kind, id));
                    }
                }
            }
            EVENT_CHECKPOINT_RECORDED => {
                let github = event.payload.get("github").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(items) = event.payload.get("files").and_then(Value::as_array) {
                    for item in items {
                        let file = item.as_str().unwrap_or_default();
                        self.contributors.entry(github.clone()).or_default().remove(&contributor_key("file", file));
                    }
                }
            }
            other => return Err(StoreError::Corrupt(format!("unknown coordinator event type {other:?}"))),
        }
        self.sequence = event.sequence;
        Ok(())
    }
}

pub(super) fn contributor_key(kind: &str, id: &str) -> String {
    format!("{kind}\u{0}{id}")
}
//#endregion 🧠️Projection
