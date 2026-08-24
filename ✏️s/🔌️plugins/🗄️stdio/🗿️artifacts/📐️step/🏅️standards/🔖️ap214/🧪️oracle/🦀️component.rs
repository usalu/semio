//! 🔮️ AP214 reference Part-21 codec and CC-ladder machinery — the ONE reader/writer/classifier the
//! `ap214` standard's seven subsets share.
//!
//! 📐️ AP214 is ISO 10303-21 clear text under the `AUTOMOTIVE_DESIGN` EXPRESS schema, so the
//! registered `ruststep` 0.4 reader parses it directly. `ruststep` has NO writer at all
//! (`ast::ser::to_record` only builds an in-memory `Record` from an already-typed struct, and no
//! `Display`/`fmt::Formatter` impl exists on `Exchange`/`DataSection`/`Record`/`Parameter`), so the
//! re-serializer below is from scratch and deliberately independent of this repository's own
//! production `step::engine::part21` writer — using that writer would make every subset oracle
//! compare this repository's implementation against itself.
//!
//! Seven subsets genuinely share this code, which is why it lives at the STANDARD level rather than
//! being copied into each subset's own `🧪️oracle/🦀️component.rs`. The split is deliberate:
//!
//! * [`part21`] knows Part-21 instances, arguments and header records, and nothing about
//!   conformance classes. `🪆️subsets/✳️any` — whose vocabulary IS the Part-21 grammar — uses only
//!   this half.
//! * [`ladder`] knows ISO 10303-214 §4.3: which `*_SHAPE_REPRESENTATION` type belongs to which
//!   conformance class, and what the three class-neutral edits do. The six `✳️ccN` subsets use both
//!   halves, each supplying its own ceiling rung and its own kind set.
//!
//! ⚠️ [`ladder`] is an INDEPENDENT re-derivation of the classification from ISO 10303-214 §4.3, not
//! a call into this repository's `engine::ladder`. The oracle crate cannot link the production
//! crate at all (its `Cargo.toml` has no such dependency), and even if it could, an oracle that
//! classified by calling the code under test would be comparing an implementation with itself. The
//! five explicitly named subtypes and the "anything else geometry-bearing is rung 2" fallback are
//! read off the standard on both sides, which is exactly what makes their agreement evidence.
//!
//! @see 🪆️subsets/✳️cc1/🧪️oracle/🦀️component.rs — config data only, the class that admits no
//!      representation at all.
//! @see 🪆️subsets/✳️cc6/🧪️oracle/🦀️component.rs — advanced B-rep, the class the committed fixture
//!      already conforms to.

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

    /// 🔤️ Optional string member — absent and `null` are the same answer.
    pub fn opt_str_field(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }

    /// 🔢️ Optional entity-id member.
    pub fn opt_u64_field(value: &Json, key: &str) -> Option<u64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number as u64),
            _ => None,
        }
    }

    /// 📇️ String array member, skipping anything that is not a string rather than failing — a
    /// scenario writes its own cells, and a malformed cell should surface as a projection
    /// divergence with a readable value, not as a parse error two layers away.
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

    /// 📇️ Entity-id array member.
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
    //#endregion 🔖️JsonGrammar

    //#region 🔖️Reader
    /// 📥️ Parses a real exchange structure with the registered third-party reader, guaranteeing at
    /// least one `DATA` section so a caller never has to special-case an empty document.
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
    /// ✍️ One Part-21 argument value. Reals carry an explicit decimal point because ISO 10303-21
    /// distinguishes `3` (integer) from `3.` (real), and apostrophes inside strings are doubled per
    /// §6.2 — the escape a real IfcOpenShell/ST-Developer export genuinely uses.
    pub fn write_param(param: &Parameter, out: &mut String) {
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
            Parameter::String(text) => {
                out.push('\'');
                out.push_str(&text.replace('\'', "''"));
                out.push('\'');
            }
            Parameter::Enumeration(text) => {
                out.push('.');
                out.push_str(text);
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

    pub fn write_record(record: &Record, out: &mut String) {
        out.push_str(&record.name);
        write_param(&record.parameter, out);
    }

    pub fn write_entity(entity: &EntityInstance, out: &mut String) {
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

    /// 📤️ The whole exchange structure, re-serialized from the parsed model alone — one line per
    /// header record and per instance, with none of the 78-column wrapping real writers use for
    /// readability (spec-optional, never semantically required).
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

    //#region 🔖️Access
    pub fn entity_id(entity: &EntityInstance) -> u64 {
        match entity {
            EntityInstance::Simple { id, .. } => *id,
            EntityInstance::Complex { id, .. } => *id,
        }
    }

    pub fn primary_record(entity: &EntityInstance) -> &Record {
        match entity {
            EntityInstance::Simple { record, .. } => record,
            EntityInstance::Complex { subsuper, .. } => &subsuper.0[0],
        }
    }

    pub fn primary_record_mut(entity: &mut EntityInstance) -> &mut Record {
        match entity {
            EntityInstance::Simple { record, .. } => record,
            EntityInstance::Complex { subsuper, .. } => &mut subsuper.0[0],
        }
    }

    /// 🏷️ Every type name an instance carries — one for a simple instance, all of them for a
    /// complex `#N=(A(..) B(..))` one, which a conformance class must see in full.
    pub fn type_names(entity: &EntityInstance) -> Vec<&str> {
        match entity {
            EntityInstance::Simple { record, .. } => vec![record.name.as_str()],
            EntityInstance::Complex { subsuper, .. } => subsuper.0.iter().map(|record| record.name.as_str()).collect(),
        }
    }

    /// 🔎️ A record's positional argument list — real Part-21 records always carry
    /// `Parameter::List(..)` at the top level; anything else is malformed input.
    pub fn args(record: &Record) -> Result<&Vec<Parameter>, String> {
        match &record.parameter {
            Parameter::List(items) => Ok(items),
            other => Err(format!("record {:?} does not carry a positional argument list ({other:?})", record.name)),
        }
    }

    pub fn args_mut(record: &mut Record) -> Result<&mut Vec<Parameter>, String> {
        match &mut record.parameter {
            Parameter::List(items) => Ok(items),
            other => Err(format!("record {:?} does not carry a positional argument list ({other:?})", record.name)),
        }
    }

    pub fn header_record<'e>(exchange: &'e Exchange, name: &str) -> Option<&'e Record> {
        exchange.header.iter().find(|record| record.name == name)
    }

    pub fn header_record_mut<'e>(exchange: &'e mut Exchange, name: &str) -> Option<&'e mut Record> {
        exchange.header.iter_mut().find(|record| record.name == name)
    }

    pub fn find<'e>(exchange: &'e Exchange, id: u64) -> Option<&'e EntityInstance> {
        exchange.data.iter().flat_map(|section| section.entities.iter()).find(|entity| entity_id(entity) == id)
    }

    pub fn find_mut<'e>(exchange: &'e mut Exchange, id: u64) -> Option<&'e mut EntityInstance> {
        exchange.data.iter_mut().flat_map(|section| section.entities.iter_mut()).find(|entity| entity_id(entity) == id)
    }

    /// 📇️ A list of entity references as an argument value.
    pub fn reference_list(ids: &[u64]) -> Parameter {
        Parameter::List(ids.iter().map(|id| Parameter::Ref(Name::Entity(*id))).collect())
    }

    /// 📇️ A list of string literals as an argument value.
    pub fn string_list(values: &[String]) -> Parameter {
        Parameter::List(values.iter().cloned().map(Parameter::String).collect())
    }

    pub fn as_ref_id(param: &Parameter) -> Option<u64> {
        match param {
            Parameter::Ref(Name::Entity(id)) | Parameter::Ref(Name::Value(id)) => Some(*id),
            _ => None,
        }
    }

    pub fn as_text(param: &Parameter) -> Option<&str> {
        match param {
            Parameter::String(text) => Some(text.as_str()),
            _ => None,
        }
    }
    //#endregion 🔖️Access

    //#region 🔖️Header
    /// 🏷️ The schema names `FILE_SCHEMA` declares, flattened out of its nested lists.
    pub fn file_schema_names(exchange: &Exchange) -> Vec<String> {
        fn walk(param: &Parameter, out: &mut Vec<String>) {
            match param {
                Parameter::String(text) => out.push(text.clone()),
                Parameter::List(items) => items.iter().for_each(|item| walk(item, out)),
                Parameter::Typed { parameter, .. } => walk(parameter, out),
                _ => {}
            }
        }
        let mut out = Vec::new();
        if let Some(record) = header_record(exchange, "FILE_SCHEMA") {
            walk(&record.parameter, &mut out);
        }
        out
    }

    /// ✍️ Replaces `FILE_SCHEMA` with exactly `names`, creating the record when the input has none.
    pub fn set_file_schema_names(exchange: &mut Exchange, names: &[String]) {
        let parameter = Parameter::List(vec![string_list(names)]);
        match header_record_mut(exchange, "FILE_SCHEMA") {
            Some(record) => record.parameter = parameter,
            None => exchange.header.push(Record { name: "FILE_SCHEMA".to_string(), parameter }),
        }
    }
    //#endregion 🔖️Header
}
//#endregion 🔖️Part21

