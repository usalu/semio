//! 📜️ The space history document: checkpoints, alternatives and member pins of a space, its snapshot, diff and steps.
//! Its SQLite snapshot io and its mutations live in `📜️space-history` and `🧬️schema/🧬️mutations`.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[cfg(test)] use serde::{Deserialize, Serialize};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_spr::HybridLogicalTimestamp;
use semio_framework_os_spr::{DiffRegions, Mutation, MutationDiff, TouchedPaths};
use semio_framework_os_vcs::Author;
use semio_framework_os_vcs::Identified;

/// 📌️ One member document's position at the moment a `SpaceCheckpoint` was recorded.
/// 🔮️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive transitively (`CommitSpaceCheckpoint` → `SpaceCheckpoint` → this type). Production never
/// serializes through serde, so this never reaches a shipped plugin component.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SpaceMemberPin {
    pub document_id: String,
    pub checkpoint_id: String,
    /// 🌿️ Empty string when the member had no active alternative (its own trunk) at pin time.
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub alternative_id: String,
}

/// 🗄️ A space-wide checkpoint: one pin per registered member, so checking it out (or an
/// alternative built on top of it) fans out deterministically to every member's own VCS.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpaceCheckpoint {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub message: String,
    pub authors: Vec<Author>,
    pub timestamp: HybridLogicalTimestamp,
    pub members: Vec<SpaceMemberPin>,
}

/// 🔮️ serde stays TEST-ONLY — see `SpaceMemberPin`'s docstring above.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SpaceAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// 🏷️ Schema id of the space-wide history meta-document — `"os.space.history"`, under the
/// `os.` schema lattice (a generic os-shell abstraction, not the `s.` product lattice), separate
/// from `space::S_SPACE_SCHEMA`/`space::S_COLLECTION_SCHEMA` (this crate sits below `space` in the
/// dependency graph, so it declares its own constant rather than depending on that crate's). The
/// `.spr` extension (`SpaceHistorySnapshot::EXTENSION`, `"space-history"`) is unchanged.
pub const S_SPACE_HISTORY_SCHEMA: &str = "os.space.history";

/// 🗄️ Snapshot of the `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`) meta-document: itself
/// an ordinary `ArtifactVcs` document kind (dogfooded — no bespoke transport), holding the
/// space-level checkpoint/alternative graph that `SpaceHost` composes on top of every registered
/// member's own history.
#[derive(Clone, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpaceHistorySnapshot {
    pub checkpoints: Vec<SpaceCheckpoint>,
    /// 🌿️ Held in ASCENDING `id` order, not in creation order. Two facts force it: a branch
    /// list is unordered in the product (nothing reads `alternatives[0]` as "the first branch"), and
    /// only a canonical order makes `RemoveSpaceAlternative`'s inverse an exact inverse — re-adding
    /// a removed alternative by appending put it back at the END, so undoing a removal changed the
    /// document (`space_history_op_round_trips`: "operation inverse did not restore pre-state"). The
    /// same canonical order is what lets two replicas that created branches concurrently converge.
    pub alternatives: Vec<SpaceAlternative>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

/// 🪜️ One ordered space-history step: a checkpoint inserted or removed at its exact position, an alternative added or removed by id, or the active alternative set.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SpaceHistoryStep {
    AddCheckpoint {
        checkpoint: SpaceCheckpoint,
        #[value(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    RemoveCheckpoint { checkpoint_id: String, index: u64 },
    AddAlternative(SpaceAlternative),
    RemoveAlternative { alternative_id: String },
    SetActive { alternative_id: Option<String> },
}

/// 🔺️ Space-history changes in application order; consecutive active-alternative sets coalesce into the last one.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SpaceHistoryDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<SpaceHistoryStep>,
}

impl SpaceHistoryDiff {
    pub fn step(step: SpaceHistoryStep) -> Self {
        Self { steps: vec![step] }
    }
}

