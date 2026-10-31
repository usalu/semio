use semio_s_artifact_stdio_ifc_declarations::IfcCatalog;
pub fn build_catalog()->IfcCatalog{
let mut entities=std::collections::BTreeMap::new();
let mut types=std::collections::BTreeMap::new();
let mut functions=std::collections::BTreeMap::new();
let mut rules=std::collections::BTreeMap::new();
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_00::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_01::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_02::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_03::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_04::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_05::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_06::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_07::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_08::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_09::populate(&mut entities);
semio_s_artifact_stdio_ifc4x3_add2_catalog_types_00::populate(&mut types);
semio_s_artifact_stdio_ifc4x3_add2_catalog_functions_00::populate(&mut functions);
semio_s_artifact_stdio_ifc4x3_add2_catalog_rules_00::populate(&mut rules);
IfcCatalog{schema:"IFC4X3_ADD2".into(),version:"4.3.2.0".into(),source_url:"https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp".into(),source_mirror:"https://raw.githubusercontent.com/buildingSMART/Standards-builds/main/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp".into(),source_sha256:"f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860".into(),entities,types,functions,rules}
}
