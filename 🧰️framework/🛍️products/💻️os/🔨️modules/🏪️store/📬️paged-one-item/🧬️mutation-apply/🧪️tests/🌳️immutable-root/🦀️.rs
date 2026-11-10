//! 🌳️ Fixture roots use actual sealed string issuers and immutable ranked pages; cold references use independent wire values.
use semio_framework_value::{retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework_value::shared_issuance::{SealedShared, SharedIssuer};
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,RetainedCloneStep,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_snapshot_projection::snapshot_projection::ImmutableRootProjection;
use serde::{Serialize,Deserialize};
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,semio_framework_value::RetireOwned)]
pub(super) struct PlainNode{pub(super) id:String,pub(super) text:String}
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,semio_framework_value::RetireOwned)]
pub(super) struct PlainDocument{pub(super) title:String,pub(super) nodes:Vec<PlainNode>}

#[derive(semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection)]
pub(super) struct Node{pub(super) id:SealedShared<String>,pub(super) text:SealedShared<String>}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Document{pub(super) title:SealedShared<String>,pub(super) nodes:PersistentList<Node>}

/// 🌳️ The fixture root projects its two sealed authorities into one independent candidate root without traversing a row.
unsafe impl ImmutableRootProjection for Document{
    fn root_projection_demand(_:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{copy_bytes:size_of::<Option<Self>>()*2+SealedShared::<String>::duplicate_copy_bytes()+PersistentList::<Node>::projection_demand().copy_bytes,depth:1,..Default::default()})
    }
    fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{
        Ok(1+original.get().title.duplicate_work_demand()?+PersistentList::<Node>::projection_work_demand(original.project(0,|document|&document.nodes))?)
    }
    fn project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
        if output.is_some(){return Err(ValueError::invariant("fixture root output occupied"))}
        let demand=Self::root_projection_demand(original)?;
        if effect.remaining_work_units()?<Self::root_projection_work_demand(original)?||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        let grant=effect.grant();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<Self>>()*2,..grant};
        let(mut nodes,mut answer)=(None,None);
        {let nested=OriginalEffectReply::borrow(&mut answer)?;drop(PersistentList::project_into(original.project(0,|document|&document.nodes),&mut nodes,&mut effect.nested(child),nested)?);}
        match answer.take(){Some(Ok(RetainedCloneStep::Complete(_)))=>{},Some(Ok(step))=>return Ok(reply.complete(Ok(step))),Some(Err(failure))=>return Ok(reply.complete(Err(failure))),None=>return Err(ValueError::invariant("fixture root nodes projection stayed pending"))}
        let nodes=nodes.take().ok_or_else(||ValueError::invariant("fixture root nodes projection published nothing"))?;
        let turn=original.get().title.try_duplicate(&mut effect.nested(child));
        let(title,_)=match turn.result{Ok(title)=>title,Err(error)=>{drain_fixture(nodes);return Ok(reply.complete(Err(error.into())))}};
        if !effect.performed(1)?{drain_fixture(nodes);drain_fixture(title);return Ok(reply.complete(Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"fixture root publication exceeded Work").into())))}
        *output=Some(Self{title,nodes});
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
    }
    fn recover_project_root_into<'r>(_:RetainedCloneRef<'_,Self>,_:&mut Option<Self>,_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Err(ValueError::invariant("fixture root has no unrecorded callback"))}
}

pub(super) fn fixture_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
pub(super) fn seal<T:Send+Sync+'static+RetireOwned>(value:T)->SealedShared<T>{let mut input=Some(value);let mut output=None;let turn=crate::effect::with_effect(fixture_grant(),crate::effect::CPU,|effect|SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect)).0;turn.result.unwrap_or_else(|_|panic!("paid fixture shared birth"));output.expect("funded fixture shared birth")}
pub(super) fn seal_text(value:String)->SealedShared<String>{seal(value)}
pub(super) fn node_owned(id:String,text:String)->Node{Node{id:seal_text(id),text:seal_text(text)}}
pub(super) fn list_owned(rows:Vec<Node>)->PersistentList<Node>{
    let mut rows=rows.into_iter();let mut pages=Vec::new();
    loop{
        let mut count=0;
        let leaf=std::array::from_fn(|_|rows.next().map(|row|{count+=1;seal(row)}));
        if count==0{break;}
        pages.push(crate::effect::with_effect(fixture_grant(),crate::effect::CPU,|effect|PersistentList::admit_leaf(leaf,count,effect)).0.result.unwrap_or_else(|_|panic!("paid fixture leaf birth")).0);
    }
    while pages.len()>1{
        let mut originals=pages.into_iter();let mut parents=Vec::new();
        loop{
            let mut count=0;
            let children=std::array::from_fn(|_|originals.next().inspect(|_|count+=1));
            if count==0{break;}
            parents.push(crate::effect::with_effect(fixture_grant(),crate::effect::CPU,|effect|PersistentList::admit_branch(children,effect)).0.result.unwrap_or_else(|_|panic!("paid fixture branch birth")).0);
        }
        pages=parents;
    }
    pages.pop().unwrap_or_default()
}
pub(super) fn document_from_plain(value:PlainDocument)->Document{Document{title:seal_text(value.title),nodes:list_owned(value.nodes.into_iter().map(|node|node_owned(node.id,node.text)).collect())}}
pub(super) fn document(value:&serde_json::Value)->Document{document_from_plain(serde_json::from_value(value.clone()).unwrap())}
pub(super) fn plain_document_json(value:&serde_json::Value)->PlainDocument{serde_json::from_value(value.clone()).unwrap()}
pub(super) fn plain_document(original:&Document)->PlainDocument{PlainDocument{title:original.title.get().clone(),nodes:(0..original.nodes.len()).map(|index|{let node=original.nodes.get(index).unwrap();PlainNode{id:node.id.get().clone(),text:node.text.get().clone()}}).collect()}}
pub(super) fn node_id(original:&Document,index:usize)->&String{original.nodes.get(index).expect("original fixture rank").id.get()}
pub(super) fn node_text(original:&Document,index:usize)->&String{original.nodes.get(index).expect("original fixture rank").text.get()}

