use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window_keeping_the_kit_action() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.actions.iter().any(|action| action.id == "set-node"), "the kit's own edit action must survive");
    assert_eq!(def.actions.len(), 1 + actions().len());
}

#[semio_framework_async_macros::async_test]
async fn every_authored_action_is_localized_in_english_and_german() {
    for action in actions() {
        assert!(
            semio_ui_locale::Terminology::ALL.iter().all(|&terminology| action.label.resolve(terminology, semio_ui_locale::Locale::En) != action.label.resolve(terminology, semio_ui_locale::Locale::De)),
            "action {} is not really translated",
            action.id
        );
        for arg in &action.args {
            assert!(
                semio_ui_locale::Terminology::ALL.iter().all(|&terminology| arg.label.resolve(terminology, semio_ui_locale::Locale::En) != arg.label.resolve(terminology, semio_ui_locale::Locale::De)),
                "arg {} of action {} is not really translated",
                arg.id,
                action.id
            );
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn render_lists_name_version_and_every_collection_count() {
    let document = EnergyModelSnapshot::empty().unwrap();
    let tree = render(&document, semio_ui_locale::Locale::En).expect("the tree window assembles");
    assert_eq!(tree.key.as_str(), WINDOW_KIND_ID);
    let root = &tree.children[0].children[0];
    assert!(root.children.iter().any(|item| item.key.as_str() == "name"));
    assert!(root.children.iter().any(|item| item.key.as_str() == "version"));
    let geometry = root.children.iter().find(|item| item.key.as_str() == "geometry").expect("the geometry group is a root child");
    assert!(geometry.children.iter().any(|item| item.key.as_str() == "zones"));
}

#[semio_framework_async_macros::async_test]
async fn every_node_stays_under_the_tree_kit_sibling_ceiling() {
    let model = crate::model::Model::default();
    let groups = semio_s_artifact_energy_model_structure::energy_structure_overview(&model);
    let root_children = 3 + groups.len();
    assert!(root_children <= semio_s_artifact_energy_model_structure::STRUCTURE_TREE_SIBLING_CEILING, "root has {root_children} children");
    for group in &groups {
        assert!(group.counts.len() <= semio_s_artifact_energy_model_structure::STRUCTURE_TREE_SIBLING_CEILING, "group {} has {} leaves", group.id, group.counts.len());
    }
    let mut ids: Vec<&str> = groups.iter().flat_map(|group| group.counts.iter().map(|(id, _)| *id)).collect();
    let total = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), total, "a collection is listed twice");
    assert_eq!(total, 35, "every collection on the model is listed exactly once");
}

#[semio_framework_async_macros::async_test]
async fn the_structure_tree_speaks_english_and_german() {
    let model = crate::model::Model::default();
    let labels = |locale| {
        fn walk(node: &semio_s_artifact_energy_model_structure::TreeNodeView, out: &mut Vec<String>) {
            out.push(node.label.clone());
            node.children.iter().for_each(|child| walk(child, out));
        }
        let mut out = Vec::new();
        semio_s_artifact_energy_model_structure::energy_structure_tree(&model, locale).roots.iter().for_each(|root| walk(root, &mut out));
        out
    };
    let (en, de) = (labels(semio_ui_locale::Locale::En), labels(semio_ui_locale::Locale::De));
    assert!(en.iter().any(|label| label == "surfaces: 0") && en.iter().any(|label| label == "Geometry: 0"));
    assert!(de.iter().any(|label| label == "Flächen: 0") && de.iter().any(|label| label == "Geometrie: 0"));
    assert!(de.iter().all(|label| !label.starts_with("surfaces:")));
}
