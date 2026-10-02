//! 🎪️ Subject adapter of the stage-trace case: the stage of the `pets` crate replays every committed script tick by tick, digests every frame and holds itself to the committed trace and to the laws of the stage.
//!
//! The digest is FNV-1a (32 bit) over the IEEE-754 bit patterns (little-endian doubles) of every number of every
//! frame, in this order: tick, rate, wake (−1 for none), the number of actors, then per actor in frame order the
//! index of its species in the menagerie, x, y, facing, the index of its activity in `ACTIVITIES`, opacity, every
//! bone number, per eye x, y and lid, and mood. It runs on from frame to frame, so a checkpoint vouches for every
//! frame before it. The TypeScript twin recorded the committed digests: agreeing with them at every checkpoint is
//! agreeing with it bit for bit.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors, scenario for scenario
//! @see ../../🔨️modules/🎪️stage/🦀️.rs
//! @see ../../🔨️modules/🧠️behavior/🦀️.rs — `MODE_LIMITS`, `followers_of`

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{advance, followers_of, frame_of, open_stage, Activity, Frame, Gait, Menagerie, Rate, Stage, StageEvent, Ticked, Ticks, ACTIVITIES, HOP_TICKS, MODE_LIMITS};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::collections::BTreeMap;

    const VECTORS: &str = "shared://🎪️stage-trace/🔣️.json";
    const FNV_OFFSET_BASIS: u32 = 2_166_136_261;
    const FNV_PRIME: u32 = 16_777_619;
    const CHUNKS: [Ticks; 8] = [1, 7, 64, 3, 500, 2, 19, 1000];
    const CONTACT: f64 = 0.0078125;

    /// 📜️ A scripted event log: the seed of the stage, how many ticks pass, how often a checkpoint is taken, the events by the tick they happen at (before that tick passes), and the committed trace.
    struct Script {
        id: String,
        seed: u32,
        ticks: Ticks,
        every: Ticks,
        events: BTreeMap<Ticks, Vec<StageEvent>>,
        expected: Value,
    }

    /// ⚖️ The laws every tick of every script keeps: grounded actors stand on perches (`perched`) outside every keep-out (`clear`), no two grounded actors of one surface stand in each other — their feet are at least half of both widths apart, within 1/128 px (`apart`) —, no more actors walk or hop than the mode allows (`paced`), at most one pair has partners (`paired`), activities follow the activity graph (`graphed`), and frames are well-formed — opacity in [0, 1], a known rate (the type `Rate` holds no other), a wake tick only at rate 0 and in the future, actors back to front (`whole`).
    #[derive(Clone, Copy, PartialEq)]
    struct Laws {
        perched: bool,
        clear: bool,
        apart: bool,
        paced: bool,
        paired: bool,
        graphed: bool,
        whole: bool,
    }

    /// 🎞️ The trace of a script as it is projected (frames, digest, checkpoints, laws), its digest, its laws and the stage it ends in.
    #[derive(PartialEq)]
    struct Replay {
        trace: Value,
        digest: u32,
        laws: Laws,
        stage: Stage,
    }

    /// 🧫️ The committed vectors: the menagerie and the scripts.
    fn vectors(ctx: &Context<'_>) -> Result<(Menagerie, Vec<Script>), String> {
        let document: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        let menagerie: Menagerie = serde_json::from_value(document["menagerie"].clone()).map_err(|error| format!("menagerie: {error}"))?;
        let mut scripts = Vec::new();
        for script in document["scripts"].as_array().ok_or_else(|| format!("{VECTORS} carries no scripts"))? {
            let whole = |field: &str| script[field].as_i64().ok_or_else(|| format!("script {} carries no whole number {field}", script["id"]));
            let mut events: BTreeMap<Ticks, Vec<StageEvent>> = BTreeMap::new();
            for step in script["steps"].as_array().ok_or_else(|| format!("script {} carries no steps", script["id"]))? {
                let at = step["at"].as_i64().ok_or_else(|| format!("step {step} carries no tick"))?;
                events.entry(at).or_default().extend(serde_json::from_value::<Vec<StageEvent>>(step["events"].clone()).map_err(|error| format!("events at {at}: {error}"))?);
            }
            scripts.push(Script {
                id: script["id"].as_str().ok_or("a script carries no id")?.to_string(),
                seed: u32::try_from(whole("seed")?).map_err(|error| format!("seed: {error}"))?,
                ticks: whole("ticks")?,
                every: whole("every")?,
                events,
                expected: script["expected"].clone(),
            });
        }
        Ok((menagerie, scripts))
    }

    /// 🔢️ FNV-1a over the eight bytes of a double, little-endian.
    fn fold(hash: u32, value: f64) -> u32 {
        value.to_le_bytes().into_iter().fold(hash, |folded, byte| (folded ^ u32::from(byte)).wrapping_mul(FNV_PRIME))
    }

    /// 🧮️ The digest after one more frame.
    fn fold_frame(hash: u32, menagerie: &Menagerie, frame: &Frame) -> u32 {
        let mut folded = [frame.tick as f64, f64::from(u8::from(frame.rate)), frame.wake.map_or(-1.0, |wake| wake as f64), frame.actors.len() as f64].into_iter().fold(hash, fold);
        for actor in &frame.actors {
            folded = fold(folded, menagerie.species.iter().position(|species| species.id == actor.species).map_or(-1.0, |index| index as f64));
            folded = [actor.x, actor.y, actor.facing.sign(), ACTIVITIES.iter().position(|activity| *activity == actor.activity).map_or(-1.0, |index| index as f64), actor.opacity].into_iter().fold(folded, fold);
            folded = actor.bones.iter().copied().fold(folded, fold);
            folded = actor.eyes.iter().flat_map(|eye| [eye.x, eye.y, eye.lid]).fold(folded, fold);
            folded = fold(folded, actor.mood);
        }
        folded
    }

    /// 🎭️ What every actor on stage does, by species: what a later stage is held against for the activity graph.
    fn doings(stage: &Stage) -> Vec<(String, Activity)> {
        stage.actors.iter().map(|actor| (actor.species.clone(), actor.activity)).collect()
    }

    /// 🕸️ Whether every actor that is on stage before and after changed its activity along the activity graph.
    fn graphed(before: &[(String, Activity)], after: &Stage) -> bool {
        after.actors.iter().all(|actor| before.iter().find(|(species, _)| *species == actor.species).is_none_or(|&(_, earlier)| earlier == actor.activity || followers_of(earlier).contains(&actor.activity)))
    }

    /// 🔤️ Whether one text comes before another by UTF-16 code units, as the TypeScript twin compares strings.
    fn before(one: &str, other: &str) -> bool {
        one.encode_utf16().lt(other.encode_utf16())
    }

    /// 🧾️ The laws of one stage and its frame, given the tick of the last change of mode.
    fn laws_of(menagerie: &Menagerie, stage: &Stage, frame: &Frame, tuned: Ticks) -> Result<Laws, String> {
        let mut perched = true;
        let mut clear = true;
        let mut whole = frame.wake.is_none_or(|wake| frame.rate == Rate::Rest && wake > frame.tick) && frame.tick == stage.tick && frame.actors.len() == stage.actors.len();
        let mut movers = 0;
        let mut partners = 0;
        for actor in &stage.actors {
            let species = menagerie.species.iter().find(|candidate| candidate.id == actor.species).ok_or_else(|| format!("{} is on stage and not in the menagerie", actor.species))?;
            let hover = if species.locomotion.gait == Gait::Float { species.locomotion.hover.unwrap_or(0.0) } else { 0.0 };
            if matches!(actor.activity, Activity::Walk | Activity::Hop) && !actor.leaving {
                movers += 1;
            }
            if let Some(partner) = &actor.partner {
                partners += 1;
                if !stage.actors.iter().any(|candidate| candidate.species == *partner) {
                    partners += 2;
                }
            }
            if !(0.0..=1.0).contains(&actor.opacity) {
                whole = false;
            }
            let Some(surface) = &actor.perch else {
                continue;
            };
            if !stage.perches.iter().any(|perch| perch.surface == *surface && perch.x0 <= actor.x && actor.x <= perch.x1 && actor.y == perch.y - hover) {
                perched = false;
            }
            let left = actor.x - species.size.width / 2.0;
            let right = actor.x + species.size.width / 2.0;
            let top = actor.y - species.size.height;
            if stage.keepouts.iter().any(|keepout| keepout.width > 0.0 && keepout.height > 0.0 && left.max(keepout.x) < right.min(keepout.x + keepout.width) && top.max(keepout.y) < actor.y.min(keepout.y + keepout.height)) {
                clear = false;
            }
            if actor.x < 0.0 || actor.x > stage.width {
                whole = false;
            }
        }
        for pair in frame.actors.windows(2) {
            if pair[0].y > pair[1].y || (pair[0].y == pair[1].y && !before(&pair[0].species, &pair[1].species)) {
                whole = false;
            }
        }
        let mut apart = true;
        for one in &stage.actors {
            for two in &stage.actors {
                if one.perch.is_none() || one.perch != two.perch || !before(&one.species, &two.species) {
                    continue;
                }
                let shoulders = (width_of(menagerie, &one.species)? + width_of(menagerie, &two.species)?) / 2.0;
                if (one.x - two.x).abs() < shoulders - CONTACT {
                    apart = false;
                }
            }
        }
        Ok(Laws { perched, clear, apart, paced: movers <= MODE_LIMITS[stage.mode].movers || stage.tick - tuned <= HOP_TICKS + 1, paired: partners <= 2, graphed: true, whole })
    }

    /// ↔️ The width of the species of an actor.
    fn width_of(menagerie: &Menagerie, species: &str) -> Result<f64, String> {
        menagerie.species.iter().find(|candidate| candidate.id == species).map(|kind| kind.size.width).ok_or_else(|| format!("{species} is on stage and not in the menagerie"))
    }

    /// 🎬️ The trace of a script over a menagerie, replayed tick by tick from an empty stage with a seed: the events of a tick are folded first, then the tick passes, then the frame is digested and the laws are checked.
    fn trace_of(menagerie: &Menagerie, script: &Script, seed: u32) -> Result<Replay, String> {
        let mut checkpoints = Vec::new();
        let mut stage = open_stage(seed);
        let mut hash = FNV_OFFSET_BASIS;
        let mut tuned = -HOP_TICKS - 2;
        let mut laws = Laws { perched: true, clear: true, apart: true, paced: true, paired: true, graphed: true, whole: true };
        for tick in 1..=script.ticks {
            let events = script.events.get(&(tick - 1)).map_or(&[][..], Vec::as_slice);
            if events.iter().any(|event| matches!(event, StageEvent::Tuned(_))) {
                tuned = tick - 1;
            }
            let earlier = doings(&stage);
            let evented = advance(menagerie, stage, events);
            let stepwise = graphed(&earlier, &evented);
            let earlier = doings(&evented);
            let ticked = advance(menagerie, evented, &[StageEvent::Ticked(Ticked { ticks: 1 })]);
            let frame = frame_of(menagerie, &ticked);
            let kept = laws_of(menagerie, &ticked, &frame, tuned)?;
            laws = Laws {
                perched: laws.perched && kept.perched,
                clear: laws.clear && kept.clear,
                apart: laws.apart && kept.apart,
                paced: laws.paced && kept.paced,
                paired: laws.paired && kept.paired,
                graphed: laws.graphed && stepwise && graphed(&earlier, &ticked),
                whole: laws.whole && kept.whole,
            };
            hash = fold_frame(hash, menagerie, &frame);
            stage = ticked;
            if tick % script.every != 0 && tick != script.ticks {
                continue;
            }
            let actors: Vec<Value> = stage
                .actors
                .iter()
                .map(|actor| json!({ "species": actor.species, "perch": actor.perch, "x": actor.x, "y": actor.y, "activity": actor.activity, "partner": actor.partner, "leaving": actor.leaving, "opacity": actor.opacity }))
                .collect();
            checkpoints.push(json!({ "tick": tick, "digest": hash, "mode": stage.mode, "actors": actors }));
        }
        Ok(Replay { trace: json!({ "frames": script.ticks, "digest": hash, "checkpoints": checkpoints, "laws": lawful(laws) }), digest: hash, laws, stage })
    }

    /// 🪧️ The laws as they are projected.
    fn lawful(laws: Laws) -> Value {
        json!({ "perched": laws.perched, "clear": laws.clear, "apart": laws.apart, "paced": laws.paced, "paired": laws.paired, "graphed": laws.graphed, "whole": laws.whole })
    }

    /// ✂️ The stage a script ends in when its ticks are not passed one by one but in the uneven chunks of `CHUNKS`, as far as the events allow.
    fn chunked_stage(menagerie: &Menagerie, script: &Script) -> Stage {
        let mut stage = open_stage(script.seed);
        let mut tick = 0;
        let mut turn = 0;
        while tick < script.ticks {
            stage = advance(menagerie, stage, script.events.get(&tick).map_or(&[][..], Vec::as_slice));
            let next = script.events.range(tick + 1..script.ticks).next().map_or(script.ticks, |(&at, _)| at);
            let ticks = Ticks::min(CHUNKS[turn % CHUNKS.len()], next - tick);
            stage = advance(menagerie, stage, &[StageEvent::Ticked(Ticked { ticks: ticks.unsigned_abs() })]);
            tick += ticks;
            turn += 1;
        }
        stage
    }

    /// 🔍️ The first place a replayed trace differs from the committed one, numbers compared as doubles; `None` when they are the same.
    fn difference(path: &str, replayed: &Value, committed: &Value) -> Option<String> {
        match (replayed, committed) {
            (Value::Number(left), Value::Number(right)) => (left.as_f64() != right.as_f64()).then(|| format!("{path}: {left} ≠ {right}")),
            (Value::Array(left), Value::Array(right)) => {
                if left.len() != right.len() {
                    return Some(format!("{path}: {} ≠ {} entries", left.len(), right.len()));
                }
                left.iter().zip(right).enumerate().find_map(|(index, (left, right))| difference(&format!("{path}/{index}"), left, right))
            }
            (Value::Object(left), Value::Object(right)) => {
                if left.len() != right.len() {
                    return Some(format!("{path}: {} ≠ {} members", left.len(), right.len()));
                }
                left.iter().find_map(|(key, value)| right.get(key).map_or_else(|| Some(format!("{path}/{key}: missing in the committed trace")), |other| difference(&format!("{path}/{key}"), value, other)))
            }
            _ => (replayed != committed).then(|| format!("{path}: {replayed} ≠ {committed}")),
        }
    }

    /// 📏️ Holds a replayed trace to the committed one, exactly.
    fn conforming(script: &Script, replay: Replay) -> Result<Value, String> {
        match difference(&format!("stage-trace/{}", script.id), &replay.trace, &script.expected) {
            Some(found) => Err(format!("the replayed trace differs from the committed one at {found}")),
            None => Ok(replay.trace),
        }
    }

    /// 📤️ A projection keyed by script id, as the host carries it.
    fn keyed(ctx: &Context<'_>, answer: impl Fn(&Menagerie, &Script) -> Result<Value, String>) -> Result<Outcome, String> {
        let (menagerie, scripts) = vectors(ctx)?;
        let mut projection = Map::new();
        for script in &scripts {
            projection.insert(script.id.clone(), answer(&menagerie, script)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🎥️ Every scripted event log replays into its committed trace.
    pub fn traces(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(ctx, |menagerie, script| conforming(script, trace_of(menagerie, script, script.seed)?))
    }

    /// 🛡️ Every tick of every script keeps the laws of the stage.
    pub fn laws(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(ctx, |menagerie, script| Ok(lawful(trace_of(menagerie, script, script.seed)?.laws)))
    }

    /// 🧬️ The same seed and events yield the same frames however time is cut.
    pub fn determinism(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(ctx, |menagerie, script| {
            let first = trace_of(menagerie, script, script.seed)?;
            let again = trace_of(menagerie, script, script.seed)?;
            let other = trace_of(menagerie, script, script.seed.wrapping_add(1))?;
            Ok(json!({ "repeatable": again == first, "seeded": other.digest != first.digest, "chunked": chunked_stage(menagerie, script) == first.stage }))
        })
    }
}

/// 🧪️ Subject role only — the case has no oracle (recorded decision `pets-stage-trace`); the two cores are held to the committed traces and to each other.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("traces", subject::traces).subject("laws", subject::laws).subject("determinism", subject::determinism);
    built
}
