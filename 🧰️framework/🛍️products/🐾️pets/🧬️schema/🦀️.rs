//! 🐾️ Hand-written Rust twin of the normative pets contract `🧬️schema/🔣️.json` (JSON Schema draft-07): menageries of rigged species, their bonds and casts, the stage they act on, its events and the frames it projects.
//!
//! One type per `$defs` entry, same names; fields are `camelCase` on the wire, tagged unions are
//! internally tagged on `kind`, optional fields are omitted when absent, members that may be `null`
//! must still be present, tuples of a fixed length are arrays (`Ease`, the two ends of a `Bond`) and
//! unknown fields are refused like the schema's `additionalProperties: false`. Constraints serde
//! cannot express (slug and colour patterns, lengths, ranges, the `schema` identifiers, references)
//! are checked by [`crate::validation`].
//!
//! Numbers: every `number` is an `f64`; [`Ticks`] are `i64` (whole ticks, exact as `f64` up to 2⁵³);
//! a seed and a draw counter are `u32`, because they are words of a randomness key and the counters
//! wrap at 32 bits like the TypeScript twin's `>>> 0`. Closed sets of numbers are enums that carry
//! their number on the wire ([`Facing`], [`Rate`]); the count of a `ticked` event is unsigned, and the
//! clock of a stage or a frame is refused when it is negative.
//!
//! @see ../🧬️schema/🔣️.json — the normative contract
//! @see ../🧬️schema/🟦️.ts — the TypeScript twin
//! @see <https://json-schema.org/draft-07/json-schema-validation>

use serde::{Deserialize, Deserializer, Serialize};

//#region 🔖️Scalars
/// 🏷️ A kebab-case identifier, unique within its scope (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 chars).
pub type Slug = String;

/// 🌍️ A human-readable text in every supported language; there is no default language.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub en: String,
    pub de: String,
}

/// 🗣️ The languages every [`Text`] carries, English first.
pub const LANGUAGES: [&str; 2] = ["en", "de"];

/// 📐️ An authored angle in degrees, clockwise on screen (the y axis points down).
pub type Degrees = f64;

/// 🔄️ A simulated angle in turns (1 = a full clockwise revolution).
pub type Turns = f64;

/// ⏱️ A whole number of simulation ticks; the stage advances [`TICKS_PER_SECOND`] ticks per second.
pub type Ticks = i64;

/// 🥁️ The fixed simulation rate: a power of two, so one tick is an exact binary fraction of a second.
pub const TICKS_PER_SECOND: Ticks = 64;

/// 🎨️ An sRGB colour as `#rrggbb` in lowercase hexadecimal digits.
pub type Color = String;

/// 🖌️ What a part is painted with: a colour of the species palette, the theme's ink or paper, or nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Paint {
    Body,
    Accent,
    Detail,
    Ink,
    Paper,
    None,
}

/// 🖍️ Every [`Paint`], in the order of the contract.
pub const PAINTS: [Paint; 6] = [Paint::Body, Paint::Accent, Paint::Detail, Paint::Ink, Paint::Paper, Paint::None];

/// 🎛️ What a track animates on a bone: offsets in pixels, an offset in degrees or scale factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    X,
    Y,
    Rotation,
    ScaleX,
    ScaleY,
}

/// 🎚️ Every [`Channel`], in the order of the contract.
pub const CHANNELS: [Channel; 5] = [Channel::X, Channel::Y, Channel::Rotation, Channel::ScaleX, Channel::ScaleY];

/// 🦿️ How a species moves along a perch: on legs, in hops or hovering above it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Gait {
    Walk,
    Hop,
    Float,
}

/// 🚶️ Every [`Gait`], in the order of the contract.
pub const GAITS: [Gait; 3] = [Gait::Walk, Gait::Hop, Gait::Float];

/// 🎬️ Everything an actor can be doing; exactly one activity at a time. The first eleven are the life on a perch; then: hanging from the learner's hand, tumbling through the air after a throw, gliding under a parachute, aiming a grappling line and reeling it in, climbing a wall or a ladder, mantling over an edge, sliding down, carrying a ladder, performing a trick, purring, being dizzy, shrugging, scooting aside for another pet and pushing a fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Activity {
    Idle,
    Fidget,
    Walk,
    Hop,
    Fall,
    Land,
    Sleep,
    Greet,
    Cuddle,
    Squabble,
    Sulk,
    Hang,
    Tumble,
    Glide,
    Aim,
    Reel,
    Climb,
    Mantle,
    Slide,
    Carry,
    Trick,
    Purr,
    Dizzy,
    Shrug,
    Scoot,
    Push,
}

impl Activity {
    /// 💬️ The wire string, e.g. `squabble`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Fidget => "fidget",
            Self::Walk => "walk",
            Self::Hop => "hop",
            Self::Fall => "fall",
            Self::Land => "land",
            Self::Sleep => "sleep",
            Self::Greet => "greet",
            Self::Cuddle => "cuddle",
            Self::Squabble => "squabble",
            Self::Sulk => "sulk",
            Self::Hang => "hang",
            Self::Tumble => "tumble",
            Self::Glide => "glide",
            Self::Aim => "aim",
            Self::Reel => "reel",
            Self::Climb => "climb",
            Self::Mantle => "mantle",
            Self::Slide => "slide",
            Self::Carry => "carry",
            Self::Trick => "trick",
            Self::Purr => "purr",
            Self::Dizzy => "dizzy",
            Self::Shrug => "shrug",
            Self::Scoot => "scoot",
            Self::Push => "push",
        }
    }
}

/// 🎭️ Every [`Activity`], in the order of the contract; weights and repertoires follow this order, and new activities are appended, so the index of an activity never changes.
pub const ACTIVITIES: [Activity; 26] = [
    Activity::Idle,
    Activity::Fidget,
    Activity::Walk,
    Activity::Hop,
    Activity::Fall,
    Activity::Land,
    Activity::Sleep,
    Activity::Greet,
    Activity::Cuddle,
    Activity::Squabble,
    Activity::Sulk,
    Activity::Hang,
    Activity::Tumble,
    Activity::Glide,
    Activity::Aim,
    Activity::Reel,
    Activity::Climb,
    Activity::Mantle,
    Activity::Slide,
    Activity::Carry,
    Activity::Trick,
    Activity::Purr,
    Activity::Dizzy,
    Activity::Shrug,
    Activity::Scoot,
    Activity::Push,
];

/// 🔆️ How lively a stage is: motionless, slightly active or busy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PetMode {
    Still,
    Calm,
    Lively,
}

/// 🌡️ Every [`PetMode`], in the order of the contract.
pub const PET_MODES: [PetMode; 3] = [PetMode::Still, PetMode::Calm, PetMode::Lively];

