use super::*;

#[derive(Clone, ToValue, FromValue)]
struct RefusingChildOperation {
    inner: TestMutation,
    refuse: bool,
}

impl protocol::Mutation<TestSnapshot> for RefusingChildOperation {
    type Diff = <TestMutation as protocol::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as protocol::Mutation<TestSnapshot>>::DESCRIPTORS;
    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor { protocol::Mutation::descriptor(&self.inner) }
    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> { protocol::Mutation::diff(&self.inner, base) }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> { Ok(protocol::Mutation::inverse(&self.inner, base)?.into_iter().map(|inner| Self { inner, refuse: self.refuse }).collect()) }
    fn retire_cold(self) { protocol::Mutation::<TestSnapshot>::retire_cold(self.inner); }
}

impl protocol::SemanticMutation<TestSnapshot> for RefusingChildOperation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] { <TestMutation as protocol::SemanticMutation<TestSnapshot>>::kinds() }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor { protocol::SemanticMutation::semantics(&self.inner) }
    fn label(&self) -> LocalizedLabel { protocol::SemanticMutation::label(&self.inner) }
    fn target(&self) -> Vec<String> { protocol::SemanticMutation::target(&self.inner) }
}

impl ::protocol::OpBinary for RefusingChildOperation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        if self.refuse { return Err(::protocol::ProtocolError::Malformed { what: "owned-child-operation", offset: 9, detail: "codec refusal \0引用😀".to_string() }); }
        ::protocol::OpBinary::encode_op(&self.inner)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> { Ok(Self { inner: <TestMutation as ::protocol::OpBinary>::decode_op(bytes)?, refuse: false }) }
}

#[test]
fn child_emission_encoder_refusal_preserves_prior_complete_owned_operations() {
    let fixture: Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧫️fixtures/🔣️.json")).expect("closed emission contract");
    let operation = RefusingChildOperation { inner: TestMutation::SetCount(SetCount { value: fixture["validValue"].as_i64().expect("i32") as i32 }), refuse: false };
    let mut emit = ChildEmit::open(fixture["slot"].as_str().expect("literal slot"), fixture["childId"].as_str().expect("literal child"), 2);
    let _ = emit.push::<TestSnapshot, _>(&operation);
    assert_eq!(emit.ops, vec![::protocol::OpBinary::encode_op(&operation).expect("actual complete operation")]);
    let accepted_ops = emit.ops.clone();
    let accepted_labels = emit.labels.clone();
    let accepted_schema = emit.op_schema.clone();
    let refusing = RefusingChildOperation { inner: operation.inner.clone(), refuse: true };
    let _ = emit.push::<TestSnapshot, _>(&refusing);
    assert_eq!(emit.ops, accepted_ops, "a codec refusal cannot publish an empty substitute operation");
    assert_eq!(emit.labels, accepted_labels, "the refused operation has no accepted label");
    assert_eq!(emit.op_schema, accepted_schema);
    assert_eq!(emit.ops.len(), fixture["expected"]["acceptedOperations"].as_u64().expect("count") as usize);
    assert_eq!(emit.labels.len(), fixture["expected"]["acceptedLabels"].as_u64().expect("count") as usize);
    <RefusingChildOperation as protocol::Mutation<TestSnapshot>>::retire_cold(operation);
    <RefusingChildOperation as protocol::Mutation<TestSnapshot>>::retire_cold(refusing);
}
