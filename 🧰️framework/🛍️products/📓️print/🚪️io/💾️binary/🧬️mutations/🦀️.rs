//! 💾️ Controlled binary representation of authored chart mutations.
use crate::ChangeChartValue;
impl protocol::OpBinary for ChangeChartValue{
 fn encode_op(&self)->Result<Vec<u8>,protocol::ProtocolError>{let body=pack::record::encode_record_body(&Self::__dsl_spec(),&self.__dsl_to_record(),&Default::default())?;let mut bytes=vec![1,1];bytes.extend(body);Ok(bytes)}
 fn decode_op(bytes:&[u8])->Result<Self,protocol::ProtocolError>{if !bytes.starts_with(&[1,1]){return Err(protocol::ProtocolError::from(protocol::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Chart operation format/tag mismatch"))))}let record=pack::record::decode_record_body_exact(&bytes[2..],&Self::__dsl_spec(),&Default::default())?;Self::__dsl_from_record(&record).map_err(|error|protocol::ProtocolError::from(protocol::PackError::from(error)))}
}
impl ChangeChartValue{
 /// ✍️ Streams the original authored chart fields through the caller's complete operation policy.
 pub fn encode_op_into(&self,options:&protocol::codec::PackEncodeOptions,output:&mut dyn protocol::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),protocol::ProtocolError>{
  protocol::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{
  let mut limited=protocol::os_spr::operation_bytes::OperationByteLimitedOutput::new(output,options.limits.max_file_len);
  let output:&mut dyn protocol::os_spr::operation_bytes::OperationByteOutput=&mut limited;
  let spec=Self::__dsl_spec_producer().encode(control).map_err(protocol::PackRefusal::from)?;
  output.write_bytes(&[1,1],control)?;
  pack::record::encode_projected_record_body_into(&spec,self,options,output,control)?;
  Ok(())
  })
 }
 /// 🫳️ Decodes the same immutable source and verifies canonical bytes under caller controls.
 pub fn decode_op_span(source:protocol::ByteSpan<'_>,options:&protocol::codec::PackDecodeOptions,canonical_options:&protocol::codec::PackEncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Self,protocol::ProtocolError>{
  decoding.checkpoint().map_err(protocol::PackRefusal::from)?;
  if source.len()as u64>options.limits.max_file_len{return Err(protocol::PackRefusal::LimitExceeded{kind:semio_framework_value::ValueRefusalKind::OwnershipLimit,limit:"Chart operation source byte length"}.into());}
  if source.get(0)!=Some(&1)||source.get(1)!=Some(&1){return Err(protocol::PackRefusal::RetainedMalformed{kind:semio_framework_value::ValueRefusalKind::InvalidValue,what:"Chart operation format/tag",offset:0,detail:"expected exact [1,1] header"}.into());}
  let spec=Self::__dsl_spec_producer().decode(decoding).map_err(protocol::PackRefusal::from)?;
  let record=pack::record::decode_record_body_span_exact_controlled(source.slice(2,source.len()-2)?,&spec,options,decoding)?;
  let operation=Self::__dsl_from_record_controlled(&record,decoding).map_err(protocol::PackRefusal::from)?;
  let mut comparison=protocol::os_spr::operation_bytes::OperationByteComparison::new(source);
  operation.encode_op_into(canonical_options,&mut comparison,encoding)?;
  comparison.finish()?;
  Ok(operation)
 }
}
