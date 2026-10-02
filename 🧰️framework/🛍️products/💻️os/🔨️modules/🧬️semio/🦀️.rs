//! 🧬️ Universal `.semio` container: content-derived envelope for every OS artifact encoding.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️Errors
/// ⚠️ Envelope parse or registry lookup failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemioError {
    InvalidPreamble(String),
    InvalidBinaryHeader(String),
    UnknownEnvelope(String),
    AmbiguousEnvelope,
    DecodingControl(String),
}

impl std::fmt::Display for SemioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPreamble(detail) => write!(formatter, "invalid semio preamble: {detail}"),
            Self::InvalidBinaryHeader(detail) => write!(formatter, "invalid binary semio header: {detail}"),
            Self::UnknownEnvelope(detail) => write!(formatter, "unknown semio envelope: {detail}"),
            Self::AmbiguousEnvelope => formatter.write_str("ambiguous semio envelope match"),
            Self::DecodingControl(detail) => formatter.write_str(detail),
        }
    }
}

impl std::error::Error for SemioError {}

pub type SemioResult<T> = Result<T, SemioError>;
//#endregion 🔖️Errors

//#region 🔖️Component
/// 🧩 Which constitutional encoding a `.semio` file carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Dsl,
    Pack,
    Op,
    Spr,
    Cmd,
}

impl Component {
    /// 🏷️ Wire token in the preamble and filename segment.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dsl => "dsl",
            Self::Pack => "pack",
            Self::Op => "op",
            Self::Spr => "spr",
            Self::Cmd => "cmd",
        }
    }

    /// 📖️ Parses a component token from preamble or filename.
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "dsl" => Some(Self::Dsl),
            "pack" => Some(Self::Pack),
            "op" => Some(Self::Op),
            "spr" => Some(Self::Spr),
            "cmd" => Some(Self::Cmd),
            _ => None,
        }
    }

    /// 📝 Whether this component uses a text preamble rather than a binary header.
    pub const fn is_text(self) -> bool {
        matches!(self, Self::Dsl | Self::Op | Self::Cmd)
    }
}
//#endregion 🔖️Component

//#region 🔖️Envelope
/// 📨 Identity of a `.semio` payload — derived from content, not from the filename.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SemioEnvelope {
    pub plugin: String,
    pub artifact: String,
    pub component: Component,
    pub version: u16,
}

impl SemioEnvelope {
    /// 🪪️ Matches the complete container identity before a domain codec reads its body.
    pub fn matches_identity(&self, envelope_id: &str, component: Component, version: u16) -> bool {
        envelope_id.strip_prefix(&self.plugin).and_then(|rest| rest.strip_prefix('.')) == Some(self.artifact.as_str()) && self.component == component && self.version == version
    }

    /// 🪪️ Dotted artifact id (`plugin.artifact`) used in `ArtifactDsl::ENVELOPE_ID`.
    pub fn envelope_id(&self) -> String {
        format!("{}.{}", self.plugin, self.artifact)
    }

    /// 📜️ Full preamble line for text encodings, e.g. `semio gis.gismap.dsl v1`.
    pub fn preamble_line(&self) -> String {
        format!("semio {}.{}.{} v{}", self.plugin, self.artifact, self.component.as_str(), self.version)
    }

    /// 🧬️ Binary envelope token without the `semio` keyword.
    pub fn binary_token(&self) -> String {
        format!("{}.{}.{} v{}", self.plugin, self.artifact, self.component.as_str(), self.version)
    }

    /// 📖️ Parses `plugin.artifact` from a document type id.
    pub fn from_envelope_id(envelope_id: &str, component: Component, version: u16) -> SemioResult<Self> {
        let (plugin, artifact) = envelope_id.split_once('.').ok_or_else(|| SemioError::InvalidPreamble(format!("envelope id must be plugin.artifact, got {envelope_id}")))?;
        Ok(Self { plugin: plugin.to_string(), artifact: artifact.to_string(), component, version })
    }
}
//#endregion 🔖️Envelope

//#region 🔖️Binary
/// 🧲️ Magic prefix for binary `.semio` files (`0x89` keeps them non-UTF-8).
pub const BINARY_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

