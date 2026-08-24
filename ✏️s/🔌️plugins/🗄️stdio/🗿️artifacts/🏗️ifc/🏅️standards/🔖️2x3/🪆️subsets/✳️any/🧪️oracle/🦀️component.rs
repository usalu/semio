//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! against a real ISO 10303-21 exchange structure parsed by the registered `ruststep` 0.4 reader,
//! then re-serialized by this module's own from-scratch Part-21 writer (ruststep 0.4 has none —
//! confirmed by reading its source, the same finding `📐️step`'s own `🔖️ap214/✳️any` oracle already
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
//! in the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/{📄️document,🖼️raster,🎒️archive,🔊️audio,📊️tabular,
//! 🧊️mesh}` family list, so nothing here is promoted there. `📐️step`'s `🔖️ap214/✳️any` oracle
//! duplicates an equivalent Part-21 writer for the identical reason (no shared family module fits a
//! bare Part-21 text writer, and adding one would mean editing `Cargo.toml`/`📦️lib.rs`, which the
//! fleet brief forbids); this module's writer is independent, not imported from it.
//!
//! ## §6: ruststep is the independent READER, never a second producer
//! Because ruststep cannot write, this module cannot be a genuine differential producer of mutated
//! bytes against a real third-party writer. Every scenario in `../../../../🧪️tests/mutate-ifc-2x3/
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
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`Ifc2x3Mutation::KINDS`).

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
    /// speak (`{"t":"real","v":1.0}`-shaped), the same grammar `step/🔖️ap214/✳️any`'s oracle uses,
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
            Parameter::String(s) => tv("string", Json::String(s.clone())),
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
    /// `step::engine::part21::write_part21_with` (`🚪️io/🦀️component.rs`'s codec): that writer would
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
mod tests {
    use super::{oracle_apply_mutation, project_ifc_2x3_any};
    use semio_repo_test_host::Json;

    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/🏗️wellness-center-sama-street-level.ifc");

    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
    fn num(v: f64) -> Json {
        Json::Number(v)
    }
    fn text(v: &str) -> Json {
        Json::String(v.to_string())
    }
    fn spec(kind: &str, params: Json) -> Json {
        obj(vec![("kind", text(kind)), ("params", params)])
    }
    fn tv(t: &str, v: Json) -> Json {
        obj(vec![("t", text(t)), ("v", v)])
    }
    fn str_arr(values: &[&str]) -> Json {
        Json::Array(values.iter().map(|v| text(v)).collect())
    }

