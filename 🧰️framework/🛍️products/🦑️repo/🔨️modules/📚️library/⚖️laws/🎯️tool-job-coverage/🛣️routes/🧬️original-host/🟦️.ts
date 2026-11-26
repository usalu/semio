/** 🧬️ Source evidence binds reserved producers to their actual original caller and mounted worker custody. */
import schema from "./📐️.json"with{type:"json"};
import {toolJobProductionSource,toolJobRustBlock} from "../../📋️inventory/🟦️.ts";
const compact=(source:string):string=>source.replace(/\s+/g,"");
const body=(source:string,name:string):string=>{
 const matches=[...source.matchAll(new RegExp("\\bfn\\s+"+name+"(?:<[^\\n]*?>)?\\s*\\(","g"))];
 if(matches.length!==1)return "";
 const start=matches[0]!.index!;
 return compact(toolJobRustBlock(source,source.indexOf("{",start))?.body??"");
};
/** 🔎️ Every fact is extracted from its actual producer, parent, worker or publication function. */
export function toolJobOriginalReservedHostEvidence(raw:string):Record<string,boolean>{
 const source=toolJobProductionSource(raw),all=compact(source);
 const mount=body(source,"advance_original_reserved_mount");
 const construction=body(source,"admit_original_reserved");
 const request=body(source,"assemble_reserved_request");
 const wire=body(source,"prepare_reserved_wire_admission");
 const lease=body(source,"admit_original_tool_lease");
 const producer=body(source,"drive_original_producer");
 const validation=body(source,"validate_original_producer_source");
 const worker=body(source,"drive_worker_step");
 const publication=body(source,"publish_mounted_typed_operation_pump");
 const freshness=body(source,"typed_operation_document_is_fresh");
 const scoped=mount+construction+request+lease+producer+worker;
 const candidate=producer.indexOf("ifletSome(job)=candidate");
 const failure=producer.indexOf("ifresult.is_err()");
 const admit=mount.indexOf("MountedTypedCommandFullOperation::<A>::admit_original_reserved(");
 const insert=mount.indexOf("self.tools.tool_operations.insert_admitted(id,output.unwrap())");
 return {
  sourceBinding:admit>=0&&insert>admit
   &&mount.includes("if!owner.matches(cx)&&!owns_terminal_mount{returnOk(false);}")
   &&mount.includes("OriginalEmissionReceipt::capture(operation,cx)")
   &&construction.includes("letbinding=semio_framework_job::OriginalSourceBinding::borrow(cx)")
   &&construction.includes("letparent=cx.original_source_parent()?.source_identity()?")
   &&construction.includes("*source!=(cx.operation(),cx.generation(),parent)")
   &&construction.includes("!authority.lease.issued_by(canonical)")
   &&construction.includes("producer_source:Some(binding)")
   &&wire.includes("OriginalWireAdmission::admit(&mutowner.wire_key,&mutowner.wire_schema,length,cx,")
   &&wire.includes("cursor.advance(&*self.tools.tool_jobs,cx,&mutowner.wire_construction,&mutowner.admission)")
   &&request.includes("registration.owner!=semio_plugin_decl_app::tool_contract::ToolOwnerWitness::of::<A>()")
   &&request.includes("source.factory_type_id!=registration.factory_type_id")
   &&request.includes("source.contract!=registration.contract"),
  sourceValidation:validation.includes("!self.source_identity.matches(cx)")&&validation.includes("self.original_emission_decoder!=cx.native_receiving_identity()")&&validation.includes("caller.validate_context(cx)?")&&validation.includes("self.producer_source.is_some_and(|binding|!binding.matches(cx))"),
  callerParent:lease.includes("cx.original_source_parent()?.source_identity()?")&&lease.includes("cx.original_source_parent()?.admit_loan(&mut*pending.source,cx)")&&lease.includes("pending.source.as_ref().unwrap().validate_context(cx)?"),
  candidateCustody:candidate>=0&&failure>candidate&&producer.includes("RefusedWorkerSource::pending(job,(self.source_identity.operation(),self.source_identity.generation()))"),
  producerContext:producer.includes("ArtifactOwnedToolJobRequest::build_or_resume(&mutself.producer_request,&mutcandidate,self.reserved_construction.admission(),cx)")
   &&producer.includes("ArtifactReservedToolJob::admit_mounted(&mutself.reserved_producer,&mutself.reserved_construction,&self.operation,cx)"),
  workerContext:worker.includes("self.validate_original_producer_source(cx)?")&&worker.includes("!self.source_identity.matches(cx)")&&worker.indexOf("self.validate_original_producer_source(cx)?")<worker.indexOf("self.drive_original_producer(cx)"),
  workerBudget:worker.includes("letgrant=cx.retained_grant()")
   &&worker.includes("cx.fuel_remaining().min(contract.max_work_units_per_step)")
   &&worker.includes("u64::from(contract.max_step_micros)")
   &&worker.includes("letproducer=RetainedCloneGrant{maximum_items:cx.retained_grant().maximum_items,..grant}")
   &&worker.includes("cx.with_retained_loan(producer,|loaned|loaned.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same)))")
   &&worker.includes("!progress.fits(issued)||!progress.fits(producer)")
   &&["items","copy_bytes","capacity_bytes","release_bytes","depth"].every(axis=>worker.includes(`issued.maximum_${axis}>producer.maximum_${axis}`)),
  pendingCustody:worker.includes("rejected.admit_pending(config,&mutself.session,cx)")&&worker.includes("rejected.close_step(cx)"),
  outcomeAck:worker.includes("session.acknowledge_checked_out_outcome(cx)?{self.worker_outcome_pending=false;self.worker_resume_pending=true;}")&&worker.includes("session.resume()"),
  terminalClose:worker.includes("session.close_step(cx)")&&worker.includes("progress.fits(grant)&&session.terminal_is_empty()"),
  cancellation:worker.includes("cx.is_cancelled()")&&worker.includes("self.user_cancel_requested")
   &&construction.includes("cancellation_lease:Some(authority.lease.into_lease())")
   &&publication.includes("ifcx.is_cancelled(){ifletSome(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}"),
  outputLimit:request.includes("decoded_items>contract.max_decoded_items")&&all.includes("self.raw.len()>self.contract.max_output_bytes"),
  publicationFreshness:freshness.includes("canonical_revision==live_revision&&matches!(semio_framework_job::validate_commit(")&&freshness.includes("CommitValidation::Accepted"),
  noReconstructedContext:scoped.length>0&&!/StepContext::new|StepContext\s*\{|OriginalJobFixtureCaller|StepBudget::new/.test(scoped),
 };
}
/** 🔐️ The schema's full original-custody requirements must all hold; missing owners fail closed. */
export function toolJobOriginalReservedHostExact(source:string):boolean{
 const evidence=toolJobOriginalReservedHostEvidence(source);
 return schema.required.every(key=>evidence[key]===true);
}
