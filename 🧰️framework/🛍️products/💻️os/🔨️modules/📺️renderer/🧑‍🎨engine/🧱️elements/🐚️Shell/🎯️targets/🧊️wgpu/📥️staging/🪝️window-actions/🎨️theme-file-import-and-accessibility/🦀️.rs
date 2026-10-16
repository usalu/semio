//! 🪝️ `shell-window-actions` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region RequestMediaFrames
/// 🔓️ Decodes a `data:<mime>;base64,<data>` URL's payload; `None` for anything malformed or missing a
/// comma separator. Used by both `RequestMediaFrames.payload` (bytes already in memory) and the
/// fallback-response encoding below.
pub fn decode_data_url(data_url: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    let comma = data_url.find(',')?;
    base64::engine::general_purpose::STANDARD.decode(&data_url[comma + 1..]).ok()
}

/// 📦️ Builds the single `fallback_action` `ActionDescriptor` — raw `bytes` re-encoded as a data URL
/// merged into `base_args`, same shape on every failure path (`ffmpeg` missing, spawn/scratch-dir I/O
/// failure, no source bytes at all) so the plugin's fallback handler only needs to handle one shape.
pub fn fallback_action_descriptor(controller_id: &str, fallback_action: &str, bytes: &[u8], name: &str, base_args: &Value) -> ActionDescriptor {
    use base64::Engine;
    let mut args = base_args.clone();
    if let Some(obj) = args.as_object_mut() {
        obj.insert("payload".into(), Value::String(format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))));
        obj.insert("name".into(), Value::String(name.to_string()));
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: fallback_action.to_string(), args: Some(semio_framework::DslValue::from(args)) }
}

/// 🧮️ Pure `ffmpeg` argument computation for D5 frame extraction (precedent: `animate/video/rs/lib.rs`'s
/// `run_ffmpeg`, which this shell doesn't depend on directly — cross-technology dep avoided per house
/// rules — but whose `Command::new("ffmpeg").args(...).status()` invocation convention this follows).
/// `sample_stride` floors at 1 (every frame); `max_frames` of 0 means "host default" (capped generously
/// rather than truly unlimited, so a pathological request can't fill disk); `max_long_edge_px` of 0
/// skips the scale filter entirely (native resolution).
#[cfg(any(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
pub fn ffmpeg_frame_extraction_args(sample_stride: u32, max_frames: u32, max_long_edge_px: u32, input: &std::path::Path, out_dir: &std::path::Path) -> Vec<String> {
    let stride = sample_stride.max(1);
    let filter = if max_long_edge_px > 0 { format!("select=not(mod(n\\,{stride})),scale={max_long_edge_px}:-2") } else { format!("select=not(mod(n\\,{stride}))") };
    let frames_cap = if max_frames > 0 { max_frames } else { 100_000 };
    vec!["-y".into(), "-i".into(), input.display().to_string(), "-vf".into(), filter, "-vsync".into(), "vfr".into(), "-frames:v".into(), frames_cap.to_string(), out_dir.join("%06d.jpg").display().to_string()]
}

/// ⏱️ Approximate per-extracted-frame timestamp from the requested sampling cadence — `ffmpeg`'s own
/// frame PTS aren't threaded back through the `%06d.jpg` sequence (would need an `ffprobe` pass or
/// `-frame_pts`/timebase math this ticket scopes out; documented simplification, same spirit as the D1
/// wgpu point-sprite pass note above `render_world_3d`). Good enough for frame *ordering*/spacing; exact
/// decode timestamps are future work if a downstream consumer needs true sub-frame sync.
#[cfg(any(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
pub fn approx_sampled_timestamp_ms(index: u32, sample_stride: u32, fps_hint: f64) -> f64 {
    let stride = sample_stride.max(1) as f64;
    let fps = if fps_hint > 0.0 { fps_hint } else { 30.0 };
    (index as f64) * stride / fps * 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
fn ffmpeg_available() -> bool {
    use std::process as system_process;
    system_process::Command::new("ffmpeg").arg("-version").stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).status().is_ok_and(|status| status.success())
}

