/** 🌱️ Admits one original identifier before publishing its empty canonical presentation. */
import {ToolRunPresentation} from "../🟦️.ts";
export interface ToolRunSeedBody {toolId:string;stage:0;completed:0;total:null;counters:never[];steps:never[];marks:never[];entities:never[];payload:null}
export class ToolRunPresentationSeed {
 private phase=0;private cancelled=false;private input:string|undefined;private output:ToolRunPresentation<ToolRunSeedBody>|undefined;
 constructor(toolId:string){this.input=toolId;}
 get complete():boolean{return this.phase===3&&!this.cancelled;}
 get retained():boolean{return this.input!==undefined||this.output!==undefined;}
 get original():string|undefined{return this.input;}
 advance(items:number):number{if(items<1||this.cancelled||this.phase>=3)return 0;if(this.phase===2)this.output=new ToolRunPresentation({toolId:this.input!,stage:0,completed:0,total:null,counters:[],steps:[],marks:[],entities:[],payload:null});this.phase++;return 1;}
 take(items:number):ToolRunPresentation<ToolRunSeedBody>|undefined{if(items<1||!this.complete)return;const output=this.output;this.output=undefined;this.input=undefined;this.phase=4;return output;}
 cancel():void{this.cancelled=true;}
 close(items:number):number{if(items<1)return 0;this.cancelled=true;if(this.output){this.output.close(1);this.output=undefined;return 1;}if(this.input!==undefined){this.input=undefined;return 1;}return 0;}
}
