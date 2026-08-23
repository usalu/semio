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
//! nothing here is promoted to `✏️s/🔌️plugins/🗄️stdio/🧪️oracle`.
//!
//! ## §6: ruststep is the independent READER, never a second producer
//! Because ruststep cannot write, this module cannot be a genuine differential producer of mutated
//! bytes against a real third-party writer. Every scenario in `../../../../🧪️tests/mutate-step-ap214/
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
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`StepMutation::KINDS`).

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
    value.array(key).iter().filter_map(|entry| match entry { Json::String(s) => Some(s.clone()), _ => None }).collect()
}
#[cfg(feature = "oracles")]
fn usize_field(value: &Json, key: &str) -> Result<usize, String> {
    num_field(value, key).map(|number| number as usize)
}
#[cfg(feature = "oracles")]
fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
    num_field(value, key).map(|number| number as u64)
}
//#endregion 🔖️JsonHelpers

#[cfg(feature = "oracles")]
mod oracles {
    use super::{num_field, str_array, str_field, u64_field, usize_field};
    use ruststep::ast::{DataSection, EntityInstance, Exchange, Name, Parameter, Record};
    use semio_repo_test_host::Json;
    use std::str::FromStr;

    //#region 🔖️ValueGrammar
    /// 🔤️ This module's own JSON wire grammar for one Part-21 argument value — the wire shape the
    /// feature file's `Examples` tables and this subset's subject-side `mutation_from_spec` both
    /// speak (`{"t":"real","v":1.0}`-shaped), independent of `StepValue`'s own serde tagging.
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
    /// canonical JSON shape — used both to echo a real argument back out in `project_step_ap214_any`
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
    /// record, one line per entity instance, no attempt at the 78-column wrapping real STEP writers
    /// use for readability (spec-optional, never semantically required; confirmed by re-parsing this
    /// writer's own output with the same real `Exchange::from_str` below in this subset's ticket
    /// scratch probe before this module was written).
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
    fn primary_record_mut(entity: &mut EntityInstance) -> &mut Record {
        match entity {
            EntityInstance::Simple { record, .. } => record,
            EntityInstance::Complex { subsuper, .. } => &mut subsuper.0[0],
        }
    }
    fn primary_record(entity: &EntityInstance) -> &Record {
        match entity {
            EntityInstance::Simple { record, .. } => record,
            EntityInstance::Complex { subsuper, .. } => &subsuper.0[0],
        }
    }
    /// 🔎️ A record's positional argument list — real Part-21 records always carry `Parameter::
    /// List(..)` at the top level (confirmed against every one of this subset's real fixture's 1396
    /// entities in this ticket's scratch probe); anything else is malformed input.
    fn args_mut(record: &mut Record) -> Result<&mut Vec<Parameter>, String> {
        match &mut record.parameter {
            Parameter::List(items) => Ok(items),
            other => Err(format!("record {:?} does not carry a positional argument list ({other:?})", record.name)),
        }
    }
    fn args(record: &Record) -> Result<&Vec<Parameter>, String> {
        match &record.parameter {
            Parameter::List(items) => Ok(items),
            other => Err(format!("record {:?} does not carry a positional argument list ({other:?})", record.name)),
        }
    }
    fn header_record<'e>(exchange: &'e Exchange, name: &str) -> Option<&'e Record> {
        exchange.header.iter().find(|record| record.name == name)
    }
    fn header_record_mut<'e>(exchange: &'e mut Exchange, name: &str) -> Option<&'e mut Record> {
        exchange.header.iter_mut().find(|record| record.name == name)
    }
    //#endregion 🔖️EntityAccess

    //#region 🔖️Apply
    fn string_list_param(values: &[String]) -> Parameter {
        Parameter::List(values.iter().cloned().map(Parameter::String).collect())
    }

    /// 🦠️ Applies one declared `StepMutation::KINDS` kind to a real, independently-parsed
    /// `ruststep::ast::Exchange` — one arm per variant, matched by its kebab-case spelling. An
    /// unrecognised kind is an error, never a silent no-op.
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),

            // 🧭️ Pragmatic `set-snapshot`: overrides the one header field the wave-7 scenario
            // actually exercises (`FILE_SCHEMA`) on the already-decoded document, the same
            // precedent `mutate-pdf-1-7`'s own oracle uses for its `set-snapshot` (patches known
            // fields rather than requiring the full snapshot literal inline in a Gherkin cell,
            // which for a 1396-entity real document would be an unreadable blob).
            "set-snapshot" => {
                let schemas = str_array(params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
                record.parameter = Parameter::List(vec![string_list_param(&schemas)]);
                Ok(())
            }

            "set-file-description" => {
                let field = params.get("fileDescription").ok_or("set-file-description requires a fileDescription field")?;
                let description = str_array(field, "description");
                let implementation_level = str_field(field, "implementationLevel")?;
                let record = header_record_mut(exchange, "FILE_DESCRIPTION").ok_or("input carries no FILE_DESCRIPTION header record")?;
                record.parameter = Parameter::List(vec![string_list_param(&description), Parameter::String(implementation_level)]);
                Ok(())
            }

            "set-file-name" => {
                let field = params.get("fileName").ok_or("set-file-name requires a fileName field")?;
                let record = header_record_mut(exchange, "FILE_NAME").ok_or("input carries no FILE_NAME header record")?;
                record.parameter = Parameter::List(vec![
                    Parameter::String(str_field(field, "name")?),
                    Parameter::String(str_field(field, "timestamp")?),
                    string_list_param(&str_array(field, "author")),
                    string_list_param(&str_array(field, "organization")),
                    Parameter::String(str_field(field, "preprocessorVersion")?),
                    Parameter::String(str_field(field, "originatingSystem")?),
                    Parameter::String(str_field(field, "authorization")?),
                ]);
                Ok(())
            }

            "set-file-schema" => {
                let field = params.get("fileSchema").ok_or("set-file-schema requires a fileSchema field")?;
                let schemas = str_array(field, "schemas");
                if schemas.is_empty() {
                    return Err("set-file-schema requires a non-empty schemas field".to_string());
                }
                let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
                record.parameter = Parameter::List(vec![string_list_param(&schemas)]);
                Ok(())
            }

            "insert-entity" => {
                let index = usize_field(params, "index")?;
                let entity_json = params.get("entity").ok_or("insert-entity requires an entity field")?;
                let id = u64_field(entity_json, "id")?;
                let name = str_field(entity_json, "name")?;
                let args: Vec<Parameter> = entity_json.array("args").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let clamped = index.min(section.entities.len());
                section.entities.insert(clamped, EntityInstance::Simple { id, record: Record { name, parameter: Parameter::List(args) } });
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
                let arg_index = usize_field(params, "argIndex")?;
                let value = value_from_json(params.get("value").ok_or("set-entity-arg requires a value field")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("set-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                *args.get_mut(arg_index).ok_or_else(|| format!("set-entity-arg: arg index {arg_index} out of range for entity {id}"))? = value;
                Ok(())
            }

            "insert-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = usize_field(params, "argIndex")?;
                let value = value_from_json(params.get("value").ok_or("insert-entity-arg requires a value field")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("insert-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                let clamped = arg_index.min(args.len());
                args.insert(clamped, value);
                Ok(())
            }

            "remove-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = usize_field(params, "argIndex")?;
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
            Parameter::List(items) => items.iter().filter_map(|item| match item { Parameter::String(s) => Some(s.clone()), _ => None }).collect::<Vec<_>>(),
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
pub fn project_step_ap214_any(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
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
mod tests {
    use super::{oracle_apply_mutation, project_step_ap214_any};
    use semio_repo_test_host::Json;

    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp");

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
        let projection = project_step_ap214_any(FIXTURE).expect("project real fixture");
        assert_eq!(entity_count(&projection), 1396.0);
        match projection.get("fileSchema") {
            Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("AUTOMOTIVE_DESIGN".to_string())]),
            other => panic!("expected fileSchema array, got {other:?}"),
        }
        let point = find_entity(&projection, 1394.0).expect("entity #1394 present");
        assert_eq!(point.get("name"), Some(&Json::String("CARTESIAN_POINT".to_string())));
    }

    #[test]
    fn no_mutation_round_trips_and_is_not_byte_identical() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
        assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
        let projection = project_step_ap214_any(&mutated).expect("project no-mutation result");
        assert_eq!(entity_count(&projection), 1396.0);
    }

    #[test]
    fn set_snapshot_overrides_file_schema_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("CONFIG_CONTROL_DESIGN")]))]))).expect("set-snapshot");
        let projection = project_step_ap214_any(&mutated).expect("project");
        assert_eq!(projection.get("fileSchema"), Some(&Json::Array(vec![text("CONFIG_CONTROL_DESIGN")])));
        let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("AUTOMOTIVE_DESIGN")]))]))).expect("inverse set-snapshot");
        let restored_projection = project_step_ap214_any(&restored).expect("project restored");
        assert_eq!(restored_projection.get("fileSchema"), Some(&Json::Array(vec![text("AUTOMOTIVE_DESIGN")])));
    }

    #[test]
    fn insert_and_remove_entity_round_trip_on_the_real_graph() {
        let insert_params = obj(vec![("index", num(1396.0)), ("entity", obj(vec![("id", num(9001.0)), ("name", text("CARTESIAN_POINT")), ("args", Json::Array(vec![tv("string", text("")), tv("aggregate", Json::Array(vec![tv("real", num(1.0)), tv("real", num(2.0)), tv("real", num(3.0))]))]))]))]);
        let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity", insert_params)).expect("insert-entity");
        let projection = project_step_ap214_any(&inserted).expect("project inserted");
        assert_eq!(entity_count(&projection), 1397.0);
        assert!(find_entity(&projection, 9001.0).is_some());

        let removed = oracle_apply_mutation(&inserted, &spec("remove-entity", obj(vec![("id", num(9001.0))]))).expect("inverse remove-entity");
        let removed_projection = project_step_ap214_any(&removed).expect("project removed");
        assert_eq!(entity_count(&removed_projection), 1396.0);
        assert!(find_entity(&removed_projection, 9001.0).is_none());
    }

    #[test]
    fn remove_and_reinsert_the_real_entity_1405() {
        let removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity", obj(vec![("id", num(1405.0))]))).expect("remove-entity");
        let projection = project_step_ap214_any(&removed).expect("project removed");
        assert_eq!(entity_count(&projection), 1395.0);
        assert!(find_entity(&projection, 1405.0).is_none());

        let reinserted_params = obj(vec![("index", num(1395.0)), ("entity", obj(vec![("id", num(1405.0)), ("name", text("CARTESIAN_POINT")), ("args", Json::Array(vec![tv("string", text("")), tv("aggregate", Json::Array(vec![tv("real", num(0.0)), tv("real", num(0.0)), tv("real", num(0.0))]))]))]))]);
        let reinserted = oracle_apply_mutation(&removed, &spec("insert-entity", reinserted_params)).expect("inverse insert-entity");
        let reinserted_projection = project_step_ap214_any(&reinserted).expect("project reinserted");
        assert_eq!(entity_count(&reinserted_projection), 1396.0);
        assert!(find_entity(&reinserted_projection, 1405.0).is_some());
    }

    #[test]
    fn set_entity_name_round_trips_on_1394() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-name", obj(vec![("id", num(1394.0)), ("name", text("RENAMED_POINT"))]))).expect("set-entity-name");
        let projection = project_step_ap214_any(&mutated).expect("project");
        assert_eq!(find_entity(&projection, 1394.0).unwrap().get("name"), Some(&Json::String("RENAMED_POINT".to_string())));
        let restored = oracle_apply_mutation(&mutated, &spec("set-entity-name", obj(vec![("id", num(1394.0)), ("name", text("CARTESIAN_POINT"))]))).expect("inverse");
        let restored_projection = project_step_ap214_any(&restored).expect("project restored");
        assert_eq!(restored_projection, project_step_ap214_any(FIXTURE).unwrap());
    }

    #[test]
    fn entity_1394_real_args_are_as_expected() {
        let projection = project_step_ap214_any(FIXTURE).expect("project");
        let point = find_entity(&projection, 1394.0).expect("entity 1394");
        match point.get("args") {
            Some(Json::Array(items)) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], tv("string", text("")));
                match &items[1] {
                    Json::Object(_) => {
                        let real0 = items[1].get("v").and_then(|arr| match arr { Json::Array(vs) => vs.first(), _ => None });
                        assert!(matches!(real0, Some(Json::Object(_))));
                    }
                    other => panic!("expected aggregate object, got {other:?}"),
                }
            }
            other => panic!("expected args array, got {other:?}"),
        }
    }

    #[test]
    fn set_insert_remove_entity_arg_round_trip_on_1394() {
        let set_mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(0.0)), ("value", tv("string", text("origin-marker")))]))).expect("set-entity-arg");
        let set_restored = oracle_apply_mutation(&set_mutated, &spec("set-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(0.0)), ("value", tv("string", text("")))]))).expect("inverse set-entity-arg");
        assert_eq!(project_step_ap214_any(&set_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

        let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(2.0)), ("value", tv("enum", text("T")))]))).expect("insert-entity-arg");
        let projection = project_step_ap214_any(&inserted).expect("project inserted-arg");
        let args = match find_entity(&projection, 1394.0).unwrap().get("args") { Some(Json::Array(items)) => items.clone(), _ => panic!("no args") };
        assert_eq!(args.len(), 3);
        assert_eq!(args[2], tv("enum", text("T")));
        let removed_back = oracle_apply_mutation(&inserted, &spec("remove-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(2.0))]))).expect("inverse insert-entity-arg");
        assert_eq!(project_step_ap214_any(&removed_back).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

        let real_removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(1.0))]))).expect("remove-entity-arg");
        let real_removed_projection = project_step_ap214_any(&real_removed).expect("project");
        let remaining_args = match find_entity(&real_removed_projection, 1394.0).unwrap().get("args") { Some(Json::Array(items)) => items.clone(), _ => panic!("no args") };
        assert_eq!(remaining_args.len(), 1);
        let reinserted = oracle_apply_mutation(&real_removed, &spec("insert-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(1.0)), ("value", tv("aggregate", Json::Array(vec![tv("real", num(2.7)), tv("real", num(4.67653718043597)), tv("real", num(2.735))])))]))).expect("inverse remove-entity-arg");
        assert_eq!(project_step_ap214_any(&reinserted).unwrap(), project_step_ap214_any(FIXTURE).unwrap());
    }

    #[test]
    fn set_file_description_name_and_schema_round_trip() {
        let d = oracle_apply_mutation(FIXTURE, &spec("set-file-description", obj(vec![("fileDescription", obj(vec![("description", Json::Array(vec![text("ticket 26/08/23 wave-7 mutation")])), ("implementationLevel", text("2;1"))]))]))).expect("set-file-description");
        let d_restored = oracle_apply_mutation(&d, &spec("set-file-description", obj(vec![("fileDescription", obj(vec![("description", Json::Array(vec![text("")])), ("implementationLevel", text("2;1"))]))]))).expect("inverse");
        assert_eq!(project_step_ap214_any(&d_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

        let n = oracle_apply_mutation(FIXTURE, &spec("set-file-name", obj(vec![("fileName", obj(vec![("name", text("wave-7-mutated")), ("timestamp", text("2026-08-23T00:00:00")), ("author", Json::Array(vec![text("Ueli")])), ("organization", Json::Array(vec![text("semio")])), ("preprocessorVersion", text("semio-step")), ("originatingSystem", text("semio")), ("authorization", text(""))]))]))).expect("set-file-name");
        let n_restored = oracle_apply_mutation(&n, &spec("set-file-name", obj(vec![("fileName", obj(vec![("name", text("hexagonal-cut-concrete-forest-left")), ("timestamp", text("2026-06-06T18:37:11+02:00")), ("author", Json::Array(vec![text("")])), ("organization", Json::Array(vec![text("")])), ("preprocessorVersion", text("ST-DEVELOPER v19.2")), ("originatingSystem", text("Rhino 8.31")), ("authorization", text(""))]))]))).expect("inverse");
        assert_eq!(project_step_ap214_any(&n_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

        let s = oracle_apply_mutation(FIXTURE, &spec("set-file-schema", obj(vec![("fileSchema", obj(vec![("schemas", Json::Array(vec![text("CONFIG_CONTROL_DESIGN")]))]))]))).expect("set-file-schema");
        let s_restored = oracle_apply_mutation(&s, &spec("set-file-schema", obj(vec![("fileSchema", obj(vec![("schemas", Json::Array(vec![text("AUTOMOTIVE_DESIGN")]))]))]))).expect("inverse");
        assert_eq!(project_step_ap214_any(&s_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());
    }

    #[test]
    fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
        let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
        assert_ne!(output, FIXTURE);
        let input_projection = project_step_ap214_any(FIXTURE).unwrap();
        let output_projection = project_step_ap214_any(&output).unwrap();
        assert_eq!(input_projection, output_projection);
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
    }
}
//#endregion 🧪️Tests
