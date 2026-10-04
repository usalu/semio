//! 🩻️ Unit tests of the schema twin: serde round trips of every shared fixture document, the wire spellings held to the normative file, and the test kit the module suites share.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../🔣️.json — the normative contract the enumerations are read from

use super::*;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::path::PathBuf;

pub(crate) fn fixture(case: &str) -> Value {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
    let directory = std::fs::read_dir(&root).into_iter().flatten().flatten().map(|entry| entry.path()).find(|path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(case)));
    let path = directory.unwrap_or_else(|| panic!("no fixture case {case} under {}", root.display())).join("🔣️.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub(crate) fn typed<T: DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).unwrap_or_else(|error| panic!("{error}: {value}"))
}

pub(crate) fn json<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or_else(|error| panic!("{error}"))
}

pub(crate) fn entries(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

pub(crate) fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("not a number: {value}"))
}

pub(crate) fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

pub(crate) fn assert_same(context: &str, produced: &Value, expected: &Value) {
    match (produced, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            assert!(left == right, "{context}: {left} ≠ {right}");
        }
        (Value::Object(left), Value::Object(right)) => {
            assert_eq!(left.keys().collect::<Vec<_>>(), right.keys().collect::<Vec<_>>(), "{context}: keys");
            for (key, value) in left {
                assert_same(&format!("{context}/{key}"), value, &right[key]);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{context}: length");
            for (index, (left, right)) in left.iter().zip(right).enumerate() {
                assert_same(&format!("{context}/{index}"), left, right);
            }
        }
        _ => assert_eq!(produced, expected, "{context}"),
    }
}

fn round_trips<T: Serialize + DeserializeOwned>(context: &str, value: &Value) {
    assert_same(context, &json(&typed::<T>(value)), value);
}

fn round_trips_as(context: &str, definition: &str, document: &Value) {
    match definition {
        "Species" => round_trips::<Species>(context, document),
        "Menagerie" => round_trips::<Menagerie>(context, document),
        "Ensemble" => round_trips::<Ensemble>(context, document),
        other => panic!("{context}: no twin is tested for the definition {other}"),
    }
}

fn normative() -> Value {
    serde_json::from_str(include_str!("../../🔣️.json")).unwrap_or_else(|error| panic!("🧬️schema/🔣️.json: {error}"))
}

fn joined(parts: &[Value]) -> Value {
    Value::Object(parts.iter().filter_map(Value::as_object).flat_map(|part| part.iter().map(|(key, value)| (key.clone(), value.clone()))).collect())
}

fn actor() -> Value {
    let circling = json!({"live": true, "inside": 120, "sx": 1, "sy": -1, "px": 30.5, "py": -12, "turn": -1, "quarters": 2, "steps": 3, "against": 1, "first": 90, "last": 118, "open": 900, "near": 640, "far": 1210.25, "rest": 0});
    let stroking = json!({"live": false, "way": 0, "from": 0, "began": 0, "peak": 0, "reached": 0, "top": 0, "bottom": 0, "topSince": 0, "bottomSince": 0, "count": -1, "first": 0, "second": 0, "rest": 64});
    let shot = json!({"surface": "card", "facing": 1, "muzzle": {"x": 14, "y": 450}, "hook": {"x": 120, "y": 300}, "length": 180.5, "reel": "swing"});
    let hang = json!({"grip": {"x": 10, "y": 440, "vx": 3, "vy": -1.5}, "bob": {"x": 11, "y": 478}, "previous": {"x": 10.75, "y": 479}});
    let canopy = json!({"x": 10, "y": 430, "vx": 2, "vy": 96, "bob": {"x": 10.5, "y": 474}, "previous": {"x": 10.25, "y": 472.5}});
    joined(&[
        json!({"species": "blobby", "perch": "floor", "host": null, "pitch": {"wall": "card-left", "surface": "card", "side": -1, "x": 200, "y0": 300, "y1": 420}, "grip": 383.75, "footing": "perch", "x": 10.5, "y": 480, "vx": 0, "vy": -2.25, "tilt": -0.125, "facing": -1, "faced": 104, "activity": "squabble", "since": 100, "until": 160, "goal": 12, "partner": "hoppy", "clip": null}),
        json!({"gaze": {"x": 0.125, "y": -0.25, "vx": 0, "vy": 1}, "blink": 140, "needs": {"energy": 0.5, "sociability": 0.25, "curiosity": 1}, "opacity": 1, "leaving": false, "draws": 4_294_967_295_u32}),
        json!({"feeling": {"mood": "grumpy", "intensity": 0.75, "since": 96}, "state": "glowing", "stateSince": 40, "former": "resting", "trick": null, "warmth": {"heat": 2.5, "since": 110, "until": 0, "tier": "trick", "run": 2, "tricks": 1}}),
        json!({"hover": {"circling": circling, "stroking": stroking}, "hang": hang, "chute": {"since": 60, "open": 1.125, "opening": -0.5, "canopy": canopy}}),
        json!({"rope": {"shot": shot, "since": 70, "length": 120, "hand": {"x": 30, "y": 440}, "before": {"x": 29, "y": 442}, "caught": true}, "emitters": [{"emitter": "sparkle", "since": 90, "until": null}, {"emitter": "hum", "since": 20, "until": 150}]}),
    ])
}

