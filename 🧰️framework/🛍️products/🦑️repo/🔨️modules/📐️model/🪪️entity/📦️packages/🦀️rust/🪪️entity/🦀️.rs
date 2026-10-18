//! 🪪️ Artifact identities, property projection and the entity templates the renderers print.
//! 🪨️ Artifact identities, property projection and the entity templates the renderers print.

//#region 🪪️Entity

use crate::ansi;
use semio_framework_repo_identity::{collection as collection_emoji, emoji_text, entity as entity_emoji, flat, humanize_seconds};
use serde_json::{Map, Value};

/// 🔣️ The presentation-normalised emoji of one entity kind.
fn e(name: &str) -> String {
    emoji_text(entity_emoji(name))
}

/// 🗃️ The presentation-normalised emoji of one collection kind.
fn c(name: &str) -> String {
    emoji_text(collection_emoji(name))
}

/// 📝️ The string member `name`, empty when absent or not a string.
fn s(data: &Map<String, Value>, name: &str) -> String {
    data.get(name).and_then(Value::as_str).unwrap_or_default().to_string()
}

/// 🔢️ The integer member `name`, zero when absent or not a number.
fn n(data: &Map<String, Value>, name: &str) -> i64 {
    data.get(name).and_then(Value::as_f64).unwrap_or(0.0) as i64
}

