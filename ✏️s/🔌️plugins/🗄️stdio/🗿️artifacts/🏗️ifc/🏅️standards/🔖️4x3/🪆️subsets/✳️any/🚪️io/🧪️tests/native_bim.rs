use super::*;
use pack::snapshot::ArtifactPack;

#[test]
fn original_cancellation_reaches_spatial_children_scan() {
    let mut source=source();for level in 2..514{source.storeys.insert(format!("storey{level}"),Storey{building:"building".into(),name:format!("Level {level}"),level,height:3.,cut_height:Some(1.2)});}
    let graph=bim_to_ifc_controlled(&source,&mut |_|true).unwrap().snapshot;
    let mut observed=false;
    let result=ifc_to_bim_controlled(&graph,&mut |p|{if p.phase=="bim-hierarchy-children"&&p.completed==257{observed=true;false}else{true}});
    assert!(observed);assert_eq!(result.unwrap_err().code,"exchange.cancelled");
}

fn source()->ModelSnapshot {
    let mut source=ModelSnapshot::default();
    source.project.name="Native spatial".into();
    source.project.author="Author".into();
    source.sites.insert("site".into(),Site {name:"Site".into(),latitude:47.5,longitude:8.25,elevation:400.,true_north:0.2,boundary:vec![]});
    source.buildings.insert("building".into(),Building {site:"site".into(),name:"Building".into(),origin:Point2 {x:1.,y:2.},rotation:0.3,elevation:0.5});
    for level in [0,1] {source.storeys.insert(format!("storey{level}"),Storey {building:"building".into(),name:format!("Level {level}"),level,height:3.,cut_height:Some(1.2)});}
    source
}

#[test]
fn native_bim_five_rotations_emit_paired_axes_and_roundtrip(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("bim-axis-cases.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let angle=row["angle"].as_f64().unwrap();let mut source=source();source.buildings.get_mut("building").unwrap().rotation=angle;
        let graph=bim_to_ifc_controlled(&source,&mut |_|true).unwrap().snapshot;
        for axis in graph.entities.iter().filter(|entity|entity.name=="IFCAXIS2PLACEMENT3D"){
            assert_eq!(axis.args.len(),3);assert_eq!(matches!(axis.args[1],IfcValue::Unset),matches!(axis.args[2],IfcValue::Unset),"{}",row["id"]);
        }
        let building=graph.entities.iter().find(|entity|entity.name=="IFCBUILDING").unwrap();
        let placement=graph.entities.iter().find(|entity|Some(entity.id)==building.args[5].as_reference()).unwrap();
        let axis=graph.entities.iter().find(|entity|Some(entity.id)==placement.args[1].as_reference()).unwrap();
        if angle==0.{assert_eq!(&axis.args[1..],&[IfcValue::Unset,IfcValue::Unset]);}
        else{
            let z=graph.entities.iter().find(|entity|Some(entity.id)==axis.args[1].as_reference()).unwrap();assert_eq!(z.name,"IFCDIRECTION");assert_eq!(z.args[0],a(vec![r(0.),r(0.),r(1.)]));
            let x=graph.entities.iter().find(|entity|Some(entity.id)==axis.args[2].as_reference()).unwrap();assert_eq!(x.name,"IFCDIRECTION");let ratios=list(x.args.first()).unwrap();assert_eq!(ratios.len(),3);assert!((number(ratios.first()).unwrap()-angle.cos()).abs()<1e-12);assert!((number(ratios.get(1)).unwrap()-angle.sin()).abs()<1e-12);assert_eq!(number(ratios.get(2)).unwrap(),0.);
        }
        let restored=ifc_to_bim_controlled(&graph,&mut |_|true).unwrap().snapshot;let building=restored.buildings.values().next().unwrap();
        assert!((building.rotation-angle).abs()<1e-12,"{}",row["id"]);assert_eq!(building.origin,source.buildings["building"].origin);assert_eq!(building.elevation,source.buildings["building"].elevation);
    }
}

