use super::*;
use crate::video::{encode_video_runs, AvcPcmEncoder, VideoFrameRun, VideoStreamParameters};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).expect("raster video fixture")
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect()
}

fn rgb(value: &serde_json::Value) -> [u8; 3] {
    let channel = |index: usize| value[index].as_u64().expect("channel") as u8;
    [channel(0), channel(1), channel(2)]
}

fn expected_pixel(run: &serde_json::Value, width: usize, height: usize, x: usize, y: usize) -> [u8; 3] {
    match run["quadrants"].as_array() {
        Some(quadrants) => rgb(&quadrants[usize::from(y >= height / 2) * 2 + usize::from(x >= width / 2)]),
        None => rgb(&run["rgb"]),
    }
}

fn close(left: &[u8], right: [u8; 3], tolerance: i32) -> bool {
    (0..3).all(|channel| (i32::from(left[channel]) - i32::from(right[channel])).abs() <= tolerance)
}

fn decode(bytes: &[u8]) -> Result<DecodedVideo, VideoDecodeError> {
    decode_avc_mp4(bytes, &mut |_, _| {}, &|| false)
}

/// ⚖️ LAW: every pinned fixture stream (bytes the encoder is separately pinned to) decodes to the pictures its runs describe.
#[test]
fn pinned_fixture_streams_decode_to_their_source_pictures() {
    for case in fixture()["cases"].as_array().expect("cases") {
        let (width, height) = (case["width"].as_u64().expect("width") as usize, case["height"].as_u64().expect("height") as usize);
        let video = decode(&unhex(case["expected"]["mp4Hex"].as_str().expect("mp4Hex"))).expect(case["id"].as_str().expect("id"));
        assert_eq!((video.width as usize, video.height as usize), (width, height), "{}", case["id"]);
        assert_eq!(video.frame_count() as u64, case["expected"]["frameCount"].as_u64().expect("frameCount"), "{}", case["id"]);
        assert_eq!(video.duration_milliseconds(), case["expected"]["durationMs"].as_u64().expect("durationMs"), "{}", case["id"]);
        let mut frame = 0;
        for run in case["runs"].as_array().expect("runs") {
            let first = video.frame_picture[frame];
            for _ in 0..run["frames"].as_u64().expect("frames") {
                assert_eq!(video.frame_picture[frame], first, "{} repeats share their reference", case["id"]);
                frame += 1;
            }
            let rgba = video.frame_rgba(frame - 1).expect("picture");
            for (x, y) in [(0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1), (width / 2, height / 2)] {
                assert!(close(&rgba[(y * width + x) * 4..], expected_pixel(run, width, height, x, y), 4), "{} pixel {x},{y}", case["id"]);
            }
        }
    }
}

/// ⚖️ LAW: encode → decode keeps a smooth picture within the 4:2:0 quantisation bound, shares repeated frames and keeps timing.
#[test]
fn encoded_runs_round_trip_with_progress() {
    let parameters = VideoStreamParameters { width: 64, height: 32, fps: 24 };
    let gradient = |shift: usize| -> Vec<u8> { (0..32usize).flat_map(|y| (0..64usize).flat_map(move |x| [(x * 3 + shift) as u8, (y * 8) as u8, 128, 255])).collect() };
    let (first, second) = (gradient(0), gradient(20));
    let mut encoder = AvcPcmEncoder::new(parameters).expect("encoder");
    let mut reported = Vec::new();
    let bytes = encode_video_runs(&mut encoder, &[VideoFrameRun { rgba: &first, frames: 2 }, VideoFrameRun { rgba: &second, frames: 3 }], &mut |done, total| reported.push((done, total)), &|| false).expect("encode");
    assert_eq!(reported.last(), Some(&(5, 5)));
    let mut seen = Vec::new();
    let video = decode_avc_mp4(&bytes, &mut |done, total| seen.push((done, total)), &|| false).expect("decode");
    assert_eq!(seen, (1..=5).map(|done| (done, 5)).collect::<Vec<_>>());
    assert_eq!((video.width, video.height, video.frame_count(), video.pictures.len()), (64, 32, 5, 2));
    assert_eq!(video.frame_picture, vec![0, 0, 1, 1, 1]);
    assert_eq!(video.duration_milliseconds(), 208);
    assert_eq!((video.frame_at_milliseconds(0), video.frame_at_milliseconds(100), video.frame_at_milliseconds(10_000)), (0, 2, 4));
    for (frame, source) in [(0, &first), (4, &second)] {
        let rgba = video.frame_rgba(frame).expect("picture");
        for pixel in 0..64 * 32 {
            assert!(close(&rgba[pixel * 4..], [source[pixel * 4], source[pixel * 4 + 1], source[pixel * 4 + 2]], 12), "frame {frame} pixel {pixel}");
        }
    }
}

/// ⚖️ LAW: a cancelled decode stops with `Cancelled`, and data that is not a stream this tier wrote is refused with a typed error.
#[test]
fn cancellation_and_refusals_are_typed() {
    let parameters = VideoStreamParameters { width: 32, height: 16, fps: 30 };
    let mut encoder = AvcPcmEncoder::new(parameters).expect("encoder");
    let picture = vec![200u8; 32 * 16 * 4];
    let bytes = encode_video_runs(&mut encoder, &[VideoFrameRun { rgba: &picture, frames: 4 }], &mut |_, _| {}, &|| false).expect("encode");
    assert_eq!(decode_avc_mp4(&bytes, &mut |_, _| {}, &|| true), Err(VideoDecodeError::Cancelled));
    assert_eq!(decode(b"definitely not an mp4 file").map_err(|error| error.code()), Err("truncated"));
    assert_eq!(decode(&[0, 0, 0, 16, b'f', b't', b'y', b'p', b'i', b's', b'o', b'm', 0, 0, 0, 0]).map_err(|error| error.code()), Err("notMp4"));
    assert_eq!(decode(&bytes[..bytes.len() - 3]).map_err(|error| error.code()), Err("truncated"));
    let mut hostile = bytes.clone();
    let stsz = hostile.windows(4).position(|window| window == b"stsz").expect("stsz box");
    hostile[stsz + 12..stsz + 16].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(decode(&hostile).map_err(|error| error.code()), Err("malformed"));
}
