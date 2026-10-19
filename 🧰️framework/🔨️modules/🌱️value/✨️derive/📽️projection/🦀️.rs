//! 📽️ Schema records stage one paid original field and retire every initialized field through cancellation.
use proc_macro2::TokenStream;
use quote::{format_ident,quote};
use syn::{Data,DeriveInput,Fields,Member,Index,parse_quote};
pub fn expand(input:&DeriveInput)->syn::Result<TokenStream>{
 let name=&input.ident;let visibility=&input.vis;let cursor=format_ident!("{}SnapshotProjection",name);let active=format_ident!("{}SnapshotProjectionField",name);
 let Data::Struct(data)=&input.data else{return Err(syn::Error::new_spanned(name,"SnapshotProjection requires a record; variant payloads must have sealed immutable row roots"));};
 if data.fields.is_empty(){let(impl_generics,ty_generics,where_clause)=input.generics.split_for_impl();let construct=match data.fields{Fields::Named(_)=>quote!(Self{}),Fields::Unnamed(_)=>quote!(Self()),Fields::Unit=>quote!(Self)};return Ok(quote!{impl #impl_generics ::semio_framework_value::snapshot_projection::ImmutableRootProjection for #name #ty_generics #where_clause{fn root_projection_demand(_: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,Self>)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{Ok(::semio_framework_value::RetirementDemand{copy_bytes: ::std::mem::size_of::<Self>()*2,depth:1,..Default::default()})}fn project_root(original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,Self>,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(Self,::semio_framework_value::retained_clone::RetainedCloneProgress)>,::semio_framework_value::ValueError>{let d=Self::root_projection_demand(original)?;Ok(::semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d).then(||(#construct,::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}}});}
 let fields:Vec<_>=data.fields.iter().enumerate().map(|(i,field)|(field.ident.as_ref().map_or(Member::Unnamed(Index::from(i)),|name|Member::Named(name.clone())),format_ident!("Field{i}"),&field.ty,i+1)).collect();
 let members:Vec<_>=fields.iter().map(|(m,_,_,_)|m).collect();let variants:Vec<_>=fields.iter().map(|(_,v,_,_)|v).collect();let types:Vec<_>=fields.iter().map(|(_,_,t,_)|*t).collect();let keys:Vec<_>=fields.iter().map(|(_,_,_,k)|*k).collect();let ranks:Vec<_>=(0..fields.len()).collect();let count=fields.len();
 let mut generics=input.generics.clone();for ty in &types{generics.make_where_clause().predicates.push(parse_quote!(#ty: ::semio_framework_value::snapshot_projection::SnapshotProjection));}
 let(impl_generics,ty_generics,where_clause)=generics.split_for_impl();
 Ok(quote!{
  enum #active #generics #where_clause{#(#variants(Box<::semio_framework_value::snapshot_projection::ProjectionField<#types>>),)*}
  #visibility struct #cursor #generics #where_clause{
   binding:Option<::semio_framework_value::retained_clone::RetainedCloneBinding>,
   active: ::std::mem::ManuallyDrop<Option<#active #ty_generics>>,
   candidate: ::std::mem::MaybeUninit<#name #ty_generics>,
   retirement: ::semio_framework_value::retained_clone::RetainedCloneClose,
   index:usize,initialized:usize,close_index:usize,field_staged:bool,assembled:bool,output_present:bool,closing:bool,
  }
  impl #impl_generics #cursor #ty_generics #where_clause{
   fn fixed()->usize{::std::mem::size_of::<usize>()*6+::std::mem::size_of::<Option<#active #ty_generics>>()+::std::mem::size_of::<bool>()*4}
   fn nested(mut d: ::semio_framework_value::RetirementDemand,copy:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{d.copy_bytes=d.copy_bytes.checked_add(copy).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::OwnershipLimit,"original record field copy quote overflow"))?;d.depth=d.depth.checked_add(1).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::DepthLimit,"original record field depth overflow"))?;Ok(d)}
   fn child(grant: ::semio_framework_value::retained_clone::RetainedCloneGrant,copy:usize)->::semio_framework_value::retained_clone::RetainedCloneGrant{::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copy,maximum_depth:grant.maximum_depth-1,..grant}}
   fn receipt(mut p: ::semio_framework_value::retained_clone::RetainedCloneProgress,copy:usize)->::semio_framework_value::retained_clone::RetainedCloneProgress{p.copied_items=1;p.copied_bytes+=copy;p}
  }
  impl #impl_generics ::semio_framework_value::snapshot_projection::SnapshotProjection for #name #ty_generics #where_clause{
   type Projection=#cursor #ty_generics;
   fn projection_birth_demand()->::semio_framework_value::RetirementDemand{::semio_framework_value::RetirementDemand{copy_bytes: ::std::mem::size_of::<Self::Projection>()*2,depth:1,..Default::default()}}
   fn begin_projection(grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Self::Projection,::semio_framework_value::retained_clone::RetainedCloneProgress),::semio_framework_value::ValueError>{
    if !<Self as ::semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::UnsupportedOwner,"immutable record fields have no controlled original retirement"));}
    let d=Self::projection_birth_demand();if !::semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::OwnershipLimit,"immutable record cursor birth is not funded"));}
    Ok((#cursor{binding:None,active: ::std::mem::ManuallyDrop::new(None),candidate: ::std::mem::MaybeUninit::uninit(),retirement:Default::default(),index:0,initialized:0,close_index:0,field_staged:false,assembled:false,output_present:false,closing:false},::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
   }
  }
  impl #impl_generics ::semio_framework_value::snapshot_projection::SnapshotProjectionCursor<#name #ty_generics> for #cursor #ty_generics #where_clause{
   fn demands(&self,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,body:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{
    use ::semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
    if self.closing||self.assembled{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable record projection is closing or prepared"));}
    if !original.check_binding(&self.binding)?{return Ok(RetirementDemand{copy_bytes:original.binding_copy_bytes(),depth:1,..Default::default()});}
    if let Some(active)=self.active.as_ref(){return match active{
     #(#active::#variants(field)=>{if self.field_staged{if !field.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"transferred original field still retains a child owner"));}return Ok(RetirementDemand{copy_bytes:Self::fixed(),release_bytes: ::std::mem::size_of::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>(),depth:1,..Default::default()});}let d=if field.ready(){field.take_demand()?}else{field.demands(original.project(#keys,|record|&record.#members),body)?};Self::nested(d,Self::fixed())},)*
    };}
    match self.index{
     #(#ranks=>Ok(RetirementDemand{copy_bytes: ::std::mem::size_of::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>()*2+Self::fixed(),capacity_bytes: ::std::mem::size_of::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>(),depth:1,..Default::default()}),)*
     #count=>Ok(RetirementDemand{copy_bytes:Self::fixed(),depth:1,..Default::default()}),
     _=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable record projection index is invalid")),
    }
   }
   fn advance(&mut self,original: ::semio_framework_value::retained_clone::RetainedCloneRef<'_,#name #ty_generics>,body:usize,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_value::ValueError>{
    use ::semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneStep,RetainedCloneProgress,retained_clone_grant_admits,admit_retained_clone_progress}};
    let d=self.demands(original,body)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
    if let Some(progress)=original.bind(&mut self.binding,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
    if self.active.is_none(){match self.index{
     #(#ranks=>{let layout=::std::alloc::Layout::new::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>();let Some(pointer)=::std::ptr::NonNull::new(unsafe{::std::alloc::alloc(layout)}.cast::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original active record field allocation failed"));};let field=unsafe{pointer.as_ptr().write(Default::default());Box::from_raw(pointer.as_ptr())};*self.active=Some(#active::#variants(field));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}));},)*
     #count=>{self.assembled=true;self.output_present=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));},
     _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original record field rank changed")),
    }}
    if self.field_staged{drop(self.active.take());self.field_staged=false;self.index+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}
    let child=Self::child(grant,Self::fixed());
    match self.active.as_mut().unwrap(){
     #(#active::#variants(field)=>{if field.ready(){let Some((value,p))=field.take(child)?else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"funded original record field transfer was refused"));};unsafe{::std::ptr::addr_of_mut!((*self.candidate.as_mut_ptr()).#members).write(value);}self.initialized+=1;self.field_staged=true;field.begin_close();Ok(RetainedCloneStep::Progress(Self::receipt(admit_retained_clone_progress(child,p,"original record field write")?,Self::fixed())))}else{let step=field.advance(original.project(#keys,|record|&record.#members),body,child)?;Ok(RetainedCloneStep::Progress(Self::receipt(admit_retained_clone_progress(child,step.progress(),"original record active field")?,Self::fixed()))) }},)*
    }
   }
   fn ready(&self)->bool{!self.closing&&self.assembled&&self.output_present}
   fn take_demand(&self)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{if !self.ready(){return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"immutable record has no admitted output"));}Ok(::semio_framework_value::RetirementDemand{copy_bytes: ::std::mem::size_of::<#name #ty_generics>()*2+Self::fixed(),depth:1,..Default::default()})}
   fn take(&mut self,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<Option<(#name #ty_generics,::semio_framework_value::retained_clone::RetainedCloneProgress)>,::semio_framework_value::ValueError>{let d=self.take_demand()?;if !::semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}let output=unsafe{self.candidate.assume_init_read()};self.initialized=0;self.output_present=false;Ok(Some((output,::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
   fn begin_close(&mut self){self.closing=true;self.output_present=false;match self.active.as_mut(){#(Some(#active::#variants(field))=>field.begin_close(),)*None=>{}}}
   fn close_demands(&self,body:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{
    use ::semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand};
    if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable record closure has not begun"));}
    if let Some(active)=self.active.as_ref(){return match active{#(#active::#variants(field)=>if field.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:Self::fixed(),release_bytes: ::std::mem::size_of::<::semio_framework_value::snapshot_projection::ProjectionField<#types>>(),depth:1,..Default::default()})}else{Self::nested(field.close_demands(body.saturating_sub(Self::fixed()))?,Self::fixed())},)*};}
    if !self.retirement.is_empty(){return Self::nested(self.retirement.next_owner_demands::<()>(false,&None,body.saturating_sub(Self::fixed()))?,Self::fixed());}
    if self.close_index<self.initialized{return match self.close_index{#(#ranks=>{let copy=Self::fixed()+::std::mem::size_of::<#types>()*2;Self::nested(self.retirement.next_owner_demands::<#types>(true,&None,body.saturating_sub(copy))?,copy)},)*_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original partial record closure rank changed"))};}
    self.retirement.next_owner_demands::<()>(false,&self.binding,body)
   }
   fn close_step(&mut self,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_value::ValueError>{
    use ::semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneStep,RetainedCloneProgress,retained_clone_grant_admits,admit_retained_clone_close}};
    if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
    if let Some(active)=self.active.as_mut(){match active{#(#active::#variants(field)=>{if !field.terminal_is_empty(){let child=Self::child(grant,Self::fixed());let step=field.close_step(child)?;return Ok(RetainedCloneStep::Progress(Self::receipt(admit_retained_clone_close(child,step,field.terminal_is_empty(),"original active record field cancellation")?.progress(),Self::fixed())));}},)*}drop(self.active.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}
    if !self.retirement.is_empty(){let child=Self::child(grant,Self::fixed());let step=self.retirement.step_granted(child)?;return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress(),Self::fixed())));}
    if self.close_index<self.initialized{match self.close_index{
     #(#ranks=>{let copy=Self::fixed()+::std::mem::size_of::<#types>()*2;let child=Self::child(grant,copy);let mut original=Some(unsafe{::std::ptr::addr_of_mut!((*self.candidate.as_mut_ptr()).#members).read()});match self.retirement.begin_granted(&mut original,child){Ok(Some(step))=>{self.close_index+=1;return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress(),copy)));},Ok(None)=>{unsafe{::std::ptr::addr_of_mut!((*self.candidate.as_mut_ptr()).#members).write(original.take().unwrap());}return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"funded original partial record field did not enter retirement"));},Err(error)=>{if let Some(original)=original.take(){unsafe{::std::ptr::addr_of_mut!((*self.candidate.as_mut_ptr()).#members).write(original);}}else{self.close_index+=1;}let p=Self::receipt(error.retained_progress(),copy);return Err(error.with_retained_progress(p));}}},)*
     _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original partial record closure rank changed")),
    }}
    ::semio_framework_value::retained_clone::close_retained_binding(&mut self.binding,grant)
   }
   fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.active.is_none()&&self.close_index==self.initialized&&self.retirement.is_empty()}
  }
  impl #impl_generics Drop for #cursor #ty_generics #where_clause{fn drop(&mut self){assert!(::std::thread::panicking()||self.binding.is_none()&&self.active.is_none()&&self.close_index==self.initialized&&self.retirement.is_empty(),"immutable record cursor retains original candidate owners");}}
 })
}
