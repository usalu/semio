//! ⚗️ Subject adapter of the chemistry-rules case: the feeling module of the `pets` crate answers every committed vector on the states, tricks and reactions of the committed menagerie, and judges the sample menagerie of the product as it stands — scenario for scenario and in the projection shapes of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits. Where the TypeScript twin
//! answers `null` (no trick, a state that stays) the crate answers `None`, which projects as `null`.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, whose projections these are
//! @see ../../🔨️modules/💗️feeling/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        at_rest, click_trick, held_ticks, ladder_of, lasting_ticks, nearby, reactions_of, rungs_of, seen, show_trick, state_after_trick, state_at, state_ends, step_state, trials_of, tricks_for, whim_trick, Activity, Cooling, Feeling, Menagerie,
        Mood, Party, Placement, Rapport, Reaction, Sighting, Slug, Species, Ticks, Trait, Trick, CUES,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://⚗️chemistry-rules/🔣️.json";
    const SAMPLE: &str = "shared://🧬️schema-conformance/🔣️.json";

    /// 🧫️ The committed vectors.
    fn document(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 🎪️ The menagerie the vectors carry.
    fn menagerie(document: &Value) -> Result<Menagerie, String> {
        serde_json::from_value(document["menagerie"].clone()).map_err(|error| format!("menagerie: {error}"))
    }

    /// 📚️ One group of the committed vectors.
    fn group<'a>(document: &'a Value, name: &str) -> Result<&'a [Value], String> {
        document.get(name).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("{VECTORS} carries no {name} group"))
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

    /// 🔢️ The numbers a member of a vector lists.
    fn numbers(vector: &Value, field: &str) -> Result<Vec<f64>, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 👀️ The sightings a member of a vector lists.
    fn sightings(vector: &Value, field: &str) -> Result<Vec<Sighting>, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 💞️ The rapports a member of a vector lists.
    fn rapports(vector: &Value) -> Result<Vec<Rapport>, String> {
        serde_json::from_value(vector["rapports"].clone()).map_err(|error| format!("rapports: {error}"))
    }

    /// 🧬️ One species of a menagerie.
    fn kind_of<'a>(menagerie: &'a Menagerie, id: &str) -> Result<&'a Species, String> {
        menagerie.species.iter().find(|species| species.id == id).ok_or_else(|| format!("the menagerie has no species {id}"))
    }

    /// 🏷️ The ids of tricks.
    fn ids(tricks: &[&Trick]) -> Vec<Slug> {
        tricks.iter().map(|trick| trick.id.clone()).collect()
    }

    /// 🔭️ What the first of two bodies is to the second.
    fn relation(vector: &Value) -> Result<Value, String> {
        let first: Sighting = serde_json::from_value(vector["first"].clone()).map_err(|error| format!("first: {error}"))?;
        let second: Sighting = serde_json::from_value(vector["second"].clone()).map_err(|error| format!("second: {error}"))?;
        let near: Vec<bool> = numbers(vector, "reaches")?.into_iter().map(|reach| nearby(&first, &second, reach)).collect();
        Ok(json!({ "nearby": near, "above": seen(&first, &second, Placement::Above), "below": seen(&first, &second, Placement::Below), "beside": seen(&first, &second, Placement::Beside), "any": seen(&first, &second, Placement::Any) }))
    }

    /// 🕰️ The state of a species at every committed tick, when it gives way and how long it lasts.
    fn standings(menagerie: &Menagerie, vector: &Value) -> Result<Value, String> {
        let species = kind_of(menagerie, text(vector, "species")?)?;
        let state = text(vector, "state")?;
        let since = vector["since"].as_i64().ok_or("a state vector carries no since")?;
        let ticks: Vec<Ticks> = serde_json::from_value(vector["ticks"].clone()).map_err(|error| format!("ticks: {error}"))?;
        let lasts = species.states.iter().find(|entry| entry.id == state).map_or(0, lasting_ticks);
        Ok(json!({ "standings": ticks.iter().map(|&tick| state_at(species, state, since, tick)).collect::<Vec<_>>(), "ends": state_ends(species, state, since), "lasts": lasts }))
    }

    /// 🪜️ The ladder of a species, the rungs of every trick, every step and the state every trick leaves.
    fn ladder(species: &Species) -> Value {
        let names = ladder_of(species);
        let rungs: Map<String, Value> = species.tricks.iter().map(|trick| (trick.id.clone(), json!(rungs_of(species, trick)))).collect();
        let steps: Map<String, Value> =
            names.iter().map(String::as_str).chain(["nowhere"]).map(|state| (state.to_string(), json!({ "up": step_state(species, state, 1), "down": step_state(species, state, -1), "stay": step_state(species, state, 0) }))).collect();
        let after: Map<String, Value> = species.tricks.iter().map(|trick| (trick.id.clone(), Value::Object(names.iter().map(|state| (state.clone(), json!(state_after_trick(species, state, trick)))).collect()))).collect();
        json!({ "ladder": names, "rungs": rungs, "steps": steps, "after": after })
    }

    /// 🙋️ The tricks on offer per cue, state and feeling, and the trick of every committed click and draw.
    fn offers(species: &Species, vector: &Value) -> Result<Value, String> {
        let names = ladder_of(species);
        let calm = at_rest(Mood::Content, 0);
        let mut feelings: Vec<(String, Feeling)> = Vec::new();
        for entry in vector["feelings"].as_array().ok_or("a trick vector carries no feelings")? {
            let mood: Mood = serde_json::from_value(entry["mood"].clone()).map_err(|error| format!("mood: {error}"))?;
            feelings.push((text(entry, "id")?.to_string(), Feeling { mood, intensity: entry["intensity"].as_f64().ok_or("a feeling carries no intensity")?, since: entry["since"].as_i64().ok_or("a feeling carries no since")? }));
        }
        let clicks: Vec<i64> = serde_json::from_value(vector["clicks"].clone()).map_err(|error| format!("clicks: {error}"))?;
        let units = numbers(vector, "units")?;
        let per_feeling = |answer: &dyn Fn(Feeling) -> Value| -> Value { Value::Object(feelings.iter().map(|(id, feeling)| (id.clone(), answer(*feeling))).collect()) };
        let offered: Map<String, Value> =
            CUES.iter().map(|&cue| (json!(cue).as_str().unwrap_or_default().to_string(), Value::Object(names.iter().map(|state| (state.clone(), per_feeling(&|feeling| json!(ids(&tricks_for(species, cue, state, feeling)))))).collect()))).collect();
        let clicked: Map<String, Value> = names.iter().map(|state| (state.clone(), json!(clicks.iter().map(|&index| click_trick(species, state, calm, index).map(|trick| trick.id.clone())).collect::<Vec<_>>()))).collect();
        let drawn = |pick: for<'s> fn(&'s Species, &str, Feeling, f64) -> Option<&'s Trick>| -> Value {
            Value::Object(names.iter().map(|state| (state.clone(), per_feeling(&|feeling| json!(units.iter().map(|&unit| pick(species, state, feeling, unit).map(|trick| trick.id.clone())).collect::<Vec<_>>())))).collect())
        };
        Ok(json!({ "offers": offered, "clicks": clicked, "whims": drawn(whim_trick), "shows": drawn(show_trick) }))
    }

    /// 🕸️ Every way from one state of a species into another — by a trick on offer there, by time, by a reaction — and what a breadth-first walk reaches from the resting state.
    fn reach(chemistry: &[Reaction], species: &Species) -> Value {
        let names = ladder_of(species);
        let mut edges: Vec<(Slug, Slug)> = Vec::new();
        let mut add = |start: &str, end: &str| {
            if start != end && !edges.iter().any(|edge| edge.0 == start && edge.1 == end) {
                edges.push((start.to_string(), end.to_string()));
            }
        };
        for trick in &species.tricks {
            for name in names.iter().filter(|name| trick.from.as_ref().is_none_or(|from| from.contains(name))) {
                add(name, &state_after_trick(species, name, trick));
            }
        }
        for state in &species.states {
            if state_ends(species, &state.id, 0).is_some() {
                add(&state.id, &state_at(species, &state.id, 0, lasting_ticks(state)).state);
            }
        }
        for reaction in chemistry {
            for effect in &reaction.then {
                let side: &Trait = if effect.on == Party::When { &reaction.when } else { &reaction.near };
                let Some(entered) = effect.state.as_ref().filter(|entered| names.contains(entered)) else {
                    continue;
                };
                if side.species.as_ref().is_some_and(|named| *named != species.id) {
                    continue;
                }
                for name in side.state.as_ref().map_or(names.clone(), |state| vec![state.clone()]).iter().filter(|name| names.contains(name)) {
                    add(name, entered);
                }
            }
        }
        let place = |name: &str| names.iter().position(|known| known == name).map_or(-1, |index| index as i64);
        edges.sort_by(|left, right| place(&left.0).cmp(&place(&right.0)).then(place(&left.1).cmp(&place(&right.1))));
        let mut reached: Vec<Slug> = names.first().cloned().into_iter().collect();
        let mut index = 0;
        while index < reached.len() {
            for edge in &edges {
                if edge.0 == reached[index] && !reached.contains(&edge.1) {
                    reached.push(edge.1.clone());
                }
            }
            index += 1;
        }
        let (reachable, unreachable): (Vec<Slug>, Vec<Slug>) = names.iter().cloned().partition(|name| reached.contains(name));
        json!({ "edges": edges.iter().map(|(start, end)| [start, end]).collect::<Vec<_>>(), "reachable": reachable, "unreachable": unreachable })
    }

    /// 🔥️ A story beat by beat, the coolings carried from one beat to the next.
    fn story(menagerie: &Menagerie, vector: &Value) -> Result<Value, String> {
        let mut coolings: Vec<Cooling> = Vec::new();
        let mut beats = Vec::new();
        for beat in vector["beats"].as_array().ok_or("a reaction vector carries no beats")? {
            let outcome = reactions_of(menagerie, &sightings(beat, "sightings")?, beat["tick"].as_i64().ok_or("a beat carries no tick")?, &coolings, &numbers(beat, "units")?, &rapports(beat)?);
            coolings.clone_from(&outcome.coolings);
            beats.push(outcome);
        }
        Ok(json!(beats))
    }

    /// 🎬️ An actor of `species` that stands as a side of a reaction asks: in the state its trait names (its resting state otherwise), held as long as the trait asks, in the mood, activity and trick it names (its resting mood, no trick and idle otherwise — performing when it names a trick), at (400, 400).
    fn actor(menagerie: &Menagerie, side: &Trait, species: &str) -> Result<Sighting, String> {
        let kind = kind_of(menagerie, species)?;
        let resting = kind.states.first().ok_or_else(|| format!("{species} has no state"))?;
        Ok(Sighting {
            species: kind.id.clone(),
            state: side.state.clone().unwrap_or_else(|| resting.id.clone()),
            held: side.held.map_or(0, held_ticks),
            mood: side.mood.unwrap_or(kind.mood),
            intensity: 0.5,
            activity: side.activity.unwrap_or(if side.trick.is_none() { Activity::Idle } else { Activity::Trick }),
            trick: side.trick.clone(),
            x: 400.0,
            y: 400.0,
            width: kind.size.width,
            height: kind.size.height,
        })
    }

    /// 📍️ Two actors of the species `first` and `second` that stand as a reaction asks, the first above, below or beside the second with half the reach between their bodies.
    fn staged(menagerie: &Menagerie, reaction: &Reaction, first: &str, second: &str) -> Result<Vec<Sighting>, String> {
        let (mut one, other) = (actor(menagerie, &reaction.when, first)?, actor(menagerie, &reaction.near, second)?);
        let gap = reaction.within / 2.0;
        match reaction.place.unwrap_or(Placement::Any) {
            Placement::Above => one.y = other.y - other.height - gap,
            Placement::Below => one.y = other.y + one.height + gap,
            Placement::Beside | Placement::Any => one.x = other.x + (one.width + other.width) / 2.0 + gap,
        }
        Ok(vec![one, other])
    }

    /// 🎭️ What a whole menagerie says about itself: per species with states its ladder and what can be reached, per reaction what happens when two of its species stand as it asks (a side that names no species is played by the first species that the other side is not) with their authored bond and every draw lucky.
    fn rehearsal(menagerie: &Menagerie) -> Result<Value, String> {
        let kinds: Vec<&Species> = menagerie.species.iter().filter(|species| !species.states.is_empty()).collect();
        let names: Vec<&str> = kinds.iter().map(|species| species.id.as_str()).collect();
        let mut plays = Map::new();
        for reaction in &menagerie.chemistry {
            let first = reaction.when.species.as_deref().or_else(|| names.iter().copied().find(|&name| Some(name) != reaction.near.species.as_deref()));
            let second = reaction.near.species.as_deref().or_else(|| names.iter().copied().find(|&name| Some(name) != first));
            let (Some(first), Some(second)) = (first, second) else {
                continue;
            };
            if !names.contains(&first) || !names.contains(&second) || first == second {
                continue;
            }
            let staging = staged(menagerie, reaction, first, second)?;
            let due: Vec<&str> = trials_of(menagerie, &staging, &[], 0, &[]).iter().map(|trial| menagerie.chemistry[trial.reaction].id.as_str()).collect();
            plays.insert(reaction.id.clone(), json!({ "due": due, "beat": reactions_of(menagerie, &staging, 0, &[], &vec![0.0; menagerie.chemistry.len()], &[]) }));
        }
        let species: Map<String, Value> = kinds
            .iter()
            .map(|species| {
                let mut entry = reach(&menagerie.chemistry, species);
                entry["ladder"] = json!(ladder_of(species));
                (species.id.clone(), entry)
            })
            .collect();
        Ok(json!({ "species": species, "reactions": plays }))
    }

    /// 📐️ Every committed pair of bodies.
    pub fn relations(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(group(&document(ctx)?, "relations")?, relation)
    }

    /// ⏳️ Every committed state over its ticks.
    pub fn states(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "states")?, |vector| standings(&menagerie, vector))
    }

    /// 🧗️ The ladder of every committed species.
    pub fn ladders(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "ladders")?, |vector| Ok(ladder(kind_of(&menagerie, text(vector, "id")?)?)))
    }

    /// 🃏️ The offers, clicks, whims and shows of every committed species.
    pub fn tricks(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "tricks")?, |vector| offers(kind_of(&menagerie, text(vector, "id")?)?, vector))
    }

    /// 🗺️ What every committed species can reach.
    pub fn reachability(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "reachability")?, |vector| Ok(reach(&menagerie.chemistry, kind_of(&menagerie, text(vector, "id")?)?)))
    }

    /// 🔎️ The trials of every committed stage.
    pub fn matching(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "matching")?, |vector| {
            let coolings: Vec<Cooling> = serde_json::from_value(vector["coolings"].clone()).map_err(|error| format!("coolings: {error}"))?;
            Ok(json!(trials_of(&menagerie, &sightings(vector, "sightings")?, &coolings, vector["tick"].as_i64().ok_or("a stage carries no tick")?, &rapports(vector)?)))
        })
    }

    /// 💥️ Every committed story of the chemistry, beat by beat.
    pub fn reactions(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = document(ctx)?;
        let menagerie = menagerie(&document)?;
        keyed(group(&document, "reactions")?, |vector| story(&menagerie, vector))
    }

    /// 🧸️ The sample menagerie of the product, rehearsed.
    pub fn sample(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document: Value = serde_json::from_slice(&ctx.fixture_bytes(SAMPLE)?).map_err(|error| format!("{SAMPLE}: {error}"))?;
        let menagerie: Menagerie = serde_json::from_value(document["menagerie"].clone()).map_err(|error| format!("{SAMPLE} menagerie: {error}"))?;
        Ok(Outcome::projection(parse_json(&json!({ "sample": rehearsal(&menagerie)? }).to_string())?))
    }
}

/// 🧪️ Subject role only — the oracle is numpy and scipy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("relations", subject::relations)
        .subject("states", subject::states)
        .subject("ladders", subject::ladders)
        .subject("tricks", subject::tricks)
        .subject("reachability", subject::reachability)
        .subject("matching", subject::matching)
        .subject("reactions", subject::reactions)
        .subject("sample", subject::sample);
    built
}
