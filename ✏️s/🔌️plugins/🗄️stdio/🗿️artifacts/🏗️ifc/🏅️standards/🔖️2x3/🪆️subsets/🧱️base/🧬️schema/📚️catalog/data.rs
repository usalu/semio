use semio_s_artifact_stdio_ifc_declarations::IfcCatalog;
pub fn build_catalog()->IfcCatalog{
let mut entities=std::collections::BTreeMap::new();
let mut types=std::collections::BTreeMap::new();
let mut functions=std::collections::BTreeMap::new();
let mut rules=std::collections::BTreeMap::new();
semio_s_artifact_stdio_ifc2x3_catalog_entities_00::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_01::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_02::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_03::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_04::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_05::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_entities_06::populate(&mut entities);
semio_s_artifact_stdio_ifc2x3_catalog_types_00::populate(&mut types);
semio_s_artifact_stdio_ifc2x3_catalog_functions_00::populate(&mut functions);
semio_s_artifact_stdio_ifc2x3_catalog_rules_00::populate(&mut rules);
IfcCatalog{schema:"IFC2X3".into(),version:"2.3.0.1".into(),source_url:"https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/EXPRESS/IFC2X3_TC1.exp".into(),source_mirror:"https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC2x3/TC1/EXPRESS/IFC2X3_TC1.exp".into(),source_sha256:"bf89eda341bccec041df37dffb79f0b8d4e3d5411d07541560d05bf69d53fc3a".into(),entities,types,functions,rules}
}
