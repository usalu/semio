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

/** 🎭️ Everything an actor can be doing; exactly one activity at a time. The first eleven are the life on a perch; then: hanging from the learner's hand, tumbling through the air after a throw, gliding under a parachute, aiming a grappling line and reeling it in, climbing a wall or a ladder, mantling over an edge, sliding down, carrying a ladder, performing a trick, purring, being dizzy, shrugging, scooting aside for another pet and pushing a fixture. New activities are appended, so the index of an activity never changes. */
export const ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"] as const;

/** 🎬️ One of {@link ACTIVITIES}. */
export type Activity = (typeof ACTIVITIES)[number];

/** 🌡️ How lively a stage is: motionless, slightly active or busy. */
export const PET_MODES = ["still", "calm", "lively"] as const;

/** 🔆️ One of {@link PET_MODES}. */
export type PetMode = (typeof PET_MODES)[number];

/** 💗️ What an actor can feel, one mood at a time with an intensity in [0, 1]. A stronger cause replaces a weaker mood: scared, then grumpy, then sad, then proud, happy and playful alike, then curious, then content; sleepy follows the energy need. */
export const MOODS = ["content", "happy", "playful", "curious", "proud", "sleepy", "grumpy", "sad", "scared"] as const;

/** 😊️ One of {@link MOODS}. */
export type Mood = (typeof MOODS)[number];

/** 👣️ What carries an actor, one footing at a time: a perch, the air (hopping, falling, thrown), its open parachute, the learner's hand, a rope, a ladder, a wall or the head of another actor. */
export const FOOTINGS = ["perch", "air", "chute", "hand", "rope", "ladder", "wall", "head"] as const;

/** 🦶️ One of {@link FOOTINGS}. */
export type Footing = (typeof FOOTINGS)[number];

/** 🧰️ What a species may own to get around: it climbs walls, carries and raises a ladder, shoots a grappling line or opens a parachute. */
export const GEARS = ["climb", "ladder", "grapple", "parachute"] as const;

/** 🧗️ One of {@link GEARS}. */
export type Gear = (typeof GEARS)[number];

/** 🔔️ What sets a trick off: a click, the pointer circling the pet clockwise or counter-clockwise, stroking it, shaking it while it is held, its own whim, or showing off to another pet. */
export const CUES = ["click", "circle", "countercircle", "stroke", "shake", "whim", "show"] as const;

/** 👋️ One of {@link CUES}. */
export type Cue = (typeof CUES)[number];

/** 🌬️ How the particles of an emitter move: down with gravity, up with buoyancy, outwards from the emitter, round it, or wandering slowly. */
export const DRIFTS = ["fall", "rise", "burst", "orbit", "drift"] as const;

/** 🍃️ One of {@link DRIFTS}. */
export type Drift = (typeof DRIFTS)[number];

/** 🎮️ What a learner can ask of a free pet without a pointer: a hello, a click trick, petting, tossing or an explicit hand gesture. */
export const DEEDS = ["hello", "trick", "pet", "toss", "circle", "countercircle", "stroke", "shake"] as const;

/** 🕹️ One of {@link DEEDS}. */
export type Deed = (typeof DEEDS)[number];

/** ✍️ What presses a pet: a mouse, a pen or a finger. */
export const POINTERS = ["mouse", "pen", "touch"] as const;

/** 🖲️ One of {@link POINTERS}. */
export type Pointer = (typeof POINTERS)[number];

/** 🚦️ Where the press of a stage stands: nothing is down (`idle`), a press waits for what it becomes (`armed`), it rests on the pet (`holding`), or the pet is picked up (`lifted`). */
export const PRESS_PHASES = ["idle", "armed", "holding", "lifted"] as const;

/** 🚥️ One of {@link PRESS_PHASES}. */
export type PressPhase = (typeof PRESS_PHASES)[number];

/** 📶️ How a pet answers attention, from cold to saturated: a hello, a trick, a purr, or it has had enough. */
export const TIERS = ["hello", "trick", "purr", "enough"] as const;

/** 🎖️ One of {@link TIERS}. */
export type Tier = (typeof TIERS)[number];

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

/** 💡️ The colours a state paints over the palette; a colour it leaves out stays as authored. */
export type Tint = { readonly body?: Color; readonly accent?: Color; readonly detail?: Color };

/** ✨️ A source of particles on a bone, `x` and `y` away from it in the bone's coordinates: every particle is drawn as `shape` around its own origin and painted like a part; `motion` moves it, at most `count` (1…32, whole) are alive at once, each lives `life` seconds and leaves at `speed` pixels per second in directions that scatter over `spread` of a full turn (0…1). */
export type Emitter = {
  readonly id: Slug;
  readonly bone: Slug;
  readonly x: number;
  readonly y: number;
  readonly shape: Shape;
  readonly fill: Paint;
  readonly stroke: Paint;
  readonly strokeWidth?: number;
  readonly motion: Drift;
  readonly count: number;
  readonly life: number;
  readonly speed: number;
  readonly spread: number;
};

/** 🔦️ A lasting condition of a species, such as shining strongly: it may tint the palette, loop the clip `clip` on top of the idle loop and run the emitter `emitter`; `lasts` seconds after it began it gives way to the state `then`. */
export type SpeciesState = { readonly id: Slug; readonly name: Text; readonly tint?: Tint; readonly clip?: Slug; readonly emitter?: Slug; readonly lasts?: number; readonly then?: Slug };

/** 🪄️ Something a species can perform: the clip `clip` played once, set off by any of `cues`, optionally with the particles of `emitter`; it is on offer in the states `from` (in every state when absent), leaves the species in the state `to` and in the mood `mood`. */
export type Trick = { readonly id: Slug; readonly name: Text; readonly clip: Slug; readonly cues: readonly Cue[]; readonly emitter?: Slug; readonly from?: readonly Slug[]; readonly to?: Slug; readonly mood?: Mood };

/** 😻️ How a species shows that it is content while it is petted: a looping clip and optionally the particles of an emitter. */
export type Purr = { readonly clip: Slug; readonly emitter?: Slug };

/** 🧬️ A kind of pet: its accessible name, what it stands for, its rig, face, motions and character; its states (at least one, the first is the resting state), tricks, purr and emitters; the gear it owns, how high above its feet it is gripped at the scruff (`grip`), how far its encounter poses lean out (`reach`), the shape drawn as its parachute (`canopy`, a plain one when absent) and its resting mood. */
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
  readonly states: readonly SpeciesState[];
  readonly tricks: readonly Trick[];
  readonly purr: Purr;
  readonly emitters: readonly Emitter[];
  readonly gear: readonly Gear[];
  readonly grip: number;
  readonly reach: number;
  readonly canopy?: Shape;
  readonly mood: Mood;
};

/** 🤝️ How two species feel about each other, from −1 (they squabble) to 1 (they adore each other); unlisted pairs are neutral. */
export type Bond = { readonly between: readonly [Slug, Slug]; readonly affinity: number };

/** 🎟️ The species of one scene: the core is on stage whenever there is room, the rotation takes turns. */
export type Cast = { readonly scene: Slug; readonly core: readonly Slug[]; readonly rotation: readonly Slug[] };

/** 🔎️ What one side of a reaction must be: an actor of a species (of any species when absent), optionally in a state it has held for at least `held` seconds, in a mood, doing an activity or performing the trick `trick`. */
export type Trait = { readonly species?: Slug; readonly state?: Slug; readonly held?: number; readonly mood?: Mood; readonly activity?: Activity; readonly trick?: Slug };

/** 💥️ What a reaction does to one of its two sides (`on`: the side `when` or the side `near` describes): it puts it into a state, gives it a mood of intensity `amount` (0…1), shifts the rapport of the two by `rapport` (−1…1), makes their next encounter the given one, has it perform a trick or sets it off on something it could start by itself (`activity`: fidget, walk, hop or sleep). */
export type Effect = { readonly on: "when" | "near"; readonly state?: Slug; readonly mood?: Mood; readonly amount?: number; readonly rapport?: number; readonly encounter?: "greet" | "cuddle" | "squabble"; readonly trick?: Slug; readonly activity?: "fidget" | "walk" | "hop" | "sleep" };

/** ⚗️ Chemistry between two species: when an actor matching `when` is within `within` pixels of one matching `near` — `where` it is seen from the second (above, below, beside or anywhere when absent), while no third actor matching `unless` is within `within` pixels of the second and while the affinity of the two lies within `affinity` (`[low, high]`, −1…1) — the effects of `then` happen, at most once in `every` seconds and with the probability `chance` (always when absent). */
export type Reaction = {
  readonly id: Slug;
  readonly when: Trait;
  readonly near: Trait;
  readonly within: number;
  readonly where?: "above" | "below" | "beside" | "any";
  readonly unless?: Trait;
  readonly affinity?: readonly [number, number];
  readonly every: number;
  readonly chance?: number;
  readonly then: readonly Effect[];
};

/** 🎪️ A menagerie document: every species of a domain, their bonds, the casts of its scenes and the chemistry between its species. */
export type Menagerie = {
  readonly $schema?: string;
  readonly schema: typeof MENAGERIE_SCHEMA;
  readonly id: Slug;
  readonly title: Text;
  readonly species: readonly Species[];
  readonly bonds: readonly Bond[];
  readonly casts: readonly Cast[];
  readonly chemistry: readonly Reaction[];
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
  readonly chemistry: readonly Reaction[];
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

/** 🧱️ A side edge of the element of a surface, which pets may climb: the surface it belongs to, the way it faces (`-1` the left edge, `1` the right edge), where it stands (`x`) and its stretch from `y0` at the top down to `y1`. */
export type Wall = { readonly id: string; readonly surface: string; readonly side: 1 | -1; readonly x: number; readonly y0: number; readonly y1: number };

/** 🪨️ A stretch of a wall that is free to climb: the wall and the surface it belongs to, the way the wall faces (`-1` the left edge, its air on the left; `1` the right edge), where it stands (`x`) and its extent from `y0` down to `y1`. */
export type Pitch = { readonly wall: string; readonly surface: string; readonly side: 1 | -1; readonly x: number; readonly y0: number; readonly y1: number };

/** 📌️ An element of the page a pet may play with: its key among the grounds of the menagerie (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`) and its box. */
export type Fixture = { readonly id: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number };
//#endregion 🔖️Terrain

//#region 🔖️Stage
/** ⏭️ Time passed: the stage advances by whole ticks. */
export type Ticked = { readonly kind: "ticked"; readonly ticks: Ticks };

/** 🖱️ The pointer moved to a position, over free space or over a control (an interactive element of the page). */
export type Pointed = { readonly kind: "pointed"; readonly x: number; readonly y: number; readonly over: "free" | "control" };

/** 🫥️ The pointer left the stage. */
export type Unpointed = { readonly kind: "unpointed" };

/** 👀️ Other things worth a look right now, such as the cursors of other people. */
export type Glanced = { readonly kind: "glanced"; readonly points: readonly Point[] };

/** 🗺️ The stage was measured anew: its size, every surface, everything actors must not cover, the walls they may climb and the fixtures they may play with. */
export type Surveyed = { readonly kind: "surveyed"; readonly width: number; readonly height: number; readonly surfaces: readonly Surface[]; readonly keepouts: readonly Rect[]; readonly walls: readonly Wall[]; readonly fixtures: readonly Fixture[] };

/** 📣️ The species that belong on stage from now on, most wanted first. */
export type Summoned = { readonly kind: "summoned"; readonly species: readonly Slug[] };

/** 📻️ The liveliness changed. */
export type Tuned = { readonly kind: "tuned"; readonly mode: PetMode };

/** 🤫️ A time of concentration began or ended; hushed actors rest. */
export type Hushed = { readonly kind: "hushed"; readonly quiet: boolean };

/** 🫳️ The learner pressed where a pet may be: the primary button or contact went down at a position, with a mouse, a pen or a finger, on nothing interactive. */
export type Pressed = { readonly kind: "pressed"; readonly x: number; readonly y: number; readonly pointer: Pointer };

/** 🧲️ The pointer of the open press moved to a position. */
export type Dragged = { readonly kind: "dragged"; readonly x: number; readonly y: number };

/** 🎈️ The open press ended at a position. */
export type Released = { readonly kind: "released"; readonly x: number; readonly y: number };

/** ✖️ The open press was called off: Escape, a cancelled pointer, lost capture, a blurred or hidden page, a pause. */
export type Cancelled = { readonly kind: "cancelled" };

/** 🫱️ The learner took a fixture back — pointed at it, pressed, focused, typed into or dragged it —: whatever a pet does with it ends at once. */
export type Reclaimed = { readonly kind: "reclaimed"; readonly fixture: string };

/** 🥄️ The learner did something on the page: any input. */
export type Stirred = { readonly kind: "stirred" };

/** 📜️ The page moved under the pointer: it was scrolled. */
export type Scrolled = { readonly kind: "scrolled" };

/** 🎲️ The learner asked a pet for a deed without a pointer (the keyboard and panel equivalents of the hand). */
export type Played = { readonly kind: "played"; readonly species: Slug; readonly deed: Deed };

/** ✅️ What the learner allows: pets answer clicks and can be picked up (`play`), pets may play with the page (`mischief`). */
export type Permitted = { readonly kind: "permitted"; readonly play: boolean; readonly mischief: boolean };

/** 📨️ Everything that can happen to a stage. */
export type StageEvent = Ticked | Pointed | Unpointed | Glanced | Surveyed | Summoned | Tuned | Hushed | Pressed | Dragged | Released | Cancelled | Reclaimed | Stirred | Scrolled | Played | Permitted;

/** 🔭️ Where the pupils rest between −1 and 1 on both axes, with the velocity of the spring that moves them. */
export type Gaze = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number };

