//! 🧰️ Shared test kit of the mesh crate: the allocation observer, bounded retry helpers and paged mesh fixtures every mesh-dependent suite reuses.

use super::*;

pub fn ownership_test_grant(work:usize,capacity:usize,release:usize,depth:usize)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}}

pub fn admit_mesh(generation:u64,revision:u64,schema:Mesh3dSchema)->Result<Mesh3dWriteToken,Mesh3dFault> {let grant=ownership_test_grant(0,65_536,0,1);let(token,receipt)=mesh_test_attempt(||mesh3d_begin(generation,revision,schema,grant))?;assert!(receipt.fits(grant));Ok(token)}

pub fn allocate_mesh(token:Mesh3dWriteToken)->Result<bool,Mesh3dFault> {let grant=ownership_test_grant(0,mesh_test_attempt(||mesh3d_allocate_capacity_byte_demand(token))?,0,1);let step=mesh_test_attempt(||mesh3d_allocate_step(token,grant))?;assert!(step.progress.fits(grant));Ok(step.complete)}

pub fn close_mesh(lease:Mesh3dLease)->Result<bool,Mesh3dFault> {let demand=mesh_test_attempt(||mesh3d_close_demands(lease,MESH3D_PAGE_BYTES))?;let grant=ownership_test_grant(MESH3D_PAGE_BYTES,demand.capacity_bytes,demand.release_bytes,demand.depth);let step=mesh_test_attempt(||mesh3d_close_step(lease,grant))?;assert!(step.progress.fits(grant));Ok(step.complete)}

pub struct LegacyMeshOracleData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub vertex_ids: Vec<u32>,
    pub edges: Vec<[[f32; 3]; 2]>,
    pub edge_ids: Vec<u32>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
}

impl LegacyMeshOracleData {
    pub fn triangle() -> Self {
        Self {
            positions: vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: vec![0, 1, 2],
            face_ids: Vec::new(),
            vertex_ids: Vec::new(),
            edges: Vec::new(),
            edge_ids: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
        }
    }
}

pub fn paged_mesh_fixture(data: LegacyMeshOracleData) -> Mesh3dLease {
    let schema = Mesh3dSchema {
        vertices: data.positions.len() as u32,
        indices: data.indices.len() as u32,
        face_ids: data.face_ids.len() as u32,
        vertex_ids: data.vertex_ids.len() as u32,
        edges: data.edges.len() as u32,
        edge_ids: data.edge_ids.len() as u32,
        uvs: data.uvs.len() as u32,
        colors: data.colors.len() as u32,
    surface_uvs:[0;4],tangents:0,};
    let token = admit_mesh(1, 1, schema).expect("test mesh claim");
    while !allocate_mesh(token).expect("test mesh page allocation") {}
    for value in data.positions {
        mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Positions, value)).unwrap();
    }
    for value in data.normals {
        mesh_test_attempt(||mesh3d_write_vec3(token, Mesh3dField::Normals, value)).unwrap();
    }
    for value in data.indices {
        mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::Indices, value)).unwrap();
    }
    for value in data.face_ids {
        mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::FaceIds, value)).unwrap();
    }
    for value in data.vertex_ids {
        mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::VertexIds, value)).unwrap();
    }
    for value in data.edges {
        mesh_test_attempt(||mesh3d_write_edge(token, value)).unwrap();
    }
    for value in data.edge_ids {
        mesh_test_attempt(||mesh3d_write_u32(token, Mesh3dField::EdgeIds, value)).unwrap();
    }
    for value in data.uvs {
        mesh_test_attempt(||mesh3d_write_vec2(token, Mesh3dField::Uvs, value)).unwrap();
    }
    for value in data.colors {
        mesh_test_attempt(||mesh3d_write_vec4(token, Mesh3dField::Colors, value)).unwrap();
    }
    mesh_test_attempt(||mesh3d_seal(token)).expect("test mesh publication")
}

pub fn test_box_mesh() -> Mesh3dLease {
    paged_mesh_fixture(LegacyMeshOracleData::triangle())
}

pub struct ObservedAllocator;

thread_local! { static RETIREMENT_ALLOCATION_EVENTS: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) }; }

pub fn retirement_allocation_event(requested: usize, released: usize) { let _ = RETIREMENT_ALLOCATION_EVENTS.try_with(|events| { if let Some((allocated, freed)) = events.get() { events.set(Some((allocated + requested, freed + released))); } }); }

pub fn observe_retirement_allocations<T>(operation: impl FnOnce() -> T) -> (T, (usize, usize)) {
    RETIREMENT_ALLOCATION_EVENTS.with(|events| { assert!(events.replace(Some((0, 0))).is_none()); });
    let result = operation();
    let events = RETIREMENT_ALLOCATION_EVENTS.with(|events| events.replace(None).unwrap());
    (result, events)
}

unsafe impl std::alloc::GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{let pointer=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !pointer.is_null(){retirement_allocation_event(layout.size(),0);}pointer}
    unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{let grown=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !grown.is_null(){retirement_allocation_event(size,layout.size());}grown}
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_allocation_event(0,layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}

pub fn mesh_test_attempt_controlled<T>(maximum_attempts:usize,deadline:std::time::Instant,cancel:&std::sync::atomic::AtomicBool,mut operation:impl FnMut()->Result<T,Mesh3dFault>)->Result<T,Mesh3dFault>{
    for _ in 0..maximum_attempts{
        if cancel.load(std::sync::atomic::Ordering::Acquire)||std::time::Instant::now()>=deadline{return Err(Mesh3dFault::Busy);}
        match operation(){Err(Mesh3dFault::Busy)=>std::thread::yield_now(),result=>return result}
    }
    Err(Mesh3dFault::Busy)
}

pub fn mesh_test_attempt<T>(operation:impl FnMut()->Result<T,Mesh3dFault>)->Result<T,Mesh3dFault>{
    mesh_test_attempt_controlled(4096,std::time::Instant::now()+std::time::Duration::from_millis(5000),&std::sync::atomic::AtomicBool::new(false),operation)
}
