//! 🖼️ The native media-slot presenter: what the browser overlay (`🎬️MediaTransportHost/🟦️.tsx`) is to an `HTMLMediaElement`, for a window
//! that decodes in-repo. One [`MediaSlotPresenter`] owns a [`NativeMediaPlayer`] and an [`AudioOutputPort`] per presented slot, drives the clock
//! and the audio queue from the host's frame ticks, hands each new video picture to a [`RasterUploadPort`] (the renderer's keyed raster upload;
//! the slot then paints one image quad under [`MediaQuad::raster_key`] in the slot rectangle) and answers the exact control set of the overlay as
//! plain [`Control`] rows (play or pause, seek, position, duration, selection range, cut, loading, cancel) with their roles, labels and spoken
//! values, so any renderer can draw and expose them. Everything here is pure data in, data out, and has no GPU or OS type.

use super::audio_output::{AudioFormat, AudioOutputError, AudioOutputOpener, AudioOutputPort};
use super::player::{load_native_media, native_media_support, NativeMedia, NativeMediaFault, NativeMediaPlayer, NativeMediaSupport};
use super::transport::{format_media_time, media_key_command, trim_value_for_selection, MediaKeyCommand, MediaKeyInput, MediaTrimRate, MediaTransportEvent};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// 📐️ A rectangle in the slot's window space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SlotRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl SlotRect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// 👆️ Whether the point lies inside.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    /// 🖼️ The largest rectangle of `width` by `height` aspect that fits inside, centred: `object-fit: contain`.
    pub fn contain(&self, width: u32, height: u32) -> SlotRect {
        if width == 0 || height == 0 || self.w <= 0.0 || self.h <= 0.0 {
            return SlotRect::new(self.x, self.y, 0.0, 0.0);
        }
        let scale = (self.w / width as f32).min(self.h / height as f32);
        let (w, h) = (width as f32 * scale, height as f32 * scale);
        SlotRect::new(self.x + (self.w - w) / 2.0, self.y + (self.h - h) / 2.0, w, h)
    }
}

/// 🏷️ The 22 host-owned labels of the contract, by their wire key.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MediaLabels(BTreeMap<String, String>);

impl MediaLabels {
    pub const KEYS: [&'static str; 22] = [
        "play", "pause", "seek", "position", "duration", "selectionStart", "selectionEnd", "loading", "progress", "cancel", "unsupported", "unknownDuration", "cancelled", "cutSelection", "hourOne", "hourOther", "minuteOne",
        "minuteOther", "secondOne", "secondOther", "audio", "video",
    ];

    /// 🔤️ The label for `key` (empty for an unknown key; the parse refuses a table that lacks one).
    pub fn get(&self, key: &str) -> &str {
        self.0.get(key).map_or("", String::as_str)
    }
}

/// 🎞️ Audio or video.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotMediaKind {
    Audio,
    Video,
}

/// 🎟️ What the plugin says about the medium before the host decodes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotCapability {
    Ready,
    Loading,
    Unsupported,
}

/// ✂️ The editor-declared range cut.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotTrim {
    pub action_id: String,
    pub revision: String,
    pub rate: MediaTrimRate,
}

/// 📥️ The parsed `framework.media.transport@1` props of one slot (`📓️parseMediaTransportProps`).
#[derive(Clone, Debug, PartialEq)]
pub struct MediaSlotProps {
    pub kind: SlotMediaKind,
    pub media_type: String,
    pub revision: String,
    pub generation: Option<String>,
    pub duration_ms: Option<u64>,
    pub position_ms: u64,
    pub selection: Option<(u64, u64)>,
    pub is_de: bool,
    pub labels: MediaLabels,
    pub capability: SlotCapability,
    pub capability_reason: Option<String>,
    pub has_resource: bool,
    pub host_content_height: f32,
    pub trim: Option<SlotTrim>,
}

/// 🇩🇪️ React's text for props that fail the contract: by the locale the value names, else the extension id and the code.
pub fn invalid_contract_label(value: &DslValue) -> String {
    match value.get("locale").and_then(DslValue::as_str) {
        Some("de") => "Ungültiger Medienvertrag.".into(),
        Some("en") => "Invalid media contract.".into(),
        _ => "framework.media.transport@1 · invalid-contract".into(),
    }
}

fn whole(value: Option<&DslValue>) -> Option<u64> {
    let number = value?.as_f64()?;
    (number.is_finite() && number >= 0.0 && number.fract() == 0.0 && number <= 9_007_199_254_740_991.0).then_some(number as u64)
}

fn text<'a>(value: Option<&'a DslValue>) -> Option<&'a str> {
    value?.as_str()
}

/// 🧬️ Parses a slot's props like `parseMediaTransportProps`: exact shapes, every refusal answers the invalid-contract text.
pub fn parse_slot_props(value: &DslValue) -> Result<MediaSlotProps, String> {
    parse_inner(value).ok_or_else(|| invalid_contract_label(value))
}

