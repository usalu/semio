//! 🦘️ Subject adapter of the hop-ballistics case: the constants, `fall_step`, `landing_of`, `hop_of`, `hop_step` and `hop_landing` of the `pets` crate answer every committed vector tick by tick.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🏞️terrain/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{fall_step, hop_landing, hop_of, hop_step, landing_of, Fall, Flight, Hop, Perch, Point, Ticks, FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🦘️hop-ballistics/🔣️.json";

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

    /// 🪺️ The perches a vector carries.
    fn perches(vector: &Value) -> Result<Vec<Perch>, String> {
        serde_json::from_value(vector["perches"].clone()).map_err(|error| error.to_string())
    }

    /// 📍️ A point member of a vector.
    fn point(vector: &Value, name: &str) -> Result<Point, String> {
        serde_json::from_value(vector[name].clone()).map_err(|error| format!("{name}: {error}"))
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

    /// 🔎️ The index of an answered perch in the list it was answered from, `None` for none.
    fn index_of(perches: &[Perch], found: Option<&Perch>) -> Option<usize> {
        found.and_then(|found| perches.iter().position(|candidate| std::ptr::eq(candidate, found)))
    }

    /// 🚀️ The granted hop between the two ends of a vector.
    fn granted(from: Point, to: Point) -> Result<Hop, String> {
        hop_of(from, to).ok_or_else(|| "the hop is out of reach".to_string())
    }

    /// 🍎️ The tuning constants of flight.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(&json!({ "gravity": GRAVITY, "fallSpeed": FALL_SPEED, "hopClearance": HOP_CLEARANCE, "hopSteepness": HOP_STEEPNESS, "hopHeight": HOP_HEIGHT, "hopDistance": HOP_DISTANCE, "hopTicks": HOP_TICKS }))
    }

    /// 🍂️ The heights and speeds of every committed fall after every tick.
    pub fn falls(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "falls")?, |vector| {
            let mut fall = Fall { y: number(vector, "y")?, vy: number(vector, "vy")? };
            let (mut heights, mut speeds) = (Vec::new(), Vec::new());
            for _ in 0..number(vector, "ticks")? as usize {
                fall = fall_step(fall.y, fall.vy);
                heights.push(fall.y);
                speeds.push(fall.vy);
            }
            Ok(json!({ "heights": heights, "speeds": speeds }))
        })?)
    }

    /// 🛬️ `landing_of` of every committed sweep, as the index of the answered perch.
    pub fn landings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "landings")?, |vector| {
            let perches = perches(vector)?;
            Ok(json!(list(vector, "sweeps")?.iter().map(|sweep| Ok(index_of(&perches, landing_of(&perches, number(sweep, "x")?, number(sweep, "fromY")?, number(sweep, "toY")?)))).collect::<Result<Vec<_>, String>>()?))
        })?)
    }

    /// 🪨️ Every committed fall until it lands or its ticks run out: the perch, the ticks it took and the height it ends at.
    pub fn drops(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "drops")?, |vector| {
            let perches = perches(vector)?;
            let (start, limit) = (&vector["start"], number(vector, "limit")? as Ticks);
            let x = number(start, "x")?;
            let mut fall = Fall { y: number(start, "y")?, vy: number(start, "vy")? };
            for tick in 1..=limit {
                let next = fall_step(fall.y, fall.vy);
                if let Some(landing) = landing_of(&perches, x, fall.y, next.y) {
                    return Ok(json!({ "perch": index_of(&perches, Some(landing)), "ticks": tick, "y": landing.y }));
                }
                fall = next;
            }
            Ok(json!({ "perch": null, "ticks": limit, "y": fall.y }))
        })?)
    }

    /// 🏹️ `hop_of` between the two ends of every committed vector, `null` when out of reach.
    pub fn hops(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "hops")?, |vector| Ok(json!(hop_of(point(vector, "from")?, point(vector, "to")?))))?)
    }

    /// 🎈️ The feet of every committed granted hop after every tick, and its apex.
    pub fn flights(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "flights")?, |vector| {
            let (from, to) = (point(vector, "from")?, point(vector, "to")?);
            let hop = granted(from, to)?;
            let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
            let mut path = Vec::new();
            let mut apex = f64::INFINITY;
            for left in (1..=hop.ticks).rev() {
                flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
                path.push([flight.x, flight.y]);
                if flight.y < apex {
                    apex = flight.y;
                }
            }
            Ok(json!({ "ticks": hop.ticks, "path": path, "apex": apex }))
        })?)
    }

    /// 🎯️ The index of the perch every committed granted hop really ends on.
    pub fn routes(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "routes")?, |vector| {
            let perches = perches(vector)?;
            let (from, to) = (point(vector, "from")?, point(vector, "to")?);
            Ok(json!(index_of(&perches, hop_landing(&perches, from, to, granted(from, to)?))))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("falls", subject::falls)
        .subject("landings", subject::landings)
        .subject("drops", subject::drops)
        .subject("hops", subject::hops)
        .subject("flights", subject::flights)
        .subject("routes", subject::routes);
    built
}