/** 🔋️ The drives of an actor, each in [0, 1]. */
export type Needs = { readonly energy: number; readonly sociability: number; readonly curiosity: number };

/** 🤏️ The press of a stage: its phase, the point it went down at (the grip, kept until the press ends), the tick it went down and the slop of its pointer in pixels. */
export type Press = { readonly phase: PressPhase; readonly x: number; readonly y: number; readonly since: Ticks; readonly slop: number };

/** 🔥️ How warm an actor is from attention: the `heat` as it was at tick `since` (it leaks from there), the tick `until` which it has had enough (deaf before, forgiving from then on), the `tier` of its latest answer, how many answers in a row had that tier (`run`, 1 for the first) and how many of all its answers were tricks (`tricks`; the latest trick is number `tricks − 1` in the species' order). */
export type Warmth = { readonly heat: number; readonly since: Ticks; readonly until: Ticks; readonly tier: Tier; readonly run: number; readonly tricks: number };

/** 🌀️ The circle the pointer may be drawing round one actor: whether it is followed (`live`) and when it was last in the band (`inside`); the signs of its offset from the body's centre as the Schmitt trigger holds them (`sx`, `sy`; 0 = not known yet) and the previous offset (`px`, `py`); the running lap — its direction (`turn`: 1 clockwise on screen, −1 counter-clockwise, 0 before its first quarter turn), its net quarter turns in that direction, all its quarter turns (`steps`) and those `against` the direction, the ticks of its `first` and `last` quarter turn, the squared distance from the centre at its first quarter turn (`open`) and the least and the greatest one since (`near`, `far`) — and the tick before which nothing is recognised (`rest`). */
export type Circling = {
  readonly live: boolean;
  readonly inside: Ticks;
  readonly sx: number;
  readonly sy: number;
  readonly px: number;
  readonly py: number;
  readonly turn: number;
  readonly quarters: number;
  readonly steps: number;
  readonly against: number;
  readonly first: Ticks;
  readonly last: Ticks;
  readonly open: number;
  readonly near: number;
  readonly far: number;
  readonly rest: Ticks;
};

/** 🖐️ The petting the pointer may be giving one actor: whether it is followed (`live`); the direction of the running stroke (`way`: 1 rightwards, −1 leftwards, 0 before it has one), the offset from the body's centre and the tick at which it began (`from`, `began`) and its running extreme (`peak`, `reached`); the highest and the lowest vertical offset of the running stroke up to its extreme (`top`, `bottom`) and since its extreme (`topSince`, `bottomSince`: these belong to the next stroke if this one ended there); how many strokes in a row counted (`count`; −1 while the run that led into the zone is still under way) and the ticks at which the oldest two of them began (`first`, `second`); and the tick before which nothing is recognised (`rest`). */
export type Stroking = {
  readonly live: boolean;
  readonly way: number;
  readonly from: number;
  readonly began: Ticks;
  readonly peak: number;
  readonly reached: Ticks;
  readonly top: number;
  readonly bottom: number;
  readonly topSince: number;
  readonly bottomSince: number;
  readonly count: number;
  readonly first: Ticks;
  readonly second: Ticks;
  readonly rest: Ticks;
};

/** 🫨️ The shake the hand may be giving the pet it holds: whether the grip is followed (`live`); the point and the tick of its latest reversal (`ax`, `ay`, `at`) and the farthest point from there on the running swing (`fx`, `fy`, `reached`); how many swings in a row counted (`count`) and the ticks of the three latest counted reversals, oldest first (`mark1`, `mark2`, `mark3`); and the tick before which nothing is recognised (`rest`). */
export type Shaking = {
  readonly live: boolean;
  readonly ax: number;
  readonly ay: number;
  readonly at: Ticks;
  readonly fx: number;
  readonly fy: number;
  readonly reached: Ticks;
  readonly count: number;
  readonly mark1: Ticks;
  readonly mark2: Ticks;
  readonly mark3: Ticks;
  readonly rest: Ticks;
};

/** 🪶️ The two gestures a free pointer can make round or over one actor: circling and stroking. */
export type Hover = { readonly circling: Circling; readonly stroking: Stroking };

/** ✊️ The point a held pet is gripped at: where it is and how fast it moves, in pixels and pixels per second. */
export type Grip = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number };

/** 🐒️ A pet in the learner's hand: its grip, where its feet are (the bob of its pendulum), and where they were one tick ago. */
export type Hang = { readonly grip: Grip; readonly bob: Point; readonly previous: Point };

/** ☂️ A pet under its open parachute: where the canopy holds the cords and how fast it moves (`vy` without the flare), where the feet are, and where they were one tick ago. */
export type Canopy = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number; readonly bob: Point; readonly previous: Point };

/** 🪂️ The parachute of an actor while it is out: the tick it began to open, how far it is open (0 packed … 1 open; it may overshoot while it fills) and how fast that changes per second, and the motion of its canopy. */
export type Parachute = { readonly since: Ticks; readonly open: number; readonly opening: number; readonly canopy: Canopy };

/** 🏹️ A shot of the grappling gun: the surface whose edge the hook bites, the way the actor faces, the muzzle the rope leaves from, the hook point, the length of the taut rope and whether it is reeled in straight (`zip`) or swung. */
export type Shot = { readonly surface: string; readonly facing: 1 | -1; readonly muzzle: Point; readonly hook: Point; readonly length: number; readonly reel: "zip" | "swing" };

/** 🧶️ The grappling rope of an actor while it is out: its shot, the tick it was fired, the length of rope left between the hook and the hands, where the hands are and where they were one tick ago, and whether the hook bites its edge (`false`: the shot misses — the hook flies past and is pulled back). */
export type Rope = { readonly shot: Shot; readonly since: Ticks; readonly length: number; readonly hand: Point; readonly before: Point; readonly caught: boolean };

/** 💓️ What an actor feels: one mood and how strongly it felt it (0…1) at the tick `since`; the present is read off it, so it is stored only when something stirs the actor. */
export type Feeling = { readonly mood: Mood; readonly intensity: number; readonly since: Ticks };

/** 💨️ An emitter of its species an actor runs: the emitter, the tick it began and the tick its cause ended (`null` while the state, the trick or the purr that runs it lasts). */
export type Plume = { readonly emitter: Slug; readonly since: Ticks; readonly until: Ticks | null };

/** 🔲️ An axis-aligned box by its edges in stage pixels: `x0 ≤ x1` from left to right, `y0 ≤ y1` from top to bottom. */
export type Extent = { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number };

/** 🍰️ The box a planned body stays inside from tick `from` to tick `until`, both included. */
export type Slice = { readonly from: Ticks; readonly until: Ticks; readonly extent: Extent };

/** 🎫️ The swept corridor of a planned motion: its slices in ascending ticks without a gap, and where its owner rests once the last slice has passed (`null`: the motion ends off stage). Everybody else stays out of it. */
export type Claim = { readonly owner: Slug; readonly slices: readonly Slice[]; readonly rest: Extent | null };

/** 🚩️ One tick of a planned motion: where the feet are at its end and how fast they move in pixels per second, the tilt of the drawing about the scruff in turns, and the canopy while a parachute is out (`null` while none is). */
export type Waypoint = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number; readonly tilt: Turns; readonly canopy: Canopy | null };

/** 🧭️ A planned motion through the air — a hop, a glide, a fall, a throw, a descent under a parachute, the way back after a cancelled pick-up —: its owner, the tick at whose end the feet are at the first waypoint, a waypoint per tick, what it ends on (`perch`: the surface `landing`; `head`: the actor `landing`; `away`: below the stage, `landing` empty) and the speed in pixels per second with which it touches down. */
export type Course = { readonly owner: Slug; readonly from: Ticks; readonly steps: readonly Waypoint[]; readonly ending: "perch" | "head" | "away"; readonly landing: string; readonly touch: number };

/** 🥾️ One tick of a trip with gear: where the feet are at its end, what carries the actor (its perch — the surface `perch` names; `null` off a perch —, a wall, a ladder or a rope), what it does and the way it faces, and which pitch of the trip its hands hold (−1: none). */
export type Foothold = { readonly x: number; readonly y: number; readonly footing: Footing; readonly perch: string | null; readonly activity: Activity; readonly facing: 1 | -1; readonly hold: number };

/** 🗻️ A planned trip with gear, leg after leg — the walk to where it sets out, then up or down walls and across the gaps between them, over a ladder it finds or raises, or up a grappling rope, across a perch it reaches on the way to the next leg —: its owner, the tick at whose end the feet are at the first foothold, a foothold per tick, the pitches its hands hold, the ladder it raises or climbs (named by its owner; `null`: none — a trip uses one ladder at most), what it ends in (`perch`: standing on the surface `landing`; `wall`: resting on the pitch its last foothold holds; `air`: letting go there) and the grip it has left then. */
export type Trip = { readonly owner: Slug; readonly from: Ticks; readonly steps: readonly Foothold[]; readonly pitches: readonly Pitch[]; readonly ladder: Slug | null; readonly ending: "perch" | "wall" | "air"; readonly landing: string; readonly grip: number };

/** 🌫️ The dust where a pet vanished on the spot: the middle and the size of its body and the tick it vanished. */
export type Puff = { readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly tick: Ticks };

/** 🧸️ One pet on stage: where its feet are and what carries it (`footing`; `perch` names the surface while it stands on a perch, `host` the actor while it stands on a head, `pitch` the stretch of wall it clings to while it is on a wall) and how long it can still hold on to a wall (`grip`, in ticks of climbing: what it has left as it climbs, and what it had left when it came to rest where it hangs — hanging still drains it from `since` on; whole while it is not on a wall), the tilt of its drawing about its scruff in turns, the way it faces and the tick from which it faces that way squarely (`faced`; it turns round over the ticks before), what it does and until when, whom it does it with, how it looks and its needs; what it feels (its spirits, the bend of its mouth, are read off the feeling) and the state of its species it is in since when (and the `former` one it was in just before, from which its drawing blends), the trick it performs, how warm it is from attention and the gestures the pointer may be making round it; the learner's hand, the parachute and the rope it hangs from, and the emitters it runs. */
export type Actor = {
  readonly species: Slug;
  readonly perch: string | null;
  readonly host: Slug | null;
  readonly pitch: Pitch | null;
  readonly grip: number;
  readonly footing: Footing;
  readonly x: number;
  readonly y: number;
  readonly vx: number;
  readonly vy: number;
  readonly tilt: Turns;
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
  readonly needs: Needs;
  readonly opacity: number;
  readonly leaving: boolean;
  readonly draws: number;
  readonly feeling: Feeling;
  readonly state: Slug;
  readonly stateSince: Ticks;
  readonly former: Slug;
  readonly trick: Slug | null;
  readonly warmth: Warmth;
  readonly hover: Hover;
  readonly hang: Hang | null;
  readonly chute: Parachute | null;
  readonly rope: Rope | null;
  readonly emitters: readonly Plume[];
};

/** 💞️ How the shared history of two actors has shifted their bond away from its authored affinity. */
export type Rapport = { readonly between: readonly [Slug, Slug]; readonly drift: number };

/** 🧊️ A reaction of the chemistry that rests for a pair of species, `when` the side it describes first, until a tick. */
export type Cooling = { readonly reaction: Slug; readonly when: Slug; readonly near: Slug; readonly until: Ticks };

/** 💍️ An encounter the chemistry promised two actors: the next time they are paired before `until`, it is this one. */
export type Pledge = { readonly between: readonly [Slug, Slug]; readonly encounter: "greet" | "cuddle" | "squabble"; readonly until: Ticks };

/** 🏗️ A ladder that stands on stage: its owner, the wall it leans against and the way that wall faces, the surface its foot stands on, its foot and its top, the tick it stood up, the tick it is taken away and who climbs it (`null`: nobody). */
export type Ladder = { readonly owner: Slug; readonly wall: string; readonly surface: string; readonly side: 1 | -1; readonly foot: Point; readonly top: Point; readonly since: Ticks; readonly until: Ticks; readonly rider: Slug | null };

/** 🃏️ The prank a pet plays with a fixture of the page, lifting its copy out of its stack: the fixture, its pusher, the tick its push begins (ahead while the pet is on its way to its post; moved on to the way back when the copy slides home without its pusher; moved so far back that the copy is over once the learner threw the pusher off — the prank lasts until the pusher is down again), the way the copy is shoved (`1` rightwards), the room it has there and the width of the fixture in pixels, and the unit the stage drew for how far it goes. */
export type Prank = { readonly fixture: string; readonly pusher: Slug; readonly since: Ticks; readonly side: 1 | -1; readonly room: number; readonly span: number; readonly unit: number };

