//! 🛬️ Native receiving derives independently typed original child owners and funded aggregate assembly; a path default is canonical-wire only, so the original record must carry that field.
use quote::{quote,format_ident};
use syn::{DeriveInput,Data,Fields,Type,Member,Index};
use super::{container_native_owned_path,parse_container_attrs,parse_field_attrs,parse_variant_attrs,field_wire_name,variant_wire_name,FieldDefault};
#[path="🔀️enum/🦀️.rs"]
mod enums;
fn optional(ty:&Type)->bool{matches!(ty,Type::Path(path) if path.path.segments.last().is_some_and(|segment|segment.ident=="Option"))}
pub fn expand(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{
 if !input.generics.params.is_empty(){return Err(syn::Error::new_spanned(input,"native record receiving requires an explicit concrete owner declaration"));}
 let attrs=parse_container_attrs(&input.attrs)?;let value:syn::Path=syn::parse_str(attrs.crate_path.as_deref().unwrap_or("::semio_framework_value"))?;let native=container_native_owned_path(&attrs);
 let name=&input.ident;let vis=&input.vis;
 if let Data::Enum(data)=&input.data{
  let defaults=data.variants.iter().filter(|variant|variant.attrs.iter().any(|attr|attr.path().is_ident("default"))).collect::<Vec<_>>();
  if defaults.len()>1{return Err(syn::Error::new_spanned(input,"native default requires exactly one declared unit variant"));}
  let default=if let Some(variant)=defaults.first(){
   if !matches!(variant.fields,Fields::Unit){return Err(syn::Error::new_spanned(variant,"native default requires a declared unit variant"));}
   let variant=&variant.ident;
   quote!{impl #native::native_decoding::owned::NativeDefault for #name{
    fn native_default_demand()->#value::RetirementDemand{#native::native_decoding::owned::tree::fixed(2*::std::mem::size_of::<Self>())}
    fn native_default(grant:#value::RetainedCloneGrant)->Result<(Self,#value::RetainedCloneProgress),#value::ValueError>{let demand=<Self as #native::native_decoding::owned::NativeDefault>::native_default_demand();if !#value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(#native::native_decoding::owned::tree::invalid());}Ok((Self::#variant,#native::native_decoding::owned::tree::receipt(demand)))}}
   }
  }else{quote!{}};
  if attrs.tag.is_some()||data.variants.iter().any(|v|!matches!(v.fields,Fields::Unit)){let tokens=enums::expand(input,&attrs)?;return Ok(quote!(#default #tokens));}
  let variants=data.variants.iter().map(|v|{let attr=parse_variant_attrs(&v.attrs)?;let wire=variant_wire_name(&v.ident.to_string(),&attr.rename,&attrs.rename_all);let id=&v.ident;Ok(quote!(#wire=>Ok(Self::#id)))}).collect::<syn::Result<Vec<_>>>()?;
  let ordinal_variants=data.variants.iter().enumerate().map(|(index,variant)|{let id=&variant.ident;let ordinal=index as u32;quote!(#ordinal=>Ok(Self::#id))}).collect::<Vec<_>>();
  return Ok(quote!{
   #default
   impl<I:#native::native_decoding::owned::tree::NativeValueSource+?Sized> #native::native_decoding::owned::tree::NativeScalarValue<I> for #name{
    fn read(input:&I)->Result<Self,#value::ValueError>{if let Ok(ordinal)=input.enum_ordinal(){return match ordinal{#(#ordinal_variants,)*_=>Err(#native::native_decoding::owned::tree::invalid())}}match input.text()?.as_str(){#(#variants,)*_=>Err(#native::native_decoding::owned::tree::invalid())}}
    fn read_copy_bytes(input:&I)->Result<usize,#value::ValueError>{Ok(if input.enum_ordinal().is_ok(){::std::mem::size_of::<u32>()}else{input.text()?.len()}+::std::mem::size_of::<Self>())}
   }
   impl<I:#native::native_decoding::owned::tree::NativeValueSource+?Sized> #native::native_decoding::NativeReceive<I> for #name{
    type Receiver=#native::native_decoding::owned::tree::NativeScalarReceiver<I,Self>;
    fn receive_birth_demand()->#value::RetirementDemand{Self::Receiver::birth()}
    fn begin_receive(grant:#value::RetainedCloneGrant)->Result<(Self::Receiver,#value::RetainedCloneProgress),#value::ValueError>{Self::Receiver::begin(grant)}
   }
  });
 }
 let Data::Struct(data)=&input.data else{return Err(syn::Error::new_spanned(input,"native receiving does not admit unions"));};
 let tuple=matches!(data.fields,Fields::Unnamed(_));let fields=match &data.fields{Fields::Named(fields)=>fields.named.iter().collect::<Vec<_>>(),Fields::Unnamed(fields)=>fields.unnamed.iter().collect::<Vec<_>>(),Fields::Unit=>vec![]};
 if fields.len()>usize::from(u16::MAX)+1{return Err(syn::Error::new_spanned(input,"native field ordinal exceeds u16"));}let receiver=format_ident!("{}NativeReceiver",name);let count=fields.len();let mut members=Vec::new();let mut types=Vec::new();let mut wires=Vec::new();let mut required=Vec::new();let mut flattened=Vec::new();
 for(index,field)in fields.iter().enumerate(){let attr=parse_field_attrs(&field.attrs)?;if attr.skip||attr.effective_deserialize_with().is_some()||tuple&&attr.flatten{return Err(syn::Error::new_spanned(field,"native fields require their explicit original receiving port"));}let member=field.ident.as_ref().map_or_else(||Member::Unnamed(Index::from(index)),|id|Member::Named(id.clone()));let ty=&field.ty;let wire=if let Some(id)=field.ident.as_ref(){field_wire_name(&id.to_string(),&attr.rename,&attrs.rename_all)}else{index.to_string()};if matches!(attr.default,FieldDefault::Path(_)){return Err(syn::Error::new_spanned(field,"custom native defaults require an explicit original receiving port"));}let opt=!tuple&&!attr.required&&(optional(ty)||matches!(attr.default,FieldDefault::Bare)||attrs.default);members.push(member);types.push(ty);wires.push(wire);required.push(!opt);flattened.push(attr.flatten);}
 let missing=members.iter().zip(types.iter()).zip(required.iter()).enumerate().map(|(index,((id,ty),required))|if *required{quote!(return Err(#native::native_decoding::owned::tree::invalid());)}else{quote!{
  let overhead=2*::std::mem::size_of::<#ty>()+::std::mem::size_of::<bool>()+::std::mem::size_of::<usize>();
  control.charge(demand.capacity_bytes)?;
  let (field,progress)=<#ty as #native::native_decoding::owned::NativeDefault>::native_default(#native::native_decoding::owned::tree::child_grant(grant,overhead))?;
  unsafe{::std::ptr::write(::std::ptr::addr_of_mut!((*self.partial.as_mut().ok_or_else(#native::native_decoding::owned::tree::invalid)?.as_mut_ptr()).#id),field)};
  self.initialized[#index]=true;self.field+=1;
  return Ok(#value::RetainedCloneStep::Progress(#native::native_decoding::owned::tree::add_receipt(progress,overhead)?));
 }}).collect::<Vec<_>>();
 let missing_demands=types.iter().zip(required.iter()).map(|(ty,required)|if *required{quote!(Err(#native::native_decoding::owned::tree::invalid()))}else{quote!(#native::native_decoding::owned::tree::nested(<#ty as #native::native_decoding::owned::NativeDefault>::native_default_demand(),2*::std::mem::size_of::<#ty>()+::std::mem::size_of::<bool>()+::std::mem::size_of::<usize>()))}).collect::<Vec<_>>();
 let indices=(0..count).collect::<Vec<_>>();let slots=members.iter().map(|member|match member{Member::Named(id)=>format_ident!("slot_{}",id),Member::Unnamed(index)=>format_ident!("slot_{}",index.index)}).collect::<Vec<_>>();let tree=quote!(#native::native_decoding::owned::tree);
 let bounds=types.iter().zip(required.iter()).map(|(ty,required)|if *required{quote!(#ty:#native::native_decoding::NativeReceive<dyn #native::native_decoding::owned::tree::NativeValueSource>)}else{quote!(#ty:#native::native_decoding::NativeReceive<dyn #native::native_decoding::owned::tree::NativeValueSource>+#native::native_decoding::owned::NativeDefault)}).collect::<Vec<_>>();
 let implementation_where=if attrs.receive_recursive{quote!{}}else{quote!{where #(#bounds,)*}};
 let schema_memberships=types.iter().zip(wires.iter()).zip(flattened.iter()).enumerate().map(|(index,((ty,wire),flatten))|if *flatten{quote!(<#ty as #native::native_decoding::owned::NativeRecordFields>::native_field_memberships(key))}else{let ordinal=index as u16;quote!(usize::from(#native::native_decoding::owned::NativeRecordField{name:#wire,ordinal:#ordinal}.matches(key)))}).collect::<Vec<_>>();
 let schema_bytes=types.iter().zip(wires.iter()).zip(flattened.iter()).map(|((ty,wire),flatten)|if *flatten{quote!(<#ty as #native::native_decoding::owned::NativeRecordFields>::native_field_key_bytes(key))}else{let bytes=wire.len();quote!(match key{#native::native_decoding::owned::NativeRecordKey::Name(_)=>#bytes,#native::native_decoding::owned::NativeRecordKey::Ordinal(_)=>::std::mem::size_of::<u16>()})}).collect::<Vec<_>>();
 let selectors=types.iter().zip(wires.iter()).zip(flattened.iter()).enumerate().map(|(index,((ty,wire),flatten))|if *flatten{quote!(if <#ty as #native::native_decoding::owned::NativeRecordFields>::native_field_memberships(key)>0{field=Some(#index);})}else{let ordinal=index as u16;quote!(if (#native::native_decoding::owned::NativeRecordField{name:#wire,ordinal:#ordinal}).matches(key){field=Some(#index);})}).collect::<Vec<_>>();
 let child_method=if tuple{format_ident!("sequence_child")}else{format_ident!("record_child")};let sources=flattened.iter().enumerate().map(|(index,flatten)|if *flatten{quote!(input.project::<dyn #tree::NativeValueSource,_>(usize::MAX-#index,|source|source.as_source()))}else{quote!({input.get().#child_method(index)?;input.project::<dyn #tree::NativeValueSource,_>(index,move|source|source.#child_method(index).unwrap())})}).collect::<Vec<_>>();
 let slot_births=flattened.iter().map(|flatten|if *flatten{quote!(#tree::NativeReceiveSlot::original_record_subset())}else{quote!(#tree::NativeReceiveSlot::default())}).collect::<Vec<_>>();
 let scan_demand=if tuple{quote!(Ok(#tree::fixed(3*::std::mem::size_of::<usize>()+::std::mem::size_of::<u8>())))}else{quote!({let count=input.get().record_len()?;let bytes=if self.scan<count{let key=input.get().record_key(self.scan)?;key.comparison_bytes()+<#name as #native::native_decoding::owned::NativeRecordFields>::native_field_key_bytes(key)}else{0};Ok(#tree::fixed(bytes+3*::std::mem::size_of::<usize>()+::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<u8>()))})};
 let scan_step=if tuple{quote!(let count=input.get().sequence_len()?;if count!=#count{return Err(#tree::invalid());}if self.scan==count{self.phase=2;}else{self.positions[self.scan]=Some(self.scan);self.scan+=1;})}else{quote!(let count=input.get().record_len()?;if self.scan==count{self.phase=2;}else{let key=input.get().record_key(self.scan)?;let memberships=<#name as #native::native_decoding::owned::NativeRecordFields>::native_field_memberships(key);if memberships>1||memberships==0&&!self.subset{return Err(#tree::invalid());}if memberships==1{let mut field:Option<usize>=None;#(#selectors)*let field=field.ok_or_else(#tree::invalid)?;let flattened:[bool;#count]=[#(#flattened),*];if !flattened[field]&&self.positions[field].is_some(){return Err(#tree::invalid());}self.positions[field]=Some(self.scan);}self.scan+=1;})};
 Ok(quote!{
  #vis struct #receiver<I:#tree::NativeValueSource+?Sized> where #(#bounds,)*{
   binding:Option<#value::retained_clone::RetainedCloneBinding>,positions:[Option<usize>;#count],
   #(#slots: ::std::mem::ManuallyDrop<Option<Box<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>>>,)*
   partial: ::std::mem::ManuallyDrop<Option<Box<::std::mem::MaybeUninit<#name>>>>,initialized:[bool;#count],retirement:#value::retained_clone::RetainedCloneClose,scan:usize,field:usize,phase:u8,stage:u8,closing:bool,subset:bool,input: ::std::marker::PhantomData<fn(&I)>
  }
  #[cfg(test)]impl<I:#tree::NativeValueSource+?Sized> #receiver<I> where #(#bounds,)*{
   fn native_field_owner_count(&self)->usize{0usize#(+usize::from(self.#slots.is_some()))*}
  }  impl #native::native_decoding::owned::NativeRecordFields for #name{
   fn native_field_memberships(key:#native::native_decoding::owned::NativeRecordKey<'_>)->usize{0usize#(+#schema_memberships)*}
   fn native_field_key_bytes(key:#native::native_decoding::owned::NativeRecordKey<'_>)->usize{0usize#(+#schema_bytes)*}
  }  impl<I:#tree::NativeValueSource+?Sized> #native::native_decoding::NativeReceive<I> for #name #implementation_where{
   type Receiver=#receiver<I>;
   fn receive_birth_demand()->#value::RetirementDemand{#tree::fixed(2*::std::mem::size_of::<Self::Receiver>())}
   fn begin_receive(grant:#value::RetainedCloneGrant)->Result<(Self::Receiver,#value::RetainedCloneProgress),#value::ValueError>{let demand=<Self as #native::native_decoding::NativeReceive<I>>::receive_birth_demand();if !#value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(#tree::invalid());}Ok((#receiver{binding:None,positions:[None;#count],#(#slots: ::std::mem::ManuallyDrop::new(None),)*partial: ::std::mem::ManuallyDrop::new(None),initialized:[false;#count],retirement:Default::default(),scan:0,field:0,phase:0,stage:0,closing:false,subset:false,input: ::std::marker::PhantomData},#tree::receipt(demand)))}
   fn begin_record_subset(grant:#value::RetainedCloneGrant)->Result<(Self::Receiver,#value::RetainedCloneProgress),#value::ValueError>{let(mut receiver,p)=Self::begin_receive(grant)?;receiver.subset=true;Ok((receiver,p))}
  }
  impl<I:#tree::NativeValueSource+?Sized> #native::native_decoding::NativeReceiver<I,#name> for #receiver<I> where #(#bounds,)*{
   fn demands(&self,input:#value::retained_clone::RetainedCloneRef<'_,I>,body:usize)->Result<#value::RetirementDemand,#value::ValueError>{
    if self.closing||self.phase>=5{return Err(#tree::invalid());}input.check_binding(&self.binding)?;
    match self.phase{
     0=>Ok(#tree::fixed(input.binding_copy_bytes()+::std::mem::size_of::<u8>())),
     1=>#scan_demand,
     2=>if self.stage==0{Ok(#tree::fixed(::std::mem::size_of::<[Option<usize>;#count]>()+::std::mem::size_of::<u8>()))}else{Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<Option<Box<::std::mem::MaybeUninit<#name>>>>()+2*::std::mem::size_of::<u8>(),capacity_bytes: ::std::mem::size_of::<#name>(),depth:1,..Default::default()})},
     3=>{let overhead=::std::mem::size_of::<usize>()+::std::mem::size_of::<u8>();if self.field==#count{return Ok(#tree::fixed(overhead));}if self.positions[self.field].is_none(){return match self.field{#(#indices=>#missing_demands,)*_=>Err(#tree::invalid())};}let index=self.positions[self.field].ok_or_else(#tree::invalid)?;let child:#value::retained_clone::RetainedCloneRef<'_,dyn #tree::NativeValueSource>=match self.field{#(#indices=>#sources,)*_=>return Err(#tree::invalid())};match self.field{
      #(#indices=>match self.stage{
       0=>Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>()+2*::std::mem::size_of::<Option<Box<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>>>()+overhead,capacity_bytes: ::std::mem::size_of::<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>(),depth:1,..Default::default()}),
       1=>#tree::nested(self.#slots.as_ref().ok_or_else(#tree::invalid)?.demands(child,body.saturating_sub(overhead))?,overhead),
       2=>Ok(#tree::fixed(2*::std::mem::size_of::<#types>()+overhead+::std::mem::size_of::<bool>())),
       3=>#tree::nested(self.#slots.as_ref().ok_or_else(#tree::invalid)?.close_demands(body.saturating_sub(overhead))?,overhead),
       4=>Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<Option<Box<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>>>()+overhead,release_bytes: ::std::mem::size_of::<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>(),depth:1,..Default::default()}),
       _=>Err(#tree::invalid()),
      },)*_=>Err(#tree::invalid())}},
     4=>Ok(#tree::fixed(::std::mem::size_of::<[bool;#count]>()+::std::mem::size_of::<u8>())),_=>Err(#tree::invalid())
    }
   }
   fn advance(&mut self,input:#value::retained_clone::RetainedCloneRef<'_,I>,control:&mut #value::NativeDecodeControl<'_>,body:usize,grant:#value::RetainedCloneGrant)->Result<#value::RetainedCloneStep,#value::ValueError>{
    let demand=<Self as #native::native_decoding::NativeReceiver<I,#name>>::demands(self,input,body)?;if !#value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(#value::RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;
    match self.phase{
     0=>{let overhead=::std::mem::size_of::<u8>();let receipt=input.bind(&mut self.binding,#value::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant})?.ok_or_else(#tree::invalid)?;self.phase=1;return Ok(#value::RetainedCloneStep::Progress(#tree::add_receipt(receipt,overhead)?));},
     1=>{#scan_step},
     2=>{if self.stage==0{let required:[bool;#count]=[#(#required),*];for index in 0..#count{if required[index]&&self.positions[index].is_none(){return Err(#tree::invalid());}}self.stage=1;}else{control.charge(demand.capacity_bytes)?;*self.partial=Some(Box::<#name>::new_uninit());self.stage=0;self.phase=3;}},
     3=>{if self.field==#count{self.phase=4;}else if let Some(index)=self.positions[self.field]{let child:#value::retained_clone::RetainedCloneRef<'_,dyn #tree::NativeValueSource>=match self.field{#(#indices=>#sources,)*_=>return Err(#tree::invalid())};let overhead=::std::mem::size_of::<usize>()+::std::mem::size_of::<u8>();let child_grant=#tree::child_grant(grant,overhead);match self.field{
      #(#indices=>match self.stage{
       0=>{control.charge(demand.capacity_bytes)?;*self.#slots=Some(Box::new(#slot_births));self.stage=1;},
       1=>{let slot=self.#slots.as_mut().ok_or_else(#tree::invalid)?;let receipt=slot.advance(child,control,body.saturating_sub(overhead),child_grant).map_err(|error|{let p=error.retained_progress();error.with_retained_progress(#tree::add_receipt(p,overhead).unwrap_or(p))})?.progress();if slot.ready(){self.stage=2;}return Ok(#value::RetainedCloneStep::Progress(#tree::add_receipt(receipt,overhead)?));},
       2=>{let slot=self.#slots.as_mut().ok_or_else(#tree::invalid)?;let field=slot.take().ok_or_else(#tree::invalid)?;unsafe{::std::ptr::write(::std::ptr::addr_of_mut!((*self.partial.as_mut().ok_or_else(#tree::invalid)?.as_mut_ptr()).#members),field)};self.initialized[#indices]=true;slot.begin_close();self.stage=if slot.terminal_is_empty(){4}else{3};},
       3=>{control.charge(demand.capacity_bytes)?;let slot=self.#slots.as_mut().ok_or_else(#tree::invalid)?;let receipt=slot.close_step(child_grant)?.progress();if slot.terminal_is_empty(){self.stage=4;}return Ok(#value::RetainedCloneStep::Progress(#tree::add_receipt(receipt,overhead)?));},
       4=>{self.#slots.take();self.field+=1;self.stage=0;},_=>return Err(#tree::invalid()),
      },)*_=>return Err(#tree::invalid())}
     }else{match self.field{#(#indices=>{#missing},)*_=>return Err(#tree::invalid())};self.field+=1;}},
     4=>{if !self.initialized.iter().all(|field|*field){return Err(#tree::invalid());}self.phase=5;},_=>return Err(#tree::invalid())
    }Ok(if self.phase==5{#value::RetainedCloneStep::Complete(#tree::receipt(demand))}else{#value::RetainedCloneStep::Progress(#tree::receipt(demand))})
   }
   fn output_ready(&self)->bool{!self.closing&&self.phase==5&&self.partial.is_some()}
   fn take_demand(&self)->Result<#value::RetirementDemand,#value::ValueError>{if !self.output_ready(){return Err(#tree::invalid());}Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<#name>()+2*::std::mem::size_of::<Option<Box<::std::mem::MaybeUninit<#name>>>>()+::std::mem::size_of::<[bool;#count]>()+::std::mem::size_of::<u8>(),release_bytes: ::std::mem::size_of::<#name>(),depth:1,..Default::default()})}
   fn take(&mut self,grant:#value::RetainedCloneGrant)->Result<Option<(#name,#value::RetainedCloneProgress)>,#value::ValueError>{let demand=<Self as #native::native_decoding::NativeReceiver<I,#name>>::take_demand(self)?;if !#value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(None);}let partial=self.partial.take().ok_or_else(#tree::invalid)?;let value=unsafe{*partial.assume_init()};self.initialized=[false;#count];self.phase=6;Ok(Some((value,#tree::receipt(demand))))}
   fn begin_close(&mut self){self.closing=true;#(if let Some(slot)=self.#slots.as_mut(){slot.begin_close();})*}
   fn close_demands(&self,body:usize)->Result<#value::RetirementDemand,#value::ValueError>{
    if !self.closing{return Err(#tree::invalid());}
    #(if let Some(slot)=self.#slots.as_ref(){return if slot.terminal_is_empty(){Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<Option<Box<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>>>(),release_bytes: ::std::mem::size_of::<#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#types>>(),depth:1,..Default::default()})}else{slot.close_demands(body)};})*
    if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<#name>(false,&None,body);}
    #(if self.initialized[#indices]{return #tree::nested(self.retirement.next_owner_demands::<#types>(true,&None,body.saturating_sub(2*::std::mem::size_of::<#types>()+::std::mem::size_of::<bool>()))?,2*::std::mem::size_of::<#types>()+::std::mem::size_of::<bool>());})*
    if self.partial.is_some(){return Ok(#value::RetirementDemand{copy_bytes:2*::std::mem::size_of::<Option<Box<::std::mem::MaybeUninit<#name>>>>(),release_bytes: ::std::mem::size_of::<#name>(),depth:1,..Default::default()});}
    #value::retained_clone::RetainedCloneBinding::close_demand(&self.binding,body)
   }
   fn close_step(&mut self,grant:#value::RetainedCloneGrant)->Result<#value::RetainedCloneStep,#value::ValueError>{
    let demand=<Self as #native::native_decoding::NativeReceiver<I,#name>>::close_demands(self,grant.maximum_copy_bytes)?;if !#value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(#value::RetainedCloneStep::Progress(Default::default()));}
    #(if let Some(slot)=self.#slots.as_mut(){if !slot.terminal_is_empty(){return slot.close_step(grant).map(|step|#value::RetainedCloneStep::Progress(step.progress()));}self.#slots.take();return Ok(#value::RetainedCloneStep::Progress(#tree::receipt(demand)));})*
    if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|#value::RetainedCloneStep::Progress(step.progress()));}
    #(if self.initialized[#indices]{let overhead=2*::std::mem::size_of::<#types>()+::std::mem::size_of::<bool>();let child=#tree::child_grant(grant,overhead);let mut field=Some(unsafe{::std::ptr::read(::std::ptr::addr_of!((*self.partial.as_ref().ok_or_else(#tree::invalid)?.as_ptr()).#members))});self.initialized[#indices]=false;let step=self.retirement.begin_granted(&mut field,child);if let Some(field)=field{unsafe{::std::ptr::write(::std::ptr::addr_of_mut!((*self.partial.as_mut().ok_or_else(#tree::invalid)?.as_mut_ptr()).#members),field)};self.initialized[#indices]=true;}let step=step.map_err(|error|{let p=error.retained_progress();error.with_retained_progress(#tree::add_receipt(p,overhead).unwrap_or(p))})?.ok_or_else(#tree::invalid)?;return Ok(#value::RetainedCloneStep::Progress(#tree::add_receipt(step.progress(),overhead)?));})*
    if self.partial.is_some(){self.partial.take();return Ok(#value::RetainedCloneStep::Progress(#tree::receipt(demand)));}
    #value::retained_clone::RetainedCloneBinding::close_one(&mut self.binding,grant)
   }
   fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.partial.is_none()&&self.retirement.is_empty()#(&&self.#slots.is_none()&&!self.initialized[#indices])*}
  }
  impl<I:#tree::NativeValueSource+?Sized> Drop for #receiver<I> where #(#bounds,)*{fn drop(&mut self){assert!(::std::thread::panicking()||<Self as #native::native_decoding::NativeReceiver<I,#name>>::terminal_is_empty(self),"native record receiving requires funded closure");}}
 })
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod recursive_tests;
