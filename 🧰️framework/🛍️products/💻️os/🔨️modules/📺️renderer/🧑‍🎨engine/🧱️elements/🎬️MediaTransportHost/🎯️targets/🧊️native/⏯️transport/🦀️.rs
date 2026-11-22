//! ⏯️ The transport of a media window for hosts with no media element: playing, position, selection and the range cut, with the keyboard
//! the browser overlay answers (`🎬️media/🌐️browser/🎛️host/🟦️.ts`) and the pure rules of `🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts`, ported rule for rule.

/// ⌨️ The transport commands the keyboard can raise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKeyCommand {
    Toggle,
    Seek { delta_ms: i64 },
    SeekTo { position_ms: u64 },
}

/// ⌨️ The key and the control that holds the focus.
#[derive(Clone, Copy, Debug, Default)]
pub struct MediaKeyInput<'a> {
    pub key: &'a str,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub target_tag: &'a str,
    pub target_type: Option<&'a str>,
}

/// ⌨️ Space or K toggles playback, the arrows seek 5 s (1 s with Shift), Home and End jump; controls that already own the key keep it.
pub fn media_key_command(input: &MediaKeyInput<'_>, duration_ms: Option<u64>) -> Option<MediaKeyCommand> {
    if input.ctrl || input.alt || input.meta {
        return None;
    }
    let tag = input.target_tag.to_uppercase();
    let owns_arrows = tag == "TEXTAREA" || tag == "SELECT" || (tag == "INPUT" && input.target_type.is_some());
    let owns_activation = matches!(tag.as_str(), "BUTTON" | "A" | "TEXTAREA" | "SELECT") || (tag == "INPUT" && input.target_type != Some("range"));
    if matches!(input.key, " " | "Spacebar" | "k" | "K") && !owns_activation {
        return Some(MediaKeyCommand::Toggle);
    }
    if owns_arrows {
        return None;
    }
    let step = if input.shift { 1000 } else { 5000 };
    match input.key {
        "ArrowLeft" => Some(MediaKeyCommand::Seek { delta_ms: -step }),
        "ArrowRight" => Some(MediaKeyCommand::Seek { delta_ms: step }),
        "Home" => Some(MediaKeyCommand::SeekTo { position_ms: 0 }),
        "End" => duration_ms.map(|position_ms| MediaKeyCommand::SeekTo { position_ms }),
        _ => None,
    }
}

/// ⌨️ Applies a seek command to a position, clamped to the known duration.
pub fn seeked_media_position(command: MediaKeyCommand, position_ms: u64, duration_ms: Option<u64>) -> u64 {
    let target = match command {
        MediaKeyCommand::Seek { delta_ms } => (position_ms as i64).saturating_add(delta_ms).max(0) as u64,
        MediaKeyCommand::SeekTo { position_ms } => position_ms,
        MediaKeyCommand::Toggle => position_ms,
    };
    duration_ms.map_or(target, |duration| target.min(duration))
}

/// ⏱️ A media time as `h:mm:ss.mmm` (hours only when present) with the locale's decimal mark, never raw milliseconds.
pub fn format_media_time(milliseconds: u64, is_de: bool) -> String {
    let (hours, minutes, seconds) = (milliseconds / 3_600_000, milliseconds / 60_000 % 60, milliseconds / 1000 % 60);
    let mark = if is_de { ',' } else { '.' };
    let head = if hours > 0 { format!("{hours}:{minutes:02}") } else { minutes.to_string() };
    format!("{head}:{seconds:02}{mark}{:03}", milliseconds % 1000)
}

/// ✂️ The unit the cut works in and how many whole units the media holds per second, as a rational.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaTrimRate {
    pub units_per_second_numerator: u64,
    pub units_per_second_denominator: u64,
    pub total_units: u64,
}

/// ✂️ Maps a selected time range to the cut-action value `start-end` in whole units: the start floors, the end ceils, both clamped
/// to the units the media holds; `None` when the range holds no whole unit.
pub fn trim_value_for_selection(trim: &MediaTrimRate, start_ms: u64, end_ms: u64) -> Option<String> {
    let denominator = u128::from(trim.units_per_second_denominator.max(1)) * 1000;
    let units = |milliseconds: u64, ceil: bool| -> u64 {
        let scaled = u128::from(milliseconds) * u128::from(trim.units_per_second_numerator);
        let whole = if ceil { scaled.div_ceil(denominator) } else { scaled / denominator };
        (whole as u64).min(trim.total_units)
    };
    let (start, end) = (units(start_ms, false), units(end_ms, true));
    (start < end).then(|| format!("{start}-{end}"))
}

/// ⏯️ What advancing the clock reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaTransportEvent {
    None,
    Ended,
}

/// ⏯️ Playing state, position and selection of one media window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaTransport {
    pub duration_ms: Option<u64>,
    pub position_ms: u64,
    pub playing: bool,
    pub selection: Option<(u64, u64)>,
}

impl MediaTransport {
    /// 🏗️ A paused transport at the start of media of `duration_ms`.
    pub fn new(duration_ms: Option<u64>) -> Self {
        Self { duration_ms, position_ms: 0, playing: false, selection: None }
    }

    /// ▶️ Starts playing; from the end it starts over.
    pub fn play(&mut self) {
        if self.duration_ms.is_some_and(|duration| self.position_ms >= duration) {
            self.position_ms = 0;
        }
        self.playing = true;
    }

    /// ⏸️ Pauses.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// 🔀️ Plays when paused, pauses when playing.
    pub fn toggle(&mut self) {
        if self.playing { self.pause() } else { self.play() }
    }

    /// 🎯️ Moves to `position_ms`, clamped to the duration.
    pub fn seek(&mut self, position_ms: u64) {
        self.position_ms = self.duration_ms.map_or(position_ms, |duration| position_ms.min(duration));
    }

    /// ⌨️ Applies a keyboard command.
    pub fn apply(&mut self, command: MediaKeyCommand) {
        match command {
            MediaKeyCommand::Toggle => self.toggle(),
            _ => self.position_ms = seeked_media_position(command, self.position_ms, self.duration_ms),
        }
    }

    /// ✂️ Selects `start_ms..end_ms`, clamped to the duration and ordered.
    pub fn select(&mut self, start_ms: u64, end_ms: u64) {
        let clamp = |value: u64| self.duration_ms.map_or(value, |duration| value.min(duration));
        let (start, end) = (clamp(start_ms), clamp(end_ms));
        self.selection = Some((start.min(end), start.max(end)));
    }

    /// ⏱️ Advances the clock by `delta_ms` while playing; reaching the duration pauses and reports [`MediaTransportEvent::Ended`].
    pub fn advance(&mut self, delta_ms: u64) -> MediaTransportEvent {
        if !self.playing {
            return MediaTransportEvent::None;
        }
        self.position_ms = self.position_ms.saturating_add(delta_ms);
        if let Some(duration) = self.duration_ms.filter(|duration| self.position_ms >= *duration) {
            self.position_ms = duration;
            self.playing = false;
            return MediaTransportEvent::Ended;
        }
        MediaTransportEvent::None
    }
}
