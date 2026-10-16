//! 🌳️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C: the declaration tree a
//! subset/standard/artifact `component.rs` builds as PLAIN DATA (every field `pub`, every value a
//! struct literal — no framework registry function is ever named by a subset), handed to
//! `PluginBuilder::declare_artifact` and walked once, atomically, by `try_build`. Nested in its
//! own module rather than flattened into `app` because the pre-existing `🔖️ArtifactDeclaration`
//! region of `📜️declaration` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE, kept per debt D1) already
//! owns the bare name `ArtifactDeclaration` — the two coexist as `app::ArtifactDeclaration` (old)
//! and `app::declarations::ArtifactDeclaration` (new), the same module-nesting trick the sibling
//! `🚪️io` file uses for its own old/new `IoPayload`/`Confidence` collision (`📓️w1-a-report.md`).
//! `PluginBuilder::artifact(ArtifactDeclaration)` already binds the OLD type on the one inherent
//! `PluginBuilder<Ready>` impl block, and Rust has no method overloading, so the new entry point
//! is named `.declare_artifact(...)` instead — a forced, documented deviation from design.md §2's
//! literal `.artifact(a: ArtifactDeclaration)` sketch.
//! 🧭️ Every NEW io-mechanism type (`IoEntry`/`IoPayload`/io_schema's wire types) is referenced by its FULL path
//! (`semio_framework_os_kernel::io::io_mechanism::…` / `semio_framework::io_schema::…`), never `use`d bare, so it can never
//! collide with the OLD same-named `IoPayload`/`Confidence` of the `🔖️Dialect` region.
use semio_framework_plugin_decl_surface::remember_subset_examples;
use semio_framework_ui_contract::schema_metadata;
use crate::plugin_program::AppFactory;
use semio_framework::AppDefinition;
use semio_framework::kernel::ActorId;
use semio_framework::kernel::ArtifactKind;
use semio_framework::kernel::CapabilityRequirement;
use semio_framework::kernel::Rights;
use semio_framework::kernel::Scope;
use std::collections::BTreeMap;
use semio_framework_artifact_reference::ArtifactKindId;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::Dialect;
use semio_framework_artifact_reference::StandardId;
use semio_framework::surface_app_id;
use semio_framework::AppRole;
use semio_framework_plugin_reactor_app::NoPluginApp;
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::mount_natural_file_actions;
use semio_framework_plugin_identity::ArtifactLocale;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use semio_framework_plugin_identity::preflight_artifact_identity;
use crate::artifact_inference::ArtifactInferenceService;
use semio_framework_plugin_ownership::store_lifecycle::OwnerMutationRoster;
use semio_framework_plugin_decl_surface::ArtifactCodecTableV1;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_surface::EditorApp;
use semio_framework_plugin_decl_surface::ViewerApp;
use semio_framework_plugin_decl_surface::artifact_codec_table;

/// 🗣️ One text/binary [`dsl::LanguageSpec`] pair for one `NativeCodecs` channel.
/// `dsl::LanguageRole` names FIVE roles total (`Document`/`Ops`/`Diff`/`Pack`/`Spr`) —
/// `snapshot` pairs `Document`+`Pack`, `mutations` pairs `Ops`+`Spr`, `diff` carries `Diff`
/// text only (no binary diff role exists), `inferences` is wholly optional. Either half may
/// be `None`: a subset with no hand-authored grammar for a channel still owns that channel's
/// codec (`NativeCodecs.codec`), just with no `.grammar.semio`/`.protocol.semio` registered
/// for LSP/verification.
#[derive(Clone, Copy)]
pub struct LanguagePair {
    pub text: Option<&'static semio_framework_dsl::LanguageSpec>,
    pub binary: Option<&'static semio_framework_dsl::LanguageSpec>,
}

/// 🧬️ The subset's own native DSL text grammar + pack binary protocol + the store-level
/// document codec that stitches them together (design.md §2) — home for the five `dsl::
/// LanguageSpec`s (`document`/`op`/`diff`/`pack`/`spr`) and the `store::os_store::serialization::ArtifactCodec` a
/// subset used to hand `PluginBuilder`/old `ArtifactDeclarationBuilder` as separate
/// registration channels (`.languages(...)` ×5 roles + `.document_codec_bare::<S,M>(...)`).
pub struct NativeCodecs {
    pub snapshot: LanguagePair,
    pub diff: LanguagePair,
    pub mutations: LanguagePair,
    pub inferences: Option<LanguagePair>,
    pub codec: store::os_store::serialization::ArtifactCodec,
}

