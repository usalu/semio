/** 📍️ Actual GPOS mark and base anchors retain borrowed font bytes while probing one record per turn. */
import {FontReader,type FontFace,type FontProgress} from "../../🟦️.ts";
import {FontIndexCursor} from "../../🤝️kerning/🟦️.ts";
import type {FontLookup} from "../🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../../🧹️retire/🟦️.ts";
export type FontMarkAttachment={x:number;y:number};
export class FontMarkJob{
 private reader:FontReader|null=null;private phase="subtable";private sub=0;private at=0;private probe:FontIndexCursor|null=null;private markIndex=-1;private baseIndex=-1;private markClass=0;private markAnchor=0;private baseAnchor=0;private markX=0;private markY=0;private output:FontMarkAttachment|null=null;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private face:FontFace,private lookup:FontLookup,private base:number,private mark:number,private maxWork:number){if(!face.gpos||lookup.kind!==4||lookup.flags!==0||base<0||base>=face.glyphs||mark<0||mark>=face.glyphs||!Number.isSafeInteger(maxWork)||maxWork<1||maxWork>1e9)this.failure=Error("Invalid font mark authority");else this.reader=new FontReader(face.bytes,face.gpos.offset,face.gpos.offset+face.gpos.length);}
 private nextSub():void{this.sub++;this.probe=null;this.phase="subtable";}
 private anchor(at:number):readonly[number,number]{const r=this.reader!,format=r.u16(at);if(format<1||format>3)throw Error("Unsupported font anchor format");r.range(at,format===1?6:format===2?8:10);return[r.i16(at+2),r.i16(at+4)];}
 private step():void{const r=this.reader!;switch(this.phase){
  case "subtable":if(this.sub===this.lookup.subtables){this.phase="complete";break;}this.at=this.lookup.offset+r.u16(this.lookup.offset+6+this.sub*2);if(r.u16(this.at)!==1)throw Error("Unsupported font mark attachment format");this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+2),this.mark);this.phase="markCoverage";break;
  case "markCoverage":if(!this.probe!.step())break;this.markIndex=this.probe!.result();this.probe=null;if(this.markIndex<0){this.nextSub();break;}this.probe=new FontIndexCursor(r,this.at+r.u16(this.at+4),this.base);this.phase="baseCoverage";break;
  case "baseCoverage":if(!this.probe!.step())break;this.baseIndex=this.probe!.result();this.probe=null;if(this.baseIndex<0){this.nextSub();break;}this.phase="markRecord";break;
  case "markRecord":{const array=this.at+r.u16(this.at+8),count=r.u16(array);if(this.markIndex>=count)throw Error("Font mark index exceeds array authority");const record=array+2+this.markIndex*4;this.markClass=r.u16(record);const offset=r.u16(record+2);if(this.markClass>=r.u16(this.at+6)||offset===0)throw Error("Font mark anchor authority invalid");this.markAnchor=array+offset;this.phase="baseRecord";}break;
  case "baseRecord":{const array=this.at+r.u16(this.at+10),count=r.u16(array),classes=r.u16(this.at+6);if(this.baseIndex>=count||classes===0)throw Error("Font base index exceeds array authority");const offset=r.u16(array+2+(this.baseIndex*classes+this.markClass)*2);if(!offset){this.nextSub();break;}this.baseAnchor=array+offset;this.phase="markAnchor";}break;
  case "markAnchor":[this.markX,this.markY]=this.anchor(this.markAnchor);this.phase="baseAnchor";break;
  case "baseAnchor":{const[x,y]=this.anchor(this.baseAnchor);this.output={x:x-this.markX,y:y-this.markY};this.phase="complete";}break;
  default:throw Error("Font mark attachment stage invalid");
 }}
 advance(grant:number):FontProgress{if(!Number.isSafeInteger(grant)||grant<1)throw Error("Invalid font mark work grant");if(this.aborted||this.transferred)throw Error("Font mark cancelled");if(this.failure)throw this.failure;try{for(let turn=0;turn<grant&&this.phase!=="complete";turn++){if(this.work===this.maxWork)throw Error("Font mark work limit exceeded");this.step();this.work++;}}catch(error){this.failure=error;throw error;}return{phase:this.phase,work:this.work,done:this.phase==="complete"};}
 result():FontMarkAttachment|null{if(this.phase!=="complete"||this.aborted||this.transferred||this.failure)throw Error("Font mark attachment incomplete");return this.output;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:WorkRetirement;output:FontMarkAttachment|null}{if(this.transferred)throw Error("Font mark ownership already transferred");const output=!this.aborted&&!this.failure&&this.phase==="complete"?this.result():null;this.transferred=true;this.aborted=true;return{output,job:new UnitRetirement(()=>{this.probe=null;this.reader=null;this.face=null as unknown as FontFace;this.output=null;return true;})};}
}