const BINARY_HEADER_PREFIX_LEN: usize = 8 + 4;

/// 📏️ Counts the exact declared framing bytes without allocating an identity token.
pub fn declared_envelope_prefix_len(envelope_id:&str,component:Component,version:u16)->Result<usize,String>{
    if !envelope_id.contains('.') { return Err("invalid declared envelope identity".into()); }
    let mut digits=1usize;let mut remaining=version;while remaining>=10{digits+=1;remaining/=10;}
    let token=envelope_id.len().checked_add(component.as_str().len()).and_then(|length|length.checked_add(3+digits)).ok_or("native envelope identity length overflow")?;
    token.checked_add(if component.is_text(){7}else{BINARY_HEADER_PREFIX_LEN}).ok_or_else(||"native envelope prefix length overflow".into())
}

/// 📦️ Wraps a binary payload with the semio binary header.
pub fn wrap_binary(envelope: &SemioEnvelope, payload: &[u8]) -> Vec<u8> {
    let token = envelope.binary_token();
    let token_bytes = token.as_bytes();
    let mut out = Vec::with_capacity(BINARY_HEADER_PREFIX_LEN + token_bytes.len() + payload.len());
    out.extend_from_slice(&BINARY_MAGIC);
    out.extend_from_slice(&(token_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(token_bytes);
    out.extend_from_slice(payload);
    out
}

/// 🛫️ Emits an exact declared binary envelope without allocating an unchecked identity token.
pub fn wrap_binary_controlled(envelope_id:&str,component:Component,version:u16,payload:&[u8],control:&mut crate::os_dsl::NativeEncodeControl<'_>)->Result<Vec<u8>,String>{
    control.scoped_stage(|control|{control.checkpoint()?;if !envelope_id.contains('.')||component.is_text(){return Err("invalid declared binary envelope identity".into())}let mut digits=[0u8;5];let mut start=digits.len();let mut value=version;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}let digits=&digits[start..];let length=envelope_id.len().checked_add(component.as_str().len()+3+digits.len()).ok_or("native envelope identity length overflow")?;let token=u32::try_from(length).map_err(|_|"native envelope identity exceeds u32")?;let total=BINARY_HEADER_PREFIX_LEN.checked_add(length).and_then(|length|length.checked_add(payload.len())).ok_or("native binary envelope length overflow")?;let mut output=control.allocate_vec::<u8>(total)?;output.extend_from_slice(&BINARY_MAGIC);output.extend_from_slice(&token.to_le_bytes());for bytes in [envelope_id.as_bytes(),b".",component.as_str().as_bytes(),b" v",digits,payload]{control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for fragment in bytes.chunks(65536){output.extend_from_slice(fragment);control.advance(fragment.len())?;}Ok::<_,String>(())})?;}Ok(output)})
}

/// 📖️ Strips the semio binary header and returns envelope + inner payload.
pub fn unwrap_binary(bytes: &[u8]) -> SemioResult<(SemioEnvelope, Vec<u8>)> {
    if bytes.len() < BINARY_HEADER_PREFIX_LEN {
        return Err(SemioError::InvalidBinaryHeader("truncated".into()));
    }
    if bytes[0..8] != BINARY_MAGIC {
        return Err(SemioError::InvalidBinaryHeader("bad magic".into()));
    }
    let token_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let token_end = BINARY_HEADER_PREFIX_LEN + token_len;
    if bytes.len() < token_end {
        return Err(SemioError::InvalidBinaryHeader("truncated token".into()));
    }
    let token = std::str::from_utf8(&bytes[BINARY_HEADER_PREFIX_LEN..token_end]).map_err(|_| SemioError::InvalidBinaryHeader("token not utf-8".into()))?;
    let envelope = parse_binary_token(token)?;
    let payload = bytes[token_end..].to_vec();
    Ok((envelope, payload))
}

