//! 🎯️ Funded BRep face, edge, and seam vertex picks for a sampled conic disk.

use super::MeshTransfer;
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_mesh_engine::{ComponentReferenceTable,MeshData};
use semio_framework_value::{RetainedCloneProgress as Progress,RetirementDemand,ValueError,ValueRefusalKind};

const DOMAINS:[&str;3]=["face","edge","vertex"];

pub(super) struct ConicComponents {
    revision:u64,
    labels:[u64;3],
    stage:usize,
    entries:Vec<(String,Vec<String>)>,
    output:MeshData,
}

semio_framework_value::artifact_retire_struct!(ConicComponents {revision,labels,stage,entries,output});

fn allocation_error()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"conic component storage allocation failed")}

fn reserve<T>(buffer:&mut Vec<T>,count:usize)->Result<usize,ValueError>{
    buffer.try_reserve_exact(count).map_err(|_|allocation_error())?;
    Ok(buffer.capacity()*size_of::<T>())
}

fn owned(text:&str)->Result<String,ValueError>{
    let mut out=String::new();
    out.try_reserve_exact(text.len()).map_err(|_|allocation_error())?;
    out.push_str(text);
    Ok(out)
}

impl ConicComponents {
    pub(super) const STORAGE_STEPS:usize=4+3*DOMAINS.len();

    pub(super) fn new(revision:u64,labels:[u64;3])->Self{
        Self{revision,labels,stage:0,entries:Vec::new(),output:MeshData::default()}
    }

    pub(super) fn check_revision(&self,revision:u64)->Result<(),ValueError>{
        if self.revision!=revision {return Err(ValueError::invariant("conic preview source revision changed"));}
        Ok(())
    }

    pub(super) fn storage_steps(&self)->usize{self.stage}

    pub(super) fn demand(&self,segments:usize)->Option<RetirementDemand>{
        let (copy_bytes,capacity_bytes)=match self.stage {
            0|1=>(0,segments*size_of::<u32>()),
            2=>(0,(segments+1)*size_of::<u32>()),
            3=>(0,DOMAINS.len()*size_of::<(String,Vec<String>)>()),
            stage if stage<Self::STORAGE_STEPS=>{
                let domain=(stage-4)/3;
                match (stage-4)%3 {
                    0=>{let length=DOMAINS[domain].len();(length,length)}
                    1=>(0,size_of::<String>()),
                    _=>{let length=PersistentLabel(self.labels[domain]).decimal(&mut [0;20]).len();(length,length)}
                }
            }
            _=>return None,
        };
        Some(RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()})
    }

    pub(super) fn advance_storage(&mut self,segments:usize,demand:RetirementDemand)->Result<Progress,ValueError>{
        let capacity=match self.stage {
            0=>reserve(&mut self.output.face_ids,segments)?,
            1=>reserve(&mut self.output.edge_ids,segments)?,
            2=>reserve(&mut self.output.vertex_ids,segments+1)?,
            3=>reserve(&mut self.entries,DOMAINS.len())?,
            stage=>{
                let domain=(stage-4)/3;
                match (stage-4)%3 {
                    0=>{
                        let key=owned(DOMAINS[domain])?;
                        let capacity=key.capacity();
                        self.entries.push((key,Vec::new()));
                        capacity
                    }
                    1=>reserve(&mut self.entries[domain].1,1)?,
                    _=>{
                        let label=owned(PersistentLabel(self.labels[domain]).decimal(&mut [0;20]))?;
                        let capacity=label.capacity();
                        self.entries[domain].1.push(label);
                        capacity
                    }
                }
            }
        };
        self.stage+=1;
        Ok(Progress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})
    }

    pub(super) fn sample_bytes(cursor:usize)->usize{
        match cursor {0..=3=>0,4=>size_of::<u32>(),_=>3*size_of::<u32>()}
    }

    pub(super) fn append_pick(&mut self,cursor:usize){
        if cursor<4{return;}
        self.output.vertex_ids.push(if cursor==5{0}else{u32::MAX});
        if cursor>4{
            self.output.edge_ids.push(0);
            self.output.face_ids.push(0);
        }
    }

    pub(super) fn finish(&mut self,mesh:MeshTransfer)->MeshData{
        self.output.positions=mesh.position;
        self.output.normals=mesh.normal;
        self.output.indices=mesh.index;
        self.output.edge_positions=mesh.edges;
        self.output.component_references=ComponentReferenceTable::from_entries(std::mem::take(&mut self.entries));
        std::mem::take(&mut self.output)
    }
}
