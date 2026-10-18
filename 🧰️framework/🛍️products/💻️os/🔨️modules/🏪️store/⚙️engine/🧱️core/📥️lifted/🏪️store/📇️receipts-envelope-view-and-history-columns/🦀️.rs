//! 📇️ Receipts, envelope view and history columns: the command receipt, the artifact envelope view, the superseded input conversion, the history column layout over the checkpoint lanes and the mirror oracle constants. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Composition
//#region 🔖️CompositionDsl

/// 🧾 Receipt from the sole store write gate (`dispatch` / `reset`). `messages`/`worst` carry
/// whatever `crate::os_spr::MutationMessage`s the command's own replay produced (empty/`None` for a
/// structural command with nothing to report) — `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6. `Eq` dropped (was `#[derive(.., PartialEq, Eq)]`): `EditMessages`
/// only derives `PartialEq`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
    pub messages: Vec<semio_framework_os_spr::EditMessages>,
    pub worst: Option<semio_framework_diagnostic::Severity>,
}

/// 👁️ Read-only view over a document envelope — mutation is sealed through `dispatch`/`reset`.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactEnvelopeView<'a, P, Mutation> {
    pub(super) envelope: &'a ArtifactEnvelope<P, Mutation>,
}

impl<'a, P, Mutation> ArtifactEnvelopeView<'a, P, Mutation> {
    pub async fn schema(&self) -> &str {
        &self.envelope.schema
    }
    pub async fn id(&self) -> &str {
        &self.envelope.id
    }
    pub async fn vcs(&self) -> &ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation> {
        &self.envelope.vcs
    }
    pub async fn backbone(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }
    pub async fn active_alternative_id(&self) -> Option<&str> {
        self.envelope.active_alternative_id.as_deref()
    }
    pub async fn cursor(&self) -> Option<&ArtifactCursor> {
        self.envelope.cursor.as_ref()
    }
    pub async fn edit_messages(&self) -> &ArtifactEditMessageLedger {
        &self.envelope.edit_messages
    }
    pub async fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] {
        &self.envelope.conflicts
    }
    pub async fn inner(&self) -> &'a ArtifactEnvelope<P, Mutation> {
        self.envelope
    }
}
//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️TextFormat
//#endregion 🔖️TextFormat
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️CommandFormat
/// 📜️ One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}
semio_framework_value::artifact_retire_struct!(HistoryColumn { checkpoint_id, timestamp, labels, authors, parent_checkpoint_id, description, lane, alternative_ids });

/// 🌿️ The branched alternatives of `envelope`: every listed alternative but the trunk, whose checkpoints are the main line
/// the history view draws on lane 0.
pub fn branched_alternatives<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> impl Iterator<Item = &Alternative> {
    let trunk = semio_framework_os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
    envelope.vcs.alternatives.iter().filter(move |alternative| alternative.id != trunk)
}

pub async fn checkpoint_alternatives<'a, P, Mutation>(envelope: &'a ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<&'a Alternative> {
    branched_alternatives(envelope).filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id)).collect()
}

pub async fn is_checkpoint_main_only<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).await.is_empty()
}

/// 🌀️ Self-recursive async fn — `Box::pin` at the recursive call site only (R10 shape 3):
/// the fn's own opaque future type would otherwise need to contain itself (E0733).
pub async fn has_main_only_descendant<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, children_of: &HashMap<String, Vec<String>>, checkpoint_id: &str, seen: &mut HashSet<String>) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id).await || Box::pin(has_main_only_descendant(envelope, children_of, child_id, seen)).await {
            return true;
        }
    }
    false
}

/// 🛤️ Assigns each checkpoint a swimlane: branched alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
pub async fn assign_history_checkpoint_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in branched_alternatives(envelope).enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id).await || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen).await {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives.await.iter().map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0)).collect();
        let lane = if lanes.len() == 1 { lanes[0] } else { lanes.into_iter().min().unwrap_or(0) };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// 📜️ Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub async fn build_history_columns<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope).await;
    let mut columns = Vec::with_capacity(envelope.vcs.checkpoints.len());
    for (index, checkpoint) in envelope.vcs.checkpoints.iter().rev().enumerate() {
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id).await;
        let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
        let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
        if labels.is_empty() && index == 0 {
            labels.push("main".into());
        }
        columns.push(HistoryColumn {
            checkpoint_id: checkpoint.id.clone(),
            timestamp: checkpoint.timestamp.clone(),
            labels,
            authors: checkpoint.authors.clone(),
            parent_checkpoint_id: checkpoint.parent_id.clone(),
            description: checkpoint.message.clone(),
            lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
            alternative_ids,
        });
    }
    columns
}
//#endregion 🔖️CommandFormat
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🔬️ Histories up to this many applied edits re-derive every incremental mirror from scratch after each test-build bump.
#[doc(hidden)]
pub const MIRROR_ORACLE_HISTORY: usize = 256;

/// 🔬️ The history length up to which the test oracle rebuilds every mirror on every bump; beyond it only every [`MIRROR_ORACLE_STRIDE`]th length is rebuilt, so a production-shaped history costs O(n²/stride) instead of O(n²).
#[doc(hidden)]
pub const MIRROR_ORACLE_EVERY_HISTORY: usize = 64;

/// 🔬️ The history-length stride of the sampled test oracle ([`MIRROR_ORACLE_EVERY_HISTORY`]).
#[doc(hidden)]
pub const MIRROR_ORACLE_STRIDE: usize = 32;
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
