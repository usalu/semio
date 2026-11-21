use super::*;
use serde_json::Value;

fn ids(values: &Value) -> Vec<&str> {
    values.as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect()
}

fn action(id: &str) -> ActionDefinition {
    let mut definition = ActionDefinition::new(id, LocalizedLabel::data(id), ActionKind::View, "pencil");
    definition.semantics.execution.interactive_job = semio_framework::manifest::InteractiveJobClassification::ForbiddenFromUi;
    definition
}

async fn builder() -> AppBuilder {
    let dialect = ArtifactDialect { artifact_kind: "s.test.declaration-collections".into(), standard: "1".into(), subset: "*".into() };
    AppBuilder::new(surface_app_id(&dialect, AppRole::Editor), LocalizedLabel::data("Declarations"))
        .await
        .document(["semio", "declarations"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "declarations.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await
}

#[semio_framework_async_macros::async_test]
async fn app_declaration_collections_keep_authored_order_and_replacement() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/declaration-collections.json")).unwrap();
    let mut authored = builder().await;
    let action_ids = ids(&fixture["actions"]);
    for id in &action_ids {
        authored = authored.action_with(action(id)).await;
    }
    authored = authored.window_kind_actions("main", ids(&fixture["windowActions"]).into_iter().map(action).collect()).await;
    for id in ids(&fixture["terminologyIds"]) {
        authored = authored.terminology(id).await;
    }
    for row in fixture["breadcrumbs"].as_array().unwrap() {
        authored = authored.terminology_document(row["id"].as_str().unwrap(), [LocalizedLabel::data(row["breadcrumb"].as_str().unwrap())]).await;
    }
    let app = authored.try_build_definition().unwrap();
    let app_actions: Vec<_> = app.actions.iter().map(|entry| entry.id.as_str()).filter(|id| action_ids.contains(id)).collect();
    assert_eq!(app_actions, ids(&fixture["expectedAppActions"]));
    assert_eq!(app.terminologies, ids(&fixture["expectedTerminologies"]));
    assert_eq!(app.terminology_breadcrumbs.len(), fixture["expectedBreadcrumbs"].as_array().unwrap().len());
    for ((id, labels), expected) in app.terminology_breadcrumbs.iter().zip(fixture["expectedBreadcrumbs"].as_array().unwrap()) {
        assert_eq!(id, expected["id"].as_str().unwrap());
        assert_eq!(labels, &[LocalizedLabel::data(expected["breadcrumb"].as_str().unwrap())]);
    }
}

#[semio_framework_async_macros::async_test]
async fn app_declaration_collections_reject_first_authored_duplicate() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/declaration-collections.json")).unwrap();
    let mut authored = builder().await;
    for id in ids(&fixture["duplicateActions"]) {
        authored = authored.action_with(action(id)).await;
    }
    let failure = authored.try_build_definition().err().expect("duplicate action declaration");
    assert!(failure.message.ends_with(&format!("duplicate action id {}", fixture["firstDuplicate"].as_str().unwrap())));
}
