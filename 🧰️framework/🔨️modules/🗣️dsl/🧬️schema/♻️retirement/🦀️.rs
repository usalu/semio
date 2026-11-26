//! ♻️ Finite Record ownership uses the neutral typed constructor and release authorities.
use super::{FieldValue,RecordSpecProducer,WireEdgeLabel,WireNode,WireValue};
use semio_framework_value::retirement::RetirementCursor;

semio_framework_value::artifact_retire_struct!(WireNode{id,kind,port});
semio_framework_value::artifact_retire_struct!(WireEdgeLabel{id,kind});
semio_framework_value::artifact_retire_struct!(WireValue{from,edge,edge_label,properties});
semio_framework_value::artifact_retire_leaf!(RecordSpecProducer);
semio_framework_value::artifact_retire_leaf!(super::RecordLayout,super::ExprOp);

/// ♻️ Retires intermediate native fields without recursive container drops.
pub fn retire_field(value:FieldValue){
    if matches!(&value,FieldValue::Absent|FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Text(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)){return;}
    let mut pending=vec![value];
    while let Some(value)=pending.pop(){match value{
        FieldValue::Tuple(mut items)|FieldValue::List(mut items)=>pending.append(&mut items),
        FieldValue::Record(record)=>pending.extend(record.fields.into_values()),
        FieldValue::Block(value)=>pending.push(*value),
        FieldValue::Statements(items)=>for(_,record)in items{pending.extend(record.fields.into_values());},
        FieldValue::Map(items)=>for(_,value)in items{pending.push(value);},
        FieldValue::Value(value)=><semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value),
        FieldValue::Wire(value)=><semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value.properties),
        FieldValue::Expr(value)=>{let mut expressions=vec![value];while let Some(value)=expressions.pop(){match value{super::ExprValue::Neg(value)=>expressions.push(*value),super::ExprValue::Binary(_,left,right)=>{expressions.push(*left);expressions.push(*right);},super::ExprValue::Call(_,mut items)=>expressions.append(&mut items),super::ExprValue::Num(_)|super::ExprValue::Var(_)=>{}}}},
        FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Text(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)|FieldValue::Absent=>{}
    }}
}