/// 🚪️ A subset's complete io surface: its native codecs plus every foreign-dialect hop it
/// contributes to the process-wide `semio_framework_os_kernel::io::io_mechanism` registry (`IoEntry`
/// rows built by the subset's own `serializer_entry`/`deserializer_entry`/`_text` calls —
/// `📓️w1-a-report.md`).
///
/// There is exactly ONE conformance mechanism and it is NOT here: `Deserializer::CONFORMANCE`
/// runs inside every `deserializer_entry`, immediately after a successful decode INTO this
/// subset's dialect, which is precisely "conformance runs after every hop into the dialect".
/// A second payload-level hook on this struct would duplicate that one job across two places,
/// so it does not exist — see `📓️w1-c-report.md` "conformance decision".
pub struct IoDeclaration {
    pub native: NativeCodecs,
    pub entries: &'static [semio_framework_os_kernel::io::io_mechanism::IoEntry],
}

/// 🧬️ A subset's four-facet schema descriptor plus its inference schema(s) and any executable
/// native inference services — sourcing what old `ArtifactDeclarationBuilder::schema(s)`/
/// `.inferences(...)`/`.inference_services(...)` used to.
pub struct SchemaDeclaration {
    pub descriptor: ::semio_framework_schema_registry::ArtifactSchemaDescriptor,
    pub inferences: &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor],
    pub inference_services: Vec<ArtifactInferenceService>,
}

/// 🎭️ One viewer or editor surface as plain data — everything `PluginBuilder::viewer::<V>`/
/// `.editor::<E>` compute from `V`/`E` today, minus the ability to capture: `factory` takes the
/// `AppDefinition` explicitly (`fn(&AppDefinition) -> Box<dyn PluginApp>`) rather than the
/// zero-arg `fn() -> Box<dyn PluginApp>` design.md §2 sketches — the SAME bare-fn-pointer-
/// cannot-capture constraint `📓️w1-a-report.md`'s "Constructor design decision" hit (a
/// monomorphized non-capturing `fn` item cannot close over `def`'s `AppActionRegistry`), so the
/// definition travels as an explicit parameter instead of being captured; the commit walk below
/// wraps it in a real capturing closure at the one call site that actually needs to (ordinary
/// application code, not a generic constructor, so capturing there is free).
/// 🗃️ sdk-dedyn (O1/§1.5): generic over the per-plugin app enum `PA: PluginApp` a
/// `dyn_enum_close!`-closed fleet crate provides — replaces `Box<dyn PluginApp>`. `factory`
/// stays a bare, non-capturing `fn` pointer (E4: the definition travels WITH it, see the doc
/// above) but now produces `PA` directly via its `From<VcsArtifactApp<..>>` impl.
pub struct SurfaceDeclaration<PA: PluginApp = NoPluginApp> {
    pub definition: AppDefinition,
    /// 🚫️async: E4 fn-pointer slot
    pub factory: for<'a, 'b> fn(&AppDefinition, ActorId, semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> PA,
    /// 🚫️async: E4 fn-pointer slot
    pub app_schema: fn() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor>,
    pub document_schema: &'static str,
    pub codec: ArtifactCodecTableV1,
    pub mutation_roster: Option<OwnerMutationRoster>,
    pub rights: Rights,
}

/// 📢️ Stamps `A`'s declared fault notices (design §20.12) into the definition its descriptor publishes. A table that
/// breaks `validate_fault_notices` (code grammar, duplicate code, blank cell, placeholder drift) is an authoring
/// error, refused like every other static declaration fault (`AppBuilder::build_definition`).
pub fn stamp_fault_notices<A: ArtifactApp>(def: &mut AppDefinition) {
    def.fault_notices = semio_framework::fault_notice_definitions(A::fault_notices());
    if let Some(error) = semio_framework::validate_fault_notices(&def.fault_notices).first() {
        panic!("app {}: {error}", def.id);
    }
}

/// 🪆️ One complete standalone subset — own schema, own io, own viewer, own editor, own
/// examples (design.md rule 2). `dialect` is the one source of truth `editor.definition.id`/
/// `viewer.definition.id` must derive from via `surface_app_id` (proven by
/// `artifact_app_laws::assert_subset_declaration_ids_are_derived`).
pub struct SubsetDeclaration<PA: PluginApp = NoPluginApp> {
    pub dialect: Dialect,
    pub schema: SchemaDeclaration,
    pub io: IoDeclaration,
    pub viewer: SurfaceDeclaration<PA>,
    pub editor: SurfaceDeclaration<PA>,
    pub examples: &'static [ExampleSource],
}

/// 🗂️ One standard's file-format identity — mimes/extensions only (design.md §2); the commit
/// walk synthesizes the remaining `semio_framework_os_kernel::io::FormatDescriptor` fields (`kind_id`/
/// `short_id`/`name`/`full_name`/`dir_name`) from the owning `ArtifactDeclaration.kind` +
/// `StandardDeclaration.id`, since a standard never needs a SEPARATE identity from its own
/// dialect coordinate.
pub struct MediaDeclaration {
    pub mimes: &'static [&'static str],
    pub extensions: &'static [&'static str],
}

