//! 🏭️ Factory fields prove Copy payloads or explicitly retain prefunded child and inline owners.
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Member, parse_quote};

pub fn expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data) = &input.data else { return Err(syn::Error::new_spanned(input, "factory payload derivation requires a struct")); };
    let name = &input.ident;
    let value_crate = crate::component::declared_value_crate(input)?;
    let mut generics = input.generics.clone();
    let mut children = Vec::new();
    let mut owned = Vec::new();
    let mut bindings = Vec::new();
    let mut members = Vec::new();
    let mut discarded = Vec::new();
    for (index, field) in data.fields.iter().enumerate() {
        let member = field.ident.clone().map(Member::Named).unwrap_or_else(|| Member::Unnamed(index.into()));
        let binding = format_ident!("factory_field_{index}");
        bindings.push(binding.clone());
        members.push(member.clone());
        let ty = &field.ty;
        let child = field.attrs.iter().any(|attribute| attribute.path().is_ident("factory_child"));
        let inline = field.attrs.iter().any(|attribute| attribute.path().is_ident("factory_owned"));
        if child && inline { return Err(syn::Error::new_spanned(field, "factory field cannot be both a child alias and an inline owner")); }
        if child {
            let syn::Type::Path(path) = ty else { return Err(syn::Error::new_spanned(field, "factory child requires an original Arc alias")); };
            let Some(segment) = path.path.segments.last().filter(|segment| segment.ident == "Arc") else { return Err(syn::Error::new_spanned(field, "factory child requires an original Arc alias")); };
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else { return Err(syn::Error::new_spanned(field, "factory child Arc requires its concrete authority")); };
            let Some(syn::GenericArgument::Type(authority)) = arguments.args.first() else { return Err(syn::Error::new_spanned(field, "factory child Arc requires its concrete authority")); };
            generics.make_where_clause().predicates.push(parse_quote!(#authority: #value_crate::FactoryRetirement));
            children.push(member);
            discarded.push(binding);
        } else if inline {
            generics.make_where_clause().predicates.push(parse_quote!(#ty: #value_crate::FactoryPayloadRetirement));
            owned.push((member, binding, ty.clone()));
        } else {
            generics.make_where_clause().predicates.push(parse_quote!(#ty: ::core::marker::Copy));
            discarded.push(binding);
        }
    }
    let count = children.len();
    let child_indices:Vec<_>=(0..count).map(syn::Index::from).collect();
    let indices: Vec<_> = (1..=owned.len()).map(syn::Index::from).collect();
    let owned_members: Vec<_> = owned.iter().map(|row| &row.0).collect();
    let owned_bindings: Vec<_> = owned.iter().map(|row| &row.1).collect();
    let owned_types: Vec<_> = owned.iter().map(|row| &row.2).collect();
    let unpack = match &data.fields {
        Fields::Named(_) => quote!(let Self { #(#members: #bindings),* } = value;),
        Fields::Unnamed(_) => quote!(let Self(#(#bindings),*) = value;),
        Fields::Unit => quote!(drop(value);),
    };
    let (_, original_types, _) = input.generics.split_for_impl();
    generics.make_where_clause().predicates.push(parse_quote!(#name #original_types: ::core::marker::Send + ::core::marker::Sync + 'static));
    let (implementation, types, predicates) = generics.split_for_impl();
    Ok(quote! {
        impl #implementation #value_crate::FactoryPayloadRetirement for #name #types #predicates {
            type CloseState = (#value_crate::FactoryChildTickets<#count>, #(<#owned_types as #value_crate::FactoryPayloadRetirement>::CloseState,)*);
            fn close_state_birth_bytes(&self) -> usize {
                let mut bytes = 0usize;
                #(bytes = bytes.checked_add(<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_birth_bytes(&self.#owned_members)).expect("factory inline close state birth");)*
                bytes
            }
            fn close_state_constructor_depth(&self) -> usize {
                let mut depth = 0usize;
                #(depth = depth.max(<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_constructor_depth(&self.#owned_members));)*
                depth
            }
            fn prepare_close_state(&self) -> Self::CloseState {
                (#value_crate::FactoryChildTickets{slots: ::std::array::from_fn(|_|#value_crate::FactoryChildSlot::empty()),prepared:0}, #(<#owned_types as #value_crate::FactoryPayloadRetirement>::prepare_close_state(&self.#owned_members),)*)
            }
            fn close_state_constructor_copy_bytes(&self)->usize {
                let mut bytes=0usize;
                #(bytes=bytes.checked_add(<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_constructor_copy_bytes(&self.#owned_members)).expect("factory original inline construction copy");)*
                bytes
            }
            fn close_state_preparation_demands(&self,state:&Self::CloseState,body:usize)->Result<#value_crate::RetirementDemand,#value_crate::ValueError>{
                #(if state.0.prepared==#child_indices {
                    let slot=&state.0.slots[#child_indices];
                    if slot.preparation_is_complete(){return Ok(#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<usize>(),depth:1,..Default::default()});}
                    return slot.preparation_demands(self.#children.as_ref(),body);
                })*
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_demands(&self.#owned_members,&state.#indices,body);})*
                Ok(Default::default())
            }
            fn close_state_preparation_work_demand(&self,state:&Self::CloseState,body:usize)->Result<u64,#value_crate::ValueError>{
                #(if state.0.prepared==#child_indices {let slot=&state.0.slots[#child_indices];if slot.preparation_is_complete(){return Ok(1)}return slot.preparation_work_demand(self.#children.as_ref(),body);})*
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_work_demand(&self.#owned_members,&state.#indices,body);})*
                Ok(0)
            }
            fn prepare_close_state_into<'reply>(&self,state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                #(if state.0.prepared==#child_indices{let slot=&mut state.0.slots[#child_indices];if slot.preparation_is_complete(){let demand=#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<usize>(),depth:1,..Default::default()};if !#value_crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1||!effect.performed(1)?{return Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Progress(Default::default()))))}state.0.prepared+=1;return Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Progress(#value_crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
                    if slot.needs_capture(){let Some(capture)=slot.begin_capture(self.#children.as_ref(),effect)? else{return Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Progress(Default::default()))))};let mut alias:Option<::std::sync::Arc<dyn #value_crate::FactoryRetirement>>=Some(self.#children.clone());return capture.publish(&mut alias,reply.progress())}
                    return slot.prepare_borrowed_into(self.#children.as_ref(),body,effect,reply.progress(),false)
                })*
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::prepare_close_state_into(&self.#owned_members,&mut state.#indices,body,effect,reply.progress())})*
                Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Complete(Default::default()))))
            }
            fn recover_prepare_close_state_into<'reply>(&self,state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                #(if state.0.prepared==#child_indices{return state.0.slots[#child_indices].prepare_borrowed_into(self.#children.as_ref(),body,effect,reply.progress(),true)})*
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::recover_prepare_close_state_into(&self.#owned_members,&mut state.#indices,body,effect,reply.progress())})*
                Err(#value_crate::ValueError::invariant("factory state has no retained preparation to recover"))
            }
            fn prepare_close_state_step(&self,state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::RetainedWorkPoll{let grant=effect.grant();let units=match <Self as #value_crate::FactoryPayloadRetirement>::close_state_preparation_work_demand(self,state,body){Ok(units)=>units,Err(error)=>return #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(error),work:0})};if effect.maximum_work_units()<units{return #value_crate::RetainedWorkPoll::Pending{work:0}}
                #(if state.0.prepared==#child_indices{
                    let slot=&mut state.0.slots[#child_indices];if slot.preparation_is_complete(){
                        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes < ::std::mem::size_of::<usize>(){return #value_crate::RetainedWorkPoll::Pending{work:0}}
                        match effect.performed(1){Ok(true)=>{},Ok(false)=>return #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::WorkLimit,"factory preparation header exceeded original work")),work:1}),Err(error)=>return #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(error),work:1})}
                        state.0.prepared+=1;return #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes: ::std::mem::size_of::<usize>(),..Default::default()})),work:1})
                    }
                    let turn=slot.prepare_step(self.#children.as_ref(),||{let original: ::std::sync::Arc<dyn #value_crate::FactoryRetirement>=self.#children.clone();original},body,effect);return turn.map(|step|#value_crate::retained_clone::RetainedCloneStep::Progress(step.progress()))
                })*
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices){let turn=<#owned_types as #value_crate::FactoryPayloadRetirement>::prepare_close_state_step(&self.#owned_members,&mut state.#indices,body,effect);return turn.map(|step|#value_crate::retained_clone::RetainedCloneStep::Progress(step.progress()))})*
                #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(Default::default())),work:0})
            }
            fn close_state_preparation_is_complete(state:&Self::CloseState)->bool {
                state.0.prepared==#count #(&& <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_preparation_is_complete(&state.#indices))*
            }
            fn transfer_payload(value: Self, state: &mut Self::CloseState) {
                #unpack
                #(<#owned_types as #value_crate::FactoryPayloadRetirement>::transfer_payload(#owned_bindings, &mut state.#indices);)*
                #(drop(#discarded);)*
            }
            fn close_state_demands(state: &Self::CloseState, copy: usize) -> Result<#value_crate::RetirementDemand, #value_crate::ValueError> {
                if !state.0.terminal_is_empty() { return state.0.demands(copy); }
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices) { return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_demands(&state.#indices, copy); })*
                Ok(#value_crate::RetirementDemand::default())
            }
            fn close_state_requires_original_control(state:&Self::CloseState)->bool{if !state.0.terminal_is_empty(){return state.0.requires_original_control()}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_requires_original_control(&state.#indices)})*false}
            fn close_state_requires_original_birth(state:&Self::CloseState)->bool{if !state.0.terminal_is_empty(){return state.0.requires_original_birth()}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_requires_original_birth(&state.#indices)})*false}
            fn close_state_original_birth_step(state:&mut Self::CloseState,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,grant:#value_crate::RetainedCloneGrant)->#value_crate::OriginalRetirementPoll{if !state.0.terminal_is_empty(){return state.0.close_original_birth_step(original,grant)}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_original_birth_step(&mut state.#indices,original,grant)})*#value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"factory payload has no selected original birth").into()),work:0})}
            fn close_state_work_demand(state:&Self::CloseState,body:usize)->Result<u64,#value_crate::ValueError>{if !state.0.terminal_is_empty(){return state.0.next_computational_work(body)}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_work_demand(&state.#indices,body)})*Ok(0)}
            fn close_state_step_controlled(state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,control:&mut dyn #value_crate::OriginalRetirementControl)->#value_crate::OriginalRetirementPoll{if !state.0.terminal_is_empty(){return state.0.close_step_controlled(body,effect,control)}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){let turn=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_step_controlled(&mut state.#indices,body,effect,control);return turn.and_then(|step|{let complete=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices);match step{#value_crate::retained_clone::RetainedCloneStep::Complete(progress)if complete=>Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(progress)),#value_crate::retained_clone::RetainedCloneStep::Complete(progress)=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"factory inline payload returned false terminal").with_retained_progress(progress).into()),step=>Ok(step)}})})*#value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"factory state has no selected original child").into()),work:0})}
            fn close_state_original_step(state:&mut Self::CloseState,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::OriginalRetirementPoll{if !state.0.terminal_is_empty(){return state.0.close_original_step(original,body,effect)}#(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){let turn=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_original_step(&mut state.#indices,original,body,effect);return turn.and_then(|step|{let complete=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices);match step{#value_crate::retained_clone::RetainedCloneStep::Complete(progress)if complete=>Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(progress)),#value_crate::retained_clone::RetainedCloneStep::Complete(progress)=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"factory inline payload returned false terminal").with_retained_progress(progress).into()),step=>Ok(step)}})})*#value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"factory state has no selected original child").into()),work:0})}
            fn close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                if !state.0.terminal_is_empty(){return state.0.close_into(None,body,effect,reply.progress(),false)}
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_into(&mut state.#indices,body,effect,reply.progress())})*
                Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Complete(Default::default()))))
            }
            fn recover_close_state_into<'reply>(state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                if !state.0.terminal_is_empty(){return state.0.close_into(None,body,effect,reply.progress(),true)}
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::recover_close_state_into(&mut state.#indices,body,effect,reply.progress())})*
                Err(#value_crate::ValueError::invariant("factory state has no retained close to recover"))
            }
            fn close_state_original_into<'reply>(state:&mut Self::CloseState,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                if !state.0.terminal_is_empty(){return state.0.close_into(Some(original),body,effect,reply.progress(),false)}
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_original_into(&mut state.#indices,original,body,effect,reply.progress())})*
                Ok(reply.complete(Ok(#value_crate::RetainedCloneStep::Complete(Default::default()))))
            }
            fn recover_close_state_original_into<'reply>(state:&mut Self::CloseState,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::OriginalEffectReply<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::OriginalEffectReplyComplete<'reply,#value_crate::RetainedCloneStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{
                if !state.0.terminal_is_empty(){return state.0.close_into(Some(original),body,effect,reply.progress(),true)}
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){return <#owned_types as #value_crate::FactoryPayloadRetirement>::recover_close_state_original_into(&mut state.#indices,original,body,effect,reply.progress())})*
                Err(#value_crate::ValueError::invariant("factory state has no retained original close to recover"))
            }
            fn close_state_step(state:&mut Self::CloseState,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::OriginalRetirementPoll{
                if !state.0.terminal_is_empty(){return state.0.close_step(body,effect)}
                #(if !<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices){
                    let turn=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_step(&mut state.#indices,body,effect);return turn.and_then(|step|{let complete=<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices);match step{
                        #value_crate::retained_clone::RetainedCloneStep::Complete(progress)if complete=>Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(progress)),
                        #value_crate::retained_clone::RetainedCloneStep::Complete(progress)=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"factory inline payload returned false terminal").with_retained_progress(progress).into()),
                        step=>Ok(step),
                    }})
                })*
                #value_crate::RetainedWorkPoll::Complete(#value_crate::RetainedWorkOutcome{result:Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(Default::default())),work:0})
            }
            unsafe fn close_state_terminal_native_release_safe()->bool{true #(&& unsafe{<#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_native_release_safe()})*}
            fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool { state.0.terminal_is_empty() #(&& <#owned_types as #value_crate::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices))* }
        }
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod namespace_tests;

