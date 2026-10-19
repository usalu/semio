//! 🏃️ Motion: one tick of a stride, slipping, scooting, plunging, flying, arriving and the dust of a disappearance. See [`super`].

use super::*;
use semio_framework_pets_character::feeling::Occasion;
use semio_framework_pets_contract::schema::Facing;
use semio_framework_pets_contract::schema::Ending;
use semio_framework_pets_contract::schema::Waypoint;
use semio_framework_pets_contract::schema::Course;

//#region 🔖️Motion
/// 👣️ One tick of a walk. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor stands on it then) or when somebody is in the way of the next beat; otherwise a stride towards the goal follows. A hopping gait that reaches its goal within a hop, or finds somebody in its way in mid-hop, finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest.
pub(crate) fn stride(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    let body = &draft.stage.actors[index];
    let beat = beat_of(kind, clip_of(kind, body.clip.as_deref()));
    let step = larger(kind.locomotion.speed, 0.0) / RATE;
    let mut done = now >= body.until;
    if !done && (now - body.since) % beat == 0 {
        let gap = body.goal - body.x;
        if gap.abs() <= step {
            let allowed = guarded(draft, index, gap, now);
            let body = &mut draft.stage.actors[index];
            body.x = if allowed == gap { body.goal } else { body.x + allowed };
            done = true;
        } else {
            let far = step * beat as f64;
            let ahead = clamp(body.goal, body.x - far, body.x + far) - body.x;
            done = guarded(draft, index, ahead, now) != ahead;
        }
    }
    if !done {
        let body = &draft.stage.actors[index];
        let next = stride_to(body.x, body.goal, kind.locomotion.speed);
        let wanted = next - body.x;
        let allowed = guarded(draft, index, wanted, now);
        let body = &mut draft.stage.actors[index];
        if allowed == wanted { body.x = next; return; }
        body.x += allowed;
        body.goal = body.x;
        if beat > 1 { return; }
    }
    let body = &mut draft.stage.actors[index];
    if body.leaving {
        shift(draft, index, Activity::Idle, now);
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Idle, 0.0);
        body.goal = body.x;
        return;
    }
    if body.partner.is_some() {
        attend(draft, index, now);
        return;
    }
    settle(draft, index, now);
}

/// 🛡️ A stride respects every solid body and planned corridor through this tick.
pub(super) fn guarded(draft: &Draft<'_>, index: usize, wanted: f64, now: Ticks) -> f64 {
    guarded_stride(extent_of(&draft.stage.actors[index], draft.kinds[index]), wanted, &obstacles_for(draft, index, now))
}

/// 🛷️ Slide off a host's head through guarded positions, then plan a fall.
pub(crate) fn slip(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let host = body.host.as_deref().and_then(|host| index_of(&draft.stage.actors, host));
    let box_of = extent_of(body, draft.kinds[index]);
    if !host.is_some_and(|host| rests_on(box_of, extent_of(&draft.stage.actors[host], draft.kinds[host]))) {
        let vx = body.vx;
        unfoot(draft, index, vx, 0.0, Activity::Fall, now);
        return plunge(draft, index, now);
    }
    let side = if body.vx < 0.0 { Facing::Left } else { Facing::Right };
    let ticks = now - body.since;
    let slid = slide_of(box_of, side, ticks, 0.0, draft.stage.width, &obstacles_for(draft, index, now));
    let body = &mut draft.stage.actors[index];
    body.x += slid.stride;
    body.goal = body.x;
    let sign = if slid.side == Facing::Right { 1.0 } else { -1.0 };
    if slid.side == side { body.vx = sign * slide_stride(ticks) * RATE; } else { body.since = now; body.vx = sign * SLIDE_OFF_SPEED; }
    true
}

