//! 🎣️ Subject adapter of the grapple-reach case: `shot_for`, `hook_step`, the hauls `zip_step` and `sway_step`, `shot_holds`, `landing_for`, `miss_of` and `route_of` of the climbing module of the `pets` crate answer every committed vector tick by tick, scenario for scenario the projections of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits. A standing ladder and a
//! pitch of a route are projected as their index in the list the route was asked with, like the TypeScript adapter's
//! `indexOf`.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors
//! @see ../../🔨️modules/🧗️climbing/🦀️.rs
//! @see ../../🔨️modules/🪢️swing/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        haul_of, hook_step, hook_ticks, landing_for, miss_of, route_of, shot_for, shot_holds, sway_step, zip_step, Gear, Haul, LadderStand, Leg, Perch, Pitch, Point, Rect, Shot, Size, Ticks, HOOK_INSET, HOOK_LIFT, HOOK_RETURN, HOOK_SPEED,
        MUZZLE_FORWARD, MUZZLE_HEIGHT, REEL_LEAST, ROPE_AIM_TICKS, ROPE_DETOUR, ROPE_ELEVATION, ROPE_FOLLOW, ROPE_HOIST_TICKS, ROPE_LONG, ROPE_MARGIN, ROPE_MISS_CHANCE, ROPE_MISS_OVERSHOOT, ROPE_RECOIL_TICKS, ROPE_REST, ROPE_RISE, ROPE_SHORT,
        ROPE_SHRUG_TICKS, ROPE_SULK, ROPE_TUG_TICKS, ZIP_RAMP, ZIP_SLANT, ZIP_SPEED,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🎣️grapple-reach/🔣️.json";
    const PATIENCE: usize = 1024;

    /// 🧩️ A member of a vector decoded as the type the call site asks for (a macro: `serde` itself cannot be named here).
    macro_rules! member {
        ($vector:expr, $name:expr) => {
            serde_json::from_value($vector[$name].clone()).map_err(|error| format!("{}: {error}", $name))
        };
    }

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
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

    /// 🔎️ The index of an answered entry in the list it was answered from, −1 when it is not one of them.
    fn index_of<T>(list: &[T], found: &T) -> i64 {
        list.iter().position(|candidate| std::ptr::eq(candidate, found)).map_or(-1, |index| index as i64)
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

    /// 🎬️ The rope, the hands and the feet of a committed haul after every tick until the least rope is left.
    fn hauled(vector: &Value, step: fn(&Shot, Haul, Ticks, Size) -> Haul) -> Result<Value, String> {
        let (size, shot): (Size, Shot) = (member!(vector, "size")?, member!(vector, "shot")?);
        let least = REEL_LEAST * size.height;
        let mut states: Vec<Haul> = Vec::new();
        let mut haul = haul_of(&shot, shot.length, size);
        while haul.rope > least && states.len() < PATIENCE {
            haul = step(&shot, haul, states.len() as Ticks, size);
            states.push(haul);
        }
        Ok(
            json!({ "ticks": states.len(), "ropes": states.iter().map(|state| state.rope).collect::<Vec<_>>(), "hands": states.iter().map(|state| [state.hand.x, state.hand.y]).collect::<Vec<_>>(), "feet": states.iter().map(|state| [state.x, state.y]).collect::<Vec<_>>() }),
        )
    }

    /// 🎚️ The tuning constants of the grappling rope.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "hookSpeed": HOOK_SPEED, "hookReturn": HOOK_RETURN, "ropeShort": ROPE_SHORT, "ropeLong": ROPE_LONG, "ropeElevation": ROPE_ELEVATION, "ropeMargin": ROPE_MARGIN, "ropeDetour": ROPE_DETOUR, "ropeFollow": ROPE_FOLLOW, "ropeRise": ROPE_RISE, "muzzleForward": MUZZLE_FORWARD, "muzzleHeight": MUZZLE_HEIGHT, "hookInset": HOOK_INSET, "hookLift": HOOK_LIFT, "zipSlant": ZIP_SLANT, "zipSpeed": ZIP_SPEED, "zipRamp": ZIP_RAMP, "ropeAimTicks": ROPE_AIM_TICKS, "ropeRecoilTicks": ROPE_RECOIL_TICKS, "ropeTugTicks": ROPE_TUG_TICKS, "ropeHoistTicks": ROPE_HOIST_TICKS, "ropeShrugTicks": ROPE_SHRUG_TICKS, "ropeMissChance": ROPE_MISS_CHANCE, "ropeMissOvershoot": ROPE_MISS_OVERSHOOT, "ropeRest": ROPE_REST, "ropeSulk": ROPE_SULK }),
        )
    }

    /// 🎇️ The best shot from every committed spot, `null` when no edge is in reach.
    pub fn shots(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "shots")?, |vector| {
            let (size, feet, perches, keepouts): (Size, Point, Vec<Perch>, Vec<Rect>) = (member!(vector, "size")?, member!(vector, "feet")?, member!(vector, "perches")?, member!(vector, "keepouts")?);
            Ok(json!(shot_for(feet, &perches, &keepouts, size)))
        })?)
    }

    /// 🏹️ The ticks of every committed flight and the hook after every one of them, from tick 0 to one tick after its arrival.
    pub fn flights(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "flights")?, |vector| {
            let (from, to, speed): (Point, Point, f64) = (member!(vector, "from")?, member!(vector, "to")?, number(vector, "speed")?);
            let ticks = hook_ticks(from, to, speed);
            let path: Vec<[f64; 2]> = (0..ticks + 2).map(|tick| hook_step(from, to, speed, tick)).map(|at| [at.x, at.y]).collect();
            Ok(json!({ "ticks": ticks, "path": path }))
        })?)
    }

    /// 🚠️ Every committed straight haul, tick by tick.
    pub fn zips(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "zips")?, |vector| hauled(vector, zip_step))?)
    }

    /// 🎪️ Every committed swinging haul, tick by tick.
    pub fn swings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "swings")?, |vector| hauled(vector, sway_step))?)
    }

    /// 🧿️ Every committed shot as it holds after its survey, `null` when the hook lost its edge.
    pub fn surveys(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "surveys")?, |vector| {
            let (shot, perches, keepouts): (Shot, Vec<Perch>, Vec<Rect>) = (member!(vector, "shot")?, member!(vector, "perches")?, member!(vector, "keepouts")?);
            Ok(json!(shot_holds(&shot, &perches, &keepouts)))
        })?)
    }

    /// 🏕️ Where every committed actor lands at the top of its rope, and where a shot meant to miss is aimed.
    pub fn landings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "landings")?, |vector| {
            let (size, shot, perch): (Size, Shot, Perch) = (member!(vector, "size")?, member!(vector, "shot")?, member!(vector, "perch")?);
            Ok(json!({ "landing": landing_for(&shot, &perch, size), "miss": miss_of(&shot, &perch) }))
        })?)
    }

    /// 🗾️ The ways between two perches of every committed stage, a standing ladder and a pitch named by their index.
    pub fn routes(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "routes")?, |vector| {
            let (size, gear, perches): (Size, Vec<Gear>, Vec<Perch>) = (member!(vector, "size")?, member!(vector, "gear")?, member!(vector, "perches")?);
            let (pitches, ladders, keepouts): (Vec<Pitch>, Vec<LadderStand>, Vec<Rect>) = (member!(vector, "pitches")?, member!(vector, "ladders")?, member!(vector, "keepouts")?);
            let (from, to): (usize, usize) = (member!(vector, "from")?, member!(vector, "to")?);
            let (from, to) = (perches.get(from).ok_or("no perch from")?, perches.get(to).ok_or("no perch to")?);
            let legs = route_of(number(vector, "x")?, from, to, &gear, size, number(vector, "grip")?, &pitches, &ladders, &keepouts);
            Ok(legs.map_or(Value::Null, |legs| {
                Value::Array(
                    legs.iter()
                        .map(|leg| match leg {
                            Leg::Ladder { at, ladder, up } => json!({ "means": "ladder", "at": at, "up": up, "ladder": index_of(&ladders, *ladder) }),
                            Leg::Wall { at, pitch, hold, exit, goal } => json!({ "means": "wall", "at": at, "hold": hold, "goal": goal, "pitch": index_of(&pitches, *pitch), "exit": index_of(&pitches, *exit) }),
                            Leg::Raise { at, ladder } => json!({ "means": "raise", "at": at, "ladder": ladder }),
                            Leg::Grapple { at, shot } => json!({ "means": "grapple", "at": at, "shot": shot }),
                        })
                        .collect(),
                )
            }))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("shots", subject::shots)
        .subject("flights", subject::flights)
        .subject("zips", subject::zips)
        .subject("swings", subject::swings)
        .subject("surveys", subject::surveys)
        .subject("landings", subject::landings)
        .subject("routes", subject::routes);
    built
}
