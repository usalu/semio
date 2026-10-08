//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! against a real ISO 10303-21 exchange structure parsed by the registered `ruststep` 0.4 reader,
//! then re-serialized by this module's own from-scratch Part-21 writer (ruststep 0.4 has none —
//! confirmed by reading its source: `ast::ser::to_record` only builds an in-memory `Record` from an
//! already-typed struct, and grepping the crate for `Display`/`fmt::Formatter` impls on
//! `Exchange`/`DataSection`/`Record`/`Parameter` finds none).
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! a shared family module rather than by copying it — this subset has no such sibling yet, so
//! nothing here is promoted to `✏️s/🔌️plugins/🗄️stdio/🔮️oracles`.
//!
//! ## §6: ruststep is the independent READER, never a second producer
//! Because ruststep cannot write, this module cannot be a genuine differential producer of mutated
//! bytes against a real third-party writer. Every scenario in `../../../../🧪️tests/📐️mutate-step-ap214/
//! component.feature` is therefore typed `@mode-property`/`@mode-round-trip`, never
//! `@mode-differential` — the fleet brief's §6 situation, confirmed empirically (not assumed): a
//! standalone probe (this ticket's scratch folder) fed ruststep this subset's own real derived
//! fixture and it parsed all 1396 real entities with zero errors, which is what justifies
//! registering it as the real reader below rather than skipping an oracle entirely. `ruststep::ast::
//! Exchange::from_str` is what actually reads both the real input and every re-serialized result —
//! including this dispatcher's own mutation output and (once the subject phase compiles) the
//! subject's — through `project_step_ap214_any` below, which is the one place a genuinely
//! independent, third-party parse of the result happens.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`StepMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
mod oracles {
    use ruststep::ast::{DataSection, EntityInstance, Exchange, Name, Parameter};
    use semio_repo_test_host::Json;
    use std::str::FromStr;

    //#region 🔖️ValueGrammar
    /// 🔤️ An independently-parsed `Parameter` in this module's own canonical projection shape — the argument form
    /// `project_step_ap214_any` echoes back, recursively for aggregates.
    fn value_to_json(param: &Parameter) -> Json {
        let tv = |t: &str, v: Json| Json::Object(vec![("t".to_string(), Json::String(t.to_string())), ("v".to_string(), v)]);
        match param {
            Parameter::NotProvided => Json::Object(vec![("t".to_string(), Json::String("unset".to_string()))]),
            Parameter::Omitted => Json::Object(vec![("t".to_string(), Json::String("derived".to_string()))]),
            Parameter::Integer(i) => tv("integer", Json::Number(*i as f64)),
            Parameter::Real(r) => tv("real", Json::Number(*r)),
            // 🔤️ A Part-21 string ARGUMENT is the value the literal DENOTES, not the literal.
            // `ruststep`'s `string` combinator is `many0(none_of("'"))` — it decodes no control
            // directive at all — so passing its text straight through compared ENCODINGS and made
            // two conformant writers that spell one character differently diverge for no semantic
            // reason. Decoded here through the shared oracle's OWN from-scratch reader, never
            // through the production codec this projection is evidence about.
            Parameter::String(s) => tv("string", match decode_string_literal(s) {
                Ok(value) => Json::String(value),
                Err(error) => Json::Object(vec![("undecodableStringLiteral".to_string(), Json::String(error))]),
            }),
            Parameter::Enumeration(s) => tv("enum", Json::String(s.clone())),
            Parameter::List(items) => tv("aggregate", Json::Array(items.iter().map(value_to_json).collect())),
            Parameter::Ref(name) => tv("reference", Json::Number(name_id(name) as f64)),
            Parameter::Typed { keyword, parameter } => Json::Object(vec![("t".to_string(), Json::String("typed".to_string())), ("name".to_string(), Json::String(keyword.clone())), ("v".to_string(), value_to_json(parameter))]),
        }
    }

    fn name_id(name: &Name) -> u64 {
        match name {
            Name::Entity(id) | Name::Value(id) => *id,
            Name::ConstantEntity(_) | Name::ConstantValue(_) => 0,
        }
    }
    //#endregion 🔖️ValueGrammar

