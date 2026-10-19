//! 🧵️ Materialize cursor step: the materialize phases and cursor state with its constructor and the long materialize step. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
#[derive(Clone, Copy)]
pub enum GlbMaterializePhase {
    Plan,
    Header,
    TextureBytes,
    TextureDecode,
    Outlines,
    Allocate,
    Positions,
    Normals,
    Tangents,
    Uvs,
    Colors,
    Indices,
    FaceIds,
    VertexIds,
    GenerateNormals,
    NormalizeNormals,
    OutlineEdges,
    Seal,
    Ready,
    Closing,
}

pub struct GlbMaterializeCursor {
    pub(super) schema: GlbSchemaOutput,
    pub(super) plan: GlbInstancePlanCursor,
    pub(super) write: Option<Mesh3dWriteToken>,
    pub lease: Option<Mesh3dLease>,
    pub appearance: Option<infinite_world::world::World3dMeshAppearance>,
    pub(super) textures: Vec<(String, SceneRasterLease)>,
    pub(super) texture_bytes: Vec<u8>,
    pub(super) decoded_texture: Option<DecodedReferenceImage>,
    texture_hash:semio_framework_hash::Sha256,
    pub(super) texture_digest:[u64;2],
    pub(super) texture_png:Option<semio_framework_pixels::png_decoding::PngDecodeJob>,
    pub(super) texture_submission:Option<semio_framework_async::Job>,
    pub(super) texture_worker:Option<Arc<Mutex<Option<Option<semio_framework_pixels::RasterImage>>>>>,
    pub(super) texture_cancelled:Arc<AtomicBool>,
    pub(super) texture: u16,
    uv_role:usize,
    pub(super) texture_role: u8,
    outline: Option<GlbOutlineAccumulator>,
    pub phase: GlbMaterializePhase,
    pub(super) bin_start: usize,
    pub(super) bin_bytes: usize,
    pub(super) instance: u16,
    pub(super) item: u32,
    pub(super) normal_substep: u8,
    normal_indices: [u32; 3],
    normal_positions: [[f32; 3]; 3],
    normal_face: [f32; 3],
}

impl GlbMaterializeCursor {
    pub fn new(schema: GlbSchemaOutput) -> Self {
        let plan = GlbInstancePlanCursor::new(&schema);
        Self { schema, plan, write: None, lease: None, appearance: None, textures: Vec::new(), texture_bytes: Vec::new(), decoded_texture: None,texture_hash:Default::default(),texture_digest:[0;2],texture_png:None,texture_submission:None,texture_worker:None,texture_cancelled:Arc::new(AtomicBool::new(false)), texture: 0, uv_role:0, texture_role: 0, outline: None, phase: GlbMaterializePhase::Plan, bin_start: 0, bin_bytes: 0, instance: 0, item: 0, normal_substep: 0, normal_indices: [0; 3], normal_positions: [[0.0; 3]; 3], normal_face: [0.0; 3] }
    }

