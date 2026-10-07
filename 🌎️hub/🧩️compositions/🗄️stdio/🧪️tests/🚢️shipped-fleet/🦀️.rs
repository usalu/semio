//! 🛡️ Every stdio package ships a bounded, declared app fleet, and together the stdio packages ship every stdio app
//! and open every stdio artifact kind exactly once.

use semio_framework::{AppRole, PackageDescriptor};
use semio_framework_plugin::kernel::ActivationEvent;
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};
use semio_framework_plugin::{ArtifactRuntimeCapabilityRequirement, Plugin, PluginApp, PluginAssemblyError};
use semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS;
use semio_s_artifact_stdio_contract::ArtifactAssembly;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

/// 📏️ Apps one stdio component may assemble. Every registered app monomorphises the whole app runtime
/// (`VcsArtifactApp<EditorApp<E>/ViewerApp<V>, Members>`, its retained tool-job factories and snapshot-edit factory) and
/// is live code inside its component: all 176 stdio apps as ONE component drove the wasm-dev rustc to 85 GB in one codegen
/// unit and `wasm-component-ld` refused it ("functions count exceeds limit of 1000000", chain b3 run 2, 2026-09-27). A
/// package ships whole artifact kinds, because the host resolves who opens a kind per kind (`on-artifact-kind:`), never per
/// subset — so the largest kind, `s.stdio.semio` (19 subsets, 38 apps), sets the bound; ST1's wasm32 measurement of
/// `stdio-semio` is recorded in ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP `📓️wp-st1.md`.
const SHIPPED_APP_CEILING: usize = 40;

/// 🧾️ One stdio package as it ships: the Cargo manifest whose playground rows declare its shipped editors, and the
/// descriptor its own component describes.
struct ShippedPackage {
    id: &'static str,
    manifest: &'static str,
    descriptor: PackageDescriptor,
}

/// 🛂️ Describes one assembled bundle through the runtime path its component's `describe` export takes.
fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> PackageDescriptor {
    let runtime = PluginRuntime::<PA>::new();
    let bundle = bundle.unwrap_or_else(|error| panic!("[DEBUG] outward component assembly rejected: {error:?}"));
    install_plugin_bundle_result(&runtime, Ok(bundle));
    let bytes = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
    semio_framework_value::FromValue::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect("descriptor bytes")).expect("strict descriptor")
}

/// 📦️ The stdio component and its nine family components.
const PACKAGE_IDS: [&str; 10] = ["stdio", "stdio-image", "stdio-media", "stdio-cad", "stdio-bim", "stdio-mesh", "stdio-pdf", "stdio-office", "stdio-semio", "stdio-binary"];

