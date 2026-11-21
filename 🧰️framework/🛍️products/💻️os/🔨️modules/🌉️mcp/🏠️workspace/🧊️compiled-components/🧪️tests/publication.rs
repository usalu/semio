//! 🔐️ Only the live shared runtime may publish a prepared component for the original native cache authority.

use super::*;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};

#[path = "cold-image.rs"]
mod cold_image;

fn prepared_component() -> Vec<u8> {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🧫️fixtures/prepared.json")).unwrap();
    let hex = fixture["coreHex"].as_str().unwrap();
    let mut length = hex.len() / 2;
    let mut component = b"\0asm\x0d\0\x01\0\x01".to_vec();
    while length >= 128 {
        component.push((length & 127) as u8 | 128);
        length >>= 7;
    }
    component.push(length as u8);
    component.extend((0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()));
    component
}

#[test]
fn prepared_publication_requires_the_live_shared_runtime() {
    let law: Value = serde_json::from_str(include_str!("🧫️fixtures/publication.json")).unwrap();
    let shared = shared_plugin_runtime().unwrap();
    let unrelated = GuestRuntimes::Owned(OwnedRuntime::new(semio_framework_job::default_now_us));
    let bytes = prepared_component();
    let hash = scoped_component_hash(&bytes, &ActivationScope::detached()).unwrap();
    assert!(cached_component(hash, &ActivationScope::detached()).unwrap().is_none());

    for row in law["steps"].as_array().unwrap() {
        let runtime = match row["runtime"].as_str().unwrap() {
            "shared" => shared.as_ref(),
            "unrelated" => &unrelated,
            value => panic!("unknown fixture runtime {value}"),
        };
        let cancel = row["cancel"].as_str().unwrap();
        let token = semio_framework_async::CancelToken::root_now();
        let stop = token.clone();
        let preparation_finished = Arc::new(AtomicBool::new(false));
        let reported = Arc::clone(&preparation_finished);
        let cancel_before_publication = cancel == "before-publication";
        let scope = ActivationScope::new(token, move |phase, progress| {
            if phase == ActivationPhase::CompilingComponent && progress == 1.0 {
                reported.store(true, Ordering::Release);
                if cancel_before_publication {
                    stop.cancel_now();
                }
            }
        });
        if cancel == "at-cache-check" {
            assert!(cached_component(hash, &ActivationScope::detached()).unwrap().is_some());
            scope.cancel().cancel_now();
            let cache_error = cached_component(hash, &scope).unwrap_err();
            assert_eq!(cache_error.code, GatewayErrorCode::Cancelled);
        }
        let result = scoped_compiled_component(runtime, "prepared-authority-fixture", ComponentBytes::Held(&bytes), &scope);
        let cached = cached_component(hash, &ActivationScope::detached()).unwrap();
        assert_eq!(cached.is_some(), row["shared"].as_bool().unwrap());
        let disposition = match result {
            Ok(handle) => {
                assert_eq!(handle.package_hash, hash);
                if cached.as_ref().is_some_and(|owner| owner.same_origin(&handle)) { "owner" } else { "local" }
            }
            Err(error) => {
                assert_eq!(error.code, GatewayErrorCode::Cancelled);
                "cancelled"
            }
        };
        assert_eq!(disposition, row["result"].as_str().unwrap());
        assert_eq!(preparation_finished.load(Ordering::Acquire), row["prepared"].as_bool().unwrap());
    }
}
