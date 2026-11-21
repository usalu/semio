//! 🎨️ Copies polygon metadata one native key, value or backing event at a time.

use super::*;
use semio_framework_value::{DslValue,retirement::RetireOwned};

const PAGE:usize=256;

#[derive(Clone,Copy)]
pub(super) struct Remap<'a>{pub corners:&'a [u32],pub edges:&'a [u32],pub changed:[bool;2]}
impl Remap<'_>{
    fn domain(&self,domain:MeshAttributeDomain)->Option<(&[u32],bool)> {
        match domain {MeshAttributeDomain::Corner=>Some((self.corners,self.changed[0])),
            MeshAttributeDomain::Edge=>Some((self.edges,self.changed[1])),_=>None}
    }
}

pub(super) fn attribute_index(attribute:&MeshAttribute,id:u32)->Option<u32>{
    attribute.indices.as_ref().map_or(Some(id),|indices|indices.get(id as usize).copied())
}

fn text_end(text:&str,start:usize,copy:usize)->usize{
    if start==text.len(){return start;}
    let minimum=text[start..].chars().next().unwrap().len_utf8();
    let mut end=(start+copy.clamp(minimum,PAGE)).min(text.len());
    while !text.is_char_boundary(end){end-=1;}
    end
}

fn reserve_text(target:&mut String,len:usize,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<(),ValueError>{
    let mut buffer=Vec::new();reserved(&mut buffer,len,effect,receipt)?;
    *target=String::from_utf8(buffer).expect("empty reserved text is UTF-8");
    Ok(())
}

fn copy_text(source:&str,target:&mut String,copy:usize,receipt:&mut RetainedCloneProgress){
    let start=target.len();let end=text_end(source,start,copy);
    target.push_str(&source[start..end]);receipt.copied_bytes=end-start;
}

#[derive(semio_framework_value_derive::RetireOwned)]
struct Frame {value:DslValue,key:Option<String>}

#[derive(Default,semio_framework_value_derive::RetireOwned)]
struct ValueCapture {frames:Vec<Frame>,pending:Option<DslValue>}

enum ValueEvent<'a>{Grow,Begin(&'a DslValue),ReserveKey(&'a str),Key(&'a str),Text(&'a str),Bytes(&'a [u8]),Finish,Transfer}