fn stage() -> Value {
    let press = json!({"phase": "armed", "x": 12.5, "y": 460, "since": 120, "slop": 6});
    let shaking = json!({"live": true, "ax": 10, "ay": 440, "at": 100, "fx": 52, "fy": 438, "reached": 110, "count": 2, "mark1": 0, "mark2": 80, "mark3": 100, "rest": 0});
    let ladder = json!({"owner": "hoppy", "wall": "card-left", "surface": "floor", "side": -1, "foot": {"x": 180, "y": 480}, "top": {"x": 200, "y": 320}, "since": 30, "until": 1310, "rider": null});
    let lift = json!({"fixture": "f1", "pusher": "blobby", "since": 100, "side": 1, "room": 40, "span": 300, "unit": 0.25});
    joined(&[
        json!({"seed": 4_294_967_295_u32, "tick": 128, "mode": "calm", "quiet": false, "width": 640, "height": 480, "pointer": {"x": 1.5, "y": 2}, "pointed": 64, "over": "control", "glances": [{"x": 3, "y": 4}]}),
        json!({"surfaces": [{"id": "floor", "x0": 0, "x1": 640, "y": 480}], "keepouts": [{"x": 1, "y": 2, "width": 3, "height": 4}], "walls": [{"id": "card-left", "surface": "card", "side": -1, "x": 200, "y0": 300, "y1": 420}]}),
        json!({"fixtures": [{"id": "f1", "key": "quiz/task", "x": 210, "y": 320, "width": 300, "height": 28}], "perches": [{"surface": "floor", "x0": 0, "x1": 640, "y": 480}], "pitches": [{"wall": "card-left", "surface": "card", "side": -1, "x": 200, "y0": 300, "y1": 420}], "wanted": ["blobby", "hoppy"], "actors": [actor()]}),
        json!({"rapports": [{"between": ["blobby", "hoppy"], "drift": -0.25}], "met": -640, "draws": 3, "play": true, "mischief": false, "stirred": 120, "scrolled": -16, "press": press, "touched": "blobby", "shaking": shaking}),
        json!({"trail": [{"x": 10, "y": 440}, {"x": 12, "y": 441.5}], "coolings": [{"reaction": "glow-cheers", "when": "blobby", "near": "hoppy", "until": 2048}], "pledges": [{"between": ["blobby", "hoppy"], "encounter": "cuddle", "until": 2048}]}),
        json!({"ladders": [ladder], "lift": lift, "rested": 0, "poofs": 2, "puffs": [{"x": 300, "y": 456, "width": 40, "height": 48, "tick": 120}]}),
        json!({"claims": [{"owner": "hoppy", "slices": [{"from": 128, "until": 131, "extent": {"x0": 296, "y0": 400.5, "x1": 340, "y1": 452}}], "rest": null}], "origin": {"x": 10.5, "y": 480}}),
        json!({"courses": [{"owner": "hoppy", "from": 128, "steps": [{"x": 318, "y": 448, "vx": 0, "vy": 96, "tilt": 0, "canopy": null}, {"x": 318, "y": 449.5, "vx": 0, "vy": 96, "tilt": 0.0625, "canopy": {"x": 318, "y": 405, "vx": 0, "vy": 96, "bob": {"x": 318, "y": 449.5}, "previous": {"x": 318, "y": 448}}}], "ending": "head", "landing": "blobby", "touch": 96}]}),
        json!({"trips": [{"owner": "blobby", "from": 129, "steps": [{"x": 180, "y": 480, "footing": "perch", "perch": "floor", "activity": "walk", "facing": 1, "hold": -1}, {"x": 180, "y": 479.5, "footing": "wall", "perch": null, "activity": "climb", "facing": 1, "hold": 0}], "pitches": [{"wall": "card-left", "surface": "card", "side": -1, "x": 200, "y0": 300, "y1": 420}], "ladder": null, "ending": "wall", "landing": "", "grip": 383}]}),
    ])
}

fn frame() -> Value {
    let tools = json!([{"kind": "chute", "open": 0.75, "sway": -0.02}, {"kind": "rope", "x": 120, "y": 300, "slack": 4}, {"kind": "hook", "x": 120, "y": 300}, {"kind": "gun", "aim": 0.875}, {"kind": "ladder", "lean": 0.25, "length": 84}]);
    let actor = joined(&[
        json!({"species": "blobby", "x": 10.5, "y": 480, "facing": 1, "activity": "greet", "opacity": 0.5, "bones": [1, 0, 0, 1, 0, -12.5], "eyes": [{"x": 0.5, "y": -0.5, "lid": 1}]}),
        json!({"footing": "hand", "state": "resting", "mood": "happy", "intensity": 0.25, "spirits": 0.25, "tilt": -0.05, "pivot": {"x": 0, "y": -27}, "tools": tools, "body": {"x": -5.5, "y": 450, "width": 32, "height": 30}}),
    ]);
    let ladders = json!([{"x0": 180, "y0": 480, "x1": 200, "y1": 320, "rungs": 15, "opacity": 1}]);
    let particles = json!([{"species": "blobby", "emitter": "sparkle", "x": 12, "y": 440, "scale": 1.2, "rotation": 0.125, "opacity": 0.5}]);
    json!({"tick": 128, "actors": [actor], "rate": 32, "wake": null, "ladders": ladders, "particles": particles, "lifts": [{"fixture": "f1", "dx": 12.5, "dy": -1, "tilt": 0.002, "opacity": 1}], "puffs": [{"x": 300, "y": 405, "width": 40, "height": 30, "phase": 0.25}], "held": "blobby"})
}

fn without(document: &Value, member: &str) -> Value {
    let mut copy = document.clone();
    copy.as_object_mut().unwrap_or_else(|| panic!("not an object: {document}")).remove(member);
    copy
}

fn with(document: &Value, member: &str, value: Value) -> Value {
    let mut copy = document.clone();
    copy[member] = value;
    copy
}

