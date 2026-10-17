//! 🧮️ Controlled allocation of the pack format: the allocation layout and size checks, the controlled vector, the inflater owner, the CRC and the controlled payload and segment appenders. See [`super`].

use super::*;

pub(super) fn check_allocation_layout<T>(count:usize,limit:&'static str)->Result<(),PackRefusal>{
    count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit})?;
    Ok(())
}

pub(super) fn check_controlled_allocation(control:&NativeDecodeControl<'_>,limits:&PackLimits,bytes:usize)->Result<(),PackRefusal>{
    let next=control.owned_bytes().checked_add(bytes).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled allocation overflow"})?;
    if next>control.maximum_bytes()||next as u64>limits.max_total_alloc{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled allocation exceeds caller limits"});}
    Ok(())
}

pub(super) fn controlled_vec<T>(control:&mut NativeDecodeControl<'_>,limits:&PackLimits,count:usize)->Result<Vec<T>,PackRefusal>{
    check_controlled_allocation(control,limits,count.checked_mul(size_of::<T>()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled collection size overflow"})?)?;
    control.allocate_vec(count).map_err(PackRefusal::ValueRefusal)
}

#[cfg(feature="deflate")]
struct OwnedControlledInflater(semio_framework_replication_codec::DeflateRetainedCursor);

#[cfg(feature="deflate")]
impl Drop for OwnedControlledInflater{fn drop(&mut self){while self.0.close_step(256,65536)!=semio_framework_replication_codec::RetainedInflateCloseStep::Complete{}}}

pub(super) async fn controlled_crc<S:PackSource>(source:&S,offset:u64,length:u64,control:&mut NativeDecodeControl<'_>)->Result<u32,S::Error>{
    let total=usize::try_from(length).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled CRC range exceeds address space"})?;
    control.begin_stage(total).map_err(PackRefusal::ValueRefusal)?;
    let mut crc=semio_framework_replication_codec::Crc32cCursor::new();let mut page=[0;4096];let mut position=0;
    while position<total{let count=(total-position).min(page.len());source.read_exact_at(offset+position as u64,&mut page[..count]).await?;crc.update_page(&page[..count]);position+=count;control.advance(count).map_err(PackRefusal::ValueRefusal)?;}
    Ok(crc.finish())
}

pub(super) async fn append_controlled_payload<S:PackSource>(source:&S,offset:u64,stored_len:u64,raw_len:u64,codec:CodecId,output:&mut Vec<u8>,limits:&PackLimits,control:&mut NativeDecodeControl<'_>)->Result<(),S::Error>{
    let expected=usize::try_from(raw_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled payload exceeds address space"})?;
    if expected>output.capacity()-output.len(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled payload exceeds admitted capacity"}).into());}
    control.begin_stage(expected).map_err(PackRefusal::ValueRefusal)?;
    let mut page=[0;4096];let start=output.len();
    match codec.0{
        0=>{
            if stored_len!=raw_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"identity",offset,detail:"stored and decoded lengths differ".into()}).into());}
            let mut position=0;while position<expected{let count=(expected-position).min(page.len());source.read_exact_at(offset+position as u64,&mut page[..count]).await?;output.extend_from_slice(&page[..count]);position+=count;control.advance(count).map_err(PackRefusal::ValueRefusal)?;}
        },
        1=>{
            #[cfg(feature="deflate")]
            {
                let workload=expected.checked_add(usize::try_from(stored_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"compressed input exceeds address space"})?).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"inflation work overflow"})?;
                control.begin_stage(workload).map_err(PackRefusal::ValueRefusal)?;
                let maximum=control.maximum_bytes().min(usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX)).checked_sub(control.owned_bytes()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled inflater allowance exhausted"})?;
                let mut inflater=OwnedControlledInflater(semio_framework_replication_codec::DeflateRetainedCursor::try_new(raw_len,limits.max_segment_len,maximum)?);
                let mut stored_position=0;let mut page_position=0;let mut page_len=0;
                loop{
                    if let Some(bytes)=inflater.0.next_allocation_bytes(){check_controlled_allocation(control,limits,bytes)?;control.charge(bytes).map_err(PackRefusal::ValueRefusal)?;inflater.0.reserve_allocation(bytes).map_err(|error|PackRefusal::RetainedAllocation{kind:error.kind,allocated_bytes:error.allocated_bytes,what:"controlled inflater allocation",offset,detail:error.reason})?;continue;}
                    match inflater.0.grant(stored_position==stored_len)?{
                        semio_framework_replication_codec::DeflateRetainedStep::Byte(byte)=>{output.push(byte);control.step().map_err(PackRefusal::ValueRefusal)?;},
                        semio_framework_replication_codec::DeflateRetainedStep::NeedInput=>{
                            if inflater.0.next_allocation_bytes().is_some(){continue;}
                            if !inflater.0.can_admit(){control.checkpoint().map_err(PackRefusal::ValueRefusal)?;continue;}
                            if stored_position==stored_len{return Err((PackRefusal::Truncated(offset+stored_position)).into());}
                            if page_position==page_len{page_len=usize::try_from((stored_len-stored_position).min(page.len() as u64)).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"controlled inflater page length"})?;source.read_exact_at(offset+stored_position,&mut page[..page_len]).await?;page_position=0;}
                            inflater.0.admit_byte(page[page_position]).map_err(|_|PackRefusal::Malformed{kind:ValueRefusalKind::InvariantViolated,what:"deflate",offset:offset+stored_position,detail:"inflater rejected available input".into()})?;page_position+=1;stored_position+=1;control.step().map_err(PackRefusal::ValueRefusal)?;
                        },
                        semio_framework_replication_codec::DeflateRetainedStep::Complete=>{if stored_position!=stored_len{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"deflate",offset:offset+stored_position,detail:"trailing compressed input".into()}).into());}break;},
                    }
                }
            }
            #[cfg(not(feature="deflate"))]
            return Err((PackRefusal::UnsupportedCodec(1)).into());
        },
        other=>return Err((PackRefusal::UnsupportedCodec(other)).into()),
    }
    if output.len()-start!=expected{return Err((PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"payload",offset,detail:"decoded payload length mismatch".into()}).into());}
    Ok(())
}

