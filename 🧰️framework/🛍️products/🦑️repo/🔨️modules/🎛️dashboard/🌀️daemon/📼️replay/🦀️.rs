//! 📼 What a session has printed, kept so that it can be shown again: a tracker that follows the
//! terminal modes of the byte stream and marks the places a replay may start from, and the store that
//! keeps the recent output in memory and a longer tail in rotating files under the dashboard cache.
//!
//! A replay never starts inside an escape sequence or a character. It starts at a mark — the start of
//! a line — and is preceded by the sequences that re-establish what the output before the mark had
//! set: colours, the alternate screen, cursor and mouse modes, the scroll region, the character sets
//! and the title. The same tracker reads the title a child sets and finds the address a task
//! announces when it is ready.
//!
//! @see https://vt100.net/emu/dec_ansi_parser
//! @see https://invisible-island.net/xterm/ctlseqs/ctlseqs.html

use super::ipc::Ready;
use super::ready::ReadyMatcher;
use std::collections::{BTreeSet, VecDeque};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// 🧠 How much recent output of a session stays in memory; a replay is served from it.
pub const RING_BYTES: usize = 2 * 1024 * 1024;
/// 🗂️ The size of one log file of a session; the current file and the one before it are kept.
pub const SEGMENT_BYTES: u64 = 8 * 1024 * 1024;
/// 📍 The least distance between two places a replay may start from.
pub const MARK_SPACING: u64 = 64 * 1024;
/// 🖼️ The most output a replay of a full-screen program starts before the present to reach the moment it took the screen.
pub const SCREEN_REPLAY_BYTES: u64 = 8 * 1024 * 1024;
const TITLE_BYTES: usize = 512;
const SEQUENCE_BYTES: usize = 4096;
const DEFAULT_OFF_MODES: [u16; 16] = [1, 6, 47, 66, 1000, 1002, 1003, 1004, 1005, 1006, 1015, 1016, 1047, 1049, 2004, 2026];
const DEFAULT_ON_MODES: [u16; 2] = [7, 25];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Color {
    #[default]
    Default,
    Indexed(u8),
    Rgb(u8, u8, u8),
}

/// 🎨 The character attributes a stream has selected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Rendition {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: u8,
    blink: bool,
    reverse: bool,
    hidden: bool,
    strike: bool,
    foreground: Color,
    background: Color,
}

impl Rendition {
    fn apply(&mut self, parameters: &[u8]) {
        let text = std::str::from_utf8(parameters).unwrap_or("");
        let mut fields = text.split(';').map(|field| field.split(':').map(|part| part.parse::<u16>().unwrap_or(0)).collect::<Vec<_>>()).peekable();
        if text.is_empty() { *self = Self::default(); return; }
        while let Some(field) = fields.next() {
            let code = field[0];
            match code {
                0 => *self = Self::default(),
                1 => self.bold = true,
                2 => self.dim = true,
                3 => self.italic = true,
                4 => self.underline = field.get(1).map_or(1, |style| (*style).min(5) as u8),
                5 | 6 => self.blink = true,
                7 => self.reverse = true,
                8 => self.hidden = true,
                9 => self.strike = true,
                21 => self.underline = 2,
                22 => { self.bold = false; self.dim = false; }
                23 => self.italic = false,
                24 => self.underline = 0,
                25 => self.blink = false,
                27 => self.reverse = false,
                28 => self.hidden = false,
                29 => self.strike = false,
                30..=37 => self.foreground = Color::Indexed((code - 30) as u8),
                39 => self.foreground = Color::Default,
                40..=47 => self.background = Color::Indexed((code - 40) as u8),
                49 => self.background = Color::Default,
                90..=97 => self.foreground = Color::Indexed((code - 90 + 8) as u8),
                100..=107 => self.background = Color::Indexed((code - 100 + 8) as u8),
                38 | 48 => {
                    let mut rest: Vec<u16> = field[1..].to_vec();
                    if rest.is_empty() {
                        let kind = fields.next().map_or(0, |next| next[0]);
                        rest.push(kind);
                        for _ in 0..if kind == 2 { 3 } else { usize::from(kind == 5) } { rest.push(fields.next().map_or(0, |next| next[0])); }
                    } else if rest[0] == 2 && rest.len() >= 5 {
                        rest.remove(1);
                    }
                    let color = match rest.as_slice() {
                        [5, index, ..] => Color::Indexed(*index as u8),
                        [2, red, green, blue, ..] => Color::Rgb(*red as u8, *green as u8, *blue as u8),
                        _ => continue,
                    };
                    if code == 38 { self.foreground = color; } else { self.background = color; }
                }
                _ => {}
            }
        }
    }

