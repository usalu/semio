use super::*;
use semio_framework_pack_json::{self as json, JsonMemberPolicy};
use semio_framework_value::{native_decoding::NativeDecodeProgress, NativeEncodeControl};

fn unbounded_grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
}

fn drain(mut owner: Box<dyn ErasedSnapshotRetirement>) {
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().unwrap();
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 32,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(),
            maximum_release_bytes: owner.next_release_byte_demand().unwrap(),
            maximum_depth: owner.next_depth_demand().unwrap().max(1),
        };
        owner.close_step(grant).unwrap();
    }
}

fn retire_value<T: semio_framework_value::retirement::RetireOwned>(value: T) -> Box<dyn ErasedSnapshotRetirement> {
    semio_framework_value::retirement::admit_owned_retirement(value, unbounded_grant()).map_err(|(error, _)| error).unwrap().0
}

fn input(value: &serde_json::Value) -> DslValue {
    let mut progress = |_: NativeDecodeProgress| true;
    let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
    json::from_json_str_controlled(&value.to_string(), JsonMemberPolicy::Reject, &mut control).unwrap()
}

fn output(value: &DslValue) -> serde_json::Value {
    let mut progress = |_| true;
    let mut control = NativeEncodeControl::new(8 * 1024 * 1024, &mut progress);
    serde_json::from_str(&json::to_json_string_controlled(value, &mut control).unwrap()).unwrap()
}

#[test]
fn puzzle_scene_projection_matches_language_neutral_cases_and_sqlite_oracle() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut results = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let value = input(&case["input"]);
        let mode = if case["mode"] == "normal" { Mode::Normal } else { Mode::Ported };
        let mut progress = |_: NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
        let mut projection = Projection::new();
        let accepted = projection.project(&value, mode, &mut control).is_ok();
        assert_eq!(accepted, case["accepted"].as_bool().unwrap(), "{}", case["id"]);
        let actual = projection.take_scene().map(|scene| { let result = output(&scene); drain(retire_value(scene)); result }).unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, case["expected"], "{}", case["id"]);
        drain(projection.take_retirement(unbounded_grant()).unwrap().0);
        drain(retire_value(value));
        results.push(serde_json::json!({"id":case["id"],"accepted":accepted,"actual":actual}));
    }
    let script = format!("{}\nconst input=JSON.parse(await Bun.stdin.text());await Bun.write(Bun.stdout,JSON.stringify(oracleCases(input.fixture)));", include_str!("🟦️.ts"));
    let mut child = Command::new("bun").args(["-e", &script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(results, serde_json::from_slice::<Vec<serde_json::Value>>(&result.stdout).unwrap());
    println!("[DEBUG] Puzzle projection: {} complete native cases match SQLite JSON1", results.len());
}

#[test]
fn puzzle_scene_projection_retains_partial_owners_at_every_cancellation_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["accepted"] == true) {
        let value = input(&case["input"]);
        let mode = if case["mode"] == "normal" { Mode::Normal } else { Mode::Ported };
        let mut calls = 0;
        let mut progress = |_: NativeDecodeProgress| { calls += 1; true };
        let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
        let mut complete = Projection::new();
        complete.project(&value, mode, &mut control).unwrap();
        drop(control);
        drain(complete.take_retirement(unbounded_grant()).unwrap().0);
        for cancel_at in 1..=calls {
            let mut observed = 0;
            let mut progress = |_: NativeDecodeProgress| { observed += 1; observed < cancel_at };
            let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
            let mut partial = Projection::new();
            assert_eq!(partial.project(&value, mode, &mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
            assert!(partial.take_scene().is_none());
            drain(partial.take_retirement(unbounded_grant()).unwrap().0);
        }
        let mut progress = |_: NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(0, &mut progress);
        let mut refused = Projection::new();
        assert_eq!(refused.project(&value, mode, &mut control).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
        drain(refused.take_retirement(unbounded_grant()).unwrap().0);
        drain(retire_value(value));
    }
    println!("[DEBUG] Puzzle projection: every observed callback cancellation retains a drainable owner");
}
