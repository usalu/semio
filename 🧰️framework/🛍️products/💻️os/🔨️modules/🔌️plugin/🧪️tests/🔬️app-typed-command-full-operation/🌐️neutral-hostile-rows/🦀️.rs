    /// 🌐️ Neutral hostile rows: the language neutral empty, single, max and plus-one rows and every hostile row against the owned state machine.

    #[test]
    fn language_neutral_empty_single_max_and_plus_one_match_the_test_only_oracle() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
        let maximum = fixture["capacities"]["maxOutputBytes"].as_u64().expect("maximum output bytes") as usize;
        let owned = OwnedTypedCommandCensus;
        let oracle = SerdeJsonTypedCommandCensus;
        for case in fixture["cases"].as_array().expect("fixture cases") {
            let description = case["description"].as_str().expect("fixture description");
            let expected = TypedCommandCensusDecision { bytes: case["expectedBytes"].as_u64().expect("fixture expected bytes") as usize, accepted: case["accepted"].as_bool().expect("fixture acceptance") };
            assert_eq!(owned.decide(description, maximum), expected);
            assert_eq!(oracle.decide(description, maximum), expected);
        }
    }

    #[test]
    fn every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
        assert_eq!(fixture["capacities"]["maxFaultBytes"].as_u64(), Some(TYPED_OPERATION_FAULT_BYTES as u64));
        assert_eq!(fixture["grantLaws"].as_array().expect("grant laws").len(), 4);
        assert_eq!(fixture["freshnessLaws"].as_array().expect("freshness laws").len(), 3);
        assert_eq!(fixture["admissionLaws"].as_array().expect("admission laws").len(), 4);
        assert_eq!(fixture["publicationLaws"].as_array().expect("publication laws").len(), 3);
        assert_eq!(fixture["laneTrace"].as_array().expect("lane trace").len(), 9);
        assert_eq!(fixture["rawPageLaws"].as_array().expect("raw page laws").len(), 3);
        assert_eq!(fixture["faultLaws"].as_array().expect("fault laws").len(), 3);
        assert_eq!(fixture["closeLaws"].as_array().expect("close laws").len(), 4);
        let declared_rows = fixture["cases"].as_array().expect("output census").len()
            + fixture["grantLaws"].as_array().expect("grant laws").len()
            + fixture["freshnessLaws"].as_array().expect("freshness laws").len()
            + fixture["admissionLaws"].as_array().expect("admission laws").len()
            + fixture["publicationLaws"].as_array().expect("publication laws").len()
            + fixture["laneTrace"].as_array().expect("lane trace").len()
            + fixture["rawPageLaws"].as_array().expect("raw page laws").len()
            + fixture["faultLaws"].as_array().expect("fault laws").len()
            + fixture["closeLaws"].as_array().expect("close laws").len();
        assert_eq!(declared_rows, 40, "the production seam assertions supplement, never replace, the forty language-neutral rows");

        for law in fixture["grantLaws"].as_array().expect("grant laws") {
            let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![Value::Null]);
            for (phase, _) in phases.iter().enumerate() {
                let mut machine = OwnedFixtureMachine::new(phase);
                let actual = machine.grant(law["fuel"].as_u64().expect("grant fuel"), law["deadline"] == "expired", law["cancelled"].as_bool().expect("grant cancellation"));
                assert_eq!(actual, SerdeFixtureOracle(law).transition());
                assert_eq!(machine.advanced, law["advancedUnits"].as_u64().expect("advanced units") as usize);
            }
        }
        for law in fixture["freshnessLaws"].as_array().expect("freshness laws") {
            let accepted = OwnedFixtureMachine::freshness(
                law["operationRevision"].as_u64().expect("operation revision"),
                law["liveRevision"].as_u64().expect("live revision"),
                law["operationGeneration"].as_u64().expect("operation generation"),
                law["liveGeneration"].as_u64().expect("live generation"),
            );
            assert_eq!(accepted, law["accepted"].as_bool().expect("freshness oracle"));
        }
        for law in fixture["admissionLaws"].as_array().expect("admission laws") {
            let roots = law["roots"].as_u64().unwrap_or(fixture["capacities"]["roots"].as_u64().expect("root capacity"));
            let required = law["requiredRoots"].as_u64().unwrap_or(roots);
            let capacity = law["capacity"].as_u64().unwrap_or(1);
            let occupied = law["occupied"].as_u64().unwrap_or(0);
            assert_eq!(OwnedFixtureMachine::admission(capacity as usize, occupied as usize, roots as usize, required as usize), law["accepted"].as_bool().expect("admission oracle"));
        }
        for law in fixture["publicationLaws"].as_array().expect("publication laws") {
            let mut machine = OwnedFixtureMachine::new(0);
            let actual = machine.publish(law["attempt"].as_u64().expect("publication attempt") as u8, law["acknowledged"].as_bool().expect("publication ACK"), fixture["capacities"]["maximumRetries"].as_u64().expect("maximum retries") as u8);
            assert_eq!(actual, SerdeFixtureOracle(law).transition());
            assert_eq!(machine.retained_page, !law["acknowledged"].as_bool().expect("publication ACK"));
        }
        let mut lane_machine = OwnedFixtureMachine::new(0);
        for law in fixture["laneTrace"].as_array().expect("lane trace") {
            assert_eq!(law["sequence"].as_u64().expect("lane sequence") as usize, lane_machine.advanced);
            assert_eq!(law["items"].as_u64(), Some(1));
            assert_eq!(law["receipt"].as_bool(), Some(true));
            assert_eq!(lane_machine.publish(0, law["acknowledged"].as_bool().expect("lane ACK"), 2), FixtureTransition::Advance);
            lane_machine.advanced += 1;
            lane_machine.retained_page = true;
        }
        for law in fixture["rawPageLaws"].as_array().expect("raw page laws") {
            let declared = law["declaredBytes"].as_u64().expect("declared bytes") as usize;
            let mut admitted = 0usize;
            let mut returned = 0usize;
            for page in law["pageBytes"].as_array().expect("page bytes") {
                let page = page.as_u64().expect("page length") as usize;
                if admitted.checked_add(page).is_some_and(|next| next <= declared) {
                    admitted += page;
                } else {
                    returned += 1;
                }
            }
            if admitted != declared && returned == 0 {
                returned = law["pageBytes"].as_array().expect("page bytes").len();
            }
            let accepted = admitted == declared && returned == 0;
            assert_eq!(accepted, law["accepted"].as_bool().expect("raw page acceptance"));
            assert_eq!(returned, law["returnedOwners"].as_u64().expect("returned page owners") as usize);
        }
        for law in fixture["faultLaws"].as_array().expect("fault laws") {
            let mut input = String::new();
            for segment in law["segments"].as_array().expect("fault segments") {
                let scalar = segment["scalar"].as_str().expect("fault scalar");
                for _ in 0..segment["count"].as_u64().expect("fault scalar count") {
                    input.push_str(scalar);
                }
            }
            let mut oracle_bytes = 0usize;
            for scalar in input.chars() {
                let next = oracle_bytes + scalar.len_utf8();
                if next > TYPED_OPERATION_FAULT_BYTES {
                    break;
                }
                oracle_bytes = next;
            }
            let bounded = ArtifactBoundedToolFault::from_fault(&Fault::new(FaultOrigin::App, FaultCode::new("fixture"), input));
            assert_eq!(bounded.as_bytes().len(), oracle_bytes);
            assert_eq!(bounded.as_bytes().len(), law["retainedBytes"].as_u64().expect("retained fault bytes") as usize);
            assert!(std::str::from_utf8(bounded.as_bytes()).is_ok());
        }
        for law in fixture["closeLaws"].as_array().expect("close laws") {
            if let Some(value) = law["value"].as_str() {
                let mut value = value.to_string();
                let (items, bytes) = OwnedFixtureMachine::close_scalar(&mut value, law["maximumBytes"].as_u64().expect("close byte credit") as usize);
                assert_eq!(items, law["releasedItems"].as_u64().expect("released items") as usize);
                assert_eq!(bytes, law["releasedBytes"].as_u64().expect("released bytes") as usize);
                assert_eq!(value, law["remaining"].as_str().expect("remaining value"));
            } else {
                let mut machine = OwnedFixtureMachine::new(0);
                machine.owners = law["owners"].as_u64().expect("close owners") as usize;
                for _ in 0..law["calls"].as_u64().expect("close calls") {
                    machine.close_owner(law["maximumItems"].as_u64().expect("close item credit") as usize);
                }
                assert_eq!(machine.terminal, law["terminal"].as_bool().expect("terminal oracle"));
            }
        }
    }
