//! 🪝️ `shell-window-actions` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl<'ctx> WindowActionsCtx<'ctx> {
    /// 🎠️ terra-shell-unpark — this used to synchronously drive the kernel
    /// round trip right here. Unlike the two `glue.rs` call sites, `ShellState`
    /// has no `Weak<RefCell<AppRuntime>>` to re-borrow after the `.await` (only
    /// `AppRuntime` carries `self_weak`), which is exactly why an earlier packet
    /// left this one alone rather than force a borrow that doesn't exist. It
    /// doesn't need one: nothing here mutates shell state after firing, only logs
    /// on failure, so `plugin` (`ProgramBridgeEntry` is `Clone`) and the already-
    /// owned `pack`/`spr` from the matched `Effect` are enough to detach this into
    /// a fully self-contained `spawn_app_task` future — `queue_host_effects`
    /// itself stays a plain, non-async `fn`.
    ///
    /// 💡️ Slice D — the host-owned ephemeral inference port. The program named only an
    /// intent; this shell resolves the document scope, checks the execution-target lease,
    /// and drives the whole lifecycle itself. It never writes the document.
    ///
    /// 📥️ The extension door. This used to fire a DETACHED `spawn_app_task` and throw the
    /// answer away — so the `flowEvalResolve` the answer arms never reached the shell and
    /// the chain stopped after exactly one evaluate (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END). React's `dispatchInvokeExtensionEffect`
    /// publishes the response through `applyHostEffects`; this parks the request for the
    /// flush below, which owns `&mut self` and can do the same.
    ///
    /// ⬇️ Every plugin's export door. This arm did not exist: a `DownloadMediaExport` fell
    /// into the loud drop below, so no plugin could export ANYTHING on this renderer while
    /// React's shell was saving the same export as corrupt base64 text (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).
    ///
    /// 📤️ Every plugin's IMPORT door. This arm did not exist either, and the only
    /// `requestFileOpen` reader this shell had read it out of a document MUTATION payload
    /// no producer in the repo emits — so `Import Document…` reached the guest, the guest
    /// emitted its effect, and the user never saw a picker (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
    /// Parked, never opened here: a picker ends on a user gesture and this funnel is a
    /// plain `fn`; `drain_deferred_actions` owns the `&mut self` the pick needs.
    pub fn queue_host_effects(&mut self, controller_id: &str, effects: Vec<semio_framework::kernel::Effect>) {
        for effect in effects {
            match effect {
                semio_framework::kernel::Effect::SetActiveUtility { window_id, utility_id } => {
                    self.windows.set_active_utility(&window_id, (!utility_id.is_empty()).then_some(utility_id.as_str()), &mut self.chrome, &mut self.docs);
                }
                semio_framework::kernel::Effect::Navigate { uri } => {
                    self.opening.push_uri(uri);
                }
                semio_framework::kernel::Effect::OpenDialog { dialog_id, args, .. } => {
                    if let Some(request) = self.docs.chrome_dialog_request(&dialog_id, args, &self.prefs) {
                        self.chrome.chrome_build.open_dialog(request);
                    } else {
                        debug_log(&format!("[TRACE] wgpu shell ignored unknown dialog {dialog_id}"));
                    }
                }
                semio_framework::kernel::Effect::LoadDocument { pack, spr } => {
                    if let Some(session) = self.docs.session.clone() {
                        if let Some(plugin) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() {
                            let instance_id = session.instance_id;
                            #[cfg(not(target_arch = "wasm32"))]
                            self.settle.submit_shell_io_future(async move { ShellIoCompletion::Program(plugin.load_app_document_pack(instance_id, &pack, &spr).await) });
                            #[cfg(target_arch = "wasm32")]
                            semio_framework_os_renderer_wgpu_host_base::spawn_app_task(async move {
                                if let Err(error) = plugin.load_app_document_pack(instance_id, &pack, &spr).await { debug_log(&format!("[TRACE] wgpu shell loadDocument effect failed: {error}")); }
                            });
                        }
                    }
                }
                semio_framework::kernel::Effect::DispatchAction { action: dispatch_action_id, args, .. } => {
                    self.settle.deferred_actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: dispatch_action_id, args });
                }
                semio_framework::kernel::Effect::RequestMediaFrames { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args, .. } => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = controller_id.to_string();
                        let args = optional_dsl_value_as_json(args);
                        self.settle.submit_shell_io_future(async move {
                            let frames = request_media_frames(&controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), args).await;
                            ShellIoCompletion::MediaFrames { controller_id, frames }
                        });
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let frames = request_media_frames(controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), optional_dsl_value_as_json(args));
                        self.opening.begin_media_frames_transfer(controller_id.to_string(), frames, &self.docs);
                    }
                }
                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {
                    let outcome =
                        self.sync.sync_channel.as_ref().ok_or_else(|| "document-backbone effect has no active shell owner".to_string()).and_then(|channel| {
                            route_document_backbone_effects(&channel.actor_uri, &channel.cmd_tx, vec![semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload }])
                                .and_then(|remaining| if remaining.is_empty() { Ok(()) } else { Err("document-backbone routing left an owned effect behind".into()) })
                        });
                    if let Err(error) = outcome {
                        self.sync.sync_terminal_fault = Some(error);
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::RequestServiceOperation {owner,service_id,action,payload} => {
                    self.open_service_port(&owner,&service_id,&action,payload);
                }
                semio_framework::kernel::Effect::InvokeExtension { req, extension_id, capability, request_json, .. } => {
                    #[cfg(target_arch = "wasm32")]
                    self.plugins.pending_extension_invocations.push(PendingExtensionInvocation { req: req.0, extension_id, capability, request_json });
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let _ = (req, extension_id, capability, request_json);
                    }
                }
                semio_framework::kernel::Effect::DownloadMediaExport { filename, mime_type, data, encoding } => {
                    download_media_export(&filename, &mime_type, &data, encoding.as_deref());
                }
                semio_framework::kernel::Effect::IconRenderExport { items } => {
                    self.opening.enqueue_icon_export(items);
                }
                semio_framework::kernel::Effect::RequestFileOpen { accept, read_as, import_action, args, multiple, .. } => {
                    #[cfg(target_arch = "wasm32")]
                    self.opening.pending_file_opens.push(PendingFileOpen { controller_id: controller_id.to_string(), accept, read_as, import_action, args, multiple });
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = controller_id.to_string();
                        self.settle.submit_shell_io_future(async move { ShellIoCompletion::Import(PickedImport { opened: request_file_open(&accept, read_as.as_deref(), multiple).await, controller_id, import_action, args, multiple }) });
                    }
                }
                other => {
                    debug_log(&format!("[TRACE] wgpu-shell effect dropped tag={other:?}"));
                }
            }
        }
    }
}

//#endregion ShellLifecycle

