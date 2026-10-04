//! 📏️ Subject adapter of the sorting-concordance case: `score_task` of the `quiz` crate for every committed sorting answer.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/📏️scoring/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{score_task, Answer, SheetTask, Task};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://📏️sorting-concordance/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🗃️ Every committed answer of one vector group — or its absence on a timed sheet task — scored against its task and sheet task.
    fn scored(ctx: &Context, group: &str) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let tasks = decode!(vectors["tasks"], Vec<Task>)?;
        let mut projection = Map::new();
        for vector in vectors[group].as_array().ok_or_else(|| format!("the vectors carry no {group}"))? {
            let task = tasks.iter().find(|task| vector["task"] == task.id().as_str()).ok_or_else(|| format!("unknown task in {vector}"))?;
            let answer = vector.get("answer").map(|answer| decode!(answer, Answer)).transpose()?;
            let result = score_task(task, &decode!(vector["sheetTask"], SheetTask)?, answer.as_ref()).ok_or_else(|| format!("score_task refused {}", vector["id"]))?;
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(result).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🧮️ The mixed sorting vectors.
    pub fn scores(ctx: &Context) -> Result<Outcome, String> {
        scored(ctx, "vectors")
    }

    /// 📶️ The equally spaced orders Spearman's ρ judges.
    pub fn rank_weights(ctx: &Context) -> Result<Outcome, String> {
        scored(ctx, "rankVectors")
    }

    /// 🔮️ Guesses where the keys are hidden, partly guessed or absent on a timed sheet task.
    pub fn guessed(ctx: &Context) -> Result<Outcome, String> {
        scored(ctx, "guessed")
    }

    /// 🩹️ Every committed input that bypasses validation, scored — `null` where the core scores none.
    pub fn degraded(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let mut projection = Map::new();
        for vector in vectors["degraded"].as_array().ok_or("the vectors carry no degraded group")? {
            let result = score_task(&decode!(vector["task"], Task)?, &decode!(vector["sheetTask"], SheetTask)?, vector.get("answer").map(|answer| decode!(answer, Answer)).transpose()?.as_ref());
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(result).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the scipy-corroborated pair loop in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("scores", subject::scores).subject("rank-weights", subject::rank_weights).subject("guessed", subject::guessed).subject("degraded", subject::degraded);
    built
}
