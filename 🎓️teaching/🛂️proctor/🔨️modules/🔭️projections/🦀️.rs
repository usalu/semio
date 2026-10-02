//! 🔭️ The proctor's read models, folded from the committed event log.
//!
//! **What is kept.** Per learner the folded `LearnerState` (`quiz.learner-state`, with the last
//! folded stream sequence), its `LearnerView` (`quiz.learner`) and — once it submitted a run — its
//! transcript, the submitted runs and badges every leaderboard row of the learner is made of, beside
//! the learner id (`quiz.leaderboard`); per run its
//! `RunView` (`quiz.run`); per handle key its holder (`quiz.handle`), which is what recalling a
//! handle reads; per quiz its crowd tally (`quiz.crowd-tally`) and `CrowdView` (`quiz.crowd`,
//! present for every quiz of the catalog from the first fold on); the number of registrations —
//! every anonymous learner and every claimed handle, which is the number of learners as long as no
//! learner claims a second handle and bounds both kinds of stream whatever a client does — and the
//! stamp of the projector revision and catalog the models were built against (`quiz.meta`).
//! The views come from the quiz core's view functions — the projector only decides *when* to
//! recompute them.
//!
//! **Bounded work per event.** Nothing here walks every learner. A batch loads only the states of
//! the learners it touches (one indexed read each) and writes them back; an answer rewrites its
//! learner's state and run view and nothing else. A learner view is rewritten only by the facts
//! that can change it, and a transcript only by a registration, a submission or a badge.
//!
//! **The leaderboards** are the [`Board`]: every transcript, kept in memory next to the
//! `quiz.leaderboard` rows it is loaded from once, and one rank index per leaderboard that was asked
//! for — a period (daily, weekly, monthly, all-time) of every quiz or of one. An index holds what
//! orders a learner and nothing else (32 bytes per learner), for the window its period had at the
//! instant it was built for. A changed transcript moves one entry per index (`O(log n)` to find, one
//! shift to place); a query takes the top rows, the count and — by one binary search — the caller's
//! own rank, and makes those rows from their transcripts. A query whose instant lies in another
//! window — the next day, week or month — builds that index anew from the transcripts, once. Every
//! answer equals the core's `leaderboard` over every transcript at that instant.
//!
//! **How it folds.** The projector walks the database-wide commit order after its checkpoint
//! (`quiz`) in batches and writes every touched view plus the advanced checkpoint in one
//! transaction, so a batch is visible entirely or not at all. A learner's facts are folded only
//! past the sequence already folded, which makes a replayed batch harmless. The fold is cancellable
//! between batches and reports progress; a cancelled fold leaves a consistent prefix whose
//! checkpoint the next catch-up resumes from.
//!
//! **Nothing is skipped.** An event of a stream kind this proctor does not know, an event that does
//! not decode, a handle registration under another key than its stream's, a stored state or
//! transcript that does not decode: each stops the fold with an error naming the position, the stream
//! and the cause. A proctor does not boot over a log it cannot read, and `rebuild` fails the same
//! way; the checkpoint stays before the offending event.
//!
//! **Why not a saga.** `Saga::on_event` cannot report a failed write, and the outbox row would be
//! acknowledged regardless; a checkpointed fold holds its position until the write landed.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs — `ProjectionStore`
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/👁️views/🦀️.rs — the view functions

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, PoisonError, RwLock};

use quiz::{BoardScope, CatalogView, Event, HandleHolder, Identity, Leaderboard, LeaderboardPeriod, LeaderboardRow, LearnerState, Merit, Timestamp, Transcript, LEADERBOARD_TOP};
use semio_framework_async::CancelToken;
use serde::{Deserialize, Serialize};
use server::storage::{ProjectionStore, StorageError};
use tokio::sync::Mutex;

use crate::actors::{HANDLE, LEARNER};
use crate::catalog::LoadedCatalog;
use crate::crowd::CrowdTally;
use crate::storage::{LoggedEvent, ProjectionWrite, SqliteAuthorityStore, SqliteProjectionStore};

