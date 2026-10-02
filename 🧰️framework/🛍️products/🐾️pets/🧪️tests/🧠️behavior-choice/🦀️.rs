//! 🧠️ Subject adapter of the behavior-choice case: the behaviour module of the `pets` crate answers every committed vector.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits. Where the TypeScript twin
//! answers `-1` ("no weight is positive") the crate answers `None`; the projection restores the `-1`.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🧠️behavior/🦀️.rs
//! @see ../../🔨️modules/🎲️randomness/🦀️.rs — `random_pick`, `weighted_index`

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{activity_weights, cast_of, dwell_of, encounter_of, encounter_shares, followers_of, mood_of, random_pick, weighted_index, Activity, Actor, Cast, PetMode, Situation, Species, ACTIVITIES, MODE_LIMITS};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🧠️behavior-choice/🔣️.json";

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

    /// 🔢️ The numbers of a list member of a vector.
    fn numbers(vector: &Value, field: &str) -> Result<Vec<f64>, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🧩️ An unsigned 32-bit member of a vector.
    fn word(vector: &Value, field: &str) -> Result<u32, String> {
        vector[field].as_u64().and_then(|word| u32::try_from(word).ok()).ok_or_else(|| format!("vector {vector} carries no unsigned 32-bit {field}"))
    }

    /// 🎬️ The activity a member of a vector names.
    fn activity(vector: &Value, field: &str) -> Result<Activity, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🔆️ The mode a member of a vector names.
    fn mode(vector: &Value, field: &str) -> Result<PetMode, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🏷️ The wire names of activities.
    fn names(activities: &[Activity]) -> Vec<&'static str> {
        activities.iter().map(Activity::as_str).collect()
    }

    /// ⚖️ The weights of a committed situation: an actor with the committed needs and a species with or without a fidget.
    fn weighed(vector: &Value) -> Result<[f64; ACTIVITIES.len()], String> {
        let actor: Actor = serde_json::from_value(json!({ "species": "blob", "perch": null, "x": 0, "y": 0, "vx": 0, "vy": 0, "facing": 1, "faced": 0, "activity": "idle", "since": 0, "until": 0, "goal": 0, "partner": null, "clip": null, "gaze": { "x": 0, "y": 0, "vx": 0, "vy": 0 }, "blink": 0, "mood": 0, "needs": vector["needs"], "opacity": 1, "leaving": false, "draws": 0 })).map_err(|error| format!("actor: {error}"))?;
        let repertoire = if vector["fidgets"] == true { json!({ "fidget": ["fidget"] }) } else { json!({}) };
        let species: Species = serde_json::from_value(json!({ "id": "blob", "name": { "en": "Blob", "de": "Klecks" }, "thing": { "en": "blob", "de": "Klecks" }, "grounds": [], "size": { "width": 40, "height": 40 }, "palette": { "body": "#000000", "accent": "#000000", "detail": "#000000" }, "bones": [], "parts": [], "face": { "eyes": [] }, "clips": [], "repertoire": repertoire, "locomotion": { "gait": "walk", "speed": 40 }, "temperament": { "energy": 0.5, "sociability": 0.5, "curiosity": 0.5 } })).map_err(|error| format!("species: {error}"))?;
        let situation: Situation = serde_json::from_value(
            json!({ "mode": vector["mode"], "quiet": vector["quiet"], "movers": vector["movers"], "fidgeters": vector["fidgeters"], "roam": vector["roam"], "hops": vector["hops"], "crowd": vector["crowd"], "watched": vector["watched"] }),
        )
        .map_err(|error| format!("situation: {error}"))?;
        Ok(activity_weights(&actor, &species, situation))
    }

    /// 🕸️ The activity graph: the followers of every activity, what a breadth-first search reaches from each one, and the number of strongly connected components (1 when everything reaches everything).
    fn graph() -> Value {
        let mut reachable: Vec<Vec<Activity>> = Vec::new();
        for start in ACTIVITIES {
            let mut queue = vec![start];
            let mut head = 0;
            while head < queue.len() {
                for &follower in followers_of(queue[head]) {
                    if !queue.contains(&follower) {
                        queue.push(follower);
                    }
                }
                head += 1;
            }
            reachable.push(ACTIVITIES.into_iter().filter(|activity| queue.contains(activity)).collect());
        }
        let mut classes: Vec<Vec<Activity>> = Vec::new();
        for (index, activity) in ACTIVITIES.into_iter().enumerate() {
            let class: Vec<Activity> = ACTIVITIES.into_iter().enumerate().filter(|&(other_index, other)| reachable[index].contains(&other) && reachable[other_index].contains(&activity)).map(|(_, other)| other).collect();
            if !classes.contains(&class) {
                classes.push(class);
            }
        }
        json!({ "followers": by_activity(ACTIVITIES.into_iter().map(followers_of)), "reachable": by_activity(reachable.iter().map(Vec::as_slice)), "components": classes.len() })
    }

    /// 🗂️ One list of activities per activity, keyed by the wire name of the activity, in `ACTIVITIES` order.
    fn by_activity<'a>(lists: impl Iterator<Item = &'a [Activity]>) -> Value {
        Value::Object(ACTIVITIES.into_iter().zip(lists).map(|(activity, list)| (activity.as_str().to_string(), json!(names(list)))).collect())
    }

    /// 🚦️ The limits of every mode.
    pub fn limits(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "limits")?, |vector| Ok(json!(MODE_LIMITS[mode(vector, "id")?])))
    }

    /// 🧭️ The eleven weights of every committed situation.
    pub fn weights(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "weights")?, |vector| Ok(json!(weighed(vector)?)))
    }

    /// 🎯️ `weighted_index` of every committed weight list at every committed unit; −1 says no weight is positive.
    pub fn picks(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "picks")?, |vector| {
            let weights = numbers(vector, "weights")?;
            Ok(json!(numbers(vector, "units")?.into_iter().map(|unit| weighted_index(&weights, unit).map_or(-1, |index| index as i64)).collect::<Vec<_>>()))
        })
    }

    /// 🙋️ What an actor decides at the counters `0 … count − 1` of its stream, and how often it decides what.
    pub fn decisions(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "decisions")?, |vector| {
            let weights = weighed(vector)?;
            let (seed, stream, count) = (word(vector, "seed")?, word(vector, "stream")?, word(vector, "count")?);
            let activities: Vec<Option<Activity>> = (0..count).map(|counter| random_pick(&[seed, stream, counter], &weights).map(|pick| ACTIVITIES[pick])).collect();
            let counts: Map<String, Value> = ACTIVITIES.into_iter().map(|activity| (activity.as_str().to_string(), json!(activities.iter().filter(|&&chosen| chosen == Some(activity)).count()))).collect();
            Ok(json!({ "activities": activities.iter().map(|chosen| chosen.map(|activity| activity.as_str())).collect::<Vec<_>>(), "counts": counts }))
        })
    }

    /// ⏳️ `dwell_of` of every committed activity and mode at every committed unit.
    pub fn dwells(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "dwells")?, |vector| {
            let (activity, mode) = (activity(vector, "activity")?, mode(vector, "mode")?);
            Ok(json!(numbers(vector, "units")?.into_iter().map(|unit| dwell_of(activity, mode, unit)).collect::<Vec<_>>()))
        })
    }

    /// 🙂️ The mood of every activity.
    pub fn moods(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "moods")?, |vector| Ok(json!(mood_of(activity(vector, "id")?))))
    }

    /// 🥧️ The shares of an encounter at every committed affinity and the kind every committed unit picks.
    pub fn encounters(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "encounters")?, |vector| {
            let affinity = vector["affinity"].as_f64().ok_or_else(|| format!("vector {vector} carries no affinity"))?;
            Ok(json!({ "shares": encounter_shares(affinity), "kinds": numbers(vector, "units")?.into_iter().map(|unit| encounter_of(affinity, unit).as_str()).collect::<Vec<_>>() }))
        })
    }

    /// 🧶️ The activity graph and what is reachable in it.
    pub fn reachability(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "graph")?, |_| Ok(graph()))
    }

    /// 🎟️ `cast_of` of every committed cast, capacity and seed at every committed epoch.
    pub fn casts(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "casts")?, |vector| {
            let cast: Cast = serde_json::from_value(vector["cast"].clone()).map_err(|error| format!("cast: {error}"))?;
            let capacity = vector["capacity"].as_u64().and_then(|capacity| usize::try_from(capacity).ok()).ok_or_else(|| format!("vector {vector} carries no capacity"))?;
            let epochs: Vec<i64> = serde_json::from_value(vector["epochs"].clone()).map_err(|error| format!("epochs: {error}"))?;
            let seed = word(vector, "seed")?;
            Ok(json!(epochs.into_iter().map(|epoch| cast_of(&cast, capacity, epoch, seed)).collect::<Vec<_>>()))
        })
    }
}

/// 🧪️ Subject role only — the oracle is numpy and scipy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("limits", subject::limits)
        .subject("weights", subject::weights)
        .subject("picks", subject::picks)
        .subject("decisions", subject::decisions)
        .subject("dwells", subject::dwells)
        .subject("moods", subject::moods)
        .subject("encounters", subject::encounters)
        .subject("reachability", subject::reachability)
        .subject("casts", subject::casts);
    built
}
