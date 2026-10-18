//! 🏠️ Build-tool-neutral workspace identity: regular build and package markers plus an explicit workspace declaration,
//! mirroring `findWorkspaceRoot` of `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts` and the vectors of `🧬️schema/🔣️.json`.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// 🪪️ Requires regular build and package markers with an explicit workspace declaration.
pub fn is_workspace_root(root: &Path, declares_workspaces: impl FnOnce(&str) -> Result<bool, String>) -> Result<bool, String> {
    if !["📋️project.json", "package.json"].iter().all(|name| fs::symlink_metadata(root.join(name)).is_ok_and(|metadata| metadata.file_type().is_file())) { return Ok(false); }
    declares_workspaces(&fs::read_to_string(root.join("package.json")).map_err(|error| error.to_string())?)
}

/// 🧭️ Nearest ancestor of `start` (itself included, at most 32 levels) that is a workspace root; a malformed `package.json` beside a build marker refuses instead of continuing upward.
pub fn find_workspace_root(start: &Path) -> Result<Option<PathBuf>, String> {
    for directory in start.ancestors().take(32) {
        if is_workspace_root(directory, declares_workspaces)? { return Ok(Some(directory.to_path_buf())); }
    }
    Ok(None)
}

/// 📜️ Whether a `package.json` text is one JSON document whose top-level `workspaces` member is an array or an object; the last duplicate member wins.
pub fn declares_workspaces(source: &str) -> Result<bool, String> {
    let mut scan = Scan { bytes: source.as_bytes(), at: 0 };
    scan.space();
    let mut declared = false;
    let top = scan.value(0, Some(&mut declared))?;
    scan.space();
    if scan.at != scan.bytes.len() { return Err("package.json holds trailing characters".into()); }
    Ok(top == b'{' && declared)
}

struct Scan<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Scan<'_> {
    fn space(&mut self) {
        while matches!(self.bytes.get(self.at), Some(b' ' | b'\t' | b'\n' | b'\r')) { self.at += 1; }
    }

    fn take(&mut self, expected: u8) -> Result<(), String> {
        if self.bytes.get(self.at) == Some(&expected) { self.at += 1; Ok(()) } else { Err(format!("package.json expects '{}' at byte {}", expected as char, self.at)) }
    }

    fn digits(&mut self) -> Result<(), String> {
        let from = self.at;
        while matches!(self.bytes.get(self.at), Some(b'0'..=b'9')) { self.at += 1; }
        if self.at == from { Err(format!("package.json expects a digit at byte {from}")) } else { Ok(()) }
    }

    fn string(&mut self) -> Result<&[u8], String> {
        self.take(b'"')?;
        let from = self.at;
        loop {
            match self.bytes.get(self.at).copied() {
                None | Some(0..=0x1f) => return Err(format!("package.json holds an unterminated string at byte {from}")),
                Some(b'"') => { self.at += 1; return Ok(&self.bytes[from..self.at - 1]); }
                Some(b'\\') => {
                    let width = match self.bytes.get(self.at + 1) {
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => 2,
                        Some(b'u') if self.bytes.get(self.at + 2..self.at + 6).is_some_and(|hex| hex.iter().all(u8::is_ascii_hexdigit)) => 6,
                        _ => return Err(format!("package.json holds an invalid escape at byte {}", self.at)),
                    };
                    self.at += width;
                }
                Some(_) => self.at += 1,
            }
        }
    }

    fn value(&mut self, depth: usize, mut workspaces: Option<&mut bool>) -> Result<u8, String> {
        if depth > 512 { return Err("package.json nests too deeply".into()); }
        self.space();
        let first = *self.bytes.get(self.at).ok_or("package.json ends before a value")?;
        match first {
            b'{' => {
                self.at += 1;
                self.space();
                if self.bytes.get(self.at) == Some(&b'}') { self.at += 1; return Ok(first); }
                loop {
                    self.space();
                    let named = self.string()? == "workspaces".as_bytes();
                    self.space();
                    self.take(b':')?;
                    self.space();
                    let kind = self.value(depth + 1, None)?;
                    if named { if let Some(declared) = workspaces.as_deref_mut() { *declared = matches!(kind, b'[' | b'{'); } }
                    self.space();
                    if self.bytes.get(self.at) == Some(&b',') { self.at += 1; } else { return self.take(b'}').map(|()| first); }
                }
            }
            b'[' => {
                self.at += 1;
                self.space();
                if self.bytes.get(self.at) == Some(&b']') { self.at += 1; return Ok(first); }
                loop {
                    self.value(depth + 1, None)?;
                    self.space();
                    if self.bytes.get(self.at) == Some(&b',') { self.at += 1; } else { return self.take(b']').map(|()| first); }
                }
            }
            b'"' => self.string().map(|_| first),
            b't' | b'f' | b'n' => {
                let word: &[u8] = match first { b't' => b"true", b'f' => b"false", _ => b"null" };
                if self.bytes[self.at..].starts_with(word) { self.at += word.len(); Ok(first) } else { Err(format!("package.json holds an invalid literal at byte {}", self.at)) }
            }
            b'-' | b'0'..=b'9' => {
                if first == b'-' { self.at += 1; }
                if self.bytes.get(self.at) == Some(&b'0') { self.at += 1; } else { self.digits()?; }
                if self.bytes.get(self.at) == Some(&b'.') { self.at += 1; self.digits()?; }
                if matches!(self.bytes.get(self.at), Some(b'e' | b'E')) {
                    self.at += 1;
                    if matches!(self.bytes.get(self.at), Some(b'+' | b'-')) { self.at += 1; }
                    self.digits()?;
                }
                Ok(b'0')
            }
            _ => Err(format!("package.json holds an unexpected byte at {}", self.at)),
        }
    }
}
