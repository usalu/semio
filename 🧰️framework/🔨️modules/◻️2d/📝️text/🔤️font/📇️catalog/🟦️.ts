/** 📇️ Explicit shipped family choices resolve genuine immutable font byte URLs. */
import policy from "./🧫️fixtures/🔣️.json";
import {FontFaceAdmissionJob,type FontFace,type FontOutlineLimits,type FontProgress} from "../🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../🧹️retire/🟦️.ts";
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
export const FONT_CATALOG_DEFAULT_FAMILY=policy.defaultFamily;
export function builtinFontLocations(family:string=policy.defaultFamily,weight=policy.weight):readonly FontSourceLocation[]{if(!policy.families.includes(family))throw Error("Unsupported authored font family");if(weight!==policy.weight)throw Error("Unsupported authored font weight");return locations.filter(source=>source.family===family).concat(family===policy.fallbackFamily?[]:locations.filter(source=>source.family===policy.fallbackFamily));}
export type FontByteProvider=(source:FontSourceLocation,signal?:AbortSignal)=>Promise<Uint8Array>;
export const fetchFontBytes:FontByteProvider=async(source,signal)=>{const response=await fetch(source.url,{signal});if(!response.ok)throw Error("Authored font source unavailable");return new Uint8Array(await response.arrayBuffer());};
/** ⏳️ Host font I/O is explicit and cancellable before bounded SFNT admission begins. */
export async function loadBuiltinFontSources(family:string,provider:FontByteProvider=fetchFontBytes,control:{signal?:AbortSignal;maxFontBytes?:number;onProgress?:(loaded:number,total:number)=>void}={}):Promise<FontByteSource[]>{const result:FontByteSource[]=[],sources=builtinFontLocations(family);for(const source of sources){if(control.signal?.aborted)throw new DOMException("Font source loading cancelled","AbortError");const bytes=await provider(source,control.signal);if(control.signal?.aborted)throw new DOMException("Font source loading cancelled","AbortError");if(bytes.byteLength>(control.maxFontBytes??67108864))throw Error("Font source byte limit exceeded");result.push({family:source.family,id:source.id,bytes});control.onProgress?.(result.length,sources.length);}return result;}
export class FontCatalogAdmissionJob{
 private child:FontFaceAdmissionJob|null=null;private children:WorkRetirement[]=[];private faces:FontFace[]=[];private expected:readonly FontSourceLocation[]=[];private at=0;private work=0;private aborted=false;private transferred=false;private failure:unknown=null;
 constructor(family:string,private sources:FontByteSource[],private readonly limits:FontOutlineLimits){try{this.expected=builtinFontLocations(family);if(sources.length!==this.expected.length)this.failure=Error("Authored font catalog source count differs");}catch(error){this.failure=error;}}
 advance(grant:number):FontProgress{if(!Number.isSafeInteger(grant)||grant<1)throw Error("Invalid font catalog grant");if(this.aborted||this.transferred)throw Error("Font catalog cancelled");if(this.failure)throw this.failure;try{for(let turn=0;turn<grant&&this.at<this.sources.length;turn++){if(this.work===this.limits.maxWork)throw Error("Font catalog work limit exceeded");if(!this.child){const source=this.sources[this.at]!,expected=this.expected[this.at]!;if(source.family!==expected.family||source.id!==expected.id)throw Error("Authored font catalog source identity differs");this.child=new FontFaceAdmissionJob(source.bytes,this.limits);}else if(this.child.advance(1).done){const moved=this.child.intoRetirement();this.faces.push(moved.output!);this.children.push(moved.job);this.child=null;this.at++;}this.work++;}}catch(error){this.failure=error;throw error;}return{phase:this.at===this.sources.length?"complete":"admitting",work:this.work,done:this.at===this.sources.length};}
 result():FontFace[]{if(this.aborted||this.transferred||this.failure||this.at!==this.sources.length)throw Error("Font catalog incomplete");return this.faces;}
 cancel():void{this.aborted=true;}
 intoRetirement():{job:WorkRetirement;output:FontFace[]|null}{if(this.transferred)throw Error("Font catalog ownership already transferred");const output=!this.aborted&&!this.failure&&this.at===this.sources.length?this.result():null;this.transferred=true;this.aborted=true;if(this.child){this.children.push(this.child.intoRetirement().job);this.child=null;}return{output,job:new UnitRetirement(()=>{if(this.children.length){if(this.children.at(-1)!.advance(1).done)this.children.pop();return false;}if(this.sources.length){this.sources.pop();return false;}if(!output&&this.faces.length){this.faces.pop();return false;}this.faces=[];return true;})};}
}
