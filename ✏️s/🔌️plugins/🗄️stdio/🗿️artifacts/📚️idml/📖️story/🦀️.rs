//! 📖️ IDML story text: addressed `Content` runs read and replaced by byte-exact splice, so every other byte of every part is preserved.
//! See <https://www.adobe.com/devnet/indesign/documentation.html> for the IDML story model.
use semio_s_artifact_stdio_idml::{IdmlMutation, IdmlSnapshot};

pub const STORY_PREFIX: &str = "Stories/";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoryContent {
    pub part: String,
    pub index: usize,
    pub revision: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoryError {
    MissingPart,
    NotAStory,
    MissingContent,
    StaleRevision,
    InvalidText,
    MalformedXml,
}

impl StoryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPart => "stdio.idml.story.missing-part",
            Self::NotAStory => "stdio.idml.story.not-a-story",
            Self::MissingContent => "stdio.idml.story.missing-content",
            Self::StaleRevision => "stdio.idml.story.stale-revision",
            Self::InvalidText => "stdio.idml.story.invalid-text",
            Self::MalformedXml => "stdio.idml.story.malformed-xml",
        }
    }
}

struct Span {
    start: usize,
    end: usize,
    empty_element: bool,
}

/// 🔏️ A stable FNV-1a 64 revision of one part's exact bytes.
pub fn part_revision(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn skip_to(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes[from..].windows(needle.len()).position(|window| window == needle).map(|offset| from + offset + needle.len())
}

fn tag_end(bytes: &[u8], from: usize) -> Option<usize> {
    let mut quote = 0u8;
    for (offset, byte) in bytes[from..].iter().enumerate() {
        match (quote, *byte) {
            (0, b'"' | b'\'') => quote = *byte,
            (0, b'>') => return Some(from + offset),
            (open, current) if open != 0 && open == current => quote = 0,
            _ => {}
        }
    }
    None
}

fn content_spans(bytes: &[u8]) -> Result<Vec<Span>, StoryError> {
    let mut spans = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = bytes[cursor..].iter().position(|byte| *byte == b'<') {
        let open = cursor + offset;
        let rest = &bytes[open..];
        if rest.starts_with(b"<!--") {
            cursor = skip_to(bytes, open + 4, b"-->").ok_or(StoryError::MalformedXml)?;
        } else if rest.starts_with(b"<![CDATA[") {
            cursor = skip_to(bytes, open + 9, b"]]>").ok_or(StoryError::MalformedXml)?;
        } else if rest.starts_with(b"<?") {
            cursor = skip_to(bytes, open + 2, b"?>").ok_or(StoryError::MalformedXml)?;
        } else if rest.starts_with(b"<Content") && matches!(rest.get(8), Some(b'>' | b'/' | b' ' | b'\t' | b'\r' | b'\n')) {
            let close = tag_end(bytes, open).ok_or(StoryError::MalformedXml)?;
            if bytes[close - 1] == b'/' {
                spans.push(Span { start: close + 1, end: close + 1, empty_element: true });
                cursor = close + 1;
            } else {
                let end = skip_to(bytes, close + 1, b"</Content>").ok_or(StoryError::MalformedXml)? - b"</Content>".len();
                spans.push(Span { start: close + 1, end, empty_element: false });
                cursor = end;
            }
        } else {
            cursor = tag_end(bytes, open).ok_or(StoryError::MalformedXml)? + 1;
        }
    }
    Ok(spans)
}

fn unescape(raw: &str) -> Option<String> {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(position) = rest.find('&') {
        out.push_str(&rest[..position]);
        let tail = &rest[position + 1..];
        let semicolon = tail.find(';')?;
        let name = &tail[..semicolon];
        match name {
            "amp" => out.push('&'),
            "lt" => out.push('<'),
            "gt" => out.push('>'),
            "quot" => out.push('"'),
            "apos" => out.push('\''),
            _ => {
                let digits = name.strip_prefix('#')?;
                let code = match digits.strip_prefix('x') {
                    Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                    None => digits.parse().ok()?,
                };
                out.push(char::from_u32(code)?);
            }
        }
        rest = &tail[semicolon + 1..];
    }
    out.push_str(rest);
    Some(out)
}

fn escape(text: &str) -> Result<String, StoryError> {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\t' | '\n' | '\r' => out.push(character),
            control if (control as u32) < 0x20 || control == '\u{fffe}' || control == '\u{ffff}' => return Err(StoryError::InvalidText),
            other => out.push(other),
        }
    }
    Ok(out)
}

fn is_story(path: &str) -> bool {
    path.starts_with(STORY_PREFIX) && path.ends_with(".xml")
}

/// 📖️ Lists every `Content` run of every story part in package order.
pub fn story_contents(snapshot: &IdmlSnapshot) -> Vec<StoryContent> {
    snapshot
        .parts
        .iter()
        .filter(|part| is_story(&part.path))
        .flat_map(|part| {
            let revision = part_revision(&part.bytes);
            let spans = content_spans(&part.bytes).unwrap_or_default();
            spans.into_iter().enumerate().filter_map(move |(index, span)| {
                let raw = std::str::from_utf8(&part.bytes[span.start..span.end]).ok()?;
                Some(StoryContent { part: part.path.clone(), index, revision: revision.clone(), text: unescape(raw)? })
            })
        })
        .collect()
}

/// 📚️ Groups the story runs by story part, each group in run order.
pub fn story_pages(snapshot: &IdmlSnapshot) -> Vec<Vec<StoryContent>> {
    let mut pages: Vec<Vec<StoryContent>> = Vec::new();
    for run in story_contents(snapshot) {
        match pages.last_mut() {
            Some(page) if page[0].part == run.part => page.push(run),
            _ => pages.push(vec![run]),
        }
    }
    pages
}

/// ✏️ Builds the `ReplacePart` mutation that sets one addressed run's text; `Ok(None)` when the text is unchanged.
pub fn set_story_content(snapshot: &IdmlSnapshot, part: &str, index: usize, revision: &str, text: &str) -> Result<Option<IdmlMutation>, StoryError> {
    let current = snapshot.parts.iter().find(|candidate| candidate.path == part).ok_or(StoryError::MissingPart)?;
    if !is_story(part) {
        return Err(StoryError::NotAStory);
    }
    if part_revision(&current.bytes) != revision {
        return Err(StoryError::StaleRevision);
    }
    let spans = content_spans(&current.bytes)?;
    let span = spans.get(index).ok_or(StoryError::MissingContent)?;
    let escaped = escape(text)?;
    let existing = std::str::from_utf8(&current.bytes[span.start..span.end]).map_err(|_| StoryError::MalformedXml)?;
    if !span.empty_element && unescape(existing).as_deref() == Some(text) {
        return Ok(None);
    }
    let mut bytes = Vec::with_capacity(current.bytes.len() + escaped.len() + 16);
    if span.empty_element {
        let open = current.bytes[..span.start].iter().rposition(|byte| *byte == b'/').ok_or(StoryError::MalformedXml)?;
        bytes.extend_from_slice(&current.bytes[..open]);
        bytes.push(b'>');
        bytes.extend_from_slice(escaped.as_bytes());
        bytes.extend_from_slice(b"</Content>");
    } else {
        bytes.extend_from_slice(&current.bytes[..span.start]);
        bytes.extend_from_slice(escaped.as_bytes());
    }
    bytes.extend_from_slice(&current.bytes[span.end..]);
    Ok(Some(IdmlMutation::ReplacePart { path: part.into(), bytes }))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
