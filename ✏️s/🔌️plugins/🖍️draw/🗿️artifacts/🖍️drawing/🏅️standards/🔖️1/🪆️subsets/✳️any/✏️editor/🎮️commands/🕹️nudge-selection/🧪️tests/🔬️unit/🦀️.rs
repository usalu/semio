//! 🧪️ Group ownership and atomic failure for document-axis keyboard movement.
use super::*;

fn document()->DrawingSnapshot {
    let mut child=crate::schema::create_layer_by_kind("shape:rect");
    crate::schema::layer_base_mut(&mut child).id="child".into();
    let mut group=crate::schema::create_layer_by_kind("group");
    if let DrawingLayerNode::Group(group)=&mut group {
        group.base.id="parent".into();group.base.transform.rotation=std::f64::consts::FRAC_PI_2;group.base.transform.scale_x=2.0;group.base.transform.scale_y=4.0;group.children=vec![child];
    }
    DrawingSnapshot {layers:vec![group],..Default::default()}
}

#[test]
fn selected_ancestor_owns_descendant_movement_once() {
    for ids in [vec!["child"],vec!["parent","child"]] {
        let before=document();let mut after=before.clone();
        let mut session=DrawingSession::default();session.interaction.ids=ids.iter().map(|id|(*id).into()).collect();
        let emit=plan(&before,&session,[10.0,-5.0]).unwrap();assert_eq!(emit.artifact_mutations.len(),1);
        for mutation in emit.artifact_mutations {crate::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap();}
        let original=crate::schema::flatten_drawing_document_to_scene_nodes(&before)[0].transform;
        let moved=crate::schema::flatten_drawing_document_to_scene_nodes(&after)[0].transform;
        assert!((moved[4]-original[4]-10.0).abs()<1e-10 && (moved[5]-original[5]+5.0).abs()<1e-10);
        for i in 0..4 {assert!((moved[i]-original[i]).abs()<1e-10);}
        if ids.len()==2 {assert_eq!(crate::schema::find_drawing_layer(&after,"child"),crate::schema::find_drawing_layer(&before,"child"));}
    }
    eprintln!("[DEBUG] selected ancestors move descendants once in document axes");
}

#[test]
fn nudge_rejects_unavailable_targets_without_mutating_source() {
    for mode in ["locked","hidden","singular","missing"] {
        let mut before=document();
        let base=crate::schema::layer_base_mut(&mut before.layers[0]);
        match mode {"locked"=>base.locked=true,"hidden"=>base.visible=false,"singular"=>base.transform.scale_x=0.0,_=>{}}
        let saved=before.clone();let mut session=DrawingSession::default();
        session.interaction.ids=vec![if mode=="missing" {"missing"} else {"child"}.into()];
        assert!(plan(&before,&session,[1.0,0.0]).is_err(),"{mode}");assert_eq!(before,saved);
    }
}

#[test]
fn other_utilities_and_empty_point_selection_do_not_move_layers() {
    let source=document();let mut session=DrawingSession::default();session.interaction.ids=vec!["child".into()];
    for utility in ["editNodes","pen","trace","shapeRect"] {
        session.active_utility_id=utility.into();assert!(plan(&source,&session,[1.0,0.0]).unwrap().artifact_mutations.is_empty());
    }
}

#[test]
fn stale_point_reference_rejects_the_entire_selection() {
    let segments=vec![crate::PathSegment::Move {to:[0.0,0.0]},crate::PathSegment::Line {to:[10.0,0.0]}];
    let mut paths=Vec::new();
    for id in ["first","second"] {let mut path=crate::schema::create_drawing_path_layer(id,segments.clone());crate::schema::layer_base_mut(&mut path).id=id.into();paths.push(path);}
    let source=DrawingSnapshot {layers:paths,..Default::default()};let saved=source.clone();
    let mut session=DrawingSession::with_active_utility("editNodes");session.interaction.ids=vec!["first".into(),"second".into()];
    let geometry=points::geometry_id(&segments).unwrap();
    session.interaction.points=vec![points::point_id("first",&geometry,1,crate::schema::geometry::editing::PathPoint::Anchor).unwrap(),points::point_id("second",&"0".repeat(64),1,crate::schema::geometry::editing::PathPoint::Anchor).unwrap()];
    assert!(plan(&source,&session,[1.0,0.0]).is_err());assert_eq!(source,saved);
}
