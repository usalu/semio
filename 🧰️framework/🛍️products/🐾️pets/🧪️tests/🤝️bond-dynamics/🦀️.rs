//! 🤝️ Subject adapter of the bond-dynamics case: the behaviour module of the `pets` crate answers every committed vector.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🧠️behavior/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{affinity_of, needs_after, needs_of, rapport_after, rapport_faded, Activity, Menagerie, Needs, Rapport, Temperament, Ticks, ACTIVITIES};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🤝️bond-dynamics/🔣️.json";

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

    /// 📚️ A list member of a vector.
    fn list<'a>(vector: &'a Value, field: &str) -> Result<&'a [Value], String> {
        vector[field].as_array().map(Vec::as_slice).ok_or_else(|| format!("vector {vector} carries no list {field}"))
    }

    /// 🔢️ A number member of a vector.
    fn number(vector: &Value, field: &str) -> Result<f64, String> {
        vector[field].as_f64().ok_or_else(|| format!("vector {vector} carries no number {field}"))
    }

    /// ⏱️ A whole number of ticks.
    fn ticks(value: &Value) -> Result<Ticks, String> {
        value.as_i64().ok_or_else(|| format!("{value} is no whole number of ticks"))
    }

    /// 🎬️ The activity a member of a vector names.
    fn activity(vector: &Value, field: &str) -> Result<Activity, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🧠️ The temperament a vector carries.
    fn temperament(vector: &Value) -> Result<Temperament, String> {
        serde_json::from_value(vector["temperament"].clone()).map_err(|error| format!("temperament: {error}"))
    }

    /// 🎪️ A menagerie that carries nothing but bonds.
    fn bonded(bonds: &Value) -> Result<Menagerie, String> {
        serde_json::from_value(json!({ "schema": "semio.pets.menagerie/v1", "id": "bonds", "title": { "en": "Bonds", "de": "Bande" }, "species": [], "bonds": bonds, "casts": [] })).map_err(|error| format!("bonds: {error}"))
    }

    /// 💞️ `affinity_of` of every committed pair over the committed bonds and rapports.
    pub fn affinities(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "affinities")?, |vector| {
            let menagerie = bonded(&vector["bonds"])?;
            let rapports: Vec<Rapport> = serde_json::from_value(vector["rapports"].clone()).map_err(|error| format!("rapports: {error}"))?;
            let pairs: Vec<[String; 2]> = serde_json::from_value(vector["pairs"].clone()).map_err(|error| format!("pairs: {error}"))?;
            Ok(json!(pairs.iter().map(|[a, b]| affinity_of(&menagerie, &rapports, a, b)).collect::<Vec<_>>()))
        })
    }

    /// 🪢️ `rapport_after` of every committed drift for every activity.
    pub fn rapport_steps(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "rapportSteps")?, |vector| {
            let drift = number(vector, "drift")?;
            Ok(Value::Object(ACTIVITIES.into_iter().map(|activity| (activity.as_str().to_string(), json!(rapport_after(drift, activity)))).collect()))
        })
    }

    /// 🍂️ `rapport_faded` of every committed drift after every committed number of ticks.
    pub fn rapport_fading(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "rapportFading")?, |vector| {
            let drift = number(vector, "drift")?;
            Ok(json!(list(vector, "ticks")?.iter().map(|elapsed| Ok(rapport_faded(drift, ticks(elapsed)?))).collect::<Result<Vec<_>, String>>()?))
        })
    }

    /// 📖️ The drift and the affinity of the pair `a`–`b` after every event of every committed history.
    pub fn histories(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "histories")?, |vector| {
            let menagerie = bonded(&json!([{ "between": ["a", "b"], "affinity": vector["affinity"] }]))?;
            let (mut drifts, mut affinities) = (Vec::new(), Vec::new());
            let mut drift = 0.0;
            for event in list(vector, "events")? {
                drift = rapport_after(rapport_faded(drift, ticks(&event["after"])?), activity(event, "activity")?);
                drifts.push(drift);
                affinities.push(affinity_of(&menagerie, &[Rapport { between: ["a".to_string(), "b".to_string()], drift }], "b", "a"));
            }
            Ok(json!({ "drifts": drifts, "affinities": affinities }))
        })
    }

    /// 🔋️ `needs_after` of every committed need, activity, span and temperament.
    pub fn needs(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "needs")?, |vector| {
            let needs: Needs = serde_json::from_value(vector["needs"].clone()).map_err(|error| format!("needs: {error}"))?;
            Ok(json!(needs_after(needs, activity(vector, "activity")?, ticks(&vector["ticks"])?, temperament(vector)?)))
        })
    }

    /// 🌗️ The needs of every committed temperament at its arrival and after every span.
    pub fn days(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "days")?, |vector| {
            let temperament = temperament(vector)?;
            let mut states = vec![needs_of(temperament)];
            for span in list(vector, "spans")? {
                states.push(needs_after(states[states.len() - 1], activity(span, "activity")?, ticks(&span["ticks"])?, temperament));
            }
            Ok(json!(states))
        })
    }
}

/// 🧪️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("affinities", subject::affinities)
        .subject("rapport-steps", subject::rapport_steps)
        .subject("rapport-fading", subject::rapport_fading)
        .subject("histories", subject::histories)
        .subject("needs", subject::needs)
        .subject("days", subject::days);
    built
}
