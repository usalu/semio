use super::*;
use semio_framework_value::DslValue;
use semio_framework_raster::video::{encode_video_runs, AvcPcmEncoder, VideoFrameRun, VideoStreamParameters};
use std::sync::{Arc, Mutex};

fn fixtures() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🧫️fixtures/🖼️presenter.json")).expect("presenter fixture")
}

fn audio_fixtures() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🧫️fixtures/🔣️.json")).expect("media fixture")
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect()
}

fn rect(value: &serde_json::Value) -> SlotRect {
    let n = |index: usize| value[index].as_f64().expect("coordinate") as f32;
    SlotRect::new(n(0), n(1), n(2), n(3))
}

struct Shared(Arc<Mutex<RecordingAudioOutput>>);

impl AudioOutputPort for Shared {
    fn format(&self) -> AudioFormat { self.0.lock().unwrap().format() }
    fn write(&mut self, samples: &[f32]) -> Result<usize, AudioOutputError> { self.0.lock().unwrap().write(samples) }
    fn queued_frames(&self) -> usize { self.0.lock().unwrap().queued_frames() }
    fn start(&mut self) -> Result<(), AudioOutputError> { self.0.lock().unwrap().start() }
    fn pause(&mut self) -> Result<(), AudioOutputError> { self.0.lock().unwrap().pause() }
    fn flush(&mut self) -> Result<(), AudioOutputError> { self.0.lock().unwrap().flush() }
    fn stats(&self) -> AudioOutputStats { self.0.lock().unwrap().stats() }
    fn close(&mut self) { self.0.lock().unwrap().close() }
}

#[derive(Default)]
struct Uploads(Vec<(String, u32, u32, u8)>);

impl RasterUploadPort for Uploads {
    fn upload(&mut self, _surface: &str, key: &str, width: u32, height: u32, rgba: &[u8]) -> Result<(), RasterUploadRefusal> {
        self.0.push((key.to_owned(), width, height, rgba[0]));
        Ok(())
    }
}

fn new_presenter(capacity: usize) -> (MediaSlotPresenter, Arc<Mutex<Option<Arc<Mutex<RecordingAudioOutput>>>>>) {
    let opened = Arc::new(Mutex::new(None));
    let slot = opened.clone();
    let opener: AudioOutputOpener = Box::new(move |format| {
        let device = Arc::new(Mutex::new(RecordingAudioOutput::new(format, capacity)));
        *slot.lock().unwrap() = Some(device.clone());
        Box::new(Shared(device))
    });
    (MediaSlotPresenter::new(opener, true), opened)
}

fn props(overrides: serde_json::Value) -> Result<MediaSlotProps, String> {
    let mut value = fixtures()["base"].clone();
    for (key, replacement) in overrides.as_object().expect("overrides") {
        value[key] = replacement.clone();
    }
    parse_slot_props(&DslValue::from(value))
}

fn descriptor(token: &str, rect: SlotRect, props: Result<MediaSlotProps, String>) -> SlotDescriptor {
    SlotDescriptor { token: token.to_owned(), rect, occluded: false, props }
}

fn device(opened: &Arc<Mutex<Option<Arc<Mutex<RecordingAudioOutput>>>>>) -> Arc<Mutex<RecordingAudioOutput>> {
    opened.lock().unwrap().clone().expect("audio opened")
}

fn first_party_mp4() -> Vec<u8> {
    let parameters = VideoStreamParameters { width: 32, height: 16, fps: 10 };
    let (dark, light) = (vec![20u8; 32 * 16 * 4], vec![230u8; 32 * 16 * 4]);
    encode_video_runs(&mut AvcPcmEncoder::new(parameters).expect("encoder"), &[VideoFrameRun { rgba: &dark, frames: 5 }, VideoFrameRun { rgba: &light, frames: 5 }], &mut |_, _| {}, &|| false).expect("mp4")
}

fn ramp(frames: usize) -> DecodedAudio {
    DecodedAudio { sample_rate: 1000, channels: 1, samples: (0..frames).map(|index| (index % 2000) as f32 / 4096.0).collect() }
}

