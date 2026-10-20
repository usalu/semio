//! 📥️ Frame ingest tests of the staged reconstruction engine: the blur gate, the stride and frame cap and the video push counts with the flat and checker frame fixtures. See [`super`].

use super::*;
use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::images as remodeling_image;
use semio_s_artifact_remodel_remodeling_video::editor::remodeling::engine::video as remodeling_video;

// #region 🔖️TestFixtures
/// 🎨️ Flat mid-gray `w x h` frame with zero gradient energy — a stand-in for a heavily blurred/
/// defocused capture, deliberately below any sensible relative-sharpness threshold.
fn flat_frame(w: u32, h: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(w, h);
    for px in img.data.chunks_mut(4) {
        px[0] = 128;
        px[1] = 128;
        px[2] = 128;
        px[3] = 255;
    }
    img
}

/// 🏁️ High-contrast `cell`-pixel checkerboard — strong Scharr gradient energy everywhere, a stand-in
/// for a crisp, well-focused frame.
pub(super) fn checker_frame(w: u32, h: u32, cell: u32) -> remodeling_image::ImageRgba8 {
    let mut img = remodeling_image::ImageRgba8::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
            let v = if on { 235u8 } else { 20u8 };
            let idx = ((y * w + x) * 4) as usize;
            img.data[idx] = v;
            img.data[idx + 1] = v;
            img.data[idx + 2] = v;
            img.data[idx + 3] = 255;
        }
    }
    img
}
// #endregion 🔖️TestFixtures

// #region 🔖️InputTests
#[test]
fn push_frame_blur_gate_rejects_planted_blurred_frame() {
    let mut source = FrameSource::new(IngestParams::default());
    let mut outcomes = Vec::new();
    for i in 0..10u32 {
        let img = if i == 5 { flat_frame(32, 32) } else { checker_frame(32, 32, 4) };
        outcomes.push(source.push_frame(i, img, f64::from(i) * 33.3));
    }
    assert_eq!(outcomes[5], FrameAcceptance::RejectedBlur, "planted flat frame at index 5 must be rejected as blur, got {:?}", outcomes[5]);
    for (i, outcome) in outcomes.iter().enumerate() {
        if i != 5 {
            assert_eq!(*outcome, FrameAcceptance::Accepted, "sharp frame at index {i} should be accepted, got {outcome:?}");
        }
    }
    assert_eq!(source.accepted_count(), 9);
}

#[test]
fn push_frame_stride_and_max_frames_sample() {
    let mut source = FrameSource::new(IngestParams { stride: 2, max_frames: 3, min_sharpness: 0.0, rolling_window: 15 });
    let mut accepted = 0;
    for i in 0..10u32 {
        if source.push_frame(i, checker_frame(16, 16, 4), f64::from(i)) == FrameAcceptance::Accepted {
            accepted += 1;
        }
    }
    assert_eq!(accepted, 3, "stride 2 + max_frames 3 should accept exactly 3 of 10 offered frames");
    assert_eq!(source.accepted_count(), 3);
}

#[test]
fn push_video_blur_gate_reports_counts() {
    let frames: Vec<Vec<u8>> = (0..9u32)
        .map(|i| {
            let img = if i == 4 { flat_frame(24, 24) } else { checker_frame(24, 24, 3) };
            remodeling_image::encode_jpeg(&img, 90)
        })
        .collect();
    let bytes = remodeling_video::container_providers::mp4::write_mjpeg(&frames, 10.0);
    let mut source = FrameSource::new(IngestParams::default());
    let opts = remodeling_video::VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
    let report = source.push_video(&bytes, &opts).expect("mjpeg mp4 push_video should succeed");
    assert_eq!(report.frames_extracted, 9);
    assert_eq!(report.frames_accepted, 8);
    assert_eq!(report.frames_rejected_blur, 1);
    assert_eq!(report.frames_rejected_sampling, 0);
    assert_eq!(source.accepted_count(), 8);
}
// #endregion 🔖️InputTests
