//! 🏞️ Subject adapter of the terrain-walking case: `perches_of`, `perch_at`, `nearest_perch` and `stride_to` of the `pets` crate answer every committed vector.
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
    use pets::{nearest_perch, perch_at, perches_of, stride_to, Perch, Rect, Surface};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🏞️terrain-walking/🔣️.json";

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

    /// ✂️ `perches_of` of every committed layout.
    pub fn perches_cut(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "layouts")?, |layout| {
            let surfaces: Vec<Surface> = serde_json::from_value(layout["surfaces"].clone()).map_err(|error| error.to_string())?;
            let keepouts: Vec<Rect> = serde_json::from_value(layout["keepouts"].clone()).map_err(|error| error.to_string())?;
            Ok(json!(perches_of(&surfaces, &keepouts, number(layout, "width")?, number(layout, "height")?, number(layout, "clearance")?, number(layout, "minimum")?)))
        })?)
    }

    /// 📍️ `perch_at` of every committed query, as the index of the answered perch.
    pub fn standing(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "standings")?, |stand| {
            let perches = perches(stand)?;
            Ok(json!(list(stand, "queries")?.iter().map(|query| Ok(index_of(&perches, perch_at(&perches, text(query, "surface")?, number(query, "x")?)))).collect::<Result<Vec<_>, String>>()?))
        })?)
    }

    /// 🧲️ `nearest_perch` of every committed point, as the index of the answered perch.
    pub fn nearest(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "nearests")?, |near| {
            let perches = perches(near)?;
            Ok(json!(list(near, "points")?.iter().map(|feet| Ok(index_of(&perches, nearest_perch(&perches, number(feet, "x")?, number(feet, "y")?)))).collect::<Result<Vec<_>, String>>()?))
        })?)
    }

    /// 👣️ The x after every tick of every committed walk.
    pub fn strides(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "strides")?, |walk| {
            let (goal, speed) = (number(walk, "goal")?, number(walk, "speed")?);
            let mut position = number(walk, "x")?;
            let mut positions = Vec::new();
            for _ in 0..number(walk, "ticks")? as usize {
                position = stride_to(position, goal, speed);
                positions.push(position);
            }
            Ok(json!(positions))
        })?)
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("perches", subject::perches_cut).subject("standing", subject::standing).subject("nearest", subject::nearest).subject("strides", subject::strides);
    built
}