//#region 🔖️Ladder
#[cfg(feature = "oracles")]
pub mod ladder {
    use super::part21;
    use ruststep::ast::{EntityInstance, Exchange, Parameter, Record};
    use semio_repo_test_host::Json;

    //#region 🔖️Classification
    /// 🪜️ Minimum ISO 10303-214 conformance class (2..=6) a `*_SHAPE_REPRESENTATION` subtype
    /// requires, or `None` for a type that is not a representation at all. Re-derived from
    /// ISO 10303-214 §4.3 rather than called out of the production classifier — an oracle that asked
    /// the code under test how to classify would be comparing an implementation with itself.
    ///
    /// The bare `SHAPE_REPRESENTATION` base type and any unlisted `*_SHAPE_REPRESENTATION` subtype
    /// classify as rung 2: they carry geometry, and CC1 admits none, so rung 1 is not an honest
    /// answer for anything the ladder recognises at all.
    pub fn rung_of(type_name: &str) -> Option<u8> {
        let upper = type_name.to_ascii_uppercase();
        if !upper.ends_with("SHAPE_REPRESENTATION") {
            return None;
        }
        Some(match upper.as_str() {
            "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION" => 2,
            "GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION" => 3,
            "MANIFOLD_SURFACE_SHAPE_REPRESENTATION" => 4,
            "FACETED_BREP_SHAPE_REPRESENTATION" => 5,
            "ADVANCED_BREP_SHAPE_REPRESENTATION" => 6,
            _ => 2,
        })
    }

