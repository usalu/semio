/** 🗃️ Borrows the published root while its original pending tick and completed children remain owned. */
import {type ToolRunIdentity,type ToolRunState} from "../../../🟦️.ts";
import {ToolRunPresentation,ToolRunCapturedView} from "../🟦️.ts";
import {ToolRunPresentationSeed} from "../🌱️seed/🟦️.ts";
import {ToolRunPresentationUpdate,type ToolRunPresentationFields,type ToolRunPresentationDelta} from "../🧬️update/🟦️.ts";
export class ToolRunPresentationSlot {
 private seed:ToolRunPresentationSeed|undefined;private published:ToolRunPresentation<ToolRunPresentationFields>|undefined;private update:ToolRunPresentationUpdate|undefined;private candidate:ToolRunPresentation<ToolRunPresentationFields>|undefined;private displaced:ToolRunPresentation<ToolRunPresentationFields>|undefined;private pendingCancellation=false;private cancelled=false;
 constructor(toolId:string,private identity:ToolRunIdentity,private state:ToolRunState){this.seed=new ToolRunPresentationSeed(toolId);}
 get ready():boolean{return !this.cancelled&&!this.pendingCancellation&&this.published!==undefined&&this.seed===undefined&&this.update===undefined&&this.candidate===undefined&&this.displaced===undefined;}
 get body():ToolRunPresentationFields|undefined{return this.published?.original;}
 get retained():boolean{return this.seed!==undefined||this.published!==undefined||this.update!==undefined||this.candidate!==undefined||this.displaced!==undefined;}
 capture(items:number):ToolRunPresentation<ToolRunPresentationFields>|undefined{return this.cancelled?undefined:this.published?.capture(items);}
 captureView(identity:ToolRunIdentity,state:ToolRunState,items:number):ToolRunCapturedView|undefined{const source=this.capture(items);return source?new ToolRunCapturedView(source,identity,state):undefined;}
 beginTick(input:ToolRunPresentationDelta,items:number):boolean{if(items<1||!this.ready)return false;this.update=new ToolRunPresentationUpdate(this.published!.capture(items)!,input);return true;}
 advance(items:number):number{
  if(items<1||this.cancelled||this.ready)return 0;
  if(this.displaced){this.displaced.close(items);this.displaced=undefined;return 1;}
  if(this.seed){if(this.candidate){if(this.seed.retained)return this.seed.close(items);this.seed=undefined;return 1;}if(!this.seed.complete)return this.seed.advance(items);const seed=this.seed.take(items)!;this.candidate=new ToolRunPresentation({toolId:seed.original.toolId,progress:{identity:this.identity,sequence:0n,state:this.state,stage:0,completed:0n,total:undefined,counters:[],unitsPerSecond:0,conflicts:0,steps:[]},provenance:{marks:[],entities:[]},payload:undefined});seed.close(items);return 1;}
  if(this.update){if(this.pendingCancellation||this.candidate){if(this.update.retained)return this.update.close(items);this.update=undefined;return 1;}if(!this.update.complete)return this.update.advance(items);this.candidate=this.update.take(items);return 1;}
  if(this.pendingCancellation){if(this.candidate){this.displaced=this.candidate;this.candidate=undefined;return 1;}this.pendingCancellation=false;return 1;}
  this.displaced=this.published;this.published=this.candidate;this.candidate=undefined;return 1;
 }
 cancel():void{this.cancelled=true;this.seed?.cancel();this.update?.cancel();}
 cancelPending():void{if(this.update||this.candidate){this.pendingCancellation=true;this.update?.cancel();}}
 close(items:number):number{if(items<1||!this.retained)return 0;this.cancel();if(this.update){if(this.update.retained)return this.update.close(items);this.update=undefined;return 1;}if(this.seed){if(this.seed.retained)return this.seed.close(items);this.seed=undefined;return 1;}if(this.candidate){this.candidate.close(items);this.candidate=undefined;return 1;}if(this.displaced){this.displaced.close(items);this.displaced=undefined;return 1;}this.published!.close(items);this.published=undefined;return 1;}
}
