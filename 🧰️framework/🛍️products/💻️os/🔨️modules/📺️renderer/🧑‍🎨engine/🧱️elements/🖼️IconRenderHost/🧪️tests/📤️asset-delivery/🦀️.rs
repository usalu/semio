//! 🧪️ Exact-request export asset delivery and cancellation across slot reuse.
use super::*;

fn retire(token: WorldAssetRequestToken) {
    let mut authority = crate::renderer_asset_io().lock().unwrap();
    authority.cancel_request(token);
    for _ in 0..4096 {
        if !authority.retire_cancelled_step() { break; }
    }
}

#[test]
fn icon_export_asset_fault_is_delivered_once_to_its_exact_request() {
    let mut first = IconExportAssetRequest::new("/mesh/export-first.glb").unwrap();
    let mut sibling = IconExportAssetRequest::new("/mesh/export-sibling.glb").unwrap();
    let first_token = first.token.unwrap();
    let sibling_token = sibling.token.unwrap();
    reject(first_token, "first transport failed");
    assert_eq!(first.take_ready().unwrap().unwrap_err(), "first transport failed");
    assert!(first.take_ready().is_none());
    assert!(sibling.take_ready().is_none());
    retire(first_token);
    let mut replacement = IconExportAssetRequest::new("/mesh/export-replacement.glb").unwrap();
    let replacement_token = replacement.token.unwrap();
    assert_ne!(first_token, replacement_token);
    reject(first_token, "stale failure");
    assert!(replacement.take_ready().is_none());
    assert!(sibling.take_ready().is_none());
    reject(sibling_token, "sibling transport failed");
    assert_eq!(sibling.take_ready().unwrap().unwrap_err(), "sibling transport failed");
    while !replacement.close_step() {}
    retire(sibling_token);
    retire(replacement_token);
    assert!(slots().lock().unwrap().iter().all(Option::is_none));
}

#[test]
fn icon_export_asset_abandoned_request_is_cancelled_and_retired() {
    let first = IconExportAssetRequest::new("/mesh/export-abandoned.glb").unwrap();
    let token = first.token.unwrap();
    let mut sibling = IconExportAssetRequest::new("/mesh/export-still-live.glb").unwrap();
    let sibling_token = sibling.token.unwrap();
    drop(first);
    assert!(crate::renderer_asset_io().lock().unwrap().cancellation_requested(token));
    assert!(!crate::renderer_asset_io().lock().unwrap().cancellation_requested(sibling_token));
    while retire_cancelled_step() {}
    assert!(sibling.take_ready().is_none());
    while !sibling.close_step() {}
    retire(token);
    retire(sibling_token);
    assert!(slots().lock().unwrap().iter().all(Option::is_none));
}

