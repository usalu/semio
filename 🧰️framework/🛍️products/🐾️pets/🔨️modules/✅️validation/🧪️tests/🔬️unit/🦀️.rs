//! 🩹️ Unit tests of the validation: every committed vector of the schema-conformance case (accepted, structurally broken, rule-breaking), the order and uniqueness of findings, the checks only a typed document can reach, and the assembly of a menagerie.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🧬️schema-conformance/🔣️.json — the shared vectors, judged by python-jsonschema and the TypeScript twin

use super::*;
use crate::schema::tests::{assert_same, entries, fixture, json, typed};
use serde_json::{json, Value};

const CODES: [IssueCode; 20] = [
    IssueCode::TypeInvalid,
    IssueCode::Required,
    IssueCode::PropertyUnknown,
    IssueCode::ValueInvalid,
    IssueCode::SlugInvalid,
    IssueCode::LengthInvalid,
    IssueCode::ItemsTooFew,
    IssueCode::DuplicateId,
    IssueCode::UnknownReference,
    IssueCode::BoneOrder,
    IssueCode::KeyOrder,
    IssueCode::LoopSeam,
    IssueCode::EaseRange,
    IssueCode::OutOfRange,
    IssueCode::SelfBond,
    IssueCode::DuplicateBond,
    IssueCode::MissingGaitClip,
    IssueCode::FloatHover,
    IssueCode::EmptyCast,
    IssueCode::DuplicateScene,
];

fn issues_of(definition: &str, document: &Value) -> Option<Vec<Issue>> {
    match definition {
        "Species" => serde_json::from_value::<Species>(document.clone()).ok().map(|species| species_issues(&species)),
        "Menagerie" => serde_json::from_value::<Menagerie>(document.clone()).ok().map(|menagerie| menagerie_issues(&menagerie)),
        "Ensemble" => serde_json::from_value::<Ensemble>(document.clone()).ok().map(|ensemble| ensemble_issues(&ensemble)),
        other => panic!("no validator for the definition {other}"),
    }
}

fn document_of<'a>(vectors: &'a Value, vector: &'a Value) -> &'a Value {
    match vector["pointer"].as_str() {
        Some(pointer) => vectors.pointer(pointer).unwrap_or_else(|| panic!("{}: {pointer} reaches nothing", vector["id"])),
        None => &vector["document"],
    }
}

fn definition_of(vector: &Value) -> &str {
    vector["definition"].as_str().unwrap_or_else(|| panic!("{}: no definition", vector["id"]))
}

fn issue(path: &str, code: IssueCode) -> Issue {
    Issue { path: path.to_string(), code }
}

fn base_species() -> Species {
    typed(&fixture("schema-conformance")["bases"]["species"])
}

fn base_menagerie() -> Menagerie {
    typed(&fixture("schema-conformance")["bases"]["menagerie"])
}

#[test]
fn every_accepted_document_decodes_and_has_no_finding() {
    let vectors = fixture("schema-conformance");
    assert_eq!(entries(&vectors["accepted"]).len(), 21);
    for vector in entries(&vectors["accepted"]) {
        assert_eq!(issues_of(definition_of(vector), document_of(&vectors, vector)), Some(Vec::new()), "{}", vector["id"]);
    }
}

#[test]
fn every_structurally_broken_document_is_refused_by_the_twin_or_by_the_validator() {
    let vectors = fixture("schema-conformance");
    assert_eq!(entries(&vectors["structural"]).len(), 26);
    let mut judged = Vec::new();
    for vector in entries(&vectors["structural"]) {
        match issues_of(definition_of(vector), &vector["document"]) {
            None => {}
            Some(issues) => {
                assert_eq!(json(&issues), vector["issues"], "{}", vector["id"]);
                assert!(!issues.is_empty(), "{}", vector["id"]);
                judged.push(vector["id"].as_str().unwrap_or_default());
            }
        }
    }
    assert_eq!(judged, ["other-schema-version"]);
}

#[test]
fn every_rule_document_yields_the_committed_findings() {
    let vectors = fixture("schema-conformance");
    assert_eq!(entries(&vectors["rules"]).len(), 63);
    let mut reported = BTreeSet::new();
    for vector in entries(&vectors["rules"]) {
        let issues = issues_of(definition_of(vector), &vector["document"]).unwrap_or_else(|| panic!("{}: the twin refuses a document of sound structure", vector["id"]));
        assert_eq!(json(&issues), vector["issues"], "{}", vector["id"]);
        assert!(!issues.is_empty(), "{}", vector["id"]);
        reported.extend(issues.iter().map(|issue| issue.code.as_str()));
    }
    let expected: BTreeSet<&str> = CODES.iter().map(IssueCode::as_str).filter(|code| !["type-invalid", "required", "property-unknown", "value-invalid"].contains(code)).collect();
    assert_eq!(reported, expected);
}