/// 🎞️ D5 native pipeline, beside `request_file_open` above: obtains video bytes (native file
/// picker, or `payload`'s data-URL bytes when the caller already has them from a drop zone), shells out
/// to `ffmpeg` to sample frames into a scratch temp directory, base64-encodes each resulting JPEG, and
/// returns one `ActionDescriptor` per frame (`frame_action`) followed by one for `done_action` — or, on
/// any failure (no `ffmpeg` on `PATH`, no source bytes, scratch I/O failure, non-zero `ffmpeg` exit), a
/// single `fallback_action` descriptor carrying the raw bytes so the plugin's own in-process decoder
/// (MJPEG/H.264-baseline) gets a chance.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
pub async fn request_media_frames(
    controller_id: &str,
    accept: &str,
    frame_action: &str,
    done_action: &str,
    fallback_action: &str,
    sample_stride: u32,
    max_frames: u32,
    max_long_edge_px: u32,
    fps_hint: f64,
    payload: Option<&str>,
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    use base64::Engine;
    use std::fs as system_fs;
    use std::process as system_process;
    let base_args = args.unwrap_or_else(|| serde_json::json!({}));
    let (bytes, name) = match payload {
        Some(data_url) => match decode_data_url(data_url) {
            Some(decoded) => (decoded, "video".to_string()),
            None => return Vec::new(),
        },
        None => {
            let extensions: Vec<String> = accept.split(',').filter_map(|entry| entry.trim().strip_prefix('.').map(str::to_string)).collect();
            let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(extensions, false)).await.into_iter().next() else {
                return Vec::new();
            };
            let Ok(bytes) = system_fs::read(&path) else {
                return Vec::new();
            };
            let name = path.file_name().map_or_else(|| "video".to_string(), |value| value.to_string_lossy().into_owned());
            (bytes, name)
        }
    };
    if !ffmpeg_available() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let scratch_dir = std::env::temp_dir().join(format!("semio-media-frames-{}-{}", system_process::id(), name.len()));
    if system_fs::create_dir_all(&scratch_dir).is_err() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let input_path = scratch_dir.join("source.bin");
    if system_fs::write(&input_path, &bytes).is_err() {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let ffmpeg_args = ffmpeg_frame_extraction_args(sample_stride, max_frames, max_long_edge_px, &input_path, &scratch_dir);
    let extracted = system_process::Command::new("ffmpeg").args(&ffmpeg_args).status().is_ok_and(|status| status.success());
    if !extracted {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let mut frame_paths: Vec<std::path::PathBuf> =
        system_fs::read_dir(&scratch_dir).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("jpg")).collect()).unwrap_or_default();
    frame_paths.sort();
    let total = frame_paths.len();
    let mut actions = Vec::with_capacity(total + 1);
    for (index, frame_path) in frame_paths.iter().enumerate() {
        let Ok(jpeg_bytes) = system_fs::read(frame_path) else {
            continue;
        };
        let mut frame_args = base_args.clone();
        if let Some(obj) = frame_args.as_object_mut() {
            obj.insert("payload".into(), Value::String(format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(&jpeg_bytes))));
            obj.insert("name".into(), Value::String(name.clone()));
            obj.insert("frameIndex".into(), serde_json::json!(index));
            obj.insert("timestampMs".into(), serde_json::json!(approx_sampled_timestamp_ms(index as u32, sample_stride, fps_hint)));
            obj.insert("index".into(), serde_json::json!(index));
            obj.insert("total".into(), serde_json::json!(total));
        }
        actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: frame_action.to_string(), args: Some(semio_framework::DslValue::from(frame_args)) });
    }
    let mut done_args = base_args;
    if let Some(obj) = done_args.as_object_mut() {
        obj.insert("name".into(), Value::String(name));
        obj.insert("frameCount".into(), serde_json::json!(total));
        obj.insert("sampledCount".into(), serde_json::json!(total));
    }
    actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: done_action.to_string(), args: Some(semio_framework::DslValue::from(done_args)) });
    let _ = system_fs::remove_dir_all(&scratch_dir);
    actions
}

