/** 🐾️ Typed twin of `🧬️schema/🔣️.json`: menageries of rigged species, their bonds and casts, the stage they act on, its events and the frames it projects.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same
 * name with `readonly` fields for TypeScript consumers. No runtime dependency.
 *
 * @see 🧬️schema/🔣️.json — the normative contract
 * @see 🧬️schema/🦀️.rs — the Rust twin
 * @see README.md — the domain model
 */

//#region 🔖️Scalars
/** 🏷️ A kebab-case identifier, unique within its scope (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 chars). */
export type Slug = string;

/** 🌍️ A human-readable text in every supported language; there is no default language. */
export type Text = { readonly en: string; readonly de: string };

/** 🗣️ The languages every {@link Text} carries, English first. */
export const LANGUAGES = ["en", "de"] as const;

/** 📐️ An authored angle in degrees, clockwise on screen (the y axis points down). */
export type Degrees = number;

/** 🔄️ A simulated angle in turns (1 = a full clockwise revolution). */
export type Turns = number;

/** ⏱️ A whole number of simulation ticks; the stage advances {@link TICKS_PER_SECOND} ticks per second. */
export type Ticks = number;

/** 🥁️ The fixed simulation rate: a power of two, so one tick is an exact binary fraction of a second. */
export const TICKS_PER_SECOND = 64;

/** 🎨️ An sRGB colour as `#rrggbb` in lowercase hexadecimal digits. */
export type Color = string;

/** 🖍️ What a part is painted with: a colour of the species palette, the theme's ink or paper, or nothing. */
export const PAINTS = ["body", "accent", "detail", "ink", "paper", "none"] as const;

/** 🖌️ One of {@link PAINTS}. */
export type Paint = (typeof PAINTS)[number];

/** 🎚️ What a track animates on a bone: offsets in pixels, an offset in degrees or scale factors. */
export const CHANNELS = ["x", "y", "rotation", "scaleX", "scaleY"] as const;

/** 🎛️ One of {@link CHANNELS}. */
export type Channel = (typeof CHANNELS)[number];

/** 🚶️ How a species moves along a perch: on legs, in hops or hovering above it. */
export const GAITS = ["walk", "hop", "float"] as const;

/** 🦿️ One of {@link GAITS}. */
export type Gait = (typeof GAITS)[number];

/** 🎭️ Everything an actor can be doing; exactly one activity at a time. */
export const ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk"] as const;

/** 🎬️ One of {@link ACTIVITIES}. */
export type Activity = (typeof ACTIVITIES)[number];

/** 🌡️ How lively a stage is: motionless, slightly active or busy. */
export const PET_MODES = ["still", "calm", "lively"] as const;

/** 🔆️ One of {@link PET_MODES}. */
export type PetMode = (typeof PET_MODES)[number];

/** 🧾️ The schema identifier every menagerie document carries. */
export const MENAGERIE_SCHEMA = "semio.pets.menagerie/v1";

/** 📇️ The schema identifier every ensemble document carries. */
export const ENSEMBLE_SCHEMA = "semio.pets.ensemble/v1";
//#endregion 🔖️Scalars

//#region 🔖️Rig
/** 🦴️ One bone of a skeleton: its rest offset and rotation relative to its parent; the root has no parent. Bones are listed parents first. */
export type Bone = { readonly id: Slug; readonly parent?: Slug; readonly x: number; readonly y: number; readonly rotation?: Degrees };

/** ➰️ A free-form outline as SVG path data in the bone's own coordinates. */
export type PathShape = { readonly kind: "path"; readonly d: string };

/** ⭕️ An ellipse around a centre. */
export type EllipseShape = { readonly kind: "ellipse"; readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number };

/** ▭️ An axis-aligned rectangle with optionally rounded corners. */
export type RectShape = { readonly kind: "rect"; readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly radius?: number };

/** ➖️ A straight stroke between two points. */
export type LineShape = { readonly kind: "line"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number };

/** 🔷️ The geometry of a part, in the coordinates of the bone that carries it. */
export type Shape = PathShape | EllipseShape | RectShape | LineShape;

/** 🧩️ One drawn piece of a species, rigidly attached to a bone; parts are drawn in list order, back to front. */
export type Part = { readonly id: Slug; readonly bone: Slug; readonly shape: Shape; readonly fill: Paint; readonly stroke: Paint; readonly strokeWidth?: number };

/** 👁️ An eye on a bone: a round white of `radius` whose pupil of radius `pupil` follows what the actor looks at. */
export type Eye = { readonly id: Slug; readonly bone: Slug; readonly x: number; readonly y: number; readonly radius: number; readonly pupil: number };

