use super::*;
use semio_framework_kernel_media_jobs::{VideoRenderImage, VideoRenderJobOutcome, VideoRenderOp, VideoRenderPath, VideoRenderProgram, VideoRenderRun, VideoRenderScene, MEDIA_VIDEO_RENDER_CAPABILITY, VIDEO_RENDER_PROGRAM_SCHEMA};
use semio_framework_pixels::RasterImage;
use semio_framework_raster::video::decode::decode_avc_mp4;

fn rectangle(width: f64, height: f64) -> VideoRenderPath {
    VideoRenderPath { verbs: "MLLLZ".into(), points: vec![0.0, 0.0, width, 0.0, width, height, 0.0, height] }
}

fn program(images: Vec<VideoRenderImage>, scenes: Vec<VideoRenderScene>, timeline: &[(u32, u32)]) -> VideoRenderProgram {
    VideoRenderProgram {
        schema: VIDEO_RENDER_PROGRAM_SCHEMA.into(),
        width: 32,
        height: 16,
        fps: 10,
        background: [0.0, 0.0, 0.0, 1.0],
        paths: vec![rectangle(1.0, 1.0)],
        images,
        scenes,
        timeline: timeline.iter().map(|(scene, frames)| VideoRenderRun { scene: *scene, frames: *frames }).collect(),
    }
}

fn fill(transform: [f64; 6], color: [f64; 4]) -> VideoRenderOp {
    VideoRenderOp::Fill { path: 0, transform, color }
}

fn no_assets(url: &str) -> Result<Vec<u8>, String> {
    Err(format!("no asset door for {url}"))
}

fn granted() -> Vec<String> {
    vec![MEDIA_VIDEO_RENDER_CAPABILITY.to_string()]
}

fn pixel(rgba: &[u8], x: usize, y: usize) -> [u8; 3] {
    let at = (y * 32 + x) * 4;
    [rgba[at], rgba[at + 1], rgba[at + 2]]
}

fn near(left: [u8; 3], right: [u8; 3]) -> bool {
    (0..3).all(|channel| (i32::from(left[channel]) - i32::from(right[channel])).abs() <= 4)
}

/// ⚖️ LAW: every message exists in English and German with the same placeholders, and a filled message leaves none unfilled.
#[test]
fn every_text_has_both_languages_with_equal_placeholders() {
    let placeholders = |text: &str| -> Vec<String> { text.split('{').skip(1).filter_map(|rest| rest.split('}').next()).map(str::to_string).collect() };
    for kind in VideoRenderText::ALL {
        assert!(!kind.template(false).is_empty() && !kind.template(true).is_empty());
        let (mut english, mut german) = (placeholders(kind.template(false)), placeholders(kind.template(true)));
        english.sort();
        german.sort();
        assert_eq!(english, german, "{kind:?}");
    }
    let values = [("filename", "a.mp4"), ("frames", "3"), ("code", "empty"), ("owner", "p"), ("capability", "c"), ("reason", "r"), ("completed", "1"), ("total", "2")];
    assert_eq!(video_render_text(VideoRenderText::Done, false, &values), "Video a.mp4 exported (3 frames).");
    assert_eq!(video_render_text(VideoRenderText::Progress, true, &values), "Bild 1 von 2");
    assert_eq!(video_render_text(VideoRenderText::Cancelled, false, &[]), "Video export {filename} cancelled.");
}

