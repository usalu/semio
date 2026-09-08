
use super::*;
use semio_framework_hash::Sha256;

#[test]
fn trusted_browser_actor_metadata_and_generation_match_neutral_corpus() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
    let source = DocumentBrowserActorSourceV1 { component_sha256: fixture["closed"]["sourceComponentSha256"].as_str().unwrap(), descriptor_byte_sha256: fixture["closed"]["sourceDescriptorByteSha256"].as_str().unwrap() };
    for law in fixture["cases"].as_array().unwrap() {
        let mut value = if law["kind"] == "none" { serde_json::json!({"kind":"none"}) } else { fixture["closed"].clone() };
        for (key, field) in law["set"].as_object().unwrap() {
            value[key] = field.clone();
        }
        if let Some(remove) = law["remove"].as_str() {
            value.as_object_mut().unwrap().remove(remove);
        }
        let parsed = serde_json::from_value::<TrustedBundleBrowserActorV1>(value);
        let result = parsed.map_err(|_| catalog("actor shape")).and_then(|actor| {
            actor.validate(source, law["renderer"].as_str().unwrap())?;
            Ok(actor)
        });
        assert_eq!(result.is_ok(), law["accepted"].as_bool().unwrap(), "{}", law["id"]);
        if let Ok(actor) = result {
            let public = directory::os_pack::json::to_json_string(&actor.identity());
            assert!(!public.contains("path") && !public.contains("byteLength"));
        }
    }
    for (actor, expected) in [(serde_json::from_value::<TrustedBundleBrowserActorV1>(fixture["closed"].clone()).unwrap(), &fixture["encodingSha256"]), (TrustedBundleBrowserActorV1::None, &fixture["noneEncodingSha256"])] {
        let mut bytes = Vec::new();
        actor.append_generation(&mut bytes).unwrap();
        assert_eq!(directory::os_directory::hex_lower(&Sha256::digest(&bytes)), expected.as_str().unwrap());
    }
    let raw = serde_json::to_string(&fixture["closed"]).unwrap();
    assert_eq!(raw.matches("\"byteLength\":3").count(), 1);
    for law in fixture["rawLengths"].as_array().unwrap() {
        let candidate = raw.replace("\"byteLength\":3", &format!("\"byteLength\":{}", law["token"].as_str().unwrap()));
        let admitted = serde_json::from_str::<TrustedBundleBrowserActorV1>(&candidate).is_ok_and(|actor| actor.validate(source, "wasm").is_ok());
        assert_eq!(admitted, law["accepted"].as_bool().unwrap(), "raw actor length {}", law["token"]);
    }
}
