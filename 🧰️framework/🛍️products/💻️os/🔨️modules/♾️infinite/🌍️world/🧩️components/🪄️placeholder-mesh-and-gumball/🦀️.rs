//! 🪄️ World3d relocate session, gumball handles, the placeholder mesh authority and mesh buffer helpers.

use super::*;

//#region WorldRelocateGesture
/// 🚚️ One live relocate drag. `origin` is where the grabbed object sat when the press landed and
/// `from` is the ground point under that press; the commit applies the ground-plane travel
/// `from → to` to `origin`, never an incremental pose delta — React's `World3dRelocateSession`
/// (`🌐️World3dHost/🟦️.tsx:4752`).
#[derive(Clone, Debug, PartialEq)]
pub struct World3dRelocateSession {
    pub object_id: String,
    pub origin: [f32; 3],
    pub from: [f32; 3],
}

#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TerrainTileMeshPayload {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub uvs: Vec<f32>,
}

//#region Gumball
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GumballHandle {
    MoveX,
    MoveY,
    MoveZ,
    MoveXY,
    MoveYZ,
    MoveXZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

impl GumballHandle {
    pub fn axis_dir(self) -> Option<Vec3> {
        match self {
            Self::MoveX | Self::RotateX | Self::ScaleX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveY | Self::RotateY | Self::ScaleY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::MoveZ | Self::RotateZ | Self::ScaleZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub fn plane_normal(self) -> Option<Vec3> {
        match self {
            Self::MoveXY => Some(Vec3::new(0.0, 0.0, 1.0)),
            Self::MoveYZ => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveXZ => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::RotateY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub fn is_translate(self) -> bool {
        matches!(self, Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ)
    }

    pub fn is_rotate(self) -> bool {
        matches!(self, Self::RotateX | Self::RotateY | Self::RotateZ)
    }

    pub fn is_scale(self) -> bool {
        matches!(self, Self::ScaleX | Self::ScaleY | Self::ScaleZ)
    }
}
//#endregion Gumball

//#region MeshHelpers
//#region PlaceholderMeshAuthority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldPlaceholderKind {
    Box,
    Plane,
    Cylinder,
    Cone,
    Icosphere,
}

impl WorldPlaceholderKind {
    pub fn resolve(value: &str) -> Self {
        match value {
            "vortex-marker" => Self::Icosphere,
            "plane" => Self::Plane,
            "cylinder" => Self::Cylinder,
            "cone" => Self::Cone,
            _ => Self::Box,
        }
    }

    pub fn triangles(self) -> u32 {
        match self {
            Self::Box => 12,
            Self::Plane => 2,
            Self::Cylinder => 64,
            Self::Cone => 32,
            Self::Icosphere => 80,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldPlaceholderMeshPhase {
    Allocate,
    Positions,
    Normals,
    Tangents,
    Uvs,
    Colors,
    Indices,
    FaceIds,
    VertexIds,
    Edges,
    EdgeIds,
    Materials,
    Textures,
    Provenance,
    AdmitProvenance,
    Seal,
    Publish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldPlaceholderOwner {
    Writing(Mesh3dWriteToken),
    Ready(Mesh3dLease),
    Empty,
}

/// 🧊️ Where one in-flight mesh publication reads its vertices from: a generated marker primitive,
/// or an inline `World3dScene.meshes_json` buffer set bridged in by [`stage_world3d_scene_bridge`].
/// Both ride the identical fixed-credit `mesh3d_*` authority ladder below.
pub enum WorldMeshInput {
    Placeholder(WorldPlaceholderKind),
    Inline(Box<WorldMeshBuffers>),
}

impl WorldMeshInput {
    pub fn position(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle(*kind, item / 3)[(item % 3) as usize],
            Self::Inline(buffers) if buffers.positions.is_empty() => {
                let offset = item as usize * 3;
                [buffers.edge_positions[offset], buffers.edge_positions[offset + 1], buffers.edge_positions[offset + 2]]
            }
            Self::Inline(buffers) => buffers.position(item),
        }
    }

    pub fn normal(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle_normal(placeholder_triangle(*kind, item / 3)),
            Self::Inline(buffers) => buffers.normal(item),
        }
    }

    pub fn index(&self, item: u32) -> u32 {
        match self {
            Self::Placeholder(_) => item,
            Self::Inline(buffers) => buffers.indices.get(item as usize).copied().unwrap_or(0),
        }
    }
}

pub struct WorldPlaceholderMeshCursor {
    pub key: String,
    pub source: WorldMeshInput,
    vertex_items: u32,
    index_items: u32,
    pub phase: WorldPlaceholderMeshPhase,
    pub item: u32,
    pub owner: WorldPlaceholderOwner,
    pub close_started: bool,
    pub faulted: bool,
    pub expanded: bool,
    source_tail_start: u32,
    pub schema:Mesh3dSchema,
    uv_role:usize,
    texture_cursor:usize,
    texture_role:usize,
    texture_name:Option<String>,
    pub texture_job:Option<semio_framework_pixels::png_decoding::PngDecodeJob>,
    pub texture_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_pixels::png_decoding::PngDecodeJob>>,
    texture_image:Option<semio_framework_pixels::RasterImage>,
    texture_digest:[u64;2],
    texture_bytes:Vec<u8>,
    texture_byte:usize,
    texture_scan:usize,
    texture_required:bool,
    texture_hash:semio_framework_hash::Sha256,
    texture_submission:Option<semio_framework_async::Job>,
    texture_cancelled:std::sync::Arc<std::sync::atomic::AtomicBool>,
    texture_worker:Option<std::sync::Arc<Mutex<Option<Option<semio_framework_pixels::RasterImage>>>>>,
    pub primitives:Vec<World3dPrimitiveMaterial>,
    pub textures:Vec<(String,SceneRasterLease)>,
    pub appearance:Option<World3dMeshAppearance>,
    pub source_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<WorldMeshBuffers>>>,
}

pub enum WorldPlaceholderMeshStep {
    Pending,
    Ready(String, Mesh3dLease,Option<World3dMeshAppearance>),
    Fault,
}

impl WorldPlaceholderMeshCursor {
    pub fn new(key: &str, kind: WorldPlaceholderKind, generation: u64, revision: u64) -> Result<Self, ui_scene::math::Mesh3dFault> {
        let items = kind.triangles().checked_mul(3).ok_or(ui_scene::math::Mesh3dFault::ItemCapacity)?;
        Self::begin(key, WorldMeshInput::Placeholder(kind), items, items, generation, revision)
    }

    /// 🌉️ The inline-wire twin of [`Self::new`]: an already-welded `meshes_json` buffer set, whose
    /// vertex and index counts are independent (the marker primitives above emit one vertex per
    /// face corner, a tessellated preview mesh does not).
    pub fn inline(key: &str, buffers: WorldMeshBuffers, generation: u64, revision: u64) -> Result<Self, ui_scene::math::Mesh3dFault> {
        let edge_only = buffers.positions.is_empty() && !buffers.edge_positions.is_empty();
        if !buffers.positions.len().is_multiple_of(3) || !buffers.edge_positions.len().is_multiple_of(6)
            || (edge_only && (!buffers.indices.is_empty() || !buffers.vertex_ids.is_empty())) {
            return Err(ui_scene::math::Mesh3dFault::Schema);
        }
        let vertex_items = u32::try_from(if edge_only { buffers.edge_positions.len() / 3 } else { buffers.vertex_count() }).map_err(|_| ui_scene::math::Mesh3dFault::ItemCapacity)?;
        let index_items = u32::try_from(buffers.indices.len()).map_err(|_| ui_scene::math::Mesh3dFault::ItemCapacity)?;
        if vertex_items == 0 || !index_items.is_multiple_of(3) || buffers.indices.iter().any(|index| *index >= vertex_items) {
            return Err(ui_scene::math::Mesh3dFault::Schema);
        }
        semio_framework::validate_mesh_surface_assets(&buffers.materials,&buffers.textures).map_err(|_|ui_scene::math::Mesh3dFault::Schema)?;
        for(name,attribute)in &buffers.attributes {let uses=if attribute.domain==semio_framework::MeshAttributeDomain::Edge{buffers.edge_ids.len()}else{index_items as usize};semio_framework::validate_mesh_attribute(name,attribute,vertex_items as usize,index_items as usize/3,uses,&buffers.materials).map_err(|_|ui_scene::math::Mesh3dFault::Schema)?;}
        let expanded=!buffers.indices.is_empty() && buffers.attributes.values().any(|attribute|attribute.domain==semio_framework::MeshAttributeDomain::Corner || attribute.domain==semio_framework::MeshAttributeDomain::Face);
        let source_tail_start = if expanded { buffers.vertex_ids.iter().position(|id| *id != u32::MAX).map_or(vertex_items, |item| item as u32) } else { vertex_items };
        let expanded_items = index_items.checked_add(vertex_items - source_tail_start).ok_or(ui_scene::math::Mesh3dFault::ItemCapacity)?;
        let mut cursor=Self::begin(key,WorldMeshInput::Inline(Box::new(buffers)),if expanded {expanded_items}else{vertex_items},index_items,generation,revision)?;
        cursor.expanded=expanded;
        cursor.source_tail_start=source_tail_start;
        Ok(cursor)
    }

    pub fn begin(key: &str, source: WorldMeshInput, vertex_items: u32, index_items: u32, generation: u64, revision: u64) -> Result<Self, ui_scene::math::Mesh3dFault> {
        if key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(ui_scene::math::Mesh3dFault::ByteCapacity);
        }
        let (uvs,colors)=match &source {WorldMeshInput::Inline(buffers)=>(!buffers.uvs.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv),!buffers.colors.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Color)),_ => (false,false)};
        let (face_ids,vertex_ids,edges,edge_ids)=match &source {WorldMeshInput::Inline(mesh)=>(mesh.face_ids.len() as u32,if mesh.vertex_ids.is_empty(){0}else{vertex_items},mesh.edge_positions.len() as u32/6,mesh.edge_ids.len() as u32),_ => (0,0,0,0)};
        let schema=Mesh3dSchema{face_ids,vertex_ids,edges,edge_ids,uvs:if uvs {vertex_items}else{0},colors:if colors {vertex_items}else{0},surface_uvs:if uvs {[vertex_items;4]}else{[0;4]},tangents:if matches!(&source,WorldMeshInput::Inline(mesh) if mesh.attributes.contains_key("tangent")){vertex_items}else{0},..Mesh3dSchema::triangle_mesh(vertex_items,index_items)};
        let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_begin_capacity_byte_demand(), 0, 1);
        let (token, progress) = mesh3d_begin(generation, revision, schema, grant)?;
        let owner = WorldPlaceholderOwner::Writing(token);
        assert!(progress.fits(grant), "original mesh constructor exceeded its admitted ownership grant");
        Ok(Self { key: key.to_owned(), source, vertex_items, index_items, phase: WorldPlaceholderMeshPhase::Allocate, item: 0, owner, close_started: false, faulted: false,expanded:false,source_tail_start:0,schema,uv_role:0,texture_cursor:0,texture_role:0,texture_name:None,texture_job:None,texture_retirement:None,texture_image:None,texture_digest:[0;2],texture_bytes:Vec::new(),texture_byte:0,texture_scan:0,texture_required:false,texture_hash:Default::default(),texture_submission:None,texture_cancelled:std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),texture_worker:None,primitives:Vec::new(),textures:Vec::new(),appearance:None,source_retirement:None })
    }

    fn source_vertex(&self,item:u32)->u32 {if self.expanded {if item < self.index_items {self.source.index(item)}else{self.source_tail_start + item - self.index_items}}else{item}}

    pub fn sample<const N:usize>(&self,semantic:semio_framework::MeshAttributeSemantic,item:u32,fallback:[f32;N])->Result<[f32;N],ui_scene::math::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(fallback)};
        let Some(attribute)=mesh.attributes.values().find(|attribute|attribute.semantic==semantic) else{return Ok(fallback)};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { return Ok(fallback); }
        let domain=match attribute.domain {semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_scene::math::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==N).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
        let mut result=[0.0;N];for axis in 0..N {result[axis]=values[axis].as_f64().filter(|value|value.is_finite()&&value.abs()<=f32::MAX as f64).ok_or(ui_scene::math::Mesh3dFault::Schema)? as f32;}Ok(result)
    }

    fn uv_sample(&self,role:usize,item:u32)->Result<[f32;2],ui_scene::math::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok([0.0;2])};
        let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];
        let name=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material).and_then(|attribute|attribute.value_at(item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str));
        let material=name.and_then(|name|mesh.materials.get(name)).and_then(|value|value.as_object());
        let coordinates=material.and_then(|fields|fields.iter().find_map(|(key,value)|(key=="textureCoordinates").then_some(value))).and_then(|value|value.as_object());
        let set=coordinates.and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(0) as usize;
        let declared=mesh.attributes.get(if set==0{"uv"}else{""}).or_else(||mesh.attributes.get(&format!("uv{set}")));
        let attribute=declared.or_else(||(set==0).then(||mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv)).flatten());
        let Some(attribute)=attribute else {if set!=0{return Err(ui_scene::math::Mesh3dFault::Schema)}let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)])};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)]); }
        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_scene::math::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==2).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
        Ok([values[0].as_f64().ok_or(ui_scene::math::Mesh3dFault::Schema)? as f32,values[1].as_f64().ok_or(ui_scene::math::Mesh3dFault::Schema)? as f32])
    }

    fn material_step(&mut self)->Result<(),ui_scene::math::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(())};
        if mesh.materials.is_empty(){return Ok(())}
        let attribute=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material);
        let name=attribute.and_then(|attribute|attribute.value_at(self.item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str)).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
        let source=mesh.materials.get(name).and_then(|value|value.as_object()).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
        let find=|name:&str|source.iter().find_map(|(key,value)|(key==name).then_some(value));
        let tuple=|name:&str,defaults:[f32;4],width:usize|->Result<[f32;4],ui_scene::math::Mesh3dFault>{let Some(value)=find(name)else{return Ok(defaults)};let values=value.as_array().filter(|values|values.len()==width).ok_or(ui_scene::math::Mesh3dFault::Schema)?;let mut result=defaults;for axis in 0..width{result[axis]=values[axis].as_f64().ok_or(ui_scene::math::Mesh3dFault::Schema)? as f32;}Ok(result)};
        let sampler=|role:usize| {let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];let owned=find("textureSamplers").and_then(|value|value.as_object()).and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_object());let value=|name:&str,default:u64|owned.and_then(|values|values.iter().find_map(|(key,value)|(key==name).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(default);let wrap=|value|match value{33071=>ui_scene::math::SceneTextureWrap3d::ClampToEdge,33648=>ui_scene::math::SceneTextureWrap3d::MirrorRepeat,_=>ui_scene::math::SceneTextureWrap3d::Repeat};let filter=|value|match value{9728=>ui_scene::math::SceneTextureFilter3d::Nearest,9984=>ui_scene::math::SceneTextureFilter3d::NearestMipmapNearest,9985=>ui_scene::math::SceneTextureFilter3d::LinearMipmapNearest,9986=>ui_scene::math::SceneTextureFilter3d::NearestMipmapLinear,9987=>ui_scene::math::SceneTextureFilter3d::LinearMipmapLinear,_=>ui_scene::math::SceneTextureFilter3d::Linear};ui_scene::math::SceneTextureSampler3d{wrap_u:wrap(value("wrapS",10497)),wrap_v:wrap(value("wrapT",10497)),mag_filter:filter(value("magFilter",9729)),min_filter:filter(value("minFilter",9729))}};
        let scalar=|name:&str,default:f32|find(name).and_then(|value|value.as_f64()).map_or(default,|value|value as f32);
        let alpha=match find("alphaMode").and_then(|value|value.as_str()).unwrap_or("OPAQUE"){"OPAQUE"=>SceneMaterialAlpha3d::Opaque,"MASK"=>SceneMaterialAlpha3d::Mask,"BLEND"=>SceneMaterialAlpha3d::Blend,_=>return Err(ui_scene::math::Mesh3dFault::Schema)};
        let mut color=tuple("baseColor",[1.0;4],4)?;if alpha==SceneMaterialAlpha3d::Opaque{color[3]=1.0;}let emissive=tuple("emissive",[0.0;4],3)?;
        let reference=|field:&str,role:usize|find(field).and_then(|value|value.as_str()).map(|id|format!("inline:{}:{}:{}",self.key,id,if role==0||role==4{"srgb"}else{"linear"}));
        let material=SceneAuthoredMaterial3d{base_color:color,emissive:[emissive[0],emissive[1],emissive[2]],metalness:scalar("metallic",1.0),roughness:scalar("roughness",1.0),alpha,alpha_cutoff:scalar("alphaCutoff",0.5),double_sided:find("doubleSided").and_then(|value|value.as_bool()).unwrap_or(false),preserve_vertex_color:self.schema.colors>0,base_color_texture:reference("baseColorTexture",0),texture_sampler:sampler(0),metallic_roughness_texture:reference("metallicRoughnessTexture",1),normal_texture:reference("normalTexture",2),occlusion_texture:reference("occlusionTexture",3),emissive_texture:reference("emissiveTexture",4),additional_texture_samplers:[sampler(1),sampler(2),sampler(3),sampler(4)],normal_scale:find("normalScale").and_then(|value|value.as_array()).map(|values|[values[0].as_f64().unwrap_or(1.0) as f32,values[1].as_f64().unwrap_or(1.0) as f32]).unwrap_or([scalar("normalScale",1.0);2]),occlusion_strength:scalar("occlusionStrength",1.0),};
        if let Some(previous)=self.primitives.last_mut().filter(|previous|previous.material==material){previous.index_count+=3;}else{if self.primitives.len()==WORLD_DYNAMIC_MESH_CAPACITY{return Err(ui_scene::math::Mesh3dFault::ItemCapacity)}self.primitives.push(World3dPrimitiveMaterial{first_index:self.item,index_count:3,material});}Ok(())
    }

    fn texture_step(&mut self)->Result<bool,ui_scene::math::Mesh3dFault> {
        if self.texture_retirement.is_some(){self.retire_texture_job()?;return Ok(false)}
        let WorldMeshInput::Inline(mesh)=&mut self.source else{return Ok(true)};
        if self.texture_name.is_none(){let Some(id)=mesh.textures.keys().nth(self.texture_cursor)else{return Ok(true)};self.texture_name=Some(id.clone());self.texture_scan=0;self.texture_required=false;}
        let id=self.texture_name.as_ref().unwrap();let role=if self.texture_role==0 {"srgb"}else{"linear"};let key=format!("inline:{}:{id}:{role}",self.key);
        if key.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY{return Err(ui_scene::math::Mesh3dFault::ByteCapacity)}
        if self.texture_scan<self.primitives.len(){let material=&self.primitives[self.texture_scan].material;let references=if self.texture_role==0 {[material.base_color_texture.as_ref(),material.emissive_texture.as_ref(),None]}else{[material.metallic_roughness_texture.as_ref(),material.normal_texture.as_ref(),material.occlusion_texture.as_ref()]};self.texture_required|=references.into_iter().flatten().any(|reference|reference==&key);self.texture_scan+=1;return Ok(false)}
        if !self.texture_required {self.next_texture_role();return Ok(false)}
        if self.texture_image.is_none() {
            if let Some(submission)=self.texture_submission.take(){
                let cores=if cfg!(target_arch="wasm32"){1}else{std::thread::available_parallelism().map_or(1,std::num::NonZeroUsize::get)};
                let pool=semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative,cores));
                if let Err(refusal)=pool.try_submit(semio_framework_async::Lane::Maintenance,submission){self.texture_submission=Some(refusal.into_job());}return Ok(false)
            }
            if let Some(worker)=self.texture_worker.as_ref(){let result=worker.lock().map_err(|_|ui_scene::math::Mesh3dFault::Schema)?.take();let Some(image)=result else{return Ok(false)};self.texture_image=Some(image.ok_or(ui_scene::math::Mesh3dFault::Schema)?);self.texture_worker=None;return Ok(false)}
            if self.texture_job.is_none() {
                let texture=mesh.textures.get(id).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
                if self.texture_byte<texture.bytes.len(){if self.texture_byte==0{self.texture_bytes.try_reserve_exact(texture.bytes.len()).map_err(|_|ui_scene::math::Mesh3dFault::ByteCapacity)?;}let end=(self.texture_byte+4096).min(texture.bytes.len());let bytes=&texture.bytes[self.texture_byte..end];self.texture_bytes.extend_from_slice(bytes);self.texture_hash.update(bytes);self.texture_byte=end;return Ok(false)}
                let digest=std::mem::take(&mut self.texture_hash).finalize();self.texture_digest=[u64::from_be_bytes(digest[..8].try_into().unwrap()),u64::from_be_bytes(digest[8..16].try_into().unwrap())];
                if texture.mime=="image/png" {match semio_framework_pixels::png_decoding::PngDecodeJob::new(semio_framework_pixels::png_decoding::PngDecodeInput{data:std::mem::take(&mut self.texture_bytes),max_pixels:WORLD_REFERENCE_SOURCE_PIXELS as usize,max_bytes:16_000_000,max_chunks:65536}){Ok(job)=>self.texture_job=Some(job),Err((_,input))=>{self.texture_bytes=input.data;return Err(ui_scene::math::Mesh3dFault::Schema)}}}
                else if texture.mime=="image/jpeg" {
                    let worker=std::sync::Arc::new(Mutex::new(None));let output=worker.clone();let canceled=self.texture_cancelled.clone();let bytes=std::mem::take(&mut self.texture_bytes);
                    self.texture_submission=Some(Box::new(move ||{let image=if canceled.load(std::sync::atomic::Ordering::Acquire){None}else{decode_mesh_surface_image_bytes(&bytes)};*output.lock().unwrap_or_else(|poisoned|poisoned.into_inner())=Some(image);}));self.texture_worker=Some(worker);return Ok(false)
                }else{return Err(ui_scene::math::Mesh3dFault::Schema)}
            }
            let job=self.texture_job.as_mut().unwrap();
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:job.next_copy_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Schema)?.max(32)*4096,maximum_capacity_bytes:job.next_capacity_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Schema)?,maximum_release_bytes:0,maximum_depth:1};
            let (progress,receipt)=job.advance(grant).map_err(|_|ui_scene::math::Mesh3dFault::Schema)?;
            if !receipt.fits(grant){return Err(ui_scene::math::Mesh3dFault::Schema)}
            if progress.done {
                let grant=world_mesh_ownership_grant(size_of::<semio_framework_pixels::RasterImage>(),0,0,1);
                let(image,receipt)=job.take_result(grant).map_err(|_|ui_scene::math::Mesh3dFault::Schema)?.ok_or(ui_scene::math::Mesh3dFault::Schema)?;
                if !receipt.fits(grant){return Err(ui_scene::math::Mesh3dFault::Schema)}
                self.texture_image=Some(image);
                self.retire_texture_job()?;
            }
            return Ok(false)
        }
        let image=self.texture_image.as_ref().unwrap();
        let descriptor=SceneRasterDescriptor{width:image.width,height:image.height,source_digest:self.texture_digest,source_revision:1,profile:if self.texture_role==0{SceneRasterProfile::MeshBaseColorSrgb}else{SceneRasterProfile::MeshNumericLinear},mesh:None};
        let pool=world_scene_raster_pool();let owner=self.texture_digest[0].wrapping_add(self.texture_digest[1]).max(1);
        let lease=match pool.begin(descriptor,owner,SceneRasterWriteMode::Moved { capacity_bytes: image.pixels.capacity() }){SceneRasterBegin::Reused(lease)=>{self.texture_image=None;lease},SceneRasterBegin::Writer(writer)=>{let image=self.texture_image.take().unwrap();match pool.prepare_moved(writer,image.pixels).and_then(|prepared|pool.seal_prepared_moved(prepared)){Ok(lease)=>lease,Err((pixels,_))=>{self.texture_image=Some(semio_framework_pixels::RasterImage{width:descriptor.width,height:descriptor.height,pixels});return Err(ui_scene::math::Mesh3dFault::ByteCapacity)}}},SceneRasterBegin::Backpressure(_)=>return Err(ui_scene::math::Mesh3dFault::ByteCapacity),SceneRasterBegin::Refused(_)=>return Err(ui_scene::math::Mesh3dFault::Schema)};
        self.textures.push((key,lease));self.next_texture_role();Ok(false)
    }

    fn next_texture_role(&mut self){self.texture_role+=1;if self.texture_role==2{self.texture_role=0;self.texture_cursor+=1;}self.texture_name=None;self.texture_byte=0;self.texture_scan=0;self.texture_required=false;}

    pub fn token(&self) -> Result<Mesh3dWriteToken, ui_scene::math::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_scene::math::Mesh3dFault::Stale),
        }
    }

    pub fn step(&mut self) -> WorldPlaceholderMeshStep {
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() { WorldPlaceholderMeshStep::Fault } else { WorldPlaceholderMeshStep::Pending };
        }
        let result = self.step_live();
        if result.is_err() {
            self.faulted = true;
            let _ = self.close_step();
            return WorldPlaceholderMeshStep::Pending;
        }
        result.expect("placeholder result checked above")
    }

    pub fn step_live(&mut self) -> Result<WorldPlaceholderMeshStep, ui_scene::math::Mesh3dFault> {
        match self.phase {
            WorldPlaceholderMeshPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.phase = WorldPlaceholderMeshPhase::Positions;
                }
            }
            WorldPlaceholderMeshPhase::Positions => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, self.source.position(self.source_vertex(self.item)))?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Normals;
                }
            }
            WorldPlaceholderMeshPhase::Normals => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, self.sample::<3>(semio_framework::MeshAttributeSemantic::Normal,self.item,self.source.normal(self.source_vertex(self.item)))?)?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Tangents;
                }
            }
            WorldPlaceholderMeshPhase::Tangents=>{
                if self.schema.tangents>0 {
                    let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_scene::math::Mesh3dFault::Schema)};
                    let attribute=mesh.attributes.get("tangent").ok_or(ui_scene::math::Mesh3dFault::Schema)?;
                    let mut value=[1.0,0.0,0.0,1.0];
                    if !(self.expanded && self.item>=self.index_items && attribute.domain!=semio_framework::MeshAttributeDomain::Vertex) {
                        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(self.item) as usize,semio_framework::MeshAttributeDomain::Corner=>self.item as usize,semio_framework::MeshAttributeDomain::Face=>self.item as usize/3,_=>return Err(ui_scene::math::Mesh3dFault::Schema)};
                        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==4).ok_or(ui_scene::math::Mesh3dFault::Schema)?;
                        for axis in 0..4 {value[axis]=values[axis].as_f64().filter(|value|value.is_finite() && value.abs()<=f32::MAX as f64).ok_or(ui_scene::math::Mesh3dFault::Schema)? as f32;}
                    }
                    if value[..3].iter().all(|value|*value==0.0) || value[3].abs()!=1.0{return Err(ui_scene::math::Mesh3dFault::Schema)}
                    mesh3d_write_vec4(self.token()?,Mesh3dField::Tangents,value)?;self.item+=1;
                }
                if self.schema.tangents==0 || self.item==self.vertex_items{self.item=0;self.phase=WorldPlaceholderMeshPhase::Uvs;}
            }
            WorldPlaceholderMeshPhase::Uvs => {
                if self.schema.uvs>0 {let role=self.uv_role;let value=self.uv_sample(role,self.item)?;ui_scene::math::mesh3d_write_vec2(self.token()?,[Mesh3dField::Uvs,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive][role],value)?;self.uv_role+=1;if self.uv_role==5{self.uv_role=0;self.item+=1;}}
                if self.schema.uvs==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Colors;}
            }
            WorldPlaceholderMeshPhase::Colors => {
                let token=self.token()?;
                if self.schema.colors>0 {let fallback=match &self.source {WorldMeshInput::Inline(mesh)=>{let at=self.source_vertex(self.item) as usize*4;std::array::from_fn(|axis|mesh.colors.get(at+axis).copied().unwrap_or(1.0))},_=>[1.0;4]};mesh3d_write_vec4(token,Mesh3dField::Colors,self.sample::<4>(semio_framework::MeshAttributeSemantic::Color,self.item,fallback)?)?;self.item+=1;}
                if self.schema.colors==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Indices;}
            }
            WorldPlaceholderMeshPhase::Indices => {
                if self.item < self.index_items {
                    mesh3d_write_u32(self.token()?, Mesh3dField::Indices, if self.expanded {self.item}else{self.source.index(self.item)})?;
                    self.item += 1;
                } else {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::FaceIds;
                }
            }
            WorldPlaceholderMeshPhase::FaceIds => {
                if self.item < self.schema.face_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_scene::math::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::FaceIds,mesh.face_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::VertexIds;}
            }
            WorldPlaceholderMeshPhase::VertexIds => {
                if self.item < self.schema.vertex_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_scene::math::Mesh3dFault::Schema)};let id=*mesh.vertex_ids.get(self.source_vertex(self.item) as usize).ok_or(ui_scene::math::Mesh3dFault::Schema)?;mesh3d_write_u32(self.token()?,Mesh3dField::VertexIds,id)?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Edges;}
            }
            WorldPlaceholderMeshPhase::Edges => {
                if self.item < self.schema.edges {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_scene::math::Mesh3dFault::Schema)};let at=self.item as usize*6;ui_scene::math::mesh3d_write_edge(self.token()?,[[mesh.edge_positions[at],mesh.edge_positions[at+1],mesh.edge_positions[at+2]],[mesh.edge_positions[at+3],mesh.edge_positions[at+4],mesh.edge_positions[at+5]]])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::EdgeIds;}
            }
            WorldPlaceholderMeshPhase::EdgeIds => {
                if self.item < self.schema.edge_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_scene::math::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::EdgeIds,mesh.edge_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Materials;}
            }
            WorldPlaceholderMeshPhase::Materials => {
                if self.item==self.index_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Textures;}
                else {self.material_step()?;self.item+=3;}
            }
            WorldPlaceholderMeshPhase::Textures => {
                if self.texture_step()? {
                    if !self.primitives.is_empty() {self.appearance=Some(World3dMeshAppearance::new(std::mem::take(&mut self.primitives),std::mem::take(&mut self.textures)).map_err(|(primitives,textures)|{self.primitives=primitives;self.textures=textures;ui_scene::math::Mesh3dFault::ByteCapacity})?);}
                    self.phase=WorldPlaceholderMeshPhase::Provenance;
                }
            }
            WorldPlaceholderMeshPhase::Provenance => {
                let token = self.token()?;
                if let WorldMeshInput::Inline(mesh) = &mut self.source {
                    let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_component_capacity_byte_demand(&mesh.component_references)?, 0, 1);
                    let progress = ui_scene::math::mesh3d_move_component_references(token, &mut mesh.component_references, grant)?;
                    if !progress.fits(grant) { return Err(ui_scene::math::Mesh3dFault::ByteCapacity); }
                }
                self.phase = WorldPlaceholderMeshPhase::AdmitProvenance;
            }
            WorldPlaceholderMeshPhase::AdmitProvenance => {
                let grant = world_mesh_ownership_grant(ui_scene::math::MESH3D_PAGE_BYTES, 0, 0, 1);
                let step = ui_scene::math::mesh3d_component_admission_step(self.token()?, grant)?;
                if !step.ownership.fits(grant) { return Err(ui_scene::math::Mesh3dFault::Closing); }
                if step.complete { self.phase = WorldPlaceholderMeshPhase::Seal; }
            }
            WorldPlaceholderMeshPhase::Seal => {
                let lease = mesh3d_seal(self.token()?)?;
                self.owner = WorldPlaceholderOwner::Ready(lease);
                self.phase = WorldPlaceholderMeshPhase::Publish;
            }
            WorldPlaceholderMeshPhase::Publish => {
                let grant=self.source_retirement_grant(4096)?;
                if !matches!(self.retire_source_step(grant)?,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){return Ok(WorldPlaceholderMeshStep::Pending)}
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_scene::math::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                return Ok(WorldPlaceholderMeshStep::Ready(std::mem::take(&mut self.key), lease,self.appearance.take()));
            }
        }
        Ok(WorldPlaceholderMeshStep::Pending)
    }

    pub fn source_retirement_grant(&self,copy_bytes:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,ui_scene::math::Mesh3dFault> {
        let Some(owner)=self.source_retirement.as_ref() else{return Ok(world_mesh_ownership_grant(0,0,0,usize::from(matches!(self.source,WorldMeshInput::Inline(_)))))};
        owner.next_copy_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?;
        Ok(world_mesh_ownership_grant(copy_bytes,owner.next_capacity_byte_demand(copy_bytes).map_err(|_|ui_scene::math::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?))
    }

    pub fn retire_source_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ui_scene::math::Mesh3dFault> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if let Some(owner)=self.source_retirement.as_mut(){
            let step=owner.step(grant).map_err(|_|ui_scene::math::Mesh3dFault::Closing)?;
            let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};
            if !progress.fits(grant){return Err(ui_scene::math::Mesh3dFault::Closing)}
            if matches!(step,RetainedCloneStep::Complete(_)){
                if !owner.terminal_is_empty(){return Err(ui_scene::math::Mesh3dFault::Closing)}
                self.source_retirement=None;
            }
            return Ok(step)
        }
        if !matches!(self.source,WorldMeshInput::Inline(_)){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
        if grant.maximum_items==0 || grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))}
        let WorldMeshInput::Inline(mesh)=std::mem::replace(&mut self.source,WorldMeshInput::Placeholder(WorldPlaceholderKind::Plane)) else{unreachable!()};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(mesh){
            Ok(owner)=>{self.source_retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..RetainedCloneProgress::default()}))},
            Err((_,mesh))=>{self.source=WorldMeshInput::Inline(mesh);Err(ui_scene::math::Mesh3dFault::Closing)}
        }
    }

    fn retire_texture_job(&mut self)->Result<bool,ui_scene::math::Mesh3dFault> {
        if let Some(owner)=self.texture_retirement.as_mut(){
            let copy=owner.next_copy_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?;
            let grant=world_mesh_ownership_grant(copy,owner.next_capacity_byte_demand(copy).map_err(|_|ui_scene::math::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_scene::math::Mesh3dFault::Closing)?);
            let step=owner.step(grant).map_err(|_|ui_scene::math::Mesh3dFault::Closing)?;
            let receipt=match step{semio_framework_value::retained_clone::RetainedCloneStep::Progress(receipt)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(receipt)=>receipt};
            if !receipt.fits(grant){return Err(ui_scene::math::Mesh3dFault::Closing)}
            if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){self.texture_retirement=None;return Ok(true)}
            return Ok(false)
        }
        let Some(job)=self.texture_job.take() else{return Ok(true)};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(job){Ok(owner)=>{self.texture_retirement=Some(owner);Ok(false)},Err((_,job))=>{self.texture_job=Some(job);Err(ui_scene::math::Mesh3dFault::Closing)}}
    }

    pub fn close_step(&mut self) -> bool {
        self.texture_cancelled.store(true,std::sync::atomic::Ordering::Release);
        if self.texture_submission.take().is_some(){self.texture_worker=None;return false}
        if let Some(worker)=self.texture_worker.as_ref(){let Ok(mut output)=worker.try_lock()else{return false};let Some(result)=output.take()else{return false};self.texture_image=result;drop(output);self.texture_worker=None;return false}
        if self.texture_bytes.pop().is_some(){return false}
        if let Some(name)=self.texture_name.as_mut(){if name.pop().is_some(){return false}self.texture_name=None;return false}
        match self.source_retirement_grant(4096).and_then(|grant|self.retire_source_step(grant)){
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))=>{},
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(_))=>return false,
            Err(_)=>{self.faulted=true;return false}
        }
        if let Some(appearance)=self.appearance.as_mut(){if !appearance.close_step(){return false}self.appearance=None;return false}
        if self.primitives.pop().is_some()||self.textures.pop().is_some(){return false}
        if let Some(job)=self.texture_job.as_mut(){job.cancel();}
        match self.retire_texture_job(){Ok(true)=>{},Ok(false)=>return false,Err(_)=>{self.faulted=true;return false}}
        if let Some(image)=self.texture_image.as_mut(){if image.pixels.pop().is_some(){return false}self.texture_image=None;return false}

        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_scene::math::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_scene::math::mesh3d_abort_demands(token, ui_scene::math::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_scene::math::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_scene::math::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_scene::math::mesh3d_close_demands(lease, ui_scene::math::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_scene::math::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.key.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(_) => false,
            WorldPlaceholderOwner::Ready(lease) => mesh3d_terminal_is_empty(lease),
            WorldPlaceholderOwner::Empty => self.key.is_empty() && self.source_retirement.is_none() && !matches!(self.source,WorldMeshInput::Inline(_)) && self.appearance.is_none() && self.primitives.is_empty() && self.textures.is_empty() && self.texture_job.is_none() && self.texture_retirement.is_none() && self.texture_image.is_none() && self.texture_worker.is_none() && self.texture_submission.is_none() && self.texture_bytes.is_empty() && self.texture_name.is_none(),
        }
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldPlaceholderMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "placeholder mesh cursor reached Drop before its exact authority witness");
    }
}

