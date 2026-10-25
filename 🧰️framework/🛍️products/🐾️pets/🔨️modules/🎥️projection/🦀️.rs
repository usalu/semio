//! 📽️ The projection of a stage into a frame: the pose of every actor (the idle loop underneath, the clip of its activity on top of it or in its place), the rate its motion needs (`pace_of`), and `frame_of` — the actors back to front with their matrices, eyes, spirits and mood, the rate of the frame and the tick it must wake at.
//!
//! Whatever a render target is to draw is projected here, from the stage alone.
//! A part of the stage, not of the crate: everything is `pub(crate)` at most.
//!
//! @see ../🎪️stage/🦀️.rs — the façade of the stage and the normative order of a tick
//! @see ../🎥️projection/🟦️.ts — the TypeScript twin

use semio_framework_pets_staging_draft::attention::{gaze_rests, leant, presence_of, squeeze_of, PERK_LINGER, POINTER_TICKS, TURN_REST};
use semio_framework_pets_staging_draft::draft::{breath_of, clip_of, RATE};
use semio_framework_pets_staging_draft::schedule::{arrival_tick, pairing_tick};
use semio_framework_pets_character::feeling::{settled, settles_at, spirits_of};
use semio_framework_pets_contract::schema::{Activity, Actor, ActorFrame, EyeFrame, Facing, Frame, Menagerie, PetMode, Point, Rate, Rect, Species, Stage, Ticks};
use semio_framework_pets_kinematics::animation::{blend_pose, clip_ticks, lid_at, sample_clip, BLINK_TICKS};
use semio_framework_pets_kinematics::rig::{pupil_reach, rest_pose, solve_rig, BonePose, Pose};
use semio_framework_pets_kinematics::terrain::{larger, smaller};
use semio_framework_pets_kinematics::trigonometry::clamp;
use std::borrow::Borrow;
use semio_framework_pets_contract::schema::Footing;
use semio_framework_pets_contract::schema::Channel;
use semio_framework_pets_contract::schema::ToolFrame;
use semio_framework_pets_contract::schema::Drift;
use semio_framework_pets_contract::schema::PressPhase;
use semio_framework_pets_contract::schema::Clip;
use semio_framework_pets_character::feeling::Standing;
use semio_framework_pets_interaction::effects::Particle;
use semio_framework_pets_contract::schema::ParticleFrame;
use semio_framework_pets_contract::schema::LadderFrame;
use semio_framework_pets_contract::schema::LiftFrame;
use semio_framework_pets_contract::schema::PuffFrame;
use semio_framework_pets_character::feeling::{at_rest, face_of, state_at, state_ends};
use semio_framework_pets_contract::schema::{ChuteTool, GunTool, HookTool, LadderTool, RopeTool};
use semio_framework_pets_interaction::effects::{STAGE_CAP, capped, emitter_ends, emitter_key, particles_of};
use semio_framework_pets_interaction::gesture::{hover_busy, press_due};
use semio_framework_pets_interaction::mischief::{LIFT_TICKS, lift_at, lift_ends, lift_wake};
use semio_framework_pets_kinematics::trigonometry::{atan_turns, cos_turns, sin_turns, smoothstep};
use semio_framework_pets_staging_draft::schedule::{beat_tick, prank_tick};
use semio_framework_pets_staging_draft::spacing::{extent_of, wall_lean_of};
use semio_framework_pets_traversal::climbing::{HOOK_RETURN, HOOK_SPEED, LadderStand, hook_step, hook_ticks, ladder_rungs};
use semio_framework_pets_traversal::swing::{CHUTE_ROD, lean_of};

//#region 🔖️Constants
const BLEND_TICKS: Ticks = 8;
const BREATH_STAGGER: Ticks = 37;
pub(crate) const STATE_BLEND: Ticks = 16;
const LADDER_FADE: Ticks = 8;
const BRISK: f64 = 32.0;
const PUFF_TICKS: Ticks = 48;
//#endregion 🔖️Constants

