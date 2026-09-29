use super::*;

/// 🧫️ The language-agnostic fixture both twins answer (`🟦️.ts` beside `🦀️.rs` reads the same file).
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("raster video fixture")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn parameters(case: &serde_json::Value) -> VideoStreamParameters {
    let number = |key: &str| case[key].as_u64().expect("fixture integer") as u32;
    VideoStreamParameters { width: number("width"), height: number("height"), fps: number("fps") }
}

fn rgb(value: &serde_json::Value) -> [u8; 3] {
    [0, 1, 2].map(|index| value[index].as_u64().expect("rgb channel") as u8)
}

fn picture(parameters: &VideoStreamParameters, run: &serde_json::Value) -> Vec<u8> {
    let (width, height) = (parameters.width as usize, parameters.height as usize);
    let mut rgba = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let [r, g, b] = match run["quadrants"].as_array() {
                Some(quadrants) => rgb(&quadrants[usize::from(y >= height / 2) * 2 + usize::from(x >= width / 2)]),
                None => rgb(&run["rgb"]),
            };
            rgba.extend_from_slice(&[r, g, b, 255]);
        }
    }
    rgba
}

fn colours(run: &serde_json::Value) -> Vec<[u8; 3]> {
    run["quadrants"].as_array().map_or_else(|| vec![rgb(&run["rgb"])], |quadrants| quadrants.iter().map(rgb).collect())
}

struct Recording<'a> {
    inner: &'a mut AvcPcmEncoder,
    samples: Vec<(u64, bool)>,
}

impl Recording<'_> {
    fn keep(&mut self, sample: Result<EncodedVideoSample, VideoEncodeError>) -> Result<EncodedVideoSample, VideoEncodeError> {
        let sample = sample?;
        self.samples.push((sample.data.len() as u64, sample.sync));
        Ok(sample)
    }
}

impl VideoEncoder for Recording<'_> {
    fn parameters(&self) -> VideoStreamParameters {
        self.inner.parameters()
    }
    fn configuration(&self) -> &AvcDecoderConfiguration {
        self.inner.configuration()
    }
    fn encode(&mut self, rgba: &[u8]) -> Result<EncodedVideoSample, VideoEncodeError> {
        let sample = self.inner.encode(rgba);
        self.keep(sample)
    }
    fn repeat(&mut self) -> Result<EncodedVideoSample, VideoEncodeError> {
        let sample = self.inner.repeat();
        self.keep(sample)
    }
}

