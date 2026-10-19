//! 🏁️ Schema finishing and plan types: the finish methods of every schema record, the numeric arrays, the exact index conversion and the instance output, node plan frame and plan cursor state. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
impl GlbSchemaCursor {
    pub(super) fn apply_literal(&mut self, value: Option<bool>) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("normalized") {
            self.current_accessor.as_mut().ok_or("GLB accessor normalized flag arrived without an owner")?.normalized = value.ok_or("GLB accessor normalized flag was null")?;
        } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.equals("doubleSided") {
            self.current_material.as_mut().ok_or("GLB material side flag arrived without an owner")?.double_sided = value.ok_or("GLB material side flag was null")?;
        }
        Ok(())
    }

    pub(super) fn apply_string(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("type") {
            self.current_accessor.as_mut().ok_or("GLB accessor type arrived without an owner")?.kind = if value.equals("SCALAR") {
                1
            } else if value.equals("VEC2") {
                2
            } else if value.equals("VEC3") {
                3
            } else if value.equals("VEC4") {
                4
            } else {
                return Err("GLB accessor type was unsupported");
            };
        } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.equals("alphaMode") {
            self.current_material.as_mut().ok_or("GLB material alpha mode arrived without an owner")?.alpha_mode = if value.equals("OPAQUE") { 0 } else if value.equals("MASK") { 1 } else if value.equals("BLEND") { 2 } else { return Err("GLB material alpha mode was unsupported") };
        } else if self.section == GlbSchemaSection::Images && self.depth == 3 && key.equals("mimeType") {
            self.current_image.as_mut().ok_or("GLB image MIME arrived without an owner")?.mime = if value.equals("image/png") { 1 } else if value.equals("image/jpeg") { 2 } else { return Err("GLB embedded image MIME type was unsupported") };
        }
        Ok(())
    }

    pub(super) fn consume_value(&mut self) {
        self.pending_key = None;
        self.expecting_value = false;
    }

    pub(super) fn finish_accessor(&mut self) -> Result<(), &'static str> {
        let accessor = self.current_accessor.take().ok_or("GLB accessor owner was missing")?;
        if accessor.component == 0 || accessor.count == 0 || accessor.kind == 0 {
            return Err("GLB accessor omitted required fixed fields");
        }
        let slot = usize::from(self.output.accessor_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB accessor count exceeded fixed item credits");
        }
        self.output.accessors[slot] = Some(accessor);
        self.output.accessor_len += 1;
        Ok(())
    }

    pub(super) fn finish_view(&mut self) -> Result<(), &'static str> {
        let view = self.current_view.take().ok_or("GLB bufferView owner was missing")?;
        if view.byte_length == 0 {
            return Err("GLB bufferView omitted its byte length");
        }
        let slot = usize::from(self.output.view_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB bufferView count exceeded fixed item credits");
        }
        self.output.views[slot] = Some(view);
        self.output.view_len += 1;
        Ok(())
    }

    pub(super) fn finish_primitive(&mut self) -> Result<(), &'static str> {
        let primitive = self.current_primitive.take().ok_or("GLB primitive owner was missing")?;
        if !primitive.position_set {
            return Err("GLB primitive omitted POSITION");
        }
        let slot = usize::from(self.output.primitive_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB primitive count exceeded fixed item credits");
        }
        self.output.primitives[slot] = Some(primitive);
        self.output.primitive_len += 1;
        let mesh = self.current_mesh.as_mut().ok_or("GLB primitive completed without a mesh owner")?;
        mesh.primitive_len = mesh.primitive_len.checked_add(1).ok_or("GLB mesh primitive count overflowed")?;
        Ok(())
    }

    pub(super) fn finish_mesh(&mut self) -> Result<(), &'static str> {
        let mesh = self.current_mesh.take().ok_or("GLB mesh owner was missing")?;
        if mesh.primitive_len == 0 {
            return Err("GLB mesh contained no primitive");
        }
        let slot = usize::from(self.output.mesh_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB mesh count exceeded fixed item credits");
        }
        self.output.meshes[slot] = Some(mesh);
        self.output.mesh_len += 1;
        Ok(())
    }

    pub(super) fn finish_node(&mut self) -> Result<(), &'static str> {
        let node = self.current_node.take().ok_or("GLB node owner was missing")?;
        let slot = usize::from(self.output.node_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node count exceeded fixed item credits");
        }
        self.output.nodes[slot] = Some(node);
        self.output.node_len += 1;
        Ok(())
    }

    pub(super) fn finish_scene(&mut self) -> Result<(), &'static str> {
        let scene = self.current_scene.take().ok_or("GLB scene owner was missing")?;
        let slot = usize::from(self.output.scene_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB scene count exceeded fixed item credits");
        }
        self.output.scenes[slot] = Some(scene);
        self.output.scene_len += 1;
        Ok(())
    }

    pub(super) fn finish_material(&mut self) -> Result<(), &'static str> {
        let material = self.current_material.take().ok_or("GLB material owner was missing")?;
        let slot = usize::from(self.output.material_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB material count exceeded fixed item credits") }
        self.output.materials[slot] = Some(material);
        self.output.material_len += 1;
        Ok(())
    }

    pub(super) fn finish_texture(&mut self) -> Result<(), &'static str> {
        let texture = self.current_texture.take().ok_or("GLB texture owner was missing")?;
        let slot = usize::from(self.output.texture_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB texture count exceeded fixed item credits") }
        self.output.textures[slot] = Some(texture);
        self.output.texture_len += 1;
        Ok(())
    }

    pub(super) fn finish_image(&mut self) -> Result<(), &'static str> {
        let image = self.current_image.take().ok_or("GLB image owner was missing")?;
        let slot = usize::from(self.output.image_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB image count exceeded fixed item credits") }
        self.output.images[slot] = Some(image);
        self.output.image_len += 1;
        Ok(())
    }

    pub(super) fn finish_sampler(&mut self) -> Result<(), &'static str> {
        let sampler = self.current_sampler.take().ok_or("GLB sampler owner was missing")?;
        let slot = usize::from(self.output.sampler_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB sampler count exceeded fixed item credits") }
        self.output.samplers[slot] = Some(sampler);
        self.output.sampler_len += 1;
        Ok(())
    }

    pub(super) fn finish_numeric_array(&mut self) -> Result<(), &'static str> {
        let array = self.numeric_array.take().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let node = self.current_node.as_mut().ok_or("GLB children completed without a node owner")?;
                node.child_start = array.start;
                node.child_len = array.len;
            }
            GlbNumericArrayKind::NodeTranslation => {
                if array.len != 3 {
                    return Err("GLB node translation did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB translation completed without a node owner")?.translation.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeRotation => {
                if array.len != 4 {
                    return Err("GLB node rotation did not contain four values");
                }
                self.current_node.as_mut().ok_or("GLB rotation completed without a node owner")?.rotation.copy_from_slice(&array.values[..4]);
            }
            GlbNumericArrayKind::NodeScale => {
                if array.len != 3 {
                    return Err("GLB node scale did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB scale completed without a node owner")?.scale.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeMatrix => {
                if array.len != 16 {
                    return Err("GLB node matrix did not contain sixteen values");
                }
                self.current_node.as_mut().ok_or("GLB matrix completed without a node owner")?.matrix = Some(array.values);
            }
            GlbNumericArrayKind::SceneNodes => {
                let scene = self.current_scene.as_mut().ok_or("GLB roots completed without a scene owner")?;
                scene.node_start = array.start;
                scene.node_len = array.len;
            }
            GlbNumericArrayKind::MaterialBaseColor => {
                if array.len != 4 { return Err("GLB material base color did not contain four values") }
                self.current_material.as_mut().ok_or("GLB base color completed without a material owner")?.base_color.copy_from_slice(&array.values[..4]);
            }
            GlbNumericArrayKind::MaterialEmissive => {
                if array.len != 3 { return Err("GLB material emissive did not contain three values") }
                self.current_material.as_mut().ok_or("GLB emissive completed without a material owner")?.emissive.copy_from_slice(&array.values[..3]);
            }
        }
        Ok(())
    }

    pub(super) fn push_numeric_array(&mut self, value: f32) -> Result<(), &'static str> {
        let array = self.numeric_array.as_mut().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let slot = usize::from(self.output.node_edge_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB child edges exceeded fixed item credits");
                }
                self.output.node_edges[slot] = Some(exact_u16(value, "GLB child index was not an admitted integer")?);
                self.output.node_edge_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB child count overflowed")?;
            }
            GlbNumericArrayKind::SceneNodes => {
                let slot = usize::from(self.output.scene_root_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB scene roots exceeded fixed item credits");
                }
                self.output.scene_roots[slot] = Some(exact_u16(value, "GLB scene node index was not an admitted integer")?);
                self.output.scene_root_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB scene root count overflowed")?;
            }
            _ => array.push(value)?,
        }
        Ok(())
    }
}