/** 👄️ A mouth on a bone: a curve of `width` that bends with the actor's mood. */
export type Mouth = { readonly bone: Slug; readonly x: number; readonly y: number; readonly width: number };

/** 🙂️ The face of a species; it is drawn above the part named by `above` (above every part when absent). */
export type Face = { readonly eyes: readonly Eye[]; readonly mouth?: Mouth; readonly above?: Slug };
//#endregion 🔖️Rig

//#region 🔖️Animation
/** 🪜️ The control points `[x1, y1, x2, y2]` of a cubic Bézier easing from (0, 0) to (1, 1), as in CSS `cubic-bezier()`. */
export type Ease = readonly [number, number, number, number];

/** 🔑️ A keyed value at phase `at` (0…1) of a clip; `ease` shapes the way to the next key (linear when absent). */
export type Key = { readonly at: number; readonly value: number; readonly ease?: Ease };

/** 🛤️ The keys of one channel of one bone, in ascending phase, starting at 0 and ending at 1. */
export type Track = { readonly bone: Slug; readonly channel: Channel; readonly keys: readonly Key[] };

/** 🎞️ A keyframed motion of `seconds` length, played once or looped; values are relative to the rest pose. */
export type Clip = { readonly id: Slug; readonly seconds: number; readonly loop: boolean; readonly tracks: readonly Track[] };

/** 🗃️ The clips a species plays per activity; one of several is drawn at random, none leaves the body at rest. */
export type Repertoire = { readonly [activity in Activity]?: readonly Slug[] };
//#endregion 🔖️Animation

//#region 🔖️Menagerie
/** 📦️ The box a species fills at rest: its feet stand on the origin, the body rises `height` above it and spans `width` around it. */
export type Size = { readonly width: number; readonly height: number };

/** 🌈️ The three colours of a species; ink and paper come from the theme. */
export type Palette = { readonly body: Color; readonly accent: Color; readonly detail: Color };

/** 🏃️ How fast a species travels in pixels per second, with which gait and, when floating, how high above its perch. */
export type Locomotion = { readonly gait: Gait; readonly speed: number; readonly hover?: number };

/** 🧠️ The character of a species, each trait in [0, 1]. */
export type Temperament = { readonly energy: number; readonly sociability: number; readonly curiosity: number };

/** 🧬️ A kind of pet: its accessible name, what it stands for, its rig, face, motions and character. */
export type Species = {
  readonly $schema?: string;
  readonly id: Slug;
  readonly name: Text;
  readonly thing: Text;
  readonly grounds: readonly string[];
  readonly size: Size;
  readonly palette: Palette;
  readonly bones: readonly Bone[];
  readonly parts: readonly Part[];
  readonly face: Face;
  readonly clips: readonly Clip[];
  readonly repertoire: Repertoire;
  readonly locomotion: Locomotion;
  readonly temperament: Temperament;
};

/** 🤝️ How two species feel about each other, from −1 (they squabble) to 1 (they adore each other); unlisted pairs are neutral. */
export type Bond = { readonly between: readonly [Slug, Slug]; readonly affinity: number };

/** 🎟️ The species of one scene: the core is on stage whenever there is room, the rotation takes turns. */
export type Cast = { readonly scene: Slug; readonly core: readonly Slug[]; readonly rotation: readonly Slug[] };

/** 🎪️ A menagerie document: every species of a domain, their bonds and the casts of its scenes. */
export type Menagerie = {
  readonly $schema?: string;
  readonly schema: typeof MENAGERIE_SCHEMA;
  readonly id: Slug;
  readonly title: Text;
  readonly species: readonly Species[];
  readonly bonds: readonly Bond[];
  readonly casts: readonly Cast[];
};

/** 🗂️ The authoring form of a menagerie: every species lives in a document of its own, referenced by a path relative to the ensemble. */
export type Ensemble = {
  readonly $schema?: string;
  readonly schema: typeof ENSEMBLE_SCHEMA;
  readonly id: Slug;
  readonly title: Text;
  readonly species: readonly string[];
  readonly bonds: readonly Bond[];
  readonly casts: readonly Cast[];
};
//#endregion 🔖️Menagerie

//#region 🔖️Terrain
/** 📍️ A position in viewport pixels. */
export type Point = { readonly x: number; readonly y: number };

/** ⬛️ An axis-aligned box in viewport pixels. */
export type Rect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

/** 🪵️ The top edge of something actors may stand on, from `x0` to `x1` at height `y`. */
export type Surface = { readonly id: string; readonly x0: number; readonly x1: number; readonly y: number };

/** 🪺️ A stretch of a surface that is free to stand on. */
export type Perch = { readonly surface: string; readonly x0: number; readonly x1: number; readonly y: number };
//#endregion 🔖️Terrain