impl ValueCapture {
    fn begin<'a>(&self,source:&'a DslValue)->ValueEvent<'a>{
        if self.frames.len()==self.frames.capacity(){ValueEvent::Grow}else{ValueEvent::Begin(source)}
    }

    fn event<'a>(&self,mut source:&'a DslValue)->Result<ValueEvent<'a>,ValueError>{
        if self.pending.is_some(){return Ok(ValueEvent::Transfer);}
        let Some((current,parents))=self.frames.split_last()else{return Ok(self.begin(source));};
        for parent in parents {
            source=match (&parent.value,source){
                (DslValue::Array(out),DslValue::Array(input))=>input.get(out.len()),
                (DslValue::Object(out),DslValue::Object(input))=>input.get(out.len()).map(|(_,child)|child),
                _=>None,
            }.ok_or_else(||absent("polygon metadata copy lost a parent path"))?;
        }
        Ok(match (&current.value,source){
            (DslValue::Array(out),DslValue::Array(input))=>input.get(out.len()).map_or(ValueEvent::Finish,|value|self.begin(value)),
            (DslValue::Object(out),DslValue::Object(input))=>{
                if let Some((key,value))=input.get(out.len()){
                    match &current.key {None=>ValueEvent::ReserveKey(key),Some(out) if out.len()<key.len()=>ValueEvent::Key(key),_=>self.begin(value)}
                }else{ValueEvent::Finish}
            },
            (DslValue::String(out),DslValue::String(input)) if out.len()<input.len()=>ValueEvent::Text(input),
            (DslValue::Bytes(out),DslValue::Bytes(input)) if out.len()<input.len()=>ValueEvent::Bytes(input),
            (DslValue::String(_),DslValue::String(_))|(DslValue::Bytes(_),DslValue::Bytes(_))=>ValueEvent::Finish,
            _=>return Err(absent("polygon metadata copy lost its source value")),
        })
    }

    fn demand(&self,source:&DslValue,copy:usize)->Result<RetirementDemand,ValueError>{
        let mut d=RetirementDemand{depth:self.frames.len()+1,..Default::default()};
        match self.event(source)? {
            ValueEvent::Grow=>{
                d.capacity_bytes=bytes::<Frame>(self.frames.capacity().checked_mul(2).ok_or_else(||absent("polygon metadata depth overflow"))?.max(4))?;
                d.copy_bytes=bytes::<Frame>(self.frames.len())?;
                d.release_bytes=bytes::<Frame>(self.frames.capacity())?;
            },
            ValueEvent::Begin(value)=>match value{
                DslValue::String(s)=>d.capacity_bytes=s.len(),
                DslValue::Bytes(s)=>d.capacity_bytes=s.len(),
                DslValue::Array(v)=>d.capacity_bytes=bytes::<DslValue>(v.len())?,
                DslValue::Object(v)=>d.capacity_bytes=bytes::<(String,DslValue)>(v.len())?,
                _=>d.copy_bytes=size_of::<DslValue>(),
            },
            ValueEvent::ReserveKey(key)=>d.capacity_bytes=key.len(),
            ValueEvent::Key(input)=>{
                let start=self.frames.last().unwrap().key.as_ref().unwrap().len();
                d.copy_bytes=text_end(input,start,copy)-start;
            },
            ValueEvent::Text(input)=>{
                let start=self.frames.last().unwrap().value.as_str().unwrap().len();
                d.copy_bytes=text_end(input,start,copy)-start;
            },
            ValueEvent::Bytes(input)=>{
                let start=self.frames.last().unwrap().value.as_bytes().unwrap().len();
                d.copy_bytes=(input.len()-start).min(copy.clamp(1,PAGE));
            },
            _=>{},
        }
        Ok(d)
    }

    fn step(&mut self,source:&DslValue,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<Option<DslValue>,ValueError>{
        let copy=effect.grant().maximum_copy_bytes;
        match self.event(source)? {
            ValueEvent::Grow=>{
                let demand=self.demand(source,copy)?;
                let count=demand.capacity_bytes/size_of::<Frame>();
                let result=self.frames.try_reserve_exact(count-self.frames.len());
                let capacity=self.frames.capacity()*size_of::<Frame>();
                *effect.actual()=Some(capacity);
                result.map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"polygon metadata frame allocation refused"))?;
                if capacity!=demand.capacity_bytes{return Err(absent("polygon metadata frame allocation differed from quote"));}
                receipt.copied_bytes=demand.copy_bytes;receipt.retained_capacity_bytes=capacity;receipt.released_bytes=demand.release_bytes;
            },
            ValueEvent::Begin(value)=>{
                let value=match value {
                    DslValue::String(text)=>{let mut out=String::new();reserve_text(&mut out,text.len(),effect,receipt)?;DslValue::String(out)},
                    DslValue::Bytes(input)=>{let mut out=Vec::new();reserved(&mut out,input.len(),effect,receipt)?;DslValue::Bytes(out)},
                    DslValue::Array(input)=>{let mut out=Vec::new();reserved(&mut out,input.len(),effect,receipt)?;DslValue::Array(out)},
                    DslValue::Object(input)=>{let mut out=Vec::new();reserved(&mut out,input.len(),effect,receipt)?;DslValue::Object(out)},
                    scalar=>{self.pending=Some(scalar.clone());receipt.copied_bytes=size_of::<DslValue>();return Ok(None);}
                };
                self.frames.push(Frame{value,key:None});
            },
            ValueEvent::ReserveKey(key)=>{
                let mut text=String::new();reserve_text(&mut text,key.len(),effect,receipt)?;
                self.frames.last_mut().unwrap().key=Some(text);
            },
            ValueEvent::Key(input)=>copy_text(input,self.frames.last_mut().unwrap().key.as_mut().unwrap(),copy,receipt),
            ValueEvent::Text(input)=>{
                let DslValue::String(out)=&mut self.frames.last_mut().unwrap().value else{unreachable!()};
                copy_text(input,out,copy,receipt);
            },
            ValueEvent::Bytes(input)=>{
                let DslValue::Bytes(out)=&mut self.frames.last_mut().unwrap().value else{unreachable!()};
                let start=out.len();let end=(start+copy.min(PAGE)).min(input.len());
                out.extend_from_slice(&input[start..end]);receipt.copied_bytes=end-start;
            },
            ValueEvent::Finish=>self.pending=self.frames.pop().map(|frame|frame.value),
            ValueEvent::Transfer=>{
                let value=self.pending.take().unwrap();
                if let Some(frame)=self.frames.last_mut(){
                    match &mut frame.value {
                        DslValue::Array(out)=>out.push(value),
                        DslValue::Object(out)=>out.push((frame.key.take().unwrap(),value)),
                        _=>unreachable!(),
                    }
                }else{return Ok(Some(value));}
            },
        }
        Ok(None)
    }
}