/// 🚌️ Move each surface's scooters together through free positions.
pub(crate) fn scoot(draft: &mut Draft<'_>, now: Ticks) {
    let mut surfaces = Vec::new();
    for body in &draft.stage.actors {
        if body.activity == Activity::Scoot && body.footing == Footing::Perch {
            if let Some(surface) = &body.perch { if !surfaces.contains(surface) { surfaces.push(surface.clone()); } }
        }
    }
    for surface in surfaces {
        let members: Vec<_> = draft.stage.actors.iter().enumerate().filter_map(|(index, body)| (body.activity == Activity::Scoot && body.footing == Footing::Perch && body.perch.as_deref() == Some(&surface)).then_some(index)).collect();
        if members.is_empty() { continue; }
        let shifts: Vec<_> = members.iter().map(|&member| draft.stage.actors[member].goal - draft.stage.actors[member].x).collect();
        let slowest = members.iter().map(|&member| larger(draft.kinds[member].locomotion.speed, 1.0)).fold(f64::INFINITY, smaller);
        let fraction = scoot_fraction(&shifts, SCOOT_HASTE * slowest / RATE);
        let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
        let others: Vec<_> = bodies.iter().enumerate().filter(|(index, _)| !members.contains(index)).map(|(_, body)| body.clone()).collect();
        let obstacles = obstacles_of("", &others, &draft.stage.claims, now);
        let places: Vec<_> = members.iter().map(|&member| scooted(draft.stage.actors[member].x, draft.stage.actors[member].goal, fraction)).collect();
        let free = members.iter().zip(&places).all(|(&member, &place)| free_among(shifted(bodies[member].extent, place - draft.stage.actors[member].x, 0.0), &obstacles));
        if free {
            for (&member, &place) in members.iter().zip(&places) { draft.stage.actors[member].x = place; }
            if fraction >= 1.0 { for &member in &members { settle(draft, member, now); } }
            continue;
        }
        if now < draft.stage.actors[members[0]].until { continue; }
        for &member in members.iter().rev() {
            if perch_at(&draft.stage.perches, &surface, draft.stage.actors[member].x).is_some() { settle(draft, member, now); } else { drop(draft, member, now); }
        }
    }
}

/// 🌬️ Plan the first legal descent or wait in a free place before the last-resort poof.
pub(crate) fn plunge(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    if let Some(plan) = ways_down(draft, index, now) {
        let activity = draft.stage.actors[index].activity;
        install(draft, index, plan, activity, now);
        return fly(draft, index, now);
    }
    let body = &draft.stage.actors[index];
    let stay = (now < body.until).then(|| extent_of(body, draft.kinds[index]));
    if !must_poof(&body.species, stay, &[], &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims, now) { return true; }
    poof(draft, index, now)
}

/// 🛩️ Follow the next owned course waypoint and complete its reserved landing.
pub(crate) fn fly(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    let Some(course) = course_of(&draft.stage.courses, &draft.stage.actors[index].species) else { return plunge(draft, index, now); };
    let last = course.steps.len() - 1;
    let at = now - course.from;
    if at < 0 { return true; }
    let step = course.steps[if at > last as Ticks { last } else { at as usize }];
    let body = &draft.stage.actors[index];
    if at <= last as Ticks && bounced(draft, index, &step, body.vx, body.vy) { feel(draft, index, Occasion::Bonked, now); }
    let body = &mut draft.stage.actors[index];
    (body.x, body.y, body.vx, body.vy, body.tilt) = (step.x, step.y, step.vx, step.vy, step.tilt);
    if let Some(canopy) = step.canopy {
        if let Some(chute) = body.chute {
            let spring = spring_step(chute.open, chute.opening, 1.0, 220.0, 12.0);
            body.chute = Some(Parachute { since: chute.since, open: spring.position, opening: spring.velocity, canopy });
        } else {
            body.chute = Some(Parachute { since: now, open: 0.0, opening: 0.0, canopy });
            body.footing = Footing::Chute;
            if body.activity != Activity::Glide { shift(draft, index, Activity::Glide, now); draft.stage.actors[index].clip = clip_at(draft.kinds[index], Activity::Glide, 0.0); }
        }
    }
    if at < last as Ticks { true } else { let course = course_of(&draft.stage.courses, &draft.stage.actors[index].species).unwrap().clone(); arrive(draft, index, &course, now) }
}

