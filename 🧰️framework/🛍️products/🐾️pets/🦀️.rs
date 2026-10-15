//! 🐾️ Pets product façade — render-independent deterministic pets and their pure core, as the one crate the subject hosts of the language-agnostic cases link (`🧪️tests/<case>/🦀️.rs`, lib `pets`). The core itself is carved into producer crates along its domain seams, each below the next:
//!
//! - `semio-framework-pets-contract` — the contract types (menageries of rigged species, the stage, its events and frames) and their validation;
//! - `semio-framework-pets-kinematics` — trigonometry in turns, counter-based randomness, the forward kinematics of a rig, animation (clips, springs, blinks) and terrain (perches, walls, strides, falls, hops);
//! - `semio-framework-pets-clearance` — the bodies and claimed corridors that keep pets apart, and the proof that no two overlap;
//! - `semio-framework-pets-traversal` — the swing of ropes and parachutes, and climbing (walls, ladders, grappling ropes, routes);
//! - `semio-framework-pets-interaction` — the press, the heat and the gestures of the learner's hand, the particles of the effects and the mischief with the page;
//! - `semio-framework-pets-character` — behaviour (weights, dwells, encounters, needs, casts) and feeling (moods, states, tricks, the chemistry between species);
//! - `semio-framework-pets-staging` — the stage that folds events into frames.
//!
//! Every function is pure, shares its vocabulary with the TypeScript core (`@semio-tech/pets`, snake_case here) and evaluates the same expressions in the same order, so both cores yield the same bits. This crate holds no code of its own: it re-exports exactly the names its subject hosts call.
//!
//! @see README.md — the domain model
//! @see 🧬️schema/🔣️.json — the normative contract