#[test]
fn every_fixture_document_round_trips_through_the_twin() {
    let conformance = fixture("schema-conformance");
    round_trips::<Menagerie>("schema-conformance/menagerie", &conformance["menagerie"]);
    round_trips::<Ensemble>("schema-conformance/ensemble", &conformance["ensemble"]);
    assert_eq!(entries(&conformance["species"]).len(), 3);
    for species in entries(&conformance["species"]) {
        round_trips::<Species>(&format!("schema-conformance/species/{}", species["path"]), &species["document"]);
    }
    round_trips::<Species>("schema-conformance/bases/species", &conformance["bases"]["species"]);
    round_trips::<Menagerie>("schema-conformance/bases/menagerie", &conformance["bases"]["menagerie"]);
    round_trips::<Ensemble>("schema-conformance/bases/ensemble", &conformance["bases"]["ensemble"]);
    let mut documents = 0;
    for group in ["accepted", "rules"] {
        for vector in entries(&conformance[group]) {
            let context = format!("schema-conformance/{group}/{}", vector["id"]);
            let document = match vector["pointer"].as_str() {
                Some(pointer) => conformance.pointer(pointer).unwrap_or_else(|| panic!("{context}: {pointer} reaches nothing")),
                None => &vector["document"],
            };
            round_trips_as(&context, vector["definition"].as_str().unwrap_or_default(), document);
            documents += 1;
        }
    }
    assert_eq!(documents, entries(&conformance["accepted"]).len() + entries(&conformance["rules"]).len());
    assert!(documents >= 81, "{documents} documents");
    let rigs = fixture("rig-solving");
    assert_eq!(entries(&rigs["species"]).len(), 2);
    for species in entries(&rigs["species"]) {
        round_trips::<Species>(&format!("rig-solving/species/{}", species["id"]), species);
    }
    let gazes = fixture("gaze-tracking");
    for vector in entries(&gazes["offsets"]).iter().chain(entries(&gazes["eyes"])) {
        round_trips::<Point>("gaze-tracking/eye", &vector["eye"]);
        round_trips::<Point>("gaze-tracking/target", &vector["target"]);
    }
    let animation = fixture("animation-sampling");
    round_trips::<Species>("animation-sampling/species", &animation["species"]);
    assert!(!entries(&animation["tracks"]).is_empty());
    for vector in entries(&animation["tracks"]) {
        round_trips::<Track>(&format!("animation-sampling/tracks/{}", vector["id"]), &vector["track"]);
    }
    for vector in entries(&animation["easings"]) {
        round_trips::<Ease>(&format!("animation-sampling/easings/{}", vector["id"]), &vector["ease"]);
    }
    let terrain = fixture("terrain-walking");
    assert!(!entries(&terrain["layouts"]).is_empty());
    for layout in entries(&terrain["layouts"]) {
        let context = format!("terrain-walking/layouts/{}", layout["id"]);
        round_trips::<Vec<Surface>>(&context, &layout["surfaces"]);
        round_trips::<Vec<Rect>>(&context, &layout["keepouts"]);
        round_trips::<Vec<Perch>>(&context, &layout["expected"]);
    }
    let bonds = fixture("bond-dynamics");
    for vector in entries(&bonds["affinities"]) {
        round_trips::<Vec<Bond>>("bond-dynamics/bonds", &vector["bonds"]);
        round_trips::<Vec<Rapport>>("bond-dynamics/rapports", &vector["rapports"]);
    }
    for vector in entries(&bonds["needs"]) {
        round_trips::<Needs>("bond-dynamics/needs", &vector["needs"]);
        round_trips::<Temperament>("bond-dynamics/temperament", &vector["temperament"]);
        round_trips::<Activity>("bond-dynamics/activity", &vector["activity"]);
    }
    for vector in entries(&fixture("behavior-choice")["casts"]) {
        round_trips::<Cast>("behavior-choice/cast", &vector["cast"]);
    }
}

#[test]
fn the_stage_trace_menagerie_and_every_scripted_event_round_trip() {
    let trace = fixture("stage-trace");
    round_trips::<Menagerie>("stage-trace/menagerie", &trace["menagerie"]);
    let mut kinds = std::collections::BTreeSet::new();
    for script in entries(&trace["scripts"]) {
        for step in entries(&script["steps"]) {
            for event in entries(&step["events"]) {
                round_trips::<StageEvent>(&format!("stage-trace/{}", script["id"]), event);
                kinds.insert(event["kind"].as_str().unwrap_or_default().to_string());
            }
        }
    }
    for kind in ["surveyed", "summoned", "pointed", "unpointed", "glanced", "tuned", "hushed", "pressed", "released"] {
        assert!(kinds.contains(kind), "no scripted event is {kind}");
    }
}

