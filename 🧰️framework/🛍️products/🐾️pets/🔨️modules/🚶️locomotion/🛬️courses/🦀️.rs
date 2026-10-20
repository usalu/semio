//! 🛬️ Courses: charting a fall, bounce or parachute descent through its exact landing and trying the ways down in contract order. See [`super`].

use super::*;
use semio_framework_pets_contract::schema::Ending;
use semio_framework_pets_contract::schema::Gear;
use semio_framework_pets_contract::schema::Waypoint;
use semio_framework_pets_contract::schema::Course;
use semio_framework_pets_clearance::Body as Solid;

//#region 🔖️Courses
/// 🪟️ Distance above the first footprint-sized perch or motionless head below the feet.
fn headroom(perches: &[Perch], statics: &[Solid], x: f64, y: f64, hover: f64) -> f64 {
    let mut least = f64::INFINITY;
    for perch in perches { if perch.x0 <= x && x <= perch.x1 && perch.y - hover >= y && perch.y - hover - y < least { least = perch.y - hover - y; } }
    for body in statics { let top = body.extent.y0 - hover - MARGIN; if body.extent.x0 < x && x < body.extent.x1 && top >= y && top - y < least { least = top - y; } }
    least
}

/// 🧭️ Chart an entire fall, bounce or parachute descent through its exact landing.
fn charted(draft: &Draft<'_>, index: usize, start: Waypoint, from: Ticks, decision: Ticks, target: f64, statics: &[Solid]) -> Course {
    let kind = draft.kinds[index];
    let owner = draft.stage.actors[index].species.clone();
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let chute = chute_of(kind.size.height);
    let phase = 0.37 * draft.streams[index] as f64;
    let perches: Vec<_> = draft.stage.perches.iter().filter(|perch| perch.x0 + half <= perch.x1 - half).map(|perch| Perch { surface: perch.surface.clone(), x0: perch.x0 + half, x1: perch.x1 - half, y: perch.y }).collect();
    let mut steps = Vec::new();
    let (mut x, mut y, mut vx, mut vy, mut canopy) = (start.x, start.y, start.vx, start.vy, start.canopy);
    let mut before = extent_at(&owner, kind, Point { x, y }, start.tilt, canopy.map(|canopy| Point { x: canopy.x, y: canopy.y }));
    for tick in 0..1024 {
        if canopy.is_none() && decision >= 0 && tick >= decision + CHUTE_REFLEX { canopy = Some(canopy_of(Point { x, y }, vx, vy, chute)); }
        let mut next = canopy.map(|canopy| chute_step(canopy, chute, target, headroom(&perches, statics, x, y, hover), chute_wind(from + tick, phase)));
        let (mut next_x, mut next_y, mut next_vx, mut next_vy) = if let Some(next) = next { (next.bob.x, next.bob.y, (next.bob.x - x) * RATE, (next.bob.y - y) * RATE) } else { let fallen = fall_step(y, vy); (x + vx / RATE, fallen.y, vx, fallen.vy) };
        let held = clamp(next_x, half, draft.stage.width - half);
        if held != next_x {
            let push = held - next_x;
            next_x = held;
            next_vx = if next.is_none() { -next_vx * 0.5 } else { 0.0 };
            if let Some(canopy) = &mut next { canopy.x += push; canopy.vx = 0.0; canopy.bob.x += push; canopy.previous.x += push; }
        }
        if next.is_none() && next_y - kind.size.height < 0.0 { next_y = kind.size.height; if next_vy < 0.0 { next_vy = -next_vy * 0.5; } }
        let tilt = if tick + 1 >= 24 { 0.0 } else { start.tilt * (1.0 - smoothstep((tick + 1) as f64 / 24.0)) };
        let after = extent_at(&owner, kind, Point { x: next_x, y: next_y }, tilt, next.map(|canopy| Point { x: canopy.x, y: canopy.y }));
        let perch = landing_of(&perches, next_x, y + hover, next_y + hover);
        let host = head_under(before, after, &owner, statics);
        if perch.is_some() || host.is_some() {
            let on_perch = perch.map_or(f64::INFINITY, |perch| perch.y - hover);
            let on_head = host.map_or(f64::INFINITY, |host| next_y + lift_onto(extent_at(&owner, kind, Point { x: next_x, y: next_y }, 0.0, None), host.extent));
            let head = host.is_some() && on_head <= on_perch;
            steps.push(Waypoint { x: next_x, y: if head { on_head } else { on_perch }, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None });
            return Course { owner, from, steps, ending: if head { Ending::Head } else { Ending::Perch }, landing: if head { host.unwrap().owner.clone() } else { perch.unwrap().surface.clone() }, touch: next_vy };
        }
        if next_y > draft.stage.height {
            steps.push(Waypoint { x: next_x, y: draft.stage.height, vx: next_vx, vy: next_vy, tilt, canopy: next });
            return Course { owner, from, steps, ending: Ending::Away, landing: String::new(), touch: next_vy };
        }
        steps.push(Waypoint { x: next_x, y: next_y, vx: next_vx, vy: next_vy, tilt, canopy: next });
        (x, y, vx, vy, canopy, before) = (next_x, next_y, next_vx, next_vy, next, after);
    }
    Course { owner, from, steps, ending: Ending::Away, landing: String::new(), touch: vy }
}

