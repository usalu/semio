//! 🧱️ Complete IFC2x3 native records preserve actual Part21 ownership and literal decimal fields.
#[path="🪆️binding/🦀️.rs"]
mod binding;
#[path="🔎️census/🦀️.rs"]
mod census;
#[cfg(test)]
#[path="🧪️tests/💰️backing/🦀️.rs"]
mod backing_tests;
use super::{Ifc2x3EdmPreamble, Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_contract::part21::{Part21Decimal, Part21Document, Part21Header, Part21Instance, Part21Value};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_os_kernel::{sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl}, io_schema::IoPayload};

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Node { kind: String, integer: Option<i64>, reference: Option<u64>, text: Option<String>, name: Option<String>, negative: Option<bool>, coefficient: Option<String>, scale: Option<u32>, exponent: Option<i32>, children: Vec<usize> }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Entity { name: String, arguments: Vec<usize> }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Instance { id: u64, entities: Vec<Entity> }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Edm { producer:String,module:String,creation_date:String,host:String,database:String,database_version:String,database_creation_date:String,schema:String,model:String,model_creation_date:String,header_model:String,header_model_creation_date:String,user:String,group:String,license:String,options:String }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Frame { schema: String, file_description: Vec<usize>, file_name: Vec<usize>, file_schema: Vec<usize>, instances: Vec<Instance>, values: Vec<Node>, edm: Option<Edm> }
semio_framework_value::artifact_retire_struct!(Node { kind,integer,reference,text,name,negative,coefficient,scale,exponent,children });
semio_framework_value::artifact_retire_struct!(Entity { name,arguments });
semio_framework_value::artifact_retire_struct!(Instance { id,entities });
semio_framework_value::artifact_retire_struct!(Edm { producer,module,creation_date,host,database,database_version,database_creation_date,schema,model,model_creation_date,header_model,header_model_creation_date,user,group,license,options });
semio_framework_value::artifact_retire_struct!(Frame { schema,file_description,file_name,file_schema,instances,values,edm });
semio_framework_value::artifact_retire_struct!(Ifc2x3EdmPreamble { producer,module,creation_date,host,database,database_version,database_creation_date,schema,model,model_creation_date,header_model,header_model_creation_date,user,group,license,options });
semio_framework_value::artifact_retire_struct!(Ifc2x3Snapshot { schema,document,edm_preamble });
pub(super) fn close<T: RetireOwned>(value:T) { let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor.terminal_is_empty(){cursor.close_step(256,65536).expect("IFC2x3 native cold grant");} }
fn positioned(error:ValueError)->TextError { TextError::from_value_error(error,TextSpan::at(1,1)) }
fn invalid(message:&'static str)->ValueError { ValueError::new(ValueRefusalKind::InvalidValue,message) }

fn project_values(values:&[Part21Value],output:&mut Vec<Node>,control:&mut NativeEncodeControl<'_>)->Result<Vec<usize>,ValueError> {
    control.scoped_stage(|control| {
        let nodes=census::group(values,control)?;let mut roots=control.allocate_vec(values.len())?;
        let mut pending=control.allocate_vec::<(&Part21Value,Option<usize>)>(nodes)?;
        control.begin_stage(nodes.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native projection workload overflow"))?)?;for value in values.iter().rev(){pending.push((value,None));control.step()?;}
        while let Some((value,parent))=pending.pop(){
            control.step()?;
            let index=output.len();
            let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Node{kind:String::new(),integer:None,reference:None,text:None,name:None,negative:None,coefficient:None,scale:None,exponent:None,children:Vec::new()},close::<Node>);
            let node=owner.as_mut();
            let (kind,children)=match value {
                Part21Value::Unset=>("unset",&[][..]),Part21Value::Derived=>("derived",&[][..]),
                Part21Value::Int(value)=>{node.integer=Some(*value);("integer",&[][..])},
                Part21Value::Ref(value)=>{node.reference=Some(*value);("reference",&[][..])},
                Part21Value::Str(value)=>{node.text=Some(control.copy_text(value)?);("string",&[][..])},
                Part21Value::Enum(value)=>{node.text=Some(control.copy_text(value)?);("enum",&[][..])},
                Part21Value::Real(value)=>{node.negative=Some(value.negative);node.coefficient=Some(control.copy_text(&value.coefficient)?);node.scale=Some(value.scale);node.exponent=value.exponent;("decimal",&[][..])},
                Part21Value::List(values)=>("list",values.as_slice()),
                Part21Value::Typed{name,items}=>{node.name=Some(control.copy_text(name)?);("typed",items.as_slice())},
            };
            node.kind=control.copy_text(kind)?;
            node.children=control.allocate_vec(children.len())?;
            if output.len()==output.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native value census differs from projection"));}
            output.push(owner.take());
            match parent {Some(parent)=>output[parent].children.push(index),None=>roots.push(index)};
            for child in children.iter().rev(){pending.push((child,Some(index)));control.step()?;}
        }
        Ok(roots)
    })
}
fn project(value:&Ifc2x3Snapshot,control:&mut NativeEncodeControl<'_>,maximum:usize)->Result<Frame,ValueError> {let count=census::measure(value,control,maximum)?;
    let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Frame{schema:control.copy_text(&value.schema)?,file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new(),instances:Vec::new(),values:Vec::new(),edm:None},close::<Frame>);
    let frame=owner.as_mut();frame.values=control.allocate_vec(count.nodes)?;
    frame.file_description=project_values(&value.document.header.file_description,&mut frame.values,control)?;
    frame.file_name=project_values(&value.document.header.file_name,&mut frame.values,control)?;
    frame.file_schema=project_values(&value.document.header.file_schema,&mut frame.values,control)?;
    frame.instances=control.allocate_vec(value.document.instances.len())?;
    control.scoped_stage(|control|{control.begin_stage(value.document.instances.len())?;for instance in &value.document.instances{
        let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Instance{id:instance.id,entities:control.allocate_vec(instance.entities.len())?},close::<Instance>);
        for (name,arguments) in &instance.entities{
            let mut entity=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Entity{name:control.copy_text(name)?,arguments:Vec::new()},close::<Entity>);
            entity.as_mut().arguments=project_values(arguments,&mut frame.values,control)?;
            held.as_mut().entities.push(entity.take());
        }
        frame.instances.push(held.take());control.step()?;
    }Ok::<_,ValueError>(())})?;
    if let Some(value)=&value.edm_preamble{
        let mut edm=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Edm{producer:String::new(),module:String::new(),creation_date:String::new(),host:String::new(),database:String::new(),database_version:String::new(),database_creation_date:String::new(),schema:String::new(),model:String::new(),model_creation_date:String::new(),header_model:String::new(),header_model_creation_date:String::new(),user:String::new(),group:String::new(),license:String::new(),options:String::new()},close::<Edm>);
        let out=edm.as_mut();out.producer=control.copy_text(&value.producer)?;out.module=control.copy_text(&value.module)?;out.creation_date=control.copy_text(&value.creation_date)?;out.host=control.copy_text(&value.host)?;out.database=control.copy_text(&value.database)?;out.database_version=control.copy_text(&value.database_version)?;out.database_creation_date=control.copy_text(&value.database_creation_date)?;out.schema=control.copy_text(&value.schema)?;out.model=control.copy_text(&value.model)?;out.model_creation_date=control.copy_text(&value.model_creation_date)?;out.header_model=control.copy_text(&value.header_model)?;out.header_model_creation_date=control.copy_text(&value.header_model_creation_date)?;out.user=control.copy_text(&value.user)?;out.group=control.copy_text(&value.group)?;out.license=control.copy_text(&value.license)?;out.options=control.copy_text(&value.options)?;frame.edm=Some(edm.take());
    }
    Ok(owner.take())
}
struct Values(Vec<Option<Part21Value>>);
impl Drop for Values { fn drop(&mut self){close(std::mem::take(&mut self.0));} }
fn take_values(indices:Vec<usize>,values:&mut Values,control:&mut NativeDecodeControl<'_>)->Result<Vec<Part21Value>,ValueError>{
    let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(indices.len())?,close::<Vec<Part21Value>>);
    control.scoped_stage(|control|{control.begin_stage(indices.len())?;for index in indices {held.as_mut().push(values.0.get_mut(index).and_then(Option::take).ok_or_else(||invalid("IFC2x3 native value has missing or multiple ownership"))?);control.step()?;}Ok::<_,ValueError>(())})?;Ok(held.take())
}
fn reconstruct(frame:Frame,control:&mut NativeDecodeControl<'_>)->Result<Ifc2x3Snapshot,ValueError>{
    let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(frame,close::<Frame>);
    let count=owner.as_mut().values.len();let mut values=Values(control.allocate_vec(count)?);values.0.resize_with(count,||None);
    control.scoped_stage(|control|{control.begin_stage(count)?;for index in (0..count).rev(){
        let node=&mut owner.as_mut().values[index];
        let valid=match node.kind.as_str(){"unset"|"derived"=>node.integer.is_none()&&node.reference.is_none()&&node.text.is_none()&&node.name.is_none()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none()&&node.children.is_empty(),"integer"=>node.integer.is_some()&&node.reference.is_none()&&node.text.is_none()&&node.name.is_none()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none()&&node.children.is_empty(),"reference"=>node.integer.is_none()&&node.reference.is_some()&&node.text.is_none()&&node.name.is_none()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none()&&node.children.is_empty(),"string"|"enum"=>node.integer.is_none()&&node.reference.is_none()&&node.text.is_some()&&node.name.is_none()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none()&&node.children.is_empty(),"decimal"=>node.integer.is_none()&&node.reference.is_none()&&node.text.is_none()&&node.name.is_none()&&node.negative.is_some()&&node.coefficient.is_some()&&node.scale.is_some()&&node.children.is_empty(),"list"=>node.integer.is_none()&&node.reference.is_none()&&node.text.is_none()&&node.name.is_none()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none(),"typed"=>node.integer.is_none()&&node.reference.is_none()&&node.text.is_none()&&node.name.is_some()&&node.negative.is_none()&&node.coefficient.is_none()&&node.scale.is_none()&&node.exponent.is_none(),_=>false};
        if !valid||node.children.iter().any(|child|*child<=index){return Err(invalid("IFC2x3 native node shape or ordered forest differs"));}
        let mut kind=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(std::mem::take(&mut node.kind),close::<String>);let text=node.text.take();let name=node.name.take();let coefficient=node.coefficient.take();let children=std::mem::take(&mut node.children);
        let value=match kind.as_mut().as_str(){"unset"=>Part21Value::Unset,"derived"=>Part21Value::Derived,"integer"=>Part21Value::Int(node.integer.unwrap()),"reference"=>Part21Value::Ref(node.reference.unwrap()),"string"=>Part21Value::Str(text.unwrap()),"enum"=>Part21Value::Enum(text.unwrap()),"decimal"=>Part21Value::Real(Part21Decimal{negative:node.negative.unwrap(),coefficient:coefficient.unwrap(),scale:node.scale.unwrap(),exponent:node.exponent}),"list"=>Part21Value::List(take_values(children,&mut values,control)?),"typed"=>{let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(name.unwrap(),close::<String>);let items=take_values(children,&mut values,control)?;Part21Value::Typed{name:name.take(),items}},_=>unreachable!()};close(kind.take());values.0[index]=Some(value);control.step()?;
    }Ok::<_,ValueError>(())})?;
    let mut result=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Ifc2x3Snapshot{schema:String::new(),document:Part21Document{header:Part21Header{file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new()},instances:Vec::new()},edm_preamble:None},close::<Ifc2x3Snapshot>);
    let native=owner.as_mut();let out=result.as_mut();out.document.header.file_description=take_values(std::mem::take(&mut native.file_description),&mut values,control)?;out.document.header.file_name=take_values(std::mem::take(&mut native.file_name),&mut values,control)?;out.document.header.file_schema=take_values(std::mem::take(&mut native.file_schema),&mut values,control)?;
    out.document.instances=control.allocate_vec(native.instances.len())?;
    control.scoped_stage(|control|{control.begin_stage(native.instances.len())?;for instance in &mut native.instances{let mut held=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Part21Instance{id:instance.id,entities:control.allocate_vec(instance.entities.len())?},close::<Part21Instance>);for entity in &mut instance.entities{let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(std::mem::take(&mut entity.name),close::<String>);let arguments=take_values(std::mem::take(&mut entity.arguments),&mut values,control)?;held.as_mut().entities.push((name.take(),arguments));}out.document.instances.push(held.take());control.step()?;}Ok::<_,ValueError>(())})?;
    if values.0.iter().any(Option::is_some){return Err(invalid("IFC2x3 native unowned value"));}
    if let Some(value)=native.edm.take(){out.edm_preamble=Some(Ifc2x3EdmPreamble{producer:value.producer,module:value.module,creation_date:value.creation_date,host:value.host,database:value.database,database_version:value.database_version,database_creation_date:value.database_creation_date,schema:value.schema,model:value.model,model_creation_date:value.model_creation_date,header_model:value.header_model,header_model_creation_date:value.header_model_creation_date,user:value.user,group:value.group,license:value.license,options:value.options});}
    out.schema=std::mem::take(&mut native.schema);Ok(result.take())
}
pub(super) fn encode(value:&Ifc2x3Snapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{
    let maximum=control.limits().max_rows;
    store::encode_sqlite_snapshot_record_native(encoding,STDIO_IFC2X3_DOCUMENT_SCHEMA,Frame::__dsl_spec_producer(),|native|{let mut frame=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(project(value,native,maximum)?,close::<Frame>);frame.as_mut().__dsl_to_record_controlled(native)},control)
}
pub(super) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Ifc2x3Snapshot,ValueError>{
    let maximum=control.limits().max_rows;
    store::decode_sqlite_snapshot_record_native(payload,STDIO_IFC2X3_DOCUMENT_SCHEMA,Frame::__dsl_spec_producer(),|record,native|{let mut frame=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(binding::frame(record,native,maximum)?,close::<Frame>);reconstruct(frame.take(),native)},control)
}

pub(super) fn encode_pack(value:&Ifc2x3Snapshot,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeEncodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().encode(&mut native).map_err(store::PackError::from)?;
 let mut frame=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(project(value,&mut native,limits.max_rows).map_err(store::PackError::from)?,close::<Frame>);
 let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(frame.as_mut().__dsl_to_record_controlled(&mut native).map_err(store::PackError::from)?);
 let body=store::pack_rt::encode_document_controlled(&spec,record.as_record(),options,&mut native)?;
 store::semio_format::wrap_binary_controlled(STDIO_IFC2X3_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&body,&mut native).map_err(store::PackError::from)
}

pub(super) fn decode_pack(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Ifc2x3Snapshot,store::PackError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeDecodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().decode(&mut native).map_err(store::PackError::from)?;
 let body=store::semio_format::unwrap_binary_controlled(bytes,STDIO_IFC2X3_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&mut native).map_err(|error|store::PackError::from(error.into_value_error()))?;
 let(record,_)=store::pack_rt::decode_document_controlled(body,&spec,options,&mut native)?;
 let frame=binding::frame(&record,&mut native,limits.max_rows).map_err(store::PackError::from)?;
 reconstruct(frame,&mut native).map_err(store::PackError::from)
}

pub(super) fn parse_text(text:&str)->Result<Ifc2x3Snapshot,TextError>{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut native=NativeDecodeControl::new(limits.max_allocation_bytes,&mut progress);
 let spec=Frame::__dsl_spec_producer().decode(&mut native).map_err(positioned)?;
 let body=store::semio_format::split_text_preamble_controlled(text,STDIO_IFC2X3_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,&mut native).map_err(|error|positioned(error.into_value_error()))?;
 let record=semio_framework_dsl_record::parse_exact_controlled(body,&spec,&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:limits.max_file_bytes,..Default::default()},mode:semio_framework_dsl_record::SourceMode::Document},&mut native)?;
 reconstruct(binding::frame(&record,&mut native,limits.max_rows).map_err(positioned)?,&mut native).map_err(positioned)
}

pub(super) fn print_text(value:&Ifc2x3Snapshot)->String{
 let limits=store::sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;
 match encode(value,SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut progress,limits)).expect("IFC2x3 complete native text within default limits"){IoPayload::Text(text)=>text,IoPayload::Binary(_)=>unreachable!()}
}
