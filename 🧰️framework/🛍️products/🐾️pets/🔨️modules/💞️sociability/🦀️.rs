//! 💞️ Sociability: what pets do with each other — the pairing on whole seconds, coming together, the encounter, the sulk after a squabble and its mending, and the rapport that moves and fades.
//!
//! Whatever happens between two actors belongs here.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../💞️sociability/🟦️.ts — the TypeScript twin

use crate::behavior::{affinity_of, dwell_of, encounter_of, needs_after, rapport_after, rapport_faded, MODE_LIMITS};
use crate::draft::{actor_key, clip_at, index_of, shift, stage_key, Draft};
use crate::locomotion::{attend, paced, stroll};
use crate::randomness::{random_pick, random_words, unit_of};
use crate::schedule::{pairing_tick, sociable};
use crate::schema::{Activity, Menagerie, Rapport, Ticks};
use crate::spacing::{clearway, COMFORT_GAP, MEET_GAP};
use crate::terrain::{larger, smaller};
use crate::trigonometry::clamp;

//#region 🔖️Constants
const ENCOUNTER_REACH: f64 = 12.0;
const ENCOUNTER_RISE: f64 = 3.0;
const PAIR_WEIGHT: f64 = 0.25;
//#endregion 🔖️Constants

//#region 🔖️Rapport
/// 😤️ An actor sulks for a drawn span, turning its back on its partner.
pub(crate) fn sulk(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let words = random_words(&actor_key(draft, index), 2);
    shift(draft, index, Activity::Sulk, now);
    let until = now + dwell_of(Activity::Sulk, draft.stage.mode, unit_of(words[0]));
    let clip = clip_at(draft.kinds[index], Activity::Sulk, unit_of(words[1]));
    let body = &mut draft.stage.actors[index];
    body.until = until;
    body.clip = clip;
}

/// 🍂️ Every rapport fades over the ticks since the last change of any of them; what has faded to 0 is forgotten. `met` becomes `now`.
fn recall(draft: &mut Draft<'_>, now: Ticks) {
    let elapsed = now - draft.stage.met;
    if elapsed > 0 {
        draft.stage.rapports.retain_mut(|rapport| {
            rapport.drift = rapport_faded(rapport.drift, elapsed);
            rapport.drift != 0.0
        });
    }
    draft.stage.met = now;
}

/// 🪢️ The rapport of two actors after something between them (`rapport_after`); a new pair is listed last, the earlier species of the menagerie first, and a drift of 0 is forgotten.
fn bond(draft: &mut Draft<'_>, first: usize, second: usize, activity: Activity) {
    let early = if first < second { first } else { second };
    let late = if first < second { second } else { first };
    let a = &draft.stage.actors[early].species;
    let b = &draft.stage.actors[late].species;
    let mut found = false;
    draft.stage.rapports.retain_mut(|rapport| {
        if rapport.between[0] != *a || rapport.between[1] != *b {
            return true;
        }
        found = true;
        rapport.drift = rapport_after(rapport.drift, activity);
        rapport.drift != 0.0
    });
    if !found {
        let drift = rapport_after(0.0, activity);
        if drift != 0.0 {
            draft.stage.rapports.push(Rapport { between: [a.clone(), b.clone()], drift });
        }
    }
}

/// 🕊️ The end of a sulk: once the partner has stopped sulking too, the two mend their rapport by a little; the actor lets go of its partner either way.
pub(crate) fn reconcile(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let Some(partner) = body.partner.as_deref() else {
        return;
    };
    let other = index_of(&draft.stage.actors, partner);
    let pending = other.is_some_and(|other| draft.stage.actors[other].activity == Activity::Sulk && draft.stage.actors[other].partner.as_deref() == Some(body.species.as_str()));
    if let Some(other) = other.filter(|_| !pending) {
        recall(draft, now);
        bond(draft, index, other, Activity::Sulk);
    }
    draft.stage.actors[index].partner = None;
}
//#endregion 🔖️Rapport

//#region 🔖️Encounters
/// 💌️ On a whole second the stage may pair two sociable actors that stand near each other (no farther apart than 12 of their mean widths, no more than 3 in height): one stage draw picks a pair, weighted `(0.25 + |authored affinity|) × mean sociability` (pairs that feel something for each other meet more often than strangers, and whoever has just had company lets others go first), and decides with the chance `rate × mean sociability`. Sociability is the need as it stands now. The pair then approaches; when the mode has room for one mover only, the more sociable of the two walks.
pub(crate) fn pair(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    if pairing_tick(&draft.stage, &draft.stage.actors, now) != now {
        return;
    }
    let limits = MODE_LIMITS[draft.stage.mode];
    let actors = &draft.stage.actors;
    let movers = actors.iter().filter(|body| matches!(body.activity, Activity::Walk | Activity::Hop)).count();
    if movers >= limits.movers {
        return;
    }
    let mut firsts: Vec<usize> = Vec::new();
    let mut seconds: Vec<usize> = Vec::new();
    let mut drives: Vec<u8> = Vec::new();
    let mut socials: Vec<f64> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();
    for (first, one) in actors.iter().enumerate() {
        if !sociable(one) {
            continue;
        }
        let eager = needs_after(one.needs, Activity::Idle, now - one.since, draft.kinds[first].temperament).sociability;
        for (second, two) in actors.iter().enumerate().skip(first + 1) {
            if !sociable(two) {
                continue;
            }
            let width = (draft.kinds[first].size.width + draft.kinds[second].size.width) / 2.0;
            if (one.x - two.x).abs() > ENCOUNTER_REACH * width || (one.y - two.y).abs() > ENCOUNTER_RISE * width || parted(draft, first, second) {
                continue;
            }
            let keen = needs_after(two.needs, Activity::Idle, now - two.since, draft.kinds[second].temperament).sociability;
            let social = (eager + keen) / 2.0;
            firsts.push(first);
            seconds.push(second);
            drives.push(if eager >= keen { 1 } else { 2 });
            socials.push(social);
            weights.push((PAIR_WEIGHT + affinity_of(menagerie, &[], &one.species, &two.species).abs()) * social);
        }
    }
    if firsts.is_empty() {
        return;
    }
    let key = stage_key(draft);
    let Some(chosen) = random_pick(&key, &weights) else {
        return;
    };
    let lucky = unit_of(random_words(&key, 2)[1]) < limits.encounter_rate * socials[chosen];
    if !lucky {
        return;
    }
    approach(draft, firsts[chosen], seconds[chosen], if limits.movers - movers >= 2 { 0 } else { drives[chosen] }, now);
}