pub struct StandardDeclaration<PA: PluginApp = NoPluginApp> {
    pub id: StandardId,
    pub media: MediaDeclaration,
    pub subsets: Vec<SubsetDeclaration<PA>>,
}

/// 🗿️ Root of the tree (design.md §2, verbatim shape) — one artifact, its standards, and
/// (unlike the old same-named type two regions up) no capabilities/migrations/composition
/// fields: see the W1-C report's "old channel → new home" table for why each is either
/// absorbed elsewhere (migrations → ordinary `IoEntry` rows, design.md §3) or left untouched
/// (composition still reads `<Snapshot as ArtifactCompositionFields>` directly; capabilities
/// still go through `PluginBuilder::capability`/`.local_backbone_storage()`).
pub struct ArtifactDeclaration<PA: PluginApp = NoPluginApp> {
    pub kind: ArtifactKindId,
    pub localization: &'static [(ArtifactLocale, &'static str)],
    pub standards: Vec<StandardDeclaration<PA>>,
}

/// 🏗️ Everything `PluginBuilder::try_build` folds into its own `app_defs`/
/// `app_schema_descriptors`/`capabilities` vectors once a declared tree commits.
pub(crate) struct DeclaredRegistration<PA: PluginApp = NoPluginApp> {
    pub app_defs: Vec<(App, AppFactory<PA>)>,
    pub app_schema_descriptors: Vec<fn() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor>>,
    pub capabilities: Vec<CapabilityRequirement>,
}

pub fn format_descriptor_of<PA: PluginApp>(artifact: &ArtifactDeclaration<PA>, standard: &StandardDeclaration<PA>) -> semio_framework_os_kernel::io::FormatDescriptor {
    let kind_id = format!("{}@{}", artifact.kind.as_str(), standard.id.0);
    semio_framework_os_kernel::io::FormatDescriptor {
        kind_id: kind_id.clone(),
        short_id: kind_id.clone(),
        aliases: Vec::new(),
        mimes: standard.media.mimes.iter().map(|mime| mime.to_string()).collect(),
        extensions: standard.media.extensions.iter().map(|extension| extension.to_string()).collect(),
        name: kind_id.clone(),
        full_name: kind_id.clone(),
        neutral: false,
        dir_name: kind_id.replace(['@', '.'], "-"),
        is_binary: true,
    }
}

/// ➕️ Baseline Read, plus Write when `surface.rights == Rights::Write` — see `editor_surface`/
/// `viewer_surface` doc for why `rights` alone drives this instead of a `Vec<Rights>` field.
pub fn capability_rows_for<PA: PluginApp>(surface: &SurfaceDeclaration<PA>) -> Vec<CapabilityRequirement> {
    let mut rows = vec![CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App }];
    if surface.rights == Rights::Write {
        rows.push(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Write, scope: Scope::App });
    }
    rows
}

/// 🪪️ A surface opens the document its subset's codec names: the hub creates every document of the subset with
/// `io.native.codec.schema` in its genesis history, and a surface whose `DOCUMENT_SCHEMA` differs refuses that history at
/// hydration (`Identity`) — measured on hub 7800 p33, where the puzzle 3d editor named `puzzle.3d.fixture` against its
/// codec's (and viewer's) `puzzle.3d` and no hub puzzle 3d document ever opened. Refused here, the drift fails `describe`.
pub fn check_surface_document_schema<PA: PluginApp>(subset: &SubsetDeclaration<PA>, surface: &SurfaceDeclaration<PA>) -> Result<(), PluginAssemblyError> {
    if surface.document_schema != subset.io.native.codec.schema {
        return Err(PluginAssemblyError::new(
            "plugin-assembly.declaration-document-schema",
            format!("surface {:?} opens document schema {:?} but its subset's codec names {:?}", surface.definition.id, surface.document_schema, subset.io.native.codec.schema),
        ));
    }
    Ok(())
}

pub fn check_surface_id<PA: PluginApp>(subset: &SubsetDeclaration<PA>, surface: &SurfaceDeclaration<PA>, role: AppRole) -> Result<(), PluginAssemblyError> {
    let expected = surface_app_id(&subset.dialect.into(), role);
    if surface.definition.id != expected {
        return Err(PluginAssemblyError::new(
            "plugin-assembly.declaration-surface-id",
            format!("surface id {:?} for dialect {} must be derived via surface_app_id (expected {:?})", surface.definition.id, subset.dialect.artifact_kind, expected),
        ));
    }
    Ok(())
}

