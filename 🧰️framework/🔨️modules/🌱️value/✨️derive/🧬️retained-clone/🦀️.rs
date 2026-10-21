use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, GenericArgument, Generics, PathArguments, ReturnType, Type, TypeParamBound, parse_quote};

fn field_types(input: &DeriveInput) -> Vec<&Type> {
    match &input.data {
        Data::Struct(data) => data.fields.iter().map(|field| &field.ty).collect(),
        Data::Enum(data) => data.variants.iter().flat_map(|variant| variant.fields.iter().map(|field| &field.ty)).collect(),
        Data::Union(_) => Vec::new(),
    }
}

/// 🧭️ Whether a field type uses the parameter itself, not only as the base of an associated-type projection (`P::Item`, `<P as Trait>::Item`).
fn type_mentions_directly(ty: &Type, ident: &syn::Ident) -> bool {
    use quote::ToTokens;
    let text = ty.to_token_stream().to_string();
    let name = ident.to_string();
    let stripped = text.replace(&format!("{name} ::"), " ").replace(&format!("< {name} as"), " ");
    stripped.split(|character: char| !(character.is_alphanumeric() || character == '_')).any(|word| word == name)
}

fn bounded_generics(input: &DeriveInput, bound: TokenStream) -> Generics {
    let mut generics = input.generics.clone();
    for parameter in input.generics.type_params().filter(|parameter|field_types(input).into_iter().any(|ty|type_mentions_directly(ty,&parameter.ident))) {
        let ident=&parameter.ident;
        generics.make_where_clause().predicates.push(parse_quote!(#ident: #bound));
    }
    for ty in field_types(input).into_iter().filter(|ty| !type_mentions_ident(ty, &input.ident) && input.generics.type_params().any(|parameter| type_mentions_ident(ty, &parameter.ident))) {
        generics.make_where_clause().predicates.push(parse_quote!(#ty: #bound));
    }
    generics
}

fn path_arguments_mention(arguments: &PathArguments, ident: &syn::Ident) -> bool {
    match arguments {
        PathArguments::None => false,
        PathArguments::AngleBracketed(arguments) => arguments.args.iter().any(|argument| match argument {
            GenericArgument::Type(ty) => type_mentions_ident(ty, ident),
            GenericArgument::AssocType(binding) => type_mentions_ident(&binding.ty, ident),
            GenericArgument::Constraint(constraint) => constraint.bounds.iter().any(|bound| bound_mentions_ident(bound, ident)),
            _ => false,
        }),
        PathArguments::Parenthesized(arguments) => {
            arguments.inputs.iter().any(|ty| type_mentions_ident(ty, ident))
                || match &arguments.output {
                    ReturnType::Default => false,
                    ReturnType::Type(_, ty) => type_mentions_ident(ty, ident),
                }
        }
    }
}

fn bound_mentions_ident(bound: &TypeParamBound, ident: &syn::Ident) -> bool {
    match bound {
        TypeParamBound::Trait(bound) => bound.path.segments.iter().any(|segment| segment.ident == *ident || path_arguments_mention(&segment.arguments, ident)),
        _ => false,
    }
}

fn type_mentions_ident(ty: &Type, ident: &syn::Ident) -> bool {
    match ty {
        Type::Array(ty) => type_mentions_ident(&ty.elem, ident),
        Type::BareFn(ty) => {
            ty.inputs.iter().any(|input| type_mentions_ident(&input.ty, ident))
                || match &ty.output {
                    ReturnType::Default => false,
                    ReturnType::Type(_, output) => type_mentions_ident(output, ident),
                }
        }
        Type::Group(ty) => type_mentions_ident(&ty.elem, ident),
        Type::ImplTrait(ty) => ty.bounds.iter().any(|bound| bound_mentions_ident(bound, ident)),
        Type::Paren(ty) => type_mentions_ident(&ty.elem, ident),
        Type::Path(ty) => ty.qself.as_ref().is_some_and(|qself| type_mentions_ident(&qself.ty, ident)) || ty.path.segments.iter().any(|segment| segment.ident == *ident || path_arguments_mention(&segment.arguments, ident)),
        Type::Ptr(ty) => type_mentions_ident(&ty.elem, ident),
        Type::Reference(ty) => type_mentions_ident(&ty.elem, ident),
        Type::Slice(ty) => type_mentions_ident(&ty.elem, ident),
        Type::TraitObject(ty) => ty.bounds.iter().any(|bound| bound_mentions_ident(bound, ident)),
        Type::Tuple(ty) => ty.elems.iter().any(|element| type_mentions_ident(element, ident)),
        _ => false,
    }
}


/// 🧭️ Packed records cannot lend aligned references to their initialized fields.
pub(crate) fn require_aligned_fields(input:&DeriveInput,owner:&str)->syn::Result<()>{for attribute in input.attrs.iter().filter(|a|a.path().is_ident("repr")){let representations=attribute.parse_args_with(syn::punctuated::Punctuated::<syn::Meta,syn::Token![,]>::parse_terminated)?;if representations.iter().any(|r|r.path().is_ident("packed")){return Err(syn::Error::new_spanned(attribute,format!("{owner} requires naturally aligned fields and refuses packed records")));}}Ok(())}

fn shared_original_retirement(input:&DeriveInput,value_crate:&syn::Path,generics:&syn::Generics)->syn::Result<(TokenStream,TokenStream)>{
 let Data::Struct(data)=&input.data else{return Ok((quote!(),quote!()));};if crate::component::declared_send_only(input)?{return Ok((quote!(),quote!()));}require_aligned_fields(input,"RetireOwned")?;
 let name=&input.ident;let cursor=format_ident!("__{name}SharedOriginalRetirement");let(_,types,_)=generics.split_for_impl();let mut owned_generics=generics.clone();owned_generics.make_where_clause().predicates.push(syn::parse_quote!(#name #types:Sync));let(implementation,types,conditions)=owned_generics.split_for_impl();
 let owner=quote!(#value_crate::retirement::arc_custody::original::SharedOriginal<#name #types>);
 let demands=data.fields.iter().enumerate().map(|(index,field)|{let access=field_access(&data.fields,index);let ty=&field.ty;quote!(#index=>{if !<#ty as #value_crate::retirement::RetireOwned>::controlled_retirement_supported(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"shared original field has no controlled native issuer"));}let field=unsafe{original.field(|root|::std::ptr::addr_of!((*root).#access))};Ok(#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<#ty>().checked_mul(2).ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::WorkLimit,"shared original field movement overflow"))?,capacity_bytes:<#ty as #value_crate::retirement::RetireOwned>::retirement_birth_bytes(field).ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"shared original field has no native cursor birth"))?,depth:1,..Default::default()})})});
 let steps=data.fields.iter().enumerate().map(|(index,field)|{let access=field_access(&data.fields,index);let ty=&field.ty;quote!(#index=>{let value=unsafe{original.take_field(|root|::std::ptr::addr_of_mut!((*root).#access))};self.ordinal+=1;let child=<#ty as #value_crate::retirement::RetireOwned>::retirement(value);#value_crate::retirement::RetirementStep::ChildProgress(child,#value_crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})})});
 let declaration=quote!{
 struct #cursor #implementation #conditions{original:Option<#owner>,ordinal:usize}
 impl #implementation #value_crate::retirement::RetirementCursor for #cursor #types #conditions{
  fn next_demand(&self,_body:usize)->::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError>{let Some(original)=self.original.as_ref()else{return Ok(Default::default())};match self.ordinal{#(#demands,)*_=>Ok(#value_crate::RetirementDemand{release_bytes:<#owner>::allocation_bytes(),depth:1,..Default::default()})}}
  fn close_step(&mut self,grant:#value_crate::retained_clone::RetainedCloneGrant)->#value_crate::retirement::RetirementStep{let demand=match <Self as #value_crate::retirement::RetirementCursor>::next_demand(self,grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return #value_crate::retirement::RetirementStep::Failure(error)};if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand){return #value_crate::retirement::RetirementStep::BudgetExhausted;}let Some(original)=self.original.as_mut()else{return #value_crate::retirement::RetirementStep::Complete};match self.ordinal{#(#steps,)*_=>{let receipt=unsafe{original.release_empty(grant)};if receipt.copied_items==0{return #value_crate::retirement::RetirementStep::BudgetExhausted;}self.original=None;#value_crate::retirement::RetirementStep::Progress(receipt)}}}
  fn terminal_is_empty(&self)->bool{self.original.is_none()}
  fn next_work_byte_demand(&self)->::core::result::Result<usize,#value_crate::ValueError>{Ok(<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,0)?.copy_bytes)}
  fn next_birth_bytes(&self,body:usize)->Option<usize>{<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,body).ok().map(|d|d.capacity_bytes)}
  fn next_close_byte_demand(&self)->Option<usize>{<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,0).ok().map(|d|d.release_bytes)}
  fn terminal_release_bytes(&self)->Option<usize>{self.original.is_none().then_some(::std::mem::size_of::<Self>())}
 }
 };
 let methods=quote!{
  fn shared_original_birth_bytes(&self)->Option<usize> where Self:Sync{trait OriginalWithoutDrop<A>{fn verify(){}}impl<T:?Sized> OriginalWithoutDrop<()> for T{}#[allow(drop_bounds)]impl<T:?Sized+Drop> OriginalWithoutDrop<u8> for T{}let _=<Self as OriginalWithoutDrop<_>>::verify;Some(::std::mem::size_of::<#cursor #types>())}
  fn shared_original_retirement(original:&mut Option<#value_crate::retirement::arc_custody::original::SharedOriginal<Self>>,grant:#value_crate::retained_clone::RetainedCloneGrant)->::core::result::Result<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>,#value_crate::ValueError> where Self:Sync{if !<Self as #value_crate::retirement::RetireOwned>::controlled_retirement_supported(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"shared original has an unsupported native field"));}if original.is_none(){return Ok(None);}let capacity_bytes=::std::mem::size_of::<#cursor #types>();let demand=#value_crate::RetirementDemand{capacity_bytes,depth:1,..Default::default()};if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::OwnershipLimit,"shared original cursor birth is not funded"));}let mut cursor=Box::new(#cursor{original:None,ordinal:0});cursor.original=original.take();Ok(Some((cursor,#value_crate::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity_bytes,..Default::default()})))}
 };
 Ok((declaration,methods))
}

pub fn expand_retire_owned(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let value_crate = crate::component::declared_value_crate(input)?;
    let generics = bounded_generics(input, quote!(#value_crate::retirement::RetireOwned));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = match &input.data {
        Data::Struct(data) => {
            let (pattern, values) = destructure_fields(name, &data.fields, None);
            quote! {
                let #pattern = self;
                #value_crate::retirement::sequence(vec![
                    #(#value_crate::retirement::deferred(#values)),*
                ])
            }
        }
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let (pattern, values) = destructure_fields(name, &variant.fields, Some(variant_name));
                quote! {
                    #pattern => #value_crate::retirement::sequence(vec![
                        #(#value_crate::retirement::deferred(#values)),*
                    ])
                }
            });
            quote! { match self { #(#arms),* } }
        }
        Data::Union(data) => return Err(syn::Error::new_spanned(data.union_token, "#[derive(RetireOwned)] does not support unions")),
    };
    let birth = match &input.data {
        Data::Enum(data) if data.variants.is_empty() => quote! { match *self {} },
        Data::Struct(data) => {
            let types = data.fields.iter().map(|field| &field.ty);
            quote! { #value_crate::retirement::sequence_birth_bytes(&[#(#value_crate::retirement::deferred_birth_bytes::<#types>()),*]) }
        }
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|variant| {
                let (pattern, values) = destructure_fields(name, &variant.fields, Some(&variant.ident));
                let types = variant.fields.iter().map(|field| &field.ty);
                quote! { #pattern => { let _ = (#(#values),*); #value_crate::retirement::sequence_birth_bytes(&[#(#value_crate::retirement::deferred_birth_bytes::<#types>()),*]) } }
            });
            quote! { match self { #(#arms),* } }
        }
        Data::Union(_) => unreachable!(),
    };
    let components = field_types(input);
    let (shared_declaration,shared_methods)=shared_original_retirement(input,&value_crate,&generics)?;
    Ok(quote! {
        #shared_declaration
        impl #impl_generics #value_crate::retirement::RetireOwned for #name #ty_generics #where_clause {
            fn retirement(self) -> Box<dyn #value_crate::retirement::RetirementCursor> {
                #body
            }
            fn retirement_birth_bytes(&self) -> Option<usize> { #birth }
            #shared_methods
            fn controlled_retirement_supported() -> bool { true #(&& <#components as #value_crate::retirement::RetireOwned>::controlled_retirement_supported())* }
            fn retirement_element_copy_bytes() -> usize {
                #value_crate::retirement::composite_copy_bytes::<Self>(&[#(<#components as #value_crate::retirement::RetireOwned>::retirement_element_copy_bytes()),*])
            }
        }
    })
}

fn destructure_fields<'a>(name: &syn::Ident, fields: &'a Fields, variant: Option<&syn::Ident>) -> (TokenStream, Vec<syn::Ident>) {
    let prefix = variant.map_or_else(|| quote!(#name), |variant| quote!(#name::#variant));
    match fields {
        Fields::Named(named) => {
            let values = named.named.iter().enumerate().map(|(index, field)| format_ident!("__retained_field_{index}_{}", field.ident.as_ref().expect("named field"))).collect::<Vec<_>>();
            let names = named.named.iter().map(|field| field.ident.as_ref().expect("named field"));
            (quote!(#prefix { #(#names: #values),* }), values)
        }
        Fields::Unnamed(unnamed) => {
            let values = (0..unnamed.unnamed.len()).map(|index| format_ident!("__retained_field_{index}")).collect::<Vec<_>>();
            (quote!(#prefix ( #(#values),* )), values)
        }
        Fields::Unit => (prefix, Vec::new()),
    }
}

pub fn expand_retained_clone(input: &DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => expand_struct(input, &data.fields),
        Data::Enum(data) => expand_enum(input, data),
        Data::Union(data) => Err(syn::Error::new_spanned(data.union_token, "#[derive(RetainedClone)] does not support unions")),
    }
}

fn close_demand_methods<'a>(value_crate:&syn::Path,owner:TokenStream,rows:impl Iterator<Item=(&'a Type,&'a syn::Ident,&'a syn::Ident,bool)>)->TokenStream {
    let rows=rows.collect::<Vec<_>>();
    let children=rows.iter().map(|(ty,cursor,_,_)|quote! {
        if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { return #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_body_bytes); }
    });
    let values=rows.iter().map(|(ty,_,value,boxed)| {
        let ty=if *boxed {quote!(Box<#ty>)}else{quote!(#ty)};
        quote! { if self.#value.is_some() { return self.close.next_owner_demands::<#ty>(true,&self.source,maximum_body_bytes); } }
    });
    let methods=[("next_close_depth_demand","depth"),("next_close_copy_byte_demand","copy_bytes"),("next_close_capacity_byte_demand","capacity_bytes"),("next_close_release_byte_demand","release_bytes")].into_iter().map(|(method,axis)| {
        let method=format_ident!("{method}");let axis=format_ident!("{axis}");
        let (argument,hint)=if axis=="capacity_bytes" {(quote!(,maximum_body_bytes:usize),quote!(maximum_body_bytes))}else{(quote!(),quote!(0))};
        quote! {
            fn #method(&self #argument)->::core::result::Result<usize,#value_crate::ValueError> {
                Ok(#value_crate::retained_clone::RetainedCloneCursor::close_demands(self,#hint)?.#axis)
            }
        }
    });
    quote! {
        fn close_demands(&self,maximum_body_bytes:usize)->::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError> {
            if !self.closing { return Ok(Default::default()); }
            #(#children)*
            #(#values)*
            self.close.next_owner_demands::<#owner>(self.output.is_some(),&self.source,maximum_body_bytes)
        }
        #(#methods)*
    }
}

fn expand_struct(input: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    let name = &input.ident;
    let visibility = &input.vis;
    let cursor_name = format_ident!("__{name}RetainedCloneCursor");
    let generics = bounded_generics(input, quote!(#value_crate::retained_clone::RetainedClone));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let bitwise = bitwise_methods(input, fields)?;
    let field_rows = fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let ty = &field.ty;
            let cursor = format_ident!("field_{index}_cursor");
            let value = format_ident!("field_{index}_value");
            (index, ty, cursor, value)
        })
        .collect::<Vec<_>>();
    let cursor_fields = field_rows.iter().map(|(_, ty, cursor, value)| quote!(#cursor: #value_crate::retained_clone::RetainedFieldCursor<#ty>, #value: Option<#ty>));
    let cursor_init = field_rows.iter().map(|(_, _ty, cursor, value)| quote!(#cursor: Default::default(), #value: None));
    let demand_arms = field_rows.iter().map(|(index, ty, cursor, _)| {
        let access=field_access(fields,*index);
        quote! {
            #index => {
                if self.draining { return if #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { Ok(#value_crate::RetirementDemand {depth:1,..Default::default()}) } else { #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_copy_bytes) }; }
                if #value_crate::retained_clone::RetainedCloneCursor::output_ready(&self.#cursor) { return Ok(#value_crate::RetirementDemand {copy_bytes: ::std::mem::size_of::<#ty>(),depth:1,..Default::default()}); }
                #value_crate::retained_clone::nested_retained_clone_demands::<#ty>(&self.#cursor,source.project(#index+1,|source|&source.#access),maximum_copy_bytes)
            }
        }
    });
    let advance_arms = field_rows.iter().map(|(index, _, cursor, value)| {
        let access=field_access(fields,*index);
        quote! {
            #index => {
                if self.draining {
                    if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
                        let nested=#value_crate::retained_clone::nested_retained_clone_grant(grant)?;
                        let step=#value_crate::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor,nested)?;
                        return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_close(nested,step,#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor),"record original field scaffold")?.progress()));
                    }
                    self.draining=false;
                    self.phase+=1;
                    return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,..Default::default()}));
                }
                if #value_crate::retained_clone::RetainedCloneCursor::output_ready(&self.#cursor) {
                    self.#value=Some(#value_crate::retained_clone::RetainedCloneCursor::take(&mut self.#cursor).ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"record ready field omitted owner"))?);
                    #value_crate::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor);
                    self.draining=true;
                    return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                let nested=#value_crate::retained_clone::nested_retained_clone_grant(grant)?;
                let step=#value_crate::retained_clone::RetainedCloneCursor::advance(&mut self.#cursor,source.project(#index+1,|source|&source.#access),body,nested)?;
                Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_progress(nested,step.progress(),"record original field clone")?))
            }
        }
    });
    let construct = construct_fields(name, fields, &field_rows.iter().map(|(_, _, _, value)| value.clone()).collect::<Vec<_>>(), None);
    let controlled_values = field_rows.iter().map(|(_, _, _, value)| quote!(if let Some(step) = self.close.begin_granted(&mut self.#value, grant)? { return Ok(step); }));
    let controlled_children = field_rows.iter().map(|(_, _, cursor, _)| quote! {
        if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            if #value_crate::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor) {
                return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            let nested = #value_crate::retained_clone::nested_retained_clone_grant(grant)?;
            let step = #value_crate::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, nested)?;
            return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_close(nested, step, #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained record child close")?.progress()));
        }
    });
    let empty_children = field_rows.iter().map(|(_, _, cursor, value)| quote!(#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) && self.#value.is_none()));
    let close_demand=close_demand_methods(&value_crate,quote!(#name #ty_generics),field_rows.iter().map(|(_,ty,cursor,value)|(*ty,cursor,value,false)));
    let field_count = field_rows.len();
    Ok(quote! {
        #[doc(hidden)]
        #visibility struct #cursor_name #impl_generics #where_clause {
            phase: usize,
            output: Option<#name #ty_generics>,
            source: Option<#value_crate::retained_clone::RetainedCloneBinding>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: #value_crate::retained_clone::RetainedCloneClose,
            #(#cursor_fields),*
        }

        impl #impl_generics Default for #cursor_name #ty_generics #where_clause {
            fn default() -> Self {
                Self { phase: 0, output: None, source: None, spent: false, draining: false, closing: false, close: Default::default(), #(#cursor_init),* }
            }
        }

        impl #impl_generics #value_crate::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause {
            fn output_ready(&self) -> bool { !self.closing && !self.spent && self.output.is_some() }
            fn close_started(&self) -> bool { self.closing }
            fn advance_demands(&self,source: #value_crate::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,maximum_copy_bytes:usize)->::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError> {
                if self.closing || self.spent { return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"record clone cannot quote a closing or spent owner")); }
                if !source.check_binding(&self.source)? { return Ok(#value_crate::RetirementDemand {copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()}); }
                if self.output.is_some() { return Ok(Default::default()); }
                match self.phase {
                    #(#demand_arms,)*
                    #field_count => Ok(#value_crate::RetirementDemand {copy_bytes: ::std::mem::size_of::<#name #ty_generics>(),depth:1,..Default::default()}),
                    _ => Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"record clone state is invalid")),
                }
            }
            fn advance(&mut self,source: #value_crate::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,body: usize, grant: #value_crate::retained_clone::RetainedCloneGrant)->::core::result::Result<#value_crate::retained_clone::RetainedCloneStep,#value_crate::ValueError> {
                let demand=#value_crate::retained_clone::RetainedCloneCursor::advance_demands(self,source,body)?;
                if self.output.is_some() { return Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(Default::default())); }
                if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand) { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if let Some(progress)=source.bind(&mut self.source,grant)? { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(progress)); }
                match self.phase {
                    #(#advance_arms,)*
                    #field_count => {
                        self.output=Some(#construct);
                        self.phase+=1;
                        Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
                    }
                    _ => Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"record clone state is invalid")),
                }
            }
            fn take(&mut self) -> Option<#name #ty_generics> {
                let output = self.output.take();
                if output.is_some() { self.spent = true; }
                output
            }
            fn begin_close(&mut self) -> bool {
                if self.closing { return false; }
                self.closing = true;
                true
            }

            fn close_step(&mut self, grant: #value_crate::retained_clone::RetainedCloneGrant) -> ::core::result::Result<#value_crate::retained_clone::RetainedCloneStep, #value_crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvariantViolated, "retained record must begin close before granted retirement")); }
                if !#value_crate::retained_clone::retained_clone_grant_admits(grant,#value_crate::retained_clone::retained_clone_close_demands::<#name #ty_generics>(self,grant.maximum_copy_bytes)?) { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                #(#controlled_children)*
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                #(#controlled_values)*
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                #value_crate::retained_clone::close_retained_binding(&mut self.source, grant)
            }
            #close_demand
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() #(&& #empty_children)*
            }
        }

        impl #impl_generics #value_crate::retained_clone::RetainedClone for #name #ty_generics #where_clause {
            type Cursor = #cursor_name #ty_generics;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
            #bitwise
        }
    })
}

/// 🚚️ Reads `#[retained_clone(bitwise)]`: the opt-in that a `Copy` struct of bitwise fields clones as one memcpy run inside vectors and arrays.
fn bitwise_opt_in(input: &DeriveInput) -> syn::Result<bool> {
    let mut bitwise = false;
    for attribute in input.attrs.iter().filter(|attribute| attribute.path().is_ident("retained_clone")) {
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("bitwise") { bitwise = true; Ok(()) } else { Err(meta.error("unknown retained_clone option; expected `bitwise`")) }
        })?;
    }
    Ok(bitwise)
}

fn bitwise_methods(input: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    if !bitwise_opt_in(input)? { return Ok(TokenStream::new()); }
    let types = fields.iter().map(|field| &field.ty);
    Ok(quote! {
        const BITWISE: bool = true #(&& <#types as #value_crate::retained_clone::RetainedClone>::BITWISE)*;
        fn bitwise_array<const N: usize>(source: &[Self; N]) -> Option<[Self; N]> { if Self::BITWISE { Some(*source) } else { None } }
        fn bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool { if Self::BITWISE { target.extend_from_slice(source); true } else { false } }
    })
}

fn expand_enum(input: &DeriveInput, data: &syn::DataEnum) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    let name = &input.ident;
    let visibility = &input.vis;
    let cursor_name = format_ident!("__{name}RetainedCloneCursor");
    let generics = bounded_generics(input, quote!(#value_crate::retained_clone::RetainedClone));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let rows = data
        .variants
        .iter()
        .enumerate()
        .flat_map(|(variant_index, variant)| {
            variant.fields.iter().enumerate().map(move |(field_index, field)| {
                let ty = &field.ty;
                let cursor = format_ident!("variant_{variant_index}_field_{field_index}_cursor");
                let value = format_ident!("variant_{variant_index}_field_{field_index}_value");
                (variant_index, field_index, ty, cursor, value)
            })
        })
        .collect::<Vec<_>>();
    let cursor_fields = rows.iter().map(|(_, _, ty, cursor, value)| quote!(#cursor: #value_crate::retained_clone::RetainedFieldCursor<#ty>, #value: Option<Box<#ty>>));
    let cursor_init = rows.iter().map(|(_, _, _ty, cursor, value)| quote!(#cursor: Default::default(), #value: None));
    let select_arms = data
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let pattern = borrowed_variant_pattern(name, variant, variant_index).0;
            quote!(#pattern => #variant_index)
        })
        .collect::<Vec<_>>();
    let select_variant = if select_arms.is_empty() { quote!(match *source.get() {}) } else { quote!(match source.get() { #(#select_arms),* }) };
    let variants=data.variants.iter().enumerate().map(|(variant_index,variant)| {
        let (pattern,bindings)=borrowed_variant_pattern(name,variant,variant_index);
        let variant_rows=rows.iter().filter(|(row_variant,..)|*row_variant==variant_index).collect::<Vec<_>>();
        let demand_arms=variant_rows.iter().map(|(_,field_index,ty,cursor,_)| {
            let binding=&bindings[*field_index];
            quote! {
                #field_index => {
                    if self.draining { return if #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { Ok(#value_crate::RetirementDemand {depth:1,..Default::default()}) } else { #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_copy_bytes) }; }
                    if #value_crate::retained_clone::RetainedCloneCursor::output_ready(&self.#cursor) { return Ok(#value_crate::RetirementDemand {copy_bytes: ::std::mem::size_of::<#ty>(),capacity_bytes: ::std::mem::size_of::<#ty>(),depth:1,..Default::default()}); }
                    #value_crate::retained_clone::nested_retained_clone_demands::<#ty>(&self.#cursor,source.project((#variant_index+1)*1024+#field_index+1,|source|match source {#pattern=>#binding,_=>unreachable!("validated enum variant")}),maximum_copy_bytes)
                }
            }
        });
        let advance_arms=variant_rows.iter().map(|(_,field_index,_,cursor,value)| {
            let binding=&bindings[*field_index];
            quote! {
                #field_index => {
                    if self.draining {
                        if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
                            let nested=#value_crate::retained_clone::nested_retained_clone_grant(grant)?;
                            let step=#value_crate::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor,nested)?;
                            return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_close(nested,step,#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor),"enum original field scaffold")?.progress()));
                        }
                        self.draining=false;
                        self.phase+=1;
                        return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,..Default::default()}));
                    }
                    if #value_crate::retained_clone::RetainedCloneCursor::output_ready(&self.#cursor) {
                        self.#value=Some(Box::new(#value_crate::retained_clone::RetainedCloneCursor::take(&mut self.#cursor).ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum ready field omitted owner"))?));
                        #value_crate::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor);
                        self.draining=true;
                        return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}));
                    }
                    let nested=#value_crate::retained_clone::nested_retained_clone_grant(grant)?;
                    let step=#value_crate::retained_clone::RetainedCloneCursor::advance(&mut self.#cursor,source.project((#variant_index+1)*1024+#field_index+1,|source|match source {#pattern=>#binding,_=>unreachable!("validated enum variant")}),body,nested)?;
                    Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_progress(nested,step.progress(),"enum original field clone")?))
                }
            }
        });
        let values=variant_rows.iter().map(|(_,_,_,_,value)|(*value).clone()).collect::<Vec<_>>();
        let construct=construct_fields(name,&variant.fields,&values,Some(&variant.ident));
        let field_count=variant_rows.len();
        let construct_bytes=variant_rows.iter().map(|(_,_,ty,_,_)|quote!(::std::mem::size_of::<#ty>()));
        let demand=quote! {
            (#variant_index,#pattern) => match self.phase {
                #(#demand_arms,)*
                #field_count => Ok(#value_crate::RetirementDemand {copy_bytes: ::std::mem::size_of::<#name #ty_generics>(),release_bytes:0usize #(.saturating_add(#construct_bytes))*,depth:1,..Default::default()}),
                _=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum clone state is invalid")),
            }
        };
        let advance=quote! {
            (#variant_index,#pattern) => match self.phase {
                #(#advance_arms,)*
                #field_count => {
                    self.output=Some(#construct);
                    self.phase+=1;
                    Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
                }
                _=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum clone state is invalid")),
            }
        };
        (demand,advance)
    }).collect::<Vec<_>>();
    let variant_demands=variants.iter().map(|(demand,_)|demand);
    let variant_arms=variants.iter().map(|(_,advance)|advance);
    let controlled_values = rows.iter().map(|(_, _, _, _, value)| quote!(if let Some(step) = self.close.begin_granted(&mut self.#value, grant)? { return Ok(step); }));
    let controlled_children = rows.iter().map(|(_, _, _, cursor, _)| quote! {
        if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            if #value_crate::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor) {
                return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            let nested = #value_crate::retained_clone::nested_retained_clone_grant(grant)?;
            let step = #value_crate::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, nested)?;
            return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::admit_retained_clone_close(nested, step, #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained record child close")?.progress()));
        }
    });
    let empty_children = rows.iter().map(|(_, _, _, cursor, value)| quote!(#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) && self.#value.is_none()));
    let close_demand=close_demand_methods(&value_crate,quote!(#name #ty_generics),rows.iter().map(|(_,_,ty,cursor,value)|(*ty,cursor,value,true)));
    Ok(quote! {
        #[doc(hidden)]
        #visibility struct #cursor_name #impl_generics #where_clause {
            variant: Option<usize>,
            phase: usize,
            output: Option<#name #ty_generics>,
            source: Option<#value_crate::retained_clone::RetainedCloneBinding>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: #value_crate::retained_clone::RetainedCloneClose,
            #(#cursor_fields),*
        }

        impl #impl_generics Default for #cursor_name #ty_generics #where_clause {
            fn default() -> Self {
                Self { variant: None, phase: 0, output: None, source: None, spent: false, draining: false, closing: false, close: Default::default(), #(#cursor_init),* }
            }
        }

        impl #impl_generics #value_crate::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause {
            fn output_ready(&self) -> bool { !self.closing && !self.spent && self.output.is_some() }
            fn close_started(&self) -> bool { self.closing }
            fn advance_demands(&self,source: #value_crate::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,maximum_copy_bytes:usize)->::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError> {
                if self.closing || self.spent { return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum clone cannot quote a closing or spent owner")); }
                if !source.check_binding(&self.source)? { return Ok(#value_crate::RetirementDemand {copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()}); }
                if self.output.is_some() { return Ok(Default::default()); }
                let Some(variant)=self.variant else { return Ok(#value_crate::RetirementDemand {depth:1,..Default::default()}); };
                match (variant,source.get()) {
                    #(#variant_demands,)*
                    _=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum source variant changed")),
                }
            }
            fn advance(&mut self,source: #value_crate::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,body: usize, grant: #value_crate::retained_clone::RetainedCloneGrant)->::core::result::Result<#value_crate::retained_clone::RetainedCloneStep,#value_crate::ValueError> {
                let demand=#value_crate::retained_clone::RetainedCloneCursor::advance_demands(self,source,body)?;
                if self.output.is_some() { return Ok(#value_crate::retained_clone::RetainedCloneStep::Complete(Default::default())); }
                if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand) { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if let Some(progress)=source.bind(&mut self.source,grant)? { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(progress)); }
                if self.variant.is_none() {
                    self.variant=Some(#select_variant);
                    return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(#value_crate::retained_clone::RetainedCloneProgress {copied_items:1,..Default::default()}));
                }
                match (self.variant.expect("initialized enum variant"),source.get()) {
                    #(#variant_arms,)*
                    _=>Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"enum source variant changed")),
                }
            }
            fn take(&mut self) -> Option<#name #ty_generics> {
                let output = self.output.take();
                if output.is_some() { self.spent = true; }
                output
            }
            fn begin_close(&mut self) -> bool {
                if self.closing { return false; }
                self.closing = true;
                true
            }

            fn close_step(&mut self, grant: #value_crate::retained_clone::RetainedCloneGrant) -> ::core::result::Result<#value_crate::retained_clone::RetainedCloneStep, #value_crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvariantViolated, "retained record must begin close before granted retirement")); }
                if !#value_crate::retained_clone::retained_clone_grant_admits(grant,#value_crate::retained_clone::retained_clone_close_demands::<#name #ty_generics>(self,grant.maximum_copy_bytes)?) { return Ok(#value_crate::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                #(#controlled_children)*
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                #(#controlled_values)*
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                #value_crate::retained_clone::close_retained_binding(&mut self.source, grant)
            }
            #close_demand
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() #(&& #empty_children)*
            }
        }

        impl #impl_generics #value_crate::retained_clone::RetainedClone for #name #ty_generics #where_clause {
            type Cursor = #cursor_name #ty_generics;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
        }
    })
}

fn field_access(fields: &Fields, index: usize) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let ident = named.named[index].ident.as_ref().expect("named field");
            quote!(#ident)
        }
        Fields::Unnamed(_) => {
            let index = syn::Index::from(index);
            quote!(#index)
        }
        Fields::Unit => TokenStream::new(),
    }
}

fn construct_fields(name: &syn::Ident, fields: &Fields, values: &[syn::Ident], variant: Option<&syn::Ident>) -> TokenStream {
    let prefix = variant.map_or_else(|| quote!(#name), |variant| quote!(#name::#variant));
    let owners = values.iter().map(|value| if variant.is_some() { quote!(*self.#value.take().expect("retained field owner")) } else { quote!(self.#value.take().expect("retained field owner")) }).collect::<Vec<_>>();
    match fields {
        Fields::Named(named) => {
            let names = named.named.iter().map(|field| field.ident.as_ref().expect("named field"));
            quote!(#prefix { #(#names: #owners),* })
        }
        Fields::Unnamed(_) => quote!(#prefix ( #(#owners),* )),
        Fields::Unit => prefix,
    }
}

fn borrowed_variant_pattern(name: &syn::Ident, variant: &syn::Variant, variant_index: usize) -> (TokenStream, Vec<syn::Ident>) {
    let variant_name = &variant.ident;
    match &variant.fields {
        Fields::Named(named) => {
            let bindings = named.named.iter().enumerate().map(|(field_index, field)| format_ident!("__source_{variant_index}_{field_index}_{}", field.ident.as_ref().expect("named field"))).collect::<Vec<_>>();
            let names = named.named.iter().map(|field| field.ident.as_ref().expect("named field"));
            (quote!(#name::#variant_name { #(#names: #bindings),* }), bindings)
        }
        Fields::Unnamed(unnamed) => {
            let bindings = (0..unnamed.unnamed.len()).map(|field_index| format_ident!("__source_{variant_index}_{field_index}")).collect::<Vec<_>>();
            (quote!(#name::#variant_name ( #(#bindings),* )), bindings)
        }
        Fields::Unit => (quote!(#name::#variant_name), Vec::new()),
    }
}
