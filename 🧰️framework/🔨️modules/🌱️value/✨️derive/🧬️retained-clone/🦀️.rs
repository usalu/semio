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

fn clone_generics(input:&DeriveInput,value_crate:&syn::Path)->Generics{
 let mut generics=bounded_generics(input,quote!(#value_crate::retained_clone::RetainedClone));let name=&input.ident;let(_,types,_)=input.generics.split_for_impl();generics.make_where_clause().predicates.push(parse_quote!(#name #types: #value_crate::retirement::RetireOwned));generics
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


/// 📹️ Packed records cannot lend aligned references to their initialized fields.
pub(crate) fn require_aligned_fields(input:&DeriveInput,owner:&str)->syn::Result<()>{for attribute in input.attrs.iter().filter(|a|a.path().is_ident("repr")){let representations=attribute.parse_args_with(syn::punctuated::Punctuated::<syn::Meta,syn::Token![,]>::parse_terminated)?;if representations.iter().any(|r|r.path().is_ident("packed")){return Err(syn::Error::new_spanned(attribute,format!("{owner} requires naturally aligned fields and refuses packed records")));}}Ok(())}

fn original_allocation_retirement(input:&DeriveInput,value_crate:&syn::Path,generics:&Generics,shared:bool)->syn::Result<TokenStream>{
 let Data::Struct(_)=&input.data else{return Ok(quote!())};if shared&&crate::component::declared_send_only(input)?||!shared&&!crate::component::declared_boxed_original(input)?{return Ok(quote!())}
 let name=&input.ident;let(_,types,_)=generics.split_for_impl();let record=quote!(#value_crate::retirement::record);
 let owner=if shared{quote!(#value_crate::retirement::arc_custody::original::SharedOriginal<#name #types>)}else{quote!(#value_crate::retirement::original_box::BoxOriginal<#name #types>)};
 let no_drop=quote!{trait OriginalWithoutDrop<A>{fn verify(){}}impl<T:?Sized> OriginalWithoutDrop<()> for T{}#[allow(drop_bounds)]impl<T:?Sized+Drop> OriginalWithoutDrop<u8> for T{}let _=<Self as OriginalWithoutDrop<_>>::verify;};
 let outcome=quote!(#value_crate::RetainedWorkOutcome<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>>);
 Ok(if shared{quote!{
  fn shared_original_demands(&self)->Result<Option<#value_crate::RetirementDemand>,#value_crate::ValueError> where Self:Sync{#no_drop if !#value_crate::retirement::RetireOwned::retirement_ready(self){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"shared original requires source recovery"))}Ok(Some(#record::original_record_demand::<#owner>()))}
  fn shared_original_work_demand(&self)->Result<Option<u64>,#value_crate::ValueError> where Self:Sync{Ok(Some(1))}
  fn shared_original_retirement(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#outcome where Self:Sync{#record::admit_original_record(original,effect)}
  fn native_shared_original_admission()->Option<#value_crate::retirement::NativeSharedRetirementAdmission<Self>> where Self:Sync{Some(unsafe{#value_crate::retirement::NativeSharedRetirementAdmission::new(#record::admit_original_record::<#owner>)})}
 }}else{quote!{
  fn boxed_original_demands(&self)->::core::result::Result<Option<#value_crate::RetirementDemand>,#value_crate::ValueError>{#no_drop Ok(Some(#record::original_record_demand::<#owner>()))}
  fn boxed_original_work_demand(&self)->Result<Option<u64>,#value_crate::ValueError>{Ok(Some(1))}
  fn boxed_original_retirement(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#outcome{#record::admit_original_record(original,effect)}
  fn native_boxed_original_admission()->Option<#value_crate::retirement::NativeBoxedRetirementAdmission<Self>>{Some(unsafe{#value_crate::retirement::NativeBoxedRetirementAdmission::new(#record::admit_original_record::<#owner>)})}
 }})
}

/// 🏛️ Emits the record's authored field ports once; both original allocations retire through the shared `retirement::record` machine.
fn original_record_fields(input:&DeriveInput,value_crate:&syn::Path,generics:&Generics)->syn::Result<TokenStream>{
 let Data::Struct(data)=&input.data else{return Ok(quote!())};if crate::component::declared_send_only(input)?&&!crate::component::declared_boxed_original(input)?{return Ok(quote!())}require_aligned_fields(input,"RetireOwned")?;
 let name=&input.ident;let(implementation,types,conditions)=generics.split_for_impl();let record=quote!(#value_crate::retirement::record);
 let demands=data.fields.iter().enumerate().map(|(index,_)|{let access=field_access(&data.fields,index);quote!(#index=>Some(#record::field_birth_demand(unsafe{original.original_field(|root|::std::ptr::addr_of!((*root).#access))})))});
 let steps=data.fields.iter().enumerate().map(|(index,field)|{let access=field_access(&data.fields,index);let ty=&field.ty;quote!(#index=>{let field=unsafe{original.original_field(|root|::std::ptr::addr_of!((*root).#access))}as *const #ty;Some(unsafe{#record::admit_field_with(field,effect,||original.take_original_field(|root|::std::ptr::addr_of_mut!((*root).#access)))})})});
 Ok(quote!{
  impl #implementation #record::OriginalRecordFields for #name #types #conditions{
   fn original_field_demand<O:#record::OriginalRecordOwner<Record=Self>>(original:&O,ordinal:usize)->Option<::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError>>{match ordinal{#(#demands,)*_=>None}}
   unsafe fn original_field_step<O:#record::OriginalRecordOwner<Record=Self>>(original:&mut O,ordinal:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->Option<#value_crate::retirement::RetirementStep>{match ordinal{#(#steps,)*_=>None}}
  }
 })
}

/// 🌿 Forms an ordered retirement tree with logarithmic depth across sibling fields.
fn retirement_fields(value: &syn::Path, fields: &[(&syn::Field, &syn::Ident)]) -> (TokenStream, TokenStream) {
    match fields {
        [] => (quote!(#value::retirement::RetirementFieldsEnd), quote!(#value::retirement::RetirementFieldsEnd)),
        [(field, binding)] => {
            let ty = &field.ty;
            (quote!(#value::retirement::Deferred<#ty>), quote!(#value::retirement::deferred(#binding)))
        }
        _ => {
            let (left, right) = fields.split_at(fields.len() / 2);
            let (left_type, left_seed) = retirement_fields(value, left);
            let (right_type, right_seed) = retirement_fields(value, right);
            (
                quote!(#value::retirement::RetirementFieldsPair<#left_type, #right_type>),
                quote!(#value::retirement::RetirementFieldsPair::new(#left_seed, #right_seed)),
            )
        }
    }
}

pub fn expand_retire_owned(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let value_crate = crate::component::declared_value_crate(input)?;
    let generics = bounded_generics(input, quote!(#value_crate::retirement::RetireOwned));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let cursor_name=format_ident!("__{name}OwnedRetirementSeed");
    let inner_name=format_ident!("__{name}OwnedRetirementFields");
    let visibility=&input.vis;
    let rows=match &input.data{
      Data::Struct(data)=>{let(pattern,values)=destructure_fields(name,&data.fields,None);vec![(format_ident!("Owner"),&data.fields,pattern,values)]},
      Data::Enum(data)=>data.variants.iter().map(|variant|{let(pattern,values)=destructure_fields(name,&variant.fields,Some(&variant.ident));(variant.ident.clone(),&variant.fields,pattern,values)}).collect::<Vec<_>>(),
      Data::Union(data)=>return Err(syn::Error::new_spanned(data.union_token,"#[derive(RetireOwned)] does not support unions")),
    };
    let (variants, arms): (Vec<_>, Vec<_>) = rows.iter().map(|(variant, fields, pattern, values)| {
        let fields: Vec<_> = fields.iter().zip(values).collect();
        let (ty, seed) = retirement_fields(&value_crate, &fields);
        (quote!(#variant(#ty)), quote!(#pattern => #value_crate::retirement::ShellCursor(#cursor_name { inner: #inner_name::#variant(#seed) })))
    }).unzip();
    let (cursor_declaration,cursor_type,body)={
        let names=rows.iter().map(|(variant,_,_,_)|variant).collect::<Vec<_>>();
        let tail=if names.is_empty(){quote!(_=>unreachable!("uninhabited retirement"))}else{quote!()};
        let declaration=quote!{
          #[doc(hidden)] #visibility struct #cursor_name #impl_generics #where_clause{inner:#inner_name #ty_generics}
          enum #inner_name #impl_generics #where_clause{#(#variants),*}
          impl #impl_generics #value_crate::retirement::RetirementShell for #cursor_name #ty_generics #where_clause{
            fn shell(&self)->&dyn #value_crate::retirement::RetirementCursor{match &self.inner{#( #inner_name::#names(value)=>value as &dyn #value_crate::retirement::RetirementCursor),*#tail}}
            fn shell_mut(&mut self)->&mut dyn #value_crate::retirement::RetirementCursor{match &mut self.inner{#( #inner_name::#names(value)=>value as &mut dyn #value_crate::retirement::RetirementCursor),*#tail}}
          }
        };(declaration,quote!(#value_crate::retirement::ShellCursor<#cursor_name #ty_generics>),quote!(match self{#(#arms),*}))
    };
    let components = field_types(input);
    let ready = match &input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().enumerate().map(|(index,_)| field_access(&data.fields,index));
            quote! { true #(&& #value_crate::retirement::RetireOwned::retirement_ready(&self.#fields))* }
        }
        Data::Enum(data) if data.variants.is_empty() => quote! { match *self {} },
        Data::Enum(data) => {
            let arms = data.variants.iter().map(|variant| {
                let (pattern, values) = destructure_fields(name,&variant.fields,Some(&variant.ident));
                quote! { #pattern => true #(&& #value_crate::retirement::RetireOwned::retirement_ready(#values))* }
            });
            quote! { match self { #(#arms),* } }
        }
        Data::Union(_) => unreachable!(),
    };
    let original_fields=original_record_fields(input,&value_crate,&generics)?;
    let shared_methods=original_allocation_retirement(input,&value_crate,&generics,true)?;
    let boxed_methods=original_allocation_retirement(input,&value_crate,&generics,false)?;
    Ok(quote! {
        #cursor_declaration
        #original_fields
        impl #impl_generics #value_crate::retirement::RetireOwned for #name #ty_generics #where_clause {
            fn retirement_ready(&self)->bool { #ready }
            type RetirementCursor=#cursor_type;
            fn retirement_seed(self)->Self::RetirementCursor{#body}
            fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(::std::mem::size_of::<Self::RetirementCursor>())}
            #shared_methods
            #boxed_methods
            fn controlled_retirement_supported() -> bool { #value_crate::retirement::support::controlled_retirement_supported::<Self>() }
            fn controlled_retirement_supported_in(path: &#value_crate::retirement::RetirementSupport<'_>) -> bool { true #(&& path.supports::<#components>())* }
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

struct CloneRow{ty:Type,row:syn::Ident,project:TokenStream}
struct CloneVariant{rows:Vec<usize>,construct:TokenStream}

/// 🧬️ Expands one struct into the row table of the framework record clone cursor.
fn expand_struct(input: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    let name = &input.ident;
    let rows = fields.iter().enumerate().map(|(index, field)| {
        let access = field_access(fields, index);
        CloneRow{ty:field.ty.clone(),row:format_ident!("field_{index}"),project:quote!(source.project(#index+1,|source|&source.#access))}
    }).collect::<Vec<_>>();
    let idents = rows.iter().map(|row| row.row.clone()).collect::<Vec<_>>();
    let construct = construct_fields(name, fields, &idents, None);
    let variants = vec![CloneVariant{rows:(0..rows.len()).collect(),construct}];
    Ok(expand_record(input, &value_crate, &rows, &variants, quote!(0usize), bitwise_methods(input, fields)?))
}

fn expand_enum(input: &DeriveInput, data: &syn::DataEnum) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    let name = &input.ident;
    let mut rows = Vec::new();
    let mut variants = Vec::new();
    let mut selectors = Vec::new();
    for (variant_index, variant) in data.variants.iter().enumerate() {
        let (pattern, bindings) = borrowed_variant_pattern(name, variant, variant_index);
        let variant_name = &variant.ident;
        let any = match &variant.fields {
            Fields::Named(_) => quote!(#name::#variant_name{..}),
            Fields::Unnamed(_) => quote!(#name::#variant_name(..)),
            Fields::Unit => quote!(#name::#variant_name),
        };
        selectors.push(quote!(#any => #variant_index));
        let first = rows.len();
        for (field_index, field) in variant.fields.iter().enumerate() {
            let binding = &bindings[field_index];
            let key = quote!((#variant_index+1)*1024+#field_index+1);
            rows.push(CloneRow{ty:field.ty.clone(),row:format_ident!("variant_{variant_index}_field_{field_index}"),project:quote!(source.project(#key,|source|match source{#pattern=>#binding,_=>unreachable!("validated enum variant")}))});
        }
        let idents = rows[first..].iter().map(|row| row.row.clone()).collect::<Vec<_>>();
        variants.push(CloneVariant{rows:(first..rows.len()).collect(),construct:construct_fields(name, &variant.fields, &idents, Some(&variant.ident))});
    }
    let select = if selectors.is_empty() { quote!(match *source {}) } else { quote!(match source { #(#selectors),* }) };
    Ok(expand_record(input, &value_crate, &rows, &variants, select, TokenStream::new()))
}

/// 🏗️ Emits the row table, live-variant selector, constructor and two thin row dispatchers of `retained_clone::record`; the machine itself is shared.
fn expand_record(input: &DeriveInput, vc: &syn::Path, rows: &[CloneRow], variants: &[CloneVariant], select: TokenStream, bitwise: TokenStream) -> TokenStream {
    let name = &input.ident;
    let visibility = &input.vis;
    let rows_name = format_ident!("__{name}RetainedCloneRows");
    let generics = clone_generics(input, vc);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let owner = quote!(#name #ty_generics);
    let rc = quote!(#vc::retained_clone);
    let record = quote!(#rc::record);
    let count = rows.len();
    let idents = rows.iter().map(|row| &row.row).collect::<Vec<_>>();
    let types = rows.iter().map(|row| &row.ty);
    let projects = rows.iter().map(|row| &row.project).collect::<Vec<_>>();
    let indices = (0..count).collect::<Vec<_>>();
    let counts = variants.iter().map(|variant| variant.rows.len());
    let finish = variants.iter().enumerate().map(|(index, variant)| {let construct = &variant.construct;quote!(#index=>Ok(#construct))});
    quote! {
        #[doc(hidden)]
        #visibility struct #rows_name #impl_generics #where_clause {#(#idents: #record::RetainedRecordRow<#types>,)* __owner: ::core::marker::PhantomData<fn()->#owner>}
        const _: () = {
            impl #impl_generics Default for #rows_name #ty_generics #where_clause {fn default()->Self{Self{#(#idents:Default::default(),)* __owner: ::core::marker::PhantomData}}}
            impl #impl_generics #record::RetainedRecordRows<#owner> for #rows_name #ty_generics #where_clause {
                const ROWS: usize = #count;
                const VARIANTS: &'static [usize] = &[#(#counts),*];
                #[allow(unused_variables)]
                fn variant(source:&#owner)->usize{#select}
                #[allow(unused_variables)]
                fn query(&self,row:usize,source:Option<#rc::RetainedCloneRef<'_,#owner>>,visit:&mut dyn FnMut(&dyn #record::RetainedRecordQuery)){match row{#(#indices=>visit(&#record::RetainedRecordField::new(&self.#idents,source.map(|source|#projects))),)*_=>{}}}
                #[allow(unused_variables)]
                fn drive(&mut self,row:usize,source:Option<#rc::RetainedCloneRef<'_,#owner>>,visit:&mut dyn FnMut(&mut dyn #record::RetainedRecordDrive)){match row{#(#indices=>visit(&mut #record::RetainedRecordFieldMut::new(&mut self.#idents,source.map(|source|#projects))),)*_=>{}}}
                fn finish(&mut self,variant:usize)->::core::result::Result<#owner,#vc::ValueError>{match variant{#(#finish,)*_=>Err(#vc::ValueError::invariant("record clone state is invalid"))}}
            }
            impl #impl_generics #rc::RetainedClone for #name #ty_generics #where_clause {
                type Cursor = #record::RetainedRecordCursor<#owner,#rows_name #ty_generics>;
                fn retained_clone_cursor() -> Self::Cursor { Default::default() }
                #bitwise
            }
        };
    }
}

/// 🚚️ Reads `#[retained_clone(bitwise)]`: the opt-in that a `Copy` struct of native-copy fields exposes its own native Copy witness to vectors and arrays.
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
        fn native_copy() -> Option<#value_crate::retained_clone::RetainedCopy<Self>> { (true #(&& <#types as #value_crate::retained_clone::RetainedClone>::native_copy().is_some())*).then(#value_crate::retained_clone::RetainedCopy::new) }
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

fn construct_fields(name: &syn::Ident, fields: &Fields, rows: &[syn::Ident], variant: Option<&syn::Ident>) -> TokenStream {
    let prefix = variant.map_or_else(|| quote!(#name), |variant| quote!(#name::#variant));
    let owners = rows.iter().map(|row|quote!(self.#row.take_value()?)).collect::<Vec<_>>();
    match fields {
        Fields::Named(named) => {
            let names = named.named.iter().filter_map(|field| field.ident.as_ref());
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
