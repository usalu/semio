//! ⛰️ The challenges of a run: one rule table, the points a score earns, when a value misses, the time
//! a task allows, the instant a learner acted and the hints an easy run gives.
//!
//! Each challenge is the one before plus one step: easy shows the keys (the numbers a task turns on)
//! and hints, medium shows the keys, hard hides them so the learner guesses, expert also runs every
//! task against a clock. A value misses when it lies farther from the truth than the reach of its set:
//! on a logarithmic scale a factor, [`REACH_FACTOR`] or the square root of the presented true values'
//! max/min ratio where that is less; on a linear scale a distance, half their spread; both widened by
//! a relative [`REACH_SLACK`] of 1e-9 so a decimal typed at exactly the reach stays within. Only `/`,
//! `*`, `+`, `sqrt`, `−`, `abs`, explicit min/max and comparisons are used, all exactly rounded, so an
//! exact ×1000 never misses and every twin agrees bit for bit. The same miss drives the hints (easy) and the
//! scoring of guesses (hard, expert). Everything here is pure, so the device's deputy and the proctor
//! decide alike.
//!
//! @see ../../README.md — the challenges
//! @see ../📏️scoring/🦀️.rs — the scoring of guesses with misses
//! @see ../⛰️challenge/🟦️.ts — the TypeScript twin

use crate::schema::{Answer, Axis, CategoryHint, Challenge, ClassificationItem, CompareHint, GroupHint, Hint, MatchingItem, Profile, ProfileHint, Scale, Score, SheetTask, Slug, Task, TaskKind, Timestamp, Verdict};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// 📋️ What a challenge asks: whether the keys show, whether far-off answers get hints, whether tasks
/// run against a clock, and the points a perfect run earns.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChallengeRules {
    pub keys: bool,
    pub hints: bool,
    pub timed: bool,
    pub par: f64,
}

/// 🗻️ The rules of every challenge, in the order of [`crate::schema::CHALLENGES`] (indexed by
/// [`challenge_rank`]).
pub const CHALLENGE_RULES: [ChallengeRules; 4] = [
    ChallengeRules { keys: true, hints: true, timed: false, par: 100.0 },
    ChallengeRules { keys: true, hints: false, timed: false, par: 200.0 },
    ChallengeRules { keys: false, hints: false, timed: false, par: 300.0 },
    ChallengeRules { keys: false, hints: false, timed: true, par: 400.0 },
];

/// 📖️ The rules of `challenge`.
pub fn challenge_rules(challenge: Challenge) -> ChallengeRules {
    CHALLENGE_RULES[challenge_rank(challenge)]
}

/// 🔢️ The rank of a challenge: 0 easy, 1 medium, 2 hard, 3 expert.
pub fn challenge_rank(challenge: Challenge) -> usize {
    match challenge {
        Challenge::Easy => 0,
        Challenge::Medium => 1,
        Challenge::Hard => 2,
        Challenge::Expert => 3,
    }
}

/// 🧗️ Whether `challenge` is at least as demanding as `least`.
pub fn challenge_meets(challenge: Challenge, least: Challenge) -> bool {
    challenge_rank(challenge) >= challenge_rank(least)
}

/// 💰️ The points a score earns at a challenge: `score × par`.
pub fn points(score: Score, challenge: Challenge) -> f64 {
    score * challenge_rules(challenge).par
}

/// 🔭️ The cap of the reach on a logarithmic scale, a factor: the reach of values that do not spread,
/// widened by [`REACH_SLACK`] before a value misses.
pub const REACH_FACTOR: f64 = 1000.0;

/// 📡️ The reach of a set of presented true values on their scale. Logarithmic: a factor,
/// `sqrt(hi / lo)` capped at [`REACH_FACTOR`], the cap itself when they do not spread. Linear: a
/// distance, `(hi − lo) / 2`, unbounded when they do not spread. A value that compares to nothing
/// (`NaN`) takes no part.
pub fn reach(values: &[f64], scale: Scale) -> f64 {
    let (mut lowest, mut highest) = (f64::INFINITY, f64::NEG_INFINITY);
    for &value in values {
        if value < lowest {
            lowest = value;
        }
        if value > highest {
            highest = value;
        }
    }
    match scale {
        Scale::Linear if highest > lowest => (highest - lowest) / 2.0,
        Scale::Linear => f64::INFINITY,
        Scale::Logarithmic if highest > lowest => {
            let root = (highest / lowest).sqrt();
            if root < REACH_FACTOR {
                root
            } else {
                REACH_FACTOR
            }
        }
        Scale::Logarithmic => REACH_FACTOR,
    }
}

