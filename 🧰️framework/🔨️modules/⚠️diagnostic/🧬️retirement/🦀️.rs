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
 type RetirementCursor=semio_framework_value::retirement::record::RetirementRecord<Self>;
 fn retirement_ready(&self)->bool{true && semio_framework_value::retirement::RetireOwned::retirement_ready(&self.plugin_id) && semio_framework_value::retirement::RetireOwned::retirement_ready(&self.app_id) && semio_framework_value::retirement::RetireOwned::retirement_ready(&self.instance_id) && semio_framework_value::retirement::RetireOwned::retirement_ready(&self.module) && semio_framework_value::retirement::RetireOwned::retirement_ready(&self.body_key)}
 fn retirement_seed(self)->Self::RetirementCursor{let Self{plugin_id,app_id,instance_id,module,body_key}=self;let original=Self{plugin_id,app_id,instance_id,module,body_key};unsafe{semio_framework_value::retirement::record::RetirementRecord::new(original,5,|root,ordinal|{let mut index=0;if ordinal==index{return semio_framework_value::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).plugin_id))}index+=1;if ordinal==index{return semio_framework_value::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).app_id))}index+=1;if ordinal==index{return semio_framework_value::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).instance_id))}index+=1;if ordinal==index{return semio_framework_value::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).module))}index+=1;if ordinal==index{return semio_framework_value::retirement::record::field_birth_demand(&*::std::ptr::addr_of!((*root).body_key))}index+=1;let _=index;Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"record field ordinal exceeded its declaration"))},|root,ordinal,effect|{let mut index=0;if ordinal==index{return semio_framework_value::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).plugin_id),effect)}index+=1;if ordinal==index{return semio_framework_value::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).app_id),effect)}index+=1;if ordinal==index{return semio_framework_value::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).instance_id),effect)}index+=1;if ordinal==index{return semio_framework_value::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).module),effect)}index+=1;if ordinal==index{return semio_framework_value::retirement::record::admit_field(::std::ptr::addr_of_mut!((*root).body_key),effect)}index+=1;let _=index;semio_framework_value::retirement::RetirementStep::Failure(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"record field ordinal exceeded its declaration").into(),0)},|root,ordinal|{let mut index=0;if index>=ordinal{::std::ptr::drop_in_place(::std::ptr::addr_of_mut!((*root).plugin_id))}index+=1;if index>=ordinal{::std::ptr::drop_in_place(::std::ptr::addr_of_mut!((*root).app_id))}index+=1;if index>=ordinal{::std::ptr::drop_in_place(::std::ptr::addr_of_mut!((*root).instance_id))}index+=1;if index>=ordinal{::std::ptr::drop_in_place(::std::ptr::addr_of_mut!((*root).module))}index+=1;if index>=ordinal{::std::ptr::drop_in_place(::std::ptr::addr_of_mut!((*root).body_key))}index+=1;let _=index;})}}
 fn controlled_retirement_supported()->bool{semio_framework_value::retirement::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&semio_framework_value::retirement::RetirementSupport<'_>)->bool{let original=::std::mem::MaybeUninit::<Self>::uninit();let root=original.as_ptr();true && semio_framework_value::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).plugin_id)},path)&& semio_framework_value::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).app_id)},path)&& semio_framework_value::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).instance_id)},path)&& semio_framework_value::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).module)},path)&& semio_framework_value::retirement::record::field_supported(unsafe{::std::ptr::addr_of!((*root).body_key)},path)}
 fn boxed_original_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError>{Ok(Some(scope::demands()))}
 fn boxed_original_work_demand(&self)->Result<Option<u64>,semio_framework_value::ValueError>{Ok(Some(1))}
    fn native_boxed_original_admission()->Option<semio_framework_value::retirement::NativeBoxedRetirementAdmission<Self>>{Some(unsafe{semio_framework_value::retirement::NativeBoxedRetirementAdmission::new(<Self as semio_framework_value::retirement::RetireOwned>::boxed_original_retirement)})}
 fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{scope::admit(original,effect)}
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
    fn boxed_original_work_demand(&self)->Result<Option<u64>,semio_framework_value::ValueError>{Ok(Some(1))}
    fn native_boxed_original_admission()->Option<semio_framework_value::retirement::NativeBoxedRetirementAdmission<Self>>{Some(unsafe{semio_framework_value::retirement::NativeBoxedRetirementAdmission::new(<Self as semio_framework_value::retirement::RetireOwned>::boxed_original_retirement)})}
    fn boxed_original_retirement(original:&mut Option<semio_framework_value::retirement::original_box::BoxOriginal<Self>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn semio_framework_value::retirement::RetirementCursor>,semio_framework_value::RetainedCloneProgress)>>{params::admit(original,effect)}

}
semio_framework_value::artifact_retire_struct!(crate::Fault { retained_progress, origin, code, severity, message, scope, span, source, domain, causes, params, retryable });

semio_framework_value::artifact_retire_struct!(crate::FaultSource { offset, byte, depth });