#[test]
fn every_code_spells_its_wire_string_and_every_committed_finding_decodes() {
    for code in CODES {
        assert_eq!(json(&code), json!(code.as_str()));
        assert_eq!(typed::<IssueCode>(&json!(code.as_str())), code);
    }
    assert_eq!(CODES.iter().map(IssueCode::as_str).collect::<BTreeSet<_>>().len(), CODES.len());
    let vectors = fixture("schema-conformance");
    let mut committed = BTreeSet::new();
    for vector in entries(&vectors["structural"]).iter().chain(entries(&vectors["rules"])) {
        for issue in typed::<Vec<Issue>>(&vector["issues"]) {
            committed.insert(issue.code.as_str());
        }
    }
    assert_eq!(committed, CODES.iter().map(IssueCode::as_str).collect());
    assert_eq!(json(&issue("/bones/2/id", IssueCode::DuplicateId)), json!({"path": "/bones/2/id", "code": "duplicate-id"}));
    assert!(serde_json::from_value::<Issue>(json!({"path": "", "code": "duplicate-id", "hint": "x"})).is_err());
    assert!(serde_json::from_value::<Issue>(json!({"path": "", "code": "duplicate"})).is_err());
}

#[test]
fn findings_are_deduplicated_and_sorted_by_path_then_code_in_code_point_order() {
    let mut findings = Findings::default();
    for (path, code) in [
        ("/b", IssueCode::OutOfRange),
        ("/a/2", IssueCode::SlugInvalid),
        ("/a/10", IssueCode::UnknownReference),
        ("/a/10", IssueCode::BoneOrder),
        ("/b", IssueCode::OutOfRange),
        ("", IssueCode::ValueInvalid),
        ("/ä", IssueCode::DuplicateId),
        ("/z", IssueCode::DuplicateId),
    ] {
        findings.report(path, code);
    }
    assert_eq!(
        findings.finish(),
        [
            issue("", IssueCode::ValueInvalid),
            issue("/a/10", IssueCode::BoneOrder),
            issue("/a/10", IssueCode::UnknownReference),
            issue("/a/2", IssueCode::SlugInvalid),
            issue("/b", IssueCode::OutOfRange),
            issue("/z", IssueCode::DuplicateId),
            issue("/ä", IssueCode::DuplicateId)
        ]
    );
    let vectors = fixture("schema-conformance");
    let several = entries(&vectors["rules"]).iter().find(|vector| vector["id"] == "several-findings").unwrap_or_else(|| panic!("no vector several-findings"));
    let issues = issues_of("Menagerie", &several["document"]).unwrap_or_default();
    assert_eq!(issues.len(), 4);
    assert!(issues.windows(2).all(|pair| (pair[0].path.as_str(), pair[0].code.as_str()) < (pair[1].path.as_str(), pair[1].code.as_str())));
}

#[test]
fn pointers_escape_tildes_and_slashes() {
    assert_eq!(at("", "id"), "/id");
    assert_eq!(at("/species", 3), "/species/3");
    assert_eq!(at("/a", "b/c~d"), "/a/b~1c~0d");
}

#[test]
fn slugs_and_colours_follow_the_patterns_of_the_normative_file() {
    for valid in ["a", "0", "blobby", "leg-left", "a-1-b", "x".repeat(64).as_str()] {
        assert!(is_slug(valid), "{valid}");
    }
    for invalid in ["", "-", "a-", "-a", "a--b", "A", "a b", "a_b", "ä", "a\n", "x".repeat(65).as_str()] {
        assert!(!is_slug(invalid), "{invalid:?}");
    }
    for valid in ["#000000", "#1e9b8d", "#ffffff"] {
        assert!(is_color(valid), "{valid}");
    }
    for invalid in ["", "#", "#1E9B8D", "#12345", "#1234567", "1e9b8d0", "teal", "#12345g", "#1e9b8d\n", "#ääää"] {
        assert!(!is_color(invalid), "{invalid:?}");
    }
}

#[test]
fn a_number_that_is_not_finite_is_a_type_finding_and_is_not_judged_any_further() {
    let mut species = base_species();
    species.size.width = f64::NAN;
    species.size.height = f64::INFINITY;
    species.temperament.energy = f64::NEG_INFINITY;
    species.bones[0].x = f64::NAN;
    species.locomotion.speed = f64::NAN;
    assert_eq!(
        species_issues(&species),
        [
            issue("/bones/0/x", IssueCode::TypeInvalid),
            issue("/locomotion/speed", IssueCode::TypeInvalid),
            issue("/size/height", IssueCode::TypeInvalid),
            issue("/size/width", IssueCode::TypeInvalid),
            issue("/temperament/energy", IssueCode::TypeInvalid)
        ]
    );
    let mut eased = base_species();
    let key = &mut eased.clips[0].tracks[0].keys[0];
    key.ease = Some([f64::NAN, 0.0, 2.0, 1.0]);
    key.value = f64::NAN;
    let base = "/clips/0/tracks/0/keys/0";
    assert_eq!(species_issues(&eased), [issue(&format!("{base}/ease/0"), IssueCode::TypeInvalid), issue(&format!("{base}/ease/2"), IssueCode::EaseRange), issue(&format!("{base}/value"), IssueCode::TypeInvalid)]);
}

