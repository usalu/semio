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

fn original_allocation_retirement(input:&DeriveInput,value_crate:&syn::Path,generics:&Generics,shared:bool)->syn::Result<(TokenStream,TokenStream)>{
 let Data::Struct(data)=&input.data else{return Ok((quote!(),quote!()))};if shared&&crate::component::declared_send_only(input)?||!shared&&!crate::component::declared_boxed_original(input)?{return Ok((quote!(),quote!()))}require_aligned_fields(input,"RetireOwned")?;
 let name=&input.ident;let cursor=if shared{format_ident!("__{name}SharedOriginalRetirement")}else{format_ident!("__{name}BoxedOriginalRetirement")};let(_,types,_)=generics.split_for_impl();let mut owned_generics=generics.clone();if shared{owned_generics.make_where_clause().predicates.push(syn::parse_quote!(#name #types:Sync));}let(implementation,types,conditions)=owned_generics.split_for_impl();
 let owner=if shared{quote!(#value_crate::retirement::arc_custody::original::SharedOriginal<#name #types>)}else{quote!(#value_crate::retirement::original_box::BoxOriginal<#name #types>)};
 let demands=data.fields.iter().enumerate().map(|(index,field)|{let access=field_access(&data.fields,index);quote!(#index=>{let field=unsafe{original.field(|root|::std::ptr::addr_of!((*root).#access))};#value_crate::retirement::record::field_birth_demand(field)})});
 let steps=data.fields.iter().enumerate().map(|(index,field)|{let access=field_access(&data.fields,index);let ty=&field.ty;quote!(#index=>{let field=unsafe{original.field(|root|::std::ptr::addr_of!((*root).#access))}as *const #ty;let turn=unsafe{#value_crate::retirement::record::admit_field_with(field,effect,||original.take_field(|root|::std::ptr::addr_of_mut!((*root).#access)))};if matches!(&turn,#value_crate::retirement::RetirementStep::ChildProgress(..)){self.ordinal+=1}turn})});
 let native_original_admission=if shared{quote!{fn admit_shared_native(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::RetainedWorkOutcome<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>>{let grant=effect.grant();let mut work=0;let result=(||{let Some(source)=original.as_ref()else{return Ok(None)};if !source.is_initialized(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"partially transferred shared original requires its existing cursor"))}let capacity_bytes=::std::mem::size_of::<#cursor #types>();let demand=#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>()+capacity_bytes+2*::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<Option<#value_crate::retirement::RetirementStep>>(),capacity_bytes,depth:1,..Default::default()};if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::OwnershipLimit,"shared original cursor birth is not funded"));}if effect.remaining_work_units()?==0{return Ok(None)}if effect.actual().is_some(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"original cursor allocator marker occupied"))}let layout=::std::alloc::Layout::new::<#cursor #types>();work=1;if !effect.performed(1)?{return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::WorkLimit,"original cursor allocator exceeded its Work"))}*effect.actual()=Some(0);let raw=unsafe{::std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}let Some(pointer)=::std::ptr::NonNull::new(raw.cast::<#cursor #types>())else{return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::AllocationFailed,"original cursor allocation failed").with_retained_progress(#value_crate::RetainedCloneProgress{copied_items:1,copied_bytes: ::std::mem::size_of::<Option<usize>>(),..Default::default()}))};let mut cursor=unsafe{pointer.as_ptr().write(#cursor{original:None,ordinal:0});Box::from_raw(pointer.as_ptr())};cursor.original=original.take();Ok(Some((cursor as Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity_bytes,..Default::default()})))})();#value_crate::RetainedWorkOutcome{result,work}}}}else{quote!{fn admit_boxed_native(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::RetainedWorkOutcome<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>>{let grant=effect.grant();let mut work=0;let result=(||{let Some(source)=original.as_ref()else{return Ok(None)};if !source.is_initialized(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"partially transferred original Box requires its existing cursor"));}let demand=#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>()+::std::mem::size_of::<#cursor #types>()+2*::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<Option<#value_crate::retirement::RetirementStep>>(),capacity_bytes: ::std::mem::size_of::<#cursor #types>(),depth:1,..Default::default()};if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::OwnershipLimit,"boxed original cursor birth is not funded"));}if effect.remaining_work_units()?==0{return Ok(None)}if effect.actual().is_some(){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"original cursor allocator marker occupied"))}let layout=::std::alloc::Layout::new::<#cursor #types>();work=1;if !effect.performed(1)?{return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::WorkLimit,"original cursor allocator exceeded its Work"))}*effect.actual()=Some(0);let raw=unsafe{::std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}let Some(pointer)=::std::ptr::NonNull::new(raw.cast::<#cursor #types>())else{return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::AllocationFailed,"original cursor allocation failed").with_retained_progress(#value_crate::RetainedCloneProgress{copied_items:1,copied_bytes: ::std::mem::size_of::<Option<usize>>(),..Default::default()}))};let mut cursor=unsafe{pointer.as_ptr().write(#cursor{original:None,ordinal:0});Box::from_raw(pointer.as_ptr())};cursor.original=original.take();Ok(Some((cursor as Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})))})();#value_crate::RetainedWorkOutcome{result,work}}}};
 let declaration=quote!{
 struct #cursor #implementation #conditions{original:Option<#owner>,ordinal:usize}
 impl #implementation #cursor #types #conditions{#native_original_admission}
 impl #implementation #value_crate::retirement::RetirementCursor for #cursor #types #conditions{
   fn next_computational_work(&self,_:usize)->::core::result::Result<u64,#value_crate::ValueError>{Ok(u64::from(self.original.is_some()))}
  fn next_demand(&self,_body:usize)->::core::result::Result<#value_crate::RetirementDemand,#value_crate::ValueError>{let Some(original)=self.original.as_ref()else{return Ok(Default::default())};match self.ordinal{#(#demands,)*_=>Ok(#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>(),release_bytes:<#owner>::allocation_bytes(),depth:1,..Default::default()})}}
  fn close_into<'reply>(&mut self,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::retained_clone::OriginalEffectReply<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::retained_clone::OriginalEffectReplyComplete<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{if effect.invocation().is_none(){return Err(#value_crate::ValueError::invariant("original native field retirement requires its actual invocation"))}let step=(||{let grant=effect.grant();let demand=match <Self as #value_crate::retirement::RetirementCursor>::next_demand(self,body){Ok(demand)=>demand,Err(error)=>return #value_crate::retirement::RetirementStep::Failure(error.into(),0)};if !#value_crate::retained_clone::retained_clone_grant_admits(grant,demand){return #value_crate::retirement::RetirementStep::BudgetExhausted;}let Some(original)=self.original.as_mut()else{return #value_crate::retirement::RetirementStep::Complete(0)};match self.ordinal{#(#steps,)*_=>{let work=match effect.performed(1){Ok(true)=>1,Ok(false)=>return #value_crate::retirement::RetirementStep::Failure(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::WorkLimit,"original header release exceeds its Work ceiling").into(),1),Err(error)=>return #value_crate::retirement::RetirementStep::Failure(error.into(),0)};let mut receipt=unsafe{original.release_empty(grant)};if receipt.copied_items==0{return #value_crate::retirement::RetirementStep::Progress(receipt,1);}self.original=None;receipt.copied_bytes+=::std::mem::size_of::<Option<#owner>>();#value_crate::retirement::RetirementStep::Progress(receipt,work)}}})();Ok(match step{#value_crate::retirement::RetirementStep::Failure(error,_)=>reply.complete(Err(error)),step=>reply.complete(Ok(step))})}
  fn terminal_is_empty(&self)->bool{self.original.is_none()}
  fn next_work_byte_demand(&self)->::core::result::Result<usize,#value_crate::ValueError>{Ok(<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,0)?.copy_bytes)}
  fn next_birth_bytes(&self,body:usize)->Option<usize>{<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,body).ok().map(|d|d.capacity_bytes)}
  fn next_close_byte_demand(&self)->Option<usize>{<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,0).ok().map(|d|d.release_bytes)}
  fn next_depth_demand(&self)->::core::result::Result<usize,#value_crate::ValueError>{Ok(<Self as #value_crate::retirement::RetirementCursor>::next_demand(self,0)?.depth)}
  fn terminal_release_bytes(&self)->Option<usize>{self.original.is_none().then_some(::std::mem::size_of::<Self>())}
 }
 };
 let no_drop=quote!{trait OriginalWithoutDrop<A>{fn verify(){}}impl<T:?Sized> OriginalWithoutDrop<()> for T{}#[allow(drop_bounds)]impl<T:?Sized+Drop> OriginalWithoutDrop<u8> for T{}let _=<Self as OriginalWithoutDrop<_>>::verify;};
 let methods=if shared{quote!{
  fn shared_original_demands(&self)->Result<Option<#value_crate::RetirementDemand>,#value_crate::ValueError> where Self:Sync{#no_drop if !#value_crate::retirement::RetireOwned::retirement_ready(self){return Err(#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::UnsupportedOwner,"shared original requires source recovery"))}Ok(Some(#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>()+::std::mem::size_of::<#cursor #types>()+2*::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<Option<#value_crate::retirement::RetirementStep>>(),capacity_bytes: ::std::mem::size_of::<#cursor #types>(),depth:1,..Default::default()}))}
  fn shared_original_work_demand(&self)->Result<Option<u64>,#value_crate::ValueError> where Self:Sync{Ok(Some(1))}
  fn shared_original_retirement(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::RetainedWorkOutcome<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>> where Self:Sync{<#cursor #types>::admit_shared_native(original,effect)}
  fn native_shared_original_admission()->Option<#value_crate::retirement::NativeSharedRetirementAdmission<Self>> where Self:Sync{Some(unsafe{#value_crate::retirement::NativeSharedRetirementAdmission::new(<#cursor #types>::admit_shared_native)})}
 }}else{quote!{
  fn boxed_original_demands(&self)->::core::result::Result<Option<#value_crate::RetirementDemand>,#value_crate::ValueError>{#no_drop Ok(Some(#value_crate::RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>()+::std::mem::size_of::<#cursor #types>()+2*::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<Option<#value_crate::retirement::RetirementStep>>(),capacity_bytes: ::std::mem::size_of::<#cursor #types>(),depth:1,..Default::default()}))}
  fn boxed_original_work_demand(&self)->Result<Option<u64>,#value_crate::ValueError>{Ok(Some(1))}
  fn boxed_original_retirement(original:&mut Option<#owner>,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::RetainedWorkOutcome<Option<(Box<dyn #value_crate::retirement::RetirementCursor>,#value_crate::retained_clone::RetainedCloneProgress)>>{<#cursor #types>::admit_boxed_native(original,effect)}
  fn native_boxed_original_admission()->Option<#value_crate::retirement::NativeBoxedRetirementAdmission<Self>>{Some(unsafe{#value_crate::retirement::NativeBoxedRetirementAdmission::new(<#cursor #types>::admit_boxed_native)})}
 }};
 Ok((declaration,methods))
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
    let variants=rows.iter().map(|(variant,fields,_,_)|{let types=fields.iter().map(|field|&field.ty);quote!(#variant(#value_crate::retirement_fields_type!(#(#types),*)))});
    let arms=rows.iter().map(|(variant,_,pattern,values)|quote!(#pattern=>#cursor_name{inner:#inner_name::#variant(#value_crate::retirement_fields_seed!(#(#values),*))}));
    let (cursor_declaration,cursor_type,body)={
        let names=rows.iter().map(|(variant,_,_,_)|variant).collect::<Vec<_>>();
        let tail=if names.is_empty(){quote!(_=>unreachable!("uninhabited retirement"))}else{quote!()};
        let declaration=quote!{
          #[doc(hidden)] #visibility struct #cursor_name #impl_generics #where_clause{inner:#inner_name #ty_generics}
          enum #inner_name #impl_generics #where_clause{#(#variants),*}
          impl #impl_generics #value_crate::retirement::RetirementCursor for #cursor_name #ty_generics #where_clause{
            fn terminal_is_empty(&self)->bool{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::terminal_is_empty(value)),*#tail}}
            fn next_demand(&self,body:usize)->Result<#value_crate::RetirementDemand,#value_crate::ValueError>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_demand(value,body)),*#tail}}
            fn next_computational_work(&self,body:usize)->Result<u64,#value_crate::ValueError>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_computational_work(value,body)),*#tail}}
            fn close_into<'reply>(&mut self,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::retained_clone::OriginalEffectReply<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::retained_clone::OriginalEffectReplyComplete<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{match &mut self.inner{#(#inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::close_into(value,body,effect,reply)),*#tail}}
            fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::retained_clone::OriginalEffectReply<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::retained_clone::OriginalEffectReplyComplete<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{match &mut self.inner{#(#inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::recover_close_into(value,body,effect,reply)),*#tail}}
            fn close_original_into<'reply>(&mut self,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::retained_clone::OriginalEffectReply<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::retained_clone::OriginalEffectReplyComplete<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{match &mut self.inner{#(#inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::close_original_into(value,original,body,effect,reply)),*#tail}}
            fn recover_close_original_into<'reply>(&mut self,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,reply:#value_crate::retained_clone::OriginalEffectReply<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>)->Result<#value_crate::retained_clone::OriginalEffectReplyComplete<'reply,#value_crate::retirement::RetirementStep,#value_crate::OriginalRetirementFailure>,#value_crate::ValueError>{match &mut self.inner{#(#inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::recover_close_original_into(value,original,body,effect,reply)),*#tail}}
            fn close_step_controlled(&mut self,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>,control:&mut dyn #value_crate::OriginalRetirementControl)->#value_crate::retirement::RetirementStep{match &mut self.inner{#(#inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::close_step_controlled(value,body,effect,control)),*#tail}}
            fn requires_original_control(&self)->bool{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::requires_original_control(value)),*#tail}}
            fn close_original_step(&mut self,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut #value_crate::RetainedAllocationEffect<'_>)->#value_crate::retirement::RetirementStep{match &mut self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::close_original_step(value,original,body,effect)),*#tail}}
            fn requires_original_birth(&self)->bool{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::requires_original_birth(value)),*#tail}}
            fn close_original_birth_step(&mut self,original:&mut #value_crate::OriginalRetirementCloseLoan<'_>,grant:#value_crate::RetainedCloneGrant)->#value_crate::retirement::RetirementStep{match &mut self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::close_original_birth_step(value,original,grant)),*#tail}}
            fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(::std::mem::size_of::<Self>())}
            fn next_birth_bytes(&self,body:usize)->Option<usize>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_birth_bytes(value,body)),*#tail}}
            fn next_close_byte_demand(&self)->Option<usize>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_close_byte_demand(value)),*#tail}}
            fn next_depth_demand(&self)->Result<usize,#value_crate::ValueError>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_depth_demand(value)),*#tail}}
            fn next_work_byte_demand(&self)->Result<usize,#value_crate::ValueError>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_work_byte_demand(value)),*#tail}}
            fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,#value_crate::ValueError>{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::next_selected_work_byte_demand(value,body)),*#tail}}
            fn allows_admitted_narrow_work(&self)->bool{match &self.inner{#( #inner_name::#names(value)=>#value_crate::retirement::RetirementCursor::allows_admitted_narrow_work(value)),*#tail}}
          }
        };(declaration,quote!(#cursor_name #ty_generics),quote!(match self{#(#arms),*}))
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
    let (shared_declaration,shared_methods)=original_allocation_retirement(input,&value_crate,&generics,true)?;
    let (boxed_declaration,boxed_methods)=original_allocation_retirement(input,&value_crate,&generics,false)?;
    Ok(quote! {
        #cursor_declaration
        #shared_declaration
        #boxed_declaration
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

struct CloneRow{ty:Type,cursor:syn::Ident,value:syn::Ident,variant:usize,local:usize,project:TokenStream}
struct CloneVariant{rows:Vec<usize>,construct:TokenStream}

/// 🧬️ Expands one struct into the reply-shaped native clone cursor of the retained clone contract.
fn expand_struct(input: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let value_crate = crate::component::declared_value_crate(input)?;
    let name = &input.ident;
    let rows = fields.iter().enumerate().map(|(index, field)| {
        let access = field_access(fields, index);
        CloneRow{ty:field.ty.clone(),cursor:format_ident!("field_{index}_cursor"),value:format_ident!("field_{index}_value"),variant:0,local:index,project:quote!(source.project(#index+1,|source|&source.#access))}
    }).collect::<Vec<_>>();
    let values = rows.iter().map(|row| row.value.clone()).collect::<Vec<_>>();
    let construct = construct_fields(&value_crate, name, fields, &values, None);
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
            rows.push(CloneRow{ty:field.ty.clone(),cursor:format_ident!("variant_{variant_index}_field_{field_index}_cursor"),value:format_ident!("variant_{variant_index}_field_{field_index}_value"),variant:variant_index,local:field_index,project:quote!(source.project(#key,|source|match source{#pattern=>#binding,_=>unreachable!("validated enum variant")}))});
        }
        let indices = (first..rows.len()).collect::<Vec<_>>();
        let values = indices.iter().map(|index| rows[*index].value.clone()).collect::<Vec<_>>();
        variants.push(CloneVariant{rows:indices,construct:construct_fields(&value_crate, name, &variant.fields, &values, Some(&variant.ident))});
    }
    let select = if selectors.is_empty() { quote!(match *source.get() {}) } else { quote!(match source.get() { #(#selectors),* }) };
    Ok(expand_record(input, &value_crate, &rows, &variants, select, TokenStream::new()))
}

fn expand_record(input: &DeriveInput, vc: &syn::Path, rows: &[CloneRow], variants: &[CloneVariant], select: TokenStream, bitwise: TokenStream) -> TokenStream {
    let name = &input.ident;
    let visibility = &input.vis;
    let cursor_name = format_ident!("__{name}RetainedCloneCursor");
    let generics = clone_generics(input, vc);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let owner = quote!(#name #ty_generics);
    let rc = quote!(#vc::retained_clone);
    let invariant = |text: &str| quote!(#vc::ValueError::invariant(#text));
    let n = rows.len();
    let done = 4 * n;
    let out = 4 * n + 1;
    let cursor_fields = rows.iter().map(|row| {let (cursor, value, ty) = (&row.cursor, &row.value, &row.ty);quote!(#cursor: #rc::RetainedFieldCursor<#ty>, #value: Option<#ty>)});
    let cursor_init = rows.iter().map(|row| {let (cursor, value) = (&row.cursor, &row.value);quote!(#cursor: Default::default(), #value: None)});
    let plan_rows = rows.iter().enumerate().map(|(g, row)| {
        let (cursor, value, variant, local) = (&row.cursor, &row.value, row.variant, row.local);
        let (begin, close, take, advance) = (4 * g + 2, 4 * g + 3, 4 * g + 1, 4 * g);
        quote!((#variant,#local)=>Ok(if self.#value.is_some(){if #rc::RetainedCloneCursor::terminal_is_empty(&self.#cursor){#done}else if #rc::RetainedCloneCursor::close_started(&self.#cursor){#close}else{#begin}}else if #rc::RetainedCloneCursor::output_ready(&self.#cursor){#take}else{#advance}))
    });
    let plan_out = variants.iter().enumerate().map(|(index, variant)| {let count = variant.rows.len();quote!((#index,#count)=>Ok(#out))});
    let output_demand = variants.iter().enumerate().map(|(index, variant)| {let tys = variant.rows.iter().map(|row| &rows[*row].ty);quote!(#index=>RetirementDemand{copy_bytes: ::std::mem::size_of::<Option<#owner>>()+::std::mem::size_of::<usize>() #(+::std::mem::size_of::<Option<#tys>>())*,depth:1,..Default::default()})});
    let finish = variants.iter().enumerate().map(|(index, variant)| {
        let present = variant.rows.iter().map(|row| {let value = &rows[*row].value;quote!(self.#value.is_some())});
        let construct = &variant.construct;
        quote!(#index=>{if !(true #(&&#present)*){return Err(#vc::ValueError::invariant("record original output field missing"))}#vc::retirement::record_retirement_work(effect,1).map_err(|(error,_)|error)?;self.output=Some(#construct);self.phase+=1;return Ok(reply.complete(Ok(#rc::RetainedCloneStep::Complete(#rc::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))})
    });
    let demand_arms = rows.iter().enumerate().map(|(g, row)| {
        let (cursor, ty, project) = (&row.cursor, &row.ty, &row.project);
        let (advance, take, begin, close) = (4 * g, 4 * g + 1, 4 * g + 2, 4 * g + 3);
        quote!(#advance=>#rc::RetainedCloneCursor::advance_demands(&self.#cursor,#project,body,maximum),#take=>#rc::RetainedCloneCursor::take_demand(&self.#cursor),#begin|#close=>#rc::clone_cursor_close_demand::<#ty>(&self.#cursor,body))
    });
    let work_arms = rows.iter().enumerate().map(|(g, row)| {
        let (cursor, project) = (&row.cursor, &row.project);
        let (advance, take, begin, close) = (4 * g, 4 * g + 1, 4 * g + 2, 4 * g + 3);
        quote!(#advance=>return #rc::RetainedCloneCursor::advance_work_demand(&self.#cursor,#project,body,maximum),#take=>#rc::RetainedCloneCursor::take_work_demand(&self.#cursor)?,#begin=>#rc::RetainedCloneCursor::begin_close_work_demand(&self.#cursor)?,#close=>#rc::RetainedCloneCursor::close_work_demand(&self.#cursor,body)?)
    });
    let selection_arms = rows.iter().enumerate().map(|(g, row)| {
        let (cursor, ty, project) = (&row.cursor, &row.ty, &row.project);
        let (advance, take, begin, close) = (4 * g, 4 * g + 1, 4 * g + 2, 4 * g + 3);
        quote!(#advance=>return #rc::RetainedCloneCursor::advance_selection(&self.#cursor,#project,body,maximum),
            #take=>(#rc::RetainedCloneCursor::take_demand(&self.#cursor)?,#rc::RetainedCloneCursor::take_work_demand(&self.#cursor)?),
            #begin=>(#rc::clone_cursor_close_demand::<#ty>(&self.#cursor,body)?,#rc::RetainedCloneCursor::begin_close_work_demand(&self.#cursor)?),
            #close=>(#rc::clone_cursor_close_demand::<#ty>(&self.#cursor,body)?,#rc::RetainedCloneCursor::close_work_demand(&self.#cursor,body)?))
    });
    let advance_routes = rows.iter().enumerate().map(|(g, row)| {
        let (cursor, value, project) = (&row.cursor, &row.value, &row.project);
        quote!(#g=>{
            if code%4==1{self.route=Some(key);if self.child_reply.is_none(){let child=#rc::OriginalEffectReply::borrow(&mut self.child_reply)?;if recover{#rc::RetainedCloneCursor::recover_take(&mut self.#cursor,&mut self.#value,effect,child)?;}else{#rc::RetainedCloneCursor::take(&mut self.#cursor,&mut self.#value,effect,child)?;}}let Some(result)=self.child_reply.take()else{return Ok(reply.pending())};let mut scope=reply.progress().scope(&mut self.route,key)?;#rc::OriginalEffectReplyScope::borrow(&mut scope)?.complete(result.map_err(Into::into));return Ok(scope.forwarded())}
            let mut scope=reply.progress().scope(&mut self.route,key)?;let child=#rc::OriginalEffectReplyScope::borrow(&mut scope)?;
            match code%4{
                0=>{let projected=#project;if recover{#rc::RetainedCloneCursor::recover_advance_into(&mut self.#cursor,projected,body,maximum,effect,child)?;}else{#rc::RetainedCloneCursor::advance_into(&mut self.#cursor,projected,body,maximum,effect,child)?;}},
                2=>{if recover{#rc::RetainedCloneCursor::recover_begin_close_into(&mut self.#cursor,effect,child)?;}else{#rc::RetainedCloneCursor::begin_close_into(&mut self.#cursor,effect,child)?;}},
                3=>{if recover{#rc::RetainedCloneCursor::recover_close_into(&mut self.#cursor,body,effect,child)?;}else{#rc::RetainedCloneCursor::close_into(&mut self.#cursor,body,effect,child)?;}},
                _=>return Err(#vc::ValueError::invariant("record child route invalid")),
            }
            Ok(scope.forwarded())
        })
    });
    let close_find = rows.iter().enumerate().map(|(g, row)| {let cursor = &row.cursor;quote!(if child.is_none()&&!#rc::RetainedCloneCursor::terminal_is_empty(&self.#cursor){child=Some(100+2*#g+usize::from(#rc::RetainedCloneCursor::close_started(&self.#cursor)));})});
    let close_values = rows.iter().map(|row| {let value = &row.value;quote!(if self.#value.is_some(){let turn=self.close.begin_granted(&mut self.#value,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(#rc::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))})});
    let close_routes = rows.iter().enumerate().map(|(g, row)| {
        let cursor = &row.cursor;
        let (begin, close) = (100 + 2 * g, 101 + 2 * g);
        quote!(code if code==#begin=>{if recover{#rc::RetainedCloneCursor::recover_begin_close_into(&mut self.#cursor,effect,child)?;}else{#rc::RetainedCloneCursor::begin_close_into(&mut self.#cursor,effect,child)?;}},
            code if code==#close=>{match original{Some(original)=>if recover{#rc::RetainedCloneCursor::recover_close_original_into(&mut self.#cursor,original,body,effect,child)?;}else{#rc::RetainedCloneCursor::close_original_into(&mut self.#cursor,original,body,effect,child)?;},None=>if recover{#rc::RetainedCloneCursor::recover_close_into(&mut self.#cursor,body,effect,child)?;}else{#rc::RetainedCloneCursor::close_into(&mut self.#cursor,body,effect,child)?;}}},)
    });
    let demand_children = rows.iter().map(|row| {let (cursor, ty) = (&row.cursor, &row.ty);quote!(if !#rc::RetainedCloneCursor::terminal_is_empty(&self.#cursor){return #rc::clone_cursor_close_demand::<#ty>(&self.#cursor,body)})});
    let demand_values = rows.iter().map(|row| {let (value, ty) = (&row.value, &row.ty);quote!(if self.#value.is_some(){return self.close.next_owner_demands::<#ty>(true,&None,body)})});
    let work_children = rows.iter().map(|row| {let cursor = &row.cursor;quote!(if !#rc::RetainedCloneCursor::terminal_is_empty(&self.#cursor){return if #rc::RetainedCloneCursor::close_started(&self.#cursor){#rc::RetainedCloneCursor::close_work_demand(&self.#cursor,body)}else{#rc::RetainedCloneCursor::begin_close_work_demand(&self.#cursor)}})});
    let work_values = rows.iter().map(|row| {let (value, ty) = (&row.value, &row.ty);quote!(if self.#value.is_some(){return self.close.next_owner_work::<#ty>(true,&None,body)})});
    let cursor_idents = rows.iter().map(|row| row.cursor.clone()).collect::<Vec<_>>();
    let empty = rows.iter().map(|row| {let (cursor, value) = (&row.cursor, &row.value);quote!(&&self.#value.is_none()&&#rc::RetainedCloneCursor::terminal_is_empty(&self.#cursor))});
    let reply = quote!(#rc::OriginalEffectReply<'r,#rc::RetainedCloneStep,#vc::OriginalRetirementFailure>);
    let complete = quote!(::core::result::Result<#rc::OriginalEffectReplyComplete<'r,#rc::RetainedCloneStep,#vc::OriginalRetirementFailure>,#vc::ValueError>);
    let loan = quote!(#vc::OriginalRetirementCloseLoan<'_>);
    let err_phase = invariant("record clone state is invalid");
    let err_route = invariant("record advance route invalid");
    let err_close = invariant("record original close route invalid");
    let err_key = invariant("record child requires its original invocation");
    let err_key_changed = invariant("record original child invocation changed");
    let err_unstarted = invariant("record close has not started");
    quote! {
        #[doc(hidden)]
        #visibility struct #cursor_name #impl_generics #where_clause {
            phase: usize,
            output: Option<#owner>,
            source: Option<#rc::RetainedCloneBinding>,
            spent: bool,
            closing: bool,
            close: #rc::RetainedCloneClose,
            route: Option<(#rc::OriginalEffectInvocationIdentity,usize)>,
            selected: Option<(#vc::RetirementDemand,u64)>,
            child_reply: Option<::core::result::Result<#rc::RetainedCloneStep,#vc::ValueError>>,
            #(#cursor_fields),*
        }
        const _: () = {
            use #vc::RetirementDemand;
            impl #impl_generics Default for #cursor_name #ty_generics #where_clause {
                fn default() -> Self { Self { phase: 0, output: None, source: None, spent: false, closing: false, close: Default::default(), route: None, selected: None, child_reply: None, #(#cursor_init),* } }
            }
            impl #impl_generics #cursor_name #ty_generics #where_clause {
                fn variant(source:#rc::RetainedCloneRef<'_,#owner>)->usize{#select}
                fn key(&self,effect:&#vc::RetainedAllocationEffect<'_>,code:usize)->::core::result::Result<(#rc::OriginalEffectInvocationIdentity,usize),#vc::ValueError>{let key=(effect.invocation().ok_or_else(||#err_key)?.identity(),code);if self.route.is_some_and(|route|route!=key){return Err(#err_key_changed)}Ok(key)}
                fn metadata_demand()->RetirementDemand{RetirementDemand{copy_bytes: ::std::mem::size_of::<usize>(),depth:1,..Default::default()}}
                fn output_demand(variant:usize)->RetirementDemand{match variant{#(#output_demand,)*_=>Default::default()}}
                fn plan(&self,source:#rc::RetainedCloneRef<'_,#owner>)->::core::result::Result<usize,#vc::ValueError>{
                    match (Self::variant(source),self.phase){#(#plan_rows,)*#(#plan_out,)*_=>Err(#err_phase)}
                }
                fn advance_reply<'r>(&mut self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply,recover:bool)->#complete{
                    let code=if let Some((_,code))=self.route{code}else{
                        if recover{return Err(#vc::ValueError::invariant("record recovery has no original advance"))}
                        let demand=#rc::RetainedCloneCursor::advance_demands(self,source,body,maximum)?;let work=#rc::RetainedCloneCursor::advance_work_demand(self,source,body,maximum)?;
                        if work==0&&!#rc::RetainedCloneCursor::output_ready(self)||effect.remaining_work_units()?<work||!#rc::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(#rc::RetainedCloneStep::Progress(Default::default()))))}
                        if #rc::RetainedCloneCursor::output_ready(self){return Ok(reply.complete(Ok(#rc::RetainedCloneStep::Complete(Default::default()))))}
                        if self.source.is_none(){let turn=source.bind(&mut self.source,effect);return Ok(reply.complete(turn.result.map(|progress|#rc::RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(Into::into)))}
                        let code=self.plan(source)?;
                        if code==#out{match Self::variant(source){#(#finish,)*_=>return Err(#err_phase)}}
                        self.selected=Some((demand,work));
                        if code==#done{#vc::retirement::record_retirement_work(effect,1).map_err(|(error,_)|error)?;self.phase+=1;return Ok(reply.complete(Ok(#rc::RetainedCloneStep::Progress(#rc::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
                        code
                    };
                    let key=self.key(effect,code)?;
                    match code/4{#(#advance_routes,)*_=>Err(#err_route)}
                }
                fn close_reply<'r>(&mut self,original:Option<&mut #loan>,body:usize,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply,recover:bool)->#complete{
                    if !self.closing{return Err(#err_unstarted)}
                    let code=if let Some((_,code))=self.route{code}else{
                        if recover{return Err(#vc::ValueError::invariant("record recovery has no original close"))}
                        let demand=#rc::RetainedCloneCursor::close_demands(self,body)?;let work=#rc::RetainedCloneCursor::close_work_demand(self,body)?;
                        if effect.remaining_work_units()?<work||!#rc::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(#rc::RetainedCloneStep::Progress(Default::default()))))}
                        self.selected=Some((demand,work));
                        let mut child=None;#(#close_find)*
                        if let Some(code)=child{code}else if !self.close.is_empty(){200}else{
                            #(#close_values)*
                            if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(#rc::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
                            201
                        }
                    };
                    let key=self.key(effect,code)?;let reply=if code==201{reply}else{reply.progress()};let mut scope=reply.scope(&mut self.route,key)?;let child=#rc::OriginalEffectReplyScope::borrow(&mut scope)?;
                    match code{
                        #(#close_routes)*
                        200=>match original{Some(original)=>if recover{self.close.recover_step_original_into(original,body,effect,child)?;}else{self.close.step_original_into(original,body,effect,child)?;},None=>if recover{self.close.recover_step_into(body,effect,child)?;}else{self.close.step_into(body,effect,child)?;}},
                        201=>match original{Some(original)=>if recover{#rc::RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child)?;}else{#rc::RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child)?;},None=>if recover{#rc::RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child)?;}else{#rc::RetainedCloneBinding::close_into(&mut self.source,body,effect,child)?;}},
                        _=>return Err(#err_close),
                    }
                    Ok(scope.forwarded())
                }
            }
            impl #impl_generics #rc::RetainedCloneCursor<#owner> for #cursor_name #ty_generics #where_clause {
                fn native_terminal_release(&self)->Option<#rc::RetainedCursorRelease<Self>>{let terminal=|cursor:&Self|#rc::RetainedCloneCursor::terminal_is_empty(cursor) #(&&#rc::RetainedCloneCursor::native_terminal_release(&cursor.#cursor_idents).is_some_and(|release|release.validates(&cursor.#cursor_idents)))*;terminal(self).then(||unsafe{#rc::RetainedCursorRelease::native(terminal)})}
                fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
                fn close_started(&self)->bool{self.closing}
                fn advance_demands(&self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64)->::core::result::Result<#vc::RetirementDemand,#vc::ValueError>{
                    if self.route.is_some(){return self.selected.map(|selected|selected.0).ok_or_else(||#vc::ValueError::invariant("record selected quote missing"))}
                    if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}
                    if self.closing||self.spent{return Err(#vc::ValueError::invariant("record cursor is closing or spent"))}
                    if #rc::RetainedCloneCursor::output_ready(self){return Ok(Default::default())}
                    let code=self.plan(source)?;
                    if code==#done{return Ok(Self::metadata_demand())}
                    if code==#out{return Ok(Self::output_demand(Self::variant(source)))}
                    match code{#(#demand_arms,)*_=>Err(#err_phase)}
                }
                fn advance_work_demand(&self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64)->::core::result::Result<u64,#vc::ValueError>{
                    if self.route.is_some(){return self.selected.map(|selected|selected.1).ok_or_else(||#vc::ValueError::invariant("record selected work missing"))}
                    source.binding_demand(&self.source)?;
                    if #rc::RetainedCloneCursor::output_ready(self){return Ok(0)}
                    if self.source.is_none(){return source.binding_work_demand(&self.source)}
                    let code=self.plan(source)?;
                    let units=if code==#done||code==#out{1}else{match code{#(#work_arms,)*_=>return Err(#err_phase)}};
                    Ok(if units<=maximum{units}else{0})
                }
                fn advance_selection(&self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64)->::core::result::Result<#rc::RetainedCloneSelection,#vc::ValueError>{
                    if self.route.is_some(){let selected=self.selected.ok_or_else(||#vc::ValueError::invariant("record selected quote missing"))?;return Ok(#rc::RetainedCloneSelection::new(selected.0,selected.1))}
                    if let Some(demand)=source.binding_demand(&self.source)?{return Ok(#rc::RetainedCloneSelection::new(demand,source.binding_work_demand(&self.source)?))}
                    if self.closing||self.spent{return Err(#vc::ValueError::invariant("record cursor is closing or spent"))}
                    if #rc::RetainedCloneCursor::output_ready(self){return Ok(#rc::RetainedCloneSelection::new(Default::default(),0))}
                    let code=self.plan(source)?;
                    let (demand,units)=if code==#done{(Self::metadata_demand(),1)}else if code==#out{(Self::output_demand(Self::variant(source)),1)}else{match code{#(#selection_arms,)*_=>return Err(#err_phase)}};
                    Ok(#rc::RetainedCloneSelection::new(demand,if units<=maximum{units}else{0}))
                }
                fn captured_advance_selection(&self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64)->::core::result::Result<#rc::RetainedCloneSelection,#vc::ValueError>{#rc::RetainedCloneCursor::advance_selection(self,source,body,maximum)}
                fn advance_into<'r>(&mut self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.advance_reply(source,body,maximum,effect,reply,false)}
                fn recover_advance_into<'r>(&mut self,source:#rc::RetainedCloneRef<'_,#owner>,body:usize,maximum:u64,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.advance_reply(source,body,maximum,effect,reply,true)}
                fn take_demand(&self)->::core::result::Result<#vc::RetirementDemand,#vc::ValueError>{#rc::retained_clone_cursor_take_demand::<#owner,bool>()}
                fn take_work_demand(&self)->::core::result::Result<u64,#vc::ValueError>{Ok(u64::from(#rc::RetainedCloneCursor::output_ready(self)))}
                fn take<'r>(&mut self,output:&mut Option<#owner>,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#rc::OriginalEffectReply<'r,#rc::RetainedCloneStep,#vc::ValueError>)->::core::result::Result<#rc::OriginalEffectReplyComplete<'r,#rc::RetainedCloneStep,#vc::ValueError>,#vc::ValueError>{
                    #rc::take_retained_clone_cursor_output!(#rc::RetainedCloneCursor::output_ready(self),self.output,output,self.spent,true,effect,reply,#owner,bool)
                }
                fn begin_close_demand(&self)->::core::result::Result<#vc::RetirementDemand,#vc::ValueError>{Ok(#rc::retained_close_start_demand(self.closing))}
                fn begin_close_work_demand(&self)->::core::result::Result<u64,#vc::ValueError>{Ok(#rc::retained_close_start_work_demand(self.closing))}
                fn begin_close_paid(&mut self,effect:&mut #vc::RetainedAllocationEffect<'_>)->#vc::RetainedWorkOutcome<#vc::RetainedCloneProgress,#vc::ValueError>{if self.route.is_some(){return #vc::RetainedWorkOutcome{result:Err(#vc::ValueError::invariant("record action must settle before close")),work:0}}#rc::start_retained_close(&mut self.closing,effect)}
                fn begin_close_into<'r>(&mut self,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{if self.route.is_some(){return Err(#vc::ValueError::invariant("record action must settle before close"))}let turn=#rc::start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(#rc::RetainedCloneStep::Progress).map_err(Into::into)))}
                fn recover_begin_close_into<'r>(&mut self,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.begin_close_into(effect,reply)}
                fn close_demands(&self,body:usize)->::core::result::Result<#vc::RetirementDemand,#vc::ValueError>{
                    if self.route.is_some(){return self.selected.map(|selected|selected.0).ok_or_else(||#vc::ValueError::invariant("record selected close quote missing"))}
                    if !self.closing{return Ok(Default::default())}
                    #(#demand_children)*
                    if !self.close.is_empty(){return self.close.demands(body)}
                    #(#demand_values)*
                    self.close.next_owner_demands::<#owner>(self.output.is_some(),&self.source,body)
                }
                fn close_work_demand(&self,body:usize)->::core::result::Result<u64,#vc::ValueError>{
                    if self.route.is_some(){return self.selected.map(|selected|selected.1).ok_or_else(||#vc::ValueError::invariant("record selected close work missing"))}
                    if !self.closing{return Ok(0)}
                    #(#work_children)*
                    if !self.close.is_empty(){return self.close.next_owner_work::<()>(false,&None,body)}
                    #(#work_values)*
                    self.close.next_owner_work::<#owner>(self.output.is_some(),&self.source,body)
                }
                fn close_into<'r>(&mut self,body:usize,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.close_reply(None,body,effect,reply,false)}
                fn recover_close_into<'r>(&mut self,body:usize,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.close_reply(None,body,effect,reply,true)}
                fn close_original_into<'r>(&mut self,original:&mut #loan,body:usize,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.close_reply(Some(original),body,effect,reply,false)}
                fn recover_close_original_into<'r>(&mut self,original:&mut #loan,body:usize,effect:&mut #vc::RetainedAllocationEffect<'_>,reply:#reply)->#complete{self.close_reply(Some(original),body,effect,reply,true)}
                fn next_close_depth_demand(&self)->::core::result::Result<usize,#vc::ValueError>{Ok(#rc::RetainedCloneCursor::close_demands(self,0)?.depth)}
                fn terminal_is_empty(&self)->bool{self.closing&&self.route.is_none()&&self.child_reply.is_none()&&self.output.is_none()&&self.source.is_none()&&self.close.is_empty() #(#empty)*}
            }
            impl #impl_generics #rc::RetainedClone for #name #ty_generics #where_clause {
                type Cursor = #cursor_name #ty_generics;
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

fn construct_fields(value_crate: &syn::Path, name: &syn::Ident, fields: &Fields, values: &[syn::Ident], variant: Option<&syn::Ident>) -> TokenStream {
    let prefix = variant.map_or_else(|| quote!(#name), |variant| quote!(#name::#variant));
    let owners = values.iter().map(|value|quote!(self.#value.take().ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"retained field owner is absent"))?)).collect::<Vec<_>>();
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
