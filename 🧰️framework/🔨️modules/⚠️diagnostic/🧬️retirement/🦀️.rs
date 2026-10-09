semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0) }
    fn controlled_retirement_supported() -> bool { <String as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported() }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
impl semio_framework_value::retirement::RetireOwned for crate::TextError {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { kind: _, message, span, expected } = self;
        semio_framework_value::artifact_retirement_sequence![message, span, expected]
    }
}
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
semio_framework_value::artifact_retire_struct!(crate::FaultScope { plugin_id, app_id, instance_id, module, body_key });
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
impl semio_framework_value::retirement::RetireOwned for crate::FaultParams {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
    fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.0)}
    fn controlled_retirement_supported()->bool{<Vec<(String,String)> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
semio_framework_value::artifact_retire_struct!(crate::Fault { origin, code, severity, message, scope, span, causes, params, retryable });
