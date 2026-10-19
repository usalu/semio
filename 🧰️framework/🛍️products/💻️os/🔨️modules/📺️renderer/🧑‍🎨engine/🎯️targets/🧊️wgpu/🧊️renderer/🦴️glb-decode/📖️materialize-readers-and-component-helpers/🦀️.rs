//! 📖️ Materialize readers and component helpers: the bin span validation, the appearance build, the component readers, the asset hand-off, the close step and the texture and component helpers. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
impl GlbMaterializeCursor {
    pub(super) fn validate_bin_spans(&self) -> Result<(), &'static str> {
        for accessor in self.schema.accessors[..usize::from(self.schema.accessor_len)].iter().flatten() {
            let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB materializer lost an accessor view")?;
            let end = (view.byte_offset as usize).checked_add(view.byte_length as usize).ok_or("GLB bufferView BIN span overflowed")?;
            if end > self.bin_bytes {
                return Err("GLB bufferView exceeded the BIN payload");
            }
        }
        for image in self.schema.images[..usize::from(self.schema.image_len)].iter().flatten() {
            let view = self.schema.views.get(usize::from(image.view.ok_or("GLB image lost its embedded bufferView")?)).and_then(Option::as_ref).ok_or("GLB image referenced a missing bufferView")?;
            let end = (view.byte_offset as usize).checked_add(view.byte_length as usize).ok_or("GLB image bufferView BIN span overflowed")?;
            if end > self.bin_bytes {
                return Err("GLB image bufferView exceeded the BIN payload");
            }
        }
        Ok(())
    }

    pub(super) fn texture_key(&self, owner: &RendererAssetFetchOwner) -> String {
        format!("glb:{}:texture:{}:{}", owner.generation(), self.texture, if self.texture_role == 0 {"srgb"} else {"linear"})
    }

    pub(super) fn build_appearance(&mut self, owner: &RendererAssetFetchOwner) -> Result<World3dMeshAppearance, &'static str> {
        let mut primitives = Vec::new();
        primitives.try_reserve_exact(usize::from(self.plan.instance_len)).map_err(|_| "GLB material appearance exceeded allocation credits")?;
        for instance in self.plan.instances[..usize::from(self.plan.instance_len)].iter().flatten() {
            let primitive = self.schema.primitives.get(usize::from(instance.primitive)).and_then(Option::as_ref).ok_or("GLB materializer lost a primitive material")?;
            let material = primitive
                .material
                .and_then(|index| self.schema.materials.get(usize::from(index)).and_then(Option::as_ref).copied())
                .unwrap_or_default();
            let base_color_texture = material.base_color_texture.map(|texture| format!("glb:{}:texture:{texture}:srgb", owner.generation()));
            let sampler_for = |reference:Option<u16>| -> Result<ui_scene::math::SceneTextureSampler3d, &'static str> {Ok(if let Some(texture) = reference {
                let texture = self.schema.textures.get(usize::from(texture)).and_then(Option::as_ref).ok_or("GLB materializer lost a texture sampler owner")?;
                let sampler = texture.sampler.and_then(|index| self.schema.samplers.get(usize::from(index)).and_then(Option::as_ref).copied()).unwrap_or_default();
                ui_scene::math::SceneTextureSampler3d {
                    wrap_u: glb_texture_wrap(sampler.wrap_s)?,
                    wrap_v: glb_texture_wrap(sampler.wrap_t)?,
                    mag_filter: glb_mag_filter(sampler.mag_filter)?,
                    min_filter: glb_min_filter(sampler.min_filter)?,
                }
            } else {
                Default::default()
            })};
            let texture_sampler = sampler_for(material.base_color_texture)?;
            let additional_texture_samplers = [sampler_for(material.additional_textures[0])?,sampler_for(material.additional_textures[1])?,sampler_for(material.additional_textures[2])?,sampler_for(material.additional_textures[3])?];
            let reference = |role:usize| material.additional_textures[role].map(|texture|format!("glb:{}:texture:{texture}:{}",owner.generation(),if role==3 {"srgb"} else {"linear"}));
            primitives.push(World3dPrimitiveMaterial {
                first_index: instance.index_base,
                index_count: instance.index_count,
                material: ui_scene::math::SceneAuthoredMaterial3d {
                    base_color: material.base_color,
                    emissive: material.emissive,
                    metalness: material.metalness,
                    roughness: material.roughness,
                    alpha: match material.alpha_mode {
                        0 => ui_scene::math::SceneMaterialAlpha3d::Opaque,
                        1 => ui_scene::math::SceneMaterialAlpha3d::Mask,
                        2 => ui_scene::math::SceneMaterialAlpha3d::Blend,
                        _ => return Err("GLB materializer reached an unsupported alpha mode"),
                    },
                    alpha_cutoff: material.alpha_cutoff,
                    double_sided: material.double_sided,
                    preserve_vertex_color: primitive.color.is_some(),
                    base_color_texture,
                    texture_sampler,
                    metallic_roughness_texture:reference(0),normal_texture:reference(1),occlusion_texture:reference(2),emissive_texture:reference(3),additional_texture_samplers,
                normal_scale:[material.normal_scale,if primitive.tangent.is_some(){material.normal_scale}else{-material.normal_scale}],occlusion_strength:material.occlusion_strength,},
            });
        }
        let textures = std::mem::take(&mut self.textures);
        World3dMeshAppearance::new(primitives, textures).map_err(|(_primitives, textures)| {
            self.textures = textures;
            "GLB material appearance exceeded fixed owner credits"
        })
    }

    pub(super) fn read_vec(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: u16, item: u32, components: usize) -> Result<[f32; 4], &'static str> {
        let schema = self.schema.accessor(accessor)?;
        if usize::from(schema.kind) != components || item >= schema.count {
            return Err("GLB accessor semantic read exceeded its shape");
        }
        let mut result = [0.0; 4];
        for (component, output) in result[..components].iter_mut().enumerate() {
            *output = self.read_component(owner, pages, schema, item, component)?;
        }
        Ok(result)
    }

    fn read_component(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: &GlbAccessorSchema, item: u32, component: usize) -> Result<f32, &'static str> {
        let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor semantic read lost its bufferView")?;
        let component_bytes = glb_component_bytes(accessor.component)?;
        let item_bytes = usize::from(accessor.kind).checked_mul(component_bytes).ok_or("GLB accessor semantic item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        let relative = (view.byte_offset as usize)
            .checked_add(accessor.byte_offset as usize)
            .and_then(|offset| usize::try_from(item).ok().and_then(|item| item.checked_mul(stride)).and_then(|item| offset.checked_add(item)))
            .and_then(|offset| offset.checked_add(component * component_bytes))
            .ok_or("GLB accessor semantic address overflowed")?;
        let absolute = self.bin_start.checked_add(relative).ok_or("GLB accessor absolute address overflowed")?;
        glb_read_component(owner, pages, absolute, accessor.component, accessor.normalized)
    }

    /// 🎯️ One exact `u32` sub-element id; `read_component` widens through `f32` and would corrupt ids past 2^24.
    pub(super) fn read_unsigned_id(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: u16, item: u32) -> Result<u32, &'static str> {
        let schema = self.schema.accessor(accessor)?;
        if schema.component != 5125 || schema.kind != 1 || item >= schema.count {
            return Err("GLB sub-element id read exceeded its shape");
        }
        let view = self.schema.views.get(usize::from(schema.view)).and_then(Option::as_ref).ok_or("GLB sub-element id read lost its bufferView")?;
        let stride = if view.byte_stride == 0 { size_of::<u32>() } else { usize::from(view.byte_stride) };
        let relative = (view.byte_offset as usize)
            .checked_add(schema.byte_offset as usize)
            .and_then(|offset| usize::try_from(item).ok().and_then(|item| item.checked_mul(stride)).and_then(|item| offset.checked_add(item)))
            .ok_or("GLB sub-element id address overflowed")?;
        let absolute = self.bin_start.checked_add(relative).ok_or("GLB sub-element id absolute address overflowed")?;
        Ok(u32::from_le_bytes(pages.read::<4>(owner, absolute)?))
    }

    pub(super) fn read_output_index(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, primitive: &GlbPrimitiveSchema, output: u32, vertex_count: u32) -> Result<u32, &'static str> {
        let triangle = output / 3;
        let component = output % 3;
        let source = match (primitive.mode, component) {
            (4, component) => triangle * 3 + component,
            (5, component) if triangle.is_multiple_of(2) => triangle + component,
            (5, 0) => triangle + 1,
            (5, 1) => triangle,
            (5, 2) => triangle + 2,
            (6, 0) => 0,
            (6, 1) => triangle + 1,
            (6, 2) => triangle + 2,
            _ => return Err("GLB materializer reached an unsupported primitive mode"),
        };
        let result = if let Some(accessor) = primitive.indices {
            let accessor = self.schema.accessor(accessor)?;
            let value = self.read_component(owner, pages, accessor, source, 0)?;
            if value < 0.0 || value > u32::MAX as f32 || value.fract() != 0.0 {
                return Err("GLB index value was not an admitted integer");
            }
            value as u32
        } else {
            source
        };
        (result < vertex_count).then_some(result).ok_or("GLB index exceeded POSITION count")
    }

    pub(super) fn instance(&self) -> Result<GlbInstanceOutput, &'static str> {
        self.plan.instances.get(usize::from(self.instance)).and_then(Option::as_ref).copied().ok_or("GLB materializer lost an instance plan")
    }

    pub(super) fn primitive(&self, instance: GlbInstanceOutput) -> Result<&GlbPrimitiveSchema, &'static str> {
        self.schema.primitives.get(usize::from(instance.primitive)).and_then(Option::as_ref).ok_or("GLB materializer lost a primitive schema")
    }

    pub(super) fn write_token(&self) -> Result<Mesh3dWriteToken, &'static str> {
        self.write.ok_or("GLB materializer lost its mesh write claim")
    }

    pub fn take_asset(&mut self) -> Option<World3dMeshAsset> {
        Some(World3dMeshAsset { mesh: self.lease.take()?, appearance: self.appearance.take()? })
    }

    pub fn restore_asset(&mut self, asset: World3dMeshAsset) {
        assert!(self.lease.replace(asset.mesh).is_none(), "GLB publication restores exactly one retained mesh lease");
        assert!(self.appearance.replace(asset.appearance).is_none(), "GLB publication restores exactly one retained appearance owner");
    }

    pub(super) fn next_instance(&mut self) {
        self.instance += 1;
        self.item = 0;
        self.normal_substep = 0;
    }

    pub(super) fn advance_phase(&mut self, phase: GlbMaterializePhase) {
        self.phase = phase;
        self.instance = 0;
        self.item = 0;
        self.normal_substep = 0;
    }

    pub fn begin_close(&mut self) {
        self.texture_cancelled.store(true,Ordering::Release);
        self.phase = GlbMaterializePhase::Closing;
    }

    pub fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.texture_submission.take().is_some(){self.texture_worker=None;return false}
        if let Some(worker)=self.texture_worker.as_ref(){let Ok(mut output)=worker.try_lock()else{return false};let Some(image)=output.take()else{return false};self.decoded_texture=image.map(|image|DecodedReferenceImage{width:image.width,height:image.height,pixels:image.pixels,source_digest:self.texture_digest});drop(output);self.texture_worker=None;return false}
        if let Some(mut job)=self.texture_png.take(){job.cancel();return false}
        if let Some(image)=self.decoded_texture.as_mut(){if !image.pixels.is_empty(){image.pixels.truncate(image.pixels.len().saturating_sub(4096));return false}self.decoded_texture=None;return false}
        if !self.texture_bytes.is_empty(){self.texture_bytes.truncate(self.texture_bytes.len().saturating_sub(4096));return false}
        if let Some(appearance)=self.appearance.as_mut(){if !appearance.close_step(){return false}self.appearance=None;return false}
        if self.textures.pop().is_some(){return false}
        if let Some(token) = self.write {
            match mesh3d_abort(token) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.write = None;
                    return self.lease.is_none() && self.appearance.is_none() && self.textures.is_empty();
                }
                Err(_) => return false,
            }
            match mesh3d_abort_step(token) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.write = None,
                Ok(false) | Err(_) => return false,
            }
            return self.lease.is_none() && self.appearance.is_none() && self.textures.is_empty();
        }
        if let Some(lease) = self.lease {
            match mesh3d_begin_close(lease) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.lease = None;
                    return true;
                }
                Err(_) => return false,
            }
            match mesh3d_close_step(lease) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.lease = None,
                Ok(false) | Err(_) => return false,
            }
        }
        true
    }
}

