//! 🌳️ The editor and viewer surface declarations of the instance app: 4 item(s), about 63 lines, lifted out of `📜️declaration/🌳️tree/🦀️.rs`, held above `instance-app` so the app crate stays inside the source gate.
//!
//! WP2 owns the source file and cuts the items here (editor_surface, viewer_surface, EditorSurfaceApp, ViewerSurfaceApp); WP7 wires this file with one `mod` line in the crate root. Until `instance-app` exists, the source module re-exports the items with `pub use`.
use semio_framework::AppDefinition;
use semio_framework::kernel::ActorId;
use semio_framework::kernel::Rights;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_app::mount_natural_file_actions;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_surface::EditorApp;
use semio_framework_plugin_decl_surface::ViewerApp;
use semio_framework_plugin_decl_surface::artifact_codec_table;
use semio_framework_plugin_decl_registry::declarations::SurfaceDeclaration;
use semio_framework_plugin_decl_registry::declarations::stamp_fault_notices;

/// ✏️ Builds an editor `SurfaceDeclaration` from `E` — `def` is `Editor::builder(E::DIALECT)
/// ...build_definition()`, same as `PluginBuilder::editor::<E>`. `rights: Rights::Write`
/// signals the commit walk to attach BOTH Read and Write document capabilities (baseline Read
/// always, plus Write when `rights == Rights::Write`) — see `capability_rows_for`. The
/// concrete app retains `E::Members`, so a composed editor keeps its typed child fleet in every
/// bundle that declares this subset. Schema-first (C8.2), exactly as `PluginBuilder::editor`:
/// an empty `io.artifact_schema` is stamped with `E::DOCUMENT_SCHEMA`, so the manifest names the
/// document this surface opens and a shell attaching a hub document checks the lease against it.
///
/// 🚫️async: E4 fn-pointer slot
///
/// 🚫️async: E4 fn-pointer slot — `E::app_schema()` is a genuine (pure, non-suspending)
/// AFIT trait method; resolved synchronously via `resolve_ready` to fit the bare slot.
pub fn editor_surface<E: ArtifactEditor, PA: PluginApp + From<VcsArtifactApp<EditorApp<E>, E::Members>>>(mut def: AppDefinition) -> SurfaceDeclaration<PA> {
    fn factory<E: ArtifactEditor, PA: PluginApp + From<VcsArtifactApp<EditorApp<E>, E::Members>>>(def: &AppDefinition, actor: ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> PA {
        PA::from(::semio_framework_async::poll::resolve_ready(VcsArtifactApp::<EditorApp<E>, E::Members>::with_registry_on_bus(EditorApp::<E>::default(), AppActionRegistry::from_definition(def), semio_framework::ActionBus::production(), actor, mounted_policy, identity)))
    }
    fn app_schema<E: ArtifactEditor>() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        E::app_schema()
    }
    if def.io.artifact_schema.is_empty() {
        def.io.artifact_schema = E::DOCUMENT_SCHEMA.to_string();
    }
    if let Some(codec) = E::natural_file_codec() {
        mount_natural_file_actions(&mut def, codec);
    }
    stamp_fault_notices::<EditorApp<E>>(&mut def);
    SurfaceDeclaration { definition: def, factory: factory::<E, PA>, app_schema: app_schema::<E>, document_schema: E::DOCUMENT_SCHEMA, codec: artifact_codec_table::<EditorApp<E>>(), mutation_roster: None, rights: Rights::Write }
}

/// 👁️ Viewer twin of `editor_surface` — `rights: Rights::Read` (baseline Read only, contract
/// §2.3 clause 4: a viewer's document store attaches Read only, never Write), and the same
/// schema-first stamp from `V::DOCUMENT_SCHEMA`.
///
/// 🚫️async: E4 fn-pointer slot
///
/// 🚫️async: E4 fn-pointer slot — see `editor_surface`'s `app_schema` doc.
pub fn viewer_surface<V: ArtifactViewer, PA: PluginApp + From<VcsArtifactApp<ViewerApp<V>, V::Members>>>(mut def: AppDefinition) -> SurfaceDeclaration<PA> {
    fn factory<V: ArtifactViewer, PA: PluginApp + From<VcsArtifactApp<ViewerApp<V>, V::Members>>>(def: &AppDefinition, actor: ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> PA {
        PA::from(::semio_framework_async::poll::resolve_ready(VcsArtifactApp::<ViewerApp<V>, V::Members>::with_registry_on_bus(ViewerApp::<V>::default(), AppActionRegistry::from_definition(def), semio_framework::ActionBus::production(), actor, mounted_policy, identity)))
    }
    fn app_schema<V: ArtifactViewer>() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        V::app_schema()
    }
    if def.io.artifact_schema.is_empty() {
        def.io.artifact_schema = V::DOCUMENT_SCHEMA.to_string();
    }
    stamp_fault_notices::<ViewerApp<V>>(&mut def);
    SurfaceDeclaration { definition: def, factory: factory::<V, PA>, app_schema: app_schema::<V>, document_schema: V::DOCUMENT_SCHEMA, codec: artifact_codec_table::<ViewerApp<V>>(), mutation_roster: None, rights: Rights::Read }
}

/// 🎭️ Authoring traits for artifact surfaces split by mutation capability (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1/§2.2/§2.4). `ArtifactEditor`
/// and `ArtifactViewer` replace `ArtifactApp` as the trait an app AUTHOR implements; the runtime
/// `ArtifactApp` trait above is unchanged and is satisfied by the `EditorApp`/`ViewerApp` adapters
/// below through pure delegation — nothing here alters runtime dispatch.
/// 👁️✏️ Authoring trait for a mutation-capable surface — a structural copy of `ArtifactApp`'s
/// members (contract §2.1) plus `ROLE`/`DIALECT`; `APP_ID` is intentionally absent, the runtime id
/// being derived from `DIALECT`+`ROLE` via `surface_app_id` (contract §7.4) instead of hand-written.
/// `EditorApp<E>` below is the sole `ArtifactApp` implementor for any `E: ArtifactEditor`.
/// ✏️🧩️ The concrete runtime app an editor surface instantiates — the roster read off `E` itself, so
/// a fleet variant spelled this way can never register `E` over a roster it does not compose with.
pub type EditorSurfaceApp<E> = VcsArtifactApp<EditorApp<E>, <E as ArtifactEditor>::Members>;

/// 👁️🧩️ Viewer twin of [`EditorSurfaceApp`].
pub type ViewerSurfaceApp<V> = VcsArtifactApp<ViewerApp<V>, <V as ArtifactViewer>::Members>;
