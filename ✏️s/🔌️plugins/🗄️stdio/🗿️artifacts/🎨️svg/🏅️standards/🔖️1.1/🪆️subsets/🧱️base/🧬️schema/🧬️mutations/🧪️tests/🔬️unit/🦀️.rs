use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(SvgMutation::kinds().iter().map(|item|item.kind).collect::<Vec<_>>(),[
        "set-declaration","set-doctype","insert-element","remove-element","set-element-name","set-attribute","set-text","set-view-box","set-transform"
    ]);
}

#[semio_framework_async_macros::async_test]
async fn every_leaf_satisfies_the_inverse_sum_law() {
    use crate::schema::snapshot::{SvgAttributeValue, SvgNode, TransformOp, ViewBox};
    let base = crate::schema::demo_svg_snapshot();
    for mutation in [
        SvgMutation::SetDeclaration(SetDeclarationPayload { declaration: None }),
        SvgMutation::SetDoctype(SetDoctypePayload { doctype: None }),
        SvgMutation::InsertElement(InsertElementPayload { parent: Vec::new(), index: 1, node: SvgNode::Text { text: "inserted".into() } }),
        SvgMutation::RemoveElement(RemoveElementPayload { parent: Vec::new(), index: 2 }),
        SvgMutation::SetElementName(SetElementNamePayload { path: vec![2], name: "ellipse".into() }),
        SvgMutation::SetAttribute(SetAttributePayload { path: vec![2], name: "width".into(), value: None, index: None }),
        SvgMutation::SetAttribute(SetAttributePayload { path: vec![2], name: "fill".into(), value: Some(SvgAttributeValue::Text("blue".into())), index: None }),
        SvgMutation::SetAttribute(SetAttributePayload { path: vec![2], name: "stroke".into(), value: Some(SvgAttributeValue::Text("black".into())), index: Some(1) }),
        SvgMutation::SetViewBox(SetViewBoxPayload { path: Vec::new(), view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 8.0, height: 9.0 }), index: None }),
        SvgMutation::SetTransform(SetTransformPayload { path: vec![4], transform: Some(vec![TransformOp::Scale { x: 2.0, y: None }]), index: None }),
    ] {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
