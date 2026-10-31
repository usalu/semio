import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {validateFontSource} from "../../🟦️.ts";
import {RetainedWorkLedger,RetainedWorkBuffer,type RetainedWorkInputSequence} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
/** 🔗️ Actual ligature sets inspect one borrowed component or coverage probe per turn. */
import {FontReader,type FontFace,type FontProgress} from "../../🟦️.ts";
import {FontIndexCursor} from "../../🤝️kerning/🟦️.ts";
import type {FontLookup} from "../🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkRetirementCursor,retireRetainedWorkBuffer,retireRetainedWorkBuffers,retireRetainedWorkArchive,retireRetainedWorkChildren,retainedWorkChildRetirement,retireRetainedWorkMatcher} from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
export type FontLigatureMatch={glyph:number;consumed:number};
export class FontLigatureJob{
 private glyphs:RetainedWorkInputSequence<number>|null=null;
 private producer=new RetainedWorkProducerRuntime();
 private reader:FontReader|null=null;private phase="subtable";private sub=0;private at=0;private probe:FontIndexCursor|null=null;private set=0;private ligatureAt=0;private ligatureCount=0;private ligature=0;private components=0;private componentAt=0;private output:FontLigatureMatch|null=null;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private face:FontFace,private lookup:FontLookup,glyphs:RetainedWorkInputSequence<number>,private start:number,private maxWork:number){this.glyphs=glyphs;if(!face.gsub||lookup.kind!==4||lookup.flags!==0&&lookup.flags!==8||!Number.isSafeInteger(start)||start<0||start>=glyphs.length||!Number.isSafeInteger(maxWork)||maxWork<1||maxWork>1e9)this.failure=Error("Invalid font ligature authority");else this.reader=new FontReader(face.bytes,face.gsub.offset,face.gsub.offset+face.gsub.length);}
 private glyphInput():RetainedWorkInputSequence<number>{if(this.glyphs===null)throw Error("Ligature glyph source retired");return this.glyphs;}
 private nextSub():void{this.sub++;this.probe=null;this.phase="subtable";}
 private step(context:RetainedWorkProducerStep):void{const work=context.work;if(!work.canStep())return;context.commitSemanticWork();validateFontSource(this.face);const r=this.reader!;switch(this.phase){
  case "subtable":if(this.sub===this.lookup.subtables){this.phase="complete";break;}this.at=this.lookup.offset+r.u16(this.lookup.offset+6+this.sub*2);if(r.u16(this.at)!==1)throw Error("Unsupported font ligature format");this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+2),this.glyphInput().at(this.start)!);this.phase="coverage";break;
  case "coverage":if(!this.probe!.step(work))break;{const coverage=this.probe!.result();this.probe=null;if(coverage<0){this.nextSub();break;}const count=r.u16(this.at+4);if(coverage>=count)throw Error("Font ligature coverage exceeds set authority");this.set=this.at+r.u16(this.at+6+coverage*2);this.ligatureCount=r.u16(this.set);r.range(this.set+2,this.ligatureCount*2);this.ligatureAt=0;this.phase="ligature";}break;
  case "ligature":if(this.ligatureAt===this.ligatureCount){this.nextSub();break;}this.ligature=this.set+r.u16(this.set+2+this.ligatureAt++*2);this.components=r.u16(this.ligature+2);if(this.components<1)throw Error("Font ligature has no components");r.range(this.ligature+4,(this.components-1)*2);if(this.components>this.glyphInput().length-this.start)break;this.componentAt=1;this.phase="components";break;
  case "components":if(this.componentAt===this.components){const glyph=r.u16(this.ligature);if(glyph>=this.face.glyphs)throw Error("Font ligature glyph exceeds authority");this.output={glyph,consumed:this.components};this.phase="complete";break;}if(r.u16(this.ligature+2+this.componentAt*2)!==this.glyphInput().at(this.start+this.componentAt)){this.phase="ligature";break;}this.componentAt++;break;
  default:throw Error("Font ligature stage invalid");
 }}
 settlementPending():boolean{return this.phase==="complete";}
 failurePending():boolean{return this.failure!==null;}
 lastTurnWork():number{return this.producer.lastTurnWork();}
 lastTurnSemanticWork():number{return this.producer.lastTurnSemanticWork();}
 advance(grant:number,work:RetainedWorkLedger):FontProgress{try{return this.producer.advance(grant,work,{validate:()=>{if(this.aborted||this.transferred)throw Error("Font ligature cancelled");if(this.failure)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,maximumWork:this.maxWork,workLimitMessage:"Font ligature work limit exceeded",recordSemanticWork:units=>{this.work+=units;},step:context=>this.step(context),progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}catch(error){this.failure=error;throw error;}}

 result():FontLigatureMatch|null{if(this.phase!=="complete"||this.aborted||this.transferred||this.failure)throw Error("Font ligature incomplete");return this.output;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:FontLigatureMatch|null}{if(this.transferred)throw Error("Font ligature ownership already transferred");const output=!this.aborted&&!this.failure&&this.phase==="complete"?this.result():null;this.transferred=true;this.aborted=true;return{output,job:new RetainedWorkRetirement((work,turn)=>{this.probe=null;turn.commitActualWork();this.reader=null;this.glyphs=null;this.face=null as unknown as FontFace;this.output=null;return true;})};}
}
