//! 📎 Delivers the registered natural reference output to the selected path.
use super::driver::{NativeMediaFormat, NativeMediaPeer, NativeSaveOutcome, CHUNK, MAX_TOTAL};
use super::sink::NativeDownloadSink;
use semio_framework::NATURAL_FILE_PORT_ID;
use semio_framework_value::FromValue;
use semio_os_kernel::{AppCommand, AppDocumentIdentity, AppFrame, pack_rt};
use semio_plugin::app::{MediaArtifactDescriptor, MediaWireFormat};
use std::path::Path;

pub(super) async fn save_reference(
    peer: &mut impl NativeMediaPeer,
    owner: &AppDocumentIdentity,
    format: &NativeMediaFormat<'_>,
    path: &Path,
    current: &impl Fn() -> bool,
    progress: &mut impl FnMut(u64),
) -> Result<NativeSaveOutcome, String> {
    if !current() || peer.current(owner.app_instance_id).await? != *owner || !current() {
        return Ok(NativeSaveOutcome::Cancelled);
    }
    let seq = peer.next_seq();
    let answer = peer.request(AppCommand::MediaOut {
        seq, port: NATURAL_FILE_PORT_ID.into(), request: Vec::new(),
    }, seq).await?;
    let AppFrame::Media { in_reply_to, port, descriptor, data } = answer else {
        return Err("natural Save received an invalid reference result".into());
    };
    if in_reply_to != seq || port != NATURAL_FILE_PORT_ID || data.is_empty() || data.len() > MAX_TOTAL
        || !matches_descriptor(&descriptor, format)
    {
        return Err("natural Save reference output does not match the declared binary format".into());
    }
    if !current() { return Ok(NativeSaveOutcome::Cancelled); }
    let mut sink = NativeDownloadSink::create(path, data.len() as u64).map_err(|error| error.to_string())?;
    let mut written = 0;
    for page in data.chunks(CHUNK) {
        if !current() { return Ok(NativeSaveOutcome::Cancelled); }
        sink.write_page(page).map_err(|error| error.to_string())?;
        written += page.len() as u64;
        progress(written);
        semio_framework_async::yield_once().await;
    }
    let ready = sink.prepare().map_err(|error| error.to_string())?;
    if !current() || peer.current(owner.app_instance_id).await? != *owner || !current() {
        return Ok(NativeSaveOutcome::Cancelled);
    }
    ready.commit().map_err(|error| error.to_string())?;
    Ok(NativeSaveOutcome::Delivered)
}

fn matches_descriptor(bytes: &[u8], format: &NativeMediaFormat<'_>) -> bool {
    if bytes.is_empty() || bytes.len() > super::driver::MAX_DESCRIPTOR { return false; }
    let Ok(value) = pack_rt::decode_wire_value(bytes) else { return false; };
    let Ok(descriptor) = MediaArtifactDescriptor::from_value(value) else { return false; };
    descriptor.port_id.as_deref() == Some(NATURAL_FILE_PORT_ID)
        && descriptor.kind_id.as_deref() == Some(format.kind)
        && descriptor.media_type.is_none_or(|value| value == format.media_type)
        && matches!(descriptor.wire, MediaWireFormat::Binary { format_kind } if format_kind == format.kind)
}