impl SpaceHistoryStep {
    /// ↩️ The steps that undo `self`, read from `base` or from the rows of `earlier` steps of the same diff; the active alternative
    /// before `self` is the last earlier `SetActive` row, else the base's. Nothing is applied.
    fn undo_rows(&self, base: &SpaceHistorySnapshot, earlier: &[SpaceHistoryStep]) -> Vec<SpaceHistoryStep> {
        let active = || earlier.iter().rev().find_map(|step| if let Self::SetActive { alternative_id } = step { Some(alternative_id.clone()) } else { None }).unwrap_or_else(|| base.active_alternative_id.clone());
        let length = || (base.checkpoints.len() + earlier.iter().filter(|step| matches!(step, Self::AddCheckpoint { .. })).count()).saturating_sub(earlier.iter().filter(|step| matches!(step, Self::RemoveCheckpoint { .. })).count()) as u64;
        match self {
            Self::AddCheckpoint { checkpoint, index } => vec![Self::RemoveCheckpoint { checkpoint_id: checkpoint.id.clone(), index: index.unwrap_or_else(length) }],
            Self::RemoveCheckpoint { checkpoint_id, index } => base
                .checkpoints
                .iter()
                .find(|checkpoint| checkpoint.id == *checkpoint_id)
                .or_else(|| earlier.iter().rev().find_map(|step| if let Self::AddCheckpoint { checkpoint, .. } = step { (checkpoint.id == *checkpoint_id).then_some(checkpoint) } else { None }))
                .map(|checkpoint| vec![Self::AddCheckpoint { checkpoint: checkpoint.clone(), index: Some(*index) }])
                .unwrap_or_default(),
            Self::AddAlternative(alternative) => vec![Self::RemoveAlternative { alternative_id: alternative.id.clone() }],
            Self::RemoveAlternative { alternative_id } => base
                .alternatives
                .iter()
                .find(|alternative| alternative.id == *alternative_id)
                .or_else(|| earlier.iter().rev().find_map(|step| if let Self::AddAlternative(alternative) = step { (alternative.id == *alternative_id).then_some(alternative) } else { None }))
                .map(|alternative| vec![Self::AddAlternative(alternative.clone()), Self::SetActive { alternative_id: active() }])
                .unwrap_or_default(),
            Self::SetActive { .. } => vec![Self::SetActive { alternative_id: active() }],
        }
    }

    fn step_into(&self, next: &mut SpaceHistorySnapshot) -> semio_framework_os_spr::MutationApplyResult<()> {
        match self {
            Self::AddCheckpoint { checkpoint, index } => {
                if next.checkpoints.iter().any(|existing| existing.id == checkpoint.id) {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.duplicate-target", format!("checkpoint {} already exists", checkpoint.id)).at(["checkpoints", checkpoint.id.as_str()]));
                }
                let at = index.map_or(Some(next.checkpoints.len()), |index| usize::try_from(index).ok().filter(|at| *at <= next.checkpoints.len()));
                let Some(at) = at else {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.invalid-add-index", format!("checkpoint {} is inserted past the end", checkpoint.id)).at(["checkpoints", checkpoint.id.as_str()]));
                };
                next.checkpoints.insert(at, checkpoint.clone());
            }
            Self::RemoveCheckpoint { checkpoint_id, index } => {
                let Some(at) = next.checkpoints.iter().position(|checkpoint| checkpoint.id == *checkpoint_id) else {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("checkpoint {checkpoint_id} does not exist")).at(["checkpoints", checkpoint_id.as_str()]));
                };
                if u64::try_from(at) != Ok(*index) {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.order-mismatch", format!("checkpoint {checkpoint_id} is not at index {index}")).at(["checkpoints", checkpoint_id.as_str()]));
                }
                next.checkpoints.remove(at);
            }
            Self::AddAlternative(alternative) => {
                if next.alternatives.iter().any(|existing| existing.id == alternative.id) {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.duplicate-target", format!("alternative {} already exists", alternative.id)).at(["alternatives", alternative.id.as_str()]));
                }
                let position = next.alternatives.partition_point(|existing| existing.id < alternative.id);
                next.alternatives.insert(position, alternative.clone());
            }
            Self::RemoveAlternative { alternative_id } => {
                if !next.alternatives.iter().any(|alternative| alternative.id == *alternative_id) {
                    return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("alternative {alternative_id} does not exist")).at(["alternatives", alternative_id.as_str()]));
                }
                next.alternatives.retain(|alternative| alternative.id != *alternative_id);
            }
            Self::SetActive { alternative_id } => {
                if let Some(alternative_id) = alternative_id {
                    if !next.alternatives.iter().any(|alternative| alternative.id == *alternative_id) {
                        return Err(semio_framework_os_spr::MutationApplyError::new("mutation.apply.missing-target", format!("active alternative {alternative_id} does not exist")).at(["activeAlternativeId"]));
                    }
                }
                next.active_alternative_id = alternative_id.clone();
            }
        }
        Ok(())
    }
}