fn parse_inner(value: &DslValue) -> Option<MediaSlotProps> {
    let DslValue::Object(entries) = value else { return None };
    let keys = ["schemaVersion", "kind", "mediaType", "revision", "durationMs", "positionMs", "selectionStartMs", "selectionEndMs", "locale", "labels", "resource", "capability", "hostContentHeight", "trim"];
    if entries.len() != keys.len() || !keys.iter().all(|key| value.get(key).is_some()) || whole(value.get("schemaVersion")) != Some(1) {
        return None;
    }
    let kind = match text(value.get("kind"))? {
        "audio" => SlotMediaKind::Audio,
        "video" => SlotMediaKind::Video,
        _ => return None,
    };
    let media_type = text(value.get("mediaType"))?.to_owned();
    let revision = text(value.get("revision"))?.to_owned();
    if media_type.is_empty() || media_type.chars().count() > 128 || revision.parse::<u64>().ok()?.to_string() != revision {
        return None;
    }
    let is_de = match text(value.get("locale"))? {
        "de" => true,
        "en" => false,
        _ => return None,
    };
    let duration_ms = match value.get("durationMs")? {
        DslValue::Null => None,
        other => Some(whole(Some(other)).filter(|duration| *duration >= 1)?),
    };
    let position_ms = whole(value.get("positionMs"))?;
    let selection = match (value.get("selectionStartMs")?, value.get("selectionEndMs")?) {
        (DslValue::Null, DslValue::Null) => None,
        (start, end) => {
            let (start, end) = (whole(Some(start))?, whole(Some(end))?);
            Some((start < end && end >= 1 && duration_ms.is_some_and(|duration| end <= duration)).then_some((start, end))?)
        }
    };
    if duration_ms.is_some_and(|duration| position_ms > duration) || (duration_ms.is_none() && (position_ms != 0 || selection.is_some())) {
        return None;
    }
    let DslValue::Object(label_entries) = value.get("labels")? else { return None };
    if label_entries.len() != MediaLabels::KEYS.len() {
        return None;
    }
    let mut labels = BTreeMap::new();
    for key in MediaLabels::KEYS {
        labels.insert(key.to_owned(), text(value.get("labels")?.get(key))?.to_owned());
    }
    let (has_resource, generation) = match value.get("resource")? {
        DslValue::Null => (false, None),
        resource @ DslValue::Object(_) => (true, Some(text(resource.get("generation"))?.to_owned())),
        _ => return None,
    };
    let capability_value = value.get("capability")?;
    let capability = match text(capability_value.get("status"))? {
        "ready" => SlotCapability::Ready,
        "loading" => SlotCapability::Loading,
        "unsupported" => SlotCapability::Unsupported,
        _ => return None,
    };
    let capability_reason = match capability_value.get("reason")? {
        DslValue::Null => None,
        reason => Some(reason.as_str()?.to_owned()),
    };
    let host_content_height = value.get("hostContentHeight")?.as_f64().filter(|height| height.is_finite() && (0.0..=4096.0).contains(height))? as f32;
    let trim = match value.get("trim")? {
        DslValue::Null => None,
        trim => {
            let positive = |key: &str| whole(trim.get(key)).filter(|number| *number >= 1);
            Some(SlotTrim {
                action_id: text(trim.get("actionId")).filter(|id| !id.is_empty() && id.chars().count() <= 128)?.to_owned(),
                revision: text(trim.get("revision")).filter(|id| !id.is_empty() && id.chars().count() <= 128)?.to_owned(),
                rate: MediaTrimRate { units_per_second_numerator: positive("unitsPerSecondNumerator")?, units_per_second_denominator: positive("unitsPerSecondDenominator")?, total_units: positive("totalUnits")? },
            })
        }
    };
    Some(MediaSlotProps { kind, media_type, revision, generation, duration_ms, position_ms, selection, is_de, labels: MediaLabels(labels), capability, capability_reason, has_resource, host_content_height, trim })
}

/// 🗣️ Spells a media time out for assistive technology from the label table (`mediaTimeValueText`).
pub fn media_time_value_text(milliseconds: u64, is_de: bool, labels: &MediaLabels) -> String {
    let (hours, minutes) = (milliseconds / 3_600_000, milliseconds / 60_000 % 60);
    let unit = |value: u64, one: &str, other: &str| format!("{value} {}", if value == 1 { labels.get(one) } else { labels.get(other) });
    let mark = if is_de { ',' } else { '.' };
    let seconds = format!("{}{mark}{:03}", milliseconds / 1000 % 60, milliseconds % 1000);
    let seconds_unit = if milliseconds / 1000 % 60 == 1 && milliseconds % 1000 == 0 { labels.get("secondOne") } else { labels.get("secondOther") };
    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(unit(hours, "hourOne", "hourOther"));
    }
    if minutes > 0 || hours > 0 {
        parts.push(unit(minutes, "minuteOne", "minuteOther"));
    }
    parts.push(format!("{seconds} {seconds_unit}"));
    parts.join(" ")
}

