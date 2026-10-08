//! 📝️ First-party UTF8 text owns separately admitted metadata and physical payload pages.
use crate::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,list::{PagedList,PagedListError},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

/// 🪟️ Immutable UTF8 input exposes original octets without requiring contiguous storage.
pub trait TextReadSource{fn byte_len(&self)->usize;fn byte_at(&self,index:usize)->Option<u8>;}
impl TextReadSource for [u8]{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}}
impl TextReadSource for str{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.as_bytes().get(index).copied()}}
impl<T:TextReadSource+?Sized> TextReadSource for &T{fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}}
fn error(error:PagedListError)->ValueError{ValueError::new(match error.kind{crate::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,crate::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,crate::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason)}
fn character(source:&impl TextReadSource,position:&mut usize)->Result<char,ValueError>{
    let first=source.byte_at(*position).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"truncated paged UTF8"))?;
    let count=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid paged UTF8"))};
    let mut bytes=[0;4];for(index,byte)in bytes[..count].iter_mut().enumerate(){*byte=source.byte_at(*position+index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"truncated paged UTF8"))?;}
    let value=std::str::from_utf8(&bytes[..count]).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid paged UTF8"))?.chars().next().unwrap();*position+=count;Ok(value)
}
trait TextCopyControl{fn scope_copy(&mut self,body:impl FnOnce(&mut Self)->Result<(),ValueError>)->Result<(),ValueError>;fn begin_copy(&mut self,total:usize)->Result<(),ValueError>;fn advance_copy(&mut self,units:usize)->Result<(),ValueError>;fn admit_copy(&mut self,bytes:usize)->Result<(),ValueError>;}
macro_rules! text_copy_control{($control:ty)=>{impl TextCopyControl for $control{fn scope_copy(&mut self,body:impl FnOnce(&mut Self)->Result<(),ValueError>)->Result<(),ValueError>{self.scoped_stage(body)}fn begin_copy(&mut self,total:usize)->Result<(),ValueError>{self.begin_stage(total)}fn advance_copy(&mut self,units:usize)->Result<(),ValueError>{self.advance(units)}fn admit_copy(&mut self,bytes:usize)->Result<(),ValueError>{self.charge(bytes)}}};}
text_copy_control!(NativeDecodeControl<'_>);
text_copy_control!(crate::NativeEncodeControl<'_>);
/// 📚️ Semantic UTF8 backing retains all admitted allocations on canceled or refused construction.
pub struct PagedText<const N:usize>{bytes:PagedList<u8,N>,complete:bool,closing:bool}
impl<const N:usize> PagedText<N>{
    /// 🈳️ Creates a stack-resident text owner with no physical allocation.
    pub const fn empty()->Self{Self{bytes:PagedList::empty(),complete:false,closing:false}}
    pub fn byte_len(&self)->usize{self.bytes.len()}
    pub fn allocated_bytes(&self)->usize{self.bytes.allocated_bytes()}
    /// 🏠️ Transfers genuine empty backing to its admitted parent without physical disposal credit.
    pub fn return_one<const P:usize>(&mut self,parent:&mut crate::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<crate::list::PagedListReturnProgress,ValueError>{
        if maximum_items==0||self.bytes.terminal_is_empty(){return Ok(crate::list::PagedListReturnProgress::default())}
        self.closing=true;
        if self.bytes.pop().is_some(){return Ok(crate::list::PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        self.bytes.return_empty_page(parent,1)
    }

    /// 👓️ Exposes text only after complete UTF8 validation and before retirement starts.
    pub fn borrow(&self)->Result<TextReadSpan<'_,N>,ValueError>{if !self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text lacks a complete live UTF8 capability"));}Ok(TextReadSpan{owner:self})}
    /// 🛬️ Copies original semantic octets after each exact allocation admission, retaining partial pages on every failure.
    pub fn read_from_source(&mut self,source:&impl TextReadSource,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    /// ✍️ Owns original semantic text directly under the producer's cumulative encoding authority.
    pub fn read_from_encoding_source(&mut self,source:&impl TextReadSource,control:&mut crate::NativeEncodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    fn read_controlled<C:TextCopyControl>(&mut self,source:&impl TextReadSource,control:&mut C)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text source construction requires an empty owner"));}
        let length=source.byte_len();if length>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"paged text exceeds declared logical capacity"));}
        control.scope_copy(|control|{control.begin_copy(length)?;let mut position=0;while position<length{let previous=position;character(source,&mut position)?;control.advance_copy(position-previous)?;}Ok::<_,ValueError>(())})?;
        control.scope_copy(|control|{
            control.begin_copy(length)?;let mut position=0;
            while position<length{
                let value=character(source,&mut position)?;let mut octets=[0;4];
                for byte in value.encode_utf8(&mut octets).as_bytes(){
                    while !self.bytes.has_reserved_slot(){
                        let required=self.bytes.next_exact_capacity_allocation_bytes(length).map_err(error)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"known text extent has no next payload slot"))?;control.admit_copy(required)?;
                        let step=self.bytes.reserve_exact_capacity_one(length,required).map_err(|failure|error(failure.refusal()))?;
                        if !step.progressed||step.allocated_bytes!=required{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text allocation differs from its exact admission"));}
                    }
                    self.bytes.push_reserved(*byte).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"paged text rejected its funded byte"))?;control.advance_copy(1)?;
                }
            }
            Ok::<_,ValueError>(())
        })?;self.complete=true;Ok(())
    }
}
impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<1{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"paged text retirement requires admitted depth"));}
        if !self.bytes.is_empty(){
            let count=grant.maximum_copy_bytes.min(self.bytes.len());
            if count==0{return Ok(RetainedCloneStep::Progress(empty));}
            self.closing=true;
            for _ in 0..count{self.bytes.pop().unwrap();}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:count,..empty}));
        }
        let step=self.bytes.release_empty_page(grant.maximum_release_bytes).map_err(error)?;
        if step.progressed{self.closing=true;}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..empty}))
    }
    fn terminal_is_empty(&self)->bool{self.bytes.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.bytes.is_empty()))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.bytes.is_empty(){Ok(0)}else{self.bytes.next_release_allocation_bytes().map_err(error)}}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
}
struct PagedTextRetirement<const N:usize>{owner:PagedText<N>}
impl<const N:usize> crate::retirement::RetirementCursor for PagedTextRetirement<N>{
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::retirement::RetirementStep{match self.owner.close_step(grant){Err(error)=>crate::retirement::RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>crate::retirement::RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>crate::retirement::RetirementStep::Progress(progress)}}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<const N:usize> crate::retirement::RetireOwned for PagedText<N>{
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(PagedTextRetirement{owner:self})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PagedTextRetirement<N>>())}
    fn controlled_retirement_supported()->bool{true}
}
impl<const N:usize> Drop for PagedText<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"paged semantic text retains actual physical allocations");}}

