//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

//#region 🗂️OrderedOwnership
pub use semio_framework_value_ordered::ordered;

#[path = "🗂️ordered/🔢️numeric/🧮️scratch/🦀️.rs"]
pub mod numeric_scratch;
pub use semio_framework_value_paged_list::list;
#[path = "📦️paged/🦀️.rs"]
pub mod paged;
#[path = "📝️shared-utf8/🦀️.rs"]
pub mod shared_utf8;
pub use shared_utf8::SharedUtf8;
//#endregion 🗂️OrderedOwnership

pub use semio_framework_value_intrinsic::bytes;

#[path = "🧬️schema/🔗️graphql/🔢️int64/🦀️.rs"]
pub mod graphql_int64;

#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;

pub use semio_framework_value_native_decoding::native_decoding;
pub use native_decoding::{NativeDecodeControl, NativeDecodeProgress, NativeDecodeRetirementRecipient};

pub use semio_framework_value_native_encoding::native_encoding;
pub use native_encoding::{NativeEncodeControl,NativeEncodeContinuation};



#[path = "⚠️refusal/🦀️.rs"]
pub mod refusal;
pub use refusal::{ValueError, ValueRefusalKind};

//#region 🔁️Codec
pub use semio_framework_value_intrinsic::{DslValue,DslValueSource,Number,JsonInteger,json_integer,edit_through_value,ControlledValueHasher,DecodedValue,IntrinsicRetirement,FromValue,ToValue,ValueEdit,ValueShape};
pub use semio_framework_value_intrinsic::dsl_value;
//#endregion 🔁️Codec

//#region 🔖️SchemaExports
const VALUE_SCHEMA_EXPORTS: [semio_framework_schema_registry::SchemaExport; 1] = [semio_framework_schema_registry::SchemaExport { id: "schema", leaves: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: include_str!("🧬️schema/🟦️.ts"), graphql: include_str!("🧬️schema/🔗️.graphql"), json_schema: include_str!("🧬️schema/🔣️.json"), proto: include_str!("🧬️schema/🛰️.proto") } }];

/// 📌️ Registers the framework value schema (`framework/value/schema.json`: `DslValue`, `Binary64`, `Binary64Transport`, the
/// integer carriers) as the `schema` export of the `framework.value` scope, so every contract that `$ref`s it resolves it.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_value_schema_exports() -> Result<(), semio_framework_schema_registry::SchemaExportRegistryError> {
    semio_framework_schema_registry::register_scope_schema_exports(semio_framework_schema_registry::ScopeSchemaExports { scope: "framework.value", exports: &VALUE_SCHEMA_EXPORTS })
}
//#endregion 🔖️SchemaExports

#[path="🧬️schema/🌳️intrinsic/🔣️json/🦀️.rs"]
pub mod intrinsic_json;

pub use semio_framework_value_paged_text::paged_text;
