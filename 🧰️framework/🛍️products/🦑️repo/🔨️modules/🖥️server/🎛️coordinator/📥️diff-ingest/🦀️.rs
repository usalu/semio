//! 📥️ Diff ingest of the coordinator: the diff ingest route, the repository index, the unified diff parser and the repository file walk. See [`super`].

use super::*;

//#region 🌐️Http
impl Server {
    pub(super) fn diff_ingest(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let ticket_id = payload.get("ticket_id").and_then(Value::as_str).unwrap_or_default().to_string();
        let patch = payload.get("patch").and_then(Value::as_str).unwrap_or_default().to_string();
        if ticket_id.is_empty() || patch.is_empty() {
            return Response::error(400, "ticket_id and patch required");
        }
        let files = parse_unified_diff(&patch);
        let changed: Vec<String> = files.iter().map(|file| file.path.clone()).collect();
        let mut published = Map::new();
        published.insert("ticket_id".to_string(), Value::String(ticket_id));
        published.insert("files".to_string(), Value::Array(changed.iter().map(|file| Value::String(file.clone())).collect()));
        if let Err(failure) = self.publish("DiffIngested", "repo-cli", &Value::Object(published)) {
            return Response::error(500, &failure.to_string());
        }
        let conflicts = self.repository.project_conflicts();
        let warnings: Vec<Warning> = conflicts
            .iter()
            .map(|(scope, tickets)| Warning {
                id: new_id(),
                kind: "conflict".to_string(),
                severity: "error".to_string(),
                message: format!("conflict on {scope} across tickets {}", tickets.join(", ")),
                scope_id: scope.clone(),
                created_at: now_rfc3339(),
                ..Warning::default()
            })
            .collect();
        if let Err(failure) = self.repository.record_warnings(&warnings) {
            return Response::error(500, &failure.to_string());
        }
        let blockers: Vec<Value> = warnings.iter().filter(|warning| warning.severity == "error").map(|warning| Value::String(warning.message.clone())).collect();
        let mut response = Map::new();
        response.insert("changed_files".to_string(), string_list_or_null(&changed));
        response.insert("claimed_scopes".to_string(), Value::Null);
        response.insert("warnings".to_string(), if warnings.is_empty() { Value::Null } else { Value::Array(warnings.iter().map(Warning::to_value).collect()) });
        response.insert("breachs".to_string(), Value::Array(Vec::new()));
        response.insert("blockers".to_string(), Value::Array(blockers));
        Response::json(200, &Value::Object(response))
    }

    pub(super) fn reindex(&self, _request: &Request) -> Response {
        let files = walk_repo_files(Path::new(&self.config.repo_root));
        for file in &files {
            let path = Path::new(&self.config.repo_root).join(file);
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            if let Err(failure) = self.index(file, &content) {
                return Response::error(500, &failure.to_string());
            }
        }
        let mut object = Map::new();
        object.insert("files".to_string(), Value::from(files.len()));
        Response::json(200, &Value::Object(object))
    }

    pub(super) fn index_file(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let file_path = payload.get("file_path").and_then(Value::as_str).unwrap_or_default().to_string();
        if file_path.is_empty() {
            return Response::error(400, "file_path required");
        }
        let content = payload.get("content").and_then(Value::as_str).unwrap_or_default();
        match self.index(&file_path, content) {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => {
                let mut object = Map::new();
                object.insert("status".to_string(), Value::String("ok".to_string()));
                Response::json(200, &Value::Object(object))
            }
        }
    }

    pub(super) fn index(&self, file_path: &str, content: &str) -> Result<(), StoreError> {
        let scopes = build_scopes_for_file(file_path, content);
        self.repository.record_scopes(file_path, &scopes)?;
        let mut object = Map::new();
        object.insert("file".to_string(), Value::String(file_path.to_string()));
        self.publish("IndexUpdated", "server", &Value::Object(object))
    }
}

fn string_list_or_null(values: &[String]) -> Value {
    if values.is_empty() {
        Value::Null
    } else {
        Value::Array(values.iter().map(|value| Value::String(value.clone())).collect())
    }
}

/// 📍️ One file entry of a unified diff with its hunks.
#[derive(Debug, Clone, Default)]
pub struct DiffFile {
    pub path: String,
    pub hunks: Vec<(i64, i64)>,
    pub deleted: bool,
}

/// 🧲️ Extracts file paths and new-side hunk ranges from a unified diff patch.
pub fn parse_unified_diff(patch: &str) -> Vec<DiffFile> {
    let mut files: Vec<DiffFile> = Vec::new();
    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let parts: Vec<&str> = rest.split(' ').collect();
            if parts.len() >= 2 {
                files.push(DiffFile { path: parts[1].trim_start_matches("b/").to_string(), ..DiffFile::default() });
            }
            continue;
        }
        let Some(current) = files.last_mut() else { continue };
        if line.starts_with("+++ ") {
            if line.contains("/dev/null") {
                current.deleted = true;
            }
            continue;
        }
        if let Some(range) = line.strip_prefix("@@ ") {
            if let Some(new_side) = range.split('+').nth(1).and_then(|rest| rest.split(' ').next()) {
                let mut halves = new_side.split(',');
                let start: i64 = halves.next().unwrap_or("0").parse().unwrap_or(0);
                let count: i64 = halves.next().map_or(1, |value| value.parse().unwrap_or(1));
                current.hunks.push((start, start + count - 1));
            }
        }
    }
    let mut seen: Vec<String> = Vec::new();
    files.retain(|file| {
        if file.path.is_empty() || seen.contains(&file.path) {
            return false;
        }
        seen.push(file.path.clone());
        true
    });
    files
}

/// 📖️ The scopes of one file.
///
/// Only the file scope is derived here. Section and definition parsing lives in `🗣️languages`, whose
/// Rust crate does not yet expose it; the Go twin reaches it through `repo/events`. Until that
/// symbol lands, `/scopes` answers with the file scope alone and the cases avoid comparing bodies
/// that depend on the parser — recorded in `📓️opus-coordinator.md`.
pub fn build_scopes_for_file(path: &str, _content: &str) -> Vec<Scope> {
    vec![Scope { id: format!("file:{path}"), kind: "file".to_string(), file_path: path.to_string(), updated_at: now_rfc3339(), ..Scope::default() }]
}

fn walk_repo_files(root: &Path) -> Vec<String> {
    fn walk(root: &Path, current: &Path, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(current) else { return };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            if path.is_dir() {
                if !name.starts_with('.') {
                    walk(root, &path, out);
                }
                continue;
            }
            if let Ok(relative) = path.strip_prefix(root) {
                out.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}
//#endregion 🌐️Http