/// 😊️ What an actor can feel, one mood at a time with an intensity in [0, 1]. A stronger cause replaces a weaker mood: scared, then grumpy, then sad, then proud, happy and playful alike, then curious, then content; sleepy follows the energy need.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mood {
    #[default]
    Content,
    Happy,
    Playful,
    Curious,
    Proud,
    Sleepy,
    Grumpy,
    Sad,
    Scared,
}

/// 💗️ Every [`Mood`], in the order of the contract.
pub const MOODS: [Mood; 9] = [Mood::Content, Mood::Happy, Mood::Playful, Mood::Curious, Mood::Proud, Mood::Sleepy, Mood::Grumpy, Mood::Sad, Mood::Scared];

/// 🦶️ What carries an actor, one footing at a time: a perch, the air (hopping, falling, thrown), its open parachute, the learner's hand, a rope, a ladder, a wall or the head of another actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Footing {
    Perch,
    Air,
    Chute,
    Hand,
    Rope,
    Ladder,
    Wall,
    Head,
}

/// 👣️ Every [`Footing`], in the order of the contract.
pub const FOOTINGS: [Footing; 8] = [Footing::Perch, Footing::Air, Footing::Chute, Footing::Hand, Footing::Rope, Footing::Ladder, Footing::Wall, Footing::Head];

/// 🧗️ What a species may own to get around: it climbs walls, carries and raises a ladder, shoots a grappling line or opens a parachute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Gear {
    Climb,
    Ladder,
    Grapple,
    Parachute,
}

/// 🧰️ Every [`Gear`], in the order of the contract.
pub const GEARS: [Gear; 4] = [Gear::Climb, Gear::Ladder, Gear::Grapple, Gear::Parachute];

/// 👋️ What sets a trick off: a click, the pointer circling the pet clockwise or counter-clockwise, stroking it, shaking it while it is held, its own whim, or showing off to another pet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Cue {
    Click,
    Circle,
    Countercircle,
    Stroke,
    Shake,
    Whim,
    Show,
}

/// 🔔️ Every [`Cue`], in the order of the contract.
pub const CUES: [Cue; 7] = [Cue::Click, Cue::Circle, Cue::Countercircle, Cue::Stroke, Cue::Shake, Cue::Whim, Cue::Show];

/// 🍃️ How the particles of an emitter move: down with gravity, up with buoyancy, outwards from the emitter, round it, or wandering slowly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Drift {
    Fall,
    Rise,
    Burst,
    Orbit,
    Drift,
}

/// 🌬️ Every [`Drift`], in the order of the contract.
pub const DRIFTS: [Drift; 5] = [Drift::Fall, Drift::Rise, Drift::Burst, Drift::Orbit, Drift::Drift];

/// 🕹️ What a learner can ask of a pet without a pointer: a hello, a trick, petting it or tossing it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Deed {
    Hello,
    Trick,
    Pet,
    Toss,
}

/// 🎮️ Every [`Deed`], in the order of the contract.
pub const DEEDS: [Deed; 4] = [Deed::Hello, Deed::Trick, Deed::Pet, Deed::Toss];

/// 🖲️ What presses a pet: a mouse, a pen or a finger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Pointer {
    Mouse,
    Pen,
    Touch,
}

/// ✍️ Every [`Pointer`], in the order of the contract.
pub const POINTERS: [Pointer; 3] = [Pointer::Mouse, Pointer::Pen, Pointer::Touch];

/// 🚦️ Where the press of a stage stands: nothing is down (`idle`), a press waits for what it becomes (`armed`), it rests on the pet (`holding`), or the pet is picked up (`lifted`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PressPhase {
    Idle,
    Armed,
    Holding,
    Lifted,
}

/// 🛑️ Every [`PressPhase`], in the order of the contract.
pub const PRESS_PHASES: [PressPhase; 4] = [PressPhase::Idle, PressPhase::Armed, PressPhase::Holding, PressPhase::Lifted];

/// 🎖️ How a pet answers attention, from cold to saturated: a hello, a trick, a purr, or it has had enough.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    Hello,
    Trick,
    Purr,
    Enough,
}

/// 🏅️ Every [`Tier`], in the order of the contract.
pub const TIERS: [Tier; 4] = [Tier::Hello, Tier::Trick, Tier::Purr, Tier::Enough];

/// 🧾️ The schema identifier every menagerie document carries.
pub const MENAGERIE_SCHEMA: &str = "semio.pets.menagerie/v1";

/// 📇️ The schema identifier every ensemble document carries.
pub const ENSEMBLE_SCHEMA: &str = "semio.pets.ensemble/v1";

/// 🕳️ A member that must be present and may be `null`; serde's own `Option` would also accept its absence.
fn nullable<'de, T: Deserialize<'de>, D: Deserializer<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
}
//#endregion 🔖️Scalars

//#region 🔖️Rig
/// 🦴️ One bone of a skeleton: its rest offset and rotation relative to its parent; the root has no parent. Bones are listed parents first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bone {
    pub id: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Slug>,
    pub x: f64,
    pub y: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Degrees>,
}

/// ➰️ A free-form outline as SVG path data in the bone's own coordinates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathShape {
    pub d: String,
}

/// ⭕️ An ellipse around a centre.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EllipseShape {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

/// ▭️ An axis-aligned rectangle with optionally rounded corners.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RectShape {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// ➖️ A straight stroke between two points.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineShape {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

/// 🔷️ The geometry of a part, in the coordinates of the bone that carries it, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Shape {
    Path(PathShape),
    Ellipse(EllipseShape),
    Rect(RectShape),
    Line(LineShape),
}

/// 🧩️ One drawn piece of a species, rigidly attached to a bone; parts are drawn in list order, back to front.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Part {
    pub id: Slug,
    pub bone: Slug,
    pub shape: Shape,
    pub fill: Paint,
    pub stroke: Paint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

/// 👁️ An eye on a bone: a round white of `radius` whose pupil of radius `pupil` follows what the actor looks at.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Eye {
    pub id: Slug,
    pub bone: Slug,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub pupil: f64,
}

/// 👄️ A mouth on a bone: a curve of `width` that bends with the actor's mood.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mouth {
    pub bone: Slug,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

/// 🙂️ The face of a species; it is drawn above the part named by `above` (above every part when absent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Face {
    pub eyes: Vec<Eye>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mouth: Option<Mouth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub above: Option<Slug>,
}
//#endregion 🔖️Rig

//#region 🔖️Animation
/// 🪜️ The control points `[x1, y1, x2, y2]` of a cubic Bézier easing from (0, 0) to (1, 1), as in CSS `cubic-bezier()`.
pub type Ease = [f64; 4];

/// 🔑️ A keyed value at phase `at` (0…1) of a clip; `ease` shapes the way to the next key (linear when absent).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Key {
    pub at: f64,
    pub value: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ease: Option<Ease>,
}

