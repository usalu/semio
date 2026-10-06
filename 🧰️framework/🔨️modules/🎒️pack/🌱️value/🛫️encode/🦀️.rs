//! 🛫️ Literal Pack fields with borrowed symbol discovery and controlled physical emission.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use semio_framework_value::native_encoding::NativeEncodeControl;
use semio_framework_dsl_record::NativeSchemaControl;
use std::{cmp::Ordering,mem::size_of};
#[path = "🫳️borrowed/🦀️.rs"]
mod borrowed;
pub(super) use borrowed::projected_record_body_into;


fn push<T>(values:&mut Vec<T>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{if values.len()==values.capacity(){let capacity=if values.capacity()==0{1}else{values.capacity().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Pack output frontier capacity overflow"))?};control.charge(capacity.checked_mul(size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Pack output frontier size overflow"))?)?;control.checkpoint()?;values.try_reserve_exact(capacity-values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Pack output frontier allocation"))?;}values.push(value);Ok(())}
fn nested(shape:Option<&Shape>,control:&mut NativeEncodeControl<'_>)->Result<Option<RecordSpec>,PackRefusal>{match shape{Some(Shape::Record(producer))=>producer.encode(control).map(Some).map_err(PackRefusal::from),_=>Ok(None)}}

#[derive(Clone,Copy)]
struct Symbol<'a>{text:&'a str,forced:bool}
struct Symbols<'a>{entries:Vec<Symbol<'a>>}
enum DynamicItems<'a>{Array(std::slice::Iter<'a,DslValue>),Object(std::slice::Iter<'a,(String,DslValue)>)}
struct DynamicFrame<'a>{items:DynamicItems<'a>,depth:u16}
impl<'a> DynamicFrame<'a>{
    fn next(&mut self)->Option<(Option<&'a str>,&'a DslValue)>{match &mut self.items{DynamicItems::Array(items)=>items.next().map(|value|(None,value)),DynamicItems::Object(items)=>items.next().map(|(key,value)|(Some(key.as_str()),value))}}
}
fn dynamic_child_depth(depth:u16,maximum:u16)->Result<u16,PackRefusal>{depth.checked_add(1).filter(|next|*next<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic child exceeds declared depth").into())}
impl<'a> Symbols<'a>{
    fn note(&mut self,text:&'a str,forced:bool,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{push(&mut self.entries,Symbol{text,forced},control)}
    fn record(&mut self,spec:Option<&RecordSpec>,record:&'a RecordValue,depth:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        if depth>maximum.min(64){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack symbol discovery depth exceeds limit").into())}control.scoped_stage(|control|{control.begin_stage(record.fields.len())?;for(id,value)in &record.fields{let shape=spec.and_then(|spec|spec.fields.iter().find(|field|field.id==*id)).map(|field|&field.shape);self.value(shape,value,depth+1,maximum,control)?;control.step()?;}Ok(())})
    }
    fn value(&mut self,shape:Option<&Shape>,value:&'a FieldValue,depth:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        if depth>maximum.min(64){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack symbol discovery depth exceeds limit").into())}control.scoped_stage(|control|{control.begin_stage(0)?;control.step()?;match value{
            FieldValue::Text(text)=>self.note(text,false,control)?,
            FieldValue::Tuple(items)=>{control.begin_stage(items.len())?;for item in items{self.value(elem_shape_of(shape),item,depth+1,maximum,control)?;control.step()?;}},
            FieldValue::List(items)=>{let table=table_spec_of(shape).map(|producer|producer.encode(control)).transpose()?;control.begin_stage(items.len())?;for item in items{if let(Some(spec),FieldValue::Record(row))=(&table,item){for field in &spec.fields{if let Some(value)=row.fields.get(&field.id){if matches!(field.shape,Shape::Text|Shape::Ref(_)){if let FieldValue::Text(text)=value{self.note(text,true,control)?;}}else{self.value(Some(&field.shape),value,depth+1,maximum,control)?;}}}}else{self.value(elem_shape_of(shape),item,depth+1,maximum,control)?;}control.step()?;}},
            FieldValue::Record(record)=>{let spec=nested(shape,control)?;self.record(spec.as_ref(),record,depth+1,maximum,control)?;},
            FieldValue::Block(value)=>self.value(block_inner_shape(shape),value,depth+1,maximum,control)?,
            FieldValue::Statements(items)=>{control.begin_stage(items.len())?;for(keyword,record)in items{self.note(keyword,true,control)?;let spec=statements_variants(shape).and_then(|variants|variants.iter().find(|(key,_)|key==keyword)).map(|(_,producer)|producer.encode(control)).transpose()?;self.record(spec.as_ref(),record,depth+1,maximum,control)?;control.step()?;}},
            FieldValue::Map(items)=>{control.begin_stage(items.len())?;for(key,value)in items{self.note(key,false,control)?;self.value(map_inner_shape(shape),value,depth+1,maximum,control)?;control.step()?;}},
            FieldValue::Value(value)=>self.dynamic(value,depth+1,maximum,control)?,
            FieldValue::Wire(wire)=>{self.node(&wire.from,control)?;if let Some((_,node))=&wire.edge{self.node(node,control)?;}self.dynamic(&wire.properties,depth+1,maximum,control)?;},
            FieldValue::Absent|FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)|FieldValue::Expr(_)=>{},
        }Ok(())})
    }
    fn node(&mut self,node:&'a WireNode,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.note(&node.id,false,control)?;if let Some(text)=&node.kind{self.note(text,false,control)?;}if let Some(text)=&node.port{self.note(text,false,control)?;}Ok(())}
    fn dynamic(&mut self,value:&'a DslValue,depth:u16,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        control.scoped_stage(|control|{
            control.begin_stage(0)?;let mut frames=Vec::new();let(mut value,mut depth)=(value,depth);
            'visit:loop{
                if depth>maximum{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic symbol depth exceeds declared limit").into())}control.checkpoint()?;control.step()?;
                match value{
                    DslValue::String(text)=>self.note(text,false,control)?,
                    DslValue::Array(items) if !items.is_empty()=>{let depth=dynamic_child_depth(depth,maximum)?;push(&mut frames,DynamicFrame{items:DynamicItems::Array(items.iter()),depth},control)?;},
                    DslValue::Object(items) if !items.is_empty()=>{let depth=dynamic_child_depth(depth,maximum)?;push(&mut frames,DynamicFrame{items:DynamicItems::Object(items.iter()),depth},control)?;},
                    DslValue::Null|DslValue::Bool(_)|DslValue::Number(_)|DslValue::Bytes(_)|DslValue::Array(_)|DslValue::Object(_)=>{},
                }
                loop{let Some(frame)=frames.last_mut()else{return Ok(())};if let Some((_,child))=frame.next(){value=child;depth=frame.depth;continue 'visit;}frames.pop();}
            }
        })
    }
    fn finish(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{
        controlled_schema::sort(&mut self.entries,|a,b,control|controlled_schema::compare_text(a.text,b.text,control),control)?;
        control.scoped_stage(|control|{control.begin_stage(self.entries.len())?;let mut start=0;let mut output=0;while start<self.entries.len(){let text=self.entries[start].text;let mut end=start+1;let mut forced=self.entries[start].forced;control.step()?;while end<self.entries.len()&&controlled_schema::compare_text(text,self.entries[end].text,control)?==Ordering::Equal{forced|=self.entries[end].forced;end+=1;control.step()?;}if forced||text.len()<=128||end-start>=2{self.entries[output]=Symbol{text,forced};output+=1;}start=end;}self.entries.truncate(output);Ok(())})
    }
    fn index(&self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<u64>,PackRefusal>{let mut low=0;let mut high=self.entries.len();while low<high{let middle=low+(high-low)/2;match controlled_schema::compare_text(self.entries[middle].text,text,control)?{Ordering::Less=>low=middle+1,Ordering::Greater=>high=middle,Ordering::Equal=>return Ok(Some(middle as u64))}}Ok(None)}
}

fn table_value(row:&FieldValue,id:u16)->Option<&FieldValue>{match row{FieldValue::Record(record)=>record.fields.get(&id).filter(|value|!matches!(value,FieldValue::Absent)),_=>None}}

struct Output<'a>{bytes:Option<Vec<u8>>,external:Option<&'a mut dyn protocol::io::binary::operation_bytes::OperationByteOutput>,length:usize}
impl<'a> Output<'a>{
    fn measure()->Self{Self{bytes:None,external:None,length:0}}
    fn allocated(length:usize,control:&mut NativeEncodeControl<'_>)->Result<Self,PackRefusal>{Ok(Self{bytes:Some(control.allocate_vec(length)?),external:None,length:0})}
    fn bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.length=self.length.checked_add(bytes.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Pack output length overflow"))?;if let Some(output)=&mut self.external{output.write_bytes(bytes,control)?;}if let Some(output)=&mut self.bytes{if bytes.len()>output.capacity()-output.len(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Pack output exceeded admitted length").into())}control.scoped_stage(|control| -> ::core::result::Result<(),PackRefusal> {control.begin_stage(bytes.len())?;for fragment in bytes.chunks(65536){output.extend_from_slice(fragment);control.advance(fragment.len())?;}Ok::<_,PackRefusal>(())})?;}Ok(())}
    fn byte(&mut self,byte:u8,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.bytes(&[byte],control)}
    fn varint(&mut self,mut value:u64,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{let mut bytes=[0;10];let mut length=0;loop{bytes[length]=(value as u8)&127;value>>=7;if value!=0{bytes[length]|=128;}length+=1;if value==0{break;}}self.bytes(&bytes[..length],control)}
    fn signed(&mut self,value:i64,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.varint(((value as u64)<<1)^((value>>63)as u64),control)}
}