/// 🚩️ The checkpoint every quiz read model advances together.
pub const CHECKPOINT: &str = "quiz";
/// 🧠️ Folded learner states by learner id.
pub const STATES: &str = "quiz.learner-state";
/// 🙋️ Learner views by learner id.
pub const LEARNERS: &str = "quiz.learner";
/// 🏃️ Run views by run id.
pub const RUNS: &str = "quiz.run";
/// 🏆️ Transcripts — the submitted runs and badges of a ranked learner beside its id — by learner id.
pub const LEADERBOARD: &str = "quiz.leaderboard";
/// ✒️ Handle holders by handle key.
pub const HANDLES: &str = "quiz.handle";
/// 👪️ Crowd views by quiz id.
pub const CROWDS: &str = "quiz.crowd";
/// 🧮️ Crowd tallies by quiz id.
pub const TALLIES: &str = "quiz.crowd-tally";
/// 🧾️ Build metadata.
pub const META: &str = "quiz.meta";
/// 🔑️ The key of [`META`] stamping the projector revision and catalog fingerprint.
pub const FINGERPRINT_KEY: &str = "catalog";
/// 🔢️ The key of [`META`] counting the registrations: anonymous learners and claimed handles.
pub const LEARNER_COUNT_KEY: &str = "learners";
/// 🧬️ The revision of what the projector folds; raising it rebuilds every store an earlier
/// projector built, exactly like a changed catalog does.
pub const PROJECTOR_REVISION: u32 = 5;
/// 📦️ Events folded per transaction.
pub const BATCH: usize = 256;

/// 🧳️ One learner's folded state and the last stream sequence folded into it.
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

//#region 🔖️Board
/// 🥇️ The leaderboards of one catalog: the transcript of every ranked learner and a rank index per
/// leaderboard that was asked for. Shared by the projector, which moves one entry per index for
/// every fact that changes a transcript, and the leaderboard read, which never touches a row beyond
/// the top and the caller's own.
pub struct Board {
    catalog: Arc<LoadedCatalog>,
    ledger: RwLock<Ledger>,
}

/// 📒️ What the board holds: the transcripts by learner id, how many runs they submitted in all, and
/// the rank indexes by period and quiz.
#[derive(Default)]
struct Ledger {
    loaded: bool,
    transcripts: HashMap<String, Arc<Transcript>>,
    submissions: usize,
    rankings: HashMap<(LeaderboardPeriod, Option<String>), Ranking>,
}

/// 🪜️ One leaderboard as a rank index: the learners with a run in `scope` in the order of the
/// core's `compare_standings`.
struct Ranking {
    scope: BoardScope,
    order: Vec<Ranked>,
}

/// 🎽️ One learner in a rank index: what orders it there, and the transcript its row is made from.
struct Ranked {
    merit: Merit,
    transcript: Arc<Transcript>,
}

impl Ranking {
    fn of<'a>(scope: BoardScope, transcripts: impl Iterator<Item = &'a Arc<Transcript>>, catalog: &CatalogView) -> Self {
        let mut order: Vec<Ranked> = transcripts.filter_map(|transcript| Some(Ranked { merit: quiz::standing(transcript, catalog, &scope)?.merit(), transcript: Arc::clone(transcript) })).collect();
        order.sort_by(|left, right| left.merit.compare(&left.transcript.learner, &right.merit, &right.transcript.learner));
        Self { scope, order }
    }

    fn merit(&self, transcript: &Transcript, catalog: &CatalogView) -> Option<Merit> {
        quiz::standing(transcript, catalog, &self.scope).map(|standing| standing.merit())
    }

    fn place(&self, merit: &Merit, learner: &str) -> Result<usize, usize> {
        self.order.binary_search_by(|ranked| ranked.merit.compare(&ranked.transcript.learner, merit, learner))
    }

    fn replace(&mut self, previous: Option<&Transcript>, next: &Arc<Transcript>, catalog: &CatalogView) {
        if let Some(position) = previous.and_then(|previous| self.merit(previous, catalog)).and_then(|merit| self.place(&merit, &next.learner).ok()) {
            self.order.remove(position);
        }
        if let Some(merit) = self.merit(next, catalog) {
            let position = self.place(&merit, &next.learner).unwrap_or_else(|free| free);
            self.order.insert(position, Ranked { merit, transcript: Arc::clone(next) });
        }
    }

    fn row(&self, position: usize, catalog: &CatalogView) -> Option<LeaderboardRow> {
        quiz::standing(&self.order.get(position)?.transcript, catalog, &self.scope).map(|standing| standing.row(position + 1))
    }
}

