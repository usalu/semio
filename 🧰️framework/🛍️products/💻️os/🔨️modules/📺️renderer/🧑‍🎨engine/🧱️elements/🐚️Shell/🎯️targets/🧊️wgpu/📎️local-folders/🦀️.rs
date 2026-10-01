//! 📎️ The device's remembered folder bindings on the wgpu shell (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, W2-C
//! follow-up 5) — React's `🏛️ShellHost/📎️local-folders`.
//!
//! `os.config.local-folders` is an event-sourced config facet persisted local-only, in the shell's own preferences
//! (`semio.os.config` → `preferences["os.config.local-folders"]`, React's `OsShellConfig.setPreference`): never shared, never
//! in a URL. A folder attach of a program's document records `attachLocalFolder`, keyed by the document's own identity
//! (the id its store stamps on every envelope); a detach records `detachLocalFolder`. When a program boots holding a
//! document this device once attached, the native shell — which owns a folder transport and needs no gesture to open a
//! folder — reattaches it directly, once per document, and restores its archive like a fresh load; when that reattach
//! fails it offers the accessible "Reconnect folder" / "Forget folder" band React's browser shell always offers (the
//! browser needs the person's gesture). The browser wgpu build serves no folder transport, so it neither attaches nor
//! remembers nor offers a folder.
//!
//! The event log, the reconnect offer, the folder name and the band's copy are pinned for both shells by
//! `🧑‍🎨engine/🧫️fixtures/📎️local-folder-bindings/🔣️.json`.

use super::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️EventLog
/// 🧾️ The one version of the persisted event log `{version, events}`.
const LOCAL_FOLDERS_EVENT_LOG_VERSION: u64 = 1;

/// 📏️ The payload schemas' bounds (`📎️attach-local-folder`, `✂️detach-local-folder`).
const LOCAL_FOLDER_DOCUMENT_ID_MAX: usize = 512;
const LOCAL_FOLDER_PROGRAM_ID_MAX: usize = 256;
const LOCAL_FOLDER_PATH_MAX: usize = 4096;

fn bounded_text(value: &Value, key: &str, maximum: usize) -> Option<String> {
    value.get(key)?.as_str().filter(|text| !text.is_empty() && text.chars().count() <= maximum).map(str::to_string)
}

fn has_exactly(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|object| object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key)))
}

/// 🔎️ One persisted event read against the payload schemas; `None` when it is not one — React's
/// `localFoldersMutationOfValueV1`, field for field (exact keys, bounded non-empty texts, a `path` folder).
pub(crate) fn local_folders_mutation_of_value(value: &Value) -> Option<LocalFoldersConfigMutation> {
    match value.get("mutation")?.as_str()? {
        "detachLocalFolder" if has_exactly(value, &["mutation", "documentId"]) => Some(detach_local_folder(&bounded_text(value, "documentId", LOCAL_FOLDER_DOCUMENT_ID_MAX)?)),
        "attachLocalFolder" if has_exactly(value, &["mutation", "documentId", "pluginId", "appId", "folder"]) => {
            let folder = value.get("folder")?;
            if !has_exactly(folder, &["kind", "path"]) || folder.get("kind")?.as_str()? != "path" {
                return None;
            }
            Some(attach_local_folder(LocalFolderBinding {
                document_id: bounded_text(value, "documentId", LOCAL_FOLDER_DOCUMENT_ID_MAX)?,
                plugin_id: bounded_text(value, "pluginId", LOCAL_FOLDER_PROGRAM_ID_MAX)?,
                app_id: bounded_text(value, "appId", LOCAL_FOLDER_PROGRAM_ID_MAX)?,
                folder: LocalFolderRef::Path { path: bounded_text(folder, "path", LOCAL_FOLDER_PATH_MAX)? },
            }))
        }
        _ => None,
    }
}

