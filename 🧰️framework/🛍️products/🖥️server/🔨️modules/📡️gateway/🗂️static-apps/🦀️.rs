//! 🖥️ The static app host, the install registry and the app routes.

use super::*;

//#region 🔖️StaticApp
/// 🗂️ One directory served as a single-page app: a traversal-guarded read, a content-type table and
/// an `index.html` fallback for client-side routes.
///
/// This is the only static server in the product. Every hosted surface — an admin console, an
/// extension bundle, a plugin's assets — is an instance of this type registered in the
/// [`AppRegistry`], because a second copy of a path-traversal guard is a second copy of a
/// vulnerability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticAppHost {
    pub(super) root: PathBuf,
}

impl StaticAppHost {
    /// 📂️ Serve this directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// 📍️ The directory being served.
    pub fn root(&self) -> &FsPath {
        &self.root
    }

    /// 🚧️ Resolve a request path inside the root, or refuse it.
    ///
    /// Refuses any `..` segment and any backslash (a Windows separator smuggled through a URL), and
    /// strips every leading `/` before joining — [`Path::join`](std::path::Path::join) treats an absolute argument as a
    /// full replacement of the base, so `/etc/passwd` would otherwise escape the root entirely. The
    /// joined path is checked against the root a second time as defence in depth.
    pub fn resolve(&self, rest: &str) -> Option<PathBuf> {
        if rest.contains("..") || rest.contains('\\') {
            return None;
        }
        let path = self.root.join(rest.trim_start_matches('/'));
        if !path.starts_with(&self.root) {
            return None;
        }
        Some(path)
    }

    /// 🏷️ The content type of an asset, by extension.
    pub fn content_type(path: &FsPath) -> &'static str {
        match path.extension().and_then(|value| value.to_str()) {
            Some("html") => "text/html; charset=utf-8",
            Some("js") | Some("mjs") => "text/javascript",
            Some("css") => "text/css",
            Some("json") => "application/json",
            Some("svg") => "image/svg+xml",
            Some("png") => "image/png",
            Some("woff2") => "font/woff2",
            Some("wasm") => "application/wasm",
            _ => "application/octet-stream",
        }
    }

    /// 📤️ Serve one path. A missing file that is not itself a build output falls back to
    /// `index.html` so a client-side route reloads correctly; a root that was never built at all is
    /// a 503 with a hint, never a confusing 404 loop.
    pub fn serve(&self, rest: &str) -> Response {
        if !self.root.is_dir() {
            return (StatusCode::SERVICE_UNAVAILABLE, format!("static app not built at {}", self.root.display())).into_response();
        }
        let Some(requested) = self.resolve(rest) else {
            return ServerError::BadRequest(format!("refused path '{rest}'")).into_response();
        };
        let path = if requested.is_file() { requested } else { self.root.join("index.html") };
        match std::fs::read(&path) {
            Ok(bytes) => ([(header::CONTENT_TYPE, Self::content_type(&path))], bytes).into_response(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => ServerError::NotFound(rest.to_string()).into_response(),
            Err(error) => ServerError::Internal(error.to_string()).into_response(),
        }
    }
}
//#endregion 🔖️StaticApp

//#region 🔖️Apps
/// 🧩️ One installed entry discovered under an app root: a directory holding an `install.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInstall {
    /// 🆔️ The directory name the entry was found under.
    pub id: String,
    /// 📇️ The verbatim `install.json` document; the gateway never interprets its shape.
    pub manifest: serde_json::Value,
}

/// 🔍️ Every `install.json` entry directly under `root`, ordered by id. An unreadable or malformed
/// entry is skipped rather than failing the whole scan.
pub fn scan_installs(root: &FsPath) -> Vec<AppInstall> {
    let Ok(entries) = std::fs::read_dir(root) else { return Vec::new() };
    let mut installs: Vec<AppInstall> = entries
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .filter_map(|entry| {
            let bytes = std::fs::read(entry.path().join("install.json")).ok()?;
            let manifest = serde_json::from_slice(&bytes).ok()?;
            Some(AppInstall { id: entry.file_name().to_string_lossy().into_owned(), manifest })
        })
        .collect();
    installs.sort_by(|left, right| left.id.cmp(&right.id));
    installs
}

/// 🗃️ The static surfaces this instance hosts, one [`StaticAppHost`] per registered name.
#[derive(Default)]
pub struct AppRegistry {
    pub(super) apps: ShardedMap<String, StaticAppHost>,
}

impl AppRegistry {
    /// 🌱️ A registry hosting nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ Host `dir` under `name`, replacing any app of that name.
    pub fn register(&self, name: &str, dir: impl Into<PathBuf>) {
        self.apps.insert(name.to_string(), StaticAppHost::new(dir));
    }

    /// 🔎️ The host registered under `name`.
    pub fn host(&self, name: &str) -> Option<StaticAppHost> {
        self.apps.get_cloned(name)
    }

    /// 📋️ Every registered app name, ordered.
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        self.apps.for_each(|name, _| names.push(name.clone()));
        names.sort();
        names
    }

    /// 🔣️ The `install.json` entries under one registered app's root.
    pub fn installs(&self, name: &str) -> Vec<AppInstall> {
        self.host(name).map(|host| scan_installs(host.root())).unwrap_or_default()
    }
}
//#endregion 🔖️Apps

//#region 🔖️AppRoutes
/// 📚️ The names of every app this instance hosts.
pub async fn get_apps<I: ServerInstance>(State(state): State<ServerState<I>>) -> Json<Vec<String>> {
    Json(state.apps.names())
}

/// 📜️ The `install.json` entries under one app's root.
pub async fn get_app_installs<I: ServerInstance>(Path(app): Path<String>, State(state): State<ServerState<I>>) -> Result<Json<Vec<AppInstall>>, ServerError> {
    let host = state.apps.host(&app).ok_or_else(|| ServerError::NotFound(format!("no app '{app}'")))?;
    Ok(Json(scan_installs(host.root())))
}

/// 🏠️ One app's entry document.
pub async fn get_app_root<I: ServerInstance>(Path(app): Path<String>, State(state): State<ServerState<I>>) -> Response {
    serve_app(&state, &app, "index.html")
}

/// 📎️ One asset inside an app.
pub async fn get_app_asset<I: ServerInstance>(Path((app, rest)): Path<(String, String)>, State(state): State<ServerState<I>>) -> Response {
    serve_app(&state, &app, &rest)
}

fn serve_app<I: ServerInstance>(state: &ServerState<I>, app: &str, rest: &str) -> Response {
    match state.apps.host(app) {
        Some(host) => host.serve(rest),
        None => ServerError::NotFound(format!("no app '{app}'")).into_response(),
    }
}
//#endregion 🔖️AppRoutes