/// 👓️ Borrows validated original UTF8 storage without flattening its physical ownership.
#[derive(Clone,Copy)]
pub struct TextReadView<'a>{source:TextReadRepresentation<'a>}
#[derive(Clone,Copy)]
enum TextReadRepresentation<'a>{Contiguous(&'a str),Paged(&'a dyn TextReadSource)}
impl<'a> TextReadView<'a>{
    pub const fn from_str(source:&'a str)->Self{Self{source:TextReadRepresentation::Contiguous(source)}}
    pub fn len(&self)->usize{match self.source{TextReadRepresentation::Contiguous(source)=>source.len(),TextReadRepresentation::Paged(source)=>source.byte_len()}}
    pub fn is_empty(&self)->bool{self.len()==0}
    pub fn byte_at(&self,index:usize)->Option<u8>{match self.source{TextReadRepresentation::Contiguous(source)=>source.as_bytes().get(index).copied(),TextReadRepresentation::Paged(source)=>source.byte_at(index)}}
    /// 🪪️ Exposes a contiguous original only when its representation already provides it.
    pub fn contiguous(&self)->Option<&'a str>{match self.source{TextReadRepresentation::Contiguous(source)=>Some(source),TextReadRepresentation::Paged(_)=>None}}
    /// 📥️ Copies at most the supplied output window from the same immutable original storage.
    pub fn copy_bytes(&self,offset:usize,output:&mut[u8])->Result<usize,ValueError>{
        let remaining=self.len().checked_sub(offset).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"text source offset exceeds its original extent"))?;
        let count=remaining.min(output.len());for(index,byte)in output[..count].iter_mut().enumerate(){*byte=self.byte_at(offset+index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"text source lost an original octet"))?;}Ok(count)
    }
    /// 🔤️ Advances one validated Unicode scalar, including a scalar crossing an original page boundary.
    pub fn character_at(&self,position:&mut usize)->Result<char,ValueError>{character(self,position)}
}
impl std::fmt::Display for TextReadView<'_>{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{let mut position=0;while position<self.len(){let character=self.character_at(&mut position).map_err(|_|std::fmt::Error)?;let mut bytes=[0;4];formatter.write_str(character.encode_utf8(&mut bytes))?;}Ok(())}}
impl TextReadSource for TextReadView<'_>{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.byte_at(index)}}
impl<const N:usize> TextReadSource for PagedText<N>{fn byte_len(&self)->usize{self.byte_len()}fn byte_at(&self,index:usize)->Option<u8>{self.bytes.get(index).copied()}}
impl<const N:usize> PagedText<N>{
    /// 🌱️ Captures the actual validated paged owner directly; partial and retiring owners refuse access.
    pub fn read_view(&self)->Result<TextReadView<'_>,ValueError>{let _=self.borrow()?;Ok(TextReadView{source:TextReadRepresentation::Paged(self)})}
}

