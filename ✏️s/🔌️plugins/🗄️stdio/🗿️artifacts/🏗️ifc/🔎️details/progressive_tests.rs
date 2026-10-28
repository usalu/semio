//! 🧪️ Native Store bytes use each immediate preimage and retain all original owners through close.
use super::*;
use protocol::command::ColdDiffAlgebra;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 2097152, maximum_release_bytes: 2097152, maximum_depth: 256 }
}
fn close(cursor: &mut IfcPublication, caller: &mut OriginalWorkFixture) {
    for _ in 0..65536 {
        if !cursor.codec_live() {
            break;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = cursor.close_demands()?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| cursor.close_codec(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
    assert!(!cursor.codec_live());
    let source = cursor.view.source.as_mut().unwrap();
    for _ in 0..65536 {
        if source.terminal_is_empty() {
            return;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = source.next_close_demand(grant().maximum_copy_bytes)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| source.close_step(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
    panic!("original publication source close did not drain")
}
fn capture(snapshot: &Arc<IfcSnapshot>, caller: &mut OriginalWorkFixture) -> IfcPublication {
    let mut cursor = IfcPublication::default();
    caller
        .work_turn(grant(), 1, |cx| {
            let alias_bytes = std::mem::size_of::<Arc<IfcSnapshot>>();
            let source_grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes - alias_bytes, ..cx.retained_grant() };
            let (source, mut progress) = SnapshotSource::admit_borrowed(Arc::clone(snapshot), |snapshot| snapshot.as_ref(), source_grant).map_err(|(error, _)| error)?;
            progress.copied_bytes += alias_bytes;
            cursor.view.source = Some(source);
            cx.consume_retained(progress)?;
            Ok(())
        })
        .unwrap();
    cursor
}
fn encoded(source: store::ArtifactPreparedOperationSource<'_>, caller: &mut OriginalWorkFixture) -> Vec<u8> {
    let mut cursor = store::ArtifactPreparedOperationCursor::default();
    let mut result = Vec::new();
    let mut chunk = [0; 64];
    for _ in 0..65536 {
        let (written, complete) = caller
            .work_turn(grant(), 1, |cx| {
                let step = cursor.advance(source, &mut chunk, cx.retained_grant()).map_err(|error| error.reason)?;
                let result = (step.written_bytes, step.complete);
                cx.consume_retained(receipt(step))?;
                Ok(result)
            })
            .unwrap();
        result.extend_from_slice(&chunk[..written]);
        if complete {
            for _ in 0..65536 {
                if cursor.closed {
                    return result;
                }
                caller
                    .turn(grant(), |cx| {
                        cx.with_original_retirement(|cx| {
                            let demand = cursor.retirement_demands()?;
                            semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                                let step = cursor.close(grant).map_err(|error| error.reason)?;
                                Ok(if step.complete { RetainedCloneStep::Complete(receipt(step)) } else { RetainedCloneStep::Progress(receipt(step)) })
                            })?;
                            Ok(())
                        })
                    })
                    .unwrap();
            }
        }
    }
    panic!("reference wire did not drain")
}
fn native<T: FromValue>(value: &serde_json::Value) -> T {
    T::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap()
}
#[test]
fn original_progressive_preimages_match_central_bytes_and_exact_inverse_at_every_prefix() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/progressive.json")).unwrap();
    let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/progressive-inverses.json")).unwrap();
    for (case_index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let expected_inverses = native::<Vec<IfcMutation>>(&vectors["cases"][case_index]);
        let snapshot = Arc::new(native::<IfcSnapshot>(&fixture["snapshot"]));
        let original = snapshot.to_value();
        let mutations = native::<Vec<IfcMutation>>(&case["mutations"]);
        let owner = mutations.as_ptr();
        let mut caller = OriginalWorkFixture::new();
        let mut cursor = capture(&snapshot, &mut caller);
        let mut next = snapshot.as_ref().clone();
        let mut inverse = Vec::new();
        let mut checked = 0;
        let mut turns = 0;
        for _ in 0..65536 {
            if matches!(cursor.phase, Phase::Forward) && checked == cursor.mutation {
                let immediate = protocol::ColdMutation::inverse(&mutations[checked], &next).unwrap();
                assert_eq!(immediate.len(), 1, "{} immediate inverse", case["name"]);
                assert_eq!(immediate[0].to_value(), expected_inverses[checked].to_value(), "{} neutral inverse {checked}", case["name"]);
                let expected = encoded(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&expected_inverses[checked]) }, &mut caller);
                let central = encoded(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: store::ArtifactCanonicalTreeSource::view(&immediate[0]) }, &mut caller);
                assert_eq!(central, expected, "{} central exact neutral bytes {checked}", case["name"]);
                let view = cursor.inverse_wire(&mutations);
                let actual = encoded(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: &view }, &mut caller);
                assert_eq!(actual, expected, "{} row {checked}", case["name"]);
                let outcome = protocol::ColdMutation::diff(&mutations[checked], &next);
                inverse.push(outcome.diff().inverse(&next));
                next = protocol::apply_diff_cold(outcome.diff(), &next).unwrap();
                checked += 1;
            }
            turns += 1;
            if caller.work_turn(grant(), 1, |cx| cursor.advance(&mutations, semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(32), cx)).unwrap() {
                break;
            }
        }
        assert!(matches!(cursor.phase, Phase::Done));
        assert_eq!(checked, mutations.len());
        for inverse in inverse.into_iter().rev() {
            next = protocol::apply_diff_cold(&inverse, &next).unwrap();
        }
        assert_eq!(next.to_value(), original, "{} exact values", case["name"]);
        assert_eq!(next.entities[0].args[1], IfcValue::Real(-0.0));
        assert!(matches!(next.entities[0].args[1],IfcValue::Real(value) if value.to_bits()==(-0.0f64).to_bits()));
        close(&mut cursor, &mut caller);
        caller.finish();
        for stop in 1..=turns {
            let mut caller = OriginalWorkFixture::new();
            let mut cursor = capture(&snapshot, &mut caller);
            for turn in 1..=stop {
                if turn == stop {
                    caller.cancel().cancel_now();
                }
                let result = caller.work_turn(grant(), 1, |cx| cursor.advance(&mutations, semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(32), cx));
                if turn == stop {
                    assert!(result.is_err());
                } else {
                    assert!(!result.unwrap());
                }
            }
            assert_eq!(snapshot.to_value(), original);
            assert_eq!(mutations.as_ptr(), owner);
            close(&mut cursor, &mut caller);
            caller.finish();
        }
    }
}

