//! 🎯️ One original analytic shape label for every triangle or segment of its preview.

use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_mesh_engine::{ComponentReferenceTable,MeshData};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress as Progress,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};

const IDS_PER_TURN:usize=4;

#[derive(Clone,Copy)]
pub(super) enum PickDomain{Face,Edge}
semio_framework_value::artifact_retire_leaf!(PickDomain);

impl PickDomain{
    fn key(self)->&'static str{match self{Self::Face=>"face",Self::Edge=>"edge"}}
    fn count(self,mesh:&MeshData)->usize{match self{Self::Face=>mesh.indices.len()/3,Self::Edge=>mesh.edge_positions.len()/6}}
    fn ids(self,mesh:&mut MeshData)->&mut Vec<u32>{match self{Self::Face=>&mut mesh.face_ids,Self::Edge=>&mut mesh.edge_ids}}
}

pub(super) struct AnalyticComponent {
    label:u64,
    domain:PickDomain,
    stage:usize,
    written:usize,
    mesh:Option<MeshData>,
    entries:Vec<(String,Vec<String>)>,
}
semio_framework_value::artifact_retire_struct!(AnalyticComponent{label,domain,stage,written,mesh,entries});

fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"analytic preview pick backing allocation failed")}

fn reserve<T>(values:&mut Vec<T>,count:usize)->Result<usize,ValueError>{
    values.try_reserve_exact(count).map_err(|_|allocation())?;
    Ok(values.capacity()*size_of::<T>())
}

fn owned(value:&str)->Result<String,ValueError>{
    let mut text=String::new();
    text.try_reserve_exact(value.len()).map_err(|_|allocation())?;
    text.push_str(value);
    Ok(text)
}

impl AnalyticComponent {
    pub(super) fn new(label:u64,domain:PickDomain)->Self{
        Self{label,domain,stage:0,written:0,mesh:None,entries:Vec::new()}
    }

    pub(super) fn check_label(&self,label:u64)->Result<(),ValueError>{
        if self.label!=label{return Err(ValueError::invariant("analytic preview source label changed"));}
        Ok(())
    }

    pub(super) fn hold(&mut self,mesh:MeshData){self.mesh=Some(mesh);}

    pub(super) fn demand(&self)->Option<RetirementDemand>{
        let count=self.domain.count(self.mesh.as_ref()?);
        let (copy_bytes,capacity_bytes)=match self.stage{
            0=>(0,count*size_of::<u32>()),
            1=>(0,size_of::<(String,Vec<String>)>()),
            2=>{let len=self.domain.key().len();(len,len)}
            3=>(0,size_of::<String>()),
            4=>{let len=PersistentLabel(self.label).decimal(&mut [0;20]).len();(len,len)}
            _=>((count-self.written).min(IDS_PER_TURN)*size_of::<u32>(),0),
        };
        Some(RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
    }

    pub(super) fn advance(&mut self,grant:RetainedCloneGrant)->Result<(Option<MeshData>,Progress),ValueError>{
        let demand=self.demand().ok_or_else(||ValueError::invariant("analytic preview mesh is absent"))?;
        if !retained_clone_grant_admits(grant,demand){return Ok((None,Default::default()));}
        let mesh=self.mesh.as_mut().ok_or_else(||ValueError::invariant("analytic preview mesh is absent"))?;
        let count=self.domain.count(mesh);
        let capacity=match self.stage{
            0=>reserve(self.domain.ids(mesh),count)?,
            1=>reserve(&mut self.entries,1)?,
            2=>{
                let key=owned(self.domain.key())?;
                let capacity=key.capacity();
                self.entries.push((key,Vec::new()));
                capacity
            }
            3=>reserve(&mut self.entries[0].1,1)?,
            4=>{
                let label=owned(PersistentLabel(self.label).decimal(&mut [0;20]))?;
                let capacity=label.capacity();
                self.entries[0].1.push(label);
                capacity
            }
            _=>{
                let next=(self.written+IDS_PER_TURN).min(count);
                self.domain.ids(mesh).resize(next,0);
                self.written=next;
                0
            }
        };
        let receipt=Progress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()};
        if !receipt.fits(grant){
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"analytic preview picks exceeded their original grant").with_retained_progress(receipt));
        }
        if self.stage<5{self.stage+=1;return Ok((None,receipt));}
        if self.written!=count{return Ok((None,receipt));}
        mesh.component_references=ComponentReferenceTable::from_entries(std::mem::take(&mut self.entries));
        Ok((self.mesh.take(),receipt))
    }
}
