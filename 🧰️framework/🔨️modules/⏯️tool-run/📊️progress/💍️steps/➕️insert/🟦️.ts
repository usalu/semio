/** 💍️ Owns the unpublished original ring and displaced steps through yielded insertion and cancellation. */
import {TOOL_RUN_STEP_ARGS_MAX,TOOL_RUN_STEP_RING_CAPACITY,type ToolRunStep,type ToolRunStepArg} from "../../../🟦️.ts";
export class ToolRunStepRingInsert {
 private phase=0;
 private index=0;
 private equal=true;
 private cancelled=false;
 private displaced:ToolRunStep|undefined;
 private input:ToolRunStep|undefined;
 private ring:(ToolRunStep|undefined)[];
 constructor(ring:ToolRunStep[],input:ToolRunStep){this.ring=ring;this.input=input;}
 get complete(){return this.phase===5&&!this.cancelled;}
 get retained(){return this.phase!==6||this.ring.length>0||this.input!==undefined||this.displaced!==undefined;}
 cancel(){this.cancelled=true;}
 advance(items:number){
  if(items<=0||this.cancelled||this.phase>=5)return 0;
  if(this.ring.length>TOOL_RUN_STEP_RING_CAPACITY||(this.input&&this.input.args.length>TOOL_RUN_STEP_ARGS_MAX))throw Error("Original step ring exceeds schema bounds");
  const newest=this.ring.at(-1),input=this.input!;
  if(this.phase===0){if(!newest){this.phase=4;return 1;}const fields=[newest.kind===input.kind,newest.stage===input.stage,newest.reason===input.reason,newest.subject===input.subject,newest.args.length===input.args.length];const equal=fields[this.index++]!;this.equal&&=equal;if(this.index===fields.length){this.index=0;this.phase=this.equal?1:2;}return 1;}
  if(this.phase===1){if(this.index===input.args.length){this.ring[this.ring.length-1]={...newest!,sequence:input.sequence,repeat:Math.min(0xffffffff,newest!.repeat+input.repeat)};this.displaced=input;this.input=undefined;this.phase=3;}else{const a=newest!.args[this.index]!,b=input.args[this.index++]!;this.equal&&="unsigned" in a?"unsigned" in b&&a.unsigned===b.unsigned:"float" in b&&Object.is(a.float,b.float);if(!this.equal)this.phase=2;}return 1;}
  if(this.phase===2){if(this.ring.length===TOOL_RUN_STEP_RING_CAPACITY){if(!this.displaced){this.displaced=this.ring[0];this.ring[0]=undefined;this.index=1;}if(this.index<this.ring.length){this.ring[this.index-1]=this.ring[this.index];this.ring[this.index++]=undefined;return 1;}this.ring.pop();this.phase=3;}else this.phase=4;return 1;}
  if(this.phase===3){if(this.displaced!.args.length){(this.displaced!.args as ToolRunStepArg[]).pop();return 1;}this.displaced=undefined;this.phase=this.input?4:5;return 1;}
  this.ring.push(input);this.input=undefined;this.phase=5;return 1;
 }
 take(items:number){if(items<=0||!this.complete)return undefined;const ring=this.ring as ToolRunStep[];this.ring=[];this.phase=6;return ring;}
 close(items:number){if(items<=0||!this.retained)return 0;if(this.displaced){if(this.displaced.args.length){(this.displaced.args as ToolRunStepArg[]).pop();return 1;}this.displaced=undefined;return 1;}if(this.input){this.displaced=this.input;this.input=undefined;return 1;}if(this.ring.length){this.displaced=this.ring.pop();return 1;}this.phase=6;return 1;}
}