/// ⚖️ LAW: the controls of a ready audio slot are exactly the overlay's set, in its order, with the pinned rectangles, roles, labels and spoken values,
/// in both languages; the cut button dispatches the window kit's `start-end` value.
#[test]
fn ready_audio_slot_exposes_the_overlay_control_set() {
    let fixture = fixtures();
    let (mut presenter, _) = new_presenter(10_000);
    presenter.reconcile(vec![descriptor("a", rect(&fixture["slotRect"]), props(serde_json::json!({})))]);
    assert_eq!(presenter.awaiting_bytes(), vec!["a".to_string()]);
    presenter.provide_bytes("a", &encode_wav_pcm16(&ramp(2500)));
    let kind_name = |kind: ControlKind| match kind {
        ControlKind::Root => "root", ControlKind::PlayPause => "playPause", ControlKind::Seek => "seek", ControlKind::Position => "position", ControlKind::Duration => "duration",
        ControlKind::SelectionStart => "selectionStart", ControlKind::SelectionEnd => "selectionEnd", ControlKind::Cut => "cut", ControlKind::Status => "status", ControlKind::Progress => "progress", ControlKind::CancelLoad => "cancel",
    };
    let role_name = |role: ControlRole| match role {
        ControlRole::Group => "group", ControlRole::Button => "button", ControlRole::Slider => "slider", ControlRole::Output => "output", ControlRole::Text => "text", ControlRole::Alert => "alert", ControlRole::Status => "status", ControlRole::ProgressBar => "progressbar",
    };
    let controls = presenter.controls("a");
    let actual: Vec<(String, String, SlotRect, String, Option<String>)> = controls.iter().map(|control| (kind_name(control.kind).to_owned(), role_name(control.role).to_owned(), control.rect, control.label.clone(), control.value_text.clone())).collect();
    let expected: Vec<(String, String, SlotRect, String, Option<String>)> = fixture["audioReadyControls"].as_array().expect("controls").iter().map(|row| (row[0].as_str().unwrap().to_owned(), row[1].as_str().unwrap().to_owned(), rect(&row[2]), row[3].as_str().unwrap().to_owned(), row[4].as_str().map(str::to_owned))).collect();
    assert_eq!(actual, expected);
    let text = |kind: ControlKind| controls.iter().find(|control| control.kind == kind).map(|control| control.text.clone());
    assert_eq!((text(ControlKind::Position), text(ControlKind::Duration)), (fixture["audioReadyTexts"]["position"].as_str().map(str::to_owned), fixture["audioReadyTexts"]["duration"].as_str().map(str::to_owned)));
    assert_eq!(controls[0].shortcuts, Some("Space ArrowLeft ArrowRight Home End"));
    assert_eq!(presenter.cut("a").map(|cut| (cut.action_id, cut.revision, cut.value)), Some(("media.cut".into(), "7".into(), fixture["cutAfterSelection"]["value"].as_str().unwrap().into())));
    let mut german = fixtures()["base"].clone();
    german["locale"] = "de".into();
    german["labels"] = fixture["labels"]["de"].clone();
    let (mut presenter, _) = new_presenter(10_000);
    presenter.reconcile(vec![descriptor("a", rect(&fixture["slotRect"]), parse_slot_props(&DslValue::from(german)))]);
    presenter.provide_bytes("a", &encode_wav_pcm16(&ramp(2500)));
    let labels: Vec<String> = presenter.controls("a").into_iter().map(|control| control.label).collect();
    assert_eq!(serde_json::json!(labels), fixture["germanControlLabels"]);
    assert_eq!(presenter.controls("a")[0].lang, "de");
}

/// ⚖️ LAW: props that break the contract answer the locale's invalid-contract text; unsupported and cancelled states show their detail.
#[test]
fn invalid_unsupported_loading_and_cancelled_states_read_like_the_overlay() {
    let fixture = fixtures();
    let mut broken = fixture["base"].clone();
    broken["kind"] = "film".into();
    assert_eq!(parse_slot_props(&DslValue::from(broken.clone())).err().as_deref(), fixture["invalid"]["en"].as_str());
    broken["locale"] = "de".into();
    assert_eq!(parse_slot_props(&DslValue::from(broken)).err().as_deref(), fixture["invalid"]["de"].as_str());
    assert_eq!(parse_slot_props(&DslValue::from(serde_json::json!({}))).err().as_deref(), fixture["invalid"]["none"].as_str());
    let (mut presenter, _) = new_presenter(100);
    presenter.reconcile(vec![descriptor("mp3", rect(&fixture["slotRect"]), props(serde_json::json!({"mediaType": "audio/mpeg"}))), descriptor("busy", rect(&fixture["slotRect"]), props(serde_json::json!({})))]);
    assert!(matches!(presenter.load_state("mp3"), Some(SlotLoad::Unsupported(reason)) if reason.contains("browser only")));
    assert_eq!(presenter.controls("mp3")[0].role, ControlRole::Status);
    presenter.transfer_progress("busy", 5, 10, "");
    let loading = presenter.controls("busy");
    assert_eq!(loading.iter().map(|control| control.kind).collect::<Vec<_>>(), vec![ControlKind::Status, ControlKind::Progress, ControlKind::CancelLoad]);
    assert_eq!((loading[1].value, loading[1].max, loading[1].label.as_str(), loading[0].text.as_str()), (Some(5), 10, "Loading progress", "Loading media…"));
    let flag = presenter.cancel_flag("busy").expect("flag");
    presenter.activate("busy", ControlKind::CancelLoad);
    assert!(flag.load(std::sync::atomic::Ordering::Relaxed));
    assert_eq!(presenter.controls("busy")[0].text, "Cancelled");
    presenter.provide_bytes("busy", &encode_wav_pcm16(&ramp(100)));
    assert_eq!(presenter.load_state("busy"), Some(&SlotLoad::Cancelled));
}

