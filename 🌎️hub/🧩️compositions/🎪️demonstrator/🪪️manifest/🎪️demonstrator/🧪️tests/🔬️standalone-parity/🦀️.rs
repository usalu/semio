use super::*;
use semio_plugin::ArtifactApp;
use std::collections::BTreeSet;

fn test_bundle() -> Plugin<DemonstratorApps> {
    plugin().unwrap_or_else(|error| panic!("{error}"))
}

fn standalone<PA: PluginApp>(owner: &str, built: Result<Plugin<PA>, PluginAssemblyError>) -> Plugin<PA> {
    built.unwrap_or_else(|error| panic!("standalone {owner} host assembles: {error}"))
}

/// 🧮️ The quoted entries of `key = [..]` inside `[package.metadata.semio]` of a hub component manifest — empty when the row is absent.
fn semio_row(cargo: &str, key: &str) -> BTreeSet<String> {
    let section = cargo.split("[package.metadata.semio]").nth(1).and_then(|rest| rest.split("\n[").next()).unwrap_or_default();
    let Some(start) = section.lines().position(|line| line.trim_start().starts_with(key) && line.contains('=')) else { return BTreeSet::new() };
    let mut body = String::new();
    for line in section.lines().skip(start) {
        let code = line.split('#').next().unwrap_or_default();
        body.push_str(code);
        body.push('\n');
        if code.contains(']') {
            break;
        }
    }
    body.split('"').skip(1).step_by(2).map(str::to_string).collect()
}

fn foreign_ids(bundle: &Plugin<DemonstratorApps>) -> BTreeSet<String> {
    bundle.manifest.apps.iter().filter(|app| !app.id.starts_with("s.demonstrator.")).map(|app| app.id.clone()).collect()
}

/// 🪞️ Every surface the standalone host registers that the bundle also registers is the same definition — commands, tool catalog,
/// contribution consumers, modes, io — except the kind specs, which a hosting plugin moves into `hosted_artifact_kinds` rows naming the owner.
fn assert_surfaces_match<PA: PluginApp>(bundle: &Plugin<DemonstratorApps>, host: &Plugin<PA>) -> BTreeSet<String> {
    let owner = host.manifest.plugin_id.as_str();
    let mut matched = BTreeSet::new();
    for theirs in &host.manifest.apps {
        let Some(ours) = bundle.manifest.apps.iter().find(|app| app.id == theirs.id) else { continue };
        let mut expected = theirs.clone();
        let kinds = std::mem::take(&mut expected.artifact_kinds);
        assert_eq!(*ours, expected, "bundle surface {} differs from the standalone {owner} host's", theirs.id);
        for kind in kinds {
            assert!(
                bundle.manifest.hosted_artifact_kinds.iter().any(|row| row.id == kind.id && row.schema == kind.schema && row.owner == owner),
                "bundle does not host {} ({}) of {} as a row owned by {owner}",
                kind.id,
                kind.schema,
                theirs.id
            );
        }
        matched.insert(theirs.id.clone());
    }
    matched
}

#[test]
fn every_foreign_surface_is_defined_exactly_like_its_standalone_host() {
    let bundle = test_bundle();
    let mut matched = BTreeSet::new();
    for (owner, found) in [
        ("cad", assert_surfaces_match(&bundle, &standalone("cad", semio_hub_cad::plugin::plugin()))),
        ("gis", assert_surfaces_match(&bundle, &standalone("gis", semio_hub_gis::plugin()))),
        ("procedural", assert_surfaces_match(&bundle, &standalone("procedural", semio_hub_procedural::plugin()))),
        ("process", assert_surfaces_match(&bundle, &standalone("process", semio_hub_process::plugin::plugin()))),
        ("puzzle", assert_surfaces_match(&bundle, &standalone("puzzle", semio_hub_puzzle::plugin()))),
        ("sourcing", assert_surfaces_match(&bundle, &standalone("sourcing", semio_hub_sourcing::plugin::plugin()))),
    ] {
        assert!(!found.is_empty(), "standalone {owner} host contributes no surface to the bundle");
        assert!(matched.is_disjoint(&found), "{owner} claims a surface another standalone host already owns");
        matched.extend(found);
    }
    assert_eq!(matched, foreign_ids(&bundle), "every foreign bundle surface has exactly one standalone host");
}

/// 🌱️ The member roster is the app's own: each bundled surface type is the exact variant its standalone fleet enum carries.
#[test]
fn every_bundled_surface_type_is_the_variant_its_standalone_fleet_carries() {
    fn carried<Variant, Fleet: From<Variant>>() {}
    carried::<EditorSurfaceApp<CadPlayApp>, semio_hub_cad::plugin::CadApps>();
    carried::<EditorSurfaceApp<Gis2dPlayApp>, semio_hub_gis::GisApps>();
    carried::<EditorSurfaceApp<Generation3dPlayApp>, semio_hub_procedural::ProceduralApps>();
    carried::<EditorSurfaceApp<Process3dPlayApp>, semio_hub_process::plugin::ProcessApps>();
    carried::<ViewerSurfaceApp<Process3dViewer>, semio_hub_process::plugin::ProcessApps>();
    carried::<EditorSurfaceApp<Puzzle3dPlayApp>, semio_hub_puzzle::PuzzleApps>();
    carried::<EditorSurfaceApp<SourcingCurationApp>, semio_hub_sourcing::SourcingApps>();
    carried::<ViewerSurfaceApp<SourcingViewer>, semio_hub_sourcing::SourcingApps>();
}

