
//! 🧵️ One walk over every app definition this plugin registers, proving that each id the
//! studio/home/space-index surfaces declare carries the disposition its language-neutral fixture
//! declares, and that every `Migrated` id is backed by an owned bounded tool-job factory whose
//! tool set, publication contract and execution contract are the exact ones the app's proof
//! catalog is joined against at construction time.
use super::*;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::{ArtifactApp, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactToolPublicationLane, EditorApp};
use std::collections::{BTreeMap, BTreeSet};

const STUDIO_FIXTURE: &str = include_str!("../../⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const HOME_FIXTURE: &str = include_str!("../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const SPACE_INDEX_FIXTURE: &str = include_str!("../../🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const COMPONENT_MANIFEST: &str = include_str!("../../📦️packages/🦀️rust/Cargo.toml");
const IDENTITY_FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️plugin-identity/🔣️.json");
const DEPLOYMENT_CATALOG: &str = include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json");
const GENERATED_REGISTRY: &str = include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");

/// 🚦️ Every id the app declares on any surface an interactive dispatch can address, with the
/// disposition `validate_ui_dispatch_classification` will read for it.
fn declared_dispositions(definition: &AppDefinition) -> BTreeMap<String, InteractiveJobClassification> {
    let mut declared = BTreeMap::new();
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        declared.insert(action.id.clone(), action.semantics.execution.interactive_job);
    }
    for command in definition.commands.iter().chain(definition.modes.iter().flat_map(|mode| mode.commands.iter())) {
        declared.insert(command.id.clone(), command.semantics.execution.interactive_job);
    }
    declared
}

/// 📜️ The `execution`/`status` fixture shape (studio, space index): ids whose status is
/// `Migrated`, plus the tool ids whose sole publication lane is `HostOnly`.
fn migrated_and_host_only(fixture: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    let document: pack::JsonValue = pack::parse_json(fixture).expect("language-neutral retained catalog fixture");
    let migrated = document
        .get("routes")
        .and_then(pack::JsonValue::as_array)
        .expect("routes array")
        .iter()
        .filter(|route| route.get("status").and_then(pack::JsonValue::as_str) == Some("Migrated"))
        .filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str).map(str::to_string))
        .collect::<BTreeSet<_>>();
    let host_only = document
        .get("publicationContracts")
        .and_then(pack::JsonValue::as_array)
        .expect("publication contracts array")
        .iter()
        .filter(|contract| contract.get("lanes").and_then(pack::JsonValue::as_array).is_some_and(|lanes| lanes.as_slice() == [pack::JsonValue::String("HostOnly".into())]))
        .filter_map(|contract| contract.get("toolId").and_then(pack::JsonValue::as_str).map(str::to_string))
        .collect::<BTreeSet<_>>();
    (migrated, host_only)
}

/// 📜️ The `disposition`/`lanes` fixture shape (home).
fn migrated_and_host_only_rows(fixture: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    let document: pack::JsonValue = pack::parse_json(fixture).expect("language-neutral retained catalog fixture");
    let routes = document.get("routes").and_then(pack::JsonValue::as_array).expect("routes array");
    let migrated = routes.iter().filter(|route| route.get("disposition").and_then(pack::JsonValue::as_str) == Some("Migrated")).filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str).map(str::to_string)).collect::<BTreeSet<_>>();
    let host_only = routes
        .iter()
        .filter(|route| route.get("lanes").and_then(pack::JsonValue::as_array).is_some_and(|lanes| lanes.as_slice() == [pack::JsonValue::String("HostOnly".into())]))
        .filter_map(|route| route.get("id").and_then(pack::JsonValue::as_str).map(str::to_string))
        .collect::<BTreeSet<_>>();
    (migrated, host_only)
}

fn factory_tool_ids<F: ArtifactOwnedToolJobFactory>() -> BTreeSet<String> {
    F::TOOL_IDS.iter().map(|id| (*id).to_string()).collect()
}

fn factory_host_only_ids<F: ArtifactOwnedToolJobFactory>() -> BTreeSet<String> {
    F::PUBLICATION_CONTRACTS.iter().filter(|contract| contract.lanes == [ArtifactToolPublicationLane::HostOnly]).map(|contract| contract.tool_id.to_string()).collect()
}

fn factory_contract_ids<F: ArtifactOwnedToolJobFactory>() -> BTreeSet<String> {
    F::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id.to_string()).collect()
}

fn migrated_ids(definition: &AppDefinition) -> BTreeSet<String> {
    declared_dispositions(definition).into_iter().filter(|(_, disposition)| *disposition == InteractiveJobClassification::Migrated).map(|(id, _)| id).collect()
}

