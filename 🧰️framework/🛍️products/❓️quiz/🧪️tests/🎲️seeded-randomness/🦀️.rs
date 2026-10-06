//! 🎲️ Subject adapter of the seeded-randomness case: the `quiz` crate answers every committed vector.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🎲️randomness/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, json, Map, Value};
    use quiz::{fnv1a32, run_seed, shuffle, uniform_index, Mt19937};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🎲️seeded-randomness/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🔢️ An unsigned field of a vector.
    fn number(vector: &Value, field: &str) -> Result<u64, String> {
        vector.get(field).and_then(Value::as_u64).ok_or_else(|| format!("vector {vector} carries no unsigned {field}"))
    }

    /// 🔤️ A text field of a vector.
    fn text<'a>(vector: &'a Value, field: &str) -> Result<&'a str, String> {
        vector.get(field).and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no text {field}"))
    }

    /// 🗝️ The projection keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors {
            projection.insert(text(vector, "id")?.to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🌱️ A committed seed.
    fn seed(vector: &Value) -> Result<u32, String> {
        u32::try_from(number(vector, "seed")?).map_err(|error| error.to_string())
    }

    /// #️⃣ `fnv1a32` of every committed text.
    pub fn hashes(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "hashes")?, |vector| Ok(json!(fnv1a32(text(vector, "text")?))))
    }

    /// 🪴️ `run_seed` of every committed run id.
    pub fn run_seeds(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "runSeeds")?, |vector| Ok(json!(run_seed(text(vector, "run")?))))
    }

    /// 🌀️ The committed words of every seeded generator after skipping.
    pub fn raw_outputs(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "rawOutputs")?, |vector| {
            let mut random = Mt19937::new(seed(vector)?);
            let words: Vec<u32> = (0..number(vector, "skip")? + number(vector, "count")?).map(|_| random.next_u32()).collect();
            Ok(json!(words[number(vector, "skip")? as usize..]))
        })
    }

    /// 🎯️ Sequential bounded draws, then the next raw word.
    pub fn uniform_draws(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "uniformDraws")?, |vector| {
            let mut random = Mt19937::new(seed(vector)?);
            let bounds = vector.get("bounds").and_then(Value::as_array).ok_or("vector carries no bounds")?;
            let draws = bounds.iter().map(|bound| bound.as_u64().map(|bound| uniform_index(&mut random, bound as usize)).ok_or("bound is not unsigned")).collect::<Result<Vec<_>, _>>()?;
            Ok(json!({ "draws": draws, "next": random.next_u32() }))
        })
    }

    /// 🃏️ A shuffled index range, then the next raw word.
    pub fn shuffles(ctx: &Context) -> Result<Outcome, String> {
        keyed(&group(ctx, "shuffles")?, |vector| {
            let mut random = Mt19937::new(seed(vector)?);
            let indices: Vec<u64> = (0..number(vector, "length")?).collect();
            Ok(json!({ "permutation": shuffle(&mut random, &indices), "next": random.next_u32() }))
        })
    }
}

/// 🧭️ Subject role only — the oracle is numpy's MT19937 in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("hashes", subject::hashes).subject("run-seeds", subject::run_seeds).subject("raw-outputs", subject::raw_outputs).subject("uniform-draws", subject::uniform_draws).subject("shuffles", subject::shuffles);
    built
}