/// 🛤️ The keys of one channel of one bone, in ascending phase, starting at 0 and ending at 1.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    pub bone: Slug,
    pub channel: Channel,
    pub keys: Vec<Key>,
}

/// 🎞️ A keyframed motion of `seconds` length, played once or looped; values are relative to the rest pose.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    pub id: Slug,
    pub seconds: f64,
    #[serde(rename = "loop")]
    pub looping: bool,
    pub tracks: Vec<Track>,
}

/// 🗃️ The clips a species plays per activity; one of several is drawn at random, none leaves the body at rest.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repertoire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fidget: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub walk: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hop: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fall: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub land: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sleep: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub greet: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cuddle: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squabble: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sulk: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hang: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tumble: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glide: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aim: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reel: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub climb: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mantle: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slide: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carry: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trick: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purr: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dizzy: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shrug: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoot: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<Vec<Slug>>,
}

impl Repertoire {
    /// 🎼️ The clip ids listed for an activity, `None` when the repertoire does not mention it (the TypeScript twin's `repertoire[activity]`).
    pub fn clips(&self, activity: Activity) -> Option<&[Slug]> {
        let listed = match activity {
            Activity::Idle => &self.idle,
            Activity::Fidget => &self.fidget,
            Activity::Walk => &self.walk,
            Activity::Hop => &self.hop,
            Activity::Fall => &self.fall,
            Activity::Land => &self.land,
            Activity::Sleep => &self.sleep,
            Activity::Greet => &self.greet,
            Activity::Cuddle => &self.cuddle,
            Activity::Squabble => &self.squabble,
            Activity::Sulk => &self.sulk,
            Activity::Hang => &self.hang,
            Activity::Tumble => &self.tumble,
            Activity::Glide => &self.glide,
            Activity::Aim => &self.aim,
            Activity::Reel => &self.reel,
            Activity::Climb => &self.climb,
            Activity::Mantle => &self.mantle,
            Activity::Slide => &self.slide,
            Activity::Carry => &self.carry,
            Activity::Trick => &self.trick,
            Activity::Purr => &self.purr,
            Activity::Dizzy => &self.dizzy,
            Activity::Shrug => &self.shrug,
            Activity::Scoot => &self.scoot,
            Activity::Push => &self.push,
        };
        listed.as_deref()
    }
}
//#endregion 🔖️Animation

//#region 🔖️Menagerie
/// 📦️ The box a species fills at rest: its feet stand on the origin, the body rises `height` above it and spans `width` around it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// 🌈️ The three colours of a species; ink and paper come from the theme.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Palette {
    pub body: Color,
    pub accent: Color,
    pub detail: Color,
}

/// 🏃️ How fast a species travels in pixels per second, with which gait and, when floating, how high above its perch.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Locomotion {
    pub gait: Gait,
    pub speed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<f64>,
}

/// 🧠️ The character of a species, each trait in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Temperament {
    pub energy: f64,
    pub sociability: f64,
    pub curiosity: f64,
}

/// 💡️ The colours a state paints over the palette; a colour it leaves out stays as authored.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<Color>,
}

/// ✨️ A source of particles on a bone, `x` and `y` away from it in the bone's coordinates: every particle is drawn as `shape` around its own origin and painted like a part; `motion` moves it, at most `count` (1…32) are alive at once, each lives `life` seconds and leaves at `speed` pixels per second in directions that scatter over `spread` of a full turn (0…1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Emitter {
    pub id: Slug,
    pub bone: Slug,
    pub x: f64,
    pub y: f64,
    pub shape: Shape,
    pub fill: Paint,
    pub stroke: Paint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    pub motion: Drift,
    pub count: u32,
    pub life: f64,
    pub speed: f64,
    pub spread: f64,
}

/// 🔦️ A lasting condition of a species, such as shining strongly: it may tint the palette, loop the clip `clip` on top of the idle loop and run the emitter `emitter`; `lasts` seconds after it began it gives way to the state `then`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpeciesState {
    pub id: Slug,
    pub name: Text,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tint: Option<Tint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emitter: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lasts: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then: Option<Slug>,
}

/// 🪄️ Something a species can perform: the clip `clip` played once, set off by any of `cues`, optionally with the particles of `emitter`; it is on offer in the states `from` (in every state when absent), leaves the species in the state `to` and in the mood `mood`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trick {
    pub id: Slug,
    pub name: Text,
    pub clip: Slug,
    pub cues: Vec<Cue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emitter: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<Slug>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mood: Option<Mood>,
}

/// 😻️ How a species shows that it is content while it is petted: a looping clip and optionally the particles of an emitter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Purr {
    pub clip: Slug,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emitter: Option<Slug>,
}

/// 🧬️ A kind of pet: its accessible name, what it stands for, its rig, face, motions and character; its states (at least one, the first is the resting state), tricks, purr and emitters; the gear it owns, how high above its feet it is gripped at the scruff (`grip`), how far its encounter poses lean out (`reach`), the shape drawn as its parachute (`canopy`, a plain one when absent) and its resting mood.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Species {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub id: Slug,
    pub name: Text,
    pub thing: Text,
    pub grounds: Vec<String>,
    pub size: Size,
    pub palette: Palette,
    pub bones: Vec<Bone>,
    pub parts: Vec<Part>,
    pub face: Face,
    pub clips: Vec<Clip>,
    pub repertoire: Repertoire,
    pub locomotion: Locomotion,
    pub temperament: Temperament,
    pub states: Vec<SpeciesState>,
    pub tricks: Vec<Trick>,
    pub purr: Purr,
    pub emitters: Vec<Emitter>,
    pub gear: Vec<Gear>,
    pub grip: f64,
    pub reach: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canopy: Option<Shape>,
    pub mood: Mood,
}

/// 🤝️ How two species feel about each other, from −1 (they squabble) to 1 (they adore each other); unlisted pairs are neutral.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bond {
    pub between: [Slug; 2],
    pub affinity: f64,
}

/// 🎟️ The species of one scene: the core is on stage whenever there is room, the rotation takes turns.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cast {
    pub scene: Slug,
    pub core: Vec<Slug>,
    pub rotation: Vec<Slug>,
}

/// 🔎️ What one side of a reaction must be: an actor of a species (of any species when absent), optionally in a state it has held for at least `held` seconds, in a mood, doing an activity or performing the trick `trick`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trait {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub species: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub held: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mood: Option<Mood>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trick: Option<Slug>,
}

/// 🎯️ The side of a reaction an effect acts on: the one its `when` describes or the one its `near` describes (the contract's `enum: ["when", "near"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Party {
    When,
    Near,
}

/// 🫂️ Every [`Party`], in the order of the contract.
pub const PARTIES: [Party; 2] = [Party::When, Party::Near];

