//! 🛫️ Explicit owned field projection under one native output control.
use super::{DslField,DslVariants,FieldValue,RecordValue,NativeEncodeControl,ValueError,RecordFields};

/// 🔎️ One immutable authored field at an ordinal path, without constructing a source mirror.
pub enum FieldProjectionView<'a>{Absent,Bool(bool),Int(i64),UInt(u64),Float(f64),Enum(u32),Text(&'a str),Bytes(&'a [u8]),Record(&'static [u16]),List(usize),Tuple(usize),Map(usize),Block,Statements(usize),Wire(Option<bool>),IntrinsicNull,IntrinsicBool(bool),IntrinsicNumber(semio_framework_value::Number),IntrinsicText(&'a str),IntrinsicBytes(&'a [u8]),IntrinsicArray(usize),IntrinsicObject(usize)}

/// 🌱️ The retained operation keeps this source immutable until projection and publication finish.
pub trait FieldProjectionSource{
    fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>;
    /// 🔤️ Borrows one output key from the same ordinal source without scanning previous entries.
    fn projection_key(&self,_path:&[usize],_index:usize)->Result<&str,ValueError>{Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"field owner has no retained ranked key projection"))}
}
impl<T:DslField> FieldProjectionSource for T{
    fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{DslField::projection_view(self,path)}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{DslField::projection_key(self,path,index)}
}

/// 🧭️ Refuses a path outside the same retained source owner.
pub fn projection_path_error()->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained field projection source path changed")}

struct FieldProjectionFrame{value:FieldValue,kind:u8,length:usize,next:usize,position:usize,ids:&'static [u16],key:Option<String>,key_length:usize}
impl semio_framework_value::retirement::RetireOwned for FieldProjectionFrame{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.value,self.key)}
}

#[derive(Clone,Copy,Default)]
struct FieldProjectionMeasureFrame{kind:u8,length:usize,next:usize,key:bool}

