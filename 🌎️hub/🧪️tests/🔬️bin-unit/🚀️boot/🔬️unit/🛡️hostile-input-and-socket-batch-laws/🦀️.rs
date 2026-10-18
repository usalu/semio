/// 🛡️ Hostile input and socket batch laws of the hub boot: the hostile input route coverage, the typed signed refusals, the credential optional routes, the transient resend codes and the maximal document socket batch.

/// 🚧️ The hostile-input fixture names every route the router registers: each `.route(` path of the
/// bootstrap source is one fixture `route`, so a new route cannot ship without its hostile vectors.
#[test]
fn the_hostile_input_fixture_covers_every_registered_route() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🚧️hostile-input-v1/🔣️.json")).unwrap();
    let covered: std::collections::BTreeSet<&str> = fixture["routes"].as_array().unwrap().iter().map(|row| row["route"].as_str().unwrap()).collect();
    let source = [semio_hub_testing::hub_source("🏗️bootstrap/🛣️routes/🦀️.rs"), semio_hub_testing::hub_source("🏗️bootstrap/🩺️readiness/🦀️.rs")].concat();
    let registered: std::collections::BTreeSet<String> = source
        .match_indices(".route(")
        .filter_map(|(at, _)| {
            let argument = source[at + ".route(".len()..].split([',', ')']).next()?.trim();
            Some(argument.trim_matches('"').rsplit("::").next()?.to_string())
        })
        .collect();
    let missing: Vec<&String> = registered.iter().filter(|route| !covered.contains(route.as_str())).collect();
    assert!(missing.is_empty(), "routes without hostile vectors: {missing:?}");
    assert!(registered.len() >= 60, "the source scan found the router: {}", registered.len());
}