/// ⚖️ LAW: a path's verbs become the Bézier path they name, and a data URL (base64 or percent-encoded) decodes to the picture it holds.
#[test]
fn paths_and_picture_urls_decode_to_their_content() {
    let path = video_render_path(&VideoRenderPath { verbs: "MLQCZ".into(), points: vec![0.0, 0.0, 1.0, 0.0, 2.0, 1.0, 3.0, 1.0, 4.0, 0.0, 5.0, 1.0, 6.0, 1.0] });
    assert_eq!(path.elements().len(), 5);
    let source = RasterImage { width: 2, height: 2, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] };
    let png = semio_framework_pixels::encode_png(&source).expect("png");
    let encoded = format!("data:image/png;base64,{}", semio_framework_io_base64::base64_standard_encode(&png));
    let percent = format!("data:image/png,{}", png.iter().map(|byte| format!("%{byte:02X}")).collect::<String>());
    for url in [encoded, percent] {
        let picture = decode_video_render_picture(&url, &no_assets, &|| false).expect("picture");
        assert_eq!((picture.width, picture.height, picture.pixels.as_slice()), (2, 2, source.pixels.as_slice()));
    }
    assert_eq!(video_render_picture_bytes("/assets/a.png", &|url| Ok(url.as_bytes().to_vec())), Ok(b"/assets/a.png".to_vec()));
    assert!(decode_video_render_picture("/assets/a.png", &no_assets, &|| false).is_err());
    assert!(decode_video_render_picture("data:image/png;base64,AAAA", &no_assets, &|| false).is_err());
}

/// ⚖️ LAW: the host admits only a plugin that requested the capability and a valid program, folds progress into the ledger at
/// most once per interval, cancels once, and settles every outcome with its sentence.
#[test]
fn jobs_are_admitted_progressed_cancelled_and_settled_through_the_ledger() {
    let valid = || program(vec![], vec![VideoRenderScene { ops: vec![fill([1.0, 0.0, 0.0, 1.0, 0.0, 0.0], [1.0; 4])] }], &[(0, 4)]);
    let mut host = VideoRenderExportHost::default();
    let VideoRenderStart::Refused(outcome) = host.start("a.mp4", "plugin.a", valid(), &[], 0, false) else { panic!("refused without the capability") };
    assert_eq!(outcome.outcome, VideoRenderJobOutcome::Refused { code: "capability".into() });
    assert_eq!(outcome.text, "Video export a.mp4 refused: plugin.a did not request the media.video-render capability.");
    let mut empty = valid();
    empty.timeline.clear();
    let VideoRenderStart::Refused(outcome) = host.start("a.mp4", "plugin.a", empty, &granted(), 0, true) else { panic!("refused when empty") };
    assert_eq!(outcome.outcome, VideoRenderJobOutcome::Refused { code: "empty".into() });
    assert!(outcome.text.contains("ungültig"));
    assert!(!host.is_running());
    let VideoRenderStart::Started { job, shared, announce, .. } = host.start("b.mp4", "plugin.a", valid(), &granted(), 1_000, false) else { panic!("started") };
    assert_eq!((job, announce.as_str()), (3, "Rendering video b.mp4 — progress and cancel in Task Manager."));
    assert_eq!(host.rows().len(), 1);
    assert_eq!((host.rows()[0].frames, host.rows()[0].started_at_ms), (4, 1_000));
    assert_eq!(VideoRenderExportHost::default().shared_progress(1), None);
    let reported = |host: &VideoRenderExportHost| host.rows()[0].completed;
    for done in 1..=2u32 {
        shared_store(&shared, done);
        host.sync_progress(1_000 + u64::from(done) * 10);
    }
    assert_eq!(reported(&host), 1);
    host.sync_progress(1_200);
    assert_eq!(reported(&host), 2);
    shared_store(&shared, 4);
    host.sync_progress(1_201);
    assert_eq!(reported(&host), 4);
    assert!(host.cancel(&video_render_export_task_id(job)));
    assert!(!host.cancel(&video_render_export_task_id(job)) && !host.cancel("export:video#99") && !host.cancel("other"));
    assert!(shared.is_cancelled() && host.rows()[0].cancelling);
    let outcome = host.settle(job, Err(VideoRenderFailure::Cancelled { completed: 2 }), true);
    assert_eq!((outcome.outcome, outcome.text.as_str()), (VideoRenderJobOutcome::Cancelled { completed: 2 }, "Videoexport b.mp4 abgebrochen."));
    assert!(!host.is_running());
    assert!(host.log().count() >= 6);
    let VideoRenderStart::Started { job, .. } = host.start("c.mp4", "plugin.a", valid(), &granted(), 2_000, false) else { panic!("started") };
    let outcome = host.settle(job, Err(VideoRenderFailure::Gpu { reason: "none".into() }), false);
    assert_eq!(outcome.outcome, VideoRenderJobOutcome::Failed { reason: "no wgpu rasterizer: none".into() });
    assert!(outcome.bytes.is_none() && host.rows().is_empty());
}

