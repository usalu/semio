//! 🧪️ The retained overlay observes real original coordinates without constructing a candidate graph.
use super::overlay::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;

#[test]
fn source_owned_overlay_replaces_original_slots_and_resolves_forward_insertions() {
    use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity::InsertEntity, set_entity_arg::SetEntityArg};
    let source = IfcSnapshot { entities: vec![IfcEntity { id: 7, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::String("original".into())], complex: vec![] }], ..Default::default() };
    let mutations = vec![
        IfcMutation::SetEntityArg(SetEntityArg { id: 7, index: 0, value: IfcValue::Reference(20) }),
        IfcMutation::InsertEntity(InsertEntity { index: 1, entity: IfcEntity { id: 20, name: "IFCDIRECTION".into(), args: vec![IfcValue::Reference(7)], complex: vec![] } }),
    ];
    let grant = RetainedCloneGrant { maximum_items: 10_000, maximum_copy_bytes: 16_384, maximum_capacity_bytes: 16_384, maximum_release_bytes: 16_384, maximum_depth: 256 };
    let mut prefixes = vec![None];
    while let Some(cut) = prefixes.pop() {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(921), semio_framework_job::Generation(1));
        let mut overlay = PopulationOverlay::default();
        let mut ready = false;
        for turn in 0..1_000 {
            if cut == Some(turn) {
                caller.cancel().cancel_now();
            }
            match caller.work_turn(grant, 1, |cx| overlay.advance(&source, &mutations, cx)) {
                Ok(true) => {
                    ready = true;
                    if cut.is_none() {
                        prefixes.extend((0..=turn).map(Some));
                    }
                    break;
                }
                Ok(false) => {}
                Err(_) => break,
            }
        }
        if cut.is_none() {
            assert!(ready);
            assert_eq!(overlay.len(), 2);
            let row = overlay.find(7).unwrap();
            assert!(std::ptr::eq(
                overlay.argument(&source, &mutations, row, 0).unwrap(),
                match &mutations[0] {
                    IfcMutation::SetEntityArg(change) => &change.value,
                    _ => unreachable!(),
                }
            ));
            let forward = overlay.find(20).unwrap();
            assert_eq!(overlay.entity(&source, &mutations, forward).unwrap().id, 20);
            let foreign = source.clone();
            assert!(overlay.check_authority(&foreign, &mutations).is_err());
        } else {
            assert!(!ready);
        }
        overlay.begin_close();
        for _ in 0..1_000 {
            if overlay.terminal_is_empty() {
                break;
            }
            caller.turn(grant, |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan)))).unwrap();
        }
        assert!(overlay.terminal_is_empty());
        caller.finish();
    }
}