    //#region 🔖️SharedCodec
    /// 📤️ The from-scratch Part-21 writer and the instance/record accessors are NOT declared here.
    /// They live once, at the standard level (`../../../🦀️oracle.rs`), because all seven
    /// `ap214` subsets genuinely share them — this subset writes ISO 10303-21 clear text exactly as
    /// the six `✳️ccN` conformance-class subsets do, and a second copy in this file would be the
    /// duplication the family-module rule exists to prevent. What stays here is what is genuinely
    /// this subset's own: the eleven-verb Part-21 GRAMMAR vocabulary and its projection.
    use semio_s_plugin_stdio_part21_test_oracle::decode_string_literal;
    use crate::standards::v_ap214::reference::part21::{
        args, args_mut, entity_from_wire, entity_id, file_description_record, file_name_record, file_schema_record, header_record, primary_record, primary_record_mut, set_header_record, str_field, u64_field, value_from_wire, write as write_exchange_bytes,
    };

    fn write_exchange(exchange: &Exchange) -> String {
        String::from_utf8_lossy(&write_exchange_bytes(exchange)).to_string()
    }
    //#endregion 🔖️SharedCodec

    //#region 🔖️Apply
    /// 🦠️ Applies one declared `StepMutation::KINDS` kind to a real, independently-parsed
    /// `ruststep::ast::Exchange` — one arm per variant, matched by its kebab-case spelling, each reading the leaf
    /// wire payload through the standard's own `🧾️Wire` grammar. An unrecognised kind is an error, never a
    /// silent no-op.
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        let member = |key: &str| params.get(key).ok_or_else(|| format!("{kind} carries `{key}`"));
        let index = |key: &str| u64_field(params, key).map(|index| index as usize);
        match kind {
            "set-file-description" => file_description_record(member("fileDescription")?).map(|record| set_header_record(exchange, record)),
            "set-file-name" => file_name_record(member("fileName")?).map(|record| set_header_record(exchange, record)),
            "set-file-schema" => file_schema_record(member("fileSchema")?).map(|record| set_header_record(exchange, record)),

            "insert-entity" => {
                let index = index("index")?;
                let entity = entity_from_wire(member("entity")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let clamped = index.min(section.entities.len());
                section.entities.insert(clamped, entity);
                Ok(())
            }

            "remove-entity" => {
                let id = u64_field(params, "id")?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let before = section.entities.len();
                section.entities.retain(|entity| entity_id(entity) != id);
                if section.entities.len() == before {
                    return Err(format!("remove-entity: no entity with id {id}"));
                }
                Ok(())
            }

            "set-entity-name" => {
                let id = u64_field(params, "id")?;
                let name = str_field(params, "name")?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("set-entity-name: no entity with id {id}"))?;
                primary_record_mut(entity).name = name;
                Ok(())
            }

            "set-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = index("argIndex")?;
                let value = value_from_wire(member("value")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("set-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                *args.get_mut(arg_index).ok_or_else(|| format!("set-entity-arg: arg index {arg_index} out of range for entity {id}"))? = value;
                Ok(())
            }

            "insert-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = index("argIndex")?;
                let value = value_from_wire(member("value")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("insert-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                let clamped = arg_index.min(args.len());
                args.insert(clamped, value);
                Ok(())
            }

            "remove-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = index("argIndex")?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("remove-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                if arg_index >= args.len() {
                    return Err(format!("remove-entity-arg: arg index {arg_index} out of range for entity {id}"));
                }
                args.remove(arg_index);
                Ok(())
            }

            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Apply

    //#region 🔖️Dispatch
    fn read(input: &[u8]) -> Result<Exchange, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let mut exchange = Exchange::from_str(text).map_err(|error| format!("ruststep could not parse the input: {error}"))?;
        if exchange.data.is_empty() {
            exchange.data.push(DataSection { meta: Vec::new(), entities: Vec::new() });
        }
        Ok(exchange)
    }

    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut exchange = read(input)?;
        apply(&mut exchange, kind, params)?;
        Ok(write_exchange(&exchange).into_bytes())
    }

