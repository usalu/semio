//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 18
//! schema-owning domain subsets + the `✉️base` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[cfg(test)]
#[path = "../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
pub(crate) mod snapshot_test_backing;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};


pub use schema::diff::SemioDiff;
pub use schema::mutations::SemioMutation;
pub use schema::snapshot::SemioSnapshot;
pub use schema::SemioArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";


/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}




/// 🧾️ The runtime `semio` declares: the `✉️base` envelope schema opens the declaration and every subset appends its own
/// rows (`subsets::*::io::declare`, the declarative twin of each subset's `register`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition, conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition).schema(schema::semio_artifact_schema_descriptor()).formats(formats()?).schema_documents(schema::SEMIO_SHARED_SCHEMA_DOCUMENTS);
    let builder = io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_animation::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_audio::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_brep::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_cad::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_document::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_drawing::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_flow::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_graph::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_image::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_kit::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_mesh::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_model::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_object::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_presentation::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_table::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_text::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_value::io::declare(builder);
    let builder = semio_s_artifact_stdio_semio_video::io::declare(builder);
    conversions.iter().fold(builder, |builder, conversion| (conversion.declare)(builder)).try_build()
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.semio".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Semio", "Semio"),
        source_format: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🗂️ Registers all 19 subsets imperatively, outside any plugin assembly — the twin of [`declaration`] for native callers
/// that open semio documents without assembling `stdio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register(conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) {
    semio_s_artifact_stdio_semio_brep::io::register();
    semio_s_artifact_stdio_semio_mesh::io::register();
    semio_s_artifact_stdio_semio_model::io::register();
    semio_s_artifact_stdio_semio_value::io::register();
    semio_s_artifact_stdio_semio_document::io::register();
    semio_s_artifact_stdio_semio_cad::io::register();
    semio_s_artifact_stdio_semio_drawing::io::register();
    semio_s_artifact_stdio_semio_image::io::register();
    semio_s_artifact_stdio_semio_video::io::register();
    semio_s_artifact_stdio_semio_audio::io::register();
    semio_s_artifact_stdio_semio_animation::io::register();
    semio_s_artifact_stdio_semio_presentation::io::register();
    semio_s_artifact_stdio_semio_flow::io::register();
    semio_s_artifact_stdio_semio_text::io::register();
    semio_s_artifact_stdio_semio_table::io::register();
    semio_s_artifact_stdio_semio_graph::io::register();
    semio_s_artifact_stdio_semio_object::io::register();
    semio_s_artifact_stdio_semio_kit::io::register();
    io::register();
    for conversion in conversions {
        (conversion.register)();
    }
}
//#endregion 🔖️Register

//#region 🔖️Members
/// 🧸️ The subset table, written once and expanded into both the `SemioMembers` enum and the
/// subset-name list, so a new subset cannot be added to one and forgotten in the other.
macro_rules! semio_subset_table {
    ($macro_name:ident) => {
        $macro_name! {
            animation => animation, SemioAnimationSnapshot, SemioAnimationMutation;
            audio => audio, SemioAudioSnapshot, SemioAudioMutation;
            brep => brep, SemioBrepSnapshot, SemioBrepMutation;
            cad => cad, SemioCadSnapshot, SemioCadMutation;
            document => document, SemioDocumentSnapshot, SemioDocumentMutation;
            drawing => drawing, SemioDrawingSnapshot, SemioDrawingMutation;
            flow => flow, SemioFlowSnapshot, SemioFlowMutation;
            graph => graph, SemioGraphSnapshot, SemioGraphMutation;
            image => image, SemioImageSnapshot, SemioImageMutation;
            kit => kit, SemioKitSnapshot, SemioKitMutation;
            mesh => mesh, SemioMeshSnapshot, SemioMeshMutation;
            model => model, SemioModelSnapshot, SemioModelMutation;
            object => object, SemioObjectSnapshot, SemioObjectMutation;
            presentation => presentation, SemioPresentationSnapshot, SemioPresentationMutation;
            table => table, SemioTableSnapshot, SemioTableMutation;
            text => text, SemioTextSnapshot, SemioTextMutation;
            value => value, SemioValueSnapshot, SemioValueMutation;
            video => video, SemioVideoSnapshot, SemioVideoMutation;
        }
    };
}

/// 🧸️ Every subset name this artifact can materialize a child as.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn composable_subsets() -> Vec<&'static str> {
    macro_rules! subset_names {
        ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => { vec![$(stringify!($name)),*] };
    }
    semio_subset_table!(subset_names)
}


