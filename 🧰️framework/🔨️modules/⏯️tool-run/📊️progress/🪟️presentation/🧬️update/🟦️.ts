/** 🧬️ Publishes a joint private presentation only after every owning child has completed. */
import {type ToolRunProgress,type ToolRunStep,type ToolRunStepArg,type ToolRunCounter,type ToolRunIdentity} from "../../../🟦️.ts";
import {ToolRunProgressClone} from "../../📸️clone/🟦️.ts";
import {ToolRunStepRingInsert} from "../../💍️steps/➕️insert/🟦️.ts";
import {ToolRunEntityProvenanceEdit,type EntityMark} from "../../../👥️entities/🧾️provenance/🟦️.ts";
import {ToolRunPresentation} from "../🟦️.ts";
export type ToolRunPresentationFields={toolId:string,progress:ToolRunProgress,provenance:{marks:readonly EntityMark[],entities:readonly string[]},payload?:Uint8Array};
export type ToolRunPresentationDelta={identity?:ToolRunIdentity,sequence:bigint,progress?:ToolRunProgress,steps:ToolRunStep[],retractTo:number|null,end:number,append:readonly string[],payload?:Uint8Array};
export class ToolRunPresentationUpdate{
 private phase=0;
 private cancelled=false;
 private spent=false;
 private updated=false;
 private retiredCounters:ToolRunCounter[]=[];
 private index=0;
 private clone:ToolRunProgressClone|undefined;
 private provenance:ToolRunEntityProvenanceEdit|undefined;
 private ring:ToolRunStepRingInsert|undefined;
 private progress:ToolRunProgress|undefined;
 private projected:ToolRunPresentationFields["provenance"]|undefined;
 private output:ToolRunPresentation<ToolRunPresentationFields>|undefined;
 private closingStep:ToolRunStep|undefined;
 private steps:(ToolRunStep|undefined)[];
 constructor(private original:ToolRunPresentation<ToolRunPresentationFields>,private input:ToolRunPresentationDelta){this.steps=input.steps;}
 get complete(){return this.phase===8&&!this.cancelled&&!this.spent;}
 get retained(){return this.phase!==9;}
 cancel(){this.cancelled=true;this.clone?.cancel();this.provenance?.cancel();this.ring?.cancel();}
 advance(items:number){
  if(items<=0||this.cancelled||this.phase>=8)return 0;
  if(this.phase===0){const id=this.original.original.progress.identity.id,incoming=this.input.identity?.id;if(incoming&&(incoming.appInstanceId!==id.appInstanceId||incoming.run!==id.run))throw Error("Pending presentation belongs to another original run");this.clone=new ToolRunProgressClone(this.original.original.progress);this.phase=1;return 1;}
  if(this.phase===1){if(!this.clone!.complete)return this.clone!.advance(items);this.progress=this.clone!.take(items);this.phase=2;return 1;}
  if(this.phase===2){if(this.clone!.retained)return this.clone!.close(items);this.clone=undefined;this.provenance=new ToolRunEntityProvenanceEdit(this.original.original.provenance.marks,this.input.retractTo,this.input.end,this.input.append);this.phase=3;return 1;}
  if(this.phase===3){if(!this.provenance!.complete)return this.provenance!.advance(items);this.projected=this.provenance!.take(items);this.phase=4;return 1;}
  if(this.phase===4){if(this.provenance!.retained)return this.provenance!.close(items);this.provenance=undefined;this.phase=5;return 1;}
  if(this.phase===5){if(this.ring){if(!this.ring.complete)return this.ring.advance(items);this.progress={...this.progress!,steps:this.ring.take(items)!};this.phase=6;return 1;}if(this.index===this.steps.length){this.phase=7;return 1;}const step=this.steps[this.index]!;this.steps[this.index++]=undefined;const sequence=(this.progress!.steps.at(-1)?.sequence??0n)+1n;this.ring=new ToolRunStepRingInsert(this.progress!.steps as ToolRunStep[],{...step,sequence});this.progress={...this.progress!,steps:[]};return 1;}
  if(this.phase===6){if(this.ring!.retained)return this.ring!.close(items);this.ring=undefined;this.phase=5;return 1;}
  const previous=this.original.original;if(!this.updated){const update=this.input.progress;if(update){this.retiredCounters=this.progress!.counters as ToolRunCounter[];this.progress={...this.progress!,stage:update.stage,total:update.total,completed:update.total===undefined?update.completed:update.completed<update.total?update.completed:update.total,counters:update.counters};this.input.progress={...update,counters:[]};}this.progress={...this.progress!,sequence:this.input.sequence};this.updated=true;return 1;}if(this.retiredCounters.length){this.retiredCounters.pop();return 1;}this.output=new ToolRunPresentation({toolId:previous.toolId,progress:this.progress!,provenance:this.projected!,payload:this.input.payload??previous.payload});this.input.payload=undefined;this.progress=undefined;this.projected=undefined;this.phase=8;return 1;
 }
 take(items:number){if(items<=0||!this.complete)return undefined;const output=this.output;this.output=undefined;this.spent=true;return output;}
 close(items:number){
  if(items<=0||!this.retained)return 0;
  if(this.output){this.output.close(items);this.output=undefined;return 1;}
  if(this.clone){if(this.clone.retained)return this.clone.close(items);this.clone=undefined;return 1;}
  if(this.provenance){if(this.provenance.retained)return this.provenance.close(items);this.provenance=undefined;return 1;}
  if(this.ring){if(this.ring.retained)return this.ring.close(items);this.ring=undefined;return 1;}
  if(this.closingStep){if(this.closingStep.args.length){(this.closingStep.args as ToolRunStepArg[]).pop();return 1;}this.closingStep=undefined;return 1;}
  if(this.progress){const steps=this.progress.steps as ToolRunStep[];if(steps.length){this.closingStep=steps.pop();return 1;}const counters=this.progress.counters as unknown[];if(counters.length){counters.pop();return 1;}this.progress=undefined;return 1;}
  if(this.steps.length){this.closingStep=this.steps.pop();return 1;}
  if(this.retiredCounters.length){this.retiredCounters.pop();return 1;}
  if(this.input.progress){const steps=this.input.progress.steps as ToolRunStep[],counters=this.input.progress.counters as ToolRunCounter[];if(steps.length){this.closingStep=steps.pop();return 1;}if(counters.length){counters.pop();return 1;}this.input.progress=undefined;return 1;}
  if(this.input.append.length){(this.input.append as string[]).pop();return 1;}
  if(this.input.payload){this.input.payload=undefined;return 1;}
  if(this.projected){const entities=this.projected.entities as string[],marks=this.projected.marks as EntityMark[];if(entities.length){entities.pop();return 1;}if(marks.length){marks.pop();return 1;}this.projected=undefined;return 1;}
  if(this.original.retained){this.original.close(items);return 1;}
  this.phase=9;return 1;
 }
}
