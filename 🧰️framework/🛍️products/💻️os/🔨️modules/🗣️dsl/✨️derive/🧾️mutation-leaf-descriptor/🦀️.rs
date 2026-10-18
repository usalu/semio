//! 🧾️ Mutation leaf descriptor of the dsl derive expansion: the leaf descriptor enums and JSON record, the descriptor parser with its key, string, number, kebab and surface readers and the descriptor emitter. See [`super`].

use super::*;

//#region 🔣️MutationLeafJson
#[derive(Debug, PartialEq, Eq)]
enum MutationLeafInvertibility { SelfInvertible, ExplicitMutation, Plan, NonInvertible }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafDiffParticipation { Detect, ApplyOnly, Plan, None }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafOutcomeClass { Applied, NoOp, Empty, Disjoint, Rejected }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafComposition { Atomic, Composite }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafLanguageSurface { Rust, Typescript, Graphql, Protobuf, JsonSchema, Text, Binary }

#[derive(Debug, PartialEq, Eq)]
pub(super) struct MutationLeafJson {
    schema_version: u32,
    pub(super) owner: String,
    pub(super) semantic_kind: String,
    display_name: String,
    emoji: String,
    pub(super) aggregate_variant: String,
    pub(super) payload_schema: String,
    text_opcode: Option<String>,
    binary_tag: Option<u32>,
    invertibility: MutationLeafInvertibility,
    diff_participation: MutationLeafDiffParticipation,
    outcome_classes: Vec<MutationLeafOutcomeClass>,
    composition: MutationLeafComposition,
    required_language_surfaces: Vec<MutationLeafLanguageSurface>,
    #[doc(hidden)] pub editable: bool,
}

const MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14] = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"];

/// ✏️ The one optional descriptor key: `false` declares the leaf withdraw-only (design §22.20); absent means editable.
const MUTATION_LEAF_EDITABLE_KEY: &str = "editable";

#[doc(hidden)] pub fn parse_mutation_leaf_descriptor(raw: &[u8], authority: &MutationSourceAuthority) -> Result<MutationLeafJson, String> {
    mutation_leaf_reject_duplicate_keys(raw)?;
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| format!("malformed mutation descriptor JSON: {error}"))?;
    let object = value.as_object().ok_or_else(|| "mutation descriptor must be an object".to_string())?;
    if MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str()) && key != MUTATION_LEAF_EDITABLE_KEY) { return Err("mutation descriptor must contain exactly the fourteen schema fields, and beside them only the optional editable".to_string()); }
    let string = |key| mutation_leaf_string(object.get(key).unwrap(), key);
    let schema_version = mutation_leaf_u32(object.get("schemaVersion").unwrap(), "schemaVersion")?;
    if schema_version != 1 { return Err("schemaVersion must equal 1".to_string()); }
    let owner = string("owner")?;
    if owner != authority.owner { return Err("descriptor owner does not exactly match source owner".to_string()); }
    let semantic_kind = string("semanticKind")?;
    if !mutation_leaf_kebab(&semantic_kind) { return Err("semanticKind must be lowercase kebab-case".to_string()); }
    if authority.expected_semantic_kind.as_ref().is_some_and(|expected| expected != &semantic_kind) { return Err("descriptor semanticKind does not match its exact registered domain owner".to_string()); }
    let display_name = string("displayName")?;
    let emoji = string("emoji")?;
    let aggregate_variant = string("aggregateVariant")?;
    if !mutation_leaf_pascal(&aggregate_variant) { return Err("aggregateVariant must be ASCII PascalCase".to_string()); }
    let payload_schema = string("payloadSchema")?;
    let text_opcode = match object.get("textOpcode").unwrap() { serde_json::Value::Null => None, value => { let value = mutation_leaf_string(value, "textOpcode")?; if !mutation_leaf_kebab(&value) { return Err("textOpcode must be lowercase kebab-case or null".to_string()); } Some(value) } };
    let binary_tag = match object.get("binaryTag").unwrap() { serde_json::Value::Null => None, value => Some(mutation_leaf_u32(value, "binaryTag")?) };
    let invertibility = match string("invertibility")?.as_str() { "self" => MutationLeafInvertibility::SelfInvertible, "explicit-mutation" => MutationLeafInvertibility::ExplicitMutation, "plan" => MutationLeafInvertibility::Plan, "non-invertible" => MutationLeafInvertibility::NonInvertible, _ => return Err("invertibility is not a schema enum value".to_string()) };
    let diff_participation = match string("diffParticipation")?.as_str() { "detect" => MutationLeafDiffParticipation::Detect, "apply-only" => MutationLeafDiffParticipation::ApplyOnly, "plan" => MutationLeafDiffParticipation::Plan, "none" => MutationLeafDiffParticipation::None, _ => return Err("diffParticipation is not a schema enum value".to_string()) };
    let outcome_classes = mutation_leaf_outcomes(object.get("outcomeClasses").unwrap())?;
    let composition = match string("composition")?.as_str() { "atomic" => MutationLeafComposition::Atomic, "composite" => MutationLeafComposition::Composite, _ => return Err("composition is not a schema enum value".to_string()) };
    let required_language_surfaces = mutation_leaf_surfaces(object.get("requiredLanguageSurfaces").unwrap())?;
    let editable = match object.get(MUTATION_LEAF_EDITABLE_KEY) { None => true, Some(serde_json::Value::Bool(value)) => *value, Some(_) => return Err("editable must be a boolean".to_string()) };
    Ok(MutationLeafJson { schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces, editable })
}

