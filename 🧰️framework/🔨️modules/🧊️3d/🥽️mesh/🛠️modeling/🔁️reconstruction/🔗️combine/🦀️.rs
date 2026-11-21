//! 🔗️ Joins two owned polygon sources while preserving their component identities.

use super::*;
use semio_framework_mesh_engine::{PolygonMeshSource,PolygonSurfaceKind,PolygonSurfaceSample};
use semio_framework_value::{DslValue,RetainedAllocationEffect,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneProgress,retained_clone_grant_admits};
use std::mem::size_of;
use semio_framework_3d_mesh_document::reserve_polygon_backing as reserve;

#[path="🎨️assets/🦀️.rs"]
mod assets;
use assets::Assets;

#[derive(semio_framework_value::RetireOwned)]
struct AttributeJoin {
    name:String, attribute:MeshAttribute, values:[usize;2], rows:[usize;2], cursor:usize, phase:u8,
}

/// 🧩️ Retains two polygon sources through exact allocation, offset and publication events.
#[derive(semio_framework_value::RetireOwned)]
pub struct PolygonCombine {
    left:PolygonMeshSource, right:PolygonMeshSource, output:PolygonMeshSource,
    corners:[usize;2], assets:Assets, attribute:Option<AttributeJoin>, rank:usize, cursor:usize, corner:usize, phase:u8, receipt:[usize;4],
}

fn invalid(reason:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvalidValue,reason)}

fn empty()->PolygonMeshSource {
    PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),surface:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()}
}


impl PolygonCombine {
    /// 🆕️ Validates fixed extents before accepting both owned sources.
    pub fn new(left:PolygonMeshSource,right:PolygonMeshSource)->Result<Self,(ValueError,PolygonMeshSource,PolygonMeshSource)> {
        let error=if left.vertices.len().saturating_add(right.vertices.len())>100_000 || left.faces.len().saturating_add(right.faces.len())>100_000 {
            Some(invalid("combined mesh exceeds the vertex or face limit"))
        }else if left.attributes.len()!=right.attributes.len() {
            Some(invalid("combined meshes require matching attribute names and definitions"))
        }else{None};
        if let Some(error)=error {return Err((error,left,right));}
        Ok(Self {left,right,output:empty(),corners:[0;2],assets:Assets::new(),attribute:None,rank:0,cursor:0,corner:0,phase:0,receipt:[0;4]})
    }

    fn channel(&self)->Result<Option<(&str,&MeshAttribute,&MeshAttribute)>,ValueError> {
        let Some((name,left))=self.left.attributes.ranked(self.rank) else {return Ok(None);};
        let (right_name,right)=self.right.attributes.ranked(self.rank).ok_or_else(||invalid("combined mesh lost a matching attribute"))?;
        if name!=right_name || left.domain!=right.domain || left.semantic!=right.semantic || left.interpolation!=right.interpolation {
            return Err(invalid("combined meshes require matching attribute names and definitions"));
        }
        let width=|attribute:&MeshAttribute|attribute.values.first().map(|sample|sample.as_array().map_or(0,<[DslValue]>::len));
        if left.interpolation==MeshAttributeInterpolation::Linear && width(left)!=width(right) {
            return Err(invalid("combined mesh linear attributes require compatible dimensions"));
        }
        if name.len()>512 || left.values.len().saturating_add(right.values.len())>600_000 {
            return Err(invalid("combined mesh attribute exceeds its limit"));
        }
        Ok(Some((name,left,right)))
    }

    /// 🧭️ Identifies the current retained combine phase.
    pub fn phase(&self)->&'static str {match self.phase {0|1=>"mesh-combine-corners",2..=4=>"mesh-combine-reserve",5=>"mesh-combine-vertices",6=>"mesh-combine-faces",7=>"mesh-combine-surface",8=>"mesh-combine-assets",_=>"mesh-combine-attributes"}}

    /// 📈️ Estimates progress from the retained join stages.
    pub fn progress(&self) -> f32 { f32::from(self.phase) / 10.0 }