struct Encoder<'a,'b,'c,'d>{symbols:&'a Symbols<'b>,options:&'a EncodeOptions,control:&'c mut NativeEncodeControl<'d>,writer:Option<&'c mut crate::format::ControlledPackWriter>,chunking:bool,next_chunk:u32}
impl Encoder<'_,'_,'_,'_>{
    fn text(&mut self,text:&str,inline:bool,output:&mut Output)->Result<(),PackRefusal>{if !inline{if let Some(index)=self.symbols.index(text,self.control)?{output.byte(TAG_STR,self.control)?;return output.varint(index,self.control)}}output.byte(TAG_STR_INLINE,self.control)?;output.varint(text.len()as u64,self.control)?;output.bytes(text.as_bytes(),self.control)}
    fn forced(&mut self,text:&str,output:&mut Output)->Result<(),PackRefusal>{let index=self.symbols.index(text,self.control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Pack forced symbol missing"))?;output.varint(index,self.control)}
    fn fields(&mut self,spec:Option<&RecordSpec>,record:&RecordValue,depth:u16,output:&mut Output)->Result<(),PackRefusal>{
        if depth>self.options.limits.max_depth.min(64){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack output record depth exceeds limit").into())}let mut ids=self.control.allocate_vec(record.fields.len())?;self.control.scoped_stage(|control|{control.begin_stage(record.fields.len())?;for(id,value)in &record.fields{if !matches!(value,FieldValue::Absent)&&(self.options.preserve_unknown||spec.is_some_and(|spec|spec.fields.iter().any(|field|field.id==*id))){ids.push(*id);}control.step()?;}Ok::<_,PackRefusal>(())})?;controlled_schema::sort(&mut ids,|a,b,_|Ok(a.cmp(b)),self.control)?;output.varint(ids.len()as u64,self.control)?;
        self.control.begin_stage(0)?;for id in ids{let value=record.fields.get(&id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"native record identity disappeared"))?;let shape=spec.and_then(|spec|spec.fields.iter().find(|field|field.id==id)).map(|field|&field.shape);output.varint(id as u64,self.control)?;self.value(shape,value,depth+1,output)?;}Ok(())
    }
    fn value(&mut self,shape:Option<&Shape>,value:&FieldValue,depth:u16,output:&mut Output)->Result<(),PackRefusal>{
        if depth>self.options.limits.max_depth.min(64){return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack output value depth exceeds limit").into())}self.control.checkpoint()?;match value{
            FieldValue::Absent=>output.byte(TAG_ABSENT,self.control),FieldValue::Bool(value)=>output.byte(if *value{TAG_TRUE}else{TAG_FALSE},self.control),
            FieldValue::Int(value)=>{output.byte(TAG_INT,self.control)?;output.signed(*value,self.control)},FieldValue::UInt(value)=>{output.byte(TAG_UINT,self.control)?;output.varint(*value,self.control)},FieldValue::Float(value)=>{output.byte(TAG_F64,self.control)?;output.bytes(&value.to_le_bytes(),self.control)},FieldValue::Enum(value)=>{output.byte(TAG_ENUM,self.control)?;output.varint(*value as u64,self.control)},
            FieldValue::Text(text)=>self.text(text,false,output),FieldValue::Bytes64(bytes)=>self.bytes(bytes,output),
            FieldValue::Tuple(items)=>self.sequence(items,elem_shape_of(shape),true,depth,output),FieldValue::List(items)=>if let Some(producer)=table_spec_of(shape){self.table(producer,items,depth,output)}else{self.sequence(items,elem_shape_of(shape),false,depth,output)},
            FieldValue::Record(record)=>{let spec=nested(shape,self.control)?;output.byte(TAG_RECORD,self.control)?;self.fields(spec.as_ref(),record,depth+1,output)},
            FieldValue::Block(value)=>{output.byte(TAG_BLOCK,self.control)?;self.value(block_inner_shape(shape),value,depth+1,output)},
            FieldValue::Statements(items)=>{output.byte(TAG_STATEMENTS,self.control)?;output.varint(items.len()as u64,self.control)?;for(keyword,record)in items{self.control.checkpoint()?;self.forced(keyword,output)?;let spec=statements_variants(shape).and_then(|variants|variants.iter().find(|(key,_)|key==keyword)).map(|(_,producer)|producer.encode(self.control)).transpose()?;self.fields(spec.as_ref(),record,depth+1,output)?;}Ok(())},
            FieldValue::Map(items)=>{let mut sorted=self.control.allocate_vec(items.len())?;for(index,item)in items.iter().enumerate(){self.control.checkpoint()?;if !matches!(item.1,FieldValue::Absent){sorted.push((index,item));}}controlled_schema::sort(&mut sorted,|a,b,control|{let order=controlled_schema::compare_text(&a.1.0,&b.1.0,control)?;Ok(if order==Ordering::Equal{a.0.cmp(&b.0)}else{order})},self.control)?;output.byte(TAG_MAP,self.control)?;output.varint(sorted.len()as u64,self.control)?;for(_, (key,value))in sorted{self.text(key,false,output)?;self.value(map_inner_shape(shape),value,depth+1,output)?;}Ok(())},
            FieldValue::Value(value)=>{output.byte(TAG_VALUE,self.control)?;self.dynamic(value,depth+1,output)},FieldValue::Wire(value)=>{output.byte(TAG_WIRE,self.control)?;self.wire(value,depth+1,output)},
            FieldValue::Expr(value)=>{let text=semio_framework_dsl_record::print_expr_controlled(value,self.control).map_err(PackRefusal::from)?;output.byte(TAG_EXPR,self.control)?;self.text(&text,false,output)},
        }
    }
    fn bytes(&mut self,bytes:&[u8],output:&mut Output)->Result<(),PackRefusal>{
        if self.chunking&&(bytes.len()as u64)>=self.options.chunk_threshold{let size=usize::try_from(self.options.chunk_size.max(1)).map_err(|_|ValueError::new(ValueRefusalKind::OwnershipLimit,"Pack chunk size exceeds address space"))?;let count=bytes.len().div_ceil(size);output.byte(TAG_BYTES_CHUNKED,self.control)?;output.varint(count as u64,self.control)?;for piece in bytes.chunks(size){self.control.checkpoint()?;let id=if let Some(writer)=&mut self.writer{writer.chunk(piece,self.control)?.0}else{self.next_chunk};self.next_chunk=self.next_chunk.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Pack chunk ID overflow"))?;output.varint(id as u64,self.control)?;}Ok(())}else{output.byte(TAG_BYTES,self.control)?;output.varint(bytes.len()as u64,self.control)?;output.bytes(bytes,self.control)}
    }
    fn sequence(&mut self,items:&[FieldValue],shape:Option<&Shape>,tuple:bool,depth:u16,output:&mut Output)->Result<(),PackRefusal>{
        let mut kind=None;if !items.is_empty(){let mut floats=true;let mut ints=true;let mut enums=true;let mut uints=true;self.control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in items{floats&=matches!(item,FieldValue::Float(_));ints&=matches!(item,FieldValue::Int(_));enums&=matches!(item,FieldValue::Enum(_));uints&=matches!(item,FieldValue::UInt(value)if *value<=i64::MAX as u64);control.step()?;}Ok::<_,PackRefusal>(())})?;if floats{kind=Some(NumKind::F64)}else if ints||enums||uints{kind=Some(NumKind::Varint)}}
        output.byte(match &kind{Some(NumKind::F64)=>TAG_PACKED_F64,Some(NumKind::Varint)=>TAG_PACKED_VARINT,None=>if tuple{TAG_TUPLE}else{TAG_LIST}},self.control)?;output.varint(items.len()as u64,self.control)?;for item in items{self.control.checkpoint()?;match &kind{Some(NumKind::F64)=>{let FieldValue::Float(value)=item else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Pack numeric variant changed").into())};output.bytes(&value.to_le_bytes(),self.control)?;},Some(NumKind::Varint)=>{let value=match item{FieldValue::Int(value)=>*value,FieldValue::UInt(value)=>*value as i64,FieldValue::Enum(value)=>*value as i64,_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Pack numeric variant changed").into())};output.signed(value,self.control)?;},None=>self.value(shape,item,depth+1,output)?,}}Ok(())
    }
    fn dynamic(&mut self,value:&DslValue,depth:u16,output:&mut Output)->Result<(),PackRefusal>{
        let mut frames=Vec::new();let(mut value,mut depth)=(value,depth);
        'visit:loop{
            if depth>self.options.limits.max_depth{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Pack dynamic output depth exceeds declared limit").into())}self.control.checkpoint()?;
            match value{
                DslValue::Null=>output.byte(TAG_NULL,self.control)?,DslValue::Bool(value)=>output.byte(if *value{TAG_TRUE}else{TAG_FALSE},self.control)?,
                DslValue::Number(Number::UInt(value))=>{output.byte(TAG_UINT,self.control)?;output.varint(*value,self.control)?;},
                DslValue::Number(Number::Int(value))=>{output.byte(TAG_INT,self.control)?;output.signed(*value,self.control)?;},
                DslValue::Number(Number::Float(value))=>{output.byte(TAG_F64,self.control)?;output.bytes(&value.to_le_bytes(),self.control)?;},
                DslValue::String(text)=>self.text(text,false,output)?,DslValue::Bytes(bytes)=>self.bytes(bytes,output)?,
                DslValue::Array(items)=>{output.byte(TAG_LIST,self.control)?;output.varint(items.len()as u64,self.control)?;if !items.is_empty(){let depth=dynamic_child_depth(depth,self.options.limits.max_depth)?;push(&mut frames,DynamicFrame{items:DynamicItems::Array(items.iter()),depth},self.control)?;}},
                DslValue::Object(items)=>{output.byte(TAG_MAP,self.control)?;output.varint(items.len()as u64,self.control)?;if !items.is_empty(){let depth=dynamic_child_depth(depth,self.options.limits.max_depth)?;push(&mut frames,DynamicFrame{items:DynamicItems::Object(items.iter()),depth},self.control)?;}},
            }
            loop{let Some(frame)=frames.last_mut()else{return Ok(())};if let Some((key,child))=frame.next(){value=child;depth=frame.depth;if let Some(key)=key{self.text(key,true,output)?;}continue 'visit;}frames.pop();}
        }
    }
    fn node(&mut self,node:&WireNode,output:&mut Output)->Result<(),PackRefusal>{output.byte(u8::from(node.kind.is_some())|(u8::from(node.port.is_some())<<1),self.control)?;self.text(&node.id,false,output)?;if let Some(text)=&node.kind{self.text(text,false,output)?;}if let Some(text)=&node.port{self.text(text,false,output)?;}Ok(())}
    fn wire(&mut self,wire:&WireValue,depth:u16,output:&mut Output)->Result<(),PackRefusal>{let label=!wire.edge_label.is_empty();let mut presence=u8::from(wire.edge.is_some())|(u8::from(label)<<2);if wire.edge.as_ref().is_some_and(|(directed,_)|*directed){presence|=2;}output.byte(presence,self.control)?;self.node(&wire.from,output)?;if let Some((_,node))=&wire.edge{self.node(node,output)?;}if label{output.byte(u8::from(wire.edge_label.id.is_some())|(u8::from(wire.edge_label.kind.is_some())<<1),self.control)?;if let Some(text)=&wire.edge_label.id{self.text(text,false,output)?;}if let Some(text)=&wire.edge_label.kind{self.text(text,false,output)?;}}self.dynamic(&wire.properties,depth+1,output)}
    fn table(&mut self,producer:RecordSpecProducer,rows:&[FieldValue],depth:u16,output:&mut Output)->Result<(),PackRefusal>{
        let spec=producer.encode(self.control)?;let mut columns=self.control.allocate_vec(spec.fields.len())?;for field in spec.fields.iter().enumerate(){self.control.checkpoint()?;columns.push(field);}controlled_schema::sort(&mut columns,|a,b,_|Ok((a.1.id,a.0).cmp(&(b.1.id,b.0))),self.control)?;output.byte(TAG_TABLE_SOA,self.control)?;output.varint(rows.len()as u64,self.control)?;output.varint(columns.len()as u64,self.control)?;
        for(_,field)in columns{let at=|row|table_value(row,field.id);let mut dense=true;for row in rows{self.control.checkpoint()?;dense&=at(row).is_some();}output.varint(field.id as u64,self.control)?;output.byte(u8::from(!dense),self.control)?;if !dense{for group in rows.chunks(8){let mut byte=0;for(index,row)in group.iter().enumerate(){self.control.checkpoint()?;if at(row).is_some(){byte|=1<<index;}}output.byte(byte,self.control)?;}}
            let tag=elem_tag_for_shape(&field.shape);output.byte(tag,self.control)?;if tag==ELEM_BOOL{for group in rows.chunks(8){let mut byte=0;for(index,row)in group.iter().enumerate(){self.control.checkpoint()?;if matches!(at(row),Some(FieldValue::Bool(true))){byte|=1<<index;}}output.byte(byte,self.control)?;}continue;}
            for row in rows{self.control.checkpoint()?;let Some(value)=at(row)else{continue};match(tag,value){(ELEM_F64,FieldValue::Float(value))=>output.bytes(&value.to_le_bytes(),self.control)?,(ELEM_INT,FieldValue::Int(value))=>output.signed(*value,self.control)?,(ELEM_UINT,FieldValue::UInt(value))=>output.varint(*value,self.control)?,(ELEM_ENUM,FieldValue::Enum(value))=>output.varint(*value as u64,self.control)?,(ELEM_STR,FieldValue::Text(text))=>self.forced(text,output)?,(ELEM_F64|ELEM_INT|ELEM_UINT|ELEM_ENUM|ELEM_STR,_)=>{},_=>self.value(Some(&field.shape),value,depth+1,output)?,}}
        }Ok(())
    }
    fn symbols(&mut self,output:&mut Output)->Result<(),PackRefusal>{output.varint(self.symbols.entries.len()as u64,self.control)?;for symbol in &self.symbols.entries{self.control.checkpoint()?;output.varint(symbol.text.len()as u64,self.control)?;output.bytes(symbol.text.as_bytes(),self.control)?;}Ok(())}
    fn body(&mut self,spec:&RecordSpec,record:&RecordValue,output:&mut Output)->Result<(),PackRefusal>{self.symbols(output)?;self.fields(Some(spec),record,0,output)}
    fn value_body(&mut self,field_id:u16,value:&DslValue,output:&mut Output)->Result<(),PackRefusal>{self.symbols(output)?;output.varint(1,self.control)?;output.varint(u64::from(field_id),self.control)?;output.byte(TAG_VALUE,self.control)?;self.dynamic(value,0,output)}
}

pub(super) fn record_body(spec:&RecordSpec,record:&RecordValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());if control.owned_bytes()>maximum{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"Pack output admission exceeds max_total_alloc"})}
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut symbols=Symbols{entries:Vec::new()};symbols.record(Some(spec),record,0,options.limits.max_depth,control)?;symbols.finish(control)?;let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:false,next_chunk:0};let mut measure=Output::measure();encoder.body(spec,record,&mut measure)?;if measure.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Pack output exceeds caller allocation allowance").into())}let mut output=Output::allocated(measure.length,encoder.control)?;encoder.body(spec,record,&mut output)?;if output.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Pack output length changed between admission and emission").into())}Ok(output.bytes.unwrap())}))
}