pub(super) trait FixtureDocument{
    fn plain(&self)->PlainDocument;
    fn replace_plain(&mut self,original:PlainDocument);
    fn node_len(&self)->usize;
    fn node_parts(&self,index:usize)->Option<(&String,&String)>;
}
impl FixtureDocument for PlainDocument{
    fn plain(&self)->PlainDocument{self.clone()}
    fn replace_plain(&mut self,original:PlainDocument){*self=original;}
    fn node_len(&self)->usize{self.nodes.len()}
    fn node_parts(&self,index:usize)->Option<(&String,&String)>{self.nodes.get(index).map(|node|(&node.id,&node.text))}
}
impl FixtureDocument for Document{
    fn plain(&self)->PlainDocument{plain_document(self)}
    fn replace_plain(&mut self,original:PlainDocument){let retired=std::mem::replace(self,document_from_plain(original));drain_fixture(retired);}
    fn node_len(&self)->usize{self.nodes.len()}
    fn node_parts(&self,index:usize)->Option<(&String,&String)>{self.nodes.get(index).map(|node|(node.id.get(),node.text.get()))}
}
impl Serialize for Node{
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeStruct;let mut value=serializer.serialize_struct("Node",2)?;value.serialize_field("id",self.id.get())?;value.serialize_field("text",self.text.get())?;value.end()}
}
struct Rows<'a>(&'a PersistentList<Node>);
impl Serialize for Rows<'_>{
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeSeq;let mut rows=serializer.serialize_seq(Some(self.0.len()))?;for index in 0..self.0.len(){rows.serialize_element(self.0.get(index).unwrap())?;}rows.end()}
}
impl Serialize for Document{
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeStruct;let mut value=serializer.serialize_struct("Document",2)?;value.serialize_field("title",self.title.get())?;value.serialize_field("nodes",&Rows(&self.nodes))?;value.end()}
}
impl<'de> Deserialize<'de> for Document{
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{PlainDocument::deserialize(deserializer).map(document_from_plain)}
}
impl std::fmt::Debug for Node{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("Node").field("id",self.id.get()).field("text",self.text.get()).finish()}}
impl std::fmt::Debug for Document{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("Document").field("title",self.title.get()).field("nodes",&self.nodes.len()).finish()}}
impl PartialEq for Node{fn eq(&self,other:&Self)->bool{self.id.get()==other.id.get()&&self.text.get()==other.text.get()}}
impl PartialEq for Document{fn eq(&self,other:&Self)->bool{self.title.get()==other.title.get()&&self.nodes.len()==other.nodes.len()&&(0..self.nodes.len()).all(|index|self.nodes.get(index)==other.nodes.get(index))}}

/// 🧹️ Drains a uniquely held shared fixture root through its shared controlled retirement once every owner has returned its alias.
pub(super) fn drain_root<T:RetireOwned+Send+Sync+'static>(root:std::sync::Arc<T>){
    let mut owner=semio_framework_value::retirement::arc_custody::SharedControlledRetirement::new(semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage::from_cold(std::sync::Arc::into_inner(root).expect("fixture root is unique after every owner closed")));
    for _ in 0..100000{
        if owner.terminal_is_empty(){return;}
        let copy=owner.next_copy_byte_demand().unwrap().max(4096);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let step=super::effect::settled(super::effect::with_effect(grant,super::effect::CPU,|effect|owner.close_step(copy,effect)).0).result.unwrap_or_else(|failure|panic!("fixture shared root {} retirement failed: {failure:?}",std::any::type_name::<T>()));assert!(step.progress().fits(grant));
    }
    panic!("fixture shared root retained beyond exact close budget");
}

pub(super) fn drain_fixture<T:RetireOwned>(original:T){
    let mut owner=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("fixture original typed retirement"));
    for _ in 0..100000{
        if owner.terminal_is_empty(){return;}
        let copy=owner.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(4096),maximum_capacity_bytes:owner.next_capacity_byte_demand(copy.max(4096)).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let step=super::effect::settled(super::effect::with_effect(grant,super::effect::CPU,|effect|owner.step(copy.max(4096),effect)).0).result.unwrap_or_else(|failure|panic!("fixture original {} retirement failed: {failure:?}",std::any::type_name::<T>()));assert!(step.progress().fits(grant));assert!(step.progress()!=RetainedCloneProgress::default()||owner.terminal_is_empty());
    }
    panic!("fixture originals retained beyond exact close budget");
}