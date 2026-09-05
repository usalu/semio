//! 💾️ Closed shell-persistence correlation contract; selection ids refer to shell-owned paths.

use serde::{Deserialize, Serialize};

pub const PERSISTENCE_PENDING_CAPACITY: usize = 64;
pub const PERSISTENCE_MESSAGE_LIMIT: usize = 4_096;
pub const PERSISTENCE_BYTE_LIMIT: u64 = 67_108_864;
const SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistenceScopeV1 {
    pub window_id: String,
    pub space_id: String,
    pub session_id: String,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistenceCorrelationV1 {
    pub scope: PersistenceScopeV1,
    pub request_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PersistenceSpaceKindV1 { Atelier, Studio, Archive }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PersistenceVisibilityV1 { Private, Public }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum PersistenceOperationV1 {
    CreateSpace { name: String, space_kind: PersistenceSpaceKindV1, visibility: PersistenceVisibilityV1, selection_id: String },
    BindSpace { selection_id: String },
    ImportSpace { selection_id: String },
    DeleteEntry { entry_id: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistenceRequestV1 {
    pub schema: String,
    pub correlation: PersistenceCorrelationV1,
    pub operation: PersistenceOperationV1,
    pub deadline_ms: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PersistencePhaseV1 { Queued, Selecting, Opening, Writing, Committing }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PersistenceFailureV1 { Denied, Capacity, Stale, OpenFailed, WriteFailed, InvalidDocument, Deadline, WorkerFailed }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum PersistenceOutcomeV1 {
    Committed { space_id: String, catalog_generation: u64, content_digest: String, byte_length: u64 },
    Deleted { entry_id: String, catalog_generation: u64 },
    Failed { code: PersistenceFailureV1 },
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum PersistenceEventV1 {
    Progress { schema: String, correlation: PersistenceCorrelationV1, phase: PersistencePhaseV1, completed_bytes: u64, total_bytes: u64 },
    Terminal { schema: String, correlation: PersistenceCorrelationV1, outcome: PersistenceOutcomeV1 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistenceContractError { Invalid, TooLarge, Scope, Sequence, Terminal }

fn valid_id(value: &str) -> bool {
    (1..=128).contains(&value.len()) && value.as_bytes()[0].is_ascii_alphanumeric() && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
}

fn valid_correlation(value: &PersistenceCorrelationV1) -> bool {
    valid_id(&value.scope.window_id) && valid_id(&value.scope.space_id) && valid_id(&value.scope.session_id) && value.scope.generation < SAFE_INTEGER && (1..=SAFE_INTEGER).contains(&value.request_id)
}

impl PersistenceRequestV1 {
    pub fn parse(bytes: &[u8]) -> Result<Self, PersistenceContractError> {
        if bytes.len() > PERSISTENCE_MESSAGE_LIMIT { return Err(PersistenceContractError::TooLarge); }
        let request: Self = serde_json::from_slice(bytes).map_err(|_| PersistenceContractError::Invalid)?;
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), PersistenceContractError> {
        let operation = match &self.operation {
            PersistenceOperationV1::CreateSpace { name, selection_id, .. } => (1..=256).contains(&name.chars().count()) && !name.chars().any(char::is_control) && valid_id(selection_id),
            PersistenceOperationV1::BindSpace { selection_id } | PersistenceOperationV1::ImportSpace { selection_id } => valid_id(selection_id),
            PersistenceOperationV1::DeleteEntry { entry_id } => valid_id(entry_id),
        };
        if self.schema != "semio.shell.persistence-request/v1" || !valid_correlation(&self.correlation) || !(100..=30_000).contains(&self.deadline_ms) || !operation { return Err(PersistenceContractError::Invalid); }
        Ok(())
    }
}

impl PersistenceEventV1 {
    pub fn parse(bytes: &[u8]) -> Result<Self, PersistenceContractError> {
        if bytes.len() > PERSISTENCE_MESSAGE_LIMIT { return Err(PersistenceContractError::TooLarge); }
        let event: Self = serde_json::from_slice(bytes).map_err(|_| PersistenceContractError::Invalid)?;
        event.validate()?;
        Ok(event)
    }

    fn correlation(&self) -> &PersistenceCorrelationV1 {
        match self { Self::Progress { correlation, .. } | Self::Terminal { correlation, .. } => correlation }
    }

    fn validate(&self) -> Result<(), PersistenceContractError> {
        let (schema, bounded) = match self {
            Self::Progress { schema, completed_bytes, total_bytes, .. } => (schema, completed_bytes <= total_bytes && *total_bytes <= PERSISTENCE_BYTE_LIMIT),
            Self::Terminal { schema, outcome, .. } => (schema, match outcome {
                PersistenceOutcomeV1::Committed { space_id, catalog_generation, content_digest, byte_length } => valid_id(space_id) && (1..=SAFE_INTEGER).contains(catalog_generation) && (1..=PERSISTENCE_BYTE_LIMIT).contains(byte_length) && content_digest.len() == 64 && content_digest.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) && content_digest.bytes().any(|byte| byte != b'0'),
                PersistenceOutcomeV1::Deleted { entry_id, catalog_generation } => valid_id(entry_id) && (1..=SAFE_INTEGER).contains(catalog_generation),
                PersistenceOutcomeV1::Failed { .. } | PersistenceOutcomeV1::Cancelled => true,
            }),
        };
        if schema != "semio.shell.persistence-event/v1" || !valid_correlation(self.correlation()) || !bounded { return Err(PersistenceContractError::Invalid); }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PersistenceObservation { Progress, Terminal, DurablePublication }

/// 🧷️ Receipt consumer only; admission, selected-path ownership and durable commit remain shell duties.
pub(crate) struct PendingPersistenceV1 {
    request: PersistenceRequestV1,
    phase: Option<PersistencePhaseV1>,
    completed: u64,
    total: Option<u64>,
    terminal: bool,
}

impl PendingPersistenceV1 {
    pub(crate) fn new(request: PersistenceRequestV1, owner_scope: &PersistenceScopeV1) -> Result<Self, PersistenceContractError> {
        request.validate()?;
        if &request.correlation.scope != owner_scope { return Err(PersistenceContractError::Scope); }
        Ok(Self { request, phase: None, completed: 0, total: None, terminal: false })
    }

    pub(crate) fn observe(&mut self, event: &PersistenceEventV1) -> Result<PersistenceObservation, PersistenceContractError> {
        event.validate()?;
        if self.terminal { return Err(PersistenceContractError::Terminal); }
        if event.correlation() != &self.request.correlation { return Err(PersistenceContractError::Scope); }
        match event {
            PersistenceEventV1::Progress { phase, completed_bytes, total_bytes, .. } => {
                if self.phase.is_some_and(|previous| *phase < previous) || *completed_bytes < self.completed || self.total.is_some_and(|total| total != *total_bytes) || (*phase == PersistencePhaseV1::Committing && completed_bytes != total_bytes) { return Err(PersistenceContractError::Sequence); }
                self.phase = Some(*phase);
                self.completed = *completed_bytes;
                self.total = Some(*total_bytes);
                Ok(PersistenceObservation::Progress)
            }
            PersistenceEventV1::Terminal { outcome, .. } => {
                let committed = self.phase == Some(PersistencePhaseV1::Committing);
                let valid = match outcome {
                    PersistenceOutcomeV1::Cancelled => !committed,
                    PersistenceOutcomeV1::Failed { .. } => true,
                    PersistenceOutcomeV1::Committed { space_id, catalog_generation, byte_length, .. } => committed && self.total == Some(*byte_length) && self.completed == *byte_length && *catalog_generation == self.request.correlation.scope.generation + 1 && match &self.request.operation {
                        PersistenceOperationV1::DeleteEntry { .. } => false,
                        PersistenceOperationV1::BindSpace { .. } => space_id == &self.request.correlation.scope.space_id,
                        _ => true,
                    },
                    PersistenceOutcomeV1::Deleted { entry_id, catalog_generation } => committed && *catalog_generation == self.request.correlation.scope.generation + 1 && matches!(&self.request.operation, PersistenceOperationV1::DeleteEntry { entry_id: expected } if entry_id == expected),
                };
                if !valid { return Err(PersistenceContractError::Sequence); }
                self.terminal = true;
                Ok(if matches!(outcome, PersistenceOutcomeV1::Committed { .. } | PersistenceOutcomeV1::Deleted { .. }) { PersistenceObservation::DurablePublication } else { PersistenceObservation::Terminal })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap() }

    #[test]
    fn persistence_contract_matches_neutral_scope_progress_and_terminal_traces() {
        let fixture = fixture();
        let requests = fixture["requests"].as_array().unwrap();
        assert_eq!(requests.len(), 4);
        assert_eq!(fixture["traces"].as_array().unwrap().len(), 15);
        for trace in fixture["traces"].as_array().unwrap() {
            let row = requests.iter().find(|row| row["id"] == trace["request"]).unwrap();
            let request = PersistenceRequestV1::parse(&serde_json::to_vec(&row["value"]).unwrap()).unwrap();
            let scope = request.correlation.scope.clone();
            let mut pending = PendingPersistenceV1::new(request, &scope).unwrap();
            let mut publications = 0;
            for step in trace["steps"].as_array().unwrap() {
                let outcome = PersistenceEventV1::parse(&serde_json::to_vec(&step["value"]).unwrap()).and_then(|event| pending.observe(&event));
                assert_eq!(outcome.is_ok(), step["accept"].as_bool().unwrap(), "{}: {step}", trace["id"]);
                publications += usize::from(outcome == Ok(PersistenceObservation::DurablePublication));
            }
            assert_eq!(publications as u64, trace["published"].as_u64().unwrap(), "{}", trace["id"]);
        }
    }

    #[test]
    fn persistence_contract_rejects_closed_fields_and_cross_scope_receipts() {
        let fixture = fixture();
        let source = &fixture["requests"][0]["value"];
        assert_eq!(fixture["requestNegatives"].as_array().unwrap().len(), 16);
        for row in fixture["requestNegatives"].as_array().unwrap() {
            let mut hostile = source.clone();
            let path: Vec<&str> = row["path"].as_str().unwrap().split('.').collect();
            let mut parent = &mut hostile;
            for key in &path[..path.len() - 1] { parent = &mut parent[*key]; }
            parent[path[path.len() - 1]] = row["value"].clone();
            assert!(PersistenceRequestV1::parse(&serde_json::to_vec(&hostile).unwrap()).is_err(), "{}", row["id"]);
        }
        let request = PersistenceRequestV1::parse(&serde_json::to_vec(source).unwrap()).unwrap();
        let mut scope = request.correlation.scope.clone();
        scope.window_id = "window-b".into();
        assert!(matches!(PendingPersistenceV1::new(request, &scope), Err(PersistenceContractError::Scope)));
        assert_eq!(PersistenceRequestV1::parse(&vec![b' '; PERSISTENCE_MESSAGE_LIMIT + 1]), Err(PersistenceContractError::TooLarge));
        assert_eq!(PersistenceEventV1::parse(&vec![b' '; PERSISTENCE_MESSAGE_LIMIT + 1]), Err(PersistenceContractError::TooLarge));
        for extra in ["path", "token", "message"] {
            let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
            event["outcome"][extra] = serde_json::json!("private");
            assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
        }
        for digest in ["0".repeat(64), "A".repeat(64), "a".repeat(63)] {
            let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
            event["outcome"]["contentDigest"] = serde_json::json!(digest);
            assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
        }
        for key in ["requestId", "generation"] {
            let mut event = fixture["traces"][0]["steps"][4]["value"].clone();
            let correlation = if key == "requestId" { &mut event["correlation"] } else { &mut event["correlation"]["scope"] };
            correlation[key] = serde_json::json!(SAFE_INTEGER + 1);
            assert!(PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).is_err());
        }
        for key in ["windowId", "spaceId", "sessionId", "generation"] {
            let request = PersistenceRequestV1::parse(&serde_json::to_vec(source).unwrap()).unwrap();
            let scope = request.correlation.scope.clone();
            let mut pending = PendingPersistenceV1::new(request, &scope).unwrap();
            let mut event = fixture["traces"][0]["steps"][0]["value"].clone();
            event["correlation"]["scope"][key] = if key == "generation" { serde_json::json!(8) } else { serde_json::json!("another-owner") };
            let event = PersistenceEventV1::parse(&serde_json::to_vec(&event).unwrap()).unwrap();
            assert_eq!(pending.observe(&event), Err(PersistenceContractError::Scope));
            assert_eq!(pending.phase, None);
            assert_eq!(pending.completed, 0);
            assert!(!pending.terminal);
        }
    }
}
