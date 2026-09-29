//! 🃏️ Subject adapter of the sheet-assembly case: `sheet_of` of the `quiz` crate for every committed quiz and seed.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🃏️sheet/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{sheet_of, Quiz};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::collections::BTreeMap;

    const VECTORS: &str = "shared://🃏️sheet-assembly/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🗃️ Every committed sheet assembled by the crate.
    pub fn sheets(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let quizzes: BTreeMap<String, Quiz> = decode!(vectors["quizzes"], Vec<Quiz>)?.into_iter().map(|quiz| (quiz.id.clone(), quiz)).collect();
        let mut projection = Map::new();
        for vector in vectors["sheets"].as_array().ok_or("the vectors carry no sheets")? {
            let quiz = quizzes.get(vector["quiz"].as_str().unwrap_or_default()).ok_or_else(|| format!("unknown quiz in {vector}"))?;
            let seed = vector["seed"].as_u64().and_then(|seed| u32::try_from(seed).ok()).ok_or_else(|| format!("no u32 seed in {vector}"))?;
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(sheet_of(quiz, seed)).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("sheets", subject::sheets);
    built
}
