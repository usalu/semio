/** 📇️ Explicit shipped family choices resolve genuine immutable font byte URLs. */
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import policy from "./🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import {FontFaceAdmissionJob,type FontFace,type FontOutlineLimits,type FontProgress} from "../🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkRetirementCursor,retireRetainedWorkBuffer,retireRetainedWorkChildren,retainedWorkChildRetirement,RetainedWorkBuffer,RetainedWorkLedger} from "../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";

export type FontByteSource={family:string;id:string;bytes:Uint8Array};
export type FontSourceLocation={family:string;id:string;url:URL};
const locations:readonly FontSourceLocation[]=[
 {family:"Anta",id:"latin",url:new URL("../../../../🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Anta",id:"latin-ext",url:new URL("../../../../🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Anta",id:"math",url:new URL("../../../../🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Anta",id:"symbols",url:new URL("../../../../🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Kelly Slab",id:"latin",url:new URL("../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Kelly Slab",id:"latin-ext",url:new URL("../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/➕️latin-ext/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Kelly Slab",id:"cyrillic",url:new URL("../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/🪆️cyrillic/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Share Tech Mono",id:"latin",url:new URL("../../../../🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"regions",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🌍️regions/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"flags",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🚩️flags/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"symbols",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔣️symbols/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"objects",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧰️objects/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"activities",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🎯️activities/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"travel",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧳️travel/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"food",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🍽️food/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"nature",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🌿️nature/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"people",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧑️people/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"faces",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/😀️faces/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"joined-forms",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔗️joined-forms/📖️regular/🔤️outline.ttf",import.meta.url)},
 {family:"Noto Emoji",id:"supplement",url:new URL("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/📖️regular/🔤️outline.ttf",import.meta.url)}
];
const sourceCount=(family:string):number=>{if(!policy.families.includes(family))throw Error("Unsupported authored font family");let count=0;for(const source of locations)if(source.family===family||family!==policy.fallbackFamily&&source.family===policy.fallbackFamily)count++;return count;};
const sourceAt=(family:string,index:number):FontSourceLocation=>{for(const source of locations)if(source.family===family){if(index===0)return source;index--;}if(family!==policy.fallbackFamily)for(const source of locations)if(source.family===policy.fallbackFamily){if(index===0)return source;index--;}throw Error("Authored font catalog source ordinal differs");};
export const FONT_CATALOG_DEFAULT_FAMILY=policy.defaultFamily;
export function builtinFontLocations(family:string=policy.defaultFamily,weight=policy.weight):readonly FontSourceLocation[]{if(!policy.families.includes(family))throw Error("Unsupported authored font family");if(weight!==policy.weight)throw Error("Unsupported authored font weight");return locations.filter(source=>source.family===family).concat(family===policy.fallbackFamily?[]:locations.filter(source=>source.family===policy.fallbackFamily));}
export type FontByteProvider=(source:FontSourceLocation,signal?:AbortSignal)=>Promise<Uint8Array>;
export const fetchFontBytes:FontByteProvider=async(source,signal)=>{const response=await fetch(source.url,{signal});if(!response.ok)throw Error("Authored font source unavailable");return new Uint8Array(await response.arrayBuffer());};
/** ⏳️ Host font I/O is explicit and cancellable before bounded SFNT admission begins. */
export async function loadBuiltinFontSources(family:string,provider:FontByteProvider=fetchFontBytes,control:{signal?:AbortSignal;maxFontBytes?:number;onProgress?:(loaded:number,total:number)=>void}={}):Promise<FontByteSource[]>{const result:FontByteSource[]=[],sources=builtinFontLocations(family);for(const source of sources){if(control.signal?.aborted)throw new DOMException("Font source loading cancelled","AbortError");const bytes=await provider(source,control.signal);if(control.signal?.aborted)throw new DOMException("Font source loading cancelled","AbortError");if(bytes.byteLength>(control.maxFontBytes??67108864))throw Error("Font source byte limit exceeded");result.push({family:source.family,id:source.id,bytes});control.onProgress?.(result.length,sources.length);}return result;}
export class FontCatalogAdmissionJob{
 private producer=new RetainedWorkProducerRuntime();private sourceCount=0;private phase:"source"|"child"|"publication"|"faceArchive"|"childArchive"|"complete"="source";private moved:{job:RetainedWorkRetirementCursor;output:FontFace|null}|null=null;
 private child:FontFaceAdmissionJob|null=null;private children=new RetainedWorkBuffer<RetainedWorkRetirementCursor>(16);private faces=new RetainedWorkBuffer<FontFace>(160);private at=0;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(private readonly family:string,private sources:readonly FontByteSource[],private readonly limits:FontOutlineLimits){try{this.sourceCount=sourceCount(family);if(sources.length!==this.sourceCount)this.failure=Error("Authored font catalog source count differs");}catch(error){this.failure=error;}}
 private step(context:RetainedWorkProducerStep):void{const work=context.work;if(!work.canStep())return;
  if(this.phase==="source"){context.commitSemanticWork();const source=this.sources[this.at]!,expected=sourceAt(this.family,this.at);if(source.family!==expected.family||source.id!==expected.id)throw Error("Authored font catalog source identity differs");this.child=new FontFaceAdmissionJob(source.bytes,this.limits);this.phase="child";return;}
  if(this.phase==="child"){if(context.advanceChild(this.child!).done)this.phase="publication";return;}
  if(this.phase==="publication"){this.moved=this.child!.intoRetirement();this.child=null;this.phase="faceArchive";context.commitControlWork();return;}
  if(this.phase==="faceArchive"){if(!context.slot(this.faces))return;if(!this.moved!.output)throw Error("Completed font catalog child output absent");context.push(this.faces,this.moved!.output);this.phase="childArchive";return;}
  if(this.phase==="childArchive"){if(!context.slot(this.children))return;context.push(this.children,this.moved!.job);this.moved=null;this.at++;this.phase=this.at===this.sourceCount?"complete":"source";return;}
  context.commitSemanticWork();throw Error("Font catalog source phase invalid");
 }
 settlementPending():boolean{return this.phase==="complete"||this.phase==="publication"||this.phase==="faceArchive"||this.phase==="childArchive"||this.phase==="child"&&this.child!==null&&this.child.settlementPending();}
 failurePending():boolean{return this.failure!==null||this.child?.failurePending()===true;}
 lastTurnWork():number{return this.producer.lastTurnWork();}
 lastTurnSemanticWork():number{return this.producer.lastTurnSemanticWork();}
 advance(grant:number,work:RetainedWorkLedger):FontProgress{try{return this.producer.advance(grant,work,{validate:()=>{if(this.aborted||this.transferred)throw Error("Font catalog cancelled");if(this.failure)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,maximumWork:this.limits.maxWork,workLimitMessage:"Font catalog work limit exceeded",recordSemanticWork:units=>{this.work+=units;},step:context=>this.step(context),progress:()=>({phase:this.phase==="complete"?"complete":"admitting",work:this.work,done:this.phase==="complete"})});}catch(error){this.failure=error;throw error;}}
 result():RetainedWorkBuffer<FontFace>{if(this.aborted||this.transferred||this.failure||this.at!==this.sources.length)throw Error("Font catalog incomplete");return this.faces;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:RetainedWorkBuffer<FontFace>|null}{if(this.transferred)throw Error("Font catalog ownership already transferred");const output=!this.aborted&&!this.failure&&this.at===this.sources.length?this.result():null;this.transferred=true;this.aborted=true;const child=retainedWorkChildRetirement(this.child,(value,work,turn)=>true);this.child=null;let staged=this.moved?.job??null;this.moved=null;return{output,job:new RetainedWorkRetirement((work,turn)=>{if(staged){if(!staged.terminalIsEmpty()){const before=turn.actualWork();let complete=false;try{complete=staged.advance(1,work).done;}finally{turn.commitActualWork(staged.lastTurnWork());}if(!complete||turn.actualWork()!==before)return false;}if(!work.canStep())return false;staged=null;turn.commitActualWork();return false;}if(!child(work,turn)||!retireRetainedWorkChildren(this.children,work,turn))return false;if(!output&&!retireRetainedWorkBuffer(this.faces,work,turn))return false;this.sources=[];turn.commitActualWork();return true;})};}
}
