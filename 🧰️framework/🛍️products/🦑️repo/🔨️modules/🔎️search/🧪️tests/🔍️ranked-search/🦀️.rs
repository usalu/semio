//! 🦀️ Rust side of the ranked search case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn vectors_rank_the_same_way(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_search::{Index, MatchQuery};
    let file = ctx.fixture_json("shared://📡️ranking-vectors.json")?;
    let mut rankings = Vec::new();
    for vector in file.array("vectors") {
        let name = vector.str("name");
        let mut index = Index::new();
        for (id, text) in sorted_pairs(vector.get("documents")) {
            index.index(&id, &text).map_err(|error| error.to_string())?;
        }
        for id in vector.array("deleted") {
            if let Json::String(value) = id {
                index.delete(&value).map_err(|error| error.to_string())?;
            }
        }
        for (id, text) in sorted_pairs(vector.get("reindexed")) {
            index.index(&id, &text).map_err(|error| error.to_string())?;
        }
        let terms: Vec<MatchQuery> = vector
            .array("terms")
            .iter()
            .map(|term| MatchQuery::new(&term.str("term")).with_fuzziness(number_of(term.get("fuzziness")) as usize))
            .collect();
        let size = number_of(vector.get("size")) as i64;
        let result = index.search(&terms, size).map_err(|error| error.to_string())?;
        let hits: Vec<String> = result.hits.iter().map(|hit| format!("{}@{}", hit.id, hit.score)).collect();
        rankings.push(Json::String(format!("{name}={}:{}", result.total, hits.join(","))));
    }
    Ok(Outcome::projection(Json::Object(vec![("rankings".to_string(), Json::Array(rankings))])))
}

/// 🔤️ Object members as `(key, value)` pairs sorted by key, so indexing order is identical everywhere.
#[cfg(feature = "sut")]
fn sorted_pairs(value: Option<&Json>) -> Vec<(String, String)> {
    let Some(Json::Object(entries)) = value else { return Vec::new() };
    let mut pairs: Vec<(String, String)> = entries
        .iter()
        .map(|(key, member)| {
            let text = match member {
                Json::String(inner) => inner.clone(),
                other => other.to_string(),
            };
            (key.clone(), text)
        })
        .collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
}

/// 🔢️ A numeric member, or zero.
#[cfg(feature = "sut")]
fn number_of(value: Option<&Json>) -> f64 {
    match value {
        Some(Json::Number(number)) => *number,
        _ => 0.0,
    }
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("vectors-rank-the-same-way", vectors_rank_the_same_way);
    adapter
}

//#endregion 🔖️Registration