/// 📁️ The last `/`-separated segment of a path.
fn base(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

/// 🏷️ The trailing extension of a file name, including the dot, empty when there is none.
fn extension(name: &str) -> String {
    match name.rfind('.') {
        Some(index) if index > 0 => name[index..].to_string(),
        _ => String::new(),
    }
}

/// 🛤️ Normalises separators to `/`, matching the workspace path convention.
pub fn normalize_path(value: &str) -> String {
    value.replace('\\', "/")
}

/// 📜️ The emoji a technology artifact id opens with.
fn technology_kind_emoji(data: &Map<String, Value>) -> String {
    let emoji = s(data, "emoji");
    if !emoji.is_empty() {
        return emoji_text(&emoji);
    }
    match s(data, "kind").as_str() {
        "infrastructure" => return e("technology-infrastructure"),
        "research" => return e("technology-research"),
        "mono" => return e("technology-mono"),
        "user" => return e("technology-user"),
        _ => {}
    }
    let name = s(data, "name");
    if name.contains("repo") {
        return e("technology-infrastructure");
    }
    if name.starts_with("coda") {
        return e("technology-research");
    }
    e("technology-user")
}

/// 🔵️ The emoji a bundle artifact id carries after its technology segment.
fn bundle_kind_emoji(data: &Map<String, Value>) -> String {
    let emoji = s(data, "emoji");
    if !emoji.is_empty() {
        return emoji_text(&emoji);
    }
    match s(data, "kind").as_str() {
        "schema" => e("bundle-schema"),
        "binary" => e("bundle-binary"),
        "ui" => e("bundle-ui"),
        "site" => e("bundle-site"),
        "assets" => e("bundle-assets"),
        "example" => e("bundle-example"),
        _ => e("bundle-library"),
    }
}

/// 📄️ The emoji a file artifact id carries, empty for an unknown kind.
fn file_kind_emoji(data: &Map<String, Value>) -> String {
    match s(data, "kind").as_str() {
        "code" => e("file-code"),
        "lab" => e("file-lab"),
        "script" => e("file-script"),
        "docs" => e("file-docs"),
        "config" => e("file-config"),
        "resource" => e("file-resource"),
        "template" => e("file-template"),
        "license" => e("file-license"),
        _ => String::new(),
    }
}

/// 📁️ The emoji a folder artifact id carries.
fn folder_kind_emoji(data: &Map<String, Value>) -> String {
    match s(data, "kind").as_str() {
        "organization" => e("folder-organization"),
        "root" => e("folder-root"),
        _ => e("folder-required"),
    }
}

/// 💕️ The emoji a definition artifact id carries.
fn definition_kind_emoji(data: &Map<String, Value>) -> String {
    match s(data, "kind").to_ascii_lowercase().as_str() {
        "interface" => e("definition-interface"),
        "constant" => e("definition-constant"),
        "test" => e("definition-test"),
        _ => e("definition-impl"),
    }
}

/// 💬️ The emoji an interaction artifact id ends with.
fn interaction_kind_emoji(data: &Map<String, Value>) -> String {
    match s(data, "kind").as_str() {
        "edited" => e("interaction-edited"),
        "finished" => e("interaction-finished"),
        "restarted" => e("interaction-restarted"),
        "deleted" => e("interaction-deleted"),
        _ => e("interaction-started"),
    }
}

/// 🎯️ The emoji artifact id of a `/`-separated goal path.
pub fn goal_artifact_id(raw: &str) -> String {
    raw.split('/').map(|segment| format!("{}{}", e("goal"), flat(segment))).collect()
}

/// 📄️ The emoji artifact id of a file path, resolved without a bundle registry.
///
/// The reference implementation consults the process-global bundle table here; this crate
/// holds no such global, so the id is built from the path alone. Callers that own a
/// `🗂️codebase` handle use its own `build_file_id`, which resolves the owning bundle first.
pub fn build_file_id(path: &str) -> String {
    let normalized = normalize_path(path);
    let name = base(&normalized);
    let kind = semio_framework_repo_model::derive_file_kind(&name);
    let ext = extension(&name);
    let stem = name.strip_suffix(&ext).unwrap_or(&name).to_string();
    let mut data = Map::new();
    data.insert("kind".to_string(), Value::String(kind.as_str().to_string()));
    format!("{}{}", file_kind_emoji(&data), flat(&stem))
}

/// 📑️ The emoji artifact id of a section path below a file id.
pub fn build_section_id(file_id: &str, section_path: &[String]) -> String {
    semio_framework_repo_model::build_section_id(file_id, section_path)
}

/// 🪪️ The emoji artifact id of one entity, mirroring the reference `GetArtifactID`.
pub fn artifact_id(kind: &str, data: &Map<String, Value>) -> String {
    let parent = s(data, "parentId");
    match kind {
        "root" | "statutes" => String::new(),
        "years" => parent + &c("years"),
        "year" => parent + &e("year") + &s(data, "yy"),
        "months" => parent + &c("months"),
        "month" => parent + &e("month") + &s(data, "mm"),
        "days" => parent + &c("days"),
        "day" => parent + &e("day") + &s(data, "dd"),
        "hours" => parent + &c("hours"),
        "hour" => parent + &e("hour") + &s(data, "hh"),
        "minutes" => parent + &c("minutes"),
        "minute" => parent + &e("minute") + &s(data, "mm"),
        "seconds" => parent + &c("seconds"),
        "second" => parent + &e("second") + &s(data, "ss"),
        "codebase" => parent + &c("codebase"),
        "technologies" => parent + &c("technologies"),
        "technology" => technology_kind_emoji(data) + &flat(&s(data, "name")),
        "bundles" => parent + &c("bundles"),
        "bundle" => {
            let name = s(data, "name");
            let (technology_code, bundle_code) = match name.split_once('/') {
                Some((left, right)) => (left.to_string(), right.to_string()),
                None => (name.clone(), name.clone()),
            };
            let mut technology_data = Map::new();
            technology_data.insert("name".to_string(), Value::String(technology_code.clone()));
            technology_kind_emoji(&technology_data) + &flat(&technology_code) + &bundle_kind_emoji(data) + &flat(&bundle_code)
        }
        "folders" => parent + &c("folders"),
        "folder" => parent + &folder_kind_emoji(data) + &flat(&base(&s(data, "path"))),
        "files" => parent + &c("files"),
        "file" => {
            let mut path = s(data, "path");
            if path.is_empty() {
                path = s(data, "id");
            }
            let name = base(&path);
            let ext = extension(&name);
            let stem = name.strip_suffix(&ext).unwrap_or(&name);
            parent + &file_kind_emoji(data) + &flat(stem)
        }
        "sections" => parent + &c("sections"),
        "section" => section_artifact_id(data, &parent),
        "definitions" => parent + &c("definitions"),
        "definition" => definition_artifact_id(data, &parent),
        "tickets" => parent + &c("tickets"),
        "ticket" => {
            let mut parent = parent;
            if parent.is_empty() {
                let goal = s(data, "goalId");
                if !goal.is_empty() {
                    parent = goal_artifact_id(&goal);
                }
            }
            let mut slug = s(data, "slug");
            if slug.is_empty() {
                slug = s(data, "title");
            }
            if slug.is_empty() {
                let id = s(data, "id");
                if !id.is_empty() {
                    return id;
                }
            }
            parent + &e("ticket") + &flat(&slug)
        }
        "goals" => parent + &c("goals"),
        "goal" => {
            let id = s(data, "id");
            let name = id.rsplit('/').next().unwrap_or(&id).to_string();
            parent + &e("goal") + &flat(&name)
        }
        "drafts" => parent + &c("drafts"),
        "draft" => {
            let mut slug = s(data, "slug");
            if slug.is_empty() {
                slug = s(data, "id");
            }
            parent + &e("draft") + &flat(&slug)
        }
        "todos" => parent + &c("todos"),
        "todo" => parent + &e("todo") + &flat(&s(data, "id")),
        "policies" => parent + &c("policies"),
        "policy" => parent + &e("policy") + &flat(s(data, "id").trim_start_matches('/')),
        "statute" => flat(&s(data, "id")),
        "contributors" => parent + &c("contributors"),
        "contributor" => {
            let mut alias = s(data, "alias");
            if alias.is_empty() {
                alias = s(data, "github");
            }
            e("contributor") + &flat(&alias)
        }
        "checkpoints" => parent + &c("checkpoints"),
        "checkpoint" => {
            let mut contributor = s(data, "contributorId");
            if contributor.is_empty() {
                let author = s(data, "authorId");
                if !author.is_empty() {
                    contributor = e("contributor") + &flat(&author);
                }
            }
            contributor + &e("checkpoint") + &s(data, "sha")
        }
        "line" => format!("{parent}{}{}", e("line"), n(data, "line")),
        "range" => format!("{parent}{}{}{}{}", e("line"), n(data, "startLine"), e("line"), n(data, "endLine")),
        "breach" => parent + &e("breach") + &s(data, "affected") + &e("breach-scope") + &s(data, "lineId") + &s(data, "secondId"),
        "breaches" => parent + &c("breaches"),
        "interactions" => parent + &c("interactions"),
        "interaction" => s(data, "secondId") + &s(data, "contributorId") + &s(data, "entityId") + &interaction_kind_emoji(data),
        "sessions" => parent + &c("sessions"),
        "session" => {
            let mut uuid = s(data, "uuid");
            if uuid.is_empty() {
                uuid = s(data, "id");
            }
            parent + &e("session") + &flat(&uuid)
        }
        _ => String::new(),
    }
}

/// 📑️ The section branch of [`artifact_id`], kept apart because it is the deepest one.
fn section_artifact_id(data: &Map<String, Value>, parent: &str) -> String {
    let mut path = s(data, "path");
    if path.is_empty() {
        let file_path = s(data, "filePath");
        let name = s(data, "name");
        if !file_path.is_empty() && !name.is_empty() {
            path = format!("{file_path}#{name}");
        } else if !name.is_empty() {
            path = name;
        }
    }
    if !parent.is_empty() {
        let section_name = match path.rfind('#') {
            Some(index) => path[index + 1..].to_string(),
            None => path.clone(),
        };
        return format!("{parent}{}{}", e("section"), flat(&section_name));
    }
    if let Some(index) = path.find('#') {
        let file_path = normalize_path(&path[..index]);
        let segments: Vec<String> = path[index + 1..].split('#').map(str::to_string).collect();
        return build_section_id(&build_file_id(&file_path), &segments);
    }
    let name = s(data, "name");
    if !name.is_empty() {
        return e("section") + &flat(&name);
    }
    e("section") + &flat(&path)
}

/// 💕️ The definition branch of [`artifact_id`].
fn definition_artifact_id(data: &Map<String, Value>, parent: &str) -> String {
    let marker = definition_kind_emoji(data);
    let mut name = s(data, "name");
    if name.is_empty() {
        let id = s(data, "id");
        if !id.is_empty() {
            name = match id.rfind('§') {
                Some(index) => id[index + '§'.len_utf8()..].to_string(),
                None => match id.rfind('#') {
                    Some(index) => id[index + 1..].to_string(),
                    None => id.clone(),
                },
            };
        }
    }
    if !parent.is_empty() {
        return format!("{parent}{marker}{}", flat(&name));
    }
    let mut section_path = s(data, "sectionPath");
    let mut file_path = s(data, "filePath");
    if file_path.is_empty() {
        let id = s(data, "id");
        if !id.is_empty() {
            if let Some(paragraph) = id.rfind('§') {
                name = id[paragraph + '§'.len_utf8()..].to_string();
                match id[..paragraph].find('#') {
                    Some(hash) => {
                        file_path = id[..hash].to_string();
                        section_path = id[hash + 1..paragraph].replace('#', "/");
                    }
                    None => file_path = id[..paragraph].to_string(),
                }
            } else if let Some(hash) = id.rfind('#') {
                name = id[hash + 1..].to_string();
                file_path = id[..hash].to_string();
            }
        }
    }
    if !file_path.is_empty() {
        let file_id = build_file_id(&file_path);
        let section_id = if section_path.is_empty() {
            file_id
        } else {
            let segments: Vec<String> = section_path.replace('#', "/").split('/').map(str::to_string).collect();
            build_section_id(&file_id, &segments)
        };
        return format!("{section_id}{marker}{}", flat(&name));
    }
    format!("{marker}{}", flat(&name))
}

/// 🔗️ The `repo://` uri of one entity.
pub fn artifact_uri(kind: &str, data: &Map<String, Value>) -> String {
    let id = artifact_id(kind, data);
    if id.is_empty() {
        return format!("repo://{kind}");
    }
    format!("repo://{kind}/{id}")
}

/// 🧼️ Collapses newlines and runs of spaces and swaps backticks for apostrophes.
pub fn sanitize_prop(value: &str) -> String {
    let mut value = value.replace("\r\n", " ").replace(['\n', '\r'], " ").replace('`', "'");
    while value.contains("  ") {
        value = value.replace("  ", " ");
    }
    value.trim().to_string()
}

/// 🧽️ Collapses every newline form to a space, leaving the rest untouched.
pub fn sanitize_single_line(value: &str) -> String {
    value.replace("\r\n", " ").replace(['\n', '\r'], " ")
}

/// ⏳️ A relative timestamp for one of the flexible time layouts, or the raw text.
fn relative_time(raw: &str) -> String {
    match parse_flexible_epoch(raw) {
        Some(epoch) => humanize_seconds(now_epoch() - epoch),
        None => raw.to_string(),
    }
}

/// 🕰️ The host wall clock in whole seconds since the Unix epoch.
fn now_epoch() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|value| value.as_secs() as i64).unwrap_or_default()
}