/// 💑️ One of the three activities two actors meet in; no other activity decodes (the contract's `enum: ["greet", "cuddle", "squabble"]`).
fn meeting<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Activity, D::Error> {
    let activity = Activity::deserialize(deserializer)?;
    if !matches!(activity, Activity::Greet | Activity::Cuddle | Activity::Squabble) {
        return Err(serde::de::Error::custom(format!("an encounter is greet, cuddle or squabble, not {}", activity.as_str())));
    }
    Ok(activity)
}

/// 🤼️ The encounter an effect asks for: a [`meeting`] when present.
fn encounter<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Activity>, D::Error> {
    meeting(deserializer).map(Some)
}

/// 🛼️ What an effect sets a pet off on: one of the four activities it starts by itself; no other activity decodes (the contract's `enum: ["fidget", "walk", "hop", "sleep"]`).
fn pursuit<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Activity>, D::Error> {
    let activity = Activity::deserialize(deserializer)?;
    if !matches!(activity, Activity::Fidget | Activity::Walk | Activity::Hop | Activity::Sleep) {
        return Err(serde::de::Error::custom(format!("an effect sets a pet off on fidget, walk, hop or sleep, not {}", activity.as_str())));
    }
    Ok(Some(activity))
}

/// 💥️ What a reaction does to one of its two sides: it puts it into a state, gives it a mood of intensity `amount` (0…1), shifts the rapport of the two by `rapport` (−1…1), makes their next encounter the given one, has it perform a trick or sets it off on something it could start by itself (`activity`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    pub on: Party,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Slug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mood: Option<Mood>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rapport: Option<f64>,
    #[serde(default, deserialize_with = "encounter", skip_serializing_if = "Option::is_none")]
    pub encounter: Option<Activity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trick: Option<Slug>,
    #[serde(default, deserialize_with = "pursuit", skip_serializing_if = "Option::is_none")]
    pub activity: Option<Activity>,
}

/// 🧿️ Where the first side of a reaction is seen from the second (the contract's `enum: ["above", "below", "beside", "any"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Placement {
    Above,
    Below,
    Beside,
    Any,
}

/// 🗼️ Every [`Placement`], in the order of the contract.
pub const PLACEMENTS: [Placement; 4] = [Placement::Above, Placement::Below, Placement::Beside, Placement::Any];

/// ⚗️ Chemistry between two species: when an actor matching `when` is within `within` pixels of one matching `near` — `place` it is seen from the second (`where` on the wire; anywhere when absent), while no third actor matching `unless` is within `within` pixels of the second and while the affinity of the two lies within `affinity` (`[low, high]`, −1…1) — the effects of `then` happen, at most once in `every` seconds and with the probability `chance` (always when absent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reaction {
    pub id: Slug,
    pub when: Trait,
    pub near: Trait,
    pub within: f64,
    #[serde(rename = "where", default, skip_serializing_if = "Option::is_none")]
    pub place: Option<Placement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unless: Option<Trait>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Vec<f64>>,
    pub every: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chance: Option<f64>,
    pub then: Vec<Effect>,
}

/// 🎪️ A menagerie document: every species of a domain, their bonds, the casts of its scenes and the chemistry between its species.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Menagerie {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub schema: String,
    pub id: Slug,
    pub title: Text,
    pub species: Vec<Species>,
    pub bonds: Vec<Bond>,
    pub casts: Vec<Cast>,
    pub chemistry: Vec<Reaction>,
}

/// 🗂️ The authoring form of a menagerie: every species lives in a document of its own, referenced by a path relative to the ensemble.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ensemble {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    pub schema: String,
    pub id: Slug,
    pub title: Text,
    pub species: Vec<String>,
    pub bonds: Vec<Bond>,
    pub casts: Vec<Cast>,
    pub chemistry: Vec<Reaction>,
}
//#endregion 🔖️Menagerie

//#region 🔖️Terrain
/// 📍️ A position in viewport pixels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// ⬛️ An axis-aligned box in viewport pixels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🪵️ The top edge of something actors may stand on, from `x0` to `x1` at height `y`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surface {
    pub id: String,
    pub x0: f64,
    pub x1: f64,
    pub y: f64,
}

/// 🪺️ A stretch of a surface that is free to stand on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Perch {
    pub surface: String,
    pub x0: f64,
    pub x1: f64,
    pub y: f64,
}

/// 🧱️ A side edge of the element of a surface, which pets may climb: the surface it belongs to, the way it faces (`-1` the left edge, `1` the right edge), where it stands (`x`) and its stretch from `y0` at the top down to `y1`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Wall {
    pub id: String,
    pub surface: String,
    pub side: Facing,
    pub x: f64,
    pub y0: f64,
    pub y1: f64,
}

/// 🪨️ A stretch of a wall that is free to climb: the wall and the surface it belongs to, the way the wall faces (`-1` the left edge, its air on the left; `1` the right edge), where it stands (`x`) and its extent from `y0` down to `y1`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pitch {
    pub wall: String,
    pub surface: String,
    pub side: Facing,
    pub x: f64,
    pub y0: f64,
    pub y1: f64,
}

/// 📌️ An element of the page a pet may play with: its key among the grounds of the menagerie (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`) and its box.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixture {
    pub id: String,
    pub key: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
//#endregion 🔖️Terrain

//#region 🔖️Stage
/// 🕰️ A tick of the stage clock on the wire: a whole number that is not negative (the contract's `minimum: 0`); in memory it is [`Ticks`], because it is subtracted from points in time that may lie ahead of it.
fn elapsed<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Ticks, D::Error> {
    let tick = Ticks::deserialize(deserializer)?;
    if tick < 0 {
        return Err(serde::de::Error::custom(format!("a tick of the stage clock is not negative, got {tick}")));
    }
    Ok(tick)
}

/// ⏭️ Time passed: the stage advances by whole ticks, a count that is never negative.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ticked {
    pub ticks: u64,
}

/// 🖱️ The pointer moved to a position, over free space or over a control (an interactive element of the page).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pointed {
    pub x: f64,
    pub y: f64,
    pub over: Over,
}

/// 🫥️ The pointer left the stage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unpointed {}

/// 👀️ Other things worth a look right now, such as the cursors of other people.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Glanced {
    pub points: Vec<Point>,
}

/// 🗺️ The stage was measured anew: its size, every surface, everything actors must not cover, the walls they may climb and the fixtures they may play with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surveyed {
    pub width: f64,
    pub height: f64,
    pub surfaces: Vec<Surface>,
    pub keepouts: Vec<Rect>,
    pub walls: Vec<Wall>,
    pub fixtures: Vec<Fixture>,
}

/// 📣️ The species that belong on stage from now on, most wanted first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summoned {
    pub species: Vec<Slug>,
}

/// 📻️ The liveliness changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tuned {
    pub mode: PetMode,
}