//#region 🔖️Frame
/// 🧱️ Whether the clip of an activity replaces the idle loop underneath it (cross-fade) instead of being added on top of it: the gaits, the flights, the landing and sleep bring their own body motion.
fn replaces(activity: Activity) -> bool {
    matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall | Activity::Land | Activity::Sleep | Activity::Tumble | Activity::Scoot | Activity::Carry | Activity::Climb | Activity::Slide | Activity::Mantle)
}

/// 🎛️ How much of its clip an actor shows: it fades in over the first 8 ticks of the activity and out over the last 8 — of its span, or of the way to its goal when it walks; flights end hard.
fn weight_of(kind: &Species, actor: &Actor, tick: Ticks) -> f64 {
    let activity = actor.activity;
    if clinging(actor) { return 1.0; }
    let into = (tick - actor.since) as f64 / BLEND_TICKS as f64;
    let out = if matches!(activity, Activity::Hop | Activity::Fall | Activity::Tumble) {
        1.0
    } else if matches!(activity, Activity::Walk | Activity::Scoot) {
        ((actor.goal - actor.x).abs() * RATE) / (larger(kind.locomotion.speed, 1.0) * BLEND_TICKS as f64)
    } else {
        (actor.until - tick) as f64 / BLEND_TICKS as f64
    };
    clamp(smaller(into, out), 0.0, 1.0)
}

/// 🧩️ One pose on top of another: offsets and rotations add, scale factors multiply.
fn layer(under: &[BonePose], over: &[BonePose]) -> Pose {
    under
        .iter()
        .enumerate()
        .map(|(index, bone)| BonePose { x: bone.x + over[index].x, y: bone.y + over[index].y, rotation: bone.rotation + over[index].rotation, scale_x: bone.scale_x * over[index].scale_x, scale_y: bone.scale_y * over[index].scale_y })
        .collect()
}

/// 🤸️ The pose of an actor at a tick: the first idle clip of its species loops underneath on the clock of the stage (staggered by 37 ticks per stream, so a pet that stands still is never frozen and no two breathe in step), and the clip of its activity, weighted by [`weight_of`] from the rest pose, lies on top of it or takes its place. A walker that still turns round has not set out yet: its walk weighs nothing.
fn pose_of(stage: &Stage, kind: &Species, actor: &Actor, tick: Ticks, stream: Ticks, present: Option<&Clip>, former: Option<&Clip>, blend: f64) -> Pose {
    let rest = rest_pose(kind);
    let breath = breath_of(kind);
    let phase = tick + stream * BREATH_STAGGER;
    let idle = breath.map_or_else(|| rest.clone(), |breath| sample_clip(kind, breath, phase));
    let Some(top) = clip_of(kind, actor.clip.as_deref()).filter(|top| breath.is_none_or(|breath| top.id != breath.id)) else {
        return looked_of(kind, idle, present, former, blend, phase);
    };
    let weight = if tick < actor.faced && matches!(actor.activity, Activity::Walk | Activity::Scoot) { 0.0 } else { weight_of(kind, actor, tick) };
    let over = blend_pose(&rest, &sample_clip(kind, top, clip_time(stage, kind, actor, top, tick)), weight);
    let under = if replaces(actor.activity) { blend_pose(&idle, &rest, if actor.activity == Activity::Land { clamp((actor.until - tick) as f64 / BLEND_TICKS as f64, 0.0, 1.0) } else { weight }) } else { idle };
    layer(&looked_of(kind, under, present, former, blend, phase), &over)
}

/// 🫳️ A climber rests in its climbing pose on a wall or ladder.
fn clinging(actor: &Actor) -> bool {
    actor.activity == Activity::Climb && matches!(actor.footing, Footing::Wall | Footing::Ladder)
}