    fn entity_count(projection: &Json) -> f64 {
        match projection.get("entityCount") {
            Some(Json::Number(n)) => *n,
            _ => panic!("projection carries no entityCount: {projection:?}"),
        }
    }
    fn find_entity<'a>(projection: &'a Json, id: f64) -> Option<&'a Json> {
        match projection.get("entities") {
            Some(Json::Array(items)) => items.iter().find(|entity| matches!(entity.get("id"), Some(Json::Number(n)) if *n == id)),
            _ => None,
        }
    }

    #[test]
    fn parses_the_real_fixture_and_projects_it() {
        let projection = project_ifc_2x3_any(FIXTURE).expect("project real fixture");
        assert_eq!(entity_count(&projection), 3464.0);
        match projection.get("fileSchema") {
            Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("IFC2X3".to_string())]),
            other => panic!("expected fileSchema array, got {other:?}"),
        }
        let wall = find_entity(&projection, 270549.0).expect("real wall #270549 present");
        let entities = match wall.get("entities") {
            Some(Json::Array(items)) => items,
            _ => panic!("no entities"),
        };
        assert_eq!(entities.len(), 1, "a simple instance carries exactly one record");
        assert_eq!(entities[0].get("name"), Some(&Json::String("IFCWALLSTANDARDCASE".to_string())));
    }

    #[test]
    fn no_mutation_round_trips_and_is_not_byte_identical() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
        assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
        let projection = project_ifc_2x3_any(&mutated).expect("project no-mutation result");
        assert_eq!(entity_count(&projection), 3464.0);
    }

    #[test]
    fn set_snapshot_extends_file_schema_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", str_arr(&["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"]))]))).expect("set-snapshot");
        let projection = project_ifc_2x3_any(&mutated).expect("project");
        assert_eq!(projection.get("fileSchema"), Some(&str_arr(&["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"])));
        let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", str_arr(&["IFC2X3"]))]))).expect("inverse set-snapshot");
        let restored_projection = project_ifc_2x3_any(&restored).expect("project restored");
        assert_eq!(restored_projection.get("fileSchema"), Some(&str_arr(&["IFC2X3"])));
    }

    fn wellness_header_json(name0: &str) -> Json {
        obj(vec![
            ("fileDescription", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("ViewDefinition [CoordinationView_V2.0]"))])), tv("string", text("2;1"))])),
            (
                "fileName",
                Json::Array(vec![
                    tv("string", text(name0)),
                    tv("string", text("2021-11-21T06:45:25")),
                    tv("aggregate", Json::Array(vec![tv("string", text(""))])),
                    tv("aggregate", Json::Array(vec![tv("string", text(""))])),
                    tv("string", text("The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013")),
                    tv("string", text("21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383")),
                    tv("string", text("")),
                ]),
            ),
            ("fileSchema", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC2X3"))]))])),
        ])
    }

    #[test]
    fn set_header_renames_the_model_and_inverts() {
        let before = project_ifc_2x3_any(FIXTURE).expect("project the real fixture");
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-header", obj(vec![("header", wellness_header_json("wellness-center-sama-street-level-wave8"))]))).expect("set-header");
        let projection = project_ifc_2x3_any(&mutated).expect("project");
        assert_eq!(entity_count(&projection), 3464.0, "set-header must not touch the entity graph");
        assert_ne!(projection, before, "set-header must MOVE the projection -- this assertion is the one that caught the projection being blind to FILE_NAME entirely");
        assert_eq!(
            projection.get("fileName").and_then(|value| value.get("name")).and_then(|value| value.get("v")).cloned(),
            Some(Json::String("wellness-center-sama-street-level-wave8".to_string())),
            "the renamed model must be readable back through the independent parser"
        );
        let restored = oracle_apply_mutation(&mutated, &spec("set-header", obj(vec![("header", wellness_header_json("0001"))]))).expect("inverse set-header");
        let restored_projection = project_ifc_2x3_any(&restored).expect("project restored");
        assert_eq!(restored_projection, project_ifc_2x3_any(FIXTURE).unwrap());
    }

    fn unset() -> Json {
        obj(vec![("t", text("unset"))])
    }
    fn column_args(name: &str) -> Json {
        Json::Array(vec![
            tv("string", text("0PfeWE7Aj7GBHCsLa67379")),
            tv("reference", num(41.0)),
            tv("string", text(name)),
            unset(),
            tv("string", text("UC-Universal Columns-Column:UC305x305x97")),
            tv("reference", num(619886.0)),
            tv("reference", num(619879.0)),
            tv("string", text("552739")),
        ])
    }
    fn column_instance(name: &str) -> Json {
        obj(vec![("id", num(619887.0)), ("entities", Json::Array(vec![obj(vec![("name", text("IFCCOLUMN")), ("args", column_args(name))])]))])
    }

    #[test]
    fn upsert_instance_updates_the_real_column_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("upsert-instance", obj(vec![("instance", column_instance("WAVE8-RENAMED-COLUMN"))]))).expect("upsert-instance");
        let projection = project_ifc_2x3_any(&mutated).expect("project");
        assert_eq!(entity_count(&projection), 3464.0, "updating an existing id must not change the entity count");
        let column = find_entity(&projection, 619887.0).expect("column present");
        let entities = match column.get("entities") {
            Some(Json::Array(items)) => items,
            _ => panic!("no entities"),
        };
        let args = match entities[0].get("args") {
            Some(Json::Array(items)) => items,
            _ => panic!("no args"),
        };
        assert_eq!(args[2], tv("string", text("WAVE8-RENAMED-COLUMN")));

        let restored = oracle_apply_mutation(&mutated, &spec("upsert-instance", obj(vec![("instance", column_instance("UC-Universal Columns-Column:UC305x305x97:552739"))]))).expect("inverse upsert-instance");
        assert_eq!(project_ifc_2x3_any(&restored).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
    }

    fn wall_args() -> Json {
        Json::Array(vec![
            tv("string", text("29w45MKkv9yu3UjOOOyCma")),
            tv("reference", num(41.0)),
            tv("string", text("Basic Wall:Generic - 300mm:471837")),
            unset(),
            tv("string", text("Basic Wall:Generic - 300mm")),
            tv("reference", num(270529.0)),
            tv("reference", num(270547.0)),
            tv("string", text("471837")),
        ])
    }
    fn wall_instance() -> Json {
        obj(vec![("id", num(270549.0)), ("entities", Json::Array(vec![obj(vec![("name", text("IFCWALLSTANDARDCASE")), ("args", wall_args())])]))])
    }

    /// 🧪️ The deliberate real-reference-removal case the fleet brief asks for: `#270549` is a real
    /// `IFCWALLSTANDARDCASE` referenced by 8 real entities in the source document (7 of which — 5
    /// property-set relationships, 1 material association, 1 type-definition relationship, plus the
    /// storey's own spatial-containment relationship — are carried into this fixture's own
    /// forward-reference closure). `remove-instance` is mechanical (matching production
    /// `Ifc2x3Mutation::RemoveInstance`'s own bare `retain`, no cascading integrity check), so the
    /// result genuinely contains a dangling `#270549` reference inside those real relationship
    /// entities afterward — documented here, not hidden.
    #[test]
    fn remove_instance_deletes_a_referenced_real_wall_and_leaves_a_documented_dangling_reference() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("remove-instance", obj(vec![("id", num(270549.0))]))).expect("remove-instance");
        let projection = project_ifc_2x3_any(&mutated).expect("project removed");
        assert_eq!(entity_count(&projection), 3463.0);
        assert!(find_entity(&projection, 270549.0).is_none());
        // 🕳️ The dangling reference: the spatial-containment relationship still lists #270549.
        let containment = find_entity(&projection, 710858.0).expect("containment relationship still present");
        let containment_args = match containment.get("entities") {
            Some(Json::Array(items)) => match items[0].get("args") {
                Some(Json::Array(a)) => a.clone(),
                _ => panic!("no args"),
            },
            _ => panic!("no entities"),
        };
        let related_elements = match &containment_args[4] {
            Json::Object(_) => containment_args[4].get("v").cloned(),
            _ => None,
        };
        let still_dangling = matches!(related_elements, Some(Json::Array(items)) if items.iter().any(|item| matches!(item, Json::Object(_)) && item.get("v") == Some(&Json::Number(270549.0))));
        assert!(still_dangling, "removing a referenced instance must leave the real relationship's reference dangling, not silently repair it");

        let reinserted = oracle_apply_mutation(&mutated, &spec("upsert-instance", obj(vec![("instance", wall_instance())]))).expect("inverse remove-instance (cross-kind upsert-instance)");
        assert_eq!(project_ifc_2x3_any(&reinserted).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
    }

    #[test]
    fn upsert_instance_appends_a_brand_new_id() {
        let inserted = oracle_apply_mutation(
            FIXTURE,
            &spec(
                "upsert-instance",
                obj(vec![(
                    "instance",
                    obj(vec![
                        ("id", num(9_000_001.0)),
                        ("entities", Json::Array(vec![obj(vec![("name", text("IFCCARTESIANPOINT")), ("args", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("real", num(1.0)), tv("real", num(2.0)), tv("real", num(3.0))]))]))])])),
                    ]),
                )]),
            ),
        )
        .expect("upsert-instance append");
        let projection = project_ifc_2x3_any(&inserted).expect("project inserted");
        assert_eq!(entity_count(&projection), 3465.0);
        assert!(find_entity(&projection, 9_000_001.0).is_some());
        let removed = oracle_apply_mutation(&inserted, &spec("remove-instance", obj(vec![("id", num(9_000_001.0))]))).expect("remove the appended instance");
        assert_eq!(project_ifc_2x3_any(&removed).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
    }

    #[test]
    fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
        let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
        assert_ne!(output, FIXTURE);
        let input_projection = project_ifc_2x3_any(FIXTURE).unwrap();
        let output_projection = project_ifc_2x3_any(&output).unwrap();
        assert_eq!(input_projection, output_projection);
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
    }

    #[test]
    fn remove_instance_of_absent_id_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("remove-instance", obj(vec![("id", num(999_999_999.0))]))).is_err());
    }
}
//#endregion 🧪️Tests
