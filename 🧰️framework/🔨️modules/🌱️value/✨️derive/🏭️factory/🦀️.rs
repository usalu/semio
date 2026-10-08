//! 🏭️ Factory fields prove Copy payloads or explicitly retain prefunded child and inline owners.
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Member, parse_quote};

pub fn expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data) = &input.data else { return Err(syn::Error::new_spanned(input, "factory payload derivation requires a struct")); };
    let name = &input.ident;
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
            generics.make_where_clause().predicates.push(parse_quote!(#authority: ::semio_framework_value::FactoryRetirement));
            children.push(member);
            discarded.push(binding);
        } else if inline {
            generics.make_where_clause().predicates.push(parse_quote!(#ty: ::semio_framework_value::FactoryPayloadRetirement));
            owned.push((member, binding, ty.clone()));
        } else {
            generics.make_where_clause().predicates.push(parse_quote!(#ty: ::core::marker::Copy));
            discarded.push(binding);
        }
    }
    let count = children.len();
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
        impl #implementation ::semio_framework_value::FactoryPayloadRetirement for #name #types #predicates {
            type CloseState = (::semio_framework_value::FactoryChildTickets<#count>, #(<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::CloseState,)*);
            fn close_state_birth_bytes(&self) -> usize {
                let mut bytes = 0usize;
                #(bytes = bytes.checked_add(::semio_framework_value::FactoryRetirement::factory_retirement_birth_bytes(self.#children.as_ref())).expect("factory child ticket birth");)*
                #(bytes = bytes.checked_add(<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_birth_bytes(&self.#owned_members)).expect("factory inline close state birth");)*
                bytes
            }
            fn close_state_constructor_depth(&self) -> usize {
                let mut depth = 0usize;
                #(depth = depth.max(::semio_framework_value::FactoryRetirement::factory_retirement_depth_demand(self.#children.as_ref()));)*
                #(depth = depth.max(<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_constructor_depth(&self.#owned_members));)*
                depth
            }
            fn prepare_close_state(&self) -> Self::CloseState {
                (::semio_framework_value::FactoryChildTickets([#({
                    let child = ::std::sync::Arc::clone(&self.#children);
                    let grant = ::semio_framework_value::retained_clone::RetainedCloneGrant {
                        maximum_items: 1,
                        maximum_copy_bytes: 0,
                        maximum_capacity_bytes: ::semio_framework_value::FactoryRetirement::factory_retirement_birth_bytes(child.as_ref()),
                        maximum_release_bytes: 0,
                        maximum_depth: ::semio_framework_value::FactoryRetirement::factory_retirement_depth_demand(child.as_ref()),
                    };
                    let (ticket, _) = ::semio_framework_value::FactoryRetirement::preborn_factory_retirement(child, grant).unwrap_or_else(|(error, _)| panic!("prefunded factory child refused: {error}"));
                    Some(ticket)
                }),*]), #(<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::prepare_close_state(&self.#owned_members),)*)
            }
            fn transfer_payload(value: Self, state: &mut Self::CloseState) {
                #unpack
                #(<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::transfer_payload(#owned_bindings, &mut state.#indices);)*
                #(drop(#discarded);)*
            }
            fn close_state_demands(state: &Self::CloseState, copy: usize) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                if !state.0.terminal_is_empty() { return state.0.demands(copy); }
                #(if !<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices) { return <#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_demands(&state.#indices, copy); })*
                Ok(::semio_framework_value::RetirementDemand::default())
            }
            fn close_state_step(state: &mut Self::CloseState, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                if !state.0.terminal_is_empty() { return state.0.close_step(grant); }
                #(if !<#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices) {
                    return match <#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_step(&mut state.#indices, grant)? {
                        ::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) if <#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices) => Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)),
                        ::semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated, "factory inline payload returned false terminal")),
                        step => Ok(step),
                    };
                })*
                Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress::default()))
            }
            fn close_state_terminal_is_empty(state: &Self::CloseState) -> bool { state.0.terminal_is_empty() #(&& <#owned_types as ::semio_framework_value::FactoryPayloadRetirement>::close_state_terminal_is_empty(&state.#indices))* }
        }
    })
}
