//! 📐️ Authored Record value, shape, field projection and typed receiving ownership.
#[cfg(test)]
extern crate self as semio_framework_dsl_record;
pub use semio_framework_diagnostic::{Limits,TextError,TextSpan};
pub use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value::{DslValue,Number};
pub use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[path="../🪆️binding/🦀️.rs"]
mod binding;
pub use binding::*;
#[cfg(test)]
#[path="../🛬️receive/🦀️.rs"]
mod native_receive;
#[path="../🫴️receiving/🦀️.rs"]
pub mod receiving;
pub use receiving::{DslReceivingField,DslReceivingRecord,DslReceivingVariants,DslOwnedReceivingField,DslOwnedReceivingRecord,DslOwnedReceivingVariants,OwnedFieldReceiving};
#[path="../🛫️encode/🦀️.rs"]
pub mod native_encoding;
pub use semio_framework_dsl_record_ownership::{
    RecordBytes,RecordLayout,Shape,FieldSpec,RecordSpec,GrammarSpec,WireNode,WireEdgeLabel,WireValue,
    FieldValue,RecordFields,RecordValue,Cst,ExprOp,ExprValue,shape_type_name,
    BorrowedShape,BorrowedFieldSpec,BorrowedRecordSpec,NativeSchemaControl,RecordSpecProducer,producer,retirement,
};
#[path="../🫳️borrowed/🏭️authored/🦀️.rs"]
mod authored_borrowed_schema;
pub use authored_borrowed_schema::{BorrowedRecordSpecProducer,BorrowedDslField,BorrowedDslRecord,BorrowedDslVariants,borrowed_field_shape,borrowed_record,borrowed_unit};

//#region 🔖️JsonSchema
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema §3.2: the
// gateway's JSON Schema derivation is primarily `ActionArgDef::json_schema()` (manifest-declared
// args); THIS is the fallback for `app_commands!` payload structs whose only declared shape is a
// `RecordSpec` (`#[derive(dsl::DslRecord)]`) — the catalog compiler tags whatever it emits from here
// `x-semio-confidence: "payload"`, not this module's concern.
/// 🧬️ JSON Schema 2020-12 for one `Shape` leaf/node — recurses through `Tuple`/`List`/
/// `Record`/`Block`/`Statements`/`Map`/`Table`. `Quantity`/`Angle` carry their unit as
/// `x-semio-unit`; `Ref(kind)` carries the referenced entity kind as `x-semio-ref`; every shape with
/// no native JSON Schema vocabulary (`Bytes64`/`Wire`/`Coord`/`Dir`/`Dim`/`Range`/`Count`/`Expr`/
/// `Embed`/`EmbedFrom`) additionally carries `x-semio-shape` naming the exact `Shape` variant.
pub fn shape_json_schema(shape: &Shape) -> semio_framework_pack_json::Value {
    use semio_framework_pack_json::{object, Value};
    let number_array = |min: u64, max: u64, extra: Option<(&str, &str)>| {
        let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), object([("type".to_string(), Value::from("number"))])), ("minItems".to_string(), Value::from(min)), ("maxItems".to_string(), Value::from(max))];
        if let Some((key, value)) = extra {
            fields.push((key.to_string(), Value::from(value)));
        }
        object(fields)
    };
    match shape {
        Shape::Bool => object([("type".to_string(), Value::from("boolean"))]),
        Shape::Int => object([("type".to_string(), Value::from("integer"))]),
        Shape::UInt => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64))]),
        Shape::Float => object([("type".to_string(), Value::from("number"))]),
        Shape::Text => object([("type".to_string(), Value::from("string"))]),
        Shape::Bytes64 => object([("type".to_string(), Value::from("string")), ("contentEncoding".to_string(), Value::from("base64")), ("x-semio-shape".to_string(), Value::from("bytes64"))]),
        Shape::Enum(variants) => object([("type".to_string(), Value::from("string")), ("enum".to_string(), Value::Array(variants.iter().map(|(tag, _)| Value::from(tag.clone())).collect()))]),
        Shape::Tuple(inner, len) => {
            let items = shape_json_schema(inner);
            let mut fields = vec![("type".to_string(), Value::from("array")), ("items".to_string(), items)];
            if let Some(len) = len {
                fields.push(("minItems".to_string(), Value::from(*len as u64)));
                fields.push(("maxItems".to_string(), Value::from(*len as u64)));
            }
            object(fields)
        }
        Shape::List(inner) => {
            let items = shape_json_schema(inner);
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Record(spec_fn) => record_spec_json_schema(&(spec_fn.ordinary)()),
        Shape::Block(inner) => shape_json_schema(inner),
        Shape::Statements(variants) => {
            let mut one_of = Vec::with_capacity(variants.len());
            for (keyword, spec_fn) in variants {
                let mut entry = record_spec_json_schema(&(spec_fn.ordinary)());
                if let Value::Object(map) = &mut entry {
                    map.insert("x-semio-keyword", Value::from(keyword.clone()));
                }
                one_of.push(entry);
            }
            object([("type".to_string(), Value::from("array")), ("items".to_string(), object([("oneOf".to_string(), Value::Array(one_of))]))])
        }
        Shape::Map(inner) => {
            let additional_properties = shape_json_schema(inner);
            object([("type".to_string(), Value::from("object")), ("additionalProperties".to_string(), additional_properties)])
        }
        Shape::Value => Value::Object(semio_framework_pack_json::Object::new()),
        Shape::Table(spec_fn) => {
            let items = record_spec_json_schema(&(spec_fn.ordinary)());
            object([("type".to_string(), Value::from("array")), ("items".to_string(), items)])
        }
        Shape::Wire => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("wire"))]),
        Shape::Quantity(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol))]),
        Shape::Angle(unit) => object([("type".to_string(), Value::from("number")), ("x-semio-unit".to_string(), Value::from(unit.symbol)), ("x-semio-shape".to_string(), Value::from("angle"))]),
        Shape::Ref(kind) => object([("type".to_string(), Value::from("string")), ("x-semio-ref".to_string(), Value::from(*kind))]),
        Shape::Coord(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "coord"))),
        Shape::Dir => number_array(3, 3, Some(("x-semio-shape", "dir"))),
        Shape::Dim(dims) => number_array(*dims as u64, *dims as u64, Some(("x-semio-shape", "dim"))),
        Shape::Range => number_array(2, 3, Some(("x-semio-shape", "range"))),
        Shape::Count => object([("type".to_string(), Value::from("integer")), ("minimum".to_string(), Value::from(0u64)), ("x-semio-shape".to_string(), Value::from("count"))]),
        Shape::Expr => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("expr"))]),
        Shape::Embed(lang) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang".to_string(), Value::from(*lang))]),
        Shape::EmbedFrom(key) => object([("type".to_string(), Value::from("string")), ("x-semio-shape".to_string(), Value::from("embed")), ("x-semio-lang-from".to_string(), Value::from(*key))]),
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️json-schema/🦀️.rs"]
mod json_schema_tests;
//#endregion 🔖️JsonSchema
#[cfg(test)]
#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