#[test]
fn enumerations_spell_the_literals_of_the_normative_file_in_its_order() {
    let definitions = &normative()["$defs"];
    assert_eq!(json(&PAINTS), definitions["Paint"]["enum"]);
    assert_eq!(json(&CHANNELS), definitions["Channel"]["enum"]);
    assert_eq!(json(&GAITS), definitions["Gait"]["enum"]);
    assert_eq!(json(&ACTIVITIES), definitions["Activity"]["enum"]);
    assert_eq!(json(&PET_MODES), definitions["PetMode"]["enum"]);
    assert_eq!(json(&MOODS), definitions["Mood"]["enum"]);
    assert_eq!(json(&FOOTINGS), definitions["Footing"]["enum"]);
    assert_eq!(json(&GEARS), definitions["Gear"]["enum"]);
    assert_eq!(json(&CUES), definitions["Cue"]["enum"]);
    assert_eq!(json(&DRIFTS), definitions["Drift"]["enum"]);
    assert_eq!(json(&DEEDS), definitions["Deed"]["enum"]);
    assert_eq!(json(&POINTERS), definitions["Pointer"]["enum"]);
    assert_eq!(json(&PRESS_PHASES), definitions["PressPhase"]["enum"]);
    assert_eq!(json(&TIERS), definitions["Tier"]["enum"]);
    assert_eq!(json(&REELINGS), definitions["Shot"]["properties"]["reel"]["enum"]);
    assert_eq!(json(&OVERS), definitions["Pointed"]["properties"]["over"]["enum"]);
    assert_eq!(json(&OVERS), definitions["Stage"]["properties"]["over"]["enum"]);
    assert_eq!(json(&ENDINGS), definitions["Course"]["properties"]["ending"]["enum"]);
    assert_eq!(json(&ARRIVALS), definitions["Trip"]["properties"]["ending"]["enum"]);
    assert_eq!(definitions["Foothold"]["properties"]["footing"]["$ref"], json!("#/$defs/Footing"));
    assert_eq!(definitions["Foothold"]["properties"]["activity"]["$ref"], json!("#/$defs/Activity"));
    for (definition, member, target) in [("Actor", "footing", "Footing"), ("ActorFrame", "footing", "Footing"), ("ActorFrame", "mood", "Mood"), ("Pressed", "pointer", "Pointer"), ("Press", "phase", "PressPhase"), ("Warmth", "tier", "Tier"), ("Played", "deed", "Deed")] {
        assert_eq!(definitions[definition]["properties"][member]["$ref"], json!(format!("#/$defs/{target}")), "{definition}.{member}");
    }
    assert_eq!(json(&[Activity::Greet, Activity::Cuddle, Activity::Squabble]), definitions["Pledge"]["properties"]["encounter"]["enum"]);
    for (definition, member) in [("Wall", "side"), ("Pitch", "side"), ("Shot", "facing"), ("Foothold", "facing"), ("Ladder", "side"), ("Prank", "side")] {
        assert_eq!(json(&FACINGS), definitions[definition]["properties"][member]["enum"], "{definition}.{member}");
    }
    assert_eq!(json(&PARTIES), definitions["Effect"]["properties"]["on"]["enum"]);
    assert_eq!(json(&PLACEMENTS), definitions["Reaction"]["properties"]["where"]["enum"]);
    assert_eq!(json(&[Activity::Greet, Activity::Cuddle, Activity::Squabble]), definitions["Effect"]["properties"]["encounter"]["enum"]);
    assert_eq!(Mood::default(), MOODS[0]);
    assert_eq!(json(&MENAGERIE_SCHEMA), definitions["Menagerie"]["properties"]["schema"]["const"]);
    assert_eq!(json(&ENSEMBLE_SCHEMA), definitions["Ensemble"]["properties"]["schema"]["const"]);
    assert_eq!(json(&LANGUAGES), definitions["Text"]["required"]);
    assert_eq!(json(&ACTIVITIES.map(|activity| activity.as_str())), definitions["Activity"]["enum"]);
    assert_eq!(definitions["Repertoire"]["properties"].as_object().map(|properties| properties.len()), Some(ACTIVITIES.len()));
    assert_eq!(TICKS_PER_SECOND, 64);
    assert!(definitions["Ticks"]["description"].as_str().is_some_and(|description| description.contains("64 ticks per second")));
    assert_eq!(json(&FACINGS), definitions["Actor"]["properties"]["facing"]["enum"]);
    assert_eq!(json(&FACINGS), definitions["ActorFrame"]["properties"]["facing"]["enum"]);
    assert_eq!(json(&RATES), definitions["Frame"]["properties"]["rate"]["enum"]);
    assert_eq!(definitions["Ticked"]["properties"]["ticks"]["minimum"], 0);
    assert_eq!(definitions["Stage"]["properties"]["tick"]["minimum"], 0);
    assert_eq!(definitions["Frame"]["properties"]["tick"]["minimum"], 0);
}

#[test]
fn closed_sets_of_numbers_decode_nothing_the_contract_refuses() {
    assert_eq!(FACINGS.map(Facing::sign), [1.0, -1.0]);
    assert_eq!(FACINGS.map(Facing::reversed), [Facing::Left, Facing::Right]);
    assert_eq!(FACINGS.map(i8::from), [1, -1]);
    assert_eq!(RATES.map(u8::from), [0, 16, 32, 64]);
    assert!(RATES.windows(2).all(|pair| pair[0] < pair[1]));
    for number in [0, 2, -2, 5, 127] {
        assert!(serde_json::from_value::<Facing>(json!(number)).is_err(), "facing {number}");
        assert!(serde_json::from_value::<Actor>(with(&actor(), "facing", json!(number))).is_err(), "an actor facing {number}");
    }
    for number in [1, 7, 8, 48, 128, 255] {
        assert!(serde_json::from_value::<Rate>(json!(number)).is_err(), "rate {number}");
    }
    assert!(serde_json::from_value::<Rate>(json!(-16)).is_err());
    assert!(serde_json::from_value::<Facing>(json!("right")).is_err());
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "ticked", "ticks": -1})).is_err());
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "ticked", "ticks": 0})).is_ok());
    assert!(serde_json::from_value::<Stage>(with(&stage(), "tick", json!(-1))).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage(), "tick", json!(0))).is_ok());
    let frame = json!({"tick": 0, "actors": [], "rate": 0, "wake": null, "ladders": [], "particles": [], "lifts": [], "puffs": [], "held": null});
    assert_eq!(typed::<Frame>(&frame), Frame { tick: 0, actors: Vec::new(), rate: Rate::Rest, wake: None, ladders: Vec::new(), particles: Vec::new(), lifts: Vec::new(), puffs: Vec::new(), held: None });
    for (definition, value) in [("Wall", json!({"id": "w", "surface": "s", "side": 0, "x": 0, "y0": 0, "y1": 1})), ("Ladder", with(&stage()["ladders"][0], "side", json!(2))), ("Prank", with(&stage()["lift"], "side", json!(-2)))] {
        let decodes = match definition {
            "Wall" => serde_json::from_value::<Wall>(value).is_ok(),
            "Ladder" => serde_json::from_value::<Ladder>(value).is_ok(),
            _ => serde_json::from_value::<Prank>(value).is_ok(),
        };
        assert!(!decodes, "{definition} with a side that is no facing");
    }
    assert!(serde_json::from_value::<Pledge>(with(&stage()["pledges"][0], "encounter", json!("sulk"))).is_err());
    assert!(serde_json::from_value::<Shot>(with(&actor()["rope"]["shot"], "reel", json!("yank"))).is_err());
    assert!(serde_json::from_value::<Pointed>(json!({"x": 1, "y": 2, "over": "text"})).is_err());
    assert!(serde_json::from_value::<Frame>(with(&frame, "tick", json!(-1))).is_err());
    assert!(serde_json::from_value::<Frame>(with(&frame, "rate", json!(7))).is_err());
    assert!(serde_json::from_value::<Frame>(with(&frame, "wake", json!(-64))).is_ok());
}