/// 🛫️ Projects one immutable source into its admitted field candidate across caller grants.
pub struct RetainedFieldProjection<T:FieldProjectionSource>{source_identity:usize,frames:Vec<FieldProjectionFrame>,path:Vec<usize>,complete:Option<FieldValue>,fault_value:Option<FieldValue>,phase:u8,enter:bool,position:usize,measure_frames:[FieldProjectionMeasureFrame;65],measure_path:[usize;64],measure_depth:usize,measure_enter:bool,measure_started:bool,measure_limit:Option<usize>,measure_complete:Option<usize>,measure_bytes:usize,source:std::marker::PhantomData<fn()->T>}
impl<T:FieldProjectionSource> RetainedFieldProjection<T>{
    /// 🪪️ Captures the retained primary source identity without visiting or cloning its payload.
    pub fn new(source:&T)->Self{Self{source_identity:source as*const T as usize,frames:Vec::new(),path:Vec::new(),complete:None,fault_value:None,phase:0,enter:true,position:0,measure_frames:[FieldProjectionMeasureFrame::default();65],measure_path:[0;64],measure_depth:0,measure_enter:true,measure_started:false,measure_limit:None,measure_complete:None,measure_bytes:0,source:std::marker::PhantomData}}
    /// 📊️ Reports completed scalar and structural projection units.
    pub fn position(&self)->usize{self.position}
    /// 📏️ Measures exact candidate payload and slot demand from the immutable source before allocation.
    pub fn measure_step(&mut self,source:&T,maximum_units:usize,maximum_source_bytes:usize,control:&mut NativeEncodeControl<'_>)->Result<Option<usize>,ValueError>{
        use FieldProjectionView as V;
        use semio_framework_value::ValueRefusalKind as K;
        if maximum_units==0{return Ok(None)}
        if self.source_identity!=source as*const T as usize||self.phase!=0||self.measure_limit.is_some_and(|limit|limit!=maximum_source_bytes){return Err(projection_path_error())}
        if let Some(bytes)=self.measure_complete{return Ok(Some(bytes))}
        self.measure_limit=Some(maximum_source_bytes);
        for _ in 0..maximum_units{
            control.checkpoint()?;
            if self.measure_enter{
                let depth=if self.measure_started{self.measure_depth}else{0};
                if depth>=65{return Err(ValueError::new(K::DepthLimit,"retained source demand exceeds depth limit"))}
                let mut frame=FieldProjectionMeasureFrame::default();
                let (count,width)=match source.projection_view(&self.measure_path[..depth])?{
                    V::Text(text)|V::IntrinsicText(text)=>(text.len(),1),V::Bytes(bytes)|V::IntrinsicBytes(bytes)=>(bytes.len(),1),
                    V::Record(ids)=>{if ids.len()>256{return Err(ValueError::new(K::OwnershipLimit,"retained record exceeds declared field capacity"))}frame.kind=3;frame.length=ids.len();(ids.len(),std::mem::size_of::<(u16,FieldValue)>())},
                    V::List(length)|V::Tuple(length)=>{frame.kind=4;frame.length=length;(length,std::mem::size_of::<FieldValue>())},
                    V::Map(length)=>{frame.kind=7;frame.length=length;(length,std::mem::size_of::<(String,FieldValue)>())},
                    V::Statements(length)=>{frame.kind=13;frame.length=length;(length,std::mem::size_of::<(String,RecordValue)>())},
                    V::Wire(_)=>{frame.kind=12;frame.length=9;(1,std::mem::size_of::<super::WireValue>())},
                    V::Block=>{frame.kind=6;frame.length=1;(1,std::mem::size_of::<FieldValue>())},
                    V::IntrinsicArray(length)=>{frame.kind=10;frame.length=length;(length,std::mem::size_of::<super::DslValue>())},
                    V::IntrinsicObject(length)=>{frame.kind=11;frame.length=length;(length,std::mem::size_of::<(String,super::DslValue)>())},
                    _=>(0,1),
                };
                self.measure_charge(count,width,maximum_source_bytes)?;self.measure_frames[depth]=frame;self.measure_depth=depth+1;self.measure_started=true;self.measure_enter=false;
            }else{
                let depth=self.measure_depth.checked_sub(1).ok_or_else(projection_path_error)?;let frame=self.measure_frames[depth];
                if frame.next<frame.length{
                    if matches!(frame.kind,7|11|13)&&!frame.key{let key=source.projection_key(&self.measure_path[..depth],frame.next)?;self.measure_charge(key.len(),1,maximum_source_bytes)?;self.measure_frames[depth].key=true;}
                    else{if depth>=64{return Err(ValueError::new(K::DepthLimit,"retained source demand exceeds depth limit"))}self.measure_path[depth]=frame.next;self.measure_enter=true;}
                }else{
                    self.measure_depth-=1;
                    if self.measure_depth==0{self.measure_complete=Some(self.measure_bytes);self.position+=1;control.step()?;return Ok(self.measure_complete)}
                    let parent=&mut self.measure_frames[self.measure_depth-1];parent.next+=1;parent.key=false;
                }
            }
            self.position+=1;control.step()?;
        }
        Ok(None)
    }
    fn measure_charge(&mut self,count:usize,width:usize,maximum:usize)->Result<(),ValueError>{let next=count.checked_mul(width).and_then(|bytes|self.measure_bytes.checked_add(bytes)).filter(|bytes|*bytes<=maximum).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"retained source demand exceeds admitted operation capacity"))?;self.measure_bytes=next;Ok(())}
    fn frame(view:FieldProjectionView<'_>,control:&mut NativeEncodeControl<'_>)->Result<FieldProjectionFrame,ValueError>{
        use FieldProjectionView as V;
        let mut frame=FieldProjectionFrame{value:FieldValue::Absent,kind:0,length:0,next:0,position:0,ids:&[],key:None,key_length:0};
        frame.value=match view{
            V::Absent=>FieldValue::Absent,V::Bool(value)=>FieldValue::Bool(value),V::Int(value)=>FieldValue::Int(value),V::UInt(value)=>FieldValue::UInt(value),V::Float(value)=>FieldValue::Float(value),V::Enum(value)=>FieldValue::Enum(value),
            V::Text(text)=>{control.charge(text.len())?;let mut value=String::new();value.try_reserve_exact(text.len()).map_err(|_|ValueError::new(semio_framework_value::ValueRefusalKind::AllocationFailed,"retained field text allocation failed"))?;frame.kind=1;frame.length=text.len();FieldValue::Text(value)},
            V::Bytes(bytes)=>{frame.kind=2;frame.length=bytes.len();FieldValue::Bytes64(control.allocate_vec(bytes.len())?)},
            V::Record(ids)=>{if ids.len()>256{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"retained record exceeds declared field capacity"))}frame.kind=3;frame.length=ids.len();frame.ids=ids;FieldValue::Record(RecordValue{fields:RecordFields::from_empty_slots(control.allocate_vec(ids.len())?)})},
            V::List(length)=>{frame.kind=4;frame.length=length;FieldValue::List(control.allocate_vec(length)?)},
            V::Tuple(length)=>{frame.kind=5;frame.length=length;FieldValue::Tuple(control.allocate_vec(length)?)},
            V::Map(length)=>{frame.kind=7;frame.length=length;FieldValue::Map(control.allocate_vec(length)?)},
            V::Statements(length)=>{frame.kind=13;frame.length=length;FieldValue::Statements(control.allocate_vec(length)?)},
            V::Wire(directed)=>{control.charge(std::mem::size_of::<super::WireValue>())?;frame.kind=12;frame.length=9;FieldValue::Wire(super::WireValue{from:super::WireNode::default(),edge:directed.map(|directed|(directed,super::WireNode::default())),edge_label:super::WireEdgeLabel::default(),properties:super::DslValue::Null})},
            V::Block=>{control.charge(std::mem::size_of::<FieldValue>())?;frame.kind=6;frame.length=1;FieldValue::Block(Box::new(FieldValue::Absent))},
            V::IntrinsicNull=>FieldValue::Value(super::DslValue::Null),V::IntrinsicBool(value)=>FieldValue::Value(super::DslValue::Bool(value)),V::IntrinsicNumber(value)=>FieldValue::Value(super::DslValue::Number(value)),
            V::IntrinsicText(text)=>{control.charge(text.len())?;let mut value=String::new();value.try_reserve_exact(text.len()).map_err(|_|ValueError::new(semio_framework_value::ValueRefusalKind::AllocationFailed,"retained intrinsic text allocation failed"))?;frame.kind=8;frame.length=text.len();FieldValue::Value(super::DslValue::String(value))},
            V::IntrinsicBytes(bytes)=>{frame.kind=9;frame.length=bytes.len();FieldValue::Value(super::DslValue::Bytes(control.allocate_vec(bytes.len())?))},
            V::IntrinsicArray(length)=>{frame.kind=10;frame.length=length;FieldValue::Value(super::DslValue::Array(control.allocate_vec(length)?))},
            V::IntrinsicObject(length)=>{frame.kind=11;frame.length=length;FieldValue::Value(super::DslValue::Object(control.allocate_vec(length)?))},
        };Ok(frame)
    }
    /// ⛽️ Uses the same cumulative control while moving at most one UTF-8 scalar or field per unit.
    pub fn step(&mut self,source:&T,maximum_units:usize,control:&mut NativeEncodeControl<'_>)->Result<Option<FieldValue>,ValueError>{
        if maximum_units==0{return Ok(None)}
        if self.source_identity!=source as*const T as usize{return Err(projection_path_error())}
        if self.phase==2{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained field projection was already transferred"))}
        for _ in 0..maximum_units{
            control.checkpoint()?;
            if self.phase==0{self.frames=control.allocate_vec(65)?;self.path=control.allocate_vec(64)?;self.phase=1;}
            else if self.enter{if self.frames.len()>=65{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::DepthLimit,"retained field projection exceeds depth limit"))}let frame=Self::frame(source.projection_view(&self.path)?,control)?;self.frames.push(frame);self.enter=false;}
            else{
                let frame=self.frames.last_mut().ok_or_else(projection_path_error)?;
                if matches!(frame.kind,1|8)&&frame.position<frame.length{
                    let text=match source.projection_view(&self.path)?{FieldProjectionView::Text(value)|FieldProjectionView::IntrinsicText(value)=>value,_=>return Err(projection_path_error())};if text.len()!=frame.length||!text.is_char_boundary(frame.position){return Err(projection_path_error())}let character=text[frame.position..].chars().next().ok_or_else(projection_path_error)?;let value=match &mut frame.value{FieldValue::Text(value)|FieldValue::Value(super::DslValue::String(value))=>value,_=>return Err(projection_path_error())};value.push(character);frame.position+=character.len_utf8();
                }else if matches!(frame.kind,2|9)&&frame.position<frame.length{
                    let bytes=match source.projection_view(&self.path)?{FieldProjectionView::Bytes(value)|FieldProjectionView::IntrinsicBytes(value)=>value,_=>return Err(projection_path_error())};if bytes.len()!=frame.length{return Err(projection_path_error())}let value=match &mut frame.value{FieldValue::Bytes64(value)|FieldValue::Value(super::DslValue::Bytes(value))=>value,_=>return Err(projection_path_error())};value.push(bytes[frame.position]);frame.position+=1;
                }else if matches!(frame.kind,7|11|13)&&frame.next<frame.length{
                    let text=source.projection_key(&self.path,frame.next)?;
                    if frame.key.is_none(){control.charge(text.len())?;let mut key=String::new();key.try_reserve_exact(text.len()).map_err(|_|ValueError::new(semio_framework_value::ValueRefusalKind::AllocationFailed,"retained field key allocation failed"))?;frame.key=Some(key);frame.key_length=text.len();frame.position=0;}
                    else if frame.position<frame.key_length{if text.len()!=frame.key_length||!text.is_char_boundary(frame.position){return Err(projection_path_error())}let character=text[frame.position..].chars().next().ok_or_else(projection_path_error)?;frame.key.as_mut().unwrap().push(character);frame.position+=character.len_utf8();}
                    else{if self.path.len()>=64{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::DepthLimit,"retained field projection exceeds depth limit"))}self.path.push(frame.next);self.enter=true;}
                }else if matches!(frame.kind,3..=7|10|12)&&frame.next<frame.length{if self.path.len()>=64{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::DepthLimit,"retained field projection exceeds depth limit"))}self.path.push(frame.next);self.enter=true;}
                else{
                    let value=self.frames.pop().unwrap().value;
                    if let Some(parent)=self.frames.last_mut(){match &mut parent.value{
                        FieldValue::Record(record)=>{let id=parent.ids[parent.next];if record.fields.contains_key(&id){self.fault_value=Some(value);return Err(projection_path_error())}record.fields.insert(id,value);},
                        FieldValue::List(items)|FieldValue::Tuple(items)=>items.push(value),
                        FieldValue::Map(items)=>{let Some(key)=parent.key.take()else{self.fault_value=Some(value);return Err(projection_path_error())};items.push((key,value));},
                        FieldValue::Statements(items)=>{let FieldValue::Record(record)=value else{self.fault_value=Some(value);return Err(projection_path_error())};let Some(key)=parent.key.take()else{self.fault_value=Some(FieldValue::Record(record));return Err(projection_path_error())};items.push((key,record));},
                        FieldValue::Wire(wire)=>{
                            if parent.next==8{let FieldValue::Value(properties)=value else{self.fault_value=Some(value);return Err(projection_path_error())};wire.properties=properties;}
                            else{let text=match value{FieldValue::Text(text)=>Some(text),FieldValue::Absent=>None,value=>{self.fault_value=Some(value);return Err(projection_path_error())}};match parent.next{0=>wire.from.id=text.ok_or_else(projection_path_error)?,1=>wire.from.kind=text,2=>wire.from.port=text,3=>if let Some((_,to))=&mut wire.edge{to.id=text.ok_or_else(projection_path_error)?},4=>if let Some((_,to))=&mut wire.edge{to.kind=text},5=>if let Some((_,to))=&mut wire.edge{to.port=text},6=>wire.edge_label.id=text,7=>wire.edge_label.kind=text,_=>return Err(projection_path_error())}}
                        },
                        FieldValue::Block(inner)=>**inner=value,
                        FieldValue::Value(super::DslValue::Array(items))=>{let FieldValue::Value(value)=value else{self.fault_value=Some(value);return Err(projection_path_error())};items.push(value);},
                        FieldValue::Value(super::DslValue::Object(items))=>{let FieldValue::Value(value)=value else{self.fault_value=Some(value);return Err(projection_path_error())};let Some(key)=parent.key.take()else{self.fault_value=Some(FieldValue::Value(value));return Err(projection_path_error())};items.push((key,value));},
                        _=>{self.fault_value=Some(value);return Err(projection_path_error())},
                    }parent.next+=1;self.path.pop();}
                    else{self.complete=Some(value);self.phase=2;self.position+=1;control.step()?;return Ok(self.complete.take())}
                }
            }
            self.position+=1;control.step()?;
        }Ok(None)
    }
}
impl<T:FieldProjectionSource+'static> semio_framework_value::retirement::RetireOwned for RetainedFieldProjection<T>{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.frames,self.path,self.complete,self.fault_value)}
}