impl Ledger {
    fn answer(&self, period: LeaderboardPeriod, ranking: &Ranking, caller: Option<&str>, catalog: &CatalogView) -> Leaderboard {
        let own = caller.and_then(|caller| self.transcripts.get(caller)).and_then(|transcript| ranking.place(&ranking.merit(transcript, catalog)?, &transcript.learner).ok()).and_then(|position| ranking.row(position, catalog));
        Leaderboard {
            period,
            quiz: ranking.scope.quiz.clone(),
            window: ranking.scope.window,
            rows: (0..ranking.order.len().min(LEADERBOARD_TOP)).filter_map(|position| ranking.row(position, catalog)).collect(),
            learners: ranking.order.len(),
            submissions: self.submissions,
            own,
        }
    }
}

impl Board {
    /// 🆕️ The empty, unloaded board of `catalog`.
    pub fn new(catalog: Arc<LoadedCatalog>) -> Self {
        Self { catalog, ledger: RwLock::default() }
    }

    /// 📥️ Whether the transcripts were loaded since the last [`clear`](Self::clear).
    pub fn loaded(&self) -> bool {
        self.ledger.read().unwrap_or_else(PoisonError::into_inner).loaded
    }

    /// 🧹️ Forget every transcript and rank index; the next reader loads the transcripts again.
    pub fn clear(&self) {
        *self.ledger.write().unwrap_or_else(PoisonError::into_inner) = Ledger::default();
    }

    /// 🗃️ Take `transcripts` as the whole board; its rank indexes are built when they are asked for.
    pub fn fill(&self, transcripts: Vec<Transcript>) {
        let submissions = transcripts.iter().map(|transcript| transcript.runs.len()).sum();
        let transcripts = transcripts.into_iter().map(|transcript| (transcript.learner.clone(), Arc::new(transcript))).collect();
        *self.ledger.write().unwrap_or_else(PoisonError::into_inner) = Ledger { loaded: true, transcripts, submissions, rankings: HashMap::new() };
    }

    /// 📚️ Fill the board from the stored transcripts unless it is loaded; a stored transcript that
    /// does not decode is an error.
    pub async fn load<P: ProjectionStore>(&self, projections: &P) -> Result<(), StorageError> {
        if self.loaded() {
            return Ok(());
        }
        let stored = projections.list(LEADERBOARD, "").await;
        let transcripts = stored.iter().map(|(key, bytes)| decoded::<Transcript>(LEADERBOARD, key, bytes)).collect::<Result<Vec<_>, _>>()?;
        if !self.loaded() {
            self.fill(transcripts);
        }
        Ok(())
    }

    /// 🔀️ Take a new or changed transcript and place its learner where it now ranks in every index.
    pub fn set(&self, transcript: Transcript) {
        let mut ledger = self.ledger.write().unwrap_or_else(PoisonError::into_inner);
        let next = Arc::new(transcript);
        let previous = ledger.transcripts.insert(next.learner.clone(), Arc::clone(&next));
        ledger.submissions = ledger.submissions - previous.as_ref().map_or(0, |previous| previous.runs.len()) + next.runs.len();
        for ranking in ledger.rankings.values_mut() {
            ranking.replace(previous.as_deref(), &next, self.catalog.view());
        }
    }

    /// 📜️ The transcript of one learner, if it is ranked.
    pub fn transcript(&self, learner: &str) -> Option<Arc<Transcript>> {
        self.ledger.read().unwrap_or_else(PoisonError::into_inner).transcripts.get(learner).cloned()
    }

    /// 👥️ How many learners are ranked on the all-time leaderboard.
    pub fn learners(&self) -> usize {
        self.ledger.read().unwrap_or_else(PoisonError::into_inner).transcripts.len()
    }