fn unclassified_ids(definition: &AppDefinition) -> BTreeSet<String> {
    declared_dispositions(definition).into_iter().filter(|(_, disposition)| *disposition == InteractiveJobClassification::Unclassified).map(|(id, _)| id).collect()
}

/// 🧩️ Framework-injected ids (history, clipboard, interaction, tutorial) are dispatched through
/// `dispatch_framework_reserved_action`, never through an app-owned factory, so an app's own
/// proof catalog covers exactly the app-declared migrated ids.
fn app_owned(ids: BTreeSet<String>, owned: &BTreeSet<String>) -> BTreeSet<String> {
    ids.into_iter().filter(|id| owned.contains(id)).collect()
}

#[semio_framework_async_macros::async_test]
async fn studio_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory() {
    let definition = engine::space::create_space_app().await.definition;
    let (fixture_migrated, fixture_host_only) = migrated_and_host_only(STUDIO_FIXTURE);
    let owned = factory_tool_ids::<engine::space::SpaceCommandJobFactory>();
    assert!(unclassified_ids(&definition).is_empty(), "an unclassified id aborts build_definition at runtime");
    assert_eq!(app_owned(migrated_ids(&definition), &owned), fixture_migrated, "the studio's migrated ids must equal its fixture's");
    assert_eq!(owned, fixture_migrated, "the owned factory must claim exactly the migrated ids");
    assert_eq!(factory_contract_ids::<engine::space::SpaceCommandJobFactory>(), owned, "every claimed tool needs a publication contract");
    assert_eq!(factory_host_only_ids::<engine::space::SpaceCommandJobFactory>(), fixture_host_only);
    assert_eq!(<engine::space::SpaceApp as ArtifactApp>::bounded_first_step_tool_proofs().len(), owned.len());
    for tool in ["setAppRegistrations", "openSpace", "openInstance", "importSpacePackPayload", "spawnApp"] {
        assert!(owned.contains(tool), "the shell dispatches {tool} on every studio session");
    }
}

#[semio_framework_async_macros::async_test]
async fn home_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory() {
    let definition = semio_s_artifact_space_home::editor::home::create_home_app().await;
    let (fixture_migrated, fixture_host_only) = migrated_and_host_only_rows(HOME_FIXTURE);
    let owned = factory_tool_ids::<semio_s_artifact_space_home::editor::home::HomeRetainedCommandJobFactory>();
    assert!(unclassified_ids(&definition).is_empty());
    assert_eq!(app_owned(migrated_ids(&definition), &owned), fixture_migrated);
    assert_eq!(owned, fixture_migrated);
    assert_eq!(factory_contract_ids::<semio_s_artifact_space_home::editor::home::HomeRetainedCommandJobFactory>(), owned);
    assert_eq!(factory_host_only_ids::<semio_s_artifact_space_home::editor::home::HomeRetainedCommandJobFactory>(), fixture_host_only);
    assert_eq!(<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp> as ArtifactApp>::bounded_first_step_tool_proofs().len(), owned.len());
    for tool in ["importSpace", "foldDirectoryEvents", "createStudio", "deleteVirtualFileSystemNode", "renameSpace", "bindSpaceFile"] {
        assert!(owned.contains(tool), "Home's own rows and the shell dispatch {tool}");
    }
}

#[semio_framework_async_macros::async_test]
async fn space_index_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory() {
    let definition = semio_s_artifact_space_space::editor::space_index::create_space_index_editor();
    let (fixture_migrated, fixture_host_only) = migrated_and_host_only(SPACE_INDEX_FIXTURE);
    let owned = factory_tool_ids::<semio_s_artifact_space_space::editor::space_index::SpaceIndexRetainedCommandJobFactory>();
    assert!(unclassified_ids(&definition).is_empty());
    assert_eq!(app_owned(migrated_ids(&definition), &owned), fixture_migrated);
    assert_eq!(owned, fixture_migrated);
    assert_eq!(factory_contract_ids::<semio_s_artifact_space_space::editor::space_index::SpaceIndexRetainedCommandJobFactory>(), owned);
    assert_eq!(factory_host_only_ids::<semio_s_artifact_space_space::editor::space_index::SpaceIndexRetainedCommandJobFactory>(), fixture_host_only);
    assert_eq!(<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor> as ArtifactApp>::bounded_first_step_tool_proofs().len(), owned.len());
}

