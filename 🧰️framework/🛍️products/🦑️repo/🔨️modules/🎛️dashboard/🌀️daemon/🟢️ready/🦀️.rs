//! 🟢️ Readiness of a long-running task: the local web address its output announces. A task declares the
//! port it serves on; it offers a candidate when its visible output shows `http://127.0.0.1:<port>`,
//! `http://localhost:<port>` or `http://0.0.0.0:<port>` and the digits of the port end there. The ready
//! address is that match followed by the declared path, or with `printed` the whole printed address up
//! to the next whitespace.
//! The supervisor publishes readiness only after that exact local HTTP address responds with a 2xx
//! or 3xx status. Bounded concurrent probes run outside its event loop and cancel on stop or restart.
//!
//! The matcher reads visible text only and never sees terminal control sequences, so an address whose
//! port is drawn in bold is found like any other. It reads text in arbitrary chunks: an address that
//! reaches the end of the received text is decided by the next byte, by the end of the line or by
//! [`ReadyMatcher::settle`].
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🟢️ready/🥒️.feature

#[path = "📡️http/🦀️.rs"]
pub(super) mod http;

use super::ipc::Ready;

const HOSTS: [&str; 3] = ["127.0.0.1", "localhost", "0.0.0.0"];
const SCHEME: &[u8] = b"http://";
const LINE_BYTES: usize = 4096;
const KEPT_BYTES: usize = 64;
const ADDRESS_BYTES: usize = 2048;

enum Reading {
    Absent,
    More,
    Port(usize),
}

/// 🔎 Looks for the awaited address in the visible text of one task.
#[derive(Debug, Clone)]
pub struct ReadyMatcher {
    port: String,
    path: String,
    printed: bool,
    line: Vec<u8>,
    resume: usize,
    pending: Option<(usize, usize)>,
    found: Option<String>,
    done: bool,
}

impl ReadyMatcher {
    /// 🎯 A matcher for the address a task with this readiness announces.
    pub fn new(ready: &Ready) -> Self {
        Self { port: ready.port.to_string(), path: ready.path.clone(), printed: ready.printed, line: Vec::new(), resume: 0, pending: None, found: None, done: false }
    }

    /// 📖 Reads visible text, which carries no control byte; the address may be completed by it.
    pub fn text(&mut self, text: &[u8]) {
        if self.done || text.is_empty() { return; }
        self.line.extend_from_slice(text);
        self.search();
        if !self.done && self.pending.is_none() && self.line.len() > LINE_BYTES && self.resume >= self.line.len().saturating_sub(KEPT_BYTES) {
            let cut = self.line.len() - KEPT_BYTES;
            self.line.drain(..cut);
            self.resume = self.resume.saturating_sub(cut);
        }
    }

    /// ↩️ The text read so far ends here: a line, a tab or a carriage return.
    pub fn end_line(&mut self) {
        self.settle();
        self.line.clear();
        self.resume = 0;
        self.pending = None;
    }

    /// 🔗 Reads the target of a hyperlink as a line of its own, whatever text surrounds the link.
    pub fn link(&mut self, target: &[u8]) {
        let (line, resume, pending) = (std::mem::take(&mut self.line), std::mem::take(&mut self.resume), self.pending.take());
        self.text(target);
        self.settle();
        (self.line, self.resume, self.pending) = (line, resume, pending);
    }

    /// ✅ Accepts an address the text has ended in as if the line had ended.
    pub fn settle(&mut self) {
        if let Some((start, end)) = self.pending.take() { self.finish(start, end); }
    }

    /// ⏳ Whether the text ends in an address that a further byte could still change.
    pub fn undecided(&self) -> bool { !self.done && self.pending.is_some() }

    /// 🔭️ The ready address once it was found; a matcher finds one address in its life and hands it out once.
    pub fn take(&mut self) -> Option<String> { self.found.take() }

    fn finish(&mut self, start: usize, end: usize) {
        self.pending = None;
        let address = if self.printed {
            let tail = &self.line[end..];
            let length = tail.iter().position(u8::is_ascii_whitespace).unwrap_or(tail.len());
            String::from_utf8_lossy(&self.line[start..end + length]).into_owned()
        } else {
            format!("{}{}", String::from_utf8_lossy(&self.line[start..end]), self.path)
        };
        self.found = Some(address);
        self.done = true;
    }

    fn search(&mut self) {
        let mut from = self.resume.min(self.line.len());
        self.pending = None;
        while let Some(start) = find(&self.line[from..], SCHEME).map(|at| from + at) {
            match self.read(start) {
                Reading::Absent => from = start + 1,
                Reading::More => { self.resume = start; return; }
                Reading::Port(end) if !self.printed => {
                    if end == self.line.len() { self.pending = Some((start, end)); self.resume = start; } else { self.finish(start, end); }
                    return;
                }
                Reading::Port(end) => {
                    if self.line[end..].iter().any(u8::is_ascii_whitespace) || self.line.len() - start > ADDRESS_BYTES { self.finish(start, end); } else { self.pending = Some((start, end)); self.resume = start; }
                    return;
                }
            }
        }
        self.resume = from.max(self.line.len().saturating_sub(SCHEME.len() - 1));
    }

    fn read(&self, start: usize) -> Reading {
        let rest = &self.line[start + SCHEME.len()..];
        for host in HOSTS.map(str::as_bytes) {
            if !rest.starts_with(host) {
                if host.starts_with(rest) { return Reading::More; }
                continue;
            }
            let after = &rest[host.len()..];
            match after.first() {
                None => return Reading::More,
                Some(b':') => {}
                Some(_) => return Reading::Absent,
            }
            let digits = after[1..].iter().take_while(|byte| byte.is_ascii_digit()).count();
            let seen = &after[1..1 + digits];
            if seen == self.port.as_bytes() { return Reading::Port(start + SCHEME.len() + host.len() + 1 + digits); }
            return if 1 + digits == after.len() && self.port.as_bytes().starts_with(seen) { Reading::More } else { Reading::Absent };
        }
        Reading::Absent
    }
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}
