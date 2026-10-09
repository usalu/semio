//! 🫳️ Canonical JSON octets borrow original ranked source fields and retain caller output on refusal.
use super::{JsonWriteSource,JsonWriteNode,ScalarText,write_float_to,MAX_DEPTH,ValueError,ValueRefusalKind};
use semio_framework_value::NativeEncodeControl;
use std::fmt::Write;

struct Sink<'output,F>{output:&'output mut F,maximum:u64,completed:u64,maximum_items:u64}
impl<F> Sink<'_,F>{
    fn raw<E:From<ValueError>>(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),E>
    where F:FnMut(&[u8],&mut NativeEncodeControl<'_>)->Result<(),E>{
        let next=self.completed.checked_add(text.len()as u64).filter(|length|*length<=self.maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"borrowed JSON output exceeds caller byte policy"))?;
        control.checkpoint()?;
        (self.output)(text.as_bytes(),control)?;
        self.completed=next;
        Ok(())
    }
    fn string<E:From<ValueError>>(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),E>
    where F:FnMut(&[u8],&mut NativeEncodeControl<'_>)->Result<(),E>{
        self.raw("\"",control)?;
        self.string_body(text,control)?;
        self.raw("\"",control)
    }
    fn string_body<E:From<ValueError>>(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),E>
    where F:FnMut(&[u8],&mut NativeEncodeControl<'_>)->Result<(),E>{
        control.scoped_stage(|control|{
            control.begin_stage(text.len())?;
            for character in text.chars(){
                match character{
                    '"'=>self.raw("\\\"",control)?, '\\'=>self.raw("\\\\",control)?, '\u{0008}'=>self.raw("\\b",control)?, '\u{000c}'=>self.raw("\\f",control)?, '\n'=>self.raw("\\n",control)?, '\r'=>self.raw("\\r",control)?, '\t'=>self.raw("\\t",control)?,
                    character if (character as u32)<0x20=>{let mut scalar=ScalarText::new();write!(scalar,"\\u{:04x}",character as u32).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"borrowed JSON escape exceeds fixed scalar cell"))?;self.raw(scalar.text(),control)?;},
                    character=>{let mut bytes=[0;4];self.raw(character.encode_utf8(&mut bytes),control)?;},
                }
                control.advance(character.len_utf8())?;
            }
            Ok::<_,E>(())
        })
    }
    fn node<S:JsonWriteSource,E:From<ValueError>>(&mut self,source:&S,path:&mut[usize;MAX_DEPTH as usize],depth:usize,maximum_depth:usize,control:&mut NativeEncodeControl<'_>)->Result<(),E>
    where F:FnMut(&[u8],&mut NativeEncodeControl<'_>)->Result<(),E>{
        control.scoped_depth(maximum_depth,|control|{
            control.checkpoint()?;
            let node=source.node_at_path(&path[..depth])?;
            let object=matches!(&node,JsonWriteNode::Object(_));
            match node{
                JsonWriteNode::Null=>self.raw("null",control),
                JsonWriteNode::Bool(value)=>self.raw(if value{"true"}else{"false"},control),
                JsonWriteNode::Number(value)=>{let mut scalar=ScalarText::new();match value{semio_framework_value::Number::UInt(value)=>write!(scalar,"{value}"),semio_framework_value::Number::Int(value)=>write!(scalar,"{value}"),semio_framework_value::Number::Float(value)=>write_float_to(value,&mut scalar)}.map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"borrowed JSON number exceeds fixed scalar cell"))?;self.raw(scalar.text(),control)},
                JsonWriteNode::String(value)=>self.string(value,control),
                JsonWriteNode::NativeString(value)=>{
                    self.raw("\"",control)?;
                    for index in 0..value.text_chunk_count(){let text=value.text_chunk(index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed JSON native text chunk absent"))?;self.string_body(text,control)?;}
                    self.raw("\"",control)
                },
                JsonWriteNode::Array(length)|JsonWriteNode::Object(length)=>{
                    if length as u64>self.maximum_items{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"borrowed JSON collection exceeds caller item policy").into());}
                    self.raw(if object{"{"}else{"["},control)?;
                    control.scoped_stage(|control|{
                        control.begin_stage(length)?;
                        for index in 0..length{
                            control.checkpoint()?;
                            if depth>=path.len(){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"borrowed JSON source exceeds fixed path authority").into());}
                            if index!=0{self.raw(",",control)?;}
                            if object{self.string(source.object_key_at_path(&path[..depth],index)?,control)?;self.raw(":",control)?;}
                            path[depth]=index;
                            self.node(source,path,depth+1,maximum_depth,control)?;
                            control.step()?;
                        }
                        Ok::<_,E>(())
                    })?;
                    self.raw(if object{"}"}else{"]"},control)
                },
            }
        })
    }
}

/// ✍️ Appends canonical source bytes to the same borrowed sink; refusal retains its exact accepted prefix.
pub fn write_json_source_into<S:JsonWriteSource,E:From<ValueError>,F>(source:&S,maximum_bytes:u64,maximum_depth:usize,maximum_items:u64,output:&mut F,control:&mut NativeEncodeControl<'_>)->Result<u64,E>
where F:FnMut(&[u8],&mut NativeEncodeControl<'_>)->Result<(),E>{
    let mut sink=Sink{output,maximum:maximum_bytes,completed:0,maximum_items};
    sink.node(source,&mut[0;MAX_DEPTH as usize],0,maximum_depth.min(MAX_DEPTH as usize+1),control)?;
    Ok(sink.completed)
}