/// 🧾️ `validate_tool_job_rows` joins each proof row's `controller_id`/`document_schema` against
/// the runtime surface app id and `A::DOCUMENT_SCHEMA`, and each row's contract against the
/// registered factory's — the proof macro can only take literals, so the literals are pinned here.
#[semio_framework_async_macros::async_test]
async fn tool_proof_catalogs_match_the_runtime_identity_they_are_joined_against() {
    assert_eq!(engine::space::S_PLAY_APP_ID, "s.space.studio@1/*#editor");
    assert_eq!(<engine::space::SpaceApp as ArtifactApp>::DOCUMENT_SCHEMA, "os.workflow");
    assert_eq!(<semio_s_artifact_space_home::editor::home::HomeApp as ArtifactEditor>::DIALECT.artifact_kind, "s.space.home");
    assert_eq!(<semio_s_artifact_space_home::editor::home::HomeApp as ArtifactEditor>::DOCUMENT_SCHEMA, "s.home");
    assert_eq!(<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor as ArtifactEditor>::DIALECT.artifact_kind, "s.space.space");
    assert_eq!(<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor as ArtifactEditor>::DOCUMENT_SCHEMA, "s.space");
    assert_eq!(<engine::space::SpaceCommandJobFactory as ArtifactOwnedToolJobFactory>::DOCUMENT_SCHEMA, <engine::space::SpaceApp as ArtifactApp>::DOCUMENT_SCHEMA);
    assert_eq!(<semio_s_artifact_space_home::editor::home::HomeRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::DOCUMENT_SCHEMA, <semio_s_artifact_space_home::editor::home::HomeApp as ArtifactEditor>::DOCUMENT_SCHEMA);
    assert_eq!(<semio_s_artifact_space_space::editor::space_index::SpaceIndexRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::DOCUMENT_SCHEMA, <semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor as ArtifactEditor>::DOCUMENT_SCHEMA);
}

