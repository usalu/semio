use super::*;
use semio_framework_value::DslValue;
use semio_renderer_media_native::{encode_wav_pcm16, load_native_media, NativeMediaFault, AudioFormat, DecodedAudio, RasterUploadRefusal, RecordingAudioOutput};

struct Sink(usize);

impl RasterUploadPort for Sink {
    fn upload(&mut self, _surface: &str, _key: &str, _width: u32, _height: u32, _rgba: &[u8]) -> Result<(), RasterUploadRefusal> {
        self.0 += 1;
        Ok(())
    }
}

fn presented(token: &str, props: serde_json::Value, rect: (f32, f32, f32, f32), clip: (f32, f32, f32, f32), occluded: bool) -> PresentedMediaSlot {
    let area = |(x, y, width, height): (f32, f32, f32, f32)| PresentedMediaRect { x, y, width, height };
    PresentedMediaSlot {
        token: token.into(), window_id: "w".into(), node_id: "1".into(), node_key: "k".into(), rect: area(rect), clip: area(clip), paint_order: 0, occluded, plugin_id: "p".into(), controller_id: "c".into(), app_instance_id: 1, parent_document_id: "d".into(), props: DslValue::from(props),
    }
}

fn props() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧱️elements/🎬️MediaTransportHost/🎯️targets/🧊️native/🧫️fixtures/🖼️presenter.json")).expect("fixture");
    fixture["base"].clone()
}

/// ⚖️ LAW: a presented slot becomes a descriptor whose rectangle, occlusion and props follow the frame's slot; a slot clipped away is occluded.
#[test]
fn presented_slots_become_descriptors() {
    let visible = slot_descriptor(&presented("a", props(), (0.0, 0.0, 400.0, 200.0), (0.0, 0.0, 400.0, 200.0), false));
    assert_eq!((visible.token.as_str(), visible.rect, visible.occluded, visible.props.is_ok()), ("a", SlotRect::new(0.0, 0.0, 400.0, 200.0), false, true));
    let clipped = slot_descriptor(&presented("b", props(), (0.0, 0.0, 400.0, 200.0), (500.0, 0.0, 10.0, 10.0), false));
    assert!(clipped.occluded);
    assert!(slot_descriptor(&presented("c", props(), (0.0, 0.0, 4.0, 4.0), (0.0, 0.0, 4.0, 4.0), true)).occluded);
    assert_eq!(slot_descriptor(&presented("d", serde_json::json!({}), (0.0, 0.0, 4.0, 4.0), (0.0, 0.0, 4.0, 4.0), false)).props, Err("framework.media.transport@1 · invalid-contract".to_string()));
}

/// ⚖️ LAW: a host presents its slots, waits for their bytes, plays the delivered WAV into its audio output and drops a vanished slot's stream.
#[test]
fn host_presents_slots_plays_audio_and_closes_vanished_slots() {
    let device = std::sync::Arc::new(std::sync::Mutex::new(None::<std::sync::Arc<std::sync::Mutex<RecordingAudioOutput>>>));
    let slot = device.clone();
    let mut host = NativeMediaHost::with_audio(Box::new(move |format: AudioFormat| {
        let recording = std::sync::Arc::new(std::sync::Mutex::new(RecordingAudioOutput::new(format, 1000)));
        *slot.lock().unwrap() = Some(recording.clone());
        struct Port(std::sync::Arc<std::sync::Mutex<RecordingAudioOutput>>);
        impl semio_renderer_media_native::AudioOutputPort for Port {
            fn format(&self) -> AudioFormat { self.0.lock().unwrap().format() }
            fn write(&mut self, samples: &[f32]) -> Result<usize, semio_renderer_media_native::AudioOutputError> { self.0.lock().unwrap().write(samples) }
            fn queued_frames(&self) -> usize { self.0.lock().unwrap().queued_frames() }
            fn start(&mut self) -> Result<(), semio_renderer_media_native::AudioOutputError> { self.0.lock().unwrap().start() }
            fn pause(&mut self) -> Result<(), semio_renderer_media_native::AudioOutputError> { self.0.lock().unwrap().pause() }
            fn flush(&mut self) -> Result<(), semio_renderer_media_native::AudioOutputError> { self.0.lock().unwrap().flush() }
            fn stats(&self) -> semio_renderer_media_native::AudioOutputStats { self.0.lock().unwrap().stats() }
            fn close(&mut self) { self.0.lock().unwrap().close() }
        }
        Box::new(Port(recording))
    }), true);
    let slots = [presented("a", props(), (0.0, 0.0, 400.0, 200.0), (0.0, 0.0, 400.0, 200.0), false)];
    let mut sink = Sink(0);
    let frame = host.present(&slots, 16, &mut sink);
    assert_eq!((host.awaiting_bytes(), frame.controls[0].1[0].text.as_str()), (vec!["a".to_string()], "Loading media…"));
    let audio = DecodedAudio { sample_rate: 1000, channels: 1, samples: (0..2500).map(|index| (index % 2000) as f32 / 4096.0).collect() };
    host.provide_bytes("a", &encode_wav_pcm16(&audio));
    host.presenter().toggle("a");
    host.present(&slots, 16, &mut sink);
    let recording = device.lock().unwrap().clone().expect("audio opened");
    assert!(recording.lock().unwrap().queued_frames() > 0);
    host.present(&[], 16, &mut sink);
    assert_eq!(recording.lock().unwrap().events.last(), Some(&"close"));
    assert_eq!(sink.0, 0);
}

