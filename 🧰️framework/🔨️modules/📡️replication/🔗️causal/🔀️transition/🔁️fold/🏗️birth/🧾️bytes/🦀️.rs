//! 🧾️ Original source scalars and reserved destinations advance only after their independent byte receipts are admitted.
use super::*;
use semio_framework_value::ByteReadSource;

struct FoldByteRead<'a,S:ByteReadSource+?Sized> { control:HistoryFoldControl,source:&'a S,position:&'a mut usize,copy:usize }
impl<S:ByteReadSource+?Sized> Unpin for FoldByteRead<'_,S>{}
impl<S:ByteReadSource+?Sized> Future for FoldByteRead<'_,S>{
    type Output=Option<u8>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<this.copy{return Poll::Pending;}
        let value=this.source.byte_at(*this.position);
        if value.is_some(){*this.position+=1;}
        this.control.accept_original_progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Self>()+usize::from(value.is_some()),..Default::default()});
        this.control.0.pending_original_copy.fetch_sub(this.copy,Ordering::Relaxed);this.copy=0;
        Poll::Ready(value)
    }
}
impl<S:ByteReadSource+?Sized> Drop for FoldByteRead<'_,S>{fn drop(&mut self){self.control.0.pending_original_copy.fetch_sub(self.copy,Ordering::Relaxed);}}

enum FoldByteDestination<'a>{Bytes(&'a mut Vec<u8>),Text(&'a mut String)}
struct FoldByteCopy<'a,S:ByteReadSource+?Sized>{control:HistoryFoldControl,source:&'a S,position:&'a mut usize,end:usize,target:FoldByteDestination<'a>,scalar:[u8;4],width:usize,needed:usize,copy:usize}
impl<S:ByteReadSource+?Sized> Unpin for FoldByteCopy<'_,S>{}
impl<S:ByteReadSource+?Sized> Future for FoldByteCopy<'_,S>{
    type Output=Result<(),(usize,&'static str)>;
    fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let this=self.get_mut();let grant=this.control.original_grant();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<this.copy{return Poll::Pending;}
        let mut copied=2*size_of::<Self>();let mut failure=None;
        if *this.position<this.end{
            let Some(byte)=this.source.byte_at(*this.position)else{return Poll::Ready(Err((*this.position,"truncated original byte source")))};copied+=1;
            match &mut this.target{
                FoldByteDestination::Bytes(target)=>{target.push(byte);*this.position+=1;copied+=1;},
                FoldByteDestination::Text(target)=>{
                    if this.width==0{this.needed=match byte{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>0};}
                    if this.needed==0{failure=Some((*this.position,"invalid UTF-8"));}
                    else{
                        this.scalar[this.width]=byte;this.width+=1;*this.position+=1;
                        if this.width==this.needed{
                            match std::str::from_utf8(&this.scalar[..this.width]){Ok(value)=>{target.push_str(value);copied+=this.width;this.width=0;},Err(_)=>failure=Some((*this.position-this.width,"invalid UTF-8"))}
                        }
                    }
                }
            }
        }
        this.control.accept_original_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()});
        let result=if let Some(error)=failure{Some(Err(error))}else if *this.position==this.end{Some(if this.width==0{Ok(())}else{Err((*this.position,"truncated UTF-8"))})}else{None};
        if let Some(result)=result{this.control.0.pending_original_copy.fetch_sub(this.copy,Ordering::Relaxed);this.copy=0;Poll::Ready(result)}else{Poll::Pending}
    }
}
impl<S:ByteReadSource+?Sized> Drop for FoldByteCopy<'_,S>{fn drop(&mut self){self.control.0.pending_original_copy.fetch_sub(self.copy,Ordering::Relaxed);}}

impl HistoryFoldControl{
    /// 🔢️ Reads one original source byte per admitted scalar frontier without allocating a parser buffer.
    pub async fn read_byte<S:ByteReadSource+?Sized>(&self,source:&S,position:&mut usize)->Result<u8,semio_framework_replication_error::ProtocolError>{
        let copy=2*size_of::<FoldByteRead<'_,S>>()+1;self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);
        FoldByteRead{control:self.clone(),source,position:&mut *position,copy}.await.ok_or_else(||semio_framework_pack_error::PackRefusal::Truncated(*position as u64).into())
    }

    /// 📏️ Decodes the exact ten-byte u64 wire contract using the original scalar receiving frontier.
    pub async fn read_varint<S:ByteReadSource+?Sized>(&self,source:&S,position:&mut usize)->Result<u64,semio_framework_replication_error::ProtocolError>{
        let start=*position;let mut value=0;
        for index in 0..10{
            let byte=self.read_byte(source,position).await?;let payload=byte&127;
            if index==9&&(byte&128!=0||payload>1){return Err(semio_framework_pack_error::PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"varint",offset:start as u64,detail:"overlong varint (exceeds 10 bytes / 64 bits)"}.into());}
            value|=(payload as u64)<<(7*index);if byte&128==0{return Ok(value);}
        }
        unreachable!("the tenth original varint byte is terminal or refused")
    }

    /// 📝️ Copies the original UTF8 source into an already admitted text backing, preserving partial scalars on cancellation.
    pub async fn copy_text_into<S:ByteReadSource+?Sized>(&self,target:&mut String,source:&S,position:&mut usize,end:usize)->Result<(),semio_framework_replication_error::ProtocolError>{
        if end>source.byte_len()||*position>end||target.capacity()-target.len()<end-*position{return Err(value_cause(ValueError::literal(ValueRefusalKind::InvalidValue,"history text destination was not reserved")));}
        let copy=2*size_of::<FoldByteCopy<'_,S>>()+5;self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);
        match (FoldByteCopy{control:self.clone(),source,position,end,target:FoldByteDestination::Text(target),scalar:[0;4],width:0,needed:0,copy}).await{
            Ok(())=>Ok(()),Err((offset,detail))=>{
                let mut original=self.text(detail.len()).await?;let mut at=0;
                let copy=2*size_of::<FoldByteCopy<'_,[u8]>>()+5;self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);
                (FoldByteCopy{control:self.clone(),source:detail.as_bytes(),position:&mut at,end:detail.len(),target:FoldByteDestination::Text(&mut original),scalar:[0;4],width:0,needed:0,copy}).await.unwrap_or_else(|_|unreachable!("the refusal detail is valid static UTF8"));
                Err(semio_framework_replication_error::ProtocolError::Malformed{what:"history transition",offset:offset as u64,detail:original.take()})
            }
        }
    }

    /// 📦️ Copies source bytes only into actual reserved original vector capacity.
    pub async fn copy_bytes_into<S:ByteReadSource+?Sized>(&self,target:&mut Vec<u8>,source:&S,position:&mut usize,end:usize)->Result<(),semio_framework_replication_error::ProtocolError>{
        if end>source.byte_len()||*position>end||target.capacity()-target.len()<end-*position{return Err(value_cause(ValueError::literal(ValueRefusalKind::InvalidValue,"history byte destination was not reserved")));}
        let copy=2*size_of::<FoldByteCopy<'_,S>>()+5;self.0.pending_original_copy.fetch_add(copy,Ordering::Relaxed);
        (FoldByteCopy{control:self.clone(),source,position,end,target:FoldByteDestination::Bytes(target),scalar:[0;4],width:0,needed:0,copy}).await.map_err(|_|unreachable!("binary copy has no UTF8 refusal"))
    }
}

#[path="📦️.rs"]
mod payload_pages;
