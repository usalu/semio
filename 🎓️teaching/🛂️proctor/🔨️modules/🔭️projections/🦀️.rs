//! 🔭️ The proctor's read models, folded from the committed event log.
//!
//! **What is kept.** Per learner the folded `LearnerState` (`quiz.learner-state`, with the last
//! folded stream sequence) and its `LearnerView` (`quiz.learner`); per run its `RunView`
//! (`quiz.run`); one `Leaderboard` (`quiz.leaderboard/all`); the fingerprint of the catalog the
//! models were built against (`quiz.meta/catalog`). All views come from the quiz core's view
//! functions — the projector only decides *when* to recompute them.
//!
//! **How it folds.** The projector walks the database-wide commit order after its checkpoint
//! (`quiz`) in batches and writes every touched view plus the advanced checkpoint in one
//! transaction, so a batch is visible entirely or not at all. A learner's facts are folded only
//! past the sequence already folded, which makes a replayed batch harmless. Roster facts are not
//! folded: the enrollment saga relays them into the learner streams. The fold is cancellable
//! between batches and reports progress; a cancelled fold leaves a consistent prefix whose
//! checkpoint the next catch-up resumes from.
//!
//! **Why not a saga.** `Saga::on_event` cannot report a failed write, and the outbox row would be
//! acknowledged regardless; a checkpointed fold holds its position until the write landed.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs — `ProjectionStore`
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/👁️views/🦀️.rs — the view functions

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use quiz::{Event, LearnerState};
use semio_framework_async::CancelToken;
use serde::{Deserialize, Serialize};
use server::storage::{ProjectionStore, StorageError};
use tokio::sync::Mutex;

use crate::actors::LEARNER;
use crate::catalog::LoadedCatalog;
use crate::storage::{ProjectionWrite, SqliteAuthorityStore, SqliteProjectionStore};

/// 🚩️ The checkpoint every quiz read model advances together.
pub const CHECKPOINT: &str = "quiz";
/// 🧠️ Folded learner states.
pub const STATES: &str = "quiz.learner-state";
/// 🙋️ Learner views by learner id.
pub const LEARNERS: &str = "quiz.learner";
/// 🏃️ Run views by run id.
pub const RUNS: &str = "quiz.run";
/// 🏆️ The leaderboard, under [`LEADERBOARD_KEY`].
pub const LEADERBOARD: &str = "quiz.leaderboard";
/// 🔑️ The one key of [`LEADERBOARD`].
pub const LEADERBOARD_KEY: &str = "all";
/// 🧾️ Build metadata.
pub const META: &str = "quiz.meta";
/// 🔑️ The catalog fingerprint key of [`META`].
pub const FINGERPRINT_KEY: &str = "catalog";
/// 📦️ Events folded per transaction.
pub const BATCH: usize = 256;

/// 🧠️ One learner's folded state and the last stream sequence folded into it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Folded {
    pub seq: u64,
    pub state: LearnerState,
}

/// 📶️ How far a fold has come.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub position: u64,
    pub head: u64,
    pub folded: u64,
}

/// 🏁️ How a catch-up ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatchUp {
    Current(Progress),
    Cancelled(Progress),
}

/// 🔭️ Folds committed events into the quiz read models.
pub struct Projector {
    catalog: Arc<LoadedCatalog>,
    mirror: Mutex<Option<BTreeMap<String, Folded>>>,
}

impl Projector {
    /// 🆕️ A projector for one loaded catalog.
    pub fn new(catalog: Arc<LoadedCatalog>) -> Self {
        Self { catalog, mirror: Mutex::new(None) }
    }

    /// 🧾️ Reset the read models when they were built against another catalog; `true` when reset.
    pub async fn prepare(&self, projections: &mut SqliteProjectionStore) -> Result<bool, StorageError> {
        if projections.get(META, FINGERPRINT_KEY).await.as_deref() == Some(self.catalog.fingerprint.as_bytes()) {
            return Ok(false);
        }
        self.reset(projections).await?;
        Ok(true)
    }

    /// 🧹️ Drop every quiz read model and its checkpoint, stamping the current catalog fingerprint,
    /// so the next catch-up rebuilds from the first event.
    pub async fn reset(&self, projections: &mut SqliteProjectionStore) -> Result<(), StorageError> {
        let mut mirror = self.mirror.lock().await;
        for projection in [META, STATES, LEARNERS, RUNS, LEADERBOARD, CHECKPOINT] {
            projections.clear(projection).await?;
        }
        projections.put(META, FINGERPRINT_KEY, self.catalog.fingerprint.as_bytes().to_vec()).await?;
        *mirror = Some(BTreeMap::new());
        Ok(())
    }

    /// ⏩️ Fold every event committed after the checkpoint, batch by batch, reporting progress
    /// after each batch and stopping between batches once `cancel` fires.
    pub async fn catch_up(&self, log: &SqliteAuthorityStore, projections: &mut SqliteProjectionStore, cancel: &CancelToken, mut progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        let mut guard = self.mirror.lock().await;
        if guard.is_none() {
            *guard = Some(loaded(projections).await);
        }
        let mirror = guard.get_or_insert_with(BTreeMap::new);
        let head = log.log_head()?;
        let mut at = Progress { position: projections.checkpoint(CHECKPOINT).await, head, folded: 0 };
        while at.position < head {
            if cancel.is_cancelled_now() {
                return Ok(CatchUp::Cancelled(at));
            }
            let batch = log.log_after(at.position, BATCH)?;
            let Some(last) = batch.last().map(|logged| logged.position) else { break };
            let mut staged: BTreeMap<String, Folded> = BTreeMap::new();
            let mut runs: BTreeSet<(String, String)> = BTreeSet::new();
            for logged in batch.iter().filter(|logged| logged.event.stream.kind == LEARNER) {
                let learner = &logged.event.stream.id;
                let entry = staged.entry(learner.clone()).or_insert_with(|| mirror.get(learner).cloned().unwrap_or_else(|| Folded { seq: 0, state: quiz::empty_learner_state(learner) }));
                if logged.event.seq <= entry.seq {
                    continue;
                }
                entry.seq = logged.event.seq;
                let Ok(fact) = serde_json::from_slice::<Event>(&logged.event.payload) else { continue };
                quiz::evolve_learner(&mut entry.state, &fact);
                if let Some(run) = run_of(&fact) {
                    runs.insert((learner.clone(), run.clone()));
                }
            }
            let writes = self.views(mirror, &staged, &runs)?;
            projections.commit(&writes, (CHECKPOINT, last))?;
            mirror.extend(staged);
            at = Progress { position: last, head, folded: at.folded + batch.len() as u64 };
            progress(at);
        }
        Ok(CatchUp::Current(at))
    }

    fn views(&self, mirror: &BTreeMap<String, Folded>, staged: &BTreeMap<String, Folded>, runs: &BTreeSet<(String, String)>) -> Result<Vec<ProjectionWrite>, StorageError> {
        let mut writes = Vec::new();
        for (learner, folded) in staged {
            writes.push(write(STATES, learner, folded)?);
            if let Some(view) = quiz::learner_view(&folded.state, self.catalog.view()) {
                writes.push(write(LEARNERS, learner, &view)?);
            }
        }
        for (learner, run) in runs {
            if let Some(view) = staged.get(learner).and_then(|folded| quiz::run_view(&folded.state, run, self.catalog.current())) {
                writes.push(write(RUNS, run, &view)?);
            }
        }
        if !staged.is_empty() {
            let states: Vec<&LearnerState> = mirror.iter().filter(|(learner, _)| !staged.contains_key(*learner)).chain(staged.iter()).map(|(_, folded)| &folded.state).collect();
            writes.push(write(LEADERBOARD, LEADERBOARD_KEY, &quiz::leaderboard(states, self.catalog.view()))?);
        }
        Ok(writes)
    }
}

async fn loaded(projections: &SqliteProjectionStore) -> BTreeMap<String, Folded> {
    projections.list(STATES, "").await.into_iter().filter_map(|(learner, bytes)| serde_json::from_slice(&bytes).ok().map(|folded| (learner, folded))).collect()
}

fn write<T: Serialize>(projection: &str, key: &str, value: &T) -> Result<ProjectionWrite, StorageError> {
    let value = serde_json::to_vec(value).map_err(|error| StorageError::Backend(format!("cannot encode {projection}/{key}: {error}")))?;
    Ok(ProjectionWrite { projection: projection.to_string(), key: key.to_string(), value })
}

/// 🏃️ The run a fact is about, if any.
pub fn run_of(fact: &Event) -> Option<&String> {
    match fact {
        Event::RunStarted { run, .. } | Event::RunVoided { run, .. } | Event::AnswerRecorded { run, .. } | Event::RunSubmitted { run, .. } | Event::BadgeAwarded { run, .. } => Some(run),
        Event::LearnerRegistered { .. } | Event::LearnerRecalled { .. } => None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