pub fn placeholder_triangle_normal(triangle: [[f32; 3]; 3]) -> [f32; 3] {
    let ab = [triangle[1][0] - triangle[0][0], triangle[1][1] - triangle[0][1], triangle[1][2] - triangle[0][2]];
    let ac = [triangle[2][0] - triangle[0][0], triangle[2][1] - triangle[0][1], triangle[2][2] - triangle[0][2]];
    placeholder_normalize3([ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]])
}

pub fn placeholder_triangle(kind: WorldPlaceholderKind, triangle: u32) -> [[f32; 3]; 3] {
    match kind {
        WorldPlaceholderKind::Box => {
            let faces = [
                ([-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]),
                ([0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]),
                ([-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]),
                ([-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]),
                ([0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
                ([-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]),
            ];
            let (a, b, c, d) = faces[(triangle / 2) as usize];
            if triangle.is_multiple_of(2) {
                [a, b, c]
            } else {
                [a, c, d]
            }
        }
        WorldPlaceholderKind::Plane => {
            if triangle == 0 {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, -0.5], [0.5, 0.0, 0.5]]
            } else {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, 0.5], [-0.5, 0.0, 0.5]]
            }
        }
        WorldPlaceholderKind::Cylinder => {
            let segment = triangle / 4;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p00 = [0.5 * a0.cos(), -0.5, 0.5 * a0.sin()];
            let p01 = [0.5 * a1.cos(), -0.5, 0.5 * a1.sin()];
            let p10 = [0.5 * a0.cos(), 0.5, 0.5 * a0.sin()];
            let p11 = [0.5 * a1.cos(), 0.5, 0.5 * a1.sin()];
            match triangle % 4 {
                0 => [p00, p01, p11],
                1 => [p00, p11, p10],
                2 => [[0.0, -0.5, 0.0], p01, p00],
                _ => [[0.0, 0.5, 0.0], p10, p11],
            }
        }
        WorldPlaceholderKind::Cone => {
            let segment = triangle / 2;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p0 = [0.5 * a0.cos(), 0.0, 0.5 * a0.sin()];
            let p1 = [0.5 * a1.cos(), 0.0, 0.5 * a1.sin()];
            if triangle.is_multiple_of(2) {
                [[0.0, 1.0, 0.0], p1, p0]
            } else {
                [[0.0, 0.0, 0.0], p0, p1]
            }
        }
        WorldPlaceholderKind::Icosphere => {
            let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
            let vertices = [[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
            let faces =
                [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
            let face = faces[(triangle / 4) as usize];
            let a = placeholder_normalize3(vertices[face[0]]);
            let b = placeholder_normalize3(vertices[face[1]]);
            let c = placeholder_normalize3(vertices[face[2]]);
            let ab = placeholder_normalize3([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]);
            let bc = placeholder_normalize3([(b[0] + c[0]) * 0.5, (b[1] + c[1]) * 0.5, (b[2] + c[2]) * 0.5]);
            let ca = placeholder_normalize3([(c[0] + a[0]) * 0.5, (c[1] + a[1]) * 0.5, (c[2] + a[2]) * 0.5]);
            let triangle = match triangle % 4 {
                0 => [a, ab, ca],
                1 => [b, bc, ab],
                2 => [c, ca, bc],
                _ => [ab, bc, ca],
            };
            triangle.map(|vertex| placeholder_scale3(vertex, 0.12))
        }
    }
}

//#region WorldMeshBuffers
pub type WorldMeshBuffers=semio_framework::MeshData;

pub trait WorldMeshSurface {
    fn position(&self,vertex:u32)->[f32;3];
    fn normal(&self,vertex:u32)->[f32;3];
}

impl WorldMeshSurface for WorldMeshBuffers {
    fn position(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.positions.get(at).copied().unwrap_or(0.0),self.positions.get(at+1).copied().unwrap_or(0.0),self.positions.get(at+2).copied().unwrap_or(0.0)]}
    fn normal(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.normals.get(at).copied().unwrap_or(0.0),self.normals.get(at+1).copied().unwrap_or(0.0),self.normals.get(at+2).copied().unwrap_or(1.0)]}
}

