//! 🧩️ `assets` — extension assets, trusted plugin modules and the administrator page.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::StreamExt;
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::artifact_authority::trusted_catalog::plugin_module::plugin_module_content_type;
use serde::Serialize;
use crate::state::HubState;
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️Extensions
/// 🧩️ Hub mirror of dev `staticDirVitePlugin` `/🧩️extension-modules` — lists installed extension metadata.
#[derive(Serialize)]
pub(crate) struct ExtensionListResponse {
    pub(crate) extensions: Vec<serde_json::Value>,
}

fn extension_asset_content_type(path: &std::path::Path) -> &'static str {
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
fn extension_asset_path(root: &std::path::Path, extension_id: &str, rest: &str) -> Option<std::path::PathBuf> {
    let plain = |path: &std::path::Path| path.components().all(|component| matches!(component, std::path::Component::Normal(part) if part.to_str().is_some_and(|part| !part.chars().any(|character| character.is_control() || character == '\\'))));
    let id = std::path::Path::new(extension_id);
    let relative = std::path::Path::new(rest);
    if extension_id.is_empty() || rest.is_empty() || id.components().count() != 1 || !plain(id) || !plain(relative) {
        return None;
    }
    Some(root.join(id).join(relative))
}

pub(crate) async fn list_extensions(State(state): State<HubState>) -> Result<Json<ExtensionListResponse>, StatusCode> {
    let mut extensions = Vec::new();
    let mut entries = match tokio::fs::read_dir(&state.extensions_root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Json(ExtensionListResponse { extensions })),
        Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        if !entry.file_type().await.map(|kind| kind.is_dir()).unwrap_or(false) {
            continue;
        }
        let meta_path = entry.path().join("install.json");
        let bytes = match tokio::fs::read(&meta_path).await {
            Ok(value) => value,
            Err(_) => continue,
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else { continue };
        extensions.push(value);
    }
    extensions.sort_by(|left, right| left.get("extensionId").and_then(|value| value.as_str()).unwrap_or_default().cmp(right.get("extensionId").and_then(|value| value.as_str()).unwrap_or_default()));
    Ok(Json(ExtensionListResponse { extensions }))
}

pub(crate) async fn get_extension_asset(Path((extension_id, rest)): Path<(String, String)>, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
    let path = extension_asset_path(&state.extensions_root, &extension_id, &rest).ok_or(StatusCode::BAD_REQUEST)?;
    let bytes = tokio::fs::read(&path).await.map_err(|error| match error.kind() {
        std::io::ErrorKind::NotFound | std::io::ErrorKind::IsADirectory | std::io::ErrorKind::NotADirectory => StatusCode::NOT_FOUND,
        std::io::ErrorKind::InvalidInput | std::io::ErrorKind::InvalidFilename => StatusCode::BAD_REQUEST,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    })?;
    let content_type = extension_asset_content_type(&path);
    Ok(([(axum::http::header::CONTENT_TYPE, content_type)], bytes))
}
//#endregion 🔖️Extensions

//#region 🔖️PluginModules
/// 🧊️ A plugin module manifest and its files are content-addressed and immutable.
const TRUSTED_PLUGIN_MODULE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

/// 📇️ `GET /trusted-catalog/plugin-modules`: every plugin module of the current catalog generation. As
/// public as the catalog it indexes: a module is the trusted code a shell loads, never document data.
pub(crate) async fn get_trusted_plugin_module_index(State(state): State<HubState>) -> Response {
    let Some(catalog) = state.verified_catalog.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    match serde_json::to_vec(&catalog.plugin_module_index()) {
        Ok(bytes) => (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, "no-store")], bytes).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// 🧩️ `GET /trusted-catalog/plugin-modules/{bundleSha256}`: one plugin module manifest by its content address.
pub(crate) async fn get_trusted_plugin_module_manifest(Path(bundle_sha256): Path<String>, State(state): State<HubState>) -> Response {
    let Some(module) = state.verified_catalog.as_ref().and_then(|catalog| catalog.plugin_module(&bundle_sha256)) else { return StatusCode::NOT_FOUND.into_response() };
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL)], module.manifest_bytes().to_vec()).into_response()
}

/// 📖️ `GET /trusted-catalog/plugin-modules/{bundleSha256}/{*path}`: one file of that module, streamed from the
/// generation it was verified in and re-verified while it streams (`TrustedCatalogAssetStream`): no request deadline,
/// bytes flow from the first chunk, the declared `Content-Length` is completed only by bytes that verified, and a file
/// changed on disk after verification ends in a broken body, never in its last bytes. A client that goes away stops the read.
pub(crate) async fn get_trusted_plugin_module_file(Path((bundle_sha256, path)): Path<(String, String)>, State(state): State<HubState>) -> Response {
    let Some(asset) = state.verified_catalog.as_ref().and_then(|catalog| catalog.plugin_module(&bundle_sha256)).and_then(|module| module.file(&path)) else { return StatusCode::NOT_FOUND.into_response() };
    let Ok(stream) = asset.stream() else { return StatusCode::INTERNAL_SERVER_ERROR.into_response() };
    let length = stream.byte_length();
    let body = axum::body::Body::from_stream(futures::stream::unfold(stream, |mut stream| async move {
        stream.next_chunk().await.map(|chunk| (chunk.map_err(|error| std::io::Error::other(error.to_string())), stream))
    }));
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, plugin_module_content_type(&path).to_string()), (axum::http::header::CACHE_CONTROL, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL.to_string()), (axum::http::header::CONTENT_LENGTH, length.to_string())], body).into_response()
}
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
fn admin_asset_content_type(path: &std::path::Path) -> &'static str {
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
fn admin_asset_path(root: &std::path::Path, rest: &str) -> Option<std::path::PathBuf> {
    if rest.contains("..") || rest.contains('\\') {
        return None;
    }
    let path = root.join(rest.trim_start_matches('/'));
    if !path.starts_with(root) {
        return None;
    }
    Some(path)
}

async fn admin_page(state: &HubState, rest: &str) -> Response {
    let root = &state.admin_dir;
    if !root.is_dir() {
        return (StatusCode::SERVICE_UNAVAILABLE, "admin SPA not built — run: bun repo run os-hub-admin:build").into_response();
    }
    let Some(requested) = admin_asset_path(root, rest) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let path = if requested.is_file() { requested } else { root.join("🌐️.html") };
    match tokio::fs::read(&path).await {
        Ok(bytes) => ([(axum::http::header::CONTENT_TYPE, admin_asset_content_type(&path))], bytes).into_response(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub(crate) async fn get_admin_root(State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, "🌐️.html").await
}

pub(crate) async fn get_admin_asset(Path(rest): Path<String>, State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, &rest).await
}
//#endregion 🔖️AdminPage
