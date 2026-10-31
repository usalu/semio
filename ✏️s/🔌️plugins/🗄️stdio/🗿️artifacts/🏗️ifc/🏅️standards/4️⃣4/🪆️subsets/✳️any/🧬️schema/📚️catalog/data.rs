use semio_s_artifact_stdio_ifc_declarations::IfcCatalog;
pub fn build_catalog()->IfcCatalog{
let mut entities=std::collections::BTreeMap::new();
let mut types=std::collections::BTreeMap::new();
let mut functions=std::collections::BTreeMap::new();
let mut rules=std::collections::BTreeMap::new();
semio_s_artifact_stdio_ifc4_catalog_entities_00::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_01::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_02::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_03::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_04::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_05::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_06::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_07::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_entities_08::populate(&mut entities);
semio_s_artifact_stdio_ifc4_catalog_types_00::populate(&mut types);
semio_s_artifact_stdio_ifc4_catalog_functions_00::populate(&mut functions);
semio_s_artifact_stdio_ifc4_catalog_rules_00::populate(&mut rules);
IfcCatalog{schema:"IFC4".into(),version:"4.0.2.1".into(),source_url:"https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp".into(),source_mirror:"https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp".into(),source_sha256:"a3e46d39a85c2b683e7167572165d74b4ff6f8ef7e7c1e7f314a4980a63c9edb".into(),entities,types,functions,rules}
}
