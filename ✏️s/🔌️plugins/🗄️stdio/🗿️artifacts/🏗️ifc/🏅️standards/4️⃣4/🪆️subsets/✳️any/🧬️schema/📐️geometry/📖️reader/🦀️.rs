//! 🏗️ Borrowed IFC4 representation graph reader.
use super::shared::graph::{GeometryConstituent,GeometryEntity,GeometryGraph,GeometryStandard,GeometryValue};
use super::super::snapshot::{IfcSnapshot,IfcValue};
pub struct Ifc4GeometryGraph<'a>(pub &'a IfcSnapshot);
impl GeometryValue for IfcValue{
 fn as_reference(&self)->Option<u64>{IfcValue::as_reference(self)}
 fn as_str(&self)->Option<&str>{IfcValue::as_str(self)}
 fn as_enum(&self)->Option<&str>{IfcValue::as_enum(self)}
 fn as_real(&self)->Option<f64>{IfcValue::as_real(self)}
 fn as_integer(&self)->Option<i64>{match self{Self::Integer(v)=>Some(*v),_=>None}}
 fn as_aggregate(&self)->Option<&[Self]>{IfcValue::as_aggregate(self)}
 fn as_typed(&self)->Option<(&str,&[Self])>{if let Self::TypedValue(value)=self{Some((&value.name,&value.items))}else{None}}
 fn is_absent(&self)->bool{matches!(self,Self::Unset|Self::Derived)}
 fn is_unset(&self)->bool{matches!(self,Self::Unset)}
 fn is_derived(&self)->bool{matches!(self,Self::Derived)}
}
impl GeometryGraph for Ifc4GeometryGraph<'_>{
 type Value=IfcValue;
 fn standard(&self)->GeometryStandard{GeometryStandard::Ifc4}
 fn source_identity(&self)->usize{std::ptr::from_ref(self.0) as usize}
 fn len(&self)->usize{self.0.entities.len()}
 fn entity(&self,index:usize)->Option<GeometryEntity<'_,Self::Value>>{let e=self.0.entities.get(index)?;Some(GeometryEntity{id:e.id,name:&e.name,args:&e.args,constituent_count:1+e.complex.len()})}
 fn constituent(&self,instance:usize,index:usize)->Option<GeometryConstituent<'_,Self::Value>>{let e=self.0.entities.get(instance)?;if index==0{Some(GeometryConstituent{name:&e.name,args:&e.args})}else{e.complex.get(index-1).map(|part|GeometryConstituent{name:&part.name,args:&part.args})}}
}