impl semio_framework_os_spr::DiffAlgebra<SpaceHistorySnapshot> for SpaceHistoryDiff {
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Self {
        let rows: Vec<Vec<SpaceHistoryStep>> = self.steps.iter().enumerate().map(|(index, step)| step.undo_rows(base, &self.steps[..index])).collect();
        Self { steps: rows.into_iter().rev().flatten().collect() }
    }

}

impl MutationDiff<SpaceHistorySnapshot> for SpaceHistoryDiff {
    fn apply(&self, snapshot: &mut SpaceHistorySnapshot, _capability: semio_framework_os_spr::ApplyCapability) -> semio_framework_os_spr::MutationApplyResult<()> {
        let mut prepared =  semio_framework_os_spr::PreparedField::new(snapshot.clone(), <Self as semio_framework_os_spr::MutationDiff<SpaceHistorySnapshot>>::retire_projection, _capability);
        let mut next = prepared.value_mut();
        for step in &self.steps {
            step.step_into(&mut next)?;
        }

        prepared.install(snapshot);
        Ok(())
    }

    fn absorb(&mut self, other: Self) {
        for step in other.steps {
            match (self.steps.last_mut(), &step) {
                (Some(held @ SpaceHistoryStep::SetActive { .. }), SpaceHistoryStep::SetActive { .. }) => *held = step,
                _ => self.steps.push(step),
            }
        }
    }

    fn apply(&self, snapshot: &mut SpaceHistorySnapshot, _capability: semio_framework_os_spr::ApplyCapability) -> semio_framework_os_spr::MutationApplyResult<()> {
        let mut prepared = semio_framework_os_spr::PreparedField::new(snapshot.clone(), <Self as semio_framework_os_spr::MutationDiff<SpaceHistorySnapshot>>::retire_projection, _capability);
        let mut next = prepared.value_mut();
        for step in &self.steps {
            step.step_into(&mut next)?;
        }
        prepared.install(snapshot);
        Ok(())
    }

}

impl semio_framework_os_spr::ColdDiffAlgebra<SpaceHistorySnapshot> for SpaceHistoryDiff {
    fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}


impl DiffRegions for SpaceHistoryDiff {
    fn touches(&self) -> TouchedPaths {
        let mut regions = Vec::new();
        for step in &self.steps {
            let region = match step {
                SpaceHistoryStep::AddCheckpoint { .. } | SpaceHistoryStep::RemoveCheckpoint { .. } => "checkpoints",
                SpaceHistoryStep::AddAlternative(_) | SpaceHistoryStep::RemoveAlternative { .. } => "alternatives",
                SpaceHistoryStep::SetActive { .. } => "activeAlternativeId",
            };
            if !regions.contains(&region) {
                regions.push(region);
            }
        }
        TouchedPaths::new(regions)
    }
}