fn mutation_leaf_string(value: &serde_json::Value, key: &str) -> Result<String, String> { value.as_str().filter(|value| !value.is_empty()).map(str::to_owned).ok_or_else(|| format!("{key} must be a nonempty string")) }

fn mutation_leaf_u32(value: &serde_json::Value, key: &str) -> Result<u32, String> {
    let number = value.as_f64().ok_or_else(|| format!("{key} must be an integer"))?;
    if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > u32::MAX as f64 { return Err(format!("{key} must be a u32 integer")); }
    Ok(number as u32)
}

/// 🔤️ Lowercase kebab-case with at least one hyphen — a mutation kind is always multi-word.
///
/// 🧭️This mirrors `mutation_leaf_descriptor_kebab` in the kernel
/// (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`), whose final expression IS
/// `hyphen`: a kind without one is rejected there no matter what this copy says. The two must agree.
///
/// 🐛️This once dropped the hyphen clause, on the reasoning that "kebab-case includes a single word"
/// and that no consumer splits the kind on `-`. Both are true and both are beside the point — the
/// kernel is the authority, and relaxing only this copy made single-word kinds pass the derive and
/// then panic in const-eval at `validate_mutation_leaf_source`, which reports every field failure
/// through one message ("Mutations leaf source must match its aggregate workspace and direct owner")
/// and so pointed nowhere near the kind. `✳️drawing`'s `rotate`/`scale`/`group`/`ungroup`/`flatten`/
/// `unflatten` were renamed to `rotate-node`/`scale-node`/`group-nodes`/… instead; the VERB stays the
/// single word, which is a separate descriptor field and is what `APPROVED_VERBS` gates.
pub(super) fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty() && bytes[0].is_ascii_lowercase() && bytes.contains(&b'-') && bytes.split(|byte| *byte == b'-').all(|part| !part.is_empty() && part.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn mutation_leaf_pascal(value: &str) -> bool { let bytes = value.as_bytes(); !bytes.is_empty() && bytes[0].is_ascii_uppercase() && bytes.iter().all(|byte| byte.is_ascii_alphanumeric()) }

fn mutation_leaf_outcomes(value: &serde_json::Value) -> Result<Vec<MutationLeafOutcomeClass>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "outcomeClasses must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    values.iter().map(|value| { let value = mutation_leaf_string(value, "outcomeClasses")?; if !seen.insert(value.clone()) { return Err("outcomeClasses must not contain duplicates".to_string()); } match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "no-op" => Ok(MutationLeafOutcomeClass::NoOp), "empty" => Ok(MutationLeafOutcomeClass::Empty), "disjoint" => Ok(MutationLeafOutcomeClass::Disjoint), "rejected" => Ok(MutationLeafOutcomeClass::Rejected), _ => Err("outcomeClasses contains a non-schema enum value".to_string()) } }).collect()
}

