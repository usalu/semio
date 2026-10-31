semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0) }
    fn controlled_retirement_supported() -> bool { <String as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
#[path="🎯️scope/🦀️.rs"]
mod scope;
impl semio_framework_value::retirement::RetireOwned for crate::FaultScope{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{let Self{plugin_id,app_id,instance_id,module,body_key}=self;semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(plugin_id),semio_framework_value::retirement::deferred(app_id),semio_framework_value::retirement::deferred(instance_id),semio_framework_value::retirement::deferred(module),semio_framework_value::retirement::deferred(body_key)])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.plugin_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.app_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.instance_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.module),semio_framework_value::retirement::deferred_birth_bytes_for(&self.body_key)])}
 fn controlled_retirement_supported()->bool{true}
 fn boxed_original_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError>{Ok(Some(scope::demands()))}
 fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{scope::admit(original,grant)}
}
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code, retained_progress });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
#[path="🎟️params/🦀️.rs"]
mod params;
impl semio_framework_value::retirement::RetireOwned for crate::FaultParams {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)}
    fn controlled_retirement_supported()->bool{<Vec<(String,String)> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
    fn boxed_original_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError>{Ok(Some(params::demands()))}
    fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{params::admit(original,grant)}

}
semio_framework_value::artifact_retire_struct!(crate::Fault { retained_progress, origin, code, severity, message, scope, span, source, domain, causes, params, retryable });

semio_framework_value::artifact_retire_struct!(crate::FaultSource { offset, byte, depth });
