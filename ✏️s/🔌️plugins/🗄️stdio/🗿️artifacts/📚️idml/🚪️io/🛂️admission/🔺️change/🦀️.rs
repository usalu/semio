//! 🔺 Borrowed sparse IDML field-change planning over admitted numeric indexes.
use super::{IdmlAdmissionCursor,IdmlAdmissionSource,IdmlNameLookup,IdmlVersionDecoder,SourceIdentity};
/// 🔺 Original source coordinates identify real sparse event fields before cloning.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IdmlChangeEvent {Version,Order,Remove{before:usize},Part{next:usize,before:Option<usize>}}
/// 🧾 One bounded read slice yields at most one original field-change coordinate.
#[derive(Clone,Copy,Debug)]
pub struct IdmlChangeStep {pub work:usize,pub done:bool,pub event:Option<IdmlChangeEvent>,pub error:Option<&'static str>}
/// 🧭 No original string, XML DOM, or member payload is cloned into this fixed cursor.
pub struct IdmlChangeCursor {base:Option<SourceIdentity>,next:Option<SourceIdentity>,mode:&'static str,index:usize,offset:usize,before:usize,base_decoder:Option<IdmlVersionDecoder>,next_decoder:Option<IdmlVersionDecoder>,base_byte:Option<u8>,next_byte:Option<u8>,base_done:bool,next_done:bool,lookup:IdmlNameLookup,active:bool}
impl Default for IdmlChangeCursor {fn default()->Self{Self::new()}}
impl IdmlChangeCursor {
    /// 🌱 Original member indexes remain with their separate admitted owners.
    pub fn new()->Self{Self{base:None,next:None,mode:"init",index:0,offset:0,before:0,base_decoder:None,next_decoder:None,base_byte:None,next_byte:None,base_done:false,next_done:false,lookup:IdmlNameLookup::new(),active:false}}
    /// 🛑 Revokes source views and numeric progress without taking child ownership.
    pub fn close(&mut self){self.mode="closed";self.base=None;self.next=None;self.lookup.restart();self.base_decoder=None;self.next_decoder=None;}
    /// 🪹 No original source or child payload remains retained by this cursor.
    pub fn terminal_is_empty(&self)->bool{self.mode=="closed"}
    /// 🚶 Streams version, order, removed, inserted and changed fields under source work grants.
    pub fn advance(&mut self,base:IdmlAdmissionSource<'_>,next:IdmlAdmissionSource<'_>,base_index:&IdmlAdmissionCursor,next_index:&IdmlAdmissionCursor,maximum_work:usize)->IdmlChangeStep{
        if self.mode=="closed"{return IdmlChangeStep{work:0,done:false,event:None,error:Some("stdio.idml.change-closed")};}
        if self.base.is_none(){self.base=Some(base.identity());self.next=Some(next.identity());}else if self.base!=Some(base.identity())||self.next!=Some(next.identity()){return IdmlChangeStep{work:0,done:false,event:None,error:Some("stdio.idml.change-source")};}
        if !base_index.admits_source(base)||!next_index.admits_source(next){return IdmlChangeStep{work:0,done:false,event:None,error:Some("stdio.idml.change-admission")};}
        let mut work=0;
        while work<maximum_work&&self.mode!="done"{
            match self.mode{
                "init"=>{self.base_decoder=Some(IdmlVersionDecoder::new(base_index.version_view().unwrap()));self.next_decoder=Some(IdmlVersionDecoder::new(next_index.version_view().unwrap()));self.mode="version";work+=1;}
                "version"=>{
                    if self.base_byte.is_none()&&!self.base_done{let view=base_index.version_view().unwrap();let step=self.base_decoder.as_mut().unwrap().advance(base.data(view.part_index),maximum_work-work);work+=step.work;if let Some(error)=step.error{return IdmlChangeStep{work,done:false,event:None,error:Some(error)};}self.base_byte=step.byte;self.base_done=step.done;continue;}
                    if self.next_byte.is_none()&&!self.next_done{let view=next_index.version_view().unwrap();let step=self.next_decoder.as_mut().unwrap().advance(next.data(view.part_index),maximum_work-work);work+=step.work;if let Some(error)=step.error{return IdmlChangeStep{work,done:false,event:None,error:Some(error)};}self.next_byte=step.byte;self.next_done=step.done;continue;}
                    if self.base_byte!=self.next_byte||self.base_done!=self.next_done{self.mode="order-init";work+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Version),error:None};}
                    if self.base_done&&self.next_done{self.mode="order-init";}else{self.base_byte=None;self.next_byte=None;}work+=1;
                }
                "order-init"=>{self.index=0;self.offset=0;work+=1;if base.snapshot.part_count()!=next.snapshot.part_count(){self.mode="remove";return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Order),error:None};}self.mode="order";}
                "order"=>{
                    if self.index==base.snapshot.part_count(){self.index=0;self.mode="remove";work+=1;continue;}
                    let a=base.snapshot.part_path(self.index).as_bytes();let b=next.snapshot.part_path(self.index).as_bytes();
                    if a.len()!=b.len(){self.index=0;self.mode="remove";work+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Order),error:None};}
                    if self.offset==a.len(){self.index+=1;self.offset=0;work+=1;continue;}
                    if a[self.offset]!=b[self.offset]{self.index=0;self.offset=0;self.mode="remove";work+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Order),error:None};}self.offset+=1;work+=1;
                }
                "remove"=>{
                    if self.index==base.snapshot.part_count(){self.index=0;self.mode="update";work+=1;continue;}
                    if !self.active{self.lookup.restart();self.active=true;work+=1;continue;}
                    let step=next_index.lookup(next,base.snapshot.part_path(self.index),&mut self.lookup,maximum_work-work);work+=step.work;
                    if let Some(error)=step.error{return IdmlChangeStep{work,done:false,event:None,error:Some(error)};}
                    if step.done{self.active=false;let before=self.index;self.index+=1;if step.index.is_none(){return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Remove{before}),error:None};}}
                }
                "update"=>{
                    if self.index==next.snapshot.part_count(){self.mode="done";work+=1;continue;}
                    if !self.active{self.lookup.restart();self.active=true;work+=1;continue;}
                    let step=base_index.lookup(base,next.snapshot.part_path(self.index),&mut self.lookup,maximum_work-work);work+=step.work;
                    if let Some(error)=step.error{return IdmlChangeStep{work,done:false,event:None,error:Some(error)};}
                    if step.done{self.active=false;if let Some(before)=step.index{self.before=before;self.offset=0;self.mode="bytes";}else{let next=self.index;self.index+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Part{next,before:None}),error:None};}}
                }
                "bytes"=>{
                    let a=base.data(self.before);let b=next.data(self.index);
                    if a.len()!=b.len(){let next=self.index;self.index+=1;self.mode="update";work+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Part{next,before:Some(self.before)}),error:None};}
                    if self.offset==a.len(){self.index+=1;self.mode="update";work+=1;continue;}
                    if a[self.offset]!=b[self.offset]{let next=self.index;self.index+=1;self.mode="update";work+=1;return IdmlChangeStep{work,done:false,event:Some(IdmlChangeEvent::Part{next,before:Some(self.before)}),error:None};}self.offset+=1;work+=1;
                }
                _=>return IdmlChangeStep{work,done:false,event:None,error:Some("stdio.idml.change-state")},
            }
        }
        IdmlChangeStep{work,done:self.mode=="done",event:None,error:None}
    }
}

/// 🧳 Genuine original fields enter a real typed sparse member patch.
#[path="🧳️part/🦀️.rs"]
pub mod retained_part;