/// 🚦️ Admits the exact declared envelope and borrows its binary body without a payload copy.
pub fn unwrap_binary_controlled<'input>(bytes:&'input[u8],envelope_id:&str,component:Component,version:u16,control:&mut crate::os_dsl::NativeDecodeControl<'_>)->SemioResult<&'input[u8]>{
    control.checkpoint().map_err(SemioError::DecodingControl)?;
    if bytes.len()<BINARY_HEADER_PREFIX_LEN||bytes[..8]!=BINARY_MAGIC{return Err(SemioError::InvalidBinaryHeader("invalid binary envelope prefix".into()));}
    let length=u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let end=BINARY_HEADER_PREFIX_LEN.checked_add(length).filter(|end|*end<=bytes.len()).ok_or_else(||SemioError::InvalidBinaryHeader("truncated envelope token".into()))?;
    if !matches_declared_token(&bytes[BINARY_HEADER_PREFIX_LEN..end],envelope_id,component,version,control)?{return Err(SemioError::InvalidBinaryHeader("declared envelope identity mismatch".into()));}
    Ok(&bytes[end..])
}

fn matches_declared_token(token:&[u8],envelope_id:&str,component:Component,version:u16,control:&mut crate::os_dsl::NativeDecodeControl<'_>)->SemioResult<bool>{
    let mut digits=[0u8;5];let mut start=digits.len();let mut remaining=version;
    loop{start-=1;digits[start]=b'0'+(remaining%10) as u8;remaining/=10;if remaining==0{break;}}
    let pieces=[envelope_id.as_bytes(),b".".as_slice(),component.as_str().as_bytes(),b" v".as_slice(),&digits[start..]];
    let expected=pieces.iter().try_fold(0usize,|length,piece|length.checked_add(piece.len())).ok_or_else(||SemioError::InvalidBinaryHeader("declared envelope size overflow".into()))?;
    if token.len()!=expected{return Ok(false);}
    control.scoped_stage(|control|{control.begin_stage(expected).map_err(SemioError::DecodingControl)?;let mut position=0;for piece in pieces{for chunk in piece.chunks(256){if token[position..position+chunk.len()]!=*chunk{return Ok(false);}position+=chunk.len();control.advance(chunk.len()).map_err(SemioError::DecodingControl)?;}}Ok(true)})
}