pub fn placeholder_normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn placeholder_scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

//#region 🧵️WorldInteractionTransaction
pub fn world_mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    let point = mesh.vec3(Mesh3dField::Positions, index).ok()?;
    Some(Vec3::new(point[0], point[1], point[2]))
}

pub fn world_mesh_triangle(mesh: Mesh3dLease, triangle: u32) -> Option<[u32; 3]> {
    let base = triangle.checked_mul(3)?;
    Some([mesh.u32(Mesh3dField::Indices, base).ok()?, mesh.u32(Mesh3dField::Indices, base + 1).ok()?, mesh.u32(Mesh3dField::Indices, base + 2).ok()?])
}

pub fn world_mesh_component_id(mesh: Mesh3dLease, field: Mesh3dField, index: u32) -> u32 {
    mesh.u32(field, index).unwrap_or(if field == Mesh3dField::VertexIds { u32::MAX } else { index })
}

pub const WORLD_COMPONENT_TARGET_BYTE_CAPACITY: usize = WORLD_INTERACTION_ID_BYTE_CAPACITY + 1 + 6 + 1 + 10 + 1 + 64 + 1 + 20 + 1 + 64;

pub struct WorldComponentAddress<'a> {
    pub object: &'a str,
    pub group: u32,
    pub field: Mesh3dField,
    pub source: Option<(ComponentSource3d, u64)>,
}

