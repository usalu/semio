//! 🧗️ Subject adapter of the wall-climbing case: `walls_of`, `wall_at`, `nearest_wall`, `segment_hits` and `segment_clear` of the terrain and the holds, climbs, grips, slides, mantles and wall routes of the climbing module of the `pets` crate answer every committed vector tick by tick, scenario for scenario the projections of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits. An answered pitch or perch
//! is projected as its index in the list it was answered from, like the TypeScript adapter's `indexOf`.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors
//! @see ../../🔨️modules/🏞️terrain/🦀️.rs
//! @see ../../🔨️modules/🧗️climbing/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        chain_of, climb_phase, climb_step, climb_ticks, cling_of, crossable, foot_of, grip_for, grip_step, hoist_path, ledge_of, mantle_path, nearest_wall, rim_for, rim_of, route_of, segment_clear, segment_hits, slide_step, slip_of, wall_at,
        wall_cost, wall_holds, wall_path, walls_of, Effort, Entry, Fall, Gear, Leg, Perch, Pitch, Point, Rect, Size, Ticks, Wall, CLIMB_DESCENT, CLIMB_RAMP, CLIMB_RISE, CROSS_REACH, GRIP_BITE, GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, GRIP_REST,
        GRIP_SPACING, HAND_HEIGHT, HOIST_HUMP, HOIST_RISE, LUNGE_TICKS, MANTLE_INSET, MANTLE_TICKS, SLIDE_GAIN, SLIDE_SPEED, SLIDE_START, SLIP_LIFT, SLIP_PUSH, WALL_FOLLOW, WALL_GRAB_TICKS, WALL_HANG_TICKS, WALL_LIP,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🧗️wall-climbing/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
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

    /// 🧩️ A member of a vector decoded as the type the call site asks for (a macro: `serde` itself cannot be named here).
    macro_rules! member {
        ($vector:expr, $name:expr) => {
            serde_json::from_value($vector[$name].clone()).map_err(|error| format!("{}: {error}", $name))
        };
    }

    /// 🔎️ The index of an answered entry in the list it was answered from, `null` for none.
    fn index_of<T>(list: &[T], found: Option<&T>) -> Value {
        found.and_then(|found| list.iter().position(|candidate| std::ptr::eq(candidate, found))).map_or(Value::Null, |index| json!(index))
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

    /// 🦘️ The feet after every tick of a path of `ticks` ticks, the start first.
    fn traced(ticks: Ticks, path: impl Fn(f64) -> Point) -> Value {
        json!((0..=ticks).map(|tick| path(tick as f64 / ticks as f64)).map(|at| [at.x, at.y]).collect::<Vec<_>>())
    }

    /// 🎚️ The tuning constants of walls.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "wallLip": WALL_LIP, "handHeight": HAND_HEIGHT, "climbRise": CLIMB_RISE, "climbDescent": CLIMB_DESCENT, "climbRamp": CLIMB_RAMP, "gripSpacing": GRIP_SPACING, "gripBudget": GRIP_BUDGET, "gripClimb": GRIP_CLIMB, "gripHang": GRIP_HANG, "gripRest": GRIP_REST, "gripBite": GRIP_BITE, "wallFollow": WALL_FOLLOW, "slideStart": SLIDE_START, "slideGain": SLIDE_GAIN, "slideSpeed": SLIDE_SPEED, "slipPush": SLIP_PUSH, "slipLift": SLIP_LIFT, "wallGrabTicks": WALL_GRAB_TICKS, "wallHangTicks": WALL_HANG_TICKS, "mantleTicks": MANTLE_TICKS, "mantleInset": MANTLE_INSET, "hoistHump": HOIST_HUMP, "hoistRise": HOIST_RISE, "crossReach": CROSS_REACH, "lungeTicks": LUNGE_TICKS }),
        )
    }

    /// 🗻️ The pitches of every committed layout.
    pub fn pitches(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "layouts")?, |layout| {
            let (walls, keepouts): (Vec<Wall>, Vec<Rect>) = (member!(layout, "walls")?, member!(layout, "keepouts")?);
            Ok(json!(walls_of(&walls, &keepouts, number(layout, "width")?, number(layout, "height")?, number(layout, "clearance")?, number(layout, "minimum")?)))
        })?)
    }

    /// 📌️ The pitch of every committed query, as its index.
    pub fn stretches(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "stretches")?, |stand| {
            let pitches: Vec<Pitch> = member!(stand, "pitches")?;
            Ok(Value::Array(list(stand, "queries")?.iter().map(|query| Ok(index_of(&pitches, wall_at(&pitches, text(query, "wall")?, number(query, "y")?)))).collect::<Result<_, String>>()?))
        })?)
    }

    /// 🧲️ The nearest pitch of every committed point, as its index.
    pub fn nearest(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "nearests")?, |near| {
            let pitches: Vec<Pitch> = member!(near, "pitches")?;
            Ok(Value::Array(list(near, "points")?.iter().map(|at| Ok(index_of(&pitches, nearest_wall(&pitches, number(at, "x")?, number(at, "y")?)))).collect::<Result<_, String>>()?))
        })?)
    }

    /// 🔭️ Whether every committed segment hits each box, and whether it is clear of all.
    pub fn sights(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "sights")?, |vector| {
            let (rects, margin): (Vec<Rect>, f64) = (member!(vector, "rects")?, number(vector, "margin")?);
            let answers = list(vector, "segments")?
                .iter()
                .map(|segment| {
                    let (from, to): (Point, Point) = (member!(segment, "from")?, member!(segment, "to")?);
                    Ok(json!({ "hits": rects.iter().map(|rect| segment_hits(from, to, rect, margin)).collect::<Vec<_>>(), "clear": segment_clear(from, to, &rects, margin) }))
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(json!(answers))
        })?)
    }

    /// 🧤️ The hold of every perch on every pitch of every committed stage, the perch that crowns every pitch, and the measures of every pitch.
    pub fn holds(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "holds")?, |stage| {
            let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (member!(stage, "size")?, member!(stage, "perches")?, member!(stage, "pitches")?);
            let grips: Vec<Vec<Value>> = perches.iter().map(|perch| pitches.iter().map(|pitch| json!(grip_for(perch, pitch, size))).collect()).collect();
            let rims: Vec<Value> = pitches.iter().map(|pitch| index_of(&perches, rim_for(pitch, &perches, size))).collect();
            let measures: Vec<Value> = pitches.iter().map(|pitch| json!({ "cling": cling_of(pitch, size), "ledge": ledge_of(pitch, size), "rim": rim_of(pitch, size), "foot": foot_of(pitch, size) })).collect();
            Ok(json!({ "grips": grips, "rims": rims, "measures": measures }))
        })?)
    }

    /// 🧐️ The pitch that still carries every committed climber after its survey, and the throw when none does.
    pub fn surveys(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "surveys")?, |vector| {
            let (size, held, pitches): (Size, Pitch, Vec<Pitch>) = (member!(vector, "size")?, member!(vector, "pitch")?, member!(vector, "pitches")?);
            let found = wall_holds(&held, &pitches, number(vector, "y")?, size);
            Ok(json!({ "pitch": index_of(&pitches, found), "slip": if found.is_none() { json!(slip_of(&held)) } else { Value::Null } }))
        })?)
    }

    /// 🐜️ The height and the phase of the clip after every tick of every committed climb that arrives, and its ticks.
    pub fn climbs(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "climbs")?, |vector| {
            let (size, held): (Size, Pitch) = (member!(vector, "size")?, member!(vector, "pitch")?);
            let (from, goal) = (number(vector, "from")?, number(vector, "goal")?);
            let ticks = climb_ticks(from, goal);
            let mut heights = Vec::new();
            let mut height = from;
            if (ticks as f64) <= GRIP_BUDGET {
                for tick in 0..ticks {
                    height = climb_step(height, goal, tick);
                    heights.push(height);
                }
            }
            let phases: Vec<f64> = heights.iter().map(|&reached| climb_phase(&held, reached, size)).collect();
            Ok(json!({ "ticks": ticks, "heights": heights, "phases": phases }))
        })?)
    }

    /// 🔋️ The grip after every committed run of ticks of one effort.
    pub fn grips(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "grips")?, |vector| {
            let runs: Vec<(Effort, usize)> = member!(vector, "runs")?;
            let mut left = number(vector, "grip")?;
            let mut grips = Vec::new();
            for (effort, ticks) in runs {
                for _ in 0..ticks {
                    left = grip_step(left, effort);
                }
                grips.push(left);
            }
            Ok(json!(grips))
        })?)
    }

    /// 🧈️ The heights and speeds of every committed slide after every tick.
    pub fn slides(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "slides")?, |vector| {
            let (floor, ticks) = (number(vector, "floor")?, number(vector, "ticks")? as usize);
            let mut slide = Fall { y: number(vector, "y")?, vy: number(vector, "vy")? };
            let (mut heights, mut speeds) = (Vec::new(), Vec::new());
            for _ in 0..ticks {
                slide = slide_step(slide.y, slide.vy, floor);
                heights.push(slide.y);
                speeds.push(slide.vy);
            }
            Ok(json!({ "heights": heights, "speeds": speeds }))
        })?)
    }

    /// 🧘️ The feet after every tick of every committed mantle and hoist, the start first.
    pub fn mantles(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        let mut projection = Map::new();
        for vector in list(&document, "mantles")? {
            let (size, held): (Size, Pitch) = (member!(vector, "size")?, member!(vector, "pitch")?);
            projection.insert(text(vector, "id")?.to_string(), traced(MANTLE_TICKS, |phase| mantle_path(&held, size, phase)));
        }
        for vector in list(&document, "hoists")? {
            let (from, to, height, ticks): (Point, Point, f64, Ticks) = (member!(vector, "from")?, member!(vector, "to")?, number(vector, "height")?, member!(vector, "ticks")?);
            projection.insert(text(vector, "id")?.to_string(), traced(ticks, |phase| hoist_path(from, to, height, phase)));
        }
        projected(&Value::Object(projection))
    }

    /// 🌉️ Which pitch of every committed stage lunges to which, the wall line of every pitch as indices, and every committed way along a line tick by tick (`[x, y, hold, work]`, the hold an index into its line) with the grip it costs.
    pub fn crossings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "crossings")?, |vector| {
            let (size, pitches): (Size, Vec<Pitch>) = (member!(vector, "size")?, member!(vector, "pitches")?);
            let crossings: Vec<Vec<bool>> = pitches.iter().map(|from| pitches.iter().map(|to| crossable(from, to, size)).collect()).collect();
            let chains: Vec<Vec<Value>> = pitches.iter().map(|held| chain_of(held, &pitches, size).into_iter().map(|member| index_of(&pitches, Some(member))).collect()).collect();
            let mut ways = Vec::new();
            for wish in list(vector, "ways")? {
                let index: usize = member!(wish, "pitch")?;
                let held = pitches.get(index).ok_or_else(|| format!("no pitch {index}"))?;
                let chain = chain_of(held, &pitches, size);
                let start = chain.iter().position(|member| std::ptr::eq(*member, held)).ok_or("the pitch is not in its own line")?;
                let (entry, end, mantle): (Entry, usize, bool) = (member!(wish, "entry")?, member!(wish, "end")?, member!(wish, "mantle")?);
                let path = wall_path(&chain, start, entry, number(wish, "x")?, number(wish, "y")?, end, number(wish, "goal")?, mantle, size);
                ways.push(json!({ "path": path.iter().map(|clamber| json!([clamber.x, clamber.y, clamber.hold, clamber.work])).collect::<Vec<_>>(), "cost": wall_cost(&path) }));
            }
            Ok(json!({ "crossable": crossings, "chains": chains, "ways": ways }))
        })?)
    }

    /// 🗺️ The walls between two perches of every committed stage for an actor with the gear to climb.
    pub fn routes(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "routes")?, |vector| {
            let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (member!(vector, "size")?, member!(vector, "perches")?, member!(vector, "pitches")?);
            let (from, to): (usize, usize) = (member!(vector, "from")?, member!(vector, "to")?);
            let (from, to) = (perches.get(from).ok_or("no perch from")?, perches.get(to).ok_or("no perch to")?);
            let legs = route_of(number(vector, "x")?, from, to, &[Gear::Climb], size, number(vector, "grip")?, &pitches, &[], &[]);
            Ok(legs.map_or(Value::Null, |legs| {
                Value::Array(
                    legs.iter()
                        .map(|leg| match leg {
                            Leg::Wall { at, pitch, hold, exit, goal } => json!({ "at": at, "hold": hold, "goal": goal, "pitch": index_of(&pitches, Some(*pitch)), "exit": index_of(&pitches, Some(*exit)) }),
                            Leg::Ladder { at, ladder, up } => json!({ "means": "ladder", "at": at, "ladder": ladder, "up": up }),
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
        .subject("pitches", subject::pitches)
        .subject("stretches", subject::stretches)
        .subject("nearest", subject::nearest)
        .subject("sights", subject::sights)
        .subject("holds", subject::holds)
        .subject("surveys", subject::surveys)
        .subject("climbs", subject::climbs)
        .subject("grips", subject::grips)
        .subject("slides", subject::slides)
        .subject("mantles", subject::mantles)
        .subject("routes", subject::routes)
        .subject("crossings", subject::crossings);
    built
}
