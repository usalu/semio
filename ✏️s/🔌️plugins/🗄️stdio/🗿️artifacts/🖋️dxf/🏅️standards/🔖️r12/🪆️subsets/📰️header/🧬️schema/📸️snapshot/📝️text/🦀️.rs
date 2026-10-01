//! 📝️ Complete six-root owned DXF snapshot persistence.
use super::*;
use pack::value::{DslValue,FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";
pub(super) fn spec()->dsl::RecordSpec{
    use dsl::{FieldSpec as F,Shape as S};
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,vec![F::new(1,"schema",S::Text),F::new(2,"headerVars",S::Value),F::new(3,"tables",S::Value),F::new(4,"otherTables",S::Value),F::new(5,"blocks",S::Value),F::new(6,"entities",S::Value)])
}
pub(super) fn to_record(snapshot:&DxfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[(1,V::Text(snapshot.schema.clone())),(2,V::Value(snapshot.header_vars.to_value())),(3,V::Value(snapshot.tables.to_value())),(4,V::Value(snapshot.other_tables.to_value())),(5,V::Value(snapshot.blocks.to_value())),(6,V::Value(snapshot.entities.to_value()))].into_iter().collect()}
}
pub(super) fn from_record(record:&dsl::RecordValue)->Result<DxfSnapshot,store::TextError>{
    if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(store::TextError::new("DXF snapshot contains an undeclared root field",dsl::TextSpan::at(1,1)));}
    let mut fields=Vec::with_capacity(6);
    for(id,key)in[(1,"schema"),(2,"headerVars"),(3,"tables"),(4,"otherTables"),(5,"blocks"),(6,"entities")]{
        let value=match record.get(id){Some(dsl::FieldValue::Text(value))if id==1=>DslValue::String(value.clone()),Some(dsl::FieldValue::Value(value))if id>1=>value.clone(),_=>return Err(store::TextError::new(format!("DXF snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)))};
        fields.push((key.into(),value));
    }
    DxfSnapshot::from_value(DslValue::Object(fields)).map_err(|error|store::TextError::new(error.to_string(),dsl::TextSpan::at(1,1)))
}
