use super::*;
use semio_framework_pack_json as json;
use semio_framework_pack_json::JsonMemberPolicy;
use semio_framework_value_native_owned::native_decoding::NativeDecodeProgress;
use semio_framework_value::NativeEncodeControl;

fn unbounded_grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
}

/// 🪙 Runs one retirement step inside a wallet whose stock is exactly the quoted grant.
fn funded<R>(grant: semio_framework_value::retained_clone::RetainedCloneGrant, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> R {
    semio_framework_value::AuthoringWallet::new(grant, 1_048_576).run_invoked(grant, 1_048_576, run).expect("puzzle scene wallet funds the quoted grant").expect("puzzle scene work quote is admitted").output
}

/// ♻️ Closes one admitted owner by the byte demand it quotes, one funded effect per turn.
fn drain(mut owner: Box<dyn ErasedSnapshotRetirement>) {
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("puzzle scene retirement quotes its next copy");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 32,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).expect("puzzle scene retirement quotes capacity"),
            maximum_release_bytes: owner.next_release_byte_demand().expect("puzzle scene retirement quotes release"),
            maximum_depth: owner.next_depth_demand().expect("puzzle scene retirement quotes depth").max(1),
        };
        semio_framework_value::AuthoringWallet::new(grant, 1_048_576).run_settled(grant, 1_048_576, |effect| owner.close_step(copy, effect)).expect("puzzle scene retirement step settles").expect("puzzle scene retirement quote is admitted").0.result.expect("puzzle scene retirement step");
    }
}

/// ♻️ Admits one owned value and returns the cursor that must be drained.
fn retire_value<T: semio_framework_value::retirement::RetireOwned>(value: T) -> Box<dyn ErasedSnapshotRetirement> {
    let mut slot = Some(value);
    match funded(unbounded_grant(), |effect| semio_framework_value::retirement::admit_owned_retirement(&mut slot, effect)).result.expect("puzzle scene value retirement") {
        Some((owner, _)) => owner,
        None => panic!("puzzle scene value retirement stayed pending"),
    }
}

/// ♻️ Moves a projection's partial owners onto the same retirement cursor.
fn retire_projection(projection: &mut Projection) -> Box<dyn ErasedSnapshotRetirement> {
    match funded(unbounded_grant(), |effect| projection.take_retirement(effect)).result.expect("puzzle scene projection retirement") {
        Some((owner, _)) => owner,
        None => panic!("puzzle scene projection retirement stayed pending"),
    }
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
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
        drain(retire_projection(&mut projection));
        drain(retire_value(value));
        results.push(serde_json::json!({"id":case["id"],"accepted":accepted,"actual":actual}));
    }
    let script = format!("{}\nconst input=JSON.parse(await Bun.stdin.text());await Bun.write(Bun.stdout,JSON.stringify(oracleCases(input.fixture)));", include_str!("🟦️.ts"));
    let mut child = Command::new("bun").args(["-e", &script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(results, serde_json::from_slice::<Vec<serde_json::Value>>(&result.stdout).unwrap());
}

#[test]
fn puzzle_scene_projection_retains_partial_owners_at_every_cancellation_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["accepted"] == true) {
        let value = input(&case["input"]);
        let mode = if case["mode"] == "normal" { Mode::Normal } else { Mode::Ported };
        let mut calls = 0;
        let mut progress = |_: NativeDecodeProgress| { calls += 1; true };
        let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
        let mut complete = Projection::new();
        complete.project(&value, mode, &mut control).unwrap();
        drop(control);
        drain(retire_projection(&mut complete));
        for cancel_at in 1..=calls {
            let mut observed = 0;
            let mut progress = |_: NativeDecodeProgress| { observed += 1; observed < cancel_at };
            let mut control = NativeDecodeControl::new(8 * 1024 * 1024, &mut progress);
            let mut partial = Projection::new();
            assert_eq!(partial.project(&value, mode, &mut control).unwrap_err().kind, ValueRefusalKind::Canceled);
            assert!(partial.take_scene().is_none());
            drain(retire_projection(&mut partial));
        }
        let mut progress = |_: NativeDecodeProgress| true;
        let mut control = NativeDecodeControl::new(0, &mut progress);
        let mut refused = Projection::new();
        assert_eq!(refused.project(&value, mode, &mut control).unwrap_err().kind, ValueRefusalKind::OwnershipLimit);
        drain(retire_projection(&mut refused));
        drain(retire_value(value));
    }
}
