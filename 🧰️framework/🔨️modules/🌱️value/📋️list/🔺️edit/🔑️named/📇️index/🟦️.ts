import {NamedEditPlan,type NamedEditAdmittedBase,type NamedEditSource,type NamedEditDemand,type NamedEditGrant,type NamedEditReceipt} from "../🟦️.ts";
import {BuiltPersistentList,type PersistentListSet} from "../../../🔗️persistent/📥️build/🟦️.ts";
const issuer=Symbol("original named index admission");
/** 🔐️ Original private sorted coordinates reuse their exact borrowed base root. */
export class AdmittedNamedKeyIndex implements NamedEditAdmittedBase{
 readonly #original:NamedEditSource;
 readonly #order:readonly number[];
 constructor(original:NamedEditSource,order:number[],authority:symbol){if(authority!==issuer||Object.isExtensible(order))throw Error("original sealed named index issuer required");this.#original=original;this.#order=order;}
 get length():number{return this.#order.length;}
 get order():readonly number[]{return this.#order;}
 checkSource(source:NamedEditSource):void{if(source.originalBase!==this.#original.originalBase||source.keyProjection!==this.#original.keyProjection||source.shape.baseLen!==this.length)throw Error("original named base source changed");}
 source(edits:NamedEditSource):NamedEditSource{this.checkSource(edits);return{originalBase:this.#original.originalBase,keyProjection:this.#original.keyProjection,shape:edits.shape,admittedBase:this,keyDemand:(lane,ordinal)=>lane==="base"?this.#original.keyDemand(lane,ordinal):edits.keyDemand(lane,ordinal),observeKey:(lane,ordinal,grant)=>lane==="base"?this.#original.observeKey(lane,ordinal,grant):edits.observeKey(lane,ordinal,grant),addedIndex:ordinal=>edits.addedIndex(ordinal)};}
 transfer(source:NamedEditSource,authority:symbol):AdmittedNamedKeyIndex{if(authority!==issuer||source.shape.baseLen!==this.length)throw Error("actual key-preserving producer required");return new AdmittedNamedKeyIndex(source,this.#order as number[],issuer);}
}
/** 🧭️ Exact original immutable page paths fund their borrowed key selector. */
export function persistentNamedKeySource<T>(original:BuiltPersistentList<T>,key:(row:T)=>Uint8Array):NamedEditSource{return{originalBase:original.originalAuthority,keyProjection:key,shape:{baseLen:original.length,removed:0,modified:0,added:0},keyDemand:(lane,rank)=>{if(lane!=="base")throw Error("original key source has no authored lane");const d=original.rankDemand(rank);return{...d,copyBytes:d.copyBytes+64,capacityBytes:0};},observeKey:(lane,rank,grant)=>{if(lane!=="base")throw Error("original key source has no authored lane");const d=original.rankDemand(rank);if(grant.maximumCopyBytes<d.copyBytes+64)return;const row=original.observeRank(rank,grant);if(row===undefined)return;return{key:key(row),receipt:{copiedItems:1,copiedBytes:d.copyBytes+64,retainedCapacityBytes:0}};},addedIndex:()=>{throw Error("original key source has no additions");}};}
/** 🌳️ The independently produced pages and admitted directory remain outside each other's payload. */
export class NamedPersistentRoot<T>{
 #source:BuiltPersistentList<T>|undefined;#index:AdmittedNamedKeyIndex|undefined;#key:(row:T)=>Uint8Array;
 constructor(source:BuiltPersistentList<T>,index:AdmittedNamedKeyIndex,key:(row:T)=>Uint8Array){index.checkSource(persistentNamedKeySource(source,key));this.#source=source;this.#index=index;this.#key=key;}
 get length():number{return this.#index?.length??0;}
 get empty():boolean{return this.#source===undefined&&this.#index===undefined;}
 get ranked():NamedEditSource{if(!this.#source)throw Error("independent named root is closed");return persistentNamedKeySource(this.#source,this.#key);}
 get index():AdmittedNamedKeyIndex{if(!this.#index)throw Error("independent named root is closed");return this.#index;}
 projectIndex(grant:NamedEditGrant):{source:BuiltPersistentList<T>;index:AdmittedNamedKeyIndex}|undefined{if(!this.#source||!this.#index)throw Error("independent named root is closed");if(grant.maximumItems<1||grant.maximumCopyBytes<512||grant.maximumDepth<1)return;const source=this.#source.project(grant)!;return{source,index:this.#index.transfer(persistentNamedKeySource(source,this.#key),issuer)};}
 close():void{this.#source?.close();this.#source=undefined;this.#index=undefined;}
}
/** ♻️ Complete original key comparison precedes the actual private changed-path producer. */
export class KeyPreservingPersistentSet<T>{
 #original:BuiltPersistentList<T>|undefined;#index:AdmittedNamedKeyIndex|undefined;#authored:T|undefined;#set:PersistentListSet<T>|undefined;#candidate:BuiltPersistentList<T>|undefined;#source:NamedEditSource;#rank:number;#key:(row:T)=>Uint8Array;#offset=0;#phase:"key"|"set"|"ready"|"refused"|"spent"|"closed"="key";
 constructor(original:BuiltPersistentList<T>,index:AdmittedNamedKeyIndex,rank:number,key:(row:T)=>Uint8Array,authored:T){this.#source=persistentNamedKeySource(original,key);index.checkSource(this.#source);original.rankDemand(rank);this.#original=original;this.#index=index;this.#rank=rank;this.#key=key;this.#authored=authored;}
 get ready():boolean{return this.#phase==="ready";}
 get refused():boolean{return this.#phase==="refused";}
 get empty():boolean{return this.#original===undefined&&this.#index===undefined&&this.#authored===undefined&&this.#set===undefined&&this.#candidate===undefined;}
 demand(body:number):NamedEditDemand{if(!["key","set"].includes(this.#phase))throw Error("original key-preserving producer is settled");if(this.#phase==="set")return{...this.#set!.demand(),capacityBytes:0};const base=this.#source.keyDemand("base",this.#rank);const overhead=base.copyBytes+128;return{...base,copyBytes:overhead+Math.max(1,Math.min(2048,Math.floor(Math.max(0,body-overhead)/2)))*2};}
 advance(grant:NamedEditGrant):NamedEditReceipt{const d=this.demand(grant.maximumCopyBytes);if(grant.maximumItems<1||grant.maximumCopyBytes<d.copyBytes||grant.maximumCapacityBytes<d.capacityBytes||grant.maximumDepth<d.depth)return{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0};if(this.#phase==="set"){this.#set!.advance(grant);if(this.#set!.ready){this.#candidate=this.#set!.take();this.#set!.close();this.#set=undefined;this.#phase="ready";}return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0};}const left=this.#source.observeKey("base",this.#rank,grant)!.key,right=this.#key(this.#authored!);const overhead=this.#source.keyDemand("base",this.#rank).copyBytes+128;const count=Math.min(left.length-this.#offset,Math.floor((d.copyBytes-overhead)/2));if(left.length!==right.length)this.#phase="refused";else{const end=this.#offset+count;for(let i=this.#offset;i<end;i++)if(left[i]!==right[i]){this.#phase="refused";break;}this.#offset=end;if(this.#phase!=="refused"&&end===left.length){this.#set=this.#original!.beginSet(this.#rank,this.#authored!);this.#phase="set";}}return{copiedItems:1,copiedBytes:overhead+count*2,retainedCapacityBytes:0};}
 take():{source:BuiltPersistentList<T>;index:AdmittedNamedKeyIndex}{if(!this.ready)throw Error("original key-preserving candidate is not ready");const source=this.#candidate!,index=this.#index!.transfer(persistentNamedKeySource(source,this.#key),issuer);this.#candidate=undefined;this.#index=undefined;this.#original=undefined;this.#authored=undefined;this.#phase="spent";return{source,index};}
 close():void{this.#set?.close();this.#candidate?.close();this.#set=undefined;this.#candidate=undefined;this.#original=undefined;this.#index=undefined;this.#authored=undefined;this.#phase="closed";}
}
/** 📥️ The original named planner admits each base exactly once before immutable publication. */
export class NamedKeyIndexPreparation{
 readonly #source:NamedEditSource;
 readonly #input:NamedEditSource;
 readonly #plan:NamedEditPlan;
 #order:number[]|undefined;
 #cursor=0;
 #ready=false;
 #transferred=false;
 constructor(source:NamedEditSource){this.#source=source;this.#input={...source,shape:{baseLen:source.shape.baseLen,removed:0,modified:0,added:0}};this.#plan=new NamedEditPlan(this.#input.shape);}
 get settled():boolean{return this.#ready||this.#plan.settled&&!this.#plan.ready;}
 demand(body:number):NamedEditDemand{if(this.#transferred||this.settled)throw Error("original named index is settled or transferred");return this.#plan.settled?{copyBytes:128,capacityBytes:0,depth:1}:this.#plan.demand(this.#input,body);}
 advance(grant:NamedEditGrant):NamedEditReceipt{const d=this.demand(grant.maximumCopyBytes);if(grant.maximumItems<1||grant.maximumCopyBytes<d.copyBytes||grant.maximumCapacityBytes<d.capacityBytes||grant.maximumDepth<d.depth)return{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0};if(!this.#plan.settled)return this.#plan.advance(this.#input,grant);if(!this.#order)this.#order=this.#plan.takeBaseOrder();else if(this.#cursor<this.#order.length)Object.defineProperty(this.#order,this.#cursor++,{writable:false,configurable:false});else{Object.defineProperty(this.#order,"length",{writable:false});Object.preventExtensions(this.#order);this.#ready=true;}return{copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0};}
 takeDemand():NamedEditDemand{if(!this.settled||this.#transferred)throw Error("original named index admission has not settled");return{copyBytes:128,capacityBytes:0,depth:1};}
 take(grant:NamedEditGrant):AdmittedNamedKeyIndex|undefined{const d=this.takeDemand();if(grant.maximumItems<1||grant.maximumCopyBytes<d.copyBytes||grant.maximumDepth<d.depth)return;this.#transferred=true;if(!this.#plan.ready)return;return new AdmittedNamedKeyIndex(this.#source,this.#order!,issuer);}
}
