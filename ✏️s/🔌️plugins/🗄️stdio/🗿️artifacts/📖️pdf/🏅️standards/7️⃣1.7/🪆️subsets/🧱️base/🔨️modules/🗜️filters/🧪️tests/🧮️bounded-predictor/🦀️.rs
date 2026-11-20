//! 🧮️ Exercises one-byte output, allocation, and retirement on predicted PDF streams.

use super::*;

#[test]
fn content_stream_flate_predictors_resume_with_one_byte_output() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let source = super::flate_tests::bytes(case["encoded"].as_str().unwrap());
        let expected = super::flate_tests::bytes(case["decoded"].as_str().unwrap());
        let parameter = |key: &str| case[key].as_u64().unwrap() as u32;
        let maximum_inflated = parameter("inflatedLength") as usize;
        let mut cursor = PdfPredictorCursor::flate(maximum_inflated, expected.len(), parameter("predictor"), parameter("columns"), parameter("colors"), parameter("bits")).unwrap();
        let mut actual = Vec::new();
        let mut allocation_bytes = 0;
        let mut progress = (0, 0);
        let mut turns = 0;
        let mut demands = 0;
        loop {
            assert!(turns < 256);
            turns += 1;
            let mut byte = [0];
            let grant = FlateGrant { work: 3, allocation_bytes };
            let step = cursor.advance(&source, &mut byte, grant, false);
            assert!(step.work <= grant.work && step.allocated_bytes <= grant.allocation_bytes && step.written_bytes <= byte.len());
            actual.extend_from_slice(&byte[..step.written_bytes]);
            let current = cursor.progress();
            assert!(current.0 >= progress.0 && current.1 >= progress.1);
            progress = current;
            allocation_bytes = 0;
            match step.status.unwrap() {
                FlateStatus::Complete => break,
                FlateStatus::NeedsHistory { bytes } => {
                    allocation_bytes = bytes;
                    demands += 1;
                }
                FlateStatus::Yield | FlateStatus::NeedsOutput => assert!(step.work > 0),
                FlateStatus::Cancelled => panic!("unexpected cancellation"),
            }
        }
        assert!(demands >= 2);
        assert_eq!(progress, (source.len(), expected.len()));
        assert_eq!(cursor.decoded_input_bytes(), maximum_inflated);
        close(cursor);
        assert_eq!(actual, expected, "{}", case["name"]);
        let drained = super::bounded_flate::decode_predicted_compatible(&source, parameter("predictor"), parameter("columns"), parameter("colors"), parameter("bits")).unwrap();
        assert_eq!(drained, expected, "{}", case["name"]);
        if parameter("predictor") == 2 {
            let encoded = super::bounded_flate::encode_tiff(&actual, parameter("columns"), parameter("colors"), parameter("bits")).unwrap();
            assert_eq!(encoded, super::bounded_flate::decode_compatible(&source).unwrap(), "{}", case["name"]);
        }
    }
}

#[test]
fn content_stream_flate_predictor_retirement_resumes_after_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let case = &fixture[0];
    let source = super::flate_tests::bytes(case["encoded"].as_str().unwrap());
    let mut cursor = PdfPredictorCursor::flate(1024, 1024, 15, 2, 3, 8).unwrap();
    let grant = FlateGrant { work: 100, allocation_bytes: 65_536 };
    let first = cursor.advance(&source, &mut [0; 1], grant, false);
    assert_eq!(first.written_bytes, 1);
    assert_eq!(first.status.unwrap(), FlateStatus::NeedsOutput);
    let progress = cursor.progress();
    let cancelled = cursor.advance(&source, &mut [0; 3], grant, true);
    assert_eq!(cancelled.status.unwrap(), FlateStatus::Cancelled);
    assert_eq!(cancelled.work + cancelled.allocated_bytes + cancelled.written_bytes, 0);
    assert_eq!(cursor.progress(), progress);
    close(cursor);
}

#[test]
fn content_stream_flate_predictors_bound_a_split_sample() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let source = super::flate_tests::bytes(fixture[2]["encoded"].as_str().unwrap());
    let grant = FlateGrant { work: 100, allocation_bytes: 65_536 };
    let mut limited = PdfPredictorCursor::flate(1024, 1, 2, 3, 1, 16).unwrap();
    let refused = limited.advance(&source, &mut [0; 8], grant, false);
    assert_eq!(refused.status.unwrap_err().kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    assert_eq!(refused.written_bytes, 1);
    close(limited);

    let incomplete = super::flate_tests::bytes("7801010100feff1200130013");
    let mut cursor = PdfPredictorCursor::flate(1, 2, 2, 1, 1, 16).unwrap();
    let turn = cursor.advance(&incomplete, &mut [0; 2], grant, false);
    assert_eq!(turn.status.unwrap_err().kind, semio_framework_value::ValueRefusalKind::InvalidValue);
    assert_eq!(turn.written_bytes, 0);
    close(cursor);
}

fn close(mut cursor: PdfPredictorCursor) {
    cursor.begin_close();
    let mut turns = 0;
    let mut released = 0;
    let retained = cursor.owned_bytes();
    while !cursor.terminal_is_empty() {
        assert!(turns < 16);
        turns += 1;
        let demand = cursor.retirement_demand();
        let idle = cursor.close_step(0, 0);
        assert_eq!(idle.work + idle.released_bytes + idle.released_items, 0);
        if demand.release_bytes > 0 {
            let idle = cursor.close_step(1, demand.release_bytes - 1);
            assert_eq!(idle.work + idle.released_bytes + idle.released_items, 0);
        }
        let step = cursor.close_step(1, demand.release_bytes);
        assert!(step.work <= 1 && step.released_items <= 1 && step.released_bytes <= demand.release_bytes);
        released += step.released_bytes;
    }
    assert_eq!(retained, released);
}
