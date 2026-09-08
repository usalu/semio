use super::*;
use std::error::Error;

#[semio_framework_async_macros::async_test]
async fn owned_errors_preserve_display_source_and_from() {
    let host = PluginHostError::from(std::io::Error::new(std::io::ErrorKind::Other, "disk"));
    assert_eq!(host.to_string(), "io: disk");
    assert!(host.source().is_some());
    let turn = TurnFault::from(host);
    assert_eq!(turn.to_string(), "io: disk");
    assert!(turn.source().is_some());
    let transaction = TransactionError::from(PluginHostError::Plugin("broken".into()));
    assert_eq!(transaction.to_string(), "plugin host error: plugin: broken");
    assert!(transaction.source().is_some());
    assert_eq!(PluginGraphError::Unknown { plugin_id: "missing".into() }.to_string(), "plugin `missing` is not registered");
}
