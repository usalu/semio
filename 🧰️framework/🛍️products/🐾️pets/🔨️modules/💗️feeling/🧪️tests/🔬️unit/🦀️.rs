//! 💗️ Unit tests of the Rust twin of the feeling module: every committed vector of the feeling-dynamics and chemistry-rules cases (computed by the numpy and scipy oracles) — numbers within a billionth like the TypeScript suite (the oracles interpolate and sum in their own order), moods, ticks, ids, flags and orders exactly; decays and cuts of time bit for bit — and the laws of impulses, settling, calming, states, tricks and chemistry.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../🟦️.ts — the TypeScript twin, whose suite these tests follow
//! @see ../../../../🧫️fixtures/💗️feeling-dynamics/🔣️.json — the committed vectors of moods, faces, appraisals, wishes and crowds
//! @see ../../../../🧫️fixtures/⚗️chemistry-rules/🔣️.json — the committed vectors of states, tricks and chemistry

use super::*;
use crate::randomness::{random_words, unit_of};
use crate::schema::tests::{entries, fixture, json, number, typed};
use crate::schema::{Text, CUES};
use serde_json::{json, Map, Value};

const CALM: Feeling = Feeling { mood: Mood::Content, intensity: MOOD_REST, since: 0 };

fn ticks(value: &Value) -> Vec<Ticks> {
    entries(value).iter().map(|tick| number(tick) as Ticks).collect()
}

fn difference(actual: &Value, expected: &Value, place: &str) -> Option<String> {
    match (actual, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            ((left - right).abs() > 1e-9 || left.is_nan()).then(|| format!("{place}: {left} is not {right}"))
        }
        (Value::Array(left), Value::Array(right)) if left.len() == right.len() => left.iter().zip(right).enumerate().find_map(|(index, (left, right))| difference(left, right, &format!("{place}[{index}]"))),
        (Value::Object(left), Value::Object(right)) if left.len() == right.len() => {
            right.iter().find_map(|(key, right)| left.get(key).map_or_else(|| Some(format!("{place}: {key} is missing")), |left| difference(left, right, &format!("{place}.{key}"))))
        }
        _ => (actual != expected).then(|| format!("{place}: {actual} is not {expected}")),
    }
}

fn agree(actual: &Value, expected: &Value, place: &str) {
    if let Some(found) = difference(actual, expected, place) {
        panic!("{found}");
    }
}

fn units(seed: u32, count: usize) -> Vec<f64> {
    random_words(&[seed, 77, 0], count).into_iter().map(unit_of).collect()
}

fn drawn(marks: &[f64], index: usize) -> Feeling {
    Feeling { mood: MOODS[(marks[index] * 9.0).floor() as usize], intensity: (marks[index + 1] * 65.0).floor() / 64.0, since: (marks[index + 2] * 500.0).floor() as Ticks }
}

fn name<T: Serialize>(value: T) -> String {
    json(&value).as_str().map(str::to_string).unwrap_or_default()
}

fn trick_of(mood: Option<Mood>) -> Trick {
    Trick { id: "trick".to_string(), name: Text { en: "Trick".to_string(), de: "Kunststück".to_string() }, clip: "clip".to_string(), cues: Vec::new(), emitter: None, from: None, to: None, mood }
}

fn feeling_of(value: &Value) -> Feeling {
    Feeling { mood: typed(&value["mood"]), intensity: number(&value["intensity"]), since: number(&value["since"]) as Ticks }
}

fn leaning(leaning: Leaning) -> Value {
    json!({ "affinity": leaning.affinity, "shares": leaning.shares, "show": leaning.show.map_or(-1, |side| side as i64) })
}

fn ids(tricks: &[&Trick]) -> Vec<String> {
    tricks.iter().map(|trick| trick.id.clone()).collect()
}

fn kind<'a>(menagerie: &'a Menagerie, id: &str) -> &'a Species {
    menagerie.species.iter().find(|species| species.id == id).unwrap_or_else(|| panic!("no species {id}"))
}

fn contest(tick: Ticks) -> Vec<Vec<u8>> {
    MOODS.iter().map(|&present| MOODS.iter().map(|&stirring| u8::from(stirring != present && impulse(Feeling { mood: present, intensity: 0.8, since: 0 }, stirring, 0.5, tick).mood == stirring)).collect()).collect()
}

