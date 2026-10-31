use super::*;
pub(super) fn window_len(c:&Cursor)->Result<usize,ValueError>{c.lengths[0].checked_add(c.lengths[2]).and_then(|n|n.checked_add(c.lengths[3])).ok_or_else(extent)}
pub(super) fn window(s:&Scalars,c:&Cursor,index:usize)->char{if index<c.lengths[0]{s.a[index]}else if index<c.lengths[0]+c.lengths[2]{s.c[index-c.lengths[0]]}else{s.d[index-c.lengths[0]-c.lengths[2]]}}
fn rotated(side:&[char],position:usize,base:usize,index:usize)->char{side[((position as i128-base as i128+index as i128).rem_euclid(side.len()as i128))as usize]}
pub(super) fn original(s:&Scalars,c:&Cursor,index:usize)->char{if index<c.run{window(s,c,index)}else if index<c.run+c.lengths[1]{rotated(&s.b,c.run,c.lengths[0],index-c.run)}else{window(s,c,index-c.lengths[1]+c.lengths[2])}}
pub(super) fn typed(s:&Scalars,c:&Cursor,index:usize)->char{if index<c.next{window(s,c,index)}else if index<c.next+c.lengths[6]{rotated(&s.g,c.next,c.at,index-c.next)}else{window(s,c,index-c.lengths[6]+c.lengths[5])}}
pub(super) fn shift(s:&Scalars,c:&Cursor,pair:usize,position:usize,left:bool)->Result<bool,ValueError>{
 let(base,inside,outside)=if pair==0{(c.lengths[0],&s.c,&s.b)}else{(c.at,&s.f,&s.g)};
 if inside.is_empty()&&outside.is_empty(){return Ok(false)}let length=window_len(c)?;
 let scalar=if left{if position==0{return Ok(false)}window(s,c,position-1)}else{let i=position.checked_add(inside.len()).ok_or_else(extent)?;if i>=length{return Ok(false)}window(s,c,i)};
 for side in [inside,outside]{if !side.is_empty()&&rotated(side,position,base,if left{side.len()-1}else{0})!=scalar{return Ok(false)}}Ok(true)
}
pub(super) fn output_length(c:&Cursor,fence:&Fence,field:usize)->Result<usize,ValueError>{match field{
 0=>Ok(c.original_hi-c.lo),1=>Ok(c.typed_hi-c.lo),2=>Ok(fence.context.min((c.lengths[4]-c.seen_before).checked_add(c.lo).ok_or_else(extent)?)),3=>Ok(fence.context.min((window_len(c)?-c.hi).checked_add(c.lengths[7]-c.seen_after).ok_or_else(extent)?)),_=>Err(invalid())}}
pub(super) fn output_char(s:&Scalars,c:&Cursor,fence:&Fence,field:usize,index:usize)->Result<char,ValueError>{Ok(match field{
 0=>original(s,c,c.lo+index),1=>typed(s,c,c.lo+index),
 2=>{let extra=c.lengths[4]-c.seen_before;let total=extra.checked_add(c.lo).ok_or_else(extent)?;let at=total-output_length(c,fence,2)?+index;if at<extra{s.e[at]}else{window(s,c,at-extra)}},
 3=>{let suffix=window_len(c)?-c.hi;if index<suffix{window(s,c,c.hi+index)}else{s.h[c.seen_after+index-suffix]}},
 _=>return Err(invalid())})}
