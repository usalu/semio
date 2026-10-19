//! 🚀️ Native winit app: the native `native` module with the winit application handler, the event normalisation and the native event loop. See [`super`].

use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub(super) mod native {
    use super::{advance_frame_generation, collect_scheduler_redraw, ime_event_from_winit, invalidate_scheduler_redraw, DispatchEvent, EventModifiers, InvalidationReason, OsHost, PointerButton, WindowDelegate, WindowMetrics};
                                                                                                                                                                                                                                                                use crate::os_host::OsHostRetirement;
        use semio_framework_os_renderer_wgpu_app_runtime::AppMailbox;
    use std::sync::Arc;
    use ui_render::{PhysicalSize, PointerInfo};
    #[cfg(target_arch = "wasm32")]
    use ui_render::{PointerId, PointerKind};
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
    use winit::window::{Window, WindowAttributes, WindowId};

    //#region 🚀️WinitApp

    /// 📨️ Same two-phase boot handshake `SemioApp`'s own `HostUserEvent` used — kept, not deleted, per
    /// this file's own module docstring on why `ui_host::NativeHost`'s single-phase construction cannot
    /// replace it. Only the STEADY-STATE control flow changes: `ControlFlow::Poll` → `WaitUntil`/`Wait`.
    /// No `callbacks` payload (the old variant carried `ui_wgpu::wgpu::PointerCallbacks`) — this file
    /// normalizes input itself via `ui_host::event` and drives `AppRuntime` through the enqueue-only
    /// `dispatch_normalized_event` path above, so `boot_runtime` returns only the runtime.
    pub enum HostUserEvent {
        RuntimeReady {
            runtime: AppMailbox,
            presenter: semio_framework_os_renderer_wgpu_frame_presentation::AppPresenter,
        },
        FrameReady,
        /// 🔔️ Payload-free worker-completion signal. Receiving it invalidates the retained snapshot;
        /// no future is polled by the event loop.
        Wake,
    }

    /// 🚀️ `ApplicationHandler` replacing `SemioApp` (deleted by this packet's surgical `🦀️.rs` edit —
    /// see the report's redraw audit). Boot mirrors the old `SemioApp::resumed`/`user_event` handshake
    /// verbatim (window created synchronously, `AppRuntime` booted async, delivered via
    /// `HostUserEvent::RuntimeReady`); everything AFTER boot is new: no `ControlFlow::Poll`, no
    /// `RedrawRequested` re-arm, no `start_frame_loop` — every redraw and every control-flow recompute
    /// funnels through [`OsHost`]'s `WindowDelegate` impl above plus `ui_host::should_request_redraw`.
    /// Input normalization (`pointers`/`modifiers`/`last_pointer_pos`) ports `ui_host::window::native::
    /// NativeHost::normalize`'s exact match arms — that method is private on a private struct, so this
    /// file's own `normalize` free fn below duplicates its logic over the same public `ui_host::event`
    /// functions rather than being able to call it directly.
    pub struct WinitApp {
        proxy: EventLoopProxy<HostUserEvent>,
        plugin_filter: String,
        #[cfg(target_arch = "wasm32")]
        plugins: Option<wasm_bindgen::JsValue>,
        #[cfg(target_arch = "wasm32")]
        canvas: Option<web_sys::HtmlCanvasElement>,
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: std::path::PathBuf,
        #[cfg(not(target_arch = "wasm32"))]
        services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,
        window: Option<Arc<Window>>,
        /// ♿️ The window's platform accessibility adapter, attached before the window was first shown.
        #[cfg(not(target_arch = "wasm32"))]
        accessibility: Option<semio_framework_os_renderer_wgpu_native_platform::native_accessibility::NativeAccessibilityBridge>,
        host: Option<OsHost>,
        retirement: Option<OsHostRetirement>,
        frame_caller:semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameEventIssuer,
        close_pending:bool,
        #[cfg(not(target_arch = "wasm32"))]
        pointers: ui_host::PointerRegistry,
        modifiers: EventModifiers,
        last_pointer_pos: (f32, f32),
        pending_reason: Option<InvalidationReason>,
    }

    impl WinitApp {
        /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        #[allow(clippy::too_many_arguments)]
        pub fn new(
            proxy: EventLoopProxy<HostUserEvent>,
            plugin_filter: String,
            #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
            #[cfg(target_arch = "wasm32")] canvas: Option<web_sys::HtmlCanvasElement>,
            #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
            #[cfg(not(target_arch = "wasm32"))] services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>,
            frame_caller:semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameEventIssuer,
        ) -> Self {
            Self {
                proxy,
                plugin_filter,
                #[cfg(target_arch = "wasm32")]
                plugins,
                #[cfg(target_arch = "wasm32")]
                canvas,
                #[cfg(not(target_arch = "wasm32"))]
                plugin_modules_root,
                #[cfg(not(target_arch = "wasm32"))]
                services,
                window: None,
                #[cfg(not(target_arch = "wasm32"))]
                accessibility: None,
                host: None,
                retirement: None,
                frame_caller,
                close_pending:false,
                #[cfg(not(target_arch = "wasm32"))]
                pointers: ui_host::PointerRegistry::new(),
                modifiers: EventModifiers::default(),
                last_pointer_pos: (0.0, 0.0),
                pending_reason: None,
            }
        }
    }

    //#region 🎛️WinitEventNormalization

    fn pointer_info_for_mouse(app: &mut WinitApp, device: winit::event::DeviceId) -> PointerInfo {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_info_for_mouse(&mut app.pointers, device)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (app, device);
            PointerInfo { id: PointerId(1_u64 << 62), kind: PointerKind::Mouse, pressure: None, tilt: None }
        }
    }

    fn pointer_info_for_touch(app: &mut WinitApp, touch: &winit::event::Touch) -> PointerInfo {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_info_for_touch(&mut app.pointers, touch)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = app;
            PointerInfo { id: PointerId((0b10_u64 << 62) | (touch.id & ((1_u64 << 62) - 1))), kind: PointerKind::Touch, pressure: touch.force.map(|force| force.normalized() as f32), tilt: None }
        }
    }

    fn pointer_button_from_winit(button: winit::event::MouseButton) -> Option<PointerButton> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_button_from_winit(button)
        }
        #[cfg(target_arch = "wasm32")]
        {
            match button {
                winit::event::MouseButton::Left => Some(PointerButton::Primary),
                winit::event::MouseButton::Right => Some(PointerButton::Secondary),
                winit::event::MouseButton::Middle => Some(PointerButton::Middle),
                _ => None,
            }
        }
    }

    fn normalize_wheel_delta(delta: winit::event::MouseScrollDelta) -> (f32, f32) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::normalize_wheel_delta_native(delta)
        }
        #[cfg(target_arch = "wasm32")]
        {
            match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => (x * 40.0, y * 40.0),
                winit::event::MouseScrollDelta::PixelDelta(position) => (position.x as f32, position.y as f32),
            }
        }
    }

    fn modifiers_from_winit(state: winit::keyboard::ModifiersState) -> EventModifiers {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::modifiers_from_winit(state)
        }
        #[cfg(target_arch = "wasm32")]
        {
            EventModifiers { shift: state.shift_key(), ctrl: state.control_key(), alt: state.alt_key(), meta: state.super_key() }
        }
    }

    fn logical_key_to_dispatch_string(key: &winit::keyboard::Key) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::logical_key_to_dispatch_string(key)
        }
        #[cfg(target_arch = "wasm32")]
        {
            use winit::keyboard::Key;
            match key {
                Key::Character(value) => value.to_string(),
                Key::Named(named) => named_key_label(*named).to_string(),
                Key::Dead(Some(value)) => value.to_string(),
                _ => "Unidentified".to_string(),
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn named_key_label(named: winit::keyboard::NamedKey) -> &'static str {
        use winit::keyboard::NamedKey as N;
        match named {
            N::Enter => "Enter",
            N::Tab => "Tab",
            N::Space => " ",
            N::ArrowDown => "ArrowDown",
            N::ArrowLeft => "ArrowLeft",
            N::ArrowRight => "ArrowRight",
            N::ArrowUp => "ArrowUp",
            N::End => "End",
            N::Home => "Home",
            N::PageDown => "PageDown",
            N::PageUp => "PageUp",
            N::Backspace => "Backspace",
            N::Delete => "Delete",
            N::Escape => "Escape",
            N::Shift => "Shift",
            N::Control => "Control",
            N::Alt => "Alt",
            N::Meta | N::Super => "Meta",
            N::CapsLock => "CapsLock",
            N::ContextMenu => "ContextMenu",
            N::F1 => "F1",
            N::F2 => "F2",
            N::F3 => "F3",
            N::F4 => "F4",
            N::F5 => "F5",
            N::F6 => "F6",
            N::F7 => "F7",
            N::F8 => "F8",
            N::F9 => "F9",
            N::F10 => "F10",
            N::F11 => "F11",
            N::F12 => "F12",
            _ => "Unidentified",
        }
    }

    /// 📐️ Device pixels per logical pixel for the live window, or `1.0` before one exists. `winit`
    /// hands every pointer position in PHYSICAL pixels; the renderer hit-tests, lays out and paints
    /// in LOGICAL ones, so this is the native half of the ingress conversion the browser transport
    /// performs on its own side. Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn pointer_scale_factor(app: &WinitApp) -> f32 {
        app.window.as_ref().map_or(1.0, |window| {
            let scale = window.scale_factor() as f32;
            if scale.is_finite() && scale > 0.0 {
                scale
            } else {
                1.0
            }
        })
    }

    /// 🔀️ Raw `winit::event::WindowEvent` → `ui_render::DispatchEvent`, ported from
    /// `ui_host::window::native::NativeHost::normalize` (private, see this file's own struct docstring).
    /// Pointer and touch positions arrive PHYSICAL and leave LOGICAL — see [`pointer_scale_factor`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn normalize(app: &mut WinitApp, event: &WindowEvent) -> Option<DispatchEvent> {
        use winit::event::{ElementState, TouchPhase};
        let scale = pointer_scale_factor(app);
        match event {
            WindowEvent::CursorMoved { device_id, position } => {
                app.last_pointer_pos = (position.x as f32 / scale, position.y as f32 / scale);
                let pointer = pointer_info_for_mouse(app, *device_id);
                Some(DispatchEvent::PointerMove { pointer, x: app.last_pointer_pos.0, y: app.last_pointer_pos.1, modifiers: app.modifiers })
            }
            WindowEvent::MouseInput { device_id, state, button } => {
                let pointer = pointer_info_for_mouse(app, *device_id);
                let button = pointer_button_from_winit(*button)?;
                let (x, y) = app.last_pointer_pos;
                Some(match state {
                    ElementState::Pressed => DispatchEvent::PointerDown { pointer, x, y, button, modifiers: app.modifiers },
                    ElementState::Released => DispatchEvent::PointerUp { pointer, x, y, button, modifiers: app.modifiers },
                })
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (delta_x, delta_y) = normalize_wheel_delta(*delta);
                let (x, y) = app.last_pointer_pos;
                Some(DispatchEvent::Scroll { x, y, delta_x, delta_y, modifiers: app.modifiers })
            }
            WindowEvent::Touch(touch) => {
                let pointer = pointer_info_for_touch(app, touch);
                let x = touch.location.x as f32 / scale;
                let y = touch.location.y as f32 / scale;
                Some(match touch.phase {
                    TouchPhase::Started => DispatchEvent::PointerDown { pointer, x, y, button: PointerButton::Primary, modifiers: app.modifiers },
                    TouchPhase::Moved => DispatchEvent::PointerMove { pointer, x, y, modifiers: app.modifiers },
                    TouchPhase::Ended => DispatchEvent::PointerUp { pointer, x, y, button: PointerButton::Primary, modifiers: app.modifiers },
                    TouchPhase::Cancelled => DispatchEvent::PointerCancel { pointer },
                })
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let logical = logical_key_to_dispatch_string(&event.logical_key);
                Some(ui_host::key_dispatch_event(logical, app.modifiers, event.state == ElementState::Pressed))
            }
            WindowEvent::Ime(ime) => Some(DispatchEvent::Ime(ime_event_from_winit(ime))),
            WindowEvent::ModifiersChanged(modifiers) => {
                app.modifiers = modifiers_from_winit(modifiers.state());
                None
            }
            _ => None,
        }
    }

    //#endregion 🎛️WinitEventNormalization

    /// 🌓️ Publishes the OS light/dark preference winit reports into [`crate::set_host_appearance`],
    /// leaving that value's persisted-preference term untouched. `None` — a platform that reports no
    /// preference, and every wasm realm, where winit has no OS theme to read — changes NOTHING, so a
    /// value the page thread already forwarded survives window creation instead of being reset to
    /// light by it.
    // 🚫️async: U1 — called from winit's own sync `ApplicationHandler` callbacks.
    fn publish_system_appearance(theme: Option<winit::window::Theme>) {
        let Some(theme) = theme else { return };
        semio_framework_os_renderer_wgpu_host_base::set_host_appearance(semio_framework_os_renderer_wgpu_host_base::HostAppearance { system_dark: matches!(theme, winit::window::Theme::Dark), ..semio_framework_os_renderer_wgpu_host_base::host_appearance() });
    }

    impl ApplicationHandler<HostUserEvent> for WinitApp {
        fn new_events(&mut self,event_loop:&ActiveEventLoop,cause:winit::event::StartCause){
            if matches!(cause,winit::event::StartCause::ResumeTimeReached{..})&&self.host.as_ref().is_some_and(|host|self.pending_reason.is_some()||!host.original_event_terminal_is_empty()||host.scheduler.next_deadline().is_some_and(|deadline|deadline.due<=host.now_seconds())){let _=self.frame_caller.observe_event();self.recompute_control_flow(event_loop);}
        }
        /// 🪟️ Window creation ported verbatim from the old `SemioApp::resumed` (title/size/canvas-mount
        /// logic unchanged — see this file's own module docstring for why the two-phase handshake this
        /// method starts is kept, not deleted). Only the tail changes: no `start_frame_loop` call.
        /// 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        ///
        /// ⏱️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): registers THIS thread —
        /// winit's callback thread, the only thread `resumed`/`window_event`/`about_to_wait` ever run
        /// on — as the UI thread with `semio-framework-trace`'s thread-role census, so
        /// `semio_framework_trace::is_ui_thread()`/`assert_ui_thread()` are meaningful anywhere in this
        /// process from this point on. Exactly once, first callback, before any event can be normalized.
        ///
        /// 🏷️ React's `ShellBrand.windowTitle` — the brand ROW the boot descriptor carries
        /// (`WgpuBootBrand`), `"Semio"` when this boot resolved no brand. React sets the same string
        /// as the document title on its own shell (`🧑‍💻dev/🟦️.ts`'s brand mount).
        ///
        /// ♿️ The platform accessibility adapter must exist before the window is first shown (AccessKit's own
        /// contract), so the native window is created hidden, the bridge attached, and only then shown.
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_some() {
                return;
            }
            semio_framework_trace::register_ui_thread();
            let mut attributes = WindowAttributes::default().with_title(semio_framework_os_renderer_wgpu_host_base::boot_window_title());
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                if let Some(canvas) = self.canvas.clone() {
                    let css_width = canvas.client_width().max(1) as f32;
                    let css_height = canvas.client_height().max(1) as f32;
                    let _ = canvas.style().set_property("width", "100%");
                    let _ = canvas.style().set_property("height", "100vh");
                    attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(css_width, css_height)).with_canvas(Some(canvas)).with_append(true);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0)).with_visible(false);
            }
            let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
            #[cfg(not(target_arch = "wasm32"))]
            {
                let proxy = self.proxy.clone();
                self.accessibility = Some(semio_framework_os_renderer_wgpu_native_platform::native_accessibility::NativeAccessibilityBridge::new(
                    event_loop,
                    &window,
                    Arc::new(move || {
                        let _ = proxy.send_event(HostUserEvent::Wake);
                    }),
                ));
                window.set_visible(true);
            }
            publish_system_appearance(window.theme());
            self.window = Some(window.clone());
            let proxy = self.proxy.clone();
            let plugin_filter = self.plugin_filter.clone();
            #[cfg(target_arch = "wasm32")]
            let plugins = self.plugins.clone();
            #[cfg(not(target_arch = "wasm32"))]
            let plugin_modules_root = self.plugin_modules_root.clone();
            #[cfg(not(target_arch = "wasm32"))]
            let services=self.services.clone();
            let gpu = ui_gpu::platform::NativeSurfaceCapture::capture(window.clone()).expect("native GPU surface capture").prepare();
            semio_framework_os_renderer_wgpu_host_base::spawn_app_task(async move {
                let result = semio_framework_os_renderer_wgpu_frame_transaction::boot_runtime(
                    window,
                    gpu,
                    plugin_filter,
                    #[cfg(target_arch = "wasm32")]
                    plugins,
                    #[cfg(not(target_arch = "wasm32"))]
                    plugin_modules_root,
                    #[cfg(not(target_arch = "wasm32"))]
                    services,
                )
                .await;
                match result {
                    Ok((runtime, presenter)) => {
                        let _ = proxy.send_event(HostUserEvent::RuntimeReady { runtime, presenter });
                    }
                    Err(error) => semio_framework_os_renderer_wgpu_host_base::log_debug(&format!("boot_runtime failed: {error}")),
                }
            });
        }

        /// 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        ///
        /// 🔔️ Worker completion wake: invalidate only; no future is polled on this callback.
        fn user_event(&mut self, event_loop: &ActiveEventLoop, event: HostUserEvent) {
            match event {
                HostUserEvent::RuntimeReady { runtime, presenter } => {
                    semio_framework_os_renderer_wgpu_host_base::log_debug_diagnostic("[TRACE] native renderer boot ready");
                    let mut host = OsHost::new(runtime, presenter, semio_framework_os_renderer_wgpu_frame_job::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT);
                    let proxy = self.proxy.clone();
                    #[cfg(not(target_arch = "wasm32"))]
                    host.runtime.set_waker(Arc::new(move || {
                        let _ = proxy.send_event(HostUserEvent::Wake);
                    }));
                    #[cfg(target_arch = "wasm32")]
                    host.runtime.set_waker(std::rc::Rc::new(move || {
                        let _ = proxy.send_event(HostUserEvent::Wake);
                    }));
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let proxy = self.proxy.clone();
                        host.frame_build.set_completion_waker(Arc::new(move || {
                            let _ = proxy.send_event(HostUserEvent::FrameReady);
                        }));
                    }
                    host.scheduler.invalidate(InvalidationReason::STRUCTURE);
                    self.host = Some(host);
                }
                HostUserEvent::FrameReady => {
                    if let Some(host) = self.host.as_mut() {
                        host.frame_ready = true;
                        host.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                    }
                }
                HostUserEvent::Wake => {
                    if let Some(host) = self.host.as_mut() {
                        if !advance_frame_generation(&mut host.frame_generation) {
                            host.present_fault = Some("frame generation exhausted".to_string());
                        }
                        host.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                    }
                }
            }
            if self.host.is_some(){let _=self.frame_caller.observe_event();}
            self.recompute_control_flow(event_loop);
        }

        /// 🌓️ The OS switched light/dark under a shell whose appearance preference is
        /// `"system"`. The next `FrameBuildPhase::ThemeResolve` re-reads the published value,
        /// so the only thing owed here is a redraw — the twin of React's ONE shared
        /// `matchMedia("(prefers-color-scheme: dark)")` `change` listener
        /// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `ensureElementsSurfaceChromeSystemListeners`).
        /// 🫥️ The window lost focus: every open text-editor typing run ends as ONE edit (`blur`, design §13.2 of ticket
        /// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); the open run's pending outbox keeps the frame loop driving its commit.
        /// 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
            let Some(window) = self.window.clone() else { return };
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(bridge) = self.accessibility.as_mut() {
                bridge.process_window_event(&window, &event);
            }
            if self.host.is_none() && self.retirement.is_none() {
                return;
            }
            match &event {
                WindowEvent::CloseRequested => {
                    let _=self.frame_caller.observe_event();
                    let requested = self.host.as_mut().is_some_and(|host| host.close_requested());
                    if requested {
                        self.close_pending=true;
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                    }
                    return;
                }
                WindowEvent::Resized(size) => {
                    let _=self.frame_caller.observe_event();
                    let scale_factor = window.scale_factor() as f32;
                    if let Some(host) = self.host.as_mut() {
                        host.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor });
                    }
                }
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    let _=self.frame_caller.observe_event();
                    let size = window.inner_size();
                    if let Some(host) = self.host.as_mut() {
                        host.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor: *scale_factor as f32 });
                    }
                }
                WindowEvent::ThemeChanged(theme) => {
                    let _=self.frame_caller.observe_event();
                    publish_system_appearance(Some(*theme));
                    if let Some(host) = self.host.as_mut() {
                        let now = host.now_seconds();
                        if invalidate_scheduler_redraw(&mut host.scheduler, &mut self.pending_reason, now, InvalidationReason::THEME).is_some() {
                            window.request_redraw();
                        }
                    }
                }
                WindowEvent::Focused(false) => {
                    let _=self.frame_caller.observe_event();
                    semio_framework_os_renderer_wgpu_shell_window_actions::note_host_window_blur();
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    if let Some(reason) = self.pending_reason.take() {
                        if !self.frame_caller.ready()||self.frame_caller.finishing(){self.pending_reason=Some(reason);self.recompute_control_flow(event_loop);return;}
                        if let Some(host) = self.host.as_mut() {
                            let call=self.frame_caller.dispatch_step(|cx|host.redraw_on_original_caller(cx));
                            match call{Ok(original)=>{if original.result.is_err(){self.close_pending=true;}},Err(_)=>{self.pending_reason=Some(reason);window.request_redraw();}}
                        }
                    }
                    self.recompute_control_flow(event_loop);
                    return;
                }
                _ => {
                    if let Some(dispatch_event) = normalize(self, &event) {
                        let _=self.frame_caller.observe_event();
                        if let Some(host) = self.host.as_mut() {
                            host.handle_event(dispatch_event);
                        }
                    }
                }
            }
            self.recompute_control_flow(event_loop);
        }

        /// 🌙️ Requests a redraw only when the scheduler reports invalidation or a due deadline.
        /// Native futures run exclusively on the process worker pool; this callback never polls them.
        /// 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        ///
        /// ♿️ The platform tree follows the latest presented publication; every assistive-technology action is
        /// handed to the shell as the accessibility event it is, exactly like a browser mirror event.
        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            if self.frame_caller.finishing(){let _=self.frame_caller.finish_event_step();self.recompute_control_flow(event_loop);return;}
            if !self.frame_caller.terminal_is_empty()&&!self.frame_caller.ready(){
                let _=self.frame_caller.admit_step();
                event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                return;
            }
            if self.close_pending&&self.host.is_some(){
                let returned=self.frame_caller.dispatch_cleanup_step(|cx|OsHost::try_into_retirement(&mut self.host,cx));
                if let Ok(original)=returned{
                    if let Ok(Some(retirement))=original.result{self.retirement=Some(retirement);}
                }
                event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                return;
            }
            if let Some(retirement) = self.retirement.as_mut() {
                let returned=self.frame_caller.dispatch_cleanup_step(|cx|retirement.close_step(cx));
                if returned.as_ref().is_ok_and(|original|original.result.as_ref().is_ok_and(|done|*done)) && retirement.terminal_is_empty() {
                    self.retirement = None;
                }
                event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                return;
            }
            if self.close_pending&&self.host.is_none()&&self.retirement.is_none(){
                let complete=self.frame_caller.finish_event_step().is_ok_and(|complete|complete);
                if complete&&self.frame_caller.terminal_is_empty(){
                    #[cfg(not(target_arch="wasm32"))]
                    {self.accessibility=None;}
                    self.window=None;event_loop.exit();
                }else{event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));}
                return;
            }
            let Some(host) = self.host.as_mut() else { return };
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(bridge) = self.accessibility.as_mut() {
                bridge.refresh();
                for event in bridge.take_dispatch_events() {
                    let _=self.frame_caller.observe_event();
                    host.handle_event(event);
                }
            }
            let now = host.now_seconds();
            if let Some(reason) = collect_scheduler_redraw(&mut host.scheduler, &mut self.pending_reason, now) {
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                    if reason.contains(InvalidationReason::RESOURCE_READY) {
                        let _ = host.take_cursor_wake_directive();
                    }
                }
            }
            if self.pending_reason.is_none()&&self.host.as_ref().is_some_and(OsHost::original_event_terminal_is_empty)&&!self.frame_caller.terminal_is_empty(){let _=self.frame_caller.finish_event_step();}
            self.recompute_control_flow(event_loop);
        }
    }

    impl WinitApp {
        /// 🚦️ `WaitUntil(next deadline)` / `Wait` — never `Poll` (this file's own headline change).
        /// `ControlFlow::wait_duration` selects winit's target clock while the scheduler remains in
        /// elapsed seconds, so native and browser builds share the same deadline policy.
        /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        fn recompute_control_flow(&mut self, event_loop: &ActiveEventLoop) {
            if self.close_pending||self.frame_caller.finishing()||(!self.frame_caller.terminal_is_empty()&&!self.frame_caller.ready())||self.pending_reason.is_some(){event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));return;}
            let Some(host) = self.host.as_ref() else {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                return;
            };
            event_loop.set_control_flow(match host.scheduler.next_deadline() {
                Some(deadline) => {
                    let remaining = (deadline.due - host.now_seconds()).max(0.0);
                    winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_secs_f64(remaining))
                }
                None => winit::event_loop::ControlFlow::Wait,
            });
        }
    }

    //#endregion 🚀️WinitApp
}
