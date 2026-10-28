use super::*;
use super::nodal_support::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_framework_value::ValueRefusalKind;
#[test]
fn actual_owned_nodal_export_preserves_all_eighteen_bits_and_five_absent_components() {
    let vectors=cases();
    let mut count=0;
    for name in vectors["dofs"].as_array().unwrap() {
        for value in vectors["values"].as_array().unwrap() {
            let name=name.as_str().unwrap();
            let bits=value["bits"].as_str().unwrap().parse().unwrap();
            let (graph,_)=export(&input(name,bits),wallet(),usize::MAX);
            let graph=graph.unwrap();
            let force=&graph.entities[force(&graph)];
            let selected=rank(dof(name));
            for component in 0..6 {
                if component==selected { assert!(matches!(force.args[component+1],IfcValue::Real(n)if n.to_bits()==bits)); }
                else { assert_eq!(force.args[component+1],IfcValue::Unset); }
            }
            assert_eq!(graph.entities.iter().filter(|e|e.name=="IFCSTRUCTURALLOADCASE").count(),1);
            assert_eq!(graph.entities.iter().filter(|e|e.name=="IFCSTRUCTURALPOINTACTION").count(),1);
            assert_eq!(graph.entities.iter().filter(|e|e.name=="IFCRELCONNECTSSTRUCTURALACTIVITY").count(),1);
            assert!(graph.entities.iter().any(|e|e.name=="IFCSIUNIT"&&e.args.get(3).and_then(IfcValue::as_enum)==Some("NEWTON")));
            assert!(graph.entities.iter().any(|e|e.name=="IFCDERIVEDUNIT"&&e.args.get(1).and_then(IfcValue::as_enum)==Some("TORQUEUNIT")));
            if let Ok(path)=std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
                std::fs::create_dir_all(&path).unwrap();
                std::fs::write(std::path::Path::new(&path).join(format!("native-export-{name}-{}.ifc",value["id"].as_str().unwrap())),crate::io::write_ifc4x3_step(&graph).unwrap()).unwrap();
            }
            retire(graph); count+=1;
        }
    }
    assert_eq!(count,18);
}
#[test]
fn original_export_denial_and_every_observed_cancelled_prefix_publish_no_graph() {
    use semio_framework_value::RetainedCloneGrant;
    let source=input("Tz",(-0f64).to_bits());
    let (ready,events)=export(&source,wallet(),usize::MAX);
    retire(ready.unwrap());
    for stop in 1..=events { assert_eq!(export(&source,wallet(),stop).0.unwrap_err(),ValueRefusalKind::Canceled); }
    for grant in [
        RetainedCloneGrant{maximum_items:0,..wallet()},RetainedCloneGrant{maximum_copy_bytes:0,..wallet()},
        RetainedCloneGrant{maximum_capacity_bytes:0,..wallet()},RetainedCloneGrant{maximum_release_bytes:0,..wallet()},
        RetainedCloneGrant{maximum_depth:0,..wallet()},
    ] { assert!(export(&source,grant,usize::MAX).0.is_err()); }
}
#[test]
fn cases_retain_source_order_empty_cases_and_case_scoped_load_identity() {
    let mut source=input("Tx",0);
    let mut second=source.load_cases[0].clone(); second.id="second".into(); second.name="Second".into();
    source.load_cases.push(second);
    source.load_cases.push(FemLoadCase{id:"empty".into(),name:"Empty".into(),loads:vec![],self_weight:false});
    let graph=export(&source,wallet(),usize::MAX).0.unwrap();
    let names:Vec<_>=graph.entities.iter().filter(|e|e.name=="IFCSTRUCTURALLOADCASE").map(|e|e.args[2].as_str().unwrap()).collect();
    assert_eq!(names,["Case","Second","Empty"]);
    assert_eq!(graph.entities.iter().filter(|e|e.name=="IFCSTRUCTURALLOADSINGLEFORCE").count(),2);
    retire(graph);
}
#[test]
fn unsupported_source_load_semantics_and_unconnected_nodes_refuse_before_output() {
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
    for defect in 0..4 {
        let mut source=input("Tx",0);
        match defect {
            0=>source.load_cases[0].loads=vec![FemLoad::Area{id:"a".into(),solid_id:"solid".into(),pressure:1.}],
            1=>source.load_cases[0].loads=vec![FemLoad::MemberUdl{id:"u".into(),element_id:"member".into(),wx:0.,wy:0.,wz:1.}],
            2=>source.nodes.push(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::FemNodeValue{id:"isolated".into(),x:4.,y:0.,z:0.}),
            _=>{source.elements.clear();source.load_cases.clear();}
        }
        assert_eq!(export(&source,wallet(),usize::MAX).0.unwrap_err(),ValueRefusalKind::UnsupportedOwner);
    }
}
