//! 🔭️ The proctor's read models, folded from the committed event log.
//!
//! **What is kept.** Per learner the folded `LearnerState` (`quiz.learner-state`, with the last
//! folded stream sequence) and its `LearnerView` (`quiz.learner`); per run its `RunView`
//! (`quiz.run`); one `Leaderboard` (`quiz.leaderboard/all`); per quiz its crowd tally
//! (`quiz.crowd-tally`) and `CrowdView` (`quiz.crowd`, present for every quiz of the catalog from
//! the first fold on); the stamp of the projector revision and catalog the models were built
//! against (`quiz.meta/catalog`). The learner, run and leaderboard views come from the quiz core's
//! view functions — the projector only decides *when* to recompute them; the crowd views are kept
//! incrementally by [`CrowdTally`] and equal the core's `crowd_view`.
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
use crate::crowd::CrowdTally;
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
/// 👪️ Crowd views by quiz id.
pub const CROWDS: &str = "quiz.crowd";
/// 🧮️ Crowd tallies by quiz id.
pub const TALLIES: &str = "quiz.crowd-tally";
/// 🧾️ Build metadata.
pub const META: &str = "quiz.meta";
/// 🔑️ The key of [`META`] stamping the projector revision and catalog fingerprint.
pub const FINGERPRINT_KEY: &str = "catalog";
/// 🧬️ The revision of what the projector folds; raising it rebuilds every store an earlier
/// projector built, exactly like a changed catalog does.
pub const PROJECTOR_REVISION: u32 = 2;
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
    mirror: Mutex<Option<Mirror>>,
}

/// 🪞️ What the projector keeps between batches: every folded learner and every crowd tally.
#[derive(Default)]
struct Mirror {
    learners: BTreeMap<String, Folded>,
    crowds: BTreeMap<String, CrowdTally>,
}

impl Projector {
    /// 🆕️ A projector for one loaded catalog.
    pub fn new(catalog: Arc<LoadedCatalog>) -> Self {
        Self { catalog, mirror: Mutex::new(None) }
    }

    /// 🏷️ The stamp of read models this projector builds for this catalog.
    pub fn stamp(&self) -> Vec<u8> {
        format!("{PROJECTOR_REVISION}:{}", self.catalog.fingerprint).into_bytes()
    }

    /// 🧾️ Set up fresh read models, and reset those another projector revision or catalog built;
    /// `true` exactly when such read models were dropped.
    pub async fn prepare(&self, projections: &mut SqliteProjectionStore) -> Result<bool, StorageError> {
        let stamped = projections.get(META, FINGERPRINT_KEY).await;
        if stamped.as_deref() == Some(self.stamp().as_slice()) {
            return Ok(false);
        }
        self.reset(projections).await?;
        Ok(stamped.is_some())
    }

    /// 🧹️ Drop every quiz read model and its checkpoint, stamp them, and write the empty crowd view
    /// of every quiz, so the next catch-up rebuilds from the first event.
    pub async fn reset(&self, projections: &mut SqliteProjectionStore) -> Result<(), StorageError> {
        let mut mirror = self.mirror.lock().await;
        for projection in [META, STATES, LEARNERS, RUNS, LEADERBOARD, CROWDS, TALLIES, CHECKPOINT] {
            projections.clear(projection).await?;
        }
        let mut writes = vec![ProjectionWrite { projection: META.to_string(), key: FINGERPRINT_KEY.to_string(), value: self.stamp() }];
        for (id, loaded) in self.catalog.current() {
            writes.push(write(CROWDS, id, &CrowdTally::default().view(&loaded.quiz))?);
        }
        projections.commit(&writes, (CHECKPOINT, 0))?;
        *mirror = Some(Mirror::default());
        Ok(())
    }

    /// ⏩️ Fold every event committed after the checkpoint, batch by batch, reporting progress
    /// after each batch and stopping between batches once `cancel` fires.
    pub async fn catch_up(&self, log: &SqliteAuthorityStore, projections: &mut SqliteProjectionStore, cancel: &CancelToken, mut progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        let mut guard = self.mirror.lock().await;
        if guard.is_none() {
            *guard = Some(loaded(projections).await);
        }
        let mirror = guard.get_or_insert_with(Mirror::default);
        let head = log.log_head()?;
        let mut at = Progress { position: projections.checkpoint(CHECKPOINT).await, head, folded: 0 };
        while at.position < head {
            if cancel.is_cancelled_now() {
                return Ok(CatchUp::Cancelled(at));
            }
            let batch = log.log_after(at.position, BATCH)?;
            let Some(last) = batch.last().map(|logged| logged.position) else { break };
            let mut staged: BTreeMap<String, Folded> = BTreeMap::new();
            let mut crowds: BTreeMap<String, CrowdTally> = BTreeMap::new();
            let mut runs: BTreeSet<(String, String)> = BTreeSet::new();
            for logged in batch.iter().filter(|logged| logged.event.stream.kind == LEARNER) {
                let learner = &logged.event.stream.id;
                let entry = staged.entry(learner.clone()).or_insert_with(|| mirror.learners.get(learner).cloned().unwrap_or_else(|| Folded { seq: 0, state: quiz::empty_learner_state(learner) }));
                if logged.event.seq <= entry.seq {
                    continue;
                }
                entry.seq = logged.event.seq;
                let Ok(fact) = serde_json::from_slice::<Event>(&logged.event.payload) else { continue };
                quiz::evolve_learner(&mut entry.state, &fact);
                if let Event::RunSubmitted { result, .. } = &fact {
                    crowds.entry(result.quiz.clone()).or_insert_with(|| mirror.crowds.get(&result.quiz).cloned().unwrap_or_default()).fold(result);
                }
                if let Some(run) = run_of(&fact) {
                    runs.insert((learner.clone(), run.clone()));
                }
            }
            let mut writes = self.views(&mirror.learners, &staged, &runs)?;
            writes.extend(self.crowd_views(&crowds)?);
            projections.commit(&writes, (CHECKPOINT, last))?;
            mirror.learners.extend(staged);
            mirror.crowds.extend(crowds);
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

    fn crowd_views(&self, crowds: &BTreeMap<String, CrowdTally>) -> Result<Vec<ProjectionWrite>, StorageError> {
        let mut writes = Vec::new();
        for (quiz, tally) in crowds {
            writes.push(write(TALLIES, quiz, tally)?);
            if let Some(loaded) = self.catalog.current().get(quiz) {
                writes.push(write(CROWDS, quiz, &tally.view(&loaded.quiz))?);
            }
        }
        Ok(writes)
    }
}

async fn loaded(projections: &SqliteProjectionStore) -> Mirror {
    Mirror { learners: decoded(projections, STATES).await, crowds: decoded(projections, TALLIES).await }
}

async fn decoded<T: serde::de::DeserializeOwned>(projections: &SqliteProjectionStore, projection: &str) -> BTreeMap<String, T> {
    projections.list(projection, "").await.into_iter().filter_map(|(key, bytes)| serde_json::from_slice(&bytes).ok().map(|value| (key, value))).collect()
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