#[test]
fn every_definition_of_the_normative_file_has_a_twin_under_its_name() {
    let source = include_str!("../../🦀️.rs");
    let declared: std::collections::BTreeSet<&str> = source
        .lines()
        .filter_map(|line| ["pub struct ", "pub enum ", "pub type "].iter().find_map(|keyword| line.strip_prefix(keyword)))
        .map(|rest| rest.split(|character: char| !character.is_ascii_alphanumeric()).next().unwrap_or_default())
        .collect();
    let normative = normative();
    let defined: std::collections::BTreeSet<&str> = normative["$defs"].as_object().into_iter().flat_map(|definitions| definitions.keys().map(String::as_str)).collect();
    assert_eq!(defined.len(), 121);
    let inline: std::collections::BTreeSet<&str> = ["Facing", "Rate", "Party", "Placement", "Over", "Reeling", "Ending", "Arrival"].into_iter().collect();
    assert!(inline.is_subset(&declared) && inline.is_disjoint(&defined), "Facing and Rate type the inline number enums of Actor, ActorFrame, Frame, Wall, Pitch, Shot, Trip, Ladder and Prank, Party, Placement, Over, Reeling, Ending and Arrival the inline word enums of Effect, Reaction, Pointed, Shot, Course and Trip");
    assert_eq!(declared.difference(&inline).copied().collect::<std::collections::BTreeSet<&str>>(), defined);
}

#[test]
fn tagged_unions_carry_their_kind_on_the_wire() {
    assert_eq!(json(&Shape::Ellipse(EllipseShape { cx: 0.0, cy: -12.0, rx: 16.0, ry: 12.0 })), json!({"kind": "ellipse", "cx": 0.0, "cy": -12.0, "rx": 16.0, "ry": 12.0}));
    assert_eq!(json(&Shape::Path(PathShape { d: "M 0 0 L 1 1".to_string() })), json!({"kind": "path", "d": "M 0 0 L 1 1"}));
    assert_eq!(json(&Shape::Rect(RectShape { x: 0.0, y: 0.0, width: 2.0, height: 3.0, radius: None })), json!({"kind": "rect", "x": 0.0, "y": 0.0, "width": 2.0, "height": 3.0}));
    assert_eq!(json(&Shape::Line(LineShape { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0 })), json!({"kind": "line", "x1": 0.0, "y1": 1.0, "x2": 2.0, "y2": 3.0}));
    for event in [
        json!({"kind": "ticked", "ticks": 8}),
        json!({"kind": "pointed", "x": 1.5, "y": 2, "over": "free"}),
        json!({"kind": "pointed", "x": 1.5, "y": 2, "over": "control"}),
        json!({"kind": "unpointed"}),
        json!({"kind": "glanced", "points": [{"x": 1, "y": 2}]}),
        json!({"kind": "surveyed", "width": 640, "height": 480, "surfaces": [{"id": "floor", "x0": 0, "x1": 640, "y": 480}], "keepouts": [{"x": 1, "y": 2, "width": 3, "height": 4}], "walls": [{"id": "card-right", "surface": "card", "side": 1, "x": 500, "y0": 300, "y1": 420}], "fixtures": [{"id": "f1", "key": "quiz", "x": 1, "y": 2, "width": 3, "height": 4}]}),
        json!({"kind": "summoned", "species": ["blobby"]}),
        json!({"kind": "tuned", "mode": "lively"}),
        json!({"kind": "hushed", "quiet": true}),
        json!({"kind": "pressed", "x": 3, "y": 4, "pointer": "pen"}),
        json!({"kind": "dragged", "x": 5, "y": 6}),
        json!({"kind": "released", "x": 7, "y": 8}),
        json!({"kind": "cancelled"}),
        json!({"kind": "reclaimed", "fixture": "f1"}),
        json!({"kind": "stirred"}),
        json!({"kind": "scrolled"}),
        json!({"kind": "played", "species": "blobby", "deed": "toss"}),
        json!({"kind": "permitted", "play": true, "mischief": false}),
    ] {
        round_trips::<StageEvent>("event", &event);
        assert!(serde_json::from_value::<StageEvent>(with(&event, "extra", json!(1))).is_err(), "{event}");
    }
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "poked", "x": 3, "y": 4})).is_err(), "poked is no event any more");
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "pointed", "x": 1.5, "y": 2})).is_err(), "a pointer over nothing named");
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "pressed", "x": 3, "y": 4, "pointer": "stylus"})).is_err());
    assert_eq!(typed::<StageEvent>(&json!({"kind": "cancelled"})), StageEvent::Cancelled(Cancelled {}));
    for tool in entries(&frame()["actors"][0]["tools"]) {
        round_trips::<ToolFrame>("tool", tool);
        assert!(serde_json::from_value::<ToolFrame>(with(tool, "extra", json!(1))).is_err(), "{tool}");
    }
    assert_eq!(json(&ToolFrame::Gun(GunTool { aim: 0.5 })), json!({"kind": "gun", "aim": 0.5}));
    assert!(serde_json::from_value::<ToolFrame>(json!({"kind": "parasol", "open": 1})).is_err());
    assert_eq!(typed::<StageEvent>(&json!({"kind": "unpointed"})), StageEvent::Unpointed(Unpointed {}));
    assert_eq!(typed::<StageEvent>(&json!({"kind": "ticked", "ticks": 3})), StageEvent::Ticked(Ticked { ticks: 3 }));
    assert!(serde_json::from_value::<StageEvent>(json!({"kind": "whistled"})).is_err());
    assert!(serde_json::from_value::<StageEvent>(json!({"ticks": 3})).is_err());
    assert!(serde_json::from_value::<Shape>(json!({"kind": "circle", "r": 3})).is_err());
    assert_eq!(json(&Channel::ScaleX), json!("scaleX"));
    assert_eq!(json(&Part { id: "body".to_string(), bone: "root".to_string(), shape: Shape::Line(LineShape { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 }), fill: Paint::None, stroke: Paint::Ink, stroke_width: Some(1.5) })["strokeWidth"], json!(1.5));
    assert_eq!(json(&Clip { id: "breathe".to_string(), seconds: 3.0, looping: true, tracks: Vec::new() }), json!({"id": "breathe", "seconds": 3.0, "loop": true, "tracks": []}));
}

