
use super::*;

/// 🏗️ Same 4-level (project/site/building/storey) + wall + Pset_WallCommon fixture as ifc's
/// own `engine::spatial` test module — a real, non-trivial IFC4 document.
const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#10=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#40,$);\n#40=IFCAXIS2PLACEMENT3D(#42,$,$);\n#42=IFCCARTESIANPOINT((0.,0.,0.));\n#11=IFCUNITASSIGNMENT((#41));\n#41=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#3=IFCSITE('gid-site',#2,'Demo Site',$,$,#50,$,$,.ELEMENT.,$,$,$,$,$);\n#50=IFCLOCALPLACEMENT($,#51);\n#51=IFCAXIS2PLACEMENT3D(#42,$,$);\n#4=IFCBUILDING('gid-building',#2,'Demo Building',$,$,#60,$,$,.ELEMENT.,$,$,$);\n#60=IFCLOCALPLACEMENT(#50,#61);\n#61=IFCAXIS2PLACEMENT3D(#62,$,$);\n#62=IFCCARTESIANPOINT((0.,0.,10.));\n#5=IFCBUILDINGSTOREY('gid-storey',#2,'Ground Floor',$,$,#70,$,$,.ELEMENT.,3.);\n#70=IFCLOCALPLACEMENT(#60,#71);\n#71=IFCAXIS2PLACEMENT3D(#72,$,$);\n#72=IFCCARTESIANPOINT((0.,0.,0.));\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\n#80=IFCLOCALPLACEMENT(#70,#81);\n#81=IFCAXIS2PLACEMENT3D(#82,$,$);\n#82=IFCCARTESIANPOINT((1.,2.,0.));\n#100=IFCRELAGGREGATES('agg-1',#2,$,$,#1,(#3));\n#101=IFCRELAGGREGATES('agg-2',#2,$,$,#3,(#4));\n#102=IFCRELAGGREGATES('agg-3',#2,$,$,#4,(#5));\n#103=IFCRELCONTAINEDINSPATIALSTRUCTURE('cont-1',#2,$,$,(#6),#5);\n#200=IFCPROPERTYSET('pset-1',#2,'Pset_WallCommon',$,(#201));\n#201=IFCPROPERTYSINGLEVALUE('IsExternal',$,IFCBOOLEAN(.T.),$);\n#202=IFCRELDEFINESBYPROPERTIES('rel-1',#2,$,$,(#6),#200);\nENDSEC;\nEND-ISO-10303-21;\n";

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture_snapshot() -> IfcSnapshot {
    let doc = semio_s_artifact_stdio_step::engine::part21::parse_part21(FIXTURE).expect("parse fixture");
    semio_s_artifact_stdio_ifc::schema::snapshot::from_part21_document(semio_s_artifact_stdio_ifc::STDIO_IFC_DOCUMENT_SCHEMA, &doc)
}

#[semio_framework_async_macros::async_test]
async fn spatial_tree_and_element_map_from_a_real_ifc4_document() {
    let model = model_from_ifc(&fixture_snapshot());
    assert_eq!(model.spatial.len(), 3, "site/building/storey, project dropped");
    let site = model.spatial.iter().find(|n| n.kind == SpatialKind::Site).expect("site");
    assert_eq!(site.name, "Demo Site");
    assert!(site.parent_id.is_none(), "site is a root (project has no SpatialKind)");
    let building = model.spatial.iter().find(|n| n.kind == SpatialKind::Building).expect("building");
    assert_eq!(building.parent_id.as_deref(), Some(site.id.as_str()));
    let storey = model.spatial.iter().find(|n| n.kind == SpatialKind::Storey).expect("storey");
    assert_eq!(storey.parent_id.as_deref(), Some(building.id.as_str()));

    assert_eq!(model.elements.len(), 1);
    let wall = &model.elements[0];
    assert_eq!(wall.class, ElementClass::Wall);
    assert_eq!(wall.spatial_id.as_deref(), Some(storey.id.as_str()));
    assert_eq!(wall.geometry, GeometryRef::None, "geometry resolution is out of this bridge's scope");
    assert!((wall.placement.translation.x - 1.0).abs() < 1e-9, "world placement composed across 4 levels: {:?}", wall.placement);
    assert!((wall.placement.translation.z - 10.0).abs() < 1e-9);
    assert_eq!(wall.psets.len(), 1);
    assert_eq!(wall.psets[0].name, "Pset_WallCommon");
    assert_eq!(wall.psets[0].properties[0], Property { key: "IsExternal".into(), value: PsetValue::Boolean { value: true } });

    assert!(model.relations.iter().any(|r| r.kind == RelationKind::Aggregates && r.from == building.id && r.to == site.id));
    assert!(model.relations.iter().any(|r| r.kind == RelationKind::ContainedIn && r.from == wall.id && r.to == storey.id));
}

#[semio_framework_async_macros::async_test]
async fn unscalar_property_values_are_skipped_not_fabricated() {
    assert_eq!(pset_value_from_part21(&Part21Value::Unset), None);
    assert_eq!(pset_value_from_part21(&Part21Value::List(vec![])), None);
    assert_eq!(pset_value_from_part21(&Part21Value::Ref(1)), None);
}