    /// 📐️ Quotes one backing, scalar row or attribute publication.
    pub fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        let mut d=RetirementDemand {depth:1,..Default::default()};
        match self.phase {
            2=>d.capacity_bytes=(self.left.vertices.len()+self.right.vertices.len())*size_of::<[f32;3]>(),
            3=>d.capacity_bytes=(self.left.faces.len()+self.right.faces.len())*size_of::<Vec<u32>>(),
            4=>d.capacity_bytes=(self.left.surface.len()+self.right.surface.len())*size_of::<PolygonSurfaceSample>(),
            5 if self.cursor<self.left.vertices.len()+self.right.vertices.len()=>d.copy_bytes=size_of::<[f32;3]>(),
            6=>{
                let left=self.left.faces.len();
                if let Some(face)=self.left.faces.get(self.cursor).or_else(||self.right.faces.get(self.cursor.saturating_sub(left))) {
                    d.copy_bytes=if self.cursor>=left && self.corner<face.len(){size_of::<u32>()}else{size_of::<Vec<u32>>()};
                }
            },
            7 if self.cursor<self.left.surface.len()+self.right.surface.len()=>d.copy_bytes=size_of::<PolygonSurfaceSample>(),
            8=>return self.assets.demand(&self.right,&self.output,copy),
            9=>{
                if let Some(join)=&self.attribute {
                    match join.phase {
                        0=>d.capacity_bytes=join.values.iter().sum::<usize>()*size_of::<DslValue>(),
                        1=>d.capacity_bytes=join.rows.iter().sum::<usize>()*size_of::<u32>(),
                        2 if join.cursor<join.values.iter().sum::<usize>()=>d.copy_bytes=size_of::<DslValue>(),
                        3 if join.cursor<join.rows.iter().sum::<usize>()=>d.copy_bytes=size_of::<u32>(),
                        4=>{d.capacity_bytes=self.output.attributes.next_insert_capacity_byte_demand(&join.name,copy)?;d.depth=self.output.attributes.next_insert_depth_demand(&join.name)?;},
                        _=>{},
                    }
                }else if let Some((name,_,_))=self.channel()? {
                    d.capacity_bytes=name.len();d.copy_bytes=name.len();
                }
            },
            _=>{},
        }
        Ok(d)
    }

    /// 🧾️ Returns the physical progress from the most recent admitted event.
    pub fn take_progress(&mut self)->RetainedCloneProgress {
        let [copied_items,copied_bytes,retained_capacity_bytes,released_bytes]=std::mem::take(&mut self.receipt);
        RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes}
    }

    fn post(&mut self,receipt:RetainedCloneProgress) {self.receipt=[receipt.copied_items,receipt.copied_bytes,receipt.retained_capacity_bytes,receipt.released_bytes];}

    /// 🎟️ Advances one admitted event and returns the completed source once.
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PolygonMeshSource>,ValueError> {
        self.receipt=[0;4];
        let grant=effect.grant();
        let demand=self.demand(grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant,demand) || effect.remaining_work_units()?==0 {return Ok(None);}
        if !effect.performed(1)? {return Ok(None);}
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let result=(||{
            match self.phase {
                0|1=>{
                    let side=self.phase as usize;
                    let source=if side==0 {&self.left}else{&self.right};
                    if let Some(face)=source.faces.get(self.cursor) {
                        self.corners[side]=self.corners[side].saturating_add(face.len());
                        if self.corners[0].saturating_add(self.corners[1])>600_000 {return Err(invalid("combined mesh exceeds its polygon corner limit"));}
                        self.cursor+=1;
                    }else {self.cursor=0;self.phase+=1;}
                },
                2=>{receipt.retained_capacity_bytes=reserve(&mut self.output.vertices,self.left.vertices.len()+self.right.vertices.len(),effect)?;self.phase=3;}
                3=>{receipt.retained_capacity_bytes=reserve(&mut self.output.faces,self.left.faces.len()+self.right.faces.len(),effect)?;self.phase=4;}
                4=>{receipt.retained_capacity_bytes=reserve(&mut self.output.surface,self.left.surface.len()+self.right.surface.len(),effect)?;self.phase=5;}
                5=>{
                    let left=self.left.vertices.len();
                    if let Some(vertex)=self.left.vertices.get(self.cursor).or_else(||self.right.vertices.get(self.cursor.saturating_sub(left))) {
                        self.output.vertices.push(*vertex);self.cursor+=1;
                    }else{self.phase=6;self.cursor=0;}
                },
                6=>{
                    let left=self.left.faces.len();
                    let (faces,offset,adjust)=if self.cursor<left {(&mut self.left.faces,0,false)}else{(&mut self.right.faces,left,true)};
                    if let Some(face)=faces.get_mut(self.cursor-offset){
                        if adjust && self.corner<face.len() {face[self.corner]+=self.left.vertices.len() as u32;self.corner+=1;}
                        else{self.output.faces.push(std::mem::take(face));self.cursor+=1;self.corner=0;}
                    }else{self.phase=7;self.cursor=0;}
                },
                7=>{
                    let left=self.left.surface.len();
                    let (sample,adjust)=if self.cursor<left {(self.left.surface.get(self.cursor),false)}else{(self.right.surface.get(self.cursor-left),true)};
                    if let Some(sample)=sample {
                        let mut sample=*sample;
                        if adjust {sample.index+=match sample.kind {
                            PolygonSurfaceKind::VertexNormal=>self.left.vertices.len(),
                            PolygonSurfaceKind::SmoothFace=>self.left.faces.len(),
                            PolygonSurfaceKind::CornerUv|PolygonSurfaceKind::SeamEdge=>self.corners[0],
                        } as u32;}
                        self.output.surface.push(sample);self.cursor+=1;
                    }else{self.phase=8;self.cursor=0;}
                },
                8=>if self.assets.step(&mut self.left,&mut self.right,&mut self.output,effect,&mut receipt)? {self.phase=9;},
                9=>{
                    if self.step_attribute(demand,effect,&mut receipt)? {
                        self.phase=10;
                        return Ok(Some(std::mem::replace(&mut self.output,empty())));
                    }
                },
                _=>return Err(ValueError::invariant("combined polygon source already published")),
            }
            Ok(None)
        })();
        self.post(receipt);
        result
    }

    fn step_attribute(&mut self,demand:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError> {
        let Some(join)=&mut self.attribute else {
            let Some((name,left,right))=self.channel()? else {return Ok(true);};
            let mut key=String::new();
            key.try_reserve_exact(name.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"combine attribute name allocation refused"))?;
            key.push_str(name);
            *effect.actual()=Some(key.capacity());
            receipt.retained_capacity_bytes=key.capacity();
            let indexed=left.indices.is_some()||right.indices.is_some();
            let rows=if indexed {[left.domain_len(),right.domain_len()]}else{[0;2]};
            self.attribute=Some(AttributeJoin{name:key,attribute:MeshAttribute {domain:left.domain,semantic:left.semantic,interpolation:left.interpolation,values:Vec::new(),indices:indexed.then(Vec::new)},values:[left.values.len(),right.values.len()],rows,cursor:0,phase:0});
            return Ok(false);
        };
        match join.phase {
            0=>{receipt.retained_capacity_bytes=reserve(&mut join.attribute.values,join.values.iter().sum(),effect)?;join.phase=1;}
            1=>{if let Some(indices)=&mut join.attribute.indices {receipt.retained_capacity_bytes=reserve(indices,join.rows.iter().sum(),effect)?;}join.phase=2;}
            2=>{
                if join.cursor==join.values.iter().sum::<usize>(){join.phase=3;join.cursor=0;}
                else {
                    let (source,index)=if join.cursor<join.values[0] {(&mut self.left,join.cursor)}else{(&mut self.right,join.cursor-join.values[0])};
                    let from=source.attributes.get_mut(&join.name).ok_or_else(||ValueError::invariant("combine lost its original attribute"))?;
                    join.attribute.values.push(std::mem::replace(&mut from.values[index],DslValue::Null));
                    join.cursor+=1;
                }
            },
            3=>{
                if join.cursor==join.rows.iter().sum::<usize>(){join.phase=4;}
                else {
                    let (source,row,offset)=if join.cursor<join.rows[0] {(&self.left,join.cursor,0)}else{(&self.right,join.cursor-join.rows[0],join.values[0])};
                    let from=&source.attributes[&join.name];
                    let index=from.indices.as_ref().map_or(row as u32,|indices|indices[row]);
                    join.attribute.indices.as_mut().unwrap().push(index+offset as u32);
                    join.cursor+=1;
                }
            },
            4=>{
                let grant=effect.grant();
                if demand.capacity_bytes!=0 {
                    *receipt=match self.output.attributes.reserve_insert_step(&join.name,grant) {
                        Ok(progress)=>progress,
                        Err((error,progress))=>{*receipt=progress;return Err(error.with_retained_progress(progress));}
                    };
                }else {
                    let join=self.attribute.take().unwrap();
                    match self.output.attributes.insert_reserved(join.name,join.attribute,grant) {
                        Ok((old,progress))=>{assert!(old.is_none());*receipt=progress;self.rank+=1;}
                        Err((error,name,attribute))=>{self.attribute=Some(AttributeJoin{name,attribute,..join});return Err(error);}
                    }
                }
            },
            _=>unreachable!(),
        }
        Ok(false)
    }
}
