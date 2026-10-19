//! 🧳️ Trips: resting, halting and travelling an installed gear trip, alighting and moving on from a wall or ladder. See [`super`].

use super::*;
use semio_framework_pets_character::feeling::Occasion;
use semio_framework_pets_traversal::climbing::Entry;
use semio_framework_pets_contract::schema::Arrival;
use semio_framework_pets_contract::schema::Trip;
use semio_framework_pets_contract::schema::Pitch;
use semio_framework_pets_contract::schema::Ladder;

//#region 🔖️Gear
/// 🦥️ Rest on a wall or ladder while reserving enough grip to leave the wall.
pub(crate) fn rest(draft: &mut Draft<'_>, index: usize, span: Ticks, now: Ticks) {
    shift(draft, index, Activity::Climb, now);
    let kind = draft.kinds[index];
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Climb, 0.0);
    body.goal = body.x;
    body.vx = 0.0;
    body.vy = 0.0;
    let mut until = now + span;
    let body = &draft.stage.actors[index];
    if let Some(pitch) = body.pitch.as_ref().filter(|_| body.footing == Footing::Wall) {
        let chain = chain_of(pitch, &draft.stage.pitches, kind.size);
        let spare = body.grip - reserve_of(draft, index, &chain, chain.iter().position(|candidate| *candidate == pitch).unwrap(), body.x, body.y, now);
        until = std::cmp::min(until, now + (spare / GRIP_HANG).floor() as Ticks);
    }
    draft.stage.actors[index].until = std::cmp::max(until, now + 1);
}

/// 🛑️ Release an interrupted trip and rest wherever its current footing permits.
pub(crate) fn halt(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let owner = body.species.clone();
    if trip_of(&draft.stage.trips, &owner).is_none() { return; }
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let body = &draft.stage.actors[index];
    if body.footing == Footing::Rope { let (vx, vy) = (body.vx, body.vy); unfoot(draft, index, vx, vy, Activity::Fall, now); return; }
    draft.stage.actors[index].rope = None;
    if draft.stage.actors[index].footing == Footing::Ladder { rest(draft, index, 1, now); return; }
    freed(draft, &owner, now);
    if draft.stage.actors[index].footing == Footing::Wall { rest(draft, index, 1, now); }
}

/// 🚂️ Step an installed gear trip along its exact footholds and rope geometry.
pub(crate) fn travel(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let Some(trip) = trip_of(&draft.stage.trips, &draft.stage.actors[index].species) else { return true; };
    let at = now - trip.from;
    if at < 0 { return true; }
    let last = trip.steps.len() - 1;
    let at = std::cmp::min(at as usize, last);
    if now > trip.from && draft.stage.actors[index].activity != trip.steps[at.saturating_sub(1)].activity { halt(draft, index, now); return true; }
    let step = trip.steps[at].clone();
    let goal = walk_end(trip, at);
    let pitch = (step.hold >= 0).then(|| trip.pitches[step.hold as usize].clone());
    let kind = draft.kinds[index];
    let body = &mut draft.stage.actors[index];
    body.vx = (step.x - body.x) * RATE;
    body.vy = (step.y - body.y) * RATE;
    body.x = step.x;
    body.y = step.y;
    body.footing = step.footing;
    body.perch = step.perch.clone();
    body.pitch = pitch;
    body.grip = larger(body.grip - effort_of(&step), 0.0);
    if step.activity != body.activity {
        shift(draft, index, step.activity, now);
        draft.stage.actors[index].clip = clip_at(kind, step.activity, 0.0);
        if matches!(step.activity, Activity::Walk | Activity::Carry) { draft.stage.actors[index].goal = goal; }
        if step.activity == Activity::Shrug { feel(draft, index, Occasion::Missed, now); }
    }
    let body = &mut draft.stage.actors[index];
    if step.facing != body.facing { turn(body, step.facing, now); }
    if let Some(rope) = body.rope.as_ref() {
        if (!rope.caught && now >= rope.since + hook_ticks(rope.shot.muzzle, rope.shot.hook, HOOK_SPEED) + hook_ticks(rope.shot.hook, rope.shot.muzzle, HOOK_RETURN)) || (rope.caught && now > rope.since && step.footing == Footing::Perch && step.activity != Activity::Aim) { body.rope = None; }
        else if step.footing == Footing::Rope { let hand = Point { x: step.x + rope.shot.facing.sign() * MUZZLE_FORWARD * kind.size.width, y: step.y - MUZZLE_HEIGHT * kind.size.height }; let (dx, dy) = (rope.shot.hook.x - hand.x, rope.shot.hook.y - hand.y); body.rope = Some(Rope { hand, before: rope.hand, length: (dx * dx + dy * dy).sqrt(), ..rope.clone() }); }
    }
    if at < last { return true; }
    let trip = trip_of(&draft.stage.trips, &draft.stage.actors[index].species).unwrap().clone();
    alight(draft, index, &trip, now)
}