/** 🏟️ The whole state of a stage; `advance` folds events into it and `frameOf` projects it. Besides the pointer (where it is, the tick it last moved and whether it is `over` free space or a control), the survey (surfaces, keep-outs, walls, fixtures and the perches and pitches cut from them), the company and its rapports: what the learner permits (`play`, `mischief`), the tick of the learner's last input (`stirred`) and of the last scroll (`scrolled`), the press, the actor it belongs to (`touched`) and the shake of a held pet, the pointer's latest samples while it holds one (`trail`, newest last), the chemistry that rests (`coolings`) and the encounters it promised (`pledges`), the ladders that stand, the prank with a fixture of the page (`lift`) and the tick from which the mode's cooldown counts (`rested`: the end of the last prank or of the last try to start one), how many pets vanished in a puff (`poofs`) and the dust of the latest ones (`puffs`), the corridors of planned motions (`claims`), the planned motions through the air themselves (`courses`) and the trips with gear (`trips`), and where the touched actor's feet stood when it was picked up from a perch (`origin`). */
export type Stage = {
  readonly seed: number;
  readonly tick: Ticks;
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly width: number;
  readonly height: number;
  readonly pointer: Point | null;
  readonly pointed: Ticks;
  readonly over: "free" | "control";
  readonly glances: readonly Point[];
  readonly surfaces: readonly Surface[];
  readonly keepouts: readonly Rect[];
  readonly walls: readonly Wall[];
  readonly fixtures: readonly Fixture[];
  readonly perches: readonly Perch[];
  readonly pitches: readonly Pitch[];
  readonly wanted: readonly Slug[];
  readonly actors: readonly Actor[];
  readonly rapports: readonly Rapport[];
  readonly met: Ticks;
  readonly draws: number;
  readonly play: boolean;
  readonly mischief: boolean;
  readonly stirred: Ticks;
  readonly scrolled: Ticks;
  readonly press: Press;
  readonly touched: Slug | null;
  readonly shaking: Shaking;
  readonly trail: readonly Point[];
  readonly coolings: readonly Cooling[];
  readonly pledges: readonly Pledge[];
  readonly ladders: readonly Ladder[];
  readonly lift: Prank | null;
  readonly rested: Ticks;
  readonly poofs: number;
  readonly puffs: readonly Puff[];
  readonly claims: readonly Claim[];
  readonly courses: readonly Course[];
  readonly trips: readonly Trip[];
  readonly origin: Point | null;
};
//#endregion 🔖️Stage

//#region 🔖️Frame
/** 👓️ One eye as drawn: the pupil's offset from the centre of the white in pixels and how far the lid is shut (0 open, 1 shut). */
export type EyeFrame = { readonly x: number; readonly y: number; readonly lid: number };

/** 🌂️ An open or opening parachute as drawn: how far it is open (0 packed … 1 open, may overshoot) and how far canopy and cords lean, in turns on screen. */
export type ChuteTool = { readonly kind: "chute"; readonly open: number; readonly sway: Turns };

/** ➿️ A rope as drawn from the actor (from the muzzle of its gun when it holds one, else from its pivot) to its far end in stage coordinates, its middle hanging `slack` pixels below the straight line (negative: it bows up). */
export type RopeTool = { readonly kind: "rope"; readonly x: number; readonly y: number; readonly slack: number };

/** 🪝️ A grappling hook as drawn, in stage coordinates; it points the way its rope arrives. */
export type HookTool = { readonly kind: "hook"; readonly x: number; readonly y: number };

/** 🔫️ A grappling gun in the actor's hands, aimed in turns on screen (0 to the right, ¼ down). */
export type GunTool = { readonly kind: "gun"; readonly aim: Turns };

/** 🎒️ A ladder the actor carries: its lean from upright in turns on screen and its length in pixels. */
export type LadderTool = { readonly kind: "ladder"; readonly lean: Turns; readonly length: number };

/** 🔧️ Something an actor holds or hangs from as drawn; `kind` names the variant. */
export type ToolFrame = ChuteTool | RopeTool | HookTool | GunTool | LadderTool;

/** 🖼️ One actor as drawn: its feet, the way it faces, what it is doing, its opacity, a 2×3 matrix `a b c d e f` per bone in rig order and its eyes; what carries it, the state of its species, its mood and how strongly it shows (0…1), its spirits (−1 sad … 1 happy: the bend of the mouth); the tilt of the whole drawing in turns (clockwise on screen) about `pivot` (in the coordinates of its feet, x mirrored with the facing), the tools it holds or hangs from and its solid box in stage coordinates (for hit tests). */
export type ActorFrame = {
  readonly species: Slug;
  readonly x: number;
  readonly y: number;
  readonly facing: 1 | -1;
  readonly activity: Activity;
  readonly opacity: number;
  readonly bones: readonly number[];
  readonly eyes: readonly EyeFrame[];
  readonly footing: Footing;
  readonly state: Slug;
  readonly mood: Mood;
  readonly intensity: number;
  readonly spirits: number;
  readonly tilt: Turns;
  readonly pivot: Point;
  readonly tools: readonly ToolFrame[];
  readonly body: Rect;
};

/** 🚒️ A ladder that stands on stage as drawn: from its foot (`x0`, `y0`) to its top (`x1`, `y1`) in stage coordinates, its rungs and its opacity. */
export type LadderFrame = { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number; readonly rungs: number; readonly opacity: number };

/** 🎆️ One particle as drawn: the species and the emitter whose shape and paints it borrows, where it is in stage coordinates, its scale, its rotation in turns and its opacity. */
export type ParticleFrame = { readonly species: Slug; readonly emitter: Slug; readonly x: number; readonly y: number; readonly scale: number; readonly rotation: Turns; readonly opacity: number };

/** 🏋️ How far a lifted fixture is out of its place as drawn: its offset in pixels, its tilt in turns about its centre and the opacity of the copy. */
export type LiftFrame = { readonly fixture: string; readonly dx: number; readonly dy: number; readonly tilt: Turns; readonly opacity: number };

/** 💭️ The dust where a pet vanished on the spot, as drawn: the middle and the size of the body it replaces, in stage coordinates, and how far it has spread and faded (`phase`: 0 the tick the pet vanished, towards 1 as it clears). */
export type PuffFrame = { readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly phase: number };

/** 🎥️ Everything a render target needs to draw a stage: the actors back to front, how many ticks per second the motion needs (0 = nothing moves) and the tick of the next change when nothing moves; the ladders that stand, the particles in flight, the lifted fixtures, the dust where pets vanished and the actor the learner holds (`null`: none; the target shows the grabbing cursor and captures the pointer). */
export type Frame = {
  readonly tick: Ticks;
  readonly actors: readonly ActorFrame[];
  readonly rate: 0 | 16 | 32 | 64;
  readonly wake: Ticks | null;
  readonly ladders: readonly LadderFrame[];
  readonly particles: readonly ParticleFrame[];
  readonly lifts: readonly LiftFrame[];
  readonly puffs: readonly PuffFrame[];
  readonly held: Slug | null;
};
//#endregion 🔖️Frame

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class frameworkProductPetsGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const frameworkProductPetsGuardReject = (at: string, why: string): never => {
  throw new frameworkProductPetsGuardRefusal(at, why);
};

type frameworkProductPetsGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type frameworkProductPetsGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type frameworkProductPetsGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const frameworkProductPetsGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : frameworkProductPetsGuardReject(at, "value is not an object");
export const frameworkProductPetsGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) frameworkProductPetsGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const frameworkProductPetsGuardArray = (value: unknown, at: string, bounds: frameworkProductPetsGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return frameworkProductPetsGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) frameworkProductPetsGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) frameworkProductPetsGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const frameworkProductPetsGuardString = (value: unknown, at: string, bounds: frameworkProductPetsGuardTextBounds = {}): string => {
  if (typeof value !== "string") return frameworkProductPetsGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) frameworkProductPetsGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) frameworkProductPetsGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) frameworkProductPetsGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const frameworkProductPetsGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : frameworkProductPetsGuardReject(at, "value is not a boolean"));
export const frameworkProductPetsGuardNumber = (value: unknown, at: string, bounds: frameworkProductPetsGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return frameworkProductPetsGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) frameworkProductPetsGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) frameworkProductPetsGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) frameworkProductPetsGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) frameworkProductPetsGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const frameworkProductPetsGuardInteger = (value: unknown, at: string, bounds: frameworkProductPetsGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? frameworkProductPetsGuardNumber(value, at, bounds) : frameworkProductPetsGuardReject(at, "value is not an integer");
export const frameworkProductPetsGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : frameworkProductPetsGuardReject(at, `value is not one of ${members.join(", ")}`);
export const frameworkProductPetsGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : frameworkProductPetsGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseActivity(value: unknown, at = "$"): Activity {
  return frameworkProductPetsGuardMember(value, `${at}`, ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"] as const);
}

export function parseActor(value: unknown, at = "$"): Actor {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["species", "perch", "host", "pitch", "grip", "footing", "x", "y", "vx", "vy", "tilt", "facing", "faced", "activity", "since", "until", "goal", "partner", "clip", "gaze", "blink", "needs", "opacity", "leaving", "draws", "feeling", "state", "stateSince", "former", "trick", "warmth", "hover", "hang", "chute", "rope", "emitters"]);
  return {
    species: parseSlug(row["species"], `${at}.species`),
    perch: (row["perch"] === null ? null : frameworkProductPetsGuardString(row["perch"], `${at}.perch`)),
    host: (row["host"] === null ? null : parseSlug(row["host"], `${at}.host`)),
    pitch: (row["pitch"] === null ? null : parsePitch(row["pitch"], `${at}.pitch`)),
    grip: frameworkProductPetsGuardNumber(row["grip"], `${at}.grip`, {"minimum": 0}),
    footing: parseFooting(row["footing"], `${at}.footing`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    vx: frameworkProductPetsGuardNumber(row["vx"], `${at}.vx`),
    vy: frameworkProductPetsGuardNumber(row["vy"], `${at}.vy`),
    tilt: parseTurns(row["tilt"], `${at}.tilt`),
    facing: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.facing`, "value is not one of 1, -1"))(row["facing"]),
    faced: parseTicks(row["faced"], `${at}.faced`),
    activity: parseActivity(row["activity"], `${at}.activity`),
    since: parseTicks(row["since"], `${at}.since`),
    until: parseTicks(row["until"], `${at}.until`),
    goal: frameworkProductPetsGuardNumber(row["goal"], `${at}.goal`),
    partner: (row["partner"] === null ? null : parseSlug(row["partner"], `${at}.partner`)),
    clip: (row["clip"] === null ? null : parseSlug(row["clip"], `${at}.clip`)),
    gaze: parseGaze(row["gaze"], `${at}.gaze`),
    blink: parseTicks(row["blink"], `${at}.blink`),
    needs: parseNeeds(row["needs"], `${at}.needs`),
    opacity: frameworkProductPetsGuardNumber(row["opacity"], `${at}.opacity`, {"minimum": 0, "maximum": 1}),
    leaving: frameworkProductPetsGuardBoolean(row["leaving"], `${at}.leaving`),
    draws: frameworkProductPetsGuardInteger(row["draws"], `${at}.draws`, {"minimum": 0}),
    feeling: parseFeeling(row["feeling"], `${at}.feeling`),
    state: parseSlug(row["state"], `${at}.state`),
    stateSince: parseTicks(row["stateSince"], `${at}.stateSince`),
    former: parseSlug(row["former"], `${at}.former`),
    trick: (row["trick"] === null ? null : parseSlug(row["trick"], `${at}.trick`)),
    warmth: parseWarmth(row["warmth"], `${at}.warmth`),
    hover: parseHover(row["hover"], `${at}.hover`),
    hang: (row["hang"] === null ? null : parseHang(row["hang"], `${at}.hang`)),
    chute: (row["chute"] === null ? null : parseParachute(row["chute"], `${at}.chute`)),
    rope: (row["rope"] === null ? null : parseRope(row["rope"], `${at}.rope`)),
    emitters: frameworkProductPetsGuardArray(row["emitters"], `${at}.emitters`).map((item, index) => parsePlume(item, `${at}.emitters[${index}]`)),
  };
}

export function parseActorFrame(value: unknown, at = "$"): ActorFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["species", "x", "y", "facing", "activity", "opacity", "bones", "eyes", "footing", "state", "mood", "intensity", "spirits", "tilt", "pivot", "tools", "body"]);
  return {
    species: parseSlug(row["species"], `${at}.species`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    facing: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.facing`, "value is not one of 1, -1"))(row["facing"]),
    activity: parseActivity(row["activity"], `${at}.activity`),
    opacity: frameworkProductPetsGuardNumber(row["opacity"], `${at}.opacity`, {"minimum": 0, "maximum": 1}),
    bones: frameworkProductPetsGuardArray(row["bones"], `${at}.bones`).map((item, index) => frameworkProductPetsGuardNumber(item, `${at}.bones[${index}]`)),
    eyes: frameworkProductPetsGuardArray(row["eyes"], `${at}.eyes`).map((item, index) => parseEyeFrame(item, `${at}.eyes[${index}]`)),
    footing: parseFooting(row["footing"], `${at}.footing`),
    state: parseSlug(row["state"], `${at}.state`),
    mood: parseMood(row["mood"], `${at}.mood`),
    intensity: frameworkProductPetsGuardNumber(row["intensity"], `${at}.intensity`, {"minimum": 0, "maximum": 1}),
    spirits: frameworkProductPetsGuardNumber(row["spirits"], `${at}.spirits`, {"minimum": -1, "maximum": 1}),
    tilt: parseTurns(row["tilt"], `${at}.tilt`),
    pivot: parsePoint(row["pivot"], `${at}.pivot`),
    tools: frameworkProductPetsGuardArray(row["tools"], `${at}.tools`).map((item, index) => parseToolFrame(item, `${at}.tools[${index}]`)),
    body: parseRect(row["body"], `${at}.body`),
  };
}

