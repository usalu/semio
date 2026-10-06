//! 📐️ Complete AP214 native records preserve owned fields independently of external exchange syntax.
#[path="🪆️binding/🦀️.rs"]
mod binding;
#[path="🔎️census/🦀️.rs"]
mod census;
#[cfg(test)]
#[path="🧪️tests/💰️backing/🦀️.rs"]
mod backing_tests;
use crate::standards::v_ap214::subsets::base::schema::snapshot::component::native::{StepSnapshot,StepHeader,StepFileDescription,StepFileName,StepFileSchema,StepEntity,StepComplexType,StepValue,STDIO_STEP_DOCUMENT_SCHEMA};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_os_kernel::{io_schema::IoPayload,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl}};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Node{kind:String,integer:Option<i64>,real:Option<f64>,text:Option<String>,reference:Option<u64>,name:Option<String>,children:Vec<usize>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Complex{name:String,arguments:Vec<usize>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Entity{id:u64,name:String,arguments:Vec<usize>,complex:Vec<Complex>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Frame{schema:String,description:Vec<String>,implementation_level:String,file_name:String,timestamp:String,author:Vec<String>,organization:Vec<String>,preprocessor_version:String,originating_system:String,authorization:String,file_schema:Vec<String>,entities:Vec<Entity>,values:Vec<Node>}
semio_framework_value::artifact_retire_struct!(Node{kind,integer,real,text,reference,name,children});
semio_framework_value::artifact_retire_struct!(Complex{name,arguments});
semio_framework_value::artifact_retire_struct!(Entity{id,name,arguments,complex});
semio_framework_value::artifact_retire_struct!(Frame{schema,description,implementation_level,file_name,timestamp,author,organization,preprocessor_version,originating_system,authorization,file_schema,entities,values});
semio_framework_value::artifact_retire_struct!(StepFileDescription{description,implementation_level});
semio_framework_value::artifact_retire_struct!(StepFileName{name,timestamp,author,organization,preprocessor_version,originating_system,authorization});
semio_framework_value::artifact_retire_struct!(StepFileSchema{schemas});
semio_framework_value::artifact_retire_struct!(StepHeader{file_description,file_name,file_schema});
semio_framework_value::artifact_retire_struct!(StepComplexType{name,args});
semio_framework_value::artifact_retire_struct!(StepEntity{id,name,args,complex});
semio_framework_value::artifact_retire_struct!(StepSnapshot{schema,header,entities});
impl RetireOwned for StepValue{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::String(value)|Self::Enum(value)=>value.retirement(),Self::Integer(value)=>value.retirement(),Self::Real(value)=>value.retirement(),Self::Reference(value)=>value.retirement(),Self::Aggregate(values)=>values.retirement(),Self::TypedValue{type_name,value}=>semio_framework_value::artifact_retirement_sequence![type_name,value],Self::Unset|Self::Derived=>().retirement()}}
}

pub(crate) fn close<T:RetireOwned>(value:T){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor.terminal_is_empty(){cursor.close_step(256,65536).expect("STEP native cold grant");}}
pub(crate) fn box_slot(values:Vec<StepValue>)->Result<Box<StepValue>,ValueError>{let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(values,close::<Vec<StepValue>>);if owner.as_mut().len()!=1||owner.as_mut().capacity()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"STEP typed box must reuse one paid slot"));}Ok(unsafe{Box::from_raw(Box::into_raw(owner.take().into_boxed_slice())as*mut StepValue)})}
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn positioned(error:ValueError)->TextError{TextError::from_value_error(error,TextSpan::at(1,1))}
fn strings(values:&[String],control:&mut NativeEncodeControl<'_>)->Result<Vec<String>,ValueError>{control.scoped_stage(|control|{let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?,close::<Vec<String>>);control.begin_stage(values.len())?;for value in values{held.as_mut().push(control.copy_text(value)?);control.step()?;}Ok(held.take())})}
fn project_values(values:&[StepValue],output:&mut Vec<Node>,control:&mut NativeEncodeControl<'_>)->Result<Vec<usize>,ValueError>{control.scoped_stage(|control|{
 let nodes=census::group(values,control)?;let mut roots=control.allocate_vec(values.len())?;let mut pending=control.allocate_vec::<(&StepValue,Option<usize>)>(nodes)?;control.begin_stage(nodes.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native projection workload overflow"))?)?;for value in values.iter().rev(){pending.push((value,None));control.step()?;}
 while let Some((value,parent))=pending.pop(){control.step()?;let index=output.len();let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Node{kind:String::new(),integer:None,real:None,text:None,reference:None,name:None,children:Vec::new()},close::<Node>);let node=owner.as_mut();let(kind,children)=match value{
 StepValue::Unset=>("unset",&[][..]),StepValue::Derived=>("derived",&[][..]),StepValue::Integer(value)=>{node.integer=Some(*value);("integer",&[][..])},StepValue::Real(value)=>{node.real=Some(*value);("real",&[][..])},StepValue::String(value)=>{node.text=Some(control.copy_text(value)?);("string",&[][..])},StepValue::Enum(value)=>{node.text=Some(control.copy_text(value)?);("enum",&[][..])},StepValue::Reference(value)=>{node.reference=Some(*value);("reference",&[][..])},StepValue::Aggregate(values)=>("aggregate",values.as_slice()),StepValue::TypedValue{type_name,value}=>{node.name=Some(control.copy_text(type_name)?);("typed",std::slice::from_ref(value.as_ref()))}};
 node.kind=control.copy_text(kind)?;node.children=control.allocate_vec(children.len())?;if output.len()==output.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native value census differs from projection"));}output.push(owner.take());match parent{Some(parent)=>output[parent].children.push(index),None=>roots.push(index)};for child in children.iter().rev(){pending.push((child,Some(index)));control.step()?;}
 }Ok(roots)
})}
fn project(value:&StepSnapshot,control:&mut NativeEncodeControl<'_>,maximum:usize)->Result<Frame,ValueError>{let count=census::measure(value,control,maximum)?;
 let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Frame{schema:String::new(),description:Vec::new(),implementation_level:String::new(),file_name:String::new(),timestamp:String::new(),author:Vec::new(),organization:Vec::new(),preprocessor_version:String::new(),originating_system:String::new(),authorization:String::new(),file_schema:Vec::new(),entities:Vec::new(),values:Vec::new()},close::<Frame>);let frame=owner.as_mut();frame.values=control.allocate_vec(count.nodes)?;let header=&value.header;let name=&header.file_name;
 frame.schema=control.copy_text(&value.schema)?;frame.description=strings(&header.file_description.description,control)?;frame.implementation_level=control.copy_text(&header.file_description.implementation_level)?;frame.file_name=control.copy_text(&name.name)?;frame.timestamp=control.copy_text(&name.timestamp)?;frame.author=strings(&name.author,control)?;frame.organization=strings(&name.organization,control)?;frame.preprocessor_version=control.copy_text(&name.preprocessor_version)?;frame.originating_system=control.copy_text(&name.originating_system)?;frame.authorization=control.copy_text(&name.authorization)?;frame.file_schema=strings(&header.file_schema.schemas,control)?;frame.entities=control.allocate_vec(value.entities.len())?;
 control.scoped_stage(|control|{control.begin_stage(value.entities.len())?;for value in &value.entities{let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Entity{id:value.id,name:control.copy_text(&value.name)?,arguments:Vec::new(),complex:Vec::new()},close::<Entity>);held.as_mut().complex=control.allocate_vec(value.complex.len())?;held.as_mut().arguments=project_values(&value.args,&mut frame.values,control)?;for value in &value.complex{let mut complex=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Complex{name:control.copy_text(&value.name)?,arguments:Vec::new()},close::<Complex>);complex.as_mut().arguments=project_values(&value.args,&mut frame.values,control)?;held.as_mut().complex.push(complex.take());}frame.entities.push(held.take());control.step()?;}Ok::<_,ValueError>(())})?;Ok(owner.take())
}
struct Values(Vec<Option<StepValue>>);impl Drop for Values{fn drop(&mut self){close(std::mem::take(&mut self.0));}}
fn take(indices:Vec<usize>,values:&mut Values,control:&mut NativeDecodeControl<'_>)->Result<Vec<StepValue>,ValueError>{let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(indices.len())?,close::<Vec<StepValue>>);control.scoped_stage(|control|{control.begin_stage(indices.len())?;for index in indices{held.as_mut().push(values.0.get_mut(index).and_then(Option::take).ok_or_else(||invalid("STEP native value has missing or multiple ownership"))?);control.step()?;}Ok::<_,ValueError>(())})?;Ok(held.take())}
fn reconstruct(frame:Frame,control:&mut NativeDecodeControl<'_>)->Result<StepSnapshot,ValueError>{
 let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(frame,close::<Frame>);let count=owner.as_mut().values.len();let mut values=Values(control.allocate_vec(count)?);values.0.resize_with(count,||None);
 control.scoped_stage(|control|{control.begin_stage(count)?;for index in (0..count).rev(){let node=&mut owner.as_mut().values[index];let payloads=usize::from(node.integer.is_some())+usize::from(node.real.is_some())+usize::from(node.text.is_some())+usize::from(node.reference.is_some())+usize::from(node.name.is_some());let valid=match node.kind.as_str(){"unset"|"derived"=>payloads==0&&node.children.is_empty(),"integer"=>node.integer.is_some()&&payloads==1&&node.children.is_empty(),"real"=>node.real.is_some()&&payloads==1&&node.children.is_empty(),"string"|"enum"=>node.text.is_some()&&payloads==1&&node.children.is_empty(),"reference"=>node.reference.is_some()&&payloads==1&&node.children.is_empty(),"aggregate"=>payloads==0,"typed"=>node.name.is_some()&&payloads==1&&node.children.len()==1,_=>false};if !valid||node.children.iter().any(|child|*child<=index){return Err(invalid("STEP native node shape or forest differs"));}
 let mut kind=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(std::mem::take(&mut node.kind),close::<String>);let children=std::mem::take(&mut node.children);let value=match kind.as_mut().as_str(){"unset"=>StepValue::Unset,"derived"=>StepValue::Derived,"integer"=>StepValue::Integer(node.integer.unwrap()),"real"=>StepValue::Real(node.real.unwrap()),"string"=>StepValue::String(node.text.take().unwrap()),"enum"=>StepValue::Enum(node.text.take().unwrap()),"reference"=>StepValue::Reference(node.reference.unwrap()),"aggregate"=>StepValue::Aggregate(take(children,&mut values,control)?),"typed"=>{let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(node.name.take().unwrap(),close::<String>);let mut child=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(take(children,&mut values,control)?,close::<Vec<StepValue>>);StepValue::TypedValue{type_name:name.take(),value:box_slot(child.take())?}},_=>unreachable!()};close(kind.take());values.0[index]=Some(value);control.step()?;}Ok::<_,ValueError>(())})?;
 let mut result=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(StepSnapshot{schema:String::new(),header:StepHeader{file_description:StepFileDescription{description:Vec::new(),implementation_level:String::new()},file_name:StepFileName{name:String::new(),timestamp:String::new(),author:Vec::new(),organization:Vec::new(),preprocessor_version:String::new(),originating_system:String::new(),authorization:String::new()},file_schema:StepFileSchema{schemas:Vec::new()}},entities:Vec::new()},close::<StepSnapshot>);
 let frame=owner.as_mut();let out=result.as_mut();out.entities=control.allocate_vec(frame.entities.len())?;control.scoped_stage(|control|{control.begin_stage(frame.entities.len())?;for value in &mut frame.entities{let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(StepEntity{id:value.id,name:std::mem::take(&mut value.name),args:Vec::new(),complex:Vec::new()},close::<StepEntity>);held.as_mut().args=take(std::mem::take(&mut value.arguments),&mut values,control)?;held.as_mut().complex=control.allocate_vec(value.complex.len())?;for value in &mut value.complex{let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(std::mem::take(&mut value.name),close::<String>);let args=take(std::mem::take(&mut value.arguments),&mut values,control)?;held.as_mut().complex.push(StepComplexType{name:name.take(),args});}out.entities.push(held.take());control.step()?;}Ok::<_,ValueError>(())})?;if values.0.iter().any(Option::is_some){return Err(invalid("STEP native unowned value"));}
 out.schema=std::mem::take(&mut frame.schema);out.header.file_description=StepFileDescription{description:std::mem::take(&mut frame.description),implementation_level:std::mem::take(&mut frame.implementation_level)};out.header.file_name=StepFileName{name:std::mem::take(&mut frame.file_name),timestamp:std::mem::take(&mut frame.timestamp),author:std::mem::take(&mut frame.author),organization:std::mem::take(&mut frame.organization),preprocessor_version:std::mem::take(&mut frame.preprocessor_version),originating_system:std::mem::take(&mut frame.originating_system),authorization:std::mem::take(&mut frame.authorization)};out.header.file_schema=StepFileSchema{schemas:std::mem::take(&mut frame.file_schema)};Ok(result.take())
}
pub(crate) fn encode(value:&StepSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{value.admit_sqlite_values(control,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative)?;let maximum=control.limits().max_rows;store::encode_sqlite_snapshot_record_native(encoding,STDIO_STEP_DOCUMENT_SCHEMA,Frame::__dsl_spec_producer(),|native|{let mut frame=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(project(value,native,maximum)?,close::<Frame>);frame.as_mut().__dsl_to_record_controlled(native)},control)}
pub(crate) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<StepSnapshot,ValueError>{let limits=control.limits();let maximum=limits.max_rows;store::decode_sqlite_snapshot_record_native(payload,STDIO_STEP_DOCUMENT_SCHEMA,Frame::__dsl_spec_producer(),|record,native|{StepSnapshot::admit_sqlite_record(record,limits,native)?;let frame=binding::frame(record,native,maximum)?;reconstruct(frame,native)},control)}

pub(crate) fn encode_pack(value:&StepSnapshot,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeEncodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().encode(&mut native).map_err(store::PackError::from)?;
 let mut frame=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(project(value,&mut native,limits.max_rows).map_err(store::PackError::from)?,close::<Frame>);
 let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(frame.as_mut().__dsl_to_record_controlled(&mut native).map_err(store::PackError::from)?);
 let body=store::pack_rt::encode_document_controlled(&spec,record.as_record(),options,&mut native)?;
 store::semio_format::wrap_binary_controlled(STDIO_STEP_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&body,&mut native).map_err(store::PackError::from)
}

pub(crate) fn decode_pack(bytes:&[u8],options:&store::PackDecodeOptions)->Result<StepSnapshot,store::PackError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeDecodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().decode(&mut native).map_err(store::PackError::from)?;
 let body=store::semio_format::unwrap_binary_controlled(bytes,STDIO_STEP_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&mut native).map_err(|error|store::PackError::from(error.into_value_error()))?;
 let(record,_)=store::pack_rt::decode_document_controlled(body,&spec,options,&mut native)?;
 let frame=binding::frame(&record,&mut native,limits.max_rows).map_err(store::PackError::from)?;
 reconstruct(frame,&mut native).map_err(store::PackError::from)
}

pub(crate) fn parse_text(text:&str)->Result<StepSnapshot,TextError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeDecodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().decode(&mut native).map_err(positioned)?;
 let body=store::semio_format::split_text_preamble_controlled(text,STDIO_STEP_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,&mut native).map_err(|error|positioned(error.into_value_error()))?;
 let record=semio_framework_dsl_record::parse_exact_controlled(body,&spec,&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:limits.max_file_bytes,..Default::default()},mode:semio_framework_dsl_record::SourceMode::Document},&mut native)?;
 reconstruct(binding::frame(&record,&mut native,limits.max_rows).map_err(positioned)?,&mut native).map_err(positioned)
}

pub(crate) fn print_text(value:&StepSnapshot)->String{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;
 match encode(value,SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut progress,limits)).expect("STEP complete native text within default limits"){IoPayload::Text(text)=>text,IoPayload::Binary(_)=>unreachable!()}
}