pub(super) fn glb_texture_wrap(value: u16) -> Result<ui_scene::math::SceneTextureWrap3d, &'static str> {
    match value {
        33071 => Ok(ui_scene::math::SceneTextureWrap3d::ClampToEdge),
        10497 => Ok(ui_scene::math::SceneTextureWrap3d::Repeat),
        33648 => Ok(ui_scene::math::SceneTextureWrap3d::MirrorRepeat),
        _ => Err("GLB texture sampler used an unsupported wrap mode"),
    }
}

pub(super) fn glb_mag_filter(value: Option<u16>) -> Result<ui_scene::math::SceneTextureFilter3d, &'static str> {
    match value {
        None | Some(9729) => Ok(ui_scene::math::SceneTextureFilter3d::Linear),
        Some(9728) => Ok(ui_scene::math::SceneTextureFilter3d::Nearest),
        Some(_) => Err("GLB texture sampler used an unsupported magnification filter"),
    }
}

pub(super) fn glb_min_filter(value: Option<u16>) -> Result<ui_scene::math::SceneTextureFilter3d, &'static str> {
    match value {
        None => Ok(ui_scene::math::SceneTextureFilter3d::LinearMipmapLinear),
        Some(9729) => Ok(ui_scene::math::SceneTextureFilter3d::Linear),
        Some(9985)=>Ok(ui_scene::math::SceneTextureFilter3d::LinearMipmapNearest),
        Some(9987)=>Ok(ui_scene::math::SceneTextureFilter3d::LinearMipmapLinear),
        Some(9984)=>Ok(ui_scene::math::SceneTextureFilter3d::NearestMipmapNearest),
        Some(9986)=>Ok(ui_scene::math::SceneTextureFilter3d::NearestMipmapLinear),
        Some(9728) => Ok(ui_scene::math::SceneTextureFilter3d::Nearest),
        Some(_) => Err("GLB texture sampler used an unsupported minification filter"),
    }
}