#[test]
fn original_publication_refuses_a_rebound_pending_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/progressive.json")).unwrap();
    let snapshot = Arc::new(native::<IfcSnapshot>(&fixture["snapshot"]));
    let mutations = native::<Vec<IfcMutation>>(&fixture["cases"][0]["mutations"]);
    let replacement = mutations.clone();
    let mut caller = OriginalWorkFixture::new();
    let mut cursor = capture(&snapshot, &mut caller);
    caller.work_turn(grant(), 1, |cx| cursor.advance(&mutations, semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(32), cx)).unwrap();
    assert!(caller.work_turn(grant(), 1, |cx| cursor.advance(&replacement, semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(32), cx)).unwrap_err().primary.message().contains("source changed"));
    assert_eq!(cursor.bytes, 0);
    close(&mut cursor, &mut caller);
    caller.finish();
}

#[test]
fn temporary_canonical_facades_keep_their_retained_original_identity() {
    let first = IfcInverseWire::default();
    let second = IfcInverseWire::default();
    let facades = [inverse::BorrowedInverse { view: &first, mutations: &[] }, inverse::BorrowedInverse { view: &first, mutations: &[] }, inverse::BorrowedInverse { view: &second, mutations: &[] }];
    let mut caller = OriginalWorkFixture::new();
    let mut cursor = store::ArtifactPreparedOperationCursor::default();
    let mut chunk = [0; 64];
    for facade in &facades[..2] {
        caller
            .work_turn(grant(), 1, |cx| {
                let progress = cursor.advance(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: facade }, &mut chunk, cx.retained_grant()).map_err(|error| error.reason)?;
                cx.consume_retained(receipt(progress))?;
                Ok(())
            })
            .unwrap();
    }
    let error = caller.work_turn(grant(), 1, |cx| cursor.advance(store::ArtifactPreparedOperationSource::CanonicalJson { header: b"op", body: &facades[2] }, &mut chunk, cx.retained_grant()).map_err(|error| error.reason.into())).unwrap_err();
    assert!(error.primary.message().contains("source changed"));
    for _ in 0..1024 {
        if cursor.closed {
            break;
        }
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    let demand = cursor.retirement_demands()?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        let progress = cursor.close(grant).map_err(|error| error.reason)?;
                        Ok(if progress.complete { RetainedCloneStep::Complete(receipt(progress)) } else { RetainedCloneStep::Progress(receipt(progress)) })
                    })?;
                    Ok(())
                })
            })
            .unwrap();
    }
    assert!(cursor.closed);
    caller.finish();
}