    fn sequence(&self) -> String {
        let mut codes: Vec<String> = Vec::new();
        for (set, code) in [(self.bold, "1"), (self.dim, "2"), (self.italic, "3"), (self.blink, "5"), (self.reverse, "7"), (self.hidden, "8"), (self.strike, "9")] { if set { codes.push(code.into()); } }
        match self.underline { 0 => {} 1 => codes.push("4".into()), style => codes.push(format!("4:{style}")) }
        for (color, base) in [(self.foreground, 30u16), (self.background, 40u16)] {
            match color {
                Color::Default => {}
                Color::Indexed(index) if index < 8 => codes.push((base + u16::from(index)).to_string()),
                Color::Indexed(index) if index < 16 => codes.push((base + 60 + u16::from(index) - 8).to_string()),
                Color::Indexed(index) => codes.push(format!("{};5;{index}", base + 8)),
                Color::Rgb(red, green, blue) => codes.push(format!("{};2;{red};{green};{blue}", base + 8)),
            }
        }
        if codes.is_empty() { "\x1b[0m".into() } else { format!("\x1b[0;{}m", codes.join(";")) }
    }
}

/// 🎚️ Everything a terminal remembers besides its cells that a later byte depends on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Modes {
    rendition: Rendition,
    set: BTreeSet<u16>,
    reset: BTreeSet<u16>,
    insert: bool,
    scroll_region: Option<(u16, u16)>,
    charsets: [u8; 2],
    shifted: bool,
    cursor_style: Option<u16>,
    keypad: bool,
}

impl Modes {
    fn private(&mut self, mode: u16, on: bool) {
        if DEFAULT_OFF_MODES.contains(&mode) { if on { self.set.insert(mode); } else { self.set.remove(&mode); } }
        if DEFAULT_ON_MODES.contains(&mode) { if on { self.reset.remove(&mode); } else { self.reset.insert(mode); } }
    }

    fn soft_reset(&mut self) {
        self.rendition = Rendition::default();
        self.set.remove(&6);
        self.reset.clear();
        self.insert = false;
        self.scroll_region = None;
        self.charsets = [0; 2];
        self.shifted = false;
        self.keypad = false;
    }

    fn alternate(&self) -> bool { [47, 1047, 1049].iter().any(|mode| self.set.contains(mode)) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Ground,
    Escape,
    Intermediate,
    Control,
    Command,
    CommandEscape,
    Text,
    TextEscape,
}

/// 📌️ A place a replay may start from, and the bytes that restore what came before it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    pub offset: u64,
    pub preamble: Vec<u8>,
}

/// 🧭 Follows a terminal byte stream without drawing it.
pub struct Tracker {
    state: State,
    sequence: Vec<u8>,
    modes: Modes,
    title: Option<String>,
    title_changed: bool,
    matcher: Option<ReadyMatcher>,
    marks: Vec<Mark>,
    next_mark: u64,
    at: u64,
    escape_at: u64,
    screen: Option<Mark>,
}

