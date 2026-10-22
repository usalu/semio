//! 🏔️ World3d environment lighting and shadows, terrain tiles and meshing, and the placeholder meshes.

use super::*;

//#region Environment
/// ☀️🎨️ Resolves the renderer's scene-pass light direction from `environment.sun` when the sun is
/// explicitly enabled (horizontal coordinate system, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// matching `sunPositionFromAzimuthElevation` in `ui/js/react/index.tsx`), else keeps the default.
pub fn environment_light_dir(environment: &WorldEnvironmentRecord) -> [f32; 3] {
    const DEFAULT_LIGHT_DIR: [f32; 3] = [0.4, 0.6, 0.8];
    let Some(sun) = environment.sun.as_ref() else {
        return DEFAULT_LIGHT_DIR;
    };
    if sun.enabled != Some(true) {
        return DEFAULT_LIGHT_DIR;
    }
    let azimuth = sun.azimuth.unwrap_or(45.0).to_radians();
    let elevation = sun.elevation.unwrap_or(35.0).to_radians();
    let direction = Vec3::new((elevation.cos() * azimuth.cos()) as f32, (elevation.cos() * azimuth.sin()) as f32, elevation.sin() as f32);
    if direction.length() < 1e-6 {
        DEFAULT_LIGHT_DIR
    } else {
        direction.normalize().to_array()
    }
}

/// 🎨️ Resolves the neutral `MeshStandardMaterial` parameters. Environment emissive is a neutral
/// color override in World3d, so it is active only beside `material.color`; IconRender supplies its
/// own neutral default color whenever a material record exists.
pub fn environment_scene_material(environment: &WorldEnvironmentRecord) -> SceneMaterial3d {
    let Some(material) = environment.material.as_ref() else {
        return SceneMaterial3d::default();
    };
    let emissive = material.color.as_ref().and(material.emissive.as_deref()).map(parse_color).map(|rgba| rgba[..3].try_into().expect("fixed rgba prefix")).unwrap_or([0.0; 3]);
    SceneMaterial3d { metalness: material.metalness.unwrap_or(0.0) as f32, roughness: material.roughness.unwrap_or(1.0) as f32, emissive, emissive_intensity: if emissive == [0.0; 3] { 0.0 } else { material.emissive_intensity.unwrap_or(1.0) as f32 } }
}

pub fn environment_outline_color(environment: &WorldEnvironmentRecord, theme: &semio_framework_ui_theme::Theme) -> Option<[f32; 4]> {
    match environment.material.as_ref().and_then(|material| material.stroke.as_deref()).map(str::trim) {
        Some(stroke) if stroke.eq_ignore_ascii_case("none") || stroke.eq_ignore_ascii_case("transparent") => None,
        Some(stroke) if !stroke.is_empty() => Some(parse_color(stroke)),
        _ => Some([theme.border_normal.r, theme.border_normal.g, theme.border_normal.b, theme.border_normal.a]),
    }
}