/// 📥️ Where a slot's medium is in its life, as the overlay's `TransportState`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotLoad {
    Invalid(String),
    Loading { done: u64, total: u64, detail: String },
    Ready,
    Cancelled,
    Unsupported(String),
    Failed(String),
}

/// 🎛️ The controls of a slot, by what they are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControlKind {
    Root,
    Status,
    Progress,
    CancelLoad,
    PlayPause,
    Seek,
    Position,
    Duration,
    SelectionStart,
    SelectionEnd,
    Cut,
}

/// ♿️ The accessible role a control exposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlRole {
    Group,
    Alert,
    Status,
    ProgressBar,
    Button,
    Slider,
    Output,
    Text,
}

/// 🎛️ One control row: where it sits, what it is called, how it reads aloud and what it holds.
#[derive(Clone, Debug, PartialEq)]
pub struct Control {
    pub kind: ControlKind,
    pub role: ControlRole,
    pub rect: SlotRect,
    pub label: String,
    pub text: String,
    pub value_text: Option<String>,
    pub value: Option<u64>,
    pub min: u64,
    pub max: u64,
    pub enabled: bool,
    pub shortcuts: Option<&'static str>,
    pub lang: &'static str,
}

/// 🖼️ The one image quad of a video slot: the picture, contained in the video area of the slot.
#[derive(Clone, Debug, PartialEq)]
pub struct MediaQuad {
    pub rect: SlotRect,
    pub raster_key: String,
    pub width: u32,
    pub height: u32,
}

/// ✂️ The cut action a press dispatches through history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CutRequest {
    pub action_id: String,
    pub revision: String,
    pub value: String,
}

/// 🚫️ Why a raster upload was not taken.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterUploadRefusal {
    Busy,
    Invalid(String),
}

/// 🖼️ The renderer's keyed raster upload: one RGBA8 bitmap under `key` for the slot's surface. A `Busy` answer is retried with the next picture.
pub trait RasterUploadPort {
    fn upload(&mut self, surface_id: &str, key: &str, width: u32, height: u32, rgba: &[u8]) -> Result<(), RasterUploadRefusal>;
}

/// 📍️ One slot as the frame presents it.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotDescriptor {
    pub token: String,
    pub rect: SlotRect,
    pub occluded: bool,
    pub props: Result<MediaSlotProps, String>,
}

/// ⏱️ What one frame tick changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SlotTick {
    pub position_ms: u64,
    pub ended: bool,
    pub uploaded: bool,
}

const PAD: f32 = 4.0;
const GAP: f32 = 4.0;
const ROW: f32 = 32.0;
const BUTTON_WIDTH: f32 = 32.0;
const TIME_WIDTH: f32 = 96.0;
const AUDIO_TARGET_DIVISOR: u32 = 10;
const SHORTCUTS: &str = "Space ArrowLeft ArrowRight Home End";

struct Slot {
    rect: SlotRect,
    occluded: bool,
    props: Result<MediaSlotProps, String>,
    load: SlotLoad,
    player: Option<NativeMediaPlayer>,
    audio: Option<Box<dyn AudioOutputPort>>,
    carry: Vec<f32>,
    draining: bool,
    last_frame: Option<usize>,
    cancel: Arc<AtomicBool>,
    selection: Option<(u64, u64)>,
    focus: Option<ControlKind>,
    identity: String,
}

fn identity(props: &Result<MediaSlotProps, String>) -> String {
    props.as_ref().map_or_else(|label| label.clone(), |props| format!("{}|{}|{:?}|{:?}", props.media_type, props.revision, props.generation, props.capability))
}

fn initial_load(props: &Result<MediaSlotProps, String>) -> SlotLoad {
    match props {
        Err(label) => SlotLoad::Invalid(label.clone()),
        Ok(props) => match props.capability {
            SlotCapability::Unsupported => SlotLoad::Unsupported(props.capability_reason.clone().unwrap_or_else(|| props.labels.get("unsupported").to_owned())),
            _ if !props.has_resource => match props.capability {
                SlotCapability::Loading => SlotLoad::Loading { done: 0, total: 0, detail: props.capability_reason.clone().unwrap_or_else(|| props.labels.get("loading").to_owned()) },
                _ => SlotLoad::Unsupported(props.labels.get("unsupported").to_owned()),
            },
            _ => match native_media_support(&props.media_type) {
                NativeMediaSupport::Ready(_) => SlotLoad::Loading { done: 0, total: 0, detail: props.labels.get("loading").to_owned() },
                NativeMediaSupport::Unsupported { reason } => SlotLoad::Unsupported(reason),
            },
        },
    }
}

