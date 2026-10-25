//! ⚡️ The deterministic executor: execution errors, query, entity and mutation resolvers and the `Executor` front.

use super::*;

//#region ⚡️Execution

/// ❌️ One execution failure, carrying the reference implementation's error text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionError {
    /// 📝️ The rendered failure.
    pub message: String,
}

impl ExecutionError {
    /// 🔨️ Builds an execution failure from any displayable value.
    pub fn new(message: impl Into<String>) -> ExecutionError {
        ExecutionError { message: message.into() }
    }

    /// 🔗️ Prefixes this failure with the selection it happened under, as `field: cause`.
    fn under(self, field: &str) -> ExecutionError {
        ExecutionError { message: format!("{field}: {}", self.message) }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ExecutionError {}

impl From<ContextError> for ExecutionError {
    fn from(error: ContextError) -> ExecutionError {
        ExecutionError { message: error.message }
    }
}

/// 🔤️ Case- and separator-insensitive field name the default resolver matches on.
fn canonical_name(value: &str) -> String {
    value.chars().filter(|character| *character != '_' && *character != '-').flat_map(char::to_lowercase).collect()
}

/// 📖️ Reads a field off a source object when no resolver was declared.
fn resolve_default(source: &Json, name: &str) -> Json {
    let Json::Object(fields) = source else { return Json::Null };
    if let Some(value) = fields.get(name) {
        return value.clone();
    }
    let canonical = canonical_name(name);
    fields.iter().find(|(key, _)| canonical_name(key) == canonical).map_or(Json::Null, |(_, value)| value.clone())
}

/// 📥️ Coerces one resolved argument value against its declared type.
///
/// The reference implementation hands the raw literal straight to the resolver, so a bare enum
/// member reaches it as its own spelling rather than as the value the enum declares. This
/// executor coerces instead — the enum member resolves to its wire value, an input object
/// resolves field by field — which is what a conforming executor does and what the resolvers
/// below are written against.
fn coerce_input(schema: &Schema, value: &Json, declared: &TypeRef) -> Json {
    match declared {
        TypeRef::NonNull(inner) => coerce_input(schema, value, inner),
        TypeRef::List(inner) => match value {
            Json::Array(items) => Json::Array(items.iter().map(|item| coerce_input(schema, item, inner)).collect()),
            Json::Null => Json::Null,
            other => Json::Array(vec![coerce_input(schema, other, inner)]),
        },
        TypeRef::Named(name) => match schema.types.get(name) {
            Some(NamedType::Enum(enumeration)) => match value.as_str() {
                Some(member) => enumeration.values.iter().find(|(declared_name, _)| declared_name == member).map_or_else(|| value.clone(), |(_, wire)| Json::String(wire.clone())),
                None => value.clone(),
            },
            Some(NamedType::InputObject(input)) => match value {
                Json::Object(fields) => {
                    let mut coerced = Map::new();
                    for (field, kind) in &input.fields {
                        if let Some(item) = fields.get(field) {
                            coerced.insert(field.clone(), coerce_input(schema, item, kind));
                        }
                    }
                    Json::Object(coerced)
                }
                other => other.clone(),
            },
            _ => value.clone(),
        },
    }
}

/// 🧹️ Whether a text survives one filter input.
pub(crate) fn matches_filter(text: &str, filter: Option<&FilterInput>) -> bool {
    let Some(filter) = filter else { return true };
    let Some(needle) = filter.filter.as_deref().filter(|value| !value.is_empty()) else { return true };
    let (haystack, needle) = if filter.match_case.unwrap_or(false) { (text.to_string(), needle.to_string()) } else { (text.to_lowercase(), needle.to_lowercase()) };
    if filter.match_whole_word.unwrap_or(false) {
        haystack.split(|character: char| !character.is_alphanumeric()).any(|word| word == needle)
    } else {
        haystack.contains(&needle)
    }
}

/// 🕳️ Decodes one coerced input object into a domain input type, filling every absent field
/// with the type's own default rather than failing on it.
fn decode_input<T>(value: &Json) -> Result<T, ExecutionError>
where
    T: Default + Serialize + serde::de::DeserializeOwned,
{
    let mut merged = body(&T::default());
    if let Json::Object(fields) = value {
        for (name, item) in fields {
            merged.insert(name.clone(), item.clone());
        }
    }
    serde_json::from_value(Json::Object(merged)).map_err(|error| ExecutionError::new(error.to_string()))
}

/// ⚙️ One execution of one document against one schema and one context.
struct Execution<'a> {
    pub(crate) schema: &'a Schema,
    pub(crate) context: &'a dyn RepoContext,
    pub(crate) variables: Map<String, Json>,
    pub(crate) root_dir: String,
    pub(crate) statutes: Vec<StatuteMeta>,
    pub(crate) technologies: Vec<Technology>,
}

impl Execution<'_> {
    /// ▶️ Resolves every selection of one object, honouring aliases, defaults and `__typename`.
    fn execute_selections(&self, source: &Json, object: &ObjectType, selections: &[Selection]) -> Result<Map<String, Json>, ExecutionError> {
        let mut result = Map::new();
        for selected in selections {
            if selected.name == "__typename" {
                result.insert(selected.key().to_string(), Json::String(object.name.clone()));
                continue;
            }
            let Some(field) = object.field(&selected.name) else {
                return Err(ExecutionError::new(format!("unknown field {} on {}", quote_string(&selected.name), object.name)));
            };
            let defaults: Map<String, Json> = field.args.iter().map(|argument| (argument.name.clone(), argument.default.clone())).collect();
            let raw = coerce_arguments(&selected.arguments, &defaults, &self.variables);
            let mut args = Map::new();
            for (name, value) in raw {
                match field.args.iter().find(|argument| argument.name == name) {
                    Some(argument) => args.insert(name, coerce_input(self.schema, &value, &argument.ty)),
                    None => args.insert(name, value),
                };
            }
            let value = match &field.resolver {
                Some(key) => self.dispatch(key, source, &args).map_err(|error| error.under(&selected.name))?,
                None => resolve_default(source, &selected.name),
            };
            let projected = self.project(value, &field.ty, &selected.fields).map_err(|error| error.under(&selected.name))?;
            result.insert(selected.key().to_string(), projected);
        }
        Ok(result)
    }

