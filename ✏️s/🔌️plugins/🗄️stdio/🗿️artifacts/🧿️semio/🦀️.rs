//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 18
//! schema-owning domain subsets + the `✉️base` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[cfg(test)]

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
use semio_s_artifact_stdio_semio_text_model::schema::snapshot as text;

//#endregion 🧹️SnapshotRetirement

// 🧬️ Eighteen composable subsets, each bound to its exact kind, standard, subset and schema.
dsl::space_members! {
    native_producer:&::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits:semio_framework_os_kernel::os_store::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},
    maximum_history_allocation_bytes:8388608,
    pub enum SemioMembers, SemioMembersOpen {
        Animation("s.stdio.semio", "v1", "animation", "stdio.semio") => (semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot, semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot,semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot,semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation>::new()),
        Audio("s.stdio.semio", "v1", "audio", "stdio.semio") => (semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot, semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot,semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot,semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation>::new()),
        Brep("s.stdio.semio", "v1", "brep", "stdio.semio") => (semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot, semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot,semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot,semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation>::new()),
        Cad("s.stdio.semio", "v1", "cad", "stdio.semio") => (semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot, semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot,semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot,semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation>::new()),
        Document("s.stdio.semio", "v1", "document", "stdio.semio") => (semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot, semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot,semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot,semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation>::new()),
        Drawing("s.stdio.semio", "v1", "drawing", "stdio.semio") => (semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot, semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot,semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot,semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation>::new()),
        Flow("s.stdio.semio", "v1", "flow", "stdio.semio") => (semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot, semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot,semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot,semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation>::new()),
        Graph("s.stdio.semio", "v1", "graph", "stdio.semio") => (semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot, semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot,semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot,semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation>::new()),
        Image("s.stdio.semio", "v1", "image", "stdio.semio") => (semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot, semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot,semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot,semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation>::new()),
        Kit("s.stdio.semio", "v1", "kit", "stdio.semio") => (semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot, semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot,semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot,semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation>::new()),
        Mesh("s.stdio.semio", "v1", "mesh", "stdio.semio") => (semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot, semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot,semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot,semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation>::new()),
        Model("s.stdio.semio", "v1", "model", "stdio.semio") => (semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot,semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot,semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation>::new()),
        Object("s.stdio.semio", "v1", "object", "stdio.semio") => (semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot, semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot,semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot,semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation>::new()),
        Presentation("s.stdio.semio", "v1", "presentation", "stdio.semio") => (semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot, semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot,semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot,semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation>::new()),
        Table("s.stdio.semio", "v1", "table", "stdio.semio") => (semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot, semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot,semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot,semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation>::new()),
        Text("s.stdio.semio", "v1", "text", "stdio.semio") => (semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot, semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot,semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot,semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation>::new()),
        Value("s.stdio.semio", "v1", "value", "stdio.semio") => (semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot, semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot,semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot,semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation>::new()),
        Video("s.stdio.semio", "v1", "video", "stdio.semio") => (semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot, semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot,semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation>, ::semio_s_artifact_stdio_contract::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot,semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation>::new()),
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

//#region 🧪️Tests
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
    pub use semio_s_artifact_stdio_semio_schema::schema::*;

    pub use semio_s_artifact_stdio_semio_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_inferences::schema::diff;
    pub use semio_s_artifact_stdio_semio_inferences::schema::inferences;
    pub use semio_s_artifact_stdio_semio_editor::schema::mutations;
}

#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_semio_editor::editor::*;

    pub use semio_s_artifact_stdio_semio_inferences::editor::presence;
    pub use semio_s_artifact_stdio_semio_inferences::editor::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            pub use semio_s_artifact_stdio_semio_mut_family_2::editor::modes::edit::*;

            pub use semio_s_artifact_stdio_semio_inferences::editor::modes::edit::windows;
        }
    }
}

#[path = "."]
pub mod viewer {
    pub use semio_s_artifact_stdio_semio_editor::viewer::*;

    pub use semio_s_artifact_stdio_semio_inferences::viewer::presence;
    pub use semio_s_artifact_stdio_semio_inferences::viewer::config;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            pub use semio_s_artifact_stdio_semio_mut_family_2::viewer::modes::view::*;

            pub use semio_s_artifact_stdio_semio_inferences::viewer::modes::view::windows;
        }
    }
}

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/📃️note/🦀️.rs"]
pub mod examples_note;
//#endregion 🪢️TaxonomyMounts

#[cfg(test)]
#[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🖼️apply-image/🧪️tests/🚫️refuses/🦀️.rs"]
mod hoisted_tests_1;
