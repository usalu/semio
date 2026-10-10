// 🧵️ Child framing borrows original paged semantic text and retains its symbol frontier explicitly.
use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_pack::PackRefusal,os_spr::operation_bytes::{OperationByteOutput,OperationByteLimitedOutput,OperationSourceCollection,with_operation_encode_policy}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,list::PagedList,paged_text::PagedText,ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand};
use semio_framework_ui_locale::{Terminology,Locale};
use std::cmp::Ordering;
const CAPACITY:usize=isize::MAX as usize;

/// 🪟️ Both authored literals and validated semantic pages expose their original octets.
#[derive(Clone,Copy)]
pub enum ChildTextView<'a>{Literal(&'a str),Paged(&'a PagedText<CAPACITY>)}
impl ChildTextView<'_>{
    pub fn len(self)->usize{match self{Self::Literal(text)=>text.len(),Self::Paged(text)=>text.byte_len()}}
    pub fn byte(self,index:usize)->Result<u8,ValueError>{match self{Self::Literal(text)=>text.as_bytes().get(index).copied(),Self::Paged(text)=>text.borrow()?.byte_at(index)}.ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"child semantic text ordinal changed"))}
}
fn fault(kind:ValueRefusalKind,detail:&'static str)->ValueError{ValueError::new(kind,detail)}

/// 🎟️ A nested owner receives one item and one level less depth than its parent, with every byte axis unchanged.
pub fn nested_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
/// 🚦️ An under-granted axis yields, while a depth below the quote is refused.
pub fn grant_funds(demand:RetirementDemand,grant:RetainedCloneGrant)->Result<bool,ValueError>{
    if grant.maximum_items==0{return Ok(false)}
    if grant.maximum_depth<demand.depth{return Err(fault(ValueRefusalKind::DepthLimit,"child retirement turn exceeds its admitted depth"))}
    Ok(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes)
}
/// 📏️ A parent forwarding into a nested owner needs one level more than that owner.
pub fn deeper(demand:RetirementDemand)->RetirementDemand{RetirementDemand{depth:demand.depth+1,..demand}}
/// 📏️ Reads the four independent quotes of an erased owner.
pub fn quote(owner:&dyn ErasedSnapshotRetirement)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(0)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
/// 🤝️ Hands one retained authority off without moving payload.
pub fn handoff()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
/// 💤️ A turn that cannot yet fund its quote spends nothing.
pub fn yielded()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress::default())}
/// 🔁️ A parent never completes through its child's receipt; it completes only when it is itself terminal on entry.
pub fn settle(step:RetainedCloneStep)->RetainedCloneStep{RetainedCloneStep::Progress(step.progress())}
/// 📏️ Quotes the removal of the retained tail element.
pub fn pop_demand<T,const N:usize>(list:&PagedList<T,N>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:list.next_pop_depth_demand().map_err(ValueError::from)?,..Default::default()})}
/// 📏️ Quotes the physical release of the next whole empty backing allocation.
pub fn release_demand<T,const N:usize>(list:&PagedList<T,N>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{release_bytes:list.next_release_allocation_bytes().map_err(ValueError::from)?,depth:list.next_release_depth_demand().map_err(ValueError::from)?,..Default::default()})}
/// ♻️ Releases one whole empty backing allocation under the granted release axis.
pub fn release_page<T,const N:usize>(list:&mut PagedList<T,N>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=list.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()}))}

/// 🏷️ Locale and terminology fields borrow exact semantic owners rather than synthesizing strings.
pub trait PagedChildLabels:Send+Sync{fn len(&self)->usize;fn text(&self,label:usize,terminology:Terminology,locale:Locale)->Option<ChildTextView<'_>>;}
pub struct PagedChildGroup<'a>{pub owner:ChildTextView<'a>,pub slot:ChildTextView<'a>,pub child_id:ChildTextView<'a>,pub schema:ChildTextView<'a>,pub operations:&'a dyn OperationSourceCollection,pub labels:&'a dyn PagedChildLabels}
pub trait PagedChildGroups:Send+Sync{fn len(&self)->usize;fn group(&self,index:usize)->Option<PagedChildGroup<'_>>;}

#[derive(Clone,Copy)]
struct TextOrdinal{group:usize,field:usize,label:usize,terminology:Terminology,locale:Locale,repeated:bool}
fn view<'a>(groups:&'a dyn PagedChildGroups,ordinal:TextOrdinal)->Result<ChildTextView<'a>,ValueError>{
    let group=groups.group(ordinal.group).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"child symbol group changed"))?;
    match ordinal.field{0=>Ok(group.owner),1=>Ok(group.slot),2=>Ok(group.child_id),3=>Ok(group.schema),4=>group.labels.text(ordinal.label,ordinal.terminology,ordinal.locale).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"child symbol locale changed")),_=>Err(fault(ValueRefusalKind::InvariantViolated,"child symbol field changed"))}
}
fn compare(left:ChildTextView<'_>,right:ChildTextView<'_>,control:&mut NativeEncodeControl<'_>)->Result<Ordering,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(left.len().min(right.len()))?;for index in 0..left.len().min(right.len()){let order=left.byte(index)?.cmp(&right.byte(index)?);control.step()?;if order!=Ordering::Equal{return Ok(order)}}Ok(left.len().cmp(&right.len()))})
}

