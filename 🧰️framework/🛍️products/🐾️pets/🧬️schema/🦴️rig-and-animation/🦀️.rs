//! 🦴️ The rig and the animation of the pets contract: bones, shapes, parts, eyes, mouths and faces, and the keys, tracks, clips and repertoire that move them.
//!
//! @see ../🦀️.rs — the contract these types belong to
//! @see ../🔣️.json — the normative contract

use super::*;

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
