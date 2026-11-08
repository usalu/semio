semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    type RetirementCursor = <String as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
    fn retirement_ready(&self) -> bool { semio_framework_value::retirement::RetireOwned::retirement_ready(&self.0) }
    fn retirement_seed(self) -> Self::RetirementCursor {
        semio_framework_value::retirement::RetireOwned::retirement_seed(self.0)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0) }
    fn controlled_retirement_supported() -> bool { <String as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
#[path="🎯️scope/🦀️.rs"]
mod scope;
impl semio_framework_value::retirement::RetireOwned for crate::FaultScope{
 type RetirementCursor=Box<dyn semio_framework_value::retirement::RetirementCursor>;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{plugin_id,app_id,instance_id,module,body_key}=self;semio_framework_value::retirement::sequence(vec![Box::new(semio_framework_value::retirement::deferred(plugin_id)),Box::new(semio_framework_value::retirement::deferred(app_id)),Box::new(semio_framework_value::retirement::deferred(instance_id)),Box::new(semio_framework_value::retirement::deferred(module)),Box::new(semio_framework_value::retirement::deferred(body_key))])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.plugin_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.app_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.instance_id),semio_framework_value::retirement::deferred_birth_bytes_for(&self.module),semio_framework_value::retirement::deferred_birth_bytes_for(&self.body_key)])}
 fn controlled_retirement_supported()->bool{true}
 fn boxed_original_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError>{Ok(Some(scope::demands()))}
 fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{scope::admit(original,effect.grant())}
}
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code, retained_progress });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
#[path="🎟️params/🦀️.rs"]
mod params;
impl semio_framework_value::retirement::RetireOwned for crate::FaultParams {
    type RetirementCursor = <Vec<(String,String)> as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
    fn retirement_ready(&self) -> bool { semio_framework_value::retirement::RetireOwned::retirement_ready(&self.0) }
    fn retirement_seed(self) -> Self::RetirementCursor {
        semio_framework_value::retirement::RetireOwned::retirement_seed(self.0)
    }
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)}
    fn controlled_retirement_supported()->bool{<Vec<(String,String)> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
    fn boxed_original_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError>{Ok(Some(params::demands()))}
    fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{params::admit(original,effect.grant())}

}
semio_framework_value::artifact_retire_struct!(crate::Fault { retained_progress, origin, code, severity, message, scope, span, source, domain, causes, params, retryable });

semio_framework_value::artifact_retire_struct!(crate::FaultSource { offset, byte, depth });
