use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn home_config_dsl_text_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig::default());
    store::os_store::test_support::assert_dsl_round_trip(&HomeConfig { retired_local_studio_ids: vec!["studio-a".into(), "studio-雪".into()], ..HomeConfig::default() });
}

#[semio_framework_async_macros::async_test]
async fn home_config_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-a".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id: "studio-a".into() }));
}

/// 🫧️ The config carries the human's own Home choices only: its default holds no tombstone, and its op vocabulary has no
/// directory variant — the folded hub directory lives in the transient lane (`🫧️transient`), never in this history.
#[test]
fn home_config_holds_no_directory_projection() {
    assert_eq!(HomeConfig::default(), HomeConfig { retired_local_studio_ids: Vec::new() });
    let kinds: Vec<&str> = <HomeConfigMutation as Mutation<HomeConfig>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).collect();
    assert_eq!(kinds, ["retire-local-studio", "list-local-studio"]);
}

//#region 🧪️RetainedConfigPreparation
/// 🧪️ Drives a preparation's close ladder with the exact production page grant: cancel blocks further work, a close before
/// `begin_close` makes no progress, and every original owner leaves in its own fully granted turn until the terminal witness.
#[test]
fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
    use std::io::Write as _;
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    use store::ArtifactStoreOneItemPreparation as _;
    let mut preparation = HomeConfigPreparation {
        owners: store::OneItemOwners::detached(
            HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-a".into() }),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<HomeConfigMutation>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<HomeConfig>::default()),
        ),
        serialized_bytes: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        cancelled: false,
    };
    let page = semio_framework_plugin::app::TYPED_OPERATION_RESULT_PAGE_BYTES;
    let production = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: page, maximum_capacity_bytes: page, maximum_release_bytes: page, maximum_depth: 8 };
    preparation.cancel();
    assert!(matches!(preparation.advance(production).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
    assert!(matches!(preparation.close_step(production).expect("close before begin_close"), RetainedCloneStep::Progress(progress) if progress == Default::default()));
    preparation.begin_close();
    assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 0, ..production }).expect("zero-item close"), RetainedCloneStep::Progress(progress) if progress == Default::default()));
    let mut turns = 0;
    while !preparation.terminal_is_empty() {
        let demand = (
            preparation.next_close_copy_byte_demand().expect("copy demand"),
            preparation.next_close_capacity_byte_demand(page).expect("capacity demand"),
            preparation.next_close_release_byte_demand().expect("release demand"),
            preparation.next_close_depth_demand().expect("depth demand"),
        );
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.0.max(page), maximum_capacity_bytes: demand.1, maximum_release_bytes: demand.2, maximum_depth: demand.3.max(1) };
        let step = preparation.close_step(store::ArtifactStoreOneItemGrant::from_retained(grant)).expect("one owner per fully granted turn");
        assert!(step.progress().fits(grant));
        turns += 1;
        assert!(turns < 64, "the close ladder terminates");
    }
    assert!(matches!(preparation.close_step(production).expect("terminal close"), RetainedCloneStep::Complete(_)));
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
#[semio_framework_async_macros::async_test]
async fn local_studio_tombstones_are_exact_point_invertible_events() {
    let base = HomeConfig { retired_local_studio_ids: vec!["studio-b".into()], ..HomeConfig::default() };
    let retire = HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-a".into() });
    let retired = protocol::apply_diff(retire.diff(&base).diff(), &base).expect("the retirement applies");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&retire, &base).await;
    assert_eq!(retired.retired_local_studio_ids, vec!["studio-a".to_string(), "studio-b".to_string()]);
    assert!(retired.is_local_studio_retired("studio-a") && !base.is_local_studio_retired("studio-a"));
    let inverse = retire.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id: "studio-a".into() })]);
    assert_eq!(protocol::apply_diff(inverse[0].diff(&retired).diff(), &retired).expect("the restoration applies"), base);
    let again = retire.diff(&retired);
    assert!(protocol::DiffAlgebra::<HomeConfig>::is_empty(again.diff()));
    assert!(again.messages().iter().any(|message| format!("{message:?}").contains("mutation.no-op")));
    assert_eq!(retire.inverse(&retired).expect("valid retained mutation inverse fixture"), vec![retire.clone()], "the inverse of a no-op retirement changes nothing");
    for mutation in [retire, HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id: "studio-b".into() })] {
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
    for (mutation, base, code) in [(HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-over".into() }), &full, "mutation.target-mismatch"), (HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "bad\u{7}id".into() }), &HomeConfig::default(), "mutation.invariant")] {
        let outcome = mutation.diff(base);
        assert!(protocol::DiffAlgebra::<HomeConfig>::is_empty(outcome.diff()));
        assert!(outcome.messages().iter().any(|message| message.code.0 == code), "{code}");
    }
    let factory = HomeConfigPreparationFactory;
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-a".into() }), store::HistoryLane::Document).is_ok());
    assert!(factory.preflight(&HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id: "studio-a".into() }), store::HistoryLane::Document).is_ok());
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: String::new() }), store::HistoryLane::Document).is_err());
    assert!(factory.preflight(&HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "x".repeat(HOME_RETIRED_LOCAL_STUDIO_ID_BYTES + 1) }), store::HistoryLane::Document).is_err());
}
//#endregion 🪦️LocalStudioTombstones

/// 🧾️ The payload-record variants keep the former named variants' externally tagged wire byte for byte, checked against an
/// independent serde_json reading of the exact historical literals.
#[test]
fn config_mutation_wire_is_identical_to_the_former_named_variants() {
    let fixture = [
        (HomeConfigMutation::RetireLocalStudio(RetireLocalStudio { space_id: "studio-a".into() }), r#"{"RetireLocalStudio":{"space_id":"studio-a"}}"#),
        (HomeConfigMutation::ListLocalStudio(ListLocalStudio { space_id: "studio-a".into() }), r#"{"ListLocalStudio":{"space_id":"studio-a"}}"#),
    ];
    for (mutation, literal) in fixture {
        let json = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&mutation));
        let encoded: serde_json::Value = serde_json::from_str(&json).expect("the encoded wire is valid json");
        let historical: serde_json::Value = serde_json::from_str(literal).expect("the historical literal is valid json");
        assert_eq!(encoded, historical);
        let parsed = semio_framework_pack_json::parse(literal, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the historical literal parses");
        assert_eq!(<HomeConfigMutation as semio_framework_value::FromValue>::from_value(parsed).expect("the historical literal decodes"), mutation);
    }
}
