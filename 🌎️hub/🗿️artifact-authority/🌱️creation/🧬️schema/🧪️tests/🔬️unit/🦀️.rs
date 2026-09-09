use super::*;

#[test]
fn creation_facts_follow_neutral_terminal_and_exact_pair_transitions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📚️operation-v1/🔣️.json")).unwrap();
    let intent: ArtifactCreationIntentV1 = directory::os_pack::json::from_json_str(&fixture["intent"].to_string()).unwrap();
    let prepared: ArtifactCreationPreparedV1 = directory::os_pack::json::from_json_str(&fixture["prepared"].to_string()).unwrap();
    let receipt: ArtifactCreationReceiptV1 = directory::os_pack::json::from_json_str(&fixture["receipt"].to_string()).unwrap();
    assert_eq!(artifact_creation_command_digest_v1(&intent.scope.space_id, &intent.request).unwrap(), fixture["intent"]["commandSha256"]);
    let fact = |kind: &str, revision: u64| ArtifactCreationFactV1 {
        actor_user_id: intent.actor.user_id.clone(),
        request_id: intent.request.request_id.clone(),
        revision,
        recorded_at_ms: intent.accepted_at_ms + revision - 1,
        body: match kind {
            "accepted" => ArtifactCreationFactBodyV1::Accepted { intent: intent.clone() },
            "prepared" => ArtifactCreationFactBodyV1::Prepared { candidate: prepared.clone() },
            "committed" => ArtifactCreationFactBodyV1::Committed { receipt: receipt.clone() },
            "cancelled" => ArtifactCreationFactBodyV1::Cancelled,
            "failed" => ArtifactCreationFactBodyV1::Failed,
            _ => panic!("unhandled neutral fact"),
        },
    };
    for row in fixture["transitions"].as_array().unwrap() {
        let state = row["state"].as_str().unwrap();
        let mut facts = match state {
            "absent" => vec![],
            "accepted" => vec![fact("accepted", 1)],
            "prepared" => vec![fact("accepted", 1), fact("prepared", 2)],
            "committed" => vec![fact("accepted", 1), fact("prepared", 2), fact("committed", 3)],
            "cancelled" | "failed" => vec![fact("accepted", 1), fact(state, 2)],
            _ => panic!("unhandled neutral state"),
        };
        facts.push(fact(row["fact"].as_str().unwrap(), facts.len() as u64 + 1));
        let result = ArtifactCreationOperationV1::fold(&facts);
        assert_eq!(result.is_ok(), !row["next"].is_null(), "{row}: {result:?}");
        if let Ok(operation) = result {
            assert!(operation.status().validate());
            assert_eq!(operation.status().ready.is_some(), row["next"] == "committed");
            let decoded: Vec<ArtifactCreationFactV1> = directory::os_pack::json::from_json_str(&directory::os_pack::json::to_json_string(&facts)).unwrap();
            assert_eq!(decoded, facts);
        }
    }
    let cancellations: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📚️operation-v1/🛑️cancel.json")).unwrap();
    for row in cancellations["cases"].as_array().unwrap() {
        let mut facts = vec![fact("accepted", 1)];
        match row["state"].as_str().unwrap() {
            "prepared" => facts.push(fact("prepared", 2)),
            "committed" => {
                facts.push(fact("prepared", 2));
                facts.push(fact("committed", 3));
            }
            "cancelled" | "failed" => facts.push(fact(row["state"].as_str().unwrap(), 2)),
            _ => {}
        }
        let mut actor = intent.actor.clone();
        actor.session_id = "fresh-authorized-session".into();
        let append = ArtifactCreationFactAppendV1 {
            actor,
            space_id: intent.scope.space_id.clone(),
            request_id: intent.request.request_id.clone(),
            command_sha256: intent.command_sha256.clone(),
            expected_revision: row["expectedRevision"].as_u64().unwrap(),
            recorded_at_ms: 1000,
            body: ArtifactCreationFactBodyV1::Cancelled,
        };
        let result = decide_artifact_creation_fact_append_v1(&facts, &append, row["observedAtMs"].as_u64().unwrap());
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{row}");
        if let Ok(next) = result {
            if let Some(next) = next {
                facts.push(next);
            }
            let operation = ArtifactCreationOperationV1::fold(&facts).unwrap();
            assert_eq!(format!("{:?}", operation.phase).to_lowercase(), row["phase"].as_str().unwrap());
        }
    }
    for case in [
        "actor",
        "request",
        "revision",
        "clock",
        "intent-digest",
        "catalog-generation",
        "expected-catalog-generation",
        "pair",
        "descriptor",
        "receipt-sequence",
        "receipt-checkpoint",
        "receipt-ids-missing",
        "receipt-ids-duplicate",
        "receipt-ids-empty",
        "private-locator",
    ] {
        let mut facts = vec![fact("accepted", 1), fact("prepared", 2), fact("committed", 3)];
        match case {
            "actor" => facts[1].actor_user_id = "other-user".into(),
            "request" => facts[1].request_id = "22".repeat(16),
            "revision" => facts[1].revision = 3,
            "clock" => facts[1].recorded_at_ms = 999,
            "intent-digest" | "catalog-generation" | "expected-catalog-generation" => {
                if let ArtifactCreationFactBodyV1::Accepted { intent } = &mut facts[0].body {
                    if case == "intent-digest" {
                        intent.command_sha256 = "00".repeat(32);
                    } else if case == "catalog-generation" {
                        intent.catalog_generation = "00".repeat(32);
                    } else {
                        intent.request.expected_catalog_generation_id = "77".repeat(32);
                    }
                }
            }
            "pair" | "descriptor" => {
                if let ArtifactCreationFactBodyV1::Prepared { candidate } = &mut facts[1].body {
                    if case == "pair" {
                        candidate.pack[0] ^= 1;
                    } else {
                        candidate.descriptor.bootstrap_frontier.epoch = 1;
                    }
                }
            }
            "receipt-sequence" | "receipt-checkpoint" => {
                if let ArtifactCreationFactBodyV1::Committed { receipt } = &mut facts[2].body {
                    if case == "receipt-sequence" {
                        receipt.event_seq_last += 1;
                    } else {
                        receipt.checkpoint_id = ArtifactHash([1; 32]);
                    }
                }
            }
            "receipt-ids-missing" | "receipt-ids-duplicate" | "receipt-ids-empty" => {
                if let ArtifactCreationFactBodyV1::Committed { receipt } = &mut facts[2].body {
                    match case {
                        "receipt-ids-missing" => {
                            receipt.event_ids.pop();
                        }
                        "receipt-ids-duplicate" => receipt.event_ids[1] = receipt.event_ids[0].clone(),
                        _ => receipt.event_ids[1].clear(),
                    }
                }
            }
            "private-locator" => {
                if let ArtifactCreationFactBodyV1::Prepared { candidate } = &mut facts[1].body {
                    candidate.checkpoint.pack.storage_key.push('x');
                }
            }
            _ => unreachable!(),
        }
        assert!(ArtifactCreationOperationV1::fold(&facts).is_err(), "{case}");
    }
    println!("[DEBUG] creation fact reducer: neutral transitions=30 cancellation races=9 hostile identities/pairs/receipts=14 independent Node intent SHA256=1; backend persistence not executed");
}