pub(super) fn record_body_into(spec:&RecordSpec,record:&RecordValue,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut symbols=Symbols{entries:Vec::new()};
        symbols.record(Some(spec),record,0,options.limits.max_depth,control)?;
        symbols.finish(control)?;
        let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:false,next_chunk:0};
        let mut measure=Output::measure();
        encoder.body(spec,record,&mut measure)?;
        if measure.length as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"operation Record exceeds max_file_len"})}
        if measure.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"operation Record exceeds caller allocation allowance").into())}
        let mut output=Output{bytes:None,external:Some(output),length:0};
        encoder.body(spec,record,&mut output)?;
        if output.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"operation Record length changed between admission and emission").into())}
        Ok(output.length)
    }))
}

/// 🎞️ Borrows one complete intrinsic owner through symbol admission and exact byte emission.
pub(super) fn value_record_body(field_id:u16,value:&DslValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut symbols=Symbols{entries:Vec::new()};symbols.dynamic(value,0,options.limits.max_depth,control)?;symbols.finish(control)?;let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:false,next_chunk:0};let mut measure=Output::measure();encoder.value_body(field_id,value,&mut measure)?;if measure.length as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit, "intrinsic wire exceeds max_file_len")))}if measure.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic wire exceeds caller allocation allowance").into())}let mut output=Output::allocated(measure.length,encoder.control)?;encoder.value_body(field_id,value,&mut output)?;if output.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic wire changed after byte admission").into())}Ok(output.bytes.unwrap())}))
}