pub fn subset_binding<PA: PluginApp>(plugin_id: &str, artifact: &ArtifactDeclaration<PA>, subset: &SubsetDeclaration<PA>) -> semio_framework_os_kernel::io::ArtifactCodecBinding {
    let apps = [&subset.viewer.definition.id, &subset.editor.definition.id].into_iter().filter(|app| !app.is_empty()).cloned().collect();
    semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::TreeSubset, plugin_id, Some(artifact.kind.as_str().to_string()), apps, Some(subset.dialect.into()), None, &subset.io.native.codec)
}

pub fn artifact_declaration_bindings<PA: PluginApp>(plugin_id: &str, declarations: &[ArtifactDeclaration<PA>]) -> Vec<semio_framework_os_kernel::io::ArtifactCodecBinding> {
    declarations.iter().flat_map(|artifact| artifact.standards.iter().flat_map(move |standard| standard.subsets.iter().map(move |subset| subset_binding(plugin_id, artifact, subset)))).collect()
}

pub fn preflight_io_entries<PA: PluginApp>(declarations: &[ArtifactDeclaration<PA>]) -> Result<(), PluginAssemblyError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut proposed: BTreeMap<(String, String), semio_framework::io_schema::IoFidelity> = BTreeMap::new();
    for artifact in declarations {
        for standard in &artifact.standards {
            for subset in &standard.subsets {
                for entry in subset.io.entries {
                    let key = (ArtifactDialect::from(entry.from).to_coordinate(), ArtifactDialect::from(entry.into).to_coordinate());
                    match proposed.get(&key) {
                        Some(fidelity) if *fidelity == entry.fidelity => {}
                        Some(_) => return Err(PluginAssemblyError::new("plugin-assembly.declaration-io", format!("conflicting io entries declared for {} -> {}", key.0, key.1))),
                        None => {
                            proposed.insert(key, entry.fidelity);
                        }
                    }
                }
            }
        }
    }
    let existing = semio_framework_os_kernel::io::io_mechanism::io_entries();
    for ((from, into), fidelity) in &proposed {
        let mut current_fidelity = None;
        for descriptor in existing.iter() {
            if &descriptor.from.to_coordinate() == from && &descriptor.into.to_coordinate() == into {
                current_fidelity = Some(descriptor.fidelity);
                break;
            }
        }
        if let Some(current_fidelity) = current_fidelity {
            if current_fidelity != *fidelity {
                return Err(PluginAssemblyError::new("plugin-assembly.declaration-io", format!("io entry {from} -> {into} conflicts with an established entry at a different fidelity")));
            }
        }
    }
    Ok(())
}

/// 📌️ Commits every channel of every declared tree — preflight FIRST (no mutation possible
/// past this point), then schema/inference-descriptor/inference-service/language (each
/// independently atomic), then document-codec+format-descriptor under ONE shared assembly
/// guard (dropped before the next step), then one `io_register` call per subset (each
/// independently atomic — see `preflight_artifact_declarations`'s doc for why these cannot
/// share a guard with each other or with the codec/format step).
/// 📌️ The one boot-time site that publishes the named schema exports of the scopes whose
/// crates cannot register themselves: `framework.interaction` and `framework.ui.contract`
/// declare `register_scope_exports()` but nothing on their side runs at process start.
/// Registration is exact-duplicate tolerant and conflict-fatal, so calling it once per
/// plugin assembly is the same as calling it once (`📋️execution-contract.md` §C,
/// `📓️wp4c-framework-modules.md` §5.4, ledger row 95).
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_boot_scope_schema_exports() {
    semio_framework::interaction::schema::register_scope_exports();
    schema_metadata::register_scope_exports();
}

/// 🪞️ Projects app definitions without publishing any registry entry.
///
/// 📚️ The subset's examples are registered ONCE — through the editor row —
/// and stamped with the subset's dialect there, which is what makes them
/// resolve for the viewer surface too (`manifest::examples_for_app`).
pub fn project_artifact_declarations<PA: PluginApp>(declarations: &[ArtifactDeclaration<PA>]) -> DeclaredRegistration<PA> {
    let mut result = DeclaredRegistration { app_defs: Vec::new(), app_schema_descriptors: Vec::new(), capabilities: Vec::new() };
    for artifact in declarations {
        for standard in &artifact.standards {
            for subset in &standard.subsets {
                remember_subset_examples(subset.dialect, subset.examples);
                for surface in [&subset.editor, &subset.viewer] {
                    let definition = surface.definition.clone();
                    let examples = if surface.definition.role == AppRole::Editor { subset.examples.to_vec() } else { Vec::new() };
                    result.app_defs.push((App { definition: definition.clone(), examples }, AppFactory { definition, create: surface.factory, document_schema: surface.document_schema, codec: surface.codec }));
                    result.app_schema_descriptors.push(surface.app_schema);
                    result.capabilities.extend(capability_rows_for(surface));
                }
            }
        }
    }
    result
}

