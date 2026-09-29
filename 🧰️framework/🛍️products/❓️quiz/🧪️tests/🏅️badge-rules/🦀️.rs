//! 🏅️ Subject adapter of the badge-rules case: `earned_badges` of the `quiz` crate for every committed set of results.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🏅️badges/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{earned_badges, Badge, Quiz, RunResult};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::collections::BTreeSet;

    const VECTORS: &str = "shared://🏅️badge-rules/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🗃️ The newly earned badges of every committed vector.
    pub fn awards(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let quizzes = decode!(vectors["quizzes"], Vec<Quiz>)?;
        let badges = decode!(vectors["badges"], Vec<Badge>)?;
        let mut projection = Map::new();
        for vector in vectors["vectors"].as_array().ok_or("the vectors carry no vectors")? {
            let earned = earned_badges(&badges, &quizzes, &decode!(vector["results"], Vec<RunResult>)?, &decode!(vector["held"], BTreeSet<String>)?);
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(earned).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("awards", subject::awards);
    built
}
