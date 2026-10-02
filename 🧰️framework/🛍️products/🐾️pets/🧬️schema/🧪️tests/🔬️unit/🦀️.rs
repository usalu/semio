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

fn actor() -> Value {
    json!({"species": "blobby", "perch": "floor", "x": 10.5, "y": 480, "vx": 0, "vy": -2.25, "facing": -1, "faced": 104, "activity": "squabble", "since": 100, "until": 160, "goal": 12, "partner": "hoppy", "clip": null, "gaze": {"x": 0.125, "y": -0.25, "vx": 0, "vy": 1}, "blink": 140, "mood": -0.5, "needs": {"energy": 0.5, "sociability": 0.25, "curiosity": 1}, "opacity": 1, "leaving": false, "draws": 4_294_967_295_u32})
}

fn stage() -> Value {
    json!({"seed": 4_294_967_295_u32, "tick": 128, "mode": "calm", "quiet": false, "width": 640, "height": 480, "pointer": {"x": 1.5, "y": 2}, "pointed": 64, "glances": [{"x": 3, "y": 4}], "surfaces": [{"id": "floor", "x0": 0, "x1": 640, "y": 480}], "keepouts": [{"x": 1, "y": 2, "width": 3, "height": 4}], "perches": [{"surface": "floor", "x0": 0, "x1": 640, "y": 480}], "wanted": ["blobby", "hoppy"], "actors": [actor()], "rapports": [{"between": ["blobby", "hoppy"], "drift": -0.25}], "met": -640, "draws": 3})
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
    for kind in ["surveyed", "summoned", "pointed", "unpointed", "glanced", "tuned", "hushed", "poked"] {
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
    let frame = json!({"tick": 0, "actors": [], "rate": 0, "wake": null});
    assert_eq!(typed::<Frame>(&frame), Frame { tick: 0, actors: Vec::new(), rate: Rate::Rest, wake: None });
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
    assert_eq!(defined.len(), 57);
    let inline: std::collections::BTreeSet<&str> = ["Facing", "Rate"].into_iter().collect();
    assert!(inline.is_subset(&declared) && inline.is_disjoint(&defined), "Facing and Rate type the inline number enums of Actor, ActorFrame and Frame");
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
        json!({"kind": "pointed", "x": 1.5, "y": 2}),
        json!({"kind": "unpointed"}),
        json!({"kind": "glanced", "points": [{"x": 1, "y": 2}]}),
        json!({"kind": "surveyed", "width": 640, "height": 480, "surfaces": [{"id": "floor", "x0": 0, "x1": 640, "y": 480}], "keepouts": [{"x": 1, "y": 2, "width": 3, "height": 4}]}),
        json!({"kind": "summoned", "species": ["blobby"]}),
        json!({"kind": "tuned", "mode": "lively"}),
        json!({"kind": "hushed", "quiet": true}),
        json!({"kind": "poked", "x": 3, "y": 4}),
    ] {
        round_trips::<StageEvent>("event", &event);
        assert!(serde_json::from_value::<StageEvent>(with(&event, "extra", json!(1))).is_err(), "{event}");
    }
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
    let empty = with(&with(&stage, "pointer", Value::Null), "actors", json!([with(&with(&actor(), "perch", Value::Null), "partner", Value::Null)]));
    round_trips::<Stage>("stage without a pointer", &empty);
    for member in ["perch", "partner", "clip"] {
        assert!(serde_json::from_value::<Actor>(without(&actor(), member)).is_err(), "an actor without {member}");
    }
    assert!(serde_json::from_value::<Stage>(without(&stage, "pointer")).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage, "seed", json!(4_294_967_296_u64))).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage, "tick", json!(1.5))).is_err());
    assert!(serde_json::from_value::<Stage>(with(&stage, "mode", json!("off"))).is_err());
    let frame = json!({"tick": 128, "actors": [{"species": "blobby", "x": 10.5, "y": 480, "facing": 1, "activity": "greet", "opacity": 0.5, "bones": [1, 0, 0, 1, 0, -12.5], "eyes": [{"x": 0.5, "y": -0.5, "lid": 1}], "mood": 0.25}], "rate": 32, "wake": null});
    round_trips::<Frame>("frame", &frame);
    assert_eq!(typed::<Frame>(&frame).actors[0].activity, Activity::Greet);
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