/// 🔤️ A complete text borrow streams validated bytes and Unicode scalars without flattening.
pub struct TextReadSpan<'a,const N:usize>{owner:&'a PagedText<N>}
impl<const N:usize> TextReadSpan<'_,N>{
    pub fn byte_len(&self)->usize{self.owner.byte_len()}
    pub fn byte_at(&self,index:usize)->Option<u8>{self.owner.bytes.get(index).copied()}
    pub fn bytes(&self)->impl Iterator<Item=u8>+'_ {self.owner.bytes.iter().copied()}
    pub fn chars(&self)->TextChars<'_,N>{TextChars{source:self,position:0}}
}
impl<const N:usize> TextReadSource for TextReadSpan<'_,N>{fn byte_len(&self)->usize{self.byte_len()}fn byte_at(&self,index:usize)->Option<u8>{self.byte_at(index)}}
pub struct TextChars<'a,const N:usize>{source:&'a TextReadSpan<'a,N>,position:usize}
impl<const N:usize> Iterator for TextChars<'_,N>{type Item=char;fn next(&mut self)->Option<char>{if self.position==self.source.byte_len(){None}else{Some(character(&self.source,&mut self.position).expect("validated original paged UTF8"))}}}

/// ✍️ Validated semantic text writes retain their actual prefix under the supplied native authority.
pub trait TextEncodingOutput{fn measure_text_fragment(&mut self,accumulated_bytes:usize,text:&str)->Result<(),ValueError>;fn declare_exact_byte_len(&mut self,bytes:usize)->Result<(),ValueError>;fn refuse_text_producer(&mut self);fn write_text(&mut self,text:&str)->Result<(),ValueError>;}
struct TextOutput<'a,'b,const N:usize>{bytes:&'a mut PagedList<u8,N>,control:&'a mut crate::NativeEncodeControl<'b>,maximum_payload_bytes:usize,exact_bytes:Option<usize>,refused:bool}
impl<const N:usize> TextEncodingOutput for TextOutput<'_,'_,N>{
    fn measure_text_fragment(&mut self,accumulated_bytes:usize,text:&str)->Result<(),ValueError>{self.measure_extent_fragment(accumulated_bytes,text)}
    fn declare_exact_byte_len(&mut self,bytes:usize)->Result<(),ValueError>{self.declare_extent(bytes)}
    fn refuse_text_producer(&mut self){self.refused=true;}
    fn write_text(&mut self,text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output previously refused"))}
        let result=self.control.scoped_stage(|control|{
            self.bytes.len().checked_add(text.len()).filter(|length|*length<=self.maximum_payload_bytes&&*length<=N&&self.exact_bytes.is_none_or(|exact|*length<=exact)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic text output exceeds its logical payload authority"))?;
            control.begin_stage(text.len())?;
            for byte in text.as_bytes(){
                while !self.bytes.has_reserved_slot(){let required=match self.exact_bytes{Some(limit)=>self.bytes.next_exact_capacity_allocation_bytes(limit).map_err(error)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"declared text extent has no next payload slot"))?,None=>self.bytes.next_allocation_bytes().map_err(error)?};control.charge(required)?;let step=match self.exact_bytes{Some(limit)=>self.bytes.reserve_exact_capacity_one(limit,required),None=>self.bytes.reserve_one(required)}.map_err(|failure|error(failure.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output allocation differs from exact admission"))}}
                self.bytes.push_reserved(*byte).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text output lost its funded byte"))?;control.step()?;
            }Ok::<_,ValueError>(())
        });if result.is_err(){self.refused=true;}result
    }
}
impl<const N:usize> PagedText<N>{
    /// 🧾 Only an accepted whole producer establishes a live UTF8 borrow; every refusal retains the prefix.
    pub fn encode_with(&mut self,maximum_payload_bytes:usize,control:&mut crate::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn TextEncodingOutput)->Result<(),ValueError>)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text encoding requires an empty owner"))}
        if maximum_payload_bytes>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic text encoding payload authority exceeds its declared capacity"))}
        let mut output=TextOutput{bytes:&mut self.bytes,control,maximum_payload_bytes,exact_bytes:None,refused:false};producer(&mut output)?;
        if output.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic text producer ignored a refused write"))}
        if output.exact_bytes.is_some_and(|exact|output.bytes.len()!=exact){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic producer changed its declared complete extent"))}
        self.complete=true;Ok(())
    }
}
struct TextFormatOutput<'a>{output:&'a mut dyn TextEncodingOutput,refusal:Option<ValueError>}
impl std::fmt::Write for TextFormatOutput<'_>{
    fn write_str(&mut self,text:&str)->std::fmt::Result{if self.refusal.is_some(){return Err(std::fmt::Error)}match self.output.write_text(text){Ok(())=>Ok(()),Err(error)=>{self.refusal=Some(error);Err(std::fmt::Error)}}}
}
/// 🔤️ Streams authored formatting into the real text owner without creating a contiguous String.
pub fn write_encoding_format(output:&mut dyn TextEncodingOutput,arguments:std::fmt::Arguments<'_>)->Result<(),ValueError>{
    let mut counter=TextExtentCounter{output,bytes:0,refusal:None};let counted=std::fmt::write(&mut counter,arguments);if let Some(error)=counter.refusal{counter.output.refuse_text_producer();return Err(error)}if counted.is_err(){counter.output.refuse_text_producer();return Err(ValueError::new(ValueRefusalKind::InvalidValue,"semantic formatter refused exact extent measurement"))}let bytes=counter.bytes;counter.output.declare_exact_byte_len(bytes)?;
    let mut writer=TextFormatOutput{output,refusal:None};let result=std::fmt::write(&mut writer,arguments);match(writer.refusal,result){(Some(error),_)=>{writer.output.refuse_text_producer();Err(error)},(None,Ok(()))=>Ok(()),(None,Err(_))=>{writer.output.refuse_text_producer();Err(ValueError::new(ValueRefusalKind::InvalidValue,"semantic text formatter refused its output"))}}
}
/// 🧩️ A finite intrinsic semantic cell retains short and partially transitioned text without heap backing.
pub struct InlineTextBuffer{bytes:[u8;128],length:usize,complete:bool}
impl InlineTextBuffer{
    pub const fn empty()->Self{Self{bytes:[0;128],length:0,complete:false}}
    pub fn as_bytes(&self)->&[u8]{&self.bytes[..self.length]}
    pub fn borrow(&self)->Result<&str,ValueError>{if !self.complete{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic text lacks a complete live UTF8 capability"))}std::str::from_utf8(self.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"complete intrinsic UTF8 changed"))}
    pub fn close_one(&mut self,maximum_items:usize)->bool{if maximum_items==0||self.length==0{return false}self.length=0;self.complete=false;true}
    pub fn terminal_is_empty(&self)->bool{self.length==0}
    pub fn read_from_source(&mut self,source:&impl TextReadSource,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    pub fn read_from_encoding_source(&mut self,source:&impl TextReadSource,control:&mut crate::NativeEncodeControl<'_>)->Result<(),ValueError>{self.read_controlled(source,control)}
    fn read_controlled<C:TextCopyControl>(&mut self,source:&impl TextReadSource,control:&mut C)->Result<(),ValueError>{
        if self.length!=0||self.complete{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic text source construction requires an empty owner"))}
        if source.byte_len()>self.bytes.len(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic text exceeds declared128-byte capacity"))}
        control.scope_copy(|control|{control.begin_copy(source.byte_len())?;for index in 0..source.byte_len(){self.bytes[index]=source.byte_at(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"intrinsic text source changed"))?;self.length+=1;control.advance_copy(1)?;}std::str::from_utf8(self.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"intrinsic semantic source is not UTF8"))?;Ok::<_,ValueError>(())})?;self.complete=true;Ok(())
    }
}
struct InlineTextOutput<'a,'b,const N:usize>{paged:TextOutput<'a,'b,N>,inline:&'a mut InlineTextBuffer,paged_active:&'a mut bool,logical_bytes:usize,refused:bool}
impl<const N:usize> TextEncodingOutput for InlineTextOutput<'_,'_,N>{
    fn measure_text_fragment(&mut self,accumulated_bytes:usize,text:&str)->Result<(),ValueError>{self.paged.measure_extent_fragment(accumulated_bytes,text)}
    fn declare_exact_byte_len(&mut self,bytes:usize)->Result<(),ValueError>{if self.logical_bytes!=0||!self.inline.terminal_is_empty(){self.refused=true;return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic output already owns a prefix"))}self.paged.declare_extent(bytes)}
    fn refuse_text_producer(&mut self){self.refused=true;self.paged.refused=true;}
    fn write_text(&mut self,text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text output previously refused"))}
        let result=(||{
            let length=self.logical_bytes.checked_add(text.len()).filter(|length|*length<=self.paged.maximum_payload_bytes&&*length<=N&&self.paged.exact_bytes.is_none_or(|exact|*length<=exact)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic semantic text exceeds logical payload authority"))?;
            if !*self.paged_active&&length<=self.inline.bytes.len(){
                self.paged.control.scoped_stage(|control|{control.begin_stage(text.len())?;for byte in text.as_bytes(){self.inline.bytes[self.inline.length]=*byte;self.inline.length+=1;control.step()?;}Ok::<_,ValueError>(())})?;
            }else{
                if !*self.paged_active{*self.paged_active=true;let prefix=std::str::from_utf8(self.inline.as_bytes()).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"accepted intrinsic semantic prefix changed UTF8"))?;self.paged.write_text(prefix)?;self.inline.close_one(1);}
                self.paged.write_text(text)?;
            }
            self.logical_bytes=length;Ok::<_,ValueError>(())
        })();if result.is_err(){self.refused=true;}result
    }
}
impl<const N:usize> PagedText<N>{
    /// 🧾 One bound producer fills the intrinsic cell or transfers its bounded prefix into genuine pages.
    pub fn encode_with_inline(&mut self,inline:&mut InlineTextBuffer,paged_active:&mut bool,maximum_payload_bytes:usize,control:&mut crate::NativeEncodeControl<'_>,producer:impl FnOnce(&mut dyn TextEncodingOutput)->Result<(),ValueError>)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing||!inline.terminal_is_empty()||inline.complete||*paged_active{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text encoding requires an empty composite owner"))}
        if maximum_payload_bytes>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"intrinsic semantic text authority exceeds declared capacity"))}
        let paged=TextOutput{bytes:&mut self.bytes,control,maximum_payload_bytes,exact_bytes:None,refused:false};let mut output=InlineTextOutput{paged,inline,paged_active,logical_bytes:0,refused:false};producer(&mut output)?;
        if output.refused||output.paged.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic text producer ignored a refused write"))}
        if output.paged.exact_bytes.is_some_and(|exact|output.logical_bytes!=exact){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"intrinsic semantic producer changed its declared complete extent"))}
        self.complete=*output.paged_active;output.inline.complete=!*output.paged_active;Ok(())
    }
}

