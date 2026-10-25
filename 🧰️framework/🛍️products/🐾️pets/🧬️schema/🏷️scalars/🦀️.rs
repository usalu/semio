//! 🏷️ The scalars of the pets contract: slugs, texts in every language, degrees, turns, ticks, colours and the closed sets of the contract (paints, channels, gaits, activities, modes, moods, footings, gear, cues, drifts, deeds, pointers, press phases and tiers), the schema identifiers and the reader of members that may be `null`.
//!
//! @see ../🦀️.rs — the contract these types belong to
//! @see ../🔣️.json — the normative contract

use super::*;

//#region 🔖️Scalars
/// 🔖️ A kebab-case identifier, unique within its scope (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 chars).
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

/// 🕹️ What a learner can ask of a free pet without a pointer: a hello, a click trick, petting, tossing or an explicit hand gesture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Deed {
    Hello,
    Trick,
    Pet,
    Toss,
    Circle,
    Countercircle,
    Stroke,
    Shake,
}

/// 🎮️ Every [`Deed`], in the order of the contract.
pub const DEEDS: [Deed; 8] = [Deed::Hello, Deed::Trick, Deed::Pet, Deed::Toss, Deed::Circle, Deed::Countercircle, Deed::Stroke, Deed::Shake];

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
pub(super) fn nullable<'de, T: Deserialize<'de>, D: Deserializer<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
}
//#endregion 🔖️Scalars