fn shared_store(shared: &VideoRenderShared, completed: u32) {
    shared.report(completed);
}

/// ⚖️ LAW: rendering a program through the wgpu rasterizer and decoding the MP4 gives back its scenes, frame for frame, with
/// progress reaching the last frame and a cancel stopping the loop. A machine with no wgpu adapter answers `Gpu` and skips the rest.
#[semio_framework_async_macros::async_test]
async fn rendered_programs_round_trip_through_the_framework_encoder_and_decoder() {
    let red = fill([32.0, 0.0, 0.0, 16.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
    let green = fill([16.0, 0.0, 0.0, 16.0, 0.0, 0.0], [0.0, 1.0, 0.0, 1.0]);
    let source = RasterImage { width: 2, height: 2, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] };
    let png = semio_framework_pixels::encode_png(&source).expect("png");
    let url = format!("data:image/png;base64,{}", semio_framework_io_base64::base64_standard_encode(&png));
    let picture = VideoRenderOp::Image { image: 0, crop: [0.0, 0.0, 1.0, 1.0], transform: [32.0, 0.0, 0.0, 16.0, 0.0, 0.0], opacity: 1.0 };
    let program = program(vec![VideoRenderImage { url }], vec![VideoRenderScene { ops: vec![red] }, VideoRenderScene { ops: vec![green] }, VideoRenderScene { ops: vec![picture] }], &[(0, 3), (1, 2), (2, 1)]);
    let shared = VideoRenderShared::new(6);
    let output = match render_video_program(&program, &no_assets, &shared).await {
        Ok(output) => output,
        Err(VideoRenderFailure::Gpu { .. }) => return,
        Err(failure) => panic!("{failure:?}"),
    };
    assert_eq!((output.frames, shared.progress()), (6, (6, 6)));
    let video = decode_avc_mp4(&output.bytes, &mut |_, _| {}, &|| false).expect("decode");
    assert_eq!((video.width, video.height, video.frame_count(), video.pictures.len()), (32, 16, 6, 3));
    assert_eq!(video.frame_picture, vec![0, 0, 0, 1, 1, 2]);
    assert_eq!(video.duration_milliseconds(), 600);
    assert!(near(pixel(video.frame_rgba(0).expect("scene 0"), 8, 8), [255, 0, 0]));
    let second = video.frame_rgba(3).expect("scene 1");
    assert!(near(pixel(second, 8, 8), [0, 255, 0]) && near(pixel(second, 24, 8), [0, 0, 0]));
    let third = video.frame_rgba(5).expect("scene 2");
    assert!(near(pixel(third, 8, 4), [255, 0, 0]) && near(pixel(third, 24, 4), [0, 255, 0]) && near(pixel(third, 8, 12), [0, 0, 255]) && near(pixel(third, 24, 12), [255, 255, 255]));
    let cancelled = VideoRenderShared::new(6);
    cancelled.cancel();
    assert_eq!(render_video_program(&program, &no_assets, &cancelled).await.map(|_| ()), Err(VideoRenderFailure::Cancelled { completed: 0 }));
    let mut broken = program.clone();
    broken.images[0].url = "/missing.png".into();
    assert!(matches!(render_video_program(&broken, &no_assets, &VideoRenderShared::new(6)).await, Err(VideoRenderFailure::Picture { index: 0, .. })));
}
