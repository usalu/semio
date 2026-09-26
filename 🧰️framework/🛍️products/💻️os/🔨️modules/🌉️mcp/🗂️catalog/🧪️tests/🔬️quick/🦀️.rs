
use super::*;
use crate::source_builders;

#[test]
fn compiling_the_same_source_twice_is_byte_identical() {
    let source = source_builders::note_and_cad_source();
    let first = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let second = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    assert_eq!(first.hash, second.hash);
    assert_eq!(first.entries, second.entries);
}

#[test]
fn entries_are_sorted_by_id() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let ids: Vec<&str> = catalog.entries.iter().map(|entry| entry.id.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted);
}

/// 🆔️ D3: two plugins declaring the SAME bare action id must compile into two distinct
/// capability ids — a bare action id is never a capability id.
#[test]
fn two_plugins_declaring_the_same_action_id_compile_to_distinct_capability_ids() {
    let source = source_builders::colliding_action_id_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles distinct ids without error");
    assert!(catalog.get("plugin-a.surface.deleteSelection").is_some());
    assert!(catalog.get("plugin-b.surface.deleteSelection").is_some());
    assert_ne!(catalog.get("plugin-a.surface.deleteSelection").unwrap().id, catalog.get("plugin-b.surface.deleteSelection").unwrap().id);
}

#[test]
fn cad_translate_selection_compiles_with_the_dxyz_input_schema() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let capability = catalog.get("cad.editor.translateSelection").expect("translateSelection present");
    assert_eq!(capability.kind, CapabilityKind::Mutation);
    let properties = capability.input_schema["properties"].as_object().expect("object schema");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    assert_eq!(capability.id.as_str(), fixture["catalog"]["capabilityId"].as_str().expect("fixture capability id"));
    assert_eq!(properties, fixture["catalog"]["properties"].as_object().expect("fixture properties"));
}

#[test]
fn framework_actions_dedupe_into_one_entry_per_id_across_both_apps() {
    let source = source_builders::note_and_cad_source();
    let catalog = compile(&source, Locale::En, Terminology::Native).expect("compiles");
    let framework_undo_count = catalog.entries.iter().filter(|entry| entry.id.as_str() == "framework.undo").count();
    assert_eq!(framework_undo_count, 1);
    assert!(matches!(catalog.get("framework.undo").unwrap().owner, CapabilityOwner::Framework));
}

#[test]
fn duplicate_capability_id_is_rejected() {
    let mut source = source_builders::note_and_cad_source();
    let duplicate = source.gateway.first().cloned();
    if let Some(capability) = duplicate {
        source.gateway.push(capability);
    } else {
        source.gateway.push(ui_dialog_open_capability(&[]));
        source.gateway.push(ui_dialog_open_capability(&[]));
    }
    let result = compile(&source, Locale::En, Terminology::Native);
    assert!(matches!(result, Err(CatalogError::DuplicateCapabilityId(_))));
}

/// 💬️ The `CapabilityDescription` law cases (`🧫️fixtures/💬️capability-description.json`, schema
/// `🛂️manifest` `CapabilityDescriptionFixture`) replay row for row: `description_problems` reports
/// exactly the declared `(verb, problem)` list — the TypeScript twin replays the same file with AJV.
#[test]
fn description_problems_match_the_language_agnostic_fixture() {
    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!("../../🧫️fixtures/💬️capability-description.json")).expect("the fixture is JSON");
    assert!(rows.len() >= 10);
    for row in rows {
        let verbs: Vec<DescribedVerb> = row["verbs"]
            .as_array()
            .expect("verbs")
            .iter()
            .map(|verb| DescribedVerb {
                id: verb["id"].as_str().expect("id").to_string(),
                title_en: verb["title"]["en"].as_str().expect("title.en").to_string(),
                title_de: verb["title"]["de"].as_str().expect("title.de").to_string(),
                description: Some(verb["description"].clone()).filter(|description| !description.is_null()),
            })
            .collect();
        let expected: Vec<(String, String)> = row["problems"].as_array().expect("problems").iter().map(|problem| (problem["verb"].as_str().expect("verb").to_string(), problem["problem"].as_str().expect("problem").to_string())).collect();
        let actual: Vec<(String, String)> = description_problems(&verbs).into_iter().map(|(verb, problem)| (verb, problem.as_str().to_string())).collect();
        assert_eq!(actual, expected, "{}", row["name"]);
    }
}

/// 🧾️ The census names every undescribed agent verb of a source by its capability id, including the
/// framework-injected ones, and a source whose verbs all explain themselves is clean.
#[test]
fn the_description_census_reports_undescribed_agent_verbs_by_capability_id() {
    let mut source = source_builders::note_cad_and_draw_source();
    let before = description_findings(&source);
    assert!(before.iter().all(|finding| !finding.capability_id.starts_with("framework.undo")), "undo is described by the framework: {before:?}");
    for descriptor in source.descriptors.iter_mut().filter(|descriptor| descriptor.manifest.plugin_id == "draw") {
        for app in descriptor.manifest.apps.iter_mut() {
            for window_kind in app.window_kinds.iter_mut() {
                for action in window_kind.actions.iter_mut().filter(|action| action.id == "addLayer") {
                    action.semantics.description = None;
                }
            }
        }
    }
    let after = description_findings(&source);
    assert!(after.contains(&DescriptionFinding { capability_id: "draw.s.draw.drawing@1/*#editor.addLayer".into(), problem: DescriptionProblem::Missing }), "{after:?}");
    assert_eq!(after.len(), before.len() + 1, "{after:?}");
}