/// 🪪️ Builder id, `package_id` and the Cargo component package must be the same identity.
/// `PluginBuilder::try_build` already rejects any `package_id` that is not exactly
/// `semio:<plugin_id>` in canonical lowercase form
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:639-643`), and it is the ONLY
/// producer of `Plugin::manifest`, so asserting a successful assembly whose `plugin_id` equals the
/// Cargo `[package.metadata.component] package` suffix pins all three literals at once. A guest
/// that fails here mints the `assembly-failed` stub descriptor instead, which `describeBuiltPlugin`
/// rejects — 90 minutes of wasm build after the fact.
#[semio_framework_async_macros::async_test]
async fn manifest_plugin_id_matches_the_cargo_component_package() {
    let declared = COMPONENT_MANIFEST
        .lines()
        .skip_while(|line| line.trim() != "[package.metadata.component]")
        .find_map(|line| line.split_once('=').filter(|(key, _)| key.trim() == "package").map(|(_, value)| value.trim().trim_matches('"').to_string()))
        .expect("[package.metadata.component] package");
    let (namespace, id) = declared.split_once(':').expect("component package is <namespace>:<id>");
    assert_eq!(namespace, "semio");
    assert_eq!(assembled_plugin().manifest.plugin_id, id);
    assert_eq!(assembled_plugin().manifest.plugin_id, identity_fixture()["pluginId"].as_str().expect("fixture pluginId"));
}

/// 🪪️ The language-agnostic identity tuple, read once and shared by every authority assertion below.
fn identity_fixture() -> Value {
    serde_json::from_str(IDENTITY_FIXTURE).expect("plugin-identity fixture is JSON")
}

/// 🪪️ Every authority that names this plugin must name the SAME identity: the Cargo component
/// package, the root `builder(…)`/`package_id(…)` pair through the assembled manifest, the
/// hand-authored deployment catalog row (id + physical module directory), the generated registry row
/// (`pluginId`/`packageId`/`packageName`/`host`) and the canonical `s.<plugin>.<kind>` owner segment
/// the assembly gate `plugin-assembly.surface-dependency-gate` derives via `ArtifactKindId::plugin()`.
/// The playground VARIANT that selects this plugin is deliberately a different name and is pinned
/// separately, so a future rename can never silently conflate the two again (the 2026-09-05 regression:
/// `builder("space")` in Rust alone against a registry/host/deployment catalog still saying `s`).
#[semio_framework_async_macros::async_test]
async fn plugin_identity_is_the_same_in_every_authority() {
    let fixture = identity_fixture();
    let plugin_id = fixture["pluginId"].as_str().expect("fixture pluginId");
    let package_id = fixture["packageId"].as_str().expect("fixture packageId");
    assert_eq!(package_id, format!("semio:{plugin_id}"));

    let manifest = assembled_plugin().manifest;
    assert_eq!(manifest.plugin_id, plugin_id);
    assert!(COMPONENT_MANIFEST.contains(&format!("package = \"{package_id}\"")), "Cargo component package is not {package_id}");
    assert!(COMPONENT_MANIFEST.contains(&format!("name = \"{}\"", fixture["packageName"].as_str().expect("fixture packageName"))));
    assert!(COMPONENT_MANIFEST.contains(&format!("variant = \"{}\"", fixture["playgroundVariant"].as_str().expect("fixture playgroundVariant"))), "the playground variant row is the OTHER name and must stay declared");

    let prefix = fixture["artifactKindPrefix"].as_str().expect("fixture artifactKindPrefix");
    assert_eq!(prefix, format!("s.{plugin_id}."));
    for app in &manifest.apps {
        assert!(app.id.starts_with(prefix), "{} is not owned by {plugin_id} under the canonical s.<plugin>.<kind> grammar", app.id);
    }

    let catalog: Value = serde_json::from_str(DEPLOYMENT_CATALOG).expect("deployment catalog is JSON");
    let row = catalog["modules"].as_array().expect("deployment catalog modules").iter().find(|entry| entry["pluginId"] == plugin_id).unwrap_or_else(|| panic!("deployment catalog has no row for {plugin_id}"));
    assert_eq!(row["directoryName"], fixture["moduleDirectoryName"]);

    let registry: Value = serde_json::from_str(GENERATED_REGISTRY).expect("generated registry is JSON");
    let entry = registry.as_array().expect("generated registry rows").iter().find(|entry| entry["pluginId"] == plugin_id).unwrap_or_else(|| panic!("generated registry has no row for {plugin_id}"));
    assert_eq!(entry["packageId"], package_id);
    assert_eq!(entry["packageName"], fixture["packageName"]);
    assert_eq!(entry["host"], fixture["host"]);
}

/// 🏗️ The whole guest assembly, named. `plugin()` runs `build_definition` for every registered
/// surface — which is where `validate_interactive_job_classification` rejects an `Unclassified`
/// id — plus the package-identity and artifact-declaration preflights.
fn assembled_plugin() -> Plugin<SpaceApps> {
    match plugin() {
        Ok(plugin) => plugin,
        Err(error) => panic!("plugin assembly rejected: {error:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn plugin_assembly_succeeds_and_registers_all_five_surfaces() {
    let plugin = assembled_plugin();
    let ids = plugin.manifest.apps.iter().map(|app| app.id.clone()).collect::<BTreeSet<_>>();
    assert_eq!(ids, ["s.space.home@1/*#editor", "s.space.home@1/*#viewer", "s.space.space@1/*#editor", "s.space.space@1/*#viewer", "s.space.studio@1/*#editor"].iter().map(|id| (*id).to_string()).collect::<BTreeSet<_>>());
}

/// 🧰️ The build-time completeness gate, run for real: `VcsArtifactApp::with_registry` calls
/// `tool_job_registration`, which joins every migrated id to a live registered factory by owner
/// witness, controller id, document schema and exact contract equality, and every declared
/// publication lane to its installed store preparation factory. A mismatch is a panic here, the
/// same panic the guest takes on its first turn.
#[semio_framework_async_macros::async_test]
async fn every_app_instance_constructs_against_its_registered_proof_catalog() {
    let mut studio = VcsArtifactApp::<engine::space::SpaceApp>::with_registry(Default::default(), AppActionRegistry::from_definition(&engine::space::create_space_app().await.definition)).await;
    let mut home = VcsArtifactApp::<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp>>::with_registry(Default::default(), AppActionRegistry::from_definition(&semio_s_artifact_space_home::editor::home::create_home_app().await)).await;
    let mut index = VcsArtifactApp::<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>>::with_registry(Default::default(), AppActionRegistry::from_definition(&semio_s_artifact_space_space::editor::space_index::create_space_index_editor())).await;
    assert_eq!(<engine::space::SpaceApp as ArtifactApp>::bounded_first_step_tool_proofs().len(), 15);
    assert_eq!(<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp> as ArtifactApp>::bounded_first_step_tool_proofs().len(), 18);
    assert_eq!(<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor> as ArtifactApp>::bounded_first_step_tool_proofs().len(), 14);
    artifact_app_laws::close_registered_fixture_app(&mut studio);
    artifact_app_laws::close_registered_fixture_app(&mut home);
    artifact_app_laws::close_registered_fixture_app(&mut index);
}
