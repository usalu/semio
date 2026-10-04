//! 👁️ The read models a proctor answers (`CatalogView`, `LearnerView`, `RunView`, `Leaderboard`,
//! `CrowdView`), derived from the catalog, the learner states and the submitted results.
//!
//! A best is the submitted run of a quiz with the most points whatever its challenge (a later run
//! replaces it only with strictly more); totals sum the points of the bests over the catalog quizzes
//! in catalog order; `reachedAt` is the submission time of the last submission (in submission-time
//! order, ties in start order) that set or raised a best; `lastActivity` is the latest submission
//! time. A run view carries the sheet at the run's challenge, on a timed run when each opened task was
//! opened, and while the run is open at a challenge that hints the hints of its answers. The crowd
//! mixes every challenge and tallies a guessed value under the nearest authored one.
//!
//! A leaderboard has a scope ([`BoardScope`]): the window of its period around an instant
//! ([`period_window`]: the day, the ISO week or the month in UTC, none for all-time) and, when it
//! names one, a single quiz. Every [`Standing`] of a learner is made of the runs in scope of its
//! [`Transcript`] — the submitted runs and the badges, nothing else of the learner state — so a
//! proctor keeps one transcript per ranked learner and every board incrementally: only a submission,
//! a badge or a registration changes a transcript.
//!
//! @see ../../🧬️schema/🔣️.json — the view contracts
//! @see ../⛰️challenge/🦀️.rs — the rules of a challenge and the hints
//! @see ../👁️views/🟦️.ts — the TypeScript twin
//! @see <https://howardhinnant.github.io/date_algorithms.html#civil_from_days> — the calendar arithmetic

use crate::challenge::{challenge_rules, hints_of};
use crate::lifecycle::{LearnerState, LoadedQuiz};
use crate::randomness::fnv1a32;
use crate::schema::{
    Answer, Best, Catalog, CatalogBadgeView, CatalogQuizView, CatalogTaskView, CatalogView, Challenge, CrowdCount, CrowdItem, CrowdScores, CrowdTask, CrowdView, Dimension, DimensionResult, Hint, Id, Identity, Leaderboard, LeaderboardPeriod, LeaderboardRow, LeaderboardWindow, LearnerView, MatchingItemResult,
    MatchingTask, Quiz, RunResult, RunStatus, RunSummary, RunView, Scale, Score, Sheet, Slug, SortingItemResult, Task, TaskKind, TaskResult, Timestamp, CROWD_SCORE_BINS, LEADERBOARD_TOP,
};
use crate::scoring::scaled;
use crate::sheet::sheet_of;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// 🪟️ The solution-free catalog: quizzes (in the order given, i.e. catalog order) with their task
/// titles and kinds, and badges without rules.
pub fn catalog_view(catalog: &Catalog, quizzes: &[Quiz]) -> CatalogView {
    CatalogView {
        id: catalog.id.clone(),
        title: catalog.title.clone(),
        introduction: catalog.introduction.clone(),
        quizzes: quizzes
            .iter()
            .map(|quiz| CatalogQuizView { id: quiz.id.clone(), emoji: quiz.emoji.clone(), title: quiz.title.clone(), description: quiz.description.clone(), tasks: quiz.tasks.iter().map(|task| CatalogTaskView { id: task.id().clone(), kind: task.kind(), title: task.title().clone(), icon: task.icon().cloned() }).collect() })
            .collect(),
        badges: catalog.badges.iter().map(|badge| CatalogBadgeView { id: badge.id.clone(), emoji: badge.emoji.clone(), label: badge.label.clone(), description: badge.description.clone() }).collect(),
    }
}

/// 🧑‍🏫️ A registered learner's runs (newest first, each with its challenge and, once submitted, its
/// score and points), badges, bests and total; `None` before registration.
pub fn learner_view(state: &LearnerState, catalog: &CatalogView) -> Option<LearnerView> {
    let (best, _) = bests(&submissions(state));
    Some(LearnerView {
        learner: state.learner.clone(),
        identity: state.identity.clone()?,
        runs: state
            .runs
            .iter()
            .rev()
            .map(|run| RunSummary { run: run.run.clone(), quiz: run.quiz.clone(), challenge: run.challenge, status: run.status, started_at: run.started_at, score: run.result.as_ref().map(|result| result.score), points: run.result.as_ref().map(|result| result.points), submitted_at: run.submitted_at })
            .collect(),
        badges: state.badges.clone(),
        total: total(&best, catalog),
        best,
    })
}