    /// ➡️ Shapes one resolved value to its declared type and sub-selection.
    fn project(&self, value: Json, declared: &TypeRef, selections: &[Selection]) -> Result<Json, ExecutionError> {
        if value.is_null() {
            return Ok(Json::Null);
        }
        match declared {
            TypeRef::NonNull(inner) => self.project(value, inner, selections),
            TypeRef::List(inner) => {
                let Json::Array(items) = value else {
                    return Err(ExecutionError::new(format!("expected list, got {value}")));
                };
                items.into_iter().map(|item| self.project(item, inner, selections)).collect::<Result<Vec<_>, _>>().map(Json::Array)
            }
            TypeRef::Named(name) => match self.schema.types.get(name) {
                Some(NamedType::Object(object)) => {
                    if selections.is_empty() {
                        return Ok(value);
                    }
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Interface(interface)) => {
                    let object = self.concrete(&value).ok_or_else(|| ExecutionError::new(format!("cannot resolve interface {}", interface.name)))?;
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Union(union)) => {
                    let object = self.concrete(&value).ok_or_else(|| ExecutionError::new(format!("cannot resolve union {}", union.name)))?;
                    self.execute_selections(&value, object, selections).map(Json::Object)
                }
                Some(NamedType::Enum(enumeration)) => Ok(match value.as_str().and_then(|wire| enumeration.values.iter().find(|(_, declared_wire)| declared_wire == wire)) {
                    Some((member, _)) => Json::String(member.clone()),
                    None => value,
                }),
                _ => Ok(value),
            },
        }
    }

    /// 🎭️ The concrete object a source discriminates itself as.
    fn concrete(&self, value: &Json) -> Option<&ObjectType> {
        value.get("__typename").and_then(Json::as_str).and_then(|name| self.schema.object(name))
    }
}

//#endregion ⚡️Execution

//#region 🗂️QueryResolvers