/// 🖼️ Presents every media slot of a frame: players, audio, pictures, controls and inputs.
pub struct MediaSlotPresenter {
    slots: BTreeMap<String, Slot>,
    open_audio: AudioOutputOpener,
    can_apply: bool,
}

impl MediaSlotPresenter {
    /// 🏗️ A presenter that opens its audio through `open_audio` (the system output, or a law's recording output); `can_apply` says whether
    /// the owner takes a cut action, which is what shows the cut button.
    pub fn new(open_audio: AudioOutputOpener, can_apply: bool) -> Self {
        Self { slots: BTreeMap::new(), open_audio, can_apply }
    }

    /// 📍️ Brings the slots up to date with a frame: new slots start loading, vanished ones close their load and their audio, a slot whose medium
    /// identity changed starts over.
    pub fn reconcile(&mut self, descriptors: Vec<SlotDescriptor>) {
        let live: Vec<&str> = descriptors.iter().map(|descriptor| descriptor.token.as_str()).collect();
        let gone: Vec<String> = self.slots.keys().filter(|token| !live.contains(&token.as_str())).cloned().collect();
        for token in gone {
            self.remove(&token);
        }
        for descriptor in descriptors {
            let id = identity(&descriptor.props);
            let stale = self.slots.get(&descriptor.token).is_some_and(|slot| slot.identity != id);
            if stale {
                self.remove(&descriptor.token);
            }
            match self.slots.get_mut(&descriptor.token) {
                Some(slot) => {
                    slot.rect = descriptor.rect;
                    slot.occluded = descriptor.occluded;
                    slot.props = descriptor.props;
                }
                None => {
                    let selection = descriptor.props.as_ref().ok().and_then(|props| props.selection);
                    let load = initial_load(&descriptor.props);
                    self.slots.insert(descriptor.token, Slot { rect: descriptor.rect, occluded: descriptor.occluded, props: descriptor.props, load, player: None, audio: None, carry: Vec::new(), draining: false, last_frame: None, cancel: Arc::new(AtomicBool::new(false)), selection, focus: None, identity: id });
                }
            }
        }
    }

    fn remove(&mut self, token: &str) {
        if let Some(mut slot) = self.slots.remove(token) {
            slot.cancel.store(true, Ordering::Relaxed);
            if let Some(audio) = slot.audio.as_mut() {
                audio.close();
            }
        }
    }

    /// 🧹️ Closes every slot.
    pub fn close(&mut self) {
        let tokens: Vec<String> = self.slots.keys().cloned().collect();
        for token in tokens {
            self.remove(&token);
        }
    }

    /// 🔎️ The tokens of the slots that still wait for their bytes.
    pub fn awaiting_bytes(&self) -> Vec<String> {
        self.slots.iter().filter(|(_, slot)| slot.player.is_none() && matches!(slot.load, SlotLoad::Loading { .. }) && slot.props.as_ref().is_ok_and(|props| props.has_resource)).map(|(token, _)| token.clone()).collect()
    }

    /// 🛑️ The flag a byte transfer for `token` polls to stop.
    pub fn cancel_flag(&self, token: &str) -> Option<Arc<AtomicBool>> {
        self.slots.get(token).map(|slot| slot.cancel.clone())
    }

    /// 📊️ Records the transfer's progress.
    pub fn transfer_progress(&mut self, token: &str, done: u64, total: u64, detail: &str) {
        if let Some(slot) = self.slots.get_mut(token) {
            if let SlotLoad::Loading { detail: current, done: at, total: all } = &mut slot.load {
                *at = done;
                *all = total;
                if !detail.is_empty() {
                    *current = detail.to_owned();
                }
            }
        }
    }

    /// 🛑️ The person's Cancel on a loading slot.
    pub fn cancel_load(&mut self, token: &str) {
        if let Some(slot) = self.slots.get_mut(token).filter(|slot| matches!(slot.load, SlotLoad::Loading { .. })) {
            slot.cancel.store(true, Ordering::Relaxed);
            slot.load = SlotLoad::Cancelled;
        }
    }

    /// 📦️ The transfer ended without bytes (cancelled or failed).
    pub fn transfer_ended(&mut self, token: &str, failure: Option<String>) {
        if let Some(slot) = self.slots.get_mut(token).filter(|slot| slot.player.is_none() && matches!(slot.load, SlotLoad::Loading { .. } | SlotLoad::Cancelled)) {
            slot.load = match failure {
                Some(detail) => SlotLoad::Failed(detail),
                None => SlotLoad::Cancelled,
            };
        }
    }

