use super::*;

#[test]
fn original_cancellation_reaches_entity_and_material_aggregate_scans() {
    let mut builder=Builder::new("scan");
    for i in 0..512{builder.add("IFCCARTESIANPOINT",vec![a(vec![r(i as f64),r(0.),r(0.)])]);}
    let graph=builder.snapshot;
    let mut observed=false;
    let result=Reader::new(&graph,&mut |p|{if p.phase=="graph-entities"&&p.completed==257{observed=true;false}else{true}});
    assert!(observed);assert_eq!(result.err().unwrap().code,"exchange.cancelled");
    let native=super::super::domains::receive_fem_uncontrolled(&reference()).unwrap();
    let mut graph=fem_to_ifc_controlled(&native,&mut |_|true).unwrap().snapshot;
    let mut extra=Vec::new();
    for i in 0..512{let id=graph.entities.len() as u64+1;graph.entities.push(IfcEntity{id,name:"IFCPROPERTYSINGLEVALUE".into(),args:vec![s(format!("Additional{i}")),IfcValue::Unset,typed("IFCREAL",r(0.)),IfcValue::Unset],complex:vec![]});extra.push(IfcValue::Reference(id));}
    let definition=graph.entities.iter_mut().find(|e|e.name=="IFCMATERIALPROPERTIES").unwrap();
    let IfcValue::Aggregate(values)=&mut definition.args[2] else{unreachable!()};values.extend(extra);
    let mut observed=false;
    let result=ifc_to_fem_reference_controlled(&graph,&mut |p|{if p.phase=="fem-material-properties"&&p.completed==257{observed=true;false}else{true}});
    assert!(observed);assert_eq!(result.unwrap_err().code,"exchange.cancelled");
    super::super::domains::retire_fem_fixture(native);
}

fn reference()->Fem3dSnapshotValue {
    let mut source=Fem3dSnapshotValue::default();
    source.nodes=vec![FemNodeValue {id:"a".into(),x:0.,y:0.,z:0.},FemNodeValue {id:"b".into(),x:1.,y:0.,z:0.},FemNodeValue {id:"c".into(),x:2.,y:0.,z:0.}];
    source.materials=vec![FemMaterialValue {id:"steel".into(),name:"Steel".into(),e:2e11,g:8e10,nu:0.25,rho:7850.}];
    source.sections=vec![FemSection {id:"section".into(),name:"Section".into(),area:0.01,iy:2e-5,iz:3e-5,j:4e-5}];
    source.elements=vec![FemElement::Bar {id:"ab".into(),start:"a".into(),end:"b".into(),material_id:"steel".into(),section_id:"section".into()},FemElement::Bar {id:"bc".into(),start:"b".into(),end:"c".into(),material_id:"steel".into(),section_id:"section".into()}];
    source.supports=vec![FemSupport {id:"fixed".into(),node_id:"a".into(),fixed:DOFS.to_vec()}];source
}

#[test]
fn native_unloaded_analysis_retains_shared_engineering_identity_and_supports() {
    let native=super::super::domains::receive_fem_uncontrolled(&reference()).unwrap();
    let graph=fem_to_ifc_controlled(&native,&mut |_|true).unwrap().snapshot;
    let restored=ifc_to_fem_reference_controlled(&graph,&mut |_|true).unwrap().snapshot;
    assert_eq!(restored.nodes.len(),3);assert_eq!(restored.elements.len(),2);assert_eq!(restored.materials.len(),1);assert_eq!(restored.sections.len(),1);
    assert_eq!(restored.sections[0].j,4e-5);assert_eq!(restored.supports[0].fixed,DOFS);
    let restored_native=super::super::domains::receive_fem_uncontrolled(&restored).unwrap();
    fem_to_ifc_controlled(&restored_native,&mut |_|true).unwrap();
    if let Ok(folder)=std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {std::fs::create_dir_all(&folder).unwrap();std::fs::write(std::path::Path::new(&folder).join("native-fem.ifc"),crate::io::write_ifc4x3_step(&graph).unwrap()).unwrap();}
    super::super::domains::retire_fem_fixture(native);super::super::domains::retire_fem_fixture(restored_native);
}

#[test]
fn native_fem_unsupported_loads_wrong_dimensions_and_missing_membership_refuse() {
    let native=super::super::domains::receive_fem_uncontrolled(&reference()).unwrap();
    let mut graph=fem_to_ifc_controlled(&native,&mut |_|true).unwrap().snapshot;
    let unit=graph.entities.iter_mut().find(|e|e.name=="IFCDERIVEDUNITELEMENT"&&e.args[1]==IfcValue::Integer(4)).unwrap();unit.args[1]=IfcValue::Integer(1);
    assert!(ifc_to_fem_reference_controlled(&graph,&mut |_|true).is_err());
    let mut graph=fem_to_ifc_controlled(&native,&mut |_|true).unwrap().snapshot;
    graph.entities.retain(|e|e.name!="IFCRELASSIGNSTOGROUP");assert!(ifc_to_fem_reference_controlled(&graph,&mut |_|true).is_err());
    assert_eq!(fem_to_ifc_controlled(&native,&mut |_|false).unwrap_err().code,"exchange.cancelled");
    let mut loaded=reference();loaded.load_cases.push(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase {id:"case".into(),name:"Case".into(),loads:vec![],self_weight:false});
    let loaded=super::super::domains::receive_fem_uncontrolled(&loaded).unwrap();assert_eq!(fem_to_ifc_controlled(&loaded,&mut |_|true).unwrap_err().code,"fem.unsupported");
    super::super::domains::retire_fem_fixture(native);super::super::domains::retire_fem_fixture(loaded);
}