pub(super) async fn append_controlled_segment<S:PackSource>(source:&S,offset:u64,output:&mut Vec<u8>,maximum_output:Option<usize>,limits:&PackLimits,verification:VerificationLevel,control:&mut NativeDecodeControl<'_>)->Result<(u8,u64),S::Error>{
    control.checkpoint().map_err(PackRefusal::ValueRefusal)?;
    let total=source.len().await;let kind=read_u8_at(source,offset).await?;let flags=read_u8_at(source,offset.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment offset overflow"})?).await?;
    let (stored_len,n1)=read_varint_u64_at(source,offset.checked_add(2).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment offset overflow"})?).await?;
    let mut payload_offset=offset.checked_add(2+n1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment header overflow"})?;
    let raw_len=if flags&1!=0{let(length,n2)=read_varint_u64_at(source,payload_offset).await?;payload_offset=payload_offset.checked_add(n2).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment header overflow"})?;length}else{stored_len};
    if stored_len>limits.max_segment_len||raw_len>limits.max_segment_len{return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"controlled segment exceeds max_segment_len"}).into());}
    let crc_offset=payload_offset.checked_add(stored_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment range overflow"})?;let end=crc_offset.checked_add(4).filter(|end|*end<=total).ok_or(PackRefusal::Truncated(crc_offset))?;
    let length=usize::try_from(raw_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment exceeds address space"})?;
    if let Some(maximum)=maximum_output{if output.len().checked_add(length).filter(|length|*length<=maximum).is_none(){return Err((PackRefusal::LimitExceeded{kind:ValueRefusalKind::InvalidValue,limit:"segment exceeds declared output allocation"}).into());}}
    if length>output.capacity()-output.len(){
        check_controlled_allocation(control,limits,output.len().checked_add(length).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"segment allocation overflow"})?)?;
        control.charge(output.len()+length).map_err(PackRefusal::ValueRefusal)?;check_allocation_layout::<u8>(output.len()+length,"controlled segment allocation failed")?;output.try_reserve_exact(length).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::AllocationFailed,limit:"controlled segment allocation failed"})?;
    }
    if verification.checks_crc(){let mut bytes=[0;4];source.read_exact_at(crc_offset,&mut bytes).await?;if controlled_crc(source,offset,crc_offset-offset,control).await?!=u32::from_le_bytes(bytes){return Err((PackRefusal::ChecksumMismatch{segment:"segment",offset:crc_offset}).into());}}
    let codec=if flags&1!=0{CodecId((flags>>1)&7)}else{CodecId(0)};
    append_controlled_payload(source,payload_offset,stored_len,raw_len,codec,output,limits,control).await?;
    Ok((kind,end-offset))
}