/// ⏰️ Decodes RFC 3339, `YYYY-MM-DD` and `YYYY-MM-DD HH:MM:SS` to a Unix timestamp.
pub fn parse_flexible_epoch(raw: &str) -> Option<i64> {
    let bytes = raw.as_bytes();
    if bytes.len() < 10 {
        return None;
    }
    let number = |from: usize, to: usize| raw.get(from..to).and_then(|slice| slice.parse::<i64>().ok());
    let year = number(0, 4)?;
    let month = number(5, 7)?;
    let day = number(8, 10)?;
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let (hour, minute, second) = if bytes.len() >= 19 && (bytes[10] == b'T' || bytes[10] == b' ') {
        (number(11, 13)?, number(14, 16)?, number(17, 19)?)
    } else if bytes.len() == 10 {
        (0, 0, 0)
    } else {
        return None;
    };
    Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
}

/// 📅️ Howard Hinnant's civil-date to days-since-epoch conversion.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// 🗓️ The creation timestamp of an entity, from either the flat or the nested member.
fn created_of(data: &Map<String, Value>) -> String {
    let direct = s(data, "created");
    if !direct.is_empty() {
        return direct;
    }
    for wrapper in ["dates", "date"] {
        if let Some(Value::Object(nested)) = data.get(wrapper) {
            let value = s(nested, "created");
            if !value.is_empty() {
                return value;
            }
        }
    }
    String::new()
}

