//! 🪢️ Subject adapter of the swing-dynamics case: the swing module of the `pets` crate advances every committed swing, grip, drag, release and reel tick by tick, scenario for scenario the projections of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors
//! @see ../../🔨️modules/🪢️swing/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        cone_clamp, follow_step, hang_of, hang_step, lean_of, reel_step, release_velocity, ring_velocity, swing_step, throw_velocity, Grip, Hang, Point, Ticks, FOLLOW_DAMPING, FOLLOW_STIFFNESS, HANG_CONE, HANG_DAMPING, HANG_GRAVITY, HANG_ROD,
        REEL_CAP, REEL_DAMPING, REEL_LEAST, REEL_RAMP, REEL_SPEED, RELEASE_DIVISOR, RELEASE_STALE, RELEASE_WEIGHTS, THROW_LEAST, THROW_MOST, THROW_RISE, THROW_SHARE,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪢️swing-dynamics/🔣️.json";
    const SETTLED: f64 = 1.0 / 120.0;

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

    /// 🚩️ A flag member of a vector.
    fn flag(vector: &Value, name: &str) -> Result<bool, String> {
        vector.get(name).and_then(Value::as_bool).ok_or_else(|| format!("vector {vector} carries no flag {name}"))
    }

    /// 📍️ A point member of a vector.
    fn point(vector: &Value, name: &str) -> Result<Point, String> {
        serde_json::from_value(vector[name].clone()).map_err(|error| format!("{name}: {error}"))
    }

    /// 📌️ A list of points a vector carries.
    fn points(vector: &Value, name: &str) -> Result<Vec<Point>, String> {
        serde_json::from_value(vector[name].clone()).map_err(|error| format!("{name}: {error}"))
    }

    /// ⏱️ The committed ticks of a vector, 1 first.
    fn ticks(vector: &Value) -> Result<Vec<usize>, String> {
        serde_json::from_value(vector["ticks"].clone()).map_err(|error| format!("ticks: {error}"))
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

    /// 🏃️ The positions of a point after each of `ticks` ticks of `step`, which is given the point, its place one tick earlier and the tick being stepped (1 first).
    fn run(bob: Point, previous: Point, ticks: usize, step: impl Fn(Point, Point, usize) -> Point) -> Vec<Point> {
        let (mut now, mut before) = (bob, previous);
        let mut path = Vec::with_capacity(ticks);
        for tick in 1..=ticks {
            let next = step(now, before, tick);
            before = now;
            now = next;
            path.push(now);
        }
        path
    }

    /// 🎯️ The entries of a path at the committed ticks.
    fn at<T: Clone>(path: &[T], ticks: &[usize]) -> Result<Vec<T>, String> {
        ticks.iter().map(|&tick| tick.checked_sub(1).and_then(|index| path.get(index)).cloned().ok_or_else(|| format!("no state at tick {tick}"))).collect()
    }

    /// 🔝️ The last committed tick.
    fn latest(ticks: &[usize]) -> usize {
        ticks.iter().copied().max().unwrap_or_default()
    }

    /// 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits.
    fn bits(value: f64) -> String {
        format!("{:016x}", value.to_bits())
    }

    /// 🎚️ The tuning constants of the hang, the follow spring, the release, the throw and the reel.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "hangRod": HANG_ROD, "hangGravity": HANG_GRAVITY, "hangDamping": HANG_DAMPING, "hangCone": HANG_CONE, "followStiffness": FOLLOW_STIFFNESS, "followDamping": FOLLOW_DAMPING, "releaseWeights": RELEASE_WEIGHTS, "releaseDivisor": RELEASE_DIVISOR, "releaseStale": RELEASE_STALE, "throwShare": THROW_SHARE, "throwLeast": THROW_LEAST, "throwMost": THROW_MOST, "throwRise": THROW_RISE, "reelSpeed": REEL_SPEED, "reelRamp": REEL_RAMP, "reelLeast": REEL_LEAST, "reelCap": REEL_CAP, "reelDamping": REEL_DAMPING }),
        )
    }

    /// 🕰️ Every committed free swing on a rod at the committed ticks.
    pub fn rod_swings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "rods")?, |vector| {
            let (anchor, ticks) = (point(vector, "anchor")?, ticks(vector)?);
            let (length, gravity, damping) = (number(vector, "length")?, number(vector, "gravity")?, number(vector, "damping")?);
            Ok(json!(at(&run(point(vector, "bob")?, point(vector, "previous")?, latest(&ticks), |bob, previous, _| swing_step(anchor, anchor, bob, previous, length, gravity, damping, false)), &ticks)?))
        })?)
    }

    /// 🏔️ The least height of every committed free swing below its anchor in every window of ticks, and where it ends.
    pub fn amplitude_drift(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "drifts")?, |vector| {
            let anchor = point(vector, "anchor")?;
            let (length, gravity, damping) = (number(vector, "length")?, number(vector, "gravity")?, number(vector, "damping")?);
            let (ticks, window) = (number(vector, "ticks")? as usize, number(vector, "window")? as usize);
            let path = run(point(vector, "bob")?, point(vector, "previous")?, ticks, |bob, previous, _| swing_step(anchor, anchor, bob, previous, length, gravity, damping, false));
            let lowest: Vec<f64> = path.chunks(window.max(1)).map(|chunk| chunk.iter().skip(1).fold(chunk[0].y - anchor.y, |least, bob| if bob.y - anchor.y < least { bob.y - anchor.y } else { least })).collect();
            Ok(json!({ "lowest": lowest, "end": path.last() }))
        })?)
    }

    /// 🚃️ The positions of a point that hangs at rest below the first place of every committed path while its anchor travels the path.
    pub fn moving_anchors(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "anchors")?, |vector| {
            let (path, ticks) = (points(vector, "path")?, ticks(vector)?);
            let (length, gravity) = (number(vector, "length")?, number(vector, "gravity")?);
            let first = path.first().copied().ok_or_else(|| format!("vector {vector} has an empty path"))?;
            let start = Point { x: first.x, y: first.y + length };
            Ok(json!(at(&run(start, start, path.len() - 1, |bob, previous, tick| swing_step(path[tick - 1], path[tick], bob, previous, length, gravity, 1.0, false)), &ticks)?))
        })?)
    }

    /// 🧵️ Every committed point on a slack rope, tick by tick.
    pub fn slack_ropes(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "ropes")?, |vector| {
            let anchor = point(vector, "anchor")?;
            let (length, gravity, ticks) = (number(vector, "length")?, number(vector, "gravity")?, number(vector, "ticks")? as usize);
            Ok(json!(run(point(vector, "bob")?, point(vector, "previous")?, ticks, |bob, previous, _| swing_step(anchor, anchor, bob, previous, length, gravity, 1.0, true))))
        })?)
    }

    /// 🎣️ Every committed swing on a rope reeled in at a steady rate, at the committed ticks.
    pub fn reeled_swings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "reels")?, |vector| {
            let (anchor, ticks) = (point(vector, "anchor")?, ticks(vector)?);
            let (length, rate, gravity, rope) = (number(vector, "length")?, number(vector, "rate")?, number(vector, "gravity")?, flag(vector, "rope")?);
            Ok(json!(at(&run(point(vector, "bob")?, point(vector, "previous")?, latest(&ticks), |bob, previous, tick| swing_step(anchor, anchor, bob, previous, length - (rate * tick as f64) / 64.0, gravity, 1.0, rope)), &ticks)?))
        })?)
    }

    /// 🚦️ The states of every committed reeled swing at the committed ticks and its fastest step in pixels per second.
    pub fn reel_guards(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "guards")?, |vector| {
            let (anchor, ticks, least) = (point(vector, "anchor")?, ticks(vector)?, number(vector, "least")?);
            let (mut bob, mut previous, mut length) = (point(vector, "bob")?, point(vector, "previous")?, number(vector, "length")?);
            let mut fastest = 0.0;
            let mut states = Vec::new();
            for age in 0..latest(&ticks) {
                let reel = reel_step(anchor, bob, previous, length, least, age as Ticks);
                let speed = ((reel.bob.x - bob.x) * (reel.bob.x - bob.x) + (reel.bob.y - bob.y) * (reel.bob.y - bob.y)).sqrt() * 64.0;
                if speed > fastest {
                    fastest = speed;
                }
                previous = bob;
                bob = reel.bob;
                length = reel.length;
                states.push(json!({ "x": bob.x, "y": bob.y, "length": length }));
            }
            Ok(json!({ "states": at(&states, &ticks)?, "fastest": fastest }))
        })?)
    }

    /// 🔻️ Every committed point held on its rod and inside the cone.
    pub fn cone_clamps(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "cones")?, |vector| Ok(json!(cone_clamp(point(vector, "anchor")?, point(vector, "bob")?, number(vector, "length")?))))?)
    }

    /// 🐕️ Every committed grip after each committed number of ticks on its way to its target.
    pub fn follow_springs(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "follows")?, |vector| {
            let target = point(vector, "target")?;
            let mut grip: Grip = serde_json::from_value(vector["grip"].clone()).map_err(|error| format!("grip: {error}"))?;
            let mut reached = 0;
            let mut states = Vec::new();
            for ticks in ticks(vector)? {
                while reached < ticks {
                    grip = follow_step(grip, target);
                    reached += 1;
                }
                states.push(grip);
            }
            Ok(json!(states))
        })?)
    }

    /// 🤹️ The leans of a held pet along every committed pointer path at the committed ticks, their peak, and the tick from which the lean stays below 3°.
    pub fn drag_leans(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "drags")?, |vector| {
            let length = number(vector, "length")?;
            let mut hang = hang_of(point(vector, "feet")?, length);
            let (mut leans, mut peak, mut settled) = (Vec::new(), 0.0, 1);
            for (index, target) in points(vector, "pointer")?.into_iter().enumerate() {
                hang = hang_step(hang, target, length);
                let lean = lean_of(Point { x: hang.grip.x, y: hang.grip.y }, hang.bob, length);
                leans.push(lean);
                if lean.abs() > peak {
                    peak = lean.abs();
                }
                if lean.abs() >= SETTLED {
                    settled = index + 2;
                }
            }
            Ok(json!({ "leans": at(&leans, &ticks(vector)?)?, "peak": peak, "settled": settled }))
        })?)
    }

    /// 💍️ The velocity of the pointer and the throw it makes for every committed ring of samples.
    pub fn release_velocities(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "releases")?, |vector| {
            let samples = points(vector, "samples")?;
            Ok(json!({ "ring": ring_velocity(&samples), "release": release_velocity(&samples) }))
        })?)
    }

    /// 🥏️ The throw of every committed held pet at its release.
    pub fn throw_velocities(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "throws")?, |vector| {
            let hang: Hang = serde_json::from_value(vector["hang"].clone()).map_err(|error| format!("hang: {error}"))?;
            Ok(json!(throw_velocity(hang, &points(vector, "samples")?)))
        })?)
    }

    /// 🧬️ The bit patterns of both coordinates of every committed single step.
    pub fn bit_patterns(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "steps")?, |vector| {
            let next = swing_step(point(vector, "before")?, point(vector, "now")?, point(vector, "bob")?, point(vector, "previous")?, number(vector, "length")?, number(vector, "gravity")?, number(vector, "damping")?, flag(vector, "rope")?);
            Ok(json!({ "x": bits(next.x), "y": bits(next.y) }))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("constants", subject::constants)
        .subject("rod-swings", subject::rod_swings)
        .subject("amplitude-drift", subject::amplitude_drift)
        .subject("moving-anchors", subject::moving_anchors)
        .subject("slack-ropes", subject::slack_ropes)
        .subject("reeled-swings", subject::reeled_swings)
        .subject("reel-guards", subject::reel_guards)
        .subject("cone-clamps", subject::cone_clamps)
        .subject("follow-springs", subject::follow_springs)
        .subject("drag-leans", subject::drag_leans)
        .subject("release-velocities", subject::release_velocities)
        .subject("throw-velocities", subject::throw_velocities)
        .subject("bit-patterns", subject::bit_patterns);
    built
}