/// 🗂️ Actual admitted symbol pages remain caller-owned on both encoder results.
pub struct ChildSymbolOwner{entries:PagedList<TextOrdinal,CAPACITY>,finished:bool}
impl ChildSymbolOwner{
    pub const fn empty()->Self{Self{entries:PagedList::empty(),finished:false}}
    fn note(&mut self,groups:&dyn PagedChildGroups,ordinal:TextOrdinal,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let text=view(groups,ordinal)?;control.scoped_stage(|control|{control.begin_stage(text.len())?;for index in 0..text.len(){text.byte(index)?;control.step()?;}Ok::<_,ValueError>(())})?;
        let(mut low,mut high)=(0,self.entries.len());while low<high{let middle=low+(high-low)/2;match compare(view(groups,self.entries[middle])?,text,control)?{Ordering::Less=>low=middle+1,Ordering::Greater=>high=middle,Ordering::Equal=>{self.entries.get_mut(middle).unwrap().repeated=true;return Ok(())}}}
        while !self.entries.has_reserved_slot(){let required=self.entries.next_allocation_bytes().map_err(ValueError::from)?;if required>4096{return Err(fault(ValueRefusalKind::OwnershipLimit,"child symbol page exceeds its physical grant"))}control.charge(required)?;let step=self.entries.reserve_one(4096).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(fault(ValueRefusalKind::InvariantViolated,"child symbol page differs from exact admission"))}}
        self.entries.push_reserved(ordinal).map_err(|_|fault(ValueRefusalKind::InvariantViolated,"child symbol lost admitted slot"))?;
        for index in (low+1..self.entries.len()).rev(){control.step()?;let previous=self.entries[index-1];*self.entries.get_mut(index).unwrap()=previous;}*self.entries.get_mut(low).unwrap()=ordinal;Ok(())
    }
    fn collect(&mut self,groups:&dyn PagedChildGroups,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        if self.entries.allocated_bytes()!=0||self.finished{return Err(fault(ValueRefusalKind::InvariantViolated,"child encoder requires an empty retained symbol owner"))}
        for index in 0..groups.len(){control.step()?;let group=groups.group(index).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"child group ordinal changed"))?;for field in 0..4{self.note(groups,TextOrdinal{group:index,field,label:0,terminology:Terminology::ALL[0],locale:Locale::ALL[0],repeated:false},control)?;}for label in 0..group.labels.len(){for terminology in Terminology::ALL{for locale in Locale::ALL{self.note(groups,TextOrdinal{group:index,field:4,label,terminology,locale,repeated:false},control)?;}}}}
        let mut output=0;for index in 0..self.entries.len(){control.step()?;let ordinal=self.entries[index];if view(groups,ordinal)?.len()<=128||ordinal.repeated{*self.entries.get_mut(output).unwrap()=ordinal;output+=1;}}
        while self.entries.len()>output{control.step()?;self.entries.pop();}if output as u64>u64::from(options.limits.max_symbols){return Err(fault(ValueRefusalKind::WorkLimit,"child symbols exceed full caller policy"))}self.finished=true;Ok(())
    }
    fn index(&self,groups:&dyn PagedChildGroups,text:ChildTextView<'_>,control:&mut NativeEncodeControl<'_>)->Result<Option<usize>,ValueError>{let(mut low,mut high)=(0,self.entries.len());while low<high{let middle=low+(high-low)/2;match compare(view(groups,self.entries[middle])?,text,control)?{Ordering::Less=>low=middle+1,Ordering::Greater=>high=middle,Ordering::Equal=>return Ok(Some(middle))}}Ok(None)}
    pub fn allocated_bytes(&self)->usize{self.entries.allocated_bytes()}
}
impl ChildSymbolOwner{
 fn demands(&self,_:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.entries.is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<TextOrdinal>(),depth:1,..Default::default()})}Ok(RetirementDemand{release_bytes:self.entries.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}
}
impl ErasedSnapshotRetirement for ChildSymbolOwner{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}let demand=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original child retirement exceeds caller depth"))}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}if self.entries.pop().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}let step=self.entries.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}))}
 fn terminal_is_empty(&self)->bool{self.entries.terminal_is_empty()}

 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl Drop for ChildSymbolOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"child symbol owner retains actual backing pages");}}