/// 🤕️ A free-flight waypoint bounces when an edge reverses its velocity.
fn bounced(draft: &Draft<'_>, index: usize, step: &Waypoint, vx: f64, vy: f64) -> bool {
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    step.canopy.is_none() && (((step.x == half || step.x == draft.stage.width - half) && vx * step.vx < 0.0) || (step.y == kind.size.height && vy < 0.0 && step.vy >= 0.0))
}

/// 🌪️ Every disappearance is counted and leaves dust at the actor's former body.
pub(super) fn dusted(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    draft.stage.puffs.push(Puff { x: body.x, y: body.y - kind.size.height / 2.0, width: kind.size.width, height: kind.size.height, tick: now });
    draft.stage.poofs += 1;
}

/// 💨️ Remove a body that has no legal place and count its dust puff.
pub(crate) fn poof(draft: &mut Draft<'_>, index: usize, now: Ticks) -> bool {
    dusted(draft, index, now);
    vanish(draft, index, now)
}

/// 🛬️ A committed course ends exactly on its reserved perch or head, or leaves the stage.
pub(super) fn arrive(draft: &mut Draft<'_>, index: usize, course: &Course, now: Ticks) -> bool {
    let kind = draft.kinds[index];
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    if course.ending == Ending::Away { dusted(draft, index, now); return vanish(draft, index, now); }
    let body = &mut draft.stage.actors[index];
    let flown = matches!(body.activity, Activity::Fall | Activity::Tumble | Activity::Glide);
    let floated = body.chute.is_some();
    body.chute = None;
    body.tilt = 0.0;
    body.vx = 0.0;
    body.vy = 0.0;
    body.goal = body.x;
    if course.ending == Ending::Head {
        let host = index_of(&draft.stage.actors, &course.landing);
        shift(draft, index, Activity::Slide, now);
        let vx = if host.is_some_and(|host| draft.stage.actors[index].x < draft.stage.actors[host].x) { -SLIDE_OFF_SPEED } else { SLIDE_OFF_SPEED };
        let body = &mut draft.stage.actors[index];
        body.clip = clip_at(kind, Activity::Slide, 0.0);
        body.footing = Footing::Head;
        body.perch = None;
        body.host = Some(course.landing.clone());
        body.until = now + dwell_of(Activity::Slide, draft.stage.mode, 0.0);
        body.vx = vx;
        if let Some(host) = host { feel(draft, host, Occasion::Trampled, now); }
        return true;
    }
    body.footing = Footing::Perch;
    body.perch = Some(course.landing.clone());
    body.host = None;
    if floated { settle(draft, index, now); if flown { feel(draft, index, Occasion::Floated, now); } return true; }
    shift(draft, index, Activity::Land, now);
    let clip = clip_at(kind, Activity::Land, 0.0);
    let until = now + clip_of(kind, clip.as_deref()).filter(|clip| !clip.looping).map_or_else(|| dwell_of(Activity::Land, draft.stage.mode, 0.0), clip_ticks);
    let body = &mut draft.stage.actors[index];
    body.clip = clip;
    body.until = until;
    if !flown { return true; }
    let hard = course.touch > HARD_LANDING;
    let bumped = course.steps.windows(2).any(|steps| bounced(draft, index, &steps[1], steps[0].vx, steps[0].vy) && (if steps[0].vx * steps[1].vx < 0.0 { steps[0].vx.abs() } else { steps[0].vy.abs() }) >= DIZZY_SPEED);
    draft.stage.actors[index].vy = if hard && course.touch >= DIZZY_SPEED { course.touch } else if bumped { DIZZY_SPEED } else { 0.0 };
    feel(draft, index, if hard { Occasion::Dropped } else { Occasion::Landed }, now);
    true
}

/// 😵️ A hard landing or edge collision leaves a short authored dizzy activity.
pub(crate) fn daze(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Dizzy, now);
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(draft.kinds[index], Activity::Dizzy, 0.0);
    body.until = now + dwell_of(Activity::Dizzy, draft.stage.mode, 0.0);
    body.vx = 0.0;
    body.vy = 0.0;
}
//#endregion 🔖️Motion
