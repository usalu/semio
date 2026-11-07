//! 🔗️ A tuple newtype over a first-party scalar receives exactly like its inner scalar through one macro declaration.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedScalar};
use semio_framework_dsl_record::{DslReceivingField,FieldValue};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Id(u32);
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
struct Name(String);
semio_framework_value::artifact_retire_leaf!(Id);
semio_framework_dsl_record::dsl_scalar_newtype_receiving!(Id,u32);
semio_framework_dsl_record::dsl_scalar_newtype_receiving!(Name,String);

fn implements<T:DslReceivingField+DslOwnedReceivingField+OwnedScalar>(){}

#[test]
fn newtypes_move_their_scalar_and_leave_no_source(){
 implements::<Id>();
 implements::<Name>();
 let mut source=Some(FieldValue::UInt(7));
 assert_eq!(<Id as OwnedScalar>::take_source(&mut source).unwrap(),Id(7));
 assert!(source.is_none());
 let mut source=Some(FieldValue::Text("a".into()));
 assert_eq!(<Name as OwnedScalar>::take_source(&mut source).unwrap(),Name("a".into()));
 assert!(source.is_none());
}

#[test]
fn newtypes_account_like_the_inner_scalar_and_refuse_foreign_shapes_without_consuming(){
 assert_eq!(<Id as OwnedScalar>::creation_bytes(),<u32 as OwnedScalar>::creation_bytes());
 assert_eq!(<Name as OwnedScalar>::creation_bytes(),<String as OwnedScalar>::creation_bytes());
 for foreign in [FieldValue::Bool(true),FieldValue::UInt(u64::MAX),FieldValue::Absent]{
  let mut source=Some(foreign.clone());
  assert!(<Id as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
  assert!(source.is_some());
 }
}

#[test]
fn arrays_of_newtypes_are_receivable(){
 fn array<T:DslOwnedReceivingField>(){}
 array::<[Id;3]>();
}