    /// 🪜️ The representation type sitting exactly on a class's ceiling — what a demotion rewrites
    /// an over-rung instance into. `None` at ceiling 1: CC1 admits no representation, so it has no
    /// ceiling type and its only repair is deletion.
    pub fn ceiling_type_of(max_rung: u8) -> Option<&'static str> {
        match max_rung {
            2 => Some("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION"),
            3 => Some("GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION"),
            4 => Some("MANIFOLD_SURFACE_SHAPE_REPRESENTATION"),
            5 => Some("FACETED_BREP_SHAPE_REPRESENTATION"),
            6 => Some("ADVANCED_BREP_SHAPE_REPRESENTATION"),
            _ => None,
        }
    }

    /// 🔍️ Every `(id, type, rung)` on the ladder, id-sorted.
    pub fn census(exchange: &Exchange) -> Vec<(u64, String, u8)> {
        let mut out: Vec<(u64, String, u8)> = Vec::new();
        for section in &exchange.data {
            for entity in &section.entities {
                for name in part21::type_names(entity) {
                    if let Some(rung) = rung_of(name) {
                        out.push((part21::entity_id(entity), name.to_string(), rung));
                    }
                }
            }
        }
        out.sort_by_key(|(id, _, _)| *id);
        out
    }

    /// 🚧️ The census entries a class of ceiling `max_rung` must reject.
    pub fn violations(exchange: &Exchange, max_rung: u8) -> Vec<(u64, String, u8)> {
        census(exchange).into_iter().filter(|(_, _, rung)| *rung > max_rung).collect()
    }
    //#endregion 🔖️Classification

    //#region 🔖️ProductChain
    /// 🏭️ `product` has no subtypes in ISO 10303-41.
    pub const PRODUCT_TYPES: &[&str] = &["PRODUCT"];
    /// 🏭️ `product_definition_formation` and its one ISO 10303-41 subtype — the form real AP214 and
    /// AP242 exporters actually write.
    pub const FORMATION_TYPES: &[&str] = &["PRODUCT_DEFINITION_FORMATION", "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE"];
    /// 🏭️ `product_definition` and its one ISO 10303-41 subtype.
    pub const DEFINITION_TYPES: &[&str] = &["PRODUCT_DEFINITION", "PRODUCT_DEFINITION_WITH_ASSOCIATED_DOCUMENTS"];

    fn first_of<'e>(exchange: &'e Exchange, names: &[&str]) -> Option<&'e EntityInstance> {
        exchange
            .data
            .iter()
            .flat_map(|section| section.entities.iter())
            .find(|entity| part21::type_names(entity).iter().any(|found| names.iter().any(|name| found.eq_ignore_ascii_case(name))))
    }

    /// 🔗️ Does the document carry all three rungs of AP214's product identity chain, counting the
    /// ISO 10303-41 subtypes? Subtyping is enumerated, never inferred from a name prefix:
    /// `PRODUCT_DEFINITION_FORMATION` begins with `PRODUCT_DEFINITION` and is a different entity.
    pub fn has_product_chain(exchange: &Exchange) -> bool {
        first_of(exchange, PRODUCT_TYPES).is_some() && first_of(exchange, FORMATION_TYPES).is_some() && first_of(exchange, DEFINITION_TYPES).is_some()
    }
    //#endregion 🔖️ProductChain

    //#region 🔖️Edits
    fn representation_record(entity: &EntityInstance) -> Option<&Record> {
        match entity {
            EntityInstance::Simple { record, .. } => rung_of(&record.name).map(|_| record),
            EntityInstance::Complex { subsuper, .. } => subsuper.0.iter().find(|record| rung_of(&record.name).is_some()),
        }
    }

    /// 🔎️ The ladder-relevant instance at `id`, as the `{typeName, name, items, context}` wire shape
    /// the `✳️ccN` scenarios speak.
    pub fn representation_json(exchange: &Exchange, id: u64) -> Option<Json> {
        let record = representation_record(part21::find(exchange, id)?)?;
        let arguments = part21::args(record).ok()?;
        Some(Json::Object(vec![
            ("typeName".to_string(), Json::String(record.name.clone())),
            ("name".to_string(), Json::String(arguments.first().and_then(part21::as_text).unwrap_or_default().to_string())),
            (
                "items".to_string(),
                Json::Array(match arguments.get(1) {
                    Some(Parameter::List(items)) => items.iter().filter_map(part21::as_ref_id).map(|id| Json::Number(id as f64)).collect(),
                    _ => Vec::new(),
                }),
            ),
            (
                "context".to_string(),
                match arguments.get(2).and_then(part21::as_ref_id) {
                    Some(context) => Json::Number(context as f64),
                    None => Json::Null,
                },
            ),
        ]))
    }

    /// ✍️ Writes the representation described by `row` at `id`, replacing whatever was there. The
    /// ceiling guard belongs to the CALLER — each `✳️ccN` oracle owes a refusal that names its own
    /// class, which a class-neutral helper cannot phrase.
    pub fn upsert_representation(exchange: &mut Exchange, id: u64, row: &Json) -> Result<(), String> {
        let type_name = part21::str_field(row, "typeName")?;
        let arguments = vec![
            Parameter::String(part21::opt_str_field(row, "name").unwrap_or_default()),
            part21::reference_list(&part21::u64_array(row, "items")),
            match part21::opt_u64_field(row, "context") {
                Some(context) => Parameter::Ref(ruststep::ast::Name::Entity(context)),
                None => Parameter::NotProvided,
            },
        ];
        let instance = EntityInstance::Simple { id, record: Record { name: type_name.to_ascii_uppercase(), parameter: Parameter::List(arguments) } };
        let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
        match section.entities.iter().position(|entity| part21::entity_id(entity) == id) {
            Some(at) => section.entities[at] = instance,
            None => section.entities.push(instance),
        }
        Ok(())
    }

    /// 🗑️ Deletes the instance at `id`, refusing anything that is not on the ladder — a conformance
    /// repair must never delete a geometry or product record because a scenario named a wrong id.
    pub fn remove_representation(exchange: &mut Exchange, id: u64) -> Result<(), String> {
        for section in exchange.data.iter_mut() {
            if let Some(at) = section.entities.iter().position(|entity| part21::entity_id(entity) == id && representation_record(entity).is_some()) {
                section.entities.remove(at);
                return Ok(());
            }
        }
        Err(format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document -- a ladder edit addresses the ladder, never an arbitrary entity"))
    }

    /// ⬇️ Rewrites the representation at `id` onto `ceiling`, keeping its name, items and context.
    /// Returns the type name it replaced.
    pub fn demote_representation(exchange: &mut Exchange, id: u64, ceiling: &str) -> Result<String, String> {
        let entity = part21::find_mut(exchange, id).ok_or_else(|| format!("#{id} is not present in this document"))?;
        let record = match entity {
            EntityInstance::Simple { record, .. } => record,
            EntityInstance::Complex { subsuper, .. } => subsuper.0.iter_mut().find(|record| rung_of(&record.name).is_some()).ok_or_else(|| format!("#{id} carries no *_SHAPE_REPRESENTATION record"))?,
        };
        if rung_of(&record.name).is_none() {
            return Err(format!("#{id} is a {:?}, not a *_SHAPE_REPRESENTATION", record.name));
        }
        let previous = record.name.clone();
        record.name = ceiling.to_ascii_uppercase();
        Ok(previous)
    }

    /// 🏭️ Writes the whole product identity chain, or — with `None` — removes every instance of all
    /// three rungs, which is the only edit that deterministically turns the soft product-chain
    /// diagnostic ON.
    ///
    /// ⚠️ **A real defect in the reference library, reproduced standalone before being worked
    /// around.** The authored `PRODUCT` carries three of ISO 10303-41's four attributes; its
    /// `frame_of_reference` — a SET of `product_context`, empty for a chain this function authors
    /// out of nothing — is OMITTED rather than written as the empty aggregate `()` that
    /// ISO 10303-21 §6.2 explicitly permits, because **`ruststep` 0.4 cannot parse an empty
    /// aggregate as an argument value.** Measured directly in this ticket's scratch probe against
    /// the crate itself: `#1=FOO(());` and `#1=FOO('a',());` both fail with `Error while tokenizing
    /// STEP input … in Tag: DATA;`, while `#1=FOO();`, `#1=FOO('a',(''));` and `#1=FOO('a',$);` all
    /// parse. (The real committed fixture is unaffected: its four `()` occurrences —
    /// `LENGTH_UNIT()`, `PLANE_ANGLE_UNIT()`, `SOLID_ANGLE_UNIT()` — are empty RECORD argument
    /// lists inside complex instances, not empty aggregate VALUES, and those parse fine.)
    ///
    /// Emitting the spec-legal `()` would produce a document the registered independent reader
    /// refuses to read back, which would make the inverse and identity laws untestable for a reason
    /// that has nothing to do with this repository. The omission costs no evidence: the projection
    /// reports the chain's ids and names, which is exactly what `has_product_definition_chain`
    /// reads. Recorded here rather than hidden behind a loosened projection.
    pub fn set_product_identity(exchange: &mut Exchange, identity: Option<&Json>) -> Result<(), String> {
        let chain: Vec<&str> = PRODUCT_TYPES.iter().chain(FORMATION_TYPES).chain(DEFINITION_TYPES).copied().collect();
        for section in exchange.data.iter_mut() {
            section.entities.retain(|entity| !part21::type_names(entity).iter().any(|found| chain.iter().any(|name| found.eq_ignore_ascii_case(name))));
        }
        let Some(identity) = identity else { return Ok(()) };
        let product = part21::u64_field(identity, "product")?;
        let formation = part21::u64_field(identity, "formation")?;
        let definition = part21::u64_field(identity, "definition")?;
        let product_name = part21::opt_str_field(identity, "productName").unwrap_or_default();
        let formation_id = part21::opt_str_field(identity, "formationId").unwrap_or_default();
        let definition_id = part21::opt_str_field(identity, "definitionId").unwrap_or_default();
        let simple = |id: u64, name: &str, arguments: Vec<Parameter>| EntityInstance::Simple { id, record: Record { name: name.to_string(), parameter: Parameter::List(arguments) } };
        let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
        section.entities.push(simple(product, PRODUCT_TYPES[0], vec![Parameter::String(product_name.clone()), Parameter::String(product_name), Parameter::String(String::new())]));
        section.entities.push(simple(formation, FORMATION_TYPES[0], vec![Parameter::String(formation_id), Parameter::NotProvided, Parameter::Ref(ruststep::ast::Name::Entity(product))]));
        section.entities.push(simple(definition, DEFINITION_TYPES[0], vec![Parameter::String(definition_id), Parameter::NotProvided, Parameter::Ref(ruststep::ast::Name::Entity(formation)), Parameter::NotProvided]));
        section.entities.sort_by_key(|entity| part21::entity_id(entity));
        Ok(())
    }

    /// 🏭️ The product identity chain the document carries, as the wire shape the scenarios speak,
    /// or `Json::Null` when any rung is missing.
    pub fn product_identity_json(exchange: &Exchange) -> Json {
        let text = |entity: &EntityInstance| part21::args(part21::primary_record(entity)).ok().and_then(|arguments| arguments.first()).and_then(part21::as_text).unwrap_or_default().to_string();
        let (Some(product), Some(formation), Some(definition)) = (first_of(exchange, PRODUCT_TYPES), first_of(exchange, FORMATION_TYPES), first_of(exchange, DEFINITION_TYPES)) else {
            return Json::Null;
        };
        Json::Object(vec![
            ("product".to_string(), Json::Number(part21::entity_id(product) as f64)),
            ("productName".to_string(), Json::String(text(product))),
            ("formation".to_string(), Json::Number(part21::entity_id(formation) as f64)),
            ("formationId".to_string(), Json::String(text(formation))),
            ("definition".to_string(), Json::Number(part21::entity_id(definition) as f64)),
            ("definitionId".to_string(), Json::String(text(definition))),
        ])
    }
    //#endregion 🔖️Edits

    //#region 🔖️Projection
    /// 👁️ The conformance-class projection every `✳️ccN` case is compared by: the schema declaration,
    /// the whole ladder census, the count of instances above the class ceiling, and the product
    /// identity chain. It reports exactly the three axes `check_ccN_conformance` reads and nothing
    /// else — a projection that claimed the whole entity graph would be the `✳️any` subset's
    /// projection wearing a conformance class's name, and would drown every class-level difference
    /// in 1,396 entities of unrelated geometry.
    pub fn project(bytes: &[u8], max_rung: u8) -> Result<Json, String> {
        let exchange = part21::read(bytes)?;
        let census = census(&exchange);
        let entries: Vec<Json> = census
            .iter()
            .map(|(id, type_name, rung)| Json::Object(vec![("id".to_string(), Json::Number(*id as f64)), ("typeName".to_string(), Json::String(type_name.clone())), ("rung".to_string(), Json::Number(f64::from(*rung)))]))
            .collect();
        Ok(Json::Object(vec![
            ("fileSchema".to_string(), Json::Array(part21::file_schema_names(&exchange).into_iter().map(Json::String).collect())),
            ("conformsToClass".to_string(), Json::Bool(census.iter().all(|(_, _, rung)| *rung <= max_rung))),
            ("aboveCeiling".to_string(), Json::Number(census.iter().filter(|(_, _, rung)| *rung > max_rung).count() as f64)),
            ("representations".to_string(), Json::Array(entries)),
            ("hasProductChain".to_string(), Json::Bool(has_product_chain(&exchange))),
            ("productIdentity".to_string(), product_identity_json(&exchange)),
            ("entityCount".to_string(), Json::Number(exchange.data.iter().map(|section| section.entities.len()).sum::<usize>() as f64)),
        ]))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Ladder

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::{ladder, part21};
    use semio_repo_test_host::Json;

    /// 🧫️ The real committed AP214 fixture — a real Rhino 8.31 / ST-Developer v19.2 export whose
    /// entire DATA section is untouched real data.
    fn fixture() -> Vec<u8> {
        include_bytes!("../../../🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp").to_vec()
    }

    #[test]
    fn the_real_fixture_carries_exactly_the_ladder_the_standard_predicts() {
        let exchange = part21::read(&fixture()).expect("ruststep parses the real export");
        let census = ladder::census(&exchange);
        assert_eq!(census.len(), 2, "the real file carries two representations: {census:?}");
        assert_eq!(census[0], (13, "ADVANCED_BREP_SHAPE_REPRESENTATION".to_string(), 6));
        assert_eq!(census[1], (836, "SHAPE_REPRESENTATION".to_string(), 2), "the bare base type classifies as rung 2, the minimal geometry-bearing form");
        assert!(ladder::violations(&exchange, 6).is_empty(), "nothing in a real AP214 export can exceed the top of the ladder");
        assert_eq!(ladder::violations(&exchange, 1).len(), 2, "CC1 admits neither of them");
    }

    /// 🏭️ `SHAPE_REPRESENTATION_RELATIONSHIP` (`#10` of the real file) must NOT be on the ladder:
    /// it does not end in `SHAPE_REPRESENTATION`, and counting it would make every class report a
    /// violation the standard does not describe.
    #[test]
    fn a_relationship_is_not_a_representation() {
        assert_eq!(ladder::rung_of("SHAPE_REPRESENTATION_RELATIONSHIP"), None);
        assert_eq!(ladder::rung_of("SHAPE_REPRESENTATION"), Some(2));
    }

    #[test]
    fn the_real_fixture_carries_the_product_chain_through_the_iso_10303_41_subtype() {
        let exchange = part21::read(&fixture()).expect("ruststep parses the real export");
        assert!(ladder::has_product_chain(&exchange), "the formation rung is PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE, a real subtype");
        let identity = ladder::product_identity_json(&exchange);
        assert_eq!(identity.get("product"), Some(&Json::Number(827.0)));
        assert_eq!(identity.get("formation"), Some(&Json::Number(822.0)));
        assert_eq!(identity.get("definition"), Some(&Json::Number(821.0)));
    }

    #[test]
    fn the_writer_round_trips_the_real_export_through_the_independent_reader() {
        let input = fixture();
        let exchange = part21::read(&input).expect("ruststep parses the real export");
        let written = part21::write(&exchange);
        assert_ne!(written, input, "a from-scratch writer cannot reproduce another writer's layout -- identical bytes would mean the input was copied");
        let reparsed = part21::read(&written).expect("ruststep parses this module's own output");
        assert_eq!(ladder::census(&reparsed), ladder::census(&exchange));
        assert_eq!(ladder::project(&written, 6).unwrap(), ladder::project(&input, 6).unwrap());
    }

    #[test]
    fn a_demotion_keeps_the_representation_and_only_moves_its_rung() {
        let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
        let before = ladder::representation_json(&exchange, 13).expect("#13 is a representation");
        let previous = ladder::demote_representation(&mut exchange, 13, ladder::ceiling_type_of(4).unwrap()).expect("a real representation demotes");
        assert_eq!(previous, "ADVANCED_BREP_SHAPE_REPRESENTATION");
        let after = ladder::representation_json(&exchange, 13).expect("still a representation");
        assert_eq!(after.get("name"), before.get("name"), "a demotion must not rename the representation");
        assert_eq!(after.get("items"), before.get("items"), "a demotion must not discard its items");
        assert_eq!(after.get("typeName"), Some(&Json::String("MANIFOLD_SURFACE_SHAPE_REPRESENTATION".to_string())));
    }

    #[test]
    fn a_ladder_edit_refuses_an_instance_that_is_not_on_the_ladder() {
        let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
        assert!(ladder::remove_representation(&mut exchange, 827).is_err(), "#827 is the PRODUCT record");
        assert!(ladder::remove_representation(&mut exchange, 10).is_err(), "#10 is a SHAPE_REPRESENTATION_RELATIONSHIP");
        assert!(ladder::has_product_chain(&exchange), "a refused edit changes nothing");
    }

    #[test]
    fn clearing_the_product_identity_is_what_turns_the_soft_diagnostic_on() {
        let mut exchange = part21::read(&fixture()).expect("ruststep parses the real export");
        ladder::set_product_identity(&mut exchange, None).expect("clearing always succeeds");
        assert!(!ladder::has_product_chain(&exchange));
        assert_eq!(ladder::product_identity_json(&exchange), Json::Null);
    }

    #[test]
    fn every_ceiling_type_classifies_back_to_its_own_rung() {
        assert_eq!(ladder::ceiling_type_of(1), None, "CC1 admits no representation, so it has no ceiling type");
        for rung in 2..=6u8 {
            assert_eq!(ladder::rung_of(ladder::ceiling_type_of(rung).unwrap()), Some(rung));
        }
    }
}
//#endregion 🧪️Tests
