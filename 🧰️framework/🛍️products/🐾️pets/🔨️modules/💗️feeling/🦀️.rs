//! 💗️ What a pet feels, in numbers — the Rust twin of the feeling module: its mood and how it comes and goes, what that does to its face and its wishes, how moods travel between pets, the lasting states of a species, the tricks on offer, and the chemistry between species. Pure functions over the schema; the stage folds them.
//!
//! Every table is an array in the twin's order (`MOODS`; `ACTIVITIES`; `[greet, cuddle, squabble]`), every constant
//! the twin's literal and every expression the twin's, term for term and in its order (no `mul_add`, no
//! reassociation), so both cores yield the same bits. Time is whole ticks; rates are per second and meet time as
//! `(rate × ticks) ÷ 64`. Nothing here draws: whoever needs chance is handed units.
//!
//! A [`Feeling`] is an anchor, not a clock: the stage stores what [`impulse`] (or [`at_rest`]) returns and reads the
//! present through [`settled`], a closed form over any number of ticks. The `while` loops that find the tick a mood
//! fades, calms or comes to rest are the twin's, so both answer the same tick; a negated comparison of the twin is
//! written as its complement where a guard in front of it has already refused an intensity that is no number, and
//! kept negated where nothing guards it (an amount, a gain, a unit), so NaN takes the twin's branch.
//!
//! Where the twin answers `null` or `-1` this twin answers `None`: a state that stays ([`state_ends`]), a trick that
//! is not on offer, nobody showing off ([`Leaning::show`]). Tricks are answered as references into the species;
//! states, rungs and standings as owned ids. A click index and a step direction are whole numbers here.
//!
//! @see ../💗️feeling/🟦️.ts — the TypeScript twin
//! @see ../🧠️behavior/🦀️.rs — `affinity_of`, which a reaction's `affinity` bounds
//! @see ../🎲️randomness/🦀️.rs — `weighted_index`, the one weighted pick
//! @see ../📐️trigonometry/🦀️.rs — `clamp`
//! @see ../../🧬️schema/🦀️.rs — `Feeling`, `Mood`, `Cue`, `SpeciesState`, `Trick`, `Trait`, `Effect`, `Reaction`, `Species`, `Menagerie`, `Cooling`

use crate::behavior::affinity_of;
use crate::randomness::weighted_index;
use crate::schema::{Activity, Cooling, Cue, Feeling, Menagerie, Mood, Party, Placement, Rapport, Reaction, Slug, Species, SpeciesState, Temperament, Ticks, Trait, Trick, ACTIVITIES, MOODS, TICKS_PER_SECOND};
use crate::trigonometry::clamp;
use serde::{Deserialize, Serialize};

//#region 🔖️Moods
/// 🥇️ How much a mood outranks the others, in `MOODS` order: scared 6, grumpy 5, sad 4, proud, happy and playful 3, curious 2, sleepy 1 (it follows the energy need), content 0. A mood of a higher rank replaces a lower one at once.
pub const MOOD_PRIORITIES: [u8; 9] = [0, 3, 3, 2, 3, 1, 5, 4, 6];

/// ⏳️ For how many ticks a freshly stirred mood neither fades nor gives way to a mood of its own rank or below (2 s): faces do not flicker.
pub const MOOD_HOLD: Ticks = 128;

/// 🌫️ The intensity below which a mood is no longer felt: it has faded and the resting mood of the species returns. An impulse fainter than this never replaces a mood.
pub const MOOD_FAINT: f64 = 0.05;

/// 🛋️ How strongly a pet feels the resting mood of its species when nothing stirs it. A feeling at or below this level is calm: it gives way to any other mood, catches the moods of others and spreads none.
pub const MOOD_REST: f64 = 0.25;

/// 🌅️ By how much per second the resting mood returns up to [`MOOD_REST`] after another mood has faded (8 s from nothing).
pub const MOOD_RISE: f64 = 0.03125;

/// 💪️ How many times stronger than the present intensity an impulse of a lower rank must be to replace a mood whose hold is over.
pub const MOOD_OVERRIDE: f64 = 1.2;

/// 📉️ By how much per second a mood fades once its hold is over, in `MOODS` order: from full strength happy and grumpy last 64 s, playful and proud 32 s, curious 16 s, sleepy and sad 128 s, scared 6.4 s (content, when stirred above its rest, 32 s).
pub const MOOD_DECAYS: [f64; 9] = [0.03125, 0.015625, 0.03125, 0.0625, 0.03125, 0.0078125, 0.015625, 0.0078125, 0.15625];

/// 🧲️ The factor on the fading of the resting mood of a species: a pet stays twice as long in the mood it is prone to.
pub const PRONE_DECAY: f64 = 0.5;

/// 🔢️ The place of a mood in `MOODS`, the row of every table of this module.
fn mood_index(mood: Mood) -> usize {
    MOODS.iter().position(|known| *known == mood).unwrap_or_default()
}

/// 🏅️ The rank of a mood ([`MOOD_PRIORITIES`]).
fn rank_of(mood: Mood) -> u8 {
    MOOD_PRIORITIES[mood_index(mood)]
}

/// 😌️ The feeling of a pet that nothing has stirred yet: the resting mood of its species at [`MOOD_REST`].
pub fn at_rest(resting: Mood, tick: Ticks) -> Feeling {
    Feeling { mood: resting, intensity: MOOD_REST, since: tick }
}

/// 🪞️ The mood a pet is in for everybody who asks "is it grumpy?": its mood while that is stirred above [`MOOD_REST`], `content` while it is calm — whatever the resting mood of its species, a pet at rest is nobody's grump. Tricks and reactions ask this.
pub fn shown_mood(mood: Mood, intensity: f64) -> Mood {
    if intensity > MOOD_REST {
        mood
    } else {
        Mood::Content
    }
}

/// ⚡️ The feeling after an impulse of `amount` (0…1) of a mood at `tick`; `feeling` is the present one ([`settled`] at that tick), and the result is what the stage stores.
///
/// An impulse of nothing changes nothing. The same mood is reinforced: its intensity rises by `amount` (at most 1)
/// and its hold begins anew. An impulse of `content` soothes: a mood that is worse than content (sleepy, curious,
/// scared, grumpy, sad by their valence) loses `amount` and is over when less than [`MOOD_FAINT`] is left; joy is
/// never taken away. Any other mood replaces the present one — at `amount`, with a fresh hold — when the impulse is
/// at least [`MOOD_FAINT`] and the present feeling is calm (at most [`MOOD_REST`]), or the new mood outranks it
/// ([`MOOD_PRIORITIES`]), or the hold is over and the new mood is of the same rank or more than [`MOOD_OVERRIDE`]
/// times as strong as what is left. Otherwise the impulse is lost.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn impulse(feeling: Feeling, mood: Mood, amount: f64, tick: Ticks) -> Feeling {
    if !(amount > 0.0) {
        return feeling;
    }
    if mood == feeling.mood {
        let raised = feeling.intensity + amount;
        return Feeling { mood, intensity: if raised < 1.0 { raised } else { 1.0 }, since: tick };
    }
    if mood == Mood::Content {
        if !(MOOD_VALENCES[mood_index(feeling.mood)] < MOOD_VALENCES[0]) {
            return feeling;
        }
        let eased = feeling.intensity - amount;
        return if eased >= MOOD_FAINT { Feeling { mood: feeling.mood, intensity: eased, since: feeling.since } } else { Feeling { mood: feeling.mood, intensity: 0.0, since: tick } };
    }
    if amount < MOOD_FAINT {
        return feeling;
    }
    let fresh = Feeling { mood, intensity: if amount < 1.0 { amount } else { 1.0 }, since: tick };
    if feeling.intensity <= MOOD_REST {
        return fresh;
    }
    let rank = rank_of(mood);
    let held = rank_of(feeling.mood);
    if rank > held {
        return fresh;
    }
    if tick - feeling.since < MOOD_HOLD {
        return feeling;
    }
    if rank == held || amount > MOOD_OVERRIDE * feeling.intensity {
        fresh
    } else {
        feeling
    }
}

