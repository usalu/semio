//! 📥️ Items of `store` that belong to `os-store-test-support` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️Backbone
//#region 🧪️Tests

/// 🗂️ The store's own document-model schema documents, one scope per `🧬️schema` dir exactly as the catalogue owns them:
/// a composed child (`framework/schema/artifact-reference/child/schema.json`), its owner stamp (`os/store/child/owner/schema.json`), a link
/// (`framework/schema/artifact-reference/link/schema.json`) and a blob (`framework/schema/artifact-reference/blob/schema.json`), each as its whole-document `schema` export.
pub const STORE_SCHEMA_EXPORTS: [semio_framework_schema_registry::ScopeSchemaExports; 4] = [
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "framework.schema.artifact-reference.child",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: "",
                typescript: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🟦️.ts"),
                graphql: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json"),
                proto: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "os.store.child.owner",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("../../../🪆️child/🏠️owner/🧬️schema/🦀️.rs"),
                typescript: include_str!("../../../🪆️child/🏠️owner/🧬️schema/🟦️.ts"),
                graphql: include_str!("../../../🪆️child/🏠️owner/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("../../../🪆️child/🏠️owner/🧬️schema/🔣️.json"),
                proto: include_str!("../../../🪆️child/🏠️owner/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "framework.schema.artifact-reference.link",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🦀️.rs"),
                typescript: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🟦️.ts"),
                graphql: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🔣️.json"),
                proto: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/🔗️link/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "framework.schema.artifact-reference.blob",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🦀️.rs"),
                typescript: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🟦️.ts"),
                graphql: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🔣️.json"),
                proto: include_str!("../../../../../../../🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🛰️.proto"),
            },
        }],
    },
];

/// 📌️ Registers the store's own document-model schema documents ([`STORE_SCHEMA_EXPORTS`]), so every artifact contract
/// that `$ref`s a composed child, its owner, a link or a blob resolves them.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_store_schema_exports() -> Result<(), semio_framework_schema_registry::SchemaExportRegistryError> {
    STORE_SCHEMA_EXPORTS.into_iter().try_for_each(semio_framework_schema_registry::register_scope_schema_exports)
}
//#endregion 🧪️Tests
//#endregion 🔖️Backbone
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#endregion 🔖️EphemeralLanes
