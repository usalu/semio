//! ✅️ Subject adapter of the answer-validation case: `answer_rejection` and `answer_complete` of the `quiz` crate.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/✅️validation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, json, Map, Value};
    use quiz::{answer_complete, answer_rejection, Answer, SheetTask};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://✅️answer-validation/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// ⚖️ The rejection, and completeness for an answer that is valid or absent.
    fn verdict(sheet_task: &SheetTask, answer: Option<&Answer>) -> Value {
        match answer.and_then(|answer| answer_rejection(sheet_task, answer)) {
            Some(rejection) => json!({ "rejection": rejection }),
            None => json!({ "rejection": null, "complete": answer_complete(sheet_task, answer) }),
        }
    }

    /// 🗃️ Every committed answer judged against its sheet task.
    pub fn verdicts(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let tasks = decode!(vectors["sheetTasks"], Vec<SheetTask>)?;
        let mut projection = Map::new();
        for vector in vectors["vectors"].as_array().ok_or("the vectors carry no vectors")? {
            let task = tasks.iter().find(|task| vector["sheetTask"] == task.id().as_str()).ok_or_else(|| format!("unknown sheet task in {vector}"))?;
            let answer = if vector.get("answer").is_some() { Some(decode!(vector["answer"], Answer)?) } else { None };
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), verdict(task, answer.as_ref()));
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("verdicts", subject::verdicts);
    built
}
