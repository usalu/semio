use super::*;

#[test]
fn semio_envelope_identity_matches_independent_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️envelope-identity/🔣️.json")).expect("neutral envelope vectors");
    let expected = &vectors["expected"];
    for case in vectors["cases"].as_array().unwrap() {
        let item = &case["envelope"];
        let envelope = SemioEnvelope {
            plugin: item["plugin"].as_str().unwrap().into(),
            artifact: item["artifact"].as_str().unwrap().into(),
            component: Component::parse(item["component"].as_str().unwrap()).unwrap(),
            version: item["version"].as_u64().unwrap() as u16,
        };
        assert_eq!(envelope.matches_identity(expected["id"].as_str().unwrap(), Component::parse(expected["component"].as_str().unwrap()).unwrap(), expected["version"].as_u64().unwrap() as u16), case["matches"].as_bool().unwrap(), "{}", case["name"]);
    }
    println!("[DEBUG] Semio native envelope identity matches eleven neutral vectors validated independently by Ajv");
}

#[test]
fn text_preamble_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Dsl, version: 1 };
    let wrapped = wrap_text(&env, "positions [id:TEXT] { }");
    let (parsed, body) = split_text_preamble(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert!(body.starts_with("positions"));
}

#[test]
fn binary_header_round_trip() {
    let env = SemioEnvelope { plugin: "gis".into(), artifact: "gismap".into(), component: Component::Pack, version: 1 };
    let inner = b"payload-bytes";
    let wrapped = wrap_binary(&env, inner);
    let (parsed, payload) = unwrap_binary(&wrapped).unwrap();
    assert_eq!(parsed, env);
    assert_eq!(payload, inner);
}

#[test]
fn sniff_text_and_binary() {
    let dsl_env = SemioEnvelope::from_envelope_id("gis.gismap", Component::Dsl, 1).unwrap();
    let text = wrap_text(&dsl_env, "schema=gis.map id=x");
    assert_eq!(sniff(text.as_bytes()).unwrap().component, Component::Dsl);
    let bin = wrap_binary(&SemioEnvelope::from_envelope_id("gis.gismap", Component::Pack, 1).unwrap(), b"x");
    assert_eq!(sniff(&bin).unwrap().component, Component::Pack);
}