impl<'a> WorldComponentAddress<'a> {
    pub fn parse(id: &'a str, kind: &str) -> Option<Self> {
        if id.len() > WORLD_COMPONENT_TARGET_BYTE_CAPACITY { return None; }
        let (address, source) = id.split_once('~').map_or((id, None), |(address, source)| (address, Some(source)));
        let (prefix, group) = address.rsplit_once('.')?;
        let (object, actual_kind) = prefix.rsplit_once('.')?;
        if actual_kind != kind || group.is_empty() || (group.len() > 1 && group.starts_with('0')) || !group.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
        let field = match kind { "vertex" => Mesh3dField::VertexIds, "edge" => Mesh3dField::EdgeIds, "face" => Mesh3dField::FaceIds, _ => return None };
        let source = match source {
            None => None,
            Some(source) => {
                let mut fields = source.split('~');
                let handle = fields.next()?; let label = fields.next()?; let revision = fields.next()?;
                if fields.next().is_some() || label.is_empty() || label.starts_with('0') || label.len() > 20 || !label.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
                Some((ComponentSource3d::new(handle, revision)?, label.parse().ok()?))
            }
        };
        Some(Self { object, group: group.parse().ok()?, field, source })
    }

    pub(crate) fn matches_source(&self, scene_state: &WorldSceneState, source: Option<ComponentSource3d>, mesh_id: &str) -> bool {
        match (self.source, source) {
            (None, None) => true,
            (Some((expected, label)), Some(actual)) if expected == actual => scene_state.meshes.get(mesh_id).is_some_and(|mesh| mesh.component_label(self.field, self.group).ok().flatten() == Some(label)),
            _ => false,
        }
    }

    pub fn matches_registry(&self, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, entry: &WorldInteractionObjectSlot) -> bool {
        let Some(mesh) = entry.mesh.and_then(|mesh| interaction_state.interaction_meshes.resolve(mesh)) else { return false; };
        self.matches_source(scene_state, entry.component_source, mesh.id.as_str())
    }
}
//#endregion 🧵️WorldInteractionTransaction
//#endregion WorldMeshBuffers
//#endregion PlaceholderMeshAuthority
//#endregion MeshHelpers
//#endregion WorldRelocateGesture
