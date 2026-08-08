//! 🧬️ Universal `.semio` container: content-derived envelope for every OS artifact encoding.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️Errors
/// @emoji ⚠️ Envelope parse or registry lookup failure.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SemioError {
    #[error("invalid semio preamble: {0}")]
    InvalidPreamble(String),
    #[error("invalid binary semio header: {0}")]
    InvalidBinaryHeader(String),
    #[error("unknown semio envelope: {0}")]
    UnknownEnvelope(String),
    #[error("ambiguous semio envelope match")]
    AmbiguousEnvelope,
}

pub type SemioResult<T> = Result<T, SemioError>;
//#endregion 🔖️Errors

//#region 🔖️Component
/// @emoji 🧩 Which constitutional encoding a `.semio` file carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Dsl,
    Pack,
    Op,
    Spr,
    Cmd,
}

impl Component {
    /// @emoji 🏷️ Wire token in the preamble and filename segment.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dsl => "dsl",
            Self::Pack => "pack",
            Self::Op => "op",
            Self::Spr => "spr",
            Self::Cmd => "cmd",
        }
    }

    /// @emoji 📖️ Parses a component token from preamble or filename.
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

    /// @emoji 📝 Whether this component uses a text preamble rather than a binary header.
    pub const fn is_text(self) -> bool {
        matches!(self, Self::Dsl | Self::Op | Self::Cmd)
    }
}
//#endregion 🔖️Component

//#region 🔖️Envelope
/// @emoji 📨 Identity of a `.semio` payload — derived from content, not from the filename.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SemioEnvelope {
    pub plugin: String,
    pub artifact: String,
    pub component: Component,
    pub version: u16,
}

impl SemioEnvelope {
    /// @emoji 🪪️ Dotted artifact id (`plugin.artifact`) used in `DocumentDsl::ENVELOPE_ID`.
    pub fn envelope_id(&self) -> String {
        format!("{}.{}", self.plugin, self.artifact)
    }

    /// @emoji 📜️ Full preamble line for text encodings, e.g. `semio gis.gismap.dsl v1`.
    pub fn preamble_line(&self) -> String {
        format!(
            "semio {}.{}.{} v{}",
            self.plugin,
            self.artifact,
            self.component.as_str(),
            self.version
        )
    }

    /// @emoji 🧬️ Binary envelope token without the `semio` keyword.
    pub fn binary_token(&self) -> String {
        format!(
            "{}.{}.{} v{}",
            self.plugin,
            self.artifact,
            self.component.as_str(),
            self.version
        )
    }

    /// @emoji 📖️ Parses `plugin.artifact` from a document type id.
    pub fn from_envelope_id(envelope_id: &str, component: Component, version: u16) -> SemioResult<Self> {
        let (plugin, artifact) = envelope_id
            .split_once('.')
            .ok_or_else(|| SemioError::InvalidPreamble(format!("envelope id must be plugin.artifact, got {envelope_id}")))?;
        Ok(Self {
            plugin: plugin.to_string(),
            artifact: artifact.to_string(),
            component,
            version,
        })
    }
}
//#endregion 🔖️Envelope

//#region 🔖️Binary
/// @emoji 🧲️ Magic prefix for binary `.semio` files (`0x89` keeps them non-UTF-8).
pub const BINARY_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

const BINARY_HEADER_PREFIX_LEN: usize = 8 + 4;

/// @emoji 📦️ Wraps a binary payload with the semio binary header.
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

/// @emoji 📖️ Strips the semio binary header and returns envelope + inner payload.
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
    let token = std::str::from_utf8(&bytes[BINARY_HEADER_PREFIX_LEN..token_end])
        .map_err(|_| SemioError::InvalidBinaryHeader("token not utf-8".into()))?;
    let envelope = parse_binary_token(token)?;
    let payload = bytes[token_end..].to_vec();
    Ok((envelope, payload))
}

fn parse_binary_token(token: &str) -> SemioResult<SemioEnvelope> {
    let (body, version_str) = token
        .rsplit_once(" v")
        .ok_or_else(|| SemioError::InvalidBinaryHeader(format!("missing version in {token}")))?;
    let version: u16 = version_str
        .parse()
        .map_err(|_| SemioError::InvalidBinaryHeader(format!("bad version in {token}")))?;
    let parts: Vec<&str> = body.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidBinaryHeader(format!("expected plugin.artifact.component, got {body}")));
    }
    let component = Component::parse(parts[parts.len() - 1])
        .ok_or_else(|| SemioError::InvalidBinaryHeader(format!("unknown component in {body}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope {
        plugin,
        artifact,
        component,
        version,
    })
}
//#endregion 🔖️Binary

//#region 🔖️Text
/// @emoji 📜️ Prepends the mandatory preamble to DSL/op/cmd body text.
pub fn wrap_text(envelope: &SemioEnvelope, body: &str) -> String {
    let mut body_trimmed = body.trim_start_matches('\u{feff}');
    if body_trimmed.starts_with("semio ") {
        if let Ok((_, rest)) = split_text_preamble(body_trimmed) {
            body_trimmed = rest;
        }
    }
    format!("{}\n{}", envelope.preamble_line(), body_trimmed.trim_start())
}

