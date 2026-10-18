//! 🔖️ Mesh lease and cursors: the `Mesh3dLease` reads and retirement, the item and page cursors and the mesh items. See [`super`].

use super::*;

//#region Mesh
impl Mesh3dLease {
    /// 📃️ Lends one original initialized page while the existing native arena guard is held.
    pub fn with_original_page<R>(self, index: usize, read: impl FnOnce(&[u8]) -> R) -> Result<R, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        if index >= usize::from(owner.layout.page_count) { return Err(Mesh3dFault::Schema); }
        let page = owner.pages.as_ref().and_then(|pages| pages.get(index)).and_then(Option::as_ref).ok_or(Mesh3dFault::Incomplete)?;
        let initialized = owner.layout.total_bytes.saturating_sub(index * MESH3D_PAGE_BYTES).min(MESH3D_PAGE_BYTES);
        Ok(read(&page.bytes[..initialized]))
    }

    /// 📏️ Reads the original page census without allocating a transport mirror.
    pub fn original_page_count(self) -> Result<usize, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        Ok(usize::from(authority.ready(self)?.layout.page_count))
    }

    /// 📐️ Reads the initialized geometry extent of the original arena owner.
    pub fn original_geometry_bytes(self) -> Result<usize, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        Ok(authority.ready(self)?.layout.total_bytes)
    }

    /// 🧮️ Reads the three original topological label counts under the arena guard.
    pub fn original_component_counts(self) -> Result<[usize; 3], Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        Ok(std::array::from_fn(|domain| owner.component_references.as_ref().and_then(|table| table.get(["face", "edge", "vertex"][domain])).map_or(0, Vec::len)))
    }

    /// 🏷️ Lends one complete original label, including labels ineligible for numeric picking.
    pub fn with_original_component_label<R>(self, domain: usize, group: usize, read: impl FnOnce(&str) -> R) -> Result<R, Mesh3dFault> {
        let kind = ["face", "edge", "vertex"].get(domain).ok_or(Mesh3dFault::Schema)?;
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let label = owner.component_references.as_ref().and_then(|table| table.get(*kind)).and_then(|labels| labels.get(group)).ok_or(Mesh3dFault::Schema)?;
        Ok(read(label.as_str()))
    }

    /// 🔖️ Reads one original canonical label without a floating point conversion.
    pub fn component_label(self, field: Mesh3dField, group: u32) -> Result<Option<u64>, Mesh3dFault> {
        let (kind, domain) = match field { Mesh3dField::FaceIds => ("face", 0u64), Mesh3dField::EdgeIds => ("edge", 1u64), Mesh3dField::VertexIds => ("vertex", 2u64), _ => return Err(Mesh3dFault::Schema) };
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let Some(label) = owner.component_references.as_ref().and_then(|map| map.get(kind)).and_then(|labels| labels.get(group as usize)) else { return Ok(None); };
        if owner.component_ineligible.contains(&((domain << 32) | u64::from(group))) { return Err(Mesh3dFault::Schema); }
        mesh3d_component_number(label).map(Some).ok_or(Mesh3dFault::Schema)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }

    pub fn revision(self) -> u64 {
        self.revision
    }

    pub fn schema(self) -> Result<Mesh3dSchema, Mesh3dFault> {
        Ok(mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.schema)
    }

    pub fn aabb(self) -> Result<([f32; 3], [f32; 3]), Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        Ok((owner.aabb_min, owner.aabb_max))
    }

    pub fn cursor(self, field: Mesh3dField) -> Result<Mesh3dItemCursor, Mesh3dFault> {
        let len = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.schema.field_items(field);
        Ok(Mesh3dItemCursor { lease: self, field, index: 0, len })
    }

    pub fn page_cursor(self, field: Mesh3dField) -> Result<Mesh3dPageCursor, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let start = owner.layout.offsets[field.index()];
        let bytes = usize::try_from(owner.schema.field_items(field)).ok().and_then(|items| items.checked_mul(Mesh3dSchema::field_item_bytes(field))).ok_or(Mesh3dFault::ByteCapacity)?;
        Ok(Mesh3dPageCursor { lease: self, absolute: start, end: start + bytes })
    }

    pub fn vec2(self, field: Mesh3dField, item: u32) -> Result<[f32; 2], Mesh3dFault> {
        if !matches!(field,Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<8>(field, item)?;
        Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..].try_into().expect("fixed mesh scalar"))])
    }

    pub fn vec3(self, field: Mesh3dField, item: u32) -> Result<[f32; 3], Mesh3dFault> {
        if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<12>(field, item)?;
        Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
    }

    pub fn vec4(self, field: Mesh3dField, item: u32) -> Result<[f32; 4], Mesh3dFault> {
        if !matches!(field,Mesh3dField::Colors | Mesh3dField::Tangents) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<16>(field, item)?;
        Ok(std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"))))
    }

    pub fn edge(self, item: u32) -> Result<[[f32; 3]; 2], Mesh3dFault> {
        let bytes = self.read::<24>(Mesh3dField::Edges, item)?;
        let values: [f32; 6] = std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar")));
        Ok([[values[0], values[1], values[2]], [values[3], values[4], values[5]]])
    }

    pub fn u32(self, field: Mesh3dField, item: u32) -> Result<u32, Mesh3dFault> {
        if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
            return Err(Mesh3dFault::Schema);
        }
        Ok(u32::from_le_bytes(self.read::<4>(field, item)?))
    }

    pub(super) fn read<const N: usize>(self, field: Mesh3dField, item: u32) -> Result<[u8; N], Mesh3dFault> {
        mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.item_bytes(field, item)
    }
}