/// 🏁️ The completion timestamp of an entity, from either the flat or the nested member.
fn finished_of(data: &Map<String, Value>) -> String {
    let direct = s(data, "finished");
    if !direct.is_empty() {
        return direct;
    }
    for wrapper in ["dates", "date"] {
        if let Some(Value::Object(nested)) = data.get(wrapper) {
            let value = s(nested, "finished");
            if !value.is_empty() {
                return value;
            }
        }
    }
    String::new()
}

/// ✂️ Truncates to 80 characters with an ellipsis when the caller asked for it.
fn clip(value: &str, truncate: bool) -> String {
    if truncate && value.len() > 80 {
        let mut end = 80;
        while end > 0 && !value.is_char_boundary(end) {
            end -= 1;
        }
        return format!("{}...", &value[..end]);
    }
    value.to_string()
}

/// 🧾️ The rendered property list of one entity kind.
pub fn collect_props(kind: &str, data: &Map<String, Value>, truncate: bool) -> Vec<String> {
    let mut props = Vec::new();
    let push = |value: &str, props: &mut Vec<String>| {
        if value.is_empty() {
            return;
        }
        let cleaned = sanitize_prop(value);
        if !cleaned.is_empty() {
            props.push(cleaned);
        }
    };
    match kind {
        "goal" => {
            let created = created_of(data);
            let created_display = if created.is_empty() { String::new() } else { format!("created {}", relative_time(&created)) };
            let mut due = s(data, "dueDate");
            if due.is_empty() {
                if let Some(Value::Object(dates)) = data.get("dates") {
                    due = s(dates, "due");
                }
            }
            let due_display = if due.is_empty() { String::new() } else { relative_time(&due) };
            push(&s(data, "title"), &mut props);
            push(&s(data, "status"), &mut props);
            push(&created_display, &mut props);
            push(&due_display, &mut props);
            push(&clip(&s(data, "description"), truncate), &mut props);
        }
        "ticket" => {
            let status = s(data, "status");
            let open = status == "open" || status == "OPEN";
            let created = created_of(data);
            let finished = finished_of(data);
            let date_display = if open {
                if created.is_empty() { String::new() } else { format!("opened {}", relative_time(&created)) }
            } else if !finished.is_empty() {
                format!("closed {}", relative_time(&finished))
            } else if !created.is_empty() {
                relative_time(&created)
            } else {
                String::new()
            };
            let content = if open { s(data, "prompt") } else { s(data, "summary") };
            push(&s(data, "title"), &mut props);
            push(&status, &mut props);
            push(&date_display, &mut props);
            push(&clip(&content, truncate), &mut props);
        }
        "bundle" => {
            let mut project_type = s(data, "projectType");
            if project_type.is_empty() {
                project_type = s(data, "type");
            }
            push(&s(data, "root"), &mut props);
            push(&project_type, &mut props);
        }
        "section" => {
            let start = n(data, "startLine");
            let end = n(data, "endLine");
            if start > 0 || end > 0 {
                push(&format!(":{start}-{end}"), &mut props);
            }
        }
        "definition" => {
            push(&s(data, "name"), &mut props);
            let start = n(data, "startLine");
            let end = n(data, "endLine");
            if start > 0 || end > 0 {
                push(&format!(":{start}-{end}"), &mut props);
            }
        }
        "contributor" | "todo" | "root" => push(&s(data, "name"), &mut props),
        "policy" | "statute" | "technology" => push(&s(data, "description"), &mut props),
        "checkpoint" => push(&s(data, "message"), &mut props),
        _ => {}
    }
    props
}

