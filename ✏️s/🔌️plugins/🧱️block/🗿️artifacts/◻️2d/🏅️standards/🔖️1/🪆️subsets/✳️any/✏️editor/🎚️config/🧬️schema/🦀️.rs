//! 🧬️ schema leaf
use ::semio_framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.block.2d.config")]
pub struct Block2dConfig {
}

//region 📎 App-schema descriptor
/// 📎 `s.block.2d`'s config+presence schema descriptor — returned, not self-registered; `ArtifactEditor::app_schema`
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app` for registration.
pub fn app_schema_descriptor() -> ::semio_framework_schema_registry::AppSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "config", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "config", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "config", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "config", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "presence", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "presence", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "presence", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.block.2d", export: "presence", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    ::semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.block.2d",
        config: ::semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🔣️.json"),
            proto: "",
        },
        presence: ::semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: "",
        },
    }
}
//endregion 📎 App-schema descriptor