/// 🔦️ The hints of a run per task id, only for tasks that have any: what each recorded answer earns
/// from [`hints_of`].
fn run_hints(quiz: &Quiz, sheet: &Sheet, answers: &BTreeMap<Slug, Answer>) -> BTreeMap<Slug, Vec<Hint>> {
    sheet
        .tasks
        .iter()
        .filter_map(|sheet_task| {
            let task = quiz.tasks.iter().find(|candidate| candidate.id() == sheet_task.id())?;
            let hints = hints_of(task, sheet_task, Some(answers.get(sheet_task.id())?));
            (!hints.is_empty()).then(|| (sheet_task.id().clone(), hints))
        })
        .collect()
}

/// 🔬️ One run with its sheet (recomputed from the current quiz, the run seed and the run's
/// challenge), answers and result; on a timed run when each opened task was opened, and while the run
/// is open at a challenge that hints the hints of its answers, when there are any; `None` when the run
/// or its quiz is unknown.
pub fn run_view(state: &LearnerState, run: &str, quizzes: &BTreeMap<Slug, LoadedQuiz>) -> Option<RunView> {
    let found = state.run(run)?;
    let current = quizzes.get(&found.quiz)?;
    let rules = challenge_rules(found.challenge);
    let sheet = sheet_of(&current.quiz, found.seed, found.challenge);
    let hints = if found.status == RunStatus::Open && rules.hints { run_hints(&current.quiz, &sheet, &found.answers) } else { BTreeMap::new() };
    Some(RunView {
        run: found.run.clone(),
        learner: state.learner.clone(),
        quiz: found.quiz.clone(),
        status: found.status,
        sheet,
        answers: found.answers.clone(),
        result: found.result.clone(),
        started_at: found.started_at,
        submitted_at: found.submitted_at,
        opened: rules.timed.then(|| found.opened.clone()),
        hints: (!hints.is_empty()).then_some(hints),
    })
}

/// 🎫️ The public tag of a learner: FNV-1a of the learner id as 8 lowercase hex digits. A client
/// recognises its own leaderboard row by computing it from its own id.
pub fn learner_tag(learner: &str) -> String {
    format!("{:08x}", fnv1a32(learner))
}

/// 📨️ One submitted run as a standing counts it: its quiz, its challenge, its score, the points it
/// earned and when it was submitted.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptRun {
    pub quiz: Slug,
    pub challenge: Challenge,
    pub score: Score,
    pub points: f64,
    pub at: Timestamp,
}

/// 🎗️ One badge as a standing counts it: the quiz of the run that earned it and when.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptBadge {
    pub badge: Slug,
    pub quiz: Slug,
    pub at: Timestamp,
}

/// 📜️ What every standing of a learner is made of: the tag and identity, the submitted runs in
/// submission order and the badges in award order. It carries the learner id, which breaks the last
/// tie and finds the caller and never leaves the proctor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transcript {
    pub learner: Id,
    pub tag: String,
    pub identity: Identity,
    pub runs: Vec<TranscriptRun>,
    pub badges: Vec<TranscriptBadge>,
}

/// 📜️ The transcript of a registered learner with a submitted run, `None` otherwise; it changes only
/// when the learner submits a run, earns a badge or registers. A badge whose run is unknown is left
/// out.
pub fn transcript(state: &LearnerState) -> Option<Transcript> {
    let identity = state.identity.clone()?;
    let runs = submissions(state);
    let badges = state.badges.iter().filter_map(|award| Some(TranscriptBadge { badge: award.badge.clone(), quiz: state.run(&award.run)?.quiz.clone(), at: award.at })).collect();
    (!runs.is_empty()).then(|| Transcript { learner: state.learner.clone(), tag: learner_tag(&state.learner), identity, runs, badges })
}

const DAY: u64 = 86_400_000;

/// 📆️ The day (days since the Unix epoch) on which the month of `day` begins and the one on which the
/// next month does, in the proleptic Gregorian calendar: the years are counted from March inside
/// their 400-year era, so the leap day is the last of a year.
fn month_of(day: u64) -> (u64, u64) {
    let day_of_era = day.saturating_add(719_468) % 146_097;
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month = (5 * day_of_year + 2) / 153;
    let leap = year_of_era % 4 == 3 && (year_of_era % 100 != 99 || year_of_era == 399);
    let begins = |index: u64| if index == 12 { 365 + u64::from(leap) } else { (153 * index + 2) / 5 };
    let from = day - (day_of_year - begins(month));
    (from, from.saturating_add(begins(month + 1) - begins(month)))
}

