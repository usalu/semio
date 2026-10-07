//! 📝️ Complete six-root owned DXF snapshot persistence.
use crate::standards::v_r12::subsets::any::schema::snapshot::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use pack::value::{DslValue,FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio";
const FIELDS:[(u16,&str,semio_framework_dsl_record::Shape,bool);6]=[
    (1,"schema",semio_framework_dsl_record::Shape::Text,false),
    (2,"headerVars",semio_framework_dsl_record::Shape::Value,false),
    (3,"tables",semio_framework_dsl_record::Shape::Value,false),
    (4,"otherTables",semio_framework_dsl_record::Shape::Value,false),
    (5,"blocks",semio_framework_dsl_record::Shape::Value,false),
    (6,"entities",semio_framework_dsl_record::Shape::Value,false),
];
pub(crate) fn spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=semio_framework_dsl_record::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(crate) fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(crate) fn to_record(snapshot:&DxfSnapshot)->semio_framework_dsl_record::RecordValue{
    use semio_framework_dsl_record::FieldValue as V;
    semio_framework_dsl_record::RecordValue{fields:[(1,semio_framework_dsl_record::FieldValue::Text(snapshot.schema.clone())),(2,semio_framework_dsl_record::FieldValue::Value(snapshot.header_vars.to_value())),(3,semio_framework_dsl_record::FieldValue::Value(snapshot.tables.to_value())),(4,semio_framework_dsl_record::FieldValue::Value(snapshot.other_tables.to_value())),(5,semio_framework_dsl_record::FieldValue::Value(snapshot.blocks.to_value())),(6,semio_framework_dsl_record::FieldValue::Value(snapshot.entities.to_value()))].into_iter().collect()}
}

pub(crate) fn to_record_controlled(snapshot:&DxfSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,ValueError>{
        control.begin_stage(6)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(6,control)?;
        record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&snapshot.schema)?))?;control.step()?;
        record.insert(2,project(&snapshot.header_vars,control)?)?;control.step()?;
        record.insert(3,project(&snapshot.tables,control)?)?;control.step()?;
        record.insert(4,project(&snapshot.other_tables,control)?)?;control.step()?;
        record.insert(5,project(&snapshot.blocks,control)?)?;control.step()?;
        record.insert(6,project(&snapshot.entities,control)?)?;control.step()?;
        Ok(record.take())
    }))
}
fn project<T:ToValue>(value:&T,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(semio_framework_dsl_record::FieldValue::Value)})}
pub(crate) fn from_record(record:&semio_framework_dsl_record::RecordValue)->Result<DxfSnapshot,semio_framework_diagnostic::TextError>{
    if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DXF snapshot contains an undeclared root field",semio_framework_diagnostic::TextSpan::at(1,1)));}
    let mut fields=Vec::with_capacity(6);
    for(id,key)in[(1,"schema"),(2,"headerVars"),(3,"tables"),(4,"otherTables"),(5,"blocks"),(6,"entities")]{
        let value=match record.get(id){Some(semio_framework_dsl_record::FieldValue::Text(value))if id==1=>DslValue::String(value.clone()),Some(semio_framework_dsl_record::FieldValue::Value(value))if id>1=>value.clone(),_=>return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))};
        fields.push((key.into(),value));
    }
    DxfSnapshot::from_value(DslValue::Object(fields)).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))
}

fn owned<T:FromValue>(record:&semio_framework_dsl_record::RecordValue,id:u16,key:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,ValueError>{
    let Some(semio_framework_dsl_record::FieldValue::Value(value))=record.get(id)else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape")))};
    let value=T::from_value_controlled(value,control).map_err(|error|error.under(key))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step()?;Ok(owner)
}
pub(crate) fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<DxfSnapshot,ValueError>{
    control.scoped_stage(|control|->Result<_,ValueError>{
        control.begin_stage(6)?;if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, "DXF snapshot contains an undeclared root field"));}
        let Some(semio_framework_dsl_record::FieldValue::Text(schema))=record.get(1)else{return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, "DXF snapshot schema is missing or has a different shape"));};let schema=control.copy_text(schema)?;control.step()?;
        let header_vars=owned::<Vec<DxfHeaderVar>>(record,2,"headerVars",control)?;let tables=owned::<DxfTables>(record,3,"tables",control)?;let other_tables=owned::<Vec<DxfOtherTable>>(record,4,"otherTables",control)?;let blocks=owned::<Vec<DxfBlock>>(record,5,"blocks",control)?;let entities=owned::<Vec<DxfEntity>>(record,6,"entities",control)?;
        Ok(DxfSnapshot{schema,header_vars:header_vars.take(),tables:tables.take(),other_tables:other_tables.take(),blocks:blocks.take(),entities:entities.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::snapshot::*;
use crate::STDIO_DXF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::standards::v_r12::subsets::any::io::text::snapshot as snapshot_text;

impl store::ArtifactDsl for DxfSnapshot {
    const EXTENSION: &'static str = "dxf";
    fn envelope_id() -> &'static str {
        "stdio.dxf"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DXF snapshot text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
                rest
            }
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &snapshot_text::spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 272 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document })?;
        snapshot_text::from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&snapshot_text::to_record(self), &snapshot_text::spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::snapshot::*;