impl Execution<'_> {
    /// 🧽️ The filter argument of one field, decoded.
    fn filter_of(&self, args: &Map<String, Json>) -> Option<FilterInput> {
        args.get("filter").filter(|value| !value.is_null()).and_then(|value| decode_input::<FilterInput>(value).ok())
    }

    /// 📜️ Every technology, filtered and sorted by name.
    pub(crate) fn technologies(&self, args: &Map<String, Json>) -> Json {
        let filter = self.filter_of(args);
        let mut technologies = self.technologies.clone();
        technologies.sort_by(|left, right| left.name.cmp(&right.name));
        let rows = technologies
            .iter()
            .filter(|technology| matches_filter(&technology.name, filter.as_ref()) || matches_filter(&technology_id(technology), filter.as_ref()))
            .map(|technology| technology_source(technology, &self.technologies, &self.root_dir))
            .collect();
        Json::Array(rows)
    }

    /// 🔵️ Every bundle, filtered.
    pub(crate) fn bundles(&self, args: &Map<String, Json>) -> Json {
        let filter = self.filter_of(args);
        Json::Array(
            self.context
                .bundles()
                .iter()
                .filter(|bundle| matches_filter(&bundle.name, filter.as_ref()))
                .map(|bundle| bundle_source(bundle, &self.technologies, &self.root_dir))
                .collect(),
        )
    }

    /// 🎫️ Every ticket matching the date, status and text filters.
    pub(crate) fn tickets(&self, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let integer = |name: &str| args.get(name).and_then(Json::as_i64);
        let status = args.get("status").and_then(Json::as_str).and_then(|wire| match wire {
            "open" => Some(TicketStatus::Open),
            "closed" => Some(TicketStatus::Closed),
            _ => None,
        });
        let filter = self.filter_of(args);
        let tickets = self.context.tickets(integer("year"), integer("month"), integer("day"), status)?;
        Ok(Json::Array(tickets.iter().filter(|ticket| matches_filter(&ticket.slug, filter.as_ref())).map(|ticket| ticket_source(ticket, &self.root_dir)).collect()))
    }

    /// 🌿️ Routes one artifact identifier to the aggregate it names.
    fn node(&self, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let raw = args.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
        let clean = raw.replace(['\u{FE0E}', '\u{FE0F}'], "");
        if clean.is_empty() {
            return Ok(self.repo());
        }
        let strip = |value: &str, emoji: &str| -> Option<String> {
            let prefix = emoji.replace(['\u{FE0E}', '\u{FE0F}'], "");
            if prefix.is_empty() {
                return None;
            }
            value.strip_prefix(&prefix).map(ToString::to_string)
        };
        if let Some(bundle) = self.context.bundles().iter().find(|bundle| bundle_id(bundle, &self.technologies).replace(['\u{FE0E}', '\u{FE0F}'], "") == clean) {
            return Ok(bundle_source(bundle, &self.technologies, &self.root_dir));
        }
        for kind in ["technology-user", "technology-infrastructure", "technology-research", "technology-mono"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                let matched = self.technologies.iter().find(|technology| flat(&technology.name) == rest || technology.name == rest);
                return Ok(matched.map_or(Json::Null, |technology| technology_source(technology, &self.technologies, &self.root_dir)));
            }
        }
        for kind in ["folder-organization", "folder-required", "folder-root"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                return Ok(self.folder_by_path(&rest));
            }
        }
        for kind in ["file-code", "file-lab", "file-script", "file-docs", "file-config", "file-resource", "file-template", "file-license"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                return Ok(self.file_by_path(&rest));
            }
        }
        for kind in ["definition-impl", "definition-interface", "definition-constant", "definition-test"] {
            if let Some(rest) = strip(&clean, entity(kind)) {
                let matched = self.context.definitions().into_iter().find(|definition| flat(&definition.name) == rest || definition.name == rest);
                return Ok(matched.as_ref().map_or(Json::Null, definition_source));
            }
        }
        if let Some(rest) = strip(&clean, entity("ticket")) {
            let slug = rest.split('?').next().unwrap_or_default().to_string();
            let matched = self.context.tickets(None, None, None, None)?.into_iter().find(|ticket| flat(&ticket.slug) == slug || ticket.slug == slug);
            return Ok(matched.map_or(Json::Null, |ticket| ticket_source(&ticket, &self.root_dir)));
        }
        if let Some(rest) = strip(&clean, entity("contributor")) {
            let matched = self.context.contributors()?.into_iter().find(|contributor| flat(&contributor.alias) == rest || contributor.github == rest);
            return Ok(matched.as_ref().map_or(Json::Null, contributor_source));
        }
        if let Some(rest) = strip(&clean, entity("policy")) {
            return Ok(self.policy_by_id(&rest));
        }
        for (prefix, resolve) in [("repo:", 0), ("bundle:", 1), ("folder:", 2), ("file:", 3), ("contributor:", 4), ("policy:", 5)] {
            if let Some(rest) = raw.strip_prefix(prefix) {
                return Ok(match resolve {
                    0 => self.repo(),
                    1 => self.bundle_by_name(rest),
                    2 => self.folder_by_path(rest),
                    3 => self.file_by_path(rest),
                    4 => self.contributor_by_id(rest)?,
                    _ => self.policy_by_id(rest),
                });
            }
        }
        Err(ExecutionError::new(format!("invalid node id format: {raw}")))
    }

    /// 💠️ The repository root object.
    fn repo(&self) -> Json {
        let mut fields = Map::new();
        fields.insert("id".to_string(), Json::String("repo:compose".to_string()));
        fields.insert("name".to_string(), Json::String("compose".to_string()));
        fields.insert("path".to_string(), Json::String(self.root_dir.clone()));
        tagged(fields, "Repo")
    }

    /// 📦️ One bundle by name or identifier.
    fn bundle_by_name(&self, name: &str) -> Json {
        let bundles = self.context.bundles();
        match bundles.iter().find(|bundle| bundle.name == name || bundle_id(bundle, &self.technologies) == name) {
            Some(bundle) => bundle_source(bundle, &self.technologies, &self.root_dir),
            None => bundle_source(
                &Bundle {
                    name: name.to_string(),
                    root: String::new(),
                    source_root: String::new(),
                    technology_name: String::new(),
                    tags: Vec::new(),
                    kind: BundleKind::Library,
                    emoji: String::new(),
                    packages: Vec::new(),
                },
                &self.technologies,
                &self.root_dir,
            ),
        }
    }

    /// 🩷️ One folder by path.
    fn folder_by_path(&self, path: &str) -> Json {
        let normalized = path.replace('\\', "/");
        self.context.folders().iter().find(|folder| folder.path == normalized).map_or(Json::Null, folder_source)
    }

    /// 📄️ One file by path.
    fn file_by_path(&self, path: &str) -> Json {
        let normalized = path.replace('\\', "/");
        self.context.files().iter().find(|file| file.path == normalized).map_or(Json::Null, file_source)
    }

    /// 🧑️‍💻️ One contributor by github handle, falling back to a bare record.
    fn contributor_by_id(&self, id: &str) -> Result<Json, ExecutionError> {
        let contributors = self.context.contributors()?;
        Ok(match contributors.iter().find(|contributor| contributor.github == id) {
            Some(contributor) => contributor_source(contributor),
            None => contributor_source(&Contributor {
                alias: id.to_string(),
                aliases: Vec::new(),
                emoji: String::new(),
                github: id.to_string(),
                githubs: Vec::new(),
                name: String::new(),
                names: Vec::new(),
                email: String::new(),
                emails: Vec::new(),
                links: BTreeMap::new(),
                fingerprint: String::new(),
                fingerprints: Vec::new(),
                contributions: ContributorContributionsStorage::default(),
            }),
        })
    }

    /// 👮️ One policy by name or identifier, falling back to a bare record.
    fn policy_by_id(&self, id: &str) -> Json {
        let policies = self.context.policies();
        match policies.iter().find(|policy| policy.name == id || policy.id == id || policy.name.eq_ignore_ascii_case(id) || policy.id.eq_ignore_ascii_case(id)) {
            Some(policy) => policy_source(policy, &self.statutes),
            None => policy_source(
                &Policy { id: format!("repo/policy/{id}"), name: id.to_string(), description: None, scopes: Some(Vec::new()), groups: Some(Vec::new()), statutes: Some(Vec::new()) },
                &self.statutes,
            ),
        }
    }
}

