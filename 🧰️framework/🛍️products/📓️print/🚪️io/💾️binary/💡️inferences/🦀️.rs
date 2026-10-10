//! 📦️ Chart inference transport binds binary admission, native text output and canonical publication.
use crate::{ChartSnapshot,inferences,io};

pub fn execute_chart_inference(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    execute_chart_inference_controlled(request, &mut |_| {
        if semio_framework_plugin::app::inference_cancelled(request.cancellation_id)?{Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("artifact-inference.cancelled","chart inference cancelled"))}else{Ok(())}
    })
}

/// 🚦️ The controlled service binds progress, cancellation and allocation/work budgets.
pub fn execute_chart_inference_controlled(request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>, checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
    use semio_framework_value::{FromValue, ToValue};
    use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionError as Error, WireArtifactInferenceCacheMode};
    if request.cancellation_id.trim().is_empty() || request.budgets.work_units == 0 || request.budgets.recursion_depth == 0 || request.canonical_payload.len() as u64 > request.budgets.allocation_bytes {
        return Err(Error::new("print.chart.inference.admission", "cancellation identity and sufficient non-zero budgets are required"));
    }
    if request.requested_cache_mode == WireArtifactInferenceCacheMode::Incremental { return Err(Error::new("print.chart.inference.cache", "incremental inference is not declared")); }
    checkpoint(0)?;
    let mut decode_options = protocol::PackDecodeOptions::default();
    decode_options.limits.max_total_alloc = request.budgets.allocation_bytes;
    let value = protocol::pack_rt::decode_wire_value_with_options(request.canonical_payload,&decode_options).map_err(|error| Error::new("print.chart.inference.decode", error.to_string()))?;
    let mut stack=vec![(&value,0u32)];
    let mut decode_work=0;
    while let Some((value,depth))=stack.pop(){
        if depth>request.budgets.recursion_depth{return Err(Error::new("print.chart.inference.depth","chart value nesting exceeds recursion budget"));}
        decode_work+=1;
        if decode_work>request.budgets.work_units{return Err(Error::new("print.chart.inference.work","chart values exceed work budget"));}
        checkpoint(decode_work)?;
        match value{semio_framework_value::DslValue::Array(items)=>stack.extend(items.iter().map(|value|(value,depth+1))),semio_framework_value::DslValue::Object(items)=>stack.extend(items.iter().map(|(_,value)|(value,depth+1))),_=>{}}
    }
    let snapshot = ChartSnapshot::from_value(value).map_err(|error| Error::new("print.chart.inference.snapshot", error.to_string()))?;
    let mut cancellation=None;
    let result = io::text::inferences::render_chart_controlled(&snapshot, &mut |work| {
        if work.saturating_add(decode_work) > request.budgets.work_units { return Err("inference work budget exhausted".into()); }
        checkpoint(work.saturating_add(decode_work)).map_err(|error| {let message=error.to_string();cancellation=Some(error);message})
    });
    if let Some(error)=cancellation{return Err(error);}
    let inference=match result{
        Ok(tikz)=>io::text::inferences::ChartTextOutput{tikz,diagnostics:Vec::new(),complete:true},
        Err(message)=>io::text::inferences::ChartTextOutput{tikz:String::new(),diagnostics:vec![inferences::ChartDiagnostic{code:"print.chart.inference".into(),path:"chart".into(),message}],complete:false}
    };
    let canonical_payload = protocol::pack_rt::encode_wire_value(&inference.to_value());
    if canonical_payload.len() as u64 > request.budgets.allocation_bytes { return Err(Error::new("print.chart.inference.allocation", "output exceeds allocation budget")); }
    Ok(ArtifactInferenceExecution { retirement_progress: Default::default(), canonical_payload:Some(canonical_payload), diagnostics: Vec::new(), validity: if inference.complete{"valid"}else{"invalid"}.into(), quality: "exact".into(), complete: inference.complete, actual_cache_mode: WireArtifactInferenceCacheMode::Cold })
}