/// 🕯️ The first tick at which a mood that is not the resting one is no longer felt: `since` when it is faint already (or its intensity is no number in [0, 1]), else the first tick after its hold at which less than [`MOOD_FAINT`] is left.
fn fades_at(feeling: Feeling) -> Ticks {
    if !(feeling.intensity >= MOOD_FAINT && feeling.intensity <= 1.0) {
        return feeling.since;
    }
    let rate = MOOD_DECAYS[mood_index(feeling.mood)];
    let tps = TICKS_PER_SECOND as f64;
    let mut wait = (((feeling.intensity - MOOD_FAINT) * tps) / rate).floor() as Ticks + 1;
    while wait > 1 && feeling.intensity - (rate * (wait - 1) as f64) / tps < MOOD_FAINT {
        wait -= 1;
    }
    while feeling.intensity - (rate * wait as f64) / tps >= MOOD_FAINT {
        wait += 1;
    }
    feeling.since + MOOD_HOLD + wait
}

/// 🍂️ The feeling as it stands at `tick`, a closed form over any number of ticks: settling it again at the same tick returns it unchanged, and settling in two steps equals settling in one.
///
/// A mood other than the resting one keeps its intensity through its hold and then loses `MOOD_DECAYS × seconds`;
/// from the tick less than [`MOOD_FAINT`] is left ([`fades_at`]) the pet is back in the resting mood of its species,
/// which rises from 0 by [`MOOD_RISE`] per second up to [`MOOD_REST`]. The resting mood itself, stirred above its
/// rest, fades half as fast ([`PRONE_DECAY`]) down to [`MOOD_REST`]; below it (soothed), it rises back. The answer is
/// anchored at `tick` — `since` is set so that the same future follows: `tick − MOOD_HOLD` for a fading mood, `tick`
/// for a rising one, and the tick it came to rest ([`settles_at`]) for a feeling at rest; an unchanged feeling is
/// returned as it is.
pub fn settled(feeling: Feeling, resting: Mood, tick: Ticks) -> Feeling {
    let rested = settles_at(feeling, resting);
    let tps = TICKS_PER_SECOND as f64;
    if tick >= rested {
        return if feeling.mood == resting && feeling.intensity == MOOD_REST { feeling } else { Feeling { mood: resting, intensity: MOOD_REST, since: rested } };
    }
    let rate = MOOD_DECAYS[mood_index(feeling.mood)];
    if feeling.mood == resting {
        if feeling.intensity > MOOD_REST {
            let wait = tick - feeling.since - MOOD_HOLD;
            return if wait > 0 { Feeling { mood: resting, intensity: feeling.intensity - (rate * PRONE_DECAY * wait as f64) / tps, since: tick - MOOD_HOLD } } else { feeling };
        }
        let span = tick - feeling.since;
        return if span > 0 { Feeling { mood: resting, intensity: feeling.intensity + (MOOD_RISE * span as f64) / tps, since: tick } } else { feeling };
    }
    let wait = tick - feeling.since - MOOD_HOLD;
    let left = if wait > 0 { feeling.intensity - (rate * wait as f64) / tps } else { feeling.intensity };
    if left >= MOOD_FAINT {
        return if wait > 0 { Feeling { mood: feeling.mood, intensity: left, since: tick - MOOD_HOLD } } else { feeling };
    }
    let span = tick - fades_at(feeling);
    Feeling { mood: resting, intensity: if span > 0 { (MOOD_RISE * span as f64) / tps } else { 0.0 }, since: tick }
}

/// 🏁️ The tick from which [`settled`] answers the resting mood at [`MOOD_REST`] and nothing changes any more: the horizon of a feeling for whoever lets the stage rest. An intensity that is no number in [0, 1] is no feeling at all: it is at rest from `since` on.
pub fn settles_at(feeling: Feeling, resting: Mood) -> Ticks {
    if !(feeling.intensity >= 0.0 && feeling.intensity <= 1.0) {
        return feeling.since;
    }
    let tps = TICKS_PER_SECOND as f64;
    if feeling.mood != resting {
        let mut rise = ((MOOD_REST * tps) / MOOD_RISE).floor() as Ticks;
        while rise > 1 && (MOOD_RISE * (rise - 1) as f64) / tps >= MOOD_REST {
            rise -= 1;
        }
        while (MOOD_RISE * rise as f64) / tps < MOOD_REST {
            rise += 1;
        }
        return fades_at(feeling) + rise;
    }
    if feeling.intensity > MOOD_REST {
        let rate = MOOD_DECAYS[mood_index(feeling.mood)] * PRONE_DECAY;
        let mut wait = (((feeling.intensity - MOOD_REST) * tps) / rate).floor() as Ticks;
        while wait > 1 && feeling.intensity - (rate * (wait - 1) as f64) / tps <= MOOD_REST {
            wait -= 1;
        }
        while wait < 1 || feeling.intensity - (rate * wait as f64) / tps > MOOD_REST {
            wait += 1;
        }
        return feeling.since + MOOD_HOLD + wait;
    }
    if feeling.intensity == MOOD_REST {
        return feeling.since;
    }
    let mut span = (((MOOD_REST - feeling.intensity) * tps) / MOOD_RISE).floor() as Ticks;
    while span > 1 && feeling.intensity + (MOOD_RISE * (span - 1) as f64) / tps >= MOOD_REST {
        span -= 1;
    }
    while span < 1 || feeling.intensity + (MOOD_RISE * span as f64) / tps < MOOD_REST {
        span += 1;
    }
    feeling.since + span
}

/// 🌤️ The first tick from which a feeling shows `content` ([`shown_mood`] of what [`settled`] answers): `since` when it shows content already — a calm feeling, or content itself —, else the first tick after its hold at which no more than [`MOOD_REST`] is left (its resting mood fades at half the rate). The stage's horizon for whatever the shown mood steers.
pub fn calms_at(feeling: Feeling, resting: Mood) -> Ticks {
    if !(feeling.intensity > MOOD_REST && feeling.intensity <= 1.0) || feeling.mood == Mood::Content {
        return feeling.since;
    }
    let tps = TICKS_PER_SECOND as f64;
    let rate = if feeling.mood == resting { MOOD_DECAYS[mood_index(feeling.mood)] * PRONE_DECAY } else { MOOD_DECAYS[mood_index(feeling.mood)] };
    let mut wait = (((feeling.intensity - MOOD_REST) * tps) / rate).floor() as Ticks;
    while wait > 1 && feeling.intensity - (rate * (wait - 1) as f64) / tps <= MOOD_REST {
        wait -= 1;
    }
    while wait < 1 || feeling.intensity - (rate * wait as f64) / tps > MOOD_REST {
        wait += 1;
    }
    feeling.since + MOOD_HOLD + wait
}
//#endregion 🔖️Moods