fn parse_binary_token(token: &str) -> SemioResult<SemioEnvelope> {
    let (body, version_str) = token.rsplit_once(" v").ok_or_else(|| SemioError::InvalidBinaryHeader(format!("missing version in {token}")))?;
    let version: u16 = version_str.parse().map_err(|_| SemioError::InvalidBinaryHeader(format!("bad version in {token}")))?;
    let parts: Vec<&str> = body.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidBinaryHeader(format!("expected plugin.artifact.component, got {body}")));
    }
    let component = Component::parse(parts[parts.len() - 1]).ok_or_else(|| SemioError::InvalidBinaryHeader(format!("unknown component in {body}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope { plugin, artifact, component, version })
}
//#endregion 🔖️Binary

//#region 🔖️Text
/// 📜️ Prepends the mandatory preamble to DSL/op/cmd body text.
pub fn wrap_text(envelope: &SemioEnvelope, body: &str) -> String {
    let mut body_trimmed = body.trim_start_matches('\u{feff}');
    if body_trimmed.starts_with("semio ") {
        if let Ok((_, rest)) = split_text_preamble(body_trimmed) {
            body_trimmed = rest;
        }
    }
    format!("{}\n{}", envelope.preamble_line(), body_trimmed.trim_start())
}

/// 🛫️ Emits canonical declared Text and its exact body under cumulative ownership admission.
pub fn wrap_text_controlled(envelope_id:&str,component:Component,version:u16,body:&str,control:&mut crate::os_dsl::NativeEncodeControl<'_>)->Result<String,String>{
    control.scoped_stage(|control|{control.checkpoint()?;if !envelope_id.contains('.')||!component.is_text(){return Err("invalid declared text envelope identity".into())}let mut digits=[0u8;5];let mut start=digits.len();let mut value=version;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}let digits=std::str::from_utf8(&digits[start..]).map_err(|_|"native envelope version is not ASCII")?;let total=6usize.checked_add(envelope_id.len()).and_then(|length|length.checked_add(component.as_str().len()+4+digits.len())).and_then(|length|length.checked_add(body.len())).ok_or("native text envelope length overflow")?;control.charge(total)?;let mut output=String::new();output.try_reserve_exact(total).map_err(|_|"native text envelope allocation failed")?;for text in ["semio ",envelope_id,".",component.as_str()," v",digits,"\n",body]{control.scoped_stage(|control|{control.begin_stage(text.len())?;let mut start=0;while start<text.len(){let mut end=(start+65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[start..end]);control.advance(end-start)?;start=end;}Ok::<_,String>(())})?;}Ok(output)})
}

/// 📖️ Parses a text `.semio` file into envelope and body (without preamble line).
pub fn split_text_preamble(text: &str) -> SemioResult<(SemioEnvelope, &str)> {
    let mut lines = text.lines();
    let first = lines.next().ok_or_else(|| SemioError::InvalidPreamble("empty file".into()))?.trim();
    let envelope = parse_preamble_line(first)?;
    let rest = text[first.len()..].trim_start_matches(['\r', '\n']);
    Ok((envelope, rest))
}

/// 🛂️ Borrows canonical text after exact owner, component and version admission.
pub fn split_text_preamble_controlled<'input>(text:&'input str,envelope_id:&str,component:Component,version:u16,control:&mut crate::os_dsl::NativeDecodeControl<'_>)->SemioResult<&'input str>{
    control.checkpoint().map_err(SemioError::DecodingControl)?;
    let bytes=text.as_bytes();let token_start=6usize;
    if !bytes.starts_with(b"semio "){return Err(SemioError::InvalidPreamble("missing canonical envelope prefix".into()));}
    let digits=if version>=10000{5}else if version>=1000{4}else if version>=100{3}else if version>=10{2}else{1};
    let end=token_start.checked_add(envelope_id.len()).and_then(|n|n.checked_add(component.as_str().len()+3+digits)).filter(|end|*end<=bytes.len()).ok_or_else(||SemioError::InvalidPreamble("truncated envelope token".into()))?;
    if !matches_declared_token(&bytes[token_start..end],envelope_id,component,version,control)?{return Err(SemioError::InvalidPreamble("declared envelope identity mismatch".into()));}
    let body=if end==bytes.len(){end}else if bytes[end]==b'\n'{end+1}else if bytes.get(end..end+2)==Some(b"\r\n"){end+2}else{return Err(SemioError::InvalidPreamble("canonical preamble requires a line boundary".into()));};
    Ok(&text[body..])
}

/// 🔍 Parses `semio plugin.artifact.component vN`.
pub fn parse_preamble_line(line: &str) -> SemioResult<SemioEnvelope> {
    let line = line.trim();
    let rest = line.strip_prefix("semio ").ok_or_else(|| SemioError::InvalidPreamble(format!("expected semio preamble, got {line}")))?;
    let (token, version_str) = rest.rsplit_once(" v").ok_or_else(|| SemioError::InvalidPreamble(format!("missing version in {line}")))?;
    let version: u16 = version_str.parse().map_err(|_| SemioError::InvalidPreamble(format!("bad version in {line}")))?;
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidPreamble(format!("expected plugin.artifact.component, got {token}")));
    }
    let component = Component::parse(parts[parts.len() - 1]).ok_or_else(|| SemioError::InvalidPreamble(format!("unknown component in {token}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope { plugin, artifact, component, version })
}
//#endregion 🔖️Text

//#region 🔖️Sniff
/// 👃 Derives format identity from raw bytes alone.
pub fn sniff(bytes: &[u8]) -> SemioResult<SemioEnvelope> {
    if bytes.starts_with(&BINARY_MAGIC) {
        let (envelope, _) = unwrap_binary(bytes)?;
        return Ok(envelope);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| SemioError::InvalidPreamble("binary file without semio magic".into()))?;
    let (envelope, _) = split_text_preamble(text)?;
    Ok(envelope)
}
//#endregion 🔖️Sniff

//#region 🔖️Paths
/// 📁 On-disk filename for a document facet: `<id>.<plugin>.<artifact>.<component>.semio`.
pub fn semio_filename(document_id: &str, envelope_id: &str, component: Component) -> String {
    format!("{document_id}.{envelope_id}.{}.semio", component.as_str())
}

/// 📖️ Infers envelope from a decorative filename (fallback only — content wins in `sniff`).
pub fn envelope_from_filename(name: &str) -> Option<SemioEnvelope> {
    let name = name.strip_suffix(".semio")?;
    let component = name.rsplit_once('.').and_then(|(_, c)| Component::parse(c))?;
    let rest = name.strip_suffix(&format!(".{}", component.as_str()))?;
    let (_doc, envelope_id) = rest.rsplit_once('.')?;
    let version = 1u16;
    SemioEnvelope::from_envelope_id(envelope_id, component, version).ok()
}
//#endregion 🔖️Paths

//#region 🔖️Registry
/// 🗂️ Handler keyed by full envelope identity.
pub type SemioHandler = fn(&[u8]) -> Result<(), String>;

struct RegistryState {
    by_key: HashMap<String, SemioHandler>,
}

fn registry_state() -> &'static Mutex<RegistryState> {
    static STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(RegistryState { by_key: HashMap::new() }))
}

