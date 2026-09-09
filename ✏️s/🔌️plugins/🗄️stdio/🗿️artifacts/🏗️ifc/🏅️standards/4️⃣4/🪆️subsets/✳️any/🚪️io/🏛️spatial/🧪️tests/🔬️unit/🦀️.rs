use super::*;
use semio_s_artifact_stdio_step::engine::part21::parse_part21;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#10=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.E-05,#40,$);\n#40=IFCAXIS2PLACEMENT3D(#42,$,$);\n#42=IFCCARTESIANPOINT((0.,0.,0.));\n#11=IFCUNITASSIGNMENT((#41));\n#41=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);\n#3=IFCSITE('gid-site',#2,'Demo Site',$,$,#50,$,$,.ELEMENT.,$,$,$,$,$);\n#50=IFCLOCALPLACEMENT($,#51);\n#51=IFCAXIS2PLACEMENT3D(#42,$,$);\n#4=IFCBUILDING('gid-building',#2,'Demo Building',$,$,#60,$,$,.ELEMENT.,$,$,$);\n#60=IFCLOCALPLACEMENT(#50,#61);\n#61=IFCAXIS2PLACEMENT3D(#62,$,$);\n#62=IFCCARTESIANPOINT((0.,0.,10.));\n#5=IFCBUILDINGSTOREY('gid-storey',#2,'Ground Floor',$,$,#70,$,$,.ELEMENT.,3.);\n#70=IFCLOCALPLACEMENT(#60,#71);\n#71=IFCAXIS2PLACEMENT3D(#72,$,$);\n#72=IFCCARTESIANPOINT((0.,0.,0.));\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\n#80=IFCLOCALPLACEMENT(#70,#81);\n#81=IFCAXIS2PLACEMENT3D(#82,$,$);\n#82=IFCCARTESIANPOINT((1.,2.,0.));\n#100=IFCRELAGGREGATES('agg-1',#2,$,$,#1,(#3));\n#101=IFCRELAGGREGATES('agg-2',#2,$,$,#3,(#4));\n#102=IFCRELAGGREGATES('agg-3',#2,$,$,#4,(#5));\n#103=IFCRELCONTAINEDINSPATIALSTRUCTURE('cont-1',#2,$,$,(#6),#5);\n#200=IFCPROPERTYSET('pset-1',#2,'Pset_WallCommon',$,(#201));\n#201=IFCPROPERTYSINGLEVALUE('IsExternal',$,IFCBOOLEAN(.T.),$);\n#202=IFCRELDEFINESBYPROPERTIES('rel-1',#2,$,$,(#6),#200);\nENDSEC;\nEND-ISO-10303-21;\n";

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture_doc() -> Part21Document {
    parse_part21(FIXTURE).expect("parse ifc fixture")
}

#[semio_framework_async_macros::async_test]
async fn spatial_hierarchy_matches_real_chain() {
    let doc = fixture_doc();
    let analysis = analyze_spatial(&doc);
    assert!(analysis.issues.is_empty(), "unexpected issues: {:?}", analysis.issues);
    assert_eq!(analysis.roots.len(), 1);
    let project = &analysis.roots[0];
    assert_eq!(project.ifc_type, "IFCPROJECT");
    assert_eq!(project.name.as_deref(), Some("Demo Project"));
    let site = &project.children[0];
    assert_eq!(site.ifc_type, "IFCSITE");
    let building = &site.children[0];
    assert_eq!(building.ifc_type, "IFCBUILDING");
    let storey = &building.children[0];
    assert_eq!(storey.ifc_type, "IFCBUILDINGSTOREY");
    let wall = &storey.children[0];
    assert_eq!(wall.ifc_type, "IFCWALL");
    assert_eq!(wall.name.as_deref(), Some("Wall-01"));
}

#[semio_framework_async_macros::async_test]
async fn placement_matrix_composes_across_four_levels() {
    let doc = fixture_doc();
    let analysis = analyze_spatial(&doc);
    let wall_placement_id = analysis.roots[0].children[0].children[0].children[0].children[0].object_placement.expect("wall placement");
    assert_eq!(wall_placement_id, 80);
    let world = analysis.placements.get(&80).expect("world matrix for wall placement");
    let origin = world.transform_point([0.0, 0.0, 0.0]);
    assert!((origin[0] - 1.0).abs() < 1e-9, "x: {origin:?}");
    assert!((origin[1] - 2.0).abs() < 1e-9, "y: {origin:?}");
    assert!((origin[2] - 10.0).abs() < 1e-9, "z: {origin:?}");
}

#[semio_framework_async_macros::async_test]
async fn property_set_attached_to_wall() {
    let doc = fixture_doc();
    let analysis = analyze_spatial(&doc);
    let psets = analysis.property_sets.get(&6).expect("wall property sets");
    assert_eq!(psets.len(), 1);
    assert_eq!(psets[0].name, "Pset_WallCommon");
    assert_eq!(psets[0].properties.len(), 1);
    assert_eq!(psets[0].properties[0].name, "IsExternal");
    let (typed_name, inner) = psets[0].properties[0].value.as_typed().expect("typed value");
    assert_eq!(typed_name, "IFCBOOLEAN");
    assert_eq!(inner[0].as_enum(), Some("T"));
}

#[semio_framework_async_macros::async_test]
async fn cyclic_placement_is_flagged_not_infinite_loop() {
    let cyclic = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCLOCALPLACEMENT(#2,#3);\n#2=IFCLOCALPLACEMENT(#1,#3);\n#3=IFCAXIS2PLACEMENT3D(#4,$,$);\n#4=IFCCARTESIANPOINT((0.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(cyclic).expect("parse cyclic fixture");
    let analysis = analyze_spatial(&doc);
    assert!(analysis.issues.iter().any(|i| i.contains("cyclic")));
}
