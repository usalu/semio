//! 🔮️ IFC2X3 reference Part-21 codec — the ONE reader/writer/projection the `2x3` standard's three
//! model-view-definition subsets (`✳️cv20`, `✳️cobie`, `✳️sav`) share.
//!
//! 🏗️ IFC2X3 is physically ISO 10303-21 (STEP physical file) syntax under the IFC2X3 EXPRESS
//! schema, so the registered `ruststep` 0.4 reader parses it exactly as it parses STEP AP214.
//! `ruststep` has NO writer at all (`ast::ser::to_record` only builds an in-memory `Record` from an
//! already-typed struct, and no `Display`/`fmt::Formatter` impl exists on `Exchange`/`DataSection`/
//! `Record`/`Parameter`), so the re-serializer below is from scratch and deliberately independent
//! of this repository's own production `step::engine::part21` writer — using that writer would make
//! every subset oracle compare this repository's implementation against itself.
//!
//! Three subsets genuinely share this code, which is why it lives at the STANDARD level instead of
//! being copied into each subset's own `🦀️oracle.rs`. It is deliberately MVD-agnostic:
//! it knows Part-21 instances, arguments and header records, and nothing about Coordination View,
//! FM Handover or Structural Analysis View. Each subset's own oracle module owns its MVD semantics
//! and its own projection on top of `project_graph` below.
//!
//! @see 🪆️subsets/✳️cv20/🦀️oracle.rs — Coordination View 2.0 dispatcher.
//! @see 🪆️subsets/✳️cobie/🦀️oracle.rs — Basic FM Handover dispatcher.
//! @see 🪆️subsets/✳️sav/🦀️oracle.rs — Structural Analysis View dispatcher.

//#region 🔖️Part21
#[cfg(feature = "oracles")]
pub mod part21 {
    use ruststep::ast::{DataSection, EntityInstance, Exchange, Name, Parameter, Record};
    use semio_repo_test_host::Json;
    use std::str::FromStr;

