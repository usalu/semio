//! 🧪️ Original graph receiving preserves owned payloads, strict transport and every cancellation frontier.
use super::*;
use crate::details::{IfcValueReceiveMode, IfcValueReceiver};

fn value(json: &serde_json::Value) -> DslValue {
    let parsed = semio_framework_pack_json::parse(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
fn mode(name: &str) -> IfcValueReceiveMode {
    match name {
        "nativeGraph" => IfcValueReceiveMode::Native,
        "graphDecimal" => IfcValueReceiveMode::Decimal,
        _ => panic!("unknown fixture transport"),
    }
}

fn receive(input: DslValue, mode: IfcValueReceiveMode, cancel_at: Option<usize>, refused: bool) -> (Option<IfcValue>, usize) {
    let mut caller = semio_framework_plugin::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut cx = caller.context().with_original_decoder(&mut decoder);
    let policy = editing::fixture_mounted_owner_policy();
    let mut receiver = IfcValueReceiver::new(mode);
    let mut original = Some(input);
    assert!(cx.with_retained_loan(policy.preparation, |cx| receiver.receive_owned(&mut original, cx)).unwrap());
    assert!(original.is_none());
    let mut rebound = Some(DslValue::Null);
    assert!(cx.with_retained_loan(policy.preparation, |cx| receiver.receive_owned(&mut rebound, cx)).is_err());
    assert!(rebound.is_some());
    let mut output = None;
    let mut stages = 0;
    for stage in 1..10_000 {
        stages = stage;
        if cancel_at == Some(stage) {
            cx.original_cancel_token().cancel_now();
        }
        let demand = receiver.demands().unwrap();
        assert!(
            demand.copy_bytes <= policy.preparation.maximum_copy_bytes
                && demand.capacity_bytes <= policy.preparation.maximum_capacity_bytes
                && demand.release_bytes <= policy.preparation.maximum_release_bytes
                && demand.depth <= policy.preparation.maximum_depth
        );
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        match cx.with_retained_loan(grant, |cx| receiver.advance(cx)) {
            Ok(true) => {
                assert!(cancel_at.is_none() && !refused);
                assert!(cx.with_retained_loan(policy.preparation, |cx| receiver.take(&mut output, cx)).unwrap());
                break;
            }
            Err(error) => {
                assert!(cancel_at.is_some() || refused, "unexpected original receiving refusal: {error:?}");
                if cancel_at.is_some() {
                    assert!(cx.is_cancelled());
                }
                break;
            }
            _ => {}
        }
    }
    assert_eq!(output.is_none(), cancel_at.is_some() || refused);
    receiver.begin_close();
    for _ in 0..10_000 {
        if receiver.terminal_is_empty() {
            break;
        }
        let demand = receiver.close_demands().unwrap();
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        cx.with_retained_loan(grant, |cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| receiver.close_step(grant))).unwrap();
    }
    assert!(receiver.terminal_is_empty());
    let fuel = cx.fuel_remaining();
    drop(cx);
    let fuel = caller.retire_decoder(&mut decoder, fuel).unwrap();
    caller.retire(fuel).unwrap();
    (output, stages)
}

#[test]
fn neutral_receiving_modes_match_native_values_and_preserve_real_bits() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔎️details/🧪️tests/receive.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let (output, stages) = receive(value(&case["input"]), mode(case["mode"].as_str().unwrap()), None, false);
        let output = output.unwrap();
        assert_eq!(output, IfcValue::from_value(value(&case["expected"])).unwrap());
        if let Some(bits) = case["realBits"].as_str() {
            let IfcValue::TypedValue(typed) = &output else { panic!("real bits fixture requires typed value") };
            let IfcValue::Real(real) = &typed.items[0] else { panic!("real bits fixture requires REAL") };
            assert_eq!(real.to_bits(), bits.parse::<u64>().unwrap());
        }
        for prefix in 1..=stages {
            assert!(receive(value(&case["input"]), mode(case["mode"].as_str().unwrap()), Some(prefix), false).0.is_none());
        }
    }
    for case in fixture["refusals"].as_array().unwrap() {
        assert!(receive(value(&case["input"]), mode(case["mode"].as_str().unwrap()), None, true).0.is_none());
    }
}

#[test]
fn original_large_string_allocation_moves_into_native_output() {
    let text = "x".repeat(1_000_000);
    let address = text.as_ptr();
    let input = DslValue::Object(vec![("kind".into(), DslValue::String("string".into())), ("value".into(), DslValue::String(text))]);
    let output = receive(input, IfcValueReceiveMode::Native, None, false).0.unwrap();
    assert_eq!(output.as_str().unwrap().as_ptr(), address);
    assert_eq!(output.as_str().unwrap().len(), 1_000_000);
}
