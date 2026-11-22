use super::*;

#[test]
fn owned_ascii_string_turns_match_the_scalar_grammar_and_serde_json() {
    use semio_framework_value::retained_clone::AuthoringWallet;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧵️string-runs.json")).unwrap();
    let maximum = fixture["maximumTurnWork"].as_u64().unwrap() as usize;
    for row in fixture["cases"].as_array().unwrap() {
        let repetition = row["repeat"].as_u64().map(|count| row["decoded"].as_str().unwrap().repeat(count as usize));
        let repeated_source = repetition.as_ref().map(|text| serde_json::to_string(text).unwrap());
        let source = row["source"].as_str().or(repeated_source.as_deref()).unwrap().as_bytes();
        let run = |batch| {
            let mut parser = JsonParseCursor::new(JsonMemberPolicy::Reject);
            let accepts = std::cell::Cell::new(true);
            let mut accepted = |_| accepts.get();
            let mut control = NativeDecodeControl::new_retained(&mut accepted);
            let mut wallet = AuthoringWallet::new(RetainedCloneGrant {
                maximum_items: 10000, maximum_copy_bytes: 10000, maximum_capacity_bytes: 10000,
                maximum_release_bytes: 10000, maximum_depth: 16,
            }, 10000);
            let mut total = RetainedCloneProgress::default();
            let mut actions = 0;
            let mut copied_run = false;
            for _ in 0..1000 {
                let demand = parser.normal_step_demands(source).unwrap();
                let (units, copy) = if batch { parser.owned_turn_quote(source, maximum, maximum) } else { (1, 0) };
                let grant = RetainedCloneGrant {
                    maximum_items: units, maximum_copy_bytes: demand.copy_bytes.max(copy),
                    maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes,
                    maximum_depth: demand.depth,
                };
                if batch && copy > 1 && !copied_run {
                    let restricted = RetainedCloneGrant { maximum_copy_bytes: 0, ..grant };
                    let before = parser.position();
                    let denied = wallet.run(restricted, units as u64, |effect|
                        parser.step_owned_source(source, units, &mut control, effect)
                    ).unwrap().unwrap();
                    assert_eq!(denied.output, Ok(None));
                    assert_eq!(denied.performed, 0);
                    assert_eq!(denied.actual, None);
                    assert_eq!(parser.position(), before);
                    assert_eq!(parser.normal_step_progress(), RetainedCloneProgress::default());
                    assert_eq!(parser.owned_turn_quote(source, maximum, maximum), (units, copy));
                    accepts.set(false);
                    let canceled = wallet.run(grant, units as u64, |effect|
                        parser.step_owned_source(source, units, &mut control, effect)
                    ).unwrap().unwrap();
                    accepts.set(true);
                    assert_eq!(canceled.output.unwrap_err().kind(), ValueRefusalKind::Canceled);
                    assert_eq!(canceled.performed, 0);
                    assert_eq!(canceled.actual, None);
                    assert_eq!(parser.position(), before);
                    assert_eq!(parser.normal_step_progress(), RetainedCloneProgress::default());
                }
                let turn = wallet.run(grant, units as u64, |effect|
                    if batch {
                        parser.step_owned_source(source, units, &mut control, effect)
                    } else {
                        parser.step_source(source, 1, &mut control, effect, false)
                    }
                ).unwrap().unwrap();
                let progress = parser.normal_step_progress();
                wallet.post(progress).unwrap();
                assert!(progress.fits(grant));
                assert!(turn.performed <= maximum as u128);
                assert_eq!(turn.performed, u128::from(parser.last_turn_work()));
                assert_eq!(turn.performed, progress.copied_items as u128);
                if copy > 1 && progress.copied_bytes > 1 {
                    assert_eq!(progress.copied_bytes, progress.copied_items);
                    assert_eq!(progress.retained_capacity_bytes, 0);
                    copied_run = true;
                }
                total = total.checked_add(progress).unwrap();
                actions += 1;
                match turn.output {
                    Ok(None) => {},
                    Ok(Some(value)) => return (Ok(value), total, actions, copied_run),
                    Err(error) => return (Err(error), total, actions, copied_run),
                }
            }
            panic!("{} did not complete", row["name"]);
        };
        let (scalar, scalar_progress, scalar_actions, _) = run(false);
        let (batched, batched_progress, batched_actions, copied_run) = run(true);
        assert_eq!(batched, scalar, "{}", row["name"]);
        assert_eq!(batched_progress, scalar_progress, "{}", row["name"]);
        assert!(batched_actions < scalar_actions, "{}", row["name"]);
        let independent = serde_json::from_slice::<serde_json::Value>(source);
        if let Some(decoded) = repetition.as_deref().or(row["decoded"].as_str()) {
            assert_eq!(independent.unwrap(), decoded);
            assert_eq!(batched.unwrap(), Value::String(decoded.to_owned()));
            assert!(copied_run);
        } else {
            assert!(independent.is_err());
            assert_eq!(batched.unwrap_err().to_string(), row["error"].as_str().unwrap());
        }
    }
}
