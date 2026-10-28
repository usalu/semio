/** 📏️ Finite JSON encoding reservations shared by source-owned staged requests. */
import caps from './limits.json';
import type {StagedFormControl} from './🟦️.ts';
export interface StagedFormLimits {readonly utf8Bytes:number;readonly nodes:number;readonly depth:number}
export interface StagedFormCensus {readonly utf8Bytes:number;readonly nodes:number;readonly depth:number}
export const STAGED_FORM_LIMITS:StagedFormLimits=Object.freeze({...caps});
/** 📦️ Reserve standalone Value record framing: at most 32 bytes per node and 32 outer bytes. */
export function stagedFormWireBytes(policy:StagedFormLimits=STAGED_FORM_LIMITS):number{
 const limits=stagedFormLimits(policy),bytes=limits.utf8Bytes+32*limits.nodes+32;
 if(!Number.isSafeInteger(bytes))throw new StagedFormResourceError('staging.resource-limit');return bytes;
}
export class StagedFormResourceError extends Error {
 readonly code:'staging.resource-limit'|'staging.source-invalid';
 constructor(code:'staging.resource-limit'|'staging.source-invalid'){super(code);this.code=code;}
}
export class StagedFormCensusCancelled extends Error {readonly code='io.cancelled';constructor(){super('io.cancelled');}}
/** ⏳️ Check a caller-stable request cooperatively under its captured original cancellation control. */
export async function stagedFormCensusAsync(value:unknown,control:StagedFormControl,policy:StagedFormLimits=STAGED_FORM_LIMITS):Promise<StagedFormCensus>{
 const signal=control.signal,progress=control.progress,limits=stagedFormLimits(policy),steps=stagedFormCensusSteps(value,limits);let completed=0;
 for(;;){if(signal.aborted)throw new StagedFormCensusCancelled();const step=steps.next();if(step.done){if(signal.aborted)throw new StagedFormCensusCancelled();return step.value;}completed++;if(completed%1024===0){progress?.(completed);if(signal.aborted)throw new StagedFormCensusCancelled();await new Promise<void>(resolve=>setTimeout(resolve,0));}}
}
/** 🎟️ Admit only explicit positive lower policies without invoking source accessors. */
export function stagedFormLimits(input:StagedFormLimits=STAGED_FORM_LIMITS):StagedFormLimits{
 const result={...caps};const keys=Reflect.ownKeys(input);if(keys.length!==3)throw new StagedFormResourceError('staging.source-invalid');
 for(const key of ['utf8Bytes','nodes','depth'] as const){const descriptor=Object.getOwnPropertyDescriptor(input,key);if(!descriptor||!('value' in descriptor)||!Number.isSafeInteger(descriptor.value)||descriptor.value<1||descriptor.value>caps[key])throw new StagedFormResourceError('staging.source-invalid');result[key]=descriptor.value;}
 return result;
}
/** 🧮️ Count escaped UTF8 and punctuation exactly, reserving 32 bytes per finite number. */
export function* stagedFormCensusSteps(value:unknown,policy:StagedFormLimits=STAGED_FORM_LIMITS):Generator<number,StagedFormCensus,void>{
 const limits=stagedFormLimits(policy),active=new Set<object>();let bytes=0,nodes=0,depth=0;
 const reserve=(size:number)=>{const next=bytes+size;if(!Number.isSafeInteger(next)||next>limits.utf8Bytes)throw new StagedFormResourceError('staging.resource-limit');bytes=next;};
 const invalid=():never=>{throw new StagedFormResourceError('staging.source-invalid');};
 function* text(text:string):Generator<number,void,void>{reserve(2);yield 1;for(let i=0;i<text.length;i++){const unit=text.charCodeAt(i);if(unit>=0xD800&&unit<=0xDBFF){const low=text.charCodeAt(++i);if(!(low>=0xDC00&&low<=0xDFFF))invalid();reserve(4);}else if(unit>=0xDC00&&unit<=0xDFFF)invalid();else if(unit===34||unit===92||[8,9,10,12,13].includes(unit))reserve(2);else if(unit<32)reserve(6);else reserve(unit<128?1:unit<2048?2:3);yield 1;}}
 function* visit(value:unknown,level:number):Generator<number,void,void>{
  if(level>limits.depth||nodes>=limits.nodes)throw new StagedFormResourceError('staging.resource-limit');nodes++;depth=Math.max(depth,level);yield 1;
  if(value===null){reserve(4);return;}if(typeof value==='boolean'){reserve(value?4:5);return;}if(typeof value==='number'){if(!Number.isFinite(value))invalid();reserve(32);return;}if(typeof value==='string'){yield* text(value);return;}if(typeof value!=='object')invalid();
  const object=value as object,prototype=Object.getPrototypeOf(object),array=Array.isArray(object);if(prototype!==(array?Array.prototype:Object.prototype)&&prototype!==null||active.has(object))invalid();active.add(object);
  const keys=Reflect.ownKeys(object);if(keys.length>limits.nodes+1)throw new StagedFormResourceError('staging.resource-limit');reserve(2);yield 1;
  const length=array?Object.getOwnPropertyDescriptor(object,'length')?.value:undefined;let index=0;
  for(const key of keys){if(array&&key==='length')continue;if(typeof key!=='string')return invalid();const descriptor=Object.getOwnPropertyDescriptor(object,key);if(!descriptor||!('value' in descriptor)||!descriptor.enumerable)invalid();if(array&&key!==String(index))invalid();if(index++)reserve(1);if(!array){yield* text(key);reserve(1);}yield* visit(descriptor!.value,level+1);}
  if(array&&index!==length)invalid();active.delete(object);
 }
 yield* visit(value,1);return {utf8Bytes:bytes,nodes,depth};
}
