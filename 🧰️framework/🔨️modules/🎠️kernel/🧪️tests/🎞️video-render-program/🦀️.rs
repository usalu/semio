use super::*;

/// 🧫️ `🧫️fixtures/🎞️video-render-program/🔣️.json` — read by this law and its TypeScript twin alike.
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎞️video-render-program/🔣️.json")).expect("video render program fixture")
}

fn program(value: &serde_json::Value) -> VideoRenderProgram {
    serde_json::from_value(value.clone()).expect("fixture program decodes")
}

/// ⚖️ LAW: every valid program is admitted, plays the declared frames for the declared time, and
/// crosses the value codec (the WIT `pack`) unchanged.
#[test]
fn valid_programs_play_the_declared_frames() {
    let fixture = fixture();
    for case in fixture["valid"].as_array().expect("valid") {
        let id = case["id"].as_str().expect("id");
        let program = program(&case["program"]);
        assert_eq!(program.validate(), Ok(()), "{id}");
        assert_eq!(program.frame_count(), case["frameCount"].as_u64().expect("frameCount"), "{id}: frames");
        assert_eq!(program.duration_milliseconds(), case["durationMs"].as_u64().expect("durationMs"), "{id}: duration");
        let value = semio_framework_value::ToValue::to_value(&program);
        assert_eq!(<VideoRenderProgram as semio_framework_value::FromValue>::from_value(value).expect("value round trip"), program, "{id}: value codec");
        let wire = serde_json::to_value(&program).expect("serde");
        let keys = |value: &serde_json::Value| value.as_object().map(|object| object.keys().cloned().collect::<Vec<_>>()).unwrap_or_default();
        assert_eq!(keys(&wire), keys(&case["program"]), "{id}: camelCase program keys");
        for (scene, expected) in wire["scenes"].as_array().expect("scenes").iter().zip(case["program"]["scenes"].as_array().expect("scenes")) {
            for (op, expected) in scene["ops"].as_array().expect("ops").iter().zip(expected["ops"].as_array().expect("ops")) {
                assert_eq!(keys(op), keys(expected), "{id}: camelCase op keys");
            }
        }
    }
}

/// ⚖️ LAW: each invalid program (the base program with one field replaced) is refused with the declared code.
#[test]
fn invalid_programs_are_refused_with_the_declared_code() {
    let fixture = fixture();
    let base_id = fixture["invalidBase"].as_str().expect("invalidBase");
    let base = fixture["valid"].as_array().expect("valid").iter().find(|case| case["id"] == base_id).expect("base case")["program"].clone();
    for case in fixture["invalid"].as_array().expect("invalid") {
        let mut value = base.clone();
        for (key, replacement) in case["patch"].as_object().expect("patch") {
            value[key] = replacement.clone();
        }
        let error = program(&value).validate().expect_err("refused");
        assert_eq!(error.code(), case["error"].as_str().expect("error"), "{}: {error}", case["id"]);
    }
}

/// ⚖️ LAW: the effect carrying a program keeps its camelCase field names on the wire.
#[test]
fn video_render_export_effect_serializes_camel_case() {
    let fixture = fixture();
    let program = program(&fixture["valid"][0]["program"]);
    let effect = Effect::VideoRenderExport { filename: "deck.mp4".into(), program };
    let json = serde_json::to_value(&effect).expect("serde");
    assert_eq!(json["videoRenderExport"]["filename"], "deck.mp4");
    assert_eq!(json["videoRenderExport"]["program"]["schema"], VIDEO_RENDER_PROGRAM_SCHEMA);
}