//#endregion 🗂️QueryResolvers

//#region 🧪️EntityResolvers

/// 📅️ Tickets grouped by year, then month, then day — the shape a contribution tree reports.
type TicketCalendar = Vec<(i64, Vec<(i64, Vec<(i64, Vec<Json>)>)>)>;

impl Execution<'_> {
    /// 🌳️ The contribution tree of one contributor: their checkpoints and their tickets by date.
    fn contributions(&self, source: &Json) -> Result<Json, ExecutionError> {
        let github = source.get("github").and_then(Json::as_str).unwrap_or_default().to_string();
        let mut tickets: Vec<Ticket> = self.context.tickets(None, None, None, None)?.into_iter().filter(|ticket| ticket_author(ticket).eq_ignore_ascii_case(&github)).collect();
        tickets.sort_by_key(|ticket| std::cmp::Reverse(ticket_started(ticket)));
        let mut checkpoints: Vec<Json> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for ticket in &tickets {
            let sha = ticket_checkpoint(ticket);
            if sha.is_empty() || seen.contains(&sha) {
                continue;
            }
            seen.push(sha.clone());
            let mut fields = Map::new();
            fields.insert("id".to_string(), Json::String(format!("repo/checkpoint/{sha}")));
            fields.insert("sha".to_string(), Json::String(sha));
            fields.insert("title".to_string(), Json::String(ticket.title.clone()));
            fields.insert("date".to_string(), Json::String(ticket_started(ticket)));
            checkpoints.push(tagged(fields, "Checkpoint"));
        }
        let mut years: TicketCalendar = Vec::new();
        for ticket in &tickets {
            let year = years.iter_mut().find(|(value, _)| *value == ticket.year);
            let months = match year {
                Some((_, months)) => months,
                None => {
                    years.push((ticket.year, Vec::new()));
                    &mut years.last_mut().expect("year was just pushed").1
                }
            };
            let month = months.iter_mut().find(|(value, _)| *value == ticket.month);
            let days = match month {
                Some((_, days)) => days,
                None => {
                    months.push((ticket.month, Vec::new()));
                    &mut months.last_mut().expect("month was just pushed").1
                }
            };
            match days.iter_mut().find(|(value, _)| *value == ticket.day) {
                Some((_, rows)) => rows.push(ticket_source(ticket, &self.root_dir)),
                None => days.push((ticket.day, vec![ticket_source(ticket, &self.root_dir)])),
            }
        }
        let ticket_years = years
            .into_iter()
            .map(|(year, months)| {
                let month_rows = months
                    .into_iter()
                    .map(|(month, days)| {
                        let day_rows = days
                            .into_iter()
                            .map(|(day, rows)| {
                                let mut fields = Map::new();
                                fields.insert("day".to_string(), Json::from(day));
                                fields.insert("tickets".to_string(), Json::Array(rows));
                                tagged(fields, "TicketDay")
                            })
                            .collect();
                        let mut fields = Map::new();
                        fields.insert("month".to_string(), Json::from(month));
                        fields.insert("days".to_string(), Json::Array(day_rows));
                        tagged(fields, "TicketMonth")
                    })
                    .collect();
                let mut fields = Map::new();
                fields.insert("year".to_string(), Json::from(year));
                fields.insert("months".to_string(), Json::Array(month_rows));
                tagged(fields, "TicketYear")
            })
            .collect();
        let mut fields = Map::new();
        fields.insert("checkpoints".to_string(), Json::Array(checkpoints));
        fields.insert("tickets".to_string(), Json::Array(ticket_years));
        fields.insert("bundles".to_string(), Json::Array(Vec::new()));
        Ok(tagged(fields, "ContributorContributions"))
    }

    /// 🖊️ The contributor a ticket's last interaction names.
    fn ticket_author_of(&self, source: &Json) -> Result<Json, ExecutionError> {
        let Some(Json::Array(interactions)) = source.get("interactions") else { return Ok(Json::Null) };
        let Some(last) = interactions.last() else { return Ok(Json::Null) };
        let raw = last.get("author").and_then(Json::as_str).unwrap_or_default();
        let (name, email) = parse_git_author(raw);
        let author = if email.is_empty() { name } else { email };
        let contributors = self.context.contributors()?;
        let matched = contributors
            .iter()
            .find(|contributor| contributor.emails.iter().any(|candidate| candidate == &author || author.contains(candidate.as_str())) || contributor.name == author);
        Ok(match matched {
            Some(contributor) => contributor_source(contributor),
            None => contributor_source(&Contributor {
                alias: author.clone(),
                aliases: Vec::new(),
                emoji: String::new(),
                github: author.clone(),
                githubs: Vec::new(),
                name: author.clone(),
                names: Vec::new(),
                email: String::new(),
                emails: vec![author],
                links: BTreeMap::new(),
                fingerprint: String::new(),
                fingerprints: Vec::new(),
                contributions: ContributorContributionsStorage::default(),
            }),
        })
    }

    /// 💗️ The children of a section: its nested sections and its definitions, in source order.
    fn section_children(source: &Json) -> Json {
        let mut items: Vec<Json> = Vec::new();
        for key in ["children", "definitions"] {
            if let Some(Json::Array(rows)) = source.get(key) {
                items.extend(rows.iter().cloned());
            }
        }
        items.sort_by_key(|item| (item.get("startLine").and_then(Json::as_i64).unwrap_or_default(), item.get("startIndex").and_then(Json::as_i64).unwrap_or_default()));
        Json::Array(items)
    }
}

