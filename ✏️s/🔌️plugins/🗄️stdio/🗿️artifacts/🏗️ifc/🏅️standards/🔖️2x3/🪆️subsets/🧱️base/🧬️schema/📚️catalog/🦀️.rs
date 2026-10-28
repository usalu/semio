//! 🏗️ Normative IFC2x3 TC1 declarations without a runtime parser.
#[path="../../../../../../📚️catalog/🦀️.rs"]mod shared;
pub use shared::{IfcCatalog,IfcDeclaration,IfcTypeExpression,IfcAttributeDeclaration,IfcDerivedDeclaration,IfcEntityDeclaration,IfcTypeDeclaration};
include!("data.rs");
pub fn ifc2x3_catalog()->&'static IfcCatalog{static CATALOG:std::sync::OnceLock<IfcCatalog>=std::sync::OnceLock::new();CATALOG.get_or_init(build_catalog)}
pub fn ifc2x3_declaration(name:&str)->Option<IfcDeclaration<'static>>{shared::ifc_declaration(ifc2x3_catalog(),name)}
pub fn ifc2x3_attribute_declarations(name:&str)->Option<&'static[IfcAttributeDeclaration]>{shared::ifc_attributes(ifc2x3_catalog(),name)}
pub fn ifc2x3_is_subtype(actual:&str,expected:&str)->bool{shared::ifc_subtype(ifc2x3_catalog(),actual,expected)}
pub fn ifc2x3_attribute(entity:&str,index:usize)->Option<&'static IfcAttributeDeclaration>{shared::ifc_attribute(ifc2x3_catalog(),entity,index)}
pub fn ifc2x3_resolve_type<'a>(expression:&'a IfcTypeExpression)->Option<&'a IfcTypeExpression>{shared::ifc_resolve_type(ifc2x3_catalog(),expression)}
pub fn ifc2x3_enum_members(name:&str)->Option<&'static[String]>{shared::ifc_enum_members(ifc2x3_catalog(),name)}
pub fn ifc2x3_select_members(name:&str)->Option<&'static[String]>{shared::ifc_select_members(ifc2x3_catalog(),name)}
pub fn ifc2x3_select_accepts(select:&str,actual:&str)->bool{shared::ifc_select_accepts(ifc2x3_catalog(),select,actual)}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
