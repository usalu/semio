//! 🧪️ `set-relationship` satisfies the inverse sum law for a new relationship at the first, a middle and the last position, for a changed one and for a new owner.

use super::super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};

#[semio_framework_async_macros::async_test]
async fn set_relationship_inverts_to_the_exact_previous_state() {
    let base = fixture();
    let fresh = base.xml_parts.iter().map(|part| part.path.clone()).find(|path| opc_layer::with_package(&base, |opc| opc.relationships.relationships(path).is_none()).unwrap()).expect("an XML part without relationships");
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let (owner, list) = opc_layer::with_package(&base, |opc| opc.relationships.groups().map(|(owner, list)| (owner.clone(), list.clone())).next()).unwrap().expect("the fixture owns relationships");
    let added = |id: &str, index: Option<usize>| XlsxMutation::SetRelationship(set_relationship::SetRelationship { owner: owner.clone(), id: id.into(), rel_type: "http://example.invalid/relationships/added".into(), target: "https://example.invalid/added".into(), external: true, index });
    laws(&added("rIdFirst", Some(0)), &base).await;
    laws(&added("rIdMiddle", Some(list.len() / 2)), &base).await;
    laws(&added("rIdLast", None), &base).await;
    let changed = set_relationship::SetRelationship::of(&owner, &OpcRelationship { target: "https://example.invalid/changed".into(), target_mode: OpcTargetMode::External, ..list[list.len() - 1].clone() }, None);
    laws(&XlsxMutation::SetRelationship(changed), &base).await;
    laws(&XlsxMutation::SetRelationship(set_relationship::SetRelationship { owner: fresh.clone(), id: "rId1".into(), rel_type: "http://example.invalid/relationships/added".into(), target: "https://example.invalid/fresh".into(), external: true, index: None }), &base).await;
}
