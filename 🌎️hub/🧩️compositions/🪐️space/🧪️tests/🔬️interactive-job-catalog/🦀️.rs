
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
const HOME_FIXTURE: &str = include_str!("../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const SPACE_INDEX_FIXTURE: &str = include_str!("../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const COMPONENT_MANIFEST: &str = include_str!("../../📦️packages/🦀️rust/Cargo.toml");
const IDENTITY_FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️plugin-identity/🔣️.json");

/// 🚦️ Every id the app declares on any surface an interactive dispatch can address, with the
/// disposition `validate_ui_dispatch_classification` will read for it.
fn declared_dispositions(definition: &AppDefinition) -> BTreeMap<String, InteractiveJobClassification> {
    let mut declared = BTreeMap::new();
    for action in definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())) {
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
    let document: semio_framework_pack_json::Value = semio_framework_pack_json::parse(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral retained catalog fixture");
    let migrated = document
        .get("routes")
        .and_then(semio_framework_pack_json::Value::as_array)
        .expect("routes array")
        .iter()
        .filter(|route| route.get("status").and_then(semio_framework_pack_json::Value::as_str) == Some("Migrated"))
        .filter_map(|route| route.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string))
        .collect::<BTreeSet<_>>();
    let host_only = document
        .get("publicationContracts")
        .and_then(semio_framework_pack_json::Value::as_array)
        .expect("publication contracts array")
        .iter()
        .filter(|contract| contract.get("lanes").and_then(semio_framework_pack_json::Value::as_array).is_some_and(|lanes| lanes.as_slice() == [semio_framework_pack_json::Value::String("HostOnly".into())]))
        .filter_map(|contract| contract.get("toolId").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string))
        .collect::<BTreeSet<_>>();
    (migrated, host_only)
}