/// 🌑️ The two React shadow-map consumers and the SVG/material-absent exclusion path.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dShadowProfile {
    World,
    IconPng,
    IconSvg,
    Unshadowed,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dShadowGeometry {
    Glb,
    Terrain,
    Paint,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn world3d_shadow_role(geometry: World3dShadowGeometry, enabled: bool) -> SceneShadowRole3d {
    if !enabled {
        return SceneShadowRole3d::default();
    }
    match geometry {
        World3dShadowGeometry::Glb => SceneShadowRole3d { casts: true, receives: true },
        World3dShadowGeometry::Terrain => SceneShadowRole3d { casts: false, receives: true },
        World3dShadowGeometry::Paint => SceneShadowRole3d::default(),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl World3dShadowProfile {
    pub fn map_size(self) -> u32 {
        match self {
            Self::World | Self::IconSvg | Self::Unshadowed => WORLD_SHADOW_MAP_SIZE,
            Self::IconPng => ICON_SHADOW_MAP_SIZE,
        }
    }

    pub fn render_profile(self) -> ui_scene::math::SceneRenderProfile3d {
        match self {
            Self::IconSvg => ui_scene::math::SceneRenderProfile3d::SvgFlatLit,
            Self::World | Self::IconPng | Self::Unshadowed => ui_scene::math::SceneRenderProfile3d::RasterPbr,
        }
    }
}

/// 🌑️ Resolves only the controls React consumes: enabled plus the consumer's map profile.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn environment_scene_shadow(environment: &WorldEnvironmentRecord, light_dir: [f32; 3], profile: World3dShadowProfile) -> SceneShadow3d {
    let record = environment.shadow.as_ref();
    let enabled = matches!(profile, World3dShadowProfile::World | World3dShadowProfile::IconPng)
        && record.and_then(|shadow| shadow.enabled) == Some(true)
        && environment.sun.as_ref().and_then(|sun| sun.enabled) == Some(true);
    SceneShadow3d { enabled, map_size: profile.map_size(), view_proj: directional_shadow_view_projection(light_dir) }
}

//#region Terrain
#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct TerrainVisibleTileRow {
    pub z: u32,
    pub x: u32,
    pub y: u32,
}

pub fn terrain_tile_url(template: &str, z: u32, x: u32, y: u32) -> String {
    template.replace("{z}", &z.to_string()).replace("{x}", &x.to_string()).replace("{y}", &y.to_string())
}

pub fn terrain_tile_mesh_key(surface_id: &str, z: u32, x: u32, y: u32) -> String {
    format!("terrain:{surface_id}:{z}:{x}:{y}")
}

/// 🏔️📡️ Admits one visible-but-unloaded DEM tile into the bounded world-asset pipeline
/// (`WorldAssetRequestKind::Terrain`), the same lane GLB meshes and reference images travel — the
/// renderer host drains it (`take_next_world3d_asset`) and hands the bytes back through
/// [`apply_world3d_terrain_tile_bytes`]. Mirrors `TerrainTileRenderer.uploadOne` in
/// `🗺️WorldTerrainLayer/🟦️.tsx`, including its `tileMiss` set: a tile whose bytes the terrain
/// session refuses is never re-requested for this style.
///
/// 📡️ A full request table is back-pressure, not a fault: the next frame re-offers the tile.
pub fn reserve_terrain_tile_fetch(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, tile_url_template: &str, (z, x, y): (u32, u32, u32)) {
    if terrain_state.terrain_tile_misses.contains(&(z, x, y)) {
        return;
    }
    let url = terrain_tile_url(tile_url_template, z, x, y);
    if terrain_state.pending_terrain_tile_urls.contains_key(&url) {
        return;
    }
    match reserve_world3d_asset_request(session_state, assets_state, WorldAssetRequestKind::Terrain { z, x, y }, &url) {
        Ok(_) => {
            terrain_state.pending_terrain_tile_urls.insert(url, (z, x, y));
        }
        Err(WorldAssetFault::ItemCapacity | WorldAssetFault::ByteCapacity | WorldAssetFault::Closing | WorldAssetFault::Stale) => {}
        Err(_) => {
            terrain_state.terrain_tile_misses.insert((z, x, y));
        }
    }
}

/// 🏔️📥️ Decodes one fetched DEM tile into the terrain session — the wgpu twin of
/// `TerrainTileRenderer.uploadOne`'s `upload_elevation_tile` call. A refusal (not a terrarium PNG)
/// records a tile miss so the pipeline stops re-requesting it; the banded meshes are built by the
/// next `sync_terrain_state` pass, which now sees a non-`null` `terrain_tile_mesh_json`.
pub fn apply_world3d_terrain_tile_bytes(terrain_state: &mut WorldTerrainState, z: u32, x: u32, y: u32, bytes: &[u8]) -> bool {
    let uploaded = terrain_state.terrain_session.upload_elevation_tile(z, x, y, bytes);
    if let Some(style) = terrain_state.terrain_style.as_ref() {
        terrain_state.pending_terrain_tile_urls.remove(&terrain_tile_url(&style.tile_url_template, z, x, y));
    }
    if !uploaded {
        terrain_state.terrain_tile_misses.insert((z, x, y));
    }
    uploaded
}

/// 🎨️ Vertical hypsometric ramp — same stops as `getHypsometricTexture` in `world-terrain-layer.tsx`
/// (green low ground -> tan -> grey -> white peaks), sampled at a band's center elevation ratio.
pub fn hypsometric_color(t: f32) -> [f32; 4] {
    let stops: [(f32, [f32; 3]); 4] =
        [(0.0, [0x4b as f32 / 255.0, 0x6b as f32 / 255.0, 0x3a as f32 / 255.0]), (0.5, [0xa6 as f32 / 255.0, 0x8a as f32 / 255.0, 0x5b as f32 / 255.0]), (0.85, [0x8f as f32 / 255.0, 0x8f as f32 / 255.0, 0x8f as f32 / 255.0]), (1.0, [1.0, 1.0, 1.0])];
    let t = t.clamp(0.0, 1.0);
    for window in stops.windows(2) {
        let (t0, c0) = window[0];
        let (t1, c1) = window[1];
        if t <= t1 {
            let f = if (t1 - t0).abs() < 1e-6 { 0.0 } else { (t - t0) / (t1 - t0) };
            return [c0[0] + (c1[0] - c0[0]) * f, c0[1] + (c1[1] - c0[1]) * f, c0[2] + (c1[2] - c0[2]) * f, 1.0];
        }
    }
    [1.0, 1.0, 1.0, 1.0]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldTerrainMeshPhase {
    Begin,
    Allocate,
    Positions,
    Normals,
    Colors,
    Indices,
    Seal,
    Publish,
    RetireSource,
}

/// 🏔️ The terrain tile source buffers a retiring mesh cursor releases through one controlled owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct WorldTerrainSource {
    pub id: String,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub uvs: Vec<f32>,
}

pub struct WorldTerrainMeshCursor {
    pub surface_id: String,
    pub z: u32,
    pub x: u32,
    pub y: u32,
    pub payload: TerrainTileMeshPayload,
    pub generation: u64,
    pub source_revision: u64,
    pub terrain_revision: u64,
    pub phase: WorldTerrainMeshPhase,
    pub triangle: u32,
    pub vertex: u8,
    pub item: u32,
    pub owner: WorldPlaceholderOwner,
    pub close_started: bool,
    pub source_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<WorldTerrainSource>>,
    pub faulted: Option<WorldDynamicFault>,
}

pub enum WorldTerrainMeshStep {
    Pending,
    Ready(String, Mesh3dLease),
    Complete((u32, u32, u32)),
    Fault(WorldDynamicFault),
}

impl WorldTerrainMeshCursor {

    pub fn new(surface_id: &str, (z,x,y):(u32,u32,u32), payload:TerrainTileMeshPayload, generation:u64, source_revision:u64, terrain_revision:u64, grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self,semio_framework_value::retained_clone::RetainedCloneProgress),(WorldDynamicFault,TerrainTileMeshPayload)> {
        if surface_id.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY || !payload.positions.len().is_multiple_of(3) || !payload.normals.len().is_multiple_of(3) || !payload.indices.len().is_multiple_of(3) || !payload.uvs.len().is_multiple_of(2) {
            return Err((if surface_id.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY {WorldDynamicFault::IdCapacity} else {WorldDynamicFault::ByteCapacity},payload));
        }
        if u32::try_from(payload.indices.len()/3).is_err() {return Err((WorldDynamicFault::InstanceCapacity,payload));}
        let bytes=surface_id.len();
        if grant.maximum_items==0 || grant.maximum_depth==0 || bytes>grant.maximum_copy_bytes || bytes>grant.maximum_capacity_bytes {return Err((WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::OwnershipLimit),payload));}
        let mut id=match terrain_string_capacity(bytes) {Ok(id)=>id,Err(fault)=>return Err((fault,payload))};
        id.push_str(surface_id);
        Ok((Self{surface_id:id,z,x,y,payload,generation,source_revision,terrain_revision,phase:WorldTerrainMeshPhase::Begin,triangle:0,vertex:0,item:0,owner:WorldPlaceholderOwner::Empty,close_started:false,source_retirement:None,faulted:None},semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0}))
    }

    fn triangle_count(&self) -> u32 {
        (self.payload.indices.len() / 3) as u32
    }

    fn triangle_indices(&self, triangle: u32) -> Result<[u32; 3], WorldDynamicFault> {
        let start = usize::try_from(triangle).ok().and_then(|triangle| triangle.checked_mul(3)).ok_or(WorldDynamicFault::InstanceCapacity)?;
        let values = self.payload.indices.get(start..start + 3).ok_or(WorldDynamicFault::StaleToken)?;
        let result = [values[0], values[1], values[2]];
        if result.iter().any(|index| usize::try_from(*index).ok().and_then(|index| index.checked_mul(3)).is_none_or(|offset| self.payload.positions.get(offset..offset + 3).is_none())) {
            return Err(WorldDynamicFault::StaleToken);
        }
        Ok(result)
    }

    /// 🎨️ The hypsometric colour of ONE vertex of a triangle, sampled from the tile mesh's own
    /// per-vertex elevation ratio (`uvs[i * 2 + 1]`, what `build_terrain_tile_mesh` writes there and
    /// what React's `getHypsometricTexture` samples with) — a continuous ramp, not a band bucket.
    fn vertex_color(&self, triangle: u32, vertex: u8) -> Result<[f32; 4], WorldDynamicFault> {
        let index = self.triangle_indices(triangle)?[usize::from(vertex)] as usize;
        let elevation = self.payload.uvs.get(index * 2 + 1).copied().unwrap_or(0.0);
        if !elevation.is_finite() {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        Ok(hypsometric_color(elevation))
    }

    pub fn value(&self, triangle: u32, vertex: u8, normal: bool) -> Result<[f32; 3], WorldDynamicFault> {
        let index = self.triangle_indices(triangle)?[usize::from(vertex)] as usize;
        let source = if normal { &self.payload.normals } else { &self.payload.positions };
        let offset = index.checked_mul(3).ok_or(WorldDynamicFault::ByteCapacity)?;
        let value = match source.get(offset..offset + 3) {
            Some(value) => [value[0], value[1], value[2]],
            None if normal => [0.0, 0.0, 1.0],
            None => return Err(WorldDynamicFault::StaleToken),
        };
        if value.iter().all(|value| value.is_finite()) {
            Ok(value)
        } else {
            Err(WorldDynamicFault::ByteCapacity)
        }
    }

    pub fn token(&self) -> Result<Mesh3dWriteToken, WorldDynamicFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(WorldDynamicFault::StaleToken),
        }
    }


    pub fn mesh_fault(fault: ui_scene::math::Mesh3dFault) -> WorldDynamicFault { WorldDynamicFault::Mesh(fault) }

    pub fn step(&mut self, source_revision: u64, terrain_revision: u64, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
        if context.should_yield() || grant.maximum_items == 0 { return Ok((WorldTerrainMeshStep::Pending, Default::default())); }
        context.set_stage("terrainMesh");
        if context.is_cancelled() { self.faulted=Some(WorldDynamicFault::Canceled);context.consume_fuel(1);return Ok((WorldTerrainMeshStep::Pending,Default::default())); }
        if source_revision != self.source_revision || terrain_revision != self.terrain_revision { self.faulted = Some(WorldDynamicFault::StaleToken); }
        let closing=self.faulted.is_some();
        let result = if let Some(fault) = self.faulted {
            self.close_step(grant).map(|step| (if step.complete { WorldTerrainMeshStep::Fault(fault) } else { WorldTerrainMeshStep::Pending }, step.progress))
        } else { self.step_live(grant) };
        context.consume_fuel(1);
        match result {
            Err(WorldDynamicFault::Mesh(ui_scene::math::Mesh3dFault::Busy)) => Ok((WorldTerrainMeshStep::Pending, Default::default())),
            Err(fault) if closing=>Err(fault),
            Err(fault) => { self.faulted = Some(fault); Ok((WorldTerrainMeshStep::Pending, Default::default())) }
            value => value,
        }
    }

    pub fn step_live(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(WorldTerrainMeshStep, semio_framework_value::retained_clone::RetainedCloneProgress), WorldDynamicFault> {
        let empty = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok((WorldTerrainMeshStep::Pending, empty)); }
        if grant.maximum_depth == 0 { return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::DepthLimit)); }
        let mut progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..empty };
        match self.phase {
            WorldTerrainMeshPhase::Begin => {
                let items = self.triangle_count().checked_mul(3).ok_or(WorldDynamicFault::InstanceCapacity)?;
                if items == 0 { self.phase = WorldTerrainMeshPhase::RetireSource; return Ok((WorldTerrainMeshStep::Pending, progress)); }
                let schema = Mesh3dSchema { vertices: items, indices: items, face_ids:0, vertex_ids:0, edges:0, edge_ids:0, uvs:0, colors:items, surface_uvs:[0;4], tangents:0 };
                let (token, receipt) = mesh3d_begin(self.generation, self.source_revision, schema, grant).map_err(Self::mesh_fault)?;
                self.owner = WorldPlaceholderOwner::Writing(token);
                progress = receipt;
                self.phase = WorldTerrainMeshPhase::Allocate;
            }
            WorldTerrainMeshPhase::Allocate => {
                let step = mesh3d_allocate_step(self.token()?, grant).map_err(Self::mesh_fault)?;
                progress = step.progress;
                if step.complete { self.phase = WorldTerrainMeshPhase::Positions; }
            }
            WorldTerrainMeshPhase::Positions | WorldTerrainMeshPhase::Normals | WorldTerrainMeshPhase::Colors => {
                if self.triangle == self.triangle_count() {
                    self.triangle=0; self.vertex=0;
                    self.phase=match self.phase { WorldTerrainMeshPhase::Positions=>WorldTerrainMeshPhase::Normals, WorldTerrainMeshPhase::Normals=>WorldTerrainMeshPhase::Colors, _=>WorldTerrainMeshPhase::Indices };
                    return Ok((WorldTerrainMeshStep::Pending,progress));
                }
                let bytes=if self.phase==WorldTerrainMeshPhase::Colors {16} else {12};
                if grant.maximum_copy_bytes<bytes { return Ok((WorldTerrainMeshStep::Pending,empty)); }
                match self.phase {
                    WorldTerrainMeshPhase::Colors=>mesh3d_write_vec4(self.token()?,Mesh3dField::Colors,self.vertex_color(self.triangle,self.vertex)?).map_err(Self::mesh_fault)?,
                    phase=>{ let normal=phase==WorldTerrainMeshPhase::Normals; mesh3d_write_vec3(self.token()?,if normal {Mesh3dField::Normals} else {Mesh3dField::Positions},self.value(self.triangle,self.vertex,normal)?).map_err(Self::mesh_fault)?; }
                }
                progress.copied_bytes=bytes;
                self.vertex+=1;
                if self.vertex==3 { self.vertex=0; self.triangle+=1; }
            }
            WorldTerrainMeshPhase::Indices=>{
                if grant.maximum_copy_bytes<4 { return Ok((WorldTerrainMeshStep::Pending,empty)); }
                mesh3d_write_u32(self.token()?,Mesh3dField::Indices,self.item).map_err(Self::mesh_fault)?;
                progress.copied_bytes=4; self.item+=1;
                if self.item==self.triangle_count()*3 { self.item=0; self.phase=WorldTerrainMeshPhase::Seal; }
            }
            WorldTerrainMeshPhase::Seal=>{
                let lease=mesh3d_seal(self.token()?).map_err(Self::mesh_fault)?;
                self.owner=WorldPlaceholderOwner::Ready(lease); self.phase=WorldTerrainMeshPhase::Publish;
            }
            WorldTerrainMeshPhase::Publish=>{
                let WorldPlaceholderOwner::Ready(lease)=self.owner else {return Err(WorldDynamicFault::StaleToken)};
                let bytes=8+self.surface_id.len()+3+terrain_number_bytes(self.z)+terrain_number_bytes(self.x)+terrain_number_bytes(self.y);
                if grant.maximum_copy_bytes<bytes || grant.maximum_capacity_bytes<bytes {return Ok((WorldTerrainMeshStep::Pending,empty));}
                let mut key=terrain_string_capacity(bytes)?;
                use std::fmt::Write;
                write!(&mut key,"terrain:{}:{}:{}:{}",self.surface_id,self.z,self.x,self.y).map_err(|_|WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated))?;
                progress.copied_bytes=bytes; progress.retained_capacity_bytes=key.capacity();
                self.owner=WorldPlaceholderOwner::Empty; self.phase=WorldTerrainMeshPhase::RetireSource;
                return Ok((WorldTerrainMeshStep::Ready(key,lease),progress));
            }
            WorldTerrainMeshPhase::RetireSource=>{
                let step=self.close_step(grant)?;
                return Ok((if step.complete { WorldTerrainMeshStep::Complete((self.z,self.x,self.y)) } else {WorldTerrainMeshStep::Pending},step.progress));
            }
        }
        if !progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
        Ok((WorldTerrainMeshStep::Pending,progress))
    }

    pub fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<ui_scene::math::Mesh3dOwnershipStep,WorldDynamicFault> {
        let empty=ui_scene::math::Mesh3dOwnershipStep::default();
        if self.terminal_is_empty() {return Ok(ui_scene::math::Mesh3dOwnershipStep {complete:true,..empty});}
        if grant.maximum_items==0 {return Ok(empty);}
        if grant.maximum_depth==0 {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::DepthLimit));}
        match self.owner {
            WorldPlaceholderOwner::Writing(token)=>{
                if !self.close_started {
                    match mesh3d_abort(token) {Ok(())|Err(ui_scene::math::Mesh3dFault::Closing)=>self.close_started=true,Err(fault)=>return Err(Self::mesh_fault(fault))}
                    return Ok(ui_scene::math::Mesh3dOwnershipStep{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()},..empty});
                }
                let step=mesh3d_abort_step(token,grant).map_err(Self::mesh_fault)?;
                if !step.progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
                if step.complete {self.owner=WorldPlaceholderOwner::Empty;self.close_started=false;}
                return Ok(ui_scene::math::Mesh3dOwnershipStep{complete:false,..step});
            }
            WorldPlaceholderOwner::Ready(lease)=>{
                if !self.close_started {
                    match mesh3d_begin_close(lease) {Ok(())|Err(ui_scene::math::Mesh3dFault::Closing)=>self.close_started=true,Err(fault)=>return Err(Self::mesh_fault(fault))}
                    return Ok(ui_scene::math::Mesh3dOwnershipStep{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()},..empty});
                }
                let step=mesh3d_close_step(lease,grant).map_err(Self::mesh_fault)?;
                if !step.progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
                if step.complete {self.owner=WorldPlaceholderOwner::Empty;self.close_started=false;}
                return Ok(ui_scene::math::Mesh3dOwnershipStep{complete:false,..step});
            }
            WorldPlaceholderOwner::Empty=>{}
        }
        if self.source_retirement.is_none() {
            let value=WorldTerrainSource{id:std::mem::take(&mut self.surface_id),positions:std::mem::take(&mut self.payload.positions),normals:std::mem::take(&mut self.payload.normals),indices:std::mem::take(&mut self.payload.indices),uvs:std::mem::take(&mut self.payload.uvs)};
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(value) {
                Ok(owner)=>self.source_retirement=Some(owner),
                Err((error,WorldTerrainSource{id,positions,normals,indices,uvs}))=>{
                    self.surface_id=id;self.payload.positions=positions;self.payload.normals=normals;self.payload.indices=indices;self.payload.uvs=uvs;
                    return Err(WorldDynamicFault::Ownership(error.kind));
                }
            }
        }
        let owner=self.source_retirement.as_mut().unwrap();
        let step=owner.step(grant).map_err(|error|WorldDynamicFault::Ownership(error.kind))?;
        let progress=match step {semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)=>progress};
        if !progress.fits(grant) {return Err(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
        if owner.terminal_is_empty() {self.source_retirement=None;}
        Ok(ui_scene::math::Mesh3dOwnershipStep{complete:self.terminal_is_empty(),progress})
    }

    pub fn terminal_is_empty(&self)->bool {
        matches!(self.owner,WorldPlaceholderOwner::Empty) && self.source_retirement.is_none() && self.payload.positions.capacity()==0 && self.payload.normals.capacity()==0 && self.payload.indices.capacity()==0 && self.payload.uvs.capacity()==0 && self.surface_id.capacity()==0
    }

}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldTerrainMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "terrain mesh cursor reached Drop before its exact page/source witness");
    }
}

