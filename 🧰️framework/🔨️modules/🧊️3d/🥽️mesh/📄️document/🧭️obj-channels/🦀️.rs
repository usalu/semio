//! 🧭️ Authored polygon channels shared by the two OBJ writers.

use super::{EdgeId, HalfedgeMesh, MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshKernelError, MeshResult};

/// 🪡️ Resolves exported attribute identity in one canonical metadata record per step.
#[derive(Clone, Copy, Default)]
pub struct MeshObjChannels {
    cursor: usize,
    normal: Option<usize>,
    uv: Option<usize>,
    has_uv: bool,
    vertex_normals: Option<bool>,
}

impl MeshObjChannels {
    /// 🧩️ Advances the attribute identity search once, returning true after the final record.
    pub fn scan_attributes(&mut self, mesh: &HalfedgeMesh) -> bool {
        let Some((_,attribute))=mesh.attributes().ranked(self.cursor) else {return true};
        match attribute.semantic {
            MeshAttributeSemantic::Normal=>{self.normal.get_or_insert(self.cursor);}
            MeshAttributeSemantic::Uv=>{self.uv.get_or_insert(self.cursor);self.has_uv=true;}
            _=>{}
        }
        self.cursor+=1;
        false
    }

    /// 📐️ Detects complete built-in vertex normals as the position rows are read.
    pub fn scan_vertex_normal(&mut self,normal:Option<[f32;3]>) {
        let authored=normal.is_some_and(|normal|normal!=[0.0;3] && normal.into_iter().all(f32::is_finite));
        self.vertex_normals=Some(self.vertex_normals.unwrap_or(true) && authored);
    }

    /// 🪢️ Detects raw UVs one corner at a time.
    pub fn scan_corner(&mut self,mesh:&HalfedgeMesh,corner:EdgeId)->MeshResult<()> {
        let edge=mesh.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        self.has_uv|=edge.uv!=[0.0;2];
        Ok(())
    }

    /// 🧶️ Reports whether texture coordinates are authored on this OBJ mesh.
    pub fn has_uv(&self)->bool {self.has_uv}

    /// 🧭️ Reports whether the OBJ corner records carry an authored normal stream.
    pub fn has_normals(&self)->bool {self.normal.is_some() || self.vertex_normals==Some(true)}

    fn domain_index(domain:MeshAttributeDomain,edge:&super::HalfEdge,corner:u32)->MeshResult<u32> {
        Ok(match domain {
            MeshAttributeDomain::Vertex=>edge.vertex,
            MeshAttributeDomain::Face=>edge.face.ok_or(MeshKernelError::InvalidHandle)?,
            MeshAttributeDomain::Corner|MeshAttributeDomain::Edge=>corner,
        })
    }

    fn attribute<'a>(mesh:&'a HalfedgeMesh,rank:Option<usize>)->Option<&'a MeshAttribute> {
        rank.and_then(|rank|mesh.attributes().ranked(rank).map(|(_,attribute)|attribute))
    }

    /// 🧶️ Reads one authored or built-in texture coordinate pair for this corner.
    pub fn texture(&self,mesh:&HalfedgeMesh,corner:EdgeId)->MeshResult<[f32;2]> {
        let edge=mesh.halfedges.get(corner.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let Some(attribute)=Self::attribute(mesh,self.uv) else {return Ok(edge.uv);};
        let index=Self::domain_index(attribute.domain,edge,corner.0)?;
        let value=attribute.value_at(index as usize).and_then(semio_framework_value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;
        let component=|axis:usize|value.get(axis).and_then(semio_framework_value::DslValue::as_f64).filter(|value|value.is_finite()).ok_or(MeshKernelError::InvalidHandle);
        Ok([component(0)? as f32,component(1)? as f32])
    }

    /// 📏️ Counts the normal rows, retaining the authored index sharing across polygon corners.
    pub fn normal_count(&self,mesh:&HalfedgeMesh)->usize {
        if !self.has_normals() {return 0;}
        Self::attribute(mesh,self.normal).map_or(mesh.vertices.len(),|attribute|attribute.values.len())
    }

    /// 📌️ Reads one declared normal, using its original numeric values.
    pub fn normal(&self,mesh:&HalfedgeMesh,row:usize)->MeshResult<[f64;3]> {
        let Some(attribute)=Self::attribute(mesh,self.normal) else {
            let vertex=mesh.vertices.get(row).ok_or(MeshKernelError::InvalidHandle)?;
            return vertex.normal.map(|normal| normal.map(f64::from)).ok_or(MeshKernelError::InvalidHandle);
        };
        let normal=attribute.values.get(row).and_then(semio_framework_value::DslValue::as_array).ok_or(MeshKernelError::InvalidHandle)?;
        let component=|axis:usize|normal.get(axis).and_then(semio_framework_value::DslValue::as_f64).filter(|value|value.is_finite()).ok_or(MeshKernelError::InvalidHandle);
        Ok([component(0)?,component(1)?,component(2)?])
    }

    fn normal_index(&self,mesh:&HalfedgeMesh,corner:u32)->MeshResult<u32> {
        let edge=mesh.halfedges.get(corner as usize).ok_or(MeshKernelError::InvalidHandle)?;
        let Some(attribute)=Self::attribute(mesh,self.normal) else {return Ok(edge.vertex);};
        let index=Self::domain_index(attribute.domain,edge,corner)?;
        match &attribute.indices {
            Some(indices)=>indices.get(index as usize).copied().ok_or(MeshKernelError::InvalidHandle),
            None=>Ok(index),
        }
    }

    /// 🧵️ Writes one position/texture/normal reference using the OBJ slash grammar.
    pub fn write_corner(&self,mesh:&HalfedgeMesh,out:&mut String,vertex:u32,corner:u32)->MeshResult<()> {
        use std::fmt::Write;
        let normal=self.has_normals().then(||self.normal_index(mesh,corner)).transpose()?.map(|index|index+1);
        let vertex=vertex+1;
        let corner=corner+1;
        match (self.has_uv,normal) {
            (true,Some(normal))=>write!(out," {vertex}/{corner}/{normal}"),
            (true,None)=>write!(out," {vertex}/{corner}"),
            (false,Some(normal))=>write!(out," {vertex}//{normal}"),
            (false,None)=>write!(out," {vertex}"),
        }.expect("writing OBJ to a string");
        Ok(())
    }
}
