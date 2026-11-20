//! 🧾️ JSON Schema projection for declarative DSL records.

use crate::{RecordSpec,Shape};
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema §3.2: the
// gateway's JSON Schema derivation is primarily `ActionArgDef::json_schema()` (manifest-declared
// args); THIS is the fallback for `app_commands!` payload structs whose only declared shape is a
// `RecordSpec` (`#[derive(dsl::DslRecord)]`) — the catalog compiler tags whatever it emits from here
// `x-semio-confidence: "payload"`, not this module's concern.
use semio_framework_pack_json::{Object,Value};

fn typed_schema<const N:usize>(kind:&str,fields:[(&str,Value);N])->Object{
    let mut object=Object::new();
    object.insert("type",Value::from(kind));
    for (key,value) in fields{object.insert(key,value);}
    object
}

fn schema<const N:usize>(kind:&str,fields:[(&str,Value);N])->Value{
    Value::Object(typed_schema(kind,fields))
}

fn fixed_number_array(min:u8,max:u8,shape:&str)->Value{
    schema("array",[
        ("items",schema("number",[])),
        ("minItems",Value::from(u64::from(min))),
        ("maxItems",Value::from(u64::from(max))),
        ("x-semio-shape",Value::from(shape)),
    ])
}

/// 🧬️ JSON Schema 2020-12 for one `Shape` leaf/node — recurses through `Tuple`/`List`/
/// `Record`/`Block`/`Statements`/`Map`/`Table`. `Quantity`/`Angle` carry their unit as
/// `x-semio-unit`; `Ref(kind)` carries the referenced entity kind as `x-semio-ref`; every shape with
/// no native JSON Schema vocabulary (`Bytes64`/`Wire`/`Coord`/`Dir`/`Dim`/`Range`/`Count`/`Expr`/
/// `Embed`/`EmbedFrom`) additionally carries `x-semio-shape` naming the exact `Shape` variant.
pub fn shape_json_schema(shape:&Shape)->Value{
    match shape{
        Shape::Bool=>schema("boolean",[]),
        Shape::Int=>schema("integer",[]),
        Shape::UInt=>schema("integer",[("minimum",Value::from(0u64))]),
        Shape::Float=>schema("number",[]),
        Shape::Text=>schema("string",[]),
        Shape::Bytes64=>schema("string",[("contentEncoding",Value::from("base64")),("x-semio-shape",Value::from("bytes64"))]),
        Shape::Enum(variants)=>schema("string",[("enum",Value::Array(variants.iter().map(|(tag,_)|Value::from(tag.clone())).collect()))]),
        Shape::Tuple(inner,len)=>{
            let mut object=typed_schema("array",[("items",shape_json_schema(inner))]);
            if let Some(len)=len{
                object.insert("minItems",Value::from(*len as u64));
                object.insert("maxItems",Value::from(*len as u64));
            }
            Value::Object(object)
        },
        Shape::List(inner)=>schema("array",[("items",shape_json_schema(inner))]),
        Shape::Record(spec)=>record_spec_json_schema(&(spec.ordinary)()),
        Shape::Block(inner)=>shape_json_schema(inner),
        Shape::Statements(variants)=>{
            let options=variants.iter().map(|(keyword,spec)|{
                let mut entry=record_spec_json_schema(&(spec.ordinary)());
                if let Value::Object(object)=&mut entry{object.insert("x-semio-keyword",Value::from(keyword.clone()));}
                entry
            }).collect();
            let mut items=Object::new();
            items.insert("oneOf",Value::Array(options));
            schema("array",[("items",Value::Object(items))])
        },
        Shape::Map(inner)=>schema("object",[("additionalProperties",shape_json_schema(inner))]),
        Shape::Value=>Value::Object(Object::new()),
        Shape::Table(spec)=>schema("array",[("items",record_spec_json_schema(&(spec.ordinary)()))]),
        Shape::Wire=>schema("string",[("x-semio-shape",Value::from("wire"))]),
        Shape::Quantity(unit)=>schema("number",[("x-semio-unit",Value::from(unit.symbol))]),
        Shape::Angle(unit)=>schema("number",[("x-semio-unit",Value::from(unit.symbol)),("x-semio-shape",Value::from("angle"))]),
        Shape::Ref(kind)=>schema("string",[("x-semio-ref",Value::from(*kind))]),
        Shape::Coord(dims)=>fixed_number_array(*dims,*dims,"coord"),
        Shape::Dir=>fixed_number_array(3,3,"dir"),
        Shape::Dim(dims)=>fixed_number_array(*dims,*dims,"dim"),
        Shape::Range=>fixed_number_array(2,3,"range"),
        Shape::Count=>schema("integer",[("minimum",Value::from(0u64)),("x-semio-shape",Value::from("count"))]),
        Shape::Expr=>schema("string",[("x-semio-shape",Value::from("expr"))]),
        Shape::Embed(lang)=>schema("string",[("x-semio-shape",Value::from("embed")),("x-semio-lang",Value::from(*lang))]),
        Shape::EmbedFrom(key)=>schema("string",[("x-semio-shape",Value::from("embed")),("x-semio-lang-from",Value::from(*key))]),
    }
}

/// 📕️ JSON Schema 2020-12 object for one `RecordSpec` — one property per `FieldSpec.key`
/// (positional-only fields, whose `key` is empty, are omitted — no name to key a JSON object
/// property on), `flatten`ed nested-record fields splice their own fields into THIS SAME properties
/// map rather than nesting, mirroring what `flatten` means at parse/print altitude. `required` lists
/// every non-`optional`, non-empty-key field.
pub fn record_spec_json_schema(spec: &RecordSpec) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{Object, Value};
    let mut properties = Object::new();
    let mut required: Vec<Value> = Vec::new();
    collect_record_spec_properties(spec, &mut properties, &mut required);
    let mut map = Object::new();
    map.insert("type", Value::from("object"));
    map.insert("properties", Value::Object(properties));
    if let Some(keyword) = &spec.keyword {
        map.insert("x-semio-keyword", Value::from(keyword.clone()));
    }
    if !required.is_empty() {
        map.insert("required", Value::Array(required));
    }
    Value::Object(map)
}

fn collect_record_spec_properties(spec: &RecordSpec, properties: &mut semio_framework_pack_json::Object, required: &mut Vec<semio_framework_pack_json::Value>) {
    use semio_framework_pack_json::Value;
    for field in &spec.fields {
        if field.flatten {
            if let Shape::Record(spec_fn) = &field.shape {
                collect_record_spec_properties(&(spec_fn.ordinary)(), properties, required);
                continue;
            }
        }
        if field.key.is_empty() {
            continue;
        }
        properties.insert(field.key.clone(), shape_json_schema(&field.shape));
        if !field.optional {
            required.push(Value::from(field.key.clone()));
        }
    }
}