fn terrain_string_capacity(bytes:usize)->Result<String,WorldDynamicFault> {
    if bytes==0 {return Ok(String::new());}
    let layout=std::alloc::Layout::array::<u8>(bytes).map_err(|_|WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::OwnershipLimit))?;
    let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}).ok_or(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::AllocationFailed))?;
    Ok(unsafe{String::from_raw_parts(pointer.as_ptr(),0,bytes)})
}

fn terrain_number_bytes(mut value:u32)->usize {let mut bytes=1;while value>=10{value/=10;bytes+=1;}bytes}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WorldTerrainMeshPublicationStep {Idle,Pending,Complete,Fault(WorldDynamicFault)}

#[derive(Clone,Copy,Debug)]
pub struct WorldTerrainMeshProgress {pub step:WorldTerrainMeshPublicationStep,pub ownership:semio_framework_value::retained_clone::RetainedCloneProgress}

pub fn step_world3d_terrain_retirement(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    context.set_stage("terrainMeshRetirement");
    if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_some() {
        match begin_world_terrain_pending(session_state, terrain_state,grant,true) {
            Ok(progress)=>{result.ownership=progress;result.step=WorldTerrainMeshPublicationStep::Pending;}
            Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
        }
        context.consume_fuel(1);return result;
    }
    let Some(cursor)=terrain_state.terrain_build.as_mut() else{return result};
    let step=cursor.close_step(grant);
    context.consume_fuel(1);
    match step {
        Ok(step)=>{
            result.ownership=step.progress;
            if !step.progress.fits(grant) {result.step=WorldTerrainMeshPublicationStep::Fault(WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));}
            else if step.complete {terrain_state.terrain_build=None;result.step=WorldTerrainMeshPublicationStep::Complete;}
            else {result.step=WorldTerrainMeshPublicationStep::Pending;}
        }
        Err(WorldDynamicFault::Mesh(ui_scene::math::Mesh3dFault::Busy))=>result.step=WorldTerrainMeshPublicationStep::Pending,
        Err(fault)=>result.step=WorldTerrainMeshPublicationStep::Fault(fault),
    }
    result
}

