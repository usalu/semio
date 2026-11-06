//! 🤝️ Heterogeneous original pairs use the same paid record frontier and typed scalar pages.
use super::*;
struct First<A:RetainedEquality,B:RetainedEquality>{cursor:A::Equality,marker:std::marker::PhantomData<fn()->B>}
struct Second<A:RetainedEquality,B:RetainedEquality>{cursor:B::Equality,marker:std::marker::PhantomData<fn()->A>}
macro_rules! field{($owner:ident,$ty:ident,$index:tt)=>{impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityCursor<(A,B)> for $owner<A,B>{
 fn demand(&self,l:&(A,B),r:&(A,B),body:usize)->Result<RetirementDemand,ValueError>{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::demand(&self.cursor,&l.$index,&r.$index,body)}
 fn advance(&mut self,l:&(A,B),r:&(A,B),g:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedEqualityStep>{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::advance(&mut self.cursor,&l.$index,&r.$index,g)}
 fn begin_close(&mut self){<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::begin_close(&mut self.cursor)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::close_demand(&self.cursor,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let g=effect.grant();<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::close_step(&mut self.cursor,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 fn terminal_is_empty(&self)->bool{<<$ty as RetainedEquality>::Equality as RetainedEqualityCursor<$ty>>::terminal_is_empty(&self.cursor)}
}};}
field!(First,A,0);field!(Second,B,1);
impl<A:RetainedEquality,B:RetainedEquality> RetainedEquality for(A,B){
 type Equality=RetainedRecordEquality<Self>;
 const INLINE_PAIR_BYTES:Option<usize>=None;
 fn equality_cursor()->Self::Equality{RetainedRecordEquality::new()}
 fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
}
impl<A:RetainedEquality,B:RetainedEquality> RetainedEqualityFields for(A,B){
 fn equality_shape(_:&Self,_:&Self)->Option<usize>{Some(2)}
 fn equality_field_inline_bytes(_:&Self,index:usize)->Option<usize>{match index{0=>A::INLINE_PAIR_BYTES,1=>B::INLINE_PAIR_BYTES,_=>None}}
 fn equality_field_inline(l:&Self,r:&Self,index:usize)->Option<bool>{match index{0=>A::inline_equal(&l.0,&r.0),1=>B::inline_equal(&l.1,&r.1),_=>None}}
 fn equality_field_bytes(_:&Self,index:usize)->usize{match index{0=>size_of::<First<A,B>>(),1=>size_of::<Second<A,B>>(),_=>0}}
 fn equality_field(_:&Self,index:usize,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(Box<dyn RetainedEqualityCursor<Self>>,RetainedCloneProgress)>>{match index{0=>{let turn=equality_box(First::<A,B>{cursor:A::equality_cursor(),marker:Default::default()},g);crate::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<Self>>,p))) }},1=>{let turn=equality_box(Second::<A,B>{cursor:B::equality_cursor(),marker:Default::default()},g);crate::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn RetainedEqualityCursor<Self>>,p))) }},_=>crate::RetainedWorkOutcome{result:Err(invalid("typed tuple field index is outside original pair")),work:0}}}
}