/// 📖️ The facet's committed events in a stored log; a log that does not read whole as one is empty, so nothing is
/// reattached from it — React's `readLocalFolderEventsV1`.
pub(crate) fn local_folder_events_of_log(raw: Option<&str>) -> Vec<LocalFoldersConfigMutation> {
    let Some(value) = raw.and_then(|raw| serde_json::from_str::<Value>(raw).ok()) else { return Vec::new() };
    if value.get("version").and_then(Value::as_u64) != Some(LOCAL_FOLDERS_EVENT_LOG_VERSION) {
        return Vec::new();
    }
    value.get("events").and_then(Value::as_array).and_then(|events| events.iter().map(local_folders_mutation_of_value).collect::<Option<Vec<_>>>()).unwrap_or_default()
}

/// 🧮️ The bindings committed events fold to — React's `replayLocalFolderEventsV1`.
pub(crate) fn replay_local_folder_events(events: &[LocalFoldersConfigMutation]) -> LocalFolderBindings {
    let mut bindings = LocalFolderBindings::default();
    for event in events {
        let _ = apply_local_folders_config_mutation(&mut bindings, event);
    }
    bindings
}

/// ✍️ The stored log after appending `mutation`, or `None` when the mutation changes nothing (it is then not recorded) —
/// React's `commitLocalFoldersConfigMutationV1`.
pub(crate) fn local_folders_log_after(raw: Option<&str>, mutation: &LocalFoldersConfigMutation) -> Option<String> {
    let mut events = local_folder_events_of_log(raw);
    let before = replay_local_folder_events(&events);
    let mut after = before.clone();
    apply_local_folders_config_mutation(&mut after, mutation).ok()?;
    if after == before {
        return None;
    }
    events.push(mutation.clone());
    serde_json::to_string(&serde_json::json!({ "version": LOCAL_FOLDERS_EVENT_LOG_VERSION, "events": events })).ok()
}
//#endregion 🔖️EventLog

//#region 🔖️Store
#[cfg(test)]
thread_local! {
    /// 🧪️ The preference slot a law's shells share, outliving each of them like the real store outlives a restart.
    pub(crate) static TEST_LOCAL_FOLDERS_LOG: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// 📖️ The stored event log.
fn local_folders_log_read() -> Option<String> {
    #[cfg(test)]
    return TEST_LOCAL_FOLDERS_LOG.with(|slot| slot.borrow().clone());
    #[cfg(not(test))]
    prefs_get(LOCAL_FOLDERS_CONFIG_SCHEMA)
}

/// ✍️ Replaces the stored event log.
fn local_folders_log_write(raw: &str) {
    #[cfg(test)]
    TEST_LOCAL_FOLDERS_LOG.with(|slot| *slot.borrow_mut() = Some(raw.to_string()));
    #[cfg(not(test))]
    prefs_set(LOCAL_FOLDERS_CONFIG_SCHEMA, raw);
}
//#endregion 🔖️Store

//#region 🔖️Reconnect
/// 🪪️ The program and the document identity a booted session holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalFolderIdentity {
    pub document_id: String,
    pub plugin_id: String,
    pub app_id: String,
}

/// 📎️ The binding to bring back: the one remembered for this document in this program, unless the document is attached
/// already — React's `localFolderReconnectOfferV1`.
pub(crate) fn local_folder_reconnect_offer(bindings: &LocalFolderBindings, identity: Option<&LocalFolderIdentity>, attached_document_id: Option<&str>) -> Option<LocalFolderBinding> {
    let identity = identity.filter(|identity| attached_document_id != Some(identity.document_id.as_str()))?;
    bindings.bindings.iter().find(|binding| binding.document_id == identity.document_id && binding.plugin_id == identity.plugin_id && binding.app_id == identity.app_id).cloned()
}

/// 🏷️ The name a person knows a folder by: its last path segment — React's `localFolderNameV1`.
pub(crate) fn local_folder_name(path: &str) -> &str {
    path.split(['/', '\\']).filter(|segment| !segment.is_empty()).last().unwrap_or(path)
}

/// 📁️ A binding's folder path.
pub(crate) fn local_folder_path(binding: &LocalFolderBinding) -> &str {
    match &binding.folder {
        LocalFolderRef::Path { path } => path,
    }
}

