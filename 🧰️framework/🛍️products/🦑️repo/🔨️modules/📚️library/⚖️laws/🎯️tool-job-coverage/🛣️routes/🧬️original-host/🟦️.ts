/** 🧬️ Source evidence binds reserved producers to their actual original caller and mounted worker custody. */
import schema from "./📐️.json";
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
 const dispatch=body(source,"dispatch_reserved_original_producer");
 const lease=body(source,"admit_original_tool_lease");
 const producer=body(source,"drive_original_producer");
 const validation=body(source,"validate_original_producer_source");
 const worker=body(source,"drive_worker_step");
 const freshness=body(source,"typed_operation_document_is_fresh");
 const scoped=dispatch+lease+producer+worker;
 const candidate=producer.indexOf("ifletSome(job)=candidate");
 const failure=producer.indexOf("ifresult.is_err()");
 return {
  sourceBinding:dispatch.includes("producer_source=Some(")&&dispatch.includes("OriginalToolSourceBinding::borrow(cx)")&&dispatch.indexOf("mount_original_reserved_owners(")>=0&&dispatch.indexOf("letvalidation=")>=0&&dispatch.indexOf("mount_original_reserved_owners(")<dispatch.indexOf("letvalidation=")&&dispatch.includes("!proof.admits::<A>(&admitted)"),
  sourceValidation:validation.includes("!self.source_identity.matches(cx)")&&validation.includes("self.original_emission_decoder!=cx.native_receiving_identity()")&&validation.includes("caller.validate_context(cx)?")&&validation.includes("self.producer_source.is_some_and(|binding|!binding.matches(cx))"),
  callerParent:lease.includes("cx.original_source_parent()?.source_identity()?")&&lease.includes("cx.original_source_parent()?.admit_loan(cx)")&&lease.includes("pending.source.as_ref().unwrap().validate_context(cx)?"),
  candidateCustody:candidate>=0&&failure>candidate&&producer.includes("RefusedWorkerSource::pending(job,(self.source_identity.operation(),self.source_identity.generation()))"),
  producerContext:producer.includes("A::build_tool_job(&mutself.producer_request,&mutcandidate,cx)")&&producer.includes("ArtifactReservedToolJob::admit_mounted(&mutself.reserved_producer,&mutcandidate,&self.operation,cx)"),
  workerContext:worker.includes("self.validate_original_producer_source(cx)?")&&worker.includes("!self.source_identity.matches(cx)")&&worker.indexOf("self.validate_original_producer_source(cx)?")<worker.indexOf("self.drive_original_producer(cx)"),
  workerBudget:worker.includes("letgrant=cx.retained_grant()")&&worker.includes("cx.fuel_remaining().min(contract.max_work_units_per_step)")&&worker.includes("u64::from(contract.max_step_micros)")&&worker.includes("cx.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same))"),
  pendingCustody:worker.includes("rejected.admit_pending(config,cx)")&&worker.includes("self.session=Some(session)")&&worker.includes("rejected.close_step(cx)"),
  outcomeAck:worker.includes("session.acknowledge_checked_out_outcome(grant)")&&worker.includes("self.worker_resume_pending=true")&&worker.includes("session.resume()"),
  terminalClose:worker.includes("session.close_step(cx)")&&worker.includes("progress.fits(grant)&&session.terminal_is_empty()"),
  cancellation:worker.includes("cx.is_cancelled()")&&worker.includes("self.user_cancel_requested")&&dispatch.includes("mounted.cancellation_lease.as_ref()")&&dispatch.includes(".cancel()"),
  outputLimit:dispatch.includes("decoded_items>proof.contract().max_decoded_items")&&all.includes("self.raw.len()>self.contract.max_output_bytes"),
  publicationFreshness:freshness.includes("canonical_revision==live_revision&&matches!(semio_framework_job::validate_commit(")&&freshness.includes("CommitValidation::Accepted"),
  noReconstructedContext:scoped.length>0&&!/StepContext::new|StepContext\s*\{|OriginalJobFixtureCaller|StepBudget::new/.test(scoped),
 };
}
/** 🔐️ The schema's full original-custody requirements must all hold; missing owners fail closed. */
export function toolJobOriginalReservedHostExact(source:string):boolean{
 const evidence=toolJobOriginalReservedHostEvidence(source);
 return schema.required.every(key=>evidence[key]===true);
}
