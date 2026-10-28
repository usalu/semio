//! ▶️ Each registration child and original registry birth advances under its initiating source scope.
use super::*;
impl<A,M,O> OriginalSingleWindowRegistration<A,M,O>
where A:OriginalArtifactAppBirth<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>,M:SpaceMember+MemberFactory+'static,O:WindowConfigOwner{
 pub(super) fn advance_source(&mut self,definition:RegisteredDefinitionSource<'_>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  match self.phase{
   0=>{if self.tools.is_some(){self.phase(1,cx)?;}else if let Some(source)=self.tools_source.as_mut(){if source.ready(){source.take(cx,&mut self.tools)?;}else{source.advance(definition,self.catalog.as_ref().unwrap().source().ok_or_else(||invalid("original catalog alias lost its source"))?,cx)?;}}else{OriginalDeclaredEmptyOperationSource::admit(cx,&mut self.tools_source)?;}},
   1=>{if self.ephemeral.is_some(){self.phase(2,cx)?;}else if let Some(source)=self.ephemeral_source.as_mut(){if source.ready(){source.take(cx,&mut self.ephemeral)?;}else{source.advance(cx)?;}}else{OriginalEmptyEphemeralSource::admit(cx,&mut self.ephemeral_source)?;}},
   2=>{let window=self.window.as_mut().ok_or_else(||invalid("original Window registry is absent"))?;if let Some(source)=self.window_source.as_mut(){if source.terminal_is_empty(){self.phase(3,cx)?;}else{source.step(window,cx)?;}}else{window.admit_original_owner::<O>(cx,&mut self.window_source)?;}},
   3=>{if self.peer.is_some(){self.phase(4,cx)?;}else{let d=PeerPresenceRoot::birth_demands();let grant=cx.retained_grant();admit_original_construction(cx,&mut self.peer,construction(d),|_|PeerPresenceRoot::admit_empty(grant)?.ok_or_else(||invalid("funded original peer root birth refused")))?;}},
   4=>{if self.scope.is_some(){self.phase(5,cx)?;}else{let d=Lifetime::root_admission_demands();let grant=cx.retained_grant();admit_original_construction(cx,&mut self.scope,construction(d),|_|Lifetime::admit_root(grant)?.ok_or_else(||invalid("funded original peer lifetime birth refused")))?;}},
   5=>{if self.fields.is_some(){self.phase(6,cx)?;}else{let d=Fields::<A>::birth_demands();admit_original_construction(cx,&mut self.fields,construction(d),|_|Ok((Fields::<A>::new(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;}},
   6=>{if self.records.is_some(){self.phase(7,cx)?;}else{let d=Records::<A>::birth_demands();admit_original_construction(cx,&mut self.records,construction(d),|_|Ok((Records::<A>::new(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})))?;}},
   7=>{},_=>return Err(invalid("original Window registration phase is invalid").into()),
  }Ok(())
 }
}
