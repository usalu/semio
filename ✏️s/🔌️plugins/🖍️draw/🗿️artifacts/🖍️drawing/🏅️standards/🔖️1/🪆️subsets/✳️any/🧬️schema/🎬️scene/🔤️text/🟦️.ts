/** 🔤️ Real Catalog and Run output replace only genuine Scene cells under the original caller ledger. */
import {DRAWING_FONT_FAMILIES,parseDrawingFontFamily,drawingFontCatalogFamily,type DrawingFontFamily} from "../../📝️text/🔤️family/🟦️.ts";
import {ScenePlanOwner,ScenePlanContentReplaceJob,type ScenePlanRead,type ScenePlanContent,type ScenePlanNode} from "../🗺️plan-model/🎟️owned/🟦️.ts";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {FontCatalogAdmissionJob,type FontByteSource} from "@semio-tech/framework/2d/text/font/catalog";
import {UnicodeDataAdmissionJob,type UnicodeData} from "@semio-tech/framework/2d/text/font/shape/unicode";
import {FontOutlineRunJob,type FontOutlineRun,type FontRunInput} from "@semio-tech/framework/2d/text/font/run";
import type {FontFace,FontOutlineLimits} from "@semio-tech/framework/2d/text/font";
import {RetainedWorkBuffer,RetainedWorkLedger,RetainedWorkRetirement,retireRetainedWorkBuffer,type RetainedWorkRetirementCursor,type RetainedWorkDemand,type RetainedWorkTurn} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
export const DOCUMENT_FONT_LIMITS:FontOutlineLimits={maxFontBytes:67108864,maxTables:128,maxGlyphs:65536,maxPoints:1048576,maxContours:65536,maxComponents:262144,maxDepth:32,maxSegments:65536,maxWork:1e9};
export type DocumentTextProgress={phase:"nodes"|"fontSources"|"unicode"|"fonts"|"run"|"geometryAdmission"|"geometry"|"publishing"|"complete";nodes:number;work:number;done:boolean};
function drainCursor(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
/** 🧶️ Borrowed bytes remain caller-owned while every produced cell and pending child retains one close owner. */
export class DocumentTextJob{
 #owner:ScenePlanOwner|null;#view:ScenePlanRead|null;#current:ScenePlanNode|null=null;#producer=new RetainedWorkProducerRuntime();#phase="nodes";#at=0;#work=0;#segments=0;#family:DrawingFontFamily="anta";#aborted=false;#transferred=false;#failed=false;#failure:unknown;#pending:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 #face0:RetainedWorkBuffer<FontFace>|null=null;#face1:RetainedWorkBuffer<FontFace>|null=null;#face2:RetainedWorkBuffer<FontFace>|null=null;#face3:RetainedWorkBuffer<FontFace>|null=null;#faces:RetainedWorkBuffer<FontFace>|null=null;
 #sources:FontByteSource[]|null=null;#unicodeSource:Uint8Array|null=null;#unicode:UnicodeData|null=null;#unicodeAdmission:UnicodeDataAdmissionJob|null=null;#catalog:FontCatalogAdmissionJob|null=null;#run:FontOutlineRunJob|null=null;#replacement:ScenePlanContentReplaceJob|null=null;#runOutput:FontOutlineRun|null=null;#closing:RetainedWorkRetirementCursor|null=null;#afterClose="";
 #inputs=new RetainedWorkBuffer<FontRunInput>(56);#glyphSources=new RetainedWorkBuffer<ScenePlanContent>(80);readonly #limits:FontOutlineLimits;
 constructor(owner:ScenePlanOwner,limits:FontOutlineLimits=DOCUMENT_FONT_LIMITS){
  const view=ScenePlanOwner.prototype.borrow.call(owner);
  if(view.nodes.length>1024||!Number.isSafeInteger(limits.maxWork)||limits.maxWork<1||limits.maxWork>1e9||!Number.isInteger(limits.maxSegments)||limits.maxSegments<1||limits.maxSegments>65536)throw Error("Invalid text scene contract");
  this.#limits=Object.freeze({...limits});this.#owner=ScenePlanOwner.transfer(owner);this.#view=view;
 }
 #active():ScenePlanRead{if(this.#view===null)throw Error("Text source transferred");return this.#view;}
 #cachedFace():RetainedWorkBuffer<FontFace>|null{switch(this.#family){case"anta":return this.#face0;case"kellySlab":return this.#face1;case"shareTechMono":return this.#face2;case"notoEmoji":return this.#face3;}}
 #cacheFace(value:RetainedWorkBuffer<FontFace>):void{switch(this.#family){case"anta":this.#face0=value;break;case"kellySlab":this.#face1=value;break;case"shareTechMono":this.#face2=value;break;case"notoEmoji":this.#face3=value;break;}this.#faces=value;}
 needsFontSources():boolean{return this.#phase==="fontSources"&&this.#sources===null&&this.#work<this.#limits.maxWork;}
 fontSourceFamily():string{if(!this.needsFontSources())throw Error("No font source request");return drawingFontCatalogFamily(this.#family);}
 admitFontSources(sources:FontByteSource[],unicodeBytes:Uint8Array):void{if(!this.needsFontSources()||this.#aborted||this.#transferred||!Array.isArray(sources)||!(unicodeBytes instanceof Uint8Array))throw Error("Font sources already admitted");if(this.#unicode!==null&&this.#unicode.bytes!==unicodeBytes)throw Error("Original Unicode source identity differs");this.#sources=sources;this.#unicodeSource=unicodeBytes;}
 nextWorkDemand():RetainedWorkDemand{return this.#pending;}
 lastTurnWork():number{return this.#producer.lastTurnWork();}lastTurnSemanticWork():number{return this.#producer.lastTurnSemanticWork();}
 failurePending():boolean{return this.#failed;}
 settlementPending():boolean{
  if(this.#transferred)return false;
  if(this.#closing!==null)return true;
  if(this.#phase==="nodes")return this.#view!==null&&this.#at===this.#view.nodes.length;
  if(this.#phase==="unicode")return this.#unicodeAdmission?.settlementPending()===true;
  if(this.#phase==="fonts")return this.#catalog?.settlementPending()===true;
  if(this.#phase==="run")return this.#run?.settlementPending()===true;
  if(this.#phase==="geometry")return this.#replacement?.settlementPending()===true;
  return this.#phase==="unicodeTake"||this.#phase==="fontsTake"||this.#phase==="runTransfer"||this.#phase==="geometryAdmission"||this.#phase==="geometryBirth"||this.#phase==="replacementTake"||this.#phase==="publishing"||this.#phase==="outputClose"||this.#phase==="nodeDone"||this.#phase==="complete";
 }
 #publicPhase():DocumentTextProgress["phase"]{const phase=this.#phase==="childClose"?this.#afterClose:this.#phase;if(phase==="unicodeTake")return"unicode";if(phase==="fontsBirth"||phase==="fontsTake")return"fonts";if(phase==="runInput"||phase==="runBirth"||phase==="runTake"||phase==="runTransfer")return"run";if(phase==="geometryBirth")return"geometryAdmission";if(phase==="replacementTake"||phase==="outputClose"||phase==="nodeDone")return"publishing";switch(phase){case"nodes":case"fontSources":case"unicode":case"fonts":case"run":case"geometryAdmission":case"geometry":case"publishing":case"complete":return phase;default:throw Error("Unknown Text public phase");}}
 progress():DocumentTextProgress{return{phase:this.#publicPhase(),nodes:this.#at,work:this.#work,done:this.#phase==="complete"};}
 #stage(cursor:RetainedWorkRetirementCursor,after:string):void{if(this.#closing!==null)throw Error("Text resident child close already occupied");this.#closing=cursor;this.#afterClose=after;this.#phase="childClose";}
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#phase){
   case"childClose":{let done=false;try{done=this.#closing!.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.#closing!.lastTurnWork());}if(!done||c.hasActualWork())return;this.#closing=null;this.#phase=this.#afterClose;this.#afterClose="";c.commitControlWork();return;}
   case"nodes":{const view=this.#active();if(this.#at===view.nodes.length){this.#phase="complete";c.commitControlWork();return;}const node=view.nodes.at(this.#at);if(node===undefined)throw Error("Original Text node absent");this.#current=node;const content=node.content;c.commitSemanticWork();if(content.kind!=="text"){if(content.kind==="path"||content.kind==="glyphs"){if(this.#segments+content.segments.length>this.#limits.maxSegments)throw Error("Text scene segment limit exceeded");this.#segments+=content.segments.length;}this.#at++;return;}this.#family=parseDrawingFontFamily(content.fontFamily);this.#faces=this.#cachedFace();this.#phase=this.#faces===null?"fontSources":"runInput";return;}
   case"fontSources":if(this.#sources===null||this.#unicodeSource===null)throw Error("Text resolution requires admitted font sources");c.requireSemanticWork();if(this.#unicode===null){this.#unicodeAdmission=new UnicodeDataAdmissionJob(this.#unicodeSource,this.#limits.maxWork);this.#phase="unicode";}else this.#phase="fontsBirth";c.commitControlWork();return;
   case"unicode":if(c.advanceChild(this.#unicodeAdmission!).done)this.#phase="unicodeTake";return;
   case"unicodeTake":{const moved=this.#unicodeAdmission!.intoRetirement();this.#unicode=moved.output;if(this.#unicode===null)throw Error("Completed Unicode source absent");this.#unicodeAdmission=null;this.#stage(moved.job,"fontsBirth");c.commitControlWork();return;}
   case"fontsBirth":c.requireSemanticWork();this.#catalog=new FontCatalogAdmissionJob(drawingFontCatalogFamily(this.#family),this.#sources!,this.#limits);this.#phase="fonts";c.commitControlWork();return;
   case"fonts":if(c.advanceChild(this.#catalog!).done)this.#phase="fontsTake";return;
   case"fontsTake":{const moved=this.#catalog!.intoRetirement();if(moved.output===null)throw Error("Completed catalog output absent");this.#cacheFace(moved.output);this.#catalog=null;this.#sources=null;this.#unicodeSource=null;this.#stage(moved.job,"runInput");c.commitControlWork();return;}
   case"runInput":{if(!c.slot(this.#inputs))return;const content=this.#current!.content;if(content.kind!=="text")throw Error("Captured Text source changed");c.push(this.#inputs,{text:content.content,size:content.size,origin:[content.x,content.y],lineHeight:1.2,kerning:true,outlines:true});this.#phase="runBirth";return;}
   case"runBirth":c.requireSemanticWork();this.#run=new FontOutlineRunJob(this.#faces!,this.#inputs.at(0)!,this.#limits,this.#unicode!);this.#phase="run";c.commitControlWork();return;
   case"run":if(c.advanceChild(this.#run!).done)this.#phase="runTake";return;
   case"runTake":{const output=this.#run!.result();c.commitSemanticWork();if(this.#segments+output.segments.length>this.#limits.maxSegments)throw Error("Text scene segment limit exceeded");this.#segments+=output.segments.length;this.#phase="runTransfer";return;}
   case"runTransfer":{const moved=this.#run!.intoRetirement();if(moved.output===null)throw Error("Completed Text Run output absent");this.#runOutput=moved.output;this.#run=null;this.#stage(moved.job,"geometryAdmission");c.commitControlWork();return;}
   case"geometryAdmission":{if(!c.slot(this.#glyphSources))return;const content=this.#current!.content;if(content.kind!=="text")throw Error("Captured Text source changed");c.push(this.#glyphSources,{kind:"glyphs",content:content.content,x:content.x,y:content.y,size:content.size,fontFamily:content.fontFamily,fillRule:content.fillRule,fill:content.fill,stroke:content.stroke,segments:this.#runOutput!.segments});this.#phase="geometryBirth";return;}
   case"geometryBirth":c.requireSemanticWork();this.#replacement=new ScenePlanContentReplaceJob(this.#owner!,this.#at,this.#glyphSources.at(0)!,{maxWork:this.#limits.maxWork});this.#owner=null;this.#phase="geometry";c.commitControlWork();return;
   case"geometry":if(c.advanceChild(this.#replacement!).done)this.#phase="replacementTake";return;
   case"replacementTake":{const moved=this.#replacement!.intoRetirement();if(moved.output===null)throw Error("Completed Text Scene owner absent");this.#owner=moved.output;this.#replacement=null;this.#stage(moved.job,"publishing");c.commitControlWork();return;}
   case"publishing":if(this.#glyphSources.length){c.pop(this.#glyphSources);return;}if(this.#inputs.length){c.pop(this.#inputs);return;}this.#phase="outputClose";c.commitControlWork();return;
   case"outputClose":{const output=this.#runOutput!;this.#stage(new RetainedWorkRetirement((work,turn)=>{if(!retireRetainedWorkBuffer(output.glyphs,work,turn))return false;if(!retireRetainedWorkBuffer(output.paragraphLevels,work,turn))return false;if(!retireRetainedWorkBuffer(output.segments,work,turn))return false;return true;}),"nodeDone");c.commitControlWork();return;}
   case"nodeDone":this.#runOutput=null;this.#current=null;this.#at++;this.#phase="nodes";c.commitControlWork();return;
   default:throw Error("Unknown actual Text source phase");
  }
 }
 advance(items:number,work:RetainedWorkLedger):DocumentTextProgress{
  try{return this.#producer.advance(items,work,{maximumWork:this.#limits.maxWork,workLimitMessage:"Text scene work limit exceeded",validate:()=>{if(this.#aborted||this.#transferred)throw Error("Text scene cancelled");if(this.#failed)throw this.#failure;if(this.#phase!=="complete"&&!work.admitsAbsolute(this.#pending))work.requireAbsolute(this.#pending);},stopped:()=>this.#phase==="complete"||this.needsFontSources(),semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>this.#step(c),progress:()=>this.progress()});}
  catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#pending=work.nextDemand;}
 }
 result():ScenePlanRead{if(this.#aborted||this.#transferred||this.#failed||this.#phase!=="complete")throw Error("Text scene incomplete");return this.#owner!.borrow();}
 cancel():void{this.#aborted=true;}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{
  if(this.#transferred)throw Error("Text scene ownership already transferred");
  this.#transferred=true;
  const output=!this.#aborted&&!this.#failed&&this.#phase==="complete"?this.#owner:null;this.#aborted=true;
  let prior=this.#closing,unicodeClose:RetainedWorkRetirementCursor|null=null,catalogClose:RetainedWorkRetirementCursor|null=null,runClose:RetainedWorkRetirementCursor|null=null,replacementClose:RetainedWorkRetirementCursor|null=null;
  this.#closing=null;
  if(this.#unicodeAdmission){const moved=this.#unicodeAdmission.intoRetirement();unicodeClose=moved.job;if(moved.output!==null)this.#unicode=moved.output;this.#unicodeAdmission=null;}
  if(this.#catalog){const moved=this.#catalog.intoRetirement();catalogClose=moved.job;if(moved.output!==null)this.#cacheFace(moved.output);this.#catalog=null;}
  if(this.#run){const moved=this.#run.intoRetirement();runClose=moved.job;if(moved.output!==null)this.#runOutput=moved.output;this.#run=null;}
  if(this.#replacement){const moved=this.#replacement.intoRetirement();replacementClose=moved.job;if(moved.output!==null)this.#owner=moved.output;this.#replacement=null;}
  let scene=this.#owner!==null&&this.#owner!==output?new ScenePlanCloseJob(this.#owner):null;this.#owner=null;let cleared=false;
  const job=new RetainedWorkRetirement((work,turn)=>{
   if(prior){if(!drainCursor(prior,work,turn))return false;prior=null;turn.commitActualWork();return false;}
   if(unicodeClose){if(!drainCursor(unicodeClose,work,turn))return false;unicodeClose=null;turn.commitActualWork();return false;}
   if(catalogClose){if(!drainCursor(catalogClose,work,turn))return false;catalogClose=null;turn.commitActualWork();return false;}
   if(runClose){if(!drainCursor(runClose,work,turn))return false;runClose=null;turn.commitActualWork();return false;}
   if(replacementClose){if(!drainCursor(replacementClose,work,turn))return false;replacementClose=null;turn.commitActualWork();return false;}
   if(this.#runOutput){const value=this.#runOutput;if(!retireRetainedWorkBuffer(value.glyphs,work,turn)||!retireRetainedWorkBuffer(value.paragraphLevels,work,turn)||!retireRetainedWorkBuffer(value.segments,work,turn))return false;this.#runOutput=null;turn.commitActualWork();return false;}
   if(!retireRetainedWorkBuffer(this.#glyphSources,work,turn)||!retireRetainedWorkBuffer(this.#inputs,work,turn))return false;
   if(this.#face0){if(!retireRetainedWorkBuffer(this.#face0,work,turn))return false;this.#face0=null;turn.commitActualWork();return false;}
   if(this.#face1){if(!retireRetainedWorkBuffer(this.#face1,work,turn))return false;this.#face1=null;turn.commitActualWork();return false;}
   if(this.#face2){if(!retireRetainedWorkBuffer(this.#face2,work,turn))return false;this.#face2=null;turn.commitActualWork();return false;}
   if(this.#face3){if(!retireRetainedWorkBuffer(this.#face3,work,turn))return false;this.#face3=null;turn.commitActualWork();return false;}
   if(scene){if(!drainCursor(scene,work,turn))return false;scene=null;turn.commitActualWork();return false;}
   if(!cleared){this.#view=null;this.#current=null;this.#faces=null;this.#sources=null;this.#unicodeSource=null;this.#unicode=null;this.#afterClose="";cleared=true;turn.commitActualWork();return false;}
   return true;
  });
  return{job,output};
 }
}