/// ♻️ Retires intermediate native fields without recursive container drops.
pub fn retire_field(value:FieldValue){
    if matches!(&value,FieldValue::Absent|FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Text(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)){return;}
    let mut pending=vec![value];
    while let Some(value)=pending.pop(){match value{
        FieldValue::Tuple(mut items)|FieldValue::List(mut items)=>pending.append(&mut items),
        FieldValue::Record(record)=>pending.extend(record.fields.into_values()),
        FieldValue::Block(value)=>pending.push(*value),
        FieldValue::Statements(items)=>for(_,record)in items{pending.extend(record.fields.into_values());},
        FieldValue::Map(items)=>for(_,value)in items{pending.push(value);},
        FieldValue::Value(value)=><super::DslValue as semio_framework_value::FromValue>::retire_decoded(value),
        FieldValue::Wire(value)=><super::DslValue as semio_framework_value::FromValue>::retire_decoded(value.properties),
        FieldValue::Expr(value)=>{let mut expressions=vec![value];while let Some(value)=expressions.pop(){match value{super::ExprValue::Neg(value)=>expressions.push(*value),super::ExprValue::Binary(_,left,right)=>{expressions.push(*left);expressions.push(*right);},super::ExprValue::Call(_,mut items)=>expressions.append(&mut items),super::ExprValue::Num(_)|super::ExprValue::Var(_)=>{}}}},
        FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Text(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)|FieldValue::Absent=>{}
    }}
}