pub struct Mesh3dItemCursor {
    pub(super) lease: Mesh3dLease,
    pub(super) field: Mesh3dField,
    pub(super) index: u32,
    pub(super) len: u32,
}

pub struct Mesh3dPageCursor {
    pub(super) lease: Mesh3dLease,
    pub(super) absolute: usize,
    pub(super) end: usize,
}

impl Mesh3dPageCursor {
    pub fn next<R>(&mut self, read: impl FnOnce(&[u8]) -> R) -> Result<Option<R>, Mesh3dFault> {
        if self.absolute == self.end {
            return Ok(None);
        }
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self.lease)?;
        let page_index = self.absolute / MESH3D_PAGE_BYTES;
        let page_offset = self.absolute % MESH3D_PAGE_BYTES;
        let page = owner.pages.as_ref().and_then(|pages|pages.get(page_index)).and_then(Option::as_ref).ok_or(Mesh3dFault::Incomplete)?;
        let count = (MESH3D_PAGE_BYTES - page_offset).min(self.end - self.absolute);
        let result = read(&page.bytes[page_offset..page_offset + count]);
        self.absolute += count;
        Ok(Some(result))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mesh3dItem {
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Edge([[f32; 3]; 2]),
    U32(u32),
}

impl Mesh3dItemCursor {
    pub fn read_next(&mut self) -> Result<Option<Mesh3dItem>, Mesh3dFault> {
        if self.index == self.len {
            return Ok(None);
        }
        let bytes = match self.field {
            Mesh3dField::Positions | Mesh3dField::Normals => {
                let bytes = self.lease.read::<12>(self.field, self.index)?;
                Mesh3dItem::Vec3([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
            }
            Mesh3dField::Edges => {
                let bytes = self.lease.read::<24>(self.field, self.index)?;
                let mut values = [0.0; 6];
                for (index, value) in values.iter_mut().enumerate() {
                    *value = f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"));
                }
                Mesh3dItem::Edge([[values[0], values[1], values[2]], [values[3], values[4], values[5]]])
            }
            Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive => {
                let bytes = self.lease.read::<8>(self.field, self.index)?;
                Mesh3dItem::Vec2([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..].try_into().expect("fixed mesh scalar"))])
            }
            Mesh3dField::Colors | Mesh3dField::Tangents => {
                let bytes = self.lease.read::<16>(self.field, self.index)?;
                Mesh3dItem::Vec4(std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"))))
            }
            Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds => Mesh3dItem::U32(u32::from_le_bytes(self.lease.read::<4>(self.field, self.index)?)),
        };
        self.index += 1;
        Ok(Some(bytes))
    }
}
//#endregion Mesh
