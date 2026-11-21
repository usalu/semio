//! 🏷️ Canonical seven-variant value types bind through their actual owned intrinsic tree.
use semio_framework_value::{FromValue, ToValue, ValueType};
use crate::{DslField, FieldValue, Shape, NativeSchemaControl, NativeEncodeControl, NativeDecodeControl, ValueError};
impl DslField for ValueType {
    fn shape() -> Shape { Shape::Value }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self)->FieldValue{FieldValue::Value(ToValue::to_value(self))}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{Ok(FieldValue::Value(ToValue::to_value_controlled(self,control)?))}
    fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value(value.clone()).map_err(ValueError::into_message),_=>Err("expected a typed value declaration".into())}}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected a typed value declaration"))}}
    fn retire_decoded(self){<Self as FromValue>::retire_decoded(self)}
}

use crate::receiving::owned::{
    admits, done, idle, invalid, invariant, receipt, DslOwnedReceivingField, MappedReceiving,
    OwnedReceivingMap, OwnedTake,
};
use semio_framework_value::{DslValue, RetainedAllocationEffect, RetirementDemand, ValueRefusalKind};
use std::mem::size_of;

enum ValueTypeLeaf<'a> {
    Primitive(ValueType),
    Schema(&'a str),
}

impl ValueTypeLeaf<'_> {
    fn text_bytes(&self) -> usize {
        match self {
            Self::Primitive(_) => 0,
            Self::Schema(id) => id.len(),
        }
    }

    fn into_value(self) -> ValueType {
        match self {
            Self::Primitive(value) => value,
            Self::Schema(id) => ValueType::Schema(id.to_owned()),
        }
    }
}

fn value_type_shape(mut input: &DslValue) -> Result<(usize, ValueTypeLeaf<'_>), ValueError> {
    for depth in 0..64 {
        let DslValue::Object(fields) = input else { return Err(invalid()) };
        if fields.is_empty() || fields.len() > 2 {
            return Err(invalid());
        }
        let Some(DslValue::String(kind)) = input.get("kind") else { return Err(invalid()) };
        let of = input.get("of");
        if fields.len() != 1 + usize::from(of.is_some()) {
            return Err(invalid());
        }
        let value = match (kind.as_str(), of) {
            ("boolean", None) => ValueTypeLeaf::Primitive(ValueType::Boolean),
            ("integer", None) => ValueTypeLeaf::Primitive(ValueType::Integer),
            ("decimal", None) => ValueTypeLeaf::Primitive(ValueType::Decimal),
            ("text", None) => ValueTypeLeaf::Primitive(ValueType::Text),
            ("any", None) => ValueTypeLeaf::Primitive(ValueType::Any),
            ("schema", Some(DslValue::String(id))) => ValueTypeLeaf::Schema(id),
            ("list", Some(inner)) => {
                input = inner;
                continue;
            }
            _ => return Err(invalid()),
        };
        return Ok((depth, value));
    }
    Err(ValueError::literal(ValueRefusalKind::DepthLimit, "value type exceeds depth limit"))
}

fn type_demand(depth: usize, leaf: &ValueTypeLeaf<'_>) -> RetirementDemand {
    let bytes = depth * size_of::<ValueType>() + leaf.text_bytes();
    RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes, depth: depth + 1, ..Default::default() }
}

/// 🗺️ Binds a canonical type while retaining the source tree for funded retirement.
pub struct ValueTypeMap;

impl OwnedReceivingMap<DslValue> for ValueTypeMap {
    type Output = ValueType;

    fn map_demand(input: &DslValue) -> Result<RetirementDemand, ValueError> {
        let (depth, leaf) = value_type_shape(input)?;
        Ok(type_demand(depth, &leaf))
    }

    fn map_work_demand(input: &DslValue) -> Result<u64, ValueError> {
        let (depth, leaf) = value_type_shape(input)?;
        Ok(1 + depth as u64 + leaf.text_bytes() as u64)
    }

    fn map_into(input: &mut Option<DslValue>, output: &mut Option<ValueType>, effect: &mut RetainedAllocationEffect<'_>) -> OwnedTake {
        let Some(input) = input.as_ref().filter(|_| output.is_none()) else { return done(effect, Err(invariant())) };
        let (depth, leaf) = match value_type_shape(input) {
            Ok(shape) => shape,
            Err(error) => return done(effect, Err(error)),
        };
        let demand = type_demand(depth, &leaf);
        if !admits(effect, demand) {
            return idle(effect);
        }
        let work = 1 + depth as u64 + leaf.text_bytes() as u64;
        let result = effect.performed(work).and_then(|performed| {
            if !performed {
                return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "value type exceeds work grant"));
            }
            let mut value = leaf.into_value();
            for _ in 0..depth {
                value = ValueType::List(Box::new(value));
            }
            *output = Some(value);
            Ok(receipt(demand))
        });
        done(effect, result)
    }
}

impl DslOwnedReceivingField for ValueType {
    type Cursor = MappedReceiving<DslValue, ValueTypeMap>;

    fn receiving_cursor() -> Self::Cursor {
        Self::Cursor::new()
    }
}

impl crate::DslReceivingField for ValueType {
    fn receive_value(
        value: &FieldValue,
        output: &mut Option<Self>,
        owner: &mut crate::receiving::NativeSnapshotDecodeOwner<'_, '_>,
    ) -> crate::receiving::ReceivingOutcome {
        crate::receiving::receive_decoded(value, output, owner)
    }
}