/// 🛠️ Every bundled app owns a typed-action producer, so no pane of the bundle refuses its own actions.
#[test]
fn every_bundled_surface_declares_an_original_action_producer() {
    fn produces<A: ArtifactApp>(id: &str) {
        assert!(A::original_action_command_producer().is_some(), "{id} has no original native action command producer");
    }
    produces::<EditorApp<crate::editor::playground::PlaygroundEditor>>("s.demonstrator.playground@1/*#editor");
    produces::<ViewerApp<crate::viewer::playground::PlaygroundViewer>>("s.demonstrator.playground@1/*#viewer");
    produces::<EditorApp<Generation3dPlayApp>>("s.procedural.generation3d@1/*#editor");
    produces::<EditorApp<CadPlayApp>>("s.cad.cad@1/*#editor");
    produces::<EditorApp<Puzzle3dPlayApp>>("s.puzzle.puzzle3d@1/*#editor");
    produces::<EditorApp<SourcingCurationApp>>("s.sourcing.curation@1/*#editor");
    produces::<ViewerApp<SourcingViewer>>("s.sourcing.curation@1/*#viewer");
    produces::<EditorApp<Process3dPlayApp>>("s.process.process3d@1/*#editor");
    produces::<ViewerApp<Process3dViewer>>("s.process.process3d@1/*#viewer");
    produces::<EditorApp<Gis2dPlayApp>>("s.gis.gismap@1/*#editor");
}

const UNBUNDLED_REQUESTS: [&str; 1] = ["shell.clipboard"];

/// 🙏️ The capability asks of the bundled apps are the asks their standalone hosts make; `shell.clipboard` belongs to the puzzle5d surface this bundle does not carry.
#[test]
fn bundle_asks_for_the_capabilities_its_standalone_hosts_ask_for() {
    let bundle = test_bundle();
    let hosts = [
        standalone("cad", semio_hub_cad::plugin::plugin()).descriptor_extras().capability_requests.clone(),
        standalone("gis", semio_hub_gis::plugin()).descriptor_extras().capability_requests.clone(),
        standalone("procedural", semio_hub_procedural::plugin()).descriptor_extras().capability_requests.clone(),
        standalone("process", semio_hub_process::plugin::plugin()).descriptor_extras().capability_requests.clone(),
        standalone("puzzle", semio_hub_puzzle::plugin()).descriptor_extras().capability_requests.clone(),
        standalone("sourcing", semio_hub_sourcing::plugin::plugin()).descriptor_extras().capability_requests.clone(),
    ];
    let asked: BTreeSet<String> = hosts.iter().flatten().map(|request| request.id.0.clone()).filter(|id| !UNBUNDLED_REQUESTS.contains(&id.as_str())).collect();
    let ours = bundle.descriptor_extras().capability_requests.clone();
    assert_eq!(ours.iter().map(|request| request.id.0.clone()).collect::<BTreeSet<_>>(), asked);
    for request in &ours {
        let theirs: Vec<_> = hosts.iter().flatten().filter(|theirs| theirs.id == request.id).collect();
        assert!(!theirs.is_empty(), "request {} has no standalone declaration to mirror", request.id.0);
        assert!(theirs.iter().all(|theirs| theirs.scope == request.scope && theirs.optional == request.optional), "request {} differs in scope or optionality from a standalone host's", request.id.0);
    }
}

/// 🧭️ Contribution scoping is keyed by the receiver's `consumes` row: the bundle consumes exactly the union of its standalone hosts' rows.
#[test]
fn bundle_consumes_the_union_of_its_standalone_hosts_consumes() {
    let hosts = [
        include_str!("../../../../../📐️cad/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
        include_str!("../../../../../🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
        include_str!("../../../../../🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
        include_str!("../../../../../🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
        include_str!("../../../../../🧩️puzzle/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
        include_str!("../../../../../🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml"),
    ];
    let bundle = include_str!("../../../../../🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml");
    let union: BTreeSet<String> = hosts.iter().flat_map(|cargo| semio_row(cargo, "consumes")).collect();
    assert!(!union.is_empty());
    assert_eq!(semio_row(bundle, "consumes"), union);
}

const UNMATERIALIZED_DEPENDENCIES: [&str; 1] = ["flow-extension-draw"];

/// 🔗️ The bundle loads the six plugins it hosts plus every flow extension the procedural host loads, less the ones its apps never use.
#[test]
fn bundle_depends_on_what_its_standalone_hosts_depend_on() {
    let procedural = semio_row(include_str!("../../../../../🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml"), "depends-on");
    let bundle = semio_row(include_str!("../../../../../🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml"), "depends-on");
    let mut expected: BTreeSet<String> = ["cad", "gis", "procedural", "process", "puzzle", "sourcing"].into_iter().map(str::to_string).collect();
    expected.extend(procedural.into_iter().filter(|dependency| !UNMATERIALIZED_DEPENDENCIES.contains(&dependency.as_str())));
    assert_eq!(bundle, expected);
    let declared: BTreeSet<String> = test_bundle().manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect();
    assert!(["cad", "gis", "procedural", "process", "puzzle", "sourcing"].iter().all(|owner| declared.contains(*owner) && bundle.contains(*owner)));
}

#[test]
fn semio_row_reads_inline_and_multiline_rows() {
    let cargo = "[package.metadata.semio]\nconsumes = [\"a\", \"b\"]\ndepends-on = [\n  \"c\", # note\n  \"d\",\n]\n\n[lib]\nconsumes = [\"z\"]\n";
    assert_eq!(semio_row(cargo, "consumes"), BTreeSet::from(["a".to_string(), "b".to_string()]));
    assert_eq!(semio_row(cargo, "depends-on"), BTreeSet::from(["c".to_string(), "d".to_string()]));
    assert!(semio_row(cargo, "absent").is_empty());
}