#[test]
fn stages_and_frames_round_trip_with_their_nullable_members() {
    let stage = stage();
    round_trips::<Stage>("stage", &stage);
    let decoded: Stage = typed(&stage);
    assert_eq!((decoded.seed, decoded.met, decoded.actors[0].draws, decoded.actors[0].facing, decoded.actors[0].faced), (u32::MAX, -640, u32::MAX, Facing::Left, 104));
    assert_eq!(decoded.actors[0].clip, None);
    assert!(serde_json::from_value::<Actor>(without(&actor(), "faced")).is_err(), "an actor without faced");
    assert_eq!((decoded.actors[0].state_since, decoded.actors[0].warmth.tier, decoded.actors[0].hover.stroking.count, decoded.actors[0].emitters[0].until), (40, Tier::Trick, -1, None));
    assert_eq!((decoded.press.phase, decoded.touched.as_deref(), decoded.pledges[0].encounter, decoded.ladders[0].side, decoded.poofs, decoded.over), (PressPhase::Armed, Some("blobby"), Activity::Cuddle, Facing::Left, 2, Over::Control));
    assert_eq!(json(&decoded.actors[0])["stateSince"], json!(40));
    let mut empty = with(&with(&with(&with(&stage, "pointer", Value::Null), "touched", Value::Null), "lift", Value::Null), "ladders", json!([with(&stage["ladders"][0], "rider", json!("hoppy"))]));
    let mut bare = actor();
    for member in ["perch", "pitch", "partner", "trick", "hang", "chute", "rope"] {
        bare = with(&bare, member, Value::Null);
    }
    empty = with(&empty, "actors", json!([bare]));
    round_trips::<Stage>("stage without a pointer, a press, a lift or anything in an actor's hands", &empty);
    for member in ["perch", "pitch", "grip", "footing", "partner", "clip", "feeling", "state", "stateSince", "former", "trick", "warmth", "hover", "hang", "chute", "rope", "emitters"] {
        assert!(serde_json::from_value::<Actor>(without(&actor(), member)).is_err(), "an actor without {member}");
    }
    for member in ["mood", "spirits"] {
        assert!(serde_json::from_value::<Actor>(with(&actor(), member, json!(0.5))).is_err(), "an actor's {member} is read off its feeling, never stored");
    }
    assert_eq!((decoded.courses[0].ending, decoded.courses[0].steps[1].canopy.map(|canopy| canopy.y), decoded.claims[0].rest, decoded.origin), (Ending::Head, Some(405.0), None, Some(Point { x: 10.5, y: 480.0 })));
    for member in ["pointer", "over", "walls", "fixtures", "pitches", "play", "mischief", "stirred", "scrolled", "press", "touched", "shaking", "trail", "coolings", "pledges", "ladders", "lift", "rested", "poofs", "puffs", "claims", "courses", "trips", "origin"] {
        assert!(serde_json::from_value::<Stage>(without(&stage, member)).is_err(), "a stage without {member}");
    }
    assert_eq!((decoded.trips[0].ending, decoded.trips[0].steps[1].footing, decoded.trips[0].steps[1].hold, decoded.trips[0].ladder.as_deref(), decoded.actors[0].pitch.as_ref().map(|pitch| pitch.side), decoded.actors[0].rope.as_ref().map(|rope| rope.caught)), (Arrival::Wall, Footing::Wall, 0, None, Some(Facing::Left), Some(true)));
    assert!(serde_json::from_value::<Trip>(with(&stage["trips"][0], "ending", json!("head"))).is_err(), "a trip ends on a perch, on a wall or in the air");
    assert_eq!((decoded.trips[0].steps[0].perch.as_deref(), decoded.trips[0].steps[0].facing, decoded.trips[0].steps[1].perch.as_deref()), (Some("floor"), Facing::Right, None));
    for member in ["perch", "facing"] {
        assert!(serde_json::from_value::<Foothold>(without(&stage["trips"][0]["steps"][0], member)).is_err(), "a foothold without {member}");
    }
    assert!(serde_json::from_value::<Trip>(with(&stage["trips"][0], "facing", json!(1))).is_err(), "a trip faces the way of each of its footholds, never one way for all");
    assert!(serde_json::from_value::<Rope>(without(&actor()["rope"], "caught")).is_err(), "a rope without caught");
    assert!(serde_json::from_value::<Stage>(with(&stage, "seed", json!(4_294_967_296_u64))).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage, "tick", json!(1.5))).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage, "mode", json!("off"))).is_err());
    let frame = frame();
    round_trips::<Frame>("frame", &frame);
    assert_eq!(typed::<Frame>(&frame).actors[0].activity, Activity::Greet);
    assert_eq!((typed::<Frame>(&frame).actors[0].mood, typed::<Frame>(&frame).actors[0].footing, typed::<Frame>(&frame).actors[0].tools.len()), (Mood::Happy, Footing::Hand, 5));
    round_trips::<Frame>("frame that holds nobody", &with(&frame, "held", Value::Null));
    for member in ["footing", "state", "mood", "intensity", "spirits", "tilt", "pivot", "tools", "body"] {
        assert!(serde_json::from_value::<Frame>(with(&frame, "actors", json!([without(&frame["actors"][0], member)]))).is_err(), "an actor frame without {member}");
    }
    for member in ["ladders", "particles", "lifts", "puffs", "held"] {
        assert!(serde_json::from_value::<Frame>(without(&frame, member)).is_err(), "a frame without {member}");
    }
    assert!(serde_json::from_value::<Frame>(with(&frame, "actors", json!([with(&frame["actors"][0], "mood", json!(0.25))]))).is_err(), "a frame's mood is a mood, its spirits a number");
    assert!(serde_json::from_value::<Frame>(with(&frame, "actors", json!([without(&frame["actors"][0], "activity")]))).is_err(), "an actor frame without its activity");
    round_trips::<Frame>("frame with a wake tick", &with(&frame, "wake", json!(192)));
    assert!(serde_json::from_value::<Frame>(without(&frame, "wake")).is_err());
    assert!(serde_json::from_value::<Frame>(with(&frame, "extra", json!(1))).is_err());
}

