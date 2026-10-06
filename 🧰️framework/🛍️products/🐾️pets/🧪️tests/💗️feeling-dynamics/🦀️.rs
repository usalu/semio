//! 💗️ Subject adapter of the feeling-dynamics case: the feeling module of the `pets` crate answers every committed vector, scenario for scenario and in the projection shapes of the TypeScript adapter.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits. Where the TypeScript twin
//! answers `-1` (nobody shows off) the crate answers `None`; the projection restores the `-1`.
//!
//! @see ./🥒️.feature
//! @see ./🟦️.ts — the TypeScript adapter, whose projections these are
//! @see ../../🔨️modules/💗️feeling/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{
        appraised, caught, drowsed, encounter_bias, face_of, impulse, mood_weights, performed, proneness_of, settled, settles_at, spirits_of, swayed_shares, valence_of, Character, Feeling, Mood, Text, Ticks, Trick, APPRAISALS, CONTAGION_BEAT,
        CONTAGION_REACH, DROWSY_ENERGY, MOODS, MOOD_AFFINITIES, MOOD_DECAYS, MOOD_DROPS, MOOD_FAINT, MOOD_HOLD, MOOD_LIDS, MOOD_OVERRIDE, MOOD_PRIORITIES, MOOD_REST, MOOD_RISE, MOOD_SHARES, MOOD_SLANTS, MOOD_SPREADS, MOOD_VALENCES, MOOD_WEIGHTS,
        OCCASIONS, PRONENESS, PRONE_DECAY, PRONE_GAIN, TRICK_AMOUNT,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://💗️feeling-dynamics/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context<'_>, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
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

    /// 🔢️ A number member of a vector.
    fn number(vector: &Value, field: &str) -> Result<f64, String> {
        vector[field].as_f64().ok_or_else(|| format!("vector {vector} carries no number {field}"))
    }

    /// ⏱️ A whole number of ticks a member of a vector names.
    fn tick(vector: &Value, field: &str) -> Result<Ticks, String> {
        vector[field].as_i64().ok_or_else(|| format!("vector {vector} carries no tick {field}"))
    }

    /// 💓️ The feeling a member of a vector carries.
    fn feeling(vector: &Value, field: &str) -> Result<Feeling, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🫀️ The feelings a member of a vector lists.
    fn feelings(vector: &Value, field: &str) -> Result<Vec<Feeling>, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 😊️ The mood a member of a vector names.
    fn mood(vector: &Value, field: &str) -> Result<Mood, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🧮️ The numbers a member of a vector lists.
    fn numbers(vector: &Value, field: &str) -> Result<Vec<f64>, String> {
        serde_json::from_value(vector[field].clone()).map_err(|error| format!("{field}: {error}"))
    }

    /// 🏷️ The wire name of a mood.
    fn named(mood: Mood) -> String {
        json!(mood).as_str().unwrap_or_default().to_string()
    }

    /// 📋️ Every table and constant of the module, under the names the oracle uses.
    fn tables() -> Value {
        json!({ "priorities": MOOD_PRIORITIES, "hold": MOOD_HOLD, "faint": MOOD_FAINT, "rest": MOOD_REST, "rise": MOOD_RISE, "override": MOOD_OVERRIDE, "decays": MOOD_DECAYS, "proneDecay": PRONE_DECAY, "valences": MOOD_VALENCES, "lids": MOOD_LIDS, "slants": MOOD_SLANTS, "drops": MOOD_DROPS, "occasions": OCCASIONS, "appraisals": APPRAISALS, "proneness": PRONENESS, "proneGain": PRONE_GAIN, "trickAmount": TRICK_AMOUNT, "drowsyEnergy": DROWSY_ENERGY, "weights": MOOD_WEIGHTS, "affinities": MOOD_AFFINITIES, "shares": MOOD_SHARES, "spreads": MOOD_SPREADS, "contagionBeat": CONTAGION_BEAT, "contagionReach": CONTAGION_REACH })
    }

    /// 🥇️ Which impulse of 0.5 replaces which mood of 0.8: a row per present mood, a column per impulse, at a tick inside the hold or at the one that ends it.
    fn contest(tick: Ticks) -> Vec<Vec<u8>> {
        MOODS.iter().map(|&present| MOODS.iter().map(|&stirring| u8::from(stirring != present && impulse(Feeling { mood: present, intensity: 0.8, since: 0 }, stirring, 0.5, tick).mood == stirring)).collect()).collect()
    }

    /// 📖️ A feeling through its events: settled at the tick of each, then stirred by its impulse.
    fn story(vector: &Value) -> Result<Value, String> {
        let resting = mood(vector, "resting")?;
        let mut present = feeling(vector, "feeling")?;
        let mut told = Vec::new();
        for event in vector["events"].as_array().ok_or("an impulse vector carries no events")? {
            let at = tick(event, "tick")?;
            present = impulse(settled(present, resting, at), mood(event, "mood")?, number(event, "amount")?, at);
            told.push(present);
        }
        Ok(json!(told))
    }

    /// 🦘️ A feeling settled through every cut in turn, and in one step.
    fn jump(vector: &Value) -> Result<Value, String> {
        let (start, resting) = (feeling(vector, "feeling")?, mood(vector, "resting")?);
        let cuts: Vec<Ticks> = serde_json::from_value(vector["cuts"].clone()).map_err(|error| format!("cuts: {error}"))?;
        let last = *cuts.last().ok_or("a jump vector carries no cuts")?;
        let stepped = cuts.iter().fold(start, |present, &cut| settled(present, resting, cut));
        Ok(json!({ "stepped": stepped, "direct": settled(start, resting, last) }))
    }

    /// 🦠️ A crowd through its beats: before every beat everybody is settled, then everybody catches from each neighbour in order, each neighbour as it stood when the beat began.
    fn crowd(vector: &Value) -> Result<Value, String> {
        let restings: Vec<Mood> = serde_json::from_value(vector["restings"].clone()).map_err(|error| format!("restings: {error}"))?;
        let sociabilities = numbers(vector, "sociabilities")?;
        let affinities: Vec<Vec<f64>> = serde_json::from_value(vector["affinities"].clone()).map_err(|error| format!("affinities: {error}"))?;
        let near: Vec<Vec<bool>> = serde_json::from_value(vector["near"].clone()).map_err(|error| format!("near: {error}"))?;
        let mut present = feelings(vector, "feelings")?;
        let mut beats = Vec::new();
        for beat in 1..=tick(vector, "beats")? {
            let at = beat * CONTAGION_BEAT;
            let settledness: Vec<Feeling> = present.iter().zip(&restings).map(|(&feeling, &resting)| settled(feeling, resting, at)).collect();
            present = settledness
                .iter()
                .enumerate()
                .map(|(catcher, &mine)| (0..settledness.len()).filter(|&giver| giver != catcher && near[giver][catcher]).fold(mine, |feeling, giver| caught(feeling, settledness[giver], affinities[giver][catcher], sociabilities[catcher], at)))
                .collect();
            beats.push(present.clone());
        }
        Ok(json!(beats))
    }

    /// 🎭️ A trick that leaves `mood` (or none) and nothing else that a projection would read.
    fn trick_of(mood: Option<Mood>) -> Trick {
        Trick { id: "trick".to_string(), name: Text { en: "Trick".to_string(), de: "Kunststück".to_string() }, clip: "clip".to_string(), cues: Vec::new(), emitter: None, from: None, to: None, mood }
    }

    /// 🧐️ The proneness of a character and its feelings after every occasion, every kind of trick and every look at its energy.
    fn appraisal(vector: &Value) -> Result<Value, String> {
        let character: Character = serde_json::from_value(vector["character"].clone()).map_err(|error| format!("character: {error}"))?;
        let (present, at, energies) = (feelings(vector, "feelings")?, tick(vector, "tick")?, numbers(vector, "energies")?);
        let occasions: Map<String, Value> = OCCASIONS.iter().map(|&occasion| (json!(occasion).as_str().unwrap_or_default().to_string(), json!(present.iter().map(|&feeling| appraised(feeling, occasion, character, at)).collect::<Vec<_>>()))).collect();
        let tricks: Map<String, Value> =
            std::iter::once(None).chain(MOODS.map(Some)).map(|kind| (kind.map_or_else(|| "none".to_string(), named), json!(present.iter().map(|&feeling| performed(feeling, &trick_of(kind), character, at)).collect::<Vec<_>>()))).collect();
        let drowsy: Vec<Vec<Feeling>> = present.iter().map(|&feeling| energies.iter().map(|&energy| drowsed(feeling, energy, at)).collect()).collect();
        Ok(json!({ "proneness": MOODS.map(|kind| proneness_of(character, kind)), "occasions": occasions, "tricks": tricks, "drowsy": drowsy }))
    }

    /// 🤝️ How an encounter of two feelings leans and what that does to shares.
    fn leaning(vector: &Value) -> Result<Value, String> {
        let bias = encounter_bias(feeling(vector, "first")?, feeling(vector, "second")?);
        let shares: [f64; 3] = serde_json::from_value(vector["shares"].clone()).map_err(|error| format!("shares: {error}"))?;
        Ok(json!({ "leaning": { "affinity": bias.affinity, "shares": bias.shares, "show": bias.show.map_or(-1, |side| side as i64) }, "swayed": swayed_shares(shares, bias) }))
    }

    /// 🗒️ Every table and constant of the module.
    pub fn tabled(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "tables")?, |_| Ok(tables()))
    }

    /// 🏆️ The two matrices of who replaces whom.
    pub fn priorities(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "priorities")?, |_| Ok(json!({ "during": contest(MOOD_HOLD / 2), "after": contest(MOOD_HOLD) })))
    }

    /// ⚡️ The feeling after every event of every committed story.
    pub fn impulses(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "impulses")?, story)
    }

    /// 🍂️ Every committed feeling at its ticks and the tick it comes to rest.
    pub fn decays(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "decays")?, |vector| {
            let (start, resting) = (feeling(vector, "feeling")?, mood(vector, "resting")?);
            let views: Vec<Ticks> = serde_json::from_value(vector["ticks"].clone()).map_err(|error| format!("ticks: {error}"))?;
            Ok(json!({ "views": views.iter().map(|&at| settled(start, resting, at)).collect::<Vec<_>>(), "settles": settles_at(start, resting) }))
        })
    }

    /// ⏩️ Every committed cut of time, stepped and direct.
    pub fn jumps(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "jumps")?, jump)
    }

    /// 🤧️ Every committed crowd, beat by beat.
    pub fn contagion(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "contagion")?, crowd)
    }

    /// 😀️ The valence, the spirits and the face of every committed feeling.
    pub fn faces(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "faces")?, |vector| {
            let shown = feeling(vector, "feeling")?;
            Ok(json!({ "valence": valence_of(shown.mood), "spirits": spirits_of(shown), "face": face_of(shown) }))
        })
    }

    /// 📅️ What every committed character makes of every occasion, trick and energy.
    pub fn appraisals(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "appraisals")?, appraisal)
    }

    /// ⚖️ The multipliers of every mood at every committed intensity.
    pub fn wishes(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "wishes")?, |vector| {
            let intensity = number(vector, "intensity")?;
            Ok(Value::Object(MOODS.iter().map(|&kind| (named(kind), json!(mood_weights(kind, intensity).to_vec()))).collect()))
        })
    }

    /// 🧭️ How every committed pair of feelings tips an encounter.
    pub fn leanings(ctx: &Context<'_>) -> Result<Outcome, String> {
        keyed(&group(ctx, "leanings")?, leaning)
    }
}

/// 🧪️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("tables", subject::tabled)
        .subject("priorities", subject::priorities)
        .subject("impulses", subject::impulses)
        .subject("decays", subject::decays)
        .subject("jumps", subject::jumps)
        .subject("contagion", subject::contagion)
        .subject("faces", subject::faces)
        .subject("appraisals", subject::appraisals)
        .subject("wishes", subject::wishes)
        .subject("leanings", subject::leanings);
    built
}
