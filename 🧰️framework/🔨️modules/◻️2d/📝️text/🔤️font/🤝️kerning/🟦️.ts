import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
/** 🤝️ Original GPOS pair records and kern table pairs resolve under bounded work grants without shaping caches. */
import {FontReader,validateFontSource,type FontFace,type FontProgress} from "../🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkRetirementCursor,retireRetainedWorkBuffer,RetainedWorkBuffer,RetainedWorkLedger} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
/** 🔎️ Resolves one OpenType coverage index or class value with a single binary-search probe per turn. */

export class FontIndexCursor{
 private format:number;private low=0;private high=0;private value:number;private done=false;
 constructor(private r:FontReader,private at:number,private glyph:number,private classDef=false){
  this.format=r.u16(at);this.value=classDef?0:-1;
  if(!classDef&&this.format===1){this.high=r.u16(at+2);r.range(at+4,this.high*2);}
  else if(this.format===2){this.high=r.u16(at+2);r.range(at+4,this.high*6);}
  else if(classDef&&this.format===1){const from=r.u16(at+2),count=r.u16(at+4);r.range(at+6,count*2);if(glyph>=from&&glyph-from<count)this.value=r.u16(at+6+(glyph-from)*2);this.done=true;}
  else throw Error("Unsupported font index format");
 }
 step(work:RetainedWorkLedger):boolean{if(!work.canStep())return false;
  if(this.done)return true;
  if(this.low===this.high){this.done=true;return true;}
  const n=Math.floor((this.low+this.high)/2);let from:number,to:number,value:number;
  if(this.format===1){from=to=this.r.u16(this.at+4+n*2);value=n;}
  else{const at=this.at+4+n*6;from=this.r.u16(at);to=this.r.u16(at+2);value=this.r.u16(at+4)+(this.classDef?0:Math.max(0,this.glyph-from));}
  if(this.glyph<from)this.high=n;else if(this.glyph>to)this.low=n+1;else{this.value=value;this.done=true;}
  return this.done;
 }
 result():number{return this.value;}
}
const popcount=(n:number):number=>{let total=0;while(n){total+=n&1;n>>>=1;}return total;};
const pairValue=(r:FontReader,at:number,format:number):number=>{if(format&0xff00)throw Error("Unsupported font pair value");if(!(format&4))return 0;return r.i16(at+popcount(format&3)*2);};
/** 💼️ Sums every `kern` pair adjustment of the Latin script, falling back to the `kern` table. */
export class FontKerningJob{
 private producer=new RetainedWorkProducerRuntime();private pendingIndex=0;private searchAt=0;private sortProbe=0;private sortLeft=0;private sortRight=0;private probeReady=false;
 private phase="start";private reader:FontReader|null=null;private kernReader:FontReader|null=null;private scripts=0;private features=0;private lookups=0;private count=0;private at=0;private script=0;private fallback=0;private lang=0;private record=0;private recordCount=0;private recordAt=0;private selected=new RetainedWorkBuffer<number>(8);private selectedAt=0;private lookup=0;private lookupKind=0;private subCount=0;private subAt=0;private sub=0;private format=0;private leftSize=0;private rightSize=0;private leftFormat=0;private rightFormat=0;private probe:FontIndexCursor|null=null;private cls=0;private low=0;private high=0;private set=0;private flags=0;private total=0;private output=0;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private face:FontFace,private left:number,private right:number,private maxWork:number){
  if(!Number.isInteger(left)||!Number.isInteger(right)||left<0||right<0||left>=face.glyphs||right>=face.glyphs||!Number.isSafeInteger(maxWork)||maxWork<1||maxWork>1e9)this.failure=Error("Invalid font kerning authority");
  else{if(face.gpos)this.reader=new FontReader(face.bytes,face.gpos.offset,face.gpos.offset+face.gpos.length);if(face.kern)this.kernReader=new FontReader(face.bytes,face.kern.offset,face.kern.offset+face.kern.length);}
 }
 private kernTable():void{
  const r=this.kernReader;if(!r){this.output=this.total;this.phase="complete";return;}
  const table=this.face.kern!;if(r.u16(table.offset)!==0)throw Error("Unsupported font kern table");this.count=r.u16(table.offset+2);if(this.count>128)throw Error("Font kern subtable limit exceeded");this.sub=table.offset+4;this.at=0;this.phase="kernTable";
 }
 private stepKernTable():void{
  const r=this.kernReader!;
  if(this.phase==="kernTable"){if(this.at===this.count){this.output=this.total;this.phase="complete";return;}const sub=this.sub,length=r.u16(sub+2);this.flags=r.u16(sub+4);if(length<6)throw Error("Invalid font kern subtable length");this.at++;this.sub=sub+length;if((this.flags&1)&&this.flags>>>8===0){const pairs=r.u16(sub+6);r.range(sub+14,pairs*6);this.set=sub;this.low=0;this.high=pairs;this.phase="kernTablePairs";}return;}
  if(this.low===this.high){this.phase="kernTable";return;}
  const n=Math.floor((this.low+this.high)/2),at=this.set+14+n*6,key=this.left*65536+this.right,found=r.u32(at);
  if(key<found)this.high=n;else if(key>found)this.low=n+1;else{const pair=r.i16(at+4);this.total=this.flags&8?pair:this.total+pair;this.phase="kernTable";}
 }
 private step(context:RetainedWorkProducerStep):void{const work=context.work;if(!work.canStep())return;validateFontSource(this.face);
  if(this.phase==="selectionPublish"){if(!context.slot(this.selected))return;if(this.selected.length===32)throw Error("Font kerning lookup limit exceeded");context.push(this.selected,this.pendingIndex);this.sortProbe=this.selected.length-1;this.phase="selectionSort";return;}
  if(this.phase==="sortFirst"){if(!context.write(this.selected,this.sortProbe-1,this.sortRight))return;this.phase="sortSecond";return;}
  if(this.phase==="sortSecond"){if(!context.write(this.selected,this.sortProbe,this.sortLeft))return;this.sortProbe--;this.phase="selectionSort";return;}
  if(this.phase==="classLeft"&&this.probeReady||this.phase==="selectionSort"&&this.sortProbe===0||this.phase==="selectionSearch"&&this.searchAt===this.selected.length||this.phase==="lookupOpen"&&this.selectedAt===this.selected.length)context.commitControlWork();else context.commitSemanticWork();

  if(this.phase==="start"){const r=this.reader;if(!r)return this.kernTable();const table=this.face.gpos!;if(r.u16(table.offset)!==1)throw Error("Unsupported font positioning version");this.scripts=table.offset+r.u16(table.offset+4);this.features=table.offset+r.u16(table.offset+6);this.lookups=table.offset+r.u16(table.offset+8);this.count=r.u16(this.scripts);if(this.count>128)throw Error("Font script limit exceeded");this.phase="script";return;}
  if(this.phase==="kernTable"||this.phase==="kernTablePairs")return this.stepKernTable();
  const r=this.reader!;
  switch(this.phase){
   case "script":{if(this.at<this.count){const at=this.scripts+2+this.at*6;this.at++;const table=this.scripts+r.u16(at+4),tag=r.u32(at);if(tag===0x6c61746e)this.script=table;else if((tag===0x44464c54||tag===0x64666c74)&&this.fallback===0)this.fallback=table;}if(this.script!==0||this.at===this.count){const script=this.script!==0?this.script:this.fallback;if(script===0)return this.kernTable();const lang=r.u16(script);if(lang===0){this.output=0;this.phase="complete";return;}this.lang=script+lang;this.recordCount=r.u16(this.lang+4);if(this.recordCount>128)throw Error("Font feature limit exceeded");this.at=0;this.phase="feature";}break;}
   case "feature":{if(this.at>this.recordCount){this.selectedAt=0;this.phase="lookupOpen";break;}const feature=this.at===this.recordCount?r.u16(this.lang+2):r.u16(this.lang+6+this.at*2);this.at++;if(feature===0xffff)break;if(feature>=r.u16(this.features))throw Error("Font feature exceeds directory");const at=this.features+2+feature*6;if(r.u32(at)!==0x6b65726e)break;this.record=this.features+r.u16(at+4);const count=r.u16(this.record+2);if(count>128)throw Error("Font feature lookup limit exceeded");this.recordAt=0;this.subCount=count;this.phase="featureLookups";break;}
   case "featureLookups":{if(this.recordAt===this.subCount){this.phase="feature";break;}this.pendingIndex=r.u16(this.record+4+this.recordAt*2);this.recordAt++;if(this.pendingIndex>=r.u16(this.lookups))throw Error("Font feature lookup exceeds directory");this.searchAt=0;this.phase="selectionSearch";break;}
   case "selectionSearch":if(this.searchAt===this.selected.length){this.phase="selectionPublish";break;}if(this.selected.get(this.searchAt)===this.pendingIndex)this.phase="featureLookups";else if(++this.searchAt===this.selected.length)this.phase="selectionPublish";break;
   case "selectionSort":if(this.sortProbe===0){this.phase="featureLookups";break;}this.sortLeft=this.selected.get(this.sortProbe-1)!;this.sortRight=this.selected.get(this.sortProbe)!;this.phase=this.sortLeft>this.sortRight?"sortFirst":"featureLookups";break;
   case "lookupOpen":{if(this.selectedAt===this.selected.length){this.output=this.total;this.phase="complete";break;}const index=this.selected.get(this.selectedAt++)!;this.lookup=this.lookups+r.u16(this.lookups+2+index*2);this.lookupKind=r.u16(this.lookup);this.subCount=r.u16(this.lookup+4);if(this.subCount>128)throw Error("Font lookup subtable limit exceeded");this.subAt=0;this.phase="subtable";break;}
   case "subtable":{if(this.subAt===this.subCount){this.phase="lookupOpen";break;}let sub=this.lookup+r.u16(this.lookup+6+this.subAt*2);this.subAt++;let kind=this.lookupKind;if(kind===9){if(r.u16(sub)!==1)throw Error("Unsupported font positioning extension");kind=r.u16(sub+2);sub+=r.u32(sub+4);}if(kind!==2)throw Error("Unsupported font positioning lookup");this.sub=sub;this.format=r.u16(sub);if(this.format!==1&&this.format!==2)throw Error("Unsupported font pair format");this.leftFormat=r.u16(sub+4);this.rightFormat=r.u16(sub+6);this.leftSize=popcount(this.leftFormat)*2;this.rightSize=popcount(this.rightFormat)*2;this.probe=new FontIndexCursor(r,sub+r.u16(sub+2),this.left);this.phase="coverage";break;}
   case "coverage":{if(!this.probe!.step(work))break;const coverage=this.probe!.result();this.probe=null;if(coverage<0){this.phase="subtable";break;}if(this.format===1){if(coverage>=r.u16(this.sub+8))throw Error("Font pair set exceeds coverage");this.set=this.sub+r.u16(this.sub+10+coverage*2);this.low=0;this.high=r.u16(this.set);this.phase="pairSet";}else{const at=this.sub+r.u16(this.sub+8);this.probeReady=r.u16(at)===1;this.probe=new FontIndexCursor(r,at,this.left,true);this.phase="classLeft";}break;}
   case "pairSet":{if(this.low===this.high){this.phase="subtable";break;}const n=Math.floor((this.low+this.high)/2),at=this.set+2+n*(2+this.leftSize+this.rightSize),glyph=r.u16(at);if(this.right<glyph)this.high=n;else if(this.right>glyph)this.low=n+1;else{this.total+=pairValue(r,at+2,this.leftFormat)+pairValue(r,at+2+this.leftSize,this.rightFormat);this.phase="lookupOpen";}break;}
   case "classLeft":{if(!this.probeReady&&!this.probe!.step(work))break;this.cls=this.probe!.result();this.probe=null;this.probeReady=false;this.phase="classRightBirth";break;}
   case "classRightBirth":{const at=this.sub+r.u16(this.sub+10);this.probeReady=r.u16(at)===1;this.probe=new FontIndexCursor(r,at,this.right,true);this.phase="classRight";break;}
   case "classRight":{if(!this.probeReady&&!this.probe!.step(work))break;this.probeReady=false;const right=this.probe!.result(),left=this.cls;this.probe=null;const rows=r.u16(this.sub+12),columns=r.u16(this.sub+14);if(left>=rows||right>=columns)throw Error("Font pair class exceeds matrix");const at=this.sub+16+(left*columns+right)*(this.leftSize+this.rightSize);this.total+=pairValue(r,at,this.leftFormat)+pairValue(r,at+this.leftSize,this.rightFormat);this.phase="lookupOpen";break;}
   default:throw Error("Font kerning stage invalid");
  }
 }
 settlementPending():boolean{return this.phase==="selectionPublish"||this.phase==="sortFirst"||this.phase==="sortSecond"||this.phase==="selectionSort"&&this.sortProbe===0||this.phase==="selectionSearch"&&this.searchAt===this.selected.length||this.phase==="classLeft"&&this.probeReady||this.phase==="lookupOpen"&&this.selectedAt===this.selected.length||this.phase==="complete";}
 failurePending():boolean{return this.failure!==null;}
 lastTurnWork():number{return this.producer.lastTurnWork();}
 lastTurnSemanticWork():number{return this.producer.lastTurnSemanticWork();}
 advance(grant:number,work:RetainedWorkLedger):FontProgress{try{return this.producer.advance(grant,work,{validate:()=>{if(this.aborted||this.transferred)throw Error("Font kerning cancelled");if(this.failure)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,maximumWork:this.maxWork,workLimitMessage:"Font kerning work limit exceeded",recordSemanticWork:units=>{this.work+=units;},step:context=>this.step(context),progress:()=>({phase:this.phase,work:this.work,done:this.phase==="complete"})});}catch(error){this.failure=error;throw error;}}

 result():number{if(this.phase!=="complete"||this.aborted||this.transferred||this.failure)throw Error("Font kerning incomplete");return this.output;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:number|null}{if(this.transferred)throw Error("Font kerning ownership already transferred");const output=!this.aborted&&!this.failure&&this.phase==="complete"?this.result():null;this.transferred=true;this.aborted=true;return{output,job:new RetainedWorkRetirement((work,turn)=>{if(!retireRetainedWorkBuffer(this.selected,work,turn))return false;this.reader=null;turn.commitActualWork();this.kernReader=null;this.probe=null;this.face=null as unknown as FontFace;return true;})};}
}