//#region 🔖️Face
/// 🙂️ How far a mood at full strength bends the mouth, from −1 (sad) to 1 (happy), in `MOODS` order: content 0.3, happy 0.8, playful 0.6, curious 0.2, proud 0.6, sleepy 0.1, grumpy −0.5, sad −0.8, scared −0.4. It is the valence of the mood.
pub const MOOD_VALENCES: [f64; 9] = [0.3, 0.8, 0.6, 0.2, 0.6, 0.1, -0.5, -0.8, -0.4];

/// 😴️ How far a mood at full strength lowers the upper lids at rest (0 wide open … 1 shut; blinks run on top), in `MOODS` order.
pub const MOOD_LIDS: [f64; 9] = [0.1, 0.12, 0.0, 0.0, 0.15, 0.55, 0.35, 0.25, 0.0];

/// 🤨️ How a mood at full strength slants the lid edge over each eye, in degrees and `MOODS` order: positive lowers the inner end (hooded, angry), negative raises it (arched, worried).
pub const MOOD_SLANTS: [f64; 9] = [0.0, 0.0, -10.0, -10.0, 6.0, 4.0, 12.0, -10.0, -14.0];

/// 🧍️ How far a mood at full strength lets the bone that carries the first eye sink, in pixels and `MOODS` order: negative lifts it (happy, proud), positive lets it droop (sleepy, sad, scared).
pub const MOOD_DROPS: [f64; 9] = [0.0, -1.5, 0.0, -1.0, -1.5, 1.5, 0.0, 2.0, 2.0];

/// 😀️ What a feeling does to the shared face: the bend of the mouth (−1…1), the resting height of the lids (0…1), the slant of the lids in degrees and the drop of the posture in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Countenance {
    pub bend: f64,
    pub lid: f64,
    pub slant: f64,
    pub drop: f64,
}

/// ➗️ One channel of the face: from what content shows towards what the mood shows at full strength, by the intensity.
fn blend(table: &[f64; 9], feeling: Feeling) -> f64 {
    let calm = table[0];
    calm + (table[mood_index(feeling.mood)] - calm) * feeling.intensity
}

/// 👄️ The valence of a mood, from −1 (sad) to 1 (happy): [`MOOD_VALENCES`].
pub fn valence_of(mood: Mood) -> f64 {
    MOOD_VALENCES[mood_index(mood)]
}

/// 🎈️ The spirits of a feeling, the one number the mouth of the first round bends with: the valence of content (0.3) moved towards the valence of the mood by its intensity.
pub fn spirits_of(feeling: Feeling) -> f64 {
    blend(&MOOD_VALENCES, feeling)
}

/// 🎭️ The face of a feeling: every channel is the face of content moved towards the face of the mood by its intensity, so a fading mood eases back without a jump.
pub fn face_of(feeling: Feeling) -> Countenance {
    Countenance { bend: blend(&MOOD_VALENCES, feeling), lid: blend(&MOOD_LIDS, feeling), slant: blend(&MOOD_SLANTS, feeling), drop: blend(&MOOD_DROPS, feeling) }
}
//#endregion 🔖️Face

//#region 🔖️Appraisal
/// 📅️ What can happen to a pet that moves its mood: the learner said hello (`greeted`), it performed a trick (`tricked`), it is being petted (`purred`), picked up (`lifted`), still held a while later (`dangled`), shaken (`shaken`), it came down hard (`dropped`), softly under its parachute (`floated`) or softly without one (`landed`), another pet landed on its head (`trampled`), another pet greeted it (`welcomed`), cuddled it (`cuddled`), squabbled with it (`squabbled`), its sulk ended (`mended`), it was rained on (`drenched`), clicked once too often (`pestered`), thrown off what it played with (`evicted`), the pointer lingers on it (`watched`), something gave it a fright (`startled`), it woke up (`woken`), another pet showed it a trick (`entertained`), it got where it wanted with its gear — up a wall, over a ladder, up its rope (`climbed`) —, its grappling hook missed the edge (`missed`), it bounced off an edge of the stage in flight (`bonked`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Occasion {
    Greeted,
    Tricked,
    Purred,
    Lifted,
    Dangled,
    Shaken,
    Dropped,
    Floated,
    Landed,
    Trampled,
    Welcomed,
    Cuddled,
    Squabbled,
    Mended,
    Drenched,
    Pestered,
    Evicted,
    Watched,
    Startled,
    Woken,
    Entertained,
    Climbed,
    Missed,
    Bonked,
}

/// 🗓️ Every [`Occasion`], in the twin's order.
pub const OCCASIONS: [Occasion; 24] = [
    Occasion::Greeted,
    Occasion::Tricked,
    Occasion::Purred,
    Occasion::Lifted,
    Occasion::Dangled,
    Occasion::Shaken,
    Occasion::Dropped,
    Occasion::Floated,
    Occasion::Landed,
    Occasion::Trampled,
    Occasion::Welcomed,
    Occasion::Cuddled,
    Occasion::Squabbled,
    Occasion::Mended,
    Occasion::Drenched,
    Occasion::Pestered,
    Occasion::Evicted,
    Occasion::Watched,
    Occasion::Startled,
    Occasion::Woken,
    Occasion::Entertained,
    Occasion::Climbed,
    Occasion::Missed,
    Occasion::Bonked,
];

/// 💢️ One impulse an occasion gives: a mood and its amount before the character of the species scales it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stir {
    pub occasion: Occasion,
    pub mood: Mood,
    pub amount: f64,
}

/// 📖️ The appraisal table: which occasion gives which impulses, applied in this order. An impulse of `content` soothes first (a cuddle ends fear, anger and sadness before it makes happy; a hello takes a little of them).
pub const APPRAISALS: [Stir; 30] = [
    Stir { occasion: Occasion::Greeted, mood: Mood::Content, amount: 0.2 },
    Stir { occasion: Occasion::Greeted, mood: Mood::Happy, amount: 0.3 },
    Stir { occasion: Occasion::Tricked, mood: Mood::Playful, amount: 0.4 },
    Stir { occasion: Occasion::Purred, mood: Mood::Content, amount: 0.5 },
    Stir { occasion: Occasion::Purred, mood: Mood::Happy, amount: 0.3 },
    Stir { occasion: Occasion::Lifted, mood: Mood::Scared, amount: 0.3 },
    Stir { occasion: Occasion::Dangled, mood: Mood::Curious, amount: 0.4 },
    Stir { occasion: Occasion::Shaken, mood: Mood::Scared, amount: 0.5 },
    Stir { occasion: Occasion::Dropped, mood: Mood::Grumpy, amount: 0.4 },
    Stir { occasion: Occasion::Floated, mood: Mood::Content, amount: 1.0 },
    Stir { occasion: Occasion::Floated, mood: Mood::Proud, amount: 0.4 },
    Stir { occasion: Occasion::Landed, mood: Mood::Content, amount: 0.5 },
    Stir { occasion: Occasion::Landed, mood: Mood::Happy, amount: 0.3 },
    Stir { occasion: Occasion::Trampled, mood: Mood::Grumpy, amount: 0.3 },
    Stir { occasion: Occasion::Welcomed, mood: Mood::Happy, amount: 0.3 },
    Stir { occasion: Occasion::Cuddled, mood: Mood::Content, amount: 1.0 },
    Stir { occasion: Occasion::Cuddled, mood: Mood::Happy, amount: 0.5 },
    Stir { occasion: Occasion::Squabbled, mood: Mood::Grumpy, amount: 0.5 },
    Stir { occasion: Occasion::Mended, mood: Mood::Content, amount: 0.3 },
    Stir { occasion: Occasion::Drenched, mood: Mood::Sad, amount: 0.4 },
    Stir { occasion: Occasion::Pestered, mood: Mood::Grumpy, amount: 0.4 },
    Stir { occasion: Occasion::Evicted, mood: Mood::Scared, amount: 0.4 },
    Stir { occasion: Occasion::Watched, mood: Mood::Curious, amount: 0.4 },
    Stir { occasion: Occasion::Startled, mood: Mood::Scared, amount: 0.4 },
    Stir { occasion: Occasion::Woken, mood: Mood::Content, amount: 1.0 },
    Stir { occasion: Occasion::Entertained, mood: Mood::Happy, amount: 0.3 },
    Stir { occasion: Occasion::Climbed, mood: Mood::Content, amount: 0.3 },
    Stir { occasion: Occasion::Climbed, mood: Mood::Proud, amount: 0.4 },
    Stir { occasion: Occasion::Missed, mood: Mood::Grumpy, amount: 0.3 },
    Stir { occasion: Occasion::Bonked, mood: Mood::Grumpy, amount: 0.3 },
];

