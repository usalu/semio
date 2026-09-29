//! 🛡️ Every stdio package ships a bounded, declared app fleet, and together the stdio packages ship every stdio app
//! and open every stdio artifact kind exactly once.

use semio_framework::{AppRole, PackageDescriptor};
use semio_framework_plugin::kernel::ActivationEvent;
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};
use semio_framework_plugin::{Plugin, PluginApp, PluginAssemblyError};
use semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS;
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
    install_plugin_bundle_result(&runtime, bundle);
    let bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
    semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect("descriptor bytes")).expect("strict descriptor")
}

/// 📦️ The stdio component and its nine family components, each described once.
fn packages() -> &'static [ShippedPackage] {
    static PACKAGES: OnceLock<Vec<ShippedPackage>> = OnceLock::new();
    PACKAGES.get_or_init(|| {
        vec![
        ShippedPackage { id: "stdio", manifest: include_str!("../../📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio::plugin()) },
        ShippedPackage { id: "stdio-image", manifest: include_str!("../../🧩️extensions/🖼️image/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_image::plugin()) },
        ShippedPackage { id: "stdio-media", manifest: include_str!("../../🧩️extensions/🎵️media/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_media::plugin()) },
        ShippedPackage { id: "stdio-cad", manifest: include_str!("../../🧩️extensions/🛠️cad/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_cad::plugin()) },
        ShippedPackage { id: "stdio-bim", manifest: include_str!("../../🧩️extensions/🏠️bim/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_bim::plugin()) },
        ShippedPackage { id: "stdio-mesh", manifest: include_str!("../../🧩️extensions/🔺️mesh/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_mesh::plugin()) },
        ShippedPackage { id: "stdio-pdf", manifest: include_str!("../../🧩️extensions/📘️pdf/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_pdf::plugin()) },
        ShippedPackage { id: "stdio-office", manifest: include_str!("../../🧩️extensions/💼️office/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_office::plugin()) },
        ShippedPackage { id: "stdio-semio", manifest: include_str!("../../🧩️extensions/🧿️semio/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_semio::plugin()) },
        ShippedPackage { id: "stdio-binary", manifest: include_str!("../../🧩️extensions/🔢️binary/📦️packages/🦀️rust/Cargo.toml"), descriptor: describe(semio_s_plugin_stdio_binary::plugin()) },
        ]
    })
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
    for package in packages() {
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
    use semio_framework_os_kernel::DslValue;
    for package in packages() {
        for app in package.descriptor.manifest.apps.iter().filter(|app| app.role == AppRole::Editor) {
            let kind = app.id.split('@').next().expect("an app id names its artifact kind");
            assert!(semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(kind), "{} ({}) edits {kind}, whose document schema its package's assembly must publish", app.id, package.id);
        }
    }
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>, _>(async {
        semio_framework_plugin::App { definition: semio_s_artifact_stdio_json::editor::json_any::create_json_editor(), examples: Vec::new() }
    })
    .await;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let source = r#"{"edited":["node",1,true]}"#;
    let args = DslValue::Object(vec![("nodeId".into(), DslValue::String("$".into())), ("revision".into(), DslValue::String(revision)), ("value".into(), DslValue::String(source.into()))]);
    app.handle_action("set-node", Some(&args), &meta).await.expect("the shipped json editor admits a revision-bound root node edit");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("the node edit publishes");
    let edited = semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::write_json_text(&app.snapshot().expect("json snapshot").value);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&edited).expect("the edited document is JSON"), serde_json::from_str::<serde_json::Value>(source).expect("serde_json oracle"), "the published document is exactly the applied source");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
