//! 🗃️ Schema output and cursor state: the default primitive schema, the schema section and output with its accessors and the schema cursor with its constructor and step. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
impl Default for GlbPrimitiveSchema {
    fn default() -> Self {
        Self { position: 0, normal: None, uv_sets: [None;64],tangent:None, color: None, indices: None, material: None, mode: 4, position_set: false, face_id: None, vertex_id: None }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum GlbSchemaSection {
    None,
    Accessors,
    Views,
    Meshes,
    Nodes,
    Scenes,
    Materials,
    Textures,
    Images,
    Samplers,
}

pub(super) struct GlbSchemaOutput {
    pub(super) accessors: Box<[Option<GlbAccessorSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) views: Box<[Option<GlbViewSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) primitives: Box<[Option<GlbPrimitiveSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) materials: Box<[Option<GlbMaterialSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) textures: Box<[Option<GlbTextureSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) images: Box<[Option<GlbImageSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) samplers: Box<[Option<GlbSamplerSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) meshes: Box<[Option<GlbMeshSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) nodes: Box<[Option<GlbNodeSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) scenes: Box<[Option<GlbSceneSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) node_edges: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) scene_roots: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) accessor_len: u16,
    pub(super) view_len: u16,
    pub(super) primitive_len: u16,
    pub(super) material_len: u16,
    pub(super) texture_len: u16,
    pub(super) image_len: u16,
    pub(super) sampler_len: u16,
    pub(super) mesh_len: u16,
    pub(super) node_len: u16,
    pub(super) scene_len: u16,
    pub(super) node_edge_len: u16,
    pub(super) scene_root_len: u16,
    pub(super) default_scene: Option<u16>,
    pub(super) json_bytes: u32,
    pub(super) output_bytes: usize,
}

impl GlbSchemaOutput {
    pub(super) fn new() -> Self {
        Self {
            accessors: semio_framework_async_slots::boxed_fixed_slots(|| None),
            views: semio_framework_async_slots::boxed_fixed_slots(|| None),
            primitives: semio_framework_async_slots::boxed_fixed_slots(|| None),
            materials: semio_framework_async_slots::boxed_fixed_slots(|| None),
            textures: semio_framework_async_slots::boxed_fixed_slots(|| None),
            images: semio_framework_async_slots::boxed_fixed_slots(|| None),
            samplers: semio_framework_async_slots::boxed_fixed_slots(|| None),
            meshes: semio_framework_async_slots::boxed_fixed_slots(|| None),
            nodes: semio_framework_async_slots::boxed_fixed_slots(|| None),
            scenes: semio_framework_async_slots::boxed_fixed_slots(|| None),
            node_edges: semio_framework_async_slots::boxed_fixed_slots(|| None),
            scene_roots: semio_framework_async_slots::boxed_fixed_slots(|| None),
            accessor_len: 0,
            view_len: 0,
            primitive_len: 0,
            material_len: 0,
            texture_len: 0,
            image_len: 0,
            sampler_len: 0,
            mesh_len: 0,
            node_len: 0,
            scene_len: 0,
            node_edge_len: 0,
            scene_root_len: 0,
            default_scene: None,
            json_bytes: 0,
            output_bytes: 0,
        }
    }