#[test]
fn undeclared_members_and_wrong_tuples_are_refused_like_the_schema_refuses_them() {
    let conformance = fixture("schema-conformance");
    let species = &conformance["bases"]["species"];
    assert!(serde_json::from_value::<Species>(species.clone()).is_ok());
    assert!(serde_json::from_value::<Species>(with(species, "colour", json!("red"))).is_err());
    assert!(serde_json::from_value::<Species>(without(species, "size")).is_err());
    assert!(serde_json::from_value::<Species>(with(species, "$schema", json!(7))).is_err());
    assert!(serde_json::from_value::<Text>(json!({"en": "a", "de": "b", "fr": "c"})).is_err());
    assert!(serde_json::from_value::<Text>(json!({"en": "a"})).is_err());
    assert!(serde_json::from_value::<Key>(json!({"at": 0, "value": 1, "ease": [0.25, 0.1, 0.25]})).is_err());
    assert!(serde_json::from_value::<Key>(json!({"at": 0, "value": 1, "ease": [0.25, 0.1, 0.25, 1, 0]})).is_err());
    assert!(serde_json::from_value::<Key>(json!({"at": 0, "value": 1, "ease": [0.25, 0.1, 0.25, 1]})).is_ok());
    assert!(serde_json::from_value::<Bond>(json!({"between": ["a"], "affinity": 0})).is_err());
    assert!(serde_json::from_value::<Bond>(json!({"between": ["a", "b", "c"], "affinity": 0})).is_err());
    assert!(serde_json::from_value::<Shape>(json!({"kind": "ellipse", "cx": 0, "cy": 0, "rx": 1, "ry": 1, "d": "M 0 0"})).is_err());
    assert!(serde_json::from_value::<Repertoire>(json!({"idle": ["breathe"], "dance": ["jig"]})).is_err());
    assert!(serde_json::from_value::<Locomotion>(json!({"gait": "swim", "speed": 3})).is_err());
    let mut refused = 0;
    for vector in entries(&conformance["structural"]) {
        let decodes = match vector["definition"].as_str() {
            Some("Species") => serde_json::from_value::<Species>(vector["document"].clone()).is_ok(),
            Some("Menagerie") => serde_json::from_value::<Menagerie>(vector["document"].clone()).is_ok(),
            Some("Ensemble") => serde_json::from_value::<Ensemble>(vector["document"].clone()).is_ok(),
            other => panic!("{}: unknown definition {other:?}", vector["id"]),
        };
        if decodes {
            assert_eq!(vector["id"], json!("other-schema-version"), "only the schema identifier is left to the validator");
        } else {
            refused += 1;
        }
    }
    assert_eq!(refused, entries(&conformance["structural"]).len() - 1);
}