/// @emoji 📖️ Parses a text `.semio` file into envelope and body (without preamble line).
pub fn split_text_preamble(text: &str) -> SemioResult<(SemioEnvelope, &str)> {
    let mut lines = text.lines();
    let first = lines
        .next()
        .ok_or_else(|| SemioError::InvalidPreamble("empty file".into()))?
        .trim();
    let envelope = parse_preamble_line(first)?;
    let rest = text[first.len()..].trim_start_matches(['\r', '\n']);
    Ok((envelope, rest))
}

/// @emoji 🔍 Parses `semio plugin.artifact.component vN`.
pub fn parse_preamble_line(line: &str) -> SemioResult<SemioEnvelope> {
    let line = line.trim();
    let rest = line
        .strip_prefix("semio ")
        .ok_or_else(|| SemioError::InvalidPreamble(format!("expected semio preamble, got {line}")))?;
    let (token, version_str) = rest
        .rsplit_once(" v")
        .ok_or_else(|| SemioError::InvalidPreamble(format!("missing version in {line}")))?;
    let version: u16 = version_str
        .parse()
        .map_err(|_| SemioError::InvalidPreamble(format!("bad version in {line}")))?;
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidPreamble(format!("expected plugin.artifact.component, got {token}")));
    }
    let component = Component::parse(parts[parts.len() - 1])
        .ok_or_else(|| SemioError::InvalidPreamble(format!("unknown component in {token}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope {
        plugin,
        artifact,
        component,
        version,
    })
}
//#endregion 🔖️Text

//#region 🔖️Sniff
/// @emoji 👃 Derives format identity from raw bytes alone.
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
/// @emoji 📁 On-disk filename for a document facet: `<id>.<plugin>.<artifact>.<component>.semio`.
pub fn semio_filename(document_id: &str, envelope_id: &str, component: Component) -> String {
    format!("{document_id}.{envelope_id}.{}.semio", component.as_str())
}

/// @emoji 📖️ Infers envelope from a decorative filename (fallback only — content wins in `sniff`).
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
/// @emoji 🗂️ Handler keyed by full envelope identity.
pub type SemioHandler = fn(&[u8]) -> Result<(), String>;

struct RegistryState {
    by_key: HashMap<String, SemioHandler>,
}

fn registry_state() -> &'static Mutex<RegistryState> {
    static STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(RegistryState {
            by_key: HashMap::new(),
        })
    })
}

fn registry_key(envelope: &SemioEnvelope) -> String {
    format!(
        "{}.{}.{}",
        envelope.plugin,
        envelope.artifact,
        envelope.component.as_str()
    )
}

/// @emoji 📝 Registers a verify/parse handler for one envelope.
pub fn register_format(envelope: SemioEnvelope, handler: SemioHandler) {
    let key = registry_key(&envelope);
    registry_state()
        .lock()
        .expect("semio registry")
        .by_key
        .insert(key, handler);
}

/// @emoji 🔎 Resolves a handler from sniffed content.
pub fn resolve(bytes: &[u8]) -> SemioResult<SemioHandler> {
    let envelope = sniff(bytes)?;
    let key = registry_key(&envelope);
    let state = registry_state().lock().expect("semio registry");
    state
        .by_key
        .get(&key)
        .copied()
        .ok_or_else(|| SemioError::UnknownEnvelope(key))
}

/// @emoji ✅ Runs the registered handler for these bytes.
pub fn verify(bytes: &[u8]) -> SemioResult<()> {
    let handler = resolve(bytes)?;
    handler(bytes).map_err(|detail| SemioError::InvalidPreamble(detail))
}
//#endregion 🔖️Registry

//#region 🔖️Cli
/// @emoji ⌨️ `semio` CLI entry (`inspect`, `open`, `convert`, `verify`).
pub mod cli {
    use super::*;

  /// @emoji 🏃 Dispatches argv; returns process exit code.
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
                    println!("[DEBUG] semio inspect {path}: {}", env.preamble_line());
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
                    println!("[DEBUG] semio verify {path}: ok");
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
                println!("[DEBUG] semio {cmd} {path}: identity from content only -> {}", env.preamble_line());
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
mod tests {
    use super::*;

    #[test]
    fn text_preamble_round_trip() {
        let env = SemioEnvelope {
            plugin: "gis".into(),
            artifact: "gismap".into(),
            component: Component::Dsl,
            version: 1,
        };
        let wrapped = wrap_text(&env, "positions [id:TEXT] { }");
        let (parsed, body) = split_text_preamble(&wrapped).unwrap();
        assert_eq!(parsed, env);
        assert!(body.starts_with("positions"));
    }

    #[test]
    fn binary_header_round_trip() {
        let env = SemioEnvelope {
            plugin: "gis".into(),
            artifact: "gismap".into(),
            component: Component::Pack,
            version: 1,
        };
        let inner = b"payload-bytes";
        let wrapped = wrap_binary(&env, inner);
        let (parsed, payload) = unwrap_binary(&wrapped).unwrap();
        assert_eq!(parsed, env);
        assert_eq!(payload, inner);
    }

    #[test]
    fn sniff_text_and_binary() {
        let dsl_env = SemioEnvelope::from_envelope_id("gis.gismap", Component::Dsl, 1).unwrap();
        let text = wrap_text(&dsl_env, "schema=gis.map id=x");
        assert_eq!(sniff(text.as_bytes()).unwrap().component, Component::Dsl);
        let bin = wrap_binary(
            &SemioEnvelope::from_envelope_id("gis.gismap", Component::Pack, 1).unwrap(),
            b"x",
        );
        assert_eq!(sniff(&bin).unwrap().component, Component::Pack);
    }
}
