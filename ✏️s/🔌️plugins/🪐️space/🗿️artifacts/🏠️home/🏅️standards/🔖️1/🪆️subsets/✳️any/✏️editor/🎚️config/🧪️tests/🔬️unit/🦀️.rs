use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn home_config_dsl_text_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig::default());
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig { retired_local_studio_ids: vec!["studio-a".into(), "studio-雪".into()], ..HomeConfig::default() });
}

#[semio_framework_async_macros::async_test]
async fn home_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::Snapshot { config: HomeConfig::default() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::RetireLocalStudio { space_id: "studio-a".into() });
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::RestoreLocalStudio { space_id: "studio-a".into() });
}

/// 🫧️ The config carries the human's own Home choices only: its default holds no tombstone, and its op vocabulary has no
/// directory variant — the folded hub directory lives in the transient lane (`🫧️transient`), never in this history.
#[test]
fn home_config_holds_no_directory_projection() {
    assert_eq!(HomeConfig::default(), HomeConfig { retired_local_studio_ids: Vec::new() });
    let kinds: Vec<&str> = <HomeConfigMutation as Mutation<HomeConfig>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).collect();
    assert_eq!(kinds, ["set-snapshot", "retire-local-studio", "restore-local-studio"]);
}

//#region 🧪️RetainedConfigPreparation
#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use store::ArtifactStoreOneItemPreparation as _;
    let mut preparation = HomeConfigPreparation {
        base: None,
        mutation: Some(HomeConfigMutation::RetireLocalStudio { space_id: "studio-a".into() }),
        description: None,
        authority: None,
        candidate: None,
        sealed_candidate: None,
        serialized_bytes: None,
        prepared: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
        closing: false,
    };
    let page = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: semio_framework_plugin::app::TYPED_OPERATION_RESULT_PAGE_BYTES };
    preparation.cancel();
    assert!(matches!(preparation.advance(page).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
    assert!(matches!(preparation.close_step(page).expect("close before begin_close"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    preparation.begin_close();
    assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: page.maximum_bytes }).expect("zero-item close"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(preparation.close_step(page).expect("one owner per production page"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes } if released_bytes <= page.maximum_bytes));
    assert!(matches!(preparation.close_step(page).expect("terminal close"), store::SnapshotRetirementStep::Complete));
    assert!(preparation.terminal_is_empty());
    let mut counter = HomeConfigByteCounter { bytes: 0 };
    let maximum = vec![0; HOME_CONFIG_STEP_BYTES];
    assert_eq!(counter.write(&maximum).expect("maximum serialized envelope"), HOME_CONFIG_STEP_BYTES);
    assert!(counter.write(&[0]).is_err());
}
//#endregion 🧪️RetainedConfigPreparation

//#region 🪦️LocalStudioTombstones
/// 🪦️ A tombstone is an exact, point-invertible event: retiring inserts the id in order, its inverse restores the
/// config byte for byte, a repeated retirement is a named no-op whose inverse changes nothing, and the op binary round
/// trips every variant.
#[test]
fn local_studio_tombstones_are_exact_point_invertible_events() {
    let base = HomeConfig { retired_local_studio_ids: vec!["studio-b".into()], ..HomeConfig::default() };
    let retire = HomeConfigMutation::RetireLocalStudio { space_id: "studio-a".into() };
    let retired = retire.diff(&base).diff().clone();
    assert_eq!(retired.retired_local_studio_ids, vec!["studio-a".to_string(), "studio-b".to_string()]);
    assert!(retired.is_local_studio_retired("studio-a") && !base.is_local_studio_retired("studio-a"));
    let inverse = retire.inverse(&base);
    assert_eq!(inverse, vec![HomeConfigMutation::RestoreLocalStudio { space_id: "studio-a".into() }]);
    assert_eq!(inverse[0].diff(&retired).diff(), &base);
    let again = retire.diff(&retired);
    assert_eq!(again.diff(), &retired);
    assert!(again.messages().iter().any(|message| format!("{message:?}").contains("mutation.no-op")));
    assert_eq!(retire.inverse(&retired), vec![retire.clone()], "the inverse of a no-op retirement changes nothing");
    for mutation in [retire, HomeConfigMutation::RestoreLocalStudio { space_id: "studio-b".into() }] {
        let bytes = protocol::OpBinary::encode_op(&mutation).expect("op binary encodes");
        assert_eq!(<HomeConfigMutation as protocol::OpBinary>::decode_op(&bytes).expect("op binary decodes"), mutation);
    }
}

/// 🚫️ The tombstone set is bounded: an inadmissible id or a retirement past the ceiling is refused by name and leaves
/// the config unchanged; the retained config lane admits exactly the admissible tombstones.
#[test]
fn local_studio_tombstones_refuse_inadmissible_ids_and_the_ceiling() {
    use store::ArtifactStoreOneItemPreparationFactory as _;
    let full = HomeConfig { retired_local_studio_ids: (0..HOME_RETIRED_LOCAL_STUDIOS_MAXIMUM).map(|index| format!("studio-{index:04}")).collect(), ..HomeConfig::default() };
    for (mutation, base) in [(HomeConfigMutation::RetireLocalStudio { space_id: "studio-over".into() }, &full), (HomeConfigMutation::RetireLocalStudio { space_id: "bad\u{7}id".into() }, &HomeConfig::default())] {
        let outcome = mutation.diff(base);
        assert_eq!(outcome.diff(), base);
        assert!(outcome.messages().iter().any(|message| format!("{message:?}").contains("s.home.local-studio-tombstone-refused")));
    }
    let factory = HomeConfigPreparationFactory;
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio { space_id: "studio-a".into() }, None, store::HistoryLane::Document).is_ok());
    assert!(factory.preflight(&HomeConfigMutation::RestoreLocalStudio { space_id: "studio-a".into() }, None, store::HistoryLane::Document).is_ok());
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio { space_id: String::new() }, None, store::HistoryLane::Document).is_err());
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio { space_id: "x".repeat(HOME_RETIRED_LOCAL_STUDIO_ID_BYTES + 1) }, None, store::HistoryLane::Document).is_err());
    assert!(factory.preflight(&HomeConfigMutation::Snapshot { config: HomeConfig::default() }, None, store::HistoryLane::Document).is_err());
}
//#endregion 🪦️LocalStudioTombstones
