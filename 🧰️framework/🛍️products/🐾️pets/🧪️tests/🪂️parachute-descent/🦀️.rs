//! 🪂️ Subject adapter of the parachute-descent case: the parachute of the swing module of the `pets` crate answers every committed fall, opens every committed canopy and lands every committed drop tick by tick, scenario for scenario the projections of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors
//! @see ../../🔨️modules/🪢️swing/🦀️.rs
//! @see ../../🔨️modules/🏞️terrain/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        canopy_of, chute_of, chute_opens, chute_step, chute_wind, fall_step, flare_of, impact_speed, Canopy, Point, Ticks, CHUTE_DAMPING, CHUTE_DESCENT, CHUTE_FACTOR, CHUTE_FLARE, CHUTE_GRAVITY, CHUTE_HEADROOM, CHUTE_OPENING, CHUTE_REFLEX,
        CHUTE_ROD, CHUTE_STEER_EASE, CHUTE_STEER_GAIN, CHUTE_STEER_SPEED, CHUTE_WIND, CHUTE_WIND_RATE, FALL_SPEED, GRAVITY, HARD_LANDING,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪂️parachute-descent/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 📚️ A list the document carries.
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

    /// 📍️ A point member of a vector.
    fn point(vector: &Value, name: &str) -> Result<Point, String> {
        serde_json::from_value(vector[name].clone()).map_err(|error| format!("{name}: {error}"))
    }

    /// ⏱️ The committed ticks of a vector, 1 first.
    fn ticks(vector: &Value) -> Result<Vec<usize>, String> {
        serde_json::from_value(vector["ticks"].clone()).map_err(|error| format!("ticks: {error}"))
    }

    /// 🎯️ The entries of a list of states at the committed ticks.
    fn at<T: Clone>(states: &[T], ticks: &[usize]) -> Result<Vec<T>, String> {
        ticks.iter().map(|&tick| tick.checked_sub(1).and_then(|index| states.get(index)).cloned().ok_or_else(|| format!("no state at tick {tick}"))).collect()
    }

    /// 🔝️ The last committed tick.
    fn latest(ticks: &[usize]) -> usize {
        ticks.iter().copied().max().unwrap_or_default()
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

    /// 🎚️ The tuning constants of a fall and of the parachute.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "gravity": GRAVITY, "fallSpeed": FALL_SPEED, "hardLanding": HARD_LANDING, "chuteOpening": CHUTE_OPENING, "chuteHeadroom": CHUTE_HEADROOM, "chuteReflex": CHUTE_REFLEX, "chuteFactor": CHUTE_FACTOR, "chuteDescent": CHUTE_DESCENT, "chuteSteerGain": CHUTE_STEER_GAIN, "chuteSteerSpeed": CHUTE_STEER_SPEED, "chuteSteerEase": CHUTE_STEER_EASE, "chuteRod": CHUTE_ROD, "chuteGravity": CHUTE_GRAVITY, "chuteDamping": CHUTE_DAMPING, "chuteFlare": CHUTE_FLARE, "chuteWind": CHUTE_WIND, "chuteWindRate": CHUTE_WIND_RATE }),
        )
    }

    /// ☄️ The predicted impact of every committed fall.
    pub fn impact_speeds(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "impacts")?, |vector| Ok(json!(impact_speed(number(vector, "vy")?, number(vector, "height")?))))?)
    }

    /// 🚪️ Whether every committed fall opens its parachute now.
    pub fn chute_triggers(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "triggers")?, |vector| Ok(json!(chute_opens(number(vector, "vy")?, number(vector, "height")?))))?)
    }

    /// 🧮️ The measures of the parachute of every committed height.
    pub fn chute_measures(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "measures")?, |vector| Ok(json!(chute_of(number(vector, "height")?))))?)
    }

    /// 🦅️ The share of its descent every committed canopy keeps above the landing.
    pub fn flares(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "flares")?, |vector| Ok(json!(flare_of(number(vector, "remaining")?, number(vector, "flare")?))))?)
    }

    /// 🌬️ The wind of every committed tick and phase.
    pub fn winds(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "winds")?, |vector| {
            let ticks: Ticks = serde_json::from_value(vector["ticks"].clone()).map_err(|error| format!("ticks: {error}"))?;
            Ok(json!(chute_wind(ticks, number(vector, "phase")?)))
        })?)
    }

    /// ⛱️ A canopy opened above every committed pet and advanced to the last committed tick: its place, its velocity and the feet at the committed ticks.
    pub fn canopy_descents(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "descents")?, |vector| {
            let chute = chute_of(number(vector, "height")?);
            let (feet, ticks) = (point(vector, "feet")?, ticks(vector)?);
            let (target, remaining) = (number(vector, "target")?, number(vector, "remaining")?);
            let phase = if vector["phase"].is_null() { None } else { Some(number(vector, "phase")?) };
            let mut canopy = canopy_of(feet, number(vector, "vx")?, number(vector, "vy")?, chute);
            let mut states = Vec::new();
            for tick in 1..=latest(&ticks) {
                canopy = chute_step(canopy, chute, target, remaining - (canopy.bob.y - feet.y), phase.map_or(0.0, |phase| chute_wind(tick as Ticks, phase)));
                states.push(json!({ "x": canopy.x, "y": canopy.y, "vx": canopy.vx, "vy": canopy.vy, "bob": canopy.bob }));
            }
            Ok(json!(at(&states, &ticks)?))
        })?)
    }

    /// 🎐️ The feet of every committed swaying pet below a canopy that descends at its terminal speed, at the committed ticks.
    pub fn canopy_sways(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "sways")?, |vector| {
            let chute = chute_of(number(vector, "height")?);
            let (start, ticks) = (point(vector, "canopy")?, ticks(vector)?);
            let (target, remaining) = (number(vector, "target")?, number(vector, "remaining")?);
            let mut canopy = Canopy { x: start.x, y: start.y, vx: number(vector, "vx")?, vy: chute.terminal, bob: point(vector, "bob")?, previous: point(vector, "previous")? };
            let mut feet = Vec::new();
            for _ in 0..latest(&ticks) {
                canopy = chute_step(canopy, chute, target, remaining, 0.0);
                feet.push(canopy.bob);
            }
            Ok(json!(at(&feet, &ticks)?))
        })?)
    }

    /// 🪨️ Every committed drop as the stage takes it: fall and ask `chute_opens`; `CHUTE_REFLEX` ticks after the decision open the canopy and descend under it; stop at the tick the feet reach the landing.
    pub fn drops(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "drops")?, |vector| {
            let chute = chute_of(number(vector, "height")?);
            let (drop, start) = (number(vector, "drop")?, number(vector, "vy")?);
            let opens = vector.get("chute").and_then(Value::as_bool).ok_or_else(|| format!("vector {vector} carries no flag chute"))?;
            let (mut y, mut vy, mut opened, mut tick, mut touch): (f64, f64, Ticks, Ticks, f64) = (0.0, start, 0, 0, 0.0);
            let mut canopy: Option<Canopy> = None;
            while y < drop {
                tick += 1;
                let remaining = drop - y;
                if canopy.is_none() {
                    if opened == 0 && opens && chute_opens(vy, remaining) {
                        opened = tick;
                    }
                    if opened != 0 && tick - opened >= CHUTE_REFLEX {
                        canopy = Some(canopy_of(Point { x: 0.0, y }, 0.0, vy, chute));
                    }
                }
                match canopy {
                    None => {
                        let fall = fall_step(y, vy);
                        y = fall.y;
                        vy = fall.vy;
                        touch = vy;
                    }
                    Some(open) => {
                        let moved = chute_step(open, chute, 0.0, remaining, 0.0);
                        touch = (moved.y - open.y) * 64.0;
                        canopy = Some(moved);
                        y = moved.bob.y;
                        vy = moved.vy;
                    }
                }
            }
            Ok(json!({ "opened": opened, "landed": tick, "touch": touch, "glide": vy, "plain": impact_speed(start, drop) }))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("impact-speeds", subject::impact_speeds)
        .subject("chute-triggers", subject::chute_triggers)
        .subject("chute-measures", subject::chute_measures)
        .subject("flares", subject::flares)
        .subject("winds", subject::winds)
        .subject("canopy-descents", subject::canopy_descents)
        .subject("canopy-sways", subject::canopy_sways)
        .subject("drops", subject::drops);
    built
}
