//! 🖥️ A third-party terminal for the journeys: `portable-pty` drives the real binary in a pseudo-terminal
//! (ConPTY on Windows) and `vt100` models what its screen shows, so no assertion trusts the dashboard's own
//! terminal code.
//!
//! @see https://docs.rs/portable-pty
//! @see https://docs.rs/vt100

use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// ⌨️ The prefix key of the dashboard: Ctrl+B.
pub const CTRL_B: &[u8] = &[0x02];
pub const ENTER: &[u8] = b"\r";
pub const ESCAPE: &[u8] = &[0x1b];

/// 🧪️ One running binary in a pseudo-terminal with the screen it has painted so far.
pub struct Terminal {
    master: Box<dyn MasterPty + Send>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
    screen: Arc<Mutex<vt100::Parser>>,
    raw: Arc<Mutex<Vec<u8>>>,
}

impl Terminal {
    /// 🚀️ Starts `binary args…` in `cwd` with the given extra environment on a `rows` x `cols` screen.
    pub fn spawn(binary: &Path, args: &[&str], cwd: &Path, env: &[(String, String)], rows: u16, cols: u16) -> Self {
        let pair = native_pty_system().openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 }).expect("open a pseudo-terminal");
        let mut command = CommandBuilder::new(binary);
        command.args(args);
        command.cwd(cwd);
        command.env("TERM", "xterm-256color");
        command.env("COLORTERM", "truecolor");
        command.env_remove("NO_COLOR");
        for (key, value) in env { command.env(key, value); }
        let child = pair.slave.spawn_command(command).expect("start the binary in the pseudo-terminal");
        drop(pair.slave);
        let mut reader = pair.master.try_clone_reader().expect("read the pseudo-terminal");
        let writer: Arc<Mutex<Box<dyn Write + Send>>> = Arc::new(Mutex::new(pair.master.take_writer().expect("write the pseudo-terminal")));
        let screen = Arc::new(Mutex::new(vt100::Parser::new(rows, cols, 2000)));
        let raw = Arc::new(Mutex::new(Vec::new()));
        let (feed, record, answer) = (Arc::clone(&screen), Arc::clone(&raw), Arc::clone(&writer));
        std::thread::spawn(move || {
            let mut buffer = [0u8; 16 * 1024];
            while let Ok(count) = reader.read(&mut buffer) {
                if count == 0 { break; }
                record.lock().unwrap().extend_from_slice(&buffer[..count]);
                let mut parser = feed.lock().unwrap();
                parser.process(&buffer[..count]);
                let (row, column) = parser.screen().cursor_position();
                drop(parser);
                if let Some(reply) = replies(&buffer[..count], row, column) {
                    let mut writer = answer.lock().unwrap();
                    let _ = writer.write_all(&reply).and_then(|()| writer.flush());
                }
            }
        });
        Self { master: pair.master, writer, child, screen, raw }
    }

    /// 🎹 Types bytes exactly as a keyboard would.
    pub fn send(&mut self, bytes: &[u8]) {
        let mut writer = self.writer.lock().unwrap();
        writer.write_all(bytes).expect("write to the pseudo-terminal");
        writer.flush().expect("flush the pseudo-terminal");
    }

    /// 🔤️ Types text one character at a time, so the dashboard sees separate key events.
    pub fn type_text(&mut self, text: &str) {
        for character in text.chars() {
            let mut buffer = [0u8; 4];
            self.send(character.encode_utf8(&mut buffer).as_bytes());
            std::thread::sleep(Duration::from_millis(15));
        }
    }

    /// 🔣 `Ctrl+B` then one key.
    pub fn leader(&mut self, key: &str) {
        self.send(CTRL_B);
        std::thread::sleep(Duration::from_millis(60));
        self.type_text(key);
    }

    /// 📺️ The visible screen as rows of text.
    pub fn rows(&self) -> Vec<String> {
        let screen = self.screen.lock().unwrap();
        let (rows, cols) = screen.screen().size();
        screen.screen().rows(0, cols).take(rows as usize).collect()
    }

    /// 🖼️ The visible screen as one text.
    pub fn text(&self) -> String { self.rows().iter().map(|row| row.trim_end()).collect::<Vec<_>>().join("\n") }

    /// 🎨️ Debug text: the background colour and inverse flag of each cell of `row` (first `cols` cells).
    pub fn styles(&self, row: u16, cols: u16) -> String {
        let screen = self.screen.lock().unwrap();
        (0..cols).map(|col| screen.screen().cell(row, col).map_or('?', |cell| if cell.inverse() { 'I' } else if !matches!(cell.bgcolor(), vt100::Color::Default) { 'b' } else if cell.bold() { 'B' } else { '.' })).collect()
    }

    /// 🎨️ Exact independent cell attributes for a row, encoded without exposing the oracle's types.
    pub fn attributes(&self, row: u16, cols: u16) -> Vec<String> {
        let screen = self.screen.lock().unwrap();
        (0..cols).map(|col| screen.screen().cell(row, col).map_or_else(String::new, |cell| format!("{:?}/{:?}/{}/{}", cell.fgcolor(), cell.bgcolor(), cell.inverse(), cell.bold()))).collect()
    }
    /// 📐 The size the terminal model has.
    pub fn size(&self) -> (u16, u16) { self.screen.lock().unwrap().screen().size() }

    /// 🕯️ The independent screen model's hardware cursor state and position.
    pub fn cursor(&self) -> (bool, (u16, u16)) { let screen = self.screen.lock().unwrap(); (!screen.screen().hide_cursor(), screen.screen().cursor_position()) }

    /// 📜️ Every byte received so far.
    pub fn raw_bytes(&self) -> Vec<u8> { self.raw.lock().unwrap().clone() }

    /// 🔡 Every byte received so far, lossily decoded.
    pub fn transcript(&self) -> String { String::from_utf8_lossy(&self.raw.lock().unwrap()).into_owned() }

    /// ⏳️ Waits until the screen contains `needle`; fails with the screen when it does not within `limit`.
    pub fn wait_for(&self, needle: &str, limit: Duration) -> Result<String, String> {
        self.wait_until(&format!("text {needle:?}"), limit, |text| text.contains(needle))
    }

    /// ⏲️ Waits until `accept` holds for the screen text.
    pub fn wait_until(&self, what: &str, limit: Duration, accept: impl Fn(&str) -> bool) -> Result<String, String> {
        let deadline = Instant::now() + limit;
        loop {
            let text = self.text();
            if accept(&text) { return Ok(text); }
            if Instant::now() >= deadline {
                if let Some(directory) = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR") {
                    let directory = std::path::PathBuf::from(directory);
                    let _ = std::fs::create_dir_all(&directory);
                    let _ = std::fs::write(directory.join(format!("pty-{}-failure.bin", std::process::id())), self.raw_bytes());
                    let _ = std::fs::write(directory.join(format!("pty-{}-failure.txt", std::process::id())), &text);
                }
                return Err(format!("timed out after {limit:?} waiting for {what}; the screen shows:\n{text}"));
            }
            std::thread::sleep(Duration::from_millis(40));
        }
    }

    /// 📏️ Resizes the pseudo-terminal and the screen model together.
    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.master.resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 }).expect("resize the pseudo-terminal");
        self.screen.lock().unwrap().screen_mut().set_size(rows, cols);
    }

    /// 🏁️ The exit code once the binary ended within `limit`.
    pub fn exit_code(&mut self, limit: Duration) -> Option<u32> {
        let deadline = Instant::now() + limit;
        loop {
            if let Ok(Some(status)) = self.child.try_wait() { return Some(status.exit_code()); }
            if Instant::now() >= deadline { return None; }
            std::thread::sleep(Duration::from_millis(40));
        }
    }

    /// 🔪️ Ends the binary when it still runs.
    pub fn kill(&mut self) { let _ = self.child.kill(); }
}

/// 💬️ What a real terminal emulator answers to the queries in `output`: the cursor position report that
/// ConPTY waits for before it lets a program paint, and the primary device attributes.
fn replies(output: &[u8], row: u16, column: u16) -> Option<Vec<u8>> {
    let text = String::from_utf8_lossy(output);
    let mut answer = Vec::new();
    for _ in text.matches("\u{1b}[6n") { answer.extend_from_slice(format!("\u{1b}[{};{}R", row + 1, column + 1).as_bytes()); }
    for _ in text.matches("\u{1b}[c") { answer.extend_from_slice(b"\x1b[?62;c"); }
    (!answer.is_empty()).then_some(answer)
}

impl Drop for Terminal {
    fn drop(&mut self) { self.kill(); }
}
