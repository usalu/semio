use crate::{part_revision, set_story_content, story_contents};
use semio_s_artifact_stdio_idml::schema::IdmlPart;
use semio_s_artifact_stdio_idml::{IdmlMutation, IdmlSnapshot};
use quick_xml::events::Event;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("story-content.json")).unwrap()
}

fn snapshot(fixture: &serde_json::Value) -> IdmlSnapshot {
    let story = fixture["before"].as_str().unwrap().as_bytes().to_vec();
    IdmlSnapshot {
        dom_version: "8.0".into(),
        parts: vec![
            IdmlPart { path: "designmap.xml".into(), bytes: b"<Document/>".to_vec() },
            IdmlPart { path: fixture["part"].as_str().unwrap().into(), bytes: story },
        ],
    }
}

fn oracle_contents(bytes: &[u8]) -> Vec<String> {
    let mut reader = quick_xml::Reader::from_reader(bytes);
    let (mut runs, mut open) = (Vec::new(), false);
    loop {
        match reader.read_event().unwrap() {
            Event::Start(start) if start.name().as_ref() == b"Content" => {
                open = true;
                runs.push(String::new());
            }
            Event::Empty(start) if start.name().as_ref() == b"Content" => runs.push(String::new()),
            Event::Text(text) if open => runs.last_mut().unwrap().push_str(&text.decode().unwrap()),
            Event::GeneralRef(reference) if open => {
                let name = reference.decode().unwrap();
                let character = quick_xml::escape::resolve_predefined_entity(&name).map(|value| value.to_string()).or_else(|| reference.resolve_char_ref().unwrap().map(|value| value.to_string())).unwrap();
                runs.last_mut().unwrap().push_str(&character);
            }
            Event::End(end) if end.name().as_ref() == b"Content" => open = false,
            Event::Eof => return runs,
            _ => {}
        }
    }
}

fn texts(snapshot: &IdmlSnapshot) -> Vec<String> {
    story_contents(snapshot).into_iter().map(|run| run.text).collect()
}

fn apply(snapshot: &IdmlSnapshot, mutation: IdmlMutation) -> IdmlSnapshot {
    let IdmlMutation::ReplacePart { path, bytes } = mutation else { panic!("replace part expected") };
    let mut next = snapshot.clone();
    next.parts.iter_mut().find(|part| part.path == path).unwrap().bytes = bytes;
    next
}

#[test]
fn story_runs_match_the_third_party_parser() {
    let fixture = fixture();
    let base = snapshot(&fixture);
    let expected: Vec<String> = fixture["initial"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
    assert_eq!(texts(&base), expected);
    assert_eq!(oracle_contents(fixture["before"].as_str().unwrap().as_bytes()), expected);
}

#[test]
fn valid_edits_match_the_oracle_and_preserve_every_other_byte() {
    let fixture = fixture();
    let base = snapshot(&fixture);
    let part = fixture["part"].as_str().unwrap();
    let revision = part_revision(fixture["before"].as_str().unwrap().as_bytes());
    for case in fixture["valid"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mutation = set_story_content(&base, part, case["index"].as_u64().unwrap() as usize, &revision, case["text"].as_str().unwrap()).unwrap().expect(name);
        let after = apply(&base, mutation);
        let expected: Vec<String> = case["after"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().into()).collect();
        assert_eq!(texts(&after), expected, "{name}: own reader");
        assert_eq!(oracle_contents(&after.parts[1].bytes), expected, "{name}: third-party reader");
        let before = String::from_utf8(base.parts[1].bytes.clone()).unwrap();
        let changed = String::from_utf8(after.parts[1].bytes.clone()).unwrap();
        for marker in ["<!-- <Content>decoy</Content> -->", "<Br/>", "<Custom:Note xmlns:Custom=\"urn:x\" kept=\"yes\"/>", "standalone=\"yes\"", "AppliedParagraphStyle=\"ParagraphStyle/$ID/NormalParagraphStyle\""] {
            assert_eq!(before.contains(marker), changed.contains(marker), "{name}: {marker}");
        }
        assert_eq!(after.parts[0], base.parts[0], "{name}: unrelated parts");
    }
}

#[test]
fn unchanged_text_is_not_a_mutation() {
    let fixture = fixture();
    let base = snapshot(&fixture);
    let revision = part_revision(fixture["before"].as_str().unwrap().as_bytes());
    for case in fixture["unchanged"].as_array().unwrap() {
        assert_eq!(set_story_content(&base, fixture["part"].as_str().unwrap(), case["index"].as_u64().unwrap() as usize, &revision, case["text"].as_str().unwrap()).unwrap(), None);
    }
}

#[test]
fn invalid_edits_are_refused_with_their_code() {
    let fixture = fixture();
    let base = snapshot(&fixture);
    let revision = part_revision(fixture["before"].as_str().unwrap().as_bytes());
    for case in fixture["invalid"].as_array().unwrap() {
        let part = case["part"].as_str().unwrap_or(fixture["part"].as_str().unwrap());
        let supplied = case["revision"].as_str().unwrap_or(&revision);
        let error = set_story_content(&base, part, case["index"].as_u64().unwrap() as usize, supplied, case["text"].as_str().unwrap()).unwrap_err();
        assert_eq!(error.code(), case["error"].as_str().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn inverse_mutation_restores_the_original_bytes() {
    let fixture = fixture();
    let base = snapshot(&fixture);
    let revision = part_revision(fixture["before"].as_str().unwrap().as_bytes());
    let part = fixture["part"].as_str().unwrap();
    let edited = apply(&base, set_story_content(&base, part, 0, &revision, "Servus").unwrap().unwrap());
    let back_revision = part_revision(&edited.parts[1].bytes);
    let restored = apply(&edited, set_story_content(&edited, part, 0, &back_revision, "Hello & welcome").unwrap().unwrap());
    assert_eq!(restored, base);
}
