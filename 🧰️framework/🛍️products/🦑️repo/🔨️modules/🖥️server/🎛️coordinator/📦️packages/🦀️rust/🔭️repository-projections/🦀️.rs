//! 🔭️ Repository projections of the coordinator: the read side that projects tickets, scopes, claims, warnings, breaches, conflicts and contributors. See [`super`].

use super::*;

//#region 📚️Repository
impl Repository {
    /// 🎫️ Every ticket, optionally filtered by status, in id order.
    pub fn project_tickets(&self, status: &str) -> Vec<Ticket> {
        self.projection
            .lock()
            .expect("projection")
            .tickets
            .values()
            .filter(|ticket| status.is_empty() || ticket.status == status)
            .cloned()
            .collect()
    }

    /// 🎫️ One ticket by id.
    pub fn project_ticket(&self, ticket_id: &str) -> Result<Ticket, StoreError> {
        self.projection
            .lock()
            .expect("projection")
            .tickets
            .get(ticket_id)
            .cloned()
            .ok_or_else(|| StoreError::NotFound(format!("ticket {ticket_id:?}")))
    }

    /// 📖️ Every scope recorded for one file, in id order.
    pub fn project_scopes_by_file(&self, file_path: &str) -> Vec<Scope> {
        self.projection
            .lock()
            .expect("projection")
            .scopes
            .get(file_path)
            .map(|scopes| scopes.values().cloned().collect())
            .unwrap_or_default()
    }

    /// 🗡️ Every scope one ticket has claimed, in id order.
    pub fn project_claims_by_ticket(&self, ticket_id: &str) -> Vec<Scope> {
        let projection = self.projection.lock().expect("projection");
        let mut by_id: std::collections::BTreeMap<&String, &Scope> = std::collections::BTreeMap::new();
        for scopes in projection.scopes.values() {
            for (id, scope) in scopes {
                by_id.insert(id, scope);
            }
        }
        projection
            .claims
            .get(ticket_id)
            .map(|claims| claims.keys().filter_map(|id| by_id.get(id).map(|scope| (*scope).clone())).collect())
            .unwrap_or_default()
    }

    /// ⚠️ Every warning, optionally filtered by ticket, in id order.
    pub fn project_warnings(&self, ticket_id: &str) -> Vec<Warning> {
        self.projection
            .lock()
            .expect("projection")
            .warnings
            .values()
            .filter(|warning| ticket_id.is_empty() || warning.ticket_id == ticket_id)
            .cloned()
            .collect()
    }

    /// 🔷️ Breaches are not yet recorded by any command, so the projection is always empty.
    pub fn project_breachs(&self, _ticket_id: &str) -> Vec<Value> {
        Vec::new()
    }

    /// 💥️ Every scope claimed by more than one open ticket, in scope order.
    pub fn project_conflicts(&self) -> Vec<(String, Vec<String>)> {
        let projection = self.projection.lock().expect("projection");
        let mut by_scope: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        for (ticket_id, claims) in &projection.claims {
            if projection.tickets.get(ticket_id).map(|ticket| ticket.status.as_str()) != Some("open") {
                continue;
            }
            for scope_id in claims.keys() {
                by_scope.entry(scope_id.clone()).or_default().push(ticket_id.clone());
            }
        }
        by_scope
            .into_iter()
            .filter(|(_, tickets)| tickets.len() > 1)
            .map(|(scope, mut tickets)| {
                tickets.sort();
                (scope, tickets)
            })
            .collect()
    }

    /// 🧑️ Every contributor working on one item, in login order.
    pub fn project_contributors_on_item(&self, kind: &str, item_id: &str) -> Vec<String> {
        let key = contributor_key(kind, item_id);
        self.projection
            .lock()
            .expect("projection")
            .contributors
            .iter()
            .filter(|(_, items)| items.contains(&key))
            .map(|(github, _)| github.clone())
            .collect()
    }
}

pub(super) fn command_id(kind: &str, generation: u64, payload: &Value) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{kind}\u{0}{generation}\u{0}").as_bytes());
    hasher.update(to_go_json(payload).as_bytes());
    format!("command-{}", hasher.finish())
}
//#endregion 📚️Repository