pub(super) fn value_record_body_into(field_id:u16,value:&DslValue,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{control.begin_stage(0)?;let mut symbols=Symbols{entries:Vec::new()};symbols.dynamic(value,0,options.limits.max_depth,control)?;symbols.finish(control)?;let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:false,next_chunk:0};let mut measure=Output::measure();encoder.value_body(field_id,value,&mut measure)?;if measure.length as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit, "intrinsic wire exceeds max_file_len")))}if measure.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic wire exceeds caller allocation allowance").into())}let mut output=Output{bytes:None,external:Some(output),length:0};encoder.value_body(field_id,value,&mut output)?;if output.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic wire changed after byte admission").into())}Ok(output.length)}))
}

enum DocumentSource<'a>{Record{spec:&'a RecordSpec,record:&'a RecordValue},Intrinsic{spec:&'a RecordSpec,field_id:u16,value:&'a DslValue}}
impl DocumentSource<'_>{
    fn spec(&self)->&RecordSpec{match self{Self::Record{spec,..}|Self::Intrinsic{spec,..}=>spec}}
    fn symbols<'a>(&'a self,symbols:&mut Symbols<'a>,maximum:u16,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{match self{Self::Record{spec,record}=>symbols.record(Some(spec),record,0,maximum,control),Self::Intrinsic{value,..}=>symbols.dynamic(value,0,maximum,control)}}
    fn fields(&self,encoder:&mut Encoder<'_,'_,'_,'_>,output:&mut Output)->Result<(),PackRefusal>{match self{Self::Record{spec,record}=>encoder.fields(Some(spec),record,0,output),Self::Intrinsic{field_id,value,..}=>{output.varint(1,encoder.control)?;output.varint(u64::from(*field_id),encoder.control)?;output.byte(TAG_VALUE,encoder.control)?;encoder.dynamic(value,0,output)}}}
    fn field_count(&self)->u64{match self{Self::Record{record,..}=>record.fields.values().filter(|value|!matches!(value,FieldValue::Absent)).count()as u64,Self::Intrinsic{..}=>1}}
}
pub(super) fn document(spec:&RecordSpec,record:&RecordValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{document_source(DocumentSource::Record{spec,record},options,control)}
pub(super) fn intrinsic_document(spec:&RecordSpec,field_id:u16,value:&DslValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{document_source(DocumentSource::Intrinsic{spec,field_id,value},options,control)}

fn document_source(source:DocumentSource<'_>,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{
        control.begin_stage(0)?;let mut symbols=Symbols{entries:Vec::new()};source.symbols(&mut symbols,options.limits.max_depth,control)?;symbols.finish(control)?;
        let symbol_bytes=|output:&mut Output,control:&mut NativeEncodeControl<'_>|->Result<(),PackRefusal>{output.varint(symbols.entries.len()as u64,control)?;for symbol in &symbols.entries{control.checkpoint()?;output.varint(symbol.text.len()as u64,control)?;output.bytes(symbol.text.as_bytes(),control)?;}Ok(())};
        let write=crate::format::WriteOptions{required_flags:0,optional_flags:if options.canonical{crate::format::OPTIONAL_CANONICAL}else{0},codec:options.codec};let mut writer=crate::format::ControlledPackWriter::begin(&write,&options.limits,control)?;
        let mut measure=Output::measure();symbol_bytes(&mut measure,control)?;let mut payload=Output::allocated(measure.length,control)?;symbol_bytes(&mut payload,control)?;writer.symbols(&payload.bytes.unwrap(),symbols.entries.len()as u64,control)?;
        let mut measure=Output::measure();{let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:true,next_chunk:0};source.fields(&mut encoder,&mut measure)?;}
        let mut payload=Output::allocated(measure.length,control)?;{let mut encoder=Encoder{symbols:&symbols,options,control,writer:Some(&mut writer),chunking:true,next_chunk:0};source.fields(&mut encoder,&mut payload)?;}
        if payload.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"Pack document size changed after admission").into())}let bytes=payload.bytes.unwrap();let start=writer.position();let size=usize::try_from(options.frame_size.max(1)).map_err(|_|ValueError::new(ValueRefusalKind::OwnershipLimit,"Pack frame size exceeds address space"))?;let frames=bytes.len().div_ceil(size);control.scoped_stage(|control|{control.begin_stage(frames)?;for frame in bytes.chunks(size){writer.segment(crate::KIND_DOCUMENT,frame,control)?;control.step()?;}Ok::<_,PackRefusal>(())})?;let end=writer.position();let hash=super::schema_hash_controlled(source.spec(),control)?;
        let manifest=crate::format::Manifest{schema_name:String::new(),schema_hash:hash,doc_span:crate::ByteRange{offset:start,len:end-start},doc_frame_count:frames as u64,symbols_span:crate::ByteRange{offset:0,len:0},chunk_table_span:crate::ByteRange{offset:0,len:0},field_index_span:crate::ByteRange{offset:0,len:0},uncompressed_body_len:bytes.len()as u64,field_count:source.field_count(),chunk_count:0,symbol_count:symbols.entries.len()as u64};writer.finish(&manifest,control)
    }))
}

#[cfg(test)]
#[path="../🧪️tests/🚦️refusals/🦀️.rs"]
mod refusal_tests;
