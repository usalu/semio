/** 🏭️ Current typed worker admission retains original roots and pumps one funded mounted turn. */
import schema from "./📐️.json";
import {toolJobOriginalWorkerPumpScopeExact} from "./🧱️scope/🟦️.ts";
import {toolJobProductionSource,toolJobRustBlock} from "../../📋️inventory/🟦️.ts";
const compact=(s:string)=>s.replace(/\s+/g,"");
const rawBody=(s:string,name:string):string=>{
 const matches=[...s.matchAll(new RegExp("\\bfn\\s+"+name+"\\s*\\(","g"))];
 return matches.length===1?toolJobRustBlock(s,s.indexOf("{",matches[0]!.index!))?.body??"":"";
};
const body=(s:string,name:string)=>compact(rawBody(s,name));
const ordered=(s:string,parts:readonly string[]):boolean=>{
 let at=-1;
 for(const part of parts){at=s.indexOf(part,at+1);if(at<0)return false}
 return true;
};
const count=(s:string,needle:string)=>s.split(needle).length-1;
/** 🪪️ Original source and decoder custody survives producer preparation and worker admission. */
export function toolJobOriginalWorkerEvidence(raw:string):Record<string,boolean>{
 const source=toolJobProductionSource(raw),dispatch=body(source,"dispatch_typed_command_inner"),setup=body(source,"start_typed_command_operation"),advance=body(source,"advance_latest_wins_command_one"),drive=body(source,"drive_worker_step"),producer=body(source,"drive_original_producer");
 const never=["run_on_worker_async","run_to_completion","thread_local!","A::handle(","A::ephemeral(","MountedWorkerJobSession::try_new"];
 return {
  retainedQueue:ordered(dispatch,["self.media.require_complete_tool_operation_pipeline(admission)?","self.media.admit_typed_operation_slot(","self.tools.latest_wins_commands.insert_admitted("])&&!dispatch.includes("start_typed_command_operation(")&&!dispatch.includes("tool_jobs.dispatch")&&!dispatch.includes("refresh_cache().await")&&advance.includes("self.as_dispatch().start_typed_command_operation(command,admission,&mutpending.meta,&mutpending.original_caller,pending.original_body,pending.original_decoder,&mutpending.context_view,pending.operation.operation,lease,cx).await")&&count(compact(source),"start_typed_command_operation(")===2,
  capturedRoots:ordered(setup,["assert!(self.tools.tool_operations.can_insert(operation_id.0)","QualifiedToolProof::AppOwned(_)","self.capture_typed_command_roots(command.as_ref(),meta).await?"])&&setup.includes("self.tools.typed_operation_reservations[")&&setup.includes("letpresence_local=self.stores.presence_store.local_read().map_err(Fault::from)?"),
  producerRequest:ordered(setup,["letproducer_request=Some(ArtifactOwnedToolJobRequest{","source_binding:semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)","ArtifactOriginalToolCommand::from_admitted_op_binary(command,","input:retained_wire.take()","snapshot_read:request_snapshot_read.take()","context:owned_context.expect(","letsession=None;letsession_rejected=None;","self.tools.tool_operations.insert_admitted("])&&setup.includes("producer_source:producer_request.as_ref().map(|request|request.source_binding)")&&!setup.includes("dispatch_wire_retained_with_spec")&&!setup.includes("RefusedWorkerSource::pending("),
  sourceAuthority:setup.includes("source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx)")&&setup.includes("original_emission_caller:original_caller.take()")&&setup.includes("original_emission_decoder:Some(original_decoder)")&&setup.includes("original_emission_body:Some(original_body)")&&setup.includes("meta:metadata.take().unwrap()"),
  producerHandoff:ordered(drive,["self.validate_original_producer_source(cx)?","!self.source_identity.matches(cx)","self.drive_original_producer(cx)"])&&ordered(producer,["A::build_tool_job(&mutself.producer_request,&mutcandidate,cx)","ifletSome(job)=candidate","RefusedWorkerSource::pending(job,(self.source_identity.operation(),self.source_identity.generation()))"])&&producer.includes("originalproducercompletedwithoutretainingitsjob")&&!/StepContext::new|OriginalJobFixtureCaller|StepBudget::new/.test(producer),
  singleTurn:toolJobOriginalWorkerPumpScopeExact(rawBody(source,"drive_worker_step"))&&count(drive,"rejected.admit_pending(config,cx)")===1&&count(drive,"session.pump_one_for_interactive_turn(same)")===1&&count(drive,"pump_one")===1&&drive.includes("cx.with_original_work_loan(step_fuel,step_deadline_us,")&&drive.includes("original_lifecycle_step(cx,grant,demand,")&&!/\b(?:for|while|loop)\b/.test(rawBody(source,"drive_worker_step").replace(/"(?:[^"\\]|\\.)*"/g,'""'))&&!never.some(token=>dispatch.includes(token)||setup.includes(token)||drive.includes(token))
 };
}
/** 🔐️ Missing actual owners and every required worker fact fail closed. */
export function toolJobOriginalWorkerExact(source:string):boolean{
 const evidence=toolJobOriginalWorkerEvidence(source);
 return schema.required.every(key=>evidence[key]===true);
}