impl Tracker {
    /// 🟢 `ready` names the address whose appearance in the output means the task is ready.
    pub fn new(ready: Option<&Ready>) -> Self {
        Self { state: State::Ground, sequence: Vec::new(), modes: Modes::default(), title: None, title_changed: false, matcher: ready.map(ReadyMatcher::new), marks: Vec::new(), next_mark: MARK_SPACING, at: 0, escape_at: 0, screen: None }
    }

    /// 🔁 Starts looking for a ready address again, as a restarted task announces it again.
    pub fn await_ready(&mut self, ready: Option<&Ready>) {
        self.matcher = ready.map(ReadyMatcher::new);
    }

    pub fn title(&self) -> Option<&str> { self.title.as_deref() }

    pub fn take_title_change(&mut self) -> bool { std::mem::take(&mut self.title_changed) }

    /// 📡️ The address the output showed, once.
    pub fn take_ready(&mut self) -> Option<String> { self.matcher.as_mut().and_then(ReadyMatcher::take) }

    /// ⏳ Whether the output ends in the awaited address without the byte that would prove it complete.
    pub fn ready_undecided(&self) -> bool { self.matcher.as_ref().is_some_and(ReadyMatcher::undecided) }

    /// ✅ Accepts an address the output has ended in for long enough.
    pub fn settle_ready(&mut self) {
        if let Some(matcher) = self.matcher.as_mut() { matcher.settle(); }
    }

    pub fn alternate_screen(&self) -> bool { self.modes.alternate() }

    /// 🪟️ Where the program that holds the alternate screen took it, and the modes before that.
    pub fn screen_entry(&self) -> Option<&Mark> { self.screen.as_ref() }

    pub fn take_marks(&mut self) -> Vec<Mark> { std::mem::take(&mut self.marks) }

    /// 🧵 Reads `data`, whose first byte is byte `offset` of the stream.
    pub fn feed(&mut self, data: &[u8], offset: u64) {
        let mut index = 0;
        while index < data.len() {
            if self.state == State::Ground {
                let run = data[index..].iter().position(|byte| *byte < 0x20).unwrap_or(data.len() - index);
                if run > 0 {
                    if let Some(matcher) = self.matcher.as_mut() { matcher.text(&data[index..index + run]); }
                    index += run;
                    let at = offset + index as u64;
                    if at >= self.next_mark + 4 * MARK_SPACING && data[index - 1] < 0x80 { self.mark(at); }
                    continue;
                }
            }
            let byte = data[index];
            self.at = offset + index as u64;
            index += 1;
            self.step(byte);
            if byte == b'\n' && self.state == State::Ground && offset + index as u64 >= self.next_mark { self.mark(offset + index as u64); }
        }
    }

    fn mark(&mut self, offset: u64) {
        self.marks.push(Mark { offset, preamble: self.preamble() });
        self.next_mark = offset + MARK_SPACING;
    }

    fn step(&mut self, byte: u8) {
        match self.state {
            State::Ground => match byte {
                0x1b => { self.escape_at = self.at; self.enter(State::Escape) }
                0x0e => self.modes.shifted = true,
                0x0f => self.modes.shifted = false,
                0x08 | 0x07 => {}
                _ => self.end_line(),
            },
            State::Escape => match byte {
                b'[' => self.enter(State::Control),
                b']' => self.enter(State::Command),
                b'P' | b'X' | b'^' | b'_' => self.enter(State::Text),
                0x20..=0x2f => { self.sequence.push(byte); self.state = State::Intermediate; }
                0x1b => {}
                _ => {
                    match byte { b'c' => self.modes = Modes::default(), b'=' => self.modes.keypad = true, b'>' => self.modes.keypad = false, _ => {} }
                    self.state = State::Ground;
                }
            },
            State::Intermediate => match byte {
                0x20..=0x2f => self.collect(byte),
                0x1b => self.enter(State::Escape),
                0x18 | 0x1a => self.state = State::Ground,
                _ => {
                    match self.sequence.first() { Some(b'(') => self.modes.charsets[0] = if byte == b'B' { 0 } else { byte }, Some(b')') => self.modes.charsets[1] = if byte == b'B' { 0 } else { byte }, _ => {} }
                    self.state = State::Ground;
                }
            },
            State::Control => match byte {
                0x40..=0x7e => { self.control(byte); self.state = State::Ground; }
                0x1b => self.enter(State::Escape),
                0x18 | 0x1a => self.state = State::Ground,
                _ => self.collect(byte),
            },
            State::Command => match byte {
                0x07 => { self.command(); self.state = State::Ground; }
                0x1b => self.state = State::CommandEscape,
                0x18 | 0x1a => self.state = State::Ground,
                _ => self.collect(byte),
            },
            State::CommandEscape => {
                self.command();
                if byte == b'\\' { self.state = State::Ground; } else { self.enter(State::Escape); self.step(byte); }
            }
            State::Text => match byte {
                0x1b => self.state = State::TextEscape,
                0x18 | 0x1a => self.state = State::Ground,
                _ => {}
            },
            State::TextEscape => if byte == b'\\' { self.state = State::Ground; } else { self.enter(State::Escape); self.step(byte); },
        }
    }

