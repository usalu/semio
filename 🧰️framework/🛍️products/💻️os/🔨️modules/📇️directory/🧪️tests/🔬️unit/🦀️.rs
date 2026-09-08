use super::*;

async fn fixture_events() -> Vec<DirectoryEvent> {
    #[derive(semio_framework_value_derive::FromValue)]
    struct Fixture {
        events: Vec<DirectoryEvent>,
    }
    let raw = include_str!("../../../../🧫️fixtures/📇️directory/⚡️events.json");
    crate::os_pack::json::from_json_str::<Fixture>(raw).expect("fixture decodes").events
}

#[semio_framework_async_macros::async_test]
async fn folds_the_golden_fixture_into_the_expected_projection() {
    let model = fold_all(DirectoryReadModel::default(), &fixture_events().await).await;

    assert_eq!(model.cursor, 16, "cursor tracks the last folded seq");
    assert_eq!(model.spaces.len(), 1, "the deleted atelier leaves only the studio");
    assert!(!model.spaces.contains_key("sp-atelier-amara"), "space.deleted removes the entry entirely");

    let studio = model.spaces.get("sp-studio-fabrication").expect("studio survives");
    assert_eq!(studio.view.name, "Fabrication Studio");
    assert_eq!(studio.view.visibility, DirectorySpaceVisibility::Public);
    assert_eq!(studio.view.kind, DirectorySpaceKind::Archive, "space.archived sets kind archive");
    assert_eq!(studio.view.member_count, 2);

    let mut roles: Vec<(&str, DirectorySpaceRole)> = studio.members.iter().map(|m| (m.user_id.as_str(), m.role)).collect();
    roles.sort();
    assert_eq!(roles, vec![("u-amara", DirectorySpaceRole::Spectator), ("u-devon", DirectorySpaceRole::Spectator)], "member.removed dropped u-noor; the archive law demoted every remaining author to spectator");

    let devon = studio.members.iter().find(|m| m.user_id == "u-devon").expect("devon is a member");
    assert_eq!(devon.email, "devon@semio.dev", "member display data is backfilled from user.created");
}

#[semio_framework_async_macros::async_test]
async fn folding_is_idempotent_on_replay() {
    let events = fixture_events().await;
    let once = fold_all(DirectoryReadModel::default(), &events).await;
    let twice = fold_all(once.clone(), &events);
    assert_eq!(once, twice.await, "re-folding the same events changes nothing (seq <= cursor is ignored)");
}