/// 🪶️ The relative slack a reach is widened by before a value misses: a value typed in decimal at
/// exactly the reach (`0.018` against `18`) lies a hair beyond it in binary and still counts as within.
pub const REACH_SLACK: f64 = 1e-9;

/// 🎯️ Whether `value` lies farther from `truth` than `reach` widened by [`REACH_SLACK`]: by the
/// ratio `max / min` on a logarithmic scale, by the distance on a linear one; an infinite reach never
/// misses.
pub fn misses(value: f64, truth: f64, scale: Scale, reach: f64) -> bool {
    let bound = reach * (1.0 + REACH_SLACK);
    match scale {
        Scale::Linear => (value - truth).abs() > bound,
        Scale::Logarithmic => (if value > truth { value / truth } else { truth / value }) > bound,
    }
}

/// ⏲️ The seconds a timed task allows: a base and a share per presented item (per item and dimension
/// for a matching).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSeconds {
    pub base: u64,
    pub classification: u64,
    pub sorting: u64,
    pub matching: u64,
}

/// 🧭️ The [`TaskSeconds`] every timed sheet is dealt with.
pub const TASK_SECONDS: TaskSeconds = TaskSeconds { base: 30, classification: 8, sorting: 12, matching: 12 };

/// ⏳️ The seconds a timed task of `kind` allows for `items` presented items (and `dimensions` of a
/// matching).
pub fn task_seconds(kind: TaskKind, items: usize, dimensions: usize) -> u64 {
    let (share, factor) = match kind {
        TaskKind::Classification => (TASK_SECONDS.classification, 1),
        TaskKind::Sorting => (TASK_SECONDS.sorting, 1),
        TaskKind::Matching => (TASK_SECONDS.matching, dimensions as u64),
    };
    TASK_SECONDS.base.saturating_add(share.saturating_mul(items as u64).saturating_mul(factor))
}

/// 🛫️ How far, in milliseconds, a device's claim may run ahead of the decider's clock before it is
/// lowered: five minutes, more than an honest device's clock drifts, too little to buy a task more time.
pub const CLOCK_LEAD: Timestamp = 300_000;

/// 🕰️ The instant a learner acted as a decider counts it: the device's `at`, lowered to
/// [`CLOCK_LEAD`] past the decider's `now`, then raised to `floor` (the run's start, or the task's
/// opening; 0 for a start). An honest device decides at its own clock, so the lead never lowers its
/// claims and the deputy and the proctor decide alike.
pub fn acted(at: Timestamp, floor: Timestamp, now: Timestamp) -> Timestamp {
    at.min(now.saturating_add(CLOCK_LEAD)).max(floor)
}

/// 🖐️ The most hints one task gives at once: those with the largest error, the rest are not given.
pub const HINTS_PER_TASK: usize = 3;

/// 🔑️ An item with the key the learner assigned it, its true value and whether it is familiar.
struct Keyed<'a> {
    item: &'a Slug,
    key: f64,
    value: f64,
    familiar: bool,
}

/// 🔃️ How a claimed relation stands to the true one around `pivot` (1 for a ratio, 0 for a
/// difference): reversed when the claim lies on one side and the truth at the pivot or on the other
/// (a claim at the pivot never is), else under when the truth lies beyond the claim (a claim at the
/// pivot: above it), else over.
pub fn verdict_of(claim: f64, truth: f64, pivot: f64) -> Verdict {
    if (claim > pivot && truth <= pivot) || (claim < pivot && truth >= pivot) {
        Verdict::Reversed
    } else if (claim >= pivot && truth > claim) || (claim < pivot && truth < claim) {
        Verdict::Under
    } else {
        Verdict::Over
    }
}

/// 🔗️ A reference a compare hint may name: the relation the keys claim to it (`ρ = k_X / k_R` on a
/// logarithmic scale, `δ = k_X − k_R` on a linear one), the true one (`τ`, `Δ`), how wrong the claim
/// is (`max / min`, `|δ − Δ|`), how far it lies from no relation at all (`max(ρ, 1/ρ)`, `|δ|`) and
/// whether the reference is familiar.
struct Related<'a> {
    other: &'a Slug,
    familiar: bool,
    claim: f64,
    truth: f64,
    error: f64,
    oriented: f64,
}