    fn enter(&mut self, state: State) {
        self.sequence.clear();
        self.state = state;
    }

    fn collect(&mut self, byte: u8) {
        if self.sequence.len() < SEQUENCE_BYTES { self.sequence.push(byte); }
    }

    fn control(&mut self, last: u8) {
        let sequence = std::mem::take(&mut self.sequence);
        let private = sequence.first() == Some(&b'?');
        let body = &sequence[usize::from(private)..];
        let numbers = || body.split(|byte| *byte == b';').map(|field| std::str::from_utf8(field).ok().and_then(|text| text.parse::<u16>().ok()).unwrap_or(0));
        match last {
            b'm' if !private && body.iter().all(|byte| byte.is_ascii_digit() || matches!(byte, b';' | b':')) => self.modes.rendition.apply(body),
            b'h' | b'l' if private => for mode in numbers() {
                let before = (!self.modes.alternate()).then(|| self.preamble());
                self.modes.private(mode, last == b'h');
                match (before, self.modes.alternate()) {
                    (Some(preamble), true) => self.screen = Some(Mark { offset: self.escape_at, preamble }),
                    (None, false) => self.screen = None,
                    _ => {}
                }
            },
            b'h' | b'l' if body == b"4" => self.modes.insert = last == b'h',
            b'r' if !private && body.iter().all(|byte| byte.is_ascii_digit() || *byte == b';') => {
                let mut bounds = numbers();
                let (top, bottom) = (bounds.next().unwrap_or(0), bounds.next().unwrap_or(0));
                self.modes.scroll_region = (top > 1 || bottom > 0).then_some((top.max(1), bottom));
            }
            b'q' if body.last() == Some(&b' ') => self.modes.cursor_style = std::str::from_utf8(&body[..body.len() - 1]).ok().and_then(|text| text.parse().ok()).filter(|style| *style > 0),
            b'p' if body == b"!" => self.modes.soft_reset(),
            _ => {}
        }
        self.sequence = sequence;
    }

    fn command(&mut self) {
        let sequence = std::mem::take(&mut self.sequence);
        if let Some(text) = sequence.strip_prefix(b"0;").or_else(|| sequence.strip_prefix(b"2;")) {
            let title: String = String::from_utf8_lossy(&text[..text.len().min(TITLE_BYTES)]).chars().filter(|character| !character.is_control()).collect();
            let title = (!title.is_empty()).then_some(title);
            if title != self.title { self.title = title; self.title_changed = true; }
        } else if let Some(link) = sequence.strip_prefix(b"8;").and_then(|rest| rest.iter().position(|byte| *byte == b';').map(|at| &rest[at + 1..])) {
            if let Some(matcher) = self.matcher.as_mut().filter(|_| !link.is_empty()) { matcher.link(link); }
        }
        self.sequence = sequence;
    }

