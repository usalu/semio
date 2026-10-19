//! ✋️ Hand: lifting a pet by its scruff, holding, letting go, giving back and tossing it. See [`super`].

use super::*;
use semio_framework_pets_contract::schema::Ending;

//#region 🔖️Hand
/// 🚦️ A body removed from its plan makes overlapping corridor owners plan again.
fn yielded(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let owner = draft.stage.actors[index].species.clone();
    let box_of = extent_of(&draft.stage.actors[index], draft.kinds[index]);
    for other in 0..draft.stage.actors.len() {
        if other == index { continue; }
        let body = &draft.stage.actors[other];
        let own: Vec<_> = draft.stage.claims.iter().filter(|claim| claim.owner == body.species).cloned().collect();
        if own.is_empty() || free_among(box_of, &obstacles_of(&owner, &[], &own, now)) { continue; }
        let (vx, vy, activity) = (body.vx, body.vy, body.activity);
        unfoot(draft, other, vx, vy, activity, now);
    }
}

/// ✋️ Lift a pet by its scruff and remember its perch origin for cancellation.
pub(crate) fn lift(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let kind = draft.kinds[index];
    release(draft, index, now);
    let owner = draft.stage.actors[index].species.clone();
    draft.stage.claims = released(&draft.stage.claims, &owner);
    draft.stage.courses.retain(|course| course.owner != owner);
    ungear(draft, index, now);
    let body = &draft.stage.actors[index];
    draft.stage.origin = (body.footing == Footing::Perch).then_some(Point { x: body.x, y: body.y });
    draft.stage.trail.clear();
    if body.activity != Activity::Hang { shift(draft, index, Activity::Hang, now); }
    let body = &mut draft.stage.actors[index];
    body.clip = clip_at(kind, Activity::Hang, 0.0);
    body.footing = Footing::Hand;
    body.perch = None;
    body.host = None;
    body.chute = None;
    body.tilt = 0.0;
    body.hang = Some(hang_of(Point { x: body.x, y: body.y }, kind.grip));
    body.vx = 0.0;
    body.vy = 0.0;
    body.goal = body.x;
    body.until = now + dwell_of(Activity::Hang, draft.stage.mode, 0.0);
    yielded(draft, index, now);
}

/// 🦧️ Swing a held body below its grip, pushing it away from occupied extents.
pub(crate) fn hold(draft: &mut Draft<'_>, index: usize, target: Point, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let Some(hang) = body.hang else { return; };
    let next = hang_step(hang, Point { x: clamp(target.x, 0.0, draft.stage.width), y: clamp(target.y, 0.0, draft.stage.height) }, kind.grip);
    let tilt = lean_of(Point { x: next.grip.x, y: next.grip.y }, next.bob, kind.grip);
    let box_of = extent_at(&body.species, kind, next.bob, tilt, None);
    let push = pushed_out(box_of, &obstacles_for(draft, index, now), PUSHES);
    let valid = push.is_some_and(|push| {
        let moved = shifted(box_of, push.x, push.y);
        let half = kind.size.width / 2.0;
        moved.x0 + MARGIN >= 0.0 && moved.x1 - MARGIN <= draft.stage.width && moved.y0 + MARGIN >= 0.0 && moved.y1 - MARGIN <= draft.stage.height && next.bob.x + push.x - half >= 0.0 && next.bob.x + push.x + half <= draft.stage.width && next.bob.y + push.y - kind.size.height >= 0.0 && next.bob.y + push.y <= draft.stage.height
    });
    let body = &mut draft.stage.actors[index];
    if !valid { body.hang = Some(Hang { grip: Grip { vx: 0.0, vy: 0.0, ..hang.grip }, bob: hang.bob, previous: hang.bob }); return; }
    let push = push.unwrap();
    body.hang = Some(Hang { grip: Grip { x: next.grip.x + push.x, y: next.grip.y + push.y, ..next.grip }, bob: Point { x: next.bob.x + push.x, y: next.bob.y + push.y }, previous: Point { x: next.previous.x + push.x, y: next.previous.y + push.y } });
    body.x = next.bob.x + push.x;
    body.y = next.bob.y + push.y;
    body.tilt = tilt;
}

/// 🎈️ Release a held pet with the measured pointer velocity.
pub(crate) fn let_go(draft: &mut Draft<'_>, index: usize, velocity: Point, now: Ticks) {
    draft.stage.trail.clear();
    unfoot(draft, index, velocity.x, velocity.y, Activity::Tumble, now);
}

/// ↩️ Return a cancelled pick-up along a clear eased corridor to its nearest free origin.
pub(crate) fn give_back(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let origin = draft.stage.origin.take();
    let perch = origin.and_then(|origin| draft.stage.perches.iter().find(|perch| perch.y - hover_of(kind) == origin.y && perch.x0 <= origin.x && origin.x <= perch.x1));
    let spot = origin.zip(perch).and_then(|(origin, perch)| spot_on(perch, origin.x, extent_at(&body.species, kind, origin, 0.0, None), kind.size.width / 2.0, &obstacles_for(draft, index, now)));
    let Some((origin, perch, spot)) = origin.zip(perch).zip(spot).map(|((origin, perch), spot)| (origin, perch, spot)) else { let_go(draft, index, ORIGIN, now); return; };
    let (dx, dy) = (spot - body.x, origin.y - body.y);
    let ticks = std::cmp::max(((dx * dx + dy * dy).sqrt() * RATE / 384.0).floor() as Ticks + 1, 12);
    let steps = (1..=ticks).map(|tick| {
        let share = smoothstep(tick as f64 / ticks as f64);
        if tick == ticks { Waypoint { x: spot, y: origin.y, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None } } else { Waypoint { x: body.x + dx * share, y: body.y + dy * share, vx: 0.0, vy: 0.0, tilt: body.tilt * (1.0 - share), canopy: None } }
    }).collect();
    let plan = plan_of(draft, index, Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: perch.surface.clone(), touch: 0.0 });
    if !claim_clear(&plan.claim, &bodies_of(&draft.stage.actors, &draft.kinds), &draft.stage.claims) { let_go(draft, index, ORIGIN, now); return; }
    draft.stage.trail.clear();
    release(draft, index, now);
    install(draft, index, plan, Activity::Glide, now);
}

/// 🎯️ Carry a tossed pet's body sixteen pixels below the upper stage edge.
pub(crate) fn toss_target(draft: &Draft<'_>, index: usize) -> Point {
    Point { x: draft.stage.actors[index].goal, y: 16.0 + draft.kinds[index].size.height - draft.kinds[index].grip }
}

/// 🎾️ A keyboard toss carries a lifted pet up for ninety-six ticks before release.
pub(crate) fn toss(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    lift(draft, index, now);
    draft.stage.origin = None;
    draft.stage.actors[index].until = now + 96;
}
//#endregion 🔖️Hand