/// 🖥️ One entity as the human renderer prints it (`text/entity`).
pub fn render_human(kind: &str, data: &Map<String, Value>, is_tty: bool) -> String {
    let id = artifact_id(kind, data);
    let props = collect_props(kind, data, false);
    let mut out = String::new();
    if !id.is_empty() {
        out.push_str(&ansi::colorize(&sanitize_prop(&id), "bold", is_tty));
    }
    for (index, prop) in props.iter().enumerate() {
        if !id.is_empty() || index > 0 {
            out.push(' ');
        }
        out.push_str(&ansi::colorize(prop, ansi::prop_color(index), is_tty));
    }
    sanitize_single_line(&out)
}

/// 🔗️ One entity as a markdown link (`md/entity_link`).
pub fn render_markdown_link(kind: &str, data: &Map<String, Value>) -> String {
    let id = artifact_id(kind, data);
    let uri = artifact_uri(kind, data);
    let props = collect_props(kind, data, false);
    let mut out = format!("[{}]({uri})", sanitize_prop(&id));
    for prop in &props {
        out.push_str(&format!(" - `{prop}`"));
    }
    sanitize_single_line(&out)
}

/// 📋️ One entity as a markdown list item (`md/entity_item`).
pub fn render_markdown(kind: &str, data: &Map<String, Value>) -> String {
    format!("- {}", render_markdown_link(kind, data))
}

