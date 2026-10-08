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

fn bounded_generics(input: &DeriveInput, bound: TokenStream) -> Generics {
    let mut generics = input.generics.clone();
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

pub fn expand_retire_owned(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let generics = bounded_generics(input, quote!(::semio_framework_value::retirement::RetireOwned));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = match &input.data {
        Data::Struct(data) => {
            let (pattern, values) = destructure_fields(name, &data.fields, None);
            quote! {
                let #pattern = self;
                ::semio_framework_value::retirement::sequence(vec![
                    #(::semio_framework_value::retirement::deferred(#values)),*
                ])
            }
        }
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let (pattern, values) = destructure_fields(name, &variant.fields, Some(variant_name));
                quote! {
                    #pattern => ::semio_framework_value::retirement::sequence(vec![
                        #(::semio_framework_value::retirement::deferred(#values)),*
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
            quote! { ::semio_framework_value::retirement::sequence_birth_bytes(&[#(::semio_framework_value::retirement::deferred_birth_bytes::<#types>()),*]) }
        }
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|variant| {
                let (pattern, values) = destructure_fields(name, &variant.fields, Some(&variant.ident));
                let types = variant.fields.iter().map(|field| &field.ty);
                quote! { #pattern => { let _ = (#(#values),*); ::semio_framework_value::retirement::sequence_birth_bytes(&[#(::semio_framework_value::retirement::deferred_birth_bytes::<#types>()),*]) } }
            });
            quote! { match self { #(#arms),* } }
        }
        Data::Union(_) => unreachable!(),
    };
    Ok(quote! {
        impl #impl_generics ::semio_framework_value::retirement::RetireOwned for #name #ty_generics #where_clause {
            fn retirement(self) -> Box<dyn ::semio_framework_value::retirement::RetirementCursor> {
                #body
            }
            fn retirement_birth_bytes(&self) -> Option<usize> { #birth }
            fn controlled_retirement_supported() -> bool { true }
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

fn close_demand_methods<'a>(owner:TokenStream,rows:impl Iterator<Item=(&'a Type,&'a syn::Ident,&'a syn::Ident,bool)>)->TokenStream {
    let rows=rows.collect::<Vec<_>>();
    let depth_children=rows.iter().map(|(_,cursor,_,_)|quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            return ::semio_framework_value::retained_clone::RetainedCloneCursor::next_close_depth_demand(&self.#cursor);
        }
    });
    let depth_values=rows.iter().map(|(_,_,value,_)|quote!(||self.#value.is_some()));
    let copy_children=rows.iter().map(|(_,cursor,_,_)|quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            return ::semio_framework_value::retained_clone::RetainedCloneCursor::next_close_copy_byte_demand(&self.#cursor);
        }
    });
    let capacity_children=rows.iter().map(|(_,cursor,_,_)|quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            return ::semio_framework_value::retained_clone::RetainedCloneCursor::next_close_capacity_byte_demand(&self.#cursor,maximum_release_bytes);
        }
    });
    let release_children=rows.iter().map(|(_,cursor,_,_)|quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            return ::semio_framework_value::retained_clone::RetainedCloneCursor::next_close_release_byte_demand(&self.#cursor);
        }
    });
    let values=rows.iter().map(|(ty,_,value,boxed)| {
        let ty=if *boxed {quote!(Box<#ty>)} else {quote!(#ty)};
        quote!(if self.#value.is_some() { return self.close.next_owner_capacity_with_binding::<#ty>(true,maximum_release_bytes,&self.source); })
    });
    quote! {
        fn next_close_depth_demand(&self)->Result<usize,::semio_framework_value::ValueError> {
            if !self.closing {return Ok(0);}
            #(#depth_children)*
            self.close.next_owner_depth_with_binding(self.output.is_some()#(#depth_values)*,&self.source)
        }
        fn next_close_copy_byte_demand(&self)->Result<usize,::semio_framework_value::ValueError> {
            if !self.closing {return Ok(0);}
            #(#copy_children)*
            self.close.next_copy_with_binding(&self.source)
        }
        fn next_close_capacity_byte_demand(&self,maximum_release_bytes:usize)->Result<usize,::semio_framework_value::ValueError> {
            if !self.closing {return Ok(0);}
            #(#capacity_children)*
            if !self.close.is_empty() {return self.close.next_capacity_byte_demand(maximum_release_bytes);}
            #(#values)*
            self.close.next_owner_capacity_with_binding::<#owner>(self.output.is_some(),maximum_release_bytes,&self.source)
        }
        fn next_close_release_byte_demand(&self)->Result<usize,::semio_framework_value::ValueError> {
            if !self.closing {return Ok(0);}
            #(#release_children)*
            self.close.next_release_with_binding(&self.source)
        }
    }
}

fn expand_struct(input: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let visibility = &input.vis;
    let cursor_name = format_ident!("__{name}RetainedCloneCursor");
    let generics = bounded_generics(input, quote!(::semio_framework_value::retained_clone::RetainedClone));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
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
    let cursor_fields = field_rows.iter().map(|(_, ty, cursor, value)| quote!(#cursor: ::semio_framework_value::retained_clone::RetainedFieldCursor<#ty>, #value: Option<#ty>));
    let cursor_init = field_rows.iter().map(|(_, ty, cursor, value)| quote!(#cursor: Default::default(), #value: None));
    let advance_arms = field_rows.iter().map(|(index, _, cursor, value)| {
        let access = field_access(fields, *index);
        quote! {
            #index => match ::semio_framework_value::retained_clone::RetainedCloneCursor::advance(&mut self.#cursor, source.project(#index + 1, |source| &source.#access), grant)? {
                ::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(
                    ::semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "retained struct field")?
                )),
                ::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => {
                    let progress = ::semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "retained struct field")?;
                    if progress.copied_items >= grant.maximum_items {
                        return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
                    }
                    self.#value = Some(::semio_framework_value::retained_clone::RetainedCloneCursor::take(&mut self.#cursor).ok_or_else(|| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct field completed without an owner"))?);
                    let _ = ::semio_framework_value::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor);
                    self.draining = true;
                    Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress.checked_add(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                }
            }
        }
    });
    let drain_arms = field_rows.iter().map(|(index, _, cursor, _)| quote! {
            #index => {
                if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
                    if grant.maximum_items == 0 {
                        return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
                    }
                    let step = ::semio_framework_value::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, grant)?;
                    let progress = ::semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, ::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained field scaffold close")?.progress();
                    return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
            }
            if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
            self.draining = false;
            self.phase += 1;
            Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }
    });
    let construct = construct_fields(name, fields, &field_rows.iter().map(|(_, _, _, value)| value.clone()).collect::<Vec<_>>(), None);
    let controlled_values = field_rows.iter().map(|(_, _, _, value)| quote!(if let Some(step) = self.close.begin_granted(&mut self.#value, grant)? { return Ok(step); }));
    let controlled_children = field_rows.iter().map(|(_, _, cursor, _)| quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            if ::semio_framework_value::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor) {
                return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            let step = ::semio_framework_value::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, grant)?;
            return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, ::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained record child close")?.progress()));
        }
    });
    let empty_children = field_rows.iter().map(|(_, _, cursor, value)| quote!(::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) && self.#value.is_none()));
    let close_demand=close_demand_methods(quote!(#name #ty_generics),field_rows.iter().map(|(_,ty,cursor,value)|(*ty,cursor,value,false)));
    let field_count = field_rows.len();
    Ok(quote! {
        #[doc(hidden)]
        #visibility struct #cursor_name #impl_generics #where_clause {
            phase: usize,
            output: Option<#name #ty_generics>,
            source: Option<::semio_framework_value::retained_clone::RetainedCloneBinding>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: ::semio_framework_value::retained_clone::RetainedCloneClose,
            #(#cursor_fields),*
        }

        impl #impl_generics Default for #cursor_name #ty_generics #where_clause {
            fn default() -> Self {
                Self { phase: 0, output: None, source: None, spent: false, draining: false, closing: false, close: Default::default(), #(#cursor_init),* }
            }
        }

        impl #impl_generics ::semio_framework_value::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause {
            fn advance(&mut self, source: ::semio_framework_value::retained_clone::RetainedCloneRef<'_, #name #ty_generics>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is closing")); }
                if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone cursor is spent")); }
                if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                source.bind(&mut self.source)?;
                if self.draining {
                    return match self.phase { #(#drain_arms,)* _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct drain state is invalid")) };
                }
                match self.phase {
                    #(#advance_arms,)*
                    #field_count => {
                        if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                        self.output = Some(#construct);
                        self.phase += 1;
                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                    }
                    _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained struct clone state is invalid")),
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
            
            fn close_step(&mut self, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained record must begin close before granted retirement")); }
                #(#controlled_children)*
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                #(#controlled_values)*
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                ::semio_framework_value::retained_clone::close_retained_binding(&mut self.source, grant)
            }
            #close_demand
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() #(&& #empty_children)*
            }
        }

        impl #impl_generics ::semio_framework_value::retained_clone::RetainedClone for #name #ty_generics #where_clause {
            type Cursor = #cursor_name #ty_generics;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
        }
    })
}

fn expand_enum(input: &DeriveInput, data: &syn::DataEnum) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let visibility = &input.vis;
    let cursor_name = format_ident!("__{name}RetainedCloneCursor");
    let generics = bounded_generics(input, quote!(::semio_framework_value::retained_clone::RetainedClone));
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
    let cursor_fields = rows.iter().map(|(_, _, ty, cursor, value)| quote!(#cursor: ::semio_framework_value::retained_clone::RetainedFieldCursor<#ty>, #value: Option<Box<#ty>>));
    let cursor_init = rows.iter().map(|(_, _, ty, cursor, value)| quote!(#cursor: Default::default(), #value: None));
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
    let variant_arms = data.variants.iter().enumerate().map(|(variant_index, variant)| {
        let (pattern, bindings) = borrowed_variant_pattern(name, variant, variant_index);
        let variant_rows = rows.iter().filter(|(row_variant, _, _, _, _)| *row_variant == variant_index).collect::<Vec<_>>();
        let advance_arms = variant_rows.iter().map(|(_, field_index, ty, cursor, value)| {
            let binding = &bindings[*field_index];
            let projection_pattern = pattern.clone();
            quote! {
                #field_index => match ::semio_framework_value::retained_clone::RetainedCloneCursor::advance(
                    &mut self.#cursor,
                    source.project((#variant_index + 1) * 1024 + #field_index + 1, |source| match source {
                        #projection_pattern => #binding,
                        _ => unreachable!("retained enum selected variant changed"),
                    }),
                    grant,
                )? {
                    ::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(
                        ::semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "retained enum field")?
                    )),
                    ::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => {
                        let progress = ::semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "retained enum field")?;
                        if progress.copied_items >= grant.maximum_items {
                            return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
                        }
                        let capacity = ::std::mem::size_of::<#ty>();
                        if capacity > grant.maximum_capacity_bytes.saturating_sub(progress.retained_capacity_bytes) { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)); }
                        self.#value = Some(Box::new(::semio_framework_value::retained_clone::RetainedCloneCursor::take(&mut self.#cursor).ok_or_else(|| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum field completed without an owner"))?));
                        let _ = ::semio_framework_value::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor);
                        self.draining = true;
                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress.checked_add(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?))
                    }
                }
            }
        });
        let drain_arms = variant_rows.iter().map(|(_, field_index, _, cursor, _)| quote! {
            #field_index => {
                if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
                    if grant.maximum_items == 0 {
                        return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
                    }
                    let step = ::semio_framework_value::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, grant)?;
                    let progress = ::semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, ::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained field scaffold close")?.progress();
                    return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
                }
                if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                self.draining = false;
                self.phase += 1;
                Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
        });
        let values = variant_rows.iter().map(|(_, _, _, _, value)| (*value).clone()).collect::<Vec<_>>();
        let construct = construct_fields(name, &variant.fields, &values, Some(&variant.ident));
        let field_count = variant_rows.len();
        let construct_bytes = variant_rows.iter().map(|(_, _, ty, _, _)| quote!(::std::mem::size_of::<#ty>()));
        quote! {
            (#variant_index, #pattern) => {
                if self.draining {
                    return match self.phase { #(#drain_arms,)* _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum drain state is invalid")) };
                }
                match self.phase {
                    #(#advance_arms,)*
                    #field_count => {
                        if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                        let bytes = 0usize #(.saturating_add(#construct_bytes))*;
                        if bytes > grant.maximum_release_bytes { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                        self.output = Some(#construct);
                        self.phase += 1;
                        Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }))
                    }
                    _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone state is invalid")),
                }
            }
        }
    });
    let controlled_values = rows.iter().map(|(_, _, _, _, value)| quote!(if let Some(step) = self.close.begin_granted(&mut self.#value, grant)? { return Ok(step); }));
    let controlled_children = rows.iter().map(|(_, _, _, cursor, _)| quote! {
        if !::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) {
            if ::semio_framework_value::retained_clone::RetainedCloneCursor::begin_close(&mut self.#cursor) {
                return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            let step = ::semio_framework_value::retained_clone::RetainedCloneCursor::close_step(&mut self.#cursor, grant)?;
            return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, ::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor), "retained record child close")?.progress()));
        }
    });
    let empty_children = rows.iter().map(|(_, _, _, cursor, value)| quote!(::semio_framework_value::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) && self.#value.is_none()));
    let close_demand=close_demand_methods(quote!(#name #ty_generics),rows.iter().map(|(_,_,ty,cursor,value)|(*ty,cursor,value,true)));
    Ok(quote! {
        #[doc(hidden)]
        #visibility struct #cursor_name #impl_generics #where_clause {
            variant: Option<usize>,
            phase: usize,
            output: Option<#name #ty_generics>,
            source: Option<::semio_framework_value::retained_clone::RetainedCloneBinding>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: ::semio_framework_value::retained_clone::RetainedCloneClose,
            #(#cursor_fields),*
        }

        impl #impl_generics Default for #cursor_name #ty_generics #where_clause {
            fn default() -> Self {
                Self { variant: None, phase: 0, output: None, source: None, spent: false, draining: false, closing: false, close: Default::default(), #(#cursor_init),* }
            }
        }

        impl #impl_generics ::semio_framework_value::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause {
            fn advance(&mut self, source: ::semio_framework_value::retained_clone::RetainedCloneRef<'_, #name #ty_generics>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                if self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is closing")); }
                if self.spent { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum clone cursor is spent")); }
                if self.output.is_some() { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                source.bind(&mut self.source)?;
                if self.variant.is_none() {
                    if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                    self.variant = Some(#select_variant);
                    return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                match (self.variant.expect("initialized retained enum variant"), source.get()) {
                    #(#variant_arms,)*
                    _ => Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained enum source variant changed during copy")),
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
            
            fn close_step(&mut self, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                if grant.maximum_items == 0 { return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvariantViolated, "retained record must begin close before granted retirement")); }
                #(#controlled_children)*
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                #(#controlled_values)*
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                ::semio_framework_value::retained_clone::close_retained_binding(&mut self.source, grant)
            }
            #close_demand
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() #(&& #empty_children)*
            }
        }

        impl #impl_generics ::semio_framework_value::retained_clone::RetainedClone for #name #ty_generics #where_clause {
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