/// 🛡️ Holds intermediate fields until the complete record is committed.
pub struct EncodedRecord{record:Option<RecordValue>}
impl EncodedRecord{
    /// 🛡️ Retains a producer-admitted complete record across physical output failures.
    pub fn from_record(record:RecordValue)->Self{Self{record:Some(record)}}
    /// 🔎️ Borrows the guarded record without materializing another field graph.
    pub fn as_record(&self)->&RecordValue{self.record.as_ref().unwrap()}
    /// 📦️ Admits the complete concrete record slot buffer before allocation.
    pub fn new(count:usize,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{let fields=RecordFields::from_empty_slots(control.allocate_vec(count)?);Ok(Self{record:Some(RecordValue{fields})})}
    /// 📥️ Publishes a completed explicit field into this guarded record.
    pub fn insert(&mut self,id:u16,value:FieldValue)->Result<(),ValueError>{let record=self.record.as_mut().unwrap();if !record.fields.contains_key(&id)&&record.fields.len()==record.fields.capacity(){retire_field(value);return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"native record exceeds admitted slots"));}if let Some(previous)=record.fields.insert(id,value){retire_field(previous);}Ok(())}
    /// 📤️ Transfers the fully constructed record to its physical encoder.
    pub fn take(mut self)->RecordValue{self.record.take().unwrap()}
}
impl Drop for EncodedRecord{fn drop(&mut self){if let Some(record)=self.record.take(){for value in record.fields.into_values(){retire_field(value);}}}}

