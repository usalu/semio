//! 🧮️ Schema token application: the token, unsigned, literal, string and value application of the schema cursor. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
impl GlbSchemaCursor {
    pub(super) fn apply_token(&mut self, token: GlbJsonToken) -> Result<(), &'static str> {
        match token {
            GlbJsonToken::ObjectStart => {
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.section == GlbSchemaSection::Accessors && self.depth == 2 {
                    self.current_accessor = Some(GlbAccessorSchema::default());
                } else if self.section == GlbSchemaSection::Views && self.depth == 2 {
                    self.current_view = Some(GlbViewSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 2 {
                    self.current_mesh = Some(GlbMeshSchema { primitive_start: self.output.primitive_len, primitive_len: 0 });
                } else if self.section == GlbSchemaSection::Meshes && self.primitives_depth == Some(self.depth) {
                    self.current_primitive = Some(GlbPrimitiveSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 5 && self.pending_key.as_ref().is_some_and(|key| key.equals("attributes")) {
                    self.attributes_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 2 {
                    self.current_node = Some(GlbNodeSchema::default());
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 2 {
                    self.current_scene = Some(GlbSceneSchema::default());
                } else if self.section == GlbSchemaSection::Materials && self.depth == 2 {
                    self.current_material = Some(GlbMaterialSchema::default());
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && self.pending_key.as_ref().is_some_and(|key| key.equals("pbrMetallicRoughness")) {
                    self.pbr_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Materials && self.pending_key.as_ref().is_some_and(|key| key.equals("baseColorTexture") || key.equals("metallicRoughnessTexture") || key.equals("normalTexture") || key.equals("occlusionTexture") || key.equals("emissiveTexture")) {
                    let key = self.pending_key.as_ref().unwrap();
                    self.material_texture_role = if key.equals("baseColorTexture") {0} else if key.equals("metallicRoughnessTexture") {1} else if key.equals("normalTexture") {2} else if key.equals("occlusionTexture") {3} else {4};
                    self.base_color_texture_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Textures && self.depth == 2 {
                    self.current_texture = Some(GlbTextureSchema::default());
                } else if self.section == GlbSchemaSection::Images && self.depth == 2 {
                    self.current_image = Some(GlbImageSchema::default());
                } else if self.section == GlbSchemaSection::Samplers && self.depth == 2 {
                    self.current_sampler = Some(GlbSamplerSchema::default());
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ObjectEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON object depth underflowed");
                }
                if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
                    self.finish_accessor()?;
                } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
                    self.finish_view()?;
                } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && self.primitives_depth == Some(self.depth - 1) {
                    self.finish_primitive()?;
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 {
                    self.finish_mesh()?;
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.finish_node()?;
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 {
                    self.finish_scene()?;
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 {
                    self.finish_material()?;
                } else if self.section == GlbSchemaSection::Textures && self.depth == 3 {
                    self.finish_texture()?;
                } else if self.section == GlbSchemaSection::Images && self.depth == 3 {
                    self.finish_image()?;
                } else if self.section == GlbSchemaSection::Samplers && self.depth == 3 {
                    self.finish_sampler()?;
                }
                if self.attributes_depth == Some(self.depth) {
                    self.attributes_depth = None;
                }
                if self.base_color_texture_depth == Some(self.depth) {
                    self.base_color_texture_depth = None;
                }
                if self.pbr_depth == Some(self.depth) {
                    self.pbr_depth = None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::ArrayStart => {
                let key = self.pending_key;
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.depth == 1 {
                    self.section = if key.as_ref().is_some_and(|key| key.equals("accessors")) {
                        GlbSchemaSection::Accessors
                    } else if key.as_ref().is_some_and(|key| key.equals("bufferViews")) {
                        GlbSchemaSection::Views
                    } else if key.as_ref().is_some_and(|key| key.equals("meshes")) {
                        GlbSchemaSection::Meshes
                    } else if key.as_ref().is_some_and(|key| key.equals("nodes")) {
                        GlbSchemaSection::Nodes
                    } else if key.as_ref().is_some_and(|key| key.equals("scenes")) {
                        GlbSchemaSection::Scenes
                    } else if key.as_ref().is_some_and(|key| key.equals("materials")) {
                        GlbSchemaSection::Materials
                    } else if key.as_ref().is_some_and(|key| key.equals("textures")) {
                        GlbSchemaSection::Textures
                    } else if key.as_ref().is_some_and(|key| key.equals("images")) {
                        GlbSchemaSection::Images
                    } else if key.as_ref().is_some_and(|key| key.equals("samplers")) {
                        GlbSchemaSection::Samplers
                    } else {
                        GlbSchemaSection::None
                    };
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("primitives")) {
                    self.primitives_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.numeric_array = key.as_ref().and_then(|key| {
                        if key.equals("children") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeChildren, next_depth, self.output.node_edge_len))
                        } else if key.equals("translation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeTranslation, next_depth, 0))
                        } else if key.equals("rotation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeRotation, next_depth, 0))
                        } else if key.equals("scale") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeScale, next_depth, 0))
                        } else if key.equals("matrix") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeMatrix, next_depth, 0))
                        } else {
                            None
                        }
                    });
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("nodes")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::SceneNodes, next_depth, self.output.scene_root_len));
                } else if self.section == GlbSchemaSection::Materials && self.pbr_depth == Some(self.depth) && key.as_ref().is_some_and(|key| key.equals("baseColorFactor")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::MaterialBaseColor, next_depth, 0));
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("emissiveFactor")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::MaterialEmissive, next_depth, 0));
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ArrayEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON array depth underflowed");
                }
                if self.primitives_depth == Some(self.depth) {
                    self.primitives_depth = None;
                }
                if self.numeric_array.as_ref().is_some_and(|array| array.depth == self.depth) {
                    self.finish_numeric_array()?;
                }
                if self.depth == 2 {
                    self.section = GlbSchemaSection::None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::Colon => {
                self.pending_key = self.last_string.take();
                self.expecting_value = true;
            }
            GlbJsonToken::Comma => {
                self.last_string = None;
            }
            GlbJsonToken::String(atom) => {
                if self.expecting_value {
                    self.apply_string(atom)?;
                    self.consume_value();
                } else {
                    self.last_string = Some(atom);
                }
            }
            GlbJsonToken::Number(atom) => {
                if self.numeric_array.is_some() {
                    self.push_numeric_array(atom.float()?)?;
                } else if self.expecting_value {
                    self.apply_unsigned(atom)?;
                }
                self.consume_value();
            }
            GlbJsonToken::Literal(value) => {
                if self.expecting_value {
                    self.apply_literal(value)?;
                }
                self.consume_value();
            }
        }
        Ok(())
    }

    fn apply_unsigned(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
            let value = value.unsigned()?;
            let accessor = self.current_accessor.as_mut().ok_or("GLB accessor scalar arrived without an owner")?;
            if key.equals("bufferView") {
                accessor.view = u16::try_from(value).map_err(|_| "GLB accessor bufferView exceeded fixed index credits")?;
                accessor.view_set = true;
            } else if key.equals("byteOffset") {
                accessor.byte_offset = u32::try_from(value).map_err(|_| "GLB accessor byteOffset overflowed")?;
            } else if key.equals("componentType") {
                accessor.component = u16::try_from(value).map_err(|_| "GLB accessor componentType overflowed")?;
            } else if key.equals("count") {
                accessor.count = u32::try_from(value).map_err(|_| "GLB accessor count overflowed")?;
            }
        } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
            let value = value.unsigned()?;
            let view = self.current_view.as_mut().ok_or("GLB bufferView scalar arrived without an owner")?;
            if key.equals("buffer") && value != 0 {
                return Err("GLB external buffer index was unsupported");
            } else if key.equals("byteOffset") {
                view.byte_offset = u32::try_from(value).map_err(|_| "GLB bufferView byteOffset overflowed")?;
            } else if key.equals("byteLength") {
                view.byte_length = u32::try_from(value).map_err(|_| "GLB bufferView byteLength overflowed")?;
            } else if key.equals("byteStride") {
                view.byte_stride = u16::try_from(value).map_err(|_| "GLB bufferView byteStride exceeded fixed credits")?;
            }
        } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && (self.attributes_depth == Some(self.depth) || self.depth == 5) {
            let value = value.unsigned()?;
            let primitive = self.current_primitive.as_mut().expect("checked GLB primitive owner");
            if self.attributes_depth == Some(self.depth) {
                let index = u16::try_from(value).map_err(|_| "GLB primitive attribute index exceeded fixed credits")?;
                if key.equals("POSITION") {
                    primitive.position = index;
                    primitive.position_set = true;
                } else if key.equals("NORMAL") {
                    primitive.normal = Some(index);
                } else if let Some(set)=std::str::from_utf8(&key.bytes[..usize::from(key.len)]).ok().and_then(|key|key.strip_prefix("TEXCOORD_")).and_then(|set|set.parse::<usize>().ok()) {
                    if set>=64{return Err("GLB UV set exceeded declared surface credits");}primitive.uv_sets[set] = Some(index);
                } else if key.equals("TANGENT") { primitive.tangent=Some(index);
                } else if key.equals("COLOR_0") {
                    primitive.color = Some(index);
                } else if key.equals(GLB_FACE_ID_ATTRIBUTE) {
                    primitive.face_id = Some(index);
                } else if key.equals(GLB_VERTEX_ID_ATTRIBUTE) {
                    primitive.vertex_id = Some(index);
                }
            } else if self.depth == 5 {
                if key.equals("indices") {
                    primitive.indices = Some(u16::try_from(value).map_err(|_| "GLB primitive index accessor exceeded fixed credits")?);
                } else if key.equals("mode") {
                    primitive.mode = u8::try_from(value).map_err(|_| "GLB primitive mode overflowed")?;
                } else if key.equals("material") {
                    primitive.material = Some(u16::try_from(value).map_err(|_| "GLB primitive material index exceeded fixed credits")?);
                }
            }
        } else if self.section == GlbSchemaSection::Materials {
            let material = self.current_material.as_mut().ok_or("GLB material scalar arrived without an owner")?;
            if self.pbr_depth == Some(self.depth) && key.equals("metallicFactor") {
                material.metalness = value.float()?;
            } else if self.pbr_depth == Some(self.depth) && key.equals("roughnessFactor") {
                material.roughness = value.float()?;
            } else if self.base_color_texture_depth == Some(self.depth) && key.equals("index") {
                let index = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB material texture index exceeded fixed credits")?);
                if self.material_texture_role == 0 {material.base_color_texture=index;} else {material.additional_textures[self.material_texture_role-1]=index;}
            } else if self.base_color_texture_depth == Some(self.depth) && key.equals("texCoord") {
                let set=value.unsigned()?;if set>=64{return Err("GLB UV selector exceeded declared surface credits");}material.texture_coordinates[self.material_texture_role]=set as u8;
            } else if self.base_color_texture_depth == Some(self.depth) && self.material_texture_role==2 && key.equals("scale") {material.normal_scale=value.float()?;
            } else if self.base_color_texture_depth == Some(self.depth) && self.material_texture_role==3 && key.equals("strength") {material.occlusion_strength=value.float()?;
            } else if self.depth == 3 && key.equals("alphaCutoff") {
                material.alpha_cutoff = value.float()?;
            }
        } else if self.section == GlbSchemaSection::Textures && self.depth == 3 {
            let value = u16::try_from(value.unsigned()?).map_err(|_| "GLB texture index exceeded fixed credits")?;
            let texture = self.current_texture.as_mut().ok_or("GLB texture scalar arrived without an owner")?;
            if key.equals("sampler") {
                texture.sampler = Some(value);
            } else if key.equals("source") {
                texture.source = Some(value);
            }
        } else if self.section == GlbSchemaSection::Images && self.depth == 3 && key.equals("bufferView") {
            self.current_image.as_mut().ok_or("GLB image bufferView arrived without an owner")?.view = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB image bufferView exceeded fixed credits")?);
        } else if self.section == GlbSchemaSection::Samplers && self.depth == 3 {
            let value = u16::try_from(value.unsigned()?).map_err(|_| "GLB sampler value overflowed")?;
            let sampler = self.current_sampler.as_mut().ok_or("GLB sampler scalar arrived without an owner")?;
            if key.equals("wrapS") {
                sampler.wrap_s = value;
            } else if key.equals("wrapT") {
                sampler.wrap_t = value;
            } else if key.equals("magFilter") {
                sampler.mag_filter = Some(value);
            } else if key.equals("minFilter") {
                sampler.min_filter = Some(value);
            }
        } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 && key.equals("mesh") {
            self.current_node.as_mut().ok_or("GLB node mesh arrived without an owner")?.mesh = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB node mesh index exceeded fixed credits")?);
        } else if self.section == GlbSchemaSection::None && self.depth == 1 && key.equals("scene") {
            self.output.default_scene = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB default scene index exceeded fixed credits")?);
        }
        Ok(())
    }

}
//#endregion 📡️RendererAssetAuthority
