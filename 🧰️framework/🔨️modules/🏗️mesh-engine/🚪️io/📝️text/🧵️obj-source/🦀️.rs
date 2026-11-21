//! 🧵️ Indexed OBJ polygon intake and independent corner attribute projection.

use super::*;
use pack::value::{DslValue,RetainedAllocationEffect,RetirementDemand};
use pack::value::retained_clone::RetainedCloneProgress;
use std::mem::size_of;

const MAX_VERTICES:usize=100_000;
const MAX_CORNERS:usize=600_000;
const MAX_FACE_ROWS:usize=100_000;

#[derive(Copy,Clone)]
enum Buffer {
    Positions,Faces,Face,References,TextureValues,NormalValues,TextureIndices,NormalIndices,
}

struct Growth {
    buffer:Buffer,
    capacity:usize,
    demand:RetirementDemand,
}

impl Buffer {
    fn quote<T>(self,rows:&Vec<T>,limit:usize)->Option<Growth> {
        if rows.len()!=rows.capacity() || rows.len()==limit {return None;}
        let capacity=(rows.capacity()*2).max(4).min(limit);
        Some(Growth {buffer:self,capacity,demand:RetirementDemand {
            copy_bytes:rows.len()*size_of::<T>(),
            capacity_bytes:capacity*size_of::<T>(),
            release_bytes:rows.capacity()*size_of::<T>(),
            depth:1,
        }})
    }
}

fn grow<T>(rows:&mut Vec<T>,growth:Growth,effect:&mut RetainedAllocationEffect<'_>)->Result<[usize;3],String> {
    if effect.actual().is_some() {return Err("obj: allocation attempt is already occupied".into());}
    let mut replacement=Vec::new();
    let allocated=replacement.try_reserve_exact(growth.capacity);
    let bytes=replacement.capacity()*size_of::<T>();
    *effect.actual()=Some(bytes);
    allocated.map_err(|_|"obj: source capacity allocation refused")?;
    if bytes!=growth.demand.capacity_bytes {return Err("obj: source capacity differed from its quote".into());}
    replacement.append(rows);
    *rows=replacement;
    Ok([growth.demand.copy_bytes,bytes,growth.demand.release_bytes])
}


/// 📜️ Drives the same polygon-preserving OBJ cursor in synchronous authoring.
pub fn polygon_mesh_source_from_obj(text:&str)->Result<PolygonMeshSource,String> {
    let mut cursor=ObjSourceCursor::new();
    loop {
        if let Some(source)=cursor.step(text,4096)? {return Ok(source);}
    }
}

#[derive(Default,semio_framework_value_derive::RetireOwned)]
struct ObjChannel {
    values:Vec<DslValue>,
    indices:Vec<u32>,
    used:bool,
}

impl ObjChannel {
    fn append(&mut self,values:&[f64])->u32 {
        let index=self.values.len() as u32;
        self.values.push(DslValue::Array(values.iter().copied().map(DslValue::float).collect()));
        index
    }

    fn take(&mut self,semantic:MeshAttributeSemantic)->Option<MeshAttribute> {
        self.used.then(||MeshAttribute {
            domain:MeshAttributeDomain::Corner,semantic,interpolation:MeshAttributeInterpolation::Linear,
            values:std::mem::take(&mut self.values),indices:Some(std::mem::take(&mut self.indices)),
        })
    }
}

/// 🖱️ Reads bounded source events, then validates and projects one face corner per step.
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct ObjSourceCursor {
    offset:usize,
    lines:usize,
    token_start:usize,
    line:ObjLine,
    vertices:Vec<[f32;3]>,
    faces:Vec<Vec<u32>>,
    references:Vec<(Option<u32>,Option<u32>)>,
    texture:ObjChannel,
    normal:ObjChannel,
    declared_texture:usize,
    declared_normals:usize,
    face:usize,
    corner:usize,
    reference:usize,
    validate:usize,
    face_normal:[f64;3],
    missing_normal:bool,
    fallback_normal:Option<u32>,
    fallback_texture:Option<u32>,
    receipt:[usize;3],
}

#[derive(Default,semio_framework_value_derive::RetireOwned)]
enum ObjLine {
    #[default]
    Start,
    Coordinates(u8,[f64;3],usize),
    Face(Vec<u32>),
    Skip,
}