/// 🧬️ What of a species shapes its feelings: its resting mood and its temperament (the twin's `Pick<Species, "mood" | "temperament">`; every species gives one).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub mood: Mood,
    pub temperament: Temperament,
}

impl From<&Species> for Character {
    /// 🪪️ The character of a species: its resting mood and its temperament.
    fn from(species: &Species) -> Self {
        Self { mood: species.mood, temperament: species.temperament }
    }
}

/// 🧮️ How the temperament of a species scales an impulse, per mood in `MOODS` order as `[base, energy, sociability, curiosity]`: the scale is `base + energy × temperament.energy + …`. The energetic lean playful, the sociable happy and sad, the curious curious; low energy leans sleepy and low sociability grumpy; content, proud and scared are taken as they come.
pub const PRONENESS: [[f64; 4]; 9] = [[1.0, 0.0, 0.0, 0.0], [0.5, 0.0, 1.0, 0.0], [0.5, 1.0, 0.0, 0.0], [0.5, 0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 0.0], [1.5, -1.0, 0.0, 0.0], [1.5, 0.0, -1.0, 0.0], [0.5, 0.0, 1.0, 0.0], [1.0, 0.0, 0.0, 0.0]];

/// 📈️ The factor on every impulse of the resting mood of a species: a pet is half again as easily stirred into the mood it is prone to.
pub const PRONE_GAIN: f64 = 1.5;

/// 🎚️ How strongly a species takes an impulse of a mood: the scale of its temperament ([`PRONENESS`]), times [`PRONE_GAIN`] for its resting mood.
pub fn proneness_of(character: Character, mood: Mood) -> f64 {
    let row = PRONENESS[mood_index(mood)];
    let lean = row[0] + row[1] * character.temperament.energy + row[2] * character.temperament.sociability + row[3] * character.temperament.curiosity;
    if character.mood == mood {
        lean * PRONE_GAIN
    } else {
        lean
    }
}

/// 🌊️ [`impulse`] as a species takes it: the amount times its [`proneness_of`], held inside [0, 1]. This is how the mood effect of a reaction lands.
pub fn stirred(feeling: Feeling, mood: Mood, amount: f64, character: Character, tick: Ticks) -> Feeling {
    impulse(feeling, mood, clamp(amount * proneness_of(character, mood), 0.0, 1.0), tick)
}

/// 🧐️ The feeling after an occasion: every impulse of its rows in [`APPRAISALS`], in table order, each as the species takes it ([`stirred`]).
pub fn appraised(feeling: Feeling, occasion: Occasion, character: Character, tick: Ticks) -> Feeling {
    APPRAISALS.iter().filter(|stir| stir.occasion == occasion).fold(feeling, |present, stir| stirred(present, stir.mood, stir.amount, character, tick))
}

/// 🪄️ The amount of the impulse a trick leaves: of the mood the trick names, or playful when it names none (the `tricked` row of [`APPRAISALS`]).
pub const TRICK_AMOUNT: f64 = 0.4;

/// 🎉️ The feeling after a trick was performed: an impulse of [`TRICK_AMOUNT`] of the mood the trick leaves (`Trick.mood`), as the species takes it; playful when the trick names none.
pub fn performed(feeling: Feeling, trick: &Trick, character: Character, tick: Ticks) -> Feeling {
    match trick.mood {
        None => appraised(feeling, Occasion::Tricked, character, tick),
        Some(mood) => stirred(feeling, mood, TRICK_AMOUNT, character, tick),
    }
}

/// 🥱️ The energy below which a pet grows sleepy — the same 0.6 below which the behaviour module lets it doze off.
pub const DROWSY_ENERGY: f64 = 0.6;

/// 💤️ The feeling after the energy need was looked at: sleepy follows it. With `tired = (0.6 − energy) ÷ 0.6` held in [0, 1], a pet that is less sleepy than tired gets the difference as an impulse of sleepy — which, being of low rank, takes a calm mood, and a stirred one only once its hold is over and the pet is more than [`MOOD_OVERRIDE`] times as tired as it is stirred; a rested pet is left as it is and its sleepiness fades by itself, or at once when it is `woken`.
pub fn drowsed(feeling: Feeling, energy: f64, tick: Ticks) -> Feeling {
    let tired = clamp((DROWSY_ENERGY - energy) / DROWSY_ENERGY, 0.0, 1.0);
    let felt = if feeling.mood == Mood::Sleepy { feeling.intensity } else { 0.0 };
    if tired > felt {
        impulse(feeling, Mood::Sleepy, tired - felt, tick)
    } else {
        feeling
    }
}
//#endregion 🔖️Appraisal

