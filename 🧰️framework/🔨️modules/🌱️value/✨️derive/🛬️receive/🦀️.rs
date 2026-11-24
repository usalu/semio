//! 🛬️ Native records retain real original children, typed partial fields and phased defaults through every funded frontier.
use quote::{quote,format_ident};
use syn::{DeriveInput,Data,Fields,Type,Member,Index};
use super::{container_native_owned_path,parse_container_attrs,parse_field_attrs,parse_variant_attrs,field_wire_name,statement_keyword,variant_wire_name,FieldDefault};
#[path="🔀️enum/🦀️.rs"]mod enums;
fn optional(ty:&Type)->bool{matches!(ty,Type::Path(path)if path.path.segments.last().is_some_and(|s|s.ident=="Option"))}
/// 🍃️ A bare primitive scalar field is received by its record directly, without an owned child slot.
fn scalar_leaf(ty:&Type)->bool{match ty{Type::Tuple(tuple)=>!tuple.elems.is_empty()&&tuple.elems.iter().all(scalar_leaf),_=>matches!(ty,Type::Path(path)if path.qself.is_none()&&path.path.segments.len()==1&&path.path.segments[0].arguments.is_empty()&&["bool","f32","f64","u8","u16","u32","u64","usize","i8","i16","i32","i64","isize"].iter().any(|name|path.path.segments[0].ident==name))}}
pub fn expand(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{
 if !input.generics.params.is_empty(){return Err(syn::Error::new_spanned(input,"native record receiving requires an explicit concrete owner declaration"))}
 let attrs=parse_container_attrs(&input.attrs)?;if attrs.validate.is_some()||attrs.validate_controlled.is_some()||attrs.deserialize_controlled_with.is_some(){return Err(syn::Error::new_spanned(input,"validated native receiving requires its authored controlled owner port"))}let value:syn::Path=syn::parse_str(attrs.crate_path.as_deref().unwrap_or("::semio_framework_value"))?;let native=container_native_owned_path(&attrs);let owner=quote!(#native::native_decoding::owned);let tree=quote!(#owner::tree);let name=&input.ident;let vis=&input.vis;
 if let Data::Struct(data)=&input.data{if attrs.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1){
  if data.fields.len()!=1{return Err(syn::Error::new_spanned(input,"transparent native receiving requires exactly one original field"))}
  let field=data.fields.iter().next().unwrap();let field_attrs=parse_field_attrs(&field.attrs)?;
  if field_attrs.skip||field_attrs.flatten||field_attrs.effective_deserialize_with().is_some(){return Err(syn::Error::new_spanned(field,"transparent native receiving requires the original child receiving port"))}
  let ty=&field.ty;let construct=if let Some(member)=&field.ident{quote!(Self{#member:value})}else{quote!(Self(value))};
  return Ok(quote!{
   impl #owner::NativeReceiveMap<#ty> for #name{
 fn from_native(value:#ty)->Self{#construct}
 }
   impl<I:#tree::NativeValueSource+?Sized>#native::native_decoding::NativeReceive<I> for #name where #ty:#native::native_decoding::NativeReceive<I>{type Receiver=#owner::NativeMappedReceiver<I,#ty,Self>;fn receiver()->Self::Receiver{#owner::NativeMappedReceiver::new()}}
  })
 }}
 if let Data::Enum(data)=&input.data{
  let defaults=data.variants.iter().filter(|v|v.attrs.iter().any(|a|a.path().is_ident("default"))).collect::<Vec<_>>();if defaults.len()>1{return Err(syn::Error::new_spanned(input,"native default requires exactly one declared unit variant"))}
  let default=if let Some(variant)=defaults.first(){if !matches!(variant.fields,Fields::Unit){return Err(syn::Error::new_spanned(variant,"native default requires a declared unit variant"))}let variant=&variant.ident;quote!{
   impl #owner::NativeDefaultValue for #name{fn default_demand()->#value::RetirementDemand{#tree::fixed(::std::mem::size_of::<Option<Self>>())}fn default_work_demand()->Result<u64,#value::ValueError>{Ok(1)}
    fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut #value::RetainedAllocationEffect<'_>,reply:#native::native_decoding::OriginalEffectReply<'reply,#value::RetainedCloneStep,#native::native_decoding::OriginalReceivingPrimary>)->Result<#native::native_decoding::OriginalEffectReplyComplete<'reply,#value::RetainedCloneStep,#native::native_decoding::OriginalReceivingPrimary>,#value::ValueError>{if output.is_some(){return Ok(reply.complete(Err(#tree::invalid().into())))}let d=<Self as #owner::NativeDefaultValue>::default_demand();if !#value::retained_clone::retained_clone_grant_admits(effect.grant(),d)||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(#value::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(#value::ValueError::literal(#value::ValueRefusalKind::WorkLimit,"native default exceeded its work authority").into())))}*output=Some(Self::#variant);Ok(reply.complete(Ok(#value::RetainedCloneStep::Complete(#tree::receipt(d)))))}
    fn recover_default<'reply>(_:&mut Option<Self>,_:&mut #value::RetainedAllocationEffect<'_>,reply:#native::native_decoding::OriginalEffectReply<'reply,#value::RetainedCloneStep,#native::native_decoding::OriginalReceivingPrimary>)->Result<#native::native_decoding::OriginalEffectReplyComplete<'reply,#value::RetainedCloneStep,#native::native_decoding::OriginalReceivingPrimary>,#value::ValueError>{Ok(reply.pending())}}
   impl #owner::NativeDefault for #name{type Cursor=#owner::NativeDefaultValueCursor<Self>;fn default_cursor()->Self::Cursor{#owner::NativeDefaultValueCursor::new()}}
  }}else{quote!{}};
  if attrs.tag.is_some()||data.variants.iter().any(|v|!matches!(v.fields,Fields::Unit)){let tokens=enums::expand(input,&attrs)?;return Ok(quote!(#default #tokens))}
  let variants=data.variants.iter().enumerate().map(|(index,v)|{let attr=parse_variant_attrs(&v.attrs)?;let wire=variant_wire_name(&v.ident.to_string(),&attr.rename,&attrs.rename_all);let ordinal=index as u32;Ok(quote!(#index=>Some(#owner::NativeRecordSchema::Leaf(#owner::NativeSchemaLeaf{name:#wire,ordinal:#ordinal}))))}).collect::<syn::Result<Vec<_>>>()?;
  let statements=data.variants.iter().enumerate().map(|(index,v)|{let keyword=statement_keyword(&v.ident.to_string(),&v.attrs);let ordinal=index as u32;quote!(#index=>Some(#owner::NativeRecordSchema::Leaf(#owner::NativeSchemaLeaf{name:#keyword,ordinal:#ordinal})))}).collect::<Vec<_>>();
  let constructors=data.variants.iter().enumerate().map(|(index,v)|{let id=&v.ident;quote!(#index=>Ok(Self::#id))}).collect::<Vec<_>>();let count=data.variants.len();
  return Ok(quote!{#default impl #name{fn __semio_native_unit_schema(index:usize)->Option<#owner::NativeRecordSchema>{match index{#(#variants,)*_=>None}}fn __semio_native_unit_data(index:usize)->Option<#owner::NativeRecordSchema>{if index<#count{Some(#owner::NativeRecordSchema::Skip)}else{None}}fn __semio_native_unit_statements(index:usize)->Option<#owner::NativeRecordSchema>{match index{#(#statements,)*_=>None}}}impl #tree::variant::NativeUnitVariants for #name{fn variant_spec()->#tree::variant::NativeVariantSpec{#tree::variant::NativeVariantSpec{all:unsafe{#owner::NativeSchemaFields::native(Self::__semio_native_unit_schema)},units:unsafe{#owner::NativeSchemaFields::native(Self::__semio_native_unit_schema)},data:unsafe{#owner::NativeSchemaFields::native(Self::__semio_native_unit_data)},statements:unsafe{#owner::NativeSchemaFields::native(Self::__semio_native_unit_statements)},count:#count,tag:None}}fn from_variant_index(index:usize)->Result<Self,#value::ValueError>{match index{#(#constructors,)*_=>Err(#tree::invalid())}}}impl<I:#tree::NativeValueSource+?Sized>#native::native_decoding::NativeReceive<I> for #name{type Receiver=#tree::variant::NativeUnitEnumReceiver<I,Self>;fn receiver()-><Self as #native::native_decoding::NativeReceive<I>>::Receiver{#tree::variant::NativeUnitEnumReceiver::new()}}});

 }
 let Data::Struct(data)=&input.data else{return Err(syn::Error::new_spanned(input,"native receiving does not admit unions"))};let tuple=matches!(data.fields,Fields::Unnamed(_));let fields=match &data.fields{Fields::Named(f)=>f.named.iter().collect::<Vec<_>>(),Fields::Unnamed(f)=>f.unnamed.iter().collect(),Fields::Unit=>vec![]};if fields.len()>usize::from(u16::MAX)+1{return Err(syn::Error::new_spanned(input,"native field ordinal exceeds u16"))}let count=fields.len();
 let mut members=Vec::new();let mut types=Vec::new();let mut wires=Vec::new();let mut required=Vec::new();let mut flattened=Vec::new();for(index,field)in fields.iter().enumerate(){let attr=parse_field_attrs(&field.attrs)?;if attr.skip||attr.effective_deserialize_with().is_some()&&!attr.native_octets()||tuple&&attr.flatten{return Err(syn::Error::new_spanned(field,"native fields require their explicit original receiving port"))}members.push(field.ident.as_ref().map_or_else(||Member::Unnamed(Index::from(index)),|id|Member::Named(id.clone())));types.push(&field.ty);wires.push(field.ident.as_ref().map_or_else(||index.to_string(),|id|field_wire_name(&id.to_string(),&attr.rename,&attrs.rename_all)));required.push(tuple||attr.required||matches!(attr.default,FieldDefault::Path(_))||!(optional(&field.ty)||matches!(attr.default,FieldDefault::Bare)||attrs.default));flattened.push(attr.flatten);}
 let indices=(0..count).collect::<Vec<_>>();let leaf=types.iter().zip(flattened.iter()).map(|(ty,flat)|!*flat&&scalar_leaf(ty)).collect::<Vec<_>>();let slots=members.iter().map(|m|match m{Member::Named(id)=>format_ident!("slot_{}",id),Member::Unnamed(i)=>format_ident!("slot_{}",i.index)}).collect::<Vec<_>>();let pending=members.iter().map(|m|match m{Member::Named(id)=>format_ident!("pending_{}",id),Member::Unnamed(i)=>format_ident!("pending_{}",i.index)}).collect::<Vec<_>>();let defaults=members.iter().map(|m|match m{Member::Named(id)=>format_ident!("default_{}",id),Member::Unnamed(i)=>format_ident!("default_{}",i.index)}).collect::<Vec<_>>();
 let mut shared_keys:Vec<String>=Vec::new();let mut first=vec![true;count];let mut slots=slots;let mut shared=vec![false;count];
 for i in 0..count{if leaf[i]||flattened[i]{continue}let key=quote::ToTokens::to_token_stream(types[i]).to_string();if let Some(at)=shared_keys.iter().position(|k|*k==key){let owner=(0..i).find(|j|!leaf[*j]&&!flattened[*j]&&quote::ToTokens::to_token_stream(types[*j]).to_string()==key).unwrap();let _=at;slots[i]=slots[owner].clone();first[i]=false;shared[i]=true;shared[owner]=true}else{shared_keys.push(key)}}
 let skip=leaf.iter().zip(first.iter()).map(|(leaf,first)|*leaf||!*first).collect::<Vec<_>>();
 let bounds=types.iter().zip(required.iter()).map(|(ty,required)|if *required{quote!(#ty:#native::native_decoding::NativeReceive<dyn #tree::NativeValueSource>)}else{quote!(#ty:#native::native_decoding::NativeReceive<dyn #tree::NativeValueSource>+#owner::NativeDefault)}).collect::<Vec<_>>();let implementation_where=quote!{};
 let default_fields=types.iter().zip(defaults.iter()).zip(required.iter()).filter_map(|((ty,id),required)|(!required).then(||quote!(#id:#owner::NativeDefaultSlot<#ty,#owner::SharedFrame>))).collect::<Vec<_>>();let default_births=defaults.iter().zip(required.iter()).filter_map(|(id,required)|(!required).then(||quote!(#id:#owner::NativeDefaultSlot::new()))).collect::<Vec<_>>();
 let descriptors=types.iter().zip(wires.iter()).zip(flattened.iter()).enumerate().map(|(index,((ty,wire),flatten))|if *flatten{quote!(#index=>Some(#owner::NativeRecordSchema::Flatten{fields:<#ty as #owner::NativeRecordFields>::native_schema_fields(),count:<#ty as #owner::NativeRecordFields>::native_schema_count()}))}else{let ordinal=index as u32;quote!(#index=>Some(#owner::NativeRecordSchema::Leaf(#owner::NativeSchemaLeaf{name:#wire,ordinal:#ordinal})))}).collect::<Vec<_>>();
 let births=flattened.iter().map(|flatten|if *flatten{quote!(#tree::NativeReceiveSlot::original_record_subset())}else{quote!(#tree::NativeReceiveSlot::default())}).collect::<Vec<_>>();
 let slot_decls=slots.iter().zip(types.iter()).zip(skip.iter()).filter_map(|((slot,ty),leaf)|(!leaf).then(||quote!(#slot:#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#ty,#owner::SharedFrame>,))).collect::<Vec<_>>();let slot_births=slots.iter().zip(births.iter()).zip(skip.iter()).filter_map(|((slot,birth),leaf)|(!leaf).then(||quote!(#slot:#birth,))).collect::<Vec<_>>();let live_slots=slots.iter().zip(skip.iter()).filter_map(|(slot,leaf)|(!leaf).then(||slot.clone())).collect::<Vec<_>>();
 let state=format_ident!("{}NativeReceivingState",name);
 let machine=quote!(#owner::machine);
 let default_ids=defaults.iter().zip(required.iter()).filter_map(|(id,r)|(!r).then(||id.clone())).collect::<Vec<_>>();
 let has_frame=!live_slots.is_empty()||!default_ids.is_empty();
 let frame_ty=if has_frame{quote!(#owner::NativeReceivingChild<#native::native_decoding::OriginalReceivingPrimary>)}else{quote!(())};
 let frame_empty=if has_frame{quote!(self.frame.terminal_is_empty())}else{quote!(true)};
 let anchor_body=if has_frame{quote!(let frame=::std::ptr::addr_of_mut!(self.frame);#(self.#live_slots.anchor_frame(frame);)*#(self.#default_ids.anchor_frame(frame);)*)}else{quote!()};
 let frame_mark=if has_frame{quote!(::std::ptr::addr_of_mut!(self.frame))}else{quote!(::std::ptr::null_mut())};
 let seed_fields=slots.iter().zip(births.iter()).zip(pending.iter()).zip(skip.iter()).flat_map(|(((slot,birth),pending),leaf)|(!*leaf).then(||(quote!(#slot),birth.clone(),true)).into_iter().chain([(quote!(#pending),quote!(::std::mem::ManuallyDrop::new(None)),false)]))
  .chain(defaults.iter().zip(required.iter()).filter(|(_,required)|!**required).map(|(id,_)|(quote!(#id),quote!(#owner::NativeDefaultSlot::new()),true)))
  .chain([(quote!(frame),quote!(Default::default()),false)]).collect::<Vec<_>>();
 let seed_count=seed_fields.len();
 let seed_sizes=seed_fields.iter().enumerate().map(|(index,(field,_,_))|quote!(#index=>size(::std::ptr::addr_of_mut!((*target).#field)))).collect::<Vec<_>>();
 let seed_writes=seed_fields.iter().enumerate().map(|(index,(field,value,anchored))|if *anchored{quote!(#index=>{let slot=::std::ptr::addr_of_mut!((*target).#field);slot.write(#value);(*slot).anchor_frame(::std::ptr::addr_of_mut!((*target).frame))})}else{quote!(#index=>::std::ptr::addr_of_mut!((*target).#field).write(#value))}).collect::<Vec<_>>();
 let leaves=wires.iter().enumerate().map(|(index,name)|{let ordinal=index as u32;quote!(#owner::NativeSchemaLeaf{name:#name,ordinal:#ordinal})}).collect::<Vec<_>>();
 let any_flatten=flattened.iter().any(|flat|*flat);
 let shape=|index:usize|{let leaf=leaf[index];let flat=flattened[index];quote!(#machine::FieldShape{leaf:#leaf,flat:#flat})};
 let present=|index:usize|(!leaf[index]).then(||slots[index].clone());
 let phased=|index:usize|(!required[index]).then(||defaults[index].clone());
 let no_slot=quote!(#machine::NoSlot);
 let as_ref=|id:Option<syn::Ident>|match id{Some(id)=>quote!(Some(&self.#id)),None=>quote!(None::<&#no_slot>)};
 let as_mut=|id:Option<syn::Ident>|match id{Some(id)=>quote!(Some(&mut self.#id)),None=>quote!(None::<&mut #no_slot>)};
 let as_plan=|id:Option<syn::Ident>|match id{Some(id)=>quote!(Some(&self.#id)),None=>quote!(None)};
 let core=quote!(#machine::NativeRecordCore<I,#name,#count>);
 let input=quote!(#value::retained_clone::RetainedCloneRef<'_,I>);
 let plan_arms=indices.iter().map(|i|{let ty=types[*i];let sh=shape(*i);let s=as_plan(present(*i));let d=as_plan(phased(*i));quote!(#i=>#machine::field_plan::<I,#name,#ty,#count>(c,&Self::SPEC,#sh,#i,input,body,maximum,#s,#d))}).collect::<Vec<_>>();
 let work_arms=indices.iter().map(|i|{let ty=types[*i];let s=as_ref(present(*i));let d=as_ref(phased(*i));quote!(#i=>#machine::field_work::<I,#name,#ty,_,_,#count>(c,#i,body,#s,#d))}).collect::<Vec<_>>();
 let close_plan_arms=indices.iter().map(|i|{let ty=types[*i];let pending=&pending[*i];let s=as_ref(present(*i));let d=as_ref(phased(*i));quote!(#i=>#machine::field_close_plan::<I,#name,#ty,_,_,#count>(c,#i,body,&self.#pending,#s,#d))}).collect::<Vec<_>>();
 let apply_arms=indices.iter().map(|i|{let ty=types[*i];let member=&members[*i];let pending=&pending[*i];let sh=shape(*i);let shared=shared[*i];let s=as_mut(present(*i));let d=as_mut(phased(*i));quote!(#i=>#machine::field_apply::<I,#name,#ty,_,_,#count>(c,#sh,#shared,::std::mem::offset_of!(#name,#member),#i,effect,work,&mut self.#pending,#s,#d,#frame_mark))}).collect::<Vec<_>>();
 let leaf_arms=indices.iter().filter(|i|leaf[**i]).map(|i|{let ty=types[*i];let pending=&pending[*i];quote!(#i=>#machine::field_leaf::<I,#name,#ty,#count>(c,&Self::SPEC,#i,input,&mut self.#pending,work))}).collect::<Vec<_>>();
 let nested_arms=indices.iter().map(|i|{let ty=types[*i];let s=as_mut(present(*i));let d=as_mut(phased(*i));quote!(#i=>#machine::field_nested::<I,#name,#ty,_,_,#count>(c,#i,control,witness,body,maximum,grant,#s,#d))}).collect::<Vec<_>>();
 let retire_arms=indices.iter().map(|i|{let ty=types[*i];let s=as_mut(present(*i));let d=as_mut(phased(*i));quote!(#i=>#machine::field_retire::<I,#name,#ty,_,_,#count>(c,#i,body,effect,#s,#d))}).collect::<Vec<_>>();
 let begin_arms=indices.iter().map(|i|{let ty=types[*i];let s=as_mut(present(*i));let d=as_mut(phased(*i));quote!(#i=>#machine::field_begin::<I,#name,#ty,_,_,#count>(c,#i,effect,#s,#d))}).collect::<Vec<_>>();
 let close_arms=indices.iter().map(|i|{let ty=types[*i];let member=&members[*i];let pending=&pending[*i];let s=as_mut(present(*i));let d=as_mut(phased(*i));quote!(#i=>#machine::field_close_step::<I,#name,#ty,_,_,#count>(c,::std::mem::offset_of!(#name,#member),#i,original,body,demand,effect,&mut self.#pending,#s,#d))}).collect::<Vec<_>>();
 let view_ids=live_slots.iter().chain(default_ids.iter()).cloned().collect::<Vec<_>>();
 let view_indices=(0..view_ids.len()).collect::<Vec<_>>();
 let all_leaf=attrs.crate_path.is_none()&&!tuple&&count>0&&(0..count).all(|i|leaf[i]&&required[i]&&!flattened[i]);
 let leaf_indices=(0..count).collect::<Vec<_>>();
 let leaf_vars=(0..count).map(|i|format_ident!("field_{}",i)).collect::<Vec<_>>();
 let leaf_names=members.iter().map(|m|match m{Member::Named(id)=>quote!(#id),Member::Unnamed(i)=>quote!(#i)}).collect::<Vec<_>>();
 let scalar_impl=quote!{
  impl<I:#tree::NativeValueSource+?Sized>#native::native_decoding::NativeReceive<I> for #name{type Receiver=#tree::NativeScalarReceiver<I,Self>;fn receiver()->Self::Receiver{#tree::NativeScalarReceiver::new()}}
  impl<I:#tree::NativeValueSource+?Sized>#tree::NativeScalarValue<I> for #name{
   fn read(original:&I)->Result<Self,#value::ValueError>{
    #(let mut #leaf_vars:Option<#types>=None;)*
    for position in 0..original.record_len()?{
     let index=match original.record_key(position)?{#owner::NativeRecordKey::Name(name)=>match name{#(#wires=>#leaf_indices,)*_=>return Err(#tree::invalid())},#owner::NativeRecordKey::Ordinal(ordinal)=>usize::from(ordinal)};
     let child=original.record_child(position)?;
     match index{#(#leaf_indices=>{if #leaf_vars.is_some(){return Err(#tree::invalid())}#leaf_vars=Some(<#types as #tree::NativeScalarValue<dyn #tree::NativeValueSource>>::read(child)?)},)*_=>return Err(#tree::invalid())}
    }
    Ok(Self{#(#leaf_names:#leaf_vars.ok_or_else(#tree::invalid)?,)*})
   }
   fn read_copy_bytes(_:&I)->Result<usize,#value::ValueError>{Ok(::std::mem::size_of::<Option<Self>>())}
  }
 };
 let receive_impl=if all_leaf{scalar_impl}else{quote!{
 impl<I:#tree::NativeValueSource+?Sized>#native::native_decoding::NativeReceive<I>for #name #implementation_where{type Receiver=#machine::NativeRecordReceiver<I,#name,#state,#count>;fn receiver()->Self::Receiver{#machine::NativeRecordReceiver::birth(false)}fn record_subset_receiver()->Option<Self::Receiver>{Some(#machine::NativeRecordReceiver::birth(true))}
  fn receiver_template(subset:bool)->Option<#native::native_decoding::NativeSeedTemplate<Self::Receiver>>{<Self::Receiver>::template(subset)}}
 }};
 Ok(quote!{
 #vis struct #state where #(#bounds,)*{#(#slot_decls)*#(#pending: ::std::mem::ManuallyDrop<Option<#types>>,)*#(#default_fields,)*frame:#frame_ty}
 impl #state where #(#bounds,)*{
  const SPEC:#machine::RecordSpec=#machine::RecordSpec{count:#count,tuple:#tuple,flatten:&[#(#flattened),*],required:&[#(#required),*],leaves:&[#(#leaves),*],any_flatten:#any_flatten};
 }
 unsafe impl #owner::NativeRecordFields for #name{fn native_schema_count()->usize{#count}fn native_schema_field(index:usize)->Option<#owner::NativeRecordSchema>{match index{#(#descriptors,)*_=>None}}}
  #receive_impl
 impl<I:#tree::NativeValueSource+?Sized>#machine::NativeRecordArms<I,#name,#count> for #state where #(#bounds,)*{
  fn spec(&self)->&'static #machine::RecordSpec{&Self::SPEC}
  fn static_spec()->&'static #machine::RecordSpec{&Self::SPEC}
  fn born()->Self{Self{#(#slot_births)*#(#pending: ::std::mem::ManuallyDrop::new(None),)*#(#default_births,)*frame:Default::default()}}
  fn schema(&self)->(#owner::NativeSchemaFields,usize){(<#name as #owner::NativeRecordFields>::native_schema_fields(),<#name as #owner::NativeRecordFields>::native_schema_count())}
  fn plan_field(&self,c:&#core,i:usize,input:#input,body:usize,maximum:u64)->#machine::PlanOut{match i{#(#plan_arms,)*_=>Err(#tree::invalid())}}
  fn work_field(&self,c:&#core,i:usize,body:usize)->#machine::WorkOut{match i{#(#work_arms,)*_=>Err(#tree::invalid())}}
  fn close_plan_field(&self,c:&#core,i:usize,body:usize)->#machine::ClosePlanOut{match i{#(#close_plan_arms,)*_=>Err(#tree::invalid())}}
  fn apply_field(&mut self,c:&mut #core,i:usize,effect:&mut #machine::Effect<'_>,work:&mut u128)->#machine::ApplyOut{match i{#(#apply_arms,)*_=>Err(#tree::invalid().into())}}
  fn leaf_field(&mut self,c:&mut #core,i:usize,input:#input,work:&mut u128)->#machine::LeafOut{match i{#(#leaf_arms,)*_=>Err(#tree::invalid().into())}}
  fn nested_field(&mut self,c:&mut #core,i:usize,control:&mut #machine::Control<'_>,witness:#machine::Witness<'_>,body:usize,maximum:u64,grant:#machine::Grant)->#machine::NestedOut{match i{#(#nested_arms,)*_=>#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0})}}
  fn retire_field(&mut self,c:&mut #core,i:usize,body:usize,effect:&mut #machine::Effect<'_>)->#machine::CloseOut{match i{#(#retire_arms,)*_=>#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0})}}
  fn begin_field(&mut self,c:&mut #core,i:usize,effect:&mut #machine::Effect<'_>)->#machine::CloseOut{match i{#(#begin_arms,)*_=>#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0})}}
  fn close_field(&mut self,c:&mut #core,i:usize,original:Option<&mut #machine::Loan<'_>>,body:usize,demand:#machine::Demand,effect:&mut #machine::Effect<'_>)->#machine::CloseOut{match i{#(#close_arms,)*_=>#value::RetainedWorkPoll::Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0})}}
  fn slot(&self,k:usize)->Option<&dyn #machine::RecordSlotView>{match k{#(#view_indices=>Some(&self.#view_ids),)*_=>None}}
  fn slot_mut(&mut self,k:usize)->Option<&mut dyn #machine::RecordSlotView>{match k{#(#view_indices=>Some(&mut self.#view_ids),)*_=>None}}
  fn pending_empty(&self)->bool{true #(&&self.#pending.is_none())*}
  fn slots_empty(&self)->bool{true #(&&self.#view_ids.terminal_is_empty())*}
  fn frame_empty(&self)->bool{#frame_empty}
  fn anchor(&mut self){#anchor_body}
  fn seed_count()->usize{#seed_count}
  unsafe fn seed_size(target:*mut Self,index:usize)->Option<usize>{fn size<T>(_:*mut T)->usize{::std::mem::size_of::<T>()}Some(unsafe{match index{#(#seed_sizes,)*_=>return None}})}
  unsafe fn seed_write(target:*mut Self,index:usize)->bool{unsafe{match index{#(#seed_writes,)*_=>return false}}true}
 }
 })
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod recursive_tests;
