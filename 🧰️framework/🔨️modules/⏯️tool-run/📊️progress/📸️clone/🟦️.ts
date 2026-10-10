/** 📊️ Clones original schema-bounded progress leaves without publishing a partial result. */
import {TOOL_RUN_COUNTERS_MAX,TOOL_RUN_STEP_RING_CAPACITY,TOOL_RUN_STEP_ARGS_MAX,type ToolRunProgress,type ToolRunCounter,type ToolRunStep,type ToolRunStepArg} from "../../🟦️.ts";
export class ToolRunProgressClone {
 private counters:ToolRunCounter[]=[];
 private steps:ToolRunStep[]=[];
 private step:(Omit<ToolRunStep,"args">&{args:ToolRunStepArg[]})|undefined;
 private index=0;
 private arg=0;
 private phase=0;
 private output:(ToolRunProgress&{counters:ToolRunCounter[],steps:ToolRunStep[]})|undefined;
 private cancelled=false;
 constructor(readonly original:ToolRunProgress){}
 get complete(){return this.phase===3&&!this.cancelled;}
 get retained(){return this.counters.length>0||this.steps.length>0||this.step!==undefined||this.output!==undefined||this.phase!==4;}
 cancel(){this.cancelled=true;}
 advance(items:number){
  if(items<=0||this.cancelled||this.phase>=3)return 0;
  if(this.original.counters.length>TOOL_RUN_COUNTERS_MAX||this.original.steps.length>TOOL_RUN_STEP_RING_CAPACITY)throw Error("Original progress exceeds schema-bounded leaves");
  if(this.phase===0){if(this.index===this.original.counters.length){this.index=0;this.phase=1;}else this.counters.push({...this.original.counters[this.index++]!});return 1;}
  if(this.phase===1){
   if(!this.step){if(this.index===this.original.steps.length){this.phase=2;return 1;}const source=this.original.steps[this.index]!;if(source.args.length>TOOL_RUN_STEP_ARGS_MAX)throw Error("Original step arguments exceed schema limit");this.step={...source,args:[]};this.arg=0;return 1;}
   const source=this.original.steps[this.index]!;if(this.arg<source.args.length){this.step.args.push({...source.args[this.arg++]!});return 1;}
   this.steps.push(this.step);this.step=undefined;this.index++;return 1;
  }
  const identity={...this.original.identity,id:{...this.original.identity.id},baseRevision:new Uint8Array(this.original.identity.baseRevision)};this.output={...this.original,identity,counters:this.counters,steps:this.steps};this.counters=[];this.steps=[];this.phase=3;return 1;
 }
 take(items:number){if(items<=0||!this.complete)return undefined;const output=this.output;this.output=undefined;this.phase=4;return output;}
 close(items:number){if(items<=0||!this.retained)return 0;if(this.output){this.counters=this.output.counters;this.steps=this.output.steps;this.output=undefined;return 1;}if(this.step){if(this.step.args.length){this.step.args.pop();return 1;}this.step=undefined;return 1;}if(this.steps.length){const step=this.steps.pop()!;this.step={...step,args:step.args as ToolRunStepArg[]};return 1;}if(this.counters.length){this.counters.pop();return 1;}this.phase=4;return 1;}
}
