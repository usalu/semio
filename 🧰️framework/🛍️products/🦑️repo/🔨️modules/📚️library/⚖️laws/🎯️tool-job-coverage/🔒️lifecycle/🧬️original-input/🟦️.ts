/** 🎟️ Actual original input owners qualify live actions, producers and wires before their receiving stages. */
import schema from "./📐️.json";
import {toolJobProductionSource,toolJobRustBlock} from "../../📋️inventory/🟦️.ts";
const compact=(text:string)=>text.replace(/\s+/g,"");
const body=(source:string,name:string):string=>{
 const matches=[...source.matchAll(new RegExp("\\bfn\\s+"+name+"(?:<[^\\n]*?>)?\\s*\\(","g"))];
 if(matches.length!==1)return "";
 return compact(toolJobRustBlock(source,source.indexOf("{",matches[0]!.index!))?.body??"");
};
const ordered=(source:string,parts:readonly string[]):boolean=>{
 let at=-1;
 for(const part of parts){at=source.indexOf(part,at+1);if(at<0)return false}
 return true;
};
const owner=(source:string):string=>{
 const matches=[...source.matchAll(/\bimpl\s*<[^\n]*>\s*OriginalTypedCommandSource\s*</g)];
 if(matches.length!==1)return "";
 return toolJobRustBlock(source,source.indexOf("{",matches[0]!.index!))?.body??"";
};
/** 🔍️ Facts use the current compiler-owned action, typed source and actual direct intent bodies. */
export function toolJobOriginalInputEvidence(raw:string):Record<string,boolean>{
 const source=toolJobProductionSource(raw),typed=owner(source);
 const action=body(source,"dispatch_action_received"),command=body(source,"dispatch_command"),host=body(source,"drive_original_host_configuration");
 const prepare=body(source,"prepare_original_typed_action"),producer=body(typed,"produce_action"),validate=body(typed,"validate"),admit=body(typed,"admit"),wire=body(typed,"admit_wire"),intent=body(source,"handle_intent_frame");
 const direct=body(source,"capture_original_direct_tool_tag"),qualify=body(typed,"qualify_action");
 const actionAt=action.indexOf("self.tools.registry.get(action)"),commandAt=command.indexOf("validate_ui_dispatch_classification(&owner,command_id");
 const hostTail=host.indexOf(".finish_host(");
 const scoped=host+prepare+producer+intent+direct+qualify;
 return {
  actionRegistry:actionAt>=0&&ordered(action.slice(actionAt),["self.tools.registry.get(action)","validate_ui_dispatch_classification(","A::original_host_configuration(action)","self.prepare_original_action_source("])&&!action.includes("A::command_from_action("),
  commandOwnership:command.includes("app_id!=&owner_app_id")&&command.includes("active_mode_id!=Some(mode_id.as_str())")&&command.includes("self.tools.registry.app_command(command_id)")&&command.includes("self.tools.registry.mode_command(mode_id,command_id)")&&commandAt>=0&&ordered(command.slice(commandAt),["validate_ui_dispatch_classification(","A::original_host_configuration(command_id)","self.prepare_original_action_source("]),
  hostAdmission:ordered(host,["A::original_host_configuration(action)",".admit_host(",".pin_host_metadata(",".prepare_original_tool_dispatch(",".advance_host("])&&!host.includes("host_configuration_mutation(")&&!host.includes("OpBinary::encode_op("),
  hostSettlement:ordered(host,[".advance_host(",".settle_press(",".finish_host("])&&host.includes("&mutsource.original_press_result")&&hostTail>=0&&host.indexOf("self.drive_original_emission(cx)",hostTail)>hostTail,
  typedProducer:ordered(prepare,["self.original_typed_command.admit(cx)","A::original_action_command_producer()",".produce_action("])&&ordered(producer,["self.validate(cx)?","cx.original_source_parent()?.admit_loan(cx)","factory.admit(cx,&mutself.producer)","producer.advance(action,args,fault,cx)","producer.take_into(&mutself.command,&mutself.wire,cx)"])&&producer.includes("self.action.is_some_and(")&&producer.includes("factoryas*const"),
  typedIdentity:validate.includes("self.binding.is_some_and(|source|source.matches(cx))")&&validate.includes("self.decoder!=cx.native_receiving_identity()")&&validate.includes("caller.validate_context(cx)?")&&admit.includes("OriginalSourceBinding::borrow(cx)")&&ordered(admit,["admit_original_construction(","self.binding=Some(binding)","self.decoder=Some((control.ledger_identity(),control.retirement_recipient_identity()"]),
  qualifiedWire:wire.includes("self.validate(cx)?")&&wire.includes("self.catalog.is_some_and(|original|original!=identity)")&&wire.includes("qualification.take_into(&mutself.registration,cx)")&&wire.includes("OriginalToolQualification::admit(catalog,verb,cx,&mutself.qualification)")&&wire.includes("ifself.registration.is_some(){OriginalCommandAdmission::admit::<A>(&mutself.registration,&mutself.wire,verb,cx,&mutself.admitting)")&&!wire.includes("serde_json::"),
  intentSource:intent.split("context.capture_original_direct_tool_tag(").length===3&&ordered(intent,["intent.action.version!=1","ifcontext.original_typing.pending(){matchcontext.capture_original_direct_tool_tag(","ifcontext.original_emission.input.is_some()","context.capture_original_direct_tool_tag(","context.prepare_original_typed_action("])&&!intent.includes("A::command_from_intent")&&!intent.includes("serde_json::to_value"),
  directQualification:ordered(direct,["self.original_typed_command.admit(cx)?","DeclaredToolCatalogAlias::source","self.original_typed_command.qualify_action(catalog,action,cx)?","typed.prepare_original_tool_dispatch("])&&qualify.includes("self.validate(cx)?")&&qualify.includes("self.catalog.is_some_and(|old|old!=identity)")&&qualify.includes("self.qualified_action.is_some_and(|old|old!=verb)")&&qualify.includes("OriginalToolQualification::admit(catalog,action,cx,&mutself.qualification)")&&qualify.includes("qualification.advance(action,cx)?")&&qualify.includes("qualification.take_into(&mutself.registration,cx)?")&&qualify.includes("row.owner!=semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness::of::<A>()")&&qualify.includes("row.key.tool_id!=action")&&qualify.includes("admission.verb!=action")&&qualify.includes("!admitting.matches_registered_action(action)")&&!qualify.includes("A::original_action_command_producer("),
  noReconstruction:scoped.length>0&&!/StepContext::new|StepContext\s*\{|OriginalJobFixtureCaller|StepBudget::new/.test(scoped)
 };
}
/** 🔐️ Every required current input-admission fact is mandatory, including absent-owner refusal. */
export function toolJobOriginalInputAdmissionExact(source:string):boolean{
 const evidence=toolJobOriginalInputEvidence(source);
 return schema.required.every(key=>evidence[key]===true);
}
