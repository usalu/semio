//! 🧪️ Keyboard deletion uses original point indices and one atomic mutation group.
use super::*;
use semio_s_artifact_draw_drawing_geometry::editing::PathPoint;

fn paths()->DrawingSnapshot {
    let layers=["first","second"].iter().map(|id| {
        let mut path=semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit(((id)).to_string().into()).expect("nonempty authored identity"), id,vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[0.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,0.0]},semio_s_artifact_draw_drawing_model::PathSegment::Line {to:[10.0,10.0]}].into());
        semio_s_artifact_draw_drawing_model::layer_base_mut(&mut path).id=(*id).into();path
    }).collect();
    DrawingSnapshot {id:"delete-selection-test".into(),layers,..Default::default()}
}

fn point(document:&DrawingSnapshot,id:&str,index:usize)->String {
    let DrawingLayerNode::Path(path)=semio_s_artifact_draw_drawing_model::find_drawing_layer(document,id).unwrap() else {unreachable!()};
    points::point_id(id,&points::geometry_id(&path.segments).unwrap(),index,PathPoint::Anchor).unwrap()
}

#[test]
fn multi_path_deletion_preserves_style_and_is_reversible() {
    use protocol::Mutation;
    let before=paths();let ids=vec!["first".into(),"second".into()];
    let selected=vec![point(&before,"first",0),point(&before,"first",1),point(&before,"second",1)];
    let emit=plan(&before,"editNodes",&ids,&selected).unwrap();
    assert_eq!(emit.artifact_mutations.len(),2);assert_eq!(emit.effects.len(),2);
    let mut after=before.clone();let mut inverses=Vec::new();
    for mutation in &emit.artifact_mutations {inverses.push(mutation.inverse(&after).expect("valid retained mutation inverse fixture"));semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,mutation).unwrap();}
    for (layer,original) in after.layers.iter().zip(&before.layers) {assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(layer),semio_s_artifact_draw_drawing_model::layer_base(original));}
    let DrawingLayerNode::Path(first)=&after.layers[0] else {unreachable!()};
    assert_eq!(first.segments,vec![semio_s_artifact_draw_drawing_model::PathSegment::Move {to:[10.0,10.0]}].into());
    for group in inverses.into_iter().rev() {for mutation in group {semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&mutation).unwrap();}}
    assert_eq!(after,before);
}

#[test]
fn removing_every_anchor_removes_the_empty_path() {
    let before=paths();let selected=(0..3).map(|index|point(&before,"first",index)).collect::<Vec<_>>();
    let emit=plan(&before,"editNodes",&["first".into()],&selected).unwrap();
    assert!(matches!(&emit.artifact_mutations[0],DrawingMutation::DeleteLayer(_)));
}

#[test]
fn deletion_refuses_stale_or_locked_selection_atomically() {
    for invalid in ["stale","locked","hidden","missing"] {
        let mut before=paths();let mut selected=vec![point(&before,"first",1),point(&before,"second",1)];
        match invalid {
            "stale"=>selected[1]=points::point_id("second",&"0".repeat(64),1,PathPoint::Anchor).unwrap(),
            "locked"=>semio_s_artifact_draw_drawing_model::layer_base_mut(&mut before.layers[1]).locked=true,
            "hidden"=>semio_s_artifact_draw_drawing_model::layer_base_mut(&mut before.layers[1]).visible=false,
            _=>{before.layers.pop();},
        }
        let saved=before.clone();assert!(plan(&before,"editNodes",&["first".into(),"second".into()],&selected).is_err());assert_eq!(before,saved);
    }
}

#[test]
fn layer_deletion_normalizes_ancestors_and_respects_tool_context() {
    let mut before=paths();let child=before.layers.remove(0);let mut group=semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Group")).to_string().into()).expect("nonempty authored identity"), "Group");
    if let DrawingLayerNode::Group(group)=&mut group {group.base.id="group".into();group.children.push(child);}
    before.layers.push(group);
    let ids=vec!["group".into(),"first".into(),"second".into()];
    let emit=plan(&before,"selectDirect",&ids,&[]).unwrap();assert_eq!(emit.artifact_mutations.len(),2);
    let mut after=before.clone();for mutation in emit.artifact_mutations {semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&mutation).unwrap();}assert!(after.layers.is_empty());
    for utility in ["editNodes","pen","shapeRect"] {assert!(plan(&before,utility,&ids,&[]).unwrap().artifact_mutations.is_empty());}
}