impl Default for ObjSourceCursor {fn default()->Self {Self::new()}}

impl ObjSourceCursor {
    pub fn new()->Self {
        Self {offset:0,lines:1,token_start:0,line:ObjLine::Start,vertices:Vec::new(),faces:Vec::new(),references:Vec::new(),
            texture:ObjChannel::default(),normal:ObjChannel::default(),declared_texture:0,declared_normals:0,
            face:0,corner:0,reference:0,validate:0,face_normal:[0.0;3],missing_normal:false,fallback_normal:None,fallback_texture:None,receipt:[0;3]}
    }

    /// 📊 Bytes consumed and total input bytes.
    pub fn progress(&self,text:&str)->(usize,usize) {(self.offset.min(text.len()),text.len())}

    fn next_growth(&self,text:&str)->Option<Growth> {
        use Buffer::*;
        if text.len()>16_000_000 {return None;}
        if self.offset<=text.len() {
            return match &self.line {
                ObjLine::Coordinates(0,..)=>Positions.quote(&self.vertices,MAX_VERTICES).filter(|_|self.lexical_frontier(text).1),
                ObjLine::Coordinates(1,..)=>TextureValues.quote(&self.texture.values,MAX_CORNERS).filter(|_|self.lexical_frontier(text).1),
                ObjLine::Coordinates(2,..)=>NormalValues.quote(&self.normal.values,MAX_CORNERS).filter(|_|self.lexical_frontier(text).1),
                ObjLine::Face(face)=>{
                    let token=Face.quote(face,MAX_CORNERS).or_else(||References.quote(&self.references,MAX_CORNERS));
                    let end=Faces.quote(&self.faces,MAX_FACE_ROWS);
                    if token.is_none() && end.is_none() {return None;}
                    let (has_token,has_end)=self.lexical_frontier(text);
                    token.filter(|_|has_token).or_else(||end.filter(|_|has_end))
                },
                _=>None,
            };
        }
        let face=self.faces.get(self.face)?;
        if self.validate<face.len() {return None;}
        if self.normal.used && self.missing_normal && self.fallback_normal.is_none() {
            return NormalValues.quote(&self.normal.values,MAX_CORNERS+MAX_FACE_ROWS);
        }
        if self.texture.used {
            if self.fallback_texture.is_none() && self.references[self.reference+self.corner].0.is_none() {
                if let Some(growth)=TextureValues.quote(&self.texture.values,MAX_CORNERS+1) {return Some(growth);}
            }
            if let Some(growth)=TextureIndices.quote(&self.texture.indices,MAX_CORNERS) {return Some(growth);}
        }
        if self.normal.used {return NormalIndices.quote(&self.normal.indices,MAX_CORNERS);}
        None
    }

    /// 📦️ Quotes the exact next geometry container replacement when its backing is full.
    pub fn normal_step_demand(&self,text:&str)->RetirementDemand {
        self.next_growth(text).map_or(RetirementDemand {depth:1,..Default::default()},|growth|growth.demand)
    }

    /// 🧾️ Returns the physical replacement actually charged on the preceding original turn.
    pub fn take_normal_progress(&mut self)->RetainedCloneProgress {
        let [copied_bytes,retained_capacity_bytes,released_bytes]=std::mem::take(&mut self.receipt);
        RetainedCloneProgress {copied_items:0,copied_bytes,retained_capacity_bytes,released_bytes}
    }