pub(super) fn glb_mesh_fault(fault: Mesh3dFault) -> &'static str {
    match fault {
        Mesh3dFault::Closing => "GLB paged mesh authority was closing",
        Mesh3dFault::ItemCapacity => "GLB paged mesh authority exhausted fixed owner slots",
        Mesh3dFault::PageCapacity => "GLB paged mesh authority exhausted aggregate page credits",
        Mesh3dFault::ByteCapacity => "GLB paged mesh exceeded fixed byte credits",
        Mesh3dFault::Schema => "GLB paged mesh schema was invalid",
        Mesh3dFault::Stale => "GLB paged mesh generation/revision witness was stale",
        Mesh3dFault::Order => "GLB paged mesh field order was invalid",
        Mesh3dFault::Incomplete => "GLB paged mesh was not terminally complete",
    }
}

fn glb_component_bytes(component: u16) -> Result<usize, &'static str> {
    match component {
        5120 | 5121 => Ok(1),
        5122 | 5123 => Ok(2),
        5125 | 5126 => Ok(4),
        _ => Err("GLB component type was unsupported"),
    }
}

fn glb_read_component(owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, absolute: usize, component: u16, normalized: bool) -> Result<f32, &'static str> {
    let value = match component {
        5120 => {
            let value = i8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 127.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5121 => {
            let value = u8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                f32::from(value) / 255.0
            } else {
                f32::from(value)
            }
        }
        5122 => {
            let value = i16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 32767.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5123 => {
            let value = u16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                f32::from(value) / 65535.0
            } else {
                f32::from(value)
            }
        }
        5125 => {
            let value = u32::from_le_bytes(pages.read::<4>(owner, absolute)?);
            if normalized {
                value as f32 / u32::MAX as f32
            } else {
                value as f32
            }
        }
        5126 if !normalized => f32::from_le_bytes(pages.read::<4>(owner, absolute)?),
        5126 => return Err("GLB FLOAT accessor could not be normalized"),
        _ => return Err("GLB semantic component type was unsupported"),
    };
    value.is_finite().then_some(value).ok_or("GLB semantic component was not finite")
}
//#endregion 📡️RendererAssetAuthority
