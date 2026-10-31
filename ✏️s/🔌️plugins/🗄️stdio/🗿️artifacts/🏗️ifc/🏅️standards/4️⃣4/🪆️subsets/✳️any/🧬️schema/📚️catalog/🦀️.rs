//! 🏗️ Normative IFC4 ADD2 TC1 declarations without a runtime parser.
use semio_s_artifact_stdio_ifc_declarations as shared;
pub use shared::{IfcCatalog,IfcDeclaration,IfcTypeExpression,IfcAttributeDeclaration,IfcDerivedDeclaration,IfcEntityDeclaration,IfcTypeDeclaration};
/// 📏️ Largest inherited serialized tuple in this pinned normative profile.
pub const IFC4_MAXIMUM_ATTRIBUTES: usize = 21;
use semio_s_artifact_stdio_ifc4_catalog::build_catalog;
pub fn ifc4_catalog()->&'static IfcCatalog{static CATALOG:std::sync::OnceLock<IfcCatalog>=std::sync::OnceLock::new();CATALOG.get_or_init(build_catalog)}
pub fn ifc4_declaration(name:&str)->Option<IfcDeclaration<'static>>{shared::ifc_declaration(ifc4_catalog(),name)}
pub fn ifc4_attribute_declarations(name:&str)->Option<&'static[IfcAttributeDeclaration]>{shared::ifc_attributes(ifc4_catalog(),name)}
pub fn ifc4_is_subtype(actual:&str,expected:&str)->bool{shared::ifc_subtype(ifc4_catalog(),actual,expected)}
pub fn ifc4_attribute(entity:&str,index:usize)->Option<&'static IfcAttributeDeclaration>{shared::ifc_attribute(ifc4_catalog(),entity,index)}
pub fn ifc4_resolve_type<'a>(expression:&'a IfcTypeExpression)->Option<&'a IfcTypeExpression>{shared::ifc_resolve_type(ifc4_catalog(),expression)}
pub fn ifc4_enum_members(name:&str)->Option<&'static[String]>{shared::ifc_enum_members(ifc4_catalog(),name)}
pub fn ifc4_select_members(name:&str)->Option<&'static[String]>{shared::ifc_select_members(ifc4_catalog(),name)}
pub fn ifc4_select_accepts(select:&str,actual:&str)->bool{shared::ifc_select_accepts(ifc4_catalog(),select,actual)}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[cfg(test)]
mod capacity_tests {
    use super::*;
    #[test]
    fn inherited_tuple_bound_matches_the_neutral_profile_facet() {
        let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../../../../../../🔎️details/🧪️tests/capacity.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let row = &fixture["cases"][0];
        assert_eq!(IFC4_MAXIMUM_ATTRIBUTES as u64, row["maximumAttributes"].as_u64().unwrap());
        assert_eq!(ifc4_catalog().entities.values().map(|entity| entity.all_attributes.len()).max().unwrap(), IFC4_MAXIMUM_ATTRIBUTES);
    }
}