/// 🗣️ The band's copy — React's `ui.sync.reconnect.{label,message,attach,forget}` at the normal terminology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalFolderText {
    Label,
    Message,
    Attach,
    Forget,
    Unidentified,
}

/// 🗣️ One line of the band (or of a reconnect refusal) in `locale`; `folder` fills the message.
pub(crate) fn local_folder_text(text: LocalFolderText, folder: &str, locale: Locale) -> String {
    match (text, locale) {
        (LocalFolderText::Label, Locale::En) => "Folder of this document".into(),
        (LocalFolderText::Label, Locale::De) => "Ordner dieses Dokuments".into(),
        (LocalFolderText::Message, Locale::En) => format!("This document was attached to the folder “{folder}”."),
        (LocalFolderText::Message, Locale::De) => format!("Dieses Dokument war mit dem Ordner „{folder}“ verbunden."),
        (LocalFolderText::Attach, Locale::En) => "Reconnect folder".into(),
        (LocalFolderText::Attach, Locale::De) => "Ordner wieder verbinden".into(),
        (LocalFolderText::Forget, Locale::En) => "Forget folder".into(),
        (LocalFolderText::Forget, Locale::De) => "Ordner vergessen".into(),
        (LocalFolderText::Unidentified, Locale::En) => "This program has no document to attach".into(),
        (LocalFolderText::Unidentified, Locale::De) => "Dieses Programm hat kein Dokument zum Verbinden".into(),
    }
}

/// 🆔️ The band's own nodes — React's ids (`#s-folder-reconnect-message`, `#s-folder-reconnect`, `#s-folder-forget`).
pub(crate) const FOLDER_RECONNECT_STATUS_ID: &str = "s-folder-reconnect-message";
pub(crate) const FOLDER_RECONNECT_CONTROL_ID: &str = "s-folder-reconnect";
pub(crate) const FOLDER_FORGET_CONTROL_ID: &str = "s-folder-forget";

/// 🎬️ The band's two verbs, dispatched on the shell's own sync controller.
pub(crate) const FOLDER_RECONNECT_ACTION: &str = "reconnectFolder";
pub(crate) const FOLDER_FORGET_ACTION: &str = "forgetFolder";

/// 🆔️ The notice code a folder attach of a program holding no document is told under.
pub(crate) const SYNC_DOCUMENT_UNIDENTIFIED_CODE: &str = "sync.attach.document-unidentified";

/// 📐️ The band's widest extent on one row; wider, it wraps compact.
const FOLDER_BAND_MAX_WIDTH: f32 = 720.0;

/// 📐️ One laid-out reconnect band: the strip, the message's lines and the two buttons.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FolderReconnectBandPlan {
    pub band: Rect,
    pub lines: Vec<time_travel::ChromeBandLine>,
    pub buttons: [(&'static str, &'static str, String, Rect); 2],
}

/// 📐️ Lays the band out bottom-centre through the time-travel band's [`time_travel::chrome_band_layout`], stacked above
/// the time-travel band when one shows (React stacks the folder offer first), else just above the footer: one row with
/// the buttons right-aligned after the message, or on a phone-width viewport the message wrapped at its words with the
/// buttons below it.
pub(crate) fn folder_reconnect_band_plan(message: String, reconnect: String, forget: String, below: Option<Rect>, width: f32, height: f32, theme: &Theme) -> FolderReconnectBandPlan {
    let floor = below.map_or(height - theme.footer_height, |rect| rect.y);
    let layout = time_travel::chrome_band_layout(&message, " ", &[reconnect.as_str(), forget.as_str()], floor, width, FOLDER_BAND_MAX_WIDTH, theme);
    let (reconnect_rect, forget_rect) = (layout.buttons[0], layout.buttons[1]);
    FolderReconnectBandPlan { band: layout.band, lines: layout.lines, buttons: [(FOLDER_RECONNECT_CONTROL_ID, FOLDER_RECONNECT_ACTION, reconnect, reconnect_rect), (FOLDER_FORGET_CONTROL_ID, FOLDER_FORGET_ACTION, forget, forget_rect)] }
}
//#endregion 🔖️Reconnect

