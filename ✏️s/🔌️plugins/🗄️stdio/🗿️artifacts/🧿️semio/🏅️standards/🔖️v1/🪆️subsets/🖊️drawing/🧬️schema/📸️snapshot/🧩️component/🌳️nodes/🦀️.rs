//! 🌳️ Borrowed Drawing node construction with literal variant fields and guarded forests.
use super::super::{DrawNode,PathSegment,SemioPoint2,SemioTransform};
use semio_framework_value::{DslValue,ToValue,FromValue,ValueError,ValueRefusalKind,DecodedValue,NativeDecodeControl,NativeEncodeControl};

pub fn retire_node(value:DrawNode){drop(crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned::new(value));}
fn retire_nodes(values:Vec<DrawNode>){for value in values{retire_node(value);}}
fn retire_values(values:Vec<DslValue>){<Vec<DslValue> as FromValue>::retire_decoded(values);}

fn encode_push<T>(items:&mut Vec<T>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
    if items.len()==items.capacity(){let additional=items.capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing frontier overflow"))?)?;items.try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing frontier allocation"))?;}
    items.push(value);Ok(())
}
fn decode_push<T>(items:&mut Vec<T>,value:T,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
    if items.len()==items.capacity(){let additional=items.capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing frontier overflow"))?)?;items.try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing frontier allocation"))?;}
    items.push(value);Ok(())
}
fn entries<'a>(value:&'a DslValue,control:&mut NativeDecodeControl<'_>)->Result<&'a[(String,DslValue)],ValueError>{value.object_controlled(control)}
fn required<'a>(values:&'a[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<&'a DslValue,ValueError>{DslValue::field_controlled(values,key,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,format!("missing Drawing field {key}")))}
fn decode<T:FromValue>(values:&[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{T::from_value_controlled(required(values,key,control)?,control).map_err(|error|error.under(key))}
fn optional<T:FromValue>(values:&[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<T>,ValueError>{match DslValue::field_controlled(values,key,control)?{Some(value)=>Option::<T>::from_value_controlled(value,control).map_err(|error|error.under(key)),None=>Ok(None)}}
fn child_values<'a>(values:&'a[(String,DslValue)],control:&mut NativeDecodeControl<'_>)->Result<&'a[DslValue],ValueError>{match DslValue::field_controlled(values,"children",control)?{None=>Ok(&[]),Some(DslValue::Array(values))=>Ok(values),Some(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Drawing children must be an array"))}}

enum Input<'a>{Node(&'a DslValue),Group(SemioTransform,usize)}
pub fn decode_node(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawNode,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut pending=Vec::new();decode_push(&mut pending,Input::Node(value),control)?;
        let mut built=DecodedValue::new(Vec::<DrawNode>::new(),retire_nodes);
        while let Some(task)=pending.pop(){
            match task{
                Input::Node(value)=>{
                    let fields=entries(value,control)?;
                    let DslValue::String(kind)=required(fields,"kind",control)? else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Drawing kind must be text"))};
                    let node=match kind.as_str(){
                        "path"=>DrawNode::Path{segments:decode::<Vec<PathSegment>>(fields,"segments",control)?,style:optional(fields,"style",control)?},
                        "text"=>DrawNode::Text{value:decode(fields,"value",control)?,at:decode::<SemioPoint2>(fields,"at",control)?,style:optional(fields,"style",control)?},
                        "image"=>DrawNode::Image{at:decode(fields,"at",control)?,width:decode(fields,"width",control)?,height:decode(fields,"height",control)?,mime:decode(fields,"mime",control)?,bytes:decode(fields,"bytes",control)?},
                        "group"=>{let transform=decode::<SemioTransform>(fields,"transform",control)?;let children=child_values(fields,control)?;decode_push(&mut pending,Input::Group(transform,children.len()),control)?;for child in children.iter().rev(){decode_push(&mut pending,Input::Node(child),control)?;control.step()?;}control.step()?;continue;},
                        _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Drawing kind")),
                    };
                    let owner=DecodedValue::new(node,retire_node);
                    if built.get().len()==built.get().capacity(){let additional=built.get().capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<DrawNode>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing forest overflow"))?)?;built.get_mut().try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing forest allocation"))?;}
                    built.get_mut().push(owner.take());
                },
                Input::Group(transform,count)=>{
                    let start=built.get().len().checked_sub(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing child construction"))?;
                    let mut children=DecodedValue::new(control.allocate_vec::<DrawNode>(count)?,retire_nodes);
                    for node in built.get_mut().drain(start..){children.get_mut().push(node);}
                    let owner=DecodedValue::new(DrawNode::Group{transform,children:children.take()},retire_node);
                    if built.get().len()==built.get().capacity(){control.charge(std::mem::size_of::<DrawNode>())?;built.get_mut().try_reserve_exact(1).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing group allocation"))?;}
                    built.get_mut().push(owner.take());
                },
            }
            control.step()?;
        }
        if built.get().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing root construction"))}
        Ok(built.get_mut().pop().unwrap())
    })
}

fn object(count:usize,control:&mut NativeEncodeControl<'_>)->Result<DecodedValue<DslValue>,ValueError>{Ok(DecodedValue::new(DslValue::Object(control.allocate_vec(count)?),<DslValue as FromValue>::retire_decoded))}
fn add<T:ToValue+?Sized>(value:&mut DslValue,key:&str,field:&T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
    let key=control.copy_text(key)?;let child=field.to_value_controlled(control)?;
    let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing object construction"))};fields.push((key,child));Ok(())
}
fn add_owned(value:&mut DslValue,key:&str,child:DecodedValue<DslValue>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let key=control.copy_text(key)?;let DslValue::Object(fields)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing object construction"))};fields.push((key,child.take()));Ok(())}
enum Output<'a>{Node(&'a DrawNode),Group(&'a SemioTransform,usize)}
pub fn encode_node(value:&DrawNode,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut pending=Vec::new();encode_push(&mut pending,Output::Node(value),control)?;
        let mut built=DecodedValue::new(Vec::<DslValue>::new(),retire_values);
        while let Some(task)=pending.pop(){
            let output=match task{
                Output::Node(DrawNode::Path{segments,style})=>{let mut v=object(2+usize::from(style.is_some()),control)?;add(v.get_mut(),"kind",&"path",control)?;add(v.get_mut(),"segments",segments,control)?;if let Some(style)=style{add(v.get_mut(),"style",style,control)?;}v},
                Output::Node(DrawNode::Text{value,at,style})=>{let mut v=object(3+usize::from(style.is_some()),control)?;add(v.get_mut(),"kind",&"text",control)?;add(v.get_mut(),"value",value,control)?;add(v.get_mut(),"at",at,control)?;if let Some(style)=style{add(v.get_mut(),"style",style,control)?;}v},
                Output::Node(DrawNode::Image{at,width,height,mime,bytes})=>{let mut v=object(6,control)?;add(v.get_mut(),"kind",&"image",control)?;add(v.get_mut(),"at",at,control)?;add(v.get_mut(),"width",width,control)?;add(v.get_mut(),"height",height,control)?;add(v.get_mut(),"mime",mime,control)?;add(v.get_mut(),"bytes",bytes,control)?;v},
                Output::Node(DrawNode::Group{transform,children})=>{encode_push(&mut pending,Output::Group(transform,children.len()),control)?;for child in children.iter().rev(){encode_push(&mut pending,Output::Node(child),control)?;control.step()?;}control.step()?;continue;},
                Output::Group(transform,count)=>{let start=built.get().len().checked_sub(count).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing projected children"))?;let mut children=DecodedValue::new(control.allocate_vec::<DslValue>(count)?,retire_values);for child in built.get_mut().drain(start..){children.get_mut().push(child);}let mut v=object(3,control)?;add(v.get_mut(),"kind",&"group",control)?;add(v.get_mut(),"transform",transform,control)?;add_owned(v.get_mut(),"children",DecodedValue::new(DslValue::Array(children.take()),<DslValue as FromValue>::retire_decoded),control)?;v},
            };
            if built.get().len()==built.get().capacity(){let additional=built.get().capacity().max(1);control.charge(additional.checked_mul(std::mem::size_of::<DslValue>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Drawing projected forest overflow"))?)?;built.get_mut().try_reserve_exact(additional).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Drawing projected forest allocation"))?;}
            built.get_mut().push(output.take());control.step()?;
        }
        if built.get().len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Drawing projected root"))}
        Ok(built.get_mut().pop().unwrap())
    })
}