#[test]
fn states_tricks_emitters_gear_and_chemistry_carry_their_wire_names_and_refuse_what_the_contract_refuses() {
    let conformance = fixture("schema-conformance");
    let walker: Species = typed(&conformance["species"][0]["document"]);
    assert_eq!(walker.states.iter().map(|state| state.id.as_str()).collect::<Vec<_>>(), ["resting", "glowing", "radiant"]);
    assert_eq!((walker.states[2].lasts, walker.states[2].then.as_deref()), (Some(12.0), Some("glowing")));
    assert_eq!(walker.states[1].tint, Some(Tint { body: Some("#35c9b8".to_string()), accent: None, detail: None }));
    assert_eq!(walker.tricks[2].cues, [Cue::Circle, Cue::Show]);
    assert_eq!(walker.tricks[1].from.as_deref(), Some(&["resting".to_string()][..]));
    assert_eq!((walker.tricks[1].to.as_deref(), walker.tricks[1].mood), (Some("glowing"), Some(Mood::Proud)));
    assert_eq!((walker.emitters[0].motion, walker.emitters[0].count, walker.emitters[1].stroke_width), (Drift::Burst, 6, Some(1.0)));
    assert_eq!(json(&walker.emitters[1])["strokeWidth"], json!(1.0));
    assert_eq!(walker.purr, Purr { clip: "nuzzle".to_string(), emitter: Some("hum".to_string()) });
    assert_eq!(walker.gear, GEARS);
    assert!(matches!(walker.canopy, Some(Shape::Path(_))));
    assert_eq!((walker.mood, walker.grip, walker.reach), (Mood::Content, 27.0, 10.0));
    let menagerie: Menagerie = typed(&conformance["menagerie"]);
    assert_eq!(menagerie.chemistry.iter().map(|reaction| reaction.place).collect::<Vec<_>>(), [None, Some(Placement::Below), Some(Placement::Above), None, None, None]);
    assert_eq!((menagerie.chemistry[3].affinity.as_deref(), menagerie.chemistry[3].when.held, menagerie.chemistry[4].when.trick.as_deref()), (Some(&[0.4, 1.0][..]), Some(4.0), Some("boing")));
    assert_eq!((menagerie.chemistry[4].unless.as_ref().and_then(|unless| unless.mood), menagerie.chemistry[5].when.species.as_deref(), menagerie.chemistry[5].then[0].activity), (Some(Mood::Scared), None, Some(Activity::Sleep)));
    assert_eq!(json(&menagerie.chemistry[1])["where"], json!("below"));
    assert_eq!((menagerie.chemistry[2].then[0].encounter, menagerie.chemistry[0].then[2].on), (Some(Activity::Greet), Party::When));
    assert_eq!(menagerie.chemistry[2].when, Trait { species: Some("floaty".to_string()), state: None, held: None, mood: Some(Mood::Sleepy), activity: Some(Activity::Idle), trick: None });
    let anyone: Trait = serde_json::from_value(json!({"state": "resting", "held": 2.5, "trick": "cheer"})).expect("a trait of any species");
    assert_eq!((anyone.species, anyone.held, anyone.trick.as_deref()), (None, Some(2.5), Some("cheer")));
    let bounded: Reaction = serde_json::from_value(with(&with(&json(&menagerie.chemistry[0]), "unless", json!({"species": "floaty"})), "affinity", json!([0.4, 1]))).expect("a reaction with unless and affinity");
    assert_eq!((bounded.unless.and_then(|unless| unless.species).as_deref(), bounded.affinity), (Some("floaty"), Some(vec![0.4, 1.0])));
    assert!(serde_json::from_value::<Reaction>(with(&json(&menagerie.chemistry[0]), "affinity", json!(0.4))).is_err());
    let effect = json!({"on": "near", "encounter": "cuddle"});
    round_trips::<Effect>("effect", &effect);
    for activity in ["idle", "sulk", "purr", "hug"] {
        assert!(serde_json::from_value::<Effect>(with(&effect, "encounter", json!(activity))).is_err(), "an effect asking for the encounter {activity}");
    }
    round_trips::<Effect>("effect", &json!({"on": "when", "activity": "sleep"}));
    for activity in ["idle", "greet", "trick", "nap"] {
        assert!(serde_json::from_value::<Effect>(json!({"on": "when", "activity": activity})).is_err(), "an effect setting a pet off on {activity}");
    }
    assert!(serde_json::from_value::<Effect>(json!({"on": "both"})).is_err());
    assert!(serde_json::from_value::<Effect>(json!({"on": "when", "extra": 1})).is_err());
    assert!(serde_json::from_value::<Effect>(json!({"state": "resting"})).is_err());
    let reaction = json(&menagerie.chemistry[0]);
    round_trips::<Reaction>("reaction", &reaction);
    for member in ["id", "when", "near", "within", "every", "then"] {
        assert!(serde_json::from_value::<Reaction>(without(&reaction, member)).is_err(), "a reaction without {member}");
    }
    assert!(serde_json::from_value::<Reaction>(with(&reaction, "where", json!("inside"))).is_err());
    let species = &conformance["bases"]["species"];
    for member in ["states", "tricks", "purr", "emitters", "gear", "grip", "reach", "mood"] {
        assert!(serde_json::from_value::<Species>(without(species, member)).is_err(), "a species without {member}");
    }
    assert!(serde_json::from_value::<Species>(with(species, "canopy", json!({"kind": "ellipse", "cx": 0, "cy": -4, "rx": 8, "ry": 4}))).is_ok());
    assert!(serde_json::from_value::<Species>(with(species, "mood", json!("hangry"))).is_err());
    assert!(serde_json::from_value::<Species>(with(species, "gear", json!(["jetpack"]))).is_err());
    assert!(serde_json::from_value::<Menagerie>(without(&conformance["bases"]["menagerie"], "chemistry")).is_err());
    assert!(serde_json::from_value::<Ensemble>(without(&conformance["bases"]["ensemble"], "chemistry")).is_err());
    let emitter = json(&walker.emitters[0]);
    for count in [json!(2.5), json!(-1), json!("six")] {
        assert!(serde_json::from_value::<Emitter>(with(&emitter, "count", count.clone())).is_err(), "an emitter of {count} particles");
    }
    assert!(serde_json::from_value::<Emitter>(with(&emitter, "motion", json!("swirl"))).is_err());
    assert!(serde_json::from_value::<Trick>(with(&json(&walker.tricks[0]), "cues", json!(["whistle"]))).is_err());
    assert!(serde_json::from_value::<Tint>(json!({"glow": "#ffffff"})).is_err());
    assert_eq!(json(&Tint::default()), json!({}));
    round_trips::<Repertoire>("repertoire", &json!({"hang": ["dangle"], "push": [], "trick": ["wiggle"]}));
}

#[test]
fn a_repertoire_answers_per_activity_like_an_index_into_the_document() {
    let repertoire: Repertoire = typed(&json!({"idle": ["breathe"], "walk": [], "fidget": ["wiggle", "spin"]}));
    assert_eq!(repertoire.clips(Activity::Idle), Some(&["breathe".to_string()][..]));
    assert_eq!(repertoire.clips(Activity::Walk), Some(&[][..]));
    assert_eq!(repertoire.clips(Activity::Fidget).map(<[Slug]>::len), Some(2));
    let document = json(&repertoire);
    for activity in ACTIVITIES {
        assert_eq!(repertoire.clips(activity).map(|clips| json(&clips)), document.get(activity.as_str()).cloned(), "{}", activity.as_str());
    }
    assert_eq!(json(&Repertoire::default()), json!({}));
}