/// 🎡️ The clip's tick follows holds or rungs while climbing, and elapsed time otherwise.
fn clip_time(stage: &Stage, kind: &Species, actor: &Actor, clip: &Clip, tick: Ticks) -> Ticks {
    if !clinging(actor) { return tick - actor.since; }
    let phase = if actor.footing == Footing::Wall {
        actor.pitch.as_ref().map_or(0.0, |pitch| semio_framework_pets_traversal::climbing::climb_phase(pitch, actor.y, kind.size))
    } else {
        stage.ladders.iter().find(|ladder| ladder.rider.as_ref() == Some(&actor.species)).map_or(0.0, |ladder| {
            let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
            semio_framework_pets_traversal::climbing::ladder_phase(clamp((actor.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0.0, 1.0) * semio_framework_pets_traversal::climbing::ladder_length(&stand))
        })
    };
    (phase * clip_ticks(clip) as f64).floor() as Ticks
}

/// 🎭️ The authored clip for a state's look.
fn look_of<'a>(kind: &'a Species, state: &str) -> Option<&'a Clip> {
    kind.states.iter().find(|entry| entry.id == state).and_then(|entry| clip_of(kind, entry.clip.as_deref()))
}

/// 🗝️ Five keyed channels per bone, in rig order.
fn keys_of(kind: &Species, clip: Option<&Clip>) -> Option<Vec<bool>> {
    let clip = clip?;
    let mut keyed = vec![false; kind.bones.len() * 5];
    for track in &clip.tracks {
        let Some(bone) = kind.bones.iter().position(|bone| bone.id == track.bone) else { continue; };
        let channel = match track.channel { Channel::X => 0, Channel::Y => 1, Channel::Rotation => 2, Channel::ScaleX => 3, Channel::ScaleY => 4 };
        keyed[bone * 5 + channel] = true;
    }
    Some(keyed)
}

/// 🎚️ A pose channel by its contract position.
fn channel_of(bone: &BonePose, channel: usize) -> f64 {
    match channel { 0 => bone.x, 1 => bone.y, 2 => bone.rotation, 3 => bone.scale_x, _ => bone.scale_y }
}

/// 🌗️ Blend the keyed channels of successive state looks over the breathing underlay.
fn looked_of(kind: &Species, under: Pose, present: Option<&Clip>, former: Option<&Clip>, blend: f64, phase: Ticks) -> Pose {
    let done = blend >= 1.0 || present == former;
    if present.is_none() && (done || former.is_none()) { return under; }
    let now = present.map(|clip| sample_clip(kind, clip, phase));
    let now_keys = keys_of(kind, present);
    let was = if done { None } else { former.map(|clip| sample_clip(kind, clip, phase)) };
    let was_keys = if done { None } else { keys_of(kind, former) };
    under.iter().enumerate().map(|(index, bone)| {
        let values: Vec<f64> = (0..5).map(|channel| {
            let base = channel_of(bone, channel);
            let to = now.as_ref().filter(|_| now_keys.as_ref().is_some_and(|keys| keys[index * 5 + channel])).map_or(base, |pose| channel_of(&pose[index], channel));
            let from = was.as_ref().filter(|_| was_keys.as_ref().is_some_and(|keys| keys[index * 5 + channel])).map_or(base, |pose| channel_of(&pose[index], channel));
            if done { to } else { from + (to - from) * blend }
        }).collect();
        BonePose { x: values[0], y: values[1], rotation: values[2], scale_x: values[3], scale_y: values[4] }
    }).collect()
}

/// 🪶️ Sink the first eye's bone by the mood's posture.
fn drooped(kind: &Species, mut pose: Pose, drop: f64) -> Pose {
    if let Some(eye) = kind.face.eyes.first().filter(|_| drop != 0.0) {
        if let Some(index) = kind.bones.iter().position(|bone| bone.id == eye.bone) { pose[index].y += drop; }
    }
    pose
}

/// 🚩️ The current timed state, the tick it began and the look it replaces.
fn looking_of(kind: &Species, actor: &Actor, tick: Ticks) -> (Standing, String) {
    let standing = state_at(kind, &actor.state, actor.state_since, tick);
    let former = if standing.since == actor.state_since { actor.former.clone() } else { state_at(kind, &actor.state, actor.state_since, standing.since - 1).state };
    (standing, former)
}

/// 📍️ Place the rig so rotating it about its pivot leaves its feet on the actor's feet.
fn placed(feet: Point, facing: Facing, tilt: f64, pivot: Point) -> Point {
    if tilt == 0.0 { return feet; }
    let sine = sin_turns(tilt);
    let cosine = cos_turns(tilt);
    let across = facing.sign() * pivot.x;
    Point { x: feet.x + (cosine * across - sine * pivot.y) - across, y: feet.y + (sine * across + cosine * pivot.y) - pivot.y }
}