    /// 🎟️ Advances one OBJ event or one separately funded backing replacement.
    pub fn step_original(&mut self,text:&str,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PolygonMeshSource>,String> {
        self.receipt=[0;3];
        let remaining=effect.remaining_work_units().map_err(|error|error.to_string())?;
        if effect.grant().maximum_items==0 || remaining==0 {return Ok(None);}
        let growth=self.next_growth(text);
        if let Some(growth)=&growth {
            let grant=effect.grant();
            let demand=growth.demand;
            if grant.maximum_copy_bytes<demand.copy_bytes || grant.maximum_capacity_bytes<demand.capacity_bytes
                || grant.maximum_release_bytes<demand.release_bytes || grant.maximum_depth<demand.depth
            {return Ok(None);}
        }
        if !effect.performed(1).map_err(|error|error.to_string())? {return Ok(None);}
        if let Some(growth)=growth {
            self.receipt=match growth.buffer {
                Buffer::Positions=>grow(&mut self.vertices,growth,effect),
                Buffer::Faces=>grow(&mut self.faces,growth,effect),
                Buffer::Face=>match &mut self.line {ObjLine::Face(face)=>grow(face,growth,effect),_=>unreachable!()},
                Buffer::References=>grow(&mut self.references,growth,effect),
                Buffer::TextureValues=>grow(&mut self.texture.values,growth,effect),
                Buffer::NormalValues=>grow(&mut self.normal.values,growth,effect),
                Buffer::TextureIndices=>grow(&mut self.texture.indices,growth,effect),
                Buffer::NormalIndices=>grow(&mut self.normal.indices,growth,effect),
            }?;
            return Ok(None);
        }
        self.step(text,1)
    }

    /// ⏱️ Advances by at most the granted byte, topology or attribute events.
    pub fn step(&mut self,text:&str,maximum_units:usize)->Result<Option<PolygonMeshSource>,String> {
        if text.len()>16_000_000 {return Err("obj: input exceeds 16 MB".into());}
        for _ in 0..maximum_units {
            if self.offset<=text.len() {
                self.parse_chunk(text)?;
                if self.offset>text.len() {self.declared_texture=self.texture.values.len();self.declared_normals=self.normal.values.len();}
            }else if self.face<self.faces.len() {
                self.project_corner()?;
            }else {
                let mut attributes=HistoryFoldIndex::new();
                if let Some(uv)=self.texture.take(MeshAttributeSemantic::Uv) {attributes.insert("uv".into(),uv);}
                if let Some(normal)=self.normal.take(MeshAttributeSemantic::Normal) {attributes.insert("normal".into(),normal);}
                return Ok(Some(PolygonMeshSource {vertices:std::mem::take(&mut self.vertices),faces:std::mem::take(&mut self.faces),
                    surface:Vec::new(),attributes,materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()}));
            }
        }
        Ok(None)
    }

    fn lexical_frontier(&self,text:&str)->(bool,bool) {
        let mut start=self.token_start;
        let end=self.offset.saturating_add(256).min(text.len()+1);
        for at in self.offset..end {
            let byte=text.as_bytes().get(at).copied().unwrap_or(b'\n');
            let ends_line=byte==b'\n' || byte==b'#';
            if byte.is_ascii_whitespace() || byte==b'#' {
                if start<at || ends_line {return (start<at,ends_line);}
                start=at+1;
            }
        }
        (false,false)
    }

    fn parse_chunk(&mut self,text:&str)->Result<(),String> {
        let end=self.offset.saturating_add(256).min(text.len()+1);
        while self.offset<end {
            let byte=text.as_bytes().get(self.offset).copied().unwrap_or(b'\n');
            let mut token=false;
            if byte.is_ascii_whitespace() || byte==b'#' {
                if self.token_start<self.offset && !matches!(self.line,ObjLine::Skip) {
                    self.parse_token(&text[self.token_start..self.offset])?;
                    token=true;
                }
                self.token_start=self.offset+1;
                if byte==b'\n' || byte==b'#' {
                    self.finish_line()?;
                    if byte==b'#' {self.line=ObjLine::Skip;}else {self.lines+=1;}
                }
            }
            self.offset+=1;
            if token {break;}
        }
        Ok(())
    }

    fn parse_token(&mut self,token:&str)->Result<(),String> {
        let invalid=||format!("obj: malformed field on line {}",self.lines);
        match &mut self.line {
            ObjLine::Start=>self.line=match token {
                "v"=>ObjLine::Coordinates(0,[0.0;3],0),
                "vt"=>ObjLine::Coordinates(1,[0.0;3],0),
                "vn"=>ObjLine::Coordinates(2,[0.0;3],0),
                "f"=>ObjLine::Face(Vec::new()),
                _=>ObjLine::Skip,
            },
            ObjLine::Coordinates(kind,values,count)=>{
                if *count<if *kind==1 {2}else {3} {
                    values[*count]=(token.len()<=128).then(||token.parse::<f64>().ok()).flatten()
                        .filter(|value|value.is_finite() && (*kind!=0 || value.abs()<=f32::MAX as f64)).ok_or_else(invalid)?;
                    *count+=1;
                }
            }
            ObjLine::Face(face)=>{
                if token.len()>96 || self.references.len()>=MAX_CORNERS {return Err(invalid());}
                let mut fields=token.split('/');
                let position=Self::index(fields.next(),self.vertices.len())?.ok_or_else(invalid)?;
                let texture=Self::index(fields.next(),self.texture.values.len())?;
                let normal=Self::index(fields.next(),self.normal.values.len())?;
                if fields.next().is_some() {return Err(invalid());}
                self.texture.used|=texture.is_some();self.normal.used|=normal.is_some();
                face.push(position);
                self.references.push((texture,normal));
            }
            ObjLine::Skip=>{}
        }
        Ok(())
    }

    fn index(field:Option<&str>,count:usize)->Result<Option<u32>,String> {
        field.filter(|value|!value.is_empty()).map(|field|{
            let raw=field.parse::<i64>().map_err(|_|"obj: malformed face index")?;
            u32::try_from(obj_resolve_index(raw,count)?).map_err(|_|"obj: face index out of range".into())
        }).transpose()
    }

    fn finish_line(&mut self)->Result<(),String> {
        match std::mem::take(&mut self.line) {
            ObjLine::Coordinates(0,values,3) if self.vertices.len()<MAX_VERTICES=>self.vertices.push(values.map(|value|value as f32)),
            ObjLine::Coordinates(1,values,1..=2) if self.texture.values.len()<MAX_CORNERS=>{self.texture.append(&values[..2]);}
            ObjLine::Coordinates(2,values,3) if values!=[0.0;3] && self.normal.values.len()<MAX_CORNERS=>{self.normal.append(&values);}
            ObjLine::Face(face) if face.len()>=3 && self.faces.len()<MAX_FACE_ROWS=>self.faces.push(face),
            invalid @ (ObjLine::Coordinates(..)|ObjLine::Face(..))=>{
                self.line=invalid;
                return Err(format!("obj: malformed or oversized record on line {}",self.lines));
            }
            _=>{}
        }
        Ok(())
    }

    fn project_corner(&mut self)->Result<(),String> {
        let face=&self.faces[self.face];
        if self.validate<face.len() {
            let a=self.vertices.get(face[self.validate] as usize).ok_or("obj: face index out of range")?;
            let b=self.vertices.get(face[(self.validate+1)%face.len()] as usize).ok_or("obj: face index out of range")?;
            let (texture,normal)=self.references[self.reference+self.validate];
            if texture.is_some_and(|id|id as usize>=self.declared_texture) || normal.is_some_and(|id|id as usize>=self.declared_normals) {
                return Err("obj: texture or normal index out of range".into());
            }
            self.missing_normal|=normal.is_none();
            if self.normal.used {
                for (axis,(u,v)) in [(1,2),(2,0),(0,1)].into_iter().enumerate() {
                    self.face_normal[axis]+=(f64::from(a[u])-f64::from(b[u]))*(f64::from(a[v])+f64::from(b[v]));
                }
            }
            self.validate+=1;
            return Ok(());
        }
        if self.normal.used && self.missing_normal && self.fallback_normal.is_none() {
            let [x,y,z]=self.face_normal;
            let length=x.hypot(y).hypot(z);
            if length==0.0 {return Err("obj: cannot resolve missing normals on a degenerate face".into());}
            self.fallback_normal=Some(self.normal.append(&self.face_normal.map(|value|value/length)));
            return Ok(());
        }
        let (texture,normal)=self.references[self.reference+self.corner];
        if self.texture.used {
            let index=texture.unwrap_or_else(||*self.fallback_texture.get_or_insert_with(||self.texture.append(&[0.0;2])));
            self.texture.indices.push(index);
        }
        if self.normal.used {
            self.normal.indices.push(normal.or(self.fallback_normal).ok_or("obj: missing normal")?);
        }
        self.corner+=1;
        if self.corner==face.len() {
            self.reference+=face.len();
            self.face+=1;self.corner=0;self.validate=0;
            self.face_normal=[0.0;3];self.missing_normal=false;self.fallback_normal=None;
        }
        Ok(())
    }
}