/// ⚖️ LAW: every fixture case encodes to exactly the pinned SPS/PPS, sync pattern, sample sizes, container size and
/// container digest (and, for the small cases, container bytes) — the very bytes `🧪️tests/🎞️ffmpeg-decode` has FFmpeg
/// decode back to the claimed planes.
#[test]
fn every_fixture_case_encodes_to_the_pinned_stream() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expected = &case["expected"];
        let parameters = parameters(case);
        let mut encoder = AvcPcmEncoder::new(parameters).expect("admitted stream");
        let pictures: Vec<(Vec<u8>, u32)> = case["runs"].as_array().expect("runs").iter().map(|run| (picture(&parameters, run), run["frames"].as_u64().expect("frames") as u32)).collect();
        let runs: Vec<VideoFrameRun<'_>> = pictures.iter().map(|(rgba, frames)| VideoFrameRun { rgba, frames: *frames }).collect();
        let mut reported = Vec::new();
        let mut recording = Recording { inner: &mut encoder, samples: Vec::new() };
        let mp4 = encode_video_runs(&mut recording, &runs, &mut |done, total| reported.push((done, total)), &|| false).expect("encoded");
        let samples = recording.samples.clone();
        let frames = samples.len() as u32;
        let numbers = |key: &str| expected[key].as_array().expect(key).iter().map(|value| value.as_u64().expect("integer")).collect::<Vec<_>>();
        assert_eq!(u64::from(frames), expected["frameCount"].as_u64().expect("frameCount"), "{id}: frame count");
        assert_eq!(samples.iter().enumerate().filter(|(_, (_, sync))| *sync).map(|(index, _)| index as u64 + 1).collect::<Vec<_>>(), numbers("syncSamples"), "{id}: IDR per run, P per repeat");
        assert_eq!(reported.last().copied(), Some((frames, frames)), "{id}: progress ends at total");
        assert_eq!(u64::from(video_duration_milliseconds(frames, parameters.fps)), expected["durationMs"].as_u64().expect("durationMs"), "{id}: duration");
        assert_eq!(u64::from(parameters.macroblock_width()), expected["macroblocks"]["width"].as_u64().expect("mb width"), "{id}: macroblock width");
        assert_eq!(u64::from(parameters.macroblock_height()), expected["macroblocks"]["height"].as_u64().expect("mb height"), "{id}: macroblock height");
        assert_eq!(u64::from(parameters.level_idc()), expected["levelIdc"].as_u64().expect("levelIdc"), "{id}: level");
        assert_eq!(avc_codec_string(encoder.configuration()), expected["codec"].as_str().expect("codec"), "{id}: codec string");
        assert_eq!(hex(&encoder.configuration().sps), expected["spsHex"].as_str().expect("sps"), "{id}: SPS bytes");
        assert_eq!(hex(&encoder.configuration().pps), expected["ppsHex"].as_str().expect("pps"), "{id}: PPS bytes");
        assert_eq!(samples.iter().map(|(size, _)| *size).collect::<Vec<_>>(), numbers("sampleSizes"), "{id}: sample sizes");
        assert_eq!(mp4.len() as u64, expected["mp4Bytes"].as_u64().expect("mp4Bytes"), "{id}: container size");
        assert_eq!(semio_framework_hash::sha256_hex(&mp4), expected["mp4Sha256"].as_str().expect("mp4Sha256"), "{id}: container digest");
        if let Some(bytes) = expected["mp4Hex"].as_str() {
            assert_eq!(hex(&mp4), bytes, "{id}: container bytes");
        }
        for (run, yuv) in case["runs"].as_array().expect("runs").iter().zip(expected["yuv"].as_array().expect("yuv")) {
            let claimed: Vec<Vec<i64>> = colours(run).into_iter().map(|[r, g, b]| [bt601_luma(r.into(), g.into(), b.into()), bt601_cb(r.into(), g.into(), b.into()), bt601_cr(r.into(), g.into(), b.into())].map(i64::from).to_vec()).collect();
            let pinned: Vec<Vec<i64>> = yuv.as_array().expect("yuv run").iter().map(|row| row.as_array().expect("yuv row").iter().map(|value| value.as_i64().expect("yuv")).collect()).collect();
            assert_eq!(claimed, pinned, "{id}: BT.601 samples");
        }
    }
}

/// ⚖️ LAW: the tier refuses every stream no 4:2:0 H.264 picture of it can carry, with the fixture's error.
#[test]
fn refused_streams_answer_the_declared_error() {
    for case in fixture()["refusals"].as_array().expect("refusals") {
        let error = AvcPcmEncoder::new(parameters(case)).err().expect("refused stream");
        assert_eq!(error.code(), case["error"].as_str().expect("error"), "{}", case["id"]);
    }
}

/// ⚖️ LAW: a repeat needs a picture first; cancellation stops before the next frame and no container is written; a frame
/// of the wrong size is refused rather than read out of bounds.
#[test]
fn repeat_cancellation_and_frame_size_are_enforced() {
    let parameters = VideoStreamParameters { width: 32, height: 16, fps: 30 };
    let mut encoder = AvcPcmEncoder::new(parameters).expect("admitted stream");
    assert_eq!(encoder.repeat(), Err(VideoEncodeError::NothingToRepeat));
    let rgba = [0u8, 0, 0, 255].repeat(32 * 16);
    let runs = [VideoFrameRun { rgba: &rgba, frames: 3 }];
    let done = std::cell::Cell::new(0);
    let result = encode_video_runs(&mut encoder, &runs, &mut |frames, _| done.set(frames), &|| done.get() >= 1);
    assert_eq!(result, Err(VideoEncodeError::Cancelled));
    assert_eq!(done.get(), 1);
    assert_eq!(encoder.encode(&rgba[4..]), Err(VideoEncodeError::FrameBytes { expected: rgba.len(), actual: rgba.len() - 4 }));
    assert_eq!(write_avc_mp4(&parameters, encoder.configuration(), &[]), Err(VideoEncodeError::Empty));
}
