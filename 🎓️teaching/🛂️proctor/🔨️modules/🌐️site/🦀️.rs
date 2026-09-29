//! 🌐️ Static hosting of the built site (`PROCTOR_SITE`) for every GET the gateway does not claim.
//!
//! A request path is percent-decoded segment by segment and refused when any segment could leave
//! the root or address something that is not a published file (`..`, `.`, dotfiles, backslashes,
//! drive colons, control characters, Windows device names); a resolved file must still lie inside
//! the canonical root. A missing path without an extension is a client-side route and answers
//! `index.html` (single-page fallback); a missing path with an extension is a real 404, so a stale
//! asset URL never receives HTML. Everything Vite emitted into `assets/` carries a content hash in
//! its name and is cached immutably; documents (`*.html`) are always revalidated; any other file
//! (favicons, copied public files) is cached briefly. The site answers `GET` and `HEAD` only; any
//! other method is `405` with `Allow: GET, HEAD`.
//!
//! @see <https://vite.dev/guide/build> — hashed `assets/` outputs
//! @see <https://www.rfc-editor.org/rfc/rfc9111#section-5.2.2> — `Cache-Control` directives

use std::fmt;
use std::path::{Path, PathBuf};

use axum::body::Body;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use server::gateway::{ErrorBody, ServerError};

/// 📄️ The document every client-side route answers with.
pub const DOCUMENT: &str = "index.html";

/// 🧊️ `Cache-Control` of a content-hashed build output.
pub const IMMUTABLE: &str = "public, max-age=31536000, immutable";

/// 🔄️ `Cache-Control` of the document: always revalidated, so a deployment is picked up at once.
pub const REVALIDATE: &str = "no-cache";

/// ⏱️ `Cache-Control` of any other published file.
pub const SHORT: &str = "public, max-age=3600";

/// 🗂️ The directory Vite writes its content-hashed outputs to (`build.assetsDir`).
pub const HASHED_DIRECTORY: &str = "assets";

/// 🚦️ The methods the site answers, as the `Allow` header of a `405`.
pub const ALLOWED_METHODS: &str = "GET, HEAD";

const DEVICE_NAMES: [&str; 22] = ["con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9"];

/// 🗂️ A built site directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteHost {
    root: PathBuf,
}

/// 🧯️ Why a site directory cannot be served.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SiteError {
    Missing { root: PathBuf, detail: String },
    NoDocument { root: PathBuf },
}

impl fmt::Display for SiteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { root, detail } => write!(formatter, "site directory {} is unavailable: {detail}", root.display()),
            Self::NoDocument { root } => write!(formatter, "site directory {} holds no {DOCUMENT}; build the site first", root.display()),
        }
    }
}

impl std::error::Error for SiteError {}

/// 🧭️ What one request path resolves to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    File(PathBuf),
    Document,
    Missing,
    Refused,
}

impl SiteHost {
    /// 📂️ Serve `root`, which must be a directory holding `index.html`.
    pub fn open(root: &Path) -> Result<Self, SiteError> {
        let root = root.canonicalize().map_err(|error| SiteError::Missing { root: root.to_path_buf(), detail: error.to_string() })?;
        if !root.join(DOCUMENT).is_file() {
            return Err(SiteError::NoDocument { root });
        }
        Ok(Self { root })
    }

    /// 📍️ The canonical directory being served.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 🚧️ Resolve a raw (still percent-encoded) request path inside the root.
    pub fn resolve(&self, raw: &str) -> Resolution {
        let Some(segments) = decode_segments(raw) else { return Resolution::Refused };
        let Some(last) = segments.last() else { return Resolution::Document };
        let candidate = segments.iter().fold(self.root.clone(), |path, segment| path.join(segment));
        match candidate.metadata() {
            Ok(metadata) if metadata.is_file() => match candidate.canonicalize() {
                Ok(canonical) if canonical.starts_with(&self.root) => Resolution::File(canonical),
                _ => Resolution::Refused,
            },
            Ok(_) => Resolution::Document,
            Err(_) if last.contains('.') => Resolution::Missing,
            Err(_) => Resolution::Document,
        }
    }

    /// 📤️ Answer one request: the file, the document, a 404 or a refusal.
    pub(crate) async fn serve(&self, method: &Method, raw: &str) -> Response {
        if method != Method::GET && method != Method::HEAD {
            return method_not_allowed(method, raw);
        }
        let (path, cache) = match self.resolve(raw) {
            Resolution::File(path) => {
                let cache = cache_control(path.strip_prefix(&self.root).unwrap_or(&path));
                (path, cache)
            }
            Resolution::Document => (self.root.join(DOCUMENT), REVALIDATE),
            Resolution::Missing => return ServerError::NotFound(raw.to_string()).into_response(),
            Resolution::Refused => return ServerError::BadRequest(format!("refused path {raw:?}")).into_response(),
        };
        match tokio::fs::read(&path).await {
            Ok(bytes) => {
                let length = HeaderValue::from(bytes.len());
                let mut response = Response::new(if method == Method::HEAD { Body::empty() } else { Body::from(bytes) });
                let headers = response.headers_mut();
                headers.insert(header::CONTENT_LENGTH, length);
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type(&path)));
                headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
                headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
                response
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => ServerError::NotFound(raw.to_string()).into_response(),
            Err(error) => ServerError::Internal(error.to_string()).into_response(),
        }
    }
}

/// 🔓️ Percent-decode every segment of a raw path, dropping empty segments; `None` when any segment
/// is not a publishable file name.
pub fn decode_segments(raw: &str) -> Option<Vec<String>> {
    raw.split('/').filter(|segment| !segment.is_empty()).map(decode_segment).collect()
}

fn decode_segment(segment: &str) -> Option<String> {
    let bytes = segment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let pair = bytes.get(index + 1..index + 3)?;
            decoded.push(u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok()?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    let name = String::from_utf8(decoded).ok()?;
    let stem = name.split('.').next().unwrap_or_default().to_ascii_lowercase();
    let publishable = !name.starts_with('.') && !name.contains(['/', '\\', ':']) && !name.chars().any(char::is_control) && !DEVICE_NAMES.contains(&stem.as_str());
    publishable.then_some(name)
}

/// 🗃️ The `Cache-Control` of a published file by its path relative to the site root: a document
/// (`*.html`) is always revalidated, a file inside [`HASHED_DIRECTORY`] is immutable (Vite names it
/// by its content hash, whatever alphabet the hash uses), anything else is cached briefly.
pub fn cache_control(relative: &Path) -> &'static str {
    let document = relative.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("html"));
    let mut components = relative.components();
    let hashed = components.next().is_some_and(|first| first.as_os_str() == HASHED_DIRECTORY) && components.next().is_some();
    if document {
        REVALIDATE
    } else if hashed {
        IMMUTABLE
    } else {
        SHORT
    }
}

/// 🚫️ The answer to a method the site does not serve: `405` with `Allow: GET, HEAD`.
pub fn method_not_allowed(method: &Method, raw: &str) -> Response {
    let body = ErrorBody { kind: "methodNotAllowed".to_string(), message: format!("method not allowed: {method} {raw} (the site answers {ALLOWED_METHODS})") };
    let mut response = (StatusCode::METHOD_NOT_ALLOWED, Json(body)).into_response();
    response.headers_mut().insert(header::ALLOW, HeaderValue::from_static(ALLOWED_METHODS));
    response
}

/// 🏷️ The content type of a published file, by extension.
pub fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("html" | "htm") => "text/html; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json" | "map") => "application/json",
        Some("webmanifest") => "application/manifest+json",
        Some("txt") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("wasm") => "application/wasm",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
