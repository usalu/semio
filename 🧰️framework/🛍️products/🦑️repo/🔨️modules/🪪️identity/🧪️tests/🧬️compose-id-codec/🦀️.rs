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
fn the_owned_codecs_round_trip(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_identity as id;
    let file = ctx.fixture_json("shared://📡️emoji-vectors.json")?;
    let normalizations: Vec<Json> = file
        .array("normalizations")
        .iter()
        .map(|entry| {
            let emoji = text(entry);
            Json::String(format!("{}={}", code_points(&emoji), code_points(&id::emoji_text(&emoji))))
        })
        .collect();
    let refs: Vec<Json> = file
        .array("refs")
        .iter()
        .map(|vector| {
            let parsed = id::parse_artifact_ref(&vector.str("input"));
            Json::String(format!("{}={}|{}|{}", vector.str("name"), parsed.kind, parsed.path, parsed.section_parts.join("#")))
        })
        .collect();
    let goals: Vec<Json> = file
        .array("goalPaths")
        .iter()
        .map(|entry| {
            let goal_path = text(entry);
            let compose = id::goal_path_to_compose_id(&goal_path);
            Json::String(format!("{goal_path}={compose}|{}", id::compose_id_to_goal_segments(&compose).join("/")))
        })
        .collect();
    let contributors: Vec<Json> = file
        .array("contributors")
        .iter()
        .map(|entry| {
            let alias = text(entry);
            let compose = id::contributor_to_compose_id(&alias);
            Json::String(format!("{alias}={compose}|{}", id::compose_id_to_contributor_flat(&compose)))
        })
        .collect();
    let mut identifiers = Vec::new();
    for entry in file.array("identifierSeeds") {
        let Json::Number(seed) = entry else { continue };
        let seed = seed as u64;
        let mut entropy = id::SeededEntropy::new(seed);
        let value = id::Id::new_from(&mut entropy)?;
        identifiers.push(Json::String(format!("{seed}={value}")));
    }
    let vocabulary: Vec<Json> = id::all_entity_emojis().iter().map(|emoji| Json::String(code_points(emoji))).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("normalizations".to_string(), Json::Array(normalizations)),
        ("refs".to_string(), Json::Array(refs)),
        ("goals".to_string(), Json::Array(goals)),
        ("contributors".to_string(), Json::Array(contributors)),
        ("identifiers".to_string(), Json::Array(identifiers)),
        ("vocabulary".to_string(), Json::Array(vocabulary)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-owned-codecs-round-trip", the_owned_codecs_round_trip);
    adapter
}

//#endregion 🔖️Registration