export function parseBond(value: unknown, at = "$"): Bond {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["between", "affinity"]);
  return {
    between: ((items0) => [parseSlug(items0[0], `${at}.between[0]`), parseSlug(items0[1], `${at}.between[1]`)])(frameworkProductPetsGuardArray(row["between"], `${at}.between`, {"minItems": 2, "maxItems": 2})),
    affinity: frameworkProductPetsGuardNumber(row["affinity"], `${at}.affinity`, {"minimum": -1, "maximum": 1}),
  };
}

export function parseBone(value: unknown, at = "$"): Bone {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "parent", "x", "y", "rotation"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    parent: row["parent"] === undefined ? undefined : parseSlug(row["parent"], `${at}.parent`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    rotation: row["rotation"] === undefined ? undefined : parseDegrees(row["rotation"], `${at}.rotation`),
  };
}

export function parseCancelled(value: unknown, at = "$"): Cancelled {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "cancelled"),
  };
}

export function parseCanopy(value: unknown, at = "$"): Canopy {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "vx", "vy", "bob", "previous"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    vx: frameworkProductPetsGuardNumber(row["vx"], `${at}.vx`),
    vy: frameworkProductPetsGuardNumber(row["vy"], `${at}.vy`),
    bob: parsePoint(row["bob"], `${at}.bob`),
    previous: parsePoint(row["previous"], `${at}.previous`),
  };
}

export function parseCast(value: unknown, at = "$"): Cast {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["scene", "core", "rotation"]);
  return {
    scene: parseSlug(row["scene"], `${at}.scene`),
    core: frameworkProductPetsGuardArray(row["core"], `${at}.core`, {"minItems": 1}).map((item, index) => parseSlug(item, `${at}.core[${index}]`)),
    rotation: frameworkProductPetsGuardArray(row["rotation"], `${at}.rotation`).map((item, index) => parseSlug(item, `${at}.rotation[${index}]`)),
  };
}

export function parseChannel(value: unknown, at = "$"): Channel {
  return frameworkProductPetsGuardMember(value, `${at}`, ["x", "y", "rotation", "scaleX", "scaleY"] as const);
}

export function parseChuteTool(value: unknown, at = "$"): ChuteTool {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "open", "sway"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "chute"),
    open: frameworkProductPetsGuardNumber(row["open"], `${at}.open`, {"minimum": 0}),
    sway: parseTurns(row["sway"], `${at}.sway`),
  };
}

export function parseCircling(value: unknown, at = "$"): Circling {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["live", "inside", "sx", "sy", "px", "py", "turn", "quarters", "steps", "against", "first", "last", "open", "near", "far", "rest"]);
  return {
    live: frameworkProductPetsGuardBoolean(row["live"], `${at}.live`),
    inside: parseTicks(row["inside"], `${at}.inside`),
    sx: frameworkProductPetsGuardInteger(row["sx"], `${at}.sx`, {"minimum": -1, "maximum": 1}),
    sy: frameworkProductPetsGuardInteger(row["sy"], `${at}.sy`, {"minimum": -1, "maximum": 1}),
    px: frameworkProductPetsGuardNumber(row["px"], `${at}.px`),
    py: frameworkProductPetsGuardNumber(row["py"], `${at}.py`),
    turn: frameworkProductPetsGuardInteger(row["turn"], `${at}.turn`, {"minimum": -1, "maximum": 1}),
    quarters: frameworkProductPetsGuardInteger(row["quarters"], `${at}.quarters`),
    steps: frameworkProductPetsGuardInteger(row["steps"], `${at}.steps`, {"minimum": 0}),
    against: frameworkProductPetsGuardInteger(row["against"], `${at}.against`, {"minimum": 0}),
    first: parseTicks(row["first"], `${at}.first`),
    last: parseTicks(row["last"], `${at}.last`),
    open: frameworkProductPetsGuardNumber(row["open"], `${at}.open`, {"minimum": 0}),
    near: frameworkProductPetsGuardNumber(row["near"], `${at}.near`, {"minimum": 0}),
    far: frameworkProductPetsGuardNumber(row["far"], `${at}.far`, {"minimum": 0}),
    rest: parseTicks(row["rest"], `${at}.rest`),
  };
}

export function parseClaim(value: unknown, at = "$"): Claim {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["owner", "slices", "rest"]);
  return {
    owner: parseSlug(row["owner"], `${at}.owner`),
    slices: frameworkProductPetsGuardArray(row["slices"], `${at}.slices`).map((item, index) => parseSlice(item, `${at}.slices[${index}]`)),
    rest: (row["rest"] === null ? null : parseExtent(row["rest"], `${at}.rest`)),
  };
}

export function parseClip(value: unknown, at = "$"): Clip {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "seconds", "loop", "tracks"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    seconds: frameworkProductPetsGuardNumber(row["seconds"], `${at}.seconds`, {"exclusiveMinimum": 0}),
    loop: frameworkProductPetsGuardBoolean(row["loop"], `${at}.loop`),
    tracks: frameworkProductPetsGuardArray(row["tracks"], `${at}.tracks`).map((item, index) => parseTrack(item, `${at}.tracks[${index}]`)),
  };
}

export function parseColor(value: unknown, at = "$"): Color {
  return frameworkProductPetsGuardString(value, `${at}`, {"pattern": "^#[0-9a-f]{6}$"});
}

export function parseCooling(value: unknown, at = "$"): Cooling {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["reaction", "when", "near", "until"]);
  return {
    reaction: parseSlug(row["reaction"], `${at}.reaction`),
    when: parseSlug(row["when"], `${at}.when`),
    near: parseSlug(row["near"], `${at}.near`),
    until: parseTicks(row["until"], `${at}.until`),
  };
}

export function parseCourse(value: unknown, at = "$"): Course {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["owner", "from", "steps", "ending", "landing", "touch"]);
  return {
    owner: parseSlug(row["owner"], `${at}.owner`),
    from: parseTicks(row["from"], `${at}.from`),
    steps: frameworkProductPetsGuardArray(row["steps"], `${at}.steps`, {"minItems": 1}).map((item, index) => parseWaypoint(item, `${at}.steps[${index}]`)),
    ending: frameworkProductPetsGuardMember(row["ending"], `${at}.ending`, ["perch", "head", "away"] as const),
    landing: frameworkProductPetsGuardString(row["landing"], `${at}.landing`),
    touch: frameworkProductPetsGuardNumber(row["touch"], `${at}.touch`),
  };
}

export function parseCue(value: unknown, at = "$"): Cue {
  return frameworkProductPetsGuardMember(value, `${at}`, ["click", "circle", "countercircle", "stroke", "shake", "whim", "show"] as const);
}

export function parseDeed(value: unknown, at = "$"): Deed {
  return frameworkProductPetsGuardMember(value, `${at}`, ["hello", "trick", "pet", "toss", "circle", "countercircle", "stroke", "shake"] as const);
}

export function parseDegrees(value: unknown, at = "$"): Degrees {
  return frameworkProductPetsGuardNumber(value, `${at}`);
}

export function parseDragged(value: unknown, at = "$"): Dragged {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "dragged"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseDrift(value: unknown, at = "$"): Drift {
  return frameworkProductPetsGuardMember(value, `${at}`, ["fall", "rise", "burst", "orbit", "drift"] as const);
}

export function parseEase(value: unknown, at = "$"): Ease {
  return ((items0) => [frameworkProductPetsGuardNumber(items0[0], `${at}[0]`, {"minimum": 0, "maximum": 1}), frameworkProductPetsGuardNumber(items0[1], `${at}[1]`), frameworkProductPetsGuardNumber(items0[2], `${at}[2]`, {"minimum": 0, "maximum": 1}), frameworkProductPetsGuardNumber(items0[3], `${at}[3]`)])(frameworkProductPetsGuardArray(value, `${at}`, {"minItems": 4, "maxItems": 4}));
}

export function parseEffect(value: unknown, at = "$"): Effect {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["on", "state", "mood", "amount", "rapport", "encounter", "trick", "activity"]);
  return {
    on: frameworkProductPetsGuardMember(row["on"], `${at}.on`, ["when", "near"] as const),
    state: row["state"] === undefined ? undefined : parseSlug(row["state"], `${at}.state`),
    mood: row["mood"] === undefined ? undefined : parseMood(row["mood"], `${at}.mood`),
    amount: row["amount"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["amount"], `${at}.amount`, {"minimum": 0, "maximum": 1}),
    rapport: row["rapport"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["rapport"], `${at}.rapport`, {"minimum": -1, "maximum": 1}),
    encounter: row["encounter"] === undefined ? undefined : frameworkProductPetsGuardMember(row["encounter"], `${at}.encounter`, ["greet", "cuddle", "squabble"] as const),
    trick: row["trick"] === undefined ? undefined : parseSlug(row["trick"], `${at}.trick`),
    activity: row["activity"] === undefined ? undefined : frameworkProductPetsGuardMember(row["activity"], `${at}.activity`, ["fidget", "walk", "hop", "sleep"] as const),
  };
}

export function parseEllipseShape(value: unknown, at = "$"): EllipseShape {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "cx", "cy", "rx", "ry"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "ellipse"),
    cx: frameworkProductPetsGuardNumber(row["cx"], `${at}.cx`),
    cy: frameworkProductPetsGuardNumber(row["cy"], `${at}.cy`),
    rx: frameworkProductPetsGuardNumber(row["rx"], `${at}.rx`, {"exclusiveMinimum": 0}),
    ry: frameworkProductPetsGuardNumber(row["ry"], `${at}.ry`, {"exclusiveMinimum": 0}),
  };
}

export function parseEmitter(value: unknown, at = "$"): Emitter {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "bone", "x", "y", "shape", "fill", "stroke", "strokeWidth", "motion", "count", "life", "speed", "spread"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    bone: parseSlug(row["bone"], `${at}.bone`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    shape: parseShape(row["shape"], `${at}.shape`),
    fill: parsePaint(row["fill"], `${at}.fill`),
    stroke: parsePaint(row["stroke"], `${at}.stroke`),
    strokeWidth: row["strokeWidth"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["strokeWidth"], `${at}.strokeWidth`, {"exclusiveMinimum": 0}),
    motion: parseDrift(row["motion"], `${at}.motion`),
    count: frameworkProductPetsGuardInteger(row["count"], `${at}.count`, {"minimum": 1, "maximum": 32}),
    life: frameworkProductPetsGuardNumber(row["life"], `${at}.life`, {"exclusiveMinimum": 0}),
    speed: frameworkProductPetsGuardNumber(row["speed"], `${at}.speed`, {"minimum": 0}),
    spread: frameworkProductPetsGuardNumber(row["spread"], `${at}.spread`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseEnsemble(value: unknown, at = "$"): Ensemble {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["$schema", "schema", "id", "title", "species", "bonds", "casts", "chemistry"]);
  return {
    "$schema": row["$schema"] === undefined ? undefined : frameworkProductPetsGuardString(row["$schema"], `${at}.$schema`),
    schema: frameworkProductPetsGuardConstant(row["schema"], `${at}.schema`, "semio.pets.ensemble/v1"),
    id: parseSlug(row["id"], `${at}.id`),
    title: parseText(row["title"], `${at}.title`),
    species: frameworkProductPetsGuardArray(row["species"], `${at}.species`).map((item, index) => frameworkProductPetsGuardString(item, `${at}.species[${index}]`, {"minLength": 1})),
    bonds: frameworkProductPetsGuardArray(row["bonds"], `${at}.bonds`).map((item, index) => parseBond(item, `${at}.bonds[${index}]`)),
    casts: frameworkProductPetsGuardArray(row["casts"], `${at}.casts`).map((item, index) => parseCast(item, `${at}.casts[${index}]`)),
    chemistry: frameworkProductPetsGuardArray(row["chemistry"], `${at}.chemistry`).map((item, index) => parseReaction(item, `${at}.chemistry[${index}]`)),
  };
}

export function parseExtent(value: unknown, at = "$"): Extent {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x0", "y0", "x1", "y1"]);
  return {
    x0: frameworkProductPetsGuardNumber(row["x0"], `${at}.x0`),
    y0: frameworkProductPetsGuardNumber(row["y0"], `${at}.y0`),
    x1: frameworkProductPetsGuardNumber(row["x1"], `${at}.x1`),
    y1: frameworkProductPetsGuardNumber(row["y1"], `${at}.y1`),
  };
}

export function parseEye(value: unknown, at = "$"): Eye {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "bone", "x", "y", "radius", "pupil"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    bone: parseSlug(row["bone"], `${at}.bone`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    radius: frameworkProductPetsGuardNumber(row["radius"], `${at}.radius`, {"exclusiveMinimum": 0}),
    pupil: frameworkProductPetsGuardNumber(row["pupil"], `${at}.pupil`, {"exclusiveMinimum": 0}),
  };
}

export function parseEyeFrame(value: unknown, at = "$"): EyeFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "lid"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    lid: frameworkProductPetsGuardNumber(row["lid"], `${at}.lid`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseFace(value: unknown, at = "$"): Face {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["eyes", "mouth", "above"]);
  return {
    eyes: frameworkProductPetsGuardArray(row["eyes"], `${at}.eyes`).map((item, index) => parseEye(item, `${at}.eyes[${index}]`)),
    mouth: row["mouth"] === undefined ? undefined : parseMouth(row["mouth"], `${at}.mouth`),
    above: row["above"] === undefined ? undefined : parseSlug(row["above"], `${at}.above`),
  };
}

export function parseFeeling(value: unknown, at = "$"): Feeling {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["mood", "intensity", "since"]);
  return {
    mood: parseMood(row["mood"], `${at}.mood`),
    intensity: frameworkProductPetsGuardNumber(row["intensity"], `${at}.intensity`, {"minimum": 0, "maximum": 1}),
    since: parseTicks(row["since"], `${at}.since`),
  };
}

export function parseFixture(value: unknown, at = "$"): Fixture {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "key", "x", "y", "width", "height"]);
  return {
    id: frameworkProductPetsGuardString(row["id"], `${at}.id`, {"minLength": 1}),
    key: frameworkProductPetsGuardString(row["key"], `${at}.key`, {"minLength": 1}),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
  };
}