/// 🪂️ Decide when the actual plain landing warrants opening an owned parachute.
fn decision_of(kind: &Species, start: Waypoint, plain: &Course) -> Ticks {
    if !kind.gear.contains(&Gear::Parachute) || start.canopy.is_some() || plain.ending == Ending::Away || plain.touch <= HARD_LANDING { return -1; }
    let ground = plain.steps.last().unwrap().y;
    for opening in [CHUTE_OPENING, 0.0] {
        for tick in 0..plain.steps.len() {
            let state = if tick == 0 { &start } else { &plain.steps[tick - 1] };
            if state.vy >= opening && ground - state.y >= CHUTE_HEADROOM { return tick as Ticks; }
        }
    }
    -1
}

/// 🍂️ Chart a descent, opening its chute or braking a downward throw when necessary.
fn way_down(draft: &Draft<'_>, index: usize, start: Waypoint, from: Ticks, statics: &[Solid], target: f64) -> Plan {
    let kind = draft.kinds[index];
    let mut begin = start;
    for brace in 0..=4 {
        let plain = charted(draft, index, begin, from, -1, target, statics);
        let decision = decision_of(kind, begin, &plain);
        let course = if decision < 0 { plain } else { charted(draft, index, begin, from, decision, plain.steps.last().unwrap().x, statics) };
        if brace >= 4 || !kind.gear.contains(&Gear::Parachute) || begin.canopy.is_some() || course.ending == Ending::Away || course.touch <= HARD_LANDING || begin.vy <= 0.0 { return plan_of(draft, index, course); }
        begin.vy = if brace + 1 >= 4 { 0.0 } else { begin.vy / 2.0 };
        begin.canopy = None;
    }
    unreachable!()
}

/// 🧗️ Try straight, column-guided and steered descents in contract order.
pub(super) fn ways_down(draft: &Draft<'_>, index: usize, from: Ticks) -> Option<Plan> {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
    let statics: Vec<_> = draft.stage.actors.iter().enumerate().filter(|(other, body)| *other != index && body.footing == Footing::Perch && !draft.stage.claims.iter().any(|claim| claim.owner == body.species)).map(|(other, _)| bodies[other].clone()).collect();
    let start = Waypoint { x: body.x, y: body.y, vx: body.vx, vy: body.vy, tilt: body.tilt, canopy: body.chute.map(|chute| chute.canopy) };
    let straight = way_down(draft, index, start, from, &statics, body.x);
    if claim_clear(&straight.claim, &bodies, &draft.stage.claims) { return Some(straight); }
    let mut tries = Vec::new();
    if start.canopy.is_some() {
        for steer in &STEERINGS[1..] { tries.push((start, body.x + steer)); }
    } else {
        let course = &straight.course;
        let end = course.steps.last().unwrap();
        if let Some(perch) = (course.ending == Ending::Perch).then(|| perch_at(&draft.stage.perches, &course.landing, end.x)).flatten() {
            let column = column_over(perch, body.x, extent_at(&body.species, kind, Point { x: body.x, y: body.y }, 0.0, None), perch.y - hover_of(kind) - body.y, kind.size.width / 2.0, &obstacles_of(&body.species, &bodies, &draft.stage.claims, from));
            let steer = column.map_or(0.0, |column| clamp((column - body.x) * RATE / course.steps.len() as f64, -240.0, 240.0));
            if steer != 0.0 { tries.push((Waypoint { vx: body.vx + steer, ..start }, body.x)); }
        }
        for steer in &STEERINGS[1..] { tries.push((Waypoint { vx: body.vx + steer, ..start }, body.x)); }
        if body.vx != 0.0 || body.vy != 0.0 {
            tries.push((Waypoint { vx: -body.vx / 2.0, ..start }, body.x));
            for steer in STEERINGS { tries.push((Waypoint { vx: steer, vy: 0.0, ..start }, body.x)); }
        }
    }
    for (start, target) in tries { let plan = way_down(draft, index, start, from, &statics, target); if claim_clear(&plan.claim, &bodies, &draft.stage.claims) { return Some(plan); } }
    None
}
//#endregion 🔖️Courses
