//! 🎯️ Choice: what an actor does next when its time is up — an idle one picks an activity by its weights, with its dwell, its clip and its goal or target (`decide`); everything else ends the way its activity ends (`conclude`).
//!
//! A new activity an actor may choose by itself gets its branch in `decide`, and its end in `conclude`.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🎯️choice/🟦️.ts — the TypeScript twin

use semio_framework_pets_staging_draft::attention::watched;
use semio_framework_pets_staging_draft::draft::{actor_key, clip_at, clip_of, release, settle, shift, Draft};
use semio_framework_pets_staging_draft::locomotion::{hops_of, leave, paced, stroll};
use crate::sociability::{reconcile, sulk};
use semio_framework_pets_staging_draft::spacing::{clearway, rooms_for, COMFORT_GAP};
use semio_framework_pets_character::behavior::{activity_weights, dwell_of, Situation, MODE_LIMITS};
use semio_framework_pets_contract::schema::{Activity, Footing, Gait, Ticks, ACTIVITIES};
use semio_framework_pets_kinematics::animation::clip_ticks;
use semio_framework_pets_kinematics::randomness::{random_pick, random_words, unit_of};
use semio_framework_pets_kinematics::terrain::{larger, perch_at};
use semio_framework_pets_kinematics::trigonometry::clamp;
use semio_framework_pets_character::feeling::Occasion;
use semio_framework_pets_contract::schema::PetMode;
use semio_framework_pets_contract::schema::Gear;
use semio_framework_pets_contract::schema::Cue;
use semio_framework_pets_contract::schema::Mood;
use semio_framework_pets_contract::schema::Perch;
use semio_framework_pets_contract::schema::Menagerie;

//#region 🔖️Constants
const STROLL_LEAST: f64 = 0.75;
const SHEEPISH: f64 = 0.3;
const PUSH_HOLD: Ticks = LIFT_TICKS + 2;
//#endregion 🔖️Constants

//#region 🔖️Decision
/// 🧗️ Gear destinations follow friends, open perches and then reachable wall lines.
fn ventures_of(draft: &Draft<'_>, index: usize, perch: &Perch, crowd: usize) -> Vec<Venture> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    if !geared(kind) { return Vec::new(); }
    let mut ventures = Vec::new();
    for (other, friend) in draft.stage.actors.iter().enumerate() {
        if other == index || friend.leaving || friend.footing != Footing::Perch { continue; }
        if affinity_of(draft.menagerie, &draft.stage.rapports, &body.species, &friend.species) < FRIENDS { continue; }
        if let Some(there) = friend.perch.as_deref().and_then(|surface| perch_at(&draft.stage.perches, surface, friend.x)) {
            if there != perch && !ventures.iter().any(|venture| matches!(venture, Venture::Perch(known) if known == there)) && reaches(draft, kind, body.x, perch, there, draft.stage.tick) { ventures.push(Venture::Perch(there.clone())); }
        }
    }
    let restless = draft.stage.mode == PetMode::Lively || crowd > 0;
    if restless {
        for room in rooms_for(draft, kind) {
            let there = &draft.stage.perches[room.perch];
            if there != perch && !ventures.iter().any(|venture| matches!(venture, Venture::Perch(known) if known == there)) && reaches(draft, kind, body.x, perch, there, draft.stage.tick) { ventures.push(Venture::Perch(there.clone())); }
        }
    }
    if !restless || !kind.gear.contains(&Gear::Climb) { return ventures; }
    for pitch in &draft.stage.pitches {
        if (grip_for(perch, pitch, kind.size).is_none() && (!kind.gear.contains(&Gear::Ladder) || ladder_to(perch, pitch, &draft.stage.keepouts, kind.size).is_none())) || ventures.iter().any(|venture| matches!(venture, Venture::Wall { pitch: known, .. } if known.wall == pitch.wall && known.side == pitch.side)) { continue; }
        ventures.push(Venture::Wall { pitch: pitch.clone(), y: None });
    }
    ventures
}