/// 🚻️ Whether somebody stands between two actors of one surface, so that they could not come together without walking through it.
fn parted(draft: &Draft<'_>, first: usize, second: usize) -> bool {
    let one = &draft.stage.actors[first];
    let two = &draft.stage.actors[second];
    if one.perch != two.perch {
        return false;
    }
    let low = smaller(one.x, two.x);
    let high = larger(one.x, two.x);
    draft.stage.actors.iter().enumerate().any(|(other, between)| other != first && other != second && between.perch == one.perch && between.x >= low && between.x <= high)
}

/// 🧭️ Where an actor may walk to on its perch to meet its partner (the actor at `partner`), as close to `x` as its clear way and its gait allow.
fn reachable(draft: &Draft<'_>, index: usize, partner: usize, x: f64) -> f64 {
    let kind = draft.kinds[index];
    let [low, high] = clearway(draft, index, COMFORT_GAP, Some(partner));
    paced(kind, clip_at(kind, Activity::Walk, 0.0).as_deref(), draft.stage.actors[index].x, clamp(x, low, high))
}

/// 🤝️ Two actors become partners and come together until their bodies are 6 px apart (what reaches beyond the box of a body — an arm, a ray — then just touches): both walk to the middle (`walker` 0), or only the first (1) or the second (2) walks while the other waits, and whoever need not move waits at once. Each stays on its own perch.
fn approach(draft: &mut Draft<'_>, first: usize, second: usize, walker: u8, now: Ticks) {
    let partner = draft.stage.actors[second].species.clone();
    draft.stage.actors[first].partner = Some(partner);
    let partner = draft.stage.actors[first].species.clone();
    draft.stage.actors[second].partner = Some(partner);
    let one = draft.stage.actors[first].x;
    let two = draft.stage.actors[second].x;
    let apart = (draft.kinds[first].size.width + draft.kinds[second].size.width) / 2.0 + MEET_GAP;
    let side = if two >= one { 1.0 } else { -1.0 };
    let close = (two - one).abs() <= apart;
    let middle = (one + two) / 2.0;
    let near = if close || walker == 2 { one } else { reachable(draft, first, second, if walker == 1 { two - side * apart } else { middle - (side * apart) / 2.0 }) };
    let far = if close || walker == 1 { two } else { reachable(draft, second, first, if walker == 2 { one + side * apart } else { middle + (side * apart) / 2.0 }) };
    if near == one {
        attend(draft, first, now);
    } else {
        stroll(draft, first, near, clip_at(draft.kinds[first], Activity::Walk, 0.0), now);
    }
    if far == two {
        attend(draft, second, now);
    } else {
        stroll(draft, second, far, clip_at(draft.kinds[second], Activity::Walk, 0.0), now);
    }
}

/// 🎉️ Partners that both wait begin their encounter: the rapports fade up to now, one stage draw picks the kind from their affinity (`encounter_of`), the span they share and a clip for each; they face each other and their rapport moves.
pub(crate) fn meet(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    for first in 0..draft.stage.actors.len() {
        let one = &draft.stage.actors[first];
        if one.activity != Activity::Idle {
            continue;
        }
        let Some(second) = one.partner.as_deref().and_then(|partner| index_of(&draft.stage.actors, partner)).filter(|&second| second > first) else {
            continue;
        };
        let two = &draft.stage.actors[second];
        if two.activity != Activity::Idle || two.partner.as_deref() != Some(one.species.as_str()) {
            continue;
        }
        recall(draft, now);
        let words = random_words(&stage_key(draft), 4);
        let one = &draft.stage.actors[first];
        let two = &draft.stage.actors[second];
        let kind = encounter_of(affinity_of(menagerie, &draft.stage.rapports, &one.species, &two.species), unit_of(words[0]));
        let until = now + dwell_of(kind, draft.stage.mode, unit_of(words[1]));
        shift(draft, first, kind, now);
        shift(draft, second, kind, now);
        let clip = clip_at(draft.kinds[first], kind, unit_of(words[2]));
        let one = &mut draft.stage.actors[first];
        one.until = until;
        one.clip = clip;
        let clip = clip_at(draft.kinds[second], kind, unit_of(words[3]));
        let two = &mut draft.stage.actors[second];
        two.until = until;
        two.clip = clip;
        bond(draft, first, second, kind);
    }
}
//#endregion 🔖️Encounters

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