    pub fn step(&mut self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex) -> Result<bool, &'static str> {
        match self.phase {
            GlbMaterializePhase::Plan => {
                if self.plan.step(&self.schema)? {
                    if self.plan.output_bytes == 0 {
                        return Err("GLB plan produced no reachable triangle output");
                    }
                    let maximum_edges = usize::try_from(self.plan.index_count).map_err(|_| "GLB outline edge count overflowed")?;
                    let maximum_outline_bytes = maximum_edges.checked_mul(24).ok_or("GLB outline byte count overflowed")?;
                    if self.plan.output_bytes.checked_add(maximum_outline_bytes).is_none_or(|bytes| bytes > GLB_SCHEMA_OUTPUT_BYTES) {
                        return Err("GLB mesh and outline exceeded fixed semantic output credits");
                    }
                    self.outline = Some(GlbOutlineAccumulator::new(maximum_edges)?);
                    self.phase = GlbMaterializePhase::Header;
                }
                Ok(false)
            }
            GlbMaterializePhase::Header => {
                let header_at = 20usize.checked_add(self.schema.json_bytes as usize).ok_or("GLB BIN header offset overflowed")?;
                let header = pages.read::<8>(owner, header_at)?;
                if &header[4..] != b"BIN\0" {
                    return Err("GLB semantic materializer did not find BIN");
                }
                self.bin_bytes = u32::from_le_bytes(header[..4].try_into().expect("fixed GLB BIN length")) as usize;
                self.bin_start = header_at.checked_add(8).ok_or("GLB BIN payload offset overflowed")?;
                if self.bin_bytes == 0 || self.bin_start.checked_add(self.bin_bytes).is_none_or(|end| end > pages.total) {
                    return Err("GLB BIN payload exceeded sealed page credits");
                }
                self.validate_bin_spans()?;
                self.phase = if self.schema.texture_len == 0 { GlbMaterializePhase::Outlines } else { GlbMaterializePhase::TextureBytes };
                Ok(false)
            }
            GlbMaterializePhase::TextureBytes => {
                if self.texture == self.schema.texture_len {
                    self.advance_phase(GlbMaterializePhase::Outlines);
                    return Ok(false);
                }
                let required=self.schema.materials[..usize::from(self.schema.material_len)].iter().flatten().any(|material|if self.texture_role==0 {material.base_color_texture==Some(self.texture)||material.additional_textures[3]==Some(self.texture)}else {material.additional_textures[..3].contains(&Some(self.texture))});
                if !required {self.texture_role+=1;if self.texture_role==2{self.texture_role=0;self.texture+=1;}return Ok(false)}
                let texture = self.schema.textures.get(usize::from(self.texture)).and_then(Option::as_ref).ok_or("GLB materializer lost a texture schema")?;
                let image = self
                    .schema
                    .images
                    .get(usize::from(texture.source.ok_or("GLB texture lost its image source")?))
                    .and_then(Option::as_ref)
                    .ok_or("GLB materializer lost an image schema")?;
                let view = self
                    .schema
                    .views
                    .get(usize::from(image.view.ok_or("GLB image lost its embedded bufferView")?))
                    .and_then(Option::as_ref)
                    .ok_or("GLB materializer lost an image bufferView")?;
                let expected = usize::try_from(view.byte_length).map_err(|_| "GLB image byte count exceeded address credits")?;
                if self.item == 0 {
                    self.texture_bytes.clear();
                    self.texture_bytes.try_reserve_exact(expected).map_err(|_| "GLB image exceeded allocation credits")?;
                }
                let cursor = usize::try_from(self.item).map_err(|_| "GLB image cursor exceeded address credits")?;
                if cursor == expected {
                    let digest=std::mem::take(&mut self.texture_hash).finalize();self.texture_digest=[u64::from_be_bytes(digest[..8].try_into().unwrap()),u64::from_be_bytes(digest[8..16].try_into().unwrap())];
                    let bytes=std::mem::take(&mut self.texture_bytes);
                    if image.mime==1 {self.texture_png=Some(semio_framework_pixels::png_decoding::PngDecodeJob::new(semio_framework_pixels::png_decoding::PngDecodeInput{data:Arc::new(bytes),max_pixels:16_777_216,max_bytes:16_000_000,max_chunks:65_536}).map_err(|_|"GLB owned PNG texture decoder refused its input")?);}
                    else {let worker=Arc::new(Mutex::new(None));let output=worker.clone();let canceled=self.texture_cancelled.clone();self.texture_submission=Some(Box::new(move||{let image=if canceled.load(Ordering::Acquire){None}else{infinite_world::world::decode_mesh_surface_image_bytes(&bytes)};*output.lock().unwrap_or_else(|poisoned|poisoned.into_inner())=Some(image);}));self.texture_worker=Some(worker);}
                    self.phase = GlbMaterializePhase::TextureDecode;
                    self.item = 0;
                    return Ok(false);
                }
                let take = (expected - cursor).min(RENDERER_ASSET_PARSE_BLOCK_BYTES);
                let absolute = self
                    .bin_start
                    .checked_add(usize::try_from(view.byte_offset).map_err(|_| "GLB image byte offset exceeded address credits")?)
                    .and_then(|offset| offset.checked_add(cursor))
                    .ok_or("GLB image address overflowed")?;
                for at in absolute..absolute + take {
                    self.texture_bytes.push(pages.read::<1>(owner, at)?[0]);
                }
                self.texture_hash.update(&self.texture_bytes[cursor..cursor+take]);
                self.item = self.item.checked_add(u32::try_from(take).map_err(|_| "GLB image block exceeded cursor credits")?).ok_or("GLB image cursor overflowed")?;
                Ok(false)
            }
            GlbMaterializePhase::TextureDecode => {
                if self.decoded_texture.is_none(){
                    if let Some(submission)=self.texture_submission.take(){if let Err(refusal)=renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance,submission){self.texture_submission=Some(refusal.into_job());}return Ok(false)}
                    let image=if let Some(job)=self.texture_png.as_mut(){if !job.advance(4096).map_err(|_|"GLB owned PNG decode failed")?.done{return Ok(false)}self.texture_png.take().unwrap().into_result().map_err(|_|"GLB owned PNG decode failed")?}
                    else {let worker=self.texture_worker.as_ref().ok_or("GLB texture decoder owner was lost")?;let image=worker.lock().map_err(|_|"GLB texture decoder lock failed")?.take();let Some(image)=image else{return Ok(false)};self.texture_worker=None;image.ok_or("GLB owned JPEG decode failed")?};
                    self.decoded_texture=Some(DecodedReferenceImage{width:image.width,height:image.height,source_digest:self.texture_digest,pixels:image.pixels});return Ok(false)
                }
                let decoded = self.decoded_texture.take().ok_or("GLB materializer lost its decoded texture")?;
                let descriptor = SceneRasterDescriptor {
                    width: decoded.width,
                    height: decoded.height,
                    source_digest: decoded.source_digest,
                    source_revision: 1,
                    profile: if self.texture_role == 0 {SceneRasterProfile::MeshBaseColorSrgb} else {SceneRasterProfile::MeshNumericLinear},
                    mesh: None,
                };
                let pool = infinite_world::world::world_scene_raster_pool();
                let raster_owner = decoded.source_digest[0].wrapping_add(decoded.source_digest[1]).max(1);
                let lease = match pool.begin(descriptor, raster_owner, SceneRasterWriteMode::Moved { capacity_bytes: decoded.pixels.capacity() }) {
                    SceneRasterBegin::Reused(lease) => lease,
                    SceneRasterBegin::Writer(writer) => {
                        let DecodedReferenceImage { width, height, source_digest, pixels } = decoded;
                        match pool.prepare_moved(writer, pixels).and_then(|prepared| pool.seal_prepared_moved(prepared)) {
                            Ok(lease) => lease,
                            Err((pixels, detail)) => {
                                self.decoded_texture = Some(DecodedReferenceImage { width, height, source_digest, pixels });
                                return Err(detail);
                            }
                        }
                    }
                    SceneRasterBegin::Backpressure(_) => {
                        self.decoded_texture = Some(decoded);
                        return Err("GLB texture residency admission is full");
                    }
                    SceneRasterBegin::Refused(detail) => {
                        self.decoded_texture = Some(decoded);
                        return Err(detail);
                    }
                };
                self.textures.push((self.texture_key(owner), lease));
                self.texture_role += 1;
                if self.texture_role == 2 {self.texture_role = 0;self.texture += 1;}
                self.texture_bytes.clear();
                self.phase = GlbMaterializePhase::TextureBytes;
                Ok(false)
            }
            GlbMaterializePhase::Outlines => {
                if self.instance == self.plan.instance_len {
                    let outline = self.outline.as_mut().ok_or("GLB materializer lost its outline owner")?;
                    if outline.flush_step()? {
                        let edges = u32::try_from(outline.segments.len()).map_err(|_| "GLB outline output count overflowed")?;
                        let schema = Mesh3dSchema { vertices: self.plan.vertex_count, indices: self.plan.index_count, face_ids: if self.plan.has_face_ids() { self.plan.index_count / 3 } else { 0 }, vertex_ids: if self.plan.has_vertex_ids() { self.plan.vertex_count } else { 0 }, edges, edge_ids: 0, uvs: if self.plan.has_uvs { self.plan.vertex_count } else { 0 }, colors: if self.plan.has_colors { self.plan.vertex_count } else { 0 } ,surface_uvs:if self.plan.has_uvs {[self.plan.vertex_count;4]}else{[0;4]},tangents:if self.plan.has_tangents {self.plan.vertex_count}else{0},};
                        self.write = Some(mesh3d_begin(owner.generation(), owner.revision(), schema).map_err(glb_mesh_fault)?);
                        self.phase = GlbMaterializePhase::Allocate;
                    }
                    return Ok(false);
                }
                let instance = self.instance()?;
                let triangle_count = instance.index_count / 3;
                if self.item == triangle_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let mut points = [[0.0; 3]; 3];
                for component in 0..3u32 {
                    let index = self.read_output_index(owner, pages, primitive, self.item * 3 + component, instance.vertex_count)?;
                    let value = self.read_vec(owner, pages, primitive.position, index, 3)?;
                    points[component as usize] = [value[0], value[1], value[2]];
                }
                self.outline.as_mut().ok_or("GLB materializer lost its outline owner")?.push_triangle(self.instance, instance.matrix, points)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Allocate => {
                if mesh3d_allocate_step(self.write_token()?).map_err(glb_mesh_fault)? {
                    self.advance_phase(GlbMaterializePhase::Positions);
                }
                Ok(false)
            }
            GlbMaterializePhase::Positions => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Normals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = self.read_vec(owner, pages, primitive.position, self.item, 3)?;
                let value = glb_transform_point(instance.matrix, [value[0], value[1], value[2]]);
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Positions, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Normals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Tangents);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let value = if instance.explicit_normals {
                    let primitive = self.primitive(instance)?;
                    let accessor = primitive.normal.ok_or("GLB explicit normal stage lost its accessor")?;
                    let value = self.read_vec(owner, pages, accessor, self.item, 3)?;
                    glb_transform_normal(instance.matrix, [value[0], value[1], value[2]])
                } else {
                    [0.0; 3]
                };
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Normals, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Tangents=>{
                if !self.plan.has_tangents || self.instance==self.plan.instance_len {self.advance_phase(GlbMaterializePhase::Uvs);return Ok(false);}
                let instance=self.instance()?;if self.item==instance.vertex_count {self.next_instance();return Ok(false);}
                let value=if let Some(accessor)=self.primitive(instance)?.tangent {let value=self.read_vec(owner,pages,accessor,self.item,4)?;if value[..3].iter().all(|value|*value==0.0) || value[3].abs()!=1.0 {return Err("GLB TANGENT requires a nonzero direction and signed handedness");}let matrix=instance.matrix;let direction=glb_normalize(std::array::from_fn(|axis|matrix[0][axis]*value[0]+matrix[1][axis]*value[1]+matrix[2][axis]*value[2]));let determinant=matrix[0][0]*(matrix[1][1]*matrix[2][2]-matrix[2][1]*matrix[1][2])-matrix[1][0]*(matrix[0][1]*matrix[2][2]-matrix[2][1]*matrix[0][2])+matrix[2][0]*(matrix[0][1]*matrix[1][2]-matrix[1][1]*matrix[0][2]);[direction[0],direction[1],direction[2],value[3]*if determinant<0.0{-1.0}else{1.0}]}else{[0.0;4]};
                mesh3d_write_vec4(self.write_token()?,Mesh3dField::Tangents,value).map_err(glb_mesh_fault)?;self.item+=1;Ok(false)
            }
            GlbMaterializePhase::Uvs => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Colors);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if !self.plan.has_uvs {
                    self.advance_phase(GlbMaterializePhase::Colors);
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let material=primitive.material.and_then(|index|self.schema.materials[usize::from(index)].as_ref());
                let role=self.uv_role;let set=material.map_or(0,|material|usize::from(material.texture_coordinates[role]));
                let value = if let Some(accessor) = primitive.uv_sets[set] {
                    let value = self.read_vec(owner, pages, accessor, self.item, 2)?;
                    [value[0], value[1]]
                } else {
                    [0.0; 2]
                };
                mesh3d_write_vec2(self.write_token()?, [Mesh3dField::Uvs,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive][role], value).map_err(glb_mesh_fault)?;
                self.uv_role+=1;if self.uv_role==5{self.uv_role=0;self.item += 1;}
                Ok(false)
            }
            GlbMaterializePhase::Colors => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if !self.plan.has_colors {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = if let Some(accessor) = primitive.color {
                    let accessor_schema = self.schema.accessor(accessor)?;
                    let value = self.read_vec(owner, pages, accessor, self.item, usize::from(accessor_schema.kind))?;
                    [value[0], value[1], value[2], if accessor_schema.kind == 4 { value[3] } else { 1.0 }]
                } else {
                    [1.0; 4]
                };
                mesh3d_write_vec4(self.write_token()?, Mesh3dField::Colors, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Indices => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::FaceIds);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.index_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let index = self.read_output_index(owner, pages, primitive, self.item, instance.vertex_count)?;
                let index = instance.vertex_base.checked_add(index).ok_or("GLB instantiated index overflowed")?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::FaceIds => {
                if self.instance == self.plan.instance_len || !self.plan.has_face_ids() {
                    self.advance_phase(GlbMaterializePhase::VertexIds);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.index_count / 3 {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let accessor = primitive.face_id.ok_or("GLB face id plan lost its accessor")?;
                let corner = self.read_output_index(owner, pages, primitive, self.item * 3, instance.vertex_count)?;
                let id = self.read_unsigned_id(owner, pages, accessor, corner)?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::FaceIds, id).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::VertexIds => {
                if self.instance == self.plan.instance_len || !self.plan.has_vertex_ids() {
                    self.advance_phase(GlbMaterializePhase::GenerateNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let accessor = primitive.vertex_id.ok_or("GLB vertex id plan lost its accessor")?;
                let id = self.read_unsigned_id(owner, pages, accessor, self.item)?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::VertexIds, id).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::GenerateNormals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::NormalizeNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                let triangle_count = instance.index_count / 3;
                if self.item == triangle_count {
                    self.next_instance();
                    return Ok(false);
                }
                let token = self.write_token()?;
                match self.normal_substep {
                    0..=2 => {
                        let component = usize::from(self.normal_substep);
                        let index = instance.index_base.checked_add(self.item * 3).and_then(|index| index.checked_add(component as u32)).ok_or("GLB generated-normal index address overflowed")?;
                        self.normal_indices[component] = mesh3d_read_write_u32(token, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    3..=5 => {
                        let component = usize::from(self.normal_substep - 3);
                        self.normal_positions[component] = mesh3d_read_write_vec3(token, Mesh3dField::Positions, self.normal_indices[component]).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    6 => {
                        let a = [self.normal_positions[1][0] - self.normal_positions[0][0], self.normal_positions[1][1] - self.normal_positions[0][1], self.normal_positions[1][2] - self.normal_positions[0][2]];
                        let b = [self.normal_positions[2][0] - self.normal_positions[0][0], self.normal_positions[2][1] - self.normal_positions[0][1], self.normal_positions[2][2] - self.normal_positions[0][2]];
                        self.normal_face = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
                        self.normal_substep += 1;
                    }
                    7..=9 => {
                        let component = usize::from(self.normal_substep - 7);
                        let index = self.normal_indices[component];
                        let prior = mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?;
                        mesh3d_update_vec3(token, Mesh3dField::Normals, index, [prior[0] + self.normal_face[0], prior[1] + self.normal_face[1], prior[2] + self.normal_face[2]]).map_err(glb_mesh_fault)?;
                        if self.normal_substep == 9 {
                            self.normal_substep = 0;
                            self.item += 1;
                        } else {
                            self.normal_substep += 1;
                        }
                    }
                    _ => return Err("GLB generated-normal cursor exceeded its fixed phase"),
                }
                Ok(false)
            }
            GlbMaterializePhase::NormalizeNormals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::OutlineEdges);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let index = instance.vertex_base.checked_add(self.item).ok_or("GLB normal output index overflowed")?;
                let token = self.write_token()?;
                let normal = glb_normalize(mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?);
                mesh3d_update_vec3(token, Mesh3dField::Normals, index, normal).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::OutlineEdges => {
                let outline = self.outline.as_ref().ok_or("GLB materializer lost its outline owner")?;
                let Some(edge) = outline.segments.get(self.item as usize).copied() else {
                    self.phase = GlbMaterializePhase::Seal;
                    return Ok(false);
                };
                mesh3d_write_edge(self.write_token()?, edge).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Seal => {
                let token = self.write.take().ok_or("GLB materializer lost its mesh write claim")?;
                match mesh3d_seal(token) {
                    Ok(lease) => {
                        let appearance = self.build_appearance(owner)?;
                        self.lease = Some(lease);
                        self.appearance = Some(appearance);
                        self.phase = GlbMaterializePhase::Ready;
                        Ok(true)
                    }
                    Err(fault) => {
                        self.write = Some(token);
                        Err(glb_mesh_fault(fault))
                    }
                }
            }
            GlbMaterializePhase::Ready => Ok(true),
            GlbMaterializePhase::Closing => Ok(false),
        }
    }

}
//#endregion 📡️RendererAssetAuthority