/// 🗺️ Carry a rig point through mirroring, its pivot and the stage tilt.
fn staged(origin: Point, facing: Facing, tilt: f64, pivot: Point, x: f64, y: f64) -> Point {
    let across = facing.sign() * x;
    if tilt == 0.0 { return Point { x: origin.x + across, y: origin.y + y }; }
    let sine = sin_turns(tilt);
    let cosine = cos_turns(tilt);
    let turn = facing.sign() * pivot.x;
    let dx = across - turn;
    let dy = y - pivot.y;
    Point { x: origin.x + turn + (cosine * dx - sine * dy), y: origin.y + pivot.y + (sine * dx + cosine * dy) }
}

/// 🔧️ Project a canopy, carried ladder and grappling line from their event anchors.
fn tools_of(kind: &Species, actor: &Actor, tick: Ticks) -> Vec<ToolFrame> {
    let mut tools = Vec::new();
    if let Some(chute) = &actor.chute {
        tools.push(ToolFrame::Chute(ChuteTool { open: chute.open, sway: lean_of(Point { x: chute.canopy.x, y: chute.canopy.y }, Point { x: actor.x, y: actor.y }, CHUTE_ROD * kind.size.height) }));
    }
    if actor.activity == Activity::Carry { tools.push(ToolFrame::Ladder(LadderTool { lean: actor.facing.sign() * 0.22, length: 2.0 * kind.size.height })); }
    if let Some(rope) = &actor.rope {
        let shot = &rope.shot;
        let flown = tick - rope.since;
        let out = hook_ticks(shot.muzzle, shot.hook, HOOK_SPEED);
        let flying = flown < out || !rope.caught;
        let hook = if flown < out { hook_step(shot.muzzle, shot.hook, HOOK_SPEED, flown) } else if rope.caught { shot.hook } else { hook_step(shot.hook, shot.muzzle, HOOK_RETURN, flown - out) };
        tools.push(ToolFrame::Rope(RopeTool { x: hook.x, y: hook.y, slack: 0.0 }));
        tools.push(ToolFrame::Hook(HookTool { x: hook.x, y: hook.y }));
        if flying { tools.push(ToolFrame::Gun(GunTool { aim: atan_turns(shot.hook.y - shot.muzzle.y, shot.hook.x - shot.muzzle.x) })); }
    } else if actor.activity == Activity::Aim { tools.push(ToolFrame::Gun(GunTool { aim: if actor.facing == Facing::Right { -0.125 } else { 0.125 - 0.5 } })); }
    tools
}

/// 🎇️ A particle retains its emitter and actor strength through the stage cap.
struct Spark { particle: Particle, species: String, emitter: String, opacity: f64 }

/// 💥️ Emit every active plume through its bone's matrix and the actor's stage transform.
fn sparks_of(stage: &Stage, kind: &Species, actor: &Actor, stream: usize, frame: &ActorFrame, sparks: &mut Vec<Spark>) {
    for plume in &actor.emitters {
        let Some(index) = kind.emitters.iter().position(|entry| entry.id == plume.emitter) else { continue; };
        let emitter = &kind.emitters[index];
        let bone = kind.bones.iter().position(|entry| entry.id == emitter.bone).unwrap_or(0) * 6;
        let x = frame.bones[bone] * emitter.x + frame.bones[bone + 2] * emitter.y + frame.bones[bone + 4];
        let y = frame.bones[bone + 1] * emitter.x + frame.bones[bone + 3] * emitter.y + frame.bones[bone + 5];
        let origin = staged(Point { x: frame.x, y: frame.y }, actor.facing, frame.tilt, frame.pivot, x, y);
        for particle in particles_of(emitter.into(), origin, actor.facing, plume.since, plume.until, stage.tick, emitter_key(stage.seed, stream, index, plume.since)) {
            sparks.push(Spark { particle, species: actor.species.clone(), emitter: emitter.id.clone(), opacity: frame.opacity });
        }
    }
}