/// 🙋️ An idle actor decides what to do next: one `random_pick` over `activity_weights`, then its dwell, its clip and, for a walk, a goal on the clear way of its perch (within the stroll of the mode, at least three quarters of a body away — a hopping gait at least one whole hop —, never past or into anybody; without such a goal it stays idle instead) or, for a hop, one of its launches. An actor that feels like a hop where no perch is in reach wanders off instead when another perch has room for it: it leaves, and arrives anew a moment later.
fn decide(draft: &mut Draft<'_>, index: usize, now: Ticks, forced: Option<Activity>) {
    let kind = draft.kinds[index];
    let mode = draft.stage.mode;
    let limits = MODE_LIMITS[mode];
    let width = kind.size.width;
    let mut body = draft.stage.actors[index].clone();
    body.needs = needs_after(body.needs, body.activity, now - body.since, kind.temperament);
    let felt = present(draft, index, now);
    let feeling = drowsed(felt, body.needs.energy, now);
    let state = standing(draft, index, now).state;
    let mut movers = 0;
    let mut fidgeters = 0;
    let mut crowd = 0;
    for (other, actor) in draft.stage.actors.iter().enumerate() {
        if other == index {
            continue;
        }
        if astir(draft, actor) {
            movers += 1;
        }
        if actor.activity == Activity::Fidget {
            fidgeters += 1;
        }
        if body.footing == Footing::Perch && body.perch.is_some() && actor.footing == Footing::Perch && actor.perch == body.perch && !actor.leaving {
            crowd += 1;
        }
    }
    let at = body.x;
    let perch = (body.footing == Footing::Perch).then_some(body.perch.as_deref()).flatten().and_then(|surface| perch_at(&draft.stage.perches, surface, at));
    let [low, high] = clearway(draft, index, COMFORT_GAP, None);
    let held = circled(&body.hover, now);
    let restless = !draft.stage.quiet && movers < limits.movers && limits.hop > 0.0 && perch.is_some() && !held;
    let launches = if restless { hops_of(draft, index) } else { Vec::new() };
    let ventures = if restless { ventures_of(draft, index, perch.unwrap(), crowd) } else { Vec::new() };
    let elsewhere = restless && launches.is_empty() && rooms_for(draft, kind).iter().any(|room| !perch.is_some_and(|perch| std::ptr::eq(perch, &draft.stage.perches[room.perch])));
    let wishes = mood_weights(feeling.mood, feeling.intensity);
    let weights: Vec<f64> = activity_weights(
        &body,
        kind,
        Situation { mode, quiet: draft.stage.quiet, movers, fidgeters, roam: !held && larger(at - low, high - at) >= STROLL_LEAST * width, hops: !launches.is_empty() || elsewhere || !ventures.is_empty(), crowd, watched: watched(draft.stage.pointer, &body, kind), whims: !tricks_for(kind, Cue::Whim, &state, feeling).is_empty() },
    ).into_iter().zip(wishes).map(|(weight, wish)| weight * wish).collect();
    if forced.is_some_and(|activity| !(weights[ACTIVITIES.iter().position(|candidate| *candidate == activity).unwrap()] > 0.0)) { return; }
    shift(draft, index, Activity::Idle, now);
    if feeling != felt { draft.stage.actors[index].feeling = feeling; }
    let key = actor_key(draft, index);
    let pick = random_pick(&key, &weights);
    let words = random_words(&key, 4);
    let activity = forced.unwrap_or_else(|| pick.map_or(Activity::Idle, |pick| ACTIVITIES[pick]));
    let dwell = unit_of(words[1]);
    let clip = clip_at(kind, activity, unit_of(words[2]));
    let place = unit_of(words[3]);
    if activity == Activity::Walk && mode == PetMode::Lively {
        let walls: Vec<_> = ventures.iter().filter(|venture| matches!(venture, Venture::Wall { .. }) || matches!(venture, Venture::Perch(perch) if perch.y < draft.stage.actors[index].y)).cloned().collect();
        if !walls.is_empty() {
            let gear: Vec<_> = random_words(&[draft.stage.seed, GEAR_STREAM, draft.streams[index], key[2]], 5).into_iter().map(unit_of).collect();
            if gear[0] < 0.4 && set_out(draft, index, &walls, gear[1], &gear[2..], now) { return; }
        }
    }
    if activity == Activity::Walk {
        let reach = limits.stroll * width;
        let least = STROLL_LEAST * width;
        let mut goal = clamp(low + (high - low) * place, at - reach, at + reach);
        if (goal - at).abs() < least {
            goal = if at - low > high - at { at - least } else { at + least };
        }
        goal = paced(kind, clip.as_deref(), at, clamp(goal, low, high));
        let sets_out = if kind.locomotion.gait == Gait::Hop { goal != at } else { (goal - at).abs() >= least };
        if sets_out {
            stroll(draft, index, goal, clip, now);
            return;
        }
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, unit_of(words[2]));
        body.until = now + dwell_of(Activity::Idle, mode, dwell);
        return;
    }
    if activity == Activity::Trick {
        if let Some(trick) = whim_trick(kind, &state, feeling, unit_of(words[2])) {
            perform(draft, index, trick, now);
            return;
        }
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, unit_of(words[2]));
        body.until = now + dwell_of(Activity::Idle, mode, dwell);
        return;
    }
    if activity == Activity::Hop {
        if !ventures.is_empty() {
            let gear: Vec<_> = random_words(&[draft.stage.seed, GEAR_STREAM, draft.streams[index], key[2]], 5).into_iter().map(unit_of).collect();
            let share = if mode == PetMode::Lively { 0.6 } else if mode == PetMode::Calm { 0.25 } else { 0.0 };
            if (launches.is_empty() || gear[0] < share) && set_out(draft, index, &ventures, gear[1], &gear[2..], now) { return; }
        }
        if launches.is_empty() && (forced.is_some() || !elsewhere) {
            let body = &mut draft.stage.actors[index];
            body.clip = clip_at(kind, Activity::Idle, unit_of(words[2]));
            body.until = now + dwell_of(Activity::Idle, mode, dwell);
            return;
        }
        if launches.is_empty() {
            leave(draft, index, now);
            return;
        }
        let chosen = (place * launches.len() as f64).floor();
        let chosen = &launches[if chosen < launches.len() as f64 { chosen as usize } else { launches.len() - 1 }];
        if launch(draft, index, chosen, clip, now) { return; }
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, unit_of(words[2]));
        body.until = now + dwell_of(Activity::Idle, mode, dwell);
        return;
    }
    let until = now + clip_of(kind, clip.as_deref()).filter(|played| activity == Activity::Fidget && !played.looping).map_or_else(|| dwell_of(activity, mode, dwell), clip_ticks);
    let body = &mut draft.stage.actors[index];
    body.activity = activity;
    body.clip = clip;
    body.until = until;
}