//#region 🔖️Wishes
/// ⚖️ What a mood at full strength does to how much a pet feels like each activity: a multiplier per activity in `ACTIVITIES` order, one row per mood in `MOODS` order. Only what a pet starts by itself is moved — fidget, walk, hop, sleep, the gear (aim, climb, carry), trick and push; everything entered by events stays at 1.
///
/// Happy: a little more of everything, less sleep. Playful: fidget ×2, walk ×1.5, hop ×2, trick ×3, push ×2. Curious:
/// walk ×2, gear ×2. Proud: fidget ×1.5, trick ×2. Sleepy: sleep ×4, everything else a quarter to a half. Grumpy:
/// stomps about (walk ×1.5) and plays little. Sad: sits still and sleeps more. Scared: freezes — no fidget, sleep,
/// gear, trick or push, a quarter of the walks and hops.
pub const MOOD_WEIGHTS: [[f64; ACTIVITIES.len()]; 9] = [
    [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    [1.0, 1.5, 1.25, 1.25, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.25, 1.0, 1.25, 1.0, 1.0, 1.25, 1.5, 1.0, 1.0, 1.0, 1.0, 1.25],
    [1.0, 2.0, 1.5, 2.0, 1.0, 1.0, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.5, 1.0, 1.5, 1.0, 1.0, 1.5, 3.0, 1.0, 1.0, 1.0, 1.0, 2.0],
    [1.0, 1.0, 2.0, 1.5, 1.0, 1.0, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.5],
    [1.0, 1.5, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    [1.0, 0.25, 0.5, 0.25, 1.0, 1.0, 4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25, 1.0, 0.25, 1.0, 1.0, 0.25, 0.25, 1.0, 1.0, 1.0, 1.0, 0.25],
    [1.0, 1.0, 1.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0, 0.5, 0.25, 1.0, 1.0, 1.0, 1.0, 0.5],
    [1.0, 0.25, 0.5, 0.25, 1.0, 1.0, 1.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.5, 1.0, 1.0, 0.5, 0.25, 1.0, 1.0, 1.0, 1.0, 0.25],
    [1.0, 0.0, 0.25, 0.25, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0],
];

/// 🏋️ The multipliers a mood of an intensity puts on `activity_weights`, in `ACTIVITIES` order: 1 moved towards the row of the mood in [`MOOD_WEIGHTS`] by the intensity. Multiply weight by weight; a weight of 0 stays 0.
pub fn mood_weights(mood: Mood, intensity: f64) -> [f64; ACTIVITIES.len()] {
    MOOD_WEIGHTS[mood_index(mood)].map(|multiplier| 1.0 + (multiplier - 1.0) * intensity)
}

/// 💞️ By how much a mood at full strength shifts the affinity an encounter is drawn from, in `MOODS` order: happy +0.15, playful +0.1, curious and proud +0.05, content 0, sleepy and scared −0.05, sad −0.1, grumpy −0.25.
pub const MOOD_AFFINITIES: [f64; 9] = [0.0, 0.15, 0.1, 0.05, 0.05, -0.05, -0.25, -0.1, -0.05];

/// 🥧️ What a mood at full strength does to the chances `[greet, cuddle, squabble]` of an encounter, one row per mood in `MOODS` order: the happy and the playful greet and cuddle more, the curious and the proud greet more, nobody picks on the sleepy, the grumpy squabble more and are poor company, the sad and the scared are comforted (cuddle ×2 — which only friends ever do) and left in peace.
pub const MOOD_SHARES: [[f64; 3]; 9] = [[1.0, 1.0, 1.0], [1.25, 1.25, 1.0], [1.25, 1.25, 1.0], [1.25, 1.0, 1.0], [1.5, 1.0, 1.0], [1.0, 1.0, 0.5], [0.5, 0.5, 1.5], [1.0, 2.0, 0.5], [1.0, 2.0, 0.25]];

/// 🧭️ How the moods of two pets tip their encounter: `affinity` is added to the affinity the encounter is drawn from, `shares` multiplies the chances `[greet, cuddle, squabble]`, and `show` says who shows off instead of greeting (`Some(0)` the first, `Some(1)` the second, `None` nobody — the twin's −1).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leaning {
    pub affinity: f64,
    pub shares: [f64; 3],
    pub show: Option<usize>,
}

/// 🤝️ How an encounter of two pets leans by what they feel: the mean of their [`MOOD_AFFINITIES`], each by its intensity (two grumpy pets squabble sooner); the product of their [`MOOD_SHARES`], each moved from 1 by its intensity (a scared one is comforted by a friend); and the proud one — the prouder of two, the first when they are equally proud — shows off when it feels it above [`MOOD_REST`].
pub fn encounter_bias(first: Feeling, second: Feeling) -> Leaning {
    let one = mood_index(first.mood);
    let other = mood_index(second.mood);
    let shares = std::array::from_fn(|index| (1.0 + (MOOD_SHARES[one][index] - 1.0) * first.intensity) * (1.0 + (MOOD_SHARES[other][index] - 1.0) * second.intensity));
    let vain = first.mood == Mood::Proud && first.intensity > MOOD_REST;
    let boastful = second.mood == Mood::Proud && second.intensity > MOOD_REST;
    let show = if vain && (!boastful || first.intensity >= second.intensity) {
        Some(0)
    } else if boastful {
        Some(1)
    } else {
        None
    };
    Leaning { affinity: (MOOD_AFFINITIES[one] * first.intensity + MOOD_AFFINITIES[other] * second.intensity) / 2.0, shares, show }
}

/// 🍰️ The chances `[greet, cuddle, squabble]` of an encounter as two moods tip them: share by share times the leaning. They no longer sum to 1; `weighted_index` picks in proportion.
pub fn swayed_shares(shares: [f64; 3], leaning: Leaning) -> [f64; 3] {
    [shares[0] * leaning.shares[0], shares[1] * leaning.shares[1], shares[2] * leaning.shares[2]]
}
//#endregion 🔖️Wishes

//#region 🔖️Contagion
/// 🦠️ How readily a mood travels from one pet to the next, in `MOODS` order: sleepy 0.8 (a yawn), playful 0.6, happy 0.5, curious 0.4, scared 0.3, grumpy 0.2; content, proud and sad stay where they are. Fear and anger spread weakly on purpose.
pub const MOOD_SPREADS: [f64; 9] = [0.0, 0.5, 0.6, 0.4, 0.0, 0.8, 0.2, 0.0, 0.3];

/// 🥁️ Every how many ticks moods travel (twice a second).
pub const CONTAGION_BEAT: Ticks = 32;

/// 📏️ Within how many body widths of each other two pets catch each other's moods.
pub const CONTAGION_REACH: f64 = 3.0;

/// 🤧️ The feeling of a pet after one beat beside another, both as they stood when the beat began: moods travel, and never grow on the way.
///
/// Only a stirred mood travels (above [`MOOD_REST`]). Its pull is `gain = MOOD_SPREADS × sociability × (0.5 + 0.5 ×
/// affinity)` — the sociability of the one who catches, the affinity of the two. A pet in the same mood is lifted by
/// `gain × (theirs − mine)` when the other feels it more strongly, and its hold is left alone. A calm pet (at most
/// [`MOOD_REST`]) adopts the mood at `gain × theirs` when that is at least [`MOOD_FAINT`], with a fresh hold. Anyone
/// else keeps its mood. `gain` is at most 0.8, so nobody ends above the one it caught from. Several neighbours are
/// applied one after the other in species order, each as it stood when the beat began.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn caught(mine: Feeling, theirs: Feeling, affinity: f64, sociability: f64, tick: Ticks) -> Feeling {
    if !(theirs.intensity > MOOD_REST) {
        return mine;
    }
    let gain = MOOD_SPREADS[mood_index(theirs.mood)] * sociability * (0.5 + 0.5 * affinity);
    if !(gain > 0.0) {
        return mine;
    }
    if mine.mood == theirs.mood {
        let lift = gain * (theirs.intensity - mine.intensity);
        return if lift > 0.0 { Feeling { mood: mine.mood, intensity: mine.intensity + lift, since: mine.since } } else { mine };
    }
    if mine.intensity > MOOD_REST {
        return mine;
    }
    let adopted = gain * theirs.intensity;
    if adopted >= MOOD_FAINT {
        Feeling { mood: theirs.mood, intensity: adopted, since: tick }
    } else {
        mine
    }
}
//#endregion 🔖️Contagion

//#region 🔖️States
/// 🚩️ The state a pet is in and the tick it began.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Standing {
    pub state: Slug,
    pub since: Ticks,
}

/// ⌚️ Whole ticks of an authored number of seconds as every duration of this module rounds them: `floor(seconds × 64 + 0.5)`, at least 1 (also for a number that is none).
fn whole_ticks(seconds: f64) -> Ticks {
    let ticks = (seconds * TICKS_PER_SECOND as f64 + 0.5).floor();
    if ticks > 1.0 {
        ticks as Ticks
    } else {
        1
    }
}