/// 📐️ The [`Related`] of `other` as the reference of `item`.
fn relation<'a>(item: &Keyed<'_>, other: &Keyed<'a>, scale: Scale) -> Related<'a> {
    let (claim, truth, error, oriented) = match scale {
        Scale::Logarithmic => {
            let (claim, truth) = (item.key / other.key, item.value / other.value);
            (claim, truth, if claim > truth { claim / truth } else { truth / claim }, claim.max(1.0 / claim))
        }
        Scale::Linear => {
            let (claim, truth) = (item.key - other.key, item.value - other.value);
            (claim, truth, (claim - truth).abs(), claim.abs())
        }
    };
    Related { other: other.item, familiar: other.familiar, claim, truth, error, oriented }
}

/// ⏸️ A compare hint before its reference is chosen: the missed item, its dimension, its scale and
/// the references tied for the largest error, in sheet order.
struct Pending<'a> {
    item: &'a Slug,
    dimension: Option<&'a Slug>,
    scale: Scale,
    tied: Vec<Related<'a>>,
}

/// 🧩️ A hint as the cap ranks it: made, or a compare hint whose reference is chosen after the cap.
enum Drafted<'a> {
    Made(Hint),
    Compare(Pending<'a>),
}

/// ⚖️ The compare hint of `item` when its key misses, before its reference is chosen, and its weight,
/// the largest error: the keyed items (in sheet order, `keyed`) whose claimed relation to it is the
/// most wrong, preferring anchors, keyed items whose own key does not miss; errors within the relative
/// [`REACH_SLACK`] of the largest tie; none without another keyed item.
fn compare_hint<'a>(item: &Keyed<'a>, keyed: &[Keyed<'a>], scale: Scale, within: f64, dimension: Option<&'a Slug>) -> Option<(Drafted<'a>, f64)> {
    if !misses(item.key, item.value, scale, within) {
        return None;
    }
    let others: Vec<&Keyed<'a>> = keyed.iter().filter(|other| other.item != item.item).collect();
    let anchors: Vec<&Keyed<'a>> = others.iter().copied().filter(|other| !misses(other.key, other.value, scale, within)).collect();
    let pool = if anchors.is_empty() { others } else { anchors };
    let related: Vec<Related<'a>> = pool.into_iter().map(|other| relation(item, other, scale)).collect();
    let largest = related.iter().map(|related| related.error).fold(f64::NEG_INFINITY, f64::max);
    let tied: Vec<Related<'a>> = related.into_iter().filter(|related| related.error * (1.0 + REACH_SLACK) >= largest).collect();
    (!tied.is_empty()).then_some((Drafted::Compare(Pending { item: item.item, dimension, scale, tied }), largest))
}

/// 🪝️ The compare hint of a kept [`Pending`]: among the references tied for the largest error one not
/// yet `used` by an earlier compare hint of the task (of the same dimension) wins, then a familiar
/// one, then the smallest oriented claim, then the first in sheet order; the chosen one joins `used`.
fn referenced<'a>(pending: &Pending<'a>, used: &mut Vec<(Option<&'a Slug>, &'a Slug)>) -> Hint {
    let fresh = |related: &Related<'_>| !used.contains(&(pending.dimension, related.other));
    let mut chosen = &pending.tied[0];
    for related in &pending.tied {
        let better = if fresh(related) == fresh(chosen) { if related.familiar == chosen.familiar { related.oriented < chosen.oriented } else { related.familiar } } else { fresh(related) };
        if better {
            chosen = related;
        }
    }
    used.push((pending.dimension, chosen.other));
    let (factor, difference, pivot) = match pending.scale {
        Scale::Logarithmic => (Some(chosen.claim), None, 1.0),
        Scale::Linear => (None, Some(chosen.claim), 0.0),
    };
    Hint::Compare(CompareHint { item: pending.item.clone(), other: chosen.other.clone(), dimension: pending.dimension.cloned(), factor, difference, verdict: verdict_of(chosen.claim, chosen.truth, pivot) })
}