export function parseFoothold(value: unknown, at = "$"): Foothold {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "footing", "perch", "activity", "facing", "hold"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    footing: parseFooting(row["footing"], `${at}.footing`),
    perch: (row["perch"] === null ? null : frameworkProductPetsGuardString(row["perch"], `${at}.perch`)),
    activity: parseActivity(row["activity"], `${at}.activity`),
    facing: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.facing`, "value is not one of 1, -1"))(row["facing"]),
    hold: frameworkProductPetsGuardInteger(row["hold"], `${at}.hold`, {"minimum": -1}),
  };
}

export function parseFooting(value: unknown, at = "$"): Footing {
  return frameworkProductPetsGuardMember(value, `${at}`, ["perch", "air", "chute", "hand", "rope", "ladder", "wall", "head"] as const);
}

export function parseFrame(value: unknown, at = "$"): Frame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["tick", "actors", "rate", "wake", "ladders", "particles", "lifts", "puffs", "held"]);
  return {
    tick: frameworkProductPetsGuardInteger(row["tick"], `${at}.tick`, {"minimum": 0}),
    actors: frameworkProductPetsGuardArray(row["actors"], `${at}.actors`).map((item, index) => parseActorFrame(item, `${at}.actors[${index}]`)),
    rate: ((input0: unknown) => ([0, 16, 32, 64] as readonly unknown[]).includes(input0) ? (input0 as 0 | 16 | 32 | 64) : frameworkProductPetsGuardReject(`${at}.rate`, "value is not one of 0, 16, 32, 64"))(row["rate"]),
    wake: (row["wake"] === null ? null : parseTicks(row["wake"], `${at}.wake`)),
    ladders: frameworkProductPetsGuardArray(row["ladders"], `${at}.ladders`).map((item, index) => parseLadderFrame(item, `${at}.ladders[${index}]`)),
    particles: frameworkProductPetsGuardArray(row["particles"], `${at}.particles`).map((item, index) => parseParticleFrame(item, `${at}.particles[${index}]`)),
    lifts: frameworkProductPetsGuardArray(row["lifts"], `${at}.lifts`).map((item, index) => parseLiftFrame(item, `${at}.lifts[${index}]`)),
    puffs: frameworkProductPetsGuardArray(row["puffs"], `${at}.puffs`).map((item, index) => parsePuffFrame(item, `${at}.puffs[${index}]`)),
    held: (row["held"] === null ? null : parseSlug(row["held"], `${at}.held`)),
  };
}

export function parseGait(value: unknown, at = "$"): Gait {
  return frameworkProductPetsGuardMember(value, `${at}`, ["walk", "hop", "float"] as const);
}

export function parseGaze(value: unknown, at = "$"): Gaze {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "vx", "vy"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    vx: frameworkProductPetsGuardNumber(row["vx"], `${at}.vx`),
    vy: frameworkProductPetsGuardNumber(row["vy"], `${at}.vy`),
  };
}

export function parseGear(value: unknown, at = "$"): Gear {
  return frameworkProductPetsGuardMember(value, `${at}`, ["climb", "ladder", "grapple", "parachute"] as const);
}

export function parseGlanced(value: unknown, at = "$"): Glanced {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "points"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "glanced"),
    points: frameworkProductPetsGuardArray(row["points"], `${at}.points`).map((item, index) => parsePoint(item, `${at}.points[${index}]`)),
  };
}

export function parseGrip(value: unknown, at = "$"): Grip {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "vx", "vy"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    vx: frameworkProductPetsGuardNumber(row["vx"], `${at}.vx`),
    vy: frameworkProductPetsGuardNumber(row["vy"], `${at}.vy`),
  };
}

export function parseGunTool(value: unknown, at = "$"): GunTool {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "aim"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "gun"),
    aim: parseTurns(row["aim"], `${at}.aim`),
  };
}

export function parseHang(value: unknown, at = "$"): Hang {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["grip", "bob", "previous"]);
  return {
    grip: parseGrip(row["grip"], `${at}.grip`),
    bob: parsePoint(row["bob"], `${at}.bob`),
    previous: parsePoint(row["previous"], `${at}.previous`),
  };
}

export function parseHookTool(value: unknown, at = "$"): HookTool {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "hook"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseHover(value: unknown, at = "$"): Hover {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["circling", "stroking"]);
  return {
    circling: parseCircling(row["circling"], `${at}.circling`),
    stroking: parseStroking(row["stroking"], `${at}.stroking`),
  };
}

export function parseHushed(value: unknown, at = "$"): Hushed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "quiet"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "hushed"),
    quiet: frameworkProductPetsGuardBoolean(row["quiet"], `${at}.quiet`),
  };
}

export function parseKey(value: unknown, at = "$"): Key {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["at", "value", "ease"]);
  return {
    at: frameworkProductPetsGuardNumber(row["at"], `${at}.at`, {"minimum": 0, "maximum": 1}),
    value: frameworkProductPetsGuardNumber(row["value"], `${at}.value`),
    ease: row["ease"] === undefined ? undefined : parseEase(row["ease"], `${at}.ease`),
  };
}

export function parseLadder(value: unknown, at = "$"): Ladder {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["owner", "wall", "surface", "side", "foot", "top", "since", "until", "rider"]);
  return {
    owner: parseSlug(row["owner"], `${at}.owner`),
    wall: frameworkProductPetsGuardString(row["wall"], `${at}.wall`, {"minLength": 1}),
    surface: frameworkProductPetsGuardString(row["surface"], `${at}.surface`, {"minLength": 1}),
    side: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.side`, "value is not one of 1, -1"))(row["side"]),
    foot: parsePoint(row["foot"], `${at}.foot`),
    top: parsePoint(row["top"], `${at}.top`),
    since: parseTicks(row["since"], `${at}.since`),
    until: parseTicks(row["until"], `${at}.until`),
    rider: (row["rider"] === null ? null : parseSlug(row["rider"], `${at}.rider`)),
  };
}

export function parseLadderFrame(value: unknown, at = "$"): LadderFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x0", "y0", "x1", "y1", "rungs", "opacity"]);
  return {
    x0: frameworkProductPetsGuardNumber(row["x0"], `${at}.x0`),
    y0: frameworkProductPetsGuardNumber(row["y0"], `${at}.y0`),
    x1: frameworkProductPetsGuardNumber(row["x1"], `${at}.x1`),
    y1: frameworkProductPetsGuardNumber(row["y1"], `${at}.y1`),
    rungs: frameworkProductPetsGuardInteger(row["rungs"], `${at}.rungs`, {"minimum": 0}),
    opacity: frameworkProductPetsGuardNumber(row["opacity"], `${at}.opacity`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseLadderTool(value: unknown, at = "$"): LadderTool {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "lean", "length"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "ladder"),
    lean: parseTurns(row["lean"], `${at}.lean`),
    length: frameworkProductPetsGuardNumber(row["length"], `${at}.length`, {"minimum": 0}),
  };
}

export function parseLiftFrame(value: unknown, at = "$"): LiftFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["fixture", "dx", "dy", "tilt", "opacity"]);
  return {
    fixture: frameworkProductPetsGuardString(row["fixture"], `${at}.fixture`, {"minLength": 1}),
    dx: frameworkProductPetsGuardNumber(row["dx"], `${at}.dx`),
    dy: frameworkProductPetsGuardNumber(row["dy"], `${at}.dy`),
    tilt: parseTurns(row["tilt"], `${at}.tilt`),
    opacity: frameworkProductPetsGuardNumber(row["opacity"], `${at}.opacity`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseLineShape(value: unknown, at = "$"): LineShape {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x1", "y1", "x2", "y2"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "line"),
    x1: frameworkProductPetsGuardNumber(row["x1"], `${at}.x1`),
    y1: frameworkProductPetsGuardNumber(row["y1"], `${at}.y1`),
    x2: frameworkProductPetsGuardNumber(row["x2"], `${at}.x2`),
    y2: frameworkProductPetsGuardNumber(row["y2"], `${at}.y2`),
  };
}

export function parseLocomotion(value: unknown, at = "$"): Locomotion {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["gait", "speed", "hover"]);
  return {
    gait: parseGait(row["gait"], `${at}.gait`),
    speed: frameworkProductPetsGuardNumber(row["speed"], `${at}.speed`, {"exclusiveMinimum": 0}),
    hover: row["hover"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["hover"], `${at}.hover`, {"exclusiveMinimum": 0}),
  };
}

export function parseMenagerie(value: unknown, at = "$"): Menagerie {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["$schema", "schema", "id", "title", "species", "bonds", "casts", "chemistry"]);
  return {
    "$schema": row["$schema"] === undefined ? undefined : frameworkProductPetsGuardString(row["$schema"], `${at}.$schema`),
    schema: frameworkProductPetsGuardConstant(row["schema"], `${at}.schema`, "semio.pets.menagerie/v1"),
    id: parseSlug(row["id"], `${at}.id`),
    title: parseText(row["title"], `${at}.title`),
    species: frameworkProductPetsGuardArray(row["species"], `${at}.species`).map((item, index) => parseSpecies(item, `${at}.species[${index}]`)),
    bonds: frameworkProductPetsGuardArray(row["bonds"], `${at}.bonds`).map((item, index) => parseBond(item, `${at}.bonds[${index}]`)),
    casts: frameworkProductPetsGuardArray(row["casts"], `${at}.casts`).map((item, index) => parseCast(item, `${at}.casts[${index}]`)),
    chemistry: frameworkProductPetsGuardArray(row["chemistry"], `${at}.chemistry`).map((item, index) => parseReaction(item, `${at}.chemistry[${index}]`)),
  };
}

export function parseMood(value: unknown, at = "$"): Mood {
  return frameworkProductPetsGuardMember(value, `${at}`, ["content", "happy", "playful", "curious", "proud", "sleepy", "grumpy", "sad", "scared"] as const);
}

export function parseMouth(value: unknown, at = "$"): Mouth {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["bone", "x", "y", "width"]);
  return {
    bone: parseSlug(row["bone"], `${at}.bone`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"exclusiveMinimum": 0}),
  };
}

export function parseNeeds(value: unknown, at = "$"): Needs {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["energy", "sociability", "curiosity"]);
  return {
    energy: frameworkProductPetsGuardNumber(row["energy"], `${at}.energy`, {"minimum": 0, "maximum": 1}),
    sociability: frameworkProductPetsGuardNumber(row["sociability"], `${at}.sociability`, {"minimum": 0, "maximum": 1}),
    curiosity: frameworkProductPetsGuardNumber(row["curiosity"], `${at}.curiosity`, {"minimum": 0, "maximum": 1}),
  };
}

export function parsePaint(value: unknown, at = "$"): Paint {
  return frameworkProductPetsGuardMember(value, `${at}`, ["body", "accent", "detail", "ink", "paper", "none"] as const);
}

export function parsePalette(value: unknown, at = "$"): Palette {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["body", "accent", "detail"]);
  return {
    body: parseColor(row["body"], `${at}.body`),
    accent: parseColor(row["accent"], `${at}.accent`),
    detail: parseColor(row["detail"], `${at}.detail`),
  };
}