struct TextExtentCounter<'a>{output:&'a mut dyn TextEncodingOutput,bytes:usize,refusal:Option<ValueError>}
impl std::fmt::Write for TextExtentCounter<'_>{
    fn write_str(&mut self,text:&str)->std::fmt::Result{
        if self.refusal.is_some(){return Err(std::fmt::Error)}
        let Some(next)=self.bytes.checked_add(text.len())else{self.refusal=Some(ValueError::new(ValueRefusalKind::OwnershipLimit,"formatted semantic extent exceeds native authority"));return Err(std::fmt::Error)};
        match self.output.measure_text_fragment(next,text){Ok(())=>{self.bytes=next;Ok(())},Err(error)=>{self.refusal=Some(error);Err(std::fmt::Error)}}
    }
}

impl<const N:usize> TextOutput<'_,'_,N>{
    fn measure_extent_fragment(&mut self,bytes:usize,_text:&str)->Result<(),ValueError>{
        if self.refused{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic extent output previously refused"))}
        if bytes>self.maximum_payload_bytes||bytes>N{self.refused=true;return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"formatted semantic extent exceeds full segment authority"))}
        let result=self.control.scoped_stage(|control|{control.begin_stage(1)?;control.step()});if result.is_err(){self.refused=true;}result
    }
    fn declare_extent(&mut self,bytes:usize)->Result<(),ValueError>{
        if self.refused||self.exact_bytes.is_some()||self.bytes.len()!=0{self.refused=true;return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"semantic output cannot replace its original final extent"))}
        if bytes>self.maximum_payload_bytes||bytes>N{self.refused=true;return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic final extent exceeds original payload authority"))}
        if let Err(error)=self.control.checkpoint(){self.refused=true;return Err(error)}self.exact_bytes=Some(bytes);Ok(())
    }
}