    /// 📥️ Decodes the slot's bytes natively on the caller's thread and starts the player paused at the props' position; progress and cancel follow the decoders. A host that must not decode on its UI thread decodes with [`load_native_media`] elsewhere and hands the answer to [`Self::finish_decode`].
    pub fn provide_bytes(&mut self, token: &str, bytes: &[u8]) {
        let Some(slot) = self.slots.get_mut(token) else { return };
        if slot.props.is_err() || slot.player.is_some() {
            return;
        }
        let cancel = slot.cancel.clone();
        if cancel.load(Ordering::Relaxed) {
            slot.load = SlotLoad::Cancelled;
            return;
        }
        let load = &mut slot.load;
        let decoded = load_native_media(bytes, &mut |done, total| {
            if let SlotLoad::Loading { done: at, total: all, .. } = load {
                (*at, *all) = (done, total);
            }
        }, &|| cancel.load(Ordering::Relaxed));
        self.finish_decode(token, decoded);
    }

    /// 🏁️ Takes the answer of a decode that ran elsewhere: a decoded medium starts the player paused at the props' position (and opens its audio stream), a fault or a cancel sets the slot's state as the overlay shows it.
    pub fn finish_decode(&mut self, token: &str, decoded: Result<NativeMedia, NativeMediaFault>) {
        let Some(slot) = self.slots.get_mut(token) else { return };
        let Ok(props) = slot.props.clone() else { return };
        if slot.player.is_some() {
            return;
        }
        match decoded {
            Ok(media) => {
                let kind_matches = matches!((&media, props.kind), (NativeMedia::Audio(_), SlotMediaKind::Audio) | (NativeMedia::Video(_), SlotMediaKind::Video));
                if !kind_matches {
                    slot.load = SlotLoad::Failed(props.labels.get("unsupported").to_owned());
                    return;
                }
                let mut player = NativeMediaPlayer::new(media);
                player.seek(props.position_ms);
                if let NativeMedia::Audio(audio) = &player.media {
                    slot.audio = Some((self.open_audio)(AudioFormat { sample_rate: audio.sample_rate, channels: audio.channels }));
                }
                slot.player = Some(player);
                slot.load = SlotLoad::Ready;
            }
            Err(NativeMediaFault::Cancelled) => slot.load = SlotLoad::Cancelled,
            Err(fault) => slot.load = SlotLoad::Failed(if matches!(fault, NativeMediaFault::Unrecognised) { props.labels.get("unsupported").to_owned() } else { fault.to_string() }),
        }
    }

    fn duration(slot: &Slot) -> Option<u64> {
        slot.player.as_ref().and_then(|player| player.transport.duration_ms).or_else(|| slot.props.as_ref().ok().and_then(|props| props.duration_ms))
    }

    /// 🕑️ The position the person hears and sees: an audio player's position minus what it has queued but not yet played.
    pub fn position_ms(&self, token: &str) -> u64 {
        let Some(slot) = self.slots.get(token) else { return 0 };
        let Some(player) = slot.player.as_ref() else { return slot.props.as_ref().map_or(0, |props| props.position_ms) };
        let queued = slot.audio.as_ref().zip(match &player.media { NativeMedia::Audio(audio) => Some(audio.sample_rate), _ => None }).map_or(0, |(audio, rate)| (audio.queued_frames() as u64 + (slot.carry.len() as u64 / u64::from(audio.format().channels.max(1)))) * 1000 / u64::from(rate.max(1)));
        if player.transport.playing { player.transport.position_ms.saturating_sub(queued) } else { player.transport.position_ms }
    }

    /// 🔎️ Whether the slot's medium is playing.
    pub fn playing(&self, token: &str) -> bool {
        self.slots.get(token).and_then(|slot| slot.player.as_ref()).is_some_and(|player| player.transport.playing)
    }

    fn feed_audio(slot: &mut Slot) -> Result<(), AudioOutputError> {
        let (Some(player), Some(audio)) = (slot.player.as_mut(), slot.audio.as_mut()) else { return Ok(()) };
        let NativeMedia::Audio(decoded) = &player.media else { return Ok(()) };
        let channels = usize::from(decoded.channels.max(1));
        let target = (decoded.sample_rate / AUDIO_TARGET_DIVISOR) as usize;
        if !slot.carry.is_empty() {
            let taken = audio.write(&slot.carry)?;
            slot.carry.drain(..taken * channels);
            if !slot.carry.is_empty() {
                return Ok(());
            }
        }
        let want = target.saturating_sub(audio.queued_frames());
        if want == 0 || !player.transport.playing {
            return Ok(());
        }
        let (block, event) = player.pull_audio(want);
        if !block.is_empty() {
            let taken = audio.write(&block)?;
            slot.carry.extend_from_slice(&block[taken * channels..]);
        }
        if event == MediaTransportEvent::Ended {
            slot.draining = true;
        }
        Ok(())
    }

    fn audio_failure(slot: &mut Slot, error: AudioOutputError) {
        let format = slot.audio.take().map_or(AudioFormat { sample_rate: 1, channels: 1 }, |mut audio| {
            audio.close();
            audio.format()
        });
        let mut silent = super::audio_output::NullAudioOutput::new(format, error.to_string());
        if slot.player.as_ref().is_some_and(|player| player.transport.playing) {
            let _ = silent.start();
        }
        slot.audio = Some(Box::new(silent));
    }