pub fn begin_world_terrain_pending(session_state: &mut WorldSessionState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,canceled:bool)->Result<semio_framework_value::retained_clone::RetainedCloneProgress,WorldDynamicFault> {
    let Some((tile,payload,generation,revision,terrain_revision))=terrain_state.terrain_pending.take() else{return Ok(Default::default())};
    match WorldTerrainMeshCursor::new(if canceled {""} else {&session_state.surface_id},tile,payload,generation,revision,terrain_revision,grant) {
        Ok((mut cursor,progress))=>{if canceled {cursor.faulted=Some(WorldDynamicFault::Canceled);}terrain_state.terrain_build=Some(cursor);Ok(progress)}
        Err((fault,payload))=>{terrain_state.terrain_pending=Some((tile,payload,generation,revision,terrain_revision));Err(fault)}
    }
}

#[cfg(any(test, feature = "testkit"))]
pub struct TerrainMeshOracleData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub vertex_ids: Vec<u32>,
    pub edge_positions: Vec<f32>,
    pub edge_ids: Vec<u32>,
    pub uvs: Vec<f32>,
    pub colors: Vec<f32>,
}

/// 🏔️🎨️ Language-neutral oracle for `WorldTerrainMeshCursor`: the de-indexed, vertex-coloured tile
/// mesh the cursor must produce — one triangle soup with the CONTINUOUS hypsometric ramp sampled per
/// vertex from the tile's own elevation ratio, which is what React's textured `MeshStandardMaterial`
/// shows.
#[cfg(any(test, feature = "testkit"))]
pub fn build_terrain_tile_mesh_oracle(mesh: &TerrainTileMeshPayload) -> Option<TerrainMeshOracleData> {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    for triangle in mesh.indices.as_chunks::<3>().0 {
        let base = (positions.len() / 3) as u32;
        for index in triangle {
            let offset = *index as usize * 3;
            let position = mesh.positions.get(offset..offset + 3)?;
            let normal = mesh.normals.get(offset..offset + 3).unwrap_or(&[0.0, 0.0, 1.0]);
            positions.extend_from_slice(position);
            normals.extend_from_slice(normal);
            colors.extend_from_slice(&hypsometric_color(mesh.uvs.get(*index as usize * 2 + 1).copied().unwrap_or(0.0)));
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }
    (!positions.is_empty()).then_some(TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors })
}