    fn end_line(&mut self) {
        if let Some(matcher) = self.matcher.as_mut() { matcher.end_line(); }
    }

    /// 🎬 The bytes that bring a fresh terminal to the modes the stream has established so far.
    pub fn preamble(&self) -> Vec<u8> {
        let mut out = String::new();
        for (slot, opener) in [(0, '('), (1, ')')] { if self.modes.charsets[slot] != 0 { out.push_str(&format!("\x1b{opener}{}", self.modes.charsets[slot] as char)); } }
        if self.modes.shifted { out.push('\x0e'); }
        for mode in [1049, 1047, 47] { if self.modes.set.contains(&mode) { out.push_str(&format!("\x1b[?{mode}h")); } }
        for mode in self.modes.set.iter().filter(|mode| ![47, 1047, 1049].contains(*mode)) { out.push_str(&format!("\x1b[?{mode}h")); }
        for mode in &self.modes.reset { out.push_str(&format!("\x1b[?{mode}l")); }
        if self.modes.insert { out.push_str("\x1b[4h"); }
        if let Some((top, bottom)) = self.modes.scroll_region { out.push_str(&if bottom > 0 { format!("\x1b[{top};{bottom}r") } else { format!("\x1b[{top}r") }); }
        if let Some(style) = self.modes.cursor_style { out.push_str(&format!("\x1b[{style} q")); }
        if self.modes.keypad { out.push_str("\x1b="); }
        if let Some(title) = &self.title { out.push_str(&format!("\x1b]2;{title}\x07")); }
        if self.modes.rendition != Rendition::default() { out.push_str(&self.modes.rendition.sequence()); }
        out.into_bytes()
    }

    /// 🧽 The bytes that return a terminal from the modes the stream has established to its defaults,
    /// ending whatever sequence the stream stopped in; a new process then starts on a clean line.
    pub fn reset_sequence(&mut self) -> Vec<u8> {
        let mut out = String::new();
        if self.state != State::Ground { out.push('\x18'); }
        out.push_str("\x1b[0m");
        for mode in self.modes.set.iter().rev() { out.push_str(&format!("\x1b[?{mode}l")); }
        for mode in &self.modes.reset { out.push_str(&format!("\x1b[?{mode}h")); }
        if self.modes.insert { out.push_str("\x1b[4l"); }
        if self.modes.scroll_region.is_some() { out.push_str("\x1b[r"); }
        if self.modes.charsets != [0; 2] { out.push_str("\x1b(B\x1b)B"); }
        if self.modes.shifted { out.push('\x0f'); }
        if self.modes.cursor_style.is_some() { out.push_str("\x1b[0 q"); }
        if self.modes.keypad { out.push_str("\x1b>"); }
        out.push_str("\r\n");
        out.into_bytes()
    }
}

/// 🏷️ The file name stem of a session's logs: its readable characters plus a hash that keeps it unique.
pub fn log_stem(session_id: &str) -> String {
    let readable: String = session_id.chars().take(48).map(|character| if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') { character } else { '_' }).collect();
    format!("{readable}-{:016x}", super::ipc::stable_hash(session_id.as_bytes()))
}

fn segment_path(directory: &Path, stem: &str, segment: u64) -> PathBuf { directory.join(format!("{stem}.{segment}.log")) }

fn marks_path(directory: &Path, stem: &str) -> PathBuf { directory.join(format!("{stem}.marks")) }

/// 🔢 The retained log files of a session as `(segment, length)`, oldest first.
fn segments(directory: &Path, stem: &str) -> Vec<(u64, u64)> {
    let Ok(entries) = std::fs::read_dir(directory) else { return Vec::new() };
    let prefix = format!("{stem}.");
    let mut found: Vec<(u64, u64)> = entries.flatten().filter_map(|entry| {
        let name = entry.file_name();
        let segment = name.to_str()?.strip_prefix(&prefix)?.strip_suffix(".log")?.parse().ok()?;
        Some((segment, std::fs::metadata(entry.path()).ok()?.len()))
    }).collect();
    found.sort_unstable();
    found
}

/// 🧹 Deletes every log file in `directory` whose session is not in `keep`.
pub fn sweep_logs(directory: &Path, keep: &BTreeSet<String>) {
    let Ok(entries) = std::fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        let stem = name.strip_suffix(".marks").or_else(|| name.strip_suffix(".log").and_then(|rest| rest.rsplit_once('.')).map(|(stem, _)| stem));
        if stem.is_some_and(|stem| !keep.contains(stem)) { let _ = std::fs::remove_file(entry.path()); }
    }
}

