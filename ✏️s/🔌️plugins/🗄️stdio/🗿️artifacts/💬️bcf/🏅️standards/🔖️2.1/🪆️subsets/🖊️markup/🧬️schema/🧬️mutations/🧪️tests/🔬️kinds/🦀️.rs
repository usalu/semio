
use super::*;

#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: BcfSnapshot::default() }),
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: BcfTopic::default() }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t".into(), title: None, description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t".into(), comment: BcfComment::default() }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t".into(), guid: "c".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t".into(), guid: "c".into(), date: None, author: None, text: None, viewpoint_ref: None }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t".into(), viewpoint: BcfViewpoint::default() }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t".into(), guid: "v".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t".into(), guid: "v".into(), camera: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t".into(), guid: "v".into(), components: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t".into(), guid: "v".into(), snapshot: None }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        let printed = print_bcf_mutation(mutation);
        let keyword = printed.split(' ').next().unwrap_or(&printed);
        assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
    }
}
