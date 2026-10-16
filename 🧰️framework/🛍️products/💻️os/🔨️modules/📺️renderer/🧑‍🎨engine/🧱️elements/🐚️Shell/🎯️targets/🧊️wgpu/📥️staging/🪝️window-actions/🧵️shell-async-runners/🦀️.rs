//! 🪝 `shell-window-actions` staging of `shell-async-runners`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellTypes
//#region 🧵️ShellDetached
/// 🪢️ Loads a hub document's canonical checkpoint pair into the guest instance, off the shell — the one
/// seed a native, a wasm32 and a React shell open a hub document on
/// ([`semio_framework_os_kernel::os_directory::client::DirectoryClient::document_canonical_checkpoint_pair`]):
/// fetched, its digests verified, admitted only as the checkpoint the open's lease authorizes, then loaded
/// before the document's actor exists, so every mutation the guest authors names the document and its
/// baseline is the hub's (a door artifact's genesis, a checked-in document's check-in). A document bound to
/// no hub keeps the guest's own document.
pub(crate) async fn seed_hub_document(plugin: ProgramBridgeEntry, instance_id: u32, seed: Option<ShellHubDocumentSeed>) -> Result<Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>, crate::program_bridge::ProgramFault> {
    let Some(seed) = seed else { return Ok(None) };
    let pair = seed.client.document_canonical_checkpoint_pair(&seed.ctx, &seed.scope, &seed.checkpoint).await.map_err(|error| format!("canonical checkpoint pair: {error}"))?;
    plugin.load_app_document_pack(instance_id, &pair.pack_bytes, &pair.spr_bytes).await?;
    Ok(Some(pair))
}
//#endregion 🧵️ShellDetached
//#endregion ShellTypes
