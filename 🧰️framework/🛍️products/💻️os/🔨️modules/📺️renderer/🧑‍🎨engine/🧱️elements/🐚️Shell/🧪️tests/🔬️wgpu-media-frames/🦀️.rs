
use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn stalled_shell_io_keeps_mailbox_poll_p99_below_two_ms() {
    let mut state = ShellState::new(Vec::new(), String::new());
    state.submit_shell_io(|| {
        std::thread::sleep(std::time::Duration::from_millis(80));
        ShellIoCompletion::Finished
    });
    let mut samples = Vec::with_capacity(4096);
    for _ in 0..4096 {
        let started = std::time::Instant::now();
        let _ = state.poll_shell_io();
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    let p99 = samples[samples.len() * 99 / 100];
    assert!(p99 < std::time::Duration::from_millis(2), "shell I/O callback p99 was {p99:?}");
}

#[test]
fn ffmpeg_args_apply_stride_scale_and_frame_cap() {
    let input = std::path::Path::new("/tmp/in.mp4");
    let out_dir = std::path::Path::new("/tmp/out");
    let args = ffmpeg_frame_extraction_args(5, 200, 1600, input, out_dir);
    assert_eq!(args[0], "-y");
    assert_eq!(args[2], "/tmp/in.mp4");
    assert_eq!(args[4], "select=not(mod(n\\,5)),scale=1600:-2");
    assert_eq!(args[8], "200");
    assert_eq!(args[9], "/tmp/out/%06d.jpg");
}

#[test]
fn ffmpeg_args_floor_stride_and_default_frame_cap() {
    let args = ffmpeg_frame_extraction_args(0, 0, 0, std::path::Path::new("in.mp4"), std::path::Path::new("out"));
    assert_eq!(args[4], "select=not(mod(n\\,1))", "stride 0 floors to 1: {args:?}");
    assert_eq!(args[8], "100000", "max_frames 0 falls back to a generous cap: {args:?}");
}

#[test]
fn ffmpeg_args_omit_scale_filter_when_max_long_edge_zero() {
    let args = ffmpeg_frame_extraction_args(1, 10, 0, std::path::Path::new("in.mp4"), std::path::Path::new("out"));
    assert!(!args[4].contains("scale"), "{args:?}");
}

#[test]
fn approx_timestamp_scales_with_stride_and_fps() {
    assert_eq!(approx_sampled_timestamp_ms(0, 5, 30.0), 0.0);
    assert!((approx_sampled_timestamp_ms(1, 5, 30.0) - (5.0 / 30.0 * 1000.0)).abs() < 1e-9);
    // fps_hint of 0 falls back to a 30 fps default rather than dividing by zero.
    assert!((approx_sampled_timestamp_ms(1, 5, 0.0) - (5.0 / 30.0 * 1000.0)).abs() < 1e-9);
}

#[test]
fn decode_data_url_round_trips_bytes() {
    use base64::Engine;
    let bytes = vec![1u8, 2, 3, 250];
    let url = format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes));
    assert_eq!(decode_data_url(&url), Some(bytes));
    assert_eq!(decode_data_url("not-a-data-url"), None);
}

#[test]
fn fallback_descriptor_merges_payload_into_base_args() {
    let descriptor = fallback_action_descriptor("app.controller", "importVideoBytesPayload", &[9, 9], "clip.mp4", &serde_json::json!({"streamId": "s1"}));
    assert_eq!(descriptor.controller_id, "app.controller");
    assert_eq!(descriptor.action, "importVideoBytesPayload");
    let args = descriptor.args.unwrap();
    assert_eq!(args.get("streamId").and_then(DslValue::as_str), Some("s1"));
    assert_eq!(args.get("name").and_then(DslValue::as_str), Some("clip.mp4"));
    assert!(args.get("payload").and_then(DslValue::as_str).is_some_and(|payload| payload.starts_with("data:application/octet-stream;base64,")));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn request_media_frames_falls_back_when_ffmpeg_missing_and_payload_given() {
    // 🧪️ Doesn't assert on `ffmpeg_available()` (may or may not be installed in CI/sandboxes) — only
    // exercises the `payload`-bytes-in-hand path, which is deterministic regardless of `ffmpeg`
    // presence when the decoded payload is deliberately not a real video (so even a present `ffmpeg`
    // fails to extract frames from it and this still falls back).
    use base64::Engine;
    let payload = format!("data:video/mp4;base64,{}", base64::engine::general_purpose::STANDARD.encode(b"not a real video"));
    let actions =
        semio_framework_async::block_on(request_media_frames("app.controller", "video/mp4", "importVideoFramePayload", "importVideoDone", "importVideoBytesPayload", 5, 200, 1600, 30.0, Some(&payload), Some(serde_json::json!({"streamId": "s1"}))));
    assert_eq!(actions.len(), 1, "garbage payload never yields real frames: {actions:?}");
    assert_eq!(actions[0].action, "importVideoBytesPayload");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("streamId")).and_then(DslValue::as_str), Some("s1"));
}