export function parseParachute(value: unknown, at = "$"): Parachute {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["since", "open", "opening", "canopy"]);
  return {
    since: parseTicks(row["since"], `${at}.since`),
    open: frameworkProductPetsGuardNumber(row["open"], `${at}.open`, {"minimum": 0}),
    opening: frameworkProductPetsGuardNumber(row["opening"], `${at}.opening`),
    canopy: parseCanopy(row["canopy"], `${at}.canopy`),
  };
}

export function parsePart(value: unknown, at = "$"): Part {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "bone", "shape", "fill", "stroke", "strokeWidth"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    bone: parseSlug(row["bone"], `${at}.bone`),
    shape: parseShape(row["shape"], `${at}.shape`),
    fill: parsePaint(row["fill"], `${at}.fill`),
    stroke: parsePaint(row["stroke"], `${at}.stroke`),
    strokeWidth: row["strokeWidth"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["strokeWidth"], `${at}.strokeWidth`, {"exclusiveMinimum": 0}),
  };
}

export function parseParticleFrame(value: unknown, at = "$"): ParticleFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["species", "emitter", "x", "y", "scale", "rotation", "opacity"]);
  return {
    species: parseSlug(row["species"], `${at}.species`),
    emitter: parseSlug(row["emitter"], `${at}.emitter`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    scale: frameworkProductPetsGuardNumber(row["scale"], `${at}.scale`),
    rotation: parseTurns(row["rotation"], `${at}.rotation`),
    opacity: frameworkProductPetsGuardNumber(row["opacity"], `${at}.opacity`, {"minimum": 0, "maximum": 1}),
  };
}

export function parsePathShape(value: unknown, at = "$"): PathShape {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "d"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "path"),
    d: frameworkProductPetsGuardString(row["d"], `${at}.d`, {"minLength": 1}),
  };
}

export function parsePerch(value: unknown, at = "$"): Perch {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["surface", "x0", "x1", "y"]);
  return {
    surface: frameworkProductPetsGuardString(row["surface"], `${at}.surface`, {"minLength": 1}),
    x0: frameworkProductPetsGuardNumber(row["x0"], `${at}.x0`),
    x1: frameworkProductPetsGuardNumber(row["x1"], `${at}.x1`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePermitted(value: unknown, at = "$"): Permitted {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "play", "mischief"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "permitted"),
    play: frameworkProductPetsGuardBoolean(row["play"], `${at}.play`),
    mischief: frameworkProductPetsGuardBoolean(row["mischief"], `${at}.mischief`),
  };
}

export function parsePetMode(value: unknown, at = "$"): PetMode {
  return frameworkProductPetsGuardMember(value, `${at}`, ["still", "calm", "lively"] as const);
}

export function parsePitch(value: unknown, at = "$"): Pitch {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["wall", "surface", "side", "x", "y0", "y1"]);
  return {
    wall: frameworkProductPetsGuardString(row["wall"], `${at}.wall`, {"minLength": 1}),
    surface: frameworkProductPetsGuardString(row["surface"], `${at}.surface`, {"minLength": 1}),
    side: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.side`, "value is not one of 1, -1"))(row["side"]),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y0: frameworkProductPetsGuardNumber(row["y0"], `${at}.y0`),
    y1: frameworkProductPetsGuardNumber(row["y1"], `${at}.y1`),
  };
}

export function parsePlayed(value: unknown, at = "$"): Played {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "species", "deed"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "played"),
    species: parseSlug(row["species"], `${at}.species`),
    deed: parseDeed(row["deed"], `${at}.deed`),
  };
}

export function parsePledge(value: unknown, at = "$"): Pledge {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["between", "encounter", "until"]);
  return {
    between: ((items0) => [parseSlug(items0[0], `${at}.between[0]`), parseSlug(items0[1], `${at}.between[1]`)])(frameworkProductPetsGuardArray(row["between"], `${at}.between`, {"minItems": 2, "maxItems": 2})),
    encounter: frameworkProductPetsGuardMember(row["encounter"], `${at}.encounter`, ["greet", "cuddle", "squabble"] as const),
    until: parseTicks(row["until"], `${at}.until`),
  };
}

export function parsePlume(value: unknown, at = "$"): Plume {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["emitter", "since", "until"]);
  return {
    emitter: parseSlug(row["emitter"], `${at}.emitter`),
    since: parseTicks(row["since"], `${at}.since`),
    until: (row["until"] === null ? null : parseTicks(row["until"], `${at}.until`)),
  };
}

export function parsePoint(value: unknown, at = "$"): Point {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePointed(value: unknown, at = "$"): Pointed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y", "over"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "pointed"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    over: frameworkProductPetsGuardMember(row["over"], `${at}.over`, ["free", "control"] as const),
  };
}

export function parsePointer(value: unknown, at = "$"): Pointer {
  return frameworkProductPetsGuardMember(value, `${at}`, ["mouse", "pen", "touch"] as const);
}

export function parsePrank(value: unknown, at = "$"): Prank {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["fixture", "pusher", "since", "side", "room", "span", "unit"]);
  return {
    fixture: frameworkProductPetsGuardString(row["fixture"], `${at}.fixture`, {"minLength": 1}),
    pusher: parseSlug(row["pusher"], `${at}.pusher`),
    since: parseTicks(row["since"], `${at}.since`),
    side: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.side`, "value is not one of 1, -1"))(row["side"]),
    room: frameworkProductPetsGuardNumber(row["room"], `${at}.room`, {"minimum": 0}),
    span: frameworkProductPetsGuardNumber(row["span"], `${at}.span`, {"minimum": 0}),
    unit: frameworkProductPetsGuardNumber(row["unit"], `${at}.unit`, {"minimum": 0, "maximum": 1}),
  };
}

export function parsePress(value: unknown, at = "$"): Press {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["phase", "x", "y", "since", "slop"]);
  return {
    phase: parsePressPhase(row["phase"], `${at}.phase`),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    since: parseTicks(row["since"], `${at}.since`),
    slop: frameworkProductPetsGuardNumber(row["slop"], `${at}.slop`, {"minimum": 0}),
  };
}

export function parsePressPhase(value: unknown, at = "$"): PressPhase {
  return frameworkProductPetsGuardMember(value, `${at}`, ["idle", "armed", "holding", "lifted"] as const);
}

export function parsePressed(value: unknown, at = "$"): Pressed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y", "pointer"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "pressed"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    pointer: parsePointer(row["pointer"], `${at}.pointer`),
  };
}

export function parsePuff(value: unknown, at = "$"): Puff {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "width", "height", "tick"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
    tick: parseTicks(row["tick"], `${at}.tick`),
  };
}

export function parsePuffFrame(value: unknown, at = "$"): PuffFrame {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "width", "height", "phase"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
    phase: frameworkProductPetsGuardNumber(row["phase"], `${at}.phase`, {"minimum": 0, "exclusiveMaximum": 1}),
  };
}

export function parsePurr(value: unknown, at = "$"): Purr {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["clip", "emitter"]);
  return {
    clip: parseSlug(row["clip"], `${at}.clip`),
    emitter: row["emitter"] === undefined ? undefined : parseSlug(row["emitter"], `${at}.emitter`),
  };
}

export function parseRapport(value: unknown, at = "$"): Rapport {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["between", "drift"]);
  return {
    between: ((items0) => [parseSlug(items0[0], `${at}.between[0]`), parseSlug(items0[1], `${at}.between[1]`)])(frameworkProductPetsGuardArray(row["between"], `${at}.between`, {"minItems": 2, "maxItems": 2})),
    drift: frameworkProductPetsGuardNumber(row["drift"], `${at}.drift`),
  };
}

export function parseReaction(value: unknown, at = "$"): Reaction {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "when", "near", "within", "where", "unless", "affinity", "every", "chance", "then"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    when: parseTrait(row["when"], `${at}.when`),
    near: parseTrait(row["near"], `${at}.near`),
    within: frameworkProductPetsGuardNumber(row["within"], `${at}.within`, {"exclusiveMinimum": 0}),
    where: row["where"] === undefined ? undefined : frameworkProductPetsGuardMember(row["where"], `${at}.where`, ["above", "below", "beside", "any"] as const),
    unless: row["unless"] === undefined ? undefined : parseTrait(row["unless"], `${at}.unless`),
    affinity: row["affinity"] === undefined ? undefined : ((items0) => [frameworkProductPetsGuardNumber(items0[0], `${at}.affinity[0]`, {"minimum": -1, "maximum": 1}), frameworkProductPetsGuardNumber(items0[1], `${at}.affinity[1]`, {"minimum": -1, "maximum": 1})])(frameworkProductPetsGuardArray(row["affinity"], `${at}.affinity`, {"minItems": 2, "maxItems": 2})),
    every: frameworkProductPetsGuardNumber(row["every"], `${at}.every`, {"exclusiveMinimum": 0}),
    chance: row["chance"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["chance"], `${at}.chance`, {"minimum": 0, "maximum": 1}),
    then: frameworkProductPetsGuardArray(row["then"], `${at}.then`, {"minItems": 1}).map((item, index) => parseEffect(item, `${at}.then[${index}]`)),
  };
}

export function parseReclaimed(value: unknown, at = "$"): Reclaimed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "fixture"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "reclaimed"),
    fixture: frameworkProductPetsGuardString(row["fixture"], `${at}.fixture`, {"minLength": 1}),
  };
}

export function parseRect(value: unknown, at = "$"): Rect {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "width", "height"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
  };
}

export function parseRectShape(value: unknown, at = "$"): RectShape {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y", "width", "height", "radius"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "rect"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"exclusiveMinimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"exclusiveMinimum": 0}),
    radius: row["radius"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["radius"], `${at}.radius`, {"minimum": 0}),
  };
}

export function parseReleased(value: unknown, at = "$"): Released {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "released"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseRepertoire(value: unknown, at = "$"): Repertoire {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"]);
  return {
    idle: row["idle"] === undefined ? undefined : frameworkProductPetsGuardArray(row["idle"], `${at}.idle`).map((item, index) => parseSlug(item, `${at}.idle[${index}]`)),
    fidget: row["fidget"] === undefined ? undefined : frameworkProductPetsGuardArray(row["fidget"], `${at}.fidget`).map((item, index) => parseSlug(item, `${at}.fidget[${index}]`)),
    walk: row["walk"] === undefined ? undefined : frameworkProductPetsGuardArray(row["walk"], `${at}.walk`).map((item, index) => parseSlug(item, `${at}.walk[${index}]`)),
    hop: row["hop"] === undefined ? undefined : frameworkProductPetsGuardArray(row["hop"], `${at}.hop`).map((item, index) => parseSlug(item, `${at}.hop[${index}]`)),
    fall: row["fall"] === undefined ? undefined : frameworkProductPetsGuardArray(row["fall"], `${at}.fall`).map((item, index) => parseSlug(item, `${at}.fall[${index}]`)),
    land: row["land"] === undefined ? undefined : frameworkProductPetsGuardArray(row["land"], `${at}.land`).map((item, index) => parseSlug(item, `${at}.land[${index}]`)),
    sleep: row["sleep"] === undefined ? undefined : frameworkProductPetsGuardArray(row["sleep"], `${at}.sleep`).map((item, index) => parseSlug(item, `${at}.sleep[${index}]`)),
    greet: row["greet"] === undefined ? undefined : frameworkProductPetsGuardArray(row["greet"], `${at}.greet`).map((item, index) => parseSlug(item, `${at}.greet[${index}]`)),
    cuddle: row["cuddle"] === undefined ? undefined : frameworkProductPetsGuardArray(row["cuddle"], `${at}.cuddle`).map((item, index) => parseSlug(item, `${at}.cuddle[${index}]`)),
    squabble: row["squabble"] === undefined ? undefined : frameworkProductPetsGuardArray(row["squabble"], `${at}.squabble`).map((item, index) => parseSlug(item, `${at}.squabble[${index}]`)),
    sulk: row["sulk"] === undefined ? undefined : frameworkProductPetsGuardArray(row["sulk"], `${at}.sulk`).map((item, index) => parseSlug(item, `${at}.sulk[${index}]`)),
    hang: row["hang"] === undefined ? undefined : frameworkProductPetsGuardArray(row["hang"], `${at}.hang`).map((item, index) => parseSlug(item, `${at}.hang[${index}]`)),
    tumble: row["tumble"] === undefined ? undefined : frameworkProductPetsGuardArray(row["tumble"], `${at}.tumble`).map((item, index) => parseSlug(item, `${at}.tumble[${index}]`)),
    glide: row["glide"] === undefined ? undefined : frameworkProductPetsGuardArray(row["glide"], `${at}.glide`).map((item, index) => parseSlug(item, `${at}.glide[${index}]`)),
    aim: row["aim"] === undefined ? undefined : frameworkProductPetsGuardArray(row["aim"], `${at}.aim`).map((item, index) => parseSlug(item, `${at}.aim[${index}]`)),
    reel: row["reel"] === undefined ? undefined : frameworkProductPetsGuardArray(row["reel"], `${at}.reel`).map((item, index) => parseSlug(item, `${at}.reel[${index}]`)),
    climb: row["climb"] === undefined ? undefined : frameworkProductPetsGuardArray(row["climb"], `${at}.climb`).map((item, index) => parseSlug(item, `${at}.climb[${index}]`)),
    mantle: row["mantle"] === undefined ? undefined : frameworkProductPetsGuardArray(row["mantle"], `${at}.mantle`).map((item, index) => parseSlug(item, `${at}.mantle[${index}]`)),
    slide: row["slide"] === undefined ? undefined : frameworkProductPetsGuardArray(row["slide"], `${at}.slide`).map((item, index) => parseSlug(item, `${at}.slide[${index}]`)),
    carry: row["carry"] === undefined ? undefined : frameworkProductPetsGuardArray(row["carry"], `${at}.carry`).map((item, index) => parseSlug(item, `${at}.carry[${index}]`)),
    trick: row["trick"] === undefined ? undefined : frameworkProductPetsGuardArray(row["trick"], `${at}.trick`).map((item, index) => parseSlug(item, `${at}.trick[${index}]`)),
    purr: row["purr"] === undefined ? undefined : frameworkProductPetsGuardArray(row["purr"], `${at}.purr`).map((item, index) => parseSlug(item, `${at}.purr[${index}]`)),
    dizzy: row["dizzy"] === undefined ? undefined : frameworkProductPetsGuardArray(row["dizzy"], `${at}.dizzy`).map((item, index) => parseSlug(item, `${at}.dizzy[${index}]`)),
    shrug: row["shrug"] === undefined ? undefined : frameworkProductPetsGuardArray(row["shrug"], `${at}.shrug`).map((item, index) => parseSlug(item, `${at}.shrug[${index}]`)),
    scoot: row["scoot"] === undefined ? undefined : frameworkProductPetsGuardArray(row["scoot"], `${at}.scoot`).map((item, index) => parseSlug(item, `${at}.scoot[${index}]`)),
    push: row["push"] === undefined ? undefined : frameworkProductPetsGuardArray(row["push"], `${at}.push`).map((item, index) => parseSlug(item, `${at}.push[${index}]`)),
  };
}

export function parseRope(value: unknown, at = "$"): Rope {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["shot", "since", "length", "hand", "before", "caught"]);
  return {
    shot: parseShot(row["shot"], `${at}.shot`),
    since: parseTicks(row["since"], `${at}.since`),
    length: frameworkProductPetsGuardNumber(row["length"], `${at}.length`, {"minimum": 0}),
    hand: parsePoint(row["hand"], `${at}.hand`),
    before: parsePoint(row["before"], `${at}.before`),
    caught: frameworkProductPetsGuardBoolean(row["caught"], `${at}.caught`),
  };
}