#[derive(Default,semio_framework_value_derive::RetireOwned)]
pub(super) struct Metadata {
    dynamic:ValueCapture,
    name:String,
    attribute:Option<MeshAttribute>,
    material:Option<DslValue>,
    texture:Option<MeshTexture>,
    slot:usize,cursor:usize,section:u8,phase:u8,
}

fn insertion<T:RetireOwned>(out:&HistoryFoldIndex<String,T>,key:&String,copy:usize)->Result<RetirementDemand,ValueError>{
    Ok(RetirementDemand{capacity_bytes:out.next_insert_capacity_byte_demand(key,copy)?,
        depth:out.next_insert_depth_demand(key)?,..Default::default()})
}

fn insert<T:RetireOwned>(out:&mut HistoryFoldIndex<String,T>,key:&mut String,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
    let grant=effect.grant();
    if out.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?!=0 {
        *receipt=out.reserve_insert_step(key,grant).map_err(|(error,progress)|error.with_retained_progress(progress))?;
        return Ok(false);
    }
    match out.insert_reserved(std::mem::take(key),value.take().unwrap(),grant) {
        Ok((None,progress))=>{*receipt=progress;Ok(true)},
        Ok((Some(_),_))=>Err(absent("duplicate polygon metadata name")),
        Err((error,name,pending))=>{*key=name;*value=Some(pending);Err(error)},
    }
}

