//! 🟰️ Typed equality visits all native fields without consulting wire annotations.
use proc_macro2::TokenStream;
use quote::{format_ident,quote};
use syn::{Data,DeriveInput,Fields,Member,parse_quote};
pub fn expand(input:&DeriveInput)->syn::Result<TokenStream>{
 let name=&input.ident;let visibility=&input.vis;let mut generics=input.generics.clone();
 for parameter in generics.type_params_mut(){parameter.bounds.push(parse_quote!(::semio_framework_value::RetainedEquality));}
 let (ig,tg,wc)=generics.split_for_impl();let root=quote!(#name #tg);let v=quote!(::semio_framework_value);let mut declarations=Vec::new();let mut shape=Vec::new();let mut sizes=Vec::new();let mut births=Vec::new();let mut inline_sizes=Vec::new();let mut inline_comparisons=Vec::new();
 let variants:Vec<(Option<&syn::Ident>,&Fields)>=match &input.data{Data::Struct(data)=>vec![(None,&data.fields)],Data::Enum(data)=>data.variants.iter().map(|variant|(Some(&variant.ident),&variant.fields)).collect(),Data::Union(data)=>return Err(syn::Error::new_spanned(data.union_token,"typed equality does not support unions"))};
 for (variant_index,(variant,fields)) in variants.iter().enumerate(){
  let count=fields.len();let prefix=variant.map(|variant|quote!(#name::#variant)).unwrap_or(quote!(#name));
  let shape_pattern=match fields{Fields::Named(_)=>quote!(#prefix{..}),Fields::Unnamed(_)=>quote!(#prefix(..)),Fields::Unit=>quote!(#prefix)};
  shape.push(quote!((#shape_pattern,#shape_pattern)=>Some(#count)));
  let mut field_sizes=Vec::new();let mut field_births=Vec::new();let mut field_inline_sizes=Vec::new();let mut field_inline_comparisons=Vec::new();
  for (index,field) in fields.iter().enumerate(){
   let ty=&field.ty;let wrapper=format_ident!("{}RetainedEqualityField{}x{}",name,variant_index,index);
   let (left,right)=if variant.is_none(){let member=field.ident.clone().map(Member::Named).unwrap_or(Member::Unnamed(index.into()));(quote!(&left.#member),quote!(&right.#member))}else{let bindings:Vec<_>=(0..fields.len()).map(|i|format_ident!("field{}",i)).collect();let selected=&bindings[index];let pattern=match fields{Fields::Named(_)=>{let names=fields.iter().map(|f|f.ident.as_ref().unwrap());quote!(#prefix{#(#names:#bindings),*})},Fields::Unnamed(_)=>quote!(#prefix(#(#bindings),*)),Fields::Unit=>unreachable!()};let lens=quote!(match source{#pattern=>#selected,_=>return Err(#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed equality variant source changed"))});(quote!({let source=left;#lens}),quote!({let source=right;#lens}))};
   declarations.push(quote!{
    #visibility struct #wrapper #ig #wc{cursor:<#ty as #v::RetainedEquality>::Equality,marker:std::marker::PhantomData<fn()->#root>}
    impl #ig #v::RetainedEqualityCursor<#root> for #wrapper #tg #wc{
     fn demand(&self,left:&#root,right:&#root,body:usize)->Result<#v::RetirementDemand,#v::ValueError>{<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::demand(&self.cursor,#left,#right,body)}
     fn advance(&mut self,left:&#root,right:&#root,grant:#v::RetainedCloneGrant)->Result<#v::RetainedEqualityStep,#v::ValueError>{<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::advance(&mut self.cursor,#left,#right,grant)}
     fn begin_close(&mut self){<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::begin_close(&mut self.cursor)}
     fn close_demand(&self,body:usize)->Result<#v::RetirementDemand,#v::ValueError>{<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::close_demand(&self.cursor,body)}
     fn close_step(&mut self,grant:#v::RetainedCloneGrant)->Result<#v::RetainedCloneStep,#v::ValueError>{<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::close_step(&mut self.cursor,grant)}
     fn terminal_is_empty(&self)->bool{<<#ty as #v::RetainedEquality>::Equality as #v::RetainedEqualityCursor<#ty>>::terminal_is_empty(&self.cursor)}
    }
   });
   field_inline_sizes.push(quote!(#index=><#ty as #v::RetainedEquality>::INLINE_PAIR_BYTES));
   let inline_left=left.clone();let inline_right=right.clone();
   field_inline_comparisons.push(quote!(#index=>{let compared=(||->Result<Option<bool>,#v::ValueError>{Ok(<#ty as #v::RetainedEquality>::inline_equal(#inline_left,#inline_right))})();compared.ok().flatten()}));
   field_sizes.push(quote!(#index=>std::mem::size_of::<#wrapper #tg>()));
   field_births.push(quote!(#index=>Ok(#v::retained_equality::equality_box(#wrapper{cursor:<#ty as #v::RetainedEquality>::equality_cursor(),marker:Default::default()})?)));
  }
  inline_sizes.push(quote!(#shape_pattern=>match index{#(#field_inline_sizes,)*_=>None}));
  inline_comparisons.push(quote!((#shape_pattern,#shape_pattern)=>match index{#(#field_inline_comparisons,)*_=>None}));
  sizes.push(quote!(#shape_pattern=>match index{#(#field_sizes,)*_=>0}));
  births.push(quote!(#shape_pattern=>match index{#(#field_births,)*_=>Err(#v::ValueError::literal(#v::ValueRefusalKind::InvariantViolated,"typed equality field index is outside original shape"))}));
 }
 Ok(quote!{
  #(#declarations)*
  impl #ig #v::RetainedEquality for #root #wc{
   type Equality=#v::RetainedRecordEquality<Self>;
   const INLINE_PAIR_BYTES:Option<usize>=None;
   fn equality_cursor()->Self::Equality{#v::RetainedRecordEquality::new()}
   fn inline_equal(_:&Self,_:&Self)->Option<bool>{None}
  }
  impl #ig #v::RetainedEqualityFields for #root #wc{
   fn equality_shape(left:&Self,right:&Self)->Option<usize>{match(left,right){#(#shape,)*_=>None}}
   fn equality_field_inline_bytes(left:&Self,index:usize)->Option<usize>{match left{#(#inline_sizes),*}}
   fn equality_field_inline(left:&Self,right:&Self,index:usize)->Option<bool>{match(left,right){#(#inline_comparisons,)*_=>None}}
   fn equality_field_bytes(left:&Self,index:usize)->usize{match left{#(#sizes),*}}
   fn equality_field(left:&Self,index:usize)->Result<Box<dyn #v::RetainedEqualityCursor<Self>>,#v::ValueError>{match left{#(#births),*}}
  }
 })
}