/// 🕸️ wasm32 mirrors `request_file_open`'s stub: no native `ffmpeg`/file-dialog surface, the browser
/// React shell handles `RequestMediaFrames` itself. If `payload` bytes are already in hand (a drop
/// zone), still honor `fallback_action` with them so an in-process program decoder gets a chance even on
/// this native/wasm shell.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
pub fn request_media_frames(
    controller_id: &str,
    _accept: &str,
    _frame_action: &str,
    _done_action: &str,
    fallback_action: &str,
    _sample_stride: u32,
    _max_frames: u32,
    _max_long_edge_px: u32,
    _fps_hint: f64,
    payload: Option<&str>,
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    match payload.and_then(decode_data_url) {
        Some(bytes) => vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, "video", &args.unwrap_or_else(|| serde_json::json!({})))],
        None => Vec::new(),
    }
}

//#endregion RequestMediaFrames

#[cfg(not(target_arch = "wasm32"))]
pub async fn request_file_save(filename: &str) -> Option<std::path::PathBuf> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(filename, ["json"])).await.into_iter().next()
}

#[cfg(target_arch = "wasm32")]
pub async fn pick_native_path_from_host(request: serde_json::Value) -> Option<String> {
    let answer = host_io_call(&request.to_string(), None).await.ok()?;
    serde_json::from_str::<Option<String>>(&answer).ok().flatten()
}

//#region 📤️FileOpenImport
/// ⏱️ The wall budget one drain spends dispatching import chunks, so a Cancel lands between chunks.
pub const IMPORT_TRANSFER_TURN_MS: f64 = 8.0;
//#endregion 📤️FileOpenImport

//#region 🎨️ThemeDocumentDoors
/// 🎨️ Commits one document into the custom-theme registry under `id` and activates it — the present
/// side of React's `saveTheme`, which writes `canonicalUiTheme(saved)` into
/// `customThemes[id]` and then selects it. The stored text is the SAME canonical JSON React stores,
/// so a theme saved on either renderer opens on the other.
pub fn save_custom_theme_document(id: &str, document: &ThemeDocument) -> Option<String> {
    let raw = document.canonical_json()?;
    with_chrome_prefs(|prefs| {
        prefs.custom_themes.insert(id.to_string(), raw.clone());
        prefs.theme_id = id.to_string();
        prefs.theme_document_draft = None;
        prefs.theme_document_generation = prefs.theme_document_generation.wrapping_add(1);
        prefs.theme_document_resolved = None;
    });
    Some(raw)
}

/// 🆔️ React's `saveTheme` slug: lowercase, non-alphanumerics collapsed to `-`, trimmed, under
/// `custom.`.
pub fn custom_theme_id_for_label(label: &str) -> Option<String> {
    let mut slug = String::new();
    let mut pending_dash = false;
    for character in label.trim().to_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(character);
        } else {
            pending_dash = true;
        }
    }
    (!slug.is_empty()).then(|| format!("custom.{slug}"))
}

/// 🔀️ Merges the value a retained control committed INTO the args its node authored, instead of
/// replacing them.
///
/// 🩸️ Every retained commit on this target used to dispatch `{ value }` alone, so an authored arg was
/// silently dropped: the Default Apps leaf authored `{ row: "<dialect>/<role>" }` on its `Select` and
/// its own handler read `args.row`, which therefore never arrived, and the whole theme editor (one
/// verb, ~450 rows, the key in the args) would have been unable to say WHICH key it wrote. React's
/// handlers are closures that captured those coordinates, which is what this restores. Keys stay
/// ascending so the projected argument object is stable.
pub fn merge_committed_args(authored: Option<&DslValue>, value: DslValue) -> Option<DslValue> {
    let mut fields: Vec<(String, DslValue)> = match authored {
        Some(semio_framework_value::DslValue::Object(existing)) => existing.iter().filter(|(key, _)| key != "value").cloned().collect(),
        _ => Vec::new(),
    };
    fields.push(("value".to_string(), value));
    fields.sort_by(|left, right| left.0.cmp(&right.0));
    Some(semio_framework_value::DslValue::Object(fields))
}
//#endregion 🎨️ThemeDocumentDoors

//#region ⚖️MergePolicyAndThemeCommits
/// ⬆️ React's `requestFileOpen(".theme.dsl,.dsl,text/plain")` accept list, verbatim.
pub const SHELL_THEME_IMPORT_ACCEPT: &str = ".theme.dsl,.dsl,text/plain";
//#endregion ⚖️MergePolicyAndThemeCommits