//#endregion 🧪️EntityResolvers

//#region 💻️MutationResolvers

impl Execution<'_> {
    /// 🏃️ Runs one named resolver.
    #[allow(clippy::too_many_lines)]
    fn dispatch(&self, key: &str, source: &Json, args: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let text = |name: &str| args.get(name).and_then(Json::as_str).unwrap_or_default().to_string();
        let optional_text = |name: &str| args.get(name).and_then(Json::as_str).map(ToString::to_string);
        let input = || args.get("input").cloned().unwrap_or(Json::Null);
        match key {
            "empty-list" => Ok(Json::Array(Vec::new())),
            "technologies" => Ok(self.technologies(args)),
            "bundles" => Ok(self.bundles(args)),
            "folders" => Ok(Json::Array(self.context.folders().iter().map(folder_source).collect())),
            "files" => Ok(Json::Array(self.context.files().iter().map(file_source).collect())),
            "sections" => Ok(Json::Array(self.context.sections().iter().map(section_source).collect())),
            "definitions" => Ok(Json::Array(self.context.definitions().iter().map(definition_source).collect())),
            "contributors" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.contributors()?.iter().filter(|contributor| matches_filter(&contributor.github, filter.as_ref())).map(contributor_source).collect()))
            }
            "goals" => Ok(Json::Array(self.context.goals()?.iter().map(goal_source).collect())),
            "tickets" => self.tickets(args),
            "policies" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.policies().iter().filter(|policy| matches_filter(&policy.name, filter.as_ref())).map(|policy| policy_source(policy, &self.statutes)).collect()))
            }
            "statutes" => Ok(Json::Array(self.statutes.iter().map(statute_source).collect())),
            "breachs" => {
                let result = self.context.analyze(optional_text("scope").as_deref())?;
                Ok(Json::Array(result.breachs.unwrap_or_default().iter().map(|breach| breach_source(breach, &self.statutes)).collect()))
            }
            "checkpoints" => Ok(Json::Array(self.context.checkpoints(args.get("limit").and_then(Json::as_i64))?.iter().map(checkpoint_source).collect())),
            "folder.children" => {
                let id = source.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.folders().iter().filter(|folder| folder.parent_id.as_deref() == Some(id.as_str())).map(folder_source).collect()))
            }
            "folder.files" => {
                let id = source.get("id").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.files().iter().filter(|file| file.folder_id.as_deref() == Some(id.as_str())).map(file_source).collect()))
            }
            "file.sections" => {
                let path = source.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.sections().iter().filter(|section| section.file_path == path).map(section_source).collect()))
            }
            "file.definitions" => {
                let path = source.get("path").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(Json::Array(self.context.definitions().iter().filter(|definition| definition.file_path == path).map(definition_source).collect()))
            }
            "item.file" => {
                let path = source.get("filePath").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(if path.is_empty() { Json::Null } else { self.file_by_path(&path) })
            }
            "definition.section" => {
                let path = source.get("sectionPath").and_then(Json::as_str).unwrap_or_default().to_string();
                if path.is_empty() {
                    return Ok(Json::Null);
                }
                let file_path = source.get("filePath").and_then(Json::as_str).unwrap_or_default().to_string();
                Ok(section_source(&Section {
                    id: String::new(),
                    name: path.clone(),
                    path,
                    file_path,
                    emoji: String::new(),
                    start_line: 0,
                    end_line: 0,
                    start_index: 0,
                    end_index: 0,
                    children: Vec::new(),
                    definitions: Vec::new(),
                }))
            }
            "section.children" => Ok(Self::section_children(source)),
            "statute.policy" => Ok(policy_source(
                &Policy {
                    id: "/policies/lint-scripts".to_string(),
                    name: "Lint scripts".to_string(),
                    description: None,
                    scopes: Some(vec!["**/*".to_string()]),
                    groups: Some(Vec::new()),
                    statutes: Some(Vec::new()),
                },
                &self.statutes,
            )),
            "ticket.author" => self.ticket_author_of(source),
            "contributor.contributions" => self.contributions(source),
            "query.node" => self.node(args),
            "query.repo" => Ok(self.repo()),
            "query.todos" => {
                let filter = self.filter_of(args);
                Ok(Json::Array(self.context.todos(filter.as_ref())?.iter().map(todo_source).collect()))
            }
            "query.interactions" => Ok(Json::Array(self.context.interactions()?.iter().map(interaction_resource_source).collect())),
            "query.drafts" => Ok(Json::Array(self.context.drafts()?.iter().map(draft_source).collect())),
            "query.bundle" => Ok(self.bundle_by_name(&text("name"))),
            "query.folder" => Ok(self.folder_by_path(&text("path"))),
            "query.file" => Ok(self.file_by_path(&text("path"))),
            "query.section" => {
                let path = text("path");
                let Some(Json::Array(segments)) = args.get("sectionPath") else { return Ok(Json::Null) };
                let joined = segments.iter().filter_map(Json::as_str).collect::<Vec<_>>().join("#");
                Ok(self.context.sections().iter().find(|section| section.file_path == path && section.path == joined).map_or_else(
                    || {
                        section_source(&Section {
                            id: String::new(),
                            name: joined.clone(),
                            path: String::new(),
                            file_path: String::new(),
                            emoji: String::new(),
                            start_line: 0,
                            end_line: 0,
                            start_index: 0,
                            end_index: 0,
                            children: Vec::new(),
                            definitions: Vec::new(),
                        })
                    },
                    section_source,
                ))
            }
            "query.definition" => {
                let path = text("path");
                let name = text("name");
                Ok(self.context.definitions().iter().find(|definition| definition.file_path == path && definition.name == name).map_or_else(
                    || {
                        definition_source(&Definition {
                            id: String::new(),
                            name: name.clone(),
                            kind: DefinitionKind::Implementation,
                            file_path: String::new(),
                            section_path: String::new(),
                            emoji: String::new(),
                            start_line: 0,
                            end_line: 0,
                            start_index: 0,
                            end_index: 0,
                        })
                    },
                    definition_source,
                ))
            }
            "query.contributor" => self.contributor_by_id(&text("id")),
            "query.ticket" => {
                let integer = |name: &str| args.get(name).and_then(Json::as_i64);
                let slug = text("slug");
                let tickets = self.context.tickets(integer("year"), integer("month"), integer("day"), None)?;
                Ok(tickets.iter().find(|ticket| ticket.slug == slug).map_or(Json::Null, |ticket| ticket_source(ticket, &self.root_dir)))
            }
            "query.policy" => Ok(self.policy_by_id(&text("id"))),
            "query.statute" => {
                let id = text("id");
                Ok(statute_source(&statute_meta_of(&Statute(id), &self.statutes)))
            }
            "query.analyze" => Ok(analyze_result_source(&self.context.analyze(optional_text("scope").as_deref())?, &self.statutes)),
            "mutation.fix" => Ok(fix_result_source(&self.context.fix(optional_text("scope").as_deref())?, &self.statutes)),
            "mutation.syncManagement" => Ok(Json::Bool(self.context.sync_management()?)),
            "mutation.goalCreate" => Ok(goal_source(&self.context.goal_create(decode_input(&input())?)?)),
            "mutation.goalChange" => {
                let mut decoded: GoalChangeInput = decode_input(&input())?;
                decoded.id = text("id");
                Ok(goal_source(&self.context.goal_change(decoded)?))
            }
            "mutation.goalClose" => Ok(goal_source(&self.context.goal_close(decode_input(&input())?)?)),
            "mutation.goalReopen" => Ok(goal_source(&self.context.goal_reopen(decode_input(&input())?)?)),
            "mutation.draftCreate" => Ok(draft_source(&self.context.draft_create(decode_input(&input())?)?)),
            "mutation.draftDelete" => Ok(Json::Bool(self.context.draft_delete(&text("id"))?)),
            "mutation.todoCreate" => Ok(todo_source(&self.context.todo_create(decode_input(&input())?)?)),
            "mutation.todoChange" => Ok(todo_source(&self.context.todo_change(decode_input(&input())?)?)),
            "mutation.todoDelete" => Ok(Json::Bool(self.context.todo_delete(&text("id"))?)),
            "mutation.ticketOpen" => Ok(ticket_source(&self.context.ticket_open(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketClose" => Ok(ticket_source(&self.context.ticket_close(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketReopen" => Ok(ticket_source(&self.context.ticket_reopen(decode_input(&input())?)?, &self.root_dir)),
            "mutation.ticketChange" => Ok(ticket_source(&self.context.ticket_change(decode_input(&input())?)?, &self.root_dir)),
            "mutation.contributorAdd" => Ok(contributor_source(&self.context.contributor_add(decode_input(&input())?)?)),
            "mutation.contributorRemove" => {
                self.context.contributor_remove(&text("github"))?;
                Ok(Json::Bool(true))
            }
            "mutation.folderCreate" => Ok(folder_source(&self.context.folder_create(&text("path"))?)),
            "mutation.folderMove" => Ok(folder_source(&self.context.folder_move(&text("src"), &text("dst"))?)),
            "mutation.folderDelete" => {
                self.context.folder_delete(&text("path"))?;
                Ok(Json::Bool(true))
            }
            "mutation.fileCreate" => Ok(file_source(&self.context.file_create(&text("path"))?)),
            "mutation.fileMove" => Ok(file_source(&self.context.file_move(&text("src"), &text("dst"))?)),
            "mutation.fileDelete" => {
                self.context.file_delete(&text("path"))?;
                Ok(Json::Bool(true))
            }
            "mutation.sectionCreate" => Ok(section_source(&self.context.section_create(&text("file"), &text("name"), optional_text("parent").as_deref())?)),
            "mutation.sectionMove" => Ok(section_source(&self.context.section_move(&text("file"), &text("oldName"), &text("newName"))?)),
            "mutation.sectionDelete" => {
                self.context.section_delete(&text("file"), &text("name"))?;
                Ok(Json::Bool(true))
            }
            "mutation.integrate" => Ok(file_source(&self.context.integrate(
                optional_text("source").as_deref(),
                optional_text("targetSection").as_deref(),
                optional_text("targetFile").as_deref(),
                optional_text("targetParent").as_deref(),
            )?)),
            "mutation.extract" => Ok(file_source(&self.context.extract(optional_text("sourceFile").as_deref(), optional_text("sourceSection").as_deref(), optional_text("targetFile").as_deref())?)),
            other => Err(ExecutionError::new(format!("no resolver registered for {other}"))),
        }
    }
}

