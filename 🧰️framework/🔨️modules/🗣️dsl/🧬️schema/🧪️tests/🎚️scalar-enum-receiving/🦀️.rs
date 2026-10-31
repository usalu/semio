//! 🎚️ A unit-variant scalar enum receives and moves its authored ordinal exactly, and refuses every other field shape.
use semio_framework_dsl_record::receiving::owned::{DslOwnedReceivingField,OwnedScalar};
use semio_framework_dsl_record::{DslReceivingField,FieldValue};

#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_dsl_record_derive::DslScalar,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
enum Direction{Asc,Desc}

fn implements<T:DslReceivingField+DslOwnedReceivingField+OwnedScalar>(){}

#[test]
fn scalar_enum_ordinals_move_into_their_variants_and_leave_no_source(){
 implements::<Direction>();
 let mut source=Some(FieldValue::Enum(1));
 assert_eq!(<Direction as OwnedScalar>::take_source(&mut source).unwrap(),Direction::Desc);
 assert!(source.is_none());
 let mut source=Some(FieldValue::Enum(0));
 assert_eq!(<Direction as OwnedScalar>::take_source(&mut source).unwrap(),Direction::Asc);
}

#[test]
fn scalar_enum_refuses_unknown_ordinals_and_foreign_shapes_without_consuming_the_source(){
 for foreign in [FieldValue::Enum(2),FieldValue::Bool(true),FieldValue::Absent]{
  let mut source=Some(foreign.clone());
  assert!(<Direction as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
  assert!(source.is_some());
 }
}