//#region 🧹️SnapshotRetirement


#[cfg(test)]
use semio_s_artifact_stdio_semio_text::schema::snapshot as text;

//#endregion 🧹️SnapshotRetirement

// 🧬️ Eighteen composable subsets, each bound to its exact kind, standard, subset and schema.
dsl::space_members! {
    history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum SemioMembers, SemioMembersOpen {
        Animation("s.stdio.semio", "v1", "animation", "stdio.semio") => (semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot, semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation),
        Audio("s.stdio.semio", "v1", "audio", "stdio.semio") => (semio_s_artifact_stdio_semio_audio::schema::snapshot::SemioAudioSnapshot, semio_s_artifact_stdio_semio_audio::schema::mutations::SemioAudioMutation),
        Brep("s.stdio.semio", "v1", "brep", "stdio.semio") => (semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot, semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation),
        Cad("s.stdio.semio", "v1", "cad", "stdio.semio") => (semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot, semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation),
        Document("s.stdio.semio", "v1", "document", "stdio.semio") => (semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot, semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation),
        Drawing("s.stdio.semio", "v1", "drawing", "stdio.semio") => (semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot, semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation),
        Flow("s.stdio.semio", "v1", "flow", "stdio.semio") => (semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot, semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation),
        Graph("s.stdio.semio", "v1", "graph", "stdio.semio") => (semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation),
        Image("s.stdio.semio", "v1", "image", "stdio.semio") => (semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot, semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation),
        Kit("s.stdio.semio", "v1", "kit", "stdio.semio") => (semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot, semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation),
        Mesh("s.stdio.semio", "v1", "mesh", "stdio.semio") => (semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot, semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation),
        Model("s.stdio.semio", "v1", "model", "stdio.semio") => (semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation),
        Object("s.stdio.semio", "v1", "object", "stdio.semio") => (semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot, semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation),
        Presentation("s.stdio.semio", "v1", "presentation", "stdio.semio") => (semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot, semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation),
        Table("s.stdio.semio", "v1", "table", "stdio.semio") => (semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot, semio_s_artifact_stdio_semio_table::schema::mutations::SemioTableMutation),
        Text("s.stdio.semio", "v1", "text", "stdio.semio") => (semio_s_artifact_stdio_semio_text::schema::snapshot::SemioTextSnapshot, semio_s_artifact_stdio_semio_text::schema::mutations::SemioTextMutation),
        Value("s.stdio.semio", "v1", "value", "stdio.semio") => (semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValueSnapshot, semio_s_artifact_stdio_semio_value::schema::mutations::SemioValueMutation),
        Video("s.stdio.semio", "v1", "video", "stdio.semio") => (semio_s_artifact_stdio_semio_video::schema::snapshot::SemioVideoSnapshot, semio_s_artifact_stdio_semio_video::schema::mutations::SemioVideoMutation),
    }
}

/// 🏭️ Mints a typed Semio child through its closed full-dialect factory.
pub async fn create_semio_member(id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: protocol::ActorId) -> Result<SemioMembers, dsl::VcsError> {
    <SemioMembers as dsl::MemberFactory>::create(id, dialect, initial_pack, actor).await
}

/// 📤️ Reopens a Semio member only when its persisted schema and dialect match the requested binding.
pub async fn open_semio_member(expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&dsl::OwnerRef>, envelope_pack: &[u8], actor: protocol::ActorId) -> Result<SemioMembers, dsl::VcsError> {
    <SemioMembers as dsl::MemberFactory>::open(expected, owner, envelope_pack, actor).await
}
//#endregion 🔖️Members

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::io::io_registry as v1;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("SemioComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register(_conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) {
        register_composer_entries(v1::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
#[path = "."]
mod subset_sqlite_snapshot_tests {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod animation;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod audio;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod brep;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod cad;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod document;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod drawing;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod flow;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod graph;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod image;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod kit;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod mesh;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod model;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod object;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod presentation;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod table;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod text;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod value;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
    pub(crate) mod video;
}
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🛬️native/🦀️.rs"]
mod sqlite_snapshot_controlled_owners;
//#endregion 🧪️Tests

#[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod kind {
            #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/🏷️kind/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/📃️note/🦀️.rs"]
pub mod examples_note;
//#endregion 🪢️TaxonomyMounts


















                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
