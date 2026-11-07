//! 📥️ Viewport poses receive their record encoding through the bounded scalar receiver and agree with the codec.
use crate::{Viewport2d,Viewport3dOrbit};
use semio_framework_dsl_record::{receiving::owned::invalid,DslField,FieldValue};
use semio_framework_value::ValueError;

fn bounded<T:DslField>(value:&FieldValue,fields:usize)->Result<T,ValueError>{match value{FieldValue::Record(record) if record.fields.len()<=fields=>T::from_value(value).map_err(|_|invalid()),_=>Err(invalid())}}
fn planar(value:&FieldValue)->Result<Viewport2d,ValueError>{bounded(value,3)}
fn orbit(value:&FieldValue)->Result<Viewport3dOrbit,ValueError>{bounded(value,4)}
semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy Viewport2d,planar);
semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy Viewport3dOrbit,orbit);