//#endregion 💻️MutationResolvers

//#region 🧱️Executor

/// 🧱️ One executable schema bound to one repository context.
pub struct Executor<'a> {
    pub(crate) schema: Schema,
    pub(crate) context: &'a dyn RepoContext,
}

impl<'a> Executor<'a> {
    /// 🔷️ Binds the repo schema to a context.
    pub fn new(context: &'a dyn RepoContext) -> Executor<'a> {
        Executor { schema: build_schema(), context }
    }

    /// 🗺️ The schema this executor serves.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// 📬️ Parses and executes one request, returning the `data` payload.
    pub fn execute(&self, query: &str, variables: &Map<String, Json>) -> Result<Json, ExecutionError> {
        let document = parse(query).map_err(|error| ExecutionError::new(format!("graphql errors: [{}]", error.message)))?;
        let root_name = if document.operation == "mutation" {
            match &self.schema.mutation {
                Some(name) => name.clone(),
                None => return Err(ExecutionError::new("graphql errors: [mutation root is not configured]".to_string())),
            }
        } else {
            self.schema.query.clone()
        };
        let Some(root) = self.schema.object(&root_name) else {
            return Err(ExecutionError::new(format!("graphql errors: [{} root is not configured]", document.operation)));
        };
        let execution = Execution {
            schema: &self.schema,
            context: self.context,
            variables: variables.clone(),
            root_dir: self.context.root_dir(),
            statutes: self.context.statutes(),
            technologies: self.context.technologies(),
        };
        execution
            .execute_selections(&Json::Null, root, &document.selections)
            .map(Json::Object)
            .map_err(|error| ExecutionError::new(format!("graphql errors: [{}]", error.message)))
    }

    /// 📋️ Executes one request and renders the `data` payload the way the CLI prints it.
    pub fn execute_json(&self, query: &str, variables: &Map<String, Json>) -> Result<String, ExecutionError> {
        let data = self.execute(query, variables)?;
        serde_json::to_string_pretty(&data).map_err(|error| ExecutionError::new(error.to_string()))
    }

    /// 🔍️ Reports whether a request string is a document this grammar accepts.
    pub fn validate_query(&self, query: &str) -> Result<(), ParseError> {
        validate(query)
    }

    /// 📨️ The operation kind of a request string, without executing it.
    pub fn operation_type(&self, query: &str) -> Result<String, ParseError> {
        operation_type(query)
    }
}

//#endregion 🧱️Executor
