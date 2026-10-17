//! 📤️ Interchange of the mesh pipeline: the resumable GLB interchange preparation and the bounded texture PNG encoding. See [`super`].

use super::*;

// #region 🔖️Interchange
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum InterchangePhase {
    AllocateNormals,
    AccumulateNormals,
    NormalizeNormals,
    Positions,
    Normals,
    Indices,
    Uvs,
    Texture,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct InterchangePreparation {
    pub(super) phase: InterchangePhase,
    pub(super) cursor: usize,
    accumulated_normals: Vec<[f64; 3]>,
    pub(super) mesh_data: semio_framework::MeshData,
    texture_png: Option<BoundedTexturePngPreparation>,
}

impl InterchangePreparation {
    pub(super) fn new(mesh: &TriMesh) -> Self {
        Self {
            phase: InterchangePhase::AllocateNormals,
            cursor: 0,
            accumulated_normals: Vec::with_capacity(mesh.positions.len()),
            mesh_data: semio_framework::MeshData { positions: Vec::with_capacity(mesh.positions.len() * 3), normals: Vec::with_capacity(mesh.positions.len() * 3), indices: Vec::with_capacity(mesh.triangles.len() * 3), ..Default::default() },
            texture_png: None,
        }
    }

    pub(super) fn advance(&mut self, mesh: &TriMesh, uvs: Option<&[[f32; 2]]>, texture: Option<&remodeling_image::ImageRgba8>, item_budget: usize) -> bool {
        let budget = item_budget.max(1);
        match self.phase {
            InterchangePhase::AllocateNormals => {
                let end = self.cursor.saturating_add(budget).min(mesh.positions.len());
                self.accumulated_normals.resize(end, [0.0; 3]);
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::AccumulateNormals;
                }
            }
            InterchangePhase::AccumulateNormals => {
                let end = self.cursor.saturating_add(budget).min(mesh.triangles.len());
                for triangle_index in self.cursor..end {
                    let triangle = mesh.triangles[triangle_index];
                    let normal = mesh.face_normal_unnormalized(triangle_index);
                    for vertex in triangle {
                        for (accumulated, coordinate) in self.accumulated_normals[vertex as usize].iter_mut().zip(normal) {
                            *accumulated += coordinate;
                        }
                    }
                }
                self.cursor = end;
                if end == mesh.triangles.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::NormalizeNormals;
                }
            }
            InterchangePhase::NormalizeNormals => {
                let end = self.cursor.saturating_add(budget).min(self.accumulated_normals.len());
                for normal in &mut self.accumulated_normals[self.cursor..end] {
                    *normal = normalize3(*normal);
                }
                self.cursor = end;
                if end == self.accumulated_normals.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Positions;
                }
            }
            InterchangePhase::Positions => {
                let end = self.cursor.saturating_add(budget).min(mesh.positions.len());
                for position in &mesh.positions[self.cursor..end] {
                    self.mesh_data.positions.extend(position.map(|coordinate| coordinate as f32));
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Normals;
                }
            }
            InterchangePhase::Normals => {
                let end = self.cursor.saturating_add(budget).min(self.accumulated_normals.len());
                for normal in &self.accumulated_normals[self.cursor..end] {
                    self.mesh_data.normals.extend(normal.map(|coordinate| coordinate as f32));
                }
                self.cursor = end;
                if end == self.accumulated_normals.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Indices;
                }
            }
            InterchangePhase::Indices => {
                let end = self.cursor.saturating_add(budget).min(mesh.triangles.len());
                for triangle in &mesh.triangles[self.cursor..end] {
                    self.mesh_data.indices.extend(*triangle);
                }
                self.cursor = end;
                if end == mesh.triangles.len() {
                    self.cursor = 0;
                    self.phase = InterchangePhase::Uvs;
                }
            }
            InterchangePhase::Uvs => {
                let values = uvs.unwrap_or_default();
                let end = self.cursor.saturating_add(budget).min(values.len());
                for uv in &values[self.cursor..end] {
                    self.mesh_data.uvs.extend(*uv);
                }
                self.cursor = end;
                if end == values.len() {
                    self.phase = InterchangePhase::Texture;
                }
            }
            InterchangePhase::Texture => {
                let Some(texture) = texture else {
                    self.phase = InterchangePhase::Done;
                    return true;
                };
                let preparation = self.texture_png.get_or_insert_with(|| BoundedTexturePngPreparation::new(texture));
                if preparation.advance(texture) {
                    self.mesh_data.paint_texture_base64 = self.texture_png.take().map(BoundedTexturePngPreparation::finish);
                    self.phase = InterchangePhase::Done;
                }
            }
            InterchangePhase::Done => {}
        }
        self.phase == InterchangePhase::Done
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTexturePngPhase {
    Header,
    Rows,
    Footer,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedTexturePngPreparation {
    pub(super) phase: BoundedTexturePngPhase,
    row: u32,
    adler_a: u32,
    adler_b: u32,
    base64: String,
    carry: Vec<u8>,
}

impl BoundedTexturePngPreparation {
    pub(super) fn new(image: &remodeling_image::ImageRgba8) -> Self {
        let encoded_len = (image.data.len() + image.height as usize * 6 + 96).div_ceil(3) * 4;
        Self { phase: BoundedTexturePngPhase::Header, row: 0, adler_a: 1, adler_b: 0, base64: String::with_capacity(encoded_len), carry: Vec::with_capacity(2) }
    }

    fn append_png_chunk(bytes: &mut Vec<u8>, kind: &[u8; 4], payload: &[u8]) {
        bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(kind);
        bytes.extend_from_slice(payload);
        let mut crc = 0xffff_ffffu32;
        for byte in kind.iter().chain(payload) {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xedb8_8320u32 & (0u32.wrapping_sub(crc & 1)));
            }
        }
        bytes.extend_from_slice(&(!crc).to_be_bytes());
    }

    fn append_stored_block(bytes: &mut Vec<u8>, final_block: bool, raw: &[u8]) {
        bytes.push(u8::from(final_block));
        let length = raw.len() as u16;
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(&(!length).to_le_bytes());
        bytes.extend_from_slice(raw);
    }

    fn append_base64(&mut self, bytes: &[u8], final_chunk: bool) {
        let mut pending = std::mem::take(&mut self.carry);
        pending.extend_from_slice(bytes);
        let encoded_end = if final_chunk { pending.len() } else { pending.len() / 3 * 3 };
        if encoded_end > 0 {
            self.base64.push_str(&base64_codec::base64_standard_encode(&pending[..encoded_end]));
        }
        if !final_chunk {
            self.carry.extend_from_slice(&pending[encoded_end..]);
        }
    }

    pub(super) fn advance(&mut self, image: &remodeling_image::ImageRgba8) -> bool {
        match self.phase {
            BoundedTexturePngPhase::Header => {
                let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
                let mut ihdr = Vec::with_capacity(13);
                ihdr.extend_from_slice(&image.width.to_be_bytes());
                ihdr.extend_from_slice(&image.height.to_be_bytes());
                ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
                Self::append_png_chunk(&mut bytes, b"IHDR", &ihdr);
                Self::append_png_chunk(&mut bytes, b"IDAT", &[0x78, 0x01]);
                self.append_base64(&bytes, false);
                self.phase = BoundedTexturePngPhase::Rows;
            }
            BoundedTexturePngPhase::Rows => {
                if self.row >= image.height {
                    self.phase = BoundedTexturePngPhase::Footer;
                    return false;
                }
                let start = self.row as usize * image.width as usize * 4;
                let end = start + image.width as usize * 4;
                let mut raw = Vec::with_capacity(end - start + 1);
                raw.push(0);
                raw.extend_from_slice(&image.data[start..end]);
                for byte in &raw {
                    self.adler_a = (self.adler_a + u32::from(*byte)) % 65_521;
                    self.adler_b = (self.adler_b + self.adler_a) % 65_521;
                }
                let mut deflate = Vec::with_capacity(raw.len() + 5);
                Self::append_stored_block(&mut deflate, false, &raw);
                let mut bytes = Vec::with_capacity(deflate.len() + 12);
                Self::append_png_chunk(&mut bytes, b"IDAT", &deflate);
                self.append_base64(&bytes, false);
                self.row += 1;
            }
            BoundedTexturePngPhase::Footer => {
                let mut deflate = Vec::with_capacity(9);
                Self::append_stored_block(&mut deflate, true, &[]);
                deflate.extend_from_slice(&((self.adler_b << 16) | self.adler_a).to_be_bytes());
                let mut bytes = Vec::with_capacity(33);
                Self::append_png_chunk(&mut bytes, b"IDAT", &deflate);
                Self::append_png_chunk(&mut bytes, b"IEND", &[]);
                self.append_base64(&bytes, true);
                self.phase = BoundedTexturePngPhase::Done;
            }
            BoundedTexturePngPhase::Done => {}
        }
        self.phase == BoundedTexturePngPhase::Done
    }

    pub(super) fn finish(self) -> String {
        self.base64
    }
}
// #endregion 🔖️Interchange
