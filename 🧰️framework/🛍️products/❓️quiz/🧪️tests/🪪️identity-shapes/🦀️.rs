//! 🪪️ Subject adapter of the identity-shapes case: the handle policy and the id shapes of the `quiz` crate.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/✅️validation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{command_rejection, handle_actor_id, normalize_handle, query_rejection, Command, Query};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🪪️identity-shapes/🔣️.json";

    /// 🧫️ One group of the committed vectors.
    fn group(ctx: &Context, name: &str) -> Result<Vec<Value>, String> {
        let document: Value = serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))?;
        document.get(name).and_then(Value::as_array).cloned().ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🔣️ A projection as the owned protocol value.
    fn projected(value: Map<String, Value>) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&Value::Object(value).to_string())?))
    }

    /// 📏️ Ascending code points as `XXXX` or `XXXX-YYYY` words.
    fn ranges(points: &[u32]) -> String {
        let mut found: Vec<(u32, u32)> = Vec::new();
        for &point in points {
            match found.last_mut() {
                Some(last) if last.1 + 1 == point => last.1 = point,
                _ => found.push((point, point)),
            }
        }
        found.iter().map(|&(low, high)| if low == high { format!("{low:04X}") } else { format!("{low:04X}-{high:04X}") }).collect::<Vec<_>>().join(" ")
    }

    /// 🏷️ Every committed handle normalized, with the id of its stream, or refused.
    pub fn handles(ctx: &Context) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in group(ctx, "handles")? {
            let answer = normalize_handle(vector["handle"].as_str().unwrap_or_default()).map(|normalized| serde_json::json!({ "display": normalized.display, "key": normalized.key, "actor": handle_actor_id(&normalized.key) }));
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), answer.unwrap_or(Value::Null));
        }
        projected(projection)
    }

    /// 🔠️ Every Unicode scalar value `normalize_handle` keeps unchanged between two letters, and the lowercase of every such character that changes.
    pub fn alphabet(_ctx: &Context) -> Result<Outcome, String> {
        let mut members = Vec::new();
        let mut folds = Map::new();
        for character in (0..=0x10FFFFu32).filter_map(char::from_u32) {
            let framed = format!("a{character}a");
            let Some(normalized) = normalize_handle(&framed).filter(|normalized| normalized.display == framed) else { continue };
            members.push(u32::from(character));
            let key: String = normalized.key.chars().skip(1).take(normalized.key.chars().count() - 2).collect();
            if key != character.to_string() {
                folds.insert(format!("{:04X}", u32::from(character)), Value::String(key));
            }
        }
        projected(Map::from_iter([("alphabet".to_string(), serde_json::json!({ "members": ranges(&members), "folds": folds }))]))
    }

    /// 🛃️ Every committed command and query, refused for its shapes or not.
    pub fn shapes(ctx: &Context) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in group(ctx, "shapes")? {
            let rejection = match vector["definition"].as_str() {
                Some("Command") => command_rejection(&serde_json::from_value::<Command>(vector["document"].clone()).map_err(|error| format!("{error} in {vector}"))?),
                _ => query_rejection(&serde_json::from_value::<Query>(vector["document"].clone()).map_err(|error| format!("{error} in {vector}"))?),
            };
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(rejection).map_err(|error| error.to_string())?);
        }
        projected(projection)
    }
}

/// 🧭️ Subject role only — the oracle is python-jsonschema held to the Unicode Character Database in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("handles", subject::handles).subject("alphabet", subject::alphabet).subject("shapes", subject::shapes);
    built
}