use crate::STDIO_DXF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use crate::standards::v_r12::subsets::any::io::text::snapshot as snapshot_text;
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_block, dec_dxf_entities, dec_header_var, dec_list, dec_str, enc_block, enc_dxf_entities, enc_header_var, enc_list, enc_str, split_top_level, strip_brackets};

/// 🧭️ Simplification of the DXF group-code value-type table (spec appendix) into the four
/// `DxfValue` kinds — good enough for every code this codec reads generically (unknown-group-code
/// retention, `Other` fallbacks); codes with dedicated typed fields (10/20/30 point triplets on
/// known entities, etc.) are parsed directly by their own field-specific logic instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn classify_group_code_value(code: i32, raw: &str) -> DxfValue {
    match code {
        10..=59 | 110..=149 | 210..=239 | 460..=469 => DxfValue::Double { value: parse_f64(raw) },
        60..=99 | 160..=179 | 270..=289 | 370..=389 | 400..=409 | 440..=459 => DxfValue::Int { value: parse_i64(raw) },
        _ => DxfValue::Str { value: raw.to_string() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_dxf_value(v: &DxfValue) -> String {
    match v {
        DxfValue::Str { value } => value.clone(),
        DxfValue::Int { value } => value.to_string(),
        DxfValue::Double { value } => format_f64(*value),
        DxfValue::Point { value } => format_f64(value[0]),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(v: &str) -> f64 {
    v.trim().parse::<f64>().unwrap_or(0.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_i64(v: &str) -> i64 {
    v.trim().parse::<i64>().unwrap_or_else(|_| parse_f64(v) as i64)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_f64(v: f64) -> String {
    let s = format!("{v}");
    if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("NaN") {
        s
    } else {
        format!("{s}.0")
    }
}

/// 📥️ Tokenizes raw DXF ASCII text into its flat `(code, value)` tag stream — the tokenizer's
/// output is consumed immediately by the section walker below; it is never itself the source of
/// truth (contrast with the pre-overhaul model).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tokenize_dxf(text: &str) -> Result<Vec<DxfTag>, String> {
    let raw: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut tags = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        let code_line = raw[i].trim();
        if code_line.is_empty() {
            i += 1;
            continue;
        }
        let value = raw.get(i + 1).ok_or_else(|| format!("dxf: group code {code_line:?} missing its value line"))?;
        let code: i32 = code_line.parse().map_err(|e| format!("dxf: invalid group code {code_line:?}: {e}"))?;
        tags.push(DxfTag { code, value: value.trim().to_string() });
        i += 2;
    }
    Ok(tags)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn push_tag(out: &mut String, code: i32, value: &str) {
    out.push_str(&code.to_string());
    out.push('\n');
    out.push_str(value);
    out.push('\n');
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_header_var(name: String, tags: &[DxfTag]) -> DxfHeaderVar {
    if tags.is_empty() {
        return DxfHeaderVar { name, group_code: 0, value: DxfValue::default(), extra_group_codes: Vec::new() };
    }
    // 🧭️ Point-component detection: an adjacent 3-code (or 2-code, z=0) run at +10/+20 offsets
    // from the primary code (the DXF convention for $INSBASE/$EXTMIN/$EXTMAX/… point vars).
    if tags.len() >= 3 && tags[1].code == tags[0].code + 10 && tags[2].code == tags[0].code + 20 {
        let value = DxfValue::Point { value: [parse_f64(&tags[0].value), parse_f64(&tags[1].value), parse_f64(&tags[2].value)] };
        let extra = tags[3..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
        return DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra };
    }
    if tags.len() >= 2 && tags[1].code == tags[0].code + 10 {
        let value = DxfValue::Point { value: [parse_f64(&tags[0].value), parse_f64(&tags[1].value), 0.0] };
        let extra = tags[2..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
        return DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra };
    }
    let value = classify_group_code_value(tags[0].code, &tags[0].value);
    let extra = tags[1..].iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect();
    DxfHeaderVar { name, group_code: tags[0].code, value, extra_group_codes: extra }
}

/// 📥️ Parses a `HEADER` section body (tags strictly between `0/SECTION,2/HEADER` and
/// `0/ENDSEC`, exclusive of both).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_header_section(tags: &[DxfTag]) -> Vec<DxfHeaderVar> {
    let mut vars = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 9 {
            let name = tags[i].value.clone();
            let start = i + 1;
            let mut end = start;
            while end < tags.len() && tags[end].code != 9 {
                end += 1;
            }
            vars.push(parse_header_var(name, &tags[start..end]));
            i = end;
        } else {
            i += 1;
        }
    }
    vars
}

/// 📤️ Value pairs a header var's `group_code`/`value` expand to on print — a `Point` expands
/// into the code/code+10/code+20 triplet convention; everything else is a single pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn header_var_value_pairs(code: i32, value: &DxfValue) -> Vec<(i32, String)> {
    match value {
        DxfValue::Point { value } => vec![(code, format_f64(value[0])), (code + 10, format_f64(value[1])), (code + 20, format_f64(value[2]))],
        other => vec![(code, format_dxf_value(other))],
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_header_section(vars: &[DxfHeaderVar], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "HEADER");
    for v in vars {
        push_tag(out, 9, &v.name);
        for (code, s) in header_var_value_pairs(v.group_code, &v.value) {
            push_tag(out, code, &s);
        }
        for (code, val) in &v.extra_group_codes {
            push_tag(out, *code, &format_dxf_value(val));
        }
    }
    push_tag(out, 0, "ENDSEC");
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_layer(body: &[DxfTag]) -> DxfLayer {
    let mut layer = DxfLayer::default();
    for t in body {
        match t.code {
            2 => layer.name = t.value.clone(),
            62 => layer.color = parse_i64(&t.value) as i32,
            6 => layer.linetype = t.value.clone(),
            70 => layer.flags = parse_i64(&t.value) as i32,
            _ => layer.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    layer
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_style(body: &[DxfTag]) -> DxfStyle {
    let mut style = DxfStyle::default();
    for t in body {
        match t.code {
            2 => style.name = t.value.clone(),
            70 => style.flags = parse_i64(&t.value) as i32,
            3 => style.font_name = t.value.clone(),
            _ => style.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    style
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_linetype(body: &[DxfTag]) -> DxfLinetype {
    let mut lt = DxfLinetype::default();
    for t in body {
        match t.code {
            2 => lt.name = t.value.clone(),
            70 => lt.flags = parse_i64(&t.value) as i32,
            3 => lt.description = t.value.clone(),
            _ => lt.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    lt
}

/// 🔎 Splits a table's entry body (between `2/<TABLENAME>` and `0/ENDTAB`, table-level fields
/// like `70`/count already skipped by the caller) into per-entry `(0/<ENTRYKIND> … )` slices.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_table_entries<'a>(tags: &'a [DxfTag], entry_kind: &str) -> Vec<&'a [DxfTag]> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == entry_kind {
            let start = i + 1;
            let mut end = start;
            while end < tags.len() && tags[end].code != 0 {
                end += 1;
            }
            out.push(&tags[start..end]);
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// 📥️ Parses a `TABLES` section body. Returns the three typed table kinds plus raw-retained
/// entries for every other table kind (VPORT/VIEW/UCS/APPID/DIMSTYLE/BLOCK_RECORD/…).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_tables_section(tags: &[DxfTag]) -> (DxfTables, Vec<DxfOtherTable>) {
    let mut tables = DxfTables::default();
    let mut others = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "TABLE" {
            i += 1;
            let table_name = if i < tags.len() && tags[i].code == 2 {
                let n = tags[i].value.clone();
                i += 1;
                n
            } else {
                String::new()
            };
            let raw_body_start = i;
            let mut body_end = raw_body_start;
            while body_end < tags.len() && !(tags[body_end].code == 0 && tags[body_end].value == "ENDTAB") {
                body_end += 1;
            }
            match table_name.as_str() {
                "LAYER" | "STYLE" | "LTYPE" => {
                    // 🧭️ Known kinds: skip table-level fields (70 count, 5 handle, …) up to the
                    // first entry's `0/<KIND>` marker before splitting into per-entry slices.
                    let mut entries_start = raw_body_start;
                    while entries_start < body_end && tags[entries_start].code != 0 {
                        entries_start += 1;
                    }
                    let body = &tags[entries_start..body_end];
                    match table_name.as_str() {
                        "LAYER" => tables.layers = split_table_entries(body, "LAYER").into_iter().map(build_layer).collect(),
                        "STYLE" => tables.styles = split_table_entries(body, "STYLE").into_iter().map(build_style).collect(),
                        "LTYPE" => tables.linetypes = split_table_entries(body, "LTYPE").into_iter().map(build_linetype).collect(),
                        _ => unreachable!(),
                    }
                }
                // 🕳️ Unknown kinds: capture the WHOLE raw span (informational fields AND any
                // entry markers alike) verbatim — no skip, so hand-built and real-parsed
                // `DxfOtherTable.tags` are both trivially lossless round-trip fixed points.
                _ => others.push(DxfOtherTable { name: table_name, tags: tags[raw_body_start..body_end].to_vec() }),
            }
            i = body_end;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDTAB" {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    (tables, others)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_layer(out: &mut String, l: &DxfLayer) {
    push_tag(out, 0, "LAYER");
    push_tag(out, 2, &l.name);
    push_tag(out, 70, &l.flags.to_string());
    push_tag(out, 62, &l.color.to_string());
    push_tag(out, 6, &l.linetype);
    for (code, v) in &l.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_style(out: &mut String, s: &DxfStyle) {
    push_tag(out, 0, "STYLE");
    push_tag(out, 2, &s.name);
    push_tag(out, 70, &s.flags.to_string());
    push_tag(out, 3, &s.font_name);
    for (code, v) in &s.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_linetype(out: &mut String, l: &DxfLinetype) {
    push_tag(out, 0, "LTYPE");
    push_tag(out, 2, &l.name);
    push_tag(out, 70, &l.flags.to_string());
    push_tag(out, 3, &l.description);
    for (code, v) in &l.unknown_group_codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_table_block(out: &mut String, name: &str, count: usize, mut body: impl FnMut(&mut String)) {
    push_tag(out, 0, "TABLE");
    push_tag(out, 2, name);
    push_tag(out, 70, &count.to_string());
    body(out);
    push_tag(out, 0, "ENDTAB");
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_tables_section(tables: &DxfTables, others: &[DxfOtherTable], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "TABLES");
    // 📐️ LTYPE MUST precede LAYER. A LAYER record names its linetype by string (group code 6), so
    // a reader that meets the LAYER table first has to invent the linetype it cannot yet resolve —
    // which is exactly what the registered `dxf` 0.6 reference reader does: fed this writer's
    // former LAYER-then-LTYPE order it reported EIGHT linetypes for the real `🚏️bus-shelter`
    // drawing, an extra `CONTINUOUS` ahead of the real seven, and all 39 `📰️mutate-dxf-r12` parity
    // comparisons diverged on `$.linetypes` alone. Confirmed by experiment rather than inferred:
    // reordering nothing but these two blocks in the very bytes this writer had produced took the
    // same reader from eight linetypes back to the real seven. The order below is the AutoCAD DXF
    // reference's own table order for R12, of which that constraint is the load-bearing part.
    print_table_block(out, "LTYPE", tables.linetypes.len(), |out| {
        for l in &tables.linetypes {
            print_linetype(out, l);
        }
    });
    print_table_block(out, "LAYER", tables.layers.len(), |out| {
        for l in &tables.layers {
            print_layer(out, l);
        }
    });
    print_table_block(out, "STYLE", tables.styles.len(), |out| {
        for s in &tables.styles {
            print_style(out, s);
        }
    });
    for t in others {
        push_tag(out, 0, "TABLE");
        push_tag(out, 2, &t.name);
        for tag in &t.tags {
            push_tag(out, tag.code, &tag.value);
        }
        push_tag(out, 0, "ENDTAB");
    }
    push_tag(out, 0, "ENDSEC");
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_vertex(body: &[DxfTag]) -> DxfVertex {
    let mut v = DxfVertex::default();
    for t in body {
        match t.code {
            10 => v.x = parse_f64(&t.value),
            20 => v.y = parse_f64(&t.value),
            30 => v.z = parse_f64(&t.value),
            42 => v.bulge = parse_f64(&t.value),
            _ => v.unknown_group_codes.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    v
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn build_entity(kind: &str, body: &[DxfTag]) -> DxfEntity {
    match kind {
        "LINE" => {
            let (mut start, mut end, mut layer, mut unknown) = ([0.0; 3], [0.0; 3], String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => start[0] = parse_f64(&t.value),
                    20 => start[1] = parse_f64(&t.value),
                    30 => start[2] = parse_f64(&t.value),
                    11 => end[0] = parse_f64(&t.value),
                    21 => end[1] = parse_f64(&t.value),
                    31 => end[2] = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Line { start, end, layer, unknown_group_codes: unknown }
        }
        "CIRCLE" => {
            let (mut center, mut radius, mut layer, mut unknown) = ([0.0; 3], 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => center[0] = parse_f64(&t.value),
                    20 => center[1] = parse_f64(&t.value),
                    30 => center[2] = parse_f64(&t.value),
                    40 => radius = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Circle { center, radius, layer, unknown_group_codes: unknown }
        }
        "ARC" => {
            let (mut center, mut radius, mut sa, mut ea, mut layer, mut unknown) = ([0.0; 3], 0.0, 0.0, 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => center[0] = parse_f64(&t.value),
                    20 => center[1] = parse_f64(&t.value),
                    30 => center[2] = parse_f64(&t.value),
                    40 => radius = parse_f64(&t.value),
                    50 => sa = parse_f64(&t.value),
                    51 => ea = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Arc { center, radius, start_angle: sa, end_angle: ea, layer, unknown_group_codes: unknown }
        }
        "TEXT" => {
            let (mut position, mut height, mut value, mut layer, mut unknown) = ([0.0; 3], 0.0, String::new(), String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => position[0] = parse_f64(&t.value),
                    20 => position[1] = parse_f64(&t.value),
                    30 => position[2] = parse_f64(&t.value),
                    40 => height = parse_f64(&t.value),
                    1 => value = t.value.clone(),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Text { position, height, value, layer, unknown_group_codes: unknown }
        }
        "SOLID" => {
            let (mut points, mut layer, mut unknown) = ([[0.0; 3]; 4], String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    10 => points[0][0] = parse_f64(&t.value),
                    20 => points[0][1] = parse_f64(&t.value),
                    30 => points[0][2] = parse_f64(&t.value),
                    11 => points[1][0] = parse_f64(&t.value),
                    21 => points[1][1] = parse_f64(&t.value),
                    31 => points[1][2] = parse_f64(&t.value),
                    12 => points[2][0] = parse_f64(&t.value),
                    22 => points[2][1] = parse_f64(&t.value),
                    32 => points[2][2] = parse_f64(&t.value),
                    13 => points[3][0] = parse_f64(&t.value),
                    23 => points[3][1] = parse_f64(&t.value),
                    33 => points[3][2] = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Solid { points, layer, unknown_group_codes: unknown }
        }
        "INSERT" => {
            let (mut block_name, mut position, mut scale, mut rotation, mut layer, mut unknown) = (String::new(), [0.0; 3], [1.0, 1.0, 1.0], 0.0, String::new(), Vec::new());
            for t in body {
                match t.code {
                    8 => layer = t.value.clone(),
                    2 => block_name = t.value.clone(),
                    10 => position[0] = parse_f64(&t.value),
                    20 => position[1] = parse_f64(&t.value),
                    30 => position[2] = parse_f64(&t.value),
                    41 => scale[0] = parse_f64(&t.value),
                    42 => scale[1] = parse_f64(&t.value),
                    43 => scale[2] = parse_f64(&t.value),
                    50 => rotation = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes: unknown }
        }
        _ => DxfEntity::Other { kind: kind.to_string(), group_codes: body.iter().map(|t| (t.code, classify_group_code_value(t.code, &t.value))).collect() },
    }
}

/// 📥️ Parses the real R12 `POLYLINE`/`VERTEX`.../`SEQEND` record group. `i` points just past
/// the `0/POLYLINE` header tag; returns the built entity plus the index just past `SEQEND`'s
/// own (usually empty) body.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_polyline(tags: &[DxfTag], mut i: usize) -> (DxfEntity, usize) {
    let header_start = i;
    let mut header_end = header_start;
    while header_end < tags.len() && tags[header_end].code != 0 {
        header_end += 1;
    }
    let (mut layer, mut closed, mut unknown) = (String::new(), false, Vec::new());
    for t in &tags[header_start..header_end] {
        match t.code {
            8 => layer = t.value.clone(),
            70 => closed = parse_i64(&t.value) & 1 == 1,
            66 => {} // "entities follow" flag — implicit in this model, not retained
            _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
        }
    }
    i = header_end;
    let mut vertices = Vec::new();
    while i < tags.len() && tags[i].code == 0 && tags[i].value == "VERTEX" {
        i += 1;
        let vstart = i;
        let mut vend = vstart;
        while vend < tags.len() && tags[vend].code != 0 {
            vend += 1;
        }
        vertices.push(build_vertex(&tags[vstart..vend]));
        i = vend;
    }
    if i < tags.len() && tags[i].code == 0 && tags[i].value == "SEQEND" {
        i += 1;
        while i < tags.len() && tags[i].code != 0 {
            i += 1;
        }
    }
    (DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes: unknown }, i)
}

/// 📥️ Consumes entities from `i` until `(0, stop_kind)` (exclusive) or end of `tags`. Used both
/// for a pre-sliced `ENTITIES` section body (`stop_kind` inert, loop ends at `tags.len()`) and
/// for a block's nested entity list within the unsliced `BLOCKS` section body (`stop_kind =
/// "ENDBLK"`, since blocks are sequential and each one's extent must be discovered, not sliced
/// up front).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_entities_until(tags: &[DxfTag], mut i: usize, stop_kind: &str) -> (Vec<DxfEntity>, usize) {
    let mut entities = Vec::new();
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == stop_kind {
            break;
        }
        if tags[i].code != 0 {
            i += 1; // defensive skip of a stray non-header tag
            continue;
        }
        let kind = tags[i].value.clone();
        i += 1;
        if kind == "POLYLINE" {
            let (entity, next_i) = parse_polyline(tags, i);
            entities.push(entity);
            i = next_i;
        } else {
            let body_start = i;
            let mut body_end = body_start;
            while body_end < tags.len() && tags[body_end].code != 0 {
                body_end += 1;
            }
            entities.push(build_entity(&kind, &tags[body_start..body_end]));
            i = body_end;
        }
    }
    (entities, i)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_unknown(out: &mut String, codes: &[(i32, DxfValue)]) {
    for (code, v) in codes {
        push_tag(out, *code, &format_dxf_value(v));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_entity(e: &DxfEntity, out: &mut String) {
    match e {
        DxfEntity::Line { start, end, layer, unknown_group_codes } => {
            push_tag(out, 0, "LINE");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(start[0]));
            push_tag(out, 20, &format_f64(start[1]));
            push_tag(out, 30, &format_f64(start[2]));
            push_tag(out, 11, &format_f64(end[0]));
            push_tag(out, 21, &format_f64(end[1]));
            push_tag(out, 31, &format_f64(end[2]));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Circle { center, radius, layer, unknown_group_codes } => {
            push_tag(out, 0, "CIRCLE");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(center[0]));
            push_tag(out, 20, &format_f64(center[1]));
            push_tag(out, 30, &format_f64(center[2]));
            push_tag(out, 40, &format_f64(*radius));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Arc { center, radius, start_angle, end_angle, layer, unknown_group_codes } => {
            push_tag(out, 0, "ARC");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(center[0]));
            push_tag(out, 20, &format_f64(center[1]));
            push_tag(out, 30, &format_f64(center[2]));
            push_tag(out, 40, &format_f64(*radius));
            push_tag(out, 50, &format_f64(*start_angle));
            push_tag(out, 51, &format_f64(*end_angle));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Polyline { vertices, closed, layer, unknown_group_codes } => {
            push_tag(out, 0, "POLYLINE");
            push_tag(out, 8, layer);
            push_tag(out, 66, "1");
            push_tag(out, 70, if *closed { "1" } else { "0" });
            print_unknown(out, unknown_group_codes);
            for v in vertices {
                push_tag(out, 0, "VERTEX");
                // 🧭️ No hardcoded `8/<layer>` here: a real vertex's own `8` code (if present in
                // the source — it's optional, inheriting the polyline's layer when absent) is
                // already captured in `unknown_group_codes` by `build_vertex`; emitting it again
                // here would duplicate the tag on decode(encode(...)).
                push_tag(out, 10, &format_f64(v.x));
                push_tag(out, 20, &format_f64(v.y));
                push_tag(out, 30, &format_f64(v.z));
                push_tag(out, 42, &format_f64(v.bulge));
                print_unknown(out, &v.unknown_group_codes);
            }
            push_tag(out, 0, "SEQEND");
        }
        DxfEntity::Text { position, height, value, layer, unknown_group_codes } => {
            push_tag(out, 0, "TEXT");
            push_tag(out, 8, layer);
            push_tag(out, 10, &format_f64(position[0]));
            push_tag(out, 20, &format_f64(position[1]));
            push_tag(out, 30, &format_f64(position[2]));
            push_tag(out, 40, &format_f64(*height));
            push_tag(out, 1, value);
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Solid { points, layer, unknown_group_codes } => {
            push_tag(out, 0, "SOLID");
            push_tag(out, 8, layer);
            let codes: [(i32, i32, i32); 4] = [(10, 20, 30), (11, 21, 31), (12, 22, 32), (13, 23, 33)];
            for (idx, (cx, cy, cz)) in codes.iter().enumerate() {
                push_tag(out, *cx, &format_f64(points[idx][0]));
                push_tag(out, *cy, &format_f64(points[idx][1]));
                push_tag(out, *cz, &format_f64(points[idx][2]));
            }
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Insert { block_name, position, scale, rotation, layer, unknown_group_codes } => {
            push_tag(out, 0, "INSERT");
            push_tag(out, 8, layer);
            push_tag(out, 2, block_name);
            push_tag(out, 10, &format_f64(position[0]));
            push_tag(out, 20, &format_f64(position[1]));
            push_tag(out, 30, &format_f64(position[2]));
            push_tag(out, 41, &format_f64(scale[0]));
            push_tag(out, 42, &format_f64(scale[1]));
            push_tag(out, 43, &format_f64(scale[2]));
            push_tag(out, 50, &format_f64(*rotation));
            print_unknown(out, unknown_group_codes);
        }
        DxfEntity::Other { kind, group_codes } => {
            push_tag(out, 0, kind);
            print_unknown(out, group_codes);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_entities(entities: &[DxfEntity], out: &mut String) {
    for e in entities {
        print_entity(e, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_blocks_section(tags: &[DxfTag]) -> Vec<DxfBlock> {
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "BLOCK" {
            i += 1;
            let header_start = i;
            let mut header_end = header_start;
            while header_end < tags.len() && tags[header_end].code != 0 {
                header_end += 1;
            }
            let (mut name, mut bx, mut by, mut bz, mut unknown) = (String::new(), 0.0, 0.0, 0.0, Vec::new());
            for t in &tags[header_start..header_end] {
                match t.code {
                    2 => name = t.value.clone(),
                    10 => bx = parse_f64(&t.value),
                    20 => by = parse_f64(&t.value),
                    30 => bz = parse_f64(&t.value),
                    _ => unknown.push((t.code, classify_group_code_value(t.code, &t.value))),
                }
            }
            i = header_end;
            let (entities, next_i) = parse_entities_until(tags, i, "ENDBLK");
            i = next_i;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDBLK" {
                i += 1;
                while i < tags.len() && tags[i].code != 0 {
                    i += 1;
                }
            }
            blocks.push(DxfBlock { name, base_point: [bx, by, bz], entities, unknown_group_codes: unknown });
        } else {
            i += 1;
        }
    }
    blocks
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_blocks_section(blocks: &[DxfBlock], out: &mut String) {
    push_tag(out, 0, "SECTION");
    push_tag(out, 2, "BLOCKS");
    for b in blocks {
        push_tag(out, 0, "BLOCK");
        push_tag(out, 2, &b.name);
        push_tag(out, 10, &format_f64(b.base_point[0]));
        push_tag(out, 20, &format_f64(b.base_point[1]));
        push_tag(out, 30, &format_f64(b.base_point[2]));
        print_unknown(out, &b.unknown_group_codes);
        print_entities(&b.entities, out);
        push_tag(out, 0, "ENDBLK");
    }
    push_tag(out, 0, "ENDSEC");
}

/// 📥️ Parses a complete R12 ASCII document: `HEADER`, `TABLES`, `BLOCKS`, `ENTITIES` sections
/// (the full R12 section set — R12 predates `CLASSES`/`OBJECTS`/thumbnails), terminated by `0/EOF`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_dxf_document(text: &str) -> Result<DxfSnapshot, String> {
    let tags = tokenize_dxf(text)?;
    let mut snap = DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), ..DxfSnapshot::default() };
    let mut i = 0usize;
    while i < tags.len() {
        if tags[i].code == 0 && tags[i].value == "SECTION" {
            i += 1;
            let section_name = if i < tags.len() && tags[i].code == 2 {
                let n = tags[i].value.clone();
                i += 1;
                n
            } else {
                String::new()
            };
            let body_start = i;
            let mut body_end = body_start;
            while body_end < tags.len() && !(tags[body_end].code == 0 && tags[body_end].value == "ENDSEC") {
                body_end += 1;
            }
            let body = &tags[body_start..body_end];
            match section_name.as_str() {
                "HEADER" => snap.header_vars = parse_header_section(body),
                "TABLES" => {
                    let (t, o) = parse_tables_section(body);
                    snap.tables = t;
                    snap.other_tables = o;
                }
                "BLOCKS" => snap.blocks = parse_blocks_section(body),
                "ENTITIES" => {
                    let (e, _) = parse_entities_until(body, 0, "ENDSEC");
                    snap.entities = e;
                }
                _ => {} // R12 has no other section kinds
            }
            i = body_end;
            if i < tags.len() && tags[i].code == 0 && tags[i].value == "ENDSEC" {
                i += 1;
            }
        } else if tags[i].code == 0 && tags[i].value == "EOF" {
            break;
        } else {
            i += 1;
        }
    }
    Ok(snap)
}

/// 📤️ Regenerates canonical R12 ASCII text from the typed model — the documented NORMAL FORM
/// (see module docs): semantic content is fully preserved; incidental source formatting is not.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_dxf_document(snap: &DxfSnapshot) -> String {
    let mut out = String::new();
    print_header_section(&snap.header_vars, &mut out);
    print_tables_section(&snap.tables, &snap.other_tables, &mut out);
    print_blocks_section(&snap.blocks, &mut out);
    push_tag(&mut out, 0, "SECTION");
    push_tag(&mut out, 2, "ENTITIES");
    print_entities(&snap.entities, &mut out);
    push_tag(&mut out, 0, "ENDSEC");
    push_tag(&mut out, 0, "EOF");
    out
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::standards::v_r12::subsets::any::io::text::diff::{dec_block, dec_dxf_entities, dec_dxf_tables, dec_header_var, dec_list, dec_other_table, dec_str, enc_block, enc_dxf_entities, enc_dxf_tables, enc_header_var, enc_list, enc_other_table, enc_str, split_top_level, strip_brackets};
/// 🧬️ Whole `DxfSnapshot` — needed by `🧬️mutations::DxfMutation::SetSnapshot`'s `OpText`/
/// `OpBinary` payload (§3a's mutation-side blocker: `SetSnapshot` always carries the whole
/// snapshot, so this grammar is exercised by the mutation codec even though `DxfDiff` never
/// embeds a full snapshot itself).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_dxf_snapshot(s: &DxfSnapshot) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&s.schema), enc_list(&s.header_vars, enc_header_var), enc_dxf_tables(&s.tables), enc_list(&s.other_tables, enc_other_table), enc_list(&s.blocks, enc_block), enc_dxf_entities(&s.entities),)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_dxf_snapshot(s: &str) -> Result<DxfSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, header_vars, tables, other_tables, blocks, entities] = parts.as_slice() else {
        return Err(format!("snapshot: expected 6 fields, got {}", parts.len()));
    };
    Ok(DxfSnapshot {
        schema: dec_str(schema)?,
        header_vars: dec_list(header_vars, dec_header_var)?,
        tables: dec_dxf_tables(tables)?,
        other_tables: dec_list(other_tables, dec_other_table)?,
        blocks: dec_list(blocks, dec_block)?,
        entities: dec_dxf_entities(entities)?,
    })
}
}
pub use diff_codec::*;