fn mutation_leaf_surfaces(value: &serde_json::Value) -> Result<Vec<MutationLeafLanguageSurface>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "requiredLanguageSurfaces must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    let surfaces: Vec<_> = values.iter().map(|value| { let value = mutation_leaf_string(value, "requiredLanguageSurfaces")?; if !seen.insert(value.clone()) { return Err("requiredLanguageSurfaces must not contain duplicates".to_string()); } match value.as_str() { "rust" => Ok(MutationLeafLanguageSurface::Rust), "typescript" => Ok(MutationLeafLanguageSurface::Typescript), "graphql" => Ok(MutationLeafLanguageSurface::Graphql), "protobuf" => Ok(MutationLeafLanguageSurface::Protobuf), "json-schema" => Ok(MutationLeafLanguageSurface::JsonSchema), "text" => Ok(MutationLeafLanguageSurface::Text), "binary" => Ok(MutationLeafLanguageSurface::Binary), _ => Err("requiredLanguageSurfaces contains a non-schema enum value".to_string()) } }).collect::<Result<_, _>>()?;
    if !surfaces.iter().any(|surface| matches!(surface, MutationLeafLanguageSurface::Rust)) { return Err("requiredLanguageSurfaces must contain rust".to_string()); }
    Ok(surfaces)
}

fn mutation_leaf_reject_duplicate_keys(raw: &[u8]) -> Result<(), String> {
    let mut index = mutation_leaf_skip_ws(raw, 0);
    if raw.get(index) != Some(&b'{') { return Err("mutation descriptor must be a JSON object".to_string()); }
    index += 1;
    let mut keys = HashSet::new();
    loop {
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) == Some(&b'}') { return Ok(()); }
        let key_start = index;
        index = mutation_leaf_string_end(raw, index).ok_or_else(|| "malformed mutation descriptor JSON key".to_string())?;
        let key: String = serde_json::from_slice(&raw[key_start..index]).map_err(|_| "malformed mutation descriptor JSON key".to_string())?;
        if !keys.insert(key) { return Err("mutation descriptor has a duplicate key".to_string()); }
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) != Some(&b':') { return Err("malformed mutation descriptor JSON key separator".to_string()); }
        index = mutation_leaf_json_value_end(raw, mutation_leaf_skip_ws(raw, index + 1)).ok_or_else(|| "malformed mutation descriptor JSON value".to_string())?;
        index = mutation_leaf_skip_ws(raw, index);
        match raw.get(index) { Some(b',') => index += 1, Some(b'}') => return Ok(()), _ => return Err("malformed mutation descriptor JSON object".to_string()) }
    }
}

fn mutation_leaf_skip_ws(raw: &[u8], mut index: usize) -> usize { while raw.get(index).is_some_and(|byte| byte.is_ascii_whitespace()) { index += 1; } index }

fn mutation_leaf_string_end(raw: &[u8], mut index: usize) -> Option<usize> { if raw.get(index) != Some(&b'\"') { return None; } index += 1; while let Some(byte) = raw.get(index) { match byte { b'\"' => return Some(index + 1), b'\\' => index += 2, 0..=0x1f => return None, _ => index += 1 } } None }

fn mutation_leaf_json_value_end(raw: &[u8], index: usize) -> Option<usize> {
    match raw.get(index)? { b'\"' => mutation_leaf_string_end(raw, index), b'{' => mutation_leaf_balanced_end(raw, index, b'{', b'}'), b'[' => mutation_leaf_balanced_end(raw, index, b'[', b']'), _ => { let end = raw[index..].iter().position(|byte| matches!(*byte, b',' | b'}' | b']') || byte.is_ascii_whitespace()).map_or(raw.len(), |offset| index + offset); (end > index).then_some(end) } }
}

