import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {validateFontSource} from "../../🟦️.ts";
import {RetainedWorkLedger,RetainedWorkBuffer} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
/** 📍️ Actual GPOS mark and base anchors retain borrowed font bytes while probing one record per turn. */
import {FontReader,type FontFace,type FontProgress} from "../../🟦️.ts";
import {FontIndexCursor} from "../../🤝️kerning/🟦️.ts";
import type {FontLookup} from "../🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkRetirementCursor,retireRetainedWorkBuffer,retireRetainedWorkBuffers,retireRetainedWorkArchive,retireRetainedWorkChildren,retainedWorkChildRetirement,retireRetainedWorkMatcher} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
export type FontMarkAttachment={x:number;y:number};
export class FontMarkJob{
 private producer=new RetainedWorkProducerRuntime();private anchorX=0;private anchorY=0;
 private reader:FontReader|null=null;private phase="subtable";private sub=0;private at=0;private probe:FontIndexCursor|null=null;private markIndex=-1;private baseIndex=-1;private markClass=0;private markAnchor=0;private baseAnchor=0;private markX=0;private markY=0;private output:FontMarkAttachment|null=null;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private face:FontFace,private lookup:FontLookup,private base:number,private mark:number,private maxWork:number){if(!face.gpos||lookup.kind!==4||lookup.flags!==0||base<0||base>=face.glyphs||mark<0||mark>=face.glyphs||!Number.isSafeInteger(maxWork)||maxWork<1||maxWork>1e9)this.failure=Error("Invalid font mark authority");else this.reader=new FontReader(face.bytes,face.gpos.offset,face.gpos.offset+face.gpos.length);}
 private nextSub():void{this.sub++;this.probe=null;this.phase="subtable";}
 private anchor(at:number):void{const r=this.reader!,format=r.u16(at);if(format<1||format>3)throw Error("Unsupported font anchor format");r.range(at,format===1?6:format===2?8:10);this.anchorX=r.i16(at+2);this.anchorY=r.i16(at+4);}
 private step(context:RetainedWorkProducerStep):void{const work=context.work;if(!work.canStep())return;context.commitSemanticWork();validateFontSource(this.face);const r=this.reader!;switch(this.phase){
  case "subtable":if(this.sub===this.lookup.subtables){this.phase="complete";break;}this.at=this.lookup.offset+r.u16(this.lookup.offset+6+this.sub*2);if(r.u16(this.at)!==1)throw Error("Unsupported font mark attachment format");this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+2),this.mark);this.phase="markCoverage";break;
  case "markCoverage":if(!this.probe!.step(work))break;this.markIndex=this.probe!.result();this.probe=null;if(this.markIndex<0){this.nextSub();break;}this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+4),this.base);this.phase="baseCoverage";break;
  case "baseCoverage":if(!this.probe!.step(work))break;this.baseIndex=this.probe!.result();this.probe=null;if(this.baseIndex<0){this.nextSub();break;}this.phase="markRecord";break;
  case "markRecord":{const array=this.at+r.u16(this.at+8),count=r.u16(array);if(this.markIndex>=count)throw Error("Font mark index exceeds array authority");const record=array+2+this.markIndex*4;this.markClass=r.u16(record);const offset=r.u16(record+2);if(this.markClass>=r.u16(this.at+6)||offset===0)throw Error("Font mark anchor authority invalid");this.markAnchor=array+offset;this.phase="baseRecord";}break;
  case "baseRecord":{const array=this.at+r.u16(this.at+10),count=r.u16(array),classes=r.u16(this.at+6);if(this.baseIndex>=count||classes===0)throw Error("Font base index exceeds array authority");const offset=r.u16(array+2+(this.baseIndex*classes+this.markClass)*2);if(!offset){this.nextSub();break;}this.baseAnchor=array+offset;this.phase="markAnchor";}break;
  case "markAnchor":this.anchor(this.markAnchor);this.markX=this.anchorX;this.markY=this.anchorY;this.phase="baseAnchor";break;
  case "baseAnchor":{this.anchor(this.baseAnchor);this.output={x:this.anchorX-this.markX,y:this.anchorY-this.markY};this.phase="complete";}break;
  default:throw Error("Font mark attachment stage invalid");
 }}
 settlementPending():boolean{return this.phase==="complete";}
 failurePending():boolean{return this.failure!==null;}
 lastTurnWork():number{return this.producer.lastTurnWork();}
 lastTurnSemanticWork():number{return this.producer.lastTurnSemanticWork();}
 advance(grant:number,work:RetainedWorkLedger):FontProgress{try{return this.producer.advance(grant,work,{validate:()=>{if(this.aborted||this.transferred)throw Error("Font mark cancelled");if(this.failure)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,maximumWork:this.maxWork,workLimitMessage:"Font mark work limit exceeded",recordSemanticWork:units=>{this.work+=units;},step:context=>this.step(context),progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}catch(error){this.failure=error;throw error;}}

 result():FontMarkAttachment|null{if(this.phase!=="complete"||this.aborted||this.transferred||this.failure)throw Error("Font mark attachment incomplete");return this.output;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:FontMarkAttachment|null}{if(this.transferred)throw Error("Font mark ownership already transferred");const output=!this.aborted&&!this.failure&&this.phase==="complete"?this.result():null;this.transferred=true;this.aborted=true;return{output,job:new RetainedWorkRetirement((work,turn)=>{this.probe=null;turn.commitActualWork();this.reader=null;this.face=null as unknown as FontFace;this.output=null;return true;})};}
}
