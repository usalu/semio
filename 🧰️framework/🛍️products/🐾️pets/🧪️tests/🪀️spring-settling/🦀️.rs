//! 🪀️ Subject adapter of the spring-settling case: `spring_step` and the gaze constants of the `pets` crate answer every committed vector by single ticks.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🎞️animation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{spring_step, Spring, Ticks, GAZE_DAMPING, GAZE_STIFFNESS};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪀️spring-settling/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 📚️ A list the document or one of its vectors carries.
    fn list<'a>(holder: &'a Value, name: &str) -> Result<&'a [Value], String> {
        holder.get(name).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("{VECTORS} carries no list {name}"))
    }

    /// 🔢️ A number member of a vector.
    fn number(vector: &Value, name: &str) -> Result<f64, String> {
        vector.get(name).and_then(Value::as_f64).ok_or_else(|| format!("vector {vector} carries no number {name}"))
    }

    /// 🔡️ A text member of a vector.
    fn text<'a>(vector: &'a Value, name: &str) -> Result<&'a str, String> {
        vector.get(name).and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no text {name}"))
    }

    /// 📤️ A projection as the host carries it.
    fn projected(projection: &Value) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&projection.to_string())?))
    }

    /// 🗝️ A group of vectors answered one by one, keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Value, String> {
        let mut projection = Map::new();
        for vector in vectors {
            projection.insert(text(vector, "id")?.to_string(), answer(vector)?);
        }
        Ok(Value::Object(projection))
    }

    /// 🌀️ A committed spring: its state when released, its target, its stiffness and its damping.
    fn released(vector: &Value) -> Result<(Spring, f64, f64, f64), String> {
        Ok((Spring { position: number(vector, "position")?, velocity: number(vector, "velocity")? }, number(vector, "target")?, number(vector, "stiffness")?, number(vector, "damping")?))
    }

    /// ⏭️ A spring after `ticks` single steps.
    fn after(spring: Spring, target: f64, stiffness: f64, damping: f64, ticks: Ticks) -> Spring {
        (0..ticks).fold(spring, |state, _| spring_step(state.position, state.velocity, target, stiffness, damping))
    }

    /// 🎯️ From which tick a resting pupil stays within the band around its new target, and its largest overshoot, both as shares of the way.
    fn settling(band: f64, horizon: Ticks, position: f64, target: f64) -> Value {
        let travel = target - position;
        let mut state = Spring { position, velocity: 0.0 };
        let mut settled = 1;
        let mut overshoot = 0.0;
        for tick in 1..=horizon {
            state = spring_step(state.position, state.velocity, target, GAZE_STIFFNESS, GAZE_DAMPING);
            let share = (state.position - target) / travel;
            if share.abs() > band {
                settled = tick + 1;
            }
            if share > overshoot {
                overshoot = share;
            }
        }
        json!({ "settled": settled, "overshoot": overshoot })
    }

    /// 🦶️ Every committed spring after one tick.
    pub fn single_steps(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "steps")?, |vector| {
            let (spring, target, stiffness, damping) = released(vector)?;
            Ok(json!(after(spring, target, stiffness, damping, 1)))
        })?)
    }

    /// 🏃️ Every committed spring after each of its committed numbers of ticks.
    pub fn tick_runs(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "runs")?, |vector| {
            let (spring, target, stiffness, damping) = released(vector)?;
            let reached = list(vector, "ticks")?.iter().map(|ticks| ticks.as_f64().map(|ticks| after(spring, target, stiffness, damping, ticks as Ticks)).ok_or_else(|| format!("{ticks} is not a number"))).collect::<Result<Vec<_>, _>>()?;
            Ok(json!(reached))
        })?)
    }

    /// 🛌️ Every committed spring that rests on its target, after its committed ticks.
    pub fn resting(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "rests")?, |vector| {
            let target = number(vector, "target")?;
            Ok(json!(after(Spring { position: target, velocity: 0.0 }, target, number(vector, "stiffness")?, number(vector, "damping")?, number(vector, "ticks")? as Ticks)))
        })?)
    }

    /// 🍃️ Every committed spring inside the stability region, after its committed ticks.
    pub fn stable_springs(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "stables")?, |vector| {
            let (spring, target, stiffness, damping) = released(vector)?;
            Ok(json!(after(spring, target, stiffness, damping, number(vector, "ticks")? as Ticks)))
        })?)
    }

    /// 🔭️ The gaze constants and how every committed jump of the gaze settles.
    pub fn gaze_settling(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        let gaze = &document["gaze"];
        let (band, horizon) = (number(gaze, "band")?, number(gaze, "horizon")? as Ticks);
        let jumps = keyed(list(gaze, "jumps")?, |jump| Ok(settling(band, horizon, number(jump, "position")?, number(jump, "target")?)))?;
        projected(&json!({ "stiffness": GAZE_STIFFNESS, "damping": GAZE_DAMPING, "jumps": jumps }))
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("single-steps", subject::single_steps).subject("tick-runs", subject::tick_runs).subject("resting", subject::resting).subject("stable-springs", subject::stable_springs).subject("gaze-settling", subject::gaze_settling);
    built
}