    //#region 🔖️JsonGrammar
    /// 🔢️ Required numeric member.
    pub fn num_field(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(number)) => Ok(*number),
            _ => Err(format!("expected numeric field {key:?}")),
        }
    }

    /// 🔢️ Required entity-id member.
    pub fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        num_field(value, key).map(|number| number as u64)
    }

    /// 🔤️ Required string member.
    pub fn str_field(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(text)) => Ok(text.clone()),
            _ => Err(format!("expected string field {key:?}")),
        }
    }

    /// 🕳️ Optional member: an explicit JSON `null` and an absent key both mean "clear it", which is
    /// the wire spelling of every `Option<…>` payload the three subsets' vocabularies carry.
    pub fn opt_u64_field(value: &Json, key: &str) -> Result<Option<u64>, String> {
        match value.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::Number(number)) => Ok(Some(*number as u64)),
            other => Err(format!("expected an entity id or null for {key:?}, got {other:?}")),
        }
    }

    /// 🕳️ Optional string member, same `null`-means-clear rule.
    pub fn opt_str_field(value: &Json, key: &str) -> Result<Option<String>, String> {
        match value.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::String(text)) => Ok(Some(text.clone())),
            other => Err(format!("expected a string or null for {key:?}, got {other:?}")),
        }
    }

    /// 🕳️ Optional real member, same `null`-means-clear rule.
    pub fn opt_num_field(value: &Json, key: &str) -> Result<Option<f64>, String> {
        match value.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::Number(number)) => Ok(Some(*number)),
            other => Err(format!("expected a number or null for {key:?}, got {other:?}")),
        }
    }

    /// 🕳️ Optional object member, same `null`-means-clear rule.
    pub fn opt_obj_field<'j>(value: &'j Json, key: &str) -> Option<&'j Json> {
        match value.get(key) {
            Some(Json::Object(_)) => value.get(key),
            _ => None,
        }
    }

    /// 🔢️ Entity-id array member.
    pub fn u64_array(value: &Json, key: &str) -> Vec<u64> {
        value
            .array(key)
            .iter()
            .filter_map(|entry| match entry {
                Json::Number(number) => Some(*number as u64),
                _ => None,
            })
            .collect()
    }

    /// 🔤️ String array member.
    pub fn str_array(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .filter_map(|entry| match entry {
                Json::String(text) => Some(text.clone()),
                _ => None,
            })
            .collect()
    }
    //#endregion 🔖️JsonGrammar

    //#region 🔖️Reader
    /// 📥️ The one independent parse: `ruststep` 0.4 over real ISO 10303-21 clear text.
    pub fn read(input: &[u8]) -> Result<Exchange, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let mut exchange = Exchange::from_str(text).map_err(|error| format!("ruststep could not parse the input: {error}"))?;
        if exchange.data.is_empty() {
            exchange.data.push(DataSection { meta: Vec::new(), entities: Vec::new() });
        }
        Ok(exchange)
    }
    //#endregion 🔖️Reader

    //#region 🔖️Writer
    fn write_param(param: &Parameter, out: &mut String) {
        match param {
            Parameter::Typed { keyword, parameter } => {
                out.push_str(keyword);
                out.push('(');
                write_param(parameter, out);
                out.push(')');
            }
            Parameter::Integer(value) => out.push_str(&value.to_string()),
            Parameter::Real(value) => {
                if value.fract() == 0.0 && value.is_finite() {
                    out.push_str(&format!("{value:.0}."));
                } else {
                    out.push_str(&format!("{value}"));
                }
            }
            Parameter::String(value) => {
                out.push('\'');
                out.push_str(&value.replace('\'', "''"));
                out.push('\'');
            }
            Parameter::Enumeration(value) => {
                out.push('.');
                out.push_str(value);
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
                Name::ConstantEntity(text) => out.push_str(&format!("#{text}")),
                Name::ConstantValue(text) => out.push_str(&format!("@{text}")),
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

    /// 📤️ From-scratch Part-21 clear-text re-serialization: one line per header record, one line
    /// per entity instance. Never this repository's own `step::engine::part21` writer.
    pub fn write(exchange: &Exchange) -> Vec<u8> {
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
        out.into_bytes()
    }
    //#endregion 🔖️Writer

    //#region 🔖️Graph
    /// 🏷️ An instance's own id, simple or complex.
    pub fn entity_id(entity: &EntityInstance) -> u64 {
        match entity {
            EntityInstance::Simple { id, .. } => *id,
            EntityInstance::Complex { id, .. } => *id,
        }
    }

    /// 🏷️ An instance's leading EXPRESS type name.
    pub fn type_name(entity: &EntityInstance) -> &str {
        match entity {
            EntityInstance::Simple { record, .. } => record.name.as_str(),
            EntityInstance::Complex { subsuper, .. } => subsuper.0.first().map(|record| record.name.as_str()).unwrap_or(""),
        }
    }

    /// 🧾️ Every record of an instance, simple or complex.
    pub fn records(entity: &EntityInstance) -> Vec<&Record> {
        match entity {
            EntityInstance::Simple { record, .. } => vec![record],
            EntityInstance::Complex { subsuper, .. } => subsuper.0.iter().collect(),
        }
    }

    fn section_mut(exchange: &mut Exchange) -> Result<&mut DataSection, String> {
        exchange.data.first_mut().ok_or_else(|| "input carries no DATA section".to_string())
    }

    /// 🔎️ The instance with `id`, if the document has one.
    pub fn find<'e>(exchange: &'e Exchange, id: u64) -> Option<&'e EntityInstance> {
        exchange.data.iter().flat_map(|section| section.entities.iter()).find(|entity| entity_id(entity) == id)
    }

    /// 🔎️ One positional argument of a simple instance, as the document currently holds it.
    pub fn arg<'e>(exchange: &'e Exchange, id: u64, index: usize) -> Option<&'e Parameter> {
        match find(exchange, id) {
            Some(EntityInstance::Simple { record, .. }) => match &record.parameter {
                Parameter::List(items) => items.get(index),
                _ => None,
            },
            _ => None,
        }
    }

    /// 🧭️ Ids of every instance whose leading type name is one of `types`, id-sorted. Physical
    /// order inside the DATA section is writer freedom — an instance this vocabulary replaces is
    /// re-appended at the end — so a concept rollup that kept document order would report a
    /// difference where the exchange structure has none.
    pub fn ids_of_types(exchange: &Exchange, types: &[&str]) -> Vec<u64> {
        let mut ids: Vec<u64> = exchange.data.iter().flat_map(|section| section.entities.iter()).filter(|entity| types.contains(&type_name(entity))).map(entity_id).collect();
        ids.sort_unstable();
        ids
    }

    /// ✏️ Replaces one positional argument of a simple instance, padding with `$` when the real
    /// record is shorter than `index`. `expect` guards the MVD concept: a mutation that claims to
    /// edit an `IFCPROJECT` must fail loudly when the id names something else, never edit it anyway.
    pub fn set_arg(exchange: &mut Exchange, id: u64, expect: &[&str], index: usize, value: Parameter) -> Result<(), String> {
        let section = section_mut(exchange)?;
        let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("no instance #{id} in the document"))?;
        let EntityInstance::Simple { record, .. } = entity else {
            return Err(format!("instance #{id} is a complex instance -- this vocabulary edits simple instances only"));
        };
        if !expect.is_empty() && !expect.contains(&record.name.as_str()) {
            return Err(format!("instance #{id} is {} -- expected one of {expect:?}", record.name));
        }
        let Parameter::List(items) = &mut record.parameter else {
            return Err(format!("instance #{id} carries no positional argument list"));
        };
        while items.len() <= index {
            items.push(Parameter::NotProvided);
        }
        items[index] = value;
        Ok(())
    }

    /// ➕ Inserts a brand-new simple instance, or replaces an existing id's whole record.
    pub fn upsert_simple(exchange: &mut Exchange, id: u64, name: &str, args: Vec<Parameter>) -> Result<(), String> {
        let instance = EntityInstance::Simple { id, record: Record { name: name.to_string(), parameter: Parameter::List(args) } };
        let section = section_mut(exchange)?;
        match section.entities.iter_mut().find(|entity| entity_id(entity) == id) {
            Some(existing) => *existing = instance,
            None => section.entities.push(instance),
        }
        Ok(())
    }

    /// ➖ Deletes an instance. An absent id is an error, never a silent no-op: a quietly skipped
    /// mutation reports as a passing test.
    pub fn remove(exchange: &mut Exchange, id: u64, expect: &[&str]) -> Result<(), String> {
        let actual = find(exchange, id).map(|entity| type_name(entity).to_string()).ok_or_else(|| format!("no instance #{id} in the document"))?;
        if !expect.is_empty() && !expect.contains(&actual.as_str()) {
            return Err(format!("instance #{id} is {actual} -- expected one of {expect:?}"));
        }
        let section = section_mut(exchange)?;
        section.entities.retain(|entity| entity_id(entity) != id);
        Ok(())
    }
    //#endregion 🔖️Graph

    //#region 🔖️Header
    fn header_record_mut<'e>(exchange: &'e mut Exchange, name: &str) -> Option<&'e mut Record> {
        exchange.header.iter_mut().find(|record| record.name == name)
    }

    fn header_record<'e>(exchange: &'e Exchange, name: &str) -> Option<&'e Record> {
        exchange.header.iter().find(|record| record.name == name)
    }

    /// 🏷️ Rewrites `FILE_DESCRIPTION`'s first description string to `ViewDefinition [<view>]` — the
    /// one header field every model view definition is identified by, and the field all three
    /// subsets' own conformance checks read.
    pub fn set_view_definition(exchange: &mut Exchange, view: &str) -> Result<(), String> {
        let record = header_record_mut(exchange, "FILE_DESCRIPTION").ok_or("input carries no FILE_DESCRIPTION header record")?;
        let Parameter::List(items) = &mut record.parameter else {
            return Err("FILE_DESCRIPTION carries no argument list".to_string());
        };
        let stamped = Parameter::List(vec![Parameter::String(format!("ViewDefinition [{view}]"))]);
        if items.is_empty() {
            items.push(stamped);
        } else {
            items[0] = stamped;
        }
        Ok(())
    }

    /// 🏷️ The view definition the document currently declares, as an independently parsed string.
    pub fn view_definition(exchange: &Exchange) -> Option<String> {
        let record = header_record(exchange, "FILE_DESCRIPTION")?;
        let Parameter::List(items) = &record.parameter else { return None };
        let Some(Parameter::List(descriptions)) = items.first() else { return None };
        descriptions.iter().find_map(|item| match item {
            Parameter::String(text) => Some(text.clone()),
            _ => None,
        })
    }

    /// 🧬️ Replaces `FILE_SCHEMA`'s declared schema names.
    pub fn set_file_schema(exchange: &mut Exchange, schemas: &[String]) -> Result<(), String> {
        if schemas.is_empty() {
            return Err("FILE_SCHEMA must declare at least one schema name".to_string());
        }
        let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
        record.parameter = Parameter::List(vec![Parameter::List(schemas.iter().cloned().map(Parameter::String).collect())]);
        Ok(())
    }

    fn file_schema(exchange: &Exchange) -> Vec<String> {
        let Some(record) = header_record(exchange, "FILE_SCHEMA") else { return Vec::new() };
        let Parameter::List(items) = &record.parameter else { return Vec::new() };
        items
            .iter()
            .filter_map(|item| match item {
                Parameter::List(inner) => Some(inner.iter().filter_map(|value| match value {
                    Parameter::String(text) => Some(text.clone()),
                    _ => None,
                })),
                _ => None,
            })
            .flatten()
            .collect()
    }
    //#endregion 🔖️Header

    //#region 🔖️Projection
    /// 🔤️ One independently parsed argument value in this standard's canonical JSON shape.
    pub fn value_to_json(param: &Parameter) -> Json {
        let tagged = |tag: &str, value: Json| Json::Object(vec![("t".to_string(), Json::String(tag.to_string())), ("v".to_string(), value)]);
        match param {
            Parameter::NotProvided => Json::Object(vec![("t".to_string(), Json::String("unset".to_string()))]),
            Parameter::Omitted => Json::Object(vec![("t".to_string(), Json::String("derived".to_string()))]),
            Parameter::Integer(value) => tagged("integer", Json::Number(*value as f64)),
            Parameter::Real(value) => tagged("real", Json::Number(*value)),
            Parameter::String(value) => tagged("string", Json::String(value.clone())),
            Parameter::Enumeration(value) => tagged("enum", Json::String(value.clone())),
            Parameter::List(items) => tagged("aggregate", Json::Array(items.iter().map(value_to_json).collect())),
            Parameter::Ref(name) => tagged(
                "reference",
                Json::Number(match name {
                    Name::Entity(id) | Name::Value(id) => *id as f64,
                    Name::ConstantEntity(_) | Name::ConstantValue(_) => 0.0,
                }),
            ),
            Parameter::Typed { keyword, parameter } => Json::Object(vec![
                ("t".to_string(), Json::String("typed".to_string())),
                ("name".to_string(), Json::String(keyword.clone())),
                ("v".to_string(), value_to_json(parameter)),
            ]),
        }
    }

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

    /// 👁️ The MVD-agnostic half of every subset's projection: declared `FILE_SCHEMA`, the declared
    /// view definition, and the whole id-keyed entity graph as `ruststep` itself parsed it, sorted
    /// by id so physical order is writer freedom rather than a comparison difference.
    pub fn project_graph(exchange: &Exchange) -> Vec<(String, Json)> {
        let mut entities: Vec<Json> = exchange.data.iter().flat_map(|section| section.entities.iter()).map(entity_to_json).collect();
        entities.sort_by(|a, b| json_number(a, "id").partial_cmp(&json_number(b, "id")).unwrap_or(std::cmp::Ordering::Equal));
        vec![
            ("fileSchema".to_string(), Json::Array(file_schema(exchange).into_iter().map(Json::String).collect())),
            ("viewDefinition".to_string(), view_definition(exchange).map(Json::String).unwrap_or(Json::Null)),
            ("entityCount".to_string(), Json::Number(entities.len() as f64)),
            ("entities".to_string(), Json::Array(entities)),
        ]
    }

    /// 🧭️ A subset rollup entry: the ids carrying one MVD concept, so a projection states the
    /// concept's own population and not only the raw graph.
    pub fn concept_ids(exchange: &Exchange, types: &[&str]) -> Json {
        Json::Array(ids_of_types(exchange, types).into_iter().map(|id| Json::Number(id as f64)).collect())
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Part21
