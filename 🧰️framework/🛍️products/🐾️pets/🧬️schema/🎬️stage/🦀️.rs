//! 🎬️ The stage of the pets contract: the events of the stage, what an actor holds, hangs on and rides, the claims and courses of its movement and the stage itself.
//!
//! @see ../🦀️.rs — the contract these types belong to
//! @see ../🔣️.json — the normative contract

use super::*;

//#region 🔖️Stage
/// 🕰️ A tick of the stage clock on the wire: a whole number that is not negative (the contract's `minimum: 0`); in memory it is [`Ticks`], because it is subtracted from points in time that may lie ahead of it.
pub(super) fn elapsed<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Ticks, D::Error> {
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