/// ⚖️ LAW: playing the Python `wave` fixture hands the recording output exactly the decoded samples, in order, ends the transport once they
/// have played, and leaves no sample unplayed or invented.
#[test]
fn audio_output_receives_exactly_the_samples_the_oracle_fixture_decodes_to() {
    let audio = audio_fixtures();
    let case = audio["wav"].as_array().expect("wav").iter().find(|case| case["id"] == "pcm16-mono-from-python-wave").expect("case");
    let expected: Vec<f32> = case["samples"].as_array().unwrap().iter().map(|sample| sample.as_f64().unwrap() as f32).collect();
    let (mut presenter, opened) = new_presenter(1000);
    presenter.reconcile(vec![descriptor("a", SlotRect::new(0.0, 0.0, 400.0, 200.0), props(serde_json::json!({"durationMs": 1, "selectionStartMs": null, "selectionEndMs": null, "trim": null})))]);
    presenter.provide_bytes("a", &unhex(case["hex"].as_str().unwrap()));
    let mut uploads = Uploads::default();
    presenter.toggle("a");
    let mut ended = false;
    for _ in 0..4 {
        let tick = presenter.tick("a", 16, &mut uploads);
        ended |= tick.ended;
        device(&opened).lock().unwrap().drain(8);
    }
    let recording = device(&opened);
    let recording = recording.lock().unwrap();
    assert_eq!((recording.played.clone(), recording.stats().submitted_frames, recording.stats().played_frames), (expected, 6, 6));
    assert!(ended && !presenter.playing("a") && recording.events.contains(&"start") && recording.events.contains(&"pause"));
    assert!(uploads.0.is_empty());
}

/// ⚖️ LAW: a device that takes only a few frames at a time loses nothing (the unwritten tail is carried to the next tick), a seek flushes the
/// queue, a pause pauses the device, and closing the slot closes the stream.
#[test]
fn back_pressure_loses_no_samples_and_seek_flushes_the_queue() {
    let (mut presenter, opened) = new_presenter(40);
    presenter.reconcile(vec![descriptor("a", SlotRect::new(0.0, 0.0, 400.0, 200.0), props(serde_json::json!({"trim": null, "selectionStartMs": null, "selectionEndMs": null})))]);
    presenter.provide_bytes("a", &encode_wav_pcm16(&ramp(2500)));
    let mut uploads = Uploads::default();
    presenter.toggle("a");
    for _ in 0..200 {
        presenter.tick("a", 16, &mut uploads);
        device(&opened).lock().unwrap().drain(25);
    }
    let played = device(&opened).lock().unwrap().played.clone();
    assert_eq!(played, ramp(2500).samples);
    let (mut presenter, opened) = new_presenter(1000);
    presenter.reconcile(vec![descriptor("a", SlotRect::new(0.0, 0.0, 400.0, 200.0), props(serde_json::json!({"trim": null, "selectionStartMs": null, "selectionEndMs": null})))]);
    presenter.provide_bytes("a", &encode_wav_pcm16(&ramp(2500)));
    presenter.toggle("a");
    presenter.tick("a", 16, &mut uploads);
    assert!(device(&opened).lock().unwrap().queued_frames() > 0);
    presenter.seek("a", 1000);
    assert_eq!((device(&opened).lock().unwrap().queued_frames(), presenter.position_ms("a")), (0, 1000));
    presenter.toggle("a");
    assert_eq!(device(&opened).lock().unwrap().events.last(), Some(&"pause"));
    presenter.close();
    assert_eq!(device(&opened).lock().unwrap().events.last(), Some(&"close"));
}