/// 🎆️ Keep the youngest particles in their authored order with their actor's opacity.
fn particles_from(sparks: Vec<Spark>) -> Vec<ParticleFrame> {
    let mut kept = capped(sparks.iter().map(|spark| spark.particle).collect(), STAGE_CAP).into_iter().peekable();
    sparks.into_iter().filter_map(|spark| {
        if kept.peek() != Some(&spark.particle) { return None; }
        kept.next();
        let particle = spark.particle;
        Some(ParticleFrame { species: spark.species, emitter: spark.emitter, x: particle.x, y: particle.y, scale: particle.scale, rotation: particle.rotation, opacity: particle.opacity * spark.opacity })
    }).collect()
}

/// 💨️ The fastest active emitter determines the plume's sampling rate.
fn plume_pace(kind: &Species, actor: &Actor, tick: Ticks) -> Rate {
    let mut pace = Rate::Rest;
    for plume in &actor.emitters {
        let Some(emitter) = kind.emitters.iter().find(|entry| entry.id == plume.emitter) else { continue; };
        if tick < plume.since || emitter_ends(emitter.into(), plume.since, plume.until).is_some_and(|ends| tick >= ends) { continue; }
        if emitter.motion == Drift::Burst || emitter.speed >= BRISK { return Rate::Full; }
        pace = Rate::Half;
    }
    pace
}

/// 🪜️ Standing ladders fade at their anchored begin and end ticks.
fn ladders_of(stage: &Stage, still: bool) -> Vec<LadderFrame> {
    stage.ladders.iter().filter_map(|ladder| {
        let opacity = if still { if stage.tick >= ladder.since && stage.tick < ladder.until { 1.0 } else { 0.0 } } else { smaller(smoothstep((stage.tick - ladder.since) as f64 / LADDER_FADE as f64), smoothstep((ladder.until - stage.tick) as f64 / LADDER_FADE as f64)) };
        if opacity <= 0.0 { return None; }
        let stand = LadderStand { wall: ladder.wall.clone(), surface: ladder.surface.clone(), side: ladder.side, foot: ladder.foot, top: ladder.top };
        Some(LadderFrame { x0: ladder.foot.x, y0: ladder.foot.y, x1: ladder.top.x, y1: ladder.top.y, rungs: ladder_rungs(&stand), opacity })
    }).collect()
}

/// 🪞️ A lifted fixture follows its deterministic departure and return.
fn lifts_of(stage: &Stage, still: bool) -> Vec<LiftFrame> {
    let Some(lift) = stage.lift.as_ref().filter(|lift| !still && stage.tick >= lift.since && stage.tick < lift.since + LIFT_TICKS) else { return Vec::new(); };
    let copy = lift_at(lift.since, stage.tick, lift.side, lift.room, lift.span, lift.unit);
    vec![LiftFrame { fixture: lift.fixture.clone(), dx: copy.dx, dy: copy.dy, tilt: copy.tilt, opacity: copy.opacity }]
}

/// 🌫️ Vanished actors leave a bounded dust phase.
fn puffs_of(stage: &Stage, still: bool) -> Vec<PuffFrame> {
    if still { return Vec::new(); }
    stage.puffs.iter().filter_map(|puff| {
        let phase = (stage.tick - puff.tick) as f64 / PUFF_TICKS as f64;
        (phase >= 0.0 && phase < 1.0).then_some(PuffFrame { x: puff.x, y: puff.y, width: puff.width, height: puff.height, phase })
    }).collect()
}

/// 🎠️ Moving scenery needs every tick even when all actors rest.
fn scenery_pace(stage: &Stage) -> Rate {
    let tick = stage.tick;
    if stage.lift.as_ref().is_some_and(|lift| lift_wake(lift.since, tick) == Some(tick + 1)) || stage.puffs.iter().any(|puff| tick >= puff.tick && tick < puff.tick + PUFF_TICKS) || stage.ladders.iter().any(|ladder| (tick >= ladder.since && tick < ladder.since + LADDER_FADE) || (tick >= ladder.until - LADDER_FADE && tick < ladder.until)) { Rate::Full } else { Rate::Rest }
}

