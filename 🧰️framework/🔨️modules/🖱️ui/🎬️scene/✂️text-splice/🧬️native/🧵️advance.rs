use super::*;
use super::math::*;
fn hash(mut state:u64,text:&[u8])->u64{for byte in text{state^=u64::from(*byte);state=state.wrapping_mul(0x100000001b3)}state}
fn output_mut(result:&mut OriginalTextSpliceComposition,field:usize)->&mut String{let OriginalTextSpliceComposition::Composed(s)=result else{unreachable!()};match field{0=>&mut s.deleted,1=>&mut s.insert,2=>&mut s.before,3=>&mut s.after,_=>unreachable!()}}
impl OriginalTextSpliceComposer{
 pub(super) fn step(&mut self,previous:TextSpliceView<'_>,next:TextSpliceView<'_>)->Result<(),ValueError>{
  let mut c=self.cursor;let owners=self.owners.original_mut().ok_or_else(invalid)?;let input=parts(previous,next);
  match c.phase{
   0|2|12=>{
    if c.byte==input[c.field].len(){
     if c.phase==2&&owners.scalars.get(c.field).len()!=c.lengths[c.field]{return Err(invalid())}
     c.field+=1;c.byte=0;c.index=0;
     if c.field==8{c.field=0;match c.phase{0=>{c.fingerprint=c.hash;c.hash=HASH;c.phase=1},2=>{if c.hash!=c.fingerprint{return Err(invalid())}c.phase=3;c.index=0},_=>{if c.hash!=c.fingerprint{return Err(invalid())}c.phase=14}}}
    }else{
     let scalar=input[c.field].get(c.byte..).and_then(|s|s.chars().next()).ok_or_else(invalid)?;let length=scalar.len_utf8();
     if c.phase==0{c.lengths[c.field]=c.lengths[c.field].checked_add(1).ok_or_else(extent)?;}
     else if c.phase==2{if owners.scalars.get(c.field).len()>=c.lengths[c.field]{return Err(invalid())}owners.scalars.get_mut(c.field).push(scalar);}
     else{if owners.scalars.get(c.field).get(c.index)!=Some(&scalar){return Err(invalid())}c.index+=1;}
     c.hash=hash(c.hash,&input[c.field].as_bytes()[c.byte..c.byte+length]);c.byte+=length;
    }
   },
   1=>{owners.scalars.get_mut(c.field).try_reserve_exact(c.lengths[c.field]).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original scalar buffer allocation refused"))?;c.field+=1;if c.field==8{c.field=0;c.byte=0;c.hash=HASH;c.phase=2}},
   3=>{
    let origin=i128::from(self.fence.starts[0])-c.lengths[0]as i128;let at=i128::from(self.fence.starts[1])-origin;let length=window_len(&c)?;
    if at<0||at>usize::MAX as i128||at as usize>length||c.lengths[5]>length-at as usize{owners.result=Some(OriginalTextSpliceComposition::Disjoint);c.phase=12;c.field=0;c.byte=0;c.index=0;c.hash=HASH;}
    else{c.at=at as usize;c.seen_before=c.lengths[4].min(c.at);c.seen_after=c.lengths[7].min(length-c.at-c.lengths[5]);c.index=0;c.field=0;c.phase=4}
   },
   4=>{
    let length=match c.field{0=>c.lengths[5],1=>c.seen_before,_=>c.seen_after};
    if c.index==length{c.index=0;c.field+=1;if c.field==3{c.phase=5;c.field=0;c.limits=[c.lengths[0],c.lengths[0],c.at,c.at]}}
    else{
     let(i,expected)=match c.field{0=>(c.at+c.index,owners.scalars.f[c.index]),1=>(c.at-c.seen_before+c.index,owners.scalars.e[c.lengths[4]-c.seen_before+c.index]),_=>(c.at+c.lengths[5]+c.index,owners.scalars.h[c.index])};
     if window(&owners.scalars,&c,i)!=expected{owners.result=Some(OriginalTextSpliceComposition::Disjoint);c.phase=12;c.field=0;c.byte=0;c.index=0;c.hash=HASH;}else{c.index+=1}
    }
   },
   5=>{
    let pair=c.field/2;let left=c.field%2==0;let pos=c.limits[c.field];
    if shift(&owners.scalars,&c,pair,pos,left)?{c.limits[c.field]=if left{pos-1}else{pos+1}}
    else{c.field+=1;if c.field==4{c.phase=6}}
   },
   6=>{
    let lo=c.limits[0].max(c.limits[2].saturating_sub(c.lengths[2]));let hi=c.limits[1].min(c.limits[3].checked_add(c.lengths[5]).ok_or_else(extent)?);
    if lo>hi{owners.result=Some(OriginalTextSpliceComposition::Disjoint);c.phase=12;c.field=0;c.byte=0;c.index=0;c.hash=HASH;}
    else{
     c.run=c.lengths[0].clamp(lo,hi);let low=c.limits[2].max(c.run.saturating_sub(c.lengths[5]));let high=c.limits[3].min(c.run.checked_add(c.lengths[2]).ok_or_else(extent)?);if low>high{return Err(invalid())}c.next=c.at.clamp(low,high);
     c.lo=c.run.min(c.next);c.hi=(c.run+c.lengths[2]).max(c.next+c.lengths[5]);c.original_hi=c.hi-c.lengths[2]+c.lengths[1];c.typed_hi=c.hi-c.lengths[5]+c.lengths[6];
     let length=window_len(&c)?;if length-c.lengths[2]+c.lengths[1]!=length-c.lengths[5]+c.lengths[6]{c.phase=8}else{c.phase=7;c.scan=0}
    }
   },
   7=>{
    let length=window_len(&c)?-c.lengths[2]+c.lengths[1];
    if c.scan==length{owners.result=Some(OriginalTextSpliceComposition::Cancelled);c.phase=12;c.field=0;c.byte=0;c.index=0;c.hash=HASH;}
    else if original(&owners.scalars,&c,c.scan)!=typed(&owners.scalars,&c,c.scan){c.phase=8}else{c.scan+=1}
   },
   8=>{let start=(i128::from(self.fence.starts[0])-c.lengths[0]as i128+c.lo as i128)as u32;owners.result=Some(OriginalTextSpliceComposition::Composed(OriginalTextSplice{start,deleted:String::new(),insert:String::new(),before:String::new(),after:String::new()}));c.phase=9;c.field=0;c.index=0},
   9=>{
    if c.index==output_length(&c,&self.fence,c.field)?{c.index=0;c.field+=1;if c.field==4{c.field=0;c.phase=10}}
    else{let scalar=output_char(&owners.scalars,&c,&self.fence,c.field,c.index)?;c.output_bytes[c.field]=c.output_bytes[c.field].checked_add(scalar.len_utf8()).ok_or_else(extent)?;c.index+=1}
   },
   10=>{output_mut(owners.result.as_mut().ok_or_else(invalid)?,c.field).try_reserve_exact(c.output_bytes[c.field]).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original splice output allocation refused"))?;c.field+=1;if c.field==4{c.field=0;c.index=0;c.phase=11}},
   11=>{
    if c.index==output_length(&c,&self.fence,c.field)?{c.index=0;c.field+=1;if c.field==4{c.field=0;c.byte=0;c.hash=HASH;c.phase=12}}
    else{let scalar=output_char(&owners.scalars,&c,&self.fence,c.field,c.index)?;output_mut(owners.result.as_mut().ok_or_else(invalid)?,c.field).push(scalar);c.index+=1}
   },
   _=>return Err(invalid())
  }
  self.cursor=c;Ok(())
 }
}
