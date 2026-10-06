//! ✨️ Subject adapter of the particle-motion case: the effects module of the `pets` crate answers every committed vector, scenario for scenario and in the projection shapes of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, so the subject is given the very doubles the TypeScript
//! adapter is given and both project the same bits. Words travel as unsigned 32-bit integers (a committed number is
//! read modulo 2³², as the twin's `>>> 0` reads it); where the TypeScript twin answers `null` (an emitter without an
//! end) the crate answers `None`, which projects as `null`.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, whose projections these are
//! @see ../../🔨️modules/✨️effects/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{born_at, capped, emitter_ends, life_ticks, lowbias32, mix, particles_of, period_of, swarm_of, unit, Emission, Facing, Particle, Point, Ticks};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://✨️particle-motion/🔣️.json";
    const BINS: usize = 16;

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context<'_>, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🗝️ The projection keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors {
            let id = vector.get("id").and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no id"))?;
            projection.insert(id.to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🧩️ A committed number as an unsigned 32-bit word: its whole part modulo 2³².
    fn word(value: &Value) -> Result<u32, String> {
        value.as_f64().map(|number| number as i64 as u32).ok_or_else(|| format!("{value} is no number"))
    }

    /// ⏱️ A whole number of ticks a member of a vector names.
    fn tick(vector: &Value, field: &str) -> Result<Ticks, String> {
        vector[field].as_f64().map(|number| number as Ticks).ok_or_else(|| format!("vector {vector} carries no tick {field}"))
    }

    /// 🏁️ The tick an emitter stopped, `None` while it runs (`null`).
    fn until(vector: &Value) -> Result<Option<Ticks>, String> {
        if vector["until"].is_null() {
            Ok(None)
        } else {
            tick(vector, "until").map(Some)
        }
    }

    /// 🚿️ The emitter a vector carries.
    fn emission(vector: &Value) -> Result<Emission, String> {
        serde_json::from_value(vector["emitter"].clone()).map_err(|error| format!("emitter: {error}"))
    }

    /// 📊️ The sum of the words and the histogram of the units a lane yields for `count` indices from `first` on, over sixteen equal bins.
    fn spread(vector: &Value) -> Result<Value, String> {
        let (key, lane, first, count) = (word(&vector["key"])?, word(&vector["lane"])?, tick(vector, "first")?, tick(vector, "count")?);
        let mut bins = [0_u64; BINS];
        let mut total = 0.0;
        for index in first..first + count {
            let drawn = mix(mix(key, index as u32), lane);
            total += f64::from(drawn);
            bins[(unit(drawn) * BINS as f64).floor() as usize] += 1;
        }
        Ok(json!({ "total": total, "bins": bins }))
    }

    /// 🗓️ The births of the first particles and, tick by tick, the ages of those alive, eldest first.
    fn lifetimes(vector: &Value) -> Result<Value, String> {
        let emitted = emission(vector)?;
        let (since, stop, key) = (tick(vector, "since")?, until(vector)?, word(&vector["key"])?);
        let ages: Vec<Vec<Ticks>> = (tick(vector, "first")?..=tick(vector, "last")?).map(|at| particles_of(emitted, Point { x: 0.0, y: 0.0 }, Facing::Right, since, stop, at, key).iter().map(|particle| particle.age).collect()).collect();
        let born: Vec<Ticks> = (0..tick(vector, "births")?).map(|index| born_at(emitted, since, key, index)).collect();
        Ok(json!({ "life": life_ticks(emitted), "swarm": swarm_of(emitted), "period": period_of(emitted), "born": born, "ages": ages }))
    }

    /// 🎇️ The particles of an emitter at every committed tick.
    fn placed(vector: &Value) -> Result<Value, String> {
        let emitted = emission(vector)?;
        let origin: Point = serde_json::from_value(vector["origin"].clone()).map_err(|error| format!("origin: {error}"))?;
        let facing: Facing = serde_json::from_value(vector["facing"].clone()).map_err(|error| format!("facing: {error}"))?;
        let (since, stop, key) = (tick(vector, "since")?, until(vector)?, word(&vector["key"])?);
        let ticks: Vec<Ticks> = vector["ticks"].as_array().ok_or("a motion vector carries no ticks")?.iter().map(|at| at.as_f64().map(|number| number as Ticks).ok_or("a tick is no number")).collect::<Result<_, _>>()?;
        Ok(json!(ticks.iter().map(|&at| particles_of(emitted, origin, facing, since, stop, at, key)).collect::<Vec<_>>()))
    }

    /// ✂️ The positions of the particles that survive a cap, the particles being known by their ages alone; a cap that is not a whole number is floored and one below 1 keeps nothing, as the twin's `Math.floor` and `room > 0` read it.
    fn kept(vector: &Value) -> Result<Value, String> {
        let ages: Vec<Ticks> = serde_json::from_value(vector["ages"].clone()).map_err(|error| format!("ages: {error}"))?;
        let room = vector["cap"].as_f64().ok_or("a cap vector carries no cap")?.floor();
        let particles: Vec<Particle> = ages.iter().enumerate().map(|(position, &age)| Particle { x: position as f64, y: 0.0, scale: 1.0, rotation: 0.0, opacity: 1.0, age }).collect();
        Ok(json!(capped(particles, if room > 0.0 { room as usize } else { 0 }).iter().map(|particle| particle.x).collect::<Vec<_>>()))
    }

    /// 🧂️ `lowbias32` of every committed word and `mix` folded over every committed chain.
    pub fn hashes(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "hashes")?, |vector| {
            if !vector["low"].is_null() {
                return Ok(json!(lowbias32(word(&vector["low"])?)));
            }
            let chain: Vec<u32> = vector["chain"].as_array().ok_or("a hash vector carries neither low nor chain")?.iter().map(word).collect::<Result<_, _>>()?;
            let (&first, rest) = chain.split_first().ok_or("an empty chain")?;
            Ok(json!(rest.iter().fold(first, |folded, &link| mix(folded, link))))
        })
    }

    /// 📈️ The sums and histograms of every committed run.
    pub fn uniformity(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "uniformity")?, spread)
    }

    /// 🐣️ Births and lives of every committed emitter.
    pub fn births(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "births")?, lifetimes)
    }

    /// 🌠️ The particles of every committed emitter at its ticks.
    pub fn motions(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "motions")?, placed)
    }

    /// 🧢️ The survivors of every committed cap.
    pub fn caps(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "caps")?, kept)
    }

    /// 🔚️ The end of every committed emitter.
    pub fn ends(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "ends")?, |vector| Ok(json!(emitter_ends(emission(vector)?, tick(vector, "since")?, until(vector)?))))
    }
}

/// 🧪️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("hashes", subject::hashes).subject("uniformity", subject::uniformity).subject("births", subject::births).subject("motions", subject::motions).subject("caps", subject::caps).subject("ends", subject::ends);
    built
}