/// 🪟️ The window of `period` that contains the instant `at`: its day, its ISO week (from Monday) or
/// its month in UTC — half-open, never starting before the epoch — and none for all-time.
pub fn period_window(period: LeaderboardPeriod, at: Timestamp) -> Option<LeaderboardWindow> {
    let day = at / DAY;
    let (from, until) = match period {
        LeaderboardPeriod::Daily => (day, day.saturating_add(1)),
        LeaderboardPeriod::Weekly => {
            let until = day.saturating_add(7 - (day % 7 + 3) % 7);
            (until.saturating_sub(7), until)
        }
        LeaderboardPeriod::Monthly => month_of(day),
        LeaderboardPeriod::AllTime => return None,
    };
    Some(LeaderboardWindow { from: from.saturating_mul(DAY), until: until.saturating_mul(DAY) })
}

/// 🔭️ Which runs a leaderboard counts: those submitted inside `window` (every run without one), of
/// `quiz` only when it names one.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BoardScope {
    pub window: Option<LeaderboardWindow>,
    pub quiz: Option<Slug>,
}

impl BoardScope {
    /// 🔭️ The scope of the leaderboard of `period` — of `quiz` only when it names one — at the
    /// instant `at`.
    pub fn of(period: LeaderboardPeriod, quiz: Option<&str>, at: Timestamp) -> Self {
        Self { window: period_window(period, at), quiz: quiz.map(str::to_string) }
    }

    fn counts(&self, quiz: &str, at: Timestamp) -> bool {
        self.quiz.as_deref().is_none_or(|only| only == quiz) && self.window.is_none_or(|window| window.from <= at && at < window.until)
    }
}

/// 🎚️ What orders a standing, beside the learner id that breaks the last tie: a rank index keeps
/// this much per learner and makes the rows it answers from the transcripts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Merit {
    pub total: f64,
    pub badges: usize,
    pub reached_at: Timestamp,
}

impl Merit {
    /// 🥈️ The order of the leaderboard between this merit of `learner` and `other` of `rival`:
    /// total descending, badge count descending, `reachedAt` ascending, learner id ascending.
    pub fn compare(&self, learner: &str, other: &Merit, rival: &str) -> Ordering {
        other.total.partial_cmp(&self.total).unwrap_or(Ordering::Equal).then(other.badges.cmp(&self.badges)).then(self.reached_at.cmp(&other.reached_at)).then_with(|| learner.cmp(rival))
    }
}

/// 🧍️ One learner's unranked leaderboard row together with the learner id that breaks the last tie and
/// finds the caller; the id never leaves the proctor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Standing {
    pub learner: Id,
    pub tag: String,
    pub identity: Identity,
    pub total: f64,
    pub reached_at: Timestamp,
    pub best: BTreeMap<Slug, Best>,
    pub badges: Vec<Slug>,
    pub runs: usize,
    pub last_activity: Timestamp,
}

impl Standing {
    /// 🎖️ The public row of this standing at `rank`: everything but the learner id.
    pub fn row(&self, rank: usize) -> LeaderboardRow {
        LeaderboardRow { rank, tag: self.tag.clone(), identity: self.identity.clone(), total: self.total, reached_at: self.reached_at, best: self.best.clone(), badges: self.badges.clone(), runs: self.runs, last_activity: self.last_activity }
    }

    /// 🎚️ What orders this standing.
    pub fn merit(&self) -> Merit {
        Merit { total: self.total, badges: self.badges.len(), reached_at: self.reached_at }
    }
}

/// 📶️ The standing of a transcript in `scope`, made of the runs in scope only — their bests, their
/// count, the last of them and the badges they earned; `None` when no run is in scope.
pub fn standing(transcript: &Transcript, catalog: &CatalogView, scope: &BoardScope) -> Option<Standing> {
    let runs: Vec<&TranscriptRun> = transcript.runs.iter().filter(|run| scope.counts(&run.quiz, run.at)).collect();
    let last_activity = runs.last()?.at;
    let (best, reached_at) = bests(&runs);
    let badges = transcript.badges.iter().filter(|award| scope.counts(&award.quiz, award.at)).map(|award| award.badge.clone()).collect();
    Some(Standing { learner: transcript.learner.clone(), tag: transcript.tag.clone(), identity: transcript.identity.clone(), total: total(&best, catalog), reached_at: reached_at?, best, badges, runs: runs.len(), last_activity })
}