/// ⚖️ LAW: a video slot uploads each distinct picture once under its raster key, quads it contained in the video area, follows the clock to
/// the next picture, and the keys answer the window kit's rules (Space toggles, arrows seek, a focused range keeps its arrows).
#[test]
fn video_slot_uploads_pictures_quads_them_and_answers_keys() {
    let fixture = fixtures();
    let (mut presenter, _) = new_presenter(100);
    let mut value = props(serde_json::json!({"kind": "video", "mediaType": "video/mp4", "durationMs": 1000, "selectionStartMs": null, "selectionEndMs": null, "trim": null})).expect("props");
    value.kind = SlotMediaKind::Video;
    presenter.reconcile(vec![descriptor("slot-v", rect(&fixture["videoRect"]), Ok(value))]);
    presenter.provide_bytes("slot-v", &first_party_mp4());
    assert_eq!(presenter.load_state("slot-v"), Some(&SlotLoad::Ready));
    assert!(presenter.quad("slot-v").is_none());
    let mut uploads = Uploads::default();
    assert!(presenter.tick("slot-v", 16, &mut uploads).uploaded);
    assert!(!presenter.tick("slot-v", 16, &mut uploads).uploaded);
    let quad = presenter.quad("slot-v").expect("quad");
    assert_eq!((quad.rect, quad.raster_key.as_str(), quad.width, quad.height), (rect(&fixture["videoQuad"]["rect"]), fixture["videoQuad"]["key"].as_str().unwrap(), 32, 16));
    assert!(presenter.key("slot-v", " ", false, false, false, false) && presenter.playing("slot-v"));
    assert!(presenter.tick("slot-v", 600, &mut uploads).uploaded);
    assert_eq!(uploads.0.iter().map(|upload| (upload.0.as_str(), upload.3 > 200)).collect::<Vec<_>>(), vec![("media-frame:slot-v", false), ("media-frame:slot-v", true)]);
    assert!(presenter.key("slot-v", "ArrowLeft", false, false, false, false) && presenter.position_ms("slot-v") == 0);
    presenter.focus("slot-v", Some(ControlKind::Seek));
    assert!(!presenter.key("slot-v", "ArrowRight", false, false, false, false));
    presenter.focus("slot-v", Some(ControlKind::PlayPause));
    assert!(!presenter.key("slot-v", " ", false, false, false, false));
    assert!(!presenter.key("slot-v", "k", false, true, false, false));
    let tick = presenter.tick("slot-v", 2000, &mut uploads);
    assert!(tick.ended && !presenter.playing("slot-v"));
    assert_eq!(presenter.hit_test("slot-v", 10.0, 130.0), Some(ControlKind::PlayPause));
}

/// ⚖️ LAW: the selection sliders keep their bounds (start below end, end above start) and the cut follows the selection; the spoken time matches
/// the window kit's plural fixture in both languages.
#[test]
fn selection_bounds_and_spoken_time_follow_the_window_kit() {
    let fixture = fixtures();
    let labels_en = props(serde_json::json!({})).expect("props").labels;
    let kit: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/⏱️time-and-keys/🧫️fixture.json")).expect("kit fixture");
    let mut german = fixture["base"].clone();
    german["labels"] = fixture["labels"]["de"].clone();
    german["locale"] = "de".into();
    let labels_de = parse_slot_props(&DslValue::from(german)).expect("german").labels;
    for case in kit["plural"].as_array().expect("plural") {
        let ms = case["ms"].as_u64().unwrap();
        assert_eq!((media_time_value_text(ms, false, &labels_en), media_time_value_text(ms, true, &labels_de)), (case["enText"].as_str().unwrap().to_owned(), case["deText"].as_str().unwrap().to_owned()));
    }
    let (mut presenter, _) = new_presenter(100);
    presenter.reconcile(vec![descriptor("a", rect(&fixture["slotRect"]), props(serde_json::json!({})))]);
    presenter.provide_bytes("a", &encode_wav_pcm16(&ramp(2500)));
    presenter.set_value("a", ControlKind::SelectionStart, 5000);
    presenter.set_value("a", ControlKind::SelectionEnd, 100);
    let controls = presenter.controls("a");
    let value = |kind: ControlKind| controls.iter().find(|control| control.kind == kind).and_then(|control| control.value);
    assert_eq!((value(ControlKind::SelectionStart), value(ControlKind::SelectionEnd)), (Some(799), Some(800)));
    assert_eq!(presenter.cut("a").map(|cut| cut.value).as_deref(), Some("799-800"));
    assert_eq!(presenter.value_at("a", ControlKind::Seek, 118.0), Some(1250));
    presenter.set_value("a", ControlKind::Seek, 99_999);
    assert_eq!(presenter.position_ms("a"), 2500);
}
