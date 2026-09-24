//! 🔮️ Third-party reference for the `s.stdio.semio@v1/base` ENVELOPE: the committed JSON carrier
//! (`../🧬️schema/📸️snapshot/🔣️.json`, `../🧬️schema/🧬️mutations/🔣️.json`) read by `json` (json-rust)
//! 0.12 and routed by the envelope's own published law, so `✉️mutate-semio-base` compares this
//! repository's `apply_semio_mutation` with an answer no line of the subject produced.
//!
//! json-rust, not `serde_json`: the subject crate declares `serde_json` as a production dependency,
//! and `🧾️json`'s own oracle records why a reference the implementation already links is no
//! independent evidence. json-rust appears nowhere in the production graph and is already the
//! registered JSON reference of this crate, so the envelope adds no dependency.
//!
//! What the carrier can say, and therefore what this module answers:
//! - `setSnapshot` REPLACES the envelope with its payload, whatever arm either side names;
//! - an `apply<Arm>` wrapper whose arm matches the envelope's `subset` REACHES that arm, and the
//!   arm's own committed result (produced by that arm's independent implementation, never by this
//!   repository's Rust) is the answer;
//! - an `apply<Arm>` wrapper whose arm does not match is REFUSED with `mutation.target-missing`, and
//!   the envelope stays exactly as it stood;
//! - every inverse restores the envelope the mutation started from.
//!
//! Numbers are projected through their decimal lexeme into `f64`, the same correctly rounded
//! reading the host's own parser performs, so the two sides agree bit for bit on every committed
//! value. No json-rust type crosses this module's public surface.
//!
//! @see ../🔣️.json — the oracle registration and the mutation catalog this module answers for.
//! @see ../🧪️tests/✉️mutate-semio-base/🥒️.feature — the case that compares against it.

use semio_repo_test_host::Json;

//#region 🔖️Routing
/// 🚦️ The fault code the envelope raises for a wrapped mutation that names another arm.
pub const TARGET_MISSING: &str = "mutation.target-missing";

/// 🧭️ The routed result of one envelope mutation read off the carrier: the envelope it leaves
/// behind and the refusing fault codes it raised.
#[derive(Clone, Debug, PartialEq)]
pub struct Routed {
    pub envelope: Json,
    pub refused: Vec<String>,
}
//#endregion 🔖️Routing

//#region 🔖️Reader
/// 📖️ Reads one committed carrier document through json-rust onto the host's own value tree.
#[cfg(feature = "oracles")]
pub fn read_carrier(bytes: &[u8]) -> Result<Json, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("envelope carrier is not UTF-8: {error}"))?;
    json::parse(text).map_err(|error| format!("json-rust refused the envelope carrier: {error}")).and_then(|value| project(&value))
}

#[cfg(not(feature = "oracles"))]
pub fn read_carrier(_bytes: &[u8]) -> Result<Json, String> {
    Err("the envelope carrier reference needs the `oracles` feature".to_string())
}

#[cfg(feature = "oracles")]
fn project(value: &json::JsonValue) -> Result<Json, String> {
    Ok(match value {
        json::JsonValue::Null => Json::Null,
        json::JsonValue::Boolean(flag) => Json::Bool(*flag),
        json::JsonValue::Number(number) => Json::Number(number.to_string().parse::<f64>().map_err(|error| format!("json-rust number {number} has no f64 reading: {error}"))?),
        json::JsonValue::Short(text) => Json::String(text.as_str().to_string()),
        json::JsonValue::String(text) => Json::String(text.clone()),
        json::JsonValue::Array(items) => Json::Array(items.iter().map(project).collect::<Result<_, _>>()?),
        json::JsonValue::Object(members) => Json::Object(members.iter().map(|(key, member)| project(member).map(|member| (key.to_string(), member))).collect::<Result<_, _>>()?),
    })
}
//#endregion 🔖️Reader

//#region 🔖️Envelope
/// 🏷️ The arm an envelope carries — its `subset.subset` discriminator.
pub fn envelope_arm(envelope: &Json) -> Option<String> {
    match envelope.get("subset")?.get("subset")? {
        Json::String(arm) => Some(arm.clone()),
        _ => None,
    }
}

/// 🏷️ The adjacently tagged verb of an envelope mutation — `setSnapshot` or an `apply<Arm>` wrapper.
pub fn mutation_tag(mutation: &Json) -> Option<String> {
    match mutation.get("mutation")? {
        Json::String(tag) => Some(tag.clone()),
        _ => None,
    }
}

/// 🏷️ The arm an `apply<Arm>` wrapper tag names on the carrier — `applyBrep` reaches `brep`.
pub fn wrapped_arm(tag: &str) -> Option<String> {
    let mut rest = tag.strip_prefix("apply")?.chars();
    let first = rest.next()?;
    Some(first.to_lowercase().chain(rest).collect())
}

/// ▶️ Routes `mutation` against `before` by the envelope's published law. `arm_result` is the wrapped
/// arm's own committed result, required only when the mutation reaches a matching arm.
pub fn route(before: &Json, mutation: &Json, arm_result: Option<&Json>) -> Result<Routed, String> {
    let tag = mutation_tag(mutation).ok_or("the envelope mutation carries no string `mutation` tag")?;
    if tag == "setSnapshot" {
        let snapshot = mutation.get("payload").and_then(|payload| payload.get("snapshot")).ok_or("setSnapshot carries no payload.snapshot")?;
        return Ok(Routed { envelope: snapshot.clone(), refused: Vec::new() });
    }
    let arm = wrapped_arm(&tag).ok_or_else(|| format!("the envelope mutation tag {tag} names neither setSnapshot nor an apply<Arm> wrapper"))?;
    if envelope_arm(before).as_deref() != Some(arm.as_str()) {
        return Ok(Routed { envelope: before.clone(), refused: vec![TARGET_MISSING.to_string()] });
    }
    let reached = arm_result.ok_or_else(|| format!("the {tag} mutation reaches the {arm} arm, and no committed arm result was supplied"))?;
    if envelope_arm(reached).as_deref() != Some(arm.as_str()) {
        return Err(format!("the committed {arm} arm result does not carry the {arm} arm"));
    }
    Ok(Routed { envelope: reached.clone(), refused: Vec::new() })
}

/// ↩️ The inverse law read off the carrier: whatever a mutation did, its inverse restores `before`.
pub fn restore(before: &Json) -> Routed {
    Routed { envelope: before.clone(), refused: Vec::new() }
}
//#endregion 🔖️Envelope
