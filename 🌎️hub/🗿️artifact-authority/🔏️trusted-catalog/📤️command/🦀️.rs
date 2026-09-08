//! 📡️ One-shot private publication transport, dispatched before any Hub service is opened.

use super::HubError;
use std::ffi::OsString;

fn selected(arguments: &[OsString]) -> Result<bool, HubError> {
    if arguments.is_empty() {
        return Ok(false);
    }
    if arguments == [OsString::from("trusted-catalog"), OsString::from("publish")] {
        return Ok(true);
    }
    Err(HubError::UnsafeAuthConfiguration("unknown Hub command".into()))
}

#[cfg(any(test, feature = "native-artifact-execution"))]
fn read_command(input: impl std::io::Read) -> Result<Vec<u8>, HubError> {
    use std::io::Read;
    let mut bytes = Vec::with_capacity(4097);
    input.take(4097).read_to_end(&mut bytes)?;
    if bytes.is_empty() || bytes.len() > 4096 {
        return Err(HubError::UnsafeAuthConfiguration("publication stdin must contain one bounded command".into()));
    }
    Ok(bytes)
}

#[cfg(feature = "native-artifact-execution")]
async fn publish() -> Result<(), HubError> {
    use super::{AuthorityError, AuthorityLimits, AuthorityOperationControl, NativeCodecProviderSetV1, OperationContext, StartupCatalogControl};
    use semio_hub::artifact_authority::trusted_catalog::{TrustedCatalogPublicationOutcome, TrustedCatalogPublisher};
    use std::io::Write;
    use std::time::Duration;
    let data = std::env::var_os("OS_HUB_DATA").map(std::path::PathBuf::from).filter(|path| path.is_absolute()).ok_or_else(|| HubError::UnsafeAuthConfiguration("publication requires an explicit absolute server-owned OS_HUB_DATA".into()))?;
    let (send, receive) = tokio::sync::oneshot::channel();
    std::thread::Builder::new().name("trusted-publication-input".into()).spawn(move || {
        let _ = send.send(read_command(std::io::stdin().lock()));
    })?;
    let mut bytes = tokio::time::timeout(Duration::from_millis(5000), receive)
        .await
        .map_err(|_| HubError::UnsafeAuthConfiguration("publication command input deadline exceeded".into()))?
        .map_err(|_| HubError::UnsafeAuthConfiguration("publication command reader failed".into()))??;
    let control = StartupCatalogControl;
    let context = OperationContext::new(control.now_ms().saturating_add(30000), AuthorityLimits::maximum(), &control);
    let providers = NativeCodecProviderSetV1::linked();
    let outcome = TrustedCatalogPublisher::publish_current(&data, &bytes, &providers, &context).await;
    bytes.fill(0);
    let (mut receipt, durable) = match outcome? {
        TrustedCatalogPublicationOutcome::Durable(receipt) => (receipt, true),
        TrustedCatalogPublicationOutcome::Unconfirmed(receipt) => (receipt, false),
    };
    if receipt.is_empty() || receipt.len() >= 4096 {
        return Err(AuthorityError::Catalog("publication receipt exceeds its transport bound; reconcile current".into()).into());
    }
    receipt.push(b'\n');
    let mut output = std::io::stdout().lock();
    output.write_all(&receipt)?;
    output.flush()?;
    if !durable {
        return Err(AuthorityError::Catalog("publication replacement is visible but synchronization is unconfirmed; do not activate".into()).into());
    }
    Ok(())
}

#[cfg(not(feature = "native-artifact-execution"))]
async fn publish() -> Result<(), HubError> {
    Err(HubError::UnsafeAuthConfiguration("publication requires the compiled native-artifact-execution provider".into()))
}

/// 🚪️ Handles only the exact publication verb and leaves ordinary no-argument startup untouched.
pub(super) async fn dispatch(arguments: &[OsString]) -> Result<bool, HubError> {
    if !selected(arguments)? {
        return Ok(false);
    }
    publish().await?;
    Ok(true)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