/// 🏁️ The end of what an actor does when its time is up: an idle one decides (or gives up waiting for its partner), a squabbler sulks, a sulker reconciles, and everyone else comes to rest. Walks, hops and falls end by their motion.
pub(crate) fn conclude(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let activity = draft.stage.actors[index].activity;
    let partnered = draft.stage.actors[index].partner.is_some();
    if draft.stage.actors[index].footing != Footing::Perch || matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall | Activity::Scoot) {
        return;
    }
    if activity == Activity::Land && draft.stage.actors[index].vy > 0.0 { daze(draft, index, now); return; }
    if activity == Activity::Idle && !partnered {
        decide(draft, index, now, None);
        return;
    }
    if activity == Activity::Squabble && partnered {
        sulk(draft, index, now);
        return;
    }
    if activity == Activity::Cuddle && partnered {
        draft.stage.actors[index].partner = None;
        purr(draft, index, 192, now);
        return;
    }
    if activity == Activity::Trick {
        let kind = draft.kinds[index];
        if let Some(trick) = trick_of(kind, draft.stage.actors[index].trick.as_deref()) {
            stand(draft, index, now);
            let after = state_after_trick(kind, &draft.stage.actors[index].state, trick);
            if after != draft.stage.actors[index].state || trick.to.is_some() { enter(draft, index, &after, now, now); }
            draft.stage.actors[index].feeling = performed(present(draft, index, now), trick, kind.into(), now);
        }
    }
    if activity == Activity::Idle {
        release(draft, index, now);
    }
    if activity == Activity::Sulk {
        reconcile(draft, index, now);
    }
    settle(draft, index, now);
}
//#endregion 🔖️Decision

/// 🥁️ Moods travel on the stage's fixed contagion beat.
pub(crate) fn beat(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    if now % CONTAGION_BEAT == 0 { contagion(menagerie, draft, now); }
    if now % CHEMISTRY_BEAT == 0 && !draft.stage.quiet && !menagerie.chemistry.is_empty() && draft.stage.actors.len() > 1 { react(menagerie, draft, now); }
}

/// ⚗️ Apply due chemistry in its declared order from a single sighting snapshot.
fn react(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    let sightings = sightings_of(menagerie, &draft.stage, now);
    let trials = trials_of(menagerie, &sightings, &draft.stage.coolings, now, &draft.stage.rapports);
    if trials.is_empty() && draft.stage.coolings.iter().all(|cooling| cooling.until > now) { return; }
    let draws = draws_of(&trials);
    let units: Vec<_> = random_words(&[draft.stage.seed, CHEMISTRY_STREAM, (now / CHEMISTRY_BEAT) as u32], draws).into_iter().map(unit_of).collect();
    let outcome = reactions_of(menagerie, &sightings, now, &draft.stage.coolings, &units, &draft.stage.rapports);
    draft.stage.coolings = outcome.coolings;
    for consequence in outcome.consequences {
        let Some(index) = index_of(&draft.stage.actors, &consequence.on) else { continue; };
        let other = index_of(&draft.stage.actors, &consequence.other);
        let kind = draft.kinds[index];
        if let Some(state) = &consequence.state { enter(draft, index, state, now, now); }
        if let Some(mood) = consequence.mood { draft.stage.actors[index].feeling = stirred(present(draft, index, now), mood, consequence.amount, kind.into(), now); }
        if let Some(other) = other.filter(|_| consequence.rapport != 0.0) { nudge(draft, index, other, consequence.rapport, now); }
        if let Some(encounter) = consequence.encounter { pledge(draft, &consequence.on, &consequence.other, encounter, now); }
        if let Some(trick) = trick_of(kind, consequence.trick.as_deref()).filter(|_| loose(&draft.stage.actors[index])) { perform(draft, index, trick, now); }
        if let Some(activity) = consequence.activity.filter(|_| draft.stage.actors[index].activity == Activity::Idle && loose(&draft.stage.actors[index])) { decide(draft, index, now, Some(activity)); }
    }
}

