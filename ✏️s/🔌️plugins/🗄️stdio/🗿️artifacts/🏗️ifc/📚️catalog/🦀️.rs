//! 📚️ Schema-neutral retained EXPRESS declarations and borrowed queries.
use std::collections::{BTreeMap,BTreeSet};
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IfcTypeExpression {
    pub kind: String,
    pub name: Option<String>,
    pub width: Option<i64>,
    pub fixed: bool,
    pub aggregate: Option<String>,
    pub lower: Option<i64>,
    pub upper: Option<i64>,
    pub lower_expression: Option<String>,
    pub upper_expression: Option<String>,
    pub unique: bool,
    pub optional: bool,
    pub item: Option<Box<IfcTypeExpression>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfcAttributeDeclaration {
    pub name: String,
    pub type_expr: IfcTypeExpression,
    pub optional: bool,
    pub derived: bool,
    pub declared_by: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfcDerivedDeclaration {
    pub name: String,
    pub type_expr: IfcTypeExpression,
    pub redeclares: Option<String>,
    pub expression: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfcEntityDeclaration {
    pub name: String,
    pub abstract_entity: bool,
    pub supertype: Option<String>,
    pub supertype_expression:String,
    pub attributes: Vec<IfcAttributeDeclaration>,
    pub all_attributes: Vec<IfcAttributeDeclaration>,
    pub derived: Vec<IfcDerivedDeclaration>,
    pub inverse: Vec<String>,
    pub where_rules: Vec<String>,
    pub unique_rules: Vec<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfcTypeDeclaration {
    pub name: String,
    pub kind: String,
    pub members: Vec<String>,
    pub type_expr: Option<IfcTypeExpression>,
    pub where_rules: Vec<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IfcCatalog {
    pub schema: String,
    pub version:String,
    pub source_url:String,
    pub source_mirror:String,
    pub source_sha256: String,
    pub entities: BTreeMap<String, IfcEntityDeclaration>,
    pub types: BTreeMap<String, IfcTypeDeclaration>,
    pub functions: BTreeMap<String, String>,
    pub rules: BTreeMap<String, String>,
}

/// 🔎 Borrow a named declaration in its explicit schema namespace.
pub enum IfcDeclaration<'a>{Entity(&'a IfcEntityDeclaration),Type(&'a IfcTypeDeclaration)}
pub fn ifc_declaration<'a>(catalog:&'a IfcCatalog,name:&str)->Option<IfcDeclaration<'a>>{let key=name.to_ascii_uppercase();catalog.entities.get(&key).map(IfcDeclaration::Entity).or_else(||catalog.types.get(&key).map(IfcDeclaration::Type))}
/// 🧬 Borrow original inherited Part-21 serialization attributes.
pub fn ifc_attributes<'a>(catalog:&'a IfcCatalog,name:&str)->Option<&'a[IfcAttributeDeclaration]>{catalog.entities.get(&name.to_ascii_uppercase()).map(|e|e.all_attributes.as_slice())}
/// 🌳 Resolve a declared entity's supertypes with finite cycle protection.
pub fn ifc_subtype(catalog:&IfcCatalog,actual:&str,expected:&str)->bool{let mut current=actual.to_ascii_uppercase();let target=expected.to_ascii_uppercase();let mut seen=BTreeSet::new();loop{if !seen.insert(current.clone()){return false;}let Some(e)=catalog.entities.get(&current)else{return false;};if current==target{return true;}let Some(parent)=e.supertype.as_ref()else{return false;};current=parent.clone();}}
pub fn ifc_attribute<'a>(catalog:&'a IfcCatalog,entity:&str,index:usize)->Option<&'a IfcAttributeDeclaration>{ifc_attributes(catalog,entity)?.get(index)}
/// 🪢 Follow defined type aliases without losing aggregate and select domains.
pub fn ifc_resolve_type<'a>(catalog:&'a IfcCatalog,expression:&'a IfcTypeExpression)->Option<&'a IfcTypeExpression>{let mut current=expression;let mut seen=BTreeSet::new();while current.kind=="named"{let name=current.name.as_ref()?.to_ascii_uppercase();if !seen.insert(name.clone()){return None;}let Some(t)=catalog.types.get(&name)else{return Some(current);};if t.kind!="defined"{return Some(current);}current=t.type_expr.as_ref()?;}Some(current)}
fn named_domain<'a>(catalog:&'a IfcCatalog,name:&str)->Option<&'a IfcTypeDeclaration>{let mut current=name.to_ascii_uppercase();let mut seen=BTreeSet::new();loop{if !seen.insert(current.clone()){return None;}let t=catalog.types.get(&current)?;if t.kind!="defined"{return Some(t);}let expression=t.type_expr.as_ref()?;if expression.kind!="named"{return None;}current=expression.name.as_ref()?.to_ascii_uppercase();}}
/// 🔤 Borrow a named enumeration's exact closed domain.
pub fn ifc_enum_members<'a>(catalog:&'a IfcCatalog,name:&str)->Option<&'a[String]>{let t=named_domain(catalog,name)?;(t.kind=="enum").then_some(t.members.as_slice())}
/// 🔀 Borrow a select's declared members without flattening their semantics.
pub fn ifc_select_members<'a>(catalog:&'a IfcCatalog,name:&str)->Option<&'a[String]>{let t=named_domain(catalog,name)?;(t.kind=="select").then_some(t.members.as_slice())}
/// 🎯 Resolve nested select membership and entity subtyping against an explicit catalog.
pub fn ifc_select_accepts(catalog:&IfcCatalog,select:&str,actual:&str)->bool{fn accepts(catalog:&IfcCatalog,name:&str,target:&str,seen:&mut BTreeSet<String>)->bool{if !seen.insert(name.into()){return false;}if name==target{return catalog.entities.contains_key(name)||catalog.types.contains_key(name);}if ifc_subtype(catalog,target,name){return true;}ifc_select_members(catalog,name).is_some_and(|members|members.iter().any(|item|accepts(catalog,item,target,seen)))}let mut seen=BTreeSet::new();let target=actual.to_ascii_uppercase();ifc_select_members(catalog,select).is_some_and(|members|members.iter().any(|name|accepts(catalog,name,&target,&mut seen)))}

impl IfcCatalog {
    pub fn is_subtype(&self,actual:&str,expected:&str)->bool{ifc_subtype(self,actual,expected)}
    pub fn attribute_declarations(&self,name:&str)->Option<&[IfcAttributeDeclaration]>{ifc_attributes(self,name)}
}
