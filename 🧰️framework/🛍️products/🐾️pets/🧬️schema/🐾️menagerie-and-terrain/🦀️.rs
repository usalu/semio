//! 🐾️ The menagerie and the terrain of the pets contract: species with their size, palette, locomotion, temperament, tints, emitters, states, tricks and purrs, bonds, casts, traits, effects and reactions, and the points, rectangles, surfaces, perches, walls, pitches and fixtures they act on.
//!
//! @see ../🦀️.rs — the contract these types belong to
//! @see ../🔣️.json — the normative contract

use super::*;

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
pub(super) fn meeting<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Activity, D::Error> {
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