/// 🤫️ A time of concentration began or ended; hushed actors rest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hushed {
    pub quiet: bool,
}

/// 🫣️ What lies under the pointer: free space or a control, an interactive element of the page (the contract's `enum: ["free", "control"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Over {
    Free,
    Control,
}

/// 🙈️ Every [`Over`], in the order of the contract.
pub const OVERS: [Over; 2] = [Over::Free, Over::Control];

/// 🫳️ The learner pressed where a pet may be: the primary button or contact went down at a position, with a mouse, a pen or a finger, on nothing interactive.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pressed {
    pub x: f64,
    pub y: f64,
    pub pointer: Pointer,
}

/// 🧲️ The pointer of the open press moved to a position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dragged {
    pub x: f64,
    pub y: f64,
}

/// 🎈️ The open press ended at a position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Released {
    pub x: f64,
    pub y: f64,
}

/// ✖️ The open press was called off: Escape, a cancelled pointer, lost capture, a blurred or hidden page, a pause.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cancelled {}

/// 🫱️ The learner took a fixture back — pointed at it, pressed, focused, typed into or dragged it —: whatever a pet does with it ends at once.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reclaimed {
    pub fixture: String,
}

/// 🥄️ The learner did something on the page: any input.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stirred {}

/// 📜️ The page moved under the pointer: it was scrolled.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scrolled {}

/// 🎲️ The learner asked a pet for a deed without a pointer (the keyboard and panel equivalents of the hand).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Played {
    pub species: Slug,
    pub deed: Deed,
}

/// ✅️ What the learner allows: pets answer clicks and can be picked up (`play`), pets may play with the page (`mischief`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Permitted {
    pub play: bool,
    pub mischief: bool,
}

/// 📨️ Everything that can happen to a stage, tagged by `kind`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StageEvent {
    Ticked(Ticked),
    Pointed(Pointed),
    Unpointed(Unpointed),
    Glanced(Glanced),
    Surveyed(Surveyed),
    Summoned(Summoned),
    Tuned(Tuned),
    Hushed(Hushed),
    Pressed(Pressed),
    Dragged(Dragged),
    Released(Released),
    Cancelled(Cancelled),
    Reclaimed(Reclaimed),
    Stirred(Stirred),
    Scrolled(Scrolled),
    Played(Played),
    Permitted(Permitted),
}

/// 🔭️ Where the pupils rest between −1 and 1 on both axes, with the velocity of the spring that moves them.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gaze {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

/// 🔋️ The drives of an actor, each in [0, 1].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Needs {
    pub energy: f64,
    pub sociability: f64,
    pub curiosity: f64,
}

/// 🧭️ The way an actor faces: right is `1` on the wire and left is `-1`; no other number decodes (the contract's `enum: [1, -1]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "i8", into = "i8")]
pub enum Facing {
    Right,
    Left,
}

/// 🪧️ Every [`Facing`], in the order of the contract.
pub const FACINGS: [Facing; 2] = [Facing::Right, Facing::Left];

impl Facing {
    /// ➕️ The facing as a factor of an x offset: 1 for right, −1 for left.
    pub fn sign(self) -> f64 {
        match self {
            Self::Right => 1.0,
            Self::Left => -1.0,
        }
    }

    /// 🔃️ The other way.
    pub fn reversed(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

impl From<Facing> for i8 {
    /// ↗️ The wire number of a facing.
    fn from(facing: Facing) -> i8 {
        match facing {
            Facing::Right => 1,
            Facing::Left => -1,
        }
    }
}

impl TryFrom<i8> for Facing {
    type Error = String;

    /// 🚧️ The facing of a wire number; anything but 1 and −1 is refused.
    fn try_from(number: i8) -> Result<Self, String> {
        match number {
            1 => Ok(Self::Right),
            -1 => Ok(Self::Left),
            other => Err(format!("a facing is 1 or -1, not {other}")),
        }
    }
}

/// 🤏️ The press of a stage: its phase, the point it went down at (the grip, kept until the press ends), the tick it went down and the slop of its pointer in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Press {
    pub phase: PressPhase,
    pub x: f64,
    pub y: f64,
    pub since: Ticks,
    pub slop: f64,
}

/// 🔥️ How warm an actor is from attention: the `heat` as it was at tick `since` (it leaks from there), the tick `until` which it has had enough (deaf before, forgiving from then on), the `tier` of its latest answer, how many answers in a row had that tier (`run`, 1 for the first) and how many of all its answers were tricks (`tricks`; the latest trick is number `tricks − 1` in the species' order).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Warmth {
    pub heat: f64,
    pub since: Ticks,
    pub until: Ticks,
    pub tier: Tier,
    pub run: i64,
    pub tricks: i64,
}

/// 🌀️ The circle the pointer may be drawing round one actor: whether it is followed (`live`) and when it was last in the band (`inside`); the signs of its offset from the body's centre as the Schmitt trigger holds them (`sx`, `sy`; 0 = not known yet) and the previous offset (`px`, `py`); the running lap — its direction (`turn`: 1 clockwise on screen, −1 counter-clockwise, 0 before its first quarter turn), its net quarter turns in that direction, all its quarter turns (`steps`) and those `against` the direction, the ticks of its `first` and `last` quarter turn, the squared distance from the centre at its first quarter turn (`open`) and the least and the greatest one since (`near`, `far`) — and the tick before which nothing is recognised (`rest`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Circling {
    pub live: bool,
    pub inside: Ticks,
    pub sx: i64,
    pub sy: i64,
    pub px: f64,
    pub py: f64,
    pub turn: i64,
    pub quarters: i64,
    pub steps: i64,
    pub against: i64,
    pub first: Ticks,
    pub last: Ticks,
    pub open: f64,
    pub near: f64,
    pub far: f64,
    pub rest: Ticks,
}

/// 🖐️ The petting the pointer may be giving one actor: whether it is followed (`live`); the direction of the running stroke (`way`: 1 rightwards, −1 leftwards, 0 before it has one), the offset from the body's centre and the tick at which it began (`from`, `began`) and its running extreme (`peak`, `reached`); the highest and the lowest vertical offset of the running stroke up to its extreme (`top`, `bottom`) and since its extreme (`topSince`, `bottomSince`); how many strokes in a row counted (`count`; −1 while the run that led into the zone is still under way) and the ticks at which the oldest two of them began (`first`, `second`); and the tick before which nothing is recognised (`rest`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Stroking {
    pub live: bool,
    pub way: i64,
    pub from: f64,
    pub began: Ticks,
    pub peak: f64,
    pub reached: Ticks,
    pub top: f64,
    pub bottom: f64,
    pub top_since: f64,
    pub bottom_since: f64,
    pub count: i64,
    pub first: Ticks,
    pub second: Ticks,
    pub rest: Ticks,
}