#[test]
fn actual_controlled_native_bim_routes_preserve_five_paired_rotations(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("bim-axis-cases.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let angle=row["angle"].as_f64().unwrap();let mut source=source();source.buildings.get_mut("building").unwrap().rotation=angle;
        let input=IoPayload::Binary(source.encode_pack());let mut receive=|_|true;let mut publish=|_|true;
        let mut dr=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut er=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
        let mut decode=semio_framework_value::NativeDecodeControl::new(256*1024*1024,&mut receive);let mut encode=semio_framework_value::NativeEncodeControl::new(256*1024*1024,&mut publish);decode.install_retirement_recipient(&mut dr).unwrap();encode.install_retirement_recipient(&mut er).unwrap();
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:256*1024*1024,maximum_capacity_bytes:256*1024*1024,maximum_release_bytes:256*1024*1024,maximum_depth:128};
        let mut control=IoRunControl::new(&mut decode,&mut encode,grant);
        let graph=semio_framework_async::poll::resolve_ready(<super::super::domains::BimIntoIfc as Deserializer<IfcSnapshot>>::deserialize(&input,&mut control)).unwrap().value;
        assert!(graph.entities.iter().filter(|entity|entity.name=="IFCAXIS2PLACEMENT3D").all(|axis|axis.args.len()==3&&matches!(axis.args[1],IfcValue::Unset)==matches!(axis.args[2],IfcValue::Unset)));drop(control);
        let mut control=IoRunControl::new(&mut decode,&mut encode,grant);
        let payload=semio_framework_async::poll::resolve_ready(<super::super::domains::IfcIntoBim as Serializer<IfcSnapshot>>::serialize(&graph,&ArchiveChildren::empty(),&mut control)).unwrap().value;drop(control);
        let IoPayload::Binary(bytes)=payload else{panic!("Native BIM pack required")};let restored=ModelSnapshot::decode_pack(&bytes).unwrap();assert!((restored.buildings.values().next().unwrap().rotation-angle).abs()<1e-12,"{}",row["id"]);
        for _ in 0..1_000_000{if !decode.has_retirement_owner()&&!encode.has_retirement_owner(){break;}decode.close_retirement_recipient(grant).unwrap();encode.close_retirement_recipient(grant).unwrap();}assert!(!decode.has_retirement_owner()&&!encode.has_retirement_owner());
    }
}

#[test]
fn native_bim_axis_pairs_coordinate_roles_and_dimensions_refuse(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("axis-cases.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row|matches!(row["id"].as_str(),Some("axis-only"|"ref-only"|"direction-origin"|"short-origin"|"short-axis"|"point-axis"|"point-ref"|"zero-axis"|"parallel-axes"))){
        let mut graph=bim_to_ifc_controlled(&source(),&mut |_|true).unwrap().snapshot;
        let building=graph.entities.iter().find(|entity|entity.name=="IFCBUILDING").unwrap();let placement=id(&building.args[5]).unwrap();let axis=id(&graph.entities.iter().find(|entity|entity.id==placement).unwrap().args[1]).unwrap();
        let mut fields=Vec::new();
        for field in ["origin","axis","ref"]{
            let value=&row[field];if value.is_null(){fields.push(IfcValue::Unset);continue;}
            let next=graph.entities.len()as u64+1;graph.entities.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity{id:next,name:value["kind"].as_str().unwrap().into(),args:vec![a(value["ratios"].as_array().unwrap().iter().map(|value|r(value.as_f64().unwrap())).collect())],complex:Vec::new()});fields.push(IfcValue::Reference(next));
        }
        graph.entities.iter_mut().find(|entity|entity.id==axis).unwrap().args=fields;
        assert!(ifc_to_bim_controlled(&graph,&mut |_|true).is_err(),"{}",row["id"]);
    }
    for present in [1,2]{
        let mut graph=bim_to_ifc_controlled(&source(),&mut |_|true).unwrap().snapshot;let axis=graph.entities.iter_mut().find(|entity|entity.name=="IFCAXIS2PLACEMENT3D"&&entity.args[1]!=IfcValue::Unset).unwrap();axis.args[present]=IfcValue::Unset;
        assert_eq!(ifc_to_bim_controlled(&graph,&mut |_|true).unwrap_err().code,"exchange.placement");
    }
}

