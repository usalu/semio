//! 🧳️ Retained aggregate dispatch requires every leaf and admits exactly one typed child body.
use quote::{quote,format_ident};
use syn::{DeriveInput,Data,Fields,Type};

/// 🧬️ Produces a closed retained owner without deriving semantic decision from cold methods.
pub fn expand(input:&DeriveInput,snapshot:&Type)->syn::Result<proc_macro2::TokenStream>{
 let name=&input.ident;let preparation=format_ident!("__{}RetainedMutationPreparation",name);let state=format_ident!("__{}RetainedMutationState",name);
 let Data::Enum(data)=&input.data else{return Err(syn::Error::new_spanned(input,"retained mutation dispatch requires an enum"));};
 let generics=&input.generics;let(impl_generics,ty_generics,where_clause)=generics.split_for_impl();let aggregate=quote!(#name #ty_generics);
 let mut variants=Vec::new();let mut select_demand=Vec::new();let mut select=Vec::new();let mut demand=Vec::new();let mut advance=Vec::new();let mut outcome=Vec::new();let mut inverse_demand=Vec::new();let mut inverse=Vec::new();let mut take_inverse=Vec::new();let mut begin_close=Vec::new();let mut close_demand=Vec::new();let mut close_step=Vec::new();let mut terminal=Vec::new();
 for(index,variant)in data.variants.iter().enumerate(){
  let variant_name=&variant.ident;let Fields::Unnamed(fields)=&variant.fields else{return Err(syn::Error::new_spanned(variant,"retained mutation variants require a single leaf"));};
  if fields.unnamed.len()!=1{return Err(syn::Error::new_spanned(variant,"retained mutation variants require a single leaf"));}
  let leaf=&fields.unnamed[0].ty;
  let child=quote!(<#leaf as ::semio_framework_os_kernel::RetainedMutationLeaf<#snapshot,#aggregate>>::Preparation);
  let ports=quote!(<#child as ::semio_framework_os_kernel::RetainedMutationLeafPreparation<#snapshot,#aggregate,#leaf>>);
  let payload=quote!(mutation.project(#index,|operation|match operation{#name::#variant_name(payload)=>payload,_=>unreachable!("immutable retained leaf discriminator")}));
  variants.push(quote!(#variant_name(::std::boxed::Box<#child>)));
  select_demand.push(quote!(#name::#variant_name(_)=>::semio_framework_value::RetirementDemand{copy_bytes: ::core::mem::size_of::<#child>()+::core::mem::size_of::<#state #ty_generics>(),capacity_bytes: ::core::mem::size_of::<#child>(),release_bytes:0,depth:2}));
  select.push(quote!(#name::#variant_name(_)=>{*self.state=#state::#variant_name(::std::boxed::Box::new(#ports::new()));}));
  demand.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>{let mut demand=#ports::decision_demand(owner.as_ref(),#payload,original,maximum_body_bytes)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::DepthLimit,"retained leaf depth overflow"))?;Ok(demand)}));
  advance.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::decide(owner.as_mut(),#payload,original,body,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})));
  inverse_demand.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>{let mut demand=#ports::inverse_demand(owner.as_ref(),#payload,original,diff,maximum_body_bytes)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::DepthLimit,"retained inverse leaf depth overflow"))?;Ok(demand)}));
  inverse.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::inverse(owner.as_mut(),#payload,original,diff,body,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})));
  take_inverse.push(quote!(#state::#variant_name(owner)=>#ports::take_inverse(owner.as_mut())));
  outcome.push(quote!(#state::#variant_name(owner)=>#ports::take_decision(owner.as_mut())));
  begin_close.push(quote!(#state::#variant_name(owner)=>#ports::begin_close(owner.as_mut())));
  close_demand.push(quote!(#state::#variant_name(owner)=>{if #ports::terminal_is_empty(owner.as_ref()){Ok(::semio_framework_value::RetirementDemand{copy_bytes: ::core::mem::size_of::<#state #ty_generics>(),release_bytes: ::core::mem::size_of::<#child>(),depth:1,..Default::default()})}else{let mut demand=#ports::close_demand(owner.as_ref(),context,maximum_body_bytes)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::DepthLimit,"retained leaf close depth overflow"))?;Ok(demand)}}));
  close_step.push(quote!(#state::#variant_name(owner)=>{if #ports::terminal_is_empty(owner.as_ref()){drop(::core::mem::replace(&mut *self.state,#state::Unselected));Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}else{let step=#ports::close_step(owner.as_mut(),context,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress()))}}));
  terminal.push(quote!(#state::#variant_name(_)=>false));
 }
 Ok(quote!{
  enum #state #generics #where_clause{Unselected,#(#variants),*}
  /// 🧳️ Required typed leaf custody until separately paid body closure.
  pub struct #preparation #generics #where_clause{state: ::core::mem::ManuallyDrop<#state #ty_generics>,closing:bool}
  impl #impl_generics ::semio_framework_os_kernel::RetainedMutation<#snapshot> for #aggregate #where_clause{type Preparation=#preparation #ty_generics;}
  impl #impl_generics ::semio_framework_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate> for #preparation #ty_generics #where_clause{
   fn new()->Self{Self{state: ::core::mem::ManuallyDrop::new(#state::Unselected),closing:false}}
   fn decision_demand(&self,mutation: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#aggregate>,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#snapshot>,maximum_body_bytes:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{
    if self.closing{return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained aggregate already closing"));}
    if matches!(&*self.state,#state::Unselected){return Ok(match mutation.get(){#(#select_demand),*});}
    match(&*self.state,mutation.get()){#(#demand),*,_=>Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained aggregate leaf changed"))}
   }
   fn decide(&mut self,mutation: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#aggregate>,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#snapshot>,body:usize,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<::semio_framework_os_kernel::RetainedMutationAdvance,::semio_framework_value::ValueError>{
    let demand=<Self as ::semio_framework_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::decision_demand(self,mutation,original,body)?;
    if !::semio_framework_os_kernel::retained_mutation_demand_fits(demand,grant){return Ok(::semio_framework_os_kernel::RetainedMutationAdvance::Progress(Default::default()));}
    if matches!(&*self.state,#state::Unselected){match mutation.get(){#(#select),*};return Ok(::semio_framework_os_kernel::RetainedMutationAdvance::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}));}
    match(&mut *self.state,mutation.get()){#(#advance),*,_=>Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained aggregate leaf changed"))}
   }
   fn take_decision(&mut self)->::semio_framework_os_kernel::RetainedMutationDecision<<#aggregate as ::semio_framework_os_kernel::Mutation<#snapshot>>::Diff>{match &mut *self.state{#(#outcome),*,#state::Unselected=>panic!("retained aggregate has no admitted leaf outcome")}}
   fn inverse_demand(&self,mutation: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#aggregate>,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#snapshot>,diff: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,<#aggregate as ::semio_framework_os_kernel::Mutation<#snapshot>>::Diff>,maximum_body_bytes:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{
    if self.closing{return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained aggregate inverse already closing"));}
    match(&*self.state,mutation.get()){#(#inverse_demand),*,_=>Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained inverse requires the original decided leaf"))}
   }
   fn inverse(&mut self,mutation: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#aggregate>,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#snapshot>,diff: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,<#aggregate as ::semio_framework_os_kernel::Mutation<#snapshot>>::Diff>,body:usize,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<::semio_framework_os_kernel::RetainedMutationAdvance,::semio_framework_value::ValueError>{
    let demand=<Self as ::semio_framework_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::inverse_demand(self,mutation,original,diff,body)?;
    if !::semio_framework_os_kernel::retained_mutation_demand_fits(demand,grant){return Ok(::semio_framework_os_kernel::RetainedMutationAdvance::Progress(Default::default()));}
    match(&mut *self.state,mutation.get()){#(#inverse),*,_=>Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained inverse leaf changed"))}
   }
   fn take_inverse(&mut self)->Vec<#aggregate>{match &mut *self.state{#(#take_inverse),*,#state::Unselected=>panic!("retained aggregate has no prepared inverse")}}
   fn begin_close(&mut self){self.closing=true;match &mut *self.state{#(#begin_close),*,#state::Unselected=>{}}}
   fn close_demand(&self,context: ::semio_framework_os_kernel::RetainedMutationCloseContext<'_,#aggregate>,maximum_body_bytes:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{match &*self.state{#(#close_demand),*,#state::Unselected=>Ok(Default::default())}}
   fn close_step(&mut self,context: ::semio_framework_os_kernel::RetainedMutationCloseContext<'_,#aggregate>,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_value::ValueError>{
    if !self.closing{return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"retained aggregate closure not started"));}
    let demand=<Self as ::semio_framework_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::close_demand(self,context,grant.maximum_copy_bytes)?;
    if !::semio_framework_os_kernel::retained_mutation_demand_fits(demand,grant){return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));}
    match &mut *self.state{#(#close_step),*,#state::Unselected=>Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
   }
   fn terminal_is_empty(&self)->bool{match &*self.state{#state::Unselected=>true,#(#terminal),*}}
  }
  impl #impl_generics Drop for #preparation #ty_generics #where_clause{
   fn drop(&mut self){let empty=<Self as ::semio_framework_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::terminal_is_empty(self);assert!(empty||::std::thread::panicking(),"retained aggregate requires exact funded leaf closure");if empty{unsafe{::core::mem::ManuallyDrop::drop(&mut self.state);}}}
  }
 })
}
#[cfg(test)]
mod tests{
 use super::*;
 /// 🧫️ The neutral variant roster expands into valid required leaf ports and physically quoted custody.
 #[test]
 fn retained_dispatch_requires_each_leaf_and_parses_as_rust(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
  let variants=fixture["variants"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>();
  let roster=variants.iter().map(|variant|format!("{variant}({variant})")).collect::<Vec<_>>().join(",");
  let input:syn::DeriveInput=syn::parse_str(&format!("enum Operation {{{roster}}}")).unwrap();
  let output=expand(&input,&syn::parse_str("Projection").unwrap()).unwrap();
  let source=output.to_string();let parsed:syn::File=syn::parse2(output).expect("third-party Rust parser validates generated owner");
  assert_eq!(parsed.items.len(),5);
  assert_eq!(variants.len(),fixture["requiredLeafCount"].as_u64().unwrap()as usize);
  assert!(source.matches("RetainedMutationLeaf <").count()>=variants.len());
  for variant in variants{assert!(source.contains(variant));}
  assert!(source.contains("maximum_copy_bytes"));assert!(source.contains("retained_mutation_demand_fits"));assert!(source.contains("ManuallyDrop"));assert!(source.contains("RetainedMutationCloseContext"));
  assert!(!source.contains("MutationKind"));assert!(!source.contains("apply_diff_cold"));assert!(!source.contains(". clone ("));
 }
}