/// 📋️ Projects a borrowed collection with its exact item workload and partial-field retirement.
pub fn project_list<T:DslField,C:super::DslSequenceView<T>+?Sized>(values:&C,control:&mut NativeEncodeControl<'_>)->Result<Vec<FieldValue>,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(values.field_items().len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.field_items().len())?,|items:Vec<FieldValue>|{for item in items{retire_field(item);}});for value in values.field_items(){output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control)})?);control.step()?;}Ok(output.take())})
}

/// 🗺️ Projects literal map keys and their typed values before materializing the physical map.
pub fn project_map<T:DslField>(values:&std::collections::BTreeMap<String,T>,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?,|items:Vec<(String,FieldValue)>|{for(_,value)in items{retire_field(value);}});for(key,value)in values{let key=control.copy_text(key)?;let value=control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control)})?;output.as_mut().push((key,value));control.step()?;}Ok(FieldValue::Map(output.take()))})
}

/// 🌿️ Projects tagged records with literal owner keywords and known collection progress.
pub fn project_statements<T:DslVariants,C:super::DslSequenceView<T>+?Sized>(values:&C,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(values.field_items().len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.field_items().len())?,|items:Vec<(String,RecordValue)>|{for(_,record)in items{for value in record.fields.into_values(){retire_field(value);}}});for value in values.field_items(){output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;value.to_named_record_controlled(control)})?);control.step()?;}Ok(FieldValue::Statements(output.take()))})
}

