//! 🟰️ Typed equality visits all native fields without consulting wire annotations.
use proc_macro2::TokenStream;
use quote::{format_ident,quote};
use syn::{Data,DeriveInput,Fields,Member,parse_quote};
pub fn expand(input:&DeriveInput)->syn::Result<TokenStream>{
 let name=&input.ident;let visibility=&input.vis;let mut generics=input.generics.clone();
 let equality=super::component::declared_equality_crate(input)?;
 for parameter in generics.type_params_mut(){parameter.bounds.push(parse_quote!(#equality::retained_equality::RetainedEquality));}
 let (ig,tg,wc)=generics.split_for_impl();let root=quote!(#name #tg);let v=super::component::declared_value_crate(input)?;let eq=quote!(#equality::retained_equality);let v=quote!(#v);let mut declarations=Vec::new();let mut shape=Vec::new();let mut sizes=Vec::new();let mut births=Vec::new();let mut inline_sizes=Vec::new();let mut inline_works=Vec::new();let mut inline_comparisons=Vec::new();let mut birth_works=Vec::new();
 let variants:Vec<(Option<&syn::Ident>,&Fields)>=match &input.data{Data::Struct(data)=>vec![(None,&data.fields)],Data::Enum(data)=>data.variants.iter().map(|variant|(Some(&variant.ident),&variant.fields)).collect(),Data::Union(data)=>return Err(syn::Error::new_spanned(data.union_token,"typed equality does not support unions"))};
 for (variant_index,(variant,fields)) in variants.iter().enumerate(){
  let count=fields.len();let prefix=variant.map(|variant|quote!(#name::#variant)).unwrap_or(quote!(#name));
  let shape_pattern=match fields{Fields::Named(_)=>quote!(#prefix{..}),Fields::Unnamed(_)=>quote!(#prefix(..)),Fields::Unit=>quote!(#prefix)};
  shape.push(quote!((#shape_pattern,#shape_pattern)=>Some(#count)));
  let mut field_sizes=Vec::new();let mut field_births=Vec::new();let mut field_inline_sizes=Vec::new();let mut field_inline_works=Vec::new();let mut field_inline_comparisons=Vec::new();let mut field_birth_works=Vec::new();
  for (index,field) in fields.iter().enumerate(){
   let ty=&field.ty;let wrapper=format_ident!("{}RetainedEqualityField{}x{}",name,variant_index,index);
   let (left,right)=if variant.is_none(){let member=field.ident.clone().map(Member::Named).unwrap_or(Member::Unnamed(index.into()));(quote!(&left.#member),quote!(&right.#member))}else{let bindings:Vec<_>=(0..fields.len()).map(|i|format_ident!("field{}",i)).collect();let selected=&bindings[index];let pattern=match fields{Fields::Named(_)=>{let names=fields.iter().map(|f|f.ident.as_ref().unwrap());quote!(#prefix{#(#names:#bindings),*})},Fields::Unnamed(_)=>quote!(#prefix(#(#bindings),*)),Fields::Unit=>unreachable!()};let lens=quote!(match source{#pattern=>#selected,_=>return Err(#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed equality variant source changed"))});(quote!({let source=left;#lens}),quote!({let source=right;#lens}))};
   declarations.push(quote!{
    #visibility struct #wrapper #ig #wc{cursor:<#ty as #eq::RetainedEquality>::Equality,marker:std::marker::PhantomData<fn()->#root>}
    impl #ig #eq::RetainedEqualityBirth for #wrapper #tg #wc{type Output=Self;fn work()->Result<u64,#v::ValueError>{<#ty as #eq::RetainedEquality>::CURSOR_BIRTH_WORK.checked_add(2).ok_or_else(||#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed field birth work overflow"))}fn birth()->Self{Self{cursor:<#ty as #eq::RetainedEquality>::equality_cursor(),marker:Default::default()}}}
    impl #ig #eq::RetainedEqualityCursor<#root> for #wrapper #tg #wc{
     fn demand(&self,body:usize,work:u64)->Result<#v::RetirementDemand,#v::ValueError>{if work<2{return Ok(Default::default())}<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::demand(&self.cursor,body,work-2)}
     fn work_demand(&self,body:usize,work:u64)->Result<u64,#v::ValueError>{if work<2{return Ok(0)}let child=<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::work_demand(&self.cursor,body,work-2)?;if child==0{return Ok(0)}child.checked_add(2).ok_or_else(||#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed equality projection work overflow"))}
     fn advance(&mut self,left:&#root,right:&#root,body:usize,work:u64,grant:#v::RetainedCloneGrant,actual:&mut Option<usize>)->#v::RetainedWorkOutcome<#eq::RetainedEqualityStep,#v::OriginalRetirementFailure>{
      if actual.is_some(){return #v::RetainedWorkOutcome{result:Err(#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"equality advance needs an empty allocation marker").into()),work:0}}
      let selected=match self.work_demand(body,work){Ok(value)=>value,Err(error)=>return #v::RetainedWorkOutcome{result:Err(error.into()),work:0}};if selected==0{return #v::RetainedWorkOutcome{result:Ok(#eq::RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}
      let demand=match self.demand(body,work){Ok(value)=>value,Err(error)=>return #v::RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return #v::RetainedWorkOutcome{result:Ok(#eq::RetainedEqualityStep{equal:None,progress:Default::default()}),work:0}}
      let projected_left=(||->Result<_,#v::ValueError>{Ok(#left)})();let left=match projected_left{Ok(value)=>value,Err(error)=>return #v::RetainedWorkOutcome{result:Err(error.into()),work:1}};
      let projected_right=(||->Result<_,#v::ValueError>{Ok(#right)})();let right=match projected_right{Ok(value)=>value,Err(error)=>return #v::RetainedWorkOutcome{result:Err(error.into()),work:2}};
      let turn=<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::advance(&mut self.cursor,left,right,body,work-2,grant,actual);#v::RetainedWorkOutcome{result:turn.result,work:turn.work+2}
     }
     fn begin_close(&mut self){<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::begin_close(&mut self.cursor)}
     fn close_demand(&self,body:usize)->Result<#v::RetirementDemand,#v::ValueError>{<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::close_demand(&self.cursor,body)}
     fn close_work_demand(&self,body:usize)->Result<u64,#v::ValueError>{<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::close_work_demand(&self.cursor,body)}
     fn close_step(&mut self,grant:#v::RetainedCloneGrant)->#v::RetainedWorkOutcome{<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::close_step(&mut self.cursor,grant)}
     fn terminal_is_empty(&self)->bool{<<#ty as #eq::RetainedEquality>::Equality as #eq::RetainedEqualityCursor<#ty>>::terminal_is_empty(&self.cursor)}
    }
   });
   field_inline_sizes.push(quote!(#index=><#ty as #eq::RetainedEquality>::INLINE_PAIR_BYTES));
   field_inline_works.push(quote!(#index=><#ty as #eq::RetainedEquality>::INLINE_PAIR_WORK));
   let inline_left=left.clone();let inline_right=right.clone();
   field_inline_comparisons.push(quote!(#index=>{let compared=(||->Result<Option<bool>,#v::ValueError>{Ok(<#ty as #eq::RetainedEquality>::inline_equal(#inline_left,#inline_right))})();compared.ok().flatten()}));
   field_sizes.push(quote!(#index=>std::mem::size_of::<#wrapper #tg>()));
   field_birth_works.push(quote!(#index=>#eq::equality_box_work::<#wrapper #tg>().map(Some)));
   field_births.push(quote!(#index=>{let turn=#eq::equality_box::<#wrapper #tg>(work,grant,actual);#v::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(owner,p)|(owner as Box<dyn #eq::RetainedEqualityCursor<Self>>,p)))}}));
  }
  inline_sizes.push(quote!(#shape_pattern=>match index{#(#field_inline_sizes,)*_=>None}));
  inline_works.push(quote!(#shape_pattern=>match index{#(#field_inline_works,)*_=>None}));
  inline_comparisons.push(quote!((#shape_pattern,#shape_pattern)=>match index{#(#field_inline_comparisons,)*_=>None}));
  sizes.push(quote!(#shape_pattern=>match index{#(#field_sizes,)*_=>0}));
  birth_works.push(quote!(#shape_pattern=>match index{#(#field_birth_works,)*_=>Ok(None)}));
  births.push(quote!(#shape_pattern=>match index{#(#field_births,)*_=>#v::RetainedWorkOutcome{result:Err(#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed equality field index is outside original shape")),work:0}}));
 }
 Ok(quote!{
  #(#declarations)*
  impl #ig #eq::RetainedEquality for #root #wc{
   type Equality=#eq::RetainedRecordEquality<Self>;
   const CURSOR_BIRTH_WORK:u64=7;
   const INLINE_PAIR_BYTES:Option<usize>=None;
   const INLINE_PAIR_WORK:Option<u64>=None;
   fn equality_cursor()->Self::Equality{#eq::RetainedRecordEquality::new()}
   fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
  }
  impl #ig #eq::RetainedEqualityFields for #root #wc{
   const SHAPE_WORK:u64=1;const FIELD_METADATA_WORK:u64=1;const FIELD_PROJECTION_WORK:u64=1;
   fn equality_field_birth_work(left:&Self,index:usize)->Result<Option<u64>,#v::ValueError>{match left{#(#birth_works),*}}
   fn equality_shape(left:&Self,right:&Self)->Option<usize>{match(left,right){#(#shape,)*_=>None}}
   fn equality_field_inline_bytes(left:&Self,index:usize)->Option<usize>{match left{#(#inline_sizes),*}}
   fn equality_field_inline_work(left:&Self,index:usize)->Option<u64>{match left{#(#inline_works),*}}
   fn equality_field_inline(left:&Self,right:&Self,index:usize)->Option<bool>{match(left,right){#(#inline_comparisons,)*_=>None}}
   fn equality_field_bytes(left:&Self,index:usize)->usize{match left{#(#sizes),*}}
   fn equality_field(left:&Self,index:usize,work:u64,grant:#v::RetainedCloneGrant,actual:&mut Option<usize>)->#v::RetainedWorkOutcome<Option<(Box<dyn #eq::RetainedEqualityCursor<Self>>,#v::RetainedCloneProgress)>>{match left{#(#births),*}}
  }
 })
}