    fn validate(&mut self) -> Result<(), &'static str> {
        if let Some(scene) = self.default_scene {
            if usize::from(scene) >= usize::from(self.scene_len) {
                return Err("GLB default scene referenced a missing scene");
            }
        }
        for node in self.nodes[..usize::from(self.node_len)].iter().flatten() {
            if node.mesh.is_some_and(|mesh| usize::from(mesh) >= usize::from(self.mesh_len)) {
                return Err("GLB node referenced a missing mesh");
            }
            let start = usize::from(node.child_start);
            let end = start.checked_add(usize::from(node.child_len)).ok_or("GLB child range overflowed")?;
            if end > usize::from(self.node_edge_len) || self.node_edges[start..end].iter().flatten().any(|child| usize::from(*child) >= usize::from(self.node_len)) {
                return Err("GLB node referenced a missing child");
            }
        }
        for scene in self.scenes[..usize::from(self.scene_len)].iter().flatten() {
            let start = usize::from(scene.node_start);
            let end = start.checked_add(usize::from(scene.node_len)).ok_or("GLB scene root range overflowed")?;
            if end > usize::from(self.scene_root_len) || self.scene_roots[start..end].iter().flatten().any(|node| usize::from(*node) >= usize::from(self.node_len)) {
                return Err("GLB scene referenced a missing root node");
            }
        }
        let mut output_bytes = 0usize;
        for primitive in self.primitives[..usize::from(self.primitive_len)].iter().flatten() {
            if !matches!(primitive.mode, 4..=6) {
                continue;
            }
            let position = self.accessor(primitive.position)?;
            if primitive.material.is_some_and(|material| material >= self.material_len) {
                return Err("GLB primitive referenced a missing material");
            }
            if let Some(material)=primitive.material.and_then(|index|self.materials[usize::from(index)].as_ref()) {
                for (role,reference) in std::iter::once(material.base_color_texture).chain(material.additional_textures).enumerate() {
                    if reference.is_some() && primitive.uv_sets[usize::from(material.texture_coordinates[role])].is_none(){return Err("GLB texture references an absent UV set");}
                }
            }
            if position.component != 5126 || position.kind != 3 {
                return Err("GLB POSITION accessor was not FLOAT VEC3");
            }
            self.validate_accessor_span(position)?;
            let position_bytes = usize::try_from(position.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB vertex output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB vertex output bytes overflowed")?;
            if let Some(normal) = primitive.normal {
                let normal = self.accessor(normal)?;
                if !matches!(normal.component, 5126 | 5120 | 5122) || (normal.component != 5126 && !normal.normalized) || normal.kind != 3 || normal.count != position.count {
                    return Err("GLB NORMAL accessor did not match POSITION");
                }
                self.validate_accessor_span(normal)?;
                let normal_bytes = usize::try_from(normal.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB normal output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(normal_bytes).ok_or("GLB normal output bytes overflowed")?;
            } else {
                output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB generated normal bytes overflowed")?;
            }
            if let Some(tangent)=primitive.tangent {let tangent=self.accessor(tangent)?;if tangent.component!=5126 || tangent.kind!=4 || tangent.count!=position.count || primitive.normal.is_none(){return Err("GLB TANGENT must be FLOAT VEC4 aligned with POSITION and NORMAL");}self.validate_accessor_span(tangent)?;output_bytes=output_bytes.checked_add(tangent.count as usize*16).ok_or("GLB tangent output bytes overflowed")?;}
            for uv in primitive.uv_sets.into_iter().flatten() {
                let uv = self.accessor(uv)?;
                if !matches!(uv.component, 5126 | 5121 | 5123) || (uv.component != 5126 && !uv.normalized) || uv.kind != 2 || uv.count != position.count {
                    return Err("GLB TEXCOORD accessor did not match POSITION");
                }
                self.validate_accessor_span(uv)?;
                let uv_bytes = usize::try_from(uv.count).ok().and_then(|count| count.checked_mul(2)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB UV output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(uv_bytes).ok_or("GLB UV output bytes overflowed")?;
            }
            if let Some(color) = primitive.color {
                let color = self.accessor(color)?;
                if !matches!(color.kind, 3 | 4) || !matches!(color.component, 5121 | 5123 | 5126) || (color.component != 5126 && !color.normalized) || color.count != position.count {
                    return Err("GLB COLOR_0 accessor had an unsupported shape");
                }
                self.validate_accessor_span(color)?;
            }
            for (id, detail) in [(primitive.face_id, "GLB _FACE_ID accessor must be SCALAR UNSIGNED_INT aligned with POSITION"), (primitive.vertex_id, "GLB _VERTEX_ID accessor must be SCALAR UNSIGNED_INT aligned with POSITION")] {
                let Some(id) = id else { continue };
                let id = self.accessor(id)?;
                if id.component != 5125 || id.kind != 1 || id.normalized || id.count != position.count {
                    return Err(detail);
                }
                self.validate_accessor_span(id)?;
            }
            let source_indices = match primitive.indices {
                Some(indices) => {
                    let indices = self.accessor(indices)?;
                    if !matches!(indices.component, 5121 | 5123 | 5125) || indices.kind != 1 || indices.normalized {
                        return Err("GLB index accessor used an unsupported component or shape");
                    }
                    self.validate_accessor_span(indices)?;
                    indices.count
                }
                None => position.count,
            };
            let triangle_indices = match primitive.mode {
                4 => source_indices,
                5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
                _ => 0,
            };
            if triangle_indices == 0 || !triangle_indices.is_multiple_of(3) {
                return Err("GLB triangle primitive had no complete triangles");
            }
            let index_bytes = usize::try_from(triangle_indices).ok().and_then(|count| count.checked_mul(size_of::<u32>())).ok_or("GLB index output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(index_bytes).ok_or("GLB index output bytes overflowed")?;
            let id_bytes = (if primitive.face_id.is_some() { usize::try_from(triangle_indices / 3).ok() } else { Some(0) }).and_then(|triangles| if primitive.vertex_id.is_some() { usize::try_from(position.count).ok().and_then(|vertices| vertices.checked_add(triangles)) } else { Some(triangles) }).and_then(|count| count.checked_mul(size_of::<u32>())).ok_or("GLB sub-element id output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(id_bytes).ok_or("GLB sub-element id output bytes overflowed")?;
            if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
                return Err("GLB semantic output exceeded fixed byte credits");
            }
        }
        for material in self.materials[..usize::from(self.material_len)].iter().flatten() {
            if material.base_color.iter().chain(material.emissive.iter()).any(|value| !value.is_finite()) || !material.metalness.is_finite() || !material.roughness.is_finite() || !material.alpha_cutoff.is_finite() || !material.normal_scale.is_finite() || !material.occlusion_strength.is_finite() || !(0.0..=1.0).contains(&material.occlusion_strength) {
                return Err("GLB material contained a non-finite scalar");
            }
            if std::iter::once(material.base_color_texture).chain(material.additional_textures).flatten().any(|texture|texture >= self.texture_len) {
                return Err("GLB material referenced a missing texture");
            }
        }
        for texture in self.textures[..usize::from(self.texture_len)].iter().flatten() {
            if texture.source.is_none_or(|image| image >= self.image_len) || texture.sampler.is_some_and(|sampler| sampler >= self.sampler_len) {
                return Err("GLB texture referenced a missing image or sampler");
            }
        }
        for image in self.images[..usize::from(self.image_len)].iter().flatten() {
            if image.view.is_none_or(|view| view >= self.view_len) || image.mime == 0 {
                return Err("GLB image omitted an embedded bufferView or supported MIME type");
            }
        }
        for sampler in self.samplers[..usize::from(self.sampler_len)].iter().flatten() {
            glb_texture_wrap(sampler.wrap_s)?;
            glb_texture_wrap(sampler.wrap_t)?;
            glb_mag_filter(sampler.mag_filter)?;
            glb_min_filter(sampler.min_filter)?;
        }
        if output_bytes == 0 {
            return Err("GLB schema contained no triangle primitive output");
        }
        self.output_bytes = output_bytes;
        Ok(())
    }

    pub(super) fn accessor(&self, index: u16) -> Result<&GlbAccessorSchema, &'static str> {
        self.accessors.get(usize::from(index)).and_then(Option::as_ref).ok_or("GLB primitive referenced a missing accessor")
    }

    fn validate_accessor_span(&self, accessor: &GlbAccessorSchema) -> Result<(), &'static str> {
        if !accessor.view_set {
            return Err("GLB sparse or viewless accessor was unsupported");
        }
        let view = self.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor referenced a missing bufferView")?;
        let components = usize::from(accessor.kind);
        let component_bytes = match accessor.component {
            5120 | 5121 => 1,
            5122 | 5123 => 2,
            5125 | 5126 => 4,
            _ => return Err("GLB accessor component type was unsupported"),
        };
        let item_bytes = components.checked_mul(component_bytes).ok_or("GLB accessor item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        if stride < item_bytes {
            return Err("GLB bufferView stride was smaller than its accessor item");
        }
        let span = usize::try_from(accessor.count.saturating_sub(1)).ok().and_then(|count| count.checked_mul(stride)).and_then(|bytes| bytes.checked_add(item_bytes)).and_then(|bytes| bytes.checked_add(accessor.byte_offset as usize));
        if span.is_none_or(|span| span > view.byte_length as usize) {
            return Err("GLB accessor exceeded its bufferView");
        }
        Ok(())
    }
}

pub struct GlbSchemaCursor {
    pub(super) page: RendererAssetPageCursor,
    pub(super) header: [u8; 20],
    header_len: u8,
    json_remaining: u32,
    lexer: GlbJsonTokenCursor,
    pub(super) depth: u16,
    pub(super) section: GlbSchemaSection,
    pub(super) last_string: Option<GlbJsonAtom>,
    pub(super) pending_key: Option<GlbJsonAtom>,
    pub(super) expecting_value: bool,
    pub(super) current_accessor: Option<GlbAccessorSchema>,
    pub(super) current_view: Option<GlbViewSchema>,
    pub(super) current_primitive: Option<GlbPrimitiveSchema>,
    pub(super) current_mesh: Option<GlbMeshSchema>,
    pub(super) current_node: Option<GlbNodeSchema>,
    pub(super) current_scene: Option<GlbSceneSchema>,
    pub(super) current_material: Option<GlbMaterialSchema>,
    pub(super) current_texture: Option<GlbTextureSchema>,
    pub(super) current_image: Option<GlbImageSchema>,
    pub(super) current_sampler: Option<GlbSamplerSchema>,
    pub(super) numeric_array: Option<GlbNumericArray>,
    pub(super) primitives_depth: Option<u16>,
    pub(super) attributes_depth: Option<u16>,
    pub(super) pbr_depth: Option<u16>,
    pub(super) base_color_texture_depth: Option<u16>,
    pub(super) material_texture_role: usize,
    pub output: GlbSchemaOutput,
    terminal: bool,
}

impl GlbSchemaCursor {
    pub fn new() -> Self {
        Self {
            page: RendererAssetPageCursor::new(),
            header: [0; 20],
            header_len: 0,
            json_remaining: 0,
            lexer: GlbJsonTokenCursor::new(),
            depth: 0,
            section: GlbSchemaSection::None,
            last_string: None,
            pending_key: None,
            expecting_value: false,
            current_accessor: None,
            current_view: None,
            current_primitive: None,
            current_mesh: None,
            current_node: None,
            current_scene: None,
            current_material: None,
            current_texture: None,
            current_image: None,
            current_sampler: None,
            numeric_array: None,
            primitives_depth: None,
            attributes_depth: None,
            pbr_depth: None,
            base_color_texture_depth: None,
            material_texture_role: 0,
            output: GlbSchemaOutput::new(),
            terminal: false,
        }
    }

