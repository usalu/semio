//! 🪄️ Subject adapter of the mischief-choice case: the mischief module of the `pets` crate answers every committed vector, scenario for scenario and in the projection shapes of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, so the subject is given the very doubles the TypeScript
//! adapter is given and both project the same bits. A host description is handed over whole — values, correctness
//! flags and answers included — as an item that is [`pets::Keyed`]: the pick can read its key and nothing else. Where
//! the TypeScript twin answers `null` (no candidate, no station, no tick) the crate answers `None`; the projection
//! restores the twin's `-1` for a choice among no positions and `null` everywhere else.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, whose projections these are
//! @see ../../🔨️modules/🪄️mischief/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{allowed, allowed_from, chosen_fixture, fits, fixture_for, lift_at, lift_ends, station_for, thrown_off, Circumstances, Facing, Fixture, Keyed, Perch, Pitch, Point, Ticks};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪄️mischief-choice/🔣️.json";

    /// 📄️ An item of a host description with everything the page knows about it: its id and key, and its value, whether it is correct and what was answered — none of which the pick may read.
    struct Described {
        id: String,
        key: String,
        value: Value,
        correct: Value,
        answered: Value,
    }

    impl Keyed for Described {
        /// 🔑️ The key of the item, the one thing the pick reads.
        fn key(&self) -> &str {
            &self.key
        }
    }

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context<'_>, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
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

    /// 🔡️ A text member of a vector.
    fn text<'a>(vector: &'a Value, field: &str) -> Result<&'a str, String> {
        vector[field].as_str().ok_or_else(|| format!("vector {vector} carries no text {field}"))
    }

    /// 🔢️ A number member of a vector.
    fn number(vector: &Value, field: &str) -> Result<f64, String> {
        vector[field].as_f64().ok_or_else(|| format!("vector {vector} carries no number {field}"))
    }

    /// ⏱️ A whole number of ticks a member of a vector names.
    fn tick(vector: &Value, field: &str) -> Result<Ticks, String> {
        vector[field].as_i64().ok_or_else(|| format!("vector {vector} carries no tick {field}"))
    }

    /// 🌱️ The grounds a vector lists.
    fn grounds(vector: &Value) -> Result<Vec<String>, String> {
        serde_json::from_value(vector["grounds"].clone()).map_err(|error| format!("grounds: {error}"))
    }

    /// 📌️ The fixture a member of a vector carries.
    fn fixture(vector: &Value, field: &str) -> Result<Fixture, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 📃️ The items of a host description.
    fn described(vector: &Value) -> Result<Vec<Described>, String> {
        vector["items"]
            .as_array()
            .ok_or("a leak vector carries no items")?
            .iter()
            .map(|item| Ok(Described { id: text(item, "id")?.to_string(), key: text(item, "key")?.to_string(), value: item["value"].clone(), correct: item["correct"].clone(), answered: item["answered"].clone() }))
            .collect()
    }

    /// 🧲️ The id of the fixture a species picks in a host description, `None` when nothing fits.
    fn picked(grounds: &[String], items: &[Described], unit: f64) -> Option<String> {
        chosen_fixture(&fixture_for(grounds, items), unit).map(|item| item.id.clone())
    }

    /// 🙈️ The pick over the plain description and over every permutation of the values, the correctness flags and the answers among its items.
    fn leak(vector: &Value) -> Result<Value, String> {
        let (grounds, items, unit) = (grounds(vector)?, described(vector)?, number(vector, "unit")?);
        let mut shuffled = Vec::new();
        for shuffle in vector["shuffles"].as_array().ok_or("a leak vector carries no shuffles")? {
            let order: Vec<usize> = serde_json::from_value(shuffle.clone()).map_err(|error| format!("shuffle: {error}"))?;
            let permuted: Vec<Described> = items
                .iter()
                .enumerate()
                .map(|(place, item)| Described { id: item.id.clone(), key: item.key.clone(), value: items[order[place]].value.clone(), correct: items[order[place]].correct.clone(), answered: items[order[place]].answered.clone() })
                .collect();
            shuffled.push(picked(&grounds, &permuted, unit));
        }
        Ok(json!({ "plain": picked(&grounds, &items, unit), "shuffled": shuffled }))
    }

    /// 🎢️ The copy of a lifted fixture at every `step`-th tick from `first` to `last`: dx, dy, tilt and opacity as four lists, and the tick the lift ends.
    fn trajectory(vector: &Value) -> Result<Value, String> {
        let since = tick(vector, "since")?;
        let side: Facing = serde_json::from_value(vector["side"].clone()).map_err(|error| format!("side: {error}"))?;
        let (room, span, unit) = (number(vector, "room")?, number(vector, "span")?, number(vector, "unit")?);
        let step = usize::try_from(tick(vector, "step")?).map_err(|error| format!("step: {error}"))?;
        let lifts: Vec<_> = (tick(vector, "first")?..=tick(vector, "last")?).step_by(step.max(1)).map(|at| lift_at(since, at, side, room, span, unit)).collect();
        Ok(
            json!({ "dx": lifts.iter().map(|lift| lift.dx).collect::<Vec<_>>(), "dy": lifts.iter().map(|lift| lift.dy).collect::<Vec<_>>(), "tilt": lifts.iter().map(|lift| lift.tilt).collect::<Vec<_>>(), "opacity": lifts.iter().map(|lift| lift.opacity).collect::<Vec<_>>(), "ends": lift_ends(since) }),
        )
    }

    /// 🧩️ Whether every committed ground covers its key.
    pub fn matches(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "matches")?, |vector| Ok(json!(fits(text(vector, "ground")?, text(vector, "key")?))))
    }

    /// 🗂️ The fitting fixtures of every committed species, by id.
    pub fn candidates(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "candidates")?, |vector| {
            let fixtures: Vec<Fixture> = serde_json::from_value(vector["fixtures"].clone()).map_err(|error| format!("fixtures: {error}"))?;
            Ok(json!(fixture_for(&grounds(vector)?, &fixtures).iter().map(|fixture| fixture.id.as_str()).collect::<Vec<_>>()))
        })
    }

    /// 🎯️ The position every committed unit picks among every committed count, −1 among none.
    pub fn choices(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "choices")?, |vector| {
            let positions: Vec<i64> = (0..tick(vector, "count")?).collect();
            let units: Vec<f64> = serde_json::from_value(vector["units"].clone()).map_err(|error| format!("units: {error}"))?;
            Ok(json!(units.iter().map(|&unit| chosen_fixture(&positions, unit).copied().unwrap_or(-1)).collect::<Vec<_>>()))
        })
    }

    /// 🕵️ The pick over every committed host description and its permutations.
    pub fn leaks(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "leaks")?, leak)
    }

    /// 🚦️ The verdict and the tick of every committed occasion.
    pub fn gates(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "gates")?, |vector| {
            let mut fields = vector.as_object().cloned().unwrap_or_default();
            fields.remove("id");
            fields.remove("expected");
            let circumstances: Circumstances = serde_json::from_value(Value::Object(fields)).map_err(|error| format!("circumstances: {error}"))?;
            Ok(json!({ "allowed": allowed(circumstances), "from": allowed_from(circumstances) }))
        })
    }

    /// 🧭️ The station of every committed fixture.
    pub fn stations(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "stations")?, |vector| {
            let pitches: Vec<Pitch> = serde_json::from_value(vector["pitches"].clone()).map_err(|error| format!("pitches: {error}"))?;
            let perches: Vec<Perch> = serde_json::from_value(vector["perches"].clone()).map_err(|error| format!("perches: {error}"))?;
            Ok(json!(station_for(&fixture(vector, "fixture")?, &pitches, &perches, number(vector, "width")?)))
        })
    }

    /// 🏗️ The path of every committed lift.
    pub fn lifts(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "lifts")?, trajectory)
    }

    /// 💨️ The throw of every committed pusher.
    pub fn throws(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "throws")?, |vector| {
            let pusher: Point = serde_json::from_value(vector["pusher"].clone()).map_err(|error| format!("pusher: {error}"))?;
            Ok(json!(thrown_off(pusher, &fixture(vector, "fixture")?, number(vector, "unit")?)))
        })
    }
}

/// 🧪️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("matches", subject::matches)
        .subject("candidates", subject::candidates)
        .subject("choices", subject::choices)
        .subject("leaks", subject::leaks)
        .subject("gates", subject::gates)
        .subject("stations", subject::stations)
        .subject("lifts", subject::lifts)
        .subject("throws", subject::throws);
    built
}
