
use super::*;

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