/// 🥈️ The order of the leaderboard: total descending, badge count descending, `reachedAt` ascending,
/// learner id ascending.
pub fn compare_standings(left: &Standing, right: &Standing) -> Ordering {
    left.merit().compare(&left.learner, &right.merit(), &right.learner)
}

/// 🏟️ The leaderboard of `period` — of `quiz` only when it names one — at the instant `at` over every
/// transcript: the top [`LEADERBOARD_TOP`] rows by rank (1-based position in [`compare_standings`]
/// order) of the standings in scope, how many learners are ranked, how many runs were submitted in
/// all, and the row of `caller` when that learner is ranked (also inside the top). Rows carry the
/// [`learner_tag`], never the learner id.
pub fn leaderboard<T: Borrow<Transcript>>(transcripts: &[T], catalog: &CatalogView, period: LeaderboardPeriod, quiz: Option<&str>, at: Timestamp, caller: Option<&str>) -> Leaderboard {
    let scope = BoardScope::of(period, quiz, at);
    let mut ranked: Vec<Standing> = transcripts.iter().filter_map(|transcript| standing(transcript.borrow(), catalog, &scope)).collect();
    ranked.sort_by(compare_standings);
    let own = caller.and_then(|caller| ranked.iter().position(|standing| standing.learner == caller)).map(|index| ranked[index].row(index + 1));
    Leaderboard {
        period,
        quiz: scope.quiz,
        window: scope.window,
        rows: ranked.iter().take(LEADERBOARD_TOP).enumerate().map(|(index, standing)| standing.row(index + 1)).collect(),
        learners: ranked.len(),
        submissions: transcripts.iter().map(|transcript| transcript.borrow().runs.len()).sum(),
        own,
    }
}

/// 🪣️ The bin of a score among the [`CROWD_SCORE_BINS`]: a tenth of its whole percent `⌊score · 100 + ½⌋`, rounded
/// down, the last bin closed (`[90, 100]`).
pub fn score_bin(score: Score) -> usize {
    ((score * 100.0 + 0.5).floor() as usize / 10).min(CROWD_SCORE_BINS - 1)
}

/// 📉️ How many of `scores` fall into each score bin.
fn binned(scores: impl Iterator<Item = Score>) -> CrowdScores {
    let mut bins = [0; CROWD_SCORE_BINS];
    for score in scores {
        bins[score_bin(score)] += 1;
    }
    bins
}

/// 🎪️ How many items a sheet of `task` presents: `draw` when it is set and smaller than the item count, else every
/// item.
pub fn presented(task: &Task) -> usize {
    let (items, draw) = match task {
        Task::Classification(task) => (task.items.len(), task.draw),
        Task::Sorting(task) => (task.items.len(), task.draw),
        Task::Matching(task) => (task.items.len(), task.draw),
    };
    draw.map_or(items, |draw| draw.min(items))
}

/// 🪑️ The place among `places` presented ones that `position` in a learner's order of `length` items counts for:
/// `position · (places − 1) / (length − 1)` rounded half up in integer arithmetic — the position itself when
/// `length` equals `places`, place 0 for an order of fewer than two items, never beyond the last place.
pub fn place_bin(position: usize, length: usize, places: usize) -> usize {
    if length < 2 || places < 1 {
        return 0;
    }
    (position.saturating_mul(2 * (places - 1)).saturating_add(length - 1) / (2 * (length - 1))).min(places - 1)
}

/// 📌️ The value among `values` nearest to `guess` on `scale`, the smaller of two equally near; `None`
/// among none — the rule a guess is tallied by in the crowd, for anyone who marks where it counts.
pub fn nearest_of(values: impl IntoIterator<Item = f64>, scale: Scale, guess: f64) -> Option<f64> {
    let position = scaled(scale, guess);
    let mut nearest: Option<(f64, f64)> = None;
    for value in values {
        let distance = (scaled(scale, value) - position).abs();
        if nearest.is_none_or(|(held, gap)| distance < gap || (distance == gap && value < held)) {
            nearest = Some((value, distance));
        }
    }
    nearest.map(|(value, _)| value)
}

