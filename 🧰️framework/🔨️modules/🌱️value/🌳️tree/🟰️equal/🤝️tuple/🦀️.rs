//! 🤝️ Heterogeneous original pairs use the same paid record frontier and typed scalar pages.
use super::*;
struct First<A:RetainedEquality,B:RetainedEquality>{cursor:A::Equality,marker:std::marker::PhantomData<fn()->B>}
struct Second<A:RetainedEquality,B:RetainedEquality>{cursor:B::Equality,marker:std::marker::PhantomData<fn()->A>}
impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityBirth for First<A,B>{type Output=Self;fn work()->Result<u64,ValueError>{A::CURSOR_BIRTH_WORK.checked_add(2).ok_or_else(||invalid("tuple birth work overflow"))}fn birth()->Self{Self{cursor:A::equality_cursor(),marker:Default::default()}}}
impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityBirth for Second<A,B>{type Output=Self;fn work()->Result<u64,ValueError>{B::CURSOR_BIRTH_WORK.checked_add(2).ok_or_else(||invalid("tuple birth work overflow"))}fn birth()->Self{Self{cursor:B::equality_cursor(),marker:Default::default()}}}
macro_rules! field{($owner:ident,$ty:ident,$index:tt)=>{impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityCursor<(A,B)> for $owner<A,B>{
 fn demand(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}self.cursor.demand(body,work-2)}
 fn demand_admitted(&self,body:usize,work:u64)->Result<RetirementDemand,ValueError>{if work<2{return Ok(Default::default())}self.cursor.demand_admitted(body,work-2)}
 fn work_demand(&self,body:usize,work:u64)->Result<u64,ValueError>{if work<2{return Ok(0)}let child=self.cursor.work_demand(body,work-2)?;if child==0{return Ok(0)}child.checked_add(2).ok_or_else(||invalid("tuple projection work overflow"))}
 fn advance(&mut self,l:&(A,B),r:&(A,B),body:usize,work:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();if effect.actual().is_some(){return crate::RetainedWorkOutcome{result:Err(invalid("equality advance needs an empty allocation marker").into()),work:0}}let selected=match self.work_demand(body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let d=match self.demand_admitted(body,work){Ok(value)=>value,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};if !retained_clone_grant_admits(g,d){return crate::RetainedWorkOutcome{result:Ok(RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}let turn=self.cursor.advance(&l.$index,&r.$index,body,work-2,effect);crate::retained_equality::settle_work(effect,crate::RetainedWorkOutcome{result:turn.result,work:turn.work+2})}
 fn begin_close(&mut self){<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::begin_close(&mut self.cursor)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::close_demand(&self.cursor,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::close_work_demand(&self.cursor,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::close_step(&mut self.cursor,effect)}
 fn terminal_is_empty(&self)->bool{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::terminal_is_empty(&self.cursor)}
}};}
field!(First,A,0);field!(Second,B,1);
impl<A:RetainedEquality,B:RetainedEquality> RetainedEquality for(A,B){
 type Equality=RetainedRecordEquality<Self>;
 const CURSOR_BIRTH_WORK:u64=7;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 const INLINE_PAIR_WORK:Option<u64>=None;
 fn equality_cursor()->Self::Equality{RetainedRecordEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityFields for(A,B){
 const SHAPE_WORK:u64=1;const FIELD_METADATA_WORK:u64=1;const FIELD_PROJECTION_WORK:u64=1;
 fn equality_field_birth_work(_:&Self,index:usize)->Result<Option<u64>,ValueError>{match index{0=>equality_box_work::<First<A,B>>().map(Some),1=>equality_box_work::<Second<A,B>>().map(Some),_=>Ok(None)}}
 fn equality_shape(_:&Self,_:&Self)->Option<usize>{Some(2)}
 fn equality_field_inline_bytes(_:&Self,index:usize)->Option<usize>{match index{0=>A::INLINE_PAIR_BYTES,1=>B::INLINE_PAIR_BYTES,_=>None}}
 fn equality_field_inline_work(_:&Self,index:usize)->Option<u64>{match index{0=>A::INLINE_PAIR_WORK,1=>B::INLINE_PAIR_WORK,_=>None}}
 fn equality_field_inline(l:&Self,r:&Self,index:usize)->Option<bool>{match index{0=>A::inline_equal(&l.0,&r.0),1=>B::inline_equal(&l.1,&r.1),_=>None}}
 fn equality_field_bytes(_:&Self,index:usize)->usize{match index{0=>size_of::<First<A,B>>(),1=>size_of::<Second<A,B>>(),_=>0}}
 fn equality_field(_:&Self,index:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<Self>>,RetainedCloneProgress)>>{match index{0=>{let turn=equality_box::<First<A,B>>(work,g,actual);crate::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<Self>>,p))) }},1=>{let turn=equality_box::<Second<A,B>>(work,g,actual);crate::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<Self>>,p))) }},_=>crate::RetainedWorkOutcome{result:Err(invalid("typed tuple field index is outside original pair")),work:0}}}
}