//#region 🔖️Mischief
/// 🃏️ Select a prospect, trying physical routes before guarded slips to its post.
fn prank(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    let prospects = prospects_of(menagerie, &draft.stage, now);
    if prospects.is_empty() { return; }
    let units: Vec<_> = random_words(&[draft.stage.seed, MISCHIEF_STREAM, now as u32, 0], 2).into_iter().map(unit_of).collect();
    let first = std::cmp::min((units[0] * prospects.len() as f64).floor() as usize, prospects.len() - 1);
    for slip in [false, true] {
        for turn in 0..prospects.len() {
            let prospect = &prospects[(first + turn) % prospects.len()];
            let index = index_of(&draft.stage.actors, &prospect.actor).unwrap();
            if let Some(arrives) = post_to(draft, index, &prospect.post, prospect.side, PUSH_HOLD, slip, now) {
                draft.stage.lift = Some(Prank { fixture: prospect.fixture.id.clone(), pusher: prospect.actor.clone(), since: arrives + 1, side: prospect.side, room: prospect.room, span: prospect.fixture.width, unit: units[1] });
                return;
            }
        }
    }
    draft.stage.rested = now;
}

/// 🙈️ Run the due prank transition after the actors have taken their turns.
pub(crate) fn mischief(menagerie: &Menagerie, draft: &mut Draft<'_>, now: Ticks) {
    if prank_tick(menagerie, &draft.stage, now) != now { return; }
    let Some(lift) = draft.stage.lift.clone() else { prank(menagerie, draft, now); return; };
    let index = index_of(&draft.stage.actors, &lift.pusher);
    let age = now - lift.since;
    if age > LIFT_TICKS {
        draft.stage.lift = None;
        if let Some(index) = index { draft.stage.actors[index].feeling = stirred(present(draft, index, now), Mood::Sad, SHEEPISH, draft.kinds[index].into(), now); }
        return;
    }
    if age == LIFT_TICKS {
        draft.stage.lift = None;
        draft.stage.rested = now;
        if let Some(index) = index.filter(|&index| draft.stage.actors[index].activity == Activity::Push) { unpush(draft, index, now); feel(draft, index, Occasion::Tricked, now); }
        return;
    }
    if age == 0 && index.is_some_and(|index| push_at(draft, index, lift.side, now + LIFT_TICKS + 1, now)) { return; }
    if age >= LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE {
        draft.stage.lift = Some(Prank { since: now - LIFT_RETURNS, ..lift });
        if let Some(index) = index { unpush(draft, index, now); }
        return;
    }
    draft.stage.lift = None;
    draft.stage.rested = now;
    if let Some(index) = index { unpush(draft, index, now); }
}

/// 🫱️ Reclaim a lifted fixture and throw its current pusher clear of it.
pub(crate) fn reclaimed(draft: &mut Draft<'_>, fixture: &str, now: Ticks) {
    let Some(lift) = draft.stage.lift.clone().filter(|lift| lift.fixture == fixture && now >= lift.since && now - lift.since < LIFT_TICKS) else { return; };
    draft.stage.rested = now;
    let Some(index) = index_of(&draft.stage.actors, &lift.pusher).filter(|&index| draft.stage.actors[index].activity == Activity::Push) else { draft.stage.lift = None; return; };
    let body = &draft.stage.actors[index];
    let toss = draft.stage.fixtures.iter().find(|entry| entry.id == fixture).map_or(crate::climbing::Toss { vx: -lift.side.sign() * THROW_SPEED, vy: -THROW_LIFT }, |fixture| thrown_off(Point { x: body.x, y: body.y }, fixture, unit_of(random_words(&[draft.stage.seed, MISCHIEF_STREAM, now as u32, 1], 1)[0])));
    draft.stage.lift = Some(Prank { since: now - LIFT_TICKS - 1, ..lift });
    feel(draft, index, Occasion::Evicted, now);
    unfoot(draft, index, toss.vx, toss.vy, Activity::Tumble, now);
}
//#endregion 🔖️Mischief
