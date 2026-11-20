//! 🧫️ Child identity receiving retains the actual original source through every interrupted frontier.
use super::*;
use semio_framework_value::DslValue;

#[path="../../../../../../../🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;

fn input(law:&serde_json::Value)->DslValue {
    let dialect=&law["target"]["dialect"];
    DslValue::Object(vec![("childId".into(),DslValue::String(law["childId"].as_str().unwrap().into())),("target".into(),DslValue::Object(vec![("artifactId".into(),DslValue::String(law["target"]["artifactId"].as_str().unwrap().into())),("dialect".into(),DslValue::Object(["artifactKind","standard","subset"].map(|key|(key.into(),DslValue::String(dialect[key].as_str().unwrap().into()))).into()))]))])
}
#[test]
fn original_child_receive_same_caller_all_partial_prefixes_and_literal_target() {
    let laws:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
    for law in laws["cases"].as_array().unwrap() {
        let mut completed=false;
        for cut in (0..2048u64).map(Some).chain(std::iter::once(None)) {
            if completed&&cut.is_some() { continue }
            let mut bank=driver::bank();
            let source=driver::admit(&mut bank,input(law));
            let mut inspected=false;
            let received=driver::lifecycle(<ArtifactChild<()> as NativeReceive<DslValue>>::receiver(),source,&mut bank,4096,cut,|output:&ArtifactChild<()>|{
                inspected=true;
                assert_eq!(output.child_id,law["childId"].as_str().unwrap());
                assert_eq!(output.target.artifact_id,law["target"]["artifactId"].as_str().unwrap());
                assert_eq!(output.target.dialect.subset,law["target"]["dialect"]["subset"].as_str().unwrap());
                assert!(output.local_owner.is_none());
            });
            assert_eq!(received,Ok(inspected));
            if cut.is_none() { assert!(inspected,"an uncut receive publishes its child"); completed=true; } else if inspected { completed=true; }
        }
        assert!(completed);
    }
}

#[test]
fn borrowed_dsl_child_receive_matches_persisted_identity() {
    use semio_framework_dsl_record::{DslReceivingField, FieldValue, RecordValue, receiving::NativeSnapshotDecodeOwner};
    use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
    use semio_framework_value::{RetainedWorkOutcome, ToValue, retirement::controlled::ControlledRetirement};
    let grant = RetainedCloneGrant {
        maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20, maximum_depth: 64,
    };
    let laws: serde_json::Value = serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
    for law in laws["cases"].as_array().unwrap() {
        for valid in [true, false] {
            let text = |value: &serde_json::Value| FieldValue::Text(value.as_str().unwrap().to_owned());
            let target = &law["target"];
            let dialect = &target["dialect"];
            let subset = if valid { text(&dialect["subset"]) } else { FieldValue::UInt(0) };
            let target = FieldValue::Record(RecordValue {
                fields: [(0, text(&target["artifactId"])), (1, text(&dialect["artifactKind"])),
                         (2, text(&dialect["standard"])), (3, subset)].into_iter().collect(),
            });
            let source = FieldValue::Record(RecordValue {
                fields: [(0, text(&law["childId"])), (1, target)].into_iter().collect(),
            });
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(902), Generation(23), grant, || Some(1), u64::MAX, 1 << 20);
            let mut decoder = OriginalJobDecodeOwner::empty();
            let mut output = None;
            let mut failure = None;
            caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_work_loan(1 << 20, u64::MAX, |cx| {
                let turn = cx.with_native_receiving::<_, ValueError>(|native| {
                    let mut owner = NativeSnapshotDecodeOwner::new(native, grant);
                    Ok(ArtifactChild::<()>::receive_value(&source, &mut output, &mut owner))
                })?;
                assert!(turn.work > 0);
                assert_eq!(turn.result.is_ok(), valid);
                cx.consume_fuel(turn.work);
                failure = turn.result.err();
                Ok(())
            })).unwrap();
            if valid {
                let child = output.as_ref().expect("borrowed receiver publishes the original identity");
                assert_eq!(child.to_value(), input(law));
                assert!(child.local_owner.is_none());
            } else {
                assert!(output.is_none());
                assert!(!decoder.terminal_is_empty(), "the malformed final field preserves prior copied texts for retirement");
            }

            let mut retirement = ControlledRetirement::retain((output, failure));
            for _ in 0..4096 {
                if retirement.terminal_is_empty() { break; }
                caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
                    let demand = retirement.selected_demand(grant.maximum_copy_bytes)?;
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                        let result = retirement.step(grant);
                        RetainedWorkOutcome { result, work: retirement.last_turn_work() }
                    })?;
                    Ok(())
                })).unwrap();
            }
            assert!(retirement.terminal_is_empty());
            for _ in 0..4096 {
                if decoder.terminal_is_empty() { break; }
                caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
                    decoder.close_on_source(cx)?;
                    Ok(())
                })).unwrap();
            }
            assert!(decoder.terminal_is_empty());
            caller.retire(grant);
        }
    }
}