fn tables() -> Value {
    json!({ "priorities": MOOD_PRIORITIES, "hold": MOOD_HOLD, "faint": MOOD_FAINT, "rest": MOOD_REST, "rise": MOOD_RISE, "override": MOOD_OVERRIDE, "decays": MOOD_DECAYS, "proneDecay": PRONE_DECAY, "valences": MOOD_VALENCES, "lids": MOOD_LIDS, "slants": MOOD_SLANTS, "drops": MOOD_DROPS, "occasions": OCCASIONS, "appraisals": APPRAISALS, "proneness": PRONENESS, "proneGain": PRONE_GAIN, "trickAmount": TRICK_AMOUNT, "drowsyEnergy": DROWSY_ENERGY, "weights": MOOD_WEIGHTS, "affinities": MOOD_AFFINITIES, "shares": MOOD_SHARES, "spreads": MOOD_SPREADS, "contagionBeat": CONTAGION_BEAT, "contagionReach": CONTAGION_REACH })
}

fn sighting(menagerie: &Menagerie, species: &str, state: &str, x: f64, y: f64) -> Sighting {
    let size = kind(menagerie, species).size;
    Sighting { species: species.to_string(), state: state.to_string(), held: 0, mood: Mood::Content, intensity: MOOD_REST, activity: Activity::Idle, trick: None, x, y, width: size.width, height: size.height }
}

#[test]
fn every_decay_vector_settles_and_comes_to_rest_where_the_oracle_says() {
    let vectors = fixture("feeling-dynamics");
    assert!(entries(&vectors["decays"]).len() >= 50);
    for vector in entries(&vectors["decays"]) {
        let feeling: Feeling = typed(&vector["feeling"]);
        let resting: Mood = typed(&vector["resting"]);
        for (at, view) in ticks(&vector["ticks"]).into_iter().zip(entries(&vector["expected"]["views"])) {
            assert_eq!(settled(feeling, resting, at), typed::<Feeling>(view), "{} at {at}", vector["id"]);
        }
        assert_eq!(settles_at(feeling, resting), number(&vector["expected"]["settles"]) as Ticks, "{}", vector["id"]);
    }
}

#[test]
fn settling_in_steps_answers_what_settling_at_once_does() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["jumps"]) {
        let feeling: Feeling = typed(&vector["feeling"]);
        let resting: Mood = typed(&vector["resting"]);
        let cuts = ticks(&vector["cuts"]);
        let stepped = cuts.iter().fold(feeling, |present, &cut| settled(present, resting, cut));
        assert_eq!(stepped, typed::<Feeling>(&vector["expected"]["stepped"]), "{}", vector["id"]);
        assert_eq!(settled(feeling, resting, cuts[cuts.len() - 1]), typed::<Feeling>(&vector["expected"]["direct"]), "{}", vector["id"]);
    }
}

