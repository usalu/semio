//! 📊️ Subject adapter of the crowd-view case: `crowd_view` of the `quiz` crate for every committed quiz and set of results.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/👁️views/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{crowd_view, Quiz, RunResult};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://📊️crowd-view/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🗃️ The crowd view of every committed vector.
    pub fn crowds(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let quizzes = decode!(vectors["quizzes"], Vec<Quiz>)?;
        let mut projection = Map::new();
        for vector in vectors["vectors"].as_array().ok_or("the vectors carry no vectors")? {
            let quiz = quizzes.iter().find(|quiz| vector["quiz"] == quiz.id.as_str()).ok_or_else(|| format!("unknown quiz in {}", vector["id"]))?;
            let view = crowd_view(quiz, &decode!(vector["results"], Vec<RunResult>)?);
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(view).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the Counter- and numpy-corroborated reference in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("crowds", subject::crowds);
    built
}
