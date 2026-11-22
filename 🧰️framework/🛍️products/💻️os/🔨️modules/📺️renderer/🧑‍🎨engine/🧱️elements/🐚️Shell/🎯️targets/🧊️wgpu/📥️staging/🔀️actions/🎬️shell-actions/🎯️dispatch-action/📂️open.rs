//! 🚪 Publishes the fresh imported program through the retained document actor.
use super::*;
use semio_renderer_wgpu_shell_doc_open::NativeOpenedFile;

impl<'ctx> ActionsCtx<'ctx> {
    pub(super) fn start_native_open(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let source = self.opening.dropped_sources.pop_front();
        let current = self.docs.action_session(action, &self.plugins).ok_or("Open File has no owning application")?;
        if self.docs.session.as_ref().is_none_or(|session| session.instance_id != current.instance_id || session.plugin_id != current.plugin_id) {
            return Err("Open File requires the current local editor".into());
        }
        if action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str)
            .is_some_and(|window_id| !current.view_state.window_instances.iter().any(|window| window.id == window_id))
        { return Err("Open File's window is no longer mounted".into()); }
        let program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == current.plugin_id)
            .cloned().ok_or("Open File's editor is unavailable")?;
        let declaration = program.declared_native_media_save(&current.app.id, semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID)
            .ok_or("Open File has no matching native binary codec")?;
        let folder = semio_renderer_wgpu_shell_doc_open::native_local_documents().ok_or("Open File has no local data directory")?;
        let label = if self.prefs.locale_id == "de" { "Datei öffnen" } else { "Open File" }.to_owned();
        if source.as_deref().is_some_and(|path| !path.extension().and_then(|value| value.to_str()).zip(declaration.extension()).is_some_and(|(dropped, declared)| dropped.eq_ignore_ascii_case(declared))) {
            return Err("Open File has no matching native binary codec".into());
        }
        if !self.opening.start_native_file_open(program, current.app.id.clone(), current.instance_id, declaration, folder, label, source) {
            return Err("document.open-busy".into());
        }
        Ok(())
    }

    pub async fn finish_native_open(&mut self, app_id: &str, imported: NativeOpenedFile) -> Result<(), String> {
        let previous = self.docs.session.clone().filter(|session| session.instance_id == imported.source_instance).ok_or("Open File's source editor closed")?;
        if !self.plugins.plugins.iter().any(|entry| entry.plugin_id == imported.program.plugin_id
            && entry.package_id == imported.program.package_id && entry.component_sha256 == imported.program.component_sha256)
        { return Err("Open File's editor package changed".into()); }
        let app = if previous.plugin_id == imported.program.plugin_id && previous.app.id == app_id { previous.app.clone() }
            else { imported.program.manifest.apps.iter().find(|app| app.id == app_id).cloned().ok_or("natural-file.kept-surface-unavailable")? };
        let previous_program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == previous.plugin_id).cloned().unwrap_or_else(|| imported.program.clone());
        let previous_folder = self.sync.sync_channel.as_ref().filter(|owner| owner.instance_id == previous.instance_id && owner.plugin_id == previous.plugin_id)
            .and_then(|owner| self.sync.sync_backbone_uri.as_deref().and_then(|uri| uri.strip_prefix("folder://").map(|path| (owner.document_id.clone(), uri.to_owned(), std::path::PathBuf::from(path)))));
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal().await?;
        let previous_window = self.windows.active_window_id.clone();
        self.windows.install_app_session(&imported.program.plugin_id, app.clone(), imported.instance, &self.prefs, &mut self.directory, &mut self.docs);
        let session = self.docs.session.clone().expect("installed native file session");
        let folder = imported.folder.clone();
        let prepared = ShellPreparedDocumentOpen {
            document_id: imported.document_id.clone(), schema: session.app.io.artifact_schema.clone(),
            bindings: vec![PersistenceBinding::Folder { path: folder.clone() }],
            backbone_uri: Some(format!("folder://{}", folder.display())),
            seed: None, plugin: imported.program.clone(), session,
        };
        if let Err(error) = self.finish_document_open(prepared).await {
            self.docs.session = Some(previous.clone());
            self.windows.active_window_id = previous_window;
            self.settle.owe_refresh(UiDirtyScope::Full);
            if let Some((document_id, uri, path)) = previous_folder {
                let restored = ShellPreparedDocumentOpen {
                    document_id, schema: previous.app.io.artifact_schema.clone(),
                    bindings: vec![PersistenceBinding::Folder { path }], backbone_uri: Some(uri),
                    seed: None, plugin: previous_program.clone(), session: previous,
                };
                if let Err(restore) = self.finish_document_open(restored).await {
                    return Err(format!("{error} ({restore})"));
                }
            }
            return Err(error);
        }
        use semio_os_config::opening_config::mutations::{LocalFolderBinding, LocalFolderRef};
        self.history.ensure_local_folder_bindings();
        self.history.remember_local_folder(LocalFolderBinding {
            document_id: imported.document_id.clone(), plugin_id: imported.program.plugin_id.clone(),
            app_id: app.id.clone(), folder: LocalFolderRef::Path { path: folder.display().to_string() },
        });
        self.history.record_local_document(&imported.document_id, &app.io.artifact_schema, &imported.name, &folder.display().to_string(), &imported.program.plugin_id, &app.id, semio_renderer_wgpu_shell_async::chrome_now_ms() as u64);
        previous_program.destroy_app(previous.instance_id);
        imported.publish();
        self.settle.owe_refresh(UiDirtyScope::Full);
        self.settle.owe_settle();
        Ok(())
    }
}
