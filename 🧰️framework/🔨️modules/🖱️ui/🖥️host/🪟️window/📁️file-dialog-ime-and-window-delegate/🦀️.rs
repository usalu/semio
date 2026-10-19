//! 📁️ File dialog, ime and window delegate: the native file dialog requests, the ime directives, the redraw outcome and the `WindowDelegate`. See [`super`].

use super::*;

//#region 🔖️Host
//#region 📁️FileDialog
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeFileDialogRequest {
    Open { extensions: Vec<String>, multiple: bool },
    Save { filename: String, extensions: Vec<String> },
    Folder,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeFileDialogRequest {
    pub fn open(extensions: impl IntoIterator<Item = impl Into<String>>, multiple: bool) -> Self {
        Self::Open { extensions: normalize_dialog_extensions(extensions), multiple }
    }

    pub fn save(filename: impl Into<String>, extensions: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Save { filename: filename.into(), extensions: normalize_dialog_extensions(extensions) }
    }

    pub fn folder() -> Self {
        Self::Folder
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn normalize_dialog_extensions(extensions: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut normalized: Vec<String> = extensions.into_iter().map(Into::into).map(|extension: String| extension.trim().trim_start_matches('.').to_ascii_lowercase()).filter(|extension| !extension.is_empty()).collect();
    normalized.sort();
    normalized.dedup();
    normalized
}

/// 📂️ Runs the native picker behind an owned request/result boundary. The returned future is
/// polled by the renderer's I/O-lane task seam; no renderer callback waits for user interaction.
#[cfg(not(target_arch = "wasm32"))]
pub async fn select_native_paths(request: NativeFileDialogRequest) -> Vec<std::path::PathBuf> {
    use rfd::AsyncFileDialog;

    match request {
        NativeFileDialogRequest::Open { extensions, multiple } => {
            let dialog = if extensions.is_empty() { AsyncFileDialog::new() } else { AsyncFileDialog::new().add_filter("import", &extensions) };
            if multiple {
                dialog.pick_files().await.unwrap_or_default().into_iter().map(|file| file.path().to_path_buf()).collect()
            } else {
                dialog.pick_file().await.into_iter().map(|file| file.path().to_path_buf()).collect()
            }
        }
        NativeFileDialogRequest::Save { filename, extensions } => {
            let dialog = AsyncFileDialog::new().set_file_name(filename);
            let dialog = if extensions.is_empty() { dialog } else { dialog.add_filter("export", &extensions) };
            dialog.save_file().await.into_iter().map(|file| file.path().to_path_buf()).collect()
        }
        NativeFileDialogRequest::Folder => AsyncFileDialog::new().pick_folder().await.into_iter().map(|folder| folder.path().to_path_buf()).collect(),
    }
}
//#endregion 📁️FileDialog

//#region 🈶️Ime
/// 🈶️ Applies a [`ImeDirective`] to a real window — `Enable` positions the platform IME candidate
/// window at the focused element's caret bounds (window-absolute, per `dispatch.rs`'s own docstring);
/// `Disable` turns IME off so plain `KeyboardInput` events resume (see `winit::Window::set_ime_allowed`'s
/// own docs on that trade-off).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn apply_ime_directive(window: &winit::window::Window, directive: ImeDirective) {
    match directive {
        ImeDirective::Enable { cursor_bounds } => {
            window.set_ime_allowed(true);
            window.set_ime_cursor_area(winit::dpi::PhysicalPosition::new(cursor_bounds.x as f64, cursor_bounds.y as f64), winit::dpi::PhysicalSize::new(cursor_bounds.w.max(1.0) as u32, cursor_bounds.h.max(1.0) as u32));
        }
        ImeDirective::Disable => window.set_ime_allowed(false),
    }
}
//#endregion 🈶️Ime

//#region 🧩️WindowDelegate
/// 📤️ What a redraw produced — the cursor/IME half of a [`ui_render::DispatchOutcome`] a delegate
/// accumulated while building the frame, handed back so [`NativeHost`]/[`CanvasHost`] can apply both
/// through the one dedup path ([`apply_window_cursor`]/[`apply_canvas_cursor`],
/// [`apply_ime_directive`]) without the delegate needing to know either exists.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedrawOutcome {
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
}

/// 🧩️ What a product/runtime crate implements to receive this host's normalized events and
/// scheduler-gated redraw ticks — the one seam that keeps `🦀️window.rs` itself ignorant of
/// `ui_render::Dispatcher`/`FrameEngine`, which belong to later packets (`runtime-present`, `os-host`).
/// Nothing in this trait's signature ever names a platform SDK type.
pub trait WindowDelegate {
    /// ⏱️ The scheduler this delegate's own frame-building code invalidates — [`NativeHost`]/
    /// [`CanvasHost`] only ever read it through [`should_request_redraw`]/`next_deadline`, never
    /// mutate it directly.
    fn scheduler_mut(&mut self) -> &mut FrameScheduler;

    /// 📥️ One normalized input event, already free of any platform type.
    fn handle_event(&mut self, event: DispatchEvent);

    /// 📐️ A resize or scale-factor change.
    fn handle_metrics(&mut self, metrics: WindowMetrics);

    /// 🖼️ Build and present one frame for the accumulated `reason`, returning the cursor/IME state
    /// that frame settled on.
    fn redraw(&mut self, reason: InvalidationReason) -> RedrawOutcome;

    /// 🚪️ Whether the host may actually close the window — `true` (close immediately) unless a
    /// delegate overrides this to, say, confirm unsaved changes first.
    fn close_requested(&mut self) -> bool {
        true
    }
}
//#endregion 🧩️WindowDelegate
//#endregion 🔖️Host
