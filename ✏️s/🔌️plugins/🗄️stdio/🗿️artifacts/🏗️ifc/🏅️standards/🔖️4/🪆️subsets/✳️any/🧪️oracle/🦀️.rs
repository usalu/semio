//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed against
//! a real ISO 10303-21 exchange structure parsed by the registered `ruststep` 0.4 reader, then
//! re-serialized by this module's own from-scratch Part-21 writer (ruststep 0.4 has none — confirmed
//! by reading its source: `ast::ser::to_record` only builds an in-memory `Record` from an
//! already-typed struct, and grepping the crate for `Display`/`fmt::Formatter` impls on
//! `Exchange`/`DataSection`/`Record`/`Parameter` finds none — the same finding this wave's STEP
//! AP214 case already made, confirmed independently here rather than assumed from that precedent).
//!
//! KEY INSIGHT this subset's assignment is built on: IFC4 is not "IFC syntax" — it is a real ISO
//! 10303-21 (STEP physical file / Part-21) EXCHANGE STRUCTURE whose DATA section happens to carry
//! IFC4's own EXPRESS schema instead of an AP-series one. `ruststep`'s `ast` module (`Exchange`,
//! `DataSection`, `Record`, `Parameter`, `EntityInstance`) parses the Part-21 GRAMMAR only — it does
//! not validate against any generated EXPRESS schema module (ruststep compiles no IFC4 schema at
//! all) — so the same reader that STEP AP214 registered this wave applies here unchanged. Confirmed
//! empirically, not assumed: `parses_the_real_fixture_and_projects_it` below feeds ruststep this
//! subset's real 2.5 MB, 24792-entity Nakagin Capsule Tower export and it parses with zero errors.
//!
//! ## §6: ruststep is the independent READER, never a second producer
//! Because ruststep cannot write, this module cannot be a genuine differential producer of mutated
//! bytes against a real third-party writer. Every scenario in
//! `../../../../🧪️tests/mutate-ifc-4/🥒️.feature` is therefore typed `@mode-property`/
//! `@mode-round-trip`, never `@mode-differential` — the fleet brief's §6 situation. `ruststep::ast::
//! Exchange::from_str` is what actually reads both the real input and every re-serialized result —
//! including this dispatcher's own mutation output and (once the subject phase compiles) the
//! subject's — through `project_ifc_4_any` below, which is the one place a genuinely independent,
//! third-party parse of the result happens.
//!
//! The from-scratch Part-21 writer below is a deliberate, brief-compliant duplication of the one
//! this wave's STEP AP214 oracle already wrote (`../../../../../📐️step/🏅️standards/🔖️ap214/🪆️subsets/
//! ✳️any/🦀️oracle.rs`) rather than a shared helper: the fleet brief's shared-family-module
//! table names exactly six families (document/raster/archive/audio/tabular/mesh), STEP is not one of
//! them, and reaching into another artifact's file to extract one is out of bounds for this ticket
//! ("Stay inside your artifact. Do not edit another artifact's files."). Both real ISO 10303-21
//! writers are ~40 lines and identical in shape because the underlying grammar is identical; a future
//! ticket owning both artifacts could promote it.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through a
//! shared family module rather than by copying it — this subset has no such sibling yet, so nothing
//! here is promoted to `✏️s/🔌️plugins/🗄️stdio/🧪️oracle`.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`IfcMutation::KINDS`).

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

    //#region 🔖️RuststepEscapeDefect
    /// 🐛️ CONFIRMED defect in `ruststep` 0.4's own tokenizer (`src/parser/token.rs::string`):
    /// `tuple((char('\''), many0(none_of("'")), char('\'')))` never implements the doubled-
    /// apostrophe escape ("`''`" inside a string means one literal `'`) that the Part-21 grammar
    /// itself defines (`string = ... | [apostrophe] [apostrophe] | ...`) and that real IFC content
    /// legitimately uses. Reproduced standalone before being worked around, not assumed: this real
    /// fixture's entity `#17012` (`IFCPROPERTYSINGLEVALUE('composePort',$,IFCLABEL('{''guid'':
    /// ''019ab243-...''}'),$)`) carries an embedded-JSON string escaped exactly this way, and a
    /// standalone probe (this ticket's scratch folder) confirmed `ruststep::ast::Exchange::from_str`
    /// fails on it with a misleading "expected `END-ISO-10303-21;`, found `DATA;`" error — the
    /// tokenizer terminates the string at the FIRST embedded apostrophe, and every following token
    /// cascades out of sync until the whole `DATA` section silently fails to match and `opt_`/
    /// `many0_` swallow the real cause.
    ///
    /// Worked around here, not hidden by loosening the projection: a real string-delimiter-aware
    /// single pass (`escape_doubled_apostrophes` below — a blind text-level `.replace("''", ..)` was
    /// tried first and confirmed WRONG, since `('')` — a list holding one EMPTY string, real content
    /// this fixture's own `FILE_NAME` header record carries twice — is also two adjacent apostrophes
    /// at the character level and must NOT be collapsed) tracks whether the cursor is currently
    /// inside an open string; only a `''` pair encountered WHILE ALREADY inside one is an escaped
    /// apostrophe (replaced with one private-use sentinel codepoint, invisible to `none_of("'")` so
    /// the string tokenizes correctly past it) — the same pair seen from a closed state is a fresh
    /// empty string and is left untouched. Every resulting `Parameter::String` is then walked and the
    /// sentinel restored to a literal `'`, recovering the exact real content ruststep's own grammar
    /// comment says it should have produced. `write_exchange`'s own `Parameter::String` arm already
    /// re-escapes `'` as `''` on the way back out, so the round trip is exact.
    const APOSTROPHE_SENTINEL: char = '\u{E000}';

    fn escape_doubled_apostrophes(text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut in_string = false;
        let mut index = 0;
        while index < chars.len() {
            let c = chars[index];
            if c != '\'' {
                out.push(c);
                index += 1;
                continue;
            }
            if !in_string {
                in_string = true;
                out.push(c);
                index += 1;
            } else if chars.get(index + 1) == Some(&'\'') {
                out.push(APOSTROPHE_SENTINEL);
                index += 2;
            } else {
                in_string = false;
                out.push(c);
                index += 1;
            }
        }
        out
    }

    fn unescape_param(param: &mut Parameter) {
        match param {
            Parameter::String(s) => {
                if s.contains(APOSTROPHE_SENTINEL) {
                    *s = s.replace(APOSTROPHE_SENTINEL, "'");
                }
            }
            Parameter::List(items) => items.iter_mut().for_each(unescape_param),
            Parameter::Typed { parameter, .. } => unescape_param(parameter),
            Parameter::NotProvided | Parameter::Omitted | Parameter::Integer(_) | Parameter::Real(_) | Parameter::Enumeration(_) | Parameter::Ref(_) => {}
        }
    }

    fn unescape_record(record: &mut Record) {
        unescape_param(&mut record.parameter);
    }

    fn unescape_exchange(exchange: &mut Exchange) {
        exchange.header.iter_mut().for_each(unescape_record);
        for section in &mut exchange.data {
            for entity in &mut section.entities {
                match entity {
                    EntityInstance::Simple { record, .. } => unescape_record(record),
                    EntityInstance::Complex { subsuper, .. } => subsuper.0.iter_mut().for_each(unescape_record),
                }
            }
        }
    }

    /// 🩹️ The one seam every real parse of this subset's Part-21 text goes through — both the input
    /// and every re-serialized result (this dispatcher's own and, once compiled, the subject's) —
    /// so the apostrophe-escape workaround above is applied uniformly rather than at each call site.
    fn parse_exchange(text: &str) -> Result<Exchange, String> {
        let preprocessed = escape_doubled_apostrophes(text);
        let mut exchange = Exchange::from_str(&preprocessed).map_err(|error| format!("ruststep could not parse the input: {error}"))?;
        unescape_exchange(&mut exchange);
        Ok(exchange)
    }
    //#endregion 🔖️RuststepEscapeDefect

    //#region 🔖️ValueGrammar
    /// 🔤️ This module's own JSON wire grammar for one Part-21 argument value — the wire shape the
    /// feature file's `Examples` tables and this subset's subject-side `mutation_from_spec` both
    /// speak (`{"t":"real","v":1.0}`-shaped), independent of `IfcValue`'s own serde tagging.
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
    /// canonical JSON shape — used both to echo a real argument back out in `project_ifc_4_any` and,
    /// transitively, inside `aggregate`'s recursion.
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
    /// record, one line per entity instance, no attempt at the 78-column wrapping real Part-21
    /// writers use for readability (spec-optional, never semantically required).
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
    /// List(..)` at the top level (confirmed against this subset's real fixture's entities in the
    /// tests below); anything else is malformed input.
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
    /// 🦠️ Applies one declared `IfcMutation::KINDS` kind to a real, independently-parsed
    /// `ruststep::ast::Exchange` — one arm per variant, matched by its kebab-case spelling. An
    /// unrecognised kind is an error, never a silent no-op. `remove-entity` is deliberately exercised
    /// (see the feature file and this subset's mutate-ifc-4 case) against a real capsule entity that
    /// OTHER entities reference by id — this dispatcher removes exactly the one DATA record and does
    /// not rewrite any other entity's argument list, so the resulting dangling `#id` reference is the
    /// honest real behaviour of a positional entity-graph removal, not hidden by a cascading delete
    /// this subset's `IfcMutation::RemoveEntity` does not itself perform either (`schema::diff::
    /// diff_remove_entity` only removes the one keyed entity — confirmed by reading that file).
    /// `set-snapshot` is pragmatic: it overrides the one header field the wave-7 scenario actually
    /// exercises (`FILE_SCHEMA`) on the already-decoded document, the same precedent this wave's
    /// `mutate-pdf-1-7`/`mutate-step-ap214` oracles use (patches known fields rather than requiring
    /// the full snapshot literal inline in a Gherkin cell).
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),

            "set-snapshot" => {
                let schemas = str_array(params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
                record.parameter = Parameter::List(vec![Parameter::List(schemas.into_iter().map(Parameter::String).collect())]);
                Ok(())
            }

            "set-file-description" => {
                let values = params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                let record = header_record_mut(exchange, "FILE_DESCRIPTION").ok_or("input carries no FILE_DESCRIPTION header record")?;
                record.parameter = Parameter::List(values);
                Ok(())
            }

            "set-file-name" => {
                let values = params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                let record = header_record_mut(exchange, "FILE_NAME").ok_or("input carries no FILE_NAME header record")?;
                record.parameter = Parameter::List(values);
                Ok(())
            }

            "set-file-schema" => {
                let values = params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                let record = header_record_mut(exchange, "FILE_SCHEMA").ok_or("input carries no FILE_SCHEMA header record")?;
                record.parameter = Parameter::List(values);
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
                let arg_index = usize_field(params, "index")?;
                let value = value_from_json(params.get("value").ok_or("set-entity-arg requires a value field")?)?;
                let section = exchange.data.first_mut().ok_or("input carries no DATA section")?;
                let entity = section.entities.iter_mut().find(|entity| entity_id(entity) == id).ok_or_else(|| format!("set-entity-arg: no entity with id {id}"))?;
                let args = args_mut(primary_record_mut(entity))?;
                *args.get_mut(arg_index).ok_or_else(|| format!("set-entity-arg: arg index {arg_index} out of range for entity {id}"))? = value;
                Ok(())
            }

            "insert-entity-arg" => {
                let id = u64_field(params, "id")?;
                let arg_index = usize_field(params, "index")?;
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
                let arg_index = usize_field(params, "index")?;
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
        let mut exchange = parse_exchange(text)?;
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
    /// parse (ruststep, never this subset's own `IfcSnapshot`/`step::engine::part21`) reads back a
    /// result before `semantic-ifc-v1` compares it: `FILE_SCHEMA` plus the full id-keyed entity graph
    /// (name, positional arguments), id-sorted for a deterministic comparison regardless of physical
    /// order.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("projection input is not UTF-8: {error}"))?;
        let exchange = parse_exchange(text)?;
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

/// 👁️ This subset's own semantic projection, re-exported at the module's public surface so the
/// case adapter can reach it as `oracle_apply_mutation`'s sibling.
#[cfg(feature = "oracles")]
pub fn project_ifc_4_any(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ifc_4_any(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Ticket 26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR wave-7 validation: exercises every declared
/// kind against the real Nakagin Capsule Tower fixture, confirming the exact ids/indices/values the
/// feature file's `Examples` tables carry are real. `cargo test --features oracles` from this
/// crate's own directory.
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::{oracle_apply_mutation, project_ifc_4_any};
    use semio_repo_test_host::Json;

    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/🏗️nakagin-capsule-tower.ifc");

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
    /// 🔎️ Whether `args` carries a `reference` to `id` anywhere, including nested inside an
    /// `aggregate`'s own `v` list (real `IFCRELAGGREGATES` args carry their member ids one level
    /// deep, inside the trailing aggregate, not as top-level positional args).
    fn args_reference(args: &[Json], id: f64) -> bool {
        args.iter().any(|value| match value.get("t") {
            Some(Json::String(t)) if t == "reference" => matches!(value.get("v"), Some(Json::Number(n)) if *n == id),
            Some(Json::String(t)) if t == "aggregate" => match value.get("v") {
                Some(Json::Array(items)) => args_reference(items, id),
                _ => false,
            },
            _ => false,
        })
    }

    #[test]
    fn parses_the_real_fixture_and_projects_it() {
        let projection = project_ifc_4_any(FIXTURE).expect("project real fixture");
        assert_eq!(entity_count(&projection), 24792.0);
        match projection.get("fileSchema") {
            Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("IFC4".to_string())]),
            other => panic!("expected fileSchema array, got {other:?}"),
        }
        let proxy = find_entity(&projection, 16976.0).expect("real capsule proxy entity #16976 present");
        assert_eq!(proxy.get("name"), Some(&Json::String("IFCBUILDINGELEMENTPROXY".to_string())));
    }

    #[test]
    fn no_mutation_round_trips_and_is_not_byte_identical() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
        assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
        let projection = project_ifc_4_any(&mutated).expect("project no-mutation result");
        assert_eq!(entity_count(&projection), 24792.0);
    }

    #[test]
    fn set_snapshot_overrides_file_schema_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC4X3")]))]))).expect("set-snapshot");
        let projection = project_ifc_4_any(&mutated).expect("project");
        assert_eq!(projection.get("fileSchema"), Some(&Json::Array(vec![text("IFC4X3")])));
        let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC4")]))]))).expect("inverse set-snapshot");
        let restored_projection = project_ifc_4_any(&restored).expect("project restored");
        assert_eq!(restored_projection.get("fileSchema"), Some(&Json::Array(vec![text("IFC4")])));
    }

    /// 🏗️ `insert-entity`/`remove-entity` on real building entities: adds a fresh
    /// `IFCCARTESIANPOINT`, then undoes it — the structural analogue of the page operations this
    /// wave is about, exercised on the real entity graph rather than a synthetic one.
    #[test]
    fn insert_and_remove_entity_round_trip_on_the_real_graph() {
        let coordinates = Json::Array(vec![tv("real", num(1000.0)), tv("real", num(2000.0)), tv("real", num(3000.0))]);
        let insert_params = obj(vec![("index", num(24792.0)), ("entity", obj(vec![("id", num(90001.0)), ("name", text("IFCCARTESIANPOINT")), ("args", Json::Array(vec![tv("aggregate", coordinates)]))]))]);
        let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity", insert_params)).expect("insert-entity");
        let projection = project_ifc_4_any(&inserted).expect("project inserted");
        assert_eq!(entity_count(&projection), 24793.0);
        assert!(find_entity(&projection, 90001.0).is_some());

        let removed = oracle_apply_mutation(&inserted, &spec("remove-entity", obj(vec![("id", num(90001.0))]))).expect("inverse remove-entity");
        let removed_projection = project_ifc_4_any(&removed).expect("project removed");
        assert_eq!(entity_count(&removed_projection), 24792.0);
        assert!(find_entity(&removed_projection, 90001.0).is_none());
    }

    /// 🏗️ Deliberately removes a real capsule proxy entity (#16976, `IFCBUILDINGELEMENTPROXY 'b'`)
    /// that `#16991`'s real `IFCRELAGGREGATES` aggregate list references by id — the "removing an
    /// entity others reference" integrity question the assignment calls out. The oracle removes only
    /// the one DATA record and leaves `#16991`'s reference dangling rather than rewriting it, which
    /// is honest: `IfcMutation::RemoveEntity`'s own production semantics
    /// (`schema::diff::diff_remove_entity`) do not cascade either.
    #[test]
    fn remove_and_reinsert_the_real_referenced_entity_16976() {
        let referencing = project_ifc_4_any(FIXTURE).unwrap();
        let before = find_entity(&referencing, 16991.0).expect("real IFCRELAGGREGATES #16991 present");
        let before_args = match before.get("args") {
            Some(Json::Array(items)) => items.clone(),
            _ => panic!("no args"),
        };
        assert!(args_reference(&before_args, 16976.0), "real #16991 must reference #16976 before removal");

        let removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity", obj(vec![("id", num(16976.0))]))).expect("remove-entity");
        let projection = project_ifc_4_any(&removed).expect("project removed");
        assert_eq!(entity_count(&projection), 24791.0);
        assert!(find_entity(&projection, 16976.0).is_none());
        let after = find_entity(&projection, 16991.0).expect("real #16991 must survive the removal of an entity it references");
        let after_args = match after.get("args") {
            Some(Json::Array(items)) => items.clone(),
            _ => panic!("no args"),
        };
        assert!(args_reference(&after_args, 16976.0), "the dangling #16976 reference must survive untouched — no cascading rewrite");

        let reinserted_params = obj(vec![
            ("index", num(16975.0)),
            (
                "entity",
                obj(vec![
                    ("id", num(16976.0)),
                    ("name", text("IFCBUILDINGELEMENTPROXY")),
                    (
                        "args",
                        Json::Array(vec![
                            tv("string", text("0POPlhUSnC1REPvcqnensi")),
                            tv("unset", Json::Object(vec![])),
                            tv("string", text("b")),
                            tv("unset", Json::Object(vec![])),
                            tv("unset", Json::Object(vec![])),
                            tv("reference", num(16996.0)),
                            tv("reference", num(16985.0)),
                            tv("unset", Json::Object(vec![])),
                            tv("unset", Json::Object(vec![])),
                        ]),
                    ),
                ]),
            ),
        ]);
        let reinserted = oracle_apply_mutation(&removed, &spec("insert-entity", reinserted_params)).expect("inverse insert-entity");
        let reinserted_projection = project_ifc_4_any(&reinserted).expect("project reinserted");
        assert_eq!(reinserted_projection, referencing, "reinserting #16976 at its original index with its original args must restore the pristine projection exactly");
    }

    #[test]
    fn set_entity_name_round_trips_on_the_real_proxy_16976() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-name", obj(vec![("id", num(16976.0)), ("name", text("RENAMED_PROXY"))]))).expect("set-entity-name");
        let projection = project_ifc_4_any(&mutated).expect("project");
        assert_eq!(find_entity(&projection, 16976.0).unwrap().get("name"), Some(&Json::String("RENAMED_PROXY".to_string())));
        let restored = oracle_apply_mutation(&mutated, &spec("set-entity-name", obj(vec![("id", num(16976.0)), ("name", text("IFCBUILDINGELEMENTPROXY"))]))).expect("inverse");
        let restored_projection = project_ifc_4_any(&restored).expect("project restored");
        assert_eq!(restored_projection, project_ifc_4_any(FIXTURE).unwrap());
    }

    #[test]
    fn entity_16976_real_args_are_as_expected() {
        let projection = project_ifc_4_any(FIXTURE).expect("project");
        let proxy = find_entity(&projection, 16976.0).expect("entity 16976");
        match proxy.get("args") {
            Some(Json::Array(items)) => {
                assert_eq!(items.len(), 9, "real IFCBUILDINGELEMENTPROXY carries 9 positional args");
                assert_eq!(items[2], tv("string", text("b")), "arg index 2 is the real Name attribute 'b'");
            }
            other => panic!("expected args array, got {other:?}"),
        }
    }

    #[test]
    fn set_insert_remove_entity_arg_round_trip_on_16976() {
        let set_mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(2.0)), ("value", tv("string", text("origin-marker")))]))).expect("set-entity-arg");
        let set_restored = oracle_apply_mutation(&set_mutated, &spec("set-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(2.0)), ("value", tv("string", text("b")))]))).expect("inverse set-entity-arg");
        assert_eq!(project_ifc_4_any(&set_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

        let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(9.0)), ("value", tv("enum", text("T")))]))).expect("insert-entity-arg");
        let projection = project_ifc_4_any(&inserted).expect("project inserted-arg");
        let args = match find_entity(&projection, 16976.0).unwrap().get("args") {
            Some(Json::Array(items)) => items.clone(),
            _ => panic!("no args"),
        };
        assert_eq!(args.len(), 10);
        assert_eq!(args[9], tv("enum", text("T")));
        let removed_back = oracle_apply_mutation(&inserted, &spec("remove-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(9.0))]))).expect("inverse insert-entity-arg");
        assert_eq!(project_ifc_4_any(&removed_back).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

        let real_removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(8.0))]))).expect("remove-entity-arg");
        let real_removed_projection = project_ifc_4_any(&real_removed).expect("project");
        let remaining_args = match find_entity(&real_removed_projection, 16976.0).unwrap().get("args") {
            Some(Json::Array(items)) => items.clone(),
            _ => panic!("no args"),
        };
        assert_eq!(remaining_args.len(), 8);
        let reinserted = oracle_apply_mutation(&real_removed, &spec("insert-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(8.0)), ("value", tv("unset", Json::Object(vec![])))]))).expect("inverse remove-entity-arg");
        assert_eq!(project_ifc_4_any(&reinserted).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
    }

    #[test]
    fn set_file_description_name_and_schema_round_trip() {
        let real_description = Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("ViewDefinition[DesignTransferView]"))])), tv("string", text("2;1"))]);
        let d =
            oracle_apply_mutation(FIXTURE, &spec("set-file-description", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("wave-7 mutation"))])), tv("string", text("2;1"))]))]))).expect("set-file-description");
        assert_ne!(project_ifc_4_any(&d).unwrap(), project_ifc_4_any(FIXTURE).unwrap(), "set-file-description must MOVE the projection -- this assertion is what caught the projection being blind to FILE_DESCRIPTION entirely");
        let d_restored = oracle_apply_mutation(&d, &spec("set-file-description", obj(vec![("values", real_description)]))).expect("inverse");
        assert_eq!(project_ifc_4_any(&d_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

        let real_name = Json::Array(vec![
            tv("string", text("/dev/null")),
            tv("string", text("2026-03-20T21:51:27+00:00")),
            tv("aggregate", Json::Array(vec![tv("string", text(""))])),
            tv("aggregate", Json::Array(vec![tv("string", text(""))])),
            tv("string", text("IfcOpenShell 0.8.4.post1")),
            tv("string", text("IfcOpenShell 0.8.4.post1")),
            tv("string", text("Nobody")),
        ]);
        let n = oracle_apply_mutation(
            FIXTURE,
            &spec(
                "set-file-name",
                obj(vec![(
                    "values",
                    Json::Array(vec![
                        tv("string", text("wave-7-mutated.ifc")),
                        tv("string", text("2026-08-23T00:00:00")),
                        tv("aggregate", Json::Array(vec![tv("string", text("Ueli"))])),
                        tv("aggregate", Json::Array(vec![tv("string", text("semio"))])),
                        tv("string", text("semio-ifc")),
                        tv("string", text("semio")),
                        tv("string", text("")),
                    ]),
                )]),
            ),
        )
        .expect("set-file-name");
        assert_ne!(project_ifc_4_any(&n).unwrap(), project_ifc_4_any(FIXTURE).unwrap(), "set-file-name must MOVE the projection");
        let n_restored = oracle_apply_mutation(&n, &spec("set-file-name", obj(vec![("values", real_name)]))).expect("inverse");
        assert_eq!(project_ifc_4_any(&n_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

        let s = oracle_apply_mutation(FIXTURE, &spec("set-file-schema", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC4X3"))]))]))]))).expect("set-file-schema");
        let s_restored = oracle_apply_mutation(&s, &spec("set-file-schema", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC4"))]))]))]))).expect("inverse");
        assert_eq!(project_ifc_4_any(&s_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
    }

    #[test]
    fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
        let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
        assert_ne!(output, FIXTURE);
        let input_projection = project_ifc_4_any(FIXTURE).unwrap();
        let output_projection = project_ifc_4_any(&output).unwrap();
        assert_eq!(input_projection, output_projection);
    }

    #[test]
    fn unknown_kind_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
    }
}
//#endregion 🧪️Tests