#[test]
fn an_id_that_is_not_a_slug_does_not_count_as_declared() {
    let mut menagerie = base_menagerie();
    assert!(menagerie_issues(&menagerie).is_empty());
    let first = menagerie.species[0].id.clone();
    menagerie.species[0].id = "Not A Slug".to_string();
    let issues = menagerie_issues(&menagerie);
    assert!(issues.contains(&issue("/species/0/id", IssueCode::SlugInvalid)));
    let dangling: Vec<&Issue> = issues.iter().filter(|issue| issue.code == IssueCode::UnknownReference).collect();
    assert!(!dangling.is_empty(), "the species {first} is still referenced by a bond or a cast");
    assert!(issues.iter().all(|issue| issue.code == IssueCode::UnknownReference || issue.path == "/species/0/id"));
    let mut species = base_species();
    let parent = species.bones[0].id.clone();
    species.bones[0].id = "Not A Slug".to_string();
    let issues = species_issues(&species);
    assert!(issues.contains(&issue("/bones/0/id", IssueCode::SlugInvalid)));
    assert!(issues.contains(&issue("/bones/1/parent", IssueCode::UnknownReference)), "the bone {parent} is the parent of the second bone");
}

#[test]
fn an_ensemble_is_judged_without_its_species_and_the_assembled_menagerie_with_them() {
    let vectors = fixture("schema-conformance");
    let ensemble: Ensemble = typed(&vectors["ensemble"]);
    let documents: Vec<Species> = entries(&vectors["species"]).iter().map(|entry| typed(&entry["document"])).collect();
    assert!(ensemble_issues(&ensemble).is_empty());
    assert!(ensemble.json_schema.is_some());
    assert!(documents.iter().all(|species| species.json_schema.is_some()));
    let assembled = assemble_menagerie(&ensemble, &documents);
    assert_eq!(assembled, typed::<Menagerie>(&vectors["menagerie"]));
    assert_same("assembled", &json(&assembled), &vectors["menagerie"]);
    assert_eq!(assembled.json_schema, None);
    assert!(assembled.species.iter().all(|species| species.json_schema.is_none()));
    assert_eq!(assembled.species.iter().map(|species| species.id.as_str()).collect::<Vec<_>>(), documents.iter().map(|species| species.id.as_str()).collect::<Vec<_>>());
    assert!(menagerie_issues(&assembled).is_empty());
    let mut stranger = ensemble.clone();
    stranger.bonds[0].between[1] = "nobody".to_string();
    stranger.casts[0].rotation.push("nobody".to_string());
    assert!(ensemble_issues(&stranger).is_empty());
    assert_eq!(menagerie_issues(&assemble_menagerie(&stranger, &documents)), [issue("/bonds/0/between/1", IssueCode::UnknownReference), issue(&format!("/casts/0/rotation/{}", stranger.casts[0].rotation.len() - 1), IssueCode::UnknownReference)]);
    let empty = assemble_menagerie(&ensemble, &[]);
    assert_eq!((empty.schema.as_str(), empty.id.as_str(), empty.species.len()), (MENAGERIE_SCHEMA, ensemble.id.as_str(), 0));
}

#[test]
fn a_loop_closes_on_equal_values_or_on_whole_turns_of_a_rotation_only() {
    let seam = |channel: Channel, looping: bool, from: f64, to: f64| {
        let mut species = base_species();
        let bone = species.bones[0].id.clone();
        species.clips[0].looping = looping;
        species.clips[0].tracks = vec![Track { bone, channel, keys: vec![Key { at: 0.0, value: from, ease: None }, Key { at: 1.0, value: to, ease: None }] }];
        species_issues(&species).into_iter().filter(|issue| issue.code == IssueCode::LoopSeam).map(|issue| issue.path).collect::<Vec<_>>()
    };
    let reported = vec!["/clips/0/tracks/0/keys/1/value".to_string()];
    assert!(seam(Channel::Rotation, true, 0.0, 360.0).is_empty());
    assert!(seam(Channel::Rotation, true, 180.0, -180.0).is_empty());
    assert!(seam(Channel::Rotation, true, 30.0, -690.0).is_empty());
    assert!(((-719.8_f64 - -359.8) / 360.0).floor() != (-719.8_f64 - -359.8) / 360.0);
    assert!(seam(Channel::Rotation, true, -359.8, -719.8).is_empty());
    assert!(seam(Channel::Rotation, true, 0.1, 720.1).is_empty());
    assert_eq!(seam(Channel::Rotation, true, 0.0, 360.5), reported);
    assert_eq!(seam(Channel::Rotation, true, 10.0, -10.0), reported);
    assert!(seam(Channel::Y, true, -2.5, -2.5).is_empty());
    assert_eq!(seam(Channel::Rotation, true, 0.0, 180.0), reported);
    assert_eq!(seam(Channel::Rotation, true, 0.0, 359.999), reported);
    assert_eq!(seam(Channel::X, true, 0.0, 360.0), reported);
    assert_eq!(seam(Channel::ScaleY, true, 1.0, 1.1), reported);
    assert!(seam(Channel::X, false, 0.0, 360.0).is_empty());
}
