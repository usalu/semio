
use super::*;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.ifc','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROJECT('gid-project',#2,'Demo Project',$,$,$,$,(#10),#11);\n#2=IFCOWNERHISTORY($,$,$,$,$,$,$,0);\n#6=IFCWALL('gid-wall',#2,'Wall-01',$,$,#80,$,$,$);\nENDSEC;\nEND-ISO-10303-21;\n";

#[semio_framework_async_macros::async_test]
async fn part21_round_trip_is_lossless() {
    let doc = parse_part21(FIXTURE).expect("parse");
    let snapshot = from_part21_document("stdio.ifc", &doc);
    assert_eq!(snapshot.entities.len(), 3);
    let wall = snapshot.entities.iter().find(|e| e.id == 6).expect("wall entity");
    assert_eq!(wall.name, "IFCWALL");
    assert_eq!(wall.args[2], IfcValue::String("Wall-01".into()));
    let back = to_part21_document(&snapshot);
    assert_eq!(back, doc, "snapshot <-> Part21Document must be lossless");
}

#[semio_framework_async_macros::async_test]
async fn complex_instance_retains_every_type() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
    let doc = parse_part21(text).expect("parse complex instance");
    let snapshot = from_part21_document("stdio.ifc", &doc);
    let e = &snapshot.entities[0];
    assert_eq!(e.name, "IFCQUANTITYAREA");
    assert_eq!(e.complex.len(), 1);
    assert_eq!(e.complex[0].name, "IFCPHYSICALSIMPLEQUANTITY");
    assert_eq!(to_part21_document(&snapshot), doc);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip_via_dsl_and_pack() {
    let snapshot = from_part21_document("stdio.ifc", &parse_part21(FIXTURE).expect("parse"));
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let parsed = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse_dsl");
    assert_eq!(parsed, snapshot);
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <IfcSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode_pack");
    assert_eq!(decoded, snapshot);
}