    /// 🏟️ The leaderboard of `period` — of `quiz` only when it names one — at the instant `at`, as
    /// `caller` is answered: the top rows, the count of ranked learners, the count of submissions
    /// and the caller's own row when the caller is ranked; `None` for a quiz the catalog does not
    /// list. The rank index is built on the first query for it and whenever `at` lies in another
    /// window than the one it was built for.
    pub fn view(&self, period: LeaderboardPeriod, quiz: Option<&str>, at: Timestamp, caller: Option<&str>) -> Option<Leaderboard> {
        if quiz.is_some_and(|quiz| !self.catalog.current().contains_key(quiz)) {
            return None;
        }
        let catalog = self.catalog.view();
        let scope = BoardScope::of(period, quiz, at);
        let key = (period, scope.quiz.clone());
        {
            let ledger = self.ledger.read().unwrap_or_else(PoisonError::into_inner);
            if let Some(ranking) = ledger.rankings.get(&key).filter(|ranking| ranking.scope == scope) {
                return Some(ledger.answer(period, ranking, caller, catalog));
            }
        }
        let mut ledger = self.ledger.write().unwrap_or_else(PoisonError::into_inner);
        if ledger.rankings.get(&key).is_none_or(|ranking| ranking.scope != scope) {
            let ranking = Ranking::of(scope, ledger.transcripts.values(), catalog);
            ledger.rankings.insert(key.clone(), ranking);
        }
        ledger.rankings.get(&key).map(|ranking| ledger.answer(period, ranking, caller, catalog))
    }
}
//#endregion 🔖️Board

//#region 🔖️Projector
/// 🛰️ Folds committed events into the quiz read models.
pub struct Projector {
    catalog: Arc<LoadedCatalog>,
    board: Arc<Board>,
    learners: Arc<AtomicU64>,
    mirror: Mutex<Option<Mirror>>,
}

/// 🪞️ What the projector keeps between batches: every crowd tally and the learner count.
#[derive(Default)]
struct Mirror {
    crowds: BTreeMap<String, CrowdTally>,
    learners: u64,
}

/// 🧺️ What one batch touches before it is written.
#[derive(Default)]
struct Fold {
    learners: BTreeMap<String, Folded>,
    viewed: BTreeSet<String>,
    ranked: BTreeSet<String>,
    runs: BTreeSet<(String, String)>,
    crowds: BTreeMap<String, CrowdTally>,
    handles: BTreeMap<String, HandleHolder>,
    registered: u64,
}

impl Projector {
    /// 🆕️ A projector for one loaded catalog, with an empty board and a learner count of zero.
    pub fn new(catalog: Arc<LoadedCatalog>) -> Self {
        Self { board: Arc::new(Board::new(Arc::clone(&catalog))), catalog, learners: Arc::new(AtomicU64::new(0)), mirror: Mutex::new(None) }
    }

    /// 🏅️ The leaderboards this projector keeps.
    pub fn board(&self) -> Arc<Board> {
        Arc::clone(&self.board)
    }