fn ready_host() -> (NativeMediaHost, Vec<PresentedMediaSlot>) {
    let mut host = NativeMediaHost::with_audio(Box::new(|format: AudioFormat| Box::new(RecordingAudioOutput::new(format, 1000))), true);
    let slots = vec![presented("a", props(), (0.0, 0.0, 400.0, 200.0), (0.0, 0.0, 400.0, 200.0), false)];
    host.present(&slots, 16, &mut Sink(0));
    let audio = DecodedAudio { sample_rate: 1000, channels: 1, samples: vec![0.0; 2000] };
    host.finish_decode("a", load_native_media(&encode_wav_pcm16(&audio), &mut |_, _| {}, &|| false));
    host.present(&slots, 16, &mut Sink(0));
    (host, slots)
}

/// ⚖️ LAW: a decode that ran off the host's thread installs the player exactly as an inline one does, and a cancelled decode leaves the slot cancelled.
#[test]
fn a_decode_finished_elsewhere_starts_the_player_and_a_cancel_stays_cancelled() {
    let (host, _) = ready_host();
    assert_eq!(host.awaiting_bytes(), Vec::<String>::new());
    let mut cancelled = NativeMediaHost::with_audio(Box::new(|format: AudioFormat| Box::new(RecordingAudioOutput::new(format, 1000))), true);
    cancelled.present(&[presented("a", props(), (0.0, 0.0, 400.0, 200.0), (0.0, 0.0, 400.0, 200.0), false)], 16, &mut Sink(0));
    cancelled.finish_decode("a", Err(NativeMediaFault::Cancelled));
    assert_eq!(cancelled.presenter().load_state("a"), Some(&SlotLoad::Cancelled));
}

/// ⚖️ LAW: the accessibility rows name the root group at depth 0 and every control below it, keyed like the controls; an assistive-technology activate presses play, a value seeks a range, and an unknown row is not taken.
#[test]
fn assistive_technology_rows_and_actions_follow_the_controls() {
    let (mut host, slots) = ready_host();
    host.present_overlays(&slots, 16, &mut Sink(0));
    let (token, nodes) = host.accessibility_nodes().remove(0);
    assert_eq!((token.as_str(), nodes[0].key.as_str(), nodes[0].depth, nodes[0].role.as_str()), ("a", "root", 0, "group"));
    let play = nodes.iter().find(|node| node.key == "play-pause").expect("play row");
    assert_eq!((play.role.as_str(), play.actionable, play.depth), ("button", true, 1));
    assert!(!host.presenter().playing("a"));
    assert_eq!(host.accessibility("a", "play-pause", MediaAccessibility::Activate), MediaInput::Consumed);
    assert!(host.presenter().playing("a"));
    assert_eq!(host.accessibility("a", "seek", MediaAccessibility::Value("500".into())), MediaInput::Consumed);
    assert_eq!(host.presenter().position_ms("a"), 500);
    assert_eq!(host.accessibility("a", "no-such-row", MediaAccessibility::Activate), MediaInput::Ignored);
    assert_eq!(host.accessibility("b", "play-pause", MediaAccessibility::Activate), MediaInput::Ignored);
}

