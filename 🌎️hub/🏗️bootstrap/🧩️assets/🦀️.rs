//! 🧩️ `assets` — extension assets, trusted plugin modules and the administrator page.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::StreamExt;
use semio_hub_artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub_trusted_catalog_contract::plugin_module::plugin_module_content_type;
use serde::Serialize;
use semio_hub_state::HubState;
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;
use semio_hub_route_models::assets::{ExtensionListResponse, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL, admin_asset_content_type, admin_asset_path, extension_asset_content_type, extension_asset_path};

//#region 🔖️Extensions

pub async fn list_extensions(State(state): State<HubState>) -> Result<Json<ExtensionListResponse>, StatusCode> {
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

pub async fn get_extension_asset(Path((extension_id, rest)): Path<(String, String)>, State(state): State<HubState>) -> Result<impl IntoResponse, StatusCode> {
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

/// 📇️ `GET /trusted-catalog/plugin-modules`: every plugin module of the current catalog generation. As
/// public as the catalog it indexes: a module is the trusted code a shell loads, never document data.
pub async fn get_trusted_plugin_module_index(State(state): State<HubState>) -> Response {
    let Some(catalog) = state.verified_catalog.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    match serde_json::to_vec(&catalog.plugin_module_index()) {
        Ok(bytes) => (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, "no-store")], bytes).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// 🧩️ `GET /trusted-catalog/plugin-modules/{bundleSha256}`: one plugin module manifest by its content address.
pub async fn get_trusted_plugin_module_manifest(Path(bundle_sha256): Path<String>, State(state): State<HubState>) -> Response {
    let Some(module) = state.verified_catalog.as_ref().and_then(|catalog| catalog.plugin_module(&bundle_sha256)) else { return StatusCode::NOT_FOUND.into_response() };
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL)], module.manifest_bytes().to_vec()).into_response()
}

/// 📖️ `GET /trusted-catalog/plugin-modules/{bundleSha256}/{*path}`: one file of that module, streamed from the
/// generation it was verified in and re-verified while it streams (`TrustedCatalogAssetStream`): no request deadline,
/// bytes flow from the first chunk, the declared `Content-Length` is completed only by bytes that verified, and a file
/// changed on disk after verification ends in a broken body, never in its last bytes. A client that goes away stops the read.
pub async fn get_trusted_plugin_module_file(Path((bundle_sha256, path)): Path<(String, String)>, State(state): State<HubState>) -> Response {
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

async fn admin_page(state: &HubState, rest: &str) -> Response {
    let root = &state.admin_dir;
    if !root.is_dir() {
        return (StatusCode::SERVICE_UNAVAILABLE, "admin SPA not built — run: repo run os-hub-admin:build").into_response();
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

pub async fn get_admin_root(State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, "🌐️.html").await
}

pub async fn get_admin_asset(Path(rest): Path<String>, State(state): State<HubState>) -> impl IntoResponse {
    admin_page(&state, &rest).await
}
//#endregion 🔖️AdminPage