/// 📜️ The `disposition`/`lanes` fixture shape (home).
fn migrated_and_host_only_rows(fixture: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    let document: semio_framework_pack_json::Value = semio_framework_pack_json::parse(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral retained catalog fixture");
    let routes = document.get("routes").and_then(semio_framework_pack_json::Value::as_array).expect("routes array");
    let migrated = routes.iter().filter(|route| route.get("disposition").and_then(semio_framework_pack_json::Value::as_str) == Some("Migrated")).filter_map(|route| route.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string)).collect::<BTreeSet<_>>();
    let host_only = routes
        .iter()
        .filter(|route| route.get("lanes").and_then(semio_framework_pack_json::Value::as_array).is_some_and(|lanes| lanes.as_slice() == [semio_framework_pack_json::Value::String("HostOnly".into())]))
        .filter_map(|route| route.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string))
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
    // 🏠️ Home's retained (migrated) surface is the rows `HOME_RETAINED_TOOL_IDS` declares and the
    // committed fixture pins above; the spot-check below keeps this assertion from going vacuous if
    // the fixture and the factory ever drift to the same empty set together.
    for tool in ["openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "presenceHeartbeat"] {
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

/// 🪪️ The assembled component and its authored Cargo deployment authority match the neutral identity fixture.
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

    assert!(COMPONENT_MANIFEST.contains(&format!("deployment-directory = \"{}\"", fixture["moduleDirectoryName"].as_str().expect("owner directory"))));
    assert!(COMPONENT_MANIFEST.contains(&format!("host = {{ landing = \"{}\", shell = \"{}\" }}", fixture["host"]["landingAppId"].as_str().expect("landing app"), fixture["host"]["hostAppId"].as_str().expect("host shell"))));
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
/// same panic the guest takes on its first turn. The pinned counts are each app's
/// `bounded_first_step_tool_proofs!` list: the studio's 40, Home's 18 (ticket 26/09/18 S4 retained
/// `applyDirectoryEventPage` and `createStudio`; the data-classification lane added
/// `promoteToHubSpace` (HostOnly) and `persistLocally` (Artifact); presence added
/// `presenceHeartbeat`; the IO-owning catalog jobs added `bindSpaceFile`, `importSpace` and
/// `deleteVirtualFileSystemNode`; the host's local catalog added `applyLocalCatalogDocument`) and the space index's 14.
#[semio_framework_async_macros::async_test]
async fn every_app_instance_constructs_against_its_registered_proof_catalog() {
    let mut studio = VcsArtifactApp::<engine::space::SpaceApp>::with_registry(Default::default(), AppActionRegistry::from_definition(&engine::space::create_space_app().await.definition), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await;
    let mut home = VcsArtifactApp::<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp>>::with_registry(Default::default(), AppActionRegistry::from_definition(&semio_s_artifact_space_home::editor::home::create_home_app().await), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await;
    let mut index = VcsArtifactApp::<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>>::with_registry(Default::default(), AppActionRegistry::from_definition(&semio_s_artifact_space_space::editor::space_index::create_space_index_editor()), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into())).await;
    assert_eq!(<engine::space::SpaceApp as ArtifactApp>::bounded_first_step_tool_proofs().len(), 40);
    assert_eq!(<EditorApp<semio_s_artifact_space_home::editor::home::HomeApp> as ArtifactApp>::bounded_first_step_tool_proofs().len(), 18);
    assert_eq!(<EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor> as ArtifactApp>::bounded_first_step_tool_proofs().len(), 14);
    artifact_app_laws::close_registered_fixture_app(&mut studio);
    artifact_app_laws::close_registered_fixture_app(&mut home);
    artifact_app_laws::close_registered_fixture_app(&mut index);
}

/// 🪶️ Pins the two actual persisted Runtime owners independently of the five app surfaces.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_composed_owner_census() {
    use semio_framework_os_kernel::{ArtifactCodec, ArtifactSqliteSnapshot};
    use semio_framework::io::io_mechanism::{io_route, native_snapshot_sqlite_schema};
    use semio_framework::io_schema::{ArtifactDialect, IoFidelity, SQLITE_SNAPSHOT};
    use semio_s_artifact_space_home::{SHomeSnapshot,SHomeMutation};
    use semio_s_artifact_space_space::{SSpaceSnapshot,SSpaceMutation};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️snapshot-owner-census/🔣️.json")).expect("closed neutral composed owner census");
    let owners=fixture["owners"].as_array().unwrap();
    assert_eq!(owners.len(),2);
    let metadata=fixture["metadataOnlyDefinitions"].as_array().unwrap();
    assert!(metadata.is_empty(),"this actual composition currently declares two Runtime owners");
    let expected=owners.iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["standard"].as_str().unwrap().to_string(),row["subset"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect::<BTreeSet<_>>();
    let declarations=[semio_s_artifact_space_home::declaration().await.unwrap(),semio_s_artifact_space_space::declaration().unwrap()];
    let declared_kinds=declarations.iter().map(|row|row.definition().identity().as_str().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(declared_kinds,owners.iter().map(|row|row["kind"].as_str().unwrap().to_string()).collect());
    let bindings=declarations.iter().flat_map(|row|row.document_codec_bindings()).collect::<Vec<_>>();
    assert_eq!(bindings.len(),2);
    let actual=bindings.iter().map(|(dialect,codec)|(dialect.artifact_kind.to_string(),dialect.standard.0.to_string(),dialect.subset.0.to_string(),codec.schema.clone())).collect::<BTreeSet<_>>();
    assert_eq!(actual,expected,"expected membership comes from direct declaration authority");
    let plugin=assembled_plugin();
    assert_eq!(plugin.manifest.plugin_id,fixture["plugin"].as_str().unwrap());
    assert_eq!(plugin.artifact_definitions().definitions().map(|row|row.identity().as_str().to_string()).collect::<BTreeSet<_>>(),declared_kinds,"metadata traversal retains the actual complete definition roster");
    let expected_hosted=fixture["hostedArtifactKinds"].as_array().unwrap().iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect::<BTreeSet<_>>();
    assert_eq!(plugin.manifest.hosted_artifact_kinds.iter().map(|row|(row.id.clone(),row.schema.clone())).collect::<BTreeSet<_>>(),expected_hosted,"foreign hosted membership is independent of the two complete owned Runtime declarations");
    let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);
    for(dialect,codec)in bindings {
        let native=ArtifactDialect::from(dialect);
        let declared=codec.snapshot_sqlite.as_ref().expect("every declared Snapshot owns its semantic SQLite provider");
        let(type_id,sql,typed)=match native.artifact_kind.as_str(){
            "s.space.home"=>(std::any::TypeId::of::<SHomeSnapshot>(),SHomeSnapshot::SQLITE_SCHEMA,ArtifactCodec::bare::<SHomeSnapshot,SHomeMutation>("s.home")),
            "s.space.space"=>(std::any::TypeId::of::<SSpaceSnapshot>(),SSpaceSnapshot::SQLITE_SCHEMA,ArtifactCodec::bare::<SSpaceSnapshot,SSpaceMutation>("s.space")),
            _=>panic!("undeclared Runtime owner")
        };
        assert_eq!(declared.snapshot_type,Some(type_id));
        assert!(!sql.trim().is_empty());
        assert_eq!(declared.schema.as_ref(),sql);
        assert!(declared.identical_to(typed.snapshot_sqlite.as_ref().unwrap()),"typed owner and declared complete SQL/hooks differ");
        let installed=semio_framework_os_kernel::document_codec(&codec.schema).await.unwrap().expect("the ordinary composition installs each actual document codec");
        assert_eq!(installed.schema,codec.schema);
        assert!(declared.identical_to(installed.snapshot_sqlite.as_ref().expect("installed semantic provider")));
        assert_eq!(native_snapshot_sqlite_schema(&native).unwrap(),sql);
        for(from,into)in[(&native,&sqlite),(&sqlite,&native)]{
            let route=io_route(from,into,1).await.unwrap().value;
            assert_eq!(route.hops.len(),1);
            assert_eq!(route.fidelity,IoFidelity::Exact);
        }
        eprintln!("[DEBUG] Hub Space composed Snapshot owner dialect={} schema={} concrete_type={:?} sql_bytes={} installed_same_hooks=true direct_exact_directions=2",native.to_coordinate(),codec.schema,declared.snapshot_type,sql.len());
    }
    eprintln!("[DEBUG] Hub Space composed Snapshot census Runtime=2 metadataOnly=0 declaredBindings=2 genuineAppSurfaces={}",plugin.manifest.apps.len());
}
