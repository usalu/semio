
use super::*;

fn leaf(key: &str) -> TreeNode {
    TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).ok().expect("bounded fixture key")
}

#[test]
fn mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate() {
    let root = leaf("root").try_with_children([leaf("a"), leaf("b")]).ok().expect("fixed child pages");
    let mut producer = ComponentTreeProducer::try_new(root, 7).ok().expect("nonzero generation");
    assert!(producer.take_complete().is_none());
    assert_eq!(producer.step(7, false, false), ComponentTreeProducerStep::MoreWork);
    assert!(producer.take_complete().is_none());
    for _ in 0..256 {
        if producer.step(7, false, false) == ComponentTreeProducerStep::Complete {
            break;
        }
    }
    assert!(producer.take_complete().is_some());
}

#[test]
fn duplicate_stale_cancel_and_deadline_fault_before_publication() {
    let duplicate = leaf("root").try_with_children([leaf("same"), leaf("same")]).ok().expect("fixed child pages");
    let mut duplicate = ComponentTreeProducer::try_new(duplicate, 9).ok().expect("producer");
    for _ in 0..256 {
        if matches!(duplicate.step(9, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::DuplicateSiblingKey)) {
            break;
        }
    }
    assert_eq!(duplicate.fault(), Some(ComponentTreeProducerFault::DuplicateSiblingKey));
    assert!(duplicate.take_complete().is_none());

    let mut stale = ComponentTreeProducer::try_new(leaf("stale"), 11).ok().expect("producer");
    assert_eq!(stale.step(12, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Generation { expected: 11, actual: 12 }));
    let mut cancelled = ComponentTreeProducer::try_new(leaf("cancel"), 13).ok().expect("producer");
    assert_eq!(cancelled.step(13, true, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Cancelled));
    let mut deadline = ComponentTreeProducer::try_new(leaf("deadline"), 15).ok().expect("producer");
    assert_eq!(deadline.step(15, false, true), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Deadline));
}

#[test]
fn deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close() {
    let mut maximum = leaf("leaf");
    for depth in 1..COMPONENT_TREE_PRODUCER_DEPTH {
        maximum = leaf(position_key(depth).expect("bounded position").as_str()).try_with_children([maximum]).ok().expect("fixed child page");
    }
    let mut maximum = ComponentTreeProducer::try_new(maximum, 17).ok().expect("producer");
    for _ in 0..4_096 {
        if maximum.step(17, false, false) == ComponentTreeProducerStep::Complete {
            break;
        }
    }
    assert!(maximum.take_complete().is_some());

    let mut plus_one = leaf("leaf");
    for depth in 1..=COMPONENT_TREE_PRODUCER_DEPTH {
        plus_one = leaf(position_key(depth).expect("bounded position").as_str()).try_with_children([plus_one]).ok().expect("fixed child page");
    }
    let mut plus_one = ComponentTreeProducer::try_new(plus_one, 19).ok().expect("producer");
    for _ in 0..4_096 {
        if matches!(plus_one.step(19, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeDepth)) {
            break;
        }
    }
    assert_eq!(plus_one.fault(), Some(ComponentTreeProducerFault::NodeDepth));
    let mut opportunities = 0;
    while !plus_one.close_step() {
        opportunities += 1;
        assert!(opportunities < 1_024);
    }
}

fn _accepts_any_present<P: Present>(_p: &P) {}

#[test]
fn a_stateless_fn_item_satisfies_present_generically() {
    fn screen(_cx: &mut PresentCx<'_>) -> ComponentTree {
        ComponentTree::new(leaf("root"))
    }
    _accepts_any_present(&screen);
}
