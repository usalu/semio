//! 🔢️ Borrow native IFC4 values without converting their graph or numeric identity.
use super::core::{DomainValue,DomainView,TypedView,NumberView};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcValue;
fn numeric_equal(integer:i64,real:f64)->bool{real.is_finite()&&real.fract()==0.0&&real>=i64::MIN as f64&&real<9223372036854775808.0&&real as i64==integer}
impl DomainValue for IfcValue{
fn number(&self)->Option<NumberView<'_>>{match self{Self::Integer(value)=>Some(NumberView::Integer(*value)),Self::Real(value)=>Some(NumberView::Binary64(*value)),_=>None}}
fn view(&self)->DomainView<'_,Self>{match self{Self::Unset=>DomainView::Unset,Self::Derived=>DomainView::Derived,Self::Integer(_)=>DomainView::Integer,Self::Real(_)=>DomainView::Real,Self::Reference(id)=>DomainView::Reference(*id),Self::String(text)=>DomainView::String(text),Self::Enum(text)=>DomainView::Enum(text),Self::Binary(text)=>DomainView::Binary(text),Self::Aggregate(items)=>DomainView::Aggregate(items),Self::TypedValue(typed)=>DomainView::TypedValue(TypedView{name:&typed.name,items:&typed.items})}}
fn numeric_valid(&self,_tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{Ok(matches!(self,Self::Integer(_))||matches!(self,Self::Real(number)if number.is_finite()))}
fn numbers_equal(&self,other:&Self,_tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{Ok(match(self,other){(Self::Integer(a),Self::Integer(b))=>a==b,(Self::Real(a),Self::Real(b))=>a==b,(Self::Integer(integer),Self::Real(real))|(Self::Real(real),Self::Integer(integer))=>numeric_equal(*integer,*real),_=>false})}
fn numbers_identical(&self,other:&Self,_tick:&mut dyn FnMut()->Result<(),String>)->Result<bool,String>{Ok(match(self,other){(Self::Integer(a),Self::Integer(b))=>a==b,(Self::Real(a),Self::Real(b))=>a.to_bits()==b.to_bits(),_=>false})}
}