#[test]
fn every_face_vector_has_the_valence_the_spirits_and_the_face_of_the_oracle_within_a_billionth() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["faces"]) {
        let feeling: Feeling = typed(&vector["feeling"]);
        assert_eq!(valence_of(feeling.mood), number(&vector["expected"]["valence"]), "{}", vector["id"]);
        agree(&json!({ "valence": valence_of(feeling.mood), "spirits": spirits_of(feeling), "face": face_of(feeling) }), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn a_feeling_at_rest_stays_put_and_shows_content() {
    for resting in MOODS {
        let calm = at_rest(resting, 640);
        assert_eq!((settled(calm, resting, 640 + 64 * 3600), settles_at(calm, resting), shown_mood(calm.mood, calm.intensity)), (calm, 640, Mood::Content));
    }
    assert_eq!(shown_mood(Mood::Grumpy, MOOD_REST + 0.0625), Mood::Grumpy);
}

#[test]
fn the_tables_are_the_committed_tables() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["tables"]) {
        agree(&tables(), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn an_impulse_reinforces_replaces_soothes_or_is_lost() {
    assert_eq!(impulse(Feeling { mood: Mood::Happy, intensity: 0.5, since: 10 }, Mood::Happy, 0.25, 90), Feeling { mood: Mood::Happy, intensity: 0.75, since: 90 });
    assert_eq!(impulse(Feeling { mood: Mood::Happy, intensity: 0.75, since: 10 }, Mood::Happy, 0.5, 90), Feeling { mood: Mood::Happy, intensity: 1.0, since: 90 });
    for mood in MOODS.into_iter().filter(|&mood| mood != Mood::Content) {
        assert_eq!(impulse(CALM, mood, 0.25, 7), Feeling { mood, intensity: 0.25, since: 7 });
    }
    let happy = Feeling { mood: Mood::Happy, intensity: 0.75, since: 0 };
    assert_eq!(
        [
            impulse(happy, Mood::Scared, 0.25, 1).mood,
            impulse(happy, Mood::Playful, 0.5, MOOD_HOLD - 1).mood,
            impulse(happy, Mood::Playful, 0.5, MOOD_HOLD).mood,
            impulse(happy, Mood::Curious, 0.75 * MOOD_OVERRIDE, MOOD_HOLD).mood,
            impulse(happy, Mood::Curious, 1.0, MOOD_HOLD).mood
        ],
        [Mood::Scared, Mood::Happy, Mood::Playful, Mood::Happy, Mood::Curious]
    );
    let sad = Feeling { mood: Mood::Sad, intensity: 0.75, since: 40 };
    assert_eq!((impulse(sad, Mood::Content, 0.25, 99), impulse(sad, Mood::Content, 0.75, 99)), (Feeling { mood: Mood::Sad, intensity: 0.5, since: 40 }, Feeling { mood: Mood::Sad, intensity: 0.0, since: 99 }));
    for mood in [Mood::Happy, Mood::Playful, Mood::Proud] {
        assert_eq!(impulse(Feeling { mood, intensity: 0.75, since: 0 }, Mood::Content, 1.0, 500), Feeling { mood, intensity: 0.75, since: 0 });
    }
    for mood in MOODS {
        for amount in [0.0, -0.5, f64::NAN] {
            assert_eq!(impulse(Feeling { mood: Mood::Curious, intensity: 0.5, since: 3 }, mood, amount, 500), Feeling { mood: Mood::Curious, intensity: 0.5, since: 3 });
        }
    }
}

#[test]
fn the_contest_of_ranks_and_every_committed_story_come_out_as_the_oracle_says() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["priorities"]) {
        agree(&json!({ "during": contest(MOOD_HOLD / 2), "after": contest(MOOD_HOLD) }), &vector["expected"], &vector["id"].to_string());
    }
    for vector in entries(&vectors["impulses"]) {
        let resting: Mood = typed(&vector["resting"]);
        let mut feeling: Feeling = typed(&vector["feeling"]);
        let mut feelings = Vec::new();
        for event in entries(&vector["events"]) {
            let tick = number(&event["tick"]) as Ticks;
            feeling = impulse(settled(feeling, resting, tick), typed(&event["mood"]), number(&event["amount"]), tick);
            feelings.push(feeling);
        }
        agree(&json(&feelings), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn a_feeling_shows_its_mood_until_no_more_than_the_rest_is_left_and_content_from_calms_at_on() {
    let marks = units(11, 2000 * 4);
    for index in 0..2000 {
        let feeling = drawn(&marks, index * 4);
        let resting = MOODS[(marks[index * 4 + 3] * 9.0).floor() as usize];
        let calm = calms_at(feeling, resting);
        let shown = |tick: Ticks| {
            let present = settled(feeling, resting, tick);
            shown_mood(present.mood, present.intensity)
        };
        assert_eq!(shown(calm), Mood::Content, "{feeling:?} {resting:?}");
        if calm > feeling.since {
            assert_eq!((shown(calm - 1), shown(feeling.since)), (feeling.mood, feeling.mood), "{feeling:?} {resting:?}");
        }
        assert!([1, 64, 6400].iter().all(|later| shown(calm + later) == Mood::Content), "{feeling:?} {resting:?}");
    }
    let proud = Feeling { mood: Mood::Proud, intensity: 0.75, since: 0 };
    assert_eq!((calms_at(Feeling { mood: Mood::Scared, intensity: 0.5, since: 100 }, Mood::Content), calms_at(proud, Mood::Proud)), (100 + MOOD_HOLD + 103, settles_at(proud, Mood::Proud)));
    assert_eq!((calms_at(Feeling { mood: Mood::Content, intensity: 0.9, since: 7 }, Mood::Happy), calms_at(Feeling { mood: Mood::Grumpy, intensity: MOOD_REST, since: 9 }, Mood::Content)), (7, 9));
}

#[test]
fn time_cut_anyhow_settles_alike_and_a_broken_intensity_is_no_feeling_at_all() {
    let marks = units(2, 2000 * 6);
    for index in 0..2000 {
        let feeling = drawn(&marks, index * 6);
        let resting = if index % 3 == 0 { feeling.mood } else { MOODS[(marks[index * 6 + 3] * 9.0).floor() as usize] };
        let middle = feeling.since + (marks[index * 6 + 4] * 9000.0).floor() as Ticks;
        let end = middle + (marks[index * 6 + 5] * 9000.0).floor() as Ticks;
        let direct = settled(feeling, resting, end);
        assert_eq!(settled(settled(feeling, resting, middle), resting, end), direct, "{feeling:?} {resting:?} {middle} {end}");
        assert_eq!(settled(direct, resting, end), direct);
        let rest = settles_at(feeling, resting);
        assert_eq!((settled(feeling, resting, rest).mood, settled(feeling, resting, rest).intensity), (resting, MOOD_REST), "{feeling:?} {resting:?}");
    }
    for intensity in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1.5, -0.5] {
        for mood in [Mood::Happy, Mood::Content] {
            let broken = Feeling { mood, intensity, since: 100 };
            assert_eq!((settles_at(broken, Mood::Content), settled(broken, Mood::Content, 100)), (100, Feeling { mood: Mood::Content, intensity: MOOD_REST, since: 100 }));
        }
    }
}

#[test]
fn every_committed_character_is_appraised_as_the_oracle_says() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["appraisals"]) {
        let character: Character = typed(&vector["character"]);
        let feelings: Vec<Feeling> = typed(&vector["feelings"]);
        let tick = number(&vector["tick"]) as Ticks;
        let energies: Vec<f64> = typed(&vector["energies"]);
        let occasions: Map<String, Value> = OCCASIONS.iter().map(|&occasion| (name(occasion), json(&feelings.iter().map(|&feeling| appraised(feeling, occasion, character, tick)).collect::<Vec<_>>()))).collect();
        let tricks: Map<String, Value> =
            std::iter::once(None).chain(MOODS.map(Some)).map(|mood| (mood.map_or_else(|| "none".to_string(), name), json(&feelings.iter().map(|&feeling| performed(feeling, &trick_of(mood), character, tick)).collect::<Vec<_>>()))).collect();
        let drowsy: Vec<Vec<Feeling>> = feelings.iter().map(|&feeling| energies.iter().map(|&energy| drowsed(feeling, energy, tick)).collect()).collect();
        agree(&json!({ "proneness": MOODS.map(|mood| proneness_of(character, mood)), "occasions": occasions, "tricks": tricks, "drowsy": drowsy }), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn every_committed_wish_and_leaning_is_the_oracles() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["wishes"]) {
        let intensity = number(&vector["intensity"]);
        let wishes: Map<String, Value> = MOODS.iter().map(|&mood| (name(mood), json(&mood_weights(mood, intensity).to_vec()))).collect();
        agree(&Value::Object(wishes), &vector["expected"], &vector["id"].to_string());
    }
    for vector in entries(&vectors["leanings"]) {
        let bias = encounter_bias(typed(&vector["first"]), typed(&vector["second"]));
        agree(&json!({ "leaning": leaning(bias), "swayed": swayed_shares(typed(&vector["shares"]), bias) }), &vector["expected"], &vector["id"].to_string());
    }
    let proud = |intensity: f64| Feeling { mood: Mood::Proud, intensity, since: 0 };
    assert_eq!([encounter_bias(proud(0.5), proud(0.75)).show, encounter_bias(proud(0.5), proud(0.5)).show, encounter_bias(proud(MOOD_REST), CALM).show], [Some(1), Some(0), None]);
    assert_eq!(encounter_bias(CALM, CALM), Leaning { affinity: 0.0, shares: [1.0; 3], show: None });
}

#[test]
fn every_committed_crowd_catches_its_moods_beat_by_beat_as_the_oracle_says() {
    let vectors = fixture("feeling-dynamics");
    for vector in entries(&vectors["contagion"]) {
        let restings: Vec<Mood> = typed(&vector["restings"]);
        let sociabilities: Vec<f64> = typed(&vector["sociabilities"]);
        let affinities: Vec<Vec<f64>> = typed(&vector["affinities"]);
        let near: Vec<Vec<bool>> = typed(&vector["near"]);
        let mut feelings: Vec<Feeling> = typed(&vector["feelings"]);
        let mut beats = Vec::new();
        for beat in 1..=number(&vector["beats"]) as Ticks {
            let tick = beat * CONTAGION_BEAT;
            let present: Vec<Feeling> = feelings.iter().zip(&restings).map(|(&feeling, &resting)| settled(feeling, resting, tick)).collect();
            feelings = present
                .iter()
                .enumerate()
                .map(|(catcher, &mine)| (0..present.len()).filter(|&giver| giver != catcher && near[giver][catcher]).fold(mine, |feeling, giver| caught(feeling, present[giver], affinities[giver][catcher], sociabilities[catcher], tick)))
                .collect();
            beats.push(feelings.clone());
        }
        agree(&json(&beats), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn every_committed_state_lasts_and_gives_way_as_the_oracle_says() {
    let vectors = fixture("chemistry-rules");
    let menagerie: Menagerie = typed(&vectors["menagerie"]);
    for vector in entries(&vectors["states"]) {
        let species = kind(&menagerie, vector["species"].as_str().unwrap_or_default());
        let (state, since) = (vector["state"].as_str().unwrap_or_default(), number(&vector["since"]) as Ticks);
        let standings: Vec<Standing> = ticks(&vector["ticks"]).into_iter().map(|tick| state_at(species, state, since, tick)).collect();
        let lasts = species.states.iter().find(|entry| entry.id == state).map_or(0, lasting_ticks);
        agree(&json!({ "standings": standings, "ends": state_ends(species, state, since), "lasts": lasts }), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn every_committed_ladder_and_trick_is_stepped_and_offered_as_the_oracle_says() {
    let vectors = fixture("chemistry-rules");
    let menagerie: Menagerie = typed(&vectors["menagerie"]);
    for vector in entries(&vectors["ladders"]) {
        let species = kind(&menagerie, vector["id"].as_str().unwrap_or_default());
        let names = ladder_of(species);
        let rungs: Map<String, Value> = species.tricks.iter().map(|trick| (trick.id.clone(), json(&rungs_of(species, trick)))).collect();
        let steps: Map<String, Value> =
            names.iter().map(String::as_str).chain(["nowhere"]).map(|state| (state.to_string(), json!({ "up": step_state(species, state, 1), "down": step_state(species, state, -1), "stay": step_state(species, state, 0) }))).collect();
        let after: Map<String, Value> = species.tricks.iter().map(|trick| (trick.id.clone(), Value::Object(names.iter().map(|state| (state.clone(), json!(state_after_trick(species, state, trick)))).collect()))).collect();
        agree(&json!({ "ladder": names, "rungs": rungs, "steps": steps, "after": after }), &vector["expected"], &vector["id"].to_string());
    }
    for vector in entries(&vectors["tricks"]) {
        let species = kind(&menagerie, vector["id"].as_str().unwrap_or_default());
        let names = ladder_of(species);
        let feelings: Vec<(String, Feeling)> = entries(&vector["feelings"]).iter().map(|feeling| (feeling["id"].as_str().unwrap_or_default().to_string(), feeling_of(feeling))).collect();
        let (clicks, draws): (Vec<i64>, Vec<f64>) = (typed(&vector["clicks"]), typed(&vector["units"]));
        let per_feeling = |answer: &dyn Fn(Feeling) -> Value| -> Value { Value::Object(feelings.iter().map(|(id, feeling)| (id.clone(), answer(*feeling))).collect()) };
        let offers: Map<String, Value> = CUES.iter().map(|&cue| (name(cue), Value::Object(names.iter().map(|state| (state.clone(), per_feeling(&|feeling| json!(ids(&tricks_for(species, cue, state, feeling)))))).collect()))).collect();
        let clicked: Map<String, Value> = names.iter().map(|state| (state.clone(), json!(clicks.iter().map(|&index| click_trick(species, state, CALM, index).map(|trick| trick.id.clone())).collect::<Vec<_>>()))).collect();
        let drawn_by = |pick: for<'s> fn(&'s Species, &str, Feeling, f64) -> Option<&'s Trick>| -> Value {
            Value::Object(names.iter().map(|state| (state.clone(), per_feeling(&|feeling| json!(draws.iter().map(|&unit| pick(species, state, feeling, unit).map(|trick| trick.id.clone())).collect::<Vec<_>>())))).collect())
        };
        agree(&json!({ "offers": offers, "clicks": clicked, "whims": drawn_by(whim_trick), "shows": drawn_by(show_trick) }), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn every_committed_pair_of_bodies_is_near_above_below_or_beside_as_the_oracle_says() {
    let vectors = fixture("chemistry-rules");
    for vector in entries(&vectors["relations"]) {
        let (first, second): (Sighting, Sighting) = (typed(&vector["first"]), typed(&vector["second"]));
        let reaches: Vec<f64> = typed(&vector["reaches"]);
        let nearness: Vec<bool> = reaches.iter().map(|&reach| nearby(&first, &second, reach)).collect();
        agree(
            &json!({ "nearby": nearness, "above": seen(&first, &second, Placement::Above), "below": seen(&first, &second, Placement::Below), "beside": seen(&first, &second, Placement::Beside), "any": seen(&first, &second, Placement::Any) }),
            &vector["expected"],
            &vector["id"].to_string(),
        );
    }
}

#[test]
fn every_committed_stage_and_story_of_the_chemistry_comes_out_as_the_oracle_says() {
    let vectors = fixture("chemistry-rules");
    let menagerie: Menagerie = typed(&vectors["menagerie"]);
    for vector in entries(&vectors["matching"]) {
        let trials = trials_of(&menagerie, &typed::<Vec<Sighting>>(&vector["sightings"]), &typed::<Vec<Cooling>>(&vector["coolings"]), number(&vector["tick"]) as Ticks, &typed::<Vec<Rapport>>(&vector["rapports"]));
        agree(&json(&trials), &vector["expected"], &vector["id"].to_string());
    }
    for vector in entries(&vectors["reactions"]) {
        let mut coolings: Vec<Cooling> = Vec::new();
        let mut beats = Vec::new();
        for beat in entries(&vector["beats"]) {
            let outcome = reactions_of(&menagerie, &typed::<Vec<Sighting>>(&beat["sightings"]), number(&beat["tick"]) as Ticks, &coolings, &typed::<Vec<f64>>(&beat["units"]), &typed::<Vec<Rapport>>(&beat["rapports"]));
            coolings = outcome.coolings.clone();
            beats.push(outcome);
        }
        agree(&json(&beats), &vector["expected"], &vector["id"].to_string());
    }
}

#[test]
fn a_beat_of_chemistry_takes_one_unit_per_chance_and_judges_everybody_as_it_stood() {
    let vectors = fixture("chemistry-rules");
    let menagerie: Menagerie = typed(&vectors["menagerie"]);
    let eclipse = [sighting(&menagerie, "mist", "heavy", 100.0, 190.0), sighting(&menagerie, "glow", "bright", 100.0, 300.0)];
    let trials = trials_of(&menagerie, &eclipse, &[], 0, &[]);
    let beat = reactions_of(&menagerie, &eclipse, 0, &[], &[], &[]);
    assert!(!trials.is_empty() && !beat.consequences.is_empty(), "{trials:?} {beat:?}");
    assert_eq!(beat.drawn, draws_of(&trials));
    assert_eq!(beat.coolings.len(), trials.len());
    assert!(beat.coolings.iter().all(|cooling| cooling.until > 0));
    let again = reactions_of(&menagerie, &eclipse, 1, &beat.coolings, &[0.0; 64], &[]);
    assert!(again.consequences.is_empty() && again.drawn == 0, "{again:?}");
    assert_eq!(held_ticks(0.5), 32);
    assert_eq!((held_ticks(0.0), held_ticks(f64::NAN)), (1, 1));
}

#[test]
fn the_module_calls_no_platform_transcendental() {
    let source = include_str!("../../🦀️.rs");
    for call in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".powf(", ".powi(", ".hypot(", ".ln(", ".mul_add(", "f64::max", "f64::min", ".max(0.0", ".min(1.0", "SystemTime", "Instant", "println!", "static "] {
        assert!(!source.contains(call), "{call}");
    }
}
