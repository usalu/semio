//! ⚙️ Config commands: config load and read and the window config load and read of one plugin exchange. See [`super`].

use super::*;

/// 🔧️ Loads the config pack and spr.
pub(super) async fn arm_load_config<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::LoadConfig { seq, pack, spr } = command else { unreachable!("arm_load_config is dispatched only for its own command") };
    let applied = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.load_config_pack(&store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }))
    });
    match applied.await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📖️ Reads the config pack, spr and ops.
pub(super) async fn arm_read_config<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadConfig { seq } = command else { unreachable!("arm_read_config is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.config_pack())
    });
    match read.await {
        Ok(files) => frames.push(protocol::AppFrame::Config { in_reply_to: seq, pack: files.pack, spr: files.spr, ops: files.ops }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🪟️ Loads one window config pack entry.
pub(super) async fn arm_load_window_config<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::LoadWindowConfig { seq, entry } = command else { unreachable!("arm_load_window_config is dispatched only for its own command") };
    match store::os_store::serialization::decode_document_pack_bytes(&entry.envelope_pack).await {
        Ok((pack, spr)) => {
            let loaded = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.load_window_config_pack(WindowConfigPack {
                    window_id: entry.window_id,
                    window_kind_id: entry.window_kind_id,
                    files: store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() },
                }))
            });
            match loaded.await {
                Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            }
        }
        Err(error) => push_app_fault(&mut frames, Some(seq), error.into_fault()).await,
    }
    Ok(())
}

/// 🗂️ Reads every window config pack entry.
pub(super) async fn arm_read_window_configs<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadWindowConfigs { seq } = command else { unreachable!("arm_read_window_configs is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.window_config_packs())
    });
    match read.await {
        Ok(packs) => {
            let mut entries = Vec::with_capacity(packs.len());
            for pack in packs {
                entries.push(protocol::WindowConfigPackEntry { window_id: pack.window_id, window_kind_id: pack.window_kind_id, envelope_pack: store::os_store::serialization::encode_document_pack_bytes(&pack.files.pack, &pack.files.spr).await });
            }
            frames.push(protocol::AppFrame::WindowConfigs { in_reply_to: seq, entries });
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}
