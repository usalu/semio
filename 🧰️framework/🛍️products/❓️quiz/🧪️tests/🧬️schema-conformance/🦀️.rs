//! 🧬️ Subject adapter of the schema-conformance case: the typed twins and `quiz_issues`/`catalog_issues` of the `quiz` crate accept or reject every quiz document.
//!
//! A document is accepted when it decodes into its typed twin — which refuses unknown members like the
//! schema's `additionalProperties: false` — and the owned validator reports no issue.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/✅️validation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{catalog_issues, quiz_issues, Catalog, Quiz};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::path::{Path, PathBuf};

    const REJECTED: &str = "shared://🧬️schema-conformance/🔣️.json";
    const TEACHING: &str = "🎓️teaching";
    const SKIPPED: &[&str] = &["node_modules", "target", "dist", "📤️dist", ".git", "🗑️generated", "__pycache__"];
    const QUIZ_DIRECTORY: &str = "❓️quiz";
    const DOCUMENT: &str = "🔣️.json";

    /// ⚖️ Whether the typed twin decodes a quiz, or a catalog with its quizzes, and the owned validator accepts it.
    fn accepted(definition: &str, document: &Value, quizzes: &[Value]) -> bool {
        match definition {
            "Catalog" => {
                let quizzes: Option<Vec<Quiz>> = quizzes.iter().map(|quiz| serde_json::from_value::<Quiz>(quiz.clone()).ok()).collect();
                match (serde_json::from_value::<Catalog>(document.clone()), quizzes) {
                    (Ok(catalog), Some(quizzes)) => catalog_issues(&catalog, &quizzes).is_empty(),
                    _ => false,
                }
            }
            _ => serde_json::from_value::<Quiz>(document.clone()).map(|quiz| quiz_issues(&quiz).is_empty()).unwrap_or(false),
        }
    }

    /// 📍️ Every value a JSON pointer with `*` wildcards reaches, with its concrete pointer.
    fn matches(document: &Value, pointer: &str) -> Vec<(String, Value)> {
        let mut found = vec![(String::new(), document.clone())];
        for part in pointer.split('/').filter(|segment| !segment.is_empty()) {
            found = found
                .into_iter()
                .flat_map(|(path, value)| match (&value, part) {
                    (Value::Array(items), "*") => items.iter().enumerate().map(|(index, child)| (format!("{path}/{index}"), child.clone())).collect::<Vec<_>>(),
                    (Value::Object(members), "*") => members.iter().map(|(key, child)| (format!("{path}/{key}"), child.clone())).collect(),
                    (Value::Array(items), _) => part.parse::<usize>().ok().and_then(|index| items.get(index)).map(|child| vec![(format!("{path}/{part}"), child.clone())]).unwrap_or_default(),
                    (Value::Object(members), _) => members.get(part).map(|child| vec![(format!("{path}/{part}"), child.clone())]).unwrap_or_default(),
                    _ => Vec::new(),
                })
                .collect();
        }
        found
    }

    /// 📋️ The feature's table of `(id, fixture, pointer, definition, quizzes)` rows.
    fn rows(ctx: &Context) -> Result<Vec<Map<String, Value>>, String> {
        let table = ctx.data_table()?;
        let header = table.first().ok_or("the table has no header")?;
        Ok(table.iter().skip(1).map(|row| header.iter().cloned().zip(row.iter().map(|cell| Value::String(cell.clone()))).collect()).collect())
    }

    /// 🔎️ Every `❓️quiz/🔣️.json` under the teaching area, repo-relative, in path order.
    fn teaching_documents(root: &Path) -> Vec<String> {
        let mut found = Vec::new();
        let mut pending = vec![root.join(TEACHING)];
        while let Some(directory) = pending.pop() {
            let Ok(entries) = std::fs::read_dir(&directory) else { continue };
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() && !SKIPPED.contains(&name.as_str()) {
                    pending.push(path);
                } else if name == DOCUMENT && directory.file_name().is_some_and(|parent| parent.to_string_lossy() == QUIZ_DIRECTORY) {
                    found.push(path.strip_prefix(root).unwrap_or(&path).components().map(|part| part.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>().join("/"));
                }
            }
        }
        found.sort();
        found
    }

    /// 📄️ A JSON document from disk.
    fn read(path: &Path) -> Result<Value, String> {
        serde_json::from_slice(&std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// 🔣️ A projection as the owned protocol value.
    fn projected(value: Map<String, Value>) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&Value::Object(value).to_string())?))
    }

    /// 🧫️ Every quiz and catalog of the committed vectors, accepted or rejected.
    pub fn fixture_documents(ctx: &Context) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for row in rows(ctx)? {
            let field = |name: &str| row.get(name).and_then(Value::as_str).unwrap_or_default().to_string();
            if field("definition") != "Quiz" && field("definition") != "Catalog" {
                continue;
            }
            let document: Value = serde_json::from_slice(&ctx.fixture_bytes(&field("fixture"))?).map_err(|error| error.to_string())?;
            let quizzes = if field("quizzes") == "-" { Vec::new() } else { matches(&document, &field("quizzes")).first().and_then(|(_, quizzes)| quizzes.as_array().cloned()).unwrap_or_default() };
            let verdicts: Map<String, Value> = matches(&document, &field("pointer")).into_iter().map(|(path, value)| (path, Value::Bool(accepted(&field("definition"), &value, &quizzes)))).collect();
            projection.insert(field("id"), Value::Object(verdicts));
        }
        projected(projection)
    }

    /// 🎓️ Every quiz and catalog authored in the teaching area, accepted or rejected.
    pub fn repository_quizzes(ctx: &Context) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for path in teaching_documents(&ctx.repo_root) {
            let absolute = ctx.repo_root.join(&path);
            let document = read(&absolute)?;
            let verdict = match document["schema"].as_str() {
                Some("semio.quiz/v1") => accepted("Quiz", &document, &[]),
                Some("semio.quiz.catalog/v1") => {
                    let directory = absolute.parent().ok_or("a catalog has no directory")?;
                    let quizzes = document["quizzes"].as_array().into_iter().flatten().filter_map(Value::as_str).map(|quiz| read(&directory.join(quiz))).collect::<Result<Vec<_>, _>>()?;
                    accepted("Catalog", &document, &quizzes)
                }
                _ => false,
            };
            projection.insert(path, Value::Bool(verdict));
        }
        projected(projection)
    }

    /// 🧩️ Whether the typed twin of a definition decodes a document — a definition the twins tag by `kind` through the union that carries the tag; a definition without a twin is an error.
    fn twin_decodes(definition: &str, document: &Value) -> Result<bool, String> {
        let union = match definition {
            "CompareHint" | "ProfileHint" | "GroupHint" | "CategoryHint" => "Hint",
            "SheetClassificationTask" | "SheetSortingTask" | "SheetMatchingTask" => "SheetTask",
            "SortingAnswer" | "MatchingAnswer" => "Answer",
            other => other,
        };
        macro_rules! decodes {
            ($($name:ident),*) => {
                match union {
                    $(stringify!($name) => Ok(serde_json::from_value::<quiz::$name>(document.clone()).is_ok()),)*
                    _ => Err(format!("no typed twin is named {definition}")),
                }
            };
        }
        decodes!(Challenge, Quantity, ShortText, Verdict, SortingItem, MatchingItem, ClassificationItem, Category, Axis, SheetItem, Hint, Best, SheetAxis, SheetDimension, SheetTask, Sheet, Answer, ClassificationItemResult, SortingItemResult, MatchingItemResult, RunResult, Command, Rejection, Event, RunView, RunSummary, LearnerView, LeaderboardRow, BadgeRule)
    }

    /// 🚫️ The unbroken bases accepted and every rejected document rejected; every conforming typed instance must decode into its twin, which is checked and not projected.
    pub fn rejected_quizzes(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(REJECTED)?).map_err(|error| error.to_string())?;
        for vector in vectors["instances"].as_array().ok_or("the vectors carry no instances")?.iter().filter(|vector| vector.get("violates").is_none()) {
            if !twin_decodes(vector["definition"].as_str().unwrap_or_default(), &vector["document"])? {
                return Err(format!("instances/{}: the typed twin refuses a conforming {}", vector["id"], vector["definition"]));
            }
        }
        let mut projection = Map::new();
        for vector in vectors["accepted"].as_array().into_iter().flatten().chain(vectors["rejected"].as_array().into_iter().flatten()) {
            let quizzes: Vec<Value> = vector["document"]["quizzes"].as_array().into_iter().flatten().filter_map(Value::as_str).filter_map(|path| vectors["quizzes"].get(path).cloned()).collect();
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), Value::Bool(accepted(vector["definition"].as_str().unwrap_or_default(), &vector["document"], &quizzes)));
        }
        projected(projection)
    }
}

/// 🧭️ Subject role only — the oracle is python-jsonschema in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("fixture-documents", subject::fixture_documents).subject("repository-quizzes", subject::repository_quizzes).subject("rejected-quizzes", subject::rejected_quizzes);
    built
}