/// ⏰️ Sleeping frames wake for an emitter, lift or ladder's next boundary.
fn scenery_wake(stage: &Stage, actors: &[&Actor], kinds: &[&Species], tick: Ticks) -> Ticks {
    let mut horizon = Ticks::MAX;
    for (index, actor) in actors.iter().enumerate() {
        for plume in &actor.emitters {
            let Some(emitter) = kinds[index].emitters.iter().find(|entry| entry.id == plume.emitter) else { continue; };
            let ends = emitter_ends(emitter.into(), plume.since, plume.until);
            if plume.since > tick { horizon = Ticks::min(horizon, plume.since); } else if let Some(ends) = ends.filter(|ends| *ends > tick) { horizon = Ticks::min(horizon, ends); }
        }
    }
    if let Some(lift) = &stage.lift {
        if let Some(next) = lift_wake(lift.since, tick) { horizon = Ticks::min(horizon, next); }
        if lift_ends(lift.since) > tick { horizon = Ticks::min(horizon, lift_ends(lift.since)); }
    }
    for ladder in &stage.ladders {
        if ladder.since > tick { horizon = Ticks::min(horizon, ladder.since); } else if ladder.until - LADDER_FADE > tick { horizon = Ticks::min(horizon, ladder.until - LADDER_FADE); }
    }
    horizon
}

/// 🎼️ How many ticks per second the motion of one actor needs at a tick: 64 while it fades (in, out, or see-through under the pointer and back), turns round, walks, flies, lands, plays a clip that does not loop or blends a clip in or out; 32 while a loop, a blink, its pupils or its feeling move (until it `settles_at` its rest); 16 while it only sleeps; 0 when nothing of it moves.
fn pace_of<A: Borrow<Actor>>(stage: &Stage, actors: &[A], kinds: &[&Species], index: usize) -> Rate {
    let actor: &Actor = actors[index].borrow();
    let kind = kinds[index];
    let tick = stage.tick;
    let activity = actor.activity;
    let held = matches!(actor.footing, Footing::Perch | Footing::Wall | Footing::Ladder);
    if !held || actor.leaving || actor.opacity != presence_of(&stage.keepouts, actor, kind) || tick < actor.faced || stage.trips.iter().any(|trip| trip.owner == actor.species) || matches!(activity, Activity::Walk | Activity::Hop | Activity::Fall | Activity::Land | Activity::Scoot | Activity::Carry) || actor.rope.is_some() || hover_busy(&actor.hover) {
        return Rate::Full;
    }
    let breath = breath_of(kind);
    let resting = clinging(actor);
    let layered = clip_of(kind, actor.clip.as_deref()).filter(|top| breath.is_none_or(|breath| top.id != breath.id) && !resting);
    if layered.is_some_and(|top| (!top.looping && tick - actor.since < clip_ticks(top)) || tick - actor.since < BLEND_TICKS || actor.until - tick < BLEND_TICKS) {
        return Rate::Full;
    }
    let (looking, former) = looking_of(kind, actor, tick);
    let look = look_of(kind, &looking.state);
    if tick - looking.since < STATE_BLEND && former != looking.state && (look.is_some() || look_of(kind, &former).is_some()) { return Rate::Full; }
    let plumes = plume_pace(kind, actor, tick);
    if plumes == Rate::Full { return Rate::Full; }
    let restless = settles_at(actor.feeling, kind.mood) > tick || !gaze_rests(stage, actors, kinds, index, tick + 1) || plumes == Rate::Half;
    let looping = look.is_some_and(|look| look.looping) || (breath.is_some() && !resting);
    if activity == Activity::Sleep {
        return if restless {
            Rate::Half
        } else if layered.is_some() || looping {
            Rate::Quarter
        } else {
            Rate::Rest
        };
    }
    if restless || (tick >= actor.blink && tick < actor.blink + BLINK_TICKS) {
        return Rate::Half;
    }
    if layered.is_some_and(|top| top.looping) || looping {
        Rate::Half
    } else {
        Rate::Rest
    }
}

/// ⬆️ Whether one actor is drawn before another: the one higher up (the smaller `y`) first, at the same height the species id that is smaller by UTF-16 code units, as the twin compares strings.
fn behind(one: &ActorFrame, other: &ActorFrame) -> bool {
    let rise = one.y - other.y;
    if rise < 0.0 {
        return true;
    }
    if rise > 0.0 {
        return false;
    }
    one.species.encode_utf16().lt(other.species.encode_utf16())
}