fn exact_u16(value: f32, detail: &'static str) -> Result<u16, &'static str> {
    if value < 0.0 || value > u16::MAX as f32 || value.fract() != 0.0 {
        return Err(detail);
    }
    Ok(value as u16)
}

pub type GlbMatrix = [[f32; 4]; 4];

#[derive(Clone, Copy)]
pub(super) struct GlbInstanceOutput {
    pub(super) primitive: u16,
    pub(super) matrix: GlbMatrix,
    pub(super) vertex_base: u32,
    pub(super) index_base: u32,
    pub(super) vertex_count: u32,
    pub(super) index_count: u32,
    pub(super) explicit_normals: bool,
}

#[derive(Clone, Copy)]
pub(super) struct GlbNodePlanFrame {
    pub(super) node: u16,
    pub(super) parent: GlbMatrix,
    pub(super) phase: u8,
    pub(super) item: u16,
}

pub(super) enum GlbPlanMode {
    Roots { scene: Option<u16>, index: u16 },
    Nodes { scene: Option<u16>, next_root: u16 },
    Fallback { primitive: u16 },
    Terminal,
}

pub(super) struct GlbInstancePlanCursor {
    pub(super) instances: Box<[Option<GlbInstanceOutput>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) instance_len: u16,
    pub(super) stack: Box<[Option<GlbNodePlanFrame>; GLB_SCHEMA_ITEM_CAPACITY]>,
    pub(super) stack_len: u16,
    pub(super) mode: GlbPlanMode,
    pub(super) vertex_count: u32,
    pub(super) index_count: u32,
    pub(super) has_uvs: bool,
    pub(super) has_tangents:bool,
    pub(super) has_colors: bool,
    pub(super) face_id_instances: u16,
    pub(super) vertex_id_instances: u16,
    pub(super) output_bytes: usize,
}
//#endregion 📡️RendererAssetAuthority