/// 🧲️ The authored value of `dimension` nearest to `guess` on `scale` ([`nearest_of`]); `None` when no
/// item of the task carries one.
pub fn nearest_value(task: &MatchingTask, dimension: &str, scale: Scale, guess: f64) -> Option<f64> {
    nearest_of(task.items.iter().filter_map(|item| item.values.get(dimension).copied()), scale, guess)
}

/// 🗳️ The value a matched item counts under in the crowd of `dimension`: the assigned card value, for
/// a guess (an item result that carries `miss`) the authored value nearest to it ([`nearest_value`]);
/// `None` for an item the learner left unanswered, which counts nowhere.
pub fn crowd_value(task: &MatchingTask, dimension: &Dimension, result: &MatchingItemResult) -> Option<f64> {
    let assigned = result.assigned?;
    match result.miss {
        None => Some(assigned),
        Some(_) => nearest_value(task, &dimension.id, dimension.quantity.scale, assigned),
    }
}

/// 🧾️ Whether a sorting result places its items in the crowd: not when nobody guessed in it — every
/// item a miss without a guess (no answer, or none that says anything) — which adds its score only.
pub fn crowd_orders(items: &[SortingItemResult]) -> bool {
    !items.iter().all(|item| item.miss == Some(true) && item.guess.is_none())
}

/// 👪️ What the learners answered and scored in the submitted runs of `quiz`, every challenge mixed (results of
/// other quizzes are ignored): the run scores per score bin ([`score_bin`]), then every task in definition order,
/// matching once per dimension in definition order; per task the scores of the results that count for it — the
/// task score, for a matching the dimension's score — and the items in definition order that at least one result
/// answered. Classification counts the assigned category ids, matching the values the items count under
/// ([`crowd_value`]) as [`json_number_text`]s, both in ascending key order (code point order, so `"120"`
/// precedes `"50"`); sorting gives the mean over the results that place their items ([`crowd_orders`]), in
/// result order, of the normalized position `position / (n − 1)` in the learner's order of `n` items (`0` when
/// `n < 2`) beside the count per place a sheet presents ([`place_bin`] over [`presented`] places). A task result
/// counts only when its kind matches the quiz task; an item left unanswered counts nowhere.
pub fn crowd_view<R: Borrow<RunResult>>(quiz: &Quiz, results: &[R]) -> CrowdView {
    let results: Vec<&RunResult> = results.iter().map(Borrow::borrow).filter(|result| result.quiz == quiz.id).collect();
    let scored = |task: &Task| -> Vec<&TaskResult> { results.iter().filter_map(|result| result.tasks.iter().find(|scored| scored.task() == task.id() && kind_of(scored) == task.kind())).collect() };
    let mut tasks = Vec::new();
    for task in &quiz.tasks {
        let scored = scored(task);
        let scores = binned(scored.iter().map(|scored| scored.score()));
        match task {
            Task::Classification(definition) => {
                let items = definition.items.iter().filter_map(|item| counted(&item.id, scored.iter().filter_map(|scored| match scored {
                    TaskResult::Classification { items, .. } => items.iter().find(|result| result.item == item.id).and_then(|result| result.assigned.clone()),
                    _ => None,
                })));
                tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Classification, dimension: None, scores, items: items.collect() });
            }
            Task::Sorting(definition) => {
                let count = presented(task);
                let items = definition.items.iter().filter_map(|item| {
                    let orders: Vec<(usize, usize)> = scored
                        .iter()
                        .filter_map(|scored| match scored {
                            TaskResult::Sorting { items, .. } if crowd_orders(items) => items.iter().find(|result| result.item == item.id).map(|result| (result.position, items.len())),
                            _ => None,
                        })
                        .collect();
                    let mut places = vec![0; count];
                    for &(position, length) in &orders {
                        if let Some(place) = places.get_mut(place_bin(position, length, count)) {
                            *place += 1;
                        }
                    }
                    let sum = orders.iter().fold(0.0, |sum, &(position, length)| sum + if length < 2 { 0.0 } else { position as f64 / (length - 1) as f64 });
                    (!orders.is_empty()).then(|| CrowdItem { item: item.id.clone(), answers: orders.len(), counts: None, mean_position: Some(sum / orders.len() as f64), places: Some(places) })
                });
                tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Sorting, dimension: None, scores, items: items.collect() });
            }
            Task::Matching(definition) => {
                for dimension in &definition.dimensions {
                    let answered: Vec<&DimensionResult> = scored
                        .iter()
                        .filter_map(|scored| match scored {
                            TaskResult::Matching { dimensions, .. } => dimensions.iter().find(|result| result.dimension == dimension.id),
                            _ => None,
                        })
                        .collect();
                    let items = definition.items.iter().filter_map(|item| counted(&item.id, answered.iter().filter_map(|answered| answered.items.iter().find(|result| result.item == item.id).and_then(|result| crowd_value(definition, dimension, result)).map(json_number_text))));
                    tasks.push(CrowdTask { task: definition.id.clone(), kind: TaskKind::Matching, dimension: Some(dimension.id.clone()), scores: binned(answered.iter().map(|answered| answered.score)), items: items.collect() });
                }
            }
        }
    }
    CrowdView { quiz: quiz.id.clone(), runs: results.len(), scores: binned(results.iter().map(|result| result.score)), tasks }
}

