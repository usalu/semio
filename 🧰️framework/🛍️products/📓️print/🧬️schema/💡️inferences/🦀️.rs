//! 💡️ Chart inference admits authored values and owns its semantic result.
use crate::ChartSnapshot;
use semio_framework_value::{DslValue,ToValue as _};
use semio_framework_value_derive::{FromValue,ToValue};
use protocol::{Inference, InferenceSpec, InferenceFieldSpec};
#[path="🎨theme/🦀️.rs"]
pub mod paint;

pub fn validate_chart(snapshot: &ChartSnapshot) -> Result<(), String> {
    static VALIDATOR: std::sync::OnceLock<Result<semio_framework_schema_validator::OwnedJsonSchemaValidator, String>> = std::sync::OnceLock::new();
    let validator = VALIDATOR.get_or_init(|| semio_framework_schema_validator::OwnedJsonSchemaValidator::compile_intrinsic_with_documents(&semio_framework_value_derive::owned_json_file!("../../🧬️schema/📸️snapshot/🔣️.json"), &[semio_framework_value_derive::owned_json_file!("../../🧬️schema/🔣️.json")]).map_err(|error| error.to_string()));
    validator.as_ref().map_err(Clone::clone)?.validate_intrinsic(&snapshot.to_value()).map(|_| ()).map_err(|error| error.to_string())
}

pub fn catalog() -> Result<&'static DslValue, String> {
    static CATALOG: std::sync::OnceLock<DslValue> = std::sync::OnceLock::new();
    Ok(CATALOG.get_or_init(|| semio_framework_value_derive::owned_json_file!("../../🖼️assets/🔣️viz-catalog.json")))
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct ChartDiagnostic { pub code:String,pub path:String,pub message:String }
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct ChartInference {
    #[value(default,skip_serializing_if="Option::is_none")]
    pub chart:Option<ChartSnapshot>,
    pub diagnostics:Vec<ChartDiagnostic>,
    pub complete:bool,
}
impl Default for ChartInference {fn default()->Self{Self::infer(&ChartSnapshot::default()).expect("owned chart inference describes admission")}}
impl Inference<ChartSnapshot> for ChartInference {
    fn infer(snapshot:&ChartSnapshot)->Result<Self,semio_framework_value::ValueError>{
        let result=validate_chart(snapshot).and_then(|()|match snapshot.chart.get("language").and_then(DslValue::as_str){Some("en"|"de")=>Ok(()),_=>Err("chart language must be explicitly en or de".into())});
        Ok(match result{Ok(())=>Self{chart:Some(snapshot.clone()),diagnostics:Vec::new(),complete:true},Err(message)=>Self{chart:None,diagnostics:vec![ChartDiagnostic{code:"print.chart.inference".into(),path:"chart".into(),message}],complete:false}})
    }
}
impl InferenceSpec<ChartSnapshot> for ChartInference {
    fn inference_schema_id()->&'static str{"framework.print.chart.inference"}
    fn schema_version()->u32{1}
    fn fields()->&'static [InferenceFieldSpec]{&[InferenceFieldSpec{id:"framework.print.chart.inference.chart",reads:&["chart"]},InferenceFieldSpec{id:"framework.print.chart.inference.diagnostics",reads:&["chart"]}]}
}