impl ShellState {
    /// 📖️ Loads the remembered bindings once; every commit keeps the cache current.
    pub(crate) fn ensure_local_folder_bindings(&mut self) {
        if self.local_folder_bindings.is_none() {
            self.local_folder_bindings = Some(replay_local_folder_events(&local_folder_events_of_log(local_folders_log_read().as_deref())));
        }
    }

    /// 📁️ This device's remembered folder bindings, as last loaded.
    pub(crate) fn local_folder_bindings(&self) -> LocalFolderBindings {
        self.local_folder_bindings.clone().unwrap_or_default()
    }

    /// ✍️ Appends one mutation to the facet's local-only log; a mutation that changes nothing is not recorded.
    fn commit_local_folders_mutation(&mut self, mutation: &LocalFoldersConfigMutation) {
        let raw = local_folders_log_read();
        if let Some(next) = local_folders_log_after(raw.as_deref(), mutation) {
            local_folders_log_write(&next);
        }
        self.local_folder_bindings = Some(replay_local_folder_events(&local_folder_events_of_log(local_folders_log_read().as_deref())));
    }

    /// 📎️ Remembers the folder a program's document was attached to.
    pub(crate) fn remember_local_folder(&mut self, binding: LocalFolderBinding) {
        self.commit_local_folders_mutation(&attach_local_folder(binding));
    }

    /// ✂️ Forgets a document's folder.
    pub(crate) fn forget_local_folder(&mut self, document_id: &str) {
        self.commit_local_folders_mutation(&detach_local_folder(document_id));
    }

    /// 🪪️ The document identity the session's program holds, as its own store stamps it — `None` until the program
    /// answered `ReadDocumentIdentity` with a document.
    pub(crate) fn sync_program_identity(&self) -> Option<LocalFolderIdentity> {
        let session = self.session.as_ref()?;
        let document_id = self.app_document_identities.document(&(session.plugin_id.clone(), session.instance_id))?;
        Some(LocalFolderIdentity { document_id: document_id.to_string(), plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone() })
    }

    /// 📎️ The remembered folder for the session's document, while that document is not attached — never on a build
    /// without a folder transport. Asked every frame, so a device remembering no folder answers before reading the
    /// session's identity.
    pub(crate) fn folder_reconnect_offer(&self) -> Option<LocalFolderBinding> {
        let bindings = self.local_folder_bindings.as_ref().filter(|bindings| SHELL_DOCUMENT_TRANSPORTS.folder && !bindings.bindings.is_empty())?;
        let attached = self.sync_channel.as_ref().map(|channel| channel.document_id.as_str());
        local_folder_reconnect_offer(bindings, self.sync_program_identity().as_ref(), attached)
    }

    /// 📎️ The offer the band shows: the remembered folder whose direct reattach already ran and left the document
    /// unattached.
    pub(crate) fn folder_reconnect_band_offer(&self) -> Option<LocalFolderBinding> {
        self.folder_reconnect_offer().filter(|offer| self.local_folder_reattach_tried.as_deref() == Some(offer.document_id.as_str()))
    }

    /// 🔗️ Reattaches a remembered folder through the same folder attach a person makes, which re-records it and restores
    /// the folder's archive.
    pub(crate) async fn reattach_local_folder(&mut self, binding: &LocalFolderBinding) -> Result<(), String> {
        self.attach_sync_backbone(format!("folder://{}", local_folder_path(binding))).await
    }

    /// 🔁️ The folder the shell reopens by itself now: the offer for the session's document, once per document.
    pub(crate) fn direct_reattach_candidate(&self) -> Option<LocalFolderBinding> {
        self.folder_reconnect_offer().filter(|offer| self.local_folder_reattach_tried.as_deref() != Some(offer.document_id.as_str()))
    }

