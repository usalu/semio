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

/// 🎬️ Everything an actor can be doing; exactly one activity at a time.
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
        }
    }
}

/// 🎭️ Every [`Activity`], in the order of the contract; weights and repertoires follow this order.
pub const ACTIVITIES: [Activity; 11] = [Activity::Idle, Activity::Fidget, Activity::Walk, Activity::Hop, Activity::Fall, Activity::Land, Activity::Sleep, Activity::Greet, Activity::Cuddle, Activity::Squabble, Activity::Sulk];

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

/// 🧬️ A kind of pet: its accessible name, what it stands for, its rig, face, motions and character.
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

/// 🎪️ A menagerie document: every species of a domain, their bonds and the casts of its scenes.
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

/// 🖱️ The pointer moved to a position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pointed {
    pub x: f64,
    pub y: f64,
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

/// 🗺️ The stage was measured anew: its size, every surface and everything actors must not cover.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surveyed {
    pub width: f64,
    pub height: f64,
    pub surfaces: Vec<Surface>,
    pub keepouts: Vec<Rect>,
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

/// 👆️ Someone tapped the stage at a position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Poked {
    pub x: f64,
    pub y: f64,
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
    Poked(Poked),
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

/// 🧸️ One pet on stage: where its feet are, the way it faces and the tick from which it faces that way squarely (`faced`; it turns round over the ticks before), what it does and until when, whom it does it with, how it looks and feels.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    pub species: Slug,
    #[serde(deserialize_with = "nullable")]
    pub perch: Option<String>,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
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
    pub mood: f64,
    pub needs: Needs,
    pub opacity: f64,
    pub leaving: bool,
    pub draws: u32,
}

/// 💞️ How the shared history of two actors has shifted their bond away from its authored affinity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rapport {
    pub between: [Slug; 2],
    pub drift: f64,
}

/// 🏟️ The whole state of a stage; `advance` folds events into it and `frame_of` projects it.
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
    pub glances: Vec<Point>,
    pub surfaces: Vec<Surface>,
    pub keepouts: Vec<Rect>,
    pub perches: Vec<Perch>,
    pub wanted: Vec<Slug>,
    pub actors: Vec<Actor>,
    pub rapports: Vec<Rapport>,
    pub met: Ticks,
    pub draws: u32,
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

/// 🖼️ One actor as drawn: its feet, the way it faces (1 or −1), what it is doing, its opacity, a 2×3 matrix `a b c d e f` per bone in rig order, its eyes and its mood (−1 sad … 1 happy).
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
    pub mood: f64,
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

/// 🎥️ Everything a render target needs to draw a stage: the actors back to front, how many ticks per second the motion needs and the tick of the next change when nothing moves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    #[serde(deserialize_with = "elapsed")]
    pub tick: Ticks,
    pub actors: Vec<ActorFrame>,
    pub rate: Rate,
    #[serde(deserialize_with = "nullable")]
    pub wake: Option<Ticks>,
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
