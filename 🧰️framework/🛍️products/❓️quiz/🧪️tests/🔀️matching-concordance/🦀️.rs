//! 🔀️ Subject adapter of the matching-concordance case: `score_task` of the `quiz` crate for every committed matching answer.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/📏️scoring/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{score_task, Answer, SheetTask, Task};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🔀️matching-concordance/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🗃️ Every committed answer scored against its task and sheet task.
    pub fn scores(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let tasks = decode!(vectors["tasks"], Vec<Task>)?;
        let mut projection = Map::new();
        for vector in vectors["vectors"].as_array().ok_or("the vectors carry no vectors")? {
            let task = tasks.iter().find(|task| vector["task"] == task.id().as_str()).ok_or_else(|| format!("unknown task in {vector}"))?;
            let result = score_task(task, &decode!(vector["sheetTask"], SheetTask)?, &decode!(vector["answer"], Answer)?).ok_or_else(|| format!("score_task refused {}", vector["id"]))?;
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(result).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🩹️ Every committed input that bypasses validation, scored — `null` where the core scores none.
    pub fn degraded(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let mut projection = Map::new();
        for vector in vectors["degraded"].as_array().ok_or("the vectors carry no degraded group")? {
            let result = score_task(&decode!(vector["task"], Task)?, &decode!(vector["sheetTask"], SheetTask)?, &decode!(vector["answer"], Answer)?);
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(result).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the scipy-corroborated pair loop in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("scores", subject::scores).subject("degraded", subject::degraded);
    built
}
