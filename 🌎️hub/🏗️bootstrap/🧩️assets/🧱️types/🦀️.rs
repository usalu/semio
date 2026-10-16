//! 🧱️ The `assets` vocabulary that carries no `HubState`: types, ledgers and helpers the state and the routes build on.

use serde::Serialize;

//#region 🔖️Extensions
/// 🧩️ Hub mirror of dev `staticDirVitePlugin` `/🧩️extension-modules` — lists installed extension metadata.
#[derive(Serialize)]
pub struct ExtensionListResponse {
    pub extensions: Vec<serde_json::Value>,
}

pub fn extension_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("js") | Some("mjs") => "text/javascript",
        Some("wasm") => "application/wasm",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

/// 🧩️ The file one extension asset request names, or `None` for a name that is not an extension id plus a
/// relative path of plain components: every component must be a normal, control-free name — no root, prefix,
/// `.` or `..`, and no NUL, which the file system refuses rather than resolves (a `%00` segment answered `500`).
pub fn extension_asset_path(root: &std::path::Path, extension_id: &str, rest: &str) -> Option<std::path::PathBuf> {
    let plain = |path: &std::path::Path| path.components().all(|component| matches!(component, std::path::Component::Normal(part) if part.to_str().is_some_and(|part| !part.chars().any(|character| character.is_control() || character == '\\'))));
    let id = std::path::Path::new(extension_id);
    let relative = std::path::Path::new(rest);
    if extension_id.is_empty() || rest.is_empty() || id.components().count() != 1 || !plain(id) || !plain(relative) {
        return None;
    }
    Some(root.join(id).join(relative))
}
//#endregion 🔖️Extensions

//#region 🔖️PluginModules
/// 🧊️ A plugin module manifest and its files are content-addressed and immutable.
pub const TRUSTED_PLUGIN_MODULE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";
//#endregion 🔖️PluginModules

//#region 🔖️AdminPage
/// 🛡️ Static SPA serving for `/admin` (contract §C2) — reads from `HubState.admin_dir`
/// (lane 1-B's field, `OS_HUB_ADMIN_DIR` else the compile-time default pointing at lane 2-E's own
/// `bun repo run os-hub-admin:build` output), mirroring `🔖️Extensions`'s `extension_asset_path`/
/// `get_extension_asset` pair exactly: plain `tokio::fs::read`, no `tower-http`, a traversal guard
/// before ever joining onto `root`, then a second `starts_with` check on the joined path as
/// defense-in-depth. SPA fallback: any requested path whose exact file is missing (a client-side
/// route, e.g. `/admin/spaces/sp-1`) falls back to `🌐️.html`; a genuinely missing admin build
/// (nobody ran the vite build yet) is a 503 with a build hint, never a confusing 404 loop.
pub fn admin_asset_content_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|value| value.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript",
        Some("css") => "text/css",
        Some("svg") => "image/svg+xml",
        Some("woff2") => "font/woff2",
        Some("json") => "application/json",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

/// 🚧️ Rejects `..`, a backslash (a Windows separator smuggled into a URL path segment), and
/// strips every leading `/` before joining onto `root` — `PathBuf::join` treats an absolute second
/// argument as a full replacement of the base, which would otherwise let `rest = "/etc/passwd"` read
/// clean outside `root` entirely (the one way this guard differs from `extension_asset_path`, which
/// never sees a leading-slash `rest` in the first place).
pub fn admin_asset_path(root: &std::path::Path, rest: &str) -> Option<std::path::PathBuf> {
    if rest.contains("..") || rest.contains('\\') {
        return None;
    }
    let path = root.join(rest.trim_start_matches('/'));
    if !path.starts_with(root) {
        return None;
    }
    Some(path)
}
//#endregion 🔖️AdminPage
