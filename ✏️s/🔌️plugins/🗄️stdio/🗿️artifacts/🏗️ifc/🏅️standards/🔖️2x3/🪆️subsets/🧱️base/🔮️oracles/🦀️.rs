//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! against a real ISO 10303-21 exchange structure parsed by the registered `ruststep` 0.4 reader,
//! then re-serialized by this module's own from-scratch Part-21 writer (ruststep 0.4 has none —
//! confirmed by reading its source, the same finding `📐️step`'s own `🔖️ap214/🧱️base` oracle already
//! made: `ast::ser::to_record` only builds an in-memory `Record` from an already-typed struct, and
//! grepping the crate for `Display`/`fmt::Formatter` impls on `Exchange`/`DataSection`/`Record`/
//! `Parameter` finds none).
//!
//! 🏗️ IFC2X3 is physically ISO 10303-21 (STEP physical file) syntax under a different EXPRESS
//! schema — `ruststep` parses it exactly as it parses STEP AP214; that is the whole premise of
//! this subset reusing the same reference library.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! a shared family module rather than by copying it — this subset has no such sibling registered
//! in the shared `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/{📄️document,🖼️raster,🎒️archive,🔊️audio,📊️tabular,
//! 🧊️mesh}` family list, so nothing here is promoted there. `📐️step`'s `🔖️ap214/🧱️base` oracle
//! duplicates an equivalent Part-21 writer for the identical reason (no shared family module fits a
//! bare Part-21 text writer, and adding one would mean editing `Cargo.toml`/`📦️lib.rs`, which the
//! fleet brief forbids); this module's writer is independent, not imported from it.
//!
//! ## §6: ruststep is the independent READER, never a second producer
//! Because ruststep cannot write, this module cannot be a genuine differential producer of mutated
//! bytes against a real third-party writer. Every scenario in `../../../../🧪️tests/🧱️mutate-ifc-2x3/
//! component.feature` is therefore typed `@mode-property`/`@mode-round-trip`, never
//! `@mode-differential` — the fleet brief's §6 situation, confirmed empirically (not assumed): a
//! standalone probe fed ruststep this subset's own real derived fixture and it parsed all 3464 real
//! entities with zero errors, which is what justifies registering it as the real reader below
//! rather than skipping an oracle entirely. `ruststep::ast::Exchange::from_str` is what actually
//! reads both the real input and every re-serialized result — including this dispatcher's own
//! mutation output and (once the subject phase compiles) the subject's — through
//! `project_ifc_2x3_any` below, which is the one place a genuinely independent, third-party parse
//! of the result happens.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`Ifc2x3Mutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️JsonHelpers
#[cfg(feature = "oracles")]
fn num_field(value: &Json, key: &str) -> Result<f64, String> {
    match value.get(key) {
        Some(Json::Number(number)) => Ok(*number),
        _ => Err(format!("expected numeric field {key:?}")),
    }
}
#[cfg(feature = "oracles")]
fn str_field(value: &Json, key: &str) -> Result<String, String> {
    match value.get(key) {
        Some(Json::String(text)) => Ok(text.clone()),
        _ => Err(format!("expected string field {key:?}")),
    }
}
#[cfg(feature = "oracles")]
fn str_array(value: &Json, key: &str) -> Vec<String> {
    value
        .array(key)
        .iter()
        .filter_map(|entry| match entry {
            Json::String(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}
#[cfg(feature = "oracles")]
fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
    num_field(value, key).map(|number| number as u64)
}
//#endregion 🔖️JsonHelpers

#[cfg(feature = "oracles")]
mod oracles {
    use super::{num_field, str_array, str_field, u64_field, Json};
    use ruststep::ast::{DataSection, EntityInstance, Exchange, Name, Parameter, Record};
    use std::str::FromStr;

    //#region 🔖️ValueGrammar
    /// 🔤️ This module's own JSON wire grammar for one Part-21 argument value — the wire shape the
    /// feature file's `Examples` tables and this subset's subject-side `mutation_from_spec` both
    /// speak (`{"t":"real","v":1.0}`-shaped), the same grammar `step/🔖️ap214/🧱️base`'s oracle uses,
    /// independent of `Part21Value`'s own serde tagging.
    fn value_from_json(value: &Json) -> Result<Parameter, String> {
        match str_field(value, "t")?.as_str() {
            "unset" => Ok(Parameter::NotProvided),
            "derived" => Ok(Parameter::Omitted),
            "integer" => Ok(Parameter::Integer(num_field(value, "v")? as i64)),
            "real" => Ok(Parameter::Real(num_field(value, "v")?)),
            "string" => Ok(Parameter::String(str_field(value, "v")?)),
            "enum" => Ok(Parameter::Enumeration(str_field(value, "v")?)),
            "reference" => Ok(Parameter::Ref(Name::Entity(u64_field(value, "v")?))),
            "aggregate" => Ok(Parameter::List(value.array("v").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?)),
            "typed" => Ok(Parameter::Typed { keyword: str_field(value, "name")?, parameter: Box::new(value_from_json(value.get("v").ok_or("typed value requires a v field")?)?) }),
            other => Err(format!("unknown value type {other:?}")),
        }
    }

    /// 🔤️ The inverse projection: an independently-parsed `Parameter` back into this module's own
    /// canonical JSON shape — used both to echo a real argument back out in `project_ifc_2x3_any`
    /// and, transitively, inside `aggregate`'s recursion.
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
            Parameter::String(s) => tv("string", match crate::artifacts::step::standards::v_ap214::reference::part21::decode_string_literal(s) {
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

    //#region 🔖️Writer
    /// 📤️ From-scratch Part-21 clear-text writer (ruststep 0.4 has none) — one line per header
    /// record, one line per entity instance. Independent of this subset's own production
    /// `step::engine::part21::write_part21_with` (`🚪️io/🦀️.rs`'s codec): that writer would
    /// make the oracle compare this repository's implementation against itself, the exact failure
    /// mode this platform exists to prevent.
    fn write_param(param: &Parameter, out: &mut String) {
        match param {
            Parameter::Typed { keyword, parameter } => {
                out.push_str(keyword);
                out.push('(');
                write_param(parameter, out);
                out.push(')');
            }
            Parameter::Integer(i) => out.push_str(&i.to_string()),
            Parameter::Real(r) => {
                if r.fract() == 0.0 && r.is_finite() {
                    out.push_str(&format!("{r:.0}."));
                } else {
                    out.push_str(&format!("{r}"));
                }
            }
            Parameter::String(s) => {
                out.push('\'');
                out.push_str(&s.replace('\'', "''"));
                out.push('\'');
            }
            Parameter::Enumeration(s) => {
                out.push('.');
                out.push_str(s);
                out.push('.');
            }
            Parameter::List(items) => {
                out.push('(');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_param(item, out);
                }
                out.push(')');
            }
            Parameter::Ref(name) => match name {
                Name::Entity(id) => out.push_str(&format!("#{id}")),
                Name::Value(id) => out.push_str(&format!("@{id}")),
                Name::ConstantEntity(s) => out.push_str(&format!("#{s}")),
                Name::ConstantValue(s) => out.push_str(&format!("@{s}")),
            },
            Parameter::NotProvided => out.push('$'),
            Parameter::Omitted => out.push('*'),
        }
    }

    fn write_record(record: &Record, out: &mut String) {
        out.push_str(&record.name);
        write_param(&record.parameter, out);
    }

    fn write_entity(entity: &EntityInstance, out: &mut String) {
        match entity {
            EntityInstance::Simple { id, record } => {
                out.push('#');
                out.push_str(&id.to_string());
                out.push('=');
                write_record(record, out);
                out.push_str(";\n");
            }
            EntityInstance::Complex { id, subsuper } => {
                out.push('#');
                out.push_str(&id.to_string());
                out.push_str("=(");
                for record in &subsuper.0 {
                    write_record(record, out);
                }
                out.push_str(");\n");
            }
        }
    }

    fn write_exchange(exchange: &Exchange) -> String {
        let mut out = String::new();
        out.push_str("ISO-10303-21;\nHEADER;\n");
        for record in &exchange.header {
            write_record(record, &mut out);
            out.push_str(";\n");
        }
        out.push_str("ENDSEC;\nDATA;\n");
        for section in &exchange.data {
            for entity in &section.entities {
                write_entity(entity, &mut out);
            }
        }
        out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        out
    }
    //#endregion 🔖️Writer

    //#region 🔖️EntityAccess
    fn entity_id(entity: &EntityInstance) -> u64 {
        match entity {
            EntityInstance::Simple { id, .. } => *id,
            EntityInstance::Complex { id, .. } => *id,
        }
    }
    fn header_record<'e>(exchange: &'e Exchange, name: &str) -> Option<&'e Record> {
        exchange.header.iter().find(|record| record.name == name)
    }
    fn header_record_mut<'e>(exchange: &'e mut Exchange, name: &str) -> Option<&'e mut Record> {
        exchange.header.iter_mut().find(|record| record.name == name)
    }
    fn records(entity: &EntityInstance) -> Vec<&Record> {
        match entity {
            EntityInstance::Simple { record, .. } => vec![record],
            EntityInstance::Complex { subsuper, .. } => subsuper.0.iter().collect(),
        }
    }
    //#endregion 🔖️EntityAccess

    //#region 🔖️Apply
    fn string_list_param(values: &[String]) -> Parameter {
        Parameter::List(values.iter().cloned().map(Parameter::String).collect())
    }

    /// 🧩️ Builds one `EntityInstance` (simple or complex) from this module's own wire shape:
    /// `{"id": u64, "entities": [{"name": str, "args": [value...]}, ...]}` — mirrors
    /// `Part21Instance{id, entities: Vec<(String, Vec<Part21Value>)>}` exactly (this subset's own
    /// per-instance vocabulary carries a WHOLE instance, simple or complex, never a single arg).
    fn instance_from_json(value: &Json) -> Result<EntityInstance, String> {
        let id = u64_field(value, "id")?;
        let entities = value.array("entities");
        if entities.is_empty() {
            return Err("instance requires a non-empty entities array".to_string());
        }
        let records = entities
            .iter()
            .map(|entry| -> Result<Record, String> {
                let name = str_field(entry, "name")?;
                let args = entry.array("args").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                Ok(Record { name, parameter: Parameter::List(args) })
            })
            .collect::<Result<Vec<_>, String>>()?;
        if records.len() == 1 {
            Ok(EntityInstance::Simple { id, record: records.into_iter().next().expect("checked len == 1") })
        } else {
            Ok(EntityInstance::Complex { id, subsuper: ruststep::ast::SubSuperRecord(records) })
        }
    }

    /// 🦠️ Applies one declared `Ifc2x3Mutation::KINDS` kind to a real, independently-parsed
    /// `ruststep::ast::Exchange` — one arm per variant, matched by its kebab-case spelling. An
    /// unrecognised kind is an error, never a silent no-op.
    ///
    /// `upsert-instance`/`remove-instance` are this subset's OWN vocabulary (richer than `4`'s
    /// `{NoMutation, SetSnapshot}` stub) and operate on the real entity graph exactly like
    /// `Ifc2x3Mutation::{UpsertInstance,RemoveInstance}` do in production: upsert replaces an
    /// existing id's whole instance or appends a brand-new one at the end (never a positional
    /// insert), remove deletes an id with NO cascading reference-integrity check — mechanical,
    /// matching production's own bare `retain`. See the feature file's own description for the
    /// deliberate real-reference-removal case this exercises (`remove-instance` on `#270549`, a
    /// real wall referenced by 8 other real entities in the source, 7 of which are carried into
    /// this fixture's own forward-reference closure).
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),

            "set-snapshot" => {
                let schemas = str_array(params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
                record.parameter = Parameter::List(vec![string_list_param(&schemas)]);
                Ok(())
            }

            "set-header" => {
                let header = params.get("header").ok_or("set-header requires a header field")?;
                for (record_name, field) in [("FILE_DESCRIPTION", "fileDescription"), ("FILE_NAME", "fileName"), ("FILE_SCHEMA", "fileSchema")] {
                    let values = header.array(field).iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                    let record = header_record_mut(exchange, record_name).ok_or_else(|| format!("input carries no {record_name} header record"))?;
                    record.parameter = Parameter::List(values);
                }
                Ok(())
            }

            "upsert-instance" => {
                let instance_json = params.get("instance").ok_or("upsert-instance requires an instance field")?;
                let id = u64_field(instance_json, "id")?;
                let instance = instance_from_json(instance_json)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                match section.entities.iter_mut().find(|entity| entity_id(entity) == id) {
                    Some(existing) => *existing = instance,
                    None => section.entities.push(instance),
                }
                Ok(())
            }

            "remove-instance" => {
                let id = u64_field(params, "id")?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let before = section.entities.len();
                section.entities.retain(|entity| entity_id(entity) != id);
                if section.entities.len() == before {
                    return Err(format!("remove-instance: no instance with id {id}"));
                }
                Ok(())
            }

            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Apply

    //#region 🔖️Dispatch
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let mut exchange = Exchange::from_str(text).map_err(|error| format!("ruststep could not parse the input: {error}"))?;
        if exchange.data.is_empty() {
            exchange.data.push(DataSection { meta: Vec::new(), entities: Vec::new() });
        }
        apply(&mut exchange, kind, params)?;
        Ok(write_exchange(&exchange).into_bytes())
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
    fn entity_to_json(entity: &EntityInstance) -> Json {
        let entities = records(entity)
            .into_iter()
            .map(|record| {
                Json::Object(vec![
                    ("name".to_string(), Json::String(record.name.clone())),
                    (
                        "args".to_string(),
                        match &record.parameter {
                            Parameter::List(items) => Json::Array(items.iter().map(value_to_json).collect()),
                            other => Json::Array(vec![value_to_json(other)]),
                        },
                    ),
                ])
            })
            .collect();
        Json::Object(vec![("id".to_string(), Json::Number(entity_id(entity) as f64)), ("entities".to_string(), Json::Array(entities))])
    }

    fn json_number(entry: &Json, key: &str) -> f64 {
        match entry.get(key) {
            Some(Json::Number(value)) => *value,
            _ => 0.0,
        }
    }

    /// 👁️ This subset's own semantic projection — the ONLY place a real, independent third-party
    /// parse (ruststep, never this subset's own `step::engine::part21`) reads back a result before
    /// `semantic-ifc-v1` compares it: `FILE_SCHEMA` plus the full id-keyed entity graph (every
    /// entity's name(s) and positional arguments, complex instances kept as a real multi-entity
    /// list), id-sorted for a deterministic comparison regardless of physical order.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("projection input is not UTF-8: {error}"))?;
        let exchange = Exchange::from_str(text).map_err(|error| format!("ruststep could not independently parse the result: {error}"))?;
        let file_schema = header_record(&exchange, "FILE_SCHEMA")
            .map(|record| match &record.parameter {
                Parameter::List(items) => items
                    .iter()
                    .filter_map(|item| match item {
                        Parameter::List(inner) => Some(inner.iter().filter_map(|v| match v {
                            Parameter::String(s) => Some(s.clone()),
                            _ => None,
                        })),
                        _ => None,
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .unwrap_or_default();
        let mut entities: Vec<Json> = Vec::new();
        for section in &exchange.data {
            for entity in &section.entities {
                entities.push(entity_to_json(entity));
            }
        }
        entities.sort_by(|a, b| json_number(a, "id").partial_cmp(&json_number(b, "id")).unwrap_or(std::cmp::Ordering::Equal));
        Ok(Json::Object(vec![
            ("fileSchema".to_string(), Json::Array(file_schema.into_iter().map(Json::String).collect())),
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

/// 👁️ This subset's own semantic projection, re-exported at the module's public surface so the
/// case adapter can reach it as `oracle_apply_mutation`'s sibling.
#[cfg(feature = "oracles")]
pub fn project_ifc_2x3_any(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ifc_2x3_any(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Ticket 26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR wave validation: exercises every declared
/// kind against the real derived fixture, confirming the exact ids/values the feature file's
/// `Examples` tables carry are real. `cargo test --features oracles` from this crate's own directory.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
