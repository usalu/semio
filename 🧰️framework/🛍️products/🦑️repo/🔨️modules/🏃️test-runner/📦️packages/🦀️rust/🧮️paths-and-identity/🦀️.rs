//! 🧮️ Paths and identity: the pending identity ports and the pure path arithmetic every other layer uses.
//!
//! Holds the `🪪️IdentityPending`, `🧮️Paths` regions of the crate root; the root re-exports its items.

//#region 🪪️IdentityPending
// 🚚️ These four helpers belong to `🪪️identity` (`semio-framework-repo-identity`) and to
// `🗣️languages`. Neither crate exports them yet, so faithful ports of the Go originals live here
// behind `EntityIdentity` / `LanguageTable`; when those crates land, this region collapses to a
// re-export and the default implementations are deleted. `id_to_uri` is the one place where this
// module is knowingly NOT yet behaviour-identical to Go: `DetectEntityKindFromId` needs the entity
// emoji table that `🪪️identity` owns, so the default port answers "" (unknown kind) for every raw
// id, exactly as Go does for an id whose kind it cannot detect. Callers that pass `repo://` URIs —
// the form the CLI and MCP both produce — are unaffected.

/// 🪪️ Identifier vocabulary the scope resolver needs from `🪪️identity`.
pub trait EntityIdentity {
    /// 🔤️ Alphanumerics and non-ASCII runes only, lowercased.
    fn flat(&self, text: &str) -> String;
    /// 🛤️ Reverses percent-encoding of spaces in a URI path segment list.
    fn path_from_uri_path(&self, uri_path: &str) -> String;
    /// 🔗️ `repo://<kind>/<id>` for an id whose entity kind is detectable, else the empty string.
    fn id_to_uri(&self, id: &str) -> String;
}

/// 🗣️ File-extension to language mapping the scope resolver needs from `🗣️languages`.
pub trait LanguageTable {
    /// 🏷️ The language name for a path, or `None` when no plugin claims the extension.
    fn language_of(&self, file_path: &str) -> Option<String>;
}

/// 🪪️ The ported-from-Go default for both ports.
#[derive(Clone, Copy, Debug, Default)]
pub struct PendingIdentity;

impl EntityIdentity for PendingIdentity {
    fn flat(&self, text: &str) -> String {
        flat(text)
    }
    fn path_from_uri_path(&self, uri_path: &str) -> String {
        path_from_uri_path(uri_path)
    }
    fn id_to_uri(&self, _id: &str) -> String {
        String::new()
    }
}

impl LanguageTable for PendingIdentity {
    fn language_of(&self, file_path: &str) -> Option<String> {
        language_of_extension(file_path).map(str::to_string)
    }
}

/// 🔤️ Keeps alphanumerics and every non-ASCII rune, then lowercases — Go `Flat`.
pub fn flat(text: &str) -> String {
    text.chars()
        .filter(|value| value.is_ascii_alphanumeric() || (*value as u32) > 0x7F)
        .collect::<String>()
        .to_lowercase()
}

/// 🛤️ Decodes `%20` per segment — Go `PathFromUriPath`.
pub fn path_from_uri_path(uri_path: &str) -> String {
    uri_path.split('/').map(|segment| segment.replace("%20", " ")).collect::<Vec<_>>().join("/")
}

/// 🗣️ The extension table the planner needs; a subset of `🗣️languages` limited to testable languages.
pub fn language_of_extension(file_path: &str) -> Option<&'static str> {
    let lower = file_path.to_lowercase();
    let extension = lower.rsplit_once('.').map(|(_, tail)| tail)?;
    match extension {
        "go" => Some("go"),
        "rs" => Some("rust"),
        "cs" => Some("csharp"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" | "mjs" | "cjs" => Some("javascript"),
        "py" => Some("python"),
        "rb" => Some("ruby"),
        _ => None,
    }
}
//#endregion 🪪️IdentityPending

//#region 🧮️Paths
/// 🧭️ Forward-slash normalisation; every path this crate stores or compares is in this form.
pub fn normalize_separators(path: &str) -> String {
    path.replace('\\', "/")
}

/// 📏️ Accepts both POSIX roots and Windows drive roots so one fixture runs on every host.
pub fn is_absolute_path(path: &str) -> bool {
    let path = normalize_separators(path);
    if path.starts_with('/') {
        return true;
    }
    let bytes = path.as_bytes();
    bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/'
}

/// 🔗️ Joins and cleans, dropping `.` segments and resolving `..` — Go `filepath.Join`.
pub fn join_path(base: &str, relative: &str) -> String {
    let base = normalize_separators(base);
    let relative = normalize_separators(relative);
    if base.is_empty() {
        return clean_path(&relative);
    }
    if relative.is_empty() {
        return clean_path(&base);
    }
    clean_path(&format!("{}/{}", base.trim_end_matches('/'), relative))
}

/// 🧹️ Collapses separators and `.`/`..` segments while keeping any root prefix.
pub fn clean_path(path: &str) -> String {
    let path = normalize_separators(path);
    let (prefix, rest) = if let Some(tail) = path.strip_prefix('/') {
        ("/".to_string(), tail.to_string())
    } else if is_absolute_path(&path) {
        (path[..3].to_string(), path[3..].to_string())
    } else {
        (String::new(), path.clone())
    };
    let mut segments: Vec<&str> = Vec::new();
    for segment in rest.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.last().is_some_and(|last| *last != "..") {
                    segments.pop();
                } else if prefix.is_empty() {
                    segments.push("..");
                }
            }
            other => segments.push(other),
        }
    }
    let body = segments.join("/");
    if prefix.is_empty() {
        if body.is_empty() {
            ".".to_string()
        } else {
            body
        }
    } else {
        format!("{prefix}{body}")
    }
}

/// 🏷️ The final segment of a path — Go `filepath.Base`.
pub fn base_name(path: &str) -> String {
    let path = normalize_separators(path);
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return path;
    }
    trimmed.rsplit('/').next().unwrap_or(trimmed).to_string()
}

/// 📁️ Everything before the final segment — Go `filepath.Dir`.
pub fn dir_name(path: &str) -> String {
    let cleaned = clean_path(path);
    match cleaned.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => cleaned[..index].to_string(),
        None => ".".to_string(),
    }
}

/// ↔️ `target` expressed relative to `base`, or `target` unchanged when it is not below it.
pub fn relative_path(base: &str, target: &str) -> String {
    let base = clean_path(base);
    let target = clean_path(target);
    if base == target {
        return ".".to_string();
    }
    let prefix = format!("{}/", base.trim_end_matches('/'));
    match target.strip_prefix(&prefix) {
        Some(rest) => rest.to_string(),
        None => target,
    }
}
//#endregion 🧮️Paths