/// 🏁️ Release the completed trip and restore its perch, wall rest or free fall.
fn alight(draft: &mut Draft<'_>, index: usize, trip: &Trip, now: Ticks) -> bool {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.trips.retain(|trip| trip.owner != owner);
    freed(draft, &owner, now);
    draft.stage.actors[index].rope = None;
    if trip.ending == Arrival::Air { let body = &draft.stage.actors[index]; let (vx, vy) = (body.vx, body.vy); unfoot(draft, index, vx, vy, Activity::Fall, now); return plunge(draft, index, now); }
    if trip.ending == Arrival::Wall {
        let body = &mut draft.stage.actors[index];
        body.footing = Footing::Wall;
        body.perch = None;
        body.grip = trip.grip;
        let words = random_words(&actor_key(draft, index), 1);
        rest(draft, index, dwell_of(Activity::Idle, draft.stage.mode, unit_of(words[0])), now);
        return true;
    }
    let body = &mut draft.stage.actors[index];
    body.footing = Footing::Perch;
    body.perch = Some(trip.landing.clone());
    body.pitch = None;
    body.grip = GRIP_BUDGET;
    settle(draft, index, now);
    if trip.steps.last().unwrap().activity == Activity::Shrug { draft.stage.actors[index].until = std::cmp::max(draft.stage.actors[index].until, now + ROPE_REST); }
    else if trip.steps.iter().any(|step| step.footing != Footing::Perch) { feel(draft, index, Occasion::Climbed, now); }
    true
}

/// 🏞️ Try an authored random rest farther along the currently held wall line.
fn explored(draft: &Draft<'_>, index: usize, chain: &[&Pitch], start: usize, grip: f64, place: f64, height: f64, now: Ticks) -> Option<Outing> {
    let body = &draft.stage.actors[index];
    let size = draft.kinds[index].size;
    let at = std::cmp::min((place * chain.len() as f64).floor() as usize, chain.len() - 1);
    let y = rim_of(chain[at], size) + (foot_of(chain[at], size) - rim_of(chain[at], size)) * height;
    scaling(draft, index, &[], chain, start, Entry::Cling, body.x, body.y, at, y, false, None, grip, now).filter(|outing| lasting(draft, index, outing, now) && spaced(draft, index, outing))
}

/// ➰️ Leave an expired rest through the first clear climb, ladder exit or wall slide.
fn onward(draft: &mut Draft<'_>, index: usize, ladder: Option<&Ladder>, now: Ticks) -> bool {
    if let Some(ladder) = ladder {
        for way in ladder_ways(draft, index, ladder, now) { if embark(draft, index, way, now) { return true; } }
        if ladder.since + LADDER_LIFE <= now + 32 { feel(draft, index, Occasion::Startled, now); unfoot(draft, index, 0.0, 0.0, Activity::Fall, now); return plunge(draft, index, now); }
        for entry in &mut draft.stage.ladders { if entry.owner == ladder.owner { entry.until = std::cmp::min(entry.since + LADDER_LIFE, std::cmp::max(entry.until, now + 32 + LADDER_IDLE)); } }
        draft.stage.actors[index].until = now + 32;
        return true;
    }
    let body = &draft.stage.actors[index];
    let pitch = body.pitch.clone().unwrap();
    let grip = grip_now(body, now);
    let chain = chain_of(&pitch, &draft.stage.pitches, draft.kinds[index].size);
    let start = chain.iter().position(|candidate| *candidate == &pitch).unwrap();
    let ways = ways_off(draft, index, &chain, start, body.x, body.y, grip, now);
    let reserve = ways.first().map_or(0.0, |way| grip - way.trip.grip);
    let has_way = !ways.is_empty();
    let mut tries = Vec::new();
    if draft.stage.mode == semio_framework_pets_contract::schema::PetMode::Lively && !draft.stage.quiet {
        let units: Vec<_> = random_words(&actor_key(draft, index), 3).into_iter().map(unit_of).collect();
        let chain = chain_of(&pitch, &draft.stage.pitches, draft.kinds[index].size);
        if units[0] < 0.5 { if let Some(spot) = explored(draft, index, &chain, start, grip, units[1], units[2], now) { tries.push(spot); } }
    }
    tries.extend(ways);
    for outing in tries { if embark(draft, index, outing, now) { return true; } }
    if has_way && grip - GRIP_HANG * 32.0 >= reserve { draft.stage.actors[index].until = now + 32; return true; }
    if let Some(slide) = slide_outing(draft, index, grip, now) { if embark(draft, index, slide, now) { return true; } }
    if grip - GRIP_HANG * 32.0 > 0.0 { draft.stage.actors[index].until = now + 32; return true; }
    feel(draft, index, Occasion::Startled, now);
    unfoot(draft, index, 0.0, 0.0, Activity::Fall, now);
    plunge(draft, index, now)
}

/// 🐨️ Rest on a held wall or ladder until it is time to move on.
pub(crate) fn cling(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let ladder = (body.footing == Footing::Ladder).then(|| draft.stage.ladders.iter().find(|entry| entry.rider.as_deref() == Some(&body.species)).cloned()).flatten();
    if (body.footing == Footing::Wall && body.pitch.is_none()) || (body.footing == Footing::Ladder && ladder.is_none()) { unfoot(draft, index, 0.0, 0.0, Activity::Fall, now); return plunge(draft, index, now); }
    if now < body.until { return true; }
    onward(draft, index, ladder.as_ref(), now)
}

/// 🔓️ Free an owner's ladder and remove a ladder not yet raised.
fn freed(draft: &mut Draft<'_>, owner: &str, now: Ticks) {
    draft.stage.ladders.retain(|ladder| ladder.owner != owner || ladder.since <= now);
    for ladder in &mut draft.stage.ladders { if ladder.rider.as_deref() == Some(owner) { ladder.rider = None; } }
}

/// 🧺️ Release trips, rope, wall grip and ladders whenever an actor loses its footing.
pub(super) fn ungear(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.trips.retain(|trip| trip.owner != owner);
    let body = &mut draft.stage.actors[index];
    body.rope = None;
    body.pitch = None;
    body.grip = GRIP_BUDGET;
    freed(draft, &owner, now);
}
//#endregion 🔖️Gear
