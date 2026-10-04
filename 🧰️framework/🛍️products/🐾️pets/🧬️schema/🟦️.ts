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

/** 🎮️ What a learner can ask of a pet without a pointer: a hello, a trick, petting it or tossing it. */
export const DEEDS = ["hello", "trick", "pet", "toss"] as const;

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