    /// ⏱️ Advances one slot by a frame of `delta_ms`: a video's clock, an audio player's queue, and the picture to upload when it changed.
    pub fn tick(&mut self, token: &str, delta_ms: u64, uploads: &mut dyn RasterUploadPort) -> SlotTick {
        let Some(slot) = self.slots.get_mut(token) else { return SlotTick::default() };
        let mut tick = SlotTick::default();
        if slot.player.is_some() {
            let event = slot.player.as_mut().map_or(MediaTransportEvent::None, |player| player.tick(delta_ms));
            tick.ended = event == MediaTransportEvent::Ended;
            if let Err(error) = Self::feed_audio(slot) {
                Self::audio_failure(slot, error);
            }
            if slot.draining && slot.audio.as_ref().is_none_or(|audio| audio.queued_frames() == 0) {
                slot.draining = false;
                tick.ended = true;
                if let Some(audio) = slot.audio.as_mut() {
                    let _ = audio.pause();
                }
            }
            if let Some(frame) = slot.player.as_ref().and_then(NativeMediaPlayer::video_frame).filter(|frame| slot.last_frame != Some(frame.index)) {
                let (index, key) = (frame.index, format!("media-frame:{token}"));
                if uploads.upload(token, &key, frame.width, frame.height, frame.rgba).is_ok() {
                    slot.last_frame = Some(index);
                    tick.uploaded = true;
                }
            }
        }
        tick.position_ms = self.position_ms(token);
        tick
    }

    /// ▶️ Plays when paused, pauses when playing; the audio device follows.
    pub fn toggle(&mut self, token: &str) {
        let Some(slot) = self.slots.get_mut(token) else { return };
        let Some(player) = slot.player.as_mut() else { return };
        player.transport.toggle();
        let playing = player.transport.playing;
        slot.draining = false;
        if let Err(error) = Self::feed_audio(slot) {
            Self::audio_failure(slot, error);
        }
        let failed = slot.audio.as_mut().and_then(|audio| (if playing { audio.start() } else { audio.pause() }).err());
        if let Some(error) = failed {
            Self::audio_failure(slot, error);
        }
    }

    /// 🎯️ Moves the position (the seek bar, the keys); queued audio of the old position is dropped.
    pub fn seek(&mut self, token: &str, position_ms: u64) {
        let Some(slot) = self.slots.get_mut(token) else { return };
        let Some(player) = slot.player.as_mut() else { return };
        player.seek(position_ms);
        slot.carry.clear();
        slot.draining = false;
        slot.last_frame = None;
        if let Some(audio) = slot.audio.as_mut() {
            let _ = audio.flush();
            if player.transport.playing {
                let _ = audio.start();
            }
        }
    }

    /// ⌨️ A key pressed while the slot has the focus; `true` when the transport took it (the caller stops its default).
    pub fn key(&mut self, token: &str, key: &str, shift: bool, ctrl: bool, alt: bool, meta: bool) -> bool {
        let Some(slot) = self.slots.get(token) else { return false };
        if slot.player.is_none() {
            return false;
        }
        let (target_tag, target_type) = match slot.focus {
            Some(ControlKind::PlayPause | ControlKind::Cut | ControlKind::CancelLoad) => ("BUTTON", None),
            Some(ControlKind::Seek | ControlKind::SelectionStart | ControlKind::SelectionEnd) => ("INPUT", Some("range")),
            _ => ("DIV", None),
        };
        let duration = Self::duration(slot);
        let Some(command) = media_key_command(&MediaKeyInput { key, shift, ctrl, alt, meta, target_tag, target_type }, duration) else { return false };
        match command {
            MediaKeyCommand::Toggle => self.toggle(token),
            other => {
                let position = self.position_ms(token);
                self.seek(token, super::transport::seeked_media_position(other, position, duration));
            }
        }
        true
    }

    /// 🎯️ Moves the focus (a click or Tab on a control).
    pub fn focus(&mut self, token: &str, control: Option<ControlKind>) {
        if let Some(slot) = self.slots.get_mut(token) {
            slot.focus = control;
        }
    }

    /// 🎚️ A range control set to `value` (milliseconds): the seek bar, or one end of the selection, bounded as the sliders are.
    pub fn set_value(&mut self, token: &str, control: ControlKind, value: u64) {
        let Some(duration) = self.slots.get(token).and_then(Self::duration) else { return };
        let maximum = duration.max(1);
        match control {
            ControlKind::Seek => self.seek(token, value.min(maximum)),
            ControlKind::SelectionStart | ControlKind::SelectionEnd => {
                let Some(slot) = self.slots.get_mut(token) else { return };
                let (start, end) = slot.selection.unwrap_or((0, maximum));
                slot.selection = Some(if control == ControlKind::SelectionStart { (value.min(end.saturating_sub(1)), end) } else { (start, value.max((start + 1).min(maximum)).min(maximum)) });
            }
            _ => {}
        }
    }

