//! 📐️ Mesh schema, faults and lease types: the mesh capacities, the vertex fields and schema, the faults and the write token, lease and admission step types. See [`super`].

use super::*;

//#region Mesh
pub const MESH3D_AUTHORITY_CAPACITY: usize = 256;

pub const MESH3D_PAGE_BYTES: usize = 16 * 1024;

pub const MESH3D_OWNER_PAGE_CAPACITY: usize = 1_024;

pub const MESH3D_AUTHORITY_PAGE_CAPACITY: usize = 4_096;

pub const MESH3D_OWNER_BYTE_CAPACITY: usize = MESH3D_PAGE_BYTES * MESH3D_OWNER_PAGE_CAPACITY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mesh3dField {
    Positions,
    Normals,
    Indices,
    FaceIds,
    VertexIds,
    Edges,
    EdgeIds,
    Uvs,
    Colors,
    UvsMetallicRoughness,
    UvsNormal,
    UvsOcclusion,
    UvsEmissive,
    Tangents,
}

impl Mesh3dField {
    pub(super) fn index(self) -> usize {
        match self {
            Self::Positions => 0,
            Self::Normals => 1,
            Self::Indices => 2,
            Self::FaceIds => 3,
            Self::VertexIds => 4,
            Self::Edges => 5,
            Self::EdgeIds => 6,
            Self::Uvs => 7,
            Self::Colors => 8,
            Self::UvsMetallicRoughness=>9,Self::UvsNormal=>10,Self::UvsOcclusion=>11,Self::UvsEmissive=>12,Self::Tangents=>13,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesh3dSchema {
    pub vertices: u32,
    pub indices: u32,
    pub face_ids: u32,
    pub vertex_ids: u32,
    pub edges: u32,
    pub edge_ids: u32,
    pub uvs: u32,
    pub colors: u32,
    pub surface_uvs:[u32;4],
    pub tangents:u32,
}

impl Mesh3dSchema {
    pub fn triangle_mesh(vertices: u32, indices: u32) -> Self {
        Self { vertices, indices, face_ids: 0, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0,surface_uvs:[0;4],tangents:0 }
    }

    pub(super) fn field_items(self, field: Mesh3dField) -> u32 {
        match field {
            Mesh3dField::Positions | Mesh3dField::Normals => self.vertices,
            Mesh3dField::Indices => self.indices,
            Mesh3dField::FaceIds => self.face_ids,
            Mesh3dField::VertexIds => self.vertex_ids,
            Mesh3dField::Edges => self.edges,
            Mesh3dField::EdgeIds => self.edge_ids,
            Mesh3dField::Uvs => self.uvs,
            Mesh3dField::Colors => self.colors,Mesh3dField::Tangents=>self.tangents,
            Mesh3dField::UvsMetallicRoughness=>self.surface_uvs[0],Mesh3dField::UvsNormal=>self.surface_uvs[1],Mesh3dField::UvsOcclusion=>self.surface_uvs[2],Mesh3dField::UvsEmissive=>self.surface_uvs[3],
        }
    }

    pub(super) fn field_item_bytes(field: Mesh3dField) -> usize {
        match field {
            Mesh3dField::Positions | Mesh3dField::Normals => 12,
            Mesh3dField::Edges => 24,
            Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive => 8,
            Mesh3dField::Colors | Mesh3dField::Tangents => 16,
            Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds => 4,
        }
    }

    /// 🧮️ Refuses a schema whose field counts disagree or whose packed geometry exceeds the mesh owner byte capacity.
    pub fn validate(self) -> Result<(), Mesh3dFault> {
        self.layout().map(|_| ())
    }

    pub(super) fn layout(self) -> Result<Mesh3dLayout, Mesh3dFault> {
        if (self.vertices == 0 && (self.edges == 0 || self.indices != 0))
            || (self.indices == 0 && self.edges == 0 && self.vertex_ids == 0)
            || !self.indices.is_multiple_of(3)
            || (self.face_ids != 0 && self.face_ids != self.indices / 3)
            || (self.vertex_ids != 0 && self.vertex_ids != self.vertices)
            || (self.edge_ids != 0 && self.edge_ids != self.edges)
            || (self.uvs != 0 && self.uvs != self.vertices)
            || self.surface_uvs.iter().any(|count|*count!=0 && *count!=self.vertices)
            || (self.colors != 0 && self.colors != self.vertices)
            || (self.tangents != 0 && self.tangents != self.vertices)
        {
            return Err(Mesh3dFault::Schema);
        }
        let mut offsets = [0usize; 14];
        let mut total = 0usize;
        for field in [Mesh3dField::Positions, Mesh3dField::Normals, Mesh3dField::Indices, Mesh3dField::FaceIds, Mesh3dField::VertexIds, Mesh3dField::Edges, Mesh3dField::EdgeIds, Mesh3dField::Uvs, Mesh3dField::Colors,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive,Mesh3dField::Tangents] {
            offsets[field.index()] = total;
            let bytes = usize::try_from(self.field_items(field)).ok().and_then(|items| items.checked_mul(Self::field_item_bytes(field))).ok_or(Mesh3dFault::ByteCapacity)?;
            total = total.checked_add(bytes).ok_or(Mesh3dFault::ByteCapacity)?;
        }
        if total > MESH3D_OWNER_BYTE_CAPACITY {
            return Err(Mesh3dFault::ByteCapacity);
        }
        Ok(Mesh3dLayout { offsets, total_bytes: total, page_count: total.div_ceil(MESH3D_PAGE_BYTES) as u16 })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mesh3dFault {
    Busy,
    Closing,
    ItemCapacity,
    PageCapacity,
    ByteCapacity,
    Schema,
    Stale,
    Order,
    Incomplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesh3dWriteToken {
    pub(super) slot: u16,
    pub(super) epoch: u64,
    pub(super) generation: u64,
    pub(super) revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Mesh3dLease {
    pub(super) slot: u16,
    pub(super) epoch: u64,
    pub(super) generation: u64,
    pub(super) revision: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mesh3dComponentAdmissionStep {
    pub complete: bool,
    pub processed_items: usize,
    pub processed_bytes: usize,
    pub retired_items: usize,
    pub retired_bytes: usize,
    pub ownership: RetainedCloneProgress,
}

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct Mesh3dOwnershipStep { pub complete:bool,pub progress:RetainedCloneProgress }
//#endregion Mesh