fn mutation_leaf_balanced_end(raw: &[u8], mut index: usize, open: u8, close: u8) -> Option<usize> { let mut depth = 0usize; while let Some(byte) = raw.get(index) { if *byte == b'\"' { index = mutation_leaf_string_end(raw, index)?; continue; }
        if *byte == open { depth += 1; } else if *byte == close { depth -= 1; if depth == 0 { return Some(index + 1); } } index += 1; } None }

#[doc(hidden)] pub fn emit_mutation_leaf_descriptor(contract: &syn::Path, descriptor: &MutationLeafJson) -> proc_macro2::TokenStream {
    let schema_version = descriptor.schema_version; let owner = &descriptor.owner; let semantic_kind = &descriptor.semantic_kind; let display_name = &descriptor.display_name; let emoji = &descriptor.emoji; let aggregate_variant = &descriptor.aggregate_variant; let payload_schema = &descriptor.payload_schema;
    let text_opcode = descriptor.text_opcode.as_ref().map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value))); let binary_tag = descriptor.binary_tag.map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value)));
    let invertibility = match &descriptor.invertibility { MutationLeafInvertibility::SelfInvertible => quote!(#contract::MutationInvertibility::SelfInvertible), MutationLeafInvertibility::ExplicitMutation => quote!(#contract::MutationInvertibility::ExplicitMutation), MutationLeafInvertibility::Plan => quote!(#contract::MutationInvertibility::Plan), MutationLeafInvertibility::NonInvertible => quote!(#contract::MutationInvertibility::NonInvertible) };
    let diff_participation = match &descriptor.diff_participation { MutationLeafDiffParticipation::Detect => quote!(#contract::MutationDiffParticipation::Detect), MutationLeafDiffParticipation::ApplyOnly => quote!(#contract::MutationDiffParticipation::ApplyOnly), MutationLeafDiffParticipation::Plan => quote!(#contract::MutationDiffParticipation::Plan), MutationLeafDiffParticipation::None => quote!(#contract::MutationDiffParticipation::None) };
    let outcome_classes = descriptor.outcome_classes.iter().map(|value| match value { MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::NoOp => quote!(#contract::MutationOutcomeClass::NoOp), MutationLeafOutcomeClass::Empty => quote!(#contract::MutationOutcomeClass::Empty), MutationLeafOutcomeClass::Disjoint => quote!(#contract::MutationOutcomeClass::Disjoint), MutationLeafOutcomeClass::Rejected => quote!(#contract::MutationOutcomeClass::Rejected) });
    let composition = match &descriptor.composition { MutationLeafComposition::Atomic => quote!(#contract::MutationComposition::Atomic), MutationLeafComposition::Composite => quote!(#contract::MutationComposition::Composite) };
    let required_language_surfaces = descriptor.required_language_surfaces.iter().map(|value| match value { MutationLeafLanguageSurface::Rust => quote!(#contract::MutationLanguageSurface::Rust), MutationLeafLanguageSurface::Typescript => quote!(#contract::MutationLanguageSurface::Typescript), MutationLeafLanguageSurface::Graphql => quote!(#contract::MutationLanguageSurface::Graphql), MutationLeafLanguageSurface::Protobuf => quote!(#contract::MutationLanguageSurface::Protobuf), MutationLeafLanguageSurface::JsonSchema => quote!(#contract::MutationLanguageSurface::JsonSchema), MutationLeafLanguageSurface::Text => quote!(#contract::MutationLanguageSurface::Text), MutationLeafLanguageSurface::Binary => quote!(#contract::MutationLanguageSurface::Binary) });
    quote!(#contract::MutationLeafDescriptor { schema_version: #schema_version, owner: #owner, semantic_kind: #semantic_kind, display_name: #display_name, emoji: #emoji, aggregate_variant: #aggregate_variant, payload_schema: #payload_schema, text_opcode: #text_opcode, binary_tag: #binary_tag, invertibility: #invertibility, diff_participation: #diff_participation, outcome_classes: &[#(#outcome_classes),*], composition: #composition, required_language_surfaces: &[#(#required_language_surfaces),*] })
}
//#endregion 🔣️MutationLeafJson
