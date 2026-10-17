//! 🔬️ Feature-owned immutable observations for retained modeling integration laws.

use super::*;
use semio_framework_3d_mesh_reconstruction::MeshTopologyInspection;

pub struct MeshSnapshotInspection<'a>{pub snapshot:&'a Snapshot}
impl<'a> MeshSnapshotInspection<'a>{pub fn snapshot(&self)->&'a Snapshot{self.snapshot}}
pub struct MeshTessellationInspection<'a>{pub snapshot:&'a Snapshot,pub tessellation:Option<()>,pub tessellation_retirement:Option<()>}
pub struct MeshTransformInspection<'a>{pub mesh:&'a HalfedgeMesh,pub phase:u8,pub vertex:usize}
pub struct MeshOrientationInspection<'a>{
    pub snapshot:&'a Snapshot,pub edges:&'a HistoryFoldIndex<(u32,u32),Vec<(usize,bool)>>,pub keys:&'a Vec<(u32,u32)>,
    pub adjacency:&'a Vec<Vec<(usize,bool)>>,pub oriented:&'a Vec<bool>,pub flipped:&'a Vec<bool>,pub stack:&'a Vec<usize>,
    pub active:Option<usize>,pub face:usize,pub corner:usize,pub cursor:usize,pub neighbor:usize,pub reversing:usize,
    pub incidence:u8,pub flip_count:usize,pub phase:u8,pub build:Option<MeshTopologyInspection<'a>>,
}
pub struct MeshBevelInspection<'a>{pub snapshot:&'a Snapshot,pub phase:u8,pub planes:&'a Vec<(Vec3,f32)>,pub edge:usize,pub build:Option<MeshTopologyInspection<'a>>}
pub struct MeshCandidateInspection<'a>{pub phase:u8,pub soup:&'a Soup,pub a:u32,pub b:u32}
pub struct MeshDecimationInspection<'a>{pub snapshot:&'a Snapshot,pub phase:u8,pub candidate:Option<MeshCandidateInspection<'a>>,pub build:Option<MeshTopologyInspection<'a>>}
pub enum MeshModelingInspection<'a>{
    Bevel(MeshBevelInspection<'a>),Decimate(MeshDecimationInspection<'a>),Mirror(MeshSnapshotInspection<'a>),Merge(MeshSnapshotInspection<'a>),
    Subdivide(MeshTessellationInspection<'a>),LoopCut(MeshSnapshotInspection<'a>),Orient(MeshOrientationInspection<'a>),FillHoles(MeshSnapshotInspection<'a>),
    Inflate(MeshSnapshotInspection<'a>),Import(()),Knife(MeshTessellationInspection<'a>),Transform(MeshTransformInspection<'a>),
}
impl MeshModelingJob{
    /// 🔬️ Borrows ordinary observed data without cloning or advancing the retained job.
    pub fn inspection(&self)->MeshModelingInspection<'_>{
        match &self.work{
            Work::Bevel(work)=>MeshModelingInspection::Bevel(MeshBevelInspection{snapshot:&work.snapshot,phase:work.phase,planes:&work.planes,edge:work.edge,build:work.build.as_ref().map(Build::inspection)}),
            Work::Decimate(work)=>MeshModelingInspection::Decimate(MeshDecimationInspection{snapshot:&work.snapshot,phase:work.phase,candidate:work.candidate.as_ref().map(|candidate|MeshCandidateInspection{phase:candidate.phase,soup:&candidate.soup,a:candidate.a,b:candidate.b}),build:work.build.as_ref().map(Build::inspection)}),
            Work::Mirror(work)=>MeshModelingInspection::Mirror(MeshSnapshotInspection{snapshot:&work.snapshot}),
            Work::Merge(work)=>MeshModelingInspection::Merge(MeshSnapshotInspection{snapshot:work.snapshot()}),
            Work::Subdivide(work)=>MeshModelingInspection::Subdivide(MeshTessellationInspection{snapshot:&work.snapshot,tessellation:work.tessellation.as_ref().map(|_|()),tessellation_retirement:work.tessellation_retirement.as_ref().map(|_|())}),
            Work::LoopCut(work)=>MeshModelingInspection::LoopCut(MeshSnapshotInspection{snapshot:&work.snapshot}),
            Work::Orient(work)=>MeshModelingInspection::Orient(MeshOrientationInspection{snapshot:&work.snapshot,edges:&work.edges,keys:&work.keys,adjacency:&work.adjacency,oriented:&work.oriented,flipped:&work.flipped,stack:&work.stack,active:work.active,face:work.face,corner:work.corner,cursor:work.cursor,neighbor:work.neighbor,reversing:work.reversing,incidence:work.incidence,flip_count:work.flip_count,phase:work.phase,build:work.build.as_ref().map(Build::inspection)}),
            Work::FillHoles(work)=>MeshModelingInspection::FillHoles(MeshSnapshotInspection{snapshot:&work.snapshot}),
            Work::Inflate(work)=>MeshModelingInspection::Inflate(MeshSnapshotInspection{snapshot:&work.snapshot}),
            Work::Import(_)=>MeshModelingInspection::Import(()),
            Work::Knife(work)=>MeshModelingInspection::Knife(MeshTessellationInspection{snapshot:&work.snapshot,tessellation:work.tessellation.as_ref().map(|_|()),tessellation_retirement:work.tessellation_retirement.as_ref().map(|_|())}),
            Work::Transform(work)=>MeshModelingInspection::Transform(MeshTransformInspection{mesh:&work.mesh,phase:work.phase,vertex:work.vertex}),
        }
    }
    /// 🛑️ Reports the retained cancellation fact without changing admission.
    pub fn inspection_cancelled(&self)->bool{self.cancelled}
}