pub use semio_framework_pets_character::behavior::{activity_weights, affinity_of, cast_of, dwell_of, encounter_of, encounter_shares, followers_of, needs_after, needs_of, rapport_after, rapport_faded, Situation, MODE_LIMITS};
pub use semio_framework_pets_character::feeling::{
    appraised, at_rest, caught, click_trick, drowsed, encounter_bias, face_of, held_ticks, impulse, ladder_of, lasting_ticks, mood_weights, nearby, performed, proneness_of, reactions_of, rungs_of, seen, settled, settles_at, show_trick, spirits_of,
    state_after_trick, state_at, state_ends, step_state, swayed_shares, trials_of, tricks_for, valence_of, whim_trick, Character, Sighting, APPRAISALS, CONTAGION_BEAT, CONTAGION_REACH, DROWSY_ENERGY, MOOD_AFFINITIES, MOOD_DECAYS, MOOD_DROPS,
    MOOD_FAINT, MOOD_HOLD, MOOD_LIDS, MOOD_OVERRIDE, MOOD_PRIORITIES, MOOD_REST, MOOD_RISE, MOOD_SHARES, MOOD_SLANTS, MOOD_SPREADS, MOOD_VALENCES, MOOD_WEIGHTS, OCCASIONS, PRONENESS, PRONE_DECAY, PRONE_GAIN, TRICK_AMOUNT,
};
pub use semio_framework_pets_clearance::{
    body_of, canopied, claim_clear, claim_of, column_over, evicted, free_at, guarded_stride, head_under, leaning, lift_onto, must_poof, near_misses, obstacles_of, order_kept, order_of, overlaps, pruned, pushed_out, released, rests_on,
    scoot_fraction, scooted, seat_of, slice_at, slide_of, slide_side, slide_stride, slot_in, spot_on, vaults, Body, Pair, Posture, DELAY, FOREVER, LANE_LIFT, LEAN, MARGIN, PATIENCE, PUSHES, SCOOT_HASTE, SEAM, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT,
    SLIDE_OFF_SPEED, STEERINGS, UPRIGHT,
};
pub use semio_framework_pets_contract::schema::{
    Activity, Actor, Canopy, Cast, Claim, Clip, Cooling, Cue, Dragged, Ease, Ensemble, Extent, Facing, Feeling, Fixture, Footing, Frame, Gait, Gaze, Gear, Grip, Hang, Menagerie, Mood, Needs, Party, Perch, PetMode, Pitch, Placement, Point, Pointer,
    Pressed, Rapport, Rate, Reaction, Rect, Released, Shot, Size, Slug, Species, Stage, StageEvent, Surface, Temperament, Text, Ticked, Ticks, Track, Trait, Trick, Wall, ACTIVITIES, CUES, MOODS,
};
pub use semio_framework_pets_contract::validation::{ensemble_issues, menagerie_issues, species_issues, Issue};
pub use semio_framework_pets_interaction::effects::{born_at, capped, emitter_ends, life_ticks, lowbias32, mix, particles_of, period_of, swarm_of, unit, Emission, Particle};
pub use semio_framework_pets_interaction::gesture::{
    hover_step, no_hover, no_shaking, press_step, shake_step, warmth_after, Caress, Guards, PressInput, CIRCLE_AGAINST, CIRCLE_CLOSE, CIRCLE_FAST, CIRCLE_HYSTERESIS, CIRCLE_MARGIN, CIRCLE_OUT_TICKS, CIRCLE_QUARTERS, CIRCLE_REACH, CIRCLE_REST,
    CIRCLE_ROUND, CIRCLE_SLOW, COLD, ENOUGH_TICKS, HEAT_CLICK, HEAT_ENOUGH, HEAT_FORGIVEN, HEAT_HELLO, HEAT_HOLD, HEAT_LEAK, HEAT_TRICK, HOLD_TICKS, IDLE, SCROLL_TICKS, SHAKE_AMPLITUDE, SHAKE_HYSTERESIS, SHAKE_PAUSE, SHAKE_REST, SHAKE_REVERSALS,
    SHAKE_SPEED, SHAKE_WINDOW, SLOP_COARSE, SLOP_FINE, STROKE_FAST, STROKE_HYSTERESIS, STROKE_LENGTH, STROKE_MARGIN, STROKE_PAUSE, STROKE_SEGMENTS, STROKE_SLANT, STROKE_SLOW, STROKE_WINDOW, UNGUARDED,
};
pub use semio_framework_pets_interaction::mischief::{allowed, allowed_from, chosen_fixture, fits, fixture_for, lift_at, lift_ends, station_for, thrown_off, Circumstances, Keyed};
pub use semio_framework_pets_kinematics::animation::{blend_pose, clip_ticks, ease_bezier, lid_at, sample_clip, sample_track, spring_step, Spring, BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS};
pub use semio_framework_pets_kinematics::randomness::{random_between, random_pick, random_unit, random_words, weighted_index};
pub use semio_framework_pets_kinematics::rig::{compose, invert, look_offset, rest_pose, solve_rig, transform, Affine, Pose};
pub use semio_framework_pets_kinematics::terrain::{
    fall_step, hop_landing, hop_of, hop_step, landing_of, nearest_perch, nearest_wall, perch_at, perches_of, segment_clear, segment_hits, stride_to, wall_at, walls_of, Fall, Flight, Hop, FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT,
    HOP_STEEPNESS, HOP_TICKS, WALL_LIP,
};
pub use semio_framework_pets_kinematics::trigonometry::{atan_turns, clamp, cos_turns, fast_neg_exp, lerp, sin_turns, smoothstep};
pub use semio_framework_pets_staging::stage::{advance, frame_of, open_stage};
pub use semio_framework_pets_traversal::climbing::{
    chain_of, climb_phase, climb_step, climb_ticks, cling_of, crossable, foot_of, grip_for, grip_step, haul_of, hoist_path, hook_step, hook_ticks, ladder_at, ladder_exit, ladder_for, ladder_holds, ladder_landing, ladder_lean, ladder_length,
    ladder_phase, ladder_rungs, ladder_step, landing_for, ledge_of, mantle_path, miss_of, rim_for, rim_of, route_of, shot_for, shot_holds, slide_step, slip_of, spill_of, sway_step, wall_cost, wall_holds, wall_path, zip_step, Effort, Entry, Haul,
    LadderStand, Leg, CLIMB_DESCENT, CLIMB_RAMP, CLIMB_RISE, CROSS_REACH, GRIP_BITE, GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, GRIP_REST, GRIP_SPACING, HAND_HEIGHT, HOIST_HUMP, HOIST_RISE, HOOK_INSET, HOOK_LIFT, HOOK_RETURN, HOOK_SPEED, LADDER_DESCENT,
    LADDER_DISMOUNT_TICKS, LADDER_EXIT, LADDER_FLAT, LADDER_FOLLOW, LADDER_FOOTING, LADDER_GIRTH, LADDER_HORNS, LADDER_IDLE, LADDER_LEAN, LADDER_LIFE, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, LADDER_RAMP, LADDER_RISE, LADDER_SHIFT, LADDER_SHORT,
    LADDER_STEEP, LADDER_TALL, LADDER_TUCK, LUNGE_TICKS, MANTLE_INSET, MANTLE_TICKS, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_AIM_TICKS, ROPE_DETOUR, ROPE_ELEVATION, ROPE_FOLLOW, ROPE_HOIST_TICKS, ROPE_LONG, ROPE_MARGIN, ROPE_MISS_CHANCE,
    ROPE_MISS_OVERSHOOT, ROPE_RECOIL_TICKS, ROPE_REST, ROPE_RISE, ROPE_SHORT, ROPE_SHRUG_TICKS, ROPE_SULK, ROPE_TUG_TICKS, RUNG_SPACING, SLIDE_GAIN, SLIDE_SPEED, SLIDE_START, SLIP_LIFT, SLIP_PUSH, TOPPLE_DAMPING, TOPPLE_PUSH, TOPPLE_STEP,
    TOPPLE_STIFFNESS, WALL_FOLLOW, WALL_GRAB_TICKS, WALL_HANG_TICKS, ZIP_RAMP, ZIP_SLANT, ZIP_SPEED,
};
pub use semio_framework_pets_traversal::swing::{
    canopy_of, chute_of, chute_opens, chute_step, chute_wind, cone_clamp, flare_of, follow_step, hang_of, hang_step, impact_speed, lean_of, reel_step, release_velocity, ring_velocity, swing_step, throw_velocity, CHUTE_DAMPING, CHUTE_DESCENT,
    CHUTE_FACTOR, CHUTE_FLARE, CHUTE_GRAVITY, CHUTE_HEADROOM, CHUTE_OPENING, CHUTE_REFLEX, CHUTE_ROD, CHUTE_STEER_EASE, CHUTE_STEER_GAIN, CHUTE_STEER_SPEED, CHUTE_WIND, CHUTE_WIND_RATE, FOLLOW_DAMPING, FOLLOW_STIFFNESS, HANG_CONE, HANG_DAMPING,
    HANG_GRAVITY, HANG_ROD, HARD_LANDING, REEL_CAP, REEL_DAMPING, REEL_LEAST, REEL_RAMP, REEL_SPEED, RELEASE_DIVISOR, RELEASE_STALE, RELEASE_WEIGHTS, THROW_LEAST, THROW_MOST, THROW_RISE, THROW_SHARE,
};

/// 🧪️ The JSON codec the Protocol v2 subject hosts decode the shared vectors with.
pub use serde_json;