/// 🎥️ The complete stage frame: sorted actor poses, eyes, moods, body extents, held tools and scruff motion; standing ladders, particles, lifted fixtures and puffs; and the rate plus next scheduled wake tick.
///
/// A still stage shows every actor in its rest pose with open eyes and centred pupils at rate 0 without a wake tick —
/// see-through at once while the pointer rests on it, since nothing eases there — and so does an empty one that nobody
/// waits to enter. At rate 0 otherwise (species without an idle loop, pupils at rest, between blinks) `wake` is the
/// earliest of the next blink, the end of an activity, the pointer losing its interest, having come to rest for half
/// a second or being allowed to turn an actor again, the next whole second on which a pair may be drawn and the next
/// whole second on which somebody who waits off stage may arrive.
pub fn frame_of(menagerie: &Menagerie, stage: &Stage) -> Frame {
    let mut actors: Vec<&Actor> = Vec::with_capacity(stage.actors.len());
    let mut kinds: Vec<&Species> = Vec::with_capacity(stage.actors.len());
    let mut streams: Vec<Ticks> = Vec::with_capacity(stage.actors.len());
    for (stream, kind) in menagerie.species.iter().enumerate() {
        if let Some(actor) = stage.actors.iter().find(|actor| actor.species == kind.id) {
            actors.push(actor);
            kinds.push(kind);
            streams.push(stream as Ticks);
        }
    }
    let tick = stage.tick;
    let still = stage.mode == PetMode::Still;
    let frames: Vec<ActorFrame> = (0..actors.len())
        .map(|index| {
            let actor = actors[index];
            let kind = kinds[index];
            let (looking, former) = looking_of(kind, actor, tick);
            let present = look_of(kind, &looking.state);
            let feeling = if still { at_rest(kind.mood, tick) } else { settled(actor.feeling, kind.mood, tick) };
            let face = face_of(feeling);
            let lid = if still {
                0.0
            } else if actor.activity == Activity::Sleep {
                1.0
            } else {
                face.lid + (1.0 - face.lid) * lid_at(tick - actor.blink)
            };
            let squeeze = if still { 1.0 } else { squeeze_of(actor, tick) };
            let forward = if squeeze < 0.0 { actor.facing == Facing::Left } else { actor.facing == Facing::Right };
            let across = if forward { actor.gaze.x } else { 0.0 - actor.gaze.x };
            let eyes = kind
                .face
                .eyes
                .iter()
                .map(|eye| {
                    let span = pupil_reach(eye);
                    EyeFrame { x: across * span, y: actor.gaze.y * span, lid }
                })
                .collect();
            let pose = if still { looked_of(kind, rest_pose(kind), present, None, 1.0, 0) } else { drooped(kind, pose_of(stage, kind, actor, tick, streams[index], present, look_of(kind, &former), smoothstep((tick - looking.since) as f64 / STATE_BLEND as f64)), face.drop) };
            let mut bones = solve_rig(kind, &if still || stage.quiet { pose } else { leant(kind, pose, across, actor.gaze.y) });
            if squeeze != 1.0 {
                for entry in bones.iter_mut().step_by(2) {
                    *entry *= squeeze;
                }
            }
            let tilt = if still { 0.0 } else { actor.tilt + wall_lean_of(kind, actor.footing, actor.pitch.as_ref(), actor.x) };
            let pivot = if actor.footing == Footing::Rope { Point { x: semio_framework_pets_traversal::climbing::MUZZLE_FORWARD * kind.size.width, y: 0.0 - semio_framework_pets_traversal::climbing::MUZZLE_HEIGHT * kind.size.height } } else { Point { x: 0.0, y: -kind.grip } };
            let origin = placed(Point { x: actor.x, y: actor.y }, actor.facing, tilt, pivot);
            let extent = extent_of(actor, kind);
            ActorFrame {
                species: actor.species.clone(),
                x: origin.x,
                y: origin.y,
                facing: actor.facing,
                activity: actor.activity,
                opacity: actor.opacity,
                bones,
                eyes,
                footing: actor.footing,
                state: if !still && tick - looking.since < STATE_BLEND / 2 { former } else { looking.state },
                mood: feeling.mood,
                intensity: feeling.intensity,
                spirits: spirits_of(feeling),
                tilt,
                pivot,
                tools: if still { Vec::new() } else { tools_of(kind, actor, tick) },
                body: Rect { x: extent.x0, y: extent.y0, width: extent.x1 - extent.x0, height: extent.y1 - extent.y0 },
            }
        })
        .collect();
    let mut order: Vec<usize> = (0..frames.len()).collect();
    for sorted in 1..order.len() {
        let mut at = sorted;
        while at > 0 && behind(&frames[order[at]], &frames[order[at - 1]]) { order.swap(at, at - 1); at -= 1; }
    }
    let mut sparks = Vec::new();
    if !still {
        for &index in &order { sparks_of(stage, kinds[index], actors[index], streams[index] as usize, &frames[index], &mut sparks); }
    }
    let mut rate = Rate::Rest;
    if !still {
        for index in 0..actors.len() {
            let pace = pace_of(stage, &actors, &kinds, index);
            if pace > rate {
                rate = pace;
            }
        }
        if scenery_pace(stage) > rate { rate = Rate::Full; }
        if stage.press.phase != PressPhase::Idle && press_due(stage.press).is_none() { rate = Rate::Full; }
    }
    let mut wake = None;
    let arrival = arrival_tick(menagerie, stage, &actors, tick + 1);
    if rate == Rate::Rest && !still && (!actors.is_empty() || arrival >= 0 || !stage.ladders.is_empty() || stage.lift.is_some()) {
        let mut horizon = Ticks::MAX;
        for actor in &actors {
            horizon = Ticks::min(horizon, actor.until);
            if actor.activity != Activity::Sleep {
                horizon = Ticks::min(horizon, if actor.blink > tick { actor.blink } else { actor.blink + BLINK_TICKS });
            }
            if stage.pointer.is_some() && tick + 1 < actor.faced + TURN_REST {
                horizon = Ticks::min(horizon, actor.faced + TURN_REST);
            }
        }
        if !actors.is_empty() && stage.pointer.is_some() && tick < stage.pointed + PERK_LINGER {
            horizon = Ticks::min(horizon, stage.pointed + PERK_LINGER);
        }
        if !actors.is_empty() && stage.pointer.is_some() && tick + 1 - stage.pointed < POINTER_TICKS {
            horizon = Ticks::min(horizon, stage.pointed + POINTER_TICKS);
        }
        let pairing = pairing_tick(stage, &actors, tick + 1);
        if pairing >= 0 {
            horizon = Ticks::min(horizon, pairing);
        }
        if arrival >= 0 {
            horizon = Ticks::min(horizon, arrival);
        }
        for (index, actor) in actors.iter().enumerate() {
            let (looking, _) = looking_of(kinds[index], actor, tick);
            if let Some(ends) = state_ends(kinds[index], &looking.state, looking.since) { horizon = Ticks::min(horizon, ends); }
        }
        horizon = Ticks::min(horizon, scenery_wake(stage, &actors, &kinds, tick));
        let beating = beat_tick(menagerie, stage, tick + 1);
        if beating >= 0 { horizon = Ticks::min(horizon, beating); }
        let pranking = prank_tick(menagerie, stage, tick + 1);
        if pranking >= 0 { horizon = Ticks::min(horizon, pranking); }
        if let Some(due) = press_due(stage.press) { horizon = Ticks::min(horizon, due); }
        for puff in &stage.puffs { horizon = Ticks::min(horizon, puff.tick + PUFF_TICKS); }
        wake = Some(if horizon > tick { horizon } else { tick + 1 });
    }
    let held = actors.iter().find(|actor| actor.hang.is_some()).map(|actor| actor.species.clone());
    Frame { tick, actors: order.into_iter().map(|index| frames[index].clone()).collect(), rate, wake, ladders: ladders_of(stage, still), particles: particles_from(sparks), lifts: lifts_of(stage, still), puffs: puffs_of(stage, still), held }
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