/// 🧭️ The entity kind a result member name projects to, empty when it names none.
pub fn infer_kind(key: &str) -> &'static str {
    let key = key.to_ascii_lowercase();
    const PREFIXES: &[(&str, &str)] = &[
        ("ticket", "ticket"),
        ("goal", "goal"),
        ("file", "file"),
        ("folder", "folder"),
        ("section", "section"),
        ("definition", "definition"),
        ("contributor", "contributor"),
        ("todo", "todo"),
        ("draft", "draft"),
        ("policy", "policy"),
        ("breachkind", "statute"),
        ("bundle", "bundle"),
        ("technology", "technology"),
        ("checkpoint", "checkpoint"),
        ("repo", "root"),
        ("syncgithub", "root"),
        ("syncmanagement", "root"),
        ("integrate", "file"),
        ("extract", "file"),
        ("fix", "root"),
    ];
    for (prefix, kind) in PREFIXES {
        if key.starts_with(prefix) {
            return kind;
        }
    }
    ""
}

/// 🌐️ Uri path form of a path or identifier: every segment carries its blanks percent-encoded,
/// so the link a markdown renderer emits is reversible by the matching decoder.
pub fn path_to_uri_path(value: &str) -> String {
    value.split('/').map(|segment| segment.replace(' ', "%20")).collect::<Vec<String>>().join("/")
}

/// 🌐️ The inverse of [`path_to_uri_path`].
pub fn path_from_uri_path(value: &str) -> String {
    value.split('/').map(|segment| segment.replace("%20", " ")).collect::<Vec<String>>().join("/")
}

//#endregion 🪪️Entity