    /// 🌡️ The gauge of registrations — anonymous learners and claimed handles — this projector
    /// keeps current; the learner cap is held against it.
    pub fn learners(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.learners)
    }

    /// 🏷️ The stamp of read models this projector builds for this catalog.
    pub fn stamp(&self) -> Vec<u8> {
        format!("{PROJECTOR_REVISION}:{}", self.catalog.fingerprint).into_bytes()
    }

    /// 🧰️ Set up fresh read models, and reset those another projector revision or catalog built;
    /// `true` exactly when such read models were dropped.
    pub async fn prepare(&self, projections: &mut SqliteProjectionStore) -> Result<bool, StorageError> {
        let stamped = projections.get(META, FINGERPRINT_KEY).await;
        if stamped.as_deref() == Some(self.stamp().as_slice()) {
            return Ok(false);
        }
        self.reset(projections).await?;
        Ok(stamped.is_some())
    }

    /// 🧽️ Drop every quiz read model and its checkpoint, stamp them, and write the empty crowd view
    /// of every quiz, so the next catch-up rebuilds from the first event.
    pub async fn reset(&self, projections: &mut SqliteProjectionStore) -> Result<(), StorageError> {
        let mut mirror = self.mirror.lock().await;
        for projection in [META, STATES, LEARNERS, RUNS, LEADERBOARD, HANDLES, CROWDS, TALLIES, CHECKPOINT] {
            projections.clear(projection).await?;
        }
        let mut writes = vec![ProjectionWrite { projection: META.to_string(), key: FINGERPRINT_KEY.to_string(), value: self.stamp() }];
        for (id, loaded) in self.catalog.current() {
            writes.push(write(CROWDS, id, &CrowdTally::default().view(&loaded.quiz))?);
        }
        projections.commit(&writes, (CHECKPOINT, 0))?;
        self.board.fill(Vec::new());
        self.learners.store(0, Ordering::Release);
        *mirror = Some(Mirror::default());
        Ok(())
    }

    /// ⏩️ Fold every event committed after the checkpoint, batch by batch, reporting progress
    /// after each batch and stopping between batches once `cancel` fires.
    pub async fn catch_up(&self, log: &SqliteAuthorityStore, projections: &mut SqliteProjectionStore, cancel: &CancelToken, mut progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        let mut guard = self.mirror.lock().await;
        if guard.is_none() {
            *guard = Some(self.remembered(projections).await?);
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
            let mut fold = Fold::default();
            for logged in &batch {
                match logged.event.stream.kind.as_str() {
                    LEARNER => self.learner(projections, mirror, &mut fold, logged).await?,
                    HANDLE => claim(projections, &mut fold, logged).await?,
                    other => return Err(unreadable(logged, format!("the stream kind {other:?} is none this proctor folds"))),
                }
            }
            let (writes, transcripts) = self.written(&fold, mirror.learners)?;
            projections.commit(&writes, (CHECKPOINT, last))?;
            transcripts.into_iter().for_each(|transcript| self.board.set(transcript));
            mirror.crowds.extend(fold.crowds);
            mirror.learners += fold.registered;
            self.learners.store(mirror.learners, Ordering::Release);
            at = Progress { position: last, head, folded: at.folded + batch.len() as u64 };
            progress(at);
        }
        Ok(CatchUp::Current(at))
    }

    async fn remembered(&self, projections: &SqliteProjectionStore) -> Result<Mirror, StorageError> {
        self.board.load(projections).await?;
        let crowds = projections.list(TALLIES, "").await.into_iter().map(|(quiz, bytes)| decoded::<CrowdTally>(TALLIES, &quiz, &bytes).map(|tally| (quiz, tally))).collect::<Result<_, _>>()?;
        let learners = match projections.get(META, LEARNER_COUNT_KEY).await {
            Some(bytes) => decoded::<u64>(META, LEARNER_COUNT_KEY, &bytes)?,
            None => 0,
        };
        self.learners.store(learners, Ordering::Release);
        Ok(Mirror { crowds, learners })
    }

    async fn learner(&self, projections: &SqliteProjectionStore, mirror: &Mirror, fold: &mut Fold, logged: &LoggedEvent) -> Result<(), StorageError> {
        let learner = &logged.event.stream.id;
        if !fold.learners.contains_key(learner) {
            let folded = match projections.get(STATES, learner).await {
                Some(bytes) => decoded::<Folded>(STATES, learner, &bytes)?,
                None => Folded { seq: 0, state: quiz::empty_learner_state(learner) },
            };
            fold.learners.insert(learner.clone(), folded);
        }
        let Some(entry) = fold.learners.get_mut(learner) else { return Ok(()) };
        if logged.event.seq <= entry.seq {
            return Ok(());
        }
        let fact: Event = serde_json::from_slice(&logged.event.payload).map_err(|error| unreadable(logged, error))?;
        if *fact.learner() != *learner {
            return Err(unreadable(logged, format!("it is a fact of learner {}", fact.learner())));
        }
        entry.seq = logged.event.seq;
        let known = entry.state.identity.is_some();
        quiz::evolve_learner(&mut entry.state, &fact);
        match &fact {
            Event::LearnerRegistered { identity, .. } => {
                fold.registered += u64::from(!known && *identity == Identity::Anonymous);
                fold.ranked.insert(learner.clone());
            }
            Event::RunSubmitted { result, .. } => {
                fold.crowds.entry(result.quiz.clone()).or_insert_with(|| mirror.crowds.get(&result.quiz).cloned().unwrap_or_default()).fold(result);
                fold.ranked.insert(learner.clone());
            }
            Event::BadgeAwarded { .. } => {
                fold.ranked.insert(learner.clone());
            }
            Event::RunStarted { .. } | Event::RunVoided { .. } | Event::AnswerRecorded { .. } => {}
        }
        if !matches!(fact, Event::AnswerRecorded { .. }) {
            fold.viewed.insert(learner.clone());
        }
        if let Some(run) = run_of(&fact) {
            fold.runs.insert((learner.clone(), run.clone()));
        }
        Ok(())
    }

    fn written(&self, fold: &Fold, learners: u64) -> Result<(Vec<ProjectionWrite>, Vec<Transcript>), StorageError> {
        let mut writes = Vec::new();
        for (learner, folded) in &fold.learners {
            writes.push(write(STATES, learner, folded)?);
        }
        for learner in &fold.viewed {
            if let Some(view) = fold.learners.get(learner).and_then(|folded| quiz::learner_view(&folded.state, self.catalog.view())) {
                writes.push(write(LEARNERS, learner, &view)?);
            }
        }
        for (learner, run) in &fold.runs {
            if let Some(view) = fold.learners.get(learner).and_then(|folded| quiz::run_view(&folded.state, run, self.catalog.current())) {
                writes.push(write(RUNS, run, &view)?);
            }
        }
        let mut transcripts = Vec::new();
        for learner in &fold.ranked {
            let Some(transcript) = fold.learners.get(learner).and_then(|folded| quiz::transcript(&folded.state)) else { continue };
            if self.board.transcript(learner).as_deref() != Some(&transcript) {
                writes.push(write(LEADERBOARD, learner, &transcript)?);
                transcripts.push(transcript);
            }
        }
        for (key, holder) in &fold.handles {
            writes.push(write(HANDLES, key, holder)?);
        }
        if fold.registered > 0 {
            writes.push(write(META, LEARNER_COUNT_KEY, &(learners + fold.registered))?);
        }
        for (quiz, tally) in &fold.crowds {
            writes.push(write(TALLIES, quiz, tally)?);
            if let Some(loaded) = self.catalog.current().get(quiz) {
                writes.push(write(CROWDS, quiz, &tally.view(&loaded.quiz))?);
            }
        }
        Ok((writes, transcripts))
    }
}