export function parseRopeTool(value: unknown, at = "$"): RopeTool {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "x", "y", "slack"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "rope"),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    slack: frameworkProductPetsGuardNumber(row["slack"], `${at}.slack`),
  };
}

export function parseScrolled(value: unknown, at = "$"): Scrolled {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "scrolled"),
  };
}

export function parseShaking(value: unknown, at = "$"): Shaking {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["live", "ax", "ay", "at", "fx", "fy", "reached", "count", "mark1", "mark2", "mark3", "rest"]);
  return {
    live: frameworkProductPetsGuardBoolean(row["live"], `${at}.live`),
    ax: frameworkProductPetsGuardNumber(row["ax"], `${at}.ax`),
    ay: frameworkProductPetsGuardNumber(row["ay"], `${at}.ay`),
    at: parseTicks(row["at"], `${at}.at`),
    fx: frameworkProductPetsGuardNumber(row["fx"], `${at}.fx`),
    fy: frameworkProductPetsGuardNumber(row["fy"], `${at}.fy`),
    reached: parseTicks(row["reached"], `${at}.reached`),
    count: frameworkProductPetsGuardInteger(row["count"], `${at}.count`, {"minimum": 0}),
    mark1: parseTicks(row["mark1"], `${at}.mark1`),
    mark2: parseTicks(row["mark2"], `${at}.mark2`),
    mark3: parseTicks(row["mark3"], `${at}.mark3`),
    rest: parseTicks(row["rest"], `${at}.rest`),
  };
}

