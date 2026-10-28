import type {VectorCanvas} from '../🧵️lower/🟦️.ts';
export interface ChildAdmissionSource{readonly value:unknown;readonly identity:object;readonly projection:object}
export interface ChildAdmissionGrant{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}
export interface ChildAdmissionDemand{copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number}
export interface ChildAdmissionProgress{copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number}
export interface ChildAdmissionAdvance{progress:ChildAdmissionProgress;done:boolean;error?:string;canvas?:VectorCanvas}
export const emptyChildAdmissionProgress=():ChildAdmissionProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
export const childAdmissionAdmits=(g:ChildAdmissionGrant,d:ChildAdmissionDemand)=>g.maximumItems>0&&g.maximumCopyBytes>=d.copyBytes&&g.maximumCapacityBytes>=d.capacityBytes&&g.maximumReleaseBytes>=d.releaseBytes&&g.maximumDepth>=d.depth;
const receiptFits=(p:ChildAdmissionProgress,g:ChildAdmissionGrant)=>[p.copiedItems,p.copiedBytes,p.retainedCapacityBytes,p.releasedBytes].every(n=>Number.isSafeInteger(n)&&n>=0)&&p.copiedItems<=g.maximumItems&&p.copiedBytes<=g.maximumCopyBytes&&p.retainedCapacityBytes<=g.maximumCapacityBytes&&p.releasedBytes<=g.maximumReleaseBytes;
export interface EmbeddedChildAdmissionCursorV1{nextDemand(source:ChildAdmissionSource,body:number):ChildAdmissionDemand;advance(source:ChildAdmissionSource,grant:ChildAdmissionGrant):ChildAdmissionAdvance;beginClose():void;closeDemand(body:number):ChildAdmissionDemand;closeStep(grant:ChildAdmissionGrant):ChildAdmissionProgress;terminalIsEmpty():boolean}
export interface EmbeddedChildAdmitterV1{birthDemand():ChildAdmissionDemand;begin(grant:ChildAdmissionGrant):{cursor:EmbeddedChildAdmission;progress:ChildAdmissionProgress}}
/** 🧳️ Logical opaque cursor accounting; native Rust retains and releases the exact physical owner box. */
export class EmbeddedChildAdmission{private cursor:EmbeddedChildAdmissionCursorV1|undefined;private closing=false;private bytes:number;private static overhead=64;
 private constructor(cursor:EmbeddedChildAdmissionCursorV1,bytes:number){this.cursor=cursor;this.bytes=bytes}
 static constructorDemand(bytes:number):ChildAdmissionDemand{if(!Number.isSafeInteger(bytes)||bytes<0||!Number.isSafeInteger(bytes+EmbeddedChildAdmission.overhead))throw Error('Child admission cursor extent is invalid');return{copyBytes:bytes+EmbeddedChildAdmission.overhead,capacityBytes:bytes,releaseBytes:0,depth:1}}
 static admit(cursor:EmbeddedChildAdmissionCursorV1,bytes:number,g:ChildAdmissionGrant):{cursor:EmbeddedChildAdmission;progress:ChildAdmissionProgress}{const d=this.constructorDemand(bytes);if(!childAdmissionAdmits(g,d))throw Error('Child admission cursor birth is unfunded');return{cursor:new EmbeddedChildAdmission(cursor,bytes),progress:{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:bytes,releasedBytes:0}}}
 private failure(error:unknown):never{const p=(error as {retainedProgress?:ChildAdmissionProgress}).retainedProgress??emptyChildAdmissionProgress();throw Object.assign(error instanceof Error?error:Error(String(error)),{retainedProgress:{...p,copiedItems:Math.max(1,p.copiedItems),copiedBytes:p.copiedBytes+EmbeddedChildAdmission.overhead}})}
 private child(g:ChildAdmissionGrant):ChildAdmissionGrant{return{...g,maximumCopyBytes:g.maximumCopyBytes-EmbeddedChildAdmission.overhead,maximumDepth:g.maximumDepth-1}}
 private nested(d:ChildAdmissionDemand):ChildAdmissionDemand{return{...d,copyBytes:d.copyBytes+EmbeddedChildAdmission.overhead,depth:d.depth+1}}
 nextDemand(source:ChildAdmissionSource,body:number):ChildAdmissionDemand{if(this.closing||this.cursor===undefined)throw Error('Child admission is closing or closed');return this.nested(this.cursor.nextDemand(source,Math.max(0,body-EmbeddedChildAdmission.overhead)))}
 advance(source:ChildAdmissionSource,g:ChildAdmissionGrant):ChildAdmissionAdvance{const d=this.nextDemand(source,g.maximumCopyBytes);if(!childAdmissionAdmits(g,d))return{progress:emptyChildAdmissionProgress(),done:false};const child=this.child(g);let result:ChildAdmissionAdvance;try{result=this.cursor!.advance(source,child)}catch(error){this.failure(error)}if(!receiptFits(result!.progress,child))this.failure(Object.assign(Error("Child admission receipt exceeds the original remaining grant"),{retainedProgress:result!.progress}));result.progress.copiedItems=Math.max(1,result.progress.copiedItems);result.progress.copiedBytes+=EmbeddedChildAdmission.overhead;return result}
 beginClose(){this.closing=true;this.cursor?.beginClose()}
 closeDemand(body:number):ChildAdmissionDemand{if(this.cursor===undefined)return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0};if(this.cursor.terminalIsEmpty())return{copyBytes:this.bytes+EmbeddedChildAdmission.overhead,capacityBytes:0,releaseBytes:this.bytes,depth:1};return this.nested(this.cursor.closeDemand(Math.max(0,body-EmbeddedChildAdmission.overhead)))}
 closeStep(g:ChildAdmissionGrant):ChildAdmissionProgress{if(!this.closing)throw Error('Child admission close has not begun');const d=this.closeDemand(g.maximumCopyBytes);if(!childAdmissionAdmits(g,d))return emptyChildAdmissionProgress();if(this.cursor===undefined)return emptyChildAdmissionProgress();if(this.cursor.terminalIsEmpty()){this.cursor=undefined;this.bytes=0;return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0,releasedBytes:d.releaseBytes}}const child=this.child(g);let p:ChildAdmissionProgress;try{p=this.cursor.closeStep(child)}catch(error){this.failure(error)}if(!receiptFits(p!,child))this.failure(Object.assign(Error("Child admission close receipt exceeds the original remaining grant"),{retainedProgress:p!}));p.copiedItems=Math.max(1,p.copiedItems);p.copiedBytes+=EmbeddedChildAdmission.overhead;return p}
 terminalIsEmpty(){return this.cursor===undefined&&this.bytes===0}
}

export {ChildAdmissionAddress} from './📖️provider/🟦️.ts';
export type {ChildAdmissionStamp,ChildAdmissionQualificationV1,ChildAdmissionRead,ChildAdmissionProviderV1} from './📖️provider/🟦️.ts';

export type * from './🧵️vector/🟦️.ts';

export {OriginalChildVectorPreparation} from './🧵️vector/🧳️prepare/🟦️.ts';
