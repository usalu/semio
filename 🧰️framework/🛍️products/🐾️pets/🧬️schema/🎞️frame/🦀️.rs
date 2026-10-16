//! 🎞️ The frames of the pets contract: what the stage shows of each actor, its tools, ladders, particles, lifts and puffs.
//!
//! @see ../🦀️.rs — the contract these types belong to
//! @see ../🔣️.json — the normative contract

use super::*;

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
