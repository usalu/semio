//! 🧪️ `remove-relationship` satisfies the inverse sum law for a removed first, middle and last relationship and for the last relationship of an owner.

use super::super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};

#[semio_framework_async_macros::async_test]
async fn remove_relationship_restores_the_relationship_at_its_position() {
    let base = fixture();
    let fresh = base.xml_parts.iter().map(|part| part.path.clone()).find(|path| opc_layer::with_package(&base, |opc| opc.relationships.relationships(path).is_none()).unwrap()).expect("an XML part without relationships");
    let laws = protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
    let (owner, list) = opc_layer::with_package(&base, |opc| opc.relationships.groups().map(|(owner, list)| (owner.clone(), list.clone())).next()).unwrap().expect("the fixture owns relationships");
    let external = list.iter().find(|relationship| relationship.target_mode == OpcTargetMode::External).expect("the fixture carries an external demo relationship").clone();
    laws(&XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: owner.clone(), id: external.id.clone() }), &base).await;
    let grown = protocol::apply_diff(opc_layer::with_package(&base, |opc| opc_layer::relationship_write_diff(opc, &owner, &OpcRelationship { id: "rIdMiddleDemo".into(), ..external.clone() }, Some(1))).unwrap().as_ref().unwrap(), &base).expect("the middle relationship is added");
    laws(&XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: owner.clone(), id: "rIdMiddleDemo".into() }), &grown).await;
    let single = XlsxMutation::SetRelationship(set_relationship::SetRelationship { owner: fresh.clone(), id: "rId1".into(), rel_type: "http://example.invalid/relationships/solo".into(), target: "https://example.invalid/solo".into(), external: true, index: None });
    let with_solo = protocol::apply_diff(Mutation::diff(&single, &base).diff(), &base).expect("the single-relationship owner is added");
    laws(&XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: fresh.clone(), id: "rId1".into() }), &with_solo).await;
}