    /// ✂️ The cut a press of the cut button dispatches, `None` when the selection holds no whole unit or no trim is declared.
    pub fn cut(&self, token: &str) -> Option<CutRequest> {
        let slot = self.slots.get(token)?;
        let trim = slot.props.as_ref().ok()?.trim.as_ref()?;
        let maximum = Self::duration(slot)?.max(1);
        let (start, end) = slot.selection.unwrap_or((0, maximum));
        Some(CutRequest { action_id: trim.action_id.clone(), revision: trim.revision.clone(), value: trim_value_for_selection(&trim.rate, start, end)? })
    }

    /// 👆️ Presses a control: play or pause toggles, cancel stops the load; ranges answer through [`Self::set_value`] and the cut through [`Self::cut`].
    pub fn activate(&mut self, token: &str, control: ControlKind) {
        self.focus(token, Some(control));
        match control {
            ControlKind::PlayPause => self.toggle(token),
            ControlKind::CancelLoad => self.cancel_load(token),
            _ => {}
        }
    }

    /// 👆️ The control under a point, topmost last.
    pub fn hit_test(&self, token: &str, x: f32, y: f32) -> Option<ControlKind> {
        self.controls(token).into_iter().rev().find(|control| control.kind != ControlKind::Root && control.enabled && control.rect.contains(x, y)).map(|control| control.kind)
    }

    /// 🖱️ A press or drag at `x` on a range control: the position on the bar the pointer names, in milliseconds.
    pub fn value_at(&self, token: &str, control: ControlKind, x: f32) -> Option<u64> {
        let row = self.controls(token).into_iter().find(|candidate| candidate.kind == control && candidate.role == ControlRole::Slider)?;
        let fraction = ((x - row.rect.x) / row.rect.w.max(1.0)).clamp(0.0, 1.0);
        Some(row.min + (fraction as f64 * (row.max - row.min) as f64).round() as u64)
    }

    /// 🖼️ The image quad of a video slot that has shown a picture, `None` for audio, while loading or when hidden behind another window.
    pub fn quad(&self, token: &str) -> Option<MediaQuad> {
        let slot = self.slots.get(token).filter(|slot| !slot.occluded)?;
        let frame = slot.player.as_ref()?.video_frame()?;
        slot.last_frame?;
        let area = Self::video_area(slot, self.can_apply);
        Some(MediaQuad { rect: area.contain(frame.width, frame.height), raster_key: format!("media-frame:{token}"), width: frame.width, height: frame.height })
    }

    fn rows(slot: &Slot, can_apply: bool) -> f32 {
        let props = slot.props.as_ref().ok();
        let known = Self::duration(slot).is_some();
        let cut = can_apply && props.is_some_and(|props| props.trim.is_some());
        ROW + if known { ROW } else { 0.0 } + if known && cut { ROW } else { 0.0 } + 2.0 * GAP
    }

    fn video_area(slot: &Slot, can_apply: bool) -> SlotRect {
        let inner = SlotRect::new(slot.rect.x + PAD, slot.rect.y + PAD, (slot.rect.w - 2.0 * PAD).max(0.0), (slot.rect.h - 2.0 * PAD).max(0.0));
        if !matches!(slot.props.as_ref().map(|props| props.kind), Ok(SlotMediaKind::Video)) {
            return SlotRect::new(inner.x, inner.y, inner.w, 0.0);
        }
        SlotRect::new(inner.x, inner.y, inner.w, (inner.h - Self::rows(slot, can_apply)).max(0.0))
    }