/// 🕸️ The axis a profile hint names for an item assigned to the category of profile `assigned` instead
/// of its own `own`, and its gap relative to its reach: the axis (in axis order, with values in both and
/// spread over the `presented` profiles) whose gap relative to its reach, half that spread, is largest
/// (the first on ties); none unless some gap exceeds its reach widened by [`REACH_SLACK`].
fn profile_axis<'a>(assigned: &Profile, own: &Profile, axes: &'a [Axis], presented: &[&Profile]) -> Option<(&'a Slug, f64)> {
    let mut exceeded = false;
    let mut chosen: Option<(&Slug, f64)> = None;
    for axis in axes {
        let (Some(&far), Some(&near)) = (assigned.get(&axis.id), own.get(&axis.id)) else { continue };
        let values: Vec<f64> = presented.iter().filter_map(|profile| profile.get(&axis.id).copied()).collect();
        let within = if values.is_empty() { 0.0 } else { (values.iter().copied().fold(f64::NEG_INFINITY, f64::max) - values.iter().copied().fold(f64::INFINITY, f64::min)) / 2.0 };
        if within.is_nan() || within <= 0.0 {
            continue;
        }
        let gap = (far - near).abs();
        exceeded |= gap > within * (1.0 + REACH_SLACK);
        let ratio = gap / within;
        if chosen.is_none_or(|(_, largest)| ratio > largest) {
            chosen = Some((&axis.id, ratio));
        }
    }
    exceeded.then_some(chosen?)
}

/// 🧷️ The anchor a profile hint questions its item against on an axis, and on which side of it the
/// placement claims the item: the item (in sheet order, `anchors`, each with its own profile's value on
/// the axis) whose value lies between the `claimed` value of the assigned profile and the `truth` of
/// the item's own, the one farthest from the truth first, then the first in sheet order.
fn profile_anchor<'a>(claimed: f64, truth: f64, anchors: impl Iterator<Item = (&'a Slug, f64)>) -> Option<(&'a Slug, bool)> {
    let mut chosen: Option<(&Slug, f64, bool)> = None;
    for (other, value) in anchors.filter(|&(_, value)| (claimed > value && truth < value) || (claimed < value && truth > value)) {
        let gap = (truth - value).abs();
        if chosen.is_none_or(|(_, largest, _)| gap > largest) {
            chosen = Some((other, gap, claimed > value));
        }
    }
    chosen.map(|(other, _, above)| (other, above))
}

/// 🪓️ The first [`HINTS_PER_TASK`] hints of a task by weight: the largest first, hints without one
/// (group and category) after them, the earlier on ties; the kept hints stay in the order they were
/// made.
fn capped<T>(ranked: Vec<(T, Option<f64>)>) -> Vec<T> {
    let mut order: Vec<usize> = (0..ranked.len()).collect();
    order.sort_by(|&left, &right| match (ranked[left].1, ranked[right].1) {
        (Some(left), Some(right)) => right.partial_cmp(&left).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    });
    order.truncate(HINTS_PER_TASK);
    order.sort_unstable();
    let mut kept: Vec<Option<T>> = ranked.into_iter().map(|(hint, _)| Some(hint)).collect();
    order.into_iter().filter_map(|index| kept[index].take()).collect()
}