/// 🚧️ Every route answers every hostile vector with a typed, signed refusal: a 4xx (or a typed 503
/// for a subsystem this hub was not configured with) carrying `x-semio-refusal` with the code its
/// status declares; the body stays the route's own, so a bare refusal discloses nothing. Never a 500,
/// never a dropped connection. A `credential-ignored` public route may succeed whatever the request
/// carries; a `credential-optional` one serves a request without a credential and refuses a forged one.
#[test]
fn every_route_answers_hostile_input_with_a_typed_signed_refusal() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🚧️hostile-input-v1/🔣️.json")).unwrap();
        let state = test_state().await;
        let session = issue_test_session(&state, "hostile-input@example.com").await;
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", session.token);
        let forged = fixture["bodies"]["forgedBearer"].as_str().unwrap();
        let malformed = fixture["bodies"]["malformedJson"].as_str().unwrap().as_bytes().to_vec();
        let wrong_schema = serde_json::to_vec(&fixture["bodies"]["wrongSchema"]).unwrap();
        let oversized = vec![b'x'; fixture["oversizedBytes"].as_u64().unwrap() as usize];
        let max = fixture["allowedStatuses"]["max"].as_u64().unwrap() as u16;
        let unavailable = fixture["allowedStatuses"]["typedUnavailable"].as_u64().unwrap() as u16;
        let mut findings = Vec::new();
        let mut answered = 0usize;
        let judge = |findings: &mut Vec<String>, label: String, public: bool, answer: Result<RawHttpResponse, String>| {
            let response = match answer {
                Ok(response) => response,
                Err(error) => return findings.push(format!("{label}: {error}")),
            };
            if response.status < 400 {
                if !public {
                    findings.push(format!("{label}: a hostile request succeeded with {}", response.status));
                }
                return;
            }
            if response.status > max && response.status != unavailable {
                findings.push(format!("{label}: answered {} {}", response.status, String::from_utf8_lossy(&response.body)));
            }
            let head = response.headers.to_ascii_lowercase();
            let Some(code) = head.lines().find_map(|line| line.strip_prefix(&format!("{}: ", semio_hub_refusal::HUB_REFUSAL_HEADER)).map(str::trim).map(str::to_string)) else {
                return findings.push(format!("{label}: {} without {}", response.status, semio_hub_refusal::HUB_REFUSAL_HEADER));
            };
            if semio_hub_refusal::refusal_code(response.status).is_some_and(|declared| declared != code && code != "insecure-transport") && !head.contains("content-type: application/json") {
                findings.push(format!("{label}: {} carries {code}, not its declared code", response.status));
            }
            if head.contains("content-type: application/json") && !response.body.is_empty() && serde_json::from_slice::<serde_json::Value>(&response.body).is_err() {
                findings.push(format!("{label}: a JSON refusal body that is not JSON"));
            }
        };
        for route in fixture["routes"].as_array().unwrap() {
            let method = route["method"].as_str().unwrap();
            let path = route["path"].as_str().unwrap();
            let public = route["public"].as_str();
            let body_kind = route["body"]["kind"].as_str();
            let json = [("content-type", "application/json"), ("origin", "http://127.0.0.1:6066")];
            let requests: Vec<(&str, Vec<(&str, &str)>, Vec<u8>)> = match body_kind {
                Some("json") => vec![
                    ("unauthenticated", json.to_vec(), wrong_schema.clone()),
                    ("forged-bearer", vec![json[0], json[1], ("authorization", forged)], wrong_schema.clone()),
                    ("malformed-json", vec![json[0], json[1], ("authorization", bearer.as_str())], malformed.clone()),
                    ("wrong-schema", vec![json[0], json[1], ("authorization", bearer.as_str())], wrong_schema.clone()),
                    ("wrong-content-type", vec![("content-type", "text/plain"), json[1], ("authorization", bearer.as_str())], wrong_schema.clone()),
                    ("oversized", vec![json[0], json[1], ("authorization", bearer.as_str())], oversized.clone()),
                ],
                Some("bytes") => vec![
                    ("unauthenticated", vec![("content-type", "application/octet-stream")], b"hostile".to_vec()),
                    ("forged-bearer", vec![("content-type", "application/octet-stream"), ("authorization", forged)], b"hostile".to_vec()),
                    ("oversized", vec![("content-type", "application/octet-stream"), ("authorization", bearer.as_str())], oversized.clone()),
                ],
                Some(_) => vec![
                    ("unauthenticated", vec![], Vec::new()),
                    ("forged-bearer", vec![("authorization", forged)], Vec::new()),
                    ("oversized", vec![("content-type", "application/json"), ("authorization", bearer.as_str())], oversized.clone()),
                ],
                None => vec![("unauthenticated", vec![], Vec::new()), ("forged-bearer", vec![("authorization", forged)], Vec::new())],
            };
            for (vector, headers, body) in &requests {
                let answer = hostile_http_request(addr, method, path, headers, body).await;
                answered += 1;
                judge(&mut findings, format!("{method} {path} [{vector}]"), public == Some("credential-ignored") || (public == Some("credential-optional") && *vector == "unauthenticated"), answer);
            }
            if !route["socket"].as_bool().unwrap_or(false) {
                let wrong_method = if method == "PATCH" { "TRACE" } else { "PATCH" };
                let answer = hostile_http_request(addr, wrong_method, path, &[("authorization", bearer.as_str())], &[]).await;
                answered += 1;
                judge(&mut findings, format!("{wrong_method} {path} [wrong-method]"), false, answer);
            }
            if let Some(segment_at) = path.find("space-h").filter(|at| !path[..*at].contains('?')) {
                for hostile in fixture["hostilePathSegments"].as_array().unwrap() {
                    let hostile_path = format!("{}{}{}", &path[..segment_at], hostile.as_str().unwrap(), &path[segment_at + "space-h".len()..]);
                    let answer = hostile_http_request(addr, method, &hostile_path, &[("authorization", bearer.as_str())], &[]).await;
                    answered += 1;
                    judge(&mut findings, format!("{method} {hostile_path} [hostile-path]"), public.is_some(), answer);
                }
            }
        }
        assert!(findings.is_empty(), "{} of {answered} hostile requests were not typed, signed refusals:\n{}", findings.len(), findings.join("\n"));
        assert!(answered > 400, "every route saw its vectors: {answered}");
    });
}

/// 🚪️ Every route that also answers anonymously refuses a PRESENTED credential that does not authenticate with the one
/// typed, localized answer of `🚧️refusal/🧫️fixtures/🪪️credential-refusal-v1`: `401`, `x-semio-refusal: unauthenticated`,
/// the RFC 6750 `invalid_token` challenge and the `HubCredentialRefusalV1` body (en + de) — identical for a malformed, a
/// forged (well-formed, never issued) and a revoked bearer, and never the anonymous view. An absent credential is still
/// the anonymous view, and a live session is served.
#[test]
fn credential_optional_routes_refuse_a_revoked_or_forged_session_instead_of_answering_anonymously() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🚧️refusal/🧪️tests/🧫️fixtures/🪪️credential-refusal-v1/🔣️.json")).expect("credential refusal fixture");
        let answer = &fixture["answer"];
        let state = test_state().await;
        let session = issue_test_session(&state, "optional-caller@example.com").await;
        let public_space = create_space_for_test(&state, &session.user_id, "Credential-optional public space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let addr = spawn_server(state.clone()).await;
        let live = format!("Bearer {}", session.token);
        let forged = format!("Bearer {}", SessionCapability::mint().expect("well-formed capability").expose_once());
        let paths: Vec<(String, u16)> = fixture["routes"].as_array().expect("routes").iter().map(|route| (route["path"].as_str().expect("path").replace("{publicSpace}", &public_space), route["anonymousStatus"].as_u64().expect("anonymous status") as u16)).collect();
        let assert_refused = |path: &str, vector: &str, response: &RawHttpResponse| {
            assert_eq!(u64::from(response.status), answer["status"].as_u64().unwrap(), "{path} {vector}: refused, never the anonymous view");
            let head = response.headers.to_ascii_lowercase();
            assert!(head.contains(&format!("{}: {}", semio_hub_refusal::HUB_REFUSAL_HEADER, answer["refusal"].as_str().unwrap())), "{path} {vector}: signed refusal: {head}");
            assert!(head.contains(&format!("www-authenticate: {}", answer["challenge"].as_str().unwrap().to_ascii_lowercase())), "{path} {vector}: RFC 6750 challenge: {head}");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&response.body).expect("refusal JSON"), answer["body"], "{path} {vector}: the declared localized body");
        };
        for (path, anonymous) in &paths {
            assert_eq!(raw_http_get(addr, path, &[]).await.status, *anonymous, "{path}: no credential is the anonymous view");
            assert_eq!(raw_http_get(addr, path, &[("Authorization", live.as_str())]).await.status, 200, "{path}: a live session is served");
            for (vector, bearer) in [("malformed", "Bearer not-a-session"), ("forged", forged.as_str())] {
                assert!(fixture["presented"].as_array().unwrap().iter().any(|name| name == vector));
                assert_refused(path, vector, &raw_http_get(addr, path, &[("Authorization", bearer)]).await);
            }
        }
        let authenticated = state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().expect("live session");
        state.directory.revoke_auth_session(&authenticated.id, "test-revocation", Some(&session.user_id), "optional-caller").await.expect("revoke");
        for (path, anonymous) in &paths {
            assert_refused(path, "revoked", &raw_http_get(addr, path, &[("Authorization", live.as_str())]).await);
            assert_eq!(raw_http_get(addr, path, &[]).await.status, *anonymous, "{path}: the anonymous view stays open");
        }
    });
}

/// ⏳️ A document batch refused for a TRANSIENT reason carries exactly the declared `HubTransientApplyRefusalMessageV1`
/// (`🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1`): the db engine's `Unavailable` (DB I/O admission or capacity
/// exhausted) and a paced-out agent command answer `[{level: warning, code: hub.unavailable, message: <reason>}]`, so the
/// client resends instead of discarding the batch; a permanent refusal carries no such message.
#[test]
fn a_transiently_refused_batch_names_the_declared_resend_code_and_a_permanent_one_does_not() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🚧️refusal/🧪️tests/🧫️fixtures/⏳️transient-apply-refusal-v1/🔣️.json")).expect("transient refusal fixture");
    let answer = &fixture["answer"];
    for cause in fixture["causes"].as_array().expect("causes") {
        let reason = cause["reason"].as_str().expect("reason");
        let encoded = match cause["cause"].as_str() {
            Some("db-unavailable") => messages_for_error(&db::DbError::Unavailable(reason.to_string())),
            Some("agent-rate-limited") => transient_apply_refusal_messages(reason),
            other => panic!("unknown transient cause {other:?}"),
        };
        let messages: serde_json::Value = serde_json::from_slice(&encoded).expect("messages are JSON");
        assert_eq!(messages, serde_json::json!([{ "level": answer["level"], "code": answer["code"], "message": reason }]), "{cause}");
    }
    for cause in fixture["permanentCauses"].as_array().expect("permanent causes") {
        let reason = cause["reason"].as_str().expect("reason").to_string();
        assert!(messages_for_error(&db::DbError::InvalidArgument(reason.clone())).is_empty(), "{cause}");
        assert!(messages_for_error(&db::DbError::Conflict(reason)).is_empty(), "{cause}");
    }
}

/// ⚖️ C12 P1 (ticket 26/09/23 session 14): a document socket's gate admits, on a full budget, the largest batch the
/// document backbone declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES` — a link cut's whole outbox arrives as
/// one); an emptied budget refuses transiently (`hub.unavailable`, the client resends); a batch that was admitted but not
/// committed is admitted again once the budget refilled — never a replay — while a committed operation is.
#[test]
fn a_document_socket_admits_a_declared_maximal_batch_and_readmits_an_uncommitted_one() {
    run_socket_test(|| async {
        let document = WireArtifactId("p1-socket-budget".to_string());
        let gate = document_socket_gate(db::security::RoleBasedPolicy::new().with_grant(db::security::Grant::allow("editor", &["db", "document", "*", "**"], &[db::security::Action::Write])));
        let principal = db::security::Principal::new(ActorId("actor-1".to_string()), db::security::TenantId::from("space-1"), vec!["editor".to_string()]);
        let tenant = db::security::TenantId::from("space-1");
        let template = sample_envelope("p1", &document).await;
        let envelopes: Vec<MutationEnvelope> = (0..protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES).map(|index| MutationEnvelope { mutation_id: protocol::MutationId(format!("p1-{index}")), ..template.clone() }).collect();
        assert!(admit_writes(&gate, &principal, &tenant, &document, &envelopes, 0).await.is_ok(), "a full budget admits a declared-maximal batch");
        let refused = admit_writes(&gate, &principal, &tenant, &document, &envelopes[..1], 1).await.expect_err("an emptied budget refuses");
        let messages: serde_json::Value = serde_json::from_slice(&messages_for_error(&refused)).expect("messages are JSON");
        assert_eq!(messages[0]["code"], semio_hub_refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE, "an emptied budget is a transient refusal: {refused}");
        let refilled_ms = (protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES as u64 * 1_000).div_ceil(DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND as u64) + 1;
        assert!(admit_writes(&gate, &principal, &tenant, &document, &envelopes, refilled_ms).await.is_ok(), "an admitted but uncommitted batch is readmitted, never a replay");
        gate.record_committed(&[(&envelopes[0].actor, &envelopes[0].mutation_id)], refilled_ms);
        let replayed = admit_writes(&gate, &principal, &tenant, &document, &envelopes[..1], refilled_ms + 1_000).await.expect_err("a committed operation is a replay within the window");
        assert!(matches!(replayed, db::DbError::Conflict(_)) && messages_for_error(&replayed).is_empty(), "a replay is permanent: {replayed}");
    });
}
