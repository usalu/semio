use super::*;
use protocol::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
use protocol::MutationKind;
use protocol::SemanticMutation;

fn titled(title: Option<&str>) -> PlaybookSnapshot {
    PlaybookSnapshot { title: title.map(str::to_string), ..PlaybookSnapshot::default() }
}

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn change_title_inverse_law() {
    assert_mutation_inverse_law(&titled(None), &PlaybookMutation::ChangeTitle(ChangeTitle { new_title: Some("Recipe".into()) })).await;
    assert_mutation_inverse_law(&titled(Some("Recipe")), &PlaybookMutation::ChangeTitle(ChangeTitle { new_title: None })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_title_diff_absorb_law() {
    let base = titled(None);
    let first = ChangeTitle { new_title: Some("Draft".into()) }.diff(&base).into_parts().0;
    let middle = protocol::MutationDiff::apply(&first, &base).expect("valid mutation diff");
    let second = ChangeTitle { new_title: Some("Recipe".into()) }.diff(&middle).into_parts().0;
    assert_mutation_diff_absorb_law(&base, first, second).await;
}

/// 🪆️ The title edit is a single-field patch: it never replaces the `flow` coordinate the steps live behind.
#[test]
fn change_title_never_touches_the_flow_coordinate() {
    let diff = ChangeTitle { new_title: Some("Recipe".into()) }.diff(&titled(None)).into_parts().0;
    assert!(diff.flow.is_none() && diff.artifact.is_none());
    assert_eq!(diff.title, Some(Some("Recipe".into())));
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_playbook_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in PlaybookMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(PlaybookMutation::kinds().len(), 1, "steps and blocks are edited on the flow child's lane, never by a parent leaf");
}
//#endregion 🔖️MutationLaws
