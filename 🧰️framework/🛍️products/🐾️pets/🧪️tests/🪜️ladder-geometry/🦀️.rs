//! 🪜️ Subject adapter of the ladder-geometry case: `ladder_for` with its measures, the climb along a ladder, `ladder_holds` and `spill_of` of the climbing module of the `pets` crate answer every committed vector tick by tick, scenario for scenario the projections of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter this one mirrors
//! @see ../../🔨️modules/🧗️climbing/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        ladder_at, ladder_exit, ladder_for, ladder_holds, ladder_landing, ladder_lean, ladder_length, ladder_phase, ladder_rungs, ladder_step, spill_of, LadderStand, Perch, Pitch, Rect, Size, Ticks, LADDER_DESCENT, LADDER_DISMOUNT_TICKS,
        LADDER_EXIT, LADDER_FLAT, LADDER_FOLLOW, LADDER_FOOTING, LADDER_GIRTH, LADDER_HORNS, LADDER_IDLE, LADDER_LEAN, LADDER_LIFE, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, LADDER_RAMP, LADDER_RISE, LADDER_SHIFT, LADDER_SHORT, LADDER_STEEP,
        LADDER_TALL, LADDER_TUCK, RUNG_SPACING, TOPPLE_DAMPING, TOPPLE_PUSH, TOPPLE_STEP, TOPPLE_STIFFNESS,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪜️ladder-geometry/🔣️.json";
    const PATIENCE: usize = 1024;

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 📚️ A list the document carries.
    fn list<'a>(holder: &'a Value, name: &str) -> Result<&'a [Value], String> {
        holder.get(name).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("{VECTORS} carries no list {name}"))
    }

    /// 🔡️ A text member of a vector.
    fn text<'a>(vector: &'a Value, name: &str) -> Result<&'a str, String> {
        vector.get(name).and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no text {name}"))
    }

    /// 🐾️ The size a vector carries.
    fn size(vector: &Value) -> Result<Size, String> {
        serde_json::from_value(vector["size"].clone()).map_err(|error| format!("size: {error}"))
    }

    /// 🪺️ A perch member of a vector.
    fn perch(vector: &Value, name: &str) -> Result<Perch, String> {
        serde_json::from_value(vector[name].clone()).map_err(|error| format!("{name}: {error}"))
    }

    /// 🪹️ The perches a vector carries.
    fn perches(vector: &Value) -> Result<Vec<Perch>, String> {
        serde_json::from_value(vector["perches"].clone()).map_err(|error| format!("perches: {error}"))
    }

    /// 🧱️ The pitch a vector carries.
    fn pitch(vector: &Value) -> Result<Pitch, String> {
        serde_json::from_value(vector["pitch"].clone()).map_err(|error| format!("pitch: {error}"))
    }

    /// 🧗️ The pitches a vector carries.
    fn pitches(vector: &Value) -> Result<Vec<Pitch>, String> {
        serde_json::from_value(vector["pitches"].clone()).map_err(|error| format!("pitches: {error}"))
    }

    /// ⬛️ The keep-outs a vector carries.
    fn keepouts(vector: &Value) -> Result<Vec<Rect>, String> {
        serde_json::from_value(vector["keepouts"].clone()).map_err(|error| format!("keepouts: {error}"))
    }

    /// 🧰️ The ladder a vector carries.
    fn ladder(vector: &Value) -> Result<LadderStand, String> {
        serde_json::from_value(vector["ladder"].clone()).map_err(|error| format!("ladder: {error}"))
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

    /// 🎚️ The tuning constants of ladders.
    pub fn constants(_: &Context<'_>) -> Result<Outcome, String> {
        projected(
            &json!({ "ladderLean": LADDER_LEAN, "ladderSteep": LADDER_STEEP, "ladderFlat": LADDER_FLAT, "ladderShort": LADDER_SHORT, "ladderTall": LADDER_TALL, "ladderTuck": LADDER_TUCK, "ladderHorns": LADDER_HORNS, "rungSpacing": RUNG_SPACING, "ladderFooting": LADDER_FOOTING, "ladderGirth": LADDER_GIRTH, "ladderRise": LADDER_RISE, "ladderDescent": LADDER_DESCENT, "ladderRamp": LADDER_RAMP, "ladderExit": LADDER_EXIT, "ladderFollow": LADDER_FOLLOW, "ladderShift": LADDER_SHIFT, "ladderMountTicks": LADDER_MOUNT_TICKS, "ladderDismountTicks": LADDER_DISMOUNT_TICKS, "ladderRaiseTicks": LADDER_RAISE_TICKS, "ladderIdle": LADDER_IDLE, "ladderLife": LADDER_LIFE, "toppleStiffness": TOPPLE_STIFFNESS, "toppleDamping": TOPPLE_DAMPING, "topplePush": TOPPLE_PUSH, "toppleStep": TOPPLE_STEP }),
        )
    }

    /// 🏗️ The ladder of every committed placement with its measures, `null` when none stands.
    pub fn placements(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "placements")?, |vector| {
            let size = size(vector)?;
            Ok(ladder_for(&perch(vector, "low")?, &perch(vector, "high")?, &pitch(vector)?, &keepouts(vector)?, size)
                .map_or(Value::Null, |ladder| json!({ "length": ladder_length(&ladder), "rungs": ladder_rungs(&ladder), "lean": ladder_lean(&ladder), "exit": ladder_exit(&ladder, size), "landing": ladder_landing(&ladder, size), "ladder": ladder })))
        })?)
    }

    /// 🐛️ The distance, the feet and the phase of the clip after every tick of every committed climb between the foot of a ladder and its exit.
    pub fn climbs(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "climbs")?, |vector| {
            let (size, ladder) = (size(vector)?, ladder(vector)?);
            let down = vector.get("down").and_then(Value::as_bool).ok_or_else(|| format!("vector {vector} carries no flag down"))?;
            let exit = ladder_exit(&ladder, size);
            let goal = if down { 0.0 } else { exit };
            let mut travel = if down { exit } else { 0.0 };
            let mut travels = Vec::new();
            while travel != goal && travels.len() < PATIENCE {
                travel = ladder_step(travel, goal, travels.len() as Ticks);
                travels.push(travel);
            }
            let feet: Vec<[f64; 2]> = travels.iter().map(|&reached| ladder_at(&ladder, reached)).map(|at| [at.x, at.y]).collect();
            let phases: Vec<f64> = travels.iter().map(|&reached| ladder_phase(reached)).collect();
            Ok(json!({ "ticks": travels.len(), "travels": travels, "feet": feet, "phases": phases }))
        })?)
    }

    /// 🩺️ Every committed ladder as it stands after its survey, `null` when it topples.
    pub fn surveys(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "surveys")?, |vector| Ok(json!(ladder_holds(&ladder(vector)?, &perches(vector)?, &pitches(vector)?, &keepouts(vector)?, size(vector)?))))?)
    }

    /// 🎳️ What every committed toppling ladder does to a climber at every committed height.
    pub fn spills(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "spills")?, |vector| {
            let (size, ladder) = (size(vector)?, ladder(vector)?);
            let heights: Vec<f64> = serde_json::from_value(vector["heights"].clone()).map_err(|error| format!("heights: {error}"))?;
            Ok(json!(heights.iter().map(|&height| spill_of(&ladder, height, size)).collect::<Vec<_>>()))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("constants", subject::constants).subject("placements", subject::placements).subject("climbs", subject::climbs).subject("surveys", subject::surveys).subject("spills", subject::spills);
    built
}
