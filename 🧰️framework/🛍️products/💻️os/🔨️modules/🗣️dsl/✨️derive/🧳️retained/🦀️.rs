//! 🧳️ Retained aggregate dispatch requires every leaf and admits exactly one typed child body.
use quote::{quote,format_ident};
use syn::{DeriveInput,Data,Fields,Type};

/// 🧬️ Produces a closed retained owner without deriving semantic decision from cold methods.
pub fn expand(input:&DeriveInput,snapshot:&Type)->syn::Result<proc_macro2::TokenStream>{
 let name=&input.ident;let preparation=format_ident!("__{}RetainedMutationPreparation",name);let state=format_ident!("__{}RetainedMutationState",name);
 let Data::Enum(data)=&input.data else{return Err(syn::Error::new_spanned(input,"retained mutation dispatch requires an enum"));};
 let generics=&input.generics;let(impl_generics,ty_generics,where_clause)=generics.split_for_impl();let aggregate=quote!(#name #ty_generics);
 let value=quote!(::semio_framework_value);let clone=quote!(::semio_framework_value::retained_clone);
 let effect_type=quote!(#value::RetainedAllocationEffect<'_>);
 let reply_type=quote!(#clone::OriginalEffectReply<'r,#clone::RetainedCloneStep,#value::OriginalRetirementFailure>);
 let done_type=quote!(#clone::OriginalEffectReplyComplete<'r,#clone::RetainedCloneStep,#value::OriginalRetirementFailure>);
 let invariant=|message:&str|quote!(#value::ValueError::literal(#value::ValueRefusalKind::InvariantViolated,#message));
 let mut variants=Vec::new();let mut wrappers=Vec::new();let mut select_demand=Vec::new();let mut select_work=Vec::new();let mut select=Vec::new();let mut demand=Vec::new();let mut decision_work=Vec::new();let mut decide=Vec::new();let mut outcome=Vec::new();let mut inverse_demand=Vec::new();let mut inverse_work=Vec::new();let mut inverse=Vec::new();let mut take_inverse=Vec::new();let mut close_demand=Vec::new();let mut close_work=Vec::new();let mut close=Vec::new();let mut terminal=Vec::new();let mut birth=Vec::new();let mut control=Vec::new();let mut birth_step=Vec::new();
 for(index,variant)in data.variants.iter().enumerate(){
  let variant_name=&variant.ident;let Fields::Unnamed(fields)=&variant.fields else{return Err(syn::Error::new_spanned(variant,"retained mutation variants require a single leaf"));};
  if fields.unnamed.len()!=1{return Err(syn::Error::new_spanned(variant,"retained mutation variants require a single leaf"));}
  let leaf=&fields.unnamed[0].ty;
  let child=quote!(<#leaf as ::semio_os_kernel::RetainedMutationLeaf<#snapshot,#aggregate>>::Preparation);
  let ports=quote!(<#child as ::semio_os_kernel::RetainedMutationLeafPreparation<#snapshot,#aggregate,#leaf>>);
  let payload=quote!(mutation.project(#index,|operation|match operation{#name::#variant_name(payload)=>payload,_=>unreachable!("immutable retained leaf discriminator")}));
  variants.push(quote!(#variant_name(::std::boxed::Box<#child>)));
  wrappers.push(quote!(impl #impl_generics ::semio_os_kernel::RetainedLeafWrapper<#leaf> for #aggregate #where_clause{fn wrap_work_demand()->u64{1}fn wrap(value:#leaf)->(Self,u64){(Self::#variant_name(value),1)}}));
  select_demand.push(quote!(#name::#variant_name(_)=>#value::RetirementDemand{copy_bytes: ::core::mem::size_of::<#child>()+::core::mem::size_of::<#state #ty_generics>(),capacity_bytes: ::core::mem::size_of::<#child>(),release_bytes:0,depth:2}));
  select_work.push(quote!(#name::#variant_name(_)=>#ports::birth_work()?.checked_add(u64::try_from(::core::mem::size_of::<#state #ty_generics>()).map_err(|_|#value::ValueError::literal(#value::ValueRefusalKind::WorkLimit,"retained leaf work overflow"))?).ok_or_else(||#value::ValueError::literal(#value::ValueRefusalKind::WorkLimit,"retained leaf work overflow"))));
  select.push(quote!(#name::#variant_name(_)=>{if effect.actual().is_some(){return Err(#value::ValueError::invariant("original retained leaf marker is occupied").into())}let layout=::std::alloc::Layout::new::<#child>();let pointer=if layout.size()==0{::std::ptr::NonNull::<#child>::dangling().as_ptr()}else{*effect.actual()=Some(0);unsafe{::std::alloc::alloc(layout)}.cast::<#child>()};if pointer.is_null(){return Err(#value::ValueError::literal(#value::ValueRefusalKind::AllocationFailed,"retained leaf allocation refused").into())}if layout.size()!=0{*effect.actual()=Some(layout.size());}unsafe{pointer.write(#ports::new());*self.state=#state::#variant_name(::std::boxed::Box::from_raw(pointer));}}));
  demand.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>{let mut demand=#ports::decision_demand(owner.as_ref(),#payload,original,maximum_body_bytes,maximum_work_units)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||#value::ValueError::literal(#value::ValueRefusalKind::DepthLimit,"retained leaf depth overflow"))?;Ok(demand)}));
  decision_work.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::decision_work_demand(owner.as_ref(),#payload,original,body,maximum_work_units)));
  decide.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::decide_reply(owner.as_mut(),#payload,original,body,maximum_work_units,&mut effect.nested(child),reply,recover)));
  inverse_demand.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>{let mut demand=#ports::inverse_demand(owner.as_ref(),#payload,original,diff,maximum_body_bytes,maximum_work_units)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||#value::ValueError::literal(#value::ValueRefusalKind::DepthLimit,"retained inverse leaf depth overflow"))?;Ok(demand)}));
  inverse_work.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::inverse_work_demand(owner.as_ref(),#payload,original,diff,body,maximum_work_units)));
  inverse.push(quote!((#state::#variant_name(owner),#name::#variant_name(_))=>#ports::inverse_reply(owner.as_mut(),#payload,original,diff,body,maximum_work_units,&mut effect.nested(child),reply,recover)));
  take_inverse.push(quote!(#state::#variant_name(owner)=>#ports::take_inverse(owner.as_mut())));
  outcome.push(quote!(#state::#variant_name(owner)=>#ports::take_decision(owner.as_mut())));
  close_demand.push(quote!(#state::#variant_name(owner)=>{if #ports::terminal_is_empty(owner.as_ref()){Ok(#value::RetirementDemand{copy_bytes: ::core::mem::size_of::<#state #ty_generics>(),release_bytes: ::core::mem::size_of::<#child>(),depth:1,..Default::default()})}else{let mut demand=if !#ports::close_started(owner.as_ref()){#ports::begin_close_demand(owner.as_ref())}else{#ports::close_demand(owner.as_ref(),context,maximum_body_bytes)?};demand.depth=demand.depth.checked_add(1).ok_or_else(||#value::ValueError::literal(#value::ValueRefusalKind::DepthLimit,"retained leaf close depth overflow"))?;Ok(demand)}}));
  close_work.push(quote!(#state::#variant_name(owner)=>{if #ports::terminal_is_empty(owner.as_ref()){Ok(1)}else if !#ports::close_started(owner.as_ref()){Ok(#ports::begin_close_work_demand(owner.as_ref()))}else{#ports::close_work_demand(owner.as_ref(),context,maximum_body_bytes)}}));
  close.push(quote!(#state::#variant_name(owner)=>{
    if !recover&&#ports::terminal_is_empty(owner.as_ref()){
     let base=effect.performed_work();let demand=#value::RetirementDemand{copy_bytes: ::core::mem::size_of::<#state #ty_generics>(),release_bytes: ::core::mem::size_of::<#child>(),depth:1,..Default::default()};
     if !::semio_os_kernel::retained_mutation_demand_fits(demand,grant){return ::semio_os_kernel::retained_conclude(effect,base,Ok(#clone::RetainedCloneStep::Progress(Default::default())),0,|step|step.progress(),reply)}
     drop(::core::mem::replace(&mut *self.state,#state::Unselected));
     return ::semio_os_kernel::retained_conclude(effect,base,Ok(#clone::RetainedCloneStep::Complete(#clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),1,|step|step.progress(),reply)
    }
    if !recover&&!#ports::close_started(owner.as_ref()){
     let base=effect.performed_work();let turn=#ports::begin_close_paid(owner.as_mut(),&mut effect.nested(child));
     return ::semio_os_kernel::retained_conclude(effect,base,turn.result.map(#clone::RetainedCloneStep::Progress).map_err(#value::RetirementPollExit::from),turn.work,|step|step.progress(),reply)
    }
    #ports::close_reply(owner.as_mut(),context,original,body,&mut effect.nested(child),reply,recover)
   }));
  terminal.push(quote!(#state::#variant_name(_)=>false));
  birth.push(quote!(#state::#variant_name(owner)=>#ports::requires_original_birth(owner.as_ref())));
  control.push(quote!(#state::#variant_name(owner)=>#ports::requires_original_control(owner.as_ref())));
  birth_step.push(quote!(#state::#variant_name(owner)=>#ports::close_original_birth_step(owner.as_mut(),original,grant)));
 }
 let closing_refusal=invariant("retained aggregate already closing");let changed_refusal=invariant("retained aggregate leaf changed");let not_started_refusal=invariant("retained aggregate closure not started");let recovered_early_refusal=invariant("retained aggregate recovered before leaf selection");let inverse_closing_refusal=invariant("retained aggregate inverse already closing");let inverse_leaf_refusal=invariant("retained inverse requires the original decided leaf");let inverse_changed_refusal=invariant("retained inverse leaf changed");
  Ok(quote!{
   #(#wrappers)*
  enum #state #generics #where_clause{Unselected,#(#variants),*}
  /// 🧳️ Required typed leaf custody until separately paid body closure.
  pub struct #preparation #generics #where_clause{state: ::core::mem::ManuallyDrop<#state #ty_generics>,closing:bool}
  impl #impl_generics ::semio_os_kernel::RetainedMutation<#snapshot> for #aggregate #where_clause{type Preparation=#preparation #ty_generics;}
  impl #impl_generics ::semio_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate> for #preparation #ty_generics #where_clause{
   fn new()->Self{Self{state: ::core::mem::ManuallyDrop::new(#state::Unselected),closing:false}}
   fn birth_work()->Result<u64,#value::ValueError>{u64::try_from(::core::mem::size_of::<Self>()).ok().and_then(|n|n.checked_add(1)).ok_or_else(||#value::ValueError::literal(#value::ValueRefusalKind::WorkLimit,"retained mutation birth work overflow"))}
   fn decision_demand(&self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,maximum_body_bytes:usize,maximum_work_units:u64)->Result<#value::RetirementDemand,#value::ValueError>{
    if self.closing{return Err(#closing_refusal);}
    if matches!(&*self.state,#state::Unselected){return Ok(match mutation.get(){#(#select_demand),*});}
    match(&*self.state,mutation.get()){#(#demand),*,_=>Err(#changed_refusal)}
   }
   fn decision_work_demand(&self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,body:usize,maximum_work_units:u64)->Result<u64,#value::ValueError>{self.decision_demand(mutation,original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if matches!(&*self.state,#state::Unselected){return match mutation.get(){#(#select_work),*}}match(&*self.state,mutation.get()){#(#decision_work),*,_=>Err(#changed_refusal)}}
   fn decide_reply<'r>(&mut self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,body:usize,maximum_work_units:u64,effect:&mut #effect_type,reply:#reply_type,recover:bool)->Result<#done_type,#value::ValueError>{
    if self.closing{return Err(#closing_refusal);}
    let grant=effect.grant();
    if matches!(&*self.state,#state::Unselected){
     let base=effect.performed_work();let mut work:u128=0;
     let result:Result<#clone::RetainedCloneStep,#value::RetirementPollExit>=(||{
      if recover{return Err(#recovered_early_refusal.into())}
      let selected=<Self as ::semio_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::decision_work_demand(self,mutation,original,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(#clone::RetainedCloneStep::Progress(Default::default()))}
      let demand=<Self as ::semio_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::decision_demand(self,mutation,original,body,maximum_work_units)?;
      if !::semio_os_kernel::retained_mutation_demand_fits(demand,grant){return Ok(#clone::RetainedCloneStep::Progress(Default::default()))}
      match mutation.get(){#(#select),*};
      work=u128::from(selected);
      Ok(#clone::RetainedCloneStep::Progress(#clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
     })();
     return ::semio_os_kernel::retained_conclude(effect,base,result,work,|step|step.progress(),reply);
    }
    let child=#clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
    match(&mut *self.state,mutation.get()){#(#decide),*,_=>Err(#changed_refusal)}
   }
   fn take_decision(&mut self)->::semio_os_kernel::RetainedMutationDecision<<#aggregate as ::semio_os_kernel::Mutation<#snapshot>>::Diff>{match &mut *self.state{#(#outcome),*,#state::Unselected=>panic!("retained aggregate has no admitted leaf outcome")}}
   fn inverse_demand(&self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,diff: #clone::RetainedCloneRef<'_,<#aggregate as ::semio_os_kernel::Mutation<#snapshot>>::Diff>,maximum_body_bytes:usize,maximum_work_units:u64)->Result<#value::RetirementDemand,#value::ValueError>{
    if self.closing{return Err(#inverse_closing_refusal);}
    match(&*self.state,mutation.get()){#(#inverse_demand),*,_=>Err(#inverse_leaf_refusal)}
   }
   fn inverse_work_demand(&self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,diff: #clone::RetainedCloneRef<'_,<#aggregate as ::semio_os_kernel::Mutation<#snapshot>>::Diff>,body:usize,maximum_work_units:u64)->Result<u64,#value::ValueError>{self.inverse_demand(mutation,original,diff,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}match(&*self.state,mutation.get()){#(#inverse_work),*,_=>Err(#inverse_changed_refusal)}}
   fn inverse_reply<'r>(&mut self,mutation: #clone::RetainedCloneRef<'_,#aggregate>,original: #clone::RetainedCloneRef<'_,#snapshot>,diff: #clone::RetainedCloneRef<'_,<#aggregate as ::semio_os_kernel::Mutation<#snapshot>>::Diff>,body:usize,maximum_work_units:u64,effect:&mut #effect_type,reply:#reply_type,recover:bool)->Result<#done_type,#value::ValueError>{
    if self.closing{return Err(#inverse_closing_refusal);}
    let grant=effect.grant();let child=#clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
    match(&mut *self.state,mutation.get()){#(#inverse),*,_=>Err(#inverse_leaf_refusal)}
   }
   fn take_inverse(&mut self)->Vec<#aggregate>{match &mut *self.state{#(#take_inverse),*,#state::Unselected=>panic!("retained aggregate has no prepared inverse")}}
   fn begin_close(&mut self){self.closing=true;}
   fn close_started(&self)->bool{self.closing}
   fn begin_close_demand(&self)->#value::RetirementDemand{if self.closing{Default::default()}else{#value::RetirementDemand{copy_bytes: ::core::mem::size_of::<bool>(),depth:1,..Default::default()}}}
   fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
   fn begin_close_paid(&mut self,effect:&mut #effect_type)->#value::RetainedWorkOutcome<#clone::RetainedCloneProgress,#value::ValueError>{let demand=<Self as ::semio_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::begin_close_demand(self);if self.closing||effect.maximum_work_units()==0||!::semio_os_kernel::retained_mutation_demand_fits(demand,effect.grant()){return #value::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let work=match effect.performed(1){Ok(true)=>1,Ok(false)=>return #value::RetainedWorkOutcome{result:Err(#value::ValueError::literal(#value::ValueRefusalKind::WorkLimit,"retained aggregate close start exceeded its original action")),work:1},Err(error)=>return #value::RetainedWorkOutcome{result:Err(error),work:1}};self.closing=true;#value::RetainedWorkOutcome{result:Ok(#clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),work}}
   fn close_demand(&self,context: ::semio_os_kernel::RetainedMutationCloseContext<'_,#aggregate>,maximum_body_bytes:usize)->Result<#value::RetirementDemand,#value::ValueError>{match &*self.state{#(#close_demand),*,#state::Unselected=>Ok(Default::default())}}
   fn close_work_demand(&self,context: ::semio_os_kernel::RetainedMutationCloseContext<'_,#aggregate>,maximum_body_bytes:usize)->Result<u64,#value::ValueError>{if !self.closing{return Err(#not_started_refusal);}match &*self.state{#(#close_work),*,#state::Unselected=>Ok(0)}}
   fn requires_original_birth(&self)->bool{match &*self.state{#(#birth),*,#state::Unselected=>false}}
   fn requires_original_control(&self)->bool{match &*self.state{#(#control),*,#state::Unselected=>false}}
   fn close_original_birth_step(&mut self,original:&mut #value::OriginalRetirementCloseLoan<'_>,grant:#clone::RetainedCloneGrant)->#value::OriginalRetirementPoll{match &mut *self.state{#(#birth_step),*,#state::Unselected=>{let _=(original,grant);#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#value::ValueError::invariant("retained aggregate has no original Birth").into()),work:0})}}}
   fn close_reply<'r>(&mut self,context: ::semio_os_kernel::RetainedMutationCloseContext<'_,#aggregate>,original:Option<&mut #value::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut #effect_type,reply:#reply_type,recover:bool)->Result<#done_type,#value::ValueError>{
    if !self.closing{return Err(#not_started_refusal);}
    let grant=effect.grant();let child=#clone::RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant};
    match &mut *self.state{#(#close),*,#state::Unselected=>{let base=effect.performed_work();::semio_os_kernel::retained_conclude(effect,base,Ok(#clone::RetainedCloneStep::Complete(Default::default())),0,|step|step.progress(),reply)}}
   }
   fn terminal_is_empty(&self)->bool{match &*self.state{#state::Unselected=>true,#(#terminal),*}}
  }
  impl #impl_generics Drop for #preparation #ty_generics #where_clause{
   fn drop(&mut self){let empty=<Self as ::semio_os_kernel::RetainedMutationPreparation<#snapshot,#aggregate>>::terminal_is_empty(self);assert!(empty||::std::thread::panicking(),"retained aggregate requires exact funded leaf closure");if empty{unsafe{::core::mem::ManuallyDrop::drop(&mut self.state);}}}
  }
 })
}
#[cfg(test)]
mod tests{
 use super::*;
 /// 🧫️ The neutral variant roster expands into valid required leaf ports and physically quoted custody.
 #[test]
 fn retained_dispatch_requires_each_leaf_and_parses_as_rust(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  let variants=fixture["variants"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>();
  let roster=variants.iter().map(|variant|format!("{variant}({variant})")).collect::<Vec<_>>().join(",");
  let input:syn::DeriveInput=syn::parse_str(&format!("enum Operation {{{roster}}}")).unwrap();
  let output=expand(&input,&syn::parse_str("Projection").unwrap()).unwrap();
  let source=output.to_string();let parsed:syn::File=syn::parse2(output).expect("third-party Rust parser validates generated owner");
  assert_eq!(parsed.items.len(),5+variants.len(),"state, preparation and three trait owners plus one wrapper per leaf");
  let method=parsed.items.iter().filter_map(|item|match item{syn::Item::Impl(item)=>Some(item),_=>None}).flat_map(|item|item.items.iter()).filter_map(|item|match item{syn::ImplItem::Fn(method)if method.sig.ident=="close_demand"=>Some(method),_=>None}).next().expect("generated close demand");assert_eq!(method.sig.inputs.len(),3,"close demand matches the actual preparation trait");
  assert_eq!(variants.len(),fixture["requiredLeafCount"].as_u64().unwrap()as usize);
  assert!(source.matches("RetainedMutationLeaf <").count()>=variants.len());
  for variant in variants{assert!(source.contains(variant));}
  assert!(source.contains("decide_reply"));assert!(source.contains("close_reply"));assert!(source.contains("retained_conclude"));assert!(!source.contains("RetainedMutationAdvance"));assert!(source.contains("retained_mutation_demand_fits"));assert!(source.contains("ManuallyDrop"));assert!(source.contains("RetainedMutationCloseContext"));
  assert!(!source.contains("MutationKind"));assert!(!source.contains("apply_diff_cold"));assert!(!source.contains(". clone ("));
 }
}