/// 🔦️ The state of a species by its id, or `None`.
fn state_of<'a>(species: &'a Species, id: &str) -> Option<&'a SpeciesState> {
    species.states.iter().find(|state| state.id == id)
}

/// ⏱️ For how many ticks a state lasts: `floor(lasts × 64 + 0.5)`, at least 1; 0 says it lasts until something changes it.
pub fn lasting_ticks(state: &SpeciesState) -> Ticks {
    state.lasts.map_or(0, whole_ticks)
}

/// ➡️ The state a lasting state gives way to: the one `then` names, or the resting state (the first) when it names none of the species.
fn follower_of<'a>(species: &'a Species, state: &'a SpeciesState) -> &'a str {
    match &state.then {
        Some(then) if state_of(species, then).is_some() => then,
        _ => &species.states[0].id,
    }
}

/// 🕰️ The state of a pet at `tick` when it entered `state` at `since`, a closed form over any number of ticks: a state that `lasts` gives way to its `then` when its time is up, that one to its own, and so on.
///
/// A state without `lasts`, and one that would give way to itself, stays. A chain that runs in a circle is not
/// walked lap by lap: once the walk has taken as many steps as the species has states it is on the circle, whole
/// laps are taken off at once (whole ticks, exact) and the rest is walked. An id the species does not know counts as
/// its resting state.
pub fn state_at(species: &Species, state: &str, since: Ticks, tick: Ticks) -> Standing {
    let count = species.states.len();
    if count == 0 {
        return Standing { state: state.to_string(), since };
    }
    let mut current = state_of(species, state).unwrap_or(&species.states[0]);
    let mut began = since;
    for steps in 0..count + count + 2 {
        let span = lasting_ticks(current);
        if span == 0 {
            break;
        }
        let next = follower_of(species, current);
        if next == current.id || tick - began < span {
            break;
        }
        if steps == count {
            let mut lap = 0;
            let mut walker = current;
            for _ in 0..count {
                lap += lasting_ticks(walker);
                walker = state_of(species, follower_of(species, walker)).unwrap_or(&species.states[0]);
                if walker.id == current.id {
                    break;
                }
            }
            began += ((tick - began) / lap) * lap;
            continue;
        }
        began += span;
        current = state_of(species, next).unwrap_or(&species.states[0]);
    }
    Standing { state: current.id.clone(), since: began }
}

/// 🔚️ The tick at which a state entered at `since` gives way to the next one, or `None` when it stays until something changes it (also for a state the species does not have).
pub fn state_ends(species: &Species, state: &str, since: Ticks) -> Option<Ticks> {
    let current = state_of(species, state)?;
    let span = lasting_ticks(current);
    if span == 0 || follower_of(species, current) == current.id {
        None
    } else {
        Some(since + span)
    }
}

/// 🪜️ The state ladder of a species: its states in the order they are authored, from the resting state up.
pub fn ladder_of(species: &Species) -> Vec<Slug> {
    species.states.iter().map(|state| state.id.clone()).collect()
}

/// 🧗️ The rungs a trick steps along: the states it lists in `from`, in that order, when it lists any that the species has; the whole ladder otherwise. A species whose resting state lies in the middle of its ladder (a sun that can dim as well as blaze) says so here.
pub fn rungs_of(species: &Species, trick: &Trick) -> Vec<Slug> {
    let rungs: Vec<Slug> = trick.from.iter().flatten().filter(|id| state_of(species, id).is_some()).cloned().collect();
    if rungs.is_empty() {
        ladder_of(species)
    } else {
        rungs
    }
}

/// 👣️ One step along rungs: up for a positive direction, down for a negative one, held at both ends (a further turn is a flourish that changes nothing); a state that is not a rung stays.
pub fn step_rung(rungs: &[Slug], state: &str, direction: i64) -> Slug {
    let Some(index) = rungs.iter().position(|rung| rung == state) else {
        return state.to_string();
    };
    if direction == 0 {
        return state.to_string();
    }
    let next = if direction > 0 { rungs.get(index + 1) } else { index.checked_sub(1).and_then(|below| rungs.get(below)) };
    next.map_or_else(|| state.to_string(), Clone::clone)
}

/// 🔼️ One step up (positive direction) or down (negative) the [`ladder_of`] a species.
pub fn step_state(species: &Species, state: &str, direction: i64) -> Slug {
    step_rung(&ladder_of(species), state, direction)
}

/// 🎬️ The state a trick leaves a pet in: the state `to` names when the species has it; else, for a trick cued by circling, one step up its rungs (`circle`) or down (`countercircle`, when it is not also cued by `circle`); else the state it was in.
pub fn state_after_trick(species: &Species, state: &str, trick: &Trick) -> Slug {
    if let Some(to) = &trick.to {
        return if state_of(species, to).is_some() { to.clone() } else { state.to_string() };
    }
    let direction = if trick.cues.contains(&Cue::Circle) {
        1
    } else if trick.cues.contains(&Cue::Countercircle) {
        -1
    } else {
        0
    };
    if direction == 0 {
        state.to_string()
    } else {
        step_rung(&rungs_of(species, trick), state, direction)
    }
}
//#endregion 🔖️States

//#region 🔖️Tricks
/// 🙋️ Whether a pet that shows a mood performs by itself (cues `whim` and `show`), in `MOODS` order: the content, happy, playful, curious and proud do; the sleepy, grumpy, sad and scared do not. What the learner asks for is always on offer.
pub const WILLING_MOODS: [bool; 9] = [true, true, true, true, true, false, false, false, false];

/// 🎪️ The tricks of a species that a cue can set off right now, in authored order: the trick lists the cue, it is on offer in the state (`from` absent or naming it), and — for the pet's own cues `whim` and `show` — the mood it shows ([`shown_mood`]) is a willing one ([`WILLING_MOODS`]).
pub fn tricks_for<'a>(species: &'a Species, cue: Cue, state: &str, feeling: Feeling) -> Vec<&'a Trick> {
    if (cue == Cue::Whim || cue == Cue::Show) && !WILLING_MOODS[mood_index(shown_mood(feeling.mood, feeling.intensity))] {
        return Vec::new();
    }
    species.tricks.iter().filter(|trick| trick.cues.contains(&cue) && trick.from.as_ref().is_none_or(|from| from.iter().any(|id| id == state))).collect()
}

/// 👆️ The trick the n-th click of a streak sets off (`index` 0 for the first trick click): the click tricks on offer, in authored order, round and round (a negative index counts from the end); `None` when none is on offer.
pub fn click_trick<'a>(species: &'a Species, state: &str, feeling: Feeling, index: i64) -> Option<&'a Trick> {
    let tricks = tricks_for(species, Cue::Click, state, feeling);
    let count = tricks.len() as i64;
    if count == 0 {
        return None;
    }
    let turn = index % count;
    tricks.get((if turn < 0 { turn + count } else { turn }) as usize).copied()
}

/// ⭐️ How many times likelier a pet picks, among the tricks on offer, one that leaves the mood it shows already: it stays in character.
pub const WHIM_FAVOR: f64 = 3.0;