impl semio_framework_value::retirement::RetireOwned for RecordFields {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.into_iter())}
}
impl semio_framework_value::retirement::RetireOwned for RecordValue {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.fields)}
}
impl semio_framework_value::retirement::RetireOwned for FieldValue {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;match self{
        Self::Text(value)=>value.retirement(),Self::Bytes64(value)=>value.retirement(),Self::Tuple(value)|Self::List(value)=>value.retirement(),Self::Record(value)=>value.retirement(),Self::Block(value)=>value.retirement(),Self::Statements(value)=>value.retirement(),Self::Map(value)=>value.retirement(),Self::Value(value)=>value.retirement(),Self::Wire(value)=>value.retirement(),Self::Expr(value)=>value.retirement(),
        Self::Bool(_)|Self::Int(_)|Self::UInt(_)|Self::Float(_)|Self::Enum(_)|Self::Absent=>semio_framework_value::retirement::leaf(()),
    }}
}
impl semio_framework_value::retirement::RetireOwned for super::ExprValue {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;match self{Self::Var(value)=>value.retirement(),Self::Neg(value)=>value.retirement(),Self::Binary(_,left,right)=>semio_framework_value::artifact_retirement_sequence!(left,right),Self::Call(name,items)=>semio_framework_value::artifact_retirement_sequence!(name,items),Self::Num(_)=>semio_framework_value::retirement::leaf(())}}
}
impl semio_framework_value::retirement::RetireOwned for super::WireNode {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.id,self.kind,self.port)}
}
impl semio_framework_value::retirement::RetireOwned for super::WireEdgeLabel {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.id,self.kind)}
}
impl semio_framework_value::retirement::RetireOwned for super::WireValue {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.from,self.edge,self.edge_label,self.properties)}
}
semio_framework_value::artifact_retire_leaf!(super::RecordSpecProducer);
impl semio_framework_value::retirement::RetireOwned for super::Shape {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;match self{Self::Enum(items)=>items.retirement(),Self::Tuple(shape,_)|Self::List(shape)|Self::Block(shape)|Self::Map(shape)=>shape.retirement(),Self::Record(make)|Self::Table(make)=>make.retirement(),Self::Statements(items)=>items.retirement(),_=>semio_framework_value::retirement::leaf(())}}
}
impl semio_framework_value::retirement::RetireOwned for super::FieldSpec {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.key,self.shape)}
}
impl semio_framework_value::retirement::RetireOwned for super::RecordSpec {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.keyword,self.fields)}
}

/// 🫳️ Borrows a declared original operation variant without owning a Record payload mirror.
pub struct VariantProjection<'a,T:DslVariants>{source:&'a T}
impl<'a,T:DslVariants> VariantProjection<'a,T>{
    /// 🌿️ Keeps the exact original variant immutable through canonical emission.
    pub fn new(source:&'a T)->Self{Self{source}}
}
impl<T:DslVariants> FieldProjectionSource for VariantProjection<'_,T>{
    fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{self.source.projected_variant_view(path)}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.source.projected_variant_key(path,index)}
}