    pub fn step(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.header_len < 20 {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB semantic header was truncated") };
            self.header[usize::from(self.header_len)] = byte;
            self.header_len += 1;
            if self.header_len == 20 {
                if &self.header[..4] != b"glTF" || &self.header[16..20] != b"JSON" {
                    return Err("GLB semantic cursor did not find the JSON chunk");
                }
                self.json_remaining = u32::from_le_bytes(self.header[12..16].try_into().expect("fixed GLB JSON length"));
                self.output.json_bytes = self.json_remaining;
                if self.json_remaining == 0 || self.json_remaining as usize > owner.received_bytes().saturating_sub(20) {
                    return Err("GLB JSON chunk exceeded sealed input credits");
                }
            }
            return Ok(false);
        }
        if let Some(token) = self.lexer.replay()? {
            self.apply_token(token)?;
            return Ok(false);
        }
        if self.json_remaining == 0 {
            if let Some(token) = self.lexer.finish()? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.depth != 0
                || self.current_accessor.is_some()
                || self.current_view.is_some()
                || self.current_primitive.is_some()
                || self.current_mesh.is_some()
                || self.current_node.is_some()
                || self.current_scene.is_some()
                || self.current_material.is_some()
                || self.current_texture.is_some()
                || self.current_image.is_some()
                || self.current_sampler.is_some()
                || self.numeric_array.is_some()
            {
                return Err("GLB JSON schema ended with live nested ownership");
            }
            self.output.validate()?;
            self.terminal = true;
            return Ok(true);
        }
        for _ in 0..RENDERER_ASSET_PARSE_BLOCK_BYTES {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB JSON chunk was truncated") };
            self.json_remaining -= 1;
            if let Some(token) = self.lexer.feed(byte)? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.json_remaining == 0 {
                return Ok(false);
            }
        }
        Ok(false)
    }

}
//#endregion 📡️RendererAssetAuthority
