//! 🌐 Opens one picked or dropped natural file in the browser: routes it to its owner editor, imports it into a fresh instance and publishes that instance through the retained document actor, React's `openNaturalFile` on the wgpu shell.
use super::*;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use semio_renderer_wgpu_shell_doc_open::{admit_local_catalog_document, retire_local_catalog_document, browser_catalog, browser_natural_format, local_document_folder_uri, picked_file_refusal_code, picked_file_refusal_text, release_browser_picked_file, BrowserOpenRouting, BrowserOpenedFile, ShellDocumentOpenPhase, ShellDocumentOpening};

impl<'ctx> ActionsCtx<'ctx> {
    pub(super) async fn start_browser_open(&mut self, action: &ActionDescriptor) -> Result<(), String> {
        let source = self.opening.dropped_files.pop_front();
        let current = self.docs.action_session(action, &self.plugins).ok_or("Open File has no owning application")?;
        if self.docs.session.as_ref().is_none_or(|session| session.instance_id != current.instance_id || session.plugin_id != current.plugin_id) {
            return Err("Open File requires the current local editor".into());
        }
        let label = if self.prefs.locale_id == "de" { "Datei öffnen" } else { "Open File" }.to_owned();
                let catalog = browser_catalog(&self.plugins.plugins).await;
        if !self.opening.start_browser_file_open(label, source.clone(), catalog, current.plugin_id.clone(), current.app.id.clone()) {
            if let Some(file) = source { self.opening.dropped_files.push_front(file); }
            return Err("document.open-busy".into());
        }
        Ok(())
    }

    fn refuse_browser_open(&mut self, reason: &'static str, name: &str) {
        let text = picked_file_refusal_text(reason, name, Some(self.prefs.locale_id.as_str()));
        self.chrome.show_transient_notice(text, semio_framework::Severity::Warning, Some(picked_file_refusal_code(reason).as_str()));
    }

    fn fail_browser_open(&mut self, opening: ShellDocumentOpening, error: String) {
        self.opening.document_opening = Some(opening);
        self.opening.fail_document_opening(error, &mut self.chrome, &self.prefs);
    }

    pub async fn continue_browser_routed(&mut self, mut opening: ShellDocumentOpening, routing: BrowserOpenRouting) {
        let (file, artifact_ref) = match routing {
            BrowserOpenRouting::Cancelled(_) => return,
            BrowserOpenRouting::Refused { name, reason, .. } => return self.refuse_browser_open(reason, &name),
            BrowserOpenRouting::Routed { file, artifact_ref } => (file, artifact_ref),
        };
        if opening.cancel_requested {
            release_browser_picked_file(file.token).await;
            opening.phase = ShellDocumentOpenPhase::Cancelled;
            self.opening.document_opening = Some(opening);
            return;
        }
        let Some(source_instance) = self.docs.session.as_ref().map(|session| session.instance_id) else {
            release_browser_picked_file(file.token).await;
            return self.fail_browser_open(opening, "Open File's source editor closed".into());
        };
        let resolved = match semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&artifact_ref) {
            Ok(dialect) => self.plugins.resolve_activation_owner_app(&dialect, semio_framework::manifest::AppRole::Editor).await,
            Err(_) => None,
        };
        let owner = resolved.and_then(|(plugin_id, app)| self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned().map(|program| (program, app)));
        let Some((program, app)) = owner else {
            release_browser_picked_file(file.token).await;
            return self.refuse_browser_open("owner-unavailable", &file.name);
        };
        let Some((format_kind, ..)) = browser_natural_format(&app) else {
            release_browser_picked_file(file.token).await;
            return self.fail_browser_open(opening, "natural-file.importer-unavailable".into());
        };
        self.opening.start_browser_file_import(opening, program, app, format_kind, file, source_instance);
    }

    pub async fn finish_browser_open(&mut self, imported: BrowserOpenedFile) -> Result<(), String> {
        let previous = self.docs.session.clone().filter(|session| session.instance_id == imported.source_instance).ok_or("Open File's source editor closed")?;
        if !self.plugins.plugins.iter().any(|entry| entry.plugin_id == imported.program.plugin_id
            && entry.package_id == imported.program.package_id && entry.component_sha256 == imported.program.component_sha256)
        { return Err("Open File's editor package changed".into()); }
        let (folder, admit) = match imported.kept_folder.clone() {
            Some(folder) => (folder, false),
            None => (local_document_folder_uri(self.directory.identity_env.as_ref().and_then(|env| env.data_dir.as_deref()), &imported.document_id).await?, true),
        };
        self.publish_browser_document(imported, folder, admit).await
    }

    /// 📂 Publishes an instance holding a kept document as the session: attach its folder lane, list it in the local catalog when `admit` (a fresh import; a reopened one is listed already), and roll everything back on any refusal, as React's keep stage does.
    async fn publish_browser_document(&mut self, imported: BrowserOpenedFile, folder: String, admit: bool) -> Result<(), String> {
        let previous = self.docs.session.clone().filter(|session| session.instance_id == imported.source_instance).ok_or("Open File's source editor closed")?;
        let previous_uri = self.sync.sync_backbone_uri.clone();
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal().await?;
        let previous_window = self.windows.active_window_id.clone();
        self.windows.install_app_session(&imported.program.plugin_id, imported.app.clone(), imported.instance, &self.prefs, &mut self.directory, &mut self.docs);
        let session = self.docs.session.clone().expect("installed browser file session");
        let schema = session.app.io.artifact_schema.clone();
        let prepared = ShellPreparedDocumentOpen {
            document_id: imported.document_id.clone(), schema: schema.clone(),
            bindings: Vec::new(), backbone_uri: None, seed: None, plugin: imported.program.clone(), session,
        };
        let kept = match self.finish_document_open(prepared).await {
            Ok(()) => self.keep_browser_document(&imported, &schema, &folder, admit).await,
            Err(error) => Err(error),
        };
        if let Err(error) = kept {
            let _ = self.detach_sync_backbone_internal().await;
            if admit {
                self.history.forget_local_folder(&imported.document_id);
            }
            if admit {
                retire_local_catalog_document(&imported.document_id).await;
            }
            self.docs.session = Some(previous);
            self.windows.active_window_id = previous_window;
            if let Some(uri) = previous_uri {
                let _ = self.attach_sync_backbone(uri).await;
            }
            self.settle.owe_refresh(UiDirtyScope::Full);
            return Err(error);
        }
        if let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == previous.plugin_id) {
            program.destroy_app(previous.instance_id);
        }
        imported.publish();
        self.settle.owe_refresh(UiDirtyScope::Full);
        self.settle.owe_settle();
        Ok(())
    }

    /// 💾 React's keep stage on the opened instance: the folder lane holds the document's archive, then the device's local catalog lists it; a refusal at either step is the caller's rollback.
    async fn keep_browser_document(&mut self, imported: &BrowserOpenedFile, schema: &str, folder: &str, admit: bool) -> Result<(), String> {
        self.attach_sync_backbone(folder.to_owned()).await?;
        if self.sync.sync_backbone_uri.as_deref() != Some(folder) { return Err("natural-file.archive-not-persisted".into()); }
        if !admit { return Ok(()); }
        let target = folder.strip_prefix("folder://").unwrap_or(folder);
        let admitted_at_ms = semio_renderer_wgpu_shell_async::chrome_now_ms() as u64;
        if !admit_local_catalog_document(&imported.document_id, schema, &imported.name, target, &imported.program.plugin_id, &imported.app.id, admitted_at_ms).await {
            return Err("natural-file.catalog-not-persisted".into());
        }
        Ok(())
    }
}
