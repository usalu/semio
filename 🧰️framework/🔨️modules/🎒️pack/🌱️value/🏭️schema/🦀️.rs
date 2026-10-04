//! 🕸️ Controlled canonical schema ownership and incremental structural hashing.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::{PackSchemaField,PackSchemaGraph,PackSchemaShape};
use semio_framework_dsl_record::{NativeSchemaControl,RecordSpec,RecordSpecProducer,Shape};
use std::{cmp::Ordering,mem::size_of};
#[path="🗂️index/🦀️.rs"]
mod index;
use index::Index;

fn push<T,C:NativeSchemaControl>(values:&mut Vec<T>,value:T,control:&mut C)->Result<(),ValueError>{
    if values.len()==values.capacity(){let capacity=if values.capacity()==0{1}else{values.capacity().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema frontier capacity overflow"))?};let bytes=capacity.checked_mul(size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema frontier backing overflow"))?;control.charge(bytes)?;control.checkpoint()?;values.try_reserve_exact(capacity-values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"schema frontier allocation"))?;}
    values.push(value);Ok(())
}
pub(super) fn compare_text<C:NativeSchemaControl>(a:&str,b:&str,control:&mut C)->Result<Ordering,ValueError>{
    control.scoped_stage(|control|{let count=a.len().min(b.len());control.begin_stage(count)?;for offset in (0..count).step_by(65536){let end=(offset+65536).min(count);let order=a.as_bytes()[offset..end].cmp(&b.as_bytes()[offset..end]);control.advance(end-offset)?;if order!=Ordering::Equal{return Ok(order);}}Ok(a.len().cmp(&b.len()))})
}
pub(super) fn sort<T,C:NativeSchemaControl>(values:&mut[T],compare:impl Fn(&T,&T,&mut C)->Result<Ordering,ValueError>,control:&mut C)->Result<(),ValueError>{
    fn sift<T,C:NativeSchemaControl>(values:&mut[T],mut root:usize,end:usize,compare:&impl Fn(&T,&T,&mut C)->Result<Ordering,ValueError>,control:&mut C)->Result<(),ValueError>{
        loop{let Some(mut child)=root.checked_mul(2).and_then(|root|root.checked_add(1)).filter(|child|*child<end)else{return Ok(())};if child+1<end{control.step()?;if compare(&values[child],&values[child+1],control)?==Ordering::Less{child+=1;}}control.step()?;if compare(&values[root],&values[child],control)?!=Ordering::Less{return Ok(())}values.swap(root,child);root=child;}
    }
    control.scoped_stage(|control|{control.begin_stage(0)?;for root in (0..values.len()/2).rev(){sift(values,root,values.len(),&compare,control)?;}for end in (1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&compare,control)?;}Ok(())})
}
fn edge<C:NativeSchemaControl>(producer:RecordSpecProducer,edges:&mut Vec<RecordSpecProducer>,control:&mut C)->Result<u32,ValueError>{
    let index=u32::try_from(edges.len()).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"schema edge address exceeds u32"))?;push(edges,producer,control)?;Ok(index)
}
fn shape<C:NativeSchemaControl>(value:&Shape,edges:&mut Vec<RecordSpecProducer>,control:&mut C)->Result<PackSchemaShape,ValueError>{
    control.scoped_depth(64,|control|{control.step()?;Ok(match value{
        Shape::Bool=>PackSchemaShape::Bool,Shape::Int=>PackSchemaShape::Int,Shape::UInt=>PackSchemaShape::UInt,Shape::Float=>PackSchemaShape::Float,Shape::Text=>PackSchemaShape::Text,Shape::Bytes64=>PackSchemaShape::Bytes64,
        Shape::Enum(labels)=>{let mut output=control.allocate_vec(labels.len())?;for(key,id)in labels{output.push((*id,control.copy_text(key)?));}sort(&mut output,|a,b,control|{let order=a.0.cmp(&b.0);if order==Ordering::Equal{compare_text(&a.1,&b.1,control)}else{Ok(order)}},control)?;PackSchemaShape::Enum(output)},
        Shape::Tuple(inner,count)=>{control.charge(size_of::<PackSchemaShape>())?;PackSchemaShape::Tuple(Box::new(shape(inner,edges,control)?),count.map(|count|count as u64))},
        Shape::List(inner)=>{control.charge(size_of::<PackSchemaShape>())?;PackSchemaShape::List(Box::new(shape(inner,edges,control)?))},
        Shape::Block(inner)=>{control.charge(size_of::<PackSchemaShape>())?;PackSchemaShape::Block(Box::new(shape(inner,edges,control)?))},
        Shape::Map(inner)=>{control.charge(size_of::<PackSchemaShape>())?;PackSchemaShape::Map(Box::new(shape(inner,edges,control)?))},
        Shape::Record(producer)=>PackSchemaShape::Record(edge(*producer,edges,control)?),Shape::Table(producer)=>PackSchemaShape::Table(edge(*producer,edges,control)?),
        Shape::Statements(variants)=>{let mut ordered=control.allocate_vec(variants.len())?;for variant in variants.iter().enumerate(){ordered.push(variant);control.step()?;}sort(&mut ordered,|a,b,control|{let order=compare_text(&a.1.0,&b.1.0,control)?;Ok(if order==Ordering::Equal{a.0.cmp(&b.0)}else{order})},control)?;let mut output=control.allocate_vec(ordered.len())?;for(_,(key,producer))in ordered{output.push((control.copy_text(key)?,edge(*producer,edges,control)?));}PackSchemaShape::Statements(output)},
        Shape::Value=>PackSchemaShape::Value,Shape::Wire=>PackSchemaShape::Wire,Shape::Quantity(unit)=>PackSchemaShape::Quantity(control.copy_text(unit.symbol)?),Shape::Angle(unit)=>PackSchemaShape::Angle(control.copy_text(unit.symbol)?),Shape::Ref(kind)=>PackSchemaShape::Ref(control.copy_text(kind)?),Shape::Coord(n)=>PackSchemaShape::Coord(*n),Shape::Dir=>PackSchemaShape::Dir,Shape::Dim(n)=>PackSchemaShape::Dim(*n),Shape::Range=>PackSchemaShape::Range,Shape::Count=>PackSchemaShape::Count,Shape::Expr=>PackSchemaShape::Expr,Shape::Embed(lang)=>PackSchemaShape::Embed(control.copy_text(lang)?),Shape::EmbedFrom(key)=>PackSchemaShape::EmbedFrom(control.copy_text(key)?),
    })})
}
struct Writer<'a,C>{hasher:semio_framework_hash::Hasher,control:&'a mut C}
impl<'a,C:NativeSchemaControl> Writer<'a,C>{
    fn new(control:&'a mut C)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self{hasher:semio_framework_hash::Hasher::new(),control})}
    fn bytes(&mut self,bytes:&[u8])->Result<(),ValueError>{let hasher=&mut self.hasher;self.control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for chunk in bytes.chunks(65536){hasher.update(chunk);control.advance(chunk.len())?;}Ok(())})}
    fn varint(&mut self,mut number:u64)->Result<(),ValueError>{let mut bytes=[0u8;10];let mut count=0;loop{bytes[count]=(number as u8)&127;number>>=7;if number!=0{bytes[count]|=128;}count+=1;if number==0{break;}}self.bytes(&bytes[..count])}
    fn text(&mut self,text:&str)->Result<(),ValueError>{self.varint(text.len()as u64)?;self.bytes(text.as_bytes())}
    fn shape(&mut self,value:&PackSchemaShape,depth:usize)->Result<(),ValueError>{
        if depth>64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"schema hash shape depth exceeds limit"));}
        match value{
            PackSchemaShape::Bool=>self.bytes(&[1]),PackSchemaShape::Int=>self.bytes(&[2]),PackSchemaShape::UInt=>self.bytes(&[3]),PackSchemaShape::Float=>self.bytes(&[4]),PackSchemaShape::Text=>self.bytes(&[5]),PackSchemaShape::Bytes64=>self.bytes(&[6]),
            PackSchemaShape::Enum(labels)=>{self.bytes(&[7])?;self.varint(labels.len()as u64)?;for(id,key)in labels{self.varint(*id as u64)?;self.text(key)?;}Ok(())},
            PackSchemaShape::Tuple(inner,count)=>{self.bytes(&[8])?;self.shape(inner,depth+1)?;match count{Some(count)=>{self.bytes(&[1])?;self.varint(*count)},None=>self.bytes(&[0])}},
            PackSchemaShape::List(inner)=>{self.bytes(&[9])?;self.shape(inner,depth+1)},PackSchemaShape::Record(index)=>{self.bytes(&[10])?;self.varint(*index as u64)},PackSchemaShape::Block(inner)=>{self.bytes(&[11])?;self.shape(inner,depth+1)},
            PackSchemaShape::Statements(variants)=>{self.bytes(&[12])?;self.varint(variants.len()as u64)?;for(key,index)in variants{self.text(key)?;self.varint(*index as u64)?;}Ok(())},
            PackSchemaShape::Map(inner)=>{self.bytes(&[13])?;self.shape(inner,depth+1)},PackSchemaShape::Value=>self.bytes(&[14]),PackSchemaShape::Table(index)=>{self.bytes(&[15])?;self.varint(*index as u64)},PackSchemaShape::Wire=>self.bytes(&[16]),
            PackSchemaShape::Quantity(unit)=>{self.bytes(&[17])?;self.text(unit)},PackSchemaShape::Angle(unit)=>{self.bytes(&[18])?;self.text(unit)},PackSchemaShape::Ref(kind)=>{self.bytes(&[19])?;self.text(kind)},PackSchemaShape::Coord(n)=>self.bytes(&[20,*n]),PackSchemaShape::Dir=>self.bytes(&[21]),PackSchemaShape::Dim(n)=>self.bytes(&[22,*n]),PackSchemaShape::Range=>self.bytes(&[23]),PackSchemaShape::Count=>self.bytes(&[24]),PackSchemaShape::Expr=>self.bytes(&[25]),PackSchemaShape::Embed(lang)=>{self.bytes(&[26])?;self.text(lang)},PackSchemaShape::EmbedFrom(key)=>{self.bytes(&[27])?;self.text(key)},
        }
    }
    fn fields(&mut self,fields:&[PackSchemaField])->Result<(),ValueError>{self.varint(fields.len()as u64)?;for field in fields{self.varint(field.id as u64)?;self.text(&field.key)?;self.bytes(&[u8::from(field.optional)|(u8::from(field.flatten)<<1)])?;self.shape(&field.shape,0)?;}Ok(())}
    fn finish(self)->[u8;32]{*self.hasher.finalize().as_bytes()}
}
fn map_edges<C:NativeSchemaControl>(shape:&mut PackSchemaShape,map:&impl Fn(u32)->u32,control:&mut C)->Result<(),ValueError>{control.scoped_depth(64,|control|{control.step()?;match shape{PackSchemaShape::Tuple(inner,_)|PackSchemaShape::List(inner)|PackSchemaShape::Block(inner)|PackSchemaShape::Map(inner)=>map_edges(inner,map,control)?,PackSchemaShape::Record(index)|PackSchemaShape::Table(index)=>*index=map(*index),PackSchemaShape::Statements(variants)=>for(_,index)in variants{control.step()?;*index=map(*index);},_=>{}}Ok(())})}
fn equal_shape<C:NativeSchemaControl>(a:&PackSchemaShape,b:&PackSchemaShape,control:&mut C)->Result<bool,ValueError>{
    control.scoped_depth(64,|control|{control.step()?;Ok(match(a,b){
        (PackSchemaShape::Enum(a),PackSchemaShape::Enum(b))=>{if a.len()!=b.len(){false}else{let mut equal=true;for((id,a),(other,b))in a.iter().zip(b){if id!=other||compare_text(a,b,control)?!=Ordering::Equal{equal=false;break;}}equal}},
        (PackSchemaShape::Tuple(a,n),PackSchemaShape::Tuple(b,m))=>n==m&&equal_shape(a,b,control)?,
        (PackSchemaShape::List(a),PackSchemaShape::List(b))|(PackSchemaShape::Block(a),PackSchemaShape::Block(b))|(PackSchemaShape::Map(a),PackSchemaShape::Map(b))=>equal_shape(a,b,control)?,
        (PackSchemaShape::Statements(a),PackSchemaShape::Statements(b))=>{if a.len()!=b.len(){false}else{let mut equal=true;for((key,a),(other,b))in a.iter().zip(b){if a!=b||compare_text(key,other,control)?!=Ordering::Equal{equal=false;break;}}equal}},
        (PackSchemaShape::Quantity(a),PackSchemaShape::Quantity(b))|(PackSchemaShape::Angle(a),PackSchemaShape::Angle(b))|(PackSchemaShape::Ref(a),PackSchemaShape::Ref(b))|(PackSchemaShape::Embed(a),PackSchemaShape::Embed(b))|(PackSchemaShape::EmbedFrom(a),PackSchemaShape::EmbedFrom(b))=>compare_text(a,b,control)?==Ordering::Equal,
        _=>a==b,
    })})
}
fn equal_fields<C:NativeSchemaControl>(a:&[PackSchemaField],b:&[PackSchemaField],control:&mut C)->Result<bool,ValueError>{
    if a.len()!=b.len(){return Ok(false)}for(a,b)in a.iter().zip(b){control.step()?;if a.id!=b.id||a.optional!=b.optional||a.flatten!=b.flatten||compare_text(&a.key,&b.key,control)?!=Ordering::Equal||!equal_shape(&a.shape,&b.shape,control)?{return Ok(false)}}Ok(true)
}


