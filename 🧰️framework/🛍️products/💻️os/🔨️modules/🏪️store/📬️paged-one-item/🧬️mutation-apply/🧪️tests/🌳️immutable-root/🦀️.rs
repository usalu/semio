//! 🌳️ Fixture roots use actual sealed string issuers and immutable ranked pages; cold references use independent wire values.
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::{SealedShared,SharedIssuer}},list::persistent::{PersistentList},retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use serde::{Serialize,Deserialize};

#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,semio_framework_value::RetireOwned)]
pub(super) struct PlainNode{pub(super) id:String,pub(super) text:String}
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize,semio_framework_value::RetireOwned)]
pub(super) struct PlainDocument{pub(super) title:String,pub(super) nodes:Vec<PlainNode>}

#[derive(semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection)]
pub(super) struct Node{pub(super) id:SealedShared<String>,pub(super) text:SealedShared<String>}
#[derive(semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection)]
pub(super) struct Document{pub(super) title:SealedShared<String>,pub(super) nodes:PersistentList<Node>}

pub(super) fn fixture_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
pub(super) fn seal_text(value:String)->SealedShared<String>{SealedShared::admit(value,SharedIssuer::owned(),fixture_grant()).unwrap_or_else(|_|panic!("paid fixture string birth")).0}
pub(super) fn node_owned(id:String,text:String)->Node{Node{id:seal_text(id),text:seal_text(text)}}
pub(super) fn list_owned(rows:Vec<Node>)->PersistentList<Node>{
    let mut rows=rows.into_iter();let mut pages=Vec::new();
    loop{
        let mut count=0;
        let leaf=std::array::from_fn(|_|rows.next().map(|row|{count+=1;SealedShared::admit(row,SharedIssuer::owned(),fixture_grant()).unwrap_or_else(|_|panic!("paid fixture row birth")).0}));
        if count==0{break;}
        pages.push(PersistentList::admit_leaf(leaf,count,fixture_grant()).unwrap_or_else(|_|panic!("paid fixture leaf birth")).0);
    }
    while pages.len()>1{
        let mut originals=pages.into_iter();let mut parents=Vec::new();
        loop{
            let mut count=0;
            let children=std::array::from_fn(|_|originals.next().inspect(|_|count+=1));
            if count==0{break;}
            parents.push(PersistentList::admit_branch(children,fixture_grant()).unwrap_or_else(|_|panic!("paid fixture branch birth")).0);
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

pub(super) fn drain_fixture<T:RetireOwned>(original:T){
    let mut owner=ControlledRetirement::new(original).unwrap_or_else(|_|panic!("fixture original typed retirement"));
    for _ in 0..100000{
        if owner.terminal_is_empty(){return;}
        let copy=owner.next_copy_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(4096),maximum_capacity_bytes:owner.next_capacity_byte_demand(copy.max(4096)).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
        let step=owner.step(grant).unwrap();assert!(step.progress().fits(grant));assert!(step.progress()!=RetainedCloneProgress::default()||owner.terminal_is_empty());
    }
    panic!("fixture originals retained beyond exact close budget");
}