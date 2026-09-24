
use super::{PresenceDomain, PresenceInteraction, PresencePeer, PresencePrincipalKind, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView, decode_presence_peer, decode_presence_tool_run, encode_presence_peer, encode_presence_tool_run};

fn bounded_codec_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json")).expect("presence peer codec fixture")
}

fn fixture_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn fixture_bytes(row: &serde_json::Value) -> Vec<u8> {
    let mut bytes = fixture_hex(row["prefixHex"].as_str().unwrap());
    let repeated = fixture_hex(row["repeatHex"].as_str().unwrap())[0];
    bytes.extend(std::iter::repeat_n(repeated, row["repeatCount"].as_u64().unwrap() as usize));
    bytes.extend(fixture_hex(row["suffixHex"].as_str().unwrap()));
    bytes
}

fn normalize_json_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap();
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_json_numbers),
        serde_json::Value::Object(values) => values.values_mut().for_each(normalize_json_numbers),
        _ => {}
    }
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_decoder_matches_neutral_bounded_exact_corpus() {
    let fixture = bounded_codec_fixture();
    let limits = &fixture["limits"];
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_entry_bytes, limits["maximumEntryBytes"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_text_bytes, limits["maximumTextBytes"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_presence_pack_bytes, limits["maximumPresencePackBytes"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_views, limits["maximumViews"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_interaction_domains, limits["maximumInteractionDomains"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_domain_ids, limits["maximumDomainIds"].as_u64().unwrap() as usize);
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_connected_at_ms, limits["maximumConnectedAtMs"].as_u64().unwrap());
    assert_eq!(super::PRESENCE_PEER_WIRE_LIMITS_V1.maximum_tool_run_units, limits["maximumToolRunUnits"].as_u64().unwrap());
    let mut ids = std::collections::BTreeSet::new();
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        assert!(ids.insert(id), "duplicate fixture id {id}");
        let bytes = fixture_bytes(row);
        let result = decode_presence_peer(&bytes).await;
        if row["accepted"].as_bool().unwrap() {
            let decoded = result.unwrap_or_else(|error| panic!("{id}: {error}"));
            assert_eq!(encode_presence_peer(&decoded).await, fixture_hex(row["canonicalHex"].as_str().unwrap()), "{id}");
            let mut semantic = serde_json::Value::from(crate::value::ToValue::to_value(&decoded));
            semantic.as_object_mut().unwrap().entry("views").or_insert_with(|| serde_json::json!([]));
            let mut expected = row["expected"].clone();
            normalize_json_numbers(&mut semantic);
            normalize_json_numbers(&mut expected);
            assert_eq!(semantic, expected, "{id}");
        } else {
            assert!(result.is_err(), "{id} was accepted");
        }
    }
    assert_eq!(ids.len(), fixture["cases"].as_array().unwrap().len());
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_decoder_rejects_hostile_counts_before_allocation() {
    let fixture = bounded_codec_fixture();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["id"].as_str().unwrap().starts_with("huge-")) {
        let bytes = fixture_bytes(row);
        assert!(bytes.len() < 32, "hostile count fixture must stay short");
        assert!(matches!(decode_presence_peer(&bytes).await, Err(crate::ProtocolError::LimitExceeded(_))), "{}", row["id"].as_str().unwrap());
    }
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_binary_round_trips_with_every_field_absent() {
    let peer = PresencePeer { actor: "peer-1".into(), connected_at_ms: 1000, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None, tool_run: None, principal_kind: None, active_tool: None };
    let bytes = encode_presence_peer(&peer).await;
    assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_binary_round_trips_with_every_field_present() {
    let peer = PresencePeer {
        actor: "peer-2".into(),
        connected_at_ms: 1_700_000_000_000,
        label: Some("Ada".into()),
        presence_pack: Some(b"{\"ids\":[1,2]}".to_vec()),
        user_id: Some("user-9".into()),
        role: Some("owner".into()),
        drag_ghost_json: Some("{\"kind\":\"move\"}".into()),
        interaction: Some(PresenceInteraction { app_id: "draw".into(), domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into()], hovered: vec!["n2".into()] }] }),
        color: Some(3),
        surface: Some("s.space.home@1/*#editor".into()),
        views: vec![
            PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: PresenceViewKind::Canvas { x: 1.0, y: 2.0, zoom: 1.5 }, size: [800.0, 600.0], pointer: Some([10.0, 20.0, 0.0]), ray_origin: Some([0.0, 1.0, 2.0]) },
            PresenceWindowView { window_id: "w2".into(), space: "world".into(), kind: PresenceViewKind::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 }, size: [1024.0, 768.0], pointer: None, ray_origin: None },
        ],
        ui: Some(PresenceUi { hovered_path: Some("row[0]#a".into()), focused_path: None, pressed_path: Some("btn[1]#save".into()) }),
        tool_run: Some(PresenceToolRun { tool_id: "fill".into(), state: PresenceToolRunState::Running, stage: 1, completed: 42, total: Some(100) }),
        principal_kind: None,
        active_tool: Some("brush".into()),
    };
    let bytes = encode_presence_peer(&peer).await;
    assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_active_tool_round_trips() {
    let peer = PresencePeer {
        actor: "peer-tool".into(),
        connected_at_ms: 42,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: None,
        surface: None,
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: Some("select".into()),
    };
    let bytes = encode_presence_peer(&peer).await;
    assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
    let value = crate::value::ToValue::to_value(&peer);
    assert_eq!(serde_json::Value::from(value.clone())["activeTool"], serde_json::json!("select"));
    let round: PresencePeer = crate::value::FromValue::from_value(value).unwrap();
    assert_eq!(round, peer);
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_round_trips_views_ui_color_surface() {
    let peer = PresencePeer {
        actor: "peer-4".into(),
        connected_at_ms: 5000,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: Some(11),
        surface: Some("s.space.home@1/*#viewer".into()),
        views: vec![PresenceWindowView { window_id: "w1".into(), space: "geo".into(), kind: PresenceViewKind::Geo { lng: 8.5, lat: 47.4, zoom: 12.0, bearing: 0.0, pitch: 0.0 }, size: [500.0, 400.0], pointer: Some([8.5, 47.4, 0.0]), ray_origin: None }],
        ui: Some(PresenceUi { hovered_path: None, focused_path: Some("panel[0]#tools".into()), pressed_path: None }),
        tool_run: None,
        principal_kind: None, active_tool: None,
    };
    let bytes = encode_presence_peer(&peer).await;
    let decoded = decode_presence_peer(&bytes).await.unwrap();
    assert_eq!(decoded, peer);
    assert_eq!(decoded.color, Some(11));
    assert_eq!(decoded.views.len(), 1);
    assert!(decoded.ui.is_some());
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_rejects_unknown_flag_bits() {
    // Hand-built rather than mutating an encode output: flags is a varint_u64. Bit 13 is one past
    // the frozen 0..=12 range (bit 12 = active_tool). The next field to take bit 13 must move this.
    let mut bytes = Vec::new();
    crate::write_str(&mut bytes, "peer-5");
    crate::wire::write_varint_u64(&mut bytes, 1 << 13);
    crate::wire::write_varint_u64(&mut bytes, 1000);
    let err = decode_presence_peer(&bytes).await.unwrap_err();
    assert!(matches!(err, crate::ProtocolError::Malformed { what: "presence peer flags", .. }));

    // Bit 12 IS known: missing body is a truncated-text error, not an unknown-flag refusal.
    let mut bit_twelve = Vec::new();
    crate::write_str(&mut bit_twelve, "peer-5");
    crate::wire::write_varint_u64(&mut bit_twelve, 1 << 12);
    crate::wire::write_varint_u64(&mut bit_twelve, 1000);
    assert!(decode_presence_peer(&bit_twelve).await.is_err());

    // Bit 11 IS known: missing body / bad principal-kind tag.
    let mut bit_eleven = Vec::new();
    crate::write_str(&mut bit_eleven, "peer-5");
    crate::wire::write_varint_u64(&mut bit_eleven, 1 << 11);
    crate::wire::write_varint_u64(&mut bit_eleven, 1000);
    assert!(matches!(decode_presence_peer(&bit_eleven).await.unwrap_err(), crate::ProtocolError::Malformed { what: "presence peer principal kind", .. }));
    let mut unknown_kind = bit_eleven.clone();
    unknown_kind.push(2);
    assert!(matches!(decode_presence_peer(&unknown_kind).await.unwrap_err(), crate::ProtocolError::Malformed { what: "presence peer principal kind", .. }));
}

/// 🤖️ The agent principal round-trips through BOTH codecs and survives an absent field as the
/// pre-agent wire shape — the one law that says a roster can tell an agent from the human who
/// delegated to it.
#[semio_framework_async_macros::async_test]
async fn presence_peer_principal_kind_round_trips_in_both_codecs() {
    for kind in [None, Some(PresencePrincipalKind::Human), Some(PresencePrincipalKind::Agent)] {
        let peer = PresencePeer {
            actor: "peer-agent".into(),
            connected_at_ms: 1_700_000_000_000,
            label: Some("Drafting agent".into()),
            presence_pack: None,
            user_id: Some("usr_ada".into()),
            role: Some("author".into()),
            drag_ghost_json: None,
            interaction: None,
            color: Some(4),
            surface: None,
            views: Vec::new(),
            ui: None,
            tool_run: None,
            principal_kind: kind,
            active_tool: None,
        };
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer, "binary codec, {kind:?}");

        let value = crate::value::ToValue::to_value(&peer);
        let decoded: PresencePeer = crate::value::FromValue::from_value(value).expect("JSON codec round-trips");
        assert_eq!(decoded, peer, "JSON codec, {kind:?}");
    }

    // 🔤️ The JSON spelling is the wire name, and an unknown one is a typed refusal, never `human`.
    let encoded = crate::value::ToValue::to_value(&PresencePeer {
        actor: "a".into(),
        connected_at_ms: 1,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: None,
        surface: None,
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: Some(PresencePrincipalKind::Agent),
        active_tool: None,
    });
    assert_eq!(serde_json::Value::from(encoded.clone())["principalKind"], serde_json::json!("agent"));
    let crate::value::DslValue::Object(mut fields) = encoded else { panic!("a peer encodes as an object") };
    for (key, entry) in &mut fields {
        if key == "principalKind" {
            *entry = crate::value::DslValue::String("administrator".into());
        }
    }
    assert!(<PresencePeer as crate::value::FromValue>::from_value(crate::value::DslValue::Object(fields)).is_err(), "an unknown principalKind is refused, never silently human");
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_tool_run_round_trips_every_state_with_wire_spelling() {
    let states = [
        (PresenceToolRunState::Starting, "starting"),
        (PresenceToolRunState::Running, "running"),
        (PresenceToolRunState::Paused, "paused"),
        (PresenceToolRunState::Complete, "complete"),
        (PresenceToolRunState::Finalizing, "finalizing"),
        (PresenceToolRunState::Finalized, "finalized"),
        (PresenceToolRunState::Aborting, "aborting"),
        (PresenceToolRunState::Aborted, "aborted"),
        (PresenceToolRunState::Faulted, "faulted"),
    ];
    for (tag, (state, spelling)) in states.into_iter().enumerate() {
        assert_eq!(state.wire_name(), spelling);
        assert_eq!(PresenceToolRunState::from_wire_name(spelling), Some(state));
        assert_eq!(state as u8, tag as u8);
        for total in [None, Some(9)] {
            let peer = PresencePeer { actor: "peer-6".into(), connected_at_ms: 1, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None, tool_run: Some(PresenceToolRun { tool_id: "fill".into(), state, stage: 2, completed: 3, total }), principal_kind: None, active_tool: None };
            let bytes = encode_presence_peer(&peer).await;
            assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
            let value = crate::value::ToValue::to_value(&peer);
            assert_eq!(serde_json::Value::from(value.clone())["toolRun"]["state"], spelling);
            assert_eq!(<PresencePeer as crate::value::FromValue>::from_value(value).unwrap(), peer);
        }
    }
    assert_eq!(PresenceToolRunState::from_wire_name("idle"), None);
}

/// ⏯️ The standalone summary body an `AppFrame::Ephemeral` carries is byte-identical to the peer's flag-bit-10
/// suffix, and its decoder keeps the peer limits (`completed ≤ total`, no trailing bytes).
#[semio_framework_async_macros::async_test]
async fn presence_tool_run_standalone_body_is_the_peer_suffix_with_the_peer_limits() {
    let tool_run = PresenceToolRun { tool_id: "fill".into(), state: PresenceToolRunState::Running, stage: 1, completed: 42, total: Some(100) };
    let mut body = Vec::new();
    encode_presence_tool_run(&tool_run, &mut body);
    let peer = PresencePeer { actor: "peer-7".into(), connected_at_ms: 1, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None, tool_run: Some(tool_run.clone()), principal_kind: None, active_tool: None };
    assert!(encode_presence_peer(&peer).await.ends_with(&body), "the frame body is the peer's own tool run section");
    assert_eq!(decode_presence_tool_run(&body).unwrap(), tool_run);
    let mut trailing = body.clone();
    trailing.push(0);
    assert!(decode_presence_tool_run(&trailing).is_err(), "trailing bytes are refused");
    let mut over = Vec::new();
    encode_presence_tool_run(&PresenceToolRun { completed: 101, ..tool_run }, &mut over);
    assert!(decode_presence_tool_run(&over).is_err(), "completed above total is refused");
    let mut checked = 0;
    for row in bounded_codec_fixture()["cases"].as_array().unwrap().iter().filter(|row| row["id"].as_str().unwrap().starts_with("tool-run-")) {
        let peer = fixture_bytes(row);
        let body = &peer[5..];
        assert_eq!(decode_presence_tool_run(body).is_ok(), row["accepted"].as_bool().unwrap(), "{}: the standalone body decoder agrees with the peer corpus", row["id"]);
        checked += 1;
    }
    assert!(checked >= 2, "the corpus pins standalone tool run bodies");
}

//#region 🔖️InteractionBit
async fn peer_with_interaction(interaction: Option<PresenceInteraction>) -> PresencePeer {
    PresencePeer { actor: "peer-3".into(), connected_at_ms: 1000, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction, color: None, surface: None, views: Vec::new(), ui: None, tool_run: None, principal_kind: None, active_tool: None }
}

/// 🔎️ Presence byte index: `actor str`'s own varint-length prefix (1 byte for `peer_with_interaction`'s
/// short actor id) plus the actor bytes themselves.
async fn presence_flag_byte(peer: &PresencePeer, bytes: &[u8]) -> u8 {
    bytes[1 + peer.actor.len()]
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_bit_5_round_trips_with_interaction_present() {
    let peer = peer_with_interaction(Some(PresenceInteraction { app_id: "draw".into(), domains: vec![PresenceDomain { domain: "graph".into(), granularity: "node".into(), selected: vec!["n1".into(), "n2".into()], hovered: vec![] }] })).await;
    let bytes = encode_presence_peer(&peer).await;
    assert_eq!(presence_flag_byte(&peer, &bytes).await & (1 << 5), 1 << 5, "bit 5 set when interaction present");
    assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_bit_5_round_trips_with_interaction_absent() {
    let peer = peer_with_interaction(None).await;
    let bytes = encode_presence_peer(&peer).await;
    assert_eq!(presence_flag_byte(&peer, &bytes).await & (1 << 5), 0, "bit 5 clear when interaction absent");
    assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
}

#[semio_framework_async_macros::async_test]
async fn presence_peer_interaction_round_trips_with_multiple_domains() {
    let peer = peer_with_interaction(Some(PresenceInteraction {
        app_id: "space".into(),
        domains: vec![
            PresenceDomain { domain: "outline".into(), granularity: "task".into(), selected: vec!["t1".into(), "t2".into()], hovered: vec!["t3".into()] },
            PresenceDomain { domain: "board".into(), granularity: "card".into(), selected: vec![], hovered: vec!["c1".into(), "c2".into(), "c3".into()] },
            PresenceDomain { domain: "canvas".into(), granularity: "node".into(), selected: vec!["n9".into()], hovered: vec![] },
        ],
    }))
    .await;
    let bytes = encode_presence_peer(&peer).await;
    let decoded = decode_presence_peer(&bytes).await.unwrap();
    assert_eq!(decoded, peer);
    assert_eq!(decoded.interaction.unwrap().domains.len(), 3);
}
//#endregion 🔖️InteractionBit