/// 🎲️ The trick a unit draw picks among those a cue can set off: [`weighted_index`] over a weight of 1 each, [`WHIM_FAVOR`] for a trick that leaves the mood the pet shows; `None` when none is on offer.
fn pick<'a>(species: &'a Species, cue: Cue, state: &str, feeling: Feeling, unit: f64) -> Option<&'a Trick> {
    let tricks = tricks_for(species, cue, state, feeling);
    let shown = shown_mood(feeling.mood, feeling.intensity);
    let weights: Vec<f64> = tricks.iter().map(|trick| if trick.mood == Some(shown) { WHIM_FAVOR } else { 1.0 }).collect();
    weighted_index(&weights, unit).map(|index| tricks[index])
}

/// 💭️ The trick a pet performs on a whim, for a unit draw: one of its `whim` tricks on offer, favouring the mood it shows; `None` when it has none or is in no mood for it.
pub fn whim_trick<'a>(species: &'a Species, state: &str, feeling: Feeling, unit: f64) -> Option<&'a Trick> {
    pick(species, Cue::Whim, state, feeling, unit)
}

/// 🎤️ The trick a pet shows another pet, for a unit draw: one of its `show` tricks on offer, favouring the mood it shows; `None` when it has none or is in no mood for it.
pub fn show_trick<'a>(species: &'a Species, state: &str, feeling: Feeling, unit: f64) -> Option<&'a Trick> {
    pick(species, Cue::Show, state, feeling, unit)
}
//#endregion 🔖️Tricks

//#region 🔖️Chemistry
/// 👀️ What the chemistry needs to know of an actor: its species, its state and for how many ticks it has held it, its mood with its intensity, its activity and the trick it performs (`None`, `null` on the wire, when none), and its body — `x` the middle and `y` the feet of a box `width` wide that rises `height` above them (viewport pixels, y pointing down).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sighting {
    pub species: Slug,
    pub state: Slug,
    pub held: Ticks,
    pub mood: Mood,
    pub intensity: f64,
    pub activity: Activity,
    pub trick: Option<Slug>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🧪️ Every how many ticks the chemistry of a stage is looked at (twice a second).
pub const CHEMISTRY_BEAT: Ticks = 32;

/// 💧️ The amount of the mood an effect gives when it names none.
pub const EFFECT_AMOUNT: f64 = 0.6;

/// ⏲️ For how many ticks the encounter a reaction promises a pair is owed (30 s): when the two meet within that time, it is the one named.
pub const LEAN_TICKS: Ticks = 1920;

/// 📐️ Whether two bodies are within `reach` pixels of each other: the gap between the two boxes (0 when they touch or overlap), across and upright taken together, is at most `reach`.
pub fn nearby(first: &Sighting, second: &Sighting, reach: f64) -> bool {
    let across = (first.x - second.x).abs() - (first.width + second.width) / 2.0;
    let under = first.y - first.height - second.y;
    let over = second.y - second.height - first.y;
    let wide = if across > 0.0 { across } else { 0.0 };
    let tall = if under > 0.0 {
        under
    } else if over > 0.0 {
        over
    } else {
        0.0
    };
    wide * wide + tall * tall <= reach * reach
}

/// 🔭️ Whether the first body is where a reaction wants it (`place`, the wire's `where`), seen from the second. `Above` and `Below`: the two share a column (their widths overlap) and the middle of the first is higher, or lower. `Beside`: they share a row (their heights overlap) but no column. `Any`: wherever it is. Bodies that share neither column nor row are diagonal to each other and only `Any` takes them.
pub fn seen(first: &Sighting, second: &Sighting, place: Placement) -> bool {
    if place == Placement::Any {
        return true;
    }
    let column = (first.x - second.x).abs() < (first.width + second.width) / 2.0;
    if place == Placement::Beside {
        return !column && first.y - first.height < second.y && second.y - second.height < first.y;
    }
    let rise = first.y - first.height / 2.0 - (second.y - second.height / 2.0);
    column && (if place == Placement::Above { rise < 0.0 } else { rise > 0.0 })
}

/// ⌛️ For how many ticks an actor must have held its state for a trait that asks `held` seconds of it: `floor(held × 64 + 0.5)`, at least 1.
pub fn held_ticks(held: f64) -> Ticks {
    whole_ticks(held)
}

/// 🔎️ Whether an actor is what one side of a reaction asks for (`wanted`, the twin's `trait`): the species (any when the trait names none), and the state — held for at least [`held_ticks`] of `held` —, the mood, the activity and the trick where the trait names them. The mood is the one the actor shows ([`shown_mood`]): `content` asks for a calm pet, any other mood for a pet that is stirred into it.
pub fn matches(wanted: &Trait, sighting: &Sighting) -> bool {
    wanted.species.as_ref().is_none_or(|species| *species == sighting.species)
        && wanted.state.as_ref().is_none_or(|state| *state == sighting.state)
        && wanted.held.is_none_or(|held| sighting.held >= held_ticks(held))
        && wanted.mood.is_none_or(|mood| mood == shown_mood(sighting.mood, sighting.intensity))
        && wanted.activity.is_none_or(|activity| activity == sighting.activity)
        && wanted.trick.as_ref().is_none_or(|trick| sighting.trick.as_ref() == Some(trick))
}

/// 🎯️ A reaction that is due for a pair: the index of the reaction in `menagerie.chemistry`, the indices of the two sightings, and whether it takes a unit draw (it has a `chance`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trial {
    pub reaction: usize,
    pub when: usize,
    pub near: usize,
    pub chancy: bool,
}

/// 💥️ What a reaction does to one actor (`on`, with `other` the second of the pair): the state it enters (`None`: none; the state it is in already begins anew), the mood it gets an impulse of and its amount, the shift of the rapport of the two, the encounter the two are promised, the trick it performs and what it is set off on by itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Consequence {
    pub reaction: Slug,
    pub on: Slug,
    pub other: Slug,
    pub state: Option<Slug>,
    pub mood: Option<Mood>,
    pub amount: f64,
    pub rapport: f64,
    pub encounter: Option<Activity>,
    pub trick: Option<Slug>,
    pub activity: Option<Activity>,
}

/// ⚗️ What one beat of chemistry yields: the consequences to apply in order, the coolings to keep for the next beat, and how many of the units were used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chemistry {
    pub consequences: Vec<Consequence>,
    pub coolings: Vec<Cooling>,
    pub drawn: usize,
}

/// 🗂️ The species of a menagerie by its id, or `None`.
fn species_of<'a>(menagerie: &'a Menagerie, id: &str) -> Option<&'a Species> {
    menagerie.species.iter().find(|species| species.id == id)
}

/// ❄️ Whether a reaction is still cooling for an ordered pair of species at `tick`.
fn cooling(coolings: &[Cooling], reaction: &str, when: &str, near: &str, tick: Ticks) -> bool {
    coolings.iter().any(|entry| entry.until > tick && entry.reaction == reaction && entry.when == when && entry.near == near)
}

/// 🔗️ Whether a list of pairs holds the two species, in either order.
fn joined(pairs: &[(&str, &str)], one: &str, other: &str) -> bool {
    pairs.iter().any(|&(first, second)| (first == one && second == other) || (first == other && second == one))
}

/// 🔁️ For how many ticks a reaction cools after its turn: `floor(every × 64 + 0.5)`, at least 1.
pub fn every_ticks(reaction: &Reaction) -> Ticks {
    whole_ticks(reaction.every)
}