    /// 🔁️ Decode and re-encode with no mutation: `ruststep`'s parse, the standard's own writer.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(write_exchange(&read(input)?).into_bytes())
    }

    //#endregion 🔖️Dispatch

    //#region 🔖️HeaderProjection
    /// 📇️ The seven attributes ISO 10303-21 §8.2.3 fixes for `FILE_NAME`, in its own order.
    const FILE_NAME_ATTRIBUTES: &[&str] = &["name", "timestamp", "author", "organization", "preprocessorVersion", "originatingSystem", "authorization"];
    /// 📇️ The two attributes ISO 10303-21 §8.2.2 fixes for `FILE_DESCRIPTION`.
    const FILE_DESCRIPTION_ATTRIBUTES: &[&str] = &["description", "implementationLevel"];

    /// 👁️ One header record projected under the attribute NAMES the standard fixes for it, rather
    /// than as a positional array.
    ///
    /// ⚠️ This exists because of a real defect the observability law caught: the projection used to
    /// report `FILE_SCHEMA` and the entity graph and NOTHING ELSE, so every mutation kind that edits
    /// `FILE_DESCRIPTION` or `FILE_NAME` — kinds this subset declares by name — was invisible to it.
    /// Those scenarios passed because the reference library did not error, not because anything was
    /// checked. Naming the attributes rather than indexing them is what lets a comparison profile's
    /// writer-freedom list (`timestamp`, `preprocessorVersion`, `originatingSystem`,
    /// `authorization`) actually address the header; against a positional array that declaration
    /// would silently stop applying.
    fn header_object(exchange: &Exchange, record_name: &str, attributes: &[&str]) -> Json {
        let arguments = header_record(exchange, record_name)
            .and_then(|record| match &record.parameter {
                Parameter::List(items) => Some(items.clone()),
                _ => None,
            })
            .unwrap_or_default();
        Json::Object(attributes.iter().enumerate().map(|(index, name)| ((*name).to_string(), arguments.get(index).map(value_to_json).unwrap_or(Json::Null))).collect())
    }
    //#endregion 🔖️HeaderProjection

    //#region 🔖️Projection
    fn entity_to_json(entity: &EntityInstance) -> Result<Json, String> {
        let record = primary_record(entity);
        let args = args(record)?.iter().map(value_to_json).collect();
        Ok(Json::Object(vec![("id".to_string(), Json::Number(entity_id(entity) as f64)), ("name".to_string(), Json::String(record.name.clone())), ("args".to_string(), Json::Array(args))]))
    }

    fn json_number(entry: &Json, key: &str) -> f64 {
        match entry.get(key) {
            Some(Json::Number(value)) => *value,
            _ => 0.0,
        }
    }

    /// 👁️ This subset's own semantic projection — the ONLY place a real, independent third-party
    /// parse (ruststep, never this subset's own `engine::part21`) reads back a result before
    /// `semantic-step-v1` compares it: `FILE_SCHEMA` plus the full id-keyed entity graph (name,
    /// positional arguments), id-sorted for a deterministic comparison regardless of physical order.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("projection input is not UTF-8: {error}"))?;
        let exchange = Exchange::from_str(text).map_err(|error| format!("ruststep could not independently parse the result: {error}"))?;
        let file_schema = header_record(&exchange, "FILE_SCHEMA").map(args).transpose()?.and_then(|params| params.first()).map(|param| match param {
            Parameter::List(items) => items
                .iter()
                .filter_map(|item| match item {
                    Parameter::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        });
        let mut entities: Vec<Json> = Vec::new();
        for section in &exchange.data {
            for entity in &section.entities {
                entities.push(entity_to_json(entity)?);
            }
        }
        entities.sort_by(|a, b| json_number(a, "id").partial_cmp(&json_number(b, "id")).unwrap_or(std::cmp::Ordering::Equal));
        Ok(Json::Object(vec![
            ("fileSchema".to_string(), Json::Array(file_schema.unwrap_or_default().into_iter().map(Json::String).collect())),
            ("fileDescription".to_string(), header_object(&exchange, "FILE_DESCRIPTION", FILE_DESCRIPTION_ATTRIBUTES)),
            ("fileName".to_string(), header_object(&exchange, "FILE_NAME", FILE_NAME_ATTRIBUTES)),
            ("entityCount".to_string(), Json::Number(entities.len() as f64)),
            ("entities".to_string(), Json::Array(entities)),
        ]))
    }
    //#endregion 🔖️Projection
}

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let empty_params = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty_params);
    oracles::apply_mutation(input, &kind, params)
}

/// 🔁️ Decodes and re-encodes the artifact with no mutation — the identity cycle every law's baseline runs.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's own semantic projection, re-exported at the module's public surface so the
/// case adapter can reach it as `oracle_apply_mutation`'s sibling.
#[cfg(feature = "oracles")]
pub fn project_step_ap214_any(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_step_ap214_any(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Ticket 26/08/23/END-TO-END-TESTING-REFACTOR validation: exercises every declared kind against
/// the real derived fixture, confirming the exact ids/indices/values the feature file's `Examples`
/// tables carry are real. `cargo test --features oracles` from this crate's own directory.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