/// 🫨️ The shake the hand may be giving the pet it holds: whether the grip is followed (`live`); the point and the tick of its latest reversal (`ax`, `ay`, `at`) and the farthest point from there on the running swing (`fx`, `fy`, `reached`); how many swings in a row counted (`count`) and the ticks of the three latest counted reversals, oldest first (`mark1`, `mark2`, `mark3`); and the tick before which nothing is recognised (`rest`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shaking {
    pub live: bool,
    pub ax: f64,
    pub ay: f64,
    pub at: Ticks,
    pub fx: f64,
    pub fy: f64,
    pub reached: Ticks,
    pub count: i64,
    pub mark1: Ticks,
    pub mark2: Ticks,
    pub mark3: Ticks,
    pub rest: Ticks,
}

/// 🪶️ The two gestures a free pointer can make round or over one actor: circling and stroking.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hover {
    pub circling: Circling,
    pub stroking: Stroking,
}

/// ✊️ The point a held pet is gripped at: where it is and how fast it moves, in pixels and pixels per second.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grip {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

/// 🐒️ A pet in the learner's hand: its grip, where its feet are (the bob of its pendulum), and where they were one tick ago.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hang {
    pub grip: Grip,
    pub bob: Point,
    pub previous: Point,
}

/// ☂️ A pet under its open parachute: where the canopy holds the cords and how fast it moves (`vy` without the flare), where the feet are, and where they were one tick ago.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Canopy {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub bob: Point,
    pub previous: Point,
}

/// 🪂️ The parachute of an actor while it is out: the tick it began to open, how far it is open (0 packed … 1 open; it may overshoot while it fills) and how fast that changes per second, and the motion of its canopy.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parachute {
    pub since: Ticks,
    pub open: f64,
    pub opening: f64,
    pub canopy: Canopy,
}

/// 🎣️ How a rope is reeled in: straight up a steep line (`zip`) or swinging on a slanted one (the contract's `enum: ["zip", "swing"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reeling {
    Zip,
    Swing,
}

/// 🔁️ Every [`Reeling`], in the order of the contract.
pub const REELINGS: [Reeling; 2] = [Reeling::Zip, Reeling::Swing];

/// 🏹️ A shot of the grappling gun: the surface whose edge the hook bites, the way the actor faces, the muzzle the rope leaves from, the hook point, the length of the taut rope and whether it is reeled in straight (`zip`) or swung.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    pub surface: String,
    pub facing: Facing,
    pub muzzle: Point,
    pub hook: Point,
    pub length: f64,
    pub reel: Reeling,
}

/// 🧶️ The grappling rope of an actor while it is out: its shot, the tick it was fired, the length of rope left between the hook and the hands, where the hands are and where they were one tick ago, and whether the hook bites its edge (`false`: the shot misses — the hook flies past and is pulled back).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rope {
    pub shot: Shot,
    pub since: Ticks,
    pub length: f64,
    pub hand: Point,
    pub before: Point,
    pub caught: bool,
}

/// 💓️ What an actor feels: one mood and how strongly it felt it (0…1) at the tick `since`; the present is read off it, so it is stored only when something stirs the actor.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Feeling {
    pub mood: Mood,
    pub intensity: f64,
    pub since: Ticks,
}

/// 💨️ An emitter of its species an actor runs: the emitter, the tick it began and the tick its cause ended (`None`, `null` on the wire, while the state, the trick or the purr that runs it lasts).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plume {
    pub emitter: Slug,
    pub since: Ticks,
    #[serde(deserialize_with = "nullable")]
    pub until: Option<Ticks>,
}

/// 🔲️ An axis-aligned box by its edges in stage pixels: `x0 ≤ x1` from left to right, `y0 ≤ y1` from top to bottom.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extent {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

/// 🍰️ The box a planned body stays inside from tick `from` to tick `until`, both included.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slice {
    pub from: Ticks,
    pub until: Ticks,
    pub extent: Extent,
}

/// 🎫️ The swept corridor of a planned motion: its slices in ascending ticks without a gap, and where its owner rests once the last slice has passed (`None`, `null` on the wire: the motion ends off stage). Everybody else stays out of it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub owner: Slug,
    pub slices: Vec<Slice>,
    #[serde(deserialize_with = "nullable")]
    pub rest: Option<Extent>,
}

/// 🚩️ One tick of a planned motion: where the feet are at its end and how fast they move in pixels per second, the tilt of the drawing about the scruff in turns, and the canopy while a parachute is out (`None`, `null` on the wire, while none is).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub tilt: Turns,
    #[serde(deserialize_with = "nullable")]
    pub canopy: Option<Canopy>,
}

/// 🏁️ What a planned motion ends on: a perch, a head, or nothing on stage (the contract's `enum: ["perch", "head", "away"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Ending {
    Perch,
    Head,
    Away,
}

/// 🛬️ Every [`Ending`], in the order of the contract.
pub const ENDINGS: [Ending; 3] = [Ending::Perch, Ending::Head, Ending::Away];

/// 🛫️ A planned motion through the air — a hop, a glide, a fall, a throw, a descent under a parachute, the way back after a cancelled pick-up —: its owner, the tick at whose end the feet are at the first waypoint, a waypoint per tick, what it ends on (`Perch`: the surface `landing`; `Head`: the actor `landing`; `Away`: below the stage, `landing` empty) and the speed in pixels per second with which it touches down.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    pub owner: Slug,
    pub from: Ticks,
    pub steps: Vec<Waypoint>,
    pub ending: Ending,
    pub landing: String,
    pub touch: f64,
}

/// 🥾️ One tick of a trip with gear: where the feet are at its end, what carries the actor (its perch — the surface `perch` names; `None`, `null` on the wire, off a perch —, a wall, a ladder or a rope), what it does and the way it faces, and which pitch of the trip its hands hold (−1: none).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Foothold {
    pub x: f64,
    pub y: f64,
    pub footing: Footing,
    #[serde(deserialize_with = "nullable")]
    pub perch: Option<String>,
    pub activity: Activity,
    pub facing: Facing,
    pub hold: i64,
}

/// 🛎️ What a trip with gear ends in: standing on a perch, resting on a wall, or letting go in the air (the contract's `enum: ["perch", "wall", "air"]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arrival {
    Perch,
    Wall,
    Air,
}

/// 🚉️ Every [`Arrival`], in the order of the contract.
pub const ARRIVALS: [Arrival; 3] = [Arrival::Perch, Arrival::Wall, Arrival::Air];