/// 🚧️ Whether a third actor of `order` — neither the `when` nor the `near` actor of a trial — matches `unless` and is [`nearby`] the `near` actor within `within` pixels: then the reaction holds back.
fn barred(unless: &Trait, sightings: &[Sighting], order: &[usize], when: usize, near: usize, within: f64) -> bool {
    order.iter().any(|&third| third != when && third != near && matches(unless, &sightings[third]) && nearby(&sightings[third], &sightings[near], within))
}

/// 🧫️ The reactions that are due at `tick`, in the order their units are drawn and their effects applied: for every ordered pair of actors — the first by the place of its species in `menagerie.species`, then the second likewise — every reaction in authored order whose `when` the first and whose `near` the second [`matches`], whose bodies are [`nearby`] within `within` and [`seen`] as `where` asks (the first seen from the second), whose `affinity` bounds hold the affinity of the two ([`affinity_of`] with the `rapports` of the stage; a bound the document lacks holds nothing back), which no third actor bars ([`barred`] by `unless`), and which is not cooling for the pair. Actors of a species the menagerie does not know are passed over.
pub fn trials_of(menagerie: &Menagerie, sightings: &[Sighting], coolings: &[Cooling], tick: Ticks, rapports: &[Rapport]) -> Vec<Trial> {
    let order: Vec<usize> = menagerie.species.iter().flat_map(|species| sightings.iter().enumerate().filter(move |(_, sighting)| sighting.species == species.id).map(|(index, _)| index)).collect();
    let mut trials = Vec::new();
    for &when in &order {
        let first = &sightings[when];
        for &near in &order {
            if near == when {
                continue;
            }
            let second = &sightings[near];
            for (index, reaction) in menagerie.chemistry.iter().enumerate() {
                if !matches(&reaction.when, first) || !matches(&reaction.near, second) {
                    continue;
                }
                if !nearby(first, second, reaction.within) || !seen(first, second, reaction.place.unwrap_or(Placement::Any)) {
                    continue;
                }
                if let Some(bounds) = &reaction.affinity {
                    let affinity = affinity_of(menagerie, rapports, &first.species, &second.species);
                    if affinity < bounds.first().copied().unwrap_or(f64::NAN) || affinity > bounds.get(1).copied().unwrap_or(f64::NAN) {
                        continue;
                    }
                }
                if reaction.unless.as_ref().is_some_and(|unless| barred(unless, sightings, &order, when, near, reaction.within)) {
                    continue;
                }
                if cooling(coolings, &reaction.id, &first.species, &second.species, tick) {
                    continue;
                }
                trials.push(Trial { reaction: index, when, near, chancy: reaction.chance.is_some() });
            }
        }
    }
    trials
}

/// 🎰️ How many units the trials of a beat take: one per trial with a chance, in trial order. The stage draws that many from its stream and hands them to [`reactions_of`].
pub fn draws_of(trials: &[Trial]) -> usize {
    trials.iter().filter(|trial| trial.chancy).count()
}

/// 🔥️ One beat of chemistry: what the reactions of a menagerie do to the actors of a stage as they stand at `tick`.
///
/// Every trial of [`trials_of`] has its turn in order, and cools for its pair for [`every_ticks`] from `tick` whether
/// it happens or not — so a reaction is tried at most once in `every` seconds. A reaction with a `chance` takes the
/// next unit and happens when `unit < chance` (a unit that was not handed in counts as 1: it does not happen). Its
/// effects become consequences in authored order: the state when the species of the actor has it, the mood with its
/// amount ([`EFFECT_AMOUNT`] when none is named), the rapport, the encounter, the trick when the species has it and
/// offers it in the state the actor is in, and the activity. Everybody is judged as it stood when the beat began — a
/// state set in this beat sets nothing else off before the next one — and nothing is applied twice in a beat: an
/// actor takes the first state, the first mood, the first trick and the first activity that reach it, a pair the
/// first shift of its rapport and the first promise of an encounter. An effect of which nothing is left is dropped.
/// The coolings returned are those still running, then the new ones in trial order.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn reactions_of(menagerie: &Menagerie, sightings: &[Sighting], tick: Ticks, coolings: &[Cooling], units: &[f64], rapports: &[Rapport]) -> Chemistry {
    let trials = trials_of(menagerie, sightings, coolings, tick, rapports);
    let mut kept: Vec<Cooling> = coolings.iter().filter(|entry| entry.until > tick).cloned().collect();
    let mut consequences = Vec::new();
    let mut stated: Vec<&str> = Vec::new();
    let mut moved: Vec<&str> = Vec::new();
    let mut tricked: Vec<&str> = Vec::new();
    let mut started: Vec<&str> = Vec::new();
    let mut shifted: Vec<(&str, &str)> = Vec::new();
    let mut promised: Vec<(&str, &str)> = Vec::new();
    let mut drawn = 0;
    for trial in &trials {
        let reaction = &menagerie.chemistry[trial.reaction];
        let first = &sightings[trial.when];
        let second = &sightings[trial.near];
        kept.push(Cooling { reaction: reaction.id.clone(), when: first.species.clone(), near: second.species.clone(), until: tick + every_ticks(reaction) });
        if let Some(chance) = reaction.chance {
            let unit = units.get(drawn).copied().unwrap_or(1.0);
            drawn += 1;
            if !(unit < chance) {
                continue;
            }
        }
        for effect in &reaction.then {
            let (actor, other) = if effect.on == Party::When { (first, second) } else { (second, first) };
            let Some(species) = species_of(menagerie, &actor.species) else {
                continue;
            };
            let state = if effect.state.as_ref().is_some_and(|entered| state_of(species, entered).is_some()) && !stated.contains(&actor.species.as_str()) { effect.state.clone() } else { None };
            if state.is_some() {
                stated.push(&actor.species);
            }
            let mood = if moved.contains(&actor.species.as_str()) { None } else { effect.mood };
            if mood.is_some() {
                moved.push(&actor.species);
            }
            let shifts = effect.rapport.is_some_and(|shift| shift != 0.0) && !joined(&shifted, &actor.species, &other.species);
            let rapport = if shifts { effect.rapport.unwrap_or_default() } else { 0.0 };
            if shifts {
                shifted.push((&actor.species, &other.species));
            }
            let encounter = if joined(&promised, &actor.species, &other.species) { None } else { effect.encounter };
            if encounter.is_some() {
                promised.push((&actor.species, &other.species));
            }
            let trick = if tricked.contains(&actor.species.as_str()) {
                None
            } else {
                effect.trick.as_ref().and_then(|wanted| species.tricks.iter().find(|offered| offered.id == *wanted && offered.from.as_ref().is_none_or(|from| from.contains(&actor.state)))).map(|offered| offered.id.clone())
            };
            if trick.is_some() {
                tricked.push(&actor.species);
            }
            let activity = if started.contains(&actor.species.as_str()) { None } else { effect.activity };
            if activity.is_some() {
                started.push(&actor.species);
            }
            if state.is_none() && mood.is_none() && rapport == 0.0 && encounter.is_none() && trick.is_none() && activity.is_none() {
                continue;
            }
            let amount = if mood.is_none() { 0.0 } else { effect.amount.unwrap_or(EFFECT_AMOUNT) };
            consequences.push(Consequence { reaction: reaction.id.clone(), on: actor.species.clone(), other: other.species.clone(), state, mood, amount, rapport, encounter, trick, activity });
        }
    }
    Chemistry { consequences, coolings: kept, drawn }
}
//#endregion 🔖️Chemistry

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
