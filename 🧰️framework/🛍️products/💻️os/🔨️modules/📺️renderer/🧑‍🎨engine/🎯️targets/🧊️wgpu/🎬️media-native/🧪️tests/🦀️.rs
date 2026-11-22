use super::*;
use semio_framework_raster::video::{encode_video_runs, AvcPcmEncoder, VideoFrameRun, VideoStreamParameters};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🧫️fixtures/🔣️.json")).expect("media fixture")
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect()
}

fn decode(bytes: &[u8]) -> Result<DecodedAudio, WavError> {
    decode_wav(bytes, &mut |_, _| {}, &|| false)
}

fn wav_with_tag(tag: u16, data: &[u8]) -> Vec<u8> {
    let mut fmt = Vec::new();
    for value in [tag, 1] {
        fmt.extend_from_slice(&value.to_le_bytes());
    }
    fmt.extend_from_slice(&8000u32.to_le_bytes());
    fmt.extend_from_slice(&8000u32.to_le_bytes());
    fmt.extend_from_slice(&1u16.to_le_bytes());
    fmt.extend_from_slice(&8u16.to_le_bytes());
    let mut body = b"WAVEfmt ".to_vec();
    body.extend_from_slice(&(fmt.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&(data.len() as u32).to_le_bytes());
    body.extend_from_slice(data);
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

/// ⚖️ LAW: every fixture WAV (written by Python's own `wave` module or by the generator) decodes to exactly the pinned samples.
#[test]
fn fixture_wav_files_decode_to_their_pinned_samples() {
    for case in fixture()["wav"].as_array().expect("wav") {
        let audio = decode(&unhex(case["hex"].as_str().expect("hex"))).expect(case["id"].as_str().expect("id"));
        assert_eq!((u64::from(audio.sample_rate), u64::from(audio.channels)), (case["sampleRate"].as_u64().expect("rate"), case["channels"].as_u64().expect("channels")), "{}", case["id"]);
        let expected: Vec<f32> = case["samples"].as_array().expect("samples").iter().map(|value| value.as_f64().expect("sample") as f32).collect();
        assert_eq!(audio.samples, expected, "{}", case["id"]);
    }
}

/// ⚖️ LAW: A-law and µ-law decode every byte to the value Python's `audioop` gives, and the refusals answer the fixture's codes.
#[test]
fn g711_tables_match_the_third_party_oracle_and_bad_files_are_refused() {
    let fixture = fixture();
    for (tag, name) in [(6, "alaw"), (7, "mulaw")] {
        let audio = decode(&wav_with_tag(tag, &(0..=255).collect::<Vec<u8>>())).expect(name);
        let expected: Vec<f32> = fixture["g711"][name].as_array().expect("table").iter().map(|value| value.as_i64().expect("value") as f32 / 32768.0).collect();
        assert_eq!(audio.samples, expected, "{name}");
    }
    for case in fixture["wavRefusals"].as_array().expect("refusals") {
        assert_eq!(decode(&unhex(case["hex"].as_str().expect("hex"))).map_err(|error| error.code()), Err(case["code"].as_str().expect("code")), "{}", case["id"]);
    }
}

/// ⚖️ LAW: encode → decode is the identity on 16-bit samples, progress reaches the end and a cancel stops the decode.
#[test]
fn pcm16_round_trips_with_progress_and_cancellation() {
    let samples: Vec<f32> = (0..400_000).map(|index| f32::from(((index * 37) % 65_536 - 32_768) as i16) / 32768.0).collect();
    let audio = DecodedAudio { sample_rate: 44_100, channels: 2, samples };
    let bytes = encode_wav_pcm16(&audio);
    let mut reported = Vec::new();
    let decoded = decode_wav(&bytes, &mut |done, total| reported.push((done, total)), &|| false).expect("decode");
    assert_eq!(decoded, audio);
    assert_eq!(reported.last().map(|(done, total)| done == total), Some(true));
    assert!(reported.windows(2).all(|pair| pair[0].0 < pair[1].0) && reported.len() >= 3);
    assert_eq!((audio.frames(), audio.duration_milliseconds()), (200_000, 4535));
    assert_eq!(decode_wav(&bytes, &mut |_, _| {}, &|| true), Err(WavError::Cancelled));
    assert_eq!(audio.read(199_999, 10).len(), 2);
}

/// ⚖️ LAW: the transport rules answer the window kit's own fixture, the one its TypeScript twin reads (keys, seeks, cut ranges, clock text).
#[test]
fn transport_rules_match_the_window_kit_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/⏱️time-and-keys/🧫️fixture.json")).expect("window kit fixture");
    let command = |value: &serde_json::Value| -> Option<MediaKeyCommand> {
        match value["kind"].as_str()? {
            "toggle" => Some(MediaKeyCommand::Toggle),
            "seek" => Some(MediaKeyCommand::Seek { delta_ms: value["deltaMs"].as_i64()? }),
            _ => Some(MediaKeyCommand::SeekTo { position_ms: value["positionMs"].as_u64()? }),
        }
    };
    for case in fixture["keys"].as_array().expect("keys") {
        let key = MediaKeyInput { key: case["key"].as_str().expect("key"), shift: case["shift"].as_bool().expect("shift"), target_tag: case["target"].as_str().expect("target"), target_type: case["type"].as_str(), ..Default::default() };
        assert_eq!(media_key_command(&key, case["duration"].as_u64()), command(&case["expect"]), "{case}");
    }
    for case in fixture["seek"].as_array().expect("seek") {
        assert_eq!(Some(seeked_media_position(command(&case["command"]).expect("command"), case["position"].as_u64().expect("position"), case["duration"].as_u64())), case["expect"].as_u64(), "{case}");
    }
    let declared = &fixture["trim"]["declared"];
    let trim = MediaTrimRate { units_per_second_numerator: declared["unitsPerSecondNumerator"].as_u64().expect("num"), units_per_second_denominator: declared["unitsPerSecondDenominator"].as_u64().expect("den"), total_units: declared["totalUnits"].as_u64().expect("total") };
    for case in fixture["trim"]["cases"].as_array().expect("cases") {
        assert_eq!(trim_value_for_selection(&trim, case["startMs"].as_u64().expect("start"), case["endMs"].as_u64().expect("end")).as_deref(), case["expect"].as_str(), "{case}");
    }
    for case in fixture["time"].as_array().expect("time") {
        let ms = case["ms"].as_u64().expect("ms");
        assert_eq!((format_media_time(ms, false), format_media_time(ms, true)), (case["en"].as_str().expect("en").to_string(), case["de"].as_str().expect("de").to_string()));
    }
}

/// ⚖️ LAW: a transport plays, ends at the duration, restarts from the end, clamps seeks and orders its selection.
#[test]
fn transport_state_follows_the_clock() {
    let mut transport = MediaTransport::new(Some(1000));
    assert_eq!(transport.advance(100), MediaTransportEvent::None);
    transport.toggle();
    assert_eq!((transport.advance(400), transport.position_ms), (MediaTransportEvent::None, 400));
    assert_eq!((transport.advance(900), transport.position_ms, transport.playing), (MediaTransportEvent::Ended, 1000, false));
    transport.play();
    assert_eq!((transport.position_ms, transport.playing), (0, true));
    transport.apply(MediaKeyCommand::Seek { delta_ms: 5000 });
    assert_eq!(transport.position_ms, 1000);
    transport.seek(5000);
    transport.select(800, 200);
    assert_eq!((transport.position_ms, transport.selection), (1000, Some((200, 800))));
}

fn first_party_mp4() -> Vec<u8> {
    let parameters = VideoStreamParameters { width: 32, height: 16, fps: 10 };
    let (dark, light) = (vec![20u8; 32 * 16 * 4], vec![230u8; 32 * 16 * 4]);
    encode_video_runs(&mut AvcPcmEncoder::new(parameters).expect("encoder"), &[VideoFrameRun { rgba: &dark, frames: 5 }, VideoFrameRun { rgba: &light, frames: 5 }], &mut |_, _| {}, &|| false).expect("mp4")
}

/// ⚖️ LAW: audio and the framework's own MP4 load by signature, play under the transport (audio by pulling blocks, video by the clock) and
/// everything else is refused with a typed reason and listed as browser-only.
#[test]
fn native_players_play_audio_and_first_party_video_and_refuse_the_rest() {
    let audio = DecodedAudio { sample_rate: 1000, channels: 1, samples: (0..2500).map(|index| index as f32 / 4096.0).collect() };
    let mut player = NativeMediaPlayer::new(load_native_media(&encode_wav_pcm16(&audio), &mut |_, _| {}, &|| false).expect("wav"));
    assert_eq!(player.transport.duration_ms, Some(2500));
    assert!(player.pull_audio(100).0.is_empty());
    player.transport.play();
    let (block, event) = player.pull_audio(1000);
    assert_eq!((block.len(), event, player.transport.position_ms), (1000, MediaTransportEvent::None, 1000));
    player.seek(2400);
    let (block, event) = player.pull_audio(1000);
    assert_eq!((block.len(), event, player.transport.position_ms, player.transport.playing), (100, MediaTransportEvent::Ended, 2500, false));
    let mut video = NativeMediaPlayer::new(load_native_media(&first_party_mp4(), &mut |_, _| {}, &|| false).expect("mp4"));
    assert_eq!((video.transport.duration_ms, video.video_frame().map(|frame| (frame.index, frame.width, frame.height))), (Some(1000), Some((0, 32, 16))));
    video.transport.play();
    assert_eq!(video.tick(600), MediaTransportEvent::None);
    let frame = video.video_frame().expect("frame");
    assert_eq!((frame.index, frame.rgba[0] > 200), (6, true));
    assert_eq!(video.tick(600), MediaTransportEvent::Ended);
    assert_eq!(load_native_media(b"OggS....", &mut |_, _| {}, &|| false), Err(NativeMediaFault::Unrecognised));
    assert_eq!(load_native_media(&first_party_mp4(), &mut |_, _| {}, &|| true), Err(NativeMediaFault::Cancelled));
    assert_eq!(native_media_support("audio/WAV; charset=x"), NativeMediaSupport::Ready(NativeMediaFormat::Wav));
    assert_eq!(native_media_support("video/mp4"), NativeMediaSupport::Ready(NativeMediaFormat::Mp4));
    assert!(matches!(native_media_support("audio/mpeg"), NativeMediaSupport::Unsupported { .. }) && !NATIVE_MEDIA_BROWSER_ONLY.is_empty());
}