fn registry_key(envelope: &SemioEnvelope) -> String {
    format!("{}.{}.{}", envelope.plugin, envelope.artifact, envelope.component.as_str())
}

/// 📝 Registers a verify/parse handler for one envelope.
pub fn register_format(envelope: SemioEnvelope, handler: SemioHandler) {
    let key = registry_key(&envelope);
    registry_state().lock().expect("semio registry").by_key.insert(key, handler);
}

/// 🔎 Resolves a handler from sniffed content.
pub fn resolve(bytes: &[u8]) -> SemioResult<SemioHandler> {
    let envelope = sniff(bytes)?;
    let key = registry_key(&envelope);
    let state = registry_state().lock().expect("semio registry");
    state.by_key.get(&key).copied().ok_or(SemioError::UnknownEnvelope(key))
}

/// ✅ Runs the registered handler for these bytes.
pub fn verify(bytes: &[u8]) -> SemioResult<()> {
    let handler = resolve(bytes)?;
    handler(bytes).map_err(SemioError::InvalidPreamble)
}
//#endregion 🔖️Registry

//#region 🔖️Cli
/// ⌨️ `semio` CLI entry (`inspect`, `open`, `convert`, `verify`).
pub mod cli {
    use super::*;

    /// 🏃 Dispatches argv; returns process exit code.
    pub fn main_impl(args: &[String]) -> i32 {
        if args.is_empty() || args[0] == "help" || args[0] == "--help" {
            eprintln!("usage: semio <inspect|verify|open|convert> <path> [...]");
            return 0;
        }
        let cmd = args[0].as_str();
        let path = match args.get(1) {
            Some(p) => p,
            None => {
                eprintln!("[semio] missing path argument");
                return 2;
            }
        };
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(err) => {
                eprintln!("[semio] read {path}: {err}");
                return 2;
            }
        };
        match cmd {
            "inspect" => match sniff(&bytes) {
                Ok(env) => {
                    println!("[TRACE] semio inspect {path}: {}", env.preamble_line());
                    println!("{}", env.preamble_line());
                    0
                }
                Err(err) => {
                    eprintln!("[semio] inspect failed: {err}");
                    1
                }
            },
            "verify" => match verify(&bytes) {
                Ok(()) => {
                    println!("[TRACE] semio verify {path}: ok");
                    0
                }
                Err(err) => {
                    eprintln!("[semio] verify failed: {err}");
                    1
                }
            },
            "open" | "convert" => {
                let env = match sniff(&bytes) {
                    Ok(e) => e,
                    Err(err) => {
                        eprintln!("[semio] {cmd} sniff failed: {err}");
                        return 1;
                    }
                };
                println!("[TRACE] semio {cmd} {path}: identity from content only -> {}", env.preamble_line());
                if let Ok(handler) = resolve(&bytes) {
                    if let Err(detail) = handler(&bytes) {
                        eprintln!("[semio] handler: {detail}");
                        return 1;
                    }
                }
                0
            }
            _ => {
                eprintln!("[semio] unknown command {cmd}");
                2
            }
        }
    }
}
//#endregion 🔖️Cli

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
