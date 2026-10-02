//! 🎲️ Subject adapter of the counter-randomness case: the randomness of the `pets` crate answers every committed vector.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🎲️randomness/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{random_between, random_pick, random_unit, random_words};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🎲️counter-randomness/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🔢️ A number field of a vector.
    fn number(vector: &Value, field: &str) -> Result<f64, String> {
        vector.get(field).and_then(Value::as_f64).ok_or_else(|| format!("vector {vector} carries no number {field}"))
    }

    /// 🧩️ An unsigned 32-bit word.
    fn word(value: &Value) -> Result<u32, String> {
        value.as_u64().and_then(|word| u32::try_from(word).ok()).ok_or_else(|| format!("{value} is no unsigned 32-bit word"))
    }

    /// 🪙️ An unsigned 32-bit field of a vector.
    fn unsigned(vector: &Value, field: &str) -> Result<u32, String> {
        word(vector.get(field).ok_or_else(|| format!("vector {vector} carries no {field}"))?)
    }

    /// 🔑️ The key of a vector: a list of unsigned 32-bit words.
    fn key(vector: &Value) -> Result<Vec<u32>, String> {
        vector.get("key").and_then(Value::as_array).ok_or_else(|| format!("vector {vector} carries no key"))?.iter().map(word).collect()
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

    /// 🧵️ One answer per counter `0 … count − 1` of the stream `[seed, stream, counter]`.
    fn counted(vector: &Value, draw: impl Fn(&[u32]) -> Value) -> Result<Value, String> {
        let (seed, stream, count) = (unsigned(vector, "seed")?, unsigned(vector, "stream")?, unsigned(vector, "count")?);
        Ok(Value::Array((0..count).map(|counter| draw(&[seed, stream, counter])).collect()))
    }

    /// 🔠️ The committed count of words for every committed key.
    pub fn words(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "words")?, |vector| Ok(json!(random_words(&key(vector)?, unsigned(vector, "count")? as usize))))
    }

    /// 🎯️ The unit draw of every committed key.
    pub fn units(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "units")?, |vector| Ok(json!(random_unit(&key(vector)?))))
    }

    /// 🌊️ The unit draws of every committed stream, counter by counter.
    pub fn streams(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "streams")?, |vector| counted(vector, |key| json!(random_unit(key))))
    }

    /// 📏️ The draw between the committed bounds for every committed key.
    pub fn ranges(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "ranges")?, |vector| Ok(json!(random_between(&key(vector)?, number(vector, "low")?, number(vector, "high")?))))
    }

    /// 🎰️ The weighted picks of every committed stream, counter by counter; −1 says no weight is positive.
    pub fn picks(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "picks")?, |vector| {
            let weights: Vec<f64> =
                vector.get("weights").and_then(Value::as_array).ok_or_else(|| format!("vector {vector} carries no weights"))?.iter().map(|weight| weight.as_f64().ok_or_else(|| format!("{weight} is no weight"))).collect::<Result<_, _>>()?;
            counted(vector, |key| json!(random_pick(key, &weights).map_or(-1, |index| index as i64)))
        })
    }
}

/// 🧭️ Subject role only — the oracle is numpy's seed sequence in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("words", subject::words).subject("units", subject::units).subject("streams", subject::streams).subject("ranges", subject::ranges).subject("picks", subject::picks);
    built
}
