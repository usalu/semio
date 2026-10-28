//! 🧊️ ADD2 attribute queries share one retained metadata roster and the actual immutable native population.
#[path="../../../../../../📚️catalog/🔎️query/🦀️.rs"]
mod shared;
pub use shared::*;
#[path="query-native.rs"]
mod native;
pub use native::with_ifc4x3_native_attribute;
use crate::validation::metadata;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcSnapshot,IfcValue};
pub struct Ifc4x3AttributeMetadata;
impl IfcAttributeQueryMetadata for Ifc4x3AttributeMetadata{
    type Attribute=metadata::StaticAttribute;type Expression=metadata::StaticTypeExpression;
    fn attributes(name:&str,direct:bool)->Option<IfcQueryAttributes<Self::Attribute>>{let declaration=metadata::entity(name)?;Some(IfcQueryAttributes{key:declaration.key,attributes:if direct{declaration.attributes}else{declaration.all_attributes}})}
    fn name(attribute:&'static Self::Attribute)->&'static str{attribute.name}
    fn declared_by(attribute:&'static Self::Attribute)->&'static str{attribute.declared_by}
    fn type_expr(attribute:&'static Self::Attribute)->&'static Self::Expression{&attribute.type_expr}
}
impl IfcAttributeQueryPopulation for IfcSnapshot{
    type Value=IfcValue;
    fn query_part_count(&self,row:usize)->Option<usize>{self.entities.get(row)?.complex.len().checked_add(1)}
    fn query_part(&self,row:usize,part:usize)->Option<IfcQueryPart<'_,Self::Value>>{let entity=self.entities.get(row)?;if part==0{Some(IfcQueryPart{name:&entity.name,args:&entity.args})}else{let part=entity.complex.get(part-1)?;Some(IfcQueryPart{name:&part.name,args:&part.args})}}
}
#[cfg(test)]
#[path="../../../../../../📚️catalog/🔎️query/🧪️tests.rs"]
mod tests;
#[cfg(test)]
#[path="query-native-tests.rs"]
mod native_tests;