/// 💱️ A value in JSON number syntax as ECMAScript's `Number.prototype.toString` renders it: the shortest
/// round-trip digits, plain notation for decimal exponents −7 < e < 21, otherwise `d.ddde±x`; `-0` → `"0"`.
pub fn json_number_text(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if !value.is_finite() {
        return if value.is_nan() { "NaN" } else if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let digits = mantissa.replace('.', "");
    let (count, point) = (digits.len() as i32, exponent.parse::<i32>().unwrap_or(0) + 1);
    let body = if count <= point && point <= 21 {
        format!("{digits}{}", "0".repeat((point - count) as usize))
    } else if 0 < point && point <= 21 {
        format!("{}.{}", &digits[..point as usize], &digits[point as usize..])
    } else if -6 < point && point <= 0 {
        format!("0.{}{digits}", "0".repeat((-point) as usize))
    } else {
        let exponent = if point > 0 { format!("+{}", point - 1) } else { (point - 1).to_string() };
        if count == 1 { format!("{digits}e{exponent}") } else { format!("{}.{}e{exponent}", &digits[..1], &digits[1..]) }
    };
    if value < 0.0 { format!("-{body}") } else { body }
}

fn kind_of(scored: &TaskResult) -> TaskKind {
    match scored {
        TaskResult::Classification { .. } => TaskKind::Classification,
        TaskResult::Sorting { .. } => TaskKind::Sorting,
        TaskResult::Matching { .. } => TaskKind::Matching,
    }
}

fn counted(item: &str, keys: impl Iterator<Item = String>) -> Option<CrowdItem> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for key in keys {
        *counts.entry(key).or_insert(0) += 1;
    }
    let answers = counts.values().sum::<usize>();
    (answers > 0).then(|| CrowdItem { item: item.to_string(), answers, counts: Some(counts.into_iter().map(|(key, count)| CrowdCount { key, count }).collect()), mean_position: None, places: None })
}

/// 📬️ The submitted runs of a learner in submission order (`submittedAt`, ties in start order).
fn submissions(state: &LearnerState) -> Vec<TranscriptRun> {
    let mut runs: Vec<TranscriptRun> = state
        .runs
        .iter()
        .filter(|run| run.status == RunStatus::Submitted)
        .filter_map(|run| {
            let result = run.result.as_ref()?;
            Some(TranscriptRun { quiz: run.quiz.clone(), challenge: result.challenge, score: result.score, points: result.points, at: run.submitted_at? })
        })
        .collect();
    runs.sort_by_key(|run| run.at);
    runs
}

/// 🥇️ The best run per quiz id over `runs` in submission order — the one with the most points, a later
/// run replacing it only with strictly more — and when the last best was raised.
fn bests<R: Borrow<TranscriptRun>>(runs: &[R]) -> (BTreeMap<Slug, Best>, Option<Timestamp>) {
    let mut best: BTreeMap<Slug, Best> = BTreeMap::new();
    let mut reached_at = None;
    for run in runs.iter().map(Borrow::borrow) {
        if best.get(&run.quiz).is_none_or(|previous| run.points > previous.points) {
            best.insert(run.quiz.clone(), Best { challenge: run.challenge, score: run.score, points: run.points });
            reached_at = Some(run.at);
        }
    }
    (best, reached_at)
}

fn total(best: &BTreeMap<Slug, Best>, catalog: &CatalogView) -> f64 {
    catalog.quizzes.iter().filter_map(|quiz| best.get(&quiz.id)).fold(0.0, |sum, best| sum + best.points)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
