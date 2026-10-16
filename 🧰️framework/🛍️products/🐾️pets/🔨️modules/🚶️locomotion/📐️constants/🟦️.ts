/** 📐️ Locomotion, constants: the waiting times, spans, speeds and shares of the motions, and the bounce and puff durations the rest of the stage reads.
 *
 * @see ../⚓️footing/🟦️.ts — the footing that uses them first
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */

//#region 🔖️Constants
export const WAITING = 1920;

export const LEAVE_REACH = 3;

export const GLIDE_TICKS = 256;

export const COURSE_TICKS = 1024;

export const CLAIM_SPAN = 4;

export const TUMBLE_TICKS = 24;

export const WIND_PHASE = 0.37;

export const OPEN_STIFFNESS = 220;

export const OPEN_DAMPING = 12;

export const DIZZY_SPEED = 780;

export const RETURN_SPEED = 384;

export const RETURN_LEAST = 12;

export const LANE_RAMP = 8;

export const TOSS_TOP = 16;

export const TOSS_TICKS = 96;

export const STEER_MOST = 240;

export const BRACES = 4;

export const LONGEST = 4096;

export const REST_RETRY = 32;

export const REST_LEAST = 128;

export const REST_HALVINGS = 3;

export const EXPLORE_SHARE = 0.5;

export const MAX_LADDERS = 2;

export const POST_UNITS: readonly number[] = [0, 0, 1];

/** 🏓️ The share of its speed a pet in flight keeps when it bounces off an edge of the stage — the left, the right or the top. */
export const BOUNCE = 0.5;

/** 🌫️ How many ticks the dust of a pet that vanished stays on the stage (0.75 s): its `Puff` is dropped then. */
export const PUFF_TICKS = 48;
//#endregion 🔖️Constants