/// 🗻️ A planned trip with gear, leg after leg — the walk to where it sets out, then up or down walls and across the gaps between them, over a ladder it finds or raises, or up a grappling rope, across a perch it reaches on the way to the next leg —: its owner, the tick at whose end the feet are at the first foothold, a foothold per tick, the pitches its hands hold, the ladder it raises or climbs (named by its owner; `None`, `null` on the wire: none — a trip uses one ladder at most), what it ends in (`Perch`: standing on the surface `landing`; `Wall`: resting on the pitch its last foothold holds; `Air`: letting go there) and the grip it has left then.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trip {
    pub owner: Slug,
    pub from: Ticks,
    pub steps: Vec<Foothold>,
    pub pitches: Vec<Pitch>,
    #[serde(deserialize_with = "nullable")]
    pub ladder: Option<Slug>,
    pub ending: Arrival,
    pub landing: String,
    pub grip: f64,
}

/// 🌫️ The dust where a pet vanished on the spot: the middle and the size of its body and the tick it vanished.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Puff {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub tick: Ticks,
}

/// 🧸️ One pet on stage: where its feet are and what carries it (`footing`; `perch` names the surface while it stands on a perch, `host` the actor while it stands on a head, `pitch` the stretch of wall it clings to while it is on a wall) and how long it can still hold on to a wall (`grip`, in ticks of climbing: what it has left as it climbs, and what it had left when it came to rest where it hangs — hanging still drains it from `since` on; whole while it is not on a wall), the tilt of its drawing about its scruff in turns, the way it faces and the tick from which it faces that way squarely (`faced`; it turns round over the ticks before), what it does and until when, whom it does it with, how it looks and its needs; what it feels (its spirits, the bend of its mouth, are read off the feeling) and the state of its species it is in since when (`stateSince` on the wire) and the `former` one it was in just before, from which its drawing blends, the trick it performs, how warm it is from attention and the gestures the pointer may be making round it; the learner's hand, the parachute and the rope it hangs from (`None`, `null` on the wire, while it does not), and the emitters it runs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Actor {
    pub species: Slug,
    #[serde(deserialize_with = "nullable")]
    pub perch: Option<String>,
    #[serde(deserialize_with = "nullable")]
    pub host: Option<Slug>,
    #[serde(deserialize_with = "nullable")]
    pub pitch: Option<Pitch>,
    pub grip: f64,
    pub footing: Footing,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub tilt: Turns,
    pub facing: Facing,
    pub faced: Ticks,
    pub activity: Activity,
    pub since: Ticks,
    pub until: Ticks,
    pub goal: f64,
    #[serde(deserialize_with = "nullable")]
    pub partner: Option<Slug>,
    #[serde(deserialize_with = "nullable")]
    pub clip: Option<Slug>,
    pub gaze: Gaze,
    pub blink: Ticks,
    pub needs: Needs,
    pub opacity: f64,
    pub leaving: bool,
    pub draws: u32,
    pub feeling: Feeling,
    pub state: Slug,
    pub state_since: Ticks,
    pub former: Slug,
    #[serde(deserialize_with = "nullable")]
    pub trick: Option<Slug>,
    pub warmth: Warmth,
    pub hover: Hover,
    #[serde(deserialize_with = "nullable")]
    pub hang: Option<Hang>,
    #[serde(deserialize_with = "nullable")]
    pub chute: Option<Parachute>,
    #[serde(deserialize_with = "nullable")]
    pub rope: Option<Rope>,
    pub emitters: Vec<Plume>,
}

/// 💞️ How the shared history of two actors has shifted their bond away from its authored affinity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rapport {
    pub between: [Slug; 2],
    pub drift: f64,
}

/// 🧊️ A reaction of the chemistry that rests for a pair of species, `when` the side it describes first, until a tick.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cooling {
    pub reaction: Slug,
    pub when: Slug,
    pub near: Slug,
    pub until: Ticks,
}

/// 💍️ An encounter the chemistry promised two actors: the next time they are paired before `until`, it is this one (a [`meeting`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pledge {
    pub between: [Slug; 2],
    #[serde(deserialize_with = "meeting")]
    pub encounter: Activity,
    pub until: Ticks,
}

/// 🏗️ A ladder that stands on stage: its owner, the wall it leans against and the way that wall faces, the surface its foot stands on, its foot and its top, the tick it stood up, the tick it is taken away and who climbs it (`None`, `null` on the wire: nobody).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ladder {
    pub owner: Slug,
    pub wall: String,
    pub surface: String,
    pub side: Facing,
    pub foot: Point,
    pub top: Point,
    pub since: Ticks,
    pub until: Ticks,
    #[serde(deserialize_with = "nullable")]
    pub rider: Option<Slug>,
}

/// 🃏️ The prank a pet plays with a fixture of the page, lifting its copy out of its stack: the fixture, its pusher, the tick its push begins (ahead while the pet is on its way to its post; moved on to the way back when the copy slides home without its pusher; moved so far back that the copy is over once the learner threw the pusher off — the prank lasts until the pusher is down again), the way the copy is shoved (`1` rightwards), the room it has there and the width of the fixture in pixels, and the unit the stage drew for how far it goes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prank {
    pub fixture: String,
    pub pusher: Slug,
    pub since: Ticks,
    pub side: Facing,
    pub room: f64,
    pub span: f64,
    pub unit: f64,
}

/// 🏟️ The whole state of a stage; `advance` folds events into it and `frame_of` projects it. Besides the pointer (where it is, the tick it last moved and whether it is `over` free space or a control), the survey (surfaces, keep-outs, walls, fixtures and the perches and pitches cut from them), the company and its rapports: what the learner permits (`play`, `mischief`), the tick of the learner's last input (`stirred`) and of the last scroll (`scrolled`), the press, the actor it belongs to (`touched`) and the shake of a held pet, the pointer's latest samples while it holds one (`trail`, newest last), the chemistry that rests (`coolings`) and the encounters it promised (`pledges`), the ladders that stand, the prank with a fixture of the page (`lift`) and the tick from which the mode's cooldown counts (`rested`: the end of the last prank or of the last try to start one), how many pets vanished in a puff (`poofs`) and the dust of the latest ones (`puffs`), the corridors of planned motions (`claims`), the planned motions through the air themselves (`courses`) and the trips with gear (`trips`), and where the touched actor's feet stood when it was picked up from a perch (`origin`; `None`, `null` on the wire, otherwise).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    pub seed: u32,
    #[serde(deserialize_with = "elapsed")]
    pub tick: Ticks,
    pub mode: PetMode,
    pub quiet: bool,
    pub width: f64,
    pub height: f64,
    #[serde(deserialize_with = "nullable")]
    pub pointer: Option<Point>,
    pub pointed: Ticks,
    pub over: Over,
    pub glances: Vec<Point>,
    pub surfaces: Vec<Surface>,
    pub keepouts: Vec<Rect>,
    pub walls: Vec<Wall>,
    pub fixtures: Vec<Fixture>,
    pub perches: Vec<Perch>,
    pub pitches: Vec<Pitch>,
    pub wanted: Vec<Slug>,
    pub actors: Vec<Actor>,
    pub rapports: Vec<Rapport>,
    pub met: Ticks,
    pub draws: u32,
    pub play: bool,
    pub mischief: bool,
    pub stirred: Ticks,
    pub scrolled: Ticks,
    pub press: Press,
    #[serde(deserialize_with = "nullable")]
    pub touched: Option<Slug>,
    pub shaking: Shaking,
    pub trail: Vec<Point>,
    pub coolings: Vec<Cooling>,
    pub pledges: Vec<Pledge>,
    pub ladders: Vec<Ladder>,
    #[serde(deserialize_with = "nullable")]
    pub lift: Option<Prank>,
    pub rested: Ticks,
    pub poofs: u32,
    pub puffs: Vec<Puff>,
    pub claims: Vec<Claim>,
    pub courses: Vec<Course>,
    pub trips: Vec<Trip>,
    #[serde(deserialize_with = "nullable")]
    pub origin: Option<Point>,
}
//#endregion 🔖️Stage