/// 📚 What one read of a session's output found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fetch {
    Data,
    Current,
    Lost,
}

/// 🗃️ The output of one session by stream offset: the recent tail in memory, a longer tail on disk.
pub struct SessionLog {
    directory: PathBuf,
    stem: String,
    end: u64,
    ring: VecDeque<u8>,
    ring_start: u64,
    hot: bool,
    writer: Option<(u64, std::fs::File)>,
    unwritten: Vec<u8>,
    reader: Option<(u64, std::fs::File)>,
    marks: VecDeque<Mark>,
    marks_loaded: bool,
    tracker: Tracker,
    known: bool,
}

impl SessionLog {
    /// 🆕 The log of a session that starts now; what an earlier session of the same name left is removed.
    pub fn create(directory: &Path, session_id: &str, ready: Option<&Ready>) -> Self {
        let stem = log_stem(session_id);
        for (segment, _) in segments(directory, &stem) { let _ = std::fs::remove_file(segment_path(directory, &stem, segment)); }
        let _ = std::fs::remove_file(marks_path(directory, &stem));
        Self { directory: directory.to_path_buf(), stem, end: 0, ring: VecDeque::new(), ring_start: 0, hot: true, writer: None, unwritten: Vec::new(), reader: None, marks: VecDeque::new(), marks_loaded: true, tracker: Tracker::new(ready), known: true }
    }

    /// ♻️ The log an earlier daemon left for a session: files only, read when someone asks.
    pub fn restore(directory: &Path, session_id: &str) -> Self {
        let stem = log_stem(session_id);
        let end = segments(directory, &stem).last().map_or(0, |(segment, length)| segment * SEGMENT_BYTES + length);
        Self { directory: directory.to_path_buf(), stem, end, ring: VecDeque::new(), ring_start: end, hot: false, writer: None, unwritten: Vec::new(), reader: None, marks: VecDeque::new(), marks_loaded: false, tracker: Tracker::new(None), known: false }
    }

    pub fn end(&self) -> u64 { self.end }

    pub fn tracker(&mut self) -> &mut Tracker { &mut self.tracker }

    /// 🧼️ What to write between two runs of the same session so the second starts from terminal defaults.
    pub fn separator(&mut self) -> Vec<u8> {
        if self.end == 0 { return Vec::new(); }
        if self.known { self.tracker.reset_sequence() } else { self.known = true; self.tracker = Tracker::new(None); b"\x18\x1bc".to_vec() }
    }

    pub fn append(&mut self, data: &[u8]) {
        if data.is_empty() { return; }
        if !self.hot { self.hot = true; self.ring.clear(); self.ring_start = self.end; }
        self.tracker.feed(data, self.end);
        for mark in self.tracker.take_marks() {
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(marks_path(&self.directory, &self.stem)) { let _ = writeln!(file, "{} {}", mark.offset, hex(&mark.preamble)); }
            self.marks.push_back(mark);
        }
        self.ring.extend(data);
        let excess = self.ring.len().saturating_sub(RING_BYTES);
        self.ring.drain(..excess);
        self.ring_start += excess as u64;
        self.unwritten.extend_from_slice(data);
        self.end += data.len() as u64;
        if self.unwritten.len() >= 256 * 1024 { self.flush(); }
    }

