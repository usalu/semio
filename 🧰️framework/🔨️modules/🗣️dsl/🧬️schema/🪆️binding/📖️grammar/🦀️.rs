//! 📖️ Typed native record fields bind their unordered presence contract into explicit grammar productions.
use crate::{DslField,FieldValue,ParseOptions,Shape,SourceMode,parse_exact};
use semio_framework_dsl::MacroMatcher;

/// 🪪️ Recognizes only the exact declared field body, including required, duplicate and nullable boundaries.
pub fn record_fields_matcher<T:DslField>(name:&'static str)->MacroMatcher{MacroMatcher{name,try_match:accepts::<T>}}
fn accepts<T:DslField>(source:&str)->bool{
    let Shape::Record(producer)=T::shape()else{return false;};let mut spec=(producer.ordinary)();spec.keyword=None;
    match parse_exact(source,&spec,&ParseOptions{mode:SourceMode::Inline,..Default::default()}){Ok(record)=>match T::from_value(&FieldValue::Record(record)){Ok(value)=>{T::retire_decoded(value);true},Err(_)=>false},Err(_)=>false}
}