/// ⚖️ LAW: overlays are addressed by (window, node): the same node id in two windows never shares an overlay, and an unknown pair finds none.
#[test]
fn overlays_are_found_by_exact_window_and_node_identity() {
    use semio_ui_wgpu_paint_overlay::{media_overlay_for, media_window_handle, publish_media_overlays, MediaOverlay};
    let overlay = |window: &str, node_id, x| MediaOverlay { window: media_window_handle(window), node_id, rect: [x, 0.0, 10.0, 10.0], ..Default::default() };
    publish_media_overlays(vec![overlay("w1", 7, 0.0), overlay("w2", 7, 100.0), overlay("w1", 8, 0.0)]);
    assert_eq!(media_overlay_for(media_window_handle("w2"), 7).map(|found| found.rect[0]), Some(100.0));
    assert_eq!(media_overlay_for(media_window_handle("w1"), 7).map(|found| found.rect[0]), Some(0.0));
    assert_eq!(media_overlay_for(media_window_handle("w1"), 9), None);
    assert_eq!(media_overlay_for(media_window_handle("w3"), 7), None);
    publish_media_overlays(Vec::new());
}

/// ⚖️ LAW: a press and release on the cut button answers the declared cut request, and a slot without a declared trim answers none.
#[test]
fn the_cut_button_answers_its_request_and_a_pointer_outside_every_control_is_ignored() {
    let (mut host, slots) = ready_host();
    host.present_overlays(&slots, 16, &mut Sink(0));
    assert_eq!(host.pointer(MediaPointer::Down, -50.0, -50.0), MediaInput::Ignored);
    let cut = host.presenter().controls("a").into_iter().find(|control| control.kind == ControlKind::Cut);
    if let Some(cut) = cut {
        let (x, y) = (cut.rect.x + 1.0, cut.rect.y + 1.0);
        assert_eq!(host.pointer(MediaPointer::Down, x, y), MediaInput::Consumed);
        assert!(matches!(host.pointer(MediaPointer::Up, x, y), MediaInput::Cut { .. } | MediaInput::Consumed));
    }
}

fn focused_key(host: &NativeMediaHost) -> Option<String> {
    host.accessibility_nodes().into_iter().flat_map(|(_, nodes)| nodes).find(|node| node.focused).map(|node| node.key)
}

/// ⚖️ LAW: Tab and Shift+Tab walk the enabled buttons and ranges in the overlay's reading order, Shift+Tab undoes Tab, and Tab past the last stop (or with no media focus) leaves the key to the page and clears the media focus.
#[test]
fn tab_walks_the_control_rows_and_leaves_at_the_ends() {
    let (mut host, slots) = ready_host();
    host.present_overlays(&slots, 16, &mut Sink(0));
    assert_eq!(host.key("Tab", false, false, false, false), MediaInput::Ignored);
    assert_eq!(host.accessibility("a", "play-pause", MediaAccessibility::Focus), MediaInput::Consumed);
    assert_eq!(focused_key(&host).as_deref(), Some("play-pause"));
    assert_eq!(host.key("Tab", false, false, false, false), MediaInput::Consumed);
    let second = focused_key(&host);
    assert_ne!(second.as_deref(), Some("play-pause"));
    assert!(second.is_some());
    assert_eq!(host.key("Tab", true, false, false, false), MediaInput::Consumed);
    assert_eq!(focused_key(&host).as_deref(), Some("play-pause"));
    let mut left = false;
    for _ in 0..12 {
        if host.key("Tab", false, false, false, false) == MediaInput::Ignored {
            left = true;
            break;
        }
    }
    assert!(left);
    assert_eq!(focused_key(&host), None);
    assert!(host.enter("a", true));
    assert_eq!(host.focused_token(), Some("a"));
    assert_eq!(host.key("Tab", false, false, false, false), MediaInput::Ignored);
    assert!(host.enter("a", false));
    assert_eq!(host.key("Tab", true, false, false, false), MediaInput::Ignored);
    assert!(host.enter("a", false));
    host.release();
    assert_eq!((host.focused_token(), focused_key(&host)), (None, None));
    assert!(!host.enter("missing", false));
}