//#region 🔖️Stage
/** ⏭️ Time passed: the stage advances by whole ticks. */
export type Ticked = { readonly kind: "ticked"; readonly ticks: Ticks };

/** 🖱️ The pointer moved to a position. */
export type Pointed = { readonly kind: "pointed"; readonly x: number; readonly y: number };

/** 🫥️ The pointer left the stage. */
export type Unpointed = { readonly kind: "unpointed" };

/** 👀️ Other things worth a look right now, such as the cursors of other people. */
export type Glanced = { readonly kind: "glanced"; readonly points: readonly Point[] };

/** 🗺️ The stage was measured anew: its size, every surface and everything actors must not cover. */
export type Surveyed = { readonly kind: "surveyed"; readonly width: number; readonly height: number; readonly surfaces: readonly Surface[]; readonly keepouts: readonly Rect[] };

/** 📣️ The species that belong on stage from now on, most wanted first. */
export type Summoned = { readonly kind: "summoned"; readonly species: readonly Slug[] };

/** 📻️ The liveliness changed. */
export type Tuned = { readonly kind: "tuned"; readonly mode: PetMode };

/** 🤫️ A time of concentration began or ended; hushed actors rest. */
export type Hushed = { readonly kind: "hushed"; readonly quiet: boolean };

/** 👆️ Someone tapped the stage at a position. */
export type Poked = { readonly kind: "poked"; readonly x: number; readonly y: number };

/** 📨️ Everything that can happen to a stage. */
export type StageEvent = Ticked | Pointed | Unpointed | Glanced | Surveyed | Summoned | Tuned | Hushed | Poked;

/** 🔭️ Where the pupils rest between −1 and 1 on both axes, with the velocity of the spring that moves them. */
export type Gaze = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number };

/** 🔋️ The drives of an actor, each in [0, 1]. */
export type Needs = { readonly energy: number; readonly sociability: number; readonly curiosity: number };

/** 🧸️ One pet on stage: where its feet are, the way it faces and the tick from which it faces that way squarely (`faced`; it turns round over the ticks before), what it does and until when, whom it does it with, how it looks and feels. */
export type Actor = {
  readonly species: Slug;
  readonly perch: string | null;
  readonly x: number;
  readonly y: number;
  readonly vx: number;
  readonly vy: number;
  readonly facing: 1 | -1;
  readonly faced: Ticks;
  readonly activity: Activity;
  readonly since: Ticks;
  readonly until: Ticks;
  readonly goal: number;
  readonly partner: Slug | null;
  readonly clip: Slug | null;
  readonly gaze: Gaze;
  readonly blink: Ticks;
  readonly mood: number;
  readonly needs: Needs;
  readonly opacity: number;
  readonly leaving: boolean;
  readonly draws: number;
};

/** 💞️ How the shared history of two actors has shifted their bond away from its authored affinity. */
export type Rapport = { readonly between: readonly [Slug, Slug]; readonly drift: number };

/** 🏟️ The whole state of a stage; `advance` folds events into it and `frameOf` projects it. */
export type Stage = {
  readonly seed: number;
  readonly tick: Ticks;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly width: number;
  readonly height: number;
  readonly pointer: Point | null;
  readonly pointed: Ticks;
  readonly glances: readonly Point[];
  readonly surfaces: readonly Surface[];
  readonly keepouts: readonly Rect[];
  readonly perches: readonly Perch[];
  readonly wanted: readonly Slug[];
  readonly actors: readonly Actor[];
  readonly rapports: readonly Rapport[];
  readonly met: Ticks;
  readonly draws: number;
};
//#endregion 🔖️Stage

//#region 🔖️Frame
/** 👓️ One eye as drawn: the pupil's offset from the centre of the white in pixels and how far the lid is shut (0 open, 1 shut). */
export type EyeFrame = { readonly x: number; readonly y: number; readonly lid: number };

/** 🖼️ One actor as drawn: its feet, the way it faces, what it is doing, its opacity, a 2×3 matrix `a b c d e f` per bone in rig order, its eyes and its mood (−1 sad … 1 happy). */
export type ActorFrame = {
  readonly species: Slug;
  readonly x: number;
  readonly y: number;
  readonly facing: 1 | -1;
  readonly activity: Activity;
  readonly opacity: number;
  readonly bones: readonly number[];
  readonly eyes: readonly EyeFrame[];
  readonly mood: number;
};

/** 🎥️ Everything a render target needs to draw a stage: the actors back to front, how many ticks per second the motion needs (0 = nothing moves) and the tick of the next change when nothing moves. */
export type Frame = { readonly tick: Ticks; readonly actors: readonly ActorFrame[]; readonly rate: 0 | 16 | 32 | 64; readonly wake: Ticks | null };
//#endregion 🔖️Frame