export function parseShape(value: unknown, at = "$"): Shape {
  return ((input0: unknown) => { try { return parsePathShape(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseEllipseShape(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseRectShape(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseLineShape(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } return frameworkProductPetsGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseShot(value: unknown, at = "$"): Shot {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["surface", "facing", "muzzle", "hook", "length", "reel"]);
  return {
    surface: frameworkProductPetsGuardString(row["surface"], `${at}.surface`, {"minLength": 1}),
    facing: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.facing`, "value is not one of 1, -1"))(row["facing"]),
    muzzle: parsePoint(row["muzzle"], `${at}.muzzle`),
    hook: parsePoint(row["hook"], `${at}.hook`),
    length: frameworkProductPetsGuardNumber(row["length"], `${at}.length`, {"minimum": 0}),
    reel: frameworkProductPetsGuardMember(row["reel"], `${at}.reel`, ["zip", "swing"] as const),
  };
}

export function parseSize(value: unknown, at = "$"): Size {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["width", "height"]);
  return {
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"exclusiveMinimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"exclusiveMinimum": 0}),
  };
}

export function parseSlice(value: unknown, at = "$"): Slice {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["from", "until", "extent"]);
  return {
    from: parseTicks(row["from"], `${at}.from`),
    until: parseTicks(row["until"], `${at}.until`),
    extent: parseExtent(row["extent"], `${at}.extent`),
  };
}

export function parseSlug(value: unknown, at = "$"): Slug {
  return frameworkProductPetsGuardString(value, `${at}`, {"minLength": 1, "maxLength": 64, "pattern": "^[a-z0-9]+(?:-[a-z0-9]+)*$"});
}

export function parseSpecies(value: unknown, at = "$"): Species {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["$schema", "id", "name", "thing", "grounds", "size", "palette", "bones", "parts", "face", "clips", "repertoire", "locomotion", "temperament", "states", "tricks", "purr", "emitters", "gear", "grip", "reach", "canopy", "mood"]);
  return {
    "$schema": row["$schema"] === undefined ? undefined : frameworkProductPetsGuardString(row["$schema"], `${at}.$schema`),
    id: parseSlug(row["id"], `${at}.id`),
    name: parseText(row["name"], `${at}.name`),
    thing: parseText(row["thing"], `${at}.thing`),
    grounds: frameworkProductPetsGuardArray(row["grounds"], `${at}.grounds`).map((item, index) => frameworkProductPetsGuardString(item, `${at}.grounds[${index}]`, {"minLength": 1})),
    size: parseSize(row["size"], `${at}.size`),
    palette: parsePalette(row["palette"], `${at}.palette`),
    bones: frameworkProductPetsGuardArray(row["bones"], `${at}.bones`, {"minItems": 1}).map((item, index) => parseBone(item, `${at}.bones[${index}]`)),
    parts: frameworkProductPetsGuardArray(row["parts"], `${at}.parts`).map((item, index) => parsePart(item, `${at}.parts[${index}]`)),
    face: parseFace(row["face"], `${at}.face`),
    clips: frameworkProductPetsGuardArray(row["clips"], `${at}.clips`).map((item, index) => parseClip(item, `${at}.clips[${index}]`)),
    repertoire: parseRepertoire(row["repertoire"], `${at}.repertoire`),
    locomotion: parseLocomotion(row["locomotion"], `${at}.locomotion`),
    temperament: parseTemperament(row["temperament"], `${at}.temperament`),
    states: frameworkProductPetsGuardArray(row["states"], `${at}.states`, {"minItems": 1}).map((item, index) => parseSpeciesState(item, `${at}.states[${index}]`)),
    tricks: frameworkProductPetsGuardArray(row["tricks"], `${at}.tricks`).map((item, index) => parseTrick(item, `${at}.tricks[${index}]`)),
    purr: parsePurr(row["purr"], `${at}.purr`),
    emitters: frameworkProductPetsGuardArray(row["emitters"], `${at}.emitters`).map((item, index) => parseEmitter(item, `${at}.emitters[${index}]`)),
    gear: frameworkProductPetsGuardArray(row["gear"], `${at}.gear`).map((item, index) => parseGear(item, `${at}.gear[${index}]`)),
    grip: frameworkProductPetsGuardNumber(row["grip"], `${at}.grip`, {"exclusiveMinimum": 0}),
    reach: frameworkProductPetsGuardNumber(row["reach"], `${at}.reach`, {"minimum": 0}),
    canopy: row["canopy"] === undefined ? undefined : parseShape(row["canopy"], `${at}.canopy`),
    mood: parseMood(row["mood"], `${at}.mood`),
  };
}

export function parseSpeciesState(value: unknown, at = "$"): SpeciesState {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "name", "tint", "clip", "emitter", "lasts", "then"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    name: parseText(row["name"], `${at}.name`),
    tint: row["tint"] === undefined ? undefined : parseTint(row["tint"], `${at}.tint`),
    clip: row["clip"] === undefined ? undefined : parseSlug(row["clip"], `${at}.clip`),
    emitter: row["emitter"] === undefined ? undefined : parseSlug(row["emitter"], `${at}.emitter`),
    lasts: row["lasts"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["lasts"], `${at}.lasts`, {"exclusiveMinimum": 0}),
    then: row["then"] === undefined ? undefined : parseSlug(row["then"], `${at}.then`),
  };
}

export function parseStage(value: unknown, at = "$"): Stage {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["seed", "tick", "mode", "quiet", "width", "height", "pointer", "pointed", "over", "glances", "surfaces", "keepouts", "walls", "fixtures", "perches", "pitches", "wanted", "actors", "rapports", "met", "draws", "play", "mischief", "stirred", "scrolled", "press", "touched", "shaking", "trail", "coolings", "pledges", "ladders", "lift", "rested", "poofs", "puffs", "claims", "courses", "trips", "origin"]);
  return {
    seed: frameworkProductPetsGuardInteger(row["seed"], `${at}.seed`, {"minimum": 0, "maximum": 4294967295}),
    tick: frameworkProductPetsGuardInteger(row["tick"], `${at}.tick`, {"minimum": 0}),
    mode: parsePetMode(row["mode"], `${at}.mode`),
    quiet: frameworkProductPetsGuardBoolean(row["quiet"], `${at}.quiet`),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
    pointer: (row["pointer"] === null ? null : parsePoint(row["pointer"], `${at}.pointer`)),
    pointed: parseTicks(row["pointed"], `${at}.pointed`),
    over: frameworkProductPetsGuardMember(row["over"], `${at}.over`, ["free", "control"] as const),
    glances: frameworkProductPetsGuardArray(row["glances"], `${at}.glances`).map((item, index) => parsePoint(item, `${at}.glances[${index}]`)),
    surfaces: frameworkProductPetsGuardArray(row["surfaces"], `${at}.surfaces`).map((item, index) => parseSurface(item, `${at}.surfaces[${index}]`)),
    keepouts: frameworkProductPetsGuardArray(row["keepouts"], `${at}.keepouts`).map((item, index) => parseRect(item, `${at}.keepouts[${index}]`)),
    walls: frameworkProductPetsGuardArray(row["walls"], `${at}.walls`).map((item, index) => parseWall(item, `${at}.walls[${index}]`)),
    fixtures: frameworkProductPetsGuardArray(row["fixtures"], `${at}.fixtures`).map((item, index) => parseFixture(item, `${at}.fixtures[${index}]`)),
    perches: frameworkProductPetsGuardArray(row["perches"], `${at}.perches`).map((item, index) => parsePerch(item, `${at}.perches[${index}]`)),
    pitches: frameworkProductPetsGuardArray(row["pitches"], `${at}.pitches`).map((item, index) => parsePitch(item, `${at}.pitches[${index}]`)),
    wanted: frameworkProductPetsGuardArray(row["wanted"], `${at}.wanted`).map((item, index) => parseSlug(item, `${at}.wanted[${index}]`)),
    actors: frameworkProductPetsGuardArray(row["actors"], `${at}.actors`).map((item, index) => parseActor(item, `${at}.actors[${index}]`)),
    rapports: frameworkProductPetsGuardArray(row["rapports"], `${at}.rapports`).map((item, index) => parseRapport(item, `${at}.rapports[${index}]`)),
    met: parseTicks(row["met"], `${at}.met`),
    draws: frameworkProductPetsGuardInteger(row["draws"], `${at}.draws`, {"minimum": 0}),
    play: frameworkProductPetsGuardBoolean(row["play"], `${at}.play`),
    mischief: frameworkProductPetsGuardBoolean(row["mischief"], `${at}.mischief`),
    stirred: parseTicks(row["stirred"], `${at}.stirred`),
    scrolled: parseTicks(row["scrolled"], `${at}.scrolled`),
    press: parsePress(row["press"], `${at}.press`),
    touched: (row["touched"] === null ? null : parseSlug(row["touched"], `${at}.touched`)),
    shaking: parseShaking(row["shaking"], `${at}.shaking`),
    trail: frameworkProductPetsGuardArray(row["trail"], `${at}.trail`).map((item, index) => parsePoint(item, `${at}.trail[${index}]`)),
    coolings: frameworkProductPetsGuardArray(row["coolings"], `${at}.coolings`).map((item, index) => parseCooling(item, `${at}.coolings[${index}]`)),
    pledges: frameworkProductPetsGuardArray(row["pledges"], `${at}.pledges`).map((item, index) => parsePledge(item, `${at}.pledges[${index}]`)),
    ladders: frameworkProductPetsGuardArray(row["ladders"], `${at}.ladders`).map((item, index) => parseLadder(item, `${at}.ladders[${index}]`)),
    lift: (row["lift"] === null ? null : parsePrank(row["lift"], `${at}.lift`)),
    rested: parseTicks(row["rested"], `${at}.rested`),
    poofs: frameworkProductPetsGuardInteger(row["poofs"], `${at}.poofs`, {"minimum": 0}),
    puffs: frameworkProductPetsGuardArray(row["puffs"], `${at}.puffs`).map((item, index) => parsePuff(item, `${at}.puffs[${index}]`)),
    claims: frameworkProductPetsGuardArray(row["claims"], `${at}.claims`).map((item, index) => parseClaim(item, `${at}.claims[${index}]`)),
    courses: frameworkProductPetsGuardArray(row["courses"], `${at}.courses`).map((item, index) => parseCourse(item, `${at}.courses[${index}]`)),
    trips: frameworkProductPetsGuardArray(row["trips"], `${at}.trips`).map((item, index) => parseTrip(item, `${at}.trips[${index}]`)),
    origin: (row["origin"] === null ? null : parsePoint(row["origin"], `${at}.origin`)),
  };
}

export function parseStageEvent(value: unknown, at = "$"): StageEvent {
  return ((input0: unknown) => { try { return parseTicked(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parsePointed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseUnpointed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseGlanced(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseSurveyed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseSummoned(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseTuned(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseHushed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parsePressed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseDragged(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseReleased(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseCancelled(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseReclaimed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseStirred(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseScrolled(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parsePlayed(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parsePermitted(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } return frameworkProductPetsGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseStirred(value: unknown, at = "$"): Stirred {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "stirred"),
  };
}

export function parseStroking(value: unknown, at = "$"): Stroking {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["live", "way", "from", "began", "peak", "reached", "top", "bottom", "topSince", "bottomSince", "count", "first", "second", "rest"]);
  return {
    live: frameworkProductPetsGuardBoolean(row["live"], `${at}.live`),
    way: frameworkProductPetsGuardInteger(row["way"], `${at}.way`, {"minimum": -1, "maximum": 1}),
    from: frameworkProductPetsGuardNumber(row["from"], `${at}.from`),
    began: parseTicks(row["began"], `${at}.began`),
    peak: frameworkProductPetsGuardNumber(row["peak"], `${at}.peak`),
    reached: parseTicks(row["reached"], `${at}.reached`),
    top: frameworkProductPetsGuardNumber(row["top"], `${at}.top`),
    bottom: frameworkProductPetsGuardNumber(row["bottom"], `${at}.bottom`),
    topSince: frameworkProductPetsGuardNumber(row["topSince"], `${at}.topSince`),
    bottomSince: frameworkProductPetsGuardNumber(row["bottomSince"], `${at}.bottomSince`),
    count: frameworkProductPetsGuardInteger(row["count"], `${at}.count`, {"minimum": -1}),
    first: parseTicks(row["first"], `${at}.first`),
    second: parseTicks(row["second"], `${at}.second`),
    rest: parseTicks(row["rest"], `${at}.rest`),
  };
}

export function parseSummoned(value: unknown, at = "$"): Summoned {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "species"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "summoned"),
    species: frameworkProductPetsGuardArray(row["species"], `${at}.species`).map((item, index) => parseSlug(item, `${at}.species[${index}]`)),
  };
}

export function parseSurface(value: unknown, at = "$"): Surface {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "x0", "x1", "y"]);
  return {
    id: frameworkProductPetsGuardString(row["id"], `${at}.id`, {"minLength": 1}),
    x0: frameworkProductPetsGuardNumber(row["x0"], `${at}.x0`),
    x1: frameworkProductPetsGuardNumber(row["x1"], `${at}.x1`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseSurveyed(value: unknown, at = "$"): Surveyed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "width", "height", "surfaces", "keepouts", "walls", "fixtures"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "surveyed"),
    width: frameworkProductPetsGuardNumber(row["width"], `${at}.width`, {"minimum": 0}),
    height: frameworkProductPetsGuardNumber(row["height"], `${at}.height`, {"minimum": 0}),
    surfaces: frameworkProductPetsGuardArray(row["surfaces"], `${at}.surfaces`).map((item, index) => parseSurface(item, `${at}.surfaces[${index}]`)),
    keepouts: frameworkProductPetsGuardArray(row["keepouts"], `${at}.keepouts`).map((item, index) => parseRect(item, `${at}.keepouts[${index}]`)),
    walls: frameworkProductPetsGuardArray(row["walls"], `${at}.walls`).map((item, index) => parseWall(item, `${at}.walls[${index}]`)),
    fixtures: frameworkProductPetsGuardArray(row["fixtures"], `${at}.fixtures`).map((item, index) => parseFixture(item, `${at}.fixtures[${index}]`)),
  };
}

export function parseTemperament(value: unknown, at = "$"): Temperament {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["energy", "sociability", "curiosity"]);
  return {
    energy: frameworkProductPetsGuardNumber(row["energy"], `${at}.energy`, {"minimum": 0, "maximum": 1}),
    sociability: frameworkProductPetsGuardNumber(row["sociability"], `${at}.sociability`, {"minimum": 0, "maximum": 1}),
    curiosity: frameworkProductPetsGuardNumber(row["curiosity"], `${at}.curiosity`, {"minimum": 0, "maximum": 1}),
  };
}

export function parseText(value: unknown, at = "$"): Text {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["en", "de"]);
  return {
    en: frameworkProductPetsGuardString(row["en"], `${at}.en`, {"minLength": 1}),
    de: frameworkProductPetsGuardString(row["de"], `${at}.de`, {"minLength": 1}),
  };
}

export function parseTicked(value: unknown, at = "$"): Ticked {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "ticks"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "ticked"),
    ticks: frameworkProductPetsGuardInteger(row["ticks"], `${at}.ticks`, {"minimum": 0}),
  };
}

export function parseTicks(value: unknown, at = "$"): Ticks {
  return frameworkProductPetsGuardInteger(value, `${at}`);
}

export function parseTier(value: unknown, at = "$"): Tier {
  return frameworkProductPetsGuardMember(value, `${at}`, ["hello", "trick", "purr", "enough"] as const);
}

export function parseTint(value: unknown, at = "$"): Tint {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["body", "accent", "detail"]);
  return {
    body: row["body"] === undefined ? undefined : parseColor(row["body"], `${at}.body`),
    accent: row["accent"] === undefined ? undefined : parseColor(row["accent"], `${at}.accent`),
    detail: row["detail"] === undefined ? undefined : parseColor(row["detail"], `${at}.detail`),
  };
}

export function parseToolFrame(value: unknown, at = "$"): ToolFrame {
  return ((input0: unknown) => { try { return parseChuteTool(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseRopeTool(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseHookTool(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseGunTool(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } try { return parseLadderTool(input0, `${at}`); } catch (error) { if (!(error instanceof frameworkProductPetsGuardRefusal)) throw error; } return frameworkProductPetsGuardReject(`${at}`, "value matches no variant"); })(value);
}

export function parseTrack(value: unknown, at = "$"): Track {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["bone", "channel", "keys"]);
  return {
    bone: parseSlug(row["bone"], `${at}.bone`),
    channel: parseChannel(row["channel"], `${at}.channel`),
    keys: frameworkProductPetsGuardArray(row["keys"], `${at}.keys`, {"minItems": 2}).map((item, index) => parseKey(item, `${at}.keys[${index}]`)),
  };
}

export function parseTrait(value: unknown, at = "$"): Trait {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["species", "state", "held", "mood", "activity", "trick"]);
  return {
    species: row["species"] === undefined ? undefined : parseSlug(row["species"], `${at}.species`),
    state: row["state"] === undefined ? undefined : parseSlug(row["state"], `${at}.state`),
    held: row["held"] === undefined ? undefined : frameworkProductPetsGuardNumber(row["held"], `${at}.held`, {"exclusiveMinimum": 0}),
    mood: row["mood"] === undefined ? undefined : parseMood(row["mood"], `${at}.mood`),
    activity: row["activity"] === undefined ? undefined : parseActivity(row["activity"], `${at}.activity`),
    trick: row["trick"] === undefined ? undefined : parseSlug(row["trick"], `${at}.trick`),
  };
}

export function parseTrick(value: unknown, at = "$"): Trick {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "name", "clip", "cues", "emitter", "from", "to", "mood"]);
  return {
    id: parseSlug(row["id"], `${at}.id`),
    name: parseText(row["name"], `${at}.name`),
    clip: parseSlug(row["clip"], `${at}.clip`),
    cues: frameworkProductPetsGuardArray(row["cues"], `${at}.cues`).map((item, index) => parseCue(item, `${at}.cues[${index}]`)),
    emitter: row["emitter"] === undefined ? undefined : parseSlug(row["emitter"], `${at}.emitter`),
    from: row["from"] === undefined ? undefined : frameworkProductPetsGuardArray(row["from"], `${at}.from`).map((item, index) => parseSlug(item, `${at}.from[${index}]`)),
    to: row["to"] === undefined ? undefined : parseSlug(row["to"], `${at}.to`),
    mood: row["mood"] === undefined ? undefined : parseMood(row["mood"], `${at}.mood`),
  };
}

export function parseTrip(value: unknown, at = "$"): Trip {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["owner", "from", "steps", "pitches", "ladder", "ending", "landing", "grip"]);
  return {
    owner: parseSlug(row["owner"], `${at}.owner`),
    from: parseTicks(row["from"], `${at}.from`),
    steps: frameworkProductPetsGuardArray(row["steps"], `${at}.steps`, {"minItems": 1}).map((item, index) => parseFoothold(item, `${at}.steps[${index}]`)),
    pitches: frameworkProductPetsGuardArray(row["pitches"], `${at}.pitches`).map((item, index) => parsePitch(item, `${at}.pitches[${index}]`)),
    ladder: (row["ladder"] === null ? null : parseSlug(row["ladder"], `${at}.ladder`)),
    ending: frameworkProductPetsGuardMember(row["ending"], `${at}.ending`, ["perch", "wall", "air"] as const),
    landing: frameworkProductPetsGuardString(row["landing"], `${at}.landing`),
    grip: frameworkProductPetsGuardNumber(row["grip"], `${at}.grip`, {"minimum": 0}),
  };
}

export function parseTuned(value: unknown, at = "$"): Tuned {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind", "mode"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "tuned"),
    mode: parsePetMode(row["mode"], `${at}.mode`),
  };
}

export function parseTurns(value: unknown, at = "$"): Turns {
  return frameworkProductPetsGuardNumber(value, `${at}`);
}

export function parseUnpointed(value: unknown, at = "$"): Unpointed {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["kind"]);
  return {
    kind: frameworkProductPetsGuardConstant(row["kind"], `${at}.kind`, "unpointed"),
  };
}

export function parseWall(value: unknown, at = "$"): Wall {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["id", "surface", "side", "x", "y0", "y1"]);
  return {
    id: frameworkProductPetsGuardString(row["id"], `${at}.id`, {"minLength": 1}),
    surface: frameworkProductPetsGuardString(row["surface"], `${at}.surface`, {"minLength": 1}),
    side: ((input0: unknown) => ([1, -1] as readonly unknown[]).includes(input0) ? (input0 as 1 | -1) : frameworkProductPetsGuardReject(`${at}.side`, "value is not one of 1, -1"))(row["side"]),
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y0: frameworkProductPetsGuardNumber(row["y0"], `${at}.y0`),
    y1: frameworkProductPetsGuardNumber(row["y1"], `${at}.y1`),
  };
}

export function parseWarmth(value: unknown, at = "$"): Warmth {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["heat", "since", "until", "tier", "run", "tricks"]);
  return {
    heat: frameworkProductPetsGuardNumber(row["heat"], `${at}.heat`, {"minimum": 0}),
    since: parseTicks(row["since"], `${at}.since`),
    until: parseTicks(row["until"], `${at}.until`),
    tier: parseTier(row["tier"], `${at}.tier`),
    run: frameworkProductPetsGuardInteger(row["run"], `${at}.run`, {"minimum": 0}),
    tricks: frameworkProductPetsGuardInteger(row["tricks"], `${at}.tricks`, {"minimum": 0}),
  };
}

export function parseWaypoint(value: unknown, at = "$"): Waypoint {
  const row = frameworkProductPetsGuardClosed(frameworkProductPetsGuardObject(value, at), at, ["x", "y", "vx", "vy", "tilt", "canopy"]);
  return {
    x: frameworkProductPetsGuardNumber(row["x"], `${at}.x`),
    y: frameworkProductPetsGuardNumber(row["y"], `${at}.y`),
    vx: frameworkProductPetsGuardNumber(row["vx"], `${at}.vx`),
    vy: frameworkProductPetsGuardNumber(row["vy"], `${at}.vy`),
    tilt: parseTurns(row["tilt"], `${at}.tilt`),
    canopy: (row["canopy"] === null ? null : parseCanopy(row["canopy"], `${at}.canopy`)),
  };
}