    /// 📝 Whether output was appended that the log files do not hold yet.
    pub fn dirty(&self) -> bool { !self.unwritten.is_empty() }

    /// 💾 Hands what was appended since the last flush to the log files.
    pub fn flush(&mut self) {
        let mut from = self.end - self.unwritten.len() as u64;
        let unwritten = std::mem::take(&mut self.unwritten);
        let mut rest = &unwritten[..];
        while !rest.is_empty() {
            let segment = from / SEGMENT_BYTES;
            if self.writer.as_ref().map(|(current, _)| *current) != Some(segment) {
                let _ = std::fs::create_dir_all(&self.directory);
                self.writer = std::fs::OpenOptions::new().create(true).append(true).open(segment_path(&self.directory, &self.stem, segment)).ok().map(|file| (segment, file));
                if segment >= 2 { self.retire(segment - 1); }
            }
            let room = ((segment + 1) * SEGMENT_BYTES - from).min(rest.len() as u64) as usize;
            if let Some((_, file)) = self.writer.as_mut() { let _ = file.write_all(&rest[..room]); }
            rest = &rest[room..];
            from += room as u64;
        }
    }

    fn retire(&mut self, oldest_kept: u64) {
        for (segment, _) in segments(&self.directory, &self.stem) { if segment < oldest_kept { let _ = std::fs::remove_file(segment_path(&self.directory, &self.stem, segment)); } }
        let floor = oldest_kept * SEGMENT_BYTES;
        while self.marks.front().is_some_and(|mark| mark.offset < floor) { self.marks.pop_front(); }
        let text: String = self.marks.iter().map(|mark| format!("{} {}\n", mark.offset, hex(&mark.preamble))).collect();
        let _ = std::fs::write(marks_path(&self.directory, &self.stem), text);
    }

    /// 🧊 Gives the memory of an ended session back; its files keep answering.
    pub fn cool(&mut self) {
        self.flush();
        self.writer = None;
        self.ring = VecDeque::new();
        self.ring_start = self.end;
        self.hot = false;
    }

    /// 🗑️ Deletes the session's files.
    pub fn discard(&mut self) {
        self.unwritten.clear();
        self.writer = None;
        self.reader = None;
        for (segment, _) in segments(&self.directory, &self.stem) { let _ = std::fs::remove_file(segment_path(&self.directory, &self.stem, segment)); }
        let _ = std::fs::remove_file(marks_path(&self.directory, &self.stem));
    }

    /// ⏮️ The first offset that can still be read.
    pub fn oldest(&self) -> u64 {
        let on_disk = segments(&self.directory, &self.stem).first().map(|(segment, _)| segment * SEGMENT_BYTES);
        on_disk.unwrap_or(if self.hot { self.ring_start } else { self.end }).min(if self.hot { self.ring_start } else { u64::MAX })
    }

    /// 🎞️ Where a replay starts and what precedes it; the third value states that older output is not replayed.
    /// A program that holds the alternate screen is replayed from the moment it took the screen, so the
    /// screen it drew is rebuilt from its own first frame.
    pub fn replay(&mut self) -> (u64, Vec<u8>, bool) {
        if !self.marks_loaded {
            self.marks_loaded = true;
            let text = std::fs::read_to_string(marks_path(&self.directory, &self.stem)).unwrap_or_default();
            self.marks = text.lines().filter_map(|line| { let (offset, preamble) = line.split_once(' ')?; Some(Mark { offset: offset.parse().ok()?, preamble: unhex(preamble)? }) }).collect();
        }
        let oldest = self.oldest();
        let target = self.end.saturating_sub(RING_BYTES as u64).max(oldest);
        let base = if target == 0 { (0, Vec::new(), false) } else {
            match self.marks.iter().find(|mark| mark.offset >= target) {
                Some(mark) => (mark.offset, mark.preamble.clone(), true),
                None => (self.end, self.tracker.preamble(), true),
            }
        };
        match self.tracker.screen_entry().filter(|entry| entry.offset >= oldest && entry.offset < base.0 && self.end - entry.offset <= SCREEN_REPLAY_BYTES) {
            Some(entry) => (entry.offset, entry.preamble.clone(), entry.offset > 0),
            None => base,
        }
    }