//#region 🔖️Frame
/// 👓️ One eye as drawn: the pupil's offset from the centre of the white in pixels and how far the lid is shut (0 open, 1 shut).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EyeFrame {
    pub x: f64,
    pub y: f64,
    pub lid: f64,
}

/// 🌂️ An open or opening parachute as drawn: how far it is open (0 packed … 1 open, may overshoot) and how far canopy and cords lean, in turns on screen.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChuteTool {
    pub open: f64,
    pub sway: Turns,
}

/// ➿️ A rope as drawn from the actor (from the muzzle of its gun when it holds one, else from its pivot) to its far end in stage coordinates, its middle hanging `slack` pixels below the straight line (negative: it bows up).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RopeTool {
    pub x: f64,
    pub y: f64,
    pub slack: f64,
}

/// 🪝️ A grappling hook as drawn, in stage coordinates; it points the way its rope arrives.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookTool {
    pub x: f64,
    pub y: f64,
}

/// 🔫️ A grappling gun in the actor's hands, aimed in turns on screen (0 to the right, ¼ down).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GunTool {
    pub aim: Turns,
}

/// 🎒️ A ladder the actor carries: its lean from upright in turns on screen and its length in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LadderTool {
    pub lean: Turns,
    pub length: f64,
}

/// 🔧️ Something an actor holds or hangs from as drawn, tagged by `kind`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ToolFrame {
    Chute(ChuteTool),
    Rope(RopeTool),
    Hook(HookTool),
    Gun(GunTool),
    Ladder(LadderTool),
}

/// 🖼️ One actor as drawn: its feet, the way it faces (1 or −1), what it is doing, its opacity, a 2×3 matrix `a b c d e f` per bone in rig order and its eyes; what carries it, the state of its species, its mood and how strongly it shows (0…1), its spirits (−1 sad … 1 happy: the bend of the mouth); the tilt of the whole drawing in turns (clockwise on screen) about `pivot` (in the coordinates of its feet, x mirrored with the facing), the tools it holds or hangs from and its solid box in stage coordinates (for hit tests).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActorFrame {
    pub species: Slug,
    pub x: f64,
    pub y: f64,
    pub facing: Facing,
    pub activity: Activity,
    pub opacity: f64,
    pub bones: Vec<f64>,
    pub eyes: Vec<EyeFrame>,
    pub footing: Footing,
    pub state: Slug,
    pub mood: Mood,
    pub intensity: f64,
    pub spirits: f64,
    pub tilt: Turns,
    pub pivot: Point,
    pub tools: Vec<ToolFrame>,
    pub body: Rect,
}

/// 🚥️ How many ticks per second the motion on stage needs, in rising order: `0` on the wire when nothing moves, `16` while only sleepers breathe, `32` while loops, blinks, pupils or moods move, `64` for everything else; no other number decodes (the contract's `enum: [0, 16, 32, 64]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum Rate {
    Rest,
    Quarter,
    Half,
    Full,
}

/// 🎹️ Every [`Rate`], in the order of the contract.
pub const RATES: [Rate; 4] = [Rate::Rest, Rate::Quarter, Rate::Half, Rate::Full];

impl From<Rate> for u8 {
    /// 📶️ The wire number of a rate: its ticks per second.
    fn from(rate: Rate) -> u8 {
        match rate {
            Rate::Rest => 0,
            Rate::Quarter => 16,
            Rate::Half => 32,
            Rate::Full => 64,
        }
    }
}

impl TryFrom<u8> for Rate {
    type Error = String;

    /// ⛔️ The rate of a wire number; anything but 0, 16, 32 and 64 is refused.
    fn try_from(number: u8) -> Result<Self, String> {
        match number {
            0 => Ok(Self::Rest),
            16 => Ok(Self::Quarter),
            32 => Ok(Self::Half),
            64 => Ok(Self::Full),
            other => Err(format!("a rate is 0, 16, 32 or 64, not {other}")),
        }
    }
}

/// 🚒️ A ladder that stands on stage as drawn: from its foot (`x0`, `y0`) to its top (`x1`, `y1`) in stage coordinates, its rungs and its opacity.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LadderFrame {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub rungs: u32,
    pub opacity: f64,
}

/// 🎆️ One particle as drawn: the species and the emitter whose shape and paints it borrows, where it is in stage coordinates, its scale, its rotation in turns and its opacity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParticleFrame {
    pub species: Slug,
    pub emitter: Slug,
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: Turns,
    pub opacity: f64,
}

/// 🏋️ How far a lifted fixture is out of its place as drawn: its offset in pixels, its tilt in turns about its centre and the opacity of the copy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiftFrame {
    pub fixture: String,
    pub dx: f64,
    pub dy: f64,
    pub tilt: Turns,
    pub opacity: f64,
}

/// 💭️ The dust where a pet vanished on the spot, as drawn: the middle and the size of the body it replaces, in stage coordinates, and how far it has spread and faded (`phase`: 0 the tick the pet vanished, towards 1 as it clears).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PuffFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub phase: f64,
}

/// 🎥️ Everything a render target needs to draw a stage: the actors back to front, how many ticks per second the motion needs and the tick of the next change when nothing moves; the ladders that stand, the particles in flight, the lifted fixtures, the dust where pets vanished and the actor the learner holds (`None`, `null` on the wire: none; the target shows the grabbing cursor and captures the pointer).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    #[serde(deserialize_with = "elapsed")]
    pub tick: Ticks,
    pub actors: Vec<ActorFrame>,
    pub rate: Rate,
    #[serde(deserialize_with = "nullable")]
    pub wake: Option<Ticks>,
    pub ladders: Vec<LadderFrame>,
    pub particles: Vec<ParticleFrame>,
    pub lifts: Vec<LiftFrame>,
    pub puffs: Vec<PuffFrame>,
    #[serde(deserialize_with = "nullable")]
    pub held: Option<Slug>,
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
