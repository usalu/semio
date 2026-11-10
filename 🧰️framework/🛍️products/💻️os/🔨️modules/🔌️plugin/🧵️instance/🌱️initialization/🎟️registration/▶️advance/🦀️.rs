//! ▶️ Each declared lane and Window sequence advances under the same original source and physical admission.
use super::*;
impl<A,M,D> OriginalNativeRegistration<A,M,D>
where A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static,D:OriginalInstanceDeclarations<A>{
 pub(super) fn advance_source(&mut self,definition:RegisteredDefinitionSource<'_>,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  match self.phase{
   0=>{if self.tools.is_some(){self.phase(1,cx)?;}else if let Some(source)=self.tools_source.as_mut(){if source.ready(){source.take(cx,&mut self.tools)?;}else{source.advance(definition,self.catalog.as_ref().unwrap().source().ok_or_else(||invalid("original catalog alias lost its source"))?,cx)?;}}else{OriginalDeclaredEmptyOperationSource::admit(cx,&mut self.birth,&mut self.tools_source)?;}},
   1=>{if self.ephemeral.is_some(){self.phase(2,cx)?;}else if let Some(source)=self.ephemeral_source.as_mut(){if source.ready(){source.take(cx,&mut self.ephemeral)?;}else{source.advance(cx)?;}}else{OriginalEphemeralSource::<A,D::Ephemeral>::admit(cx,&mut self.birth,&mut self.ephemeral_source)?;}},
   2=>{let registry=self.window.as_mut().ok_or_else(||invalid("original persisted Window registry is absent"))?;if let Some(source)=self.config_windows.as_mut(){if source.ready(){self.phase(3,cx)?;}else{source.advance(registry,cx)?;}}else{D::ConfigWindows::admit(registry,cx,&mut self.birth,&mut self.config_windows)?;}},
   3=>{if self.window_transient.is_some(){self.phase(4,cx)?;}else{let d=RetirementDemand{copy_bytes:2*size_of::<WindowTransientOwnerRegistry>(),depth:1,..Default::default()};self.birth.built(cx,&mut self.window_transient,construction(d),||Ok((WindowTransientOwnerRegistry::for_document_generation(lanes.document.generation()),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))?;}},
   4=>{let registry=self.window_transient.as_mut().ok_or_else(||invalid("original transient Window registry is absent"))?;if let Some(source)=self.transient_windows.as_mut(){if source.ready(){self.phase(5,cx)?;}else{source.advance(registry,cx)?;}}else{D::TransientWindows::admit(registry,cx,&mut self.birth,&mut self.transient_windows)?;}},
   5=>{if self.peer.is_some(){self.phase(6,cx)?;}else{let d=PeerPresenceRoot::birth_demands();let grant=cx.retained_grant();self.birth.built(cx,&mut self.peer,construction(d),||PeerPresenceRoot::admit_empty(grant)?.ok_or_else(||invalid("funded original peer root birth refused")))?;}},
   6=>{if self.scope.is_some(){self.phase(7,cx)?;}else{let d=Lifetime::root_admission_demands();let grant=cx.retained_grant();self.birth.built(cx,&mut self.scope,construction(d),||Lifetime::admit_root(grant).result?.ok_or_else(||invalid("funded original peer lifetime birth refused")))?;}},
   7=>{if self.fields.is_some(){self.phase(8,cx)?;}else{let d=Fields::<A>::birth_demands();self.birth.built(cx,&mut self.fields,construction(d),||Ok((Fields::<A>::new(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;}},
   8=>{if self.records.is_some(){self.phase(9,cx)?;}else{let d=Records::<A>::birth_demands();self.birth.built(cx,&mut self.records,construction(d),||Ok((Records::<A>::new(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;}},
   9=>{},_=>return Err(invalid("original native registration phase is invalid").into()),
  }Ok(())
 }
}