pub(super) fn graph<C:NativeSchemaControl>(spec:&RecordSpec,control:&mut C)->Result<PackSchemaGraph,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(0)?;let mut nodes:Vec<(Vec<PackSchemaField>,Vec<usize>)>=Vec::new();let mut pending:Vec<RecordSpec>=Vec::new();let mut seen=Index::new();let mut cursor=0;
        loop{let current=if cursor==0{spec}else if let Some(next)=pending.get(cursor-1){next}else{break};let mut ordered=control.allocate_vec(current.fields.len())?;control.scoped_stage(|control|{control.begin_stage(current.fields.len())?;for field in current.fields.iter().enumerate(){ordered.push(field);control.step()?;}Ok(())})?;sort(&mut ordered,|a,b,_|Ok((a.1.id,a.0).cmp(&(b.1.id,b.0))),control)?;let mut edges=Vec::new();let mut fields=control.allocate_vec(ordered.len())?;for(_,field)in ordered{fields.push(PackSchemaField{id:field.id,key:control.copy_text(&field.key)?,optional:field.optional,flatten:field.flatten,shape:shape(&field.shape,&mut edges,control)?});}let mut targets=control.allocate_vec(edges.len())?;
            for producer in edges{control.step()?;let address=producer.ordinary as usize;let target=if let Some(target)=seen.lookup(&address,control)?{*target}else{let target=pending.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"schema record count overflow"))?;seen.bind(address,target,control)?;let next=control.produce(&producer)?;push(&mut pending,next,control)?;target};targets.push(target);}push(&mut nodes,(fields,targets),control)?;cursor+=1;
        }
        let mut buckets:Index<[u8;32],Vec<usize>>=Index::new();let mut class=control.allocate_vec(nodes.len())?;let mut count=0usize;
        for(index,(fields,_))in nodes.iter().enumerate(){control.step()?;let mut writer=Writer::new(control)?;writer.fields(fields)?;let digest=writer.finish();let mut found=None;if let Some(candidates)=buckets.lookup(&digest,control)?{for candidate in candidates{if equal_fields(fields,&nodes[*candidate].0,control)?{found=Some(class[*candidate]);break;}}}let label=if let Some(label)=found{label}else{let label=u32::try_from(count).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"schema class count exceeds u32"))?;count+=1;if buckets.lookup(&digest,control)?.is_none(){buckets.bind(digest,Vec::new(),control)?;}push(buckets.lookup_mut(&digest,control)?.expect("bound schema digest"),index,control)?;label};class.push(label);}
        loop{let mut signatures:Index<[u8;32],Vec<usize>>=Index::new();let mut refined=control.allocate_vec(nodes.len())?;let mut next_count=0usize;
            for(node,(_,targets))in nodes.iter().enumerate(){control.step()?;let mut writer=Writer::new(control)?;writer.varint(class[node]as u64)?;writer.varint(targets.len()as u64)?;for target in targets{writer.varint(class[*target]as u64)?;}let digest=writer.finish();let mut found=None;if let Some(candidates)=signatures.lookup(&digest,control)?{for candidate in candidates{control.step()?;if class[node]==class[*candidate]&&targets.len()==nodes[*candidate].1.len(){let mut equal=true;for(a,b)in targets.iter().zip(&nodes[*candidate].1){control.step()?;if class[*a]!=class[*b]{equal=false;break;}}if equal{found=Some(refined[*candidate]);break;}}}}let label=if let Some(label)=found{label}else{let label=u32::try_from(next_count).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"schema class count exceeds u32"))?;next_count+=1;if signatures.lookup(&digest,control)?.is_none(){signatures.bind(digest,Vec::new(),control)?;}push(signatures.lookup_mut(&digest,control)?.expect("bound refined schema digest"),node,control)?;label};refined.push(label);}
            let stable=next_count==count;count=next_count;class=refined;if stable{break;}
        }
        let mut ordinal=control.allocate_vec(count)?;ordinal.resize(count,None);let mut order=control.allocate_vec(count)?;let mut queue=control.allocate_vec(count)?;queue.push(0usize);ordinal[class[0]as usize]=Some(0u32);let mut cursor=0;
        while cursor<queue.len(){control.step()?;let node=queue[cursor];cursor+=1;order.push(node);for target in &nodes[node].1{control.step()?;let slot=&mut ordinal[class[*target]as usize];if slot.is_none(){*slot=Some(u32::try_from(queue.len()).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"schema ordinal exceeds u32"))?);queue.push(*target);}}}
        let mut records=control.allocate_vec(order.len())?;for node in order{control.step()?;let(fields,targets)=&mut nodes[node];let fields=std::mem::take(fields);let map=|edge:u32|ordinal[class[targets[edge as usize]]as usize].expect("reachable schema ordinal");let mut fields=fields;for field in &mut fields{control.step()?;map_edges(&mut field.shape,&map,control)?;}records.push(fields);}Ok(PackSchemaGraph{records})
    })
}

pub(super) fn hash_graph<C:NativeSchemaControl>(graph:&PackSchemaGraph,control:&mut C)->Result<[u8;32],ValueError>{
    let mut writer=Writer::new(control)?;writer.varint(graph.records.len()as u64)?;for fields in &graph.records{writer.fields(fields)?;}Ok(writer.finish())
}
pub(super) fn hash<C:NativeSchemaControl>(spec:&RecordSpec,control:&mut C)->Result<[u8;32],ValueError>{
    let graph=graph(spec,control)?;hash_graph(&graph,control)
}