impl Metadata {
    fn key<'a>(&self,mesh:&'a HalfedgeMesh)->Option<&'a String>{
        match self.section {
            0=>mesh.attributes.slot_entry(self.slot).map(|(key,_)|key),
            1=>mesh.materials.slot_entry(self.slot).map(|(key,_)|key),
            2=>mesh.textures.slot_entry(self.slot).map(|(key,_)|key),
            _=>None,
        }
    }

    fn slot_count(&self,mesh:&HalfedgeMesh)->usize{
        match self.section{0=>mesh.attributes.slot_count(),1=>mesh.materials.slot_count(),2=>mesh.textures.slot_count(),_=>0}
    }

    fn row_depth(&self,mesh:&HalfedgeMesh)->Result<usize,ValueError>{
        match self.section{
            0=>mesh.attributes.next_slot_depth_demand(self.slot),
            1=>mesh.materials.next_slot_depth_demand(self.slot),
            2=>mesh.textures.next_slot_depth_demand(self.slot),
            _=>Ok(0),
        }
    }

    fn index_count(attribute:&MeshAttribute,remap:Remap<'_>)->usize{
        remap.domain(attribute.domain).map_or_else(||attribute.indices.as_ref().map_or(0,Vec::len),|(ids,_)|ids.len())
    }

    pub(super) fn demand(&self,mesh:&HalfedgeMesh,output:&PolygonMeshSource,remap:Remap<'_>,copy:usize)->Result<RetirementDemand,ValueError>{
        let mut d=RetirementDemand{depth:1,..Default::default()};
        if self.slot>=self.slot_count(mesh){return Ok(d);}
        let Some(key)=self.key(mesh)else{d.depth=self.row_depth(mesh)?.max(1);return Ok(d);};
        if self.phase==0{d.capacity_bytes=key.len();}
        else if self.phase==1{d.copy_bytes=text_end(key,self.name.len(),copy)-self.name.len();}
        else if self.phase==6 {
            d=match self.section {0=>insertion(&output.attributes,&self.name,copy),1=>insertion(&output.materials,&self.name,copy),_=>insertion(&output.textures,&self.name,copy)}?;
        }else {
            match self.section{
                0=>{
                    let source=mesh.attributes.slot_entry(self.slot).unwrap().1;
                    match self.phase{
                        2=>d.capacity_bytes=bytes::<DslValue>(source.values.len())?,
                        3=>if let Some(value)=source.values.get(self.cursor){d=self.dynamic.demand(value,copy)?;},
                        4=>if self.attribute.as_ref().unwrap().indices.is_some(){d.capacity_bytes=bytes::<u32>(Self::index_count(source,remap))?;},
                        5=>if self.cursor<Self::index_count(source,remap){d.copy_bytes=size_of::<u32>();},
                        _=>unreachable!(),
                    }
                },
                1=>d=self.dynamic.demand(mesh.materials.slot_entry(self.slot).unwrap().1,copy)?,
                2=>{
                    let source=mesh.textures.slot_entry(self.slot).unwrap().1;
                    let target=self.texture.as_ref().unwrap();
                    match self.phase {
                        2=>d.capacity_bytes=source.mime.len(),
                        3=>d.copy_bytes=text_end(&source.mime,target.mime.len(),copy)-target.mime.len(),
                        4=>d.capacity_bytes=source.bytes.len(),
                        5=>d.copy_bytes=(source.bytes.len()-target.bytes.len()).min(copy.clamp(1,PAGE)),
                        _=>unreachable!(),
                    }
                },
                _=>unreachable!(),
            }
        }
        d.depth=d.depth.max(self.row_depth(mesh)?);
        Ok(d)
    }

    pub(super) fn step(&mut self,mesh:&HalfedgeMesh,out:&mut PolygonMeshSource,remap:Remap<'_>,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        if self.section==3{return Ok(true);}
        if self.slot==self.slot_count(mesh){self.slot=0;self.section+=1;return Ok(false);}
        let Some(key)=self.key(mesh)else{self.slot+=1;return Ok(false);};
        let copy=effect.grant().maximum_copy_bytes;
        match self.phase {
            0=>{reserve_text(&mut self.name,key.len(),effect,receipt)?;self.phase=1;},
            1=>{
                if self.name.len()<key.len(){copy_text(key,&mut self.name,copy,receipt);return Ok(false);}
                if self.section==0 {
                    let source=mesh.attributes.slot_entry(self.slot).unwrap().1;
                    let indexed=source.indices.is_some()||remap.domain(source.domain).is_some_and(|(_,changed)|changed);
                    self.attribute=Some(MeshAttribute{domain:source.domain,semantic:source.semantic,interpolation:source.interpolation,
                        values:Vec::new(),indices:indexed.then(Vec::new)});
                }else if self.section==2{self.texture=Some(MeshTexture{mime:String::new(),bytes:Vec::new()});}
                self.phase=2;self.cursor=0;
            },
            6=>{
                let inserted=match self.section {
                    0=>insert(&mut out.attributes,&mut self.name,&mut self.attribute,effect,receipt),
                    1=>insert(&mut out.materials,&mut self.name,&mut self.material,effect,receipt),
                    _=>insert(&mut out.textures,&mut self.name,&mut self.texture,effect,receipt),
                }?;
                if inserted{self.phase=0;self.slot+=1;}
            },
            _=>match self.section {
                0=>{
                    let source=mesh.attributes.slot_entry(self.slot).unwrap().1;
                    let target=self.attribute.as_mut().unwrap();
                    match self.phase{
                        2=>{reserved(&mut target.values,source.values.len(),effect,receipt)?;self.phase=3;},
                        3=>{
                            if let Some(value)=source.values.get(self.cursor){
                                if let Some(clone)=self.dynamic.step(value,effect,receipt)?{target.values.push(clone);self.cursor+=1;}
                            }else{self.phase=if target.indices.is_some(){4}else{6};self.cursor=0;}
                        },
                        4=>{reserved(target.indices.as_mut().unwrap(),Self::index_count(source,remap),effect,receipt)?;self.phase=5;},
                        5=>{
                            if self.cursor==Self::index_count(source,remap){self.phase=6;}
                            else{
                                let id=remap.domain(source.domain).map_or(self.cursor as u32,|(ids,_)|ids[self.cursor]);
                                let value=attribute_index(source,id).ok_or_else(||absent("polygon attribute index is absent"))?;
                                target.indices.as_mut().unwrap().push(value);self.cursor+=1;receipt.copied_bytes=size_of::<u32>();
                            }
                        },
                        _=>unreachable!(),
                    }
                },
                1=>{
                    let source=mesh.materials.slot_entry(self.slot).unwrap().1;
                    if let Some(clone)=self.dynamic.step(source,effect,receipt)?{self.material=Some(clone);self.phase=6;}
                },
                2=>{
                    let source=mesh.textures.slot_entry(self.slot).unwrap().1;
                    let target=self.texture.as_mut().unwrap();
                    match self.phase{
                        2=>{reserve_text(&mut target.mime,source.mime.len(),effect,receipt)?;self.phase=3;},
                        3=>if target.mime.len()==source.mime.len(){self.phase=4;}else{copy_text(&source.mime,&mut target.mime,copy,receipt);},
                        4=>{reserved(&mut target.bytes,source.bytes.len(),effect,receipt)?;self.phase=5;},
                        5=>{
                            let start=target.bytes.len();
                            if start==source.bytes.len(){self.phase=6;}
                            else{let end=(start+copy.min(PAGE)).min(source.bytes.len());target.bytes.extend_from_slice(&source.bytes[start..end]);receipt.copied_bytes=end-start;}
                        },
                        _=>unreachable!(),
                    }
                },
                _=>unreachable!(),
            },
        }
        Ok(false)
    }
}