struct Writer<'a>{output:&'a mut dyn OperationByteOutput,completed:usize,options:&'a PackEncodeOptions}
impl Writer<'_>{
    fn bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.output.write_bytes(bytes,control)?;self.completed=self.completed.checked_add(bytes.len()).ok_or_else(||fault(ValueRefusalKind::WorkLimit,"child encoded source extent overflow"))?;Ok(())}
    fn byte(&mut self,byte:u8,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.bytes(&[byte],control)}
    fn unsigned(&mut self,mut value:u64,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{let mut bytes=[0;10];let mut length=0;loop{bytes[length]=(value&127)as u8;value>>=7;if value!=0{bytes[length]|=128;}length+=1;if value==0{break}}self.bytes(&bytes[..length],control)}
    fn count(&mut self,count:usize,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{if count as u64>self.options.limits.max_items{return Err(fault(ValueRefusalKind::WorkLimit,"child collection exceeds full caller item policy").into())}self.unsigned(count as u64,control)}
    fn text_bytes(&mut self,text:ChildTextView<'_>,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{if text.len()as u64>self.options.limits.max_segment_len{return Err(fault(ValueRefusalKind::WorkLimit,"child semantic text exceeds caller segment policy").into())}let mut bytes=[0;256];let mut position=0;while position<text.len(){let length=(text.len()-position).min(bytes.len());for(index,target)in bytes[..length].iter_mut().enumerate(){*target=text.byte(position+index)?;}self.bytes(&bytes[..length],control)?;position+=length;}Ok(())}
    fn text(&mut self,groups:&dyn PagedChildGroups,symbols:&ChildSymbolOwner,text:ChildTextView<'_>,inline:bool,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{if !inline{if let Some(index)=symbols.index(groups,text,control)?{self.byte(6,control)?;return self.unsigned(index as u64,control)}}self.byte(7,control)?;self.unsigned(text.len()as u64,control)?;self.text_bytes(text,control)}
    fn key(&mut self,groups:&dyn PagedChildGroups,symbols:&ChildSymbolOwner,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),PackRefusal>{self.text(groups,symbols,ChildTextView::Literal(text),true,control)}
}

/// 🎞️ Exact declared child framing keeps semantic source pages and symbol cleanup under separate owners.
pub fn encode_paged_groups_into(groups:&dyn PagedChildGroups,options:&PackEncodeOptions,output:&mut dyn OperationByteOutput,symbols:&mut ChildSymbolOwner,control:&mut NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
    with_operation_encode_policy(options,control,|control|{
        control.checkpoint()?;if groups.len()==0{return Ok(0)}if options.limits.max_depth<4{return Err(fault(ValueRefusalKind::DepthLimit,"actual child group shape exceeds caller depth policy").into())}symbols.collect(groups,options,control)?;
        let mut limited=OperationByteLimitedOutput::new(output,options.limits.max_file_len);let mut writer=Writer{output:&mut limited,completed:0,options};writer.unsigned(symbols.entries.len()as u64,control)?;for ordinal in symbols.entries.iter(){let text=view(groups,*ordinal)?;writer.unsigned(text.len()as u64,control)?;writer.text_bytes(text,control)?;}
        writer.bytes(&[1,1,17,12],control)?;writer.count(groups.len(),control)?;
        for index in 0..groups.len(){control.checkpoint()?;let group=groups.group(index).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"encoded child group ordinal changed"))?;writer.byte(16,control)?;writer.count(6,control)?;
            for(key,text)in[("owner",group.owner),("slot",group.slot),("child_id",group.child_id)]{writer.key(groups,symbols,key,control)?;writer.text(groups,symbols,text,false,control)?;}
            writer.key(groups,symbols,"ops",control)?;writer.byte(12,control)?;writer.count(group.operations.len(),control)?;
            if group.operations.len()!=0&&options.limits.max_depth<5{return Err(fault(ValueRefusalKind::DepthLimit,"actual child operation shape exceeds caller depth policy").into())}
            for operation in 0..group.operations.len(){let source=group.operations.source_at(operation).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"encoded child operation ordinal changed"))?;if !source.is_empty()&&options.limits.max_depth<6{return Err(fault(ValueRefusalKind::DepthLimit,"actual child octet shape exceeds caller depth policy").into())}writer.byte(12,control)?;writer.count(source.len(),control)?;for byte in source.iter(){writer.byte(4,control)?;writer.unsigned(u64::from(byte),control)?;}}
            writer.key(groups,symbols,"op_schema",control)?;writer.text(groups,symbols,group.schema,false,control)?;writer.key(groups,symbols,"labels",control)?;writer.byte(12,control)?;writer.count(group.labels.len(),control)?;
            if group.labels.len()!=0&&options.limits.max_depth<7{return Err(fault(ValueRefusalKind::DepthLimit,"actual child label shape exceeds caller depth policy").into())}
            for label in 0..group.labels.len(){writer.byte(16,control)?;writer.count(Terminology::COUNT,control)?;for terminology in Terminology::ALL{writer.key(groups,symbols,terminology.as_str(),control)?;writer.byte(16,control)?;writer.count(Locale::COUNT,control)?;for locale in Locale::ALL{writer.key(groups,symbols,locale.as_str(),control)?;let text=group.labels.text(label,terminology,locale).ok_or_else(||fault(ValueRefusalKind::InvariantViolated,"encoded child locale ordinal changed"))?;writer.text(groups,symbols,text,false,control)?;}}}
        }Ok(writer.completed)
    })
}
