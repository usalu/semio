/** 🔗️ Actual ligature sets inspect one borrowed component or coverage probe per turn. */
import {FontReader,type FontFace,type FontProgress} from "../../🟦️.ts";
import {FontIndexCursor} from "../../🤝️kerning/🟦️.ts";
import type {FontLookup} from "../🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../../🧹️retire/🟦️.ts";
export type FontLigatureMatch={glyph:number;consumed:number};
export class FontLigatureJob{
 private reader:FontReader|null=null;private phase="subtable";private sub=0;private at=0;private probe:FontIndexCursor|null=null;private set=0;private ligatureAt=0;private ligatureCount=0;private ligature=0;private components=0;private componentAt=0;private output:FontLigatureMatch|null=null;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private face:FontFace,private lookup:FontLookup,private glyphs:readonly number[],private start:number,private maxWork:number){if(!face.gsub||lookup.kind!==4||lookup.flags!==0&&lookup.flags!==8||!Number.isSafeInteger(start)||start<0||start>=glyphs.length||!Number.isSafeInteger(maxWork)||maxWork<1||maxWork>1e9)this.failure=Error("Invalid font ligature authority");else this.reader=new FontReader(face.bytes,face.gsub.offset,face.gsub.offset+face.gsub.length);}
 private nextSub():void{this.sub++;this.probe=null;this.phase="subtable";}
 private step():void{const r=this.reader!;switch(this.phase){
  case "subtable":if(this.sub===this.lookup.subtables){this.phase="complete";break;}this.at=this.lookup.offset+r.u16(this.lookup.offset+6+this.sub*2);if(r.u16(this.at)!==1)throw Error("Unsupported font ligature format");this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+2),this.glyphs[this.start]!);this.phase="coverage";break;
  case "coverage":if(!this.probe!.step())break;{const coverage=this.probe!.result();this.probe=null;if(coverage<0){this.nextSub();break;}const count=r.u16(this.at+4);if(coverage>=count)throw Error("Font ligature coverage exceeds set authority");this.set=this.at+r.u16(this.at+6+coverage*2);this.ligatureCount=r.u16(this.set);r.range(this.set+2,this.ligatureCount*2);this.ligatureAt=0;this.phase="ligature";}break;
  case "ligature":if(this.ligatureAt===this.ligatureCount){this.nextSub();break;}this.ligature=this.set+r.u16(this.set+2+this.ligatureAt++*2);this.components=r.u16(this.ligature+2);if(this.components<1)throw Error("Font ligature has no components");r.range(this.ligature+4,(this.components-1)*2);if(this.components>this.glyphs.length-this.start)break;this.componentAt=1;this.phase="components";break;
  case "components":if(this.componentAt===this.components){const glyph=r.u16(this.ligature);if(glyph>=this.face.glyphs)throw Error("Font ligature glyph exceeds authority");this.output={glyph,consumed:this.components};this.phase="complete";break;}if(r.u16(this.ligature+2+this.componentAt*2)!==this.glyphs[this.start+this.componentAt]){this.phase="ligature";break;}this.componentAt++;break;
  default:throw Error("Font ligature stage invalid");
 }}
 advance(grant:number):FontProgress{if(!Number.isSafeInteger(grant)||grant<1)throw Error("Invalid font ligature work grant");if(this.aborted||this.transferred)throw Error("Font ligature cancelled");if(this.failure)throw this.failure;try{for(let turn=0;turn<grant&&this.phase!=="complete";turn++){if(this.work===this.maxWork)throw Error("Font ligature work limit exceeded");this.step();this.work++;}}catch(error){this.failure=error;throw error;}return{phase:this.phase,work:this.work,done:this.phase==="complete"};}
 result():FontLigatureMatch|null{if(this.phase!=="complete"||this.aborted||this.transferred||this.failure)throw Error("Font ligature incomplete");return this.output;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:WorkRetirement;output:FontLigatureMatch|null}{if(this.transferred)throw Error("Font ligature ownership already transferred");const output=!this.aborted&&!this.failure&&this.phase==="complete"?this.result():null;this.transferred=true;this.aborted=true;return{output,job:new UnitRetirement(()=>{this.probe=null;this.reader=null;this.glyphs=[];this.face=null as unknown as FontFace;this.output=null;return true;})};}
}