    /// 📜️ The accessible, drawable rows of the slot, in reading order: the overlay's DOM.
    pub fn controls(&self, token: &str) -> Vec<Control> {
        let Some(slot) = self.slots.get(token) else { return Vec::new() };
        let inner = SlotRect::new(slot.rect.x + PAD, slot.rect.y + PAD, (slot.rect.w - 2.0 * PAD).max(0.0), (slot.rect.h - 2.0 * PAD).max(0.0));
        let plain = |kind, role, rect: SlotRect, label: &str, text: &str| Control { kind, role, rect, label: label.to_owned(), text: text.to_owned(), value_text: None, value: None, min: 0, max: 0, enabled: true, shortcuts: None, lang: "" };
        let props = match &slot.props {
            Err(label) => return vec![plain(ControlKind::Status, ControlRole::Alert, inner, label, label)],
            Ok(props) => props,
        };
        let lang = if props.is_de { "de" } else { "en" };
        let line = |row: usize| SlotRect::new(inner.x, inner.y + row as f32 * (ROW + GAP), inner.w, ROW);
        match &slot.load {
            SlotLoad::Invalid(label) => vec![plain(ControlKind::Status, ControlRole::Alert, inner, label, label)],
            SlotLoad::Unsupported(detail) | SlotLoad::Failed(detail) => {
                let shown = if detail.is_empty() { props.labels.get("unsupported") } else { detail };
                vec![plain(ControlKind::Status, ControlRole::Status, inner, shown, shown)]
            }
            SlotLoad::Cancelled => vec![plain(ControlKind::Status, ControlRole::Status, inner, props.labels.get("cancelled"), props.labels.get("cancelled"))],
            SlotLoad::Loading { done, total, detail } => {
                let shown = if detail.is_empty() { props.labels.get("loading") } else { detail };
                let mut progress = plain(ControlKind::Progress, ControlRole::ProgressBar, line(1), props.labels.get("progress"), "");
                (progress.value, progress.max) = (Some(*done), *total);
                vec![plain(ControlKind::Status, ControlRole::Status, line(0), shown, shown), progress, plain(ControlKind::CancelLoad, ControlRole::Button, line(2), props.labels.get("cancel"), props.labels.get("cancel"))]
            }
            SlotLoad::Ready => {
                let known = Self::duration(slot);
                let maximum = known.unwrap_or(1).max(1);
                let position = self.position_ms(token);
                let playing = self.playing(token);
                let area = Self::video_area(slot, self.can_apply);
                let top = if matches!(props.kind, SlotMediaKind::Video) { area.y + area.h + GAP } else { inner.y };
                let row = |index: f32| SlotRect::new(inner.x, top + index * (ROW + GAP), inner.w, ROW);
                let spoken = |ms: u64| media_time_value_text(ms, props.is_de, &props.labels);
                let transport = row(0);
                let seek_width = (transport.w - BUTTON_WIDTH - 2.0 * TIME_WIDTH - 3.0 * GAP).max(0.0);
                let mut root = plain(ControlKind::Root, ControlRole::Group, slot.rect, props.labels.get(if matches!(props.kind, SlotMediaKind::Audio) { "audio" } else { "video" }), "");
                (root.shortcuts, root.lang) = (Some(SHORTCUTS), lang);
                let mut play = plain(ControlKind::PlayPause, ControlRole::Button, SlotRect::new(transport.x, transport.y, BUTTON_WIDTH, ROW), props.labels.get(if playing { "pause" } else { "play" }), "");
                play.lang = lang;
                let mut seek = plain(ControlKind::Seek, ControlRole::Slider, SlotRect::new(transport.x + BUTTON_WIDTH + GAP, transport.y, seek_width, ROW), props.labels.get("seek"), "");
                (seek.value, seek.min, seek.max, seek.value_text, seek.lang) = (Some(position.min(maximum)), 0, maximum, Some(spoken(position)), lang);
                let mut shown = plain(ControlKind::Position, ControlRole::Output, SlotRect::new(seek.rect.x + seek.rect.w + GAP, transport.y, TIME_WIDTH, ROW), props.labels.get("position"), &format_media_time(position, props.is_de));
                (shown.value_text, shown.lang) = (Some(spoken(position)), lang);
                let mut length = plain(ControlKind::Duration, ControlRole::Text, SlotRect::new(shown.rect.x + shown.rect.w + GAP, transport.y, TIME_WIDTH, ROW), props.labels.get("duration"), &known.map_or_else(|| props.labels.get("unknownDuration").to_owned(), |duration| format_media_time(duration, props.is_de)));
                (length.value_text, length.lang) = (known.map(spoken), lang);
                let mut rows = vec![root, play, seek, shown, length];
                if let Some(duration) = known {
                    let (start, end) = slot.selection.unwrap_or((0, maximum));
                    let half = ((inner.w - GAP) / 2.0).max(0.0);
                    let selection_row = row(1);
                    let mut begin = plain(ControlKind::SelectionStart, ControlRole::Slider, SlotRect::new(selection_row.x, selection_row.y, half, ROW), props.labels.get("selectionStart"), "");
                    (begin.value, begin.min, begin.max, begin.value_text, begin.lang) = (Some(start), 0, end.saturating_sub(1), Some(spoken(start)), lang);
                    let mut finish = plain(ControlKind::SelectionEnd, ControlRole::Slider, SlotRect::new(selection_row.x + half + GAP, selection_row.y, half, ROW), props.labels.get("selectionEnd"), "");
                    (finish.value, finish.min, finish.max, finish.value_text, finish.lang) = (Some(end), maximum.min(start + 1), maximum, Some(spoken(end)), lang);
                    rows.extend([begin, finish]);
                    if props.trim.is_some() && self.can_apply {
                        let mut cut = plain(ControlKind::Cut, ControlRole::Button, row(2), props.labels.get("cutSelection"), "");
                        (cut.enabled, cut.lang) = (self.cut(token).is_some(), lang);
                        rows.push(cut);
                    }
                }
                rows
            }
        }
    }

    /// 🔎️ The slot's load state.
    pub fn load_state(&self, token: &str) -> Option<&SlotLoad> {
        self.slots.get(token).map(|slot| &slot.load)
    }
}
