//! 📐️ Subject adapter of the turn-trigonometry case: the trigonometry of the `pets` crate answers every committed vector.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/📐️trigonometry/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{atan_turns, clamp, cos_turns, fast_neg_exp, lerp, sin_turns, smoothstep};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://📐️turn-trigonometry/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🔢️ A number field of a vector.
    fn number(vector: &Value, field: &str) -> Result<f64, String> {
        vector.get(field).and_then(Value::as_f64).ok_or_else(|| format!("vector {vector} carries no number {field}"))
    }

    /// 🪜️ A whole-number field of a vector.
    fn whole(vector: &Value, field: &str) -> Result<i64, String> {
        vector.get(field).and_then(Value::as_i64).ok_or_else(|| format!("vector {vector} carries no whole number {field}"))
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

    /// 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits.
    fn bits(value: f64) -> String {
        format!("{:016x}", value.to_bits())
    }

    /// 🔄️ The angle of a vector in turns: its numerator divided by its denominator.
    fn turns(vector: &Value) -> Result<f64, String> {
        Ok(number(vector, "numerator")? / number(vector, "denominator")?)
    }

    /// 🌀️ Sine and cosine of every committed angle.
    pub fn angles(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "angles")?, |vector| {
            let turns = turns(vector)?;
            Ok(json!({"sine": sin_turns(turns), "cosine": cos_turns(turns)}))
        })
    }

    /// 🧬️ The bit patterns of the sine and cosine of every committed angle.
    pub fn bit_patterns(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "angles")?, |vector| {
            let turns = turns(vector)?;
            Ok(json!({"sine": bits(sin_turns(turns)), "cosine": bits(cos_turns(turns))}))
        })
    }

    /// 🌊️ Sines and cosines of `n ÷ denominator` turns for every `n` from `first` to `last`.
    pub fn sweeps(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "sweeps")?, |vector| {
            let denominator = number(vector, "denominator")?;
            let steps = whole(vector, "first")?..=whole(vector, "last")?;
            let sines: Vec<f64> = steps.clone().map(|step| sin_turns(step as f64 / denominator)).collect();
            let cosines: Vec<f64> = steps.map(|step| cos_turns(step as f64 / denominator)).collect();
            Ok(json!({"sines": sines, "cosines": cosines}))
        })
    }

    /// 🗜️ Every committed value held inside its bounds.
    pub fn clamps(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "clamps")?, |vector| Ok(json!(clamp(number(vector, "value")?, number(vector, "low")?, number(vector, "high")?))))
    }

    /// ↔️ Every committed blend between two values.
    pub fn lerps(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "lerps")?, |vector| Ok(json!(lerp(number(vector, "from")?, number(vector, "to")?, number(vector, "amount")?))))
    }

    /// 🛝️ The Hermite ease of every committed amount.
    pub fn smoothsteps(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "smoothsteps")?, |vector| Ok(json!(smoothstep(number(vector, "amount")?))))
    }

    /// 🎯️ The direction of every committed point in turns, with its bit pattern.
    pub fn arctangents(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "arctangents")?, |vector| {
            let turns = atan_turns(number(vector, "y")?, number(vector, "x")?);
            Ok(json!({"turns": turns, "bits": bits(turns)}))
        })
    }

    /// 🕸️ The directions of every point `(column × step, row × step)` of each committed lattice, row by row from `−span` to `span`.
    pub fn arctangent_grids(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "arctangentGrids")?, |vector| {
            let (span, step) = (whole(vector, "span")?, number(vector, "step")?);
            let turns: Vec<f64> = (-span..=span).flat_map(|row| (-span..=span).map(move |column| atan_turns(row as f64 * step, column as f64 * step))).collect();
            Ok(json!(turns))
        })
    }

    /// 📉️ The rational decay of every committed argument, with its bit pattern.
    pub fn decays(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "decays")?, |vector| {
            let value = fast_neg_exp(number(vector, "x")?);
            Ok(json!({"value": value, "bits": bits(value)}))
        })
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("angles", subject::angles)
        .subject("bit-patterns", subject::bit_patterns)
        .subject("sweeps", subject::sweeps)
        .subject("clamps", subject::clamps)
        .subject("lerps", subject::lerps)
        .subject("smoothsteps", subject::smoothsteps)
        .subject("arctangents", subject::arctangents)
        .subject("arctangent-grids", subject::arctangent_grids)
        .subject("decays", subject::decays);
    built
}
