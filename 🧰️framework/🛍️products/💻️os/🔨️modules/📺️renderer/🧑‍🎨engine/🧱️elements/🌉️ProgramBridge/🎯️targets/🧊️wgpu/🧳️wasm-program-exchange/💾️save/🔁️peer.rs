//! 🌉 Addresses the real bounded native KernelClient and returns one correlated SPR reply.
use super::driver;
use super::super::{KernelClient, wasm_program_exchange::{app_frame_error_message, exchange, next_seq, read_app_document_identity}};
use protocol::{AppCommand, AppDocumentIdentity, AppFrame};

struct SavePeer<'a> { client: &'a KernelClient, instance: u32 }

impl driver::NativeMediaPeer for SavePeer<'_> {
    fn next_seq(&mut self) -> u64 { next_seq() }

    async fn request(&mut self, command: AppCommand, sequence: u64) -> Result<AppFrame, String> {
        let outcome = exchange(self.client, self.instance, vec![command]).await.map_err(|error| error.to_string())?;
        driver::one_save_reply(outcome.frames, sequence, app_frame_error_message)
    }

    async fn current(&mut self, instance: u32) -> Result<AppDocumentIdentity, String> {
        if instance != self.instance { return Err("native Save changed its application instance".into()); }
        read_app_document_identity(self.client, instance).await.map_err(|error| error.to_string())
    }
}

pub(crate) async fn save_media(
    client: &KernelClient, owner: &AppDocumentIdentity, declaration: &super::NativeMediaDeclaration, path: &std::path::Path,
    cancelled: impl Fn() -> bool, progress: impl FnMut(u64),
) -> Result<driver::NativeSaveOutcome, String> {
    use driver::NativeMediaPeer;
    let mut peer = SavePeer { client, instance: owner.app_instance_id };
    if cancelled() || peer.current(owner.app_instance_id).await? != *owner || cancelled() {
        return Ok(driver::NativeSaveOutcome::Cancelled);
    }
    let format = driver::NativeMediaFormat { port: &declaration.port, kind: &declaration.format_kind,
        mime: &declaration.mime, media_type: declaration.media_type, reference: declaration.reference };
    driver::save(&mut peer, owner, &format, path, std::time::Duration::from_secs(120), cancelled, progress).await
}