/// 💡️ The hints of an answer to a task, none without an answer; each questions one relation the
/// answer claims, never states the truth, at most one stands per item (and dimension) and at most
/// [`HINTS_PER_TASK`] per task, those with the largest weight. Sorting (in the learner's order, where the
/// sheet shows the keys) and matching (per dimension in sheet order, then items in sheet order, where
/// the dimension shows cards): a [`CompareHint`] per item whose key misses its value, the references
/// chosen after the cap in that order so that a task's hints (of one dimension) name different
/// references where the tie window allows. Classification
/// (in sheet order), per assigned item standing in a category not its own: a [`ProfileHint`] when both
/// categories carry profiles and some axis is far off, against an item placed in its own category on
/// the other side of the item's own profile where there is one, else a [`GroupHint`] with the first
/// item put beside it from another category (`together`) or the first of its own category put
/// elsewhere, else a [`CategoryHint`]. Entries the task or the sheet task cannot resolve, and
/// assignments to unknown categories, give no hint.
pub fn hints_of(task: &Task, sheet_task: &SheetTask, answer: Option<&Answer>) -> Vec<Hint> {
    let mut used = Vec::new();
    let drafted: Vec<(Drafted<'_>, Option<f64>)> = match (task, sheet_task, answer) {
        (Task::Sorting(task), SheetTask::Sorting(sheet), Some(Answer::Sorting(answer))) => {
            let Some(keys) = &sheet.keys else { return Vec::new() };
            let scale = task.quantity.scale;
            let within = reach(keys, scale);
            let keyed: Vec<Keyed<'_>> = sheet
                .items
                .iter()
                .filter_map(|presented| {
                    let item = task.items.iter().find(|candidate| candidate.id == presented.id)?;
                    Some(Keyed { item: &item.id, key: *keys.get(answer.order.iter().position(|id| id == &presented.id)?)?, value: item.value, familiar: item.familiar == Some(true) })
                })
                .collect();
            answer.order.iter().filter_map(|id| compare_hint(keyed.iter().find(|candidate| candidate.item == id)?, &keyed, scale, within, None)).map(|(drafted, error)| (drafted, Some(error))).collect()
        }
        (Task::Matching(task), SheetTask::Matching(sheet), Some(Answer::Matching(answer))) => {
            let Some(assignments) = &answer.assignments else { return Vec::new() };
            let items: Vec<&MatchingItem> = sheet.items.iter().filter_map(|presented| task.items.iter().find(|candidate| candidate.id == presented.id)).collect();
            let mut hints = Vec::new();
            for dimension in &sheet.dimensions {
                let (Some(cards), Some(assigned)) = (&dimension.cards, assignments.get(&dimension.id)) else { continue };
                let scale = dimension.quantity.scale;
                let valued: Vec<(&MatchingItem, f64)> = items.iter().filter_map(|item| item.values.get(&dimension.id).map(|&truth| (*item, truth))).collect();
                let within = reach(&valued.iter().map(|&(_, truth)| truth).collect::<Vec<_>>(), scale);
                let keyed: Vec<Keyed<'_>> = valued.iter().filter_map(|&(item, value)| Some(Keyed { item: &item.id, key: *assigned.get(&item.id).and_then(|&card| cards.get(card))?, value, familiar: item.familiar == Some(true) })).collect();
                hints.extend(keyed.iter().filter_map(|item| compare_hint(item, &keyed, scale, within, Some(&dimension.id))).map(|(drafted, error)| (drafted, Some(error))));
            }
            hints
        }
        (Task::Classification(task), SheetTask::Classification(sheet), Some(Answer::Classification(answer))) => {
            let category = |id: &Slug| task.categories.iter().find(|category| &category.id == id);
            let profile = |id: &Slug| category(id).and_then(|category| category.profile.as_ref());
            let presented: Vec<&ClassificationItem> = sheet.items.iter().filter_map(|shown| task.items.iter().find(|candidate| candidate.id == shown.id)).collect();
            let profiles: Vec<&Profile> = sheet.categories.iter().filter_map(|category| profile(&category.id)).collect();
            let axes = task.axes.as_deref().unwrap_or_default();
            presented
                .iter()
                .filter_map(|item| {
                    let assigned = answer.assignments.get(&item.id).filter(|assigned| **assigned != item.category && category(assigned).is_some())?;
                    if let (Some(far), Some(near)) = (profile(assigned), profile(&item.category)) {
                        let (axis, ratio) = profile_axis(far, near, axes, &profiles)?;
                        let anchors = presented.iter().filter(|other| answer.assignments.get(&other.id) == Some(&other.category)).filter_map(|other| Some((&other.id, *profile(&other.category)?.get(axis)?)));
                        let anchor = profile_anchor(far[axis], near[axis], anchors);
                        return Some((Drafted::Made(Hint::Profile(ProfileHint { item: item.id.clone(), category: assigned.clone(), axis: axis.clone(), other: anchor.map(|(other, _)| other.clone()), above: anchor.map(|(_, above)| above) })), Some(ratio)));
                    }
                    let beside = presented.iter().find(|other| answer.assignments.get(&other.id) == Some(assigned) && other.category != item.category).map(|other| (other, true));
                    let apart = || presented.iter().find(|other| other.category == item.category && answer.assignments.get(&other.id).is_some_and(|placed| placed != assigned)).map(|other| (other, false));
                    Some((
                        Drafted::Made(match beside.or_else(apart) {
                            Some((other, together)) => Hint::Group(GroupHint { item: item.id.clone(), other: other.id.clone(), together }),
                            None => Hint::Category(CategoryHint { item: item.id.clone(), category: assigned.clone() }),
                        }),
                        None,
                    ))
                })
                .collect()
        }
        _ => Vec::new(),
    };
    capped(drafted)
        .into_iter()
        .map(|drafted| match drafted {
            Drafted::Made(hint) => hint,
            Drafted::Compare(pending) => referenced(&pending, &mut used),
        })
        .collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