/// 🔄️ GPU-free half of `apply_terrain_style_if_changed`: applies `state.terrain_style` to the tile
/// session and purges cached tile meshes from CPU-side maps whenever the tile source, project
/// origin, or exaggeration changes (their old positions/heights are baked into cached geometry and
/// would otherwise render stale). Returns the mesh keys the caller must also evict from the GPU.
pub fn apply_terrain_style_if_changed_state(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState) -> Vec<String> {
    let signature = terrain_state.terrain_style.as_ref().map(|style| (style.tile_url_template.clone(), style.project_origin_lon, style.project_origin_lat, style.exaggeration));
    if signature == terrain_state.terrain_applied_signature {
        return Vec::new();
    }
    terrain_state.terrain_revision = terrain_state.terrain_revision.wrapping_add(1).max(1);
    if let Some(cursor) = terrain_state.terrain_build.as_mut() {
        cursor.faulted = Some(WorldDynamicFault::StaleToken);
    }
    let prefix = format!("terrain:{}:", session_state.surface_id);
    let stale_keys: Vec<String> = scene_state.meshes.keys().filter(|key| key.starts_with(&prefix)).cloned().collect();
    for key in &stale_keys {
        if !retire_world_mesh(scene_state, key) {
            return Vec::new();
        }
    }
    let visible = std::mem::take(&mut terrain_state.terrain_visible_tiles);
    for (z, x, y) in visible {
        terrain_state.terrain_session.evict_terrain_tile(z, x, y);
    }
    terrain_state.terrain_built_tiles.clear();
    terrain_state.pending_terrain_tile_urls.clear();
    terrain_state.terrain_tile_misses.clear();
    if let Some(style) = &terrain_state.terrain_style {
        terrain_state.terrain_session.set_project_origin(style.project_origin_lon, style.project_origin_lat);
        terrain_state.terrain_session.set_exaggeration(style.exaggeration);
    }
    terrain_state.terrain_applied_signature = signature;
    stale_keys
}

