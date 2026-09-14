//! 🦀️ Rust side of the entity emoji codec case. The subject half is gated behind the `sut` feature
//! so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

/// 🔤️ Renders a string as a `U+XXXX` sequence so a projection never hides a selector.
#[cfg(feature = "sut")]
fn code_points(value: &str) -> String {
    if value.is_empty() {
        return "-".to_string();
    }
    value.chars().map(|current| format!("U+{:04X}", current as u32)).collect::<Vec<_>>().join(" ")
}

/// 🔤️ A string member of an array element, or the empty string.
#[cfg(feature = "sut")]
fn text(value: &Json) -> String {
    match value {
        Json::String(inner) => inner.clone(),
        other => other.to_string(),
    }
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_leading_emoji_grapheme_is_unicodes(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📡️emoji-vectors.json")?;
    let graphemes: Vec<Json> = file
        .array("graphemes")
        .iter()
        .map(|vector| {
            let (emoji, remaining) = semio_framework_repo_identity::extract_entity_emoji(&vector.str("input"));
            Json::String(format!("{}={emoji}|{remaining}", vector.str("name")))
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![("graphemes".to_string(), Json::Array(graphemes))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-leading-emoji-grapheme-is-unicodes", the_leading_emoji_grapheme_is_unicodes);
    adapter
}

//#endregion 🔖️Registration