    /// 📖 Appends up to `limit` bytes from `offset` to `out`.
    pub fn read(&mut self, offset: u64, limit: usize, out: &mut Vec<u8>) -> Fetch {
        if offset >= self.end { return Fetch::Current; }
        if self.hot && offset >= self.ring_start {
            let from = (offset - self.ring_start) as usize;
            let count = (self.ring.len() - from).min(limit);
            let (front, back) = self.ring.as_slices();
            if from < front.len() {
                let first = (front.len() - from).min(count);
                out.extend_from_slice(&front[from..from + first]);
                out.extend_from_slice(&back[..count - first]);
            } else {
                out.extend_from_slice(&back[from - front.len()..from - front.len() + count]);
            }
            return Fetch::Data;
        }
        self.flush();
        let segment = offset / SEGMENT_BYTES;
        if self.reader.as_ref().map(|(current, _)| *current) != Some(segment) {
            self.reader = std::fs::File::open(segment_path(&self.directory, &self.stem, segment)).ok().map(|file| (segment, file));
        }
        let Some((_, file)) = self.reader.as_mut() else { return Fetch::Lost };
        let count = ((segment + 1) * SEGMENT_BYTES).min(self.end).saturating_sub(offset).min(limit as u64) as usize;
        let start = out.len();
        out.resize(start + count, 0);
        if file.seek(SeekFrom::Start(offset % SEGMENT_BYTES)).is_err() || file.read_exact(&mut out[start..]).is_err() {
            out.truncate(start);
            self.reader = None;
            return Fetch::Lost;
        }
        Fetch::Data
    }

    /// 🔚 The last `limit` bytes still in memory.
    pub fn tail(&self, limit: usize) -> Vec<u8> {
        self.ring.iter().skip(self.ring.len().saturating_sub(limit)).copied().collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() { return "-".into(); }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Option<Vec<u8>> {
    if text == "-" { return Some(Vec::new()); }
    if !text.len().is_multiple_of(2) { return None; }
    (0..text.len() / 2).map(|index| u8::from_str_radix(text.get(index * 2..index * 2 + 2)?, 16).ok()).collect()
}

/// 📜 Reads the log files of a session in order, for `semio logs`; it follows the writer across files.
pub struct LogReader {
    directory: PathBuf,
    stem: String,
    offset: u64,
}

impl LogReader {
    pub fn open(directory: &Path, session_id: &str) -> Self {
        let stem = log_stem(session_id);
        let offset = segments(directory, &stem).first().map_or(0, |(segment, _)| segment * SEGMENT_BYTES);
        Self { directory: directory.to_path_buf(), stem, offset }
    }

    /// 🪜️ Appends what the files hold beyond what was read so far and answers how much that was.
    pub fn read(&mut self, out: &mut Vec<u8>) -> std::io::Result<usize> {
        let mut total = 0;
        loop {
            let retained = segments(&self.directory, &self.stem);
            let Some((oldest, _)) = retained.first().copied() else { return Ok(total) };
            if self.offset < oldest * SEGMENT_BYTES { self.offset = oldest * SEGMENT_BYTES; }
            let segment = self.offset / SEGMENT_BYTES;
            let Ok(mut file) = std::fs::File::open(segment_path(&self.directory, &self.stem, segment)) else { return Ok(total) };
            file.seek(SeekFrom::Start(self.offset % SEGMENT_BYTES))?;
            let count = file.take(SEGMENT_BYTES - self.offset % SEGMENT_BYTES).read_to_end(out)?;
            self.offset += count as u64;
            total += count;
            if count == 0 || !self.offset.is_multiple_of(SEGMENT_BYTES) { return Ok(total); }
        }
    }
}