pub fn apply_terrain_style_if_changed(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, gpu: &mut World3dBuildContext) {
    for key in apply_terrain_style_if_changed_state(session_state, scene_state, terrain_state) {
        gpu.evict_mesh(&key);
    }
}

/// 🏔️ One terrain band's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`) plus the flat hypsometric color for that band.
/// 🏔️ One terrain tile's resolved GPU draw inputs: mesh key/version (already present in
/// `state.meshes`/`state.mesh_versions`). The hypsometric ramp rides the mesh's OWN per-vertex
/// colours, so the instance tint is neutral white — one draw per tile, never one per colour band.
pub struct TerrainTileDraw {
    pub mesh_key: String,
    pub mesh_version: u64,
}

pub fn terrain_family_visible(terrain_state: &WorldTerrainState, tile: (u32, u32, u32)) -> bool {
    terrain_state.terrain_built_tiles.contains(&tile)
}

//#endregion WorldMeshBuffers

//#region PlaceholderMesh
/// 🧊️ Viewport placeholder markers only (gumball plane, reference plane, vortex arrow parts, and
/// the box fallback for an unresolved `mesh_id`) — demo geometry, never document content. Ports
/// only the primitive kinds actually reachable from this file's `mesh_from_kind` call sites
/// (census in 📓️wave-g1b-infinite-report.md); the dissolved mesh-engine's other primitives
/// (uv-sphere, ico-sphere at other radii, torus, …) are not reachable here and were not ported.
#[cfg(any(test, feature = "testkit"))]
pub fn placeholder_mesh(kind: &str) -> WorldMeshBuffers {
    match kind {
        "vortex-marker" => placeholder_ico_sphere(0.12, 1),
        "plane" => placeholder_plane(1.0, 1.0),
        "cylinder" => placeholder_cylinder(0.5, 1.0, 16),
        "cone" => placeholder_cone(0.5, 1.0, 16),
        _ => placeholder_box(1.0, 1.0, 1.0),
    }
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_push_triangle(mesh: &mut WorldMeshBuffers, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_box(width: f32, height: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        placeholder_push_triangle(&mut mesh, a, b, c);
        placeholder_push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_plane(width: f32, depth: f32) -> WorldMeshBuffers {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = WorldMeshBuffers::default();
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    placeholder_push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_cylinder(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, p00, p01, p11);
        placeholder_push_triangle(&mut mesh, p00, p11, p10);
        placeholder_push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        placeholder_push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_cone(radius: f32, height: f32, segments: u32) -> WorldMeshBuffers {
    let mut mesh = WorldMeshBuffers::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        placeholder_push_triangle(&mut mesh, apex, p1, p0);
        placeholder_push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_midpoint(verts: &mut Vec<[f32; 3]>, cache: &mut HashMap<(u32, u32), u32>, a: u32, b: u32) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = placeholder_normalize3([(verts[a as usize][0] + verts[b as usize][0]) * 0.5, (verts[a as usize][1] + verts[b as usize][1]) * 0.5, (verts[a as usize][2] + verts[b as usize][2]) * 0.5]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

#[cfg(any(test, feature = "testkit"))]
fn placeholder_ico_sphere(radius: f32, subdivisions: u32) -> WorldMeshBuffers {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        placeholder_normalize3([-1.0, t, 0.0]),
        placeholder_normalize3([1.0, t, 0.0]),
        placeholder_normalize3([-1.0, -t, 0.0]),
        placeholder_normalize3([1.0, -t, 0.0]),
        placeholder_normalize3([0.0, -1.0, t]),
        placeholder_normalize3([0.0, 1.0, t]),
        placeholder_normalize3([0.0, -1.0, -t]),
        placeholder_normalize3([0.0, 1.0, -t]),
        placeholder_normalize3([t, 0.0, -1.0]),
        placeholder_normalize3([t, 0.0, 1.0]),
        placeholder_normalize3([-t, 0.0, -1.0]),
        placeholder_normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces =
        vec![[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = HashMap::new();
        for face in &faces {
            let a = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = placeholder_midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[[face[0], a, c], [face[1], b, a], [face[2], c, b], [a, b, c]]);
        }
        faces = next;
    }
    let mut mesh = WorldMeshBuffers::default();
    for face in faces {
        let a = placeholder_scale3(verts[face[0] as usize], radius);
        let b = placeholder_scale3(verts[face[1] as usize], radius);
        let c = placeholder_scale3(verts[face[2] as usize], radius);
        placeholder_push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}
//#endregion PlaceholderMesh
//#endregion MeshHelpers