    /// 🔁️ The boot reattach: once per document, the remembered folder of the session's document is reopened without a
    /// gesture. A failure is told through the dispatch-fault funnel and leaves the band offering it. `true` when it ran.
    pub(crate) async fn reattach_remembered_local_folder(&mut self) -> bool {
        self.ensure_local_folder_bindings();
        let Some(offer) = self.direct_reattach_candidate() else { return false };
        self.local_folder_reattach_tried = Some(offer.document_id.clone());
        if let Err(error) = self.reattach_local_folder(&offer).await {
            self.note_dispatch_fault(&error);
        }
        true
    }

    /// 🗃️ Restores the archive a native folder holds for `document_id` into the session's program like a fresh load —
    /// React's `restoreDocumentArchiveV1`: once the program holds it, its history is re-read (rows, edits, head) and every
    /// surface refreshes. `true` when an archive was restored.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn restore_folder_archive(&mut self, path: &str, document_id: &str) -> Result<bool, String> {
        let Some(bytes) = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(path)).read_archive(document_id).await.map_err(|error| error.to_string())? else { return Ok(false) };
        let archive = protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| error.to_string())?;
        let session = self.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        plugin.load_app_document_archive(session.instance_id, &archive).await?;
        self.refresh_history_snapshot().await;
        self.refresh_ui(UiDirtyScope::Full).await?;
        Ok(true)
    }

    /// 📐️ This frame's band, localized.
    pub(crate) fn folder_reconnect_band_plan_for(&self, offer: &LocalFolderBinding, theme: &Theme) -> FolderReconnectBandPlan {
        let locale = self.active_locale();
        let below = self.history_time_travel.as_ref().map(|status| self.time_travel_band_plan_for(status, theme).band);
        let message = local_folder_text(LocalFolderText::Message, local_folder_name(local_folder_path(offer)), locale);
        folder_reconnect_band_plan(message, local_folder_text(LocalFolderText::Attach, "", locale), local_folder_text(LocalFolderText::Forget, "", locale), below, self.screen_w, self.screen_h, theme)
    }

    /// 📎️ Paints the reconnect band, one scalar, glyph run or hit per opportunity: fill, edges, each line of the message,
    /// then per button its fill, caption and hit. Each button dispatches its verb on the shell's sync controller, so a
    /// click, the keyboard ring and an assistive technology reconnect or forget the same way; none registers under a modal
    /// dialog.
    pub(super) fn render_folder_reconnect_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some(offer) = self.folder_reconnect_band_offer() else { return true };
        let plan = self.folder_reconnect_band_plan_for(&offer, theme);
        let band = plan.band;
        let hair = theme.stroke_hairline;
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let buttons_from = 5 + plan.lines.len();
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], theme.level_bg[Level::Menu.index()], theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, theme.border_normal);
            }
            scalar if scalar < buttons_from => {
                let line = &plan.lines[scalar - 5];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, theme.text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell folder reconnect band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some((control_id, action, label, rect)) = plan.buttons.get((scalar - buttons_from) / 3).cloned() else { return true };
                match (scalar - buttons_from) % 3 {
                    0 => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius),
                    1 => match chrome_text_complete_step(overlay, atlas, &label, rect.x + theme.padding_standard, baseline(rect), (rect.w - theme.padding_standard).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.error = Some("Shell folder reconnect band button text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    },
                    _ => {
                        if !self.chrome_build.dialog_open() {
                            note_chrome_control_name(control_id, Some(label.as_str()));
                            let event = ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: crate::action_args_json!({ "documentId": offer.document_id.clone() }) };
                            input.register_hit(HitTarget { rect, event: Some(event), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The band's live node — React's polite `role=status` region: named by the message it paints and described by
    /// what it is about ("Folder of this document").
    pub(crate) fn folder_reconnect_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let offer = self.folder_reconnect_band_offer()?;
        let locale = self.active_locale();
        let mut node = chrome_status_accessibility_node(node_id, FOLDER_RECONNECT_STATUS_ID, local_folder_text(LocalFolderText::Message, local_folder_name(local_folder_path(&offer)), locale));
        node.description = Some(local_folder_text(LocalFolderText::Label, "", locale));
        Some(node)
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🧪️wgpu-local-folders/🦀️.rs"]
mod tests;
