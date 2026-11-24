//! 🧪️ Process composition registration and dispatch laws, checked against the shared fixture with a `serde_json` oracle.
//!
//! @see 🧪️tests/🧫️fixtures/registration.json — the shared fixture; 🧪️tests/🧫️fixtures/registration.schema.json — its schema.

use serde_json::Value;

fn retire(mut program: semio_plugin::Plugin<super::ProcessApps>) {
    use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
    use semio_framework_value::RetainedCloneGrant;
    let grant = RetainedCloneGrant { maximum_items: 32, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let stock = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), u64::MAX, 1 << 26);
    let mut decoder = OriginalJobDecodeOwner::empty();
    for _ in 0..65536 {
        if program.original_definitions_terminal_is_empty() {
            break;
        }
        caller.turn_with_decoder(grant, &mut decoder, |cx| cx.with_original_retirement(|cx| program.close_original_definitions(cx))).unwrap();
    }
    assert!(program.original_definitions_terminal_is_empty());
    while !decoder.terminal_is_empty() {
        caller.turn(grant, |cx| {
            cx.with_original_retirement(|cx| {
                decoder.retirement_demands(32768)?;
                decoder.close_on_source(cx)?;
                Ok(())
            })
        })
        .unwrap();
    }
    caller.retire(stock);
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/registration.json")).unwrap()
}

#[test]
fn the_registered_manifest_matches_the_shared_fixture() {
    let fixture = fixture();
    let program = super::plugin().unwrap();
    let manifest = serde_json::to_value(&program.manifest).unwrap();
    for key in ["pluginId", "label", "version", "capabilities"] {
        assert_eq!(manifest[key], fixture[key], "{key}");
    }
    assert_eq!(manifest["examples"].as_array().unwrap().len() as u64, fixture["exampleCount"].as_u64().unwrap());
    let registered: Vec<(&str, &str)> = manifest["apps"].as_array().unwrap().iter().map(|app| (app["id"].as_str().unwrap(), app["role"].as_str().unwrap())).collect();
    let expected: Vec<(&str, &str)> = fixture["apps"].as_array().unwrap().iter().map(|app| (app["id"].as_str().unwrap(), app["role"].as_str().unwrap())).collect();
    assert_eq!(registered, expected);
    retire(program);
}

#[test]
fn dispatch_resolves_every_registered_app_and_refuses_unknown_ones() {
    let fixture = fixture();
    let program = super::plugin().unwrap();
    for app in fixture["apps"].as_array().unwrap() {
        assert_eq!(program.has_original_app_factory(app["id"].as_str().unwrap()), app["originalFactory"].as_bool().unwrap(), "{app}");
    }
    assert!(!program.has_original_app_factory(fixture["unknownApp"].as_str().unwrap()));
    retire(program);
}
