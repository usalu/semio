//! 🛫️ Departure: setting out for a goal on a perch, attending a partner and the hop that launches a pet to another perch. See [`super`].

use super::*;
use semio_framework_pets_contract::schema::Ending;
use crate::spacing::{bodies_of, extent_at, obstacles_for};
use semio_framework_pets_clearance::{MARGIN, claim_clear, lane_lift, spot_on};
use semio_framework_pets_contract::schema::{Course, Waypoint};

//#region 🔖️Departure
/// 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way.
pub fn paced(kind: &Species, clip: Option<&str>, x: f64, goal: f64) -> f64 {
    let beat = beat_of(kind, clip_of(kind, clip));
    if beat == 1 {
        return goal;
    }
    let hop = (larger(kind.locomotion.speed, 0.0) * beat as f64) / RATE;
    let leaps = hop > 0.0;
    if !leaps {
        return x;
    }
    let hops = ((goal - x).abs() / hop).floor();
    if goal >= x {
        x + hops * hop
    } else {
        x - hops * hop
    }
}

/// 🚶️ An actor sets out for a goal on its perch; it turns towards it first when it faces the other way.
pub fn stroll(draft: &mut Draft<'_>, index: usize, goal: f64, clip: Option<Slug>, now: Ticks) {
    shift(draft, index, Activity::Walk, now);
    let until = now + dwell_of(Activity::Walk, draft.stage.mode, 0.0);
    let body = &mut draft.stage.actors[index];
    body.goal = goal;
    body.until = until;
    body.clip = clip;
}

/// 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience.
pub fn attend(draft: &mut Draft<'_>, index: usize, now: Ticks) {
    shift(draft, index, Activity::Idle, now);
    let clip = clip_at(draft.kinds[index], Activity::Idle, 0.0);
    let body = &mut draft.stage.actors[index];
    body.until = now + PATIENCE;
    body.clip = clip;
    body.goal = body.x;
}

/// 🦘️ Where an actor could hop to: per other perch (in perch order) the place nearest to it that keeps half a body from the ends and a comfortable gap from the actors there, when `hop_of` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ([`soars`]); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds.
pub fn hops_of(draft: &Draft<'_>, index: usize) -> Vec<Launch> {
    let actors = &draft.stage.actors;
    let body = &actors[index];
    let kind = draft.kinds[index];
    let half = kind.size.width / 2.0;
    let hover = hover_of(kind);
    let obstacles = obstacles_for(draft, index, draft.stage.tick);
    let mut launches = Vec::new();
    for perch in &draft.stage.perches {
        if body.perch.as_deref() == Some(perch.surface.as_str()) && perch.x0 <= body.x && body.x <= perch.x1 {
            continue;
        }
        let low = perch.x0 + half;
        let high = perch.x1 - half;
        if high < low {
            continue;
        }
        let inset = smaller(half, (high - low) / 2.0);
        let aimed = clamp(body.x, low + inset, high - inset);
        let Some(x) = spot_on(perch, aimed, extent_at(&body.species, kind, Point { x: aimed, y: perch.y - hover }, 0.0, None), half, &obstacles) else { continue; };
        if kind.locomotion.gait == Gait::Float {
            let dx = x - body.x;
            let dy = perch.y - hover - body.y;
            if dx.abs() > HOP_DISTANCE || dy.abs() > HOP_HEIGHT {
                continue;
            }
            let ticks = (((dx * dx + dy * dy).sqrt() * RATE) / (2.0 * larger(kind.locomotion.speed, 1.0)) + 0.5).floor();
            if ticks < 1.0 || ticks > GLIDE_TICKS as f64 {
                continue;
            }
            launches.push(Launch { x, y: perch.y - hover, surface: perch.surface.clone(), vx: (dx * RATE) / ticks, vy: (dy * RATE) / ticks, ticks: ticks as Ticks });
            continue;
        }
        let from = Point { x: body.x, y: body.y };
        let to = Point { x, y: perch.y };
        let Some(hop) = hop_of(from, to) else {
            continue;
        };
        let Some(landing) = hop_landing(&draft.stage.perches, from, to, hop) else {
            continue;
        };
        if landing.surface != perch.surface || landing.x0 != perch.x0 || !soars(draft, kind, from, to, hop) {
            continue;
        }
        launches.push(Launch { x, y: perch.y, surface: perch.surface.clone(), vx: hop.vx, vy: hop.vy, ticks: hop.ticks });
    }
    launches
}

/// 🚀️ Commit a hop only when its entire arc or floating lane and landing remain clear.
pub fn launch(draft: &mut Draft<'_>, index: usize, chosen: &Launch, clip: Option<Slug>, now: Ticks) -> bool {
    let body = &draft.stage.actors[index];
    let kind = draft.kinds[index];
    let mut courses = Vec::new();
    if kind.locomotion.gait == Gait::Float {
        for lane in [0.0, lane_lift(kind.size.height, MARGIN)] {
            let steps = (1..=chosen.ticks).map(|tick| {
                let ramp = smaller(1.0, smaller(tick as f64 / LANE_RAMP, (chosen.ticks - tick) as f64 / LANE_RAMP));
                if tick == chosen.ticks { Waypoint { x: chosen.x, y: chosen.y, vx: 0.0, vy: 0.0, tilt: 0.0, canopy: None } } else { Waypoint { x: body.x + ((chosen.x - body.x) * tick as f64) / chosen.ticks as f64, y: body.y + ((chosen.y - body.y) * tick as f64) / chosen.ticks as f64 - lane * ramp, vx: chosen.vx, vy: chosen.vy, tilt: 0.0, canopy: None } }
            }).collect();
            courses.push(Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: chosen.surface.clone(), touch: 0.0 });
        }
    } else {
        let mut steps = Vec::new();
        let (mut x, mut y, mut vy) = (body.x, body.y, chosen.vy);
        for left in (1..=chosen.ticks).rev() {
            let flight = hop_step(x, y, chosen.vx, vy, Point { x: chosen.x, y: chosen.y }, left);
            steps.push(Waypoint { x: flight.x, y: flight.y, vx: flight.vx, vy: flight.vy, tilt: 0.0, canopy: None });
            (x, y, vy) = (flight.x, flight.y, flight.vy);
        }
        courses.push(Course { owner: body.species.clone(), from: now + 1, steps, ending: Ending::Perch, landing: chosen.surface.clone(), touch: vy });
    }
    let bodies = bodies_of(&draft.stage.actors, &draft.kinds);
    for course in courses {
        let plan = plan_of(draft, index, course);
        if !claim_clear(&plan.claim, &bodies, &draft.stage.claims) { continue; }
        install(draft, index, plan, Activity::Hop, now);
        draft.stage.actors[index].clip = clip;
        return true;
    }
    false
}
//#endregion 🔖️Departure