/// 🧾️ Assembles and describes one stdio package by id.
fn shipped(id: &'static str) -> ShippedPackage {
    let (manifest, descriptor) = match id {
        "stdio" => (include_str!("../../📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio::plugin())),
        "stdio-image" => (include_str!("../../🧩️extensions/🖼️image/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_image::plugin())),
        "stdio-media" => (include_str!("../../🧩️extensions/🎵️media/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_media::plugin())),
        "stdio-cad" => (include_str!("../../🧩️extensions/🛠️cad/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_cad::plugin())),
        "stdio-bim" => (include_str!("../../🧩️extensions/🏠️bim/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_bim::plugin())),
        "stdio-mesh" => (include_str!("../../🧩️extensions/🔺️mesh/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_mesh::plugin())),
        "stdio-pdf" => (include_str!("../../🧩️extensions/📘️pdf/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_pdf::plugin())),
        "stdio-office" => (include_str!("../../🧩️extensions/💼️office/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_office::plugin())),
        "stdio-semio" => (include_str!("../../🧩️extensions/🧿️semio/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_semio::plugin())),
        "stdio-binary" => (include_str!("../../🧩️extensions/🔢️binary/📦️packages/🦀️rust/Cargo.toml"), describe(semio_hub_stdio_binary::plugin())),
        other => panic!("{other} is not a stdio package"),
    };
    ShippedPackage { id, manifest, descriptor }
}

/// 📦️ Every stdio package, each assembled and described once per process.
fn packages() -> &'static [ShippedPackage] {
    static PACKAGES: OnceLock<Vec<ShippedPackage>> = OnceLock::new();
    PACKAGES.get_or_init(|| PACKAGE_IDS.into_iter().map(shipped).collect())
}

/// 📋️ The `app` of every `[[package.metadata.semio.playground]]` row of one manifest — one row per shipped editor.
fn declared_editors(manifest: &str) -> BTreeSet<String> {
    manifest
        .split("[[package.metadata.semio.playground]]")
        .skip(1)
        .filter_map(|row| row.lines().find_map(|line| line.strip_prefix("app = \"").and_then(|rest| rest.strip_suffix('"')).map(str::to_string)))
        .collect()
}

/// 🧫️ The neutral editor catalogue: every stdio editor app and the number of stdio formats.
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/✏️editor-catalog/🔣️.json")).expect("neutral editor catalogue fixture")
}

/// 🗂️ The app ids one descriptor ships in one role.
fn app_ids(descriptor: &PackageDescriptor, role: AppRole) -> BTreeSet<String> {
    descriptor.manifest.apps.iter().filter(|app| app.role == role).map(|app| app.id.clone()).collect()
}

/// 🎬️ The artifact kinds one descriptor activates on.
fn activated_kinds(descriptor: &PackageDescriptor) -> BTreeSet<String> {
    descriptor
        .activation_events
        .iter()
        .map(|event| match event {
            ActivationEvent::OnArtifactKind { kind } => kind.clone(),
            other => panic!("{} activates on {other:?}, a stdio package activates on artifact kinds only", descriptor.manifest.plugin_id),
        })
        .collect()
}

/// 🛡️ LAW: each package registers exactly its playground-declared editors plus one viewer per shipped dialect, stays
/// under [`SHIPPED_APP_CEILING`], and every family depends on exactly `stdio` — so widening a component is a declared,
/// reviewed change of its playground rows, never a feature edit that silently ships another package's fleet.
#[test]
fn every_stdio_package_ships_exactly_its_declared_bounded_fleet() {
    let fixture = fixture();
    for package in packages() {
        let owner = fixture["deployedComponents"].as_array().expect("authored owners").iter().find(|row| row["id"].as_str() == Some(package.id)).expect("component has authored owner");
        assert!(package.manifest.contains(&format!("name = \"{}\"", owner["package"].as_str().expect("Cargo package"))));
        assert!(package.manifest.contains("role = \"hub\""));
        assert!(package.manifest.contains("component-kind = \"plugin\""));
        let descriptor = &package.descriptor;
        assert_eq!(descriptor.package_id, format!("semio:{}", package.id), "{} package identity", package.id);
        assert_eq!(descriptor.manifest.plugin_id, package.id, "{} plugin identity", package.id);
        let declared = declared_editors(package.manifest);
        assert_eq!(app_ids(descriptor, AppRole::Editor), declared, "{} ships exactly its playground-declared editors", package.id);
        let viewers = app_ids(descriptor, AppRole::Viewer).iter().map(|id| id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>();
        assert_eq!(viewers, declared.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "{} ships the viewer of every shipped dialect, and only those", package.id);
        assert!(descriptor.manifest.apps.len() <= SHIPPED_APP_CEILING, "{} ships {} apps over the per-component ceiling of {SHIPPED_APP_CEILING}", package.id, descriptor.manifest.apps.len());
        let dependencies = descriptor.manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect::<Vec<_>>();
        assert_eq!(dependencies, if package.id == "stdio" { Vec::<&str>::new() } else { vec!["stdio"] }, "{} runtime dependencies", package.id);
        eprintln!("[DEBUG] outward-stdio-component id={} apps={} editors={}", package.id, descriptor.manifest.apps.len(), declared.len());
    }
}

/// 🧮️ LAW: the stdio packages together ship every editor of the neutral catalogue and its viewer, each in exactly one
/// package, and every shipped editor exposes the complete details window and every snapshot edit action.
#[test]
fn the_stdio_packages_ship_every_stdio_app_exactly_once() {
    let fixture = fixture();
    let mut owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for package in packages() {
        for app in &package.descriptor.manifest.apps {
            owners.entry(app.id.as_str()).or_default().push(package.id);
        }
    }
    let repeated = owners.iter().filter(|(_, packages)| packages.len() > 1).collect::<Vec<_>>();
    assert!(repeated.is_empty(), "apps shipped by more than one package: {repeated:?}");
    let expected = fixture["editorApps"].as_array().unwrap().iter().map(|app| app.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(expected.len(), fixture["editorCount"].as_u64().unwrap() as usize);
    assert_eq!(owners.keys().filter(|id| id.ends_with("#editor")).map(|id| id.to_string()).collect::<BTreeSet<_>>(), expected, "every catalogue editor ships");
    assert_eq!(owners.keys().filter(|id| id.ends_with("#viewer")).map(|id| id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>(), expected.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "every catalogue dialect ships its viewer");
    for package in packages() {
        for app in package.descriptor.manifest.apps.iter().filter(|app| app.role == AppRole::Editor) {
            let details = app.window_kinds.iter().find(|window| window.id == fixture["detailsWindow"].as_str().unwrap()).unwrap_or_else(|| panic!("{} has no complete details window", app.id));
            for id in SNAPSHOT_EDIT_ACTION_IDS {
                assert!(details.actions.iter().any(|action| action.id == *id), "{} details window does not expose {id}", app.id);
            }
        }
    }
}

/// 🎬️ LAW: every stdio artifact kind is opened by exactly one package — the one shipping its apps activates on it, and
/// no other package does — so the catalog's `on-artifact-kind:` rows resolve every stdio document to the package that
/// can open every one of its subsets.
#[test]
fn every_stdio_kind_is_opened_by_exactly_one_package() {
    let fixture = fixture();
    let mut openers: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for package in packages() {
        let activated = activated_kinds(&package.descriptor);
        let opened = package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>();
        assert_eq!(activated, opened, "{} activates on exactly the kinds its apps open", package.id);
        for kind in activated {
            openers.entry(kind).or_default().push(package.id);
        }
    }
    let repeated = openers.iter().filter(|(_, packages)| packages.len() > 1).collect::<Vec<_>>();
    assert!(repeated.is_empty(), "kinds opened by more than one package: {repeated:?}");
    let expected = fixture["editorApps"].as_array().unwrap().iter().map(|app| app.as_str().unwrap().split('@').next().unwrap().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(expected.len(), fixture["formatCount"].as_u64().unwrap() as usize);
    assert_eq!(openers.keys().cloned().collect::<BTreeSet<_>>(), expected, "every stdio kind is opened");
}

/// 🧬️ LAW: every stdio package's assembly publishes its editors' document schemas, so a shipped editor's snapshot-edit
/// route resolves its contract — `plugin()` alone, no test-side registration. Measured before (S18 served matrix, native
/// probe 2026-09-28): `.artifact(…)` declarations kept their schemas plugin-local, `s.stdio.json` stayed unregistered and
/// every json `set-node` was refused `snapshot-edit.schema-unregistered`. The JSON oracle is serde_json.
#[semio_framework_async_macros::async_test]
async fn the_shipped_assembly_publishes_every_editor_document_schema_and_a_json_node_edit_lands() {
    use semio_framework_value::DslValue;
    for package in packages() {
        for app in package.descriptor.manifest.apps.iter().filter(|app| app.role == AppRole::Editor) {
            let kind = app.id.split('@').next().expect("an app id names its artifact kind");
            assert!(semio_framework_schema_registry::artifact_schema_descriptor_registered(kind), "{} ({}) edits {kind}, whose document schema its package's assembly must publish", app.id, package.id);
        }
    }
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>, _>(async {
        semio_framework_plugin::App { definition: semio_s_artifact_stdio_json::editor::json_any::create_json_editor(), examples: Vec::new() }
    }, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::os_spr::LOCAL_ACTOR_ID.into()))
    .await;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let source = r#"{"edited":["node",1,true]}"#;
    let args = DslValue::Object(vec![("nodeId".into(), DslValue::String("$".into())), ("revision".into(), DslValue::String(revision)), ("value".into(), DslValue::String(source.into()))]);
    app.handle_action("set-node", Some(&args), &meta).await.expect("the shipped json editor admits a revision-bound root node edit");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("the node edit publishes");
    let edited = semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_text(&app.snapshot().expect("json snapshot").value);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&edited).expect("the edited document is JSON"), serde_json::from_str::<serde_json::Value>(source).expect("serde_json oracle"), "the published document is exactly the applied source");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}

/// 🧪️ The environment variable naming the one package [`package_runtime_probe`] assembles in its child process.
const PROBE_PACKAGE: &str = "SEMIO_STDIO_RUNTIME_PROBE_PACKAGE";

/// 🏠️ LAW (d): every stdio package, assembled ALONE in its own process as its wasm guest is, hosts the complete runtime of
/// every artifact kind it opens — each schema, inference descriptor, document codec, composer, format and subset validator
/// the kind's owner declares is live in that process. Measured before (LB2 native probe, 2026-09-29): `stdio-image`'s
/// `plugin()` alone registered none of png/jpg/bmp/svg's schemas, and the bmp, wav, epw, binary, ifc, gif and semio roots
/// declared no runtime at all — 28 shipped editors refused every snapshot edit `snapshot-edit.schema-unregistered`.
#[test]
fn every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process() {
    let binary = std::env::current_exe().expect("the test binary");
    let failures = PACKAGE_IDS
        .into_iter()
        .filter_map(|id| {
            let run = std::process::Command::new(&binary).args(["package_runtime_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(PROBE_PACKAGE, id).output().expect("the package probe runs");
            (!run.status.success()).then(|| format!("{id} alone does not host every kind it opens:\n{}\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))
        })
        .collect::<Vec<_>>();
    assert!(failures.is_empty(), "{} of {} packages fail:\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\n"));
}

/// 🔬️ The child half of LAW (d): assembles only the package [`PROBE_PACKAGE`] names and checks every runtime requirement of
/// every kind it activates on, exactly as the kind's owner declares them, against this process's live registries.
#[test]
#[ignore = "the child process of every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process"]
fn package_runtime_probe() {
    let id = std::env::var(PROBE_PACKAGE).expect("the parent law names one package");
    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));
    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");
    let assemblies = semio_hub_stdio::catalog::artifact_assemblies().expect("the stdio artifact assemblies");
    let mut unmet = Vec::new();
    for kind in activated_kinds(&package.descriptor) {
        let declaration = assemblies.iter().find_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) if declaration.definition().identity().as_str() == kind => Some(declaration),
            _ => None,
        });
        let Some(declaration) = declaration else {
            unmet.push(format!("{kind}: its owner declares no runtime"));
            continue;
        };
        for requirement in declaration.runtime_capability_requirements().expect("the declaration's runtime requirements") {
            if !requirement_is_live(&requirement) {
                unmet.push(format!("{kind}: {requirement:?}"));
            }
        }
    }
    assert!(unmet.is_empty(), "{id} alone leaves {} runtime requirements unmet: {unmet:#?}", unmet.len());
}

/// 🔎️ Whether one runtime requirement is live in this process: schemas and inference descriptors in the kernel catalogs,
/// shared schema documents (`schema-export` claims) in the schema export registry,
/// document codecs in the store, composers, formats and subset validators in the io registries. Grammar rows are captured by
/// the plugin runtime and never published (`PluginRuntimeRegistry::languages`), so no process state answers them.
fn requirement_is_live(requirement: &ArtifactRuntimeCapabilityRequirement) -> bool {
    
    let claims = ::semio_framework_async::poll::resolve_ready(requirement.claims());
    let values = |namespace: &str| claims.iter().filter(|claim| claim.namespace().as_str() == namespace).map(|claim| claim.value().to_string()).collect::<BTreeSet<_>>();
    let value = |namespace: &str| values(namespace).into_iter().next().unwrap_or_default();
    match ::semio_framework_async::poll::resolve_ready(requirement.kind()).as_str() {
        "schema" => match value("schema-export").split_once('#') {
            Some((scope, export)) => semio_framework_schema_registry::resolve_schema_export(scope, export, semio_framework_schema_registry::SchemaFormat::JsonSchema).is_ok(),
            None => semio_framework_schema_registry::artifact_schema_descriptor_registered(&value("schema")),
        },
        "inference" => semio_framework_schema_registry::artifact_inference_descriptor_registered(&value("schema")),
        "codec" => ::semio_framework_async::poll::resolve_ready(semio_framework_os_kernel::document_codec(&value("codec"))).expect("the document codec registry").is_some(),
        "composer" => ::semio_framework_async::poll::resolve_ready(semio_framework::io::list_composer_entries()).expect("the composer registry").iter().any(|(writes, _)| writes.to_coordinate() == value("dialect")),
        "subset-validator" => ::semio_framework_async::poll::resolve_ready(semio_framework::io::list_registered_subset_validator_dialects()).expect("the subset validator registry").into_iter().any(|dialect| semio_framework::ArtifactDialect::from(dialect).to_coordinate() == value("validated-dialect")),
        "representation" => values("extension").iter().filter_map(|extension| semio_framework::io::format_descriptor(extension.trim_start_matches('.')).expect("the format catalog")).any(|format| format.mimes.iter().cloned().collect::<BTreeSet<_>>() == values("mime") && format.extensions.iter().cloned().collect::<BTreeSet<_>>() == values("extension")),
        "grammar" => true,
        other => panic!("unknown runtime capability category {other}"),
    }
}

/// 🧪️ The environment variable naming the one package [`package_contract_probe`] assembles in its child process.
const CONTRACT_PROBE_PACKAGE: &str = "SEMIO_STDIO_CONTRACT_PROBE_PACKAGE";

/// 🔗️ LAW (e): in every stdio package's own process — assembled alone, as its wasm guest is — every registered artifact's
/// snapshot contract compiles: each `$ref` it makes resolves against a schema document registered in that process. Measured
/// before (LB2 scratch, 2026-09-29): las/dwg/ifc refs named absent `$defs`, and semio brep/object/kit `$ref` shared documents
/// (`base/geometry.json`, `base/child.json` → `os/store/child/schema.json`, `brep/inference.json`) that no guest registered — every
/// snapshot edit on those kinds was refused `snapshot-edit.invalid-schema-contract`.
#[test]
fn every_registered_snapshot_contract_resolves_in_each_package_process() {
    let binary = std::env::current_exe().expect("the test binary");
    let failures = PACKAGE_IDS
        .into_iter()
        .filter_map(|id| {
            eprintln!("[DEBUG] primary shipped schema contract package={id} started");
            let run = std::process::Command::new(&binary).args(["package_contract_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(CONTRACT_PROBE_PACKAGE, id).output().expect("the contract probe runs");
            eprintln!("[DEBUG] primary shipped schema contract package={id} completed status={}", run.status);
            if run.status.success() { eprint!("{}{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)); }
            (!run.status.success()).then(|| format!("{id} alone registers an unresolvable snapshot contract:\n{}\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))
        })
        .collect::<Vec<_>>();
    assert!(failures.is_empty(), "{} of {} packages fail:\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\n"));
}

/// 🔬️ The child half of LAW (e): assembles only the package [`CONTRACT_PROBE_PACKAGE`] names and compiles the snapshot
/// contract of every artifact schema registered in this process — owned and hosted alike.
#[test]
#[ignore = "the child process of every_registered_snapshot_contract_resolves_in_each_package_process"]
fn package_contract_probe() {
    let id = std::env::var(CONTRACT_PROBE_PACKAGE).expect("the parent law names one package");
    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));
    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");
    let contracts = semio_framework_schema_registry::with_artifact_schema_catalog(|entries| entries.iter().map(|entry| entry.id).collect::<Vec<_>>());
    assert!(!contracts.is_empty(), "{id} registers the snapshot contracts of the kinds it opens");
    let unresolved = contracts.iter().filter_map(|contract| {
        eprintln!("[DEBUG] primary shipped schema contract package={id} contract={contract} started");
        let result = semio_framework_schema::structural_validator_for(contract, "snapshot");
        eprintln!("[DEBUG] primary shipped schema contract package={id} contract={contract} completed accepted={}", result.is_ok());
        result.err().map(|error| format!("{contract}: {error}"))
    }).collect::<Vec<_>>();
    assert!(unresolved.is_empty(), "{id} alone registers {} unresolvable snapshot contracts: {unresolved:#?}", unresolved.len());
}

/// 🏠️ LAW (describe side of hosting, p15): every stdio FAMILY package owns no artifact kind and its descriptor hosts every
/// kind it opens, each hosted row a document codec its owner's runtime declaration declares (kind × codec schema), so a trusted
/// catalog routes the owner's documents to the family's editors and binds the owner's codec; every row names its owner `stdio`
/// explicitly; the owner `stdio` hosts nothing.
#[test]
fn every_family_descriptor_hosts_exactly_its_owners_codecs_for_the_kinds_it_opens() {
    let assemblies = semio_hub_stdio::catalog::artifact_assemblies().expect("the stdio artifact assemblies");
    let owner_codecs = assemblies
        .iter()
        .filter_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) => Some(declaration),
            _ => None,
        })
        .flat_map(|declaration| declaration.hosted_kinds().expect("a runtime declaration names its canonical owner"))
        .map(|kind| (kind.id, kind.schema, kind.owner))
        .collect::<BTreeSet<_>>();
    for package in packages() {
        let hosted = package.descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.clone(), kind.schema.clone(), kind.owner.clone())).collect::<BTreeSet<_>>();
        if package.id == "stdio" {
            assert!(hosted.is_empty(), "stdio owns its kinds and hosts none: {hosted:?}");
            continue;
        }
        assert!(package.descriptor.manifest.artifact_kinds.is_empty(), "{} owns no artifact kind", package.id);
        assert!(hosted.is_subset(&owner_codecs), "{} hosts rows its owner declares no codec for: {:?}", package.id, hosted.difference(&owner_codecs).collect::<Vec<_>>());
        assert!(hosted.iter().all(|(_, _, owner)| owner == "stdio"), "{} hosts rows of an owner other than stdio: {hosted:?}", package.id);
        let hosted_ids = hosted.iter().map(|(id, _, _)| id.as_str()).collect::<BTreeSet<_>>();
        for kind in activated_kinds(&package.descriptor) {
            assert!(hosted_ids.contains(kind.as_str()), "{} opens {kind} without hosting it", package.id);
        }
    }
}

/// 🪶️ Every selected Runtime Snapshot schema requires its actual installed semantic SQLite owner.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_declared_owner_census() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🔣️.json")).expect("closed primary Snapshot census");
    let expected = law["artifactKinds"].as_array().unwrap().iter().map(|kind| kind.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    let definitions = semio_hub_stdio::catalog::artifact_definitions().expect("actual selected definitions");
    let assemblies = semio_hub_stdio::catalog::artifact_assemblies().expect("actual selected assemblies");
    assert_eq!(definitions.iter().map(|definition| definition.identity().as_str().to_string()).collect::<BTreeSet<_>>(), expected);
    assert_eq!(assemblies.iter().map(|assembly| assembly.definition().identity().as_str().to_string()).collect::<BTreeSet<_>>(), expected);
    assert_eq!(definitions.len(), expected.len());
    assert_eq!(assemblies.len(), expected.len());
    packages();
    let mut failures = Vec::new();
    let mut rows = BTreeSet::new();
    let mut metadata_only = 0;
    for assembly in &assemblies {
        match assembly {
            ArtifactAssembly::Definition(_) => metadata_only += 1,
            ArtifactAssembly::Runtime(declaration) => {
                let hosted = declaration.hosted_kinds().expect("canonical Runtime owner");
                if hosted.is_empty() { failures.push(format!("{} declares no executable Snapshot schema", declaration.definition().identity().as_str())); }
                rows.extend(hosted.into_iter().map(|row| (row.id, row.schema, row.owner)));
            }
        }
    }
    for (kind, schema, owner) in &rows {
        let Some(codec) = semio_framework_os_kernel::document_codec(schema).await.expect("actual document codec registry") else {
            failures.push(format!("{kind} schema={schema} owner={owner}: absent installed document codec"));
            continue;
        };
        assert_eq!(&codec.schema, schema, "registered codec schema identity");
        match codec.snapshot_sqlite.as_ref() {
            None => failures.push(format!("{kind} schema={schema} owner={owner}: absent semantic SQLite provider")),
            Some(provider) => {
                if provider.snapshot_type.is_none() { failures.push(format!("{kind} schema={schema}: absent concrete Snapshot TypeId")); }
                if provider.schema.trim().is_empty() { failures.push(format!("{kind} schema={schema}: empty handwritten SQL schema")); }
                eprintln!("[DEBUG] primary-stdio-snapshot-owner kind={kind} schema={schema} owner={owner} sql_bytes={} concrete_type={}", provider.schema.len(), provider.snapshot_type.is_some());
            }
        }
    }
    eprintln!("[DEBUG] primary-stdio-snapshot-census definitions={} assemblies={} runtime={} metadata_only={} distinct_schema_owners={} refusals={}", definitions.len(), assemblies.len(), assemblies.len()-metadata_only, metadata_only, rows.len(), failures.len());
    assert!(failures.is_empty(), "{} declared Snapshot owner refusals:\n{}", failures.len(), failures.join("\n"));
}

/// 🔗️ Every authored Runtime dialect retains its actual concrete owner in the installed codec and both SQLite routes.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_declared_binding_census() {
    use semio_framework::io::io_mechanism::{io_route, native_snapshot_sqlite_schema, preflight_native_snapshots, NativeSnapshotRegistration};
    use semio_framework::io_schema::{ArtifactDialect, IoFidelity, SQLITE_SNAPSHOT};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🔣️.json")).unwrap();
    let expected = law["dialectBindings"].as_array().unwrap().iter().map(|row| (row["kind"].as_str().unwrap().to_string(), row["standard"].as_str().unwrap().to_string(), row["subset"].as_str().unwrap().to_string(), row["schema"].as_str().unwrap().to_string())).collect::<BTreeSet<_>>();
    let assemblies = semio_hub_stdio::catalog::artifact_assemblies().expect("selected assemblies");
    let bindings = assemblies.iter().filter_map(|assembly| match assembly { ArtifactAssembly::Runtime(declaration) => Some(declaration.document_codec_bindings()), ArtifactAssembly::Definition(_) => None }).flatten().collect::<Vec<_>>();
    let actual = bindings.iter().map(|(dialect, codec)| (dialect.artifact_kind.to_string(), dialect.standard.0.to_string(), dialect.subset.0.to_string(), codec.schema.clone())).collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "authored dialect membership is independent of installed entries and editor membership");
    assert_eq!(bindings.len(), expected.len(), "no duplicate declared dialect bindings");
    packages();
    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
    let mut failures = Vec::new();
    for (declared, codec) in bindings {
        let native = ArtifactDialect::from(declared);
        let label = native.to_coordinate();
        let Some(expected_provider) = codec.snapshot_sqlite.clone() else { failures.push(format!("{label}: declared Snapshot has no semantic SQLite provider")); continue; };
        if expected_provider.snapshot_type.is_none() { failures.push(format!("{label}: declared provider erases concrete Snapshot TypeId")); }
        match semio_framework_os_kernel::document_codec(&codec.schema).await.unwrap() {
            Some(installed) if installed.snapshot_sqlite.as_ref().is_some_and(|provider| provider.identical_to(&expected_provider)) => {},
            _ => failures.push(format!("{label}: installed schema codec differs from declared Snapshot type/SQL/hooks")),
        }
        match native_snapshot_sqlite_schema(&native) {
            Ok(sql) if sql == expected_provider.schema.as_ref() => {},
            result => failures.push(format!("{label}: actual installed native SQL lookup differs or is absent: {result:?}")),
        }
        let registrations = NativeSnapshotRegistration::from_capability(native.clone(), codec).into_iter().collect::<Vec<_>>();
        if let Err(error) = preflight_native_snapshots(&registrations) { failures.push(format!("{label}: installed native provider identity conflicts: {error:?}")); }
        for (from, into) in [(&native, &sqlite), (&sqlite, &native)] {
            match io_route(from, into, 1).await {
                Ok(route) if route.value.fidelity == IoFidelity::Exact => {},
                result => failures.push(format!("{label}: absent exact bidirectional SQLite route: {result:?}")),
            }
        }
        eprintln!("[DEBUG] primary-stdio-snapshot-binding dialect={label} concrete_type={:?} sql_bytes={}", expected_provider.snapshot_type, expected_provider.schema.len());
    }
    eprintln!("[DEBUG] primary-stdio-snapshot-binding-census declared={} refusals={}", expected.len(), failures.len());
    assert!(failures.is_empty(), "{} installed binding refusals:\n{}", failures.len(), failures.join("\n"));
}
/// 🪶️ Independently reads every domain row of one actual exported SQLite file.
fn primary_snapshot_physical_rows(bytes: &[u8]) -> serde_json::Value {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));try{const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT IN ('semio_snapshot','sqlite_sequence') ORDER BY name").all();const domainRows={};for(const {name}of tables){if(!/^[a-z_]+$/.test(name))throw Error('unexpected domain table');domainRows[name]=db.query('SELECT * FROM "'+name+'" ORDER BY id').all();}console.log(JSON.stringify({integrity:db.query('PRAGMA integrity_check').all(),foreignKeys:db.query('PRAGMA foreign_key_check').all(),metadata:db.query('SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot ORDER BY id').all(),domainRows}));}finally{db.close();}"#;
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("real independent SQLite reader");
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    serde_json::from_slice(&result.stdout).unwrap()
}

/// 📸️ Runs ordinary public Binary/Text I/O and preserves the full concrete Snapshot owner.
async fn assert_primary_snapshot_payload<S>(snapshot: S, row: &serde_json::Value)
where S: semio_framework_os_kernel::ArtifactPack + semio_framework_os_kernel::ArtifactDsl + PartialEq + std::fmt::Debug {
    use semio_framework::io::io_mechanism::{io_identify, io_route, io_run};
    use semio_framework::io_schema::{ArtifactDialect, Confidence, IoFidelity, IoPayload, SQLITE_SNAPSHOT};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    packages();
    let native = ArtifactDialect { artifact_kind: row["kind"].as_str().unwrap().into(), standard: row["standard"].as_str().unwrap().into(), subset: row["subset"].as_str().unwrap().into() };
    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
    let export = io_route(&native, &sqlite, 1).await.unwrap().value;
    let import = io_route(&sqlite, &native, 1).await.unwrap().value;
    assert_eq!(export.fidelity, IoFidelity::Exact);
    assert_eq!(import.fidelity, IoFidelity::Exact);
    for payload in [IoPayload::Binary(snapshot.encode_pack()), IoPayload::Text(snapshot.print_dsl())] {
        let file = io_run(&export, payload.clone()).await.expect("actual public SQLite export").value;
        let IoPayload::Binary(bytes) = &file else { panic!("SQLite file must be binary") };
        assert!(bytes.starts_with(b"SQLite format 3\0"));
        let evidence = primary_snapshot_physical_rows(bytes);
        assert_eq!(evidence["integrity"], law["integrity"]);
        assert_eq!(evidence["foreignKeys"], law["foreignKeys"]);
        assert_eq!(evidence["domainRows"], row["domainRows"], "complete authored domain rows and values");
        let encoding = match &payload { IoPayload::Binary(_) => "binary", IoPayload::Text(_) => "text" };
        assert_eq!(evidence["metadata"], serde_json::json!([{"artifact_kind":native.artifact_kind,"standard":native.standard,"subset":native.subset,"schema_version":law["schemaVersion"],"native_encoding":encoding}]));
        assert_eq!(io_identify(&file).await, vec![(sqlite.clone(), Confidence::High)]);
        let byte_length = bytes.len();
        let restored = io_run(&import, file).await.expect("actual public SQLite import").value;
        assert_eq!(restored, payload, "complete native wire");
        let decoded = match restored { IoPayload::Binary(bytes) => S::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => S::parse_dsl(&text).unwrap() };
        assert_eq!(decoded, snapshot, "full concrete Snapshot equality");
        eprintln!("[DEBUG] primary-stdio-public-snapshot dialect={} native={} bytes={} complete_domain_rows=true full_owner=true", native.to_coordinate(), encoding, byte_length);
    }
}

/// 🌳️ The actual public declaration trees agree with the Runtime owner installed by normal builders.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_new_declaration_trees() {
    async fn check<PA: PluginApp>(tree: semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>, expected: &serde_json::Value) {
        use semio_framework::io::io_mechanism::native_snapshot_sqlite_schema;
        assert_eq!(tree.kind.as_str(), expected["kind"].as_str().unwrap());
        assert_eq!(tree.standards.len(), 1, "complete authored public tree standard roster");
        for standard in tree.standards {
            assert_eq!(standard.id.0, expected["standard"].as_str().unwrap());
            assert_eq!(standard.subsets.len(), 1, "complete authored public tree subset roster");
            for subset in standard.subsets {
                assert_eq!(subset.dialect.artifact_kind, expected["kind"].as_str().unwrap());
                assert_eq!(subset.dialect.standard.0, expected["standard"].as_str().unwrap());
                assert_eq!(subset.dialect.subset.0, expected["subset"].as_str().unwrap());
                let codec = subset.io.native.codec;
                assert_eq!(codec.schema, expected["schema"].as_str().unwrap());
                let declared = codec.snapshot_sqlite.as_ref().expect("public tree semantic SQLite owner");
                assert!(declared.snapshot_type.is_some());
                let installed = semio_framework_os_kernel::document_codec(&codec.schema).await.unwrap().expect("installed public tree document codec");
                assert!(installed.snapshot_sqlite.as_ref().unwrap().identical_to(declared));
                assert_eq!(native_snapshot_sqlite_schema(&subset.dialect.into()).unwrap(), declared.schema.as_ref());
            }
        }
    }
    packages();
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    check(semio_s_artifact_stdio_binary::artifact(), &law["witnesses"][0]).await;
    check(semio_s_artifact_stdio_txt::artifact(), &law["witnesses"][1]).await;
}

/// 💾️ Binary's actual authored demo crosses both public native forms and all byte relations.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_binary_public_payload() {
    use semio_framework_os_kernel::ArtifactDsl;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][0];
    let snapshot = semio_s_artifact_stdio_binary::BinarySnapshot::parse_dsl(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(semio_s_artifact_stdio_binary::examples::demo::source().document(), row["naturalText"].as_str().unwrap());
    assert_primary_snapshot_payload(snapshot, row).await;
}

/// 🔤️ Txt's authored demo crosses both public native forms and all line relations.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_txt_public_payload() {
    use semio_framework_os_kernel::ArtifactDsl;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][1];
    let snapshot = semio_s_artifact_stdio_txt::TxtSnapshot::parse_dsl(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(semio_s_artifact_stdio_txt::examples::demo::source().document(), row["naturalText"].as_str().unwrap());
    assert_primary_snapshot_payload(snapshot, row).await;
}

/// 🌍️ The non-editor GeoJSON declaration retains its complete typed Feature owner and conformance.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_primary_geojson_public_payload() {
    use semio_s_artifact_stdio_json::JsonSnapshot;
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")).unwrap();
    let row = &law["witnesses"][2];
    let snapshot = JsonSnapshot::from_value(parse_json_text(row["naturalText"].as_str().unwrap()).unwrap());
    let independent: serde_json::Value = serde_json::from_str(row["naturalText"].as_str().unwrap()).unwrap();
    assert_eq!(independent, row["logicalValue"]);
    assert_eq!(snapshot.to_serde_value(), independent);
    assert!(semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance(&snapshot).iter().all(|diagnostic| !matches!(diagnostic.severity, semio_framework::Severity::Error | semio_framework::Severity::Fatal)));
    assert_primary_snapshot_payload(snapshot, row).await;
    use semio_framework_os_kernel::{ArtifactDsl, ArtifactPack};
    let native = semio_framework::io_schema::ArtifactDialect { artifact_kind: row["kind"].as_str().unwrap().into(), standard: row["standard"].as_str().unwrap().into(), subset: row["subset"].as_str().unwrap().into() };
    let export = semio_framework::io::io_mechanism::io_route(&native, &semio_framework::io_schema::SQLITE_SNAPSHOT.into(), 1).await.unwrap().value;
    for invalid in row["invalidNaturalTexts"].as_array().unwrap() {
        let invalid = JsonSnapshot::from_value(parse_json_text(invalid.as_str().unwrap()).unwrap());
        for payload in [semio_framework::io_schema::IoPayload::Binary(invalid.encode_pack()), semio_framework::io_schema::IoPayload::Text(invalid.print_dsl())] {
            assert!(semio_framework::io::io_mechanism::io_run(&export, payload).await.is_err(), "valid JSON that violates GeoJSON must be refused by the actual public route");
        }
    }
    eprintln!("[DEBUG] primary-stdio-geojson invalid-native-forms=2 refused-by-public-route=true");
}