#[test]
fn actual_native_bim_spatial_values_and_pack_roundtrip() {
    let source=source();
    let graph=bim_to_ifc_controlled(&source,&mut |_|true).unwrap();
    let restored=ifc_to_bim_controlled(&graph.snapshot,&mut |_|true).unwrap().snapshot;
    assert_eq!(restored.project,source.project);
    let site=restored.sites.values().next().unwrap();
    assert_eq!(site.latitude,47.5);assert_eq!(site.longitude,8.25);assert_eq!(site.elevation,400.);assert!((site.true_north-0.2).abs()<1e-12);
    let building=restored.buildings.values().next().unwrap();
    assert_eq!(building.origin,Point2 {x:1.,y:2.});
    assert!((building.rotation-0.3).abs()<1e-12);
    assert!(restored.storeys.values().all(|s|s.height==3.&&s.cut_height==Some(1.2)));
    assert_eq!(ModelSnapshot::decode_pack(&restored.encode_pack()).unwrap(),restored);
    if let Ok(folder)=std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {std::fs::create_dir_all(&folder).unwrap();std::fs::write(std::path::Path::new(&folder).join("native-bim.ifc"),crate::io::write_ifc4x3_step(&graph.snapshot).unwrap()).unwrap();}
}

#[test]
fn native_bim_explicit_length_units_convert_and_wrong_dimensions_refuse() {
    let mut graph=bim_to_ifc_controlled(&source(),&mut |_|true).unwrap().snapshot;
    let unit=graph.entities.len() as u64+1;
    graph.entities.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity {id:unit,name:"IFCSIUNIT".into(),args:vec![IfcValue::Derived,e("LENGTHUNIT"),e("MILLI"),e("METRE")],complex:vec![]});
    let property=graph.entities.iter_mut().find(|e|e.name=="IFCPROPERTYSINGLEVALUE"&&e.args[0].as_str()==Some("CutHeight")).unwrap();property.args[2]=typed("IFCLENGTHMEASURE",r(1200.));property.args[3]=IfcValue::Reference(unit);
    let model=ifc_to_bim_controlled(&graph,&mut |_|true).unwrap().snapshot;
    assert!(model.storeys.values().all(|storey|storey.cut_height==Some(1.2)));
    let unit=graph.entities.last_mut().unwrap();unit.args[1]=e("PRESSUREUNIT");unit.args[3]=e("PASCAL");
    assert!(ifc_to_bim_controlled(&graph,&mut |_|true).is_err());
}

#[test]
fn native_bim_nonidentity_site_and_contradictory_storey_frames_refuse() {
    for name in ["IFCSITE","IFCBUILDINGSTOREY"] {
        let mut graph=bim_to_ifc_controlled(&source(),&mut |_|true).unwrap().snapshot;
        let entity=graph.entities.iter().find(|e|e.name==name).unwrap();let placement=id(&entity.args[5]).unwrap();
        let axis=id(&graph.entities.iter().find(|e|e.id==placement).unwrap().args[1]).unwrap();let point=id(&graph.entities.iter().find(|e|e.id==axis).unwrap().args[0]).unwrap();
        graph.entities.iter_mut().find(|e|e.id==point).unwrap().args[0]=a(vec![r(0.),r(0.),r(99.)]);
        assert!(ifc_to_bim_controlled(&graph,&mut |_|true).is_err());
    }
}

#[test]
fn actual_native_bim_composers_publish_target_snapshot_packs() {
    use semio_framework_plugin::io::{ErasedComposeSource,IoPayload};
    let source=source();
    let exported=semio_framework_async::poll::resolve_ready((super::super::domains::entries()[1].compose)(&[ErasedComposeSource {dialect:super::super::domains::BIM_DIALECT,payload:IoPayload::Binary(source.encode_pack())}])).unwrap();
    let restored=semio_framework_async::poll::resolve_ready((super::super::domains::entries()[0].compose)(&[ErasedComposeSource {dialect:crate::io::IFC4X3_DIALECT,payload:exported.payload}])).unwrap();
    let IoPayload::Binary(bytes)=restored.payload else {panic!("Native BIM pack required")};
    assert_eq!(ModelSnapshot::decode_pack(&bytes).unwrap().storeys.len(),2);
}

#[test]
fn native_bim_cancellation_and_missing_authored_height_are_atomic() {
    let source=source();
    assert_eq!(bim_to_ifc_controlled(&source,&mut |_|false).unwrap_err().code,"exchange.cancelled");
    let mut graph=bim_to_ifc_controlled(&source,&mut |_|true).unwrap().snapshot;
    let property=graph.entities.iter_mut().find(|e|e.name=="IFCPROPERTYSINGLEVALUE"&&e.args[0].as_str()==Some("Height")).unwrap();
    property.args[0]=IfcValue::String("MissingHeight".into());
    assert!(ifc_to_bim_controlled(&graph,&mut |_|true).is_err());
}