/// 🖋️ Fold the one fact of a handle stream: the learner it registered holds the handle key, and the
/// claim counts as one registration — once, however often the fact is folded.
async fn claim(projections: &SqliteProjectionStore, fold: &mut Fold, logged: &LoggedEvent) -> Result<(), StorageError> {
    let fact: Event = serde_json::from_slice(&logged.event.payload).map_err(|error| unreadable(logged, error))?;
    let Event::LearnerRegistered { learner, identity: identity @ (Identity::Pseudonym { handle } | Identity::Name { handle }), .. } = &fact else {
        return Err(unreadable(logged, format!("a handle stream holds registrations under a handle, not {}", fact.type_name())));
    };
    let key = quiz::normalize_handle(handle).filter(|normalized| quiz::handle_actor_id(&normalized.key) == logged.event.stream.id).map(|normalized| normalized.key);
    let key = key.ok_or_else(|| unreadable(logged, "the registered handle is not the one this stream is named after"))?;
    if !fold.handles.contains_key(&key) && projections.get(HANDLES, &key).await.is_none() {
        fold.handles.insert(key, HandleHolder { learner: learner.clone(), identity: identity.clone() });
        fold.registered += 1;
    }
    Ok(())
}

/// 🚨️ The error of an event the fold cannot read: where it is, what it is and why.
fn unreadable(logged: &LoggedEvent, cause: impl std::fmt::Display) -> StorageError {
    let event = &logged.event;
    StorageError::Backend(format!("event at position {} ({}/{}/{} seq {}, {}) cannot be folded: {cause}; the log was written by another proctor version or is damaged", logged.position, event.stream.tenant.0, event.stream.kind, event.stream.id, event.seq, event.kind))
}

/// 📖️ A stored projection value, or an error naming it.
pub fn decoded<T: serde::de::DeserializeOwned>(projection: &str, key: &str, bytes: &[u8]) -> Result<T, StorageError> {
    serde_json::from_slice(bytes).map_err(|error| StorageError::Backend(format!("projection {projection}/{key} does not decode: {error}; rebuild the read models (`proctor rebuild`)")))
}

fn write<T: Serialize>(projection: &str, key: &str, value: &T) -> Result<ProjectionWrite, StorageError> {
    let value = serde_json::to_vec(value).map_err(|error| StorageError::Backend(format!("cannot encode {projection}/{key}: {error}")))?;
    Ok(ProjectionWrite { projection: projection.to_string(), key: key.to_string(), value })
}

/// 🎽️ The run a fact is about, if any.
pub fn run_of(fact: &Event) -> Option<&String> {
    match fact {
        Event::RunStarted { run, .. } | Event::RunVoided { run, .. } | Event::AnswerRecorded { run, .. } | Event::RunSubmitted { run, .. } | Event::BadgeAwarded { run, .. } => Some(run),
        Event::LearnerRegistered { .. } => None,
    }
}
//#endregion 🔖️Projector

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
