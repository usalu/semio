import {FixedKeySlots,type FixedKeySlot} from "../🎫️slots/🟦️.ts";
import {ExactKeyCursor} from "../../🔑️lookup/🟦️.ts";
import {Blake3Hasher} from "@semio-tech/framework-hash";
/** 🎟️ Canonical independent work currencies for retained original request bytes. */
export type RequestWorkGrant={maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number};
/** 🧾️ Actual retained payload effects of one bounded request turn. */
export type RequestWorkProgress={copiedItems:number;copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number};
/** 📏️ Original text headers use the same explicit64bit word model as exact-key comparison. */
export type RequestWorkDemand={items:number;copyBytes:number;capacityBytes:number;releaseBytes:number;depth:number};
const emptyProgress=():RequestWorkProgress=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});
/** 🪪️ Exact byte custody is prepared and compared under separate allocation/copy/release grants. */
export class RetainedRequestText{
 #bytes:Uint8Array|undefined;#length=0;#verified=false;#comparison=new ExactKeyCursor(8);#closed=false;#borrows=0;
 private constructor(length:number){this.#bytes=new Uint8Array(length);}
 static birth(length:number,grant:RequestWorkGrant):{owner?:RetainedRequestText;progress:RequestWorkProgress}{if(!Number.isSafeInteger(length)||length<1||length>65536)throw Error("request-original-identity-limit");if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCapacityBytes<length)return{progress:emptyProgress()};return{owner:new RetainedRequestText(length),progress:{...emptyProgress(),copiedItems:1,retainedCapacityBytes:length}};}
 step(original:Uint8Array,grant:RequestWorkGrant):{ready:boolean;progress:RequestWorkProgress;fault?:string}{
  const stored=this.#bytes;if(!stored||this.#closed||original.length!==stored.length)throw Error("request-original-selector-collision");
  if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<1)return{ready:false,progress:emptyProgress()};
  if(this.#length===stored.length){if(this.#verified)this.#comparison=new ExactKeyCursor(8);const receipt=this.#comparison.advance(stored,original,grant);const progress={...emptyProgress(),...receipt};if(this.#comparison.nextIndex!==0){return{ready:false,progress,fault:"request-original-selector-collision"};}this.#verified=this.#comparison.found===0;return{ready:this.#verified,progress};}
  const copied=Math.min(original.length-this.#length,grant.maximumCopyBytes);stored.set(original.subarray(this.#length,this.#length+copied),this.#length);this.#length+=copied;this.#verified=this.#length===stored.length;return{ready:this.#verified,progress:{...emptyProgress(),copiedItems:1,copiedBytes:copied}};
 }
 byte(index:number):number|undefined{return !this.#closed&&this.#length===this.#bytes?.length?this.#bytes[index]:undefined;}
 withBytes<R>(read:(bytes:Uint8Array)=>R):R{const bytes=this.#bytes;if(!bytes||this.#length!==bytes.length)throw Error("request-original-preparation-pending");this.#borrows++;try{return read(bytes);}finally{this.#borrows--;}}
 byteLength():number|undefined{return !this.#closed&&this.#length===this.#bytes?.length?this.#bytes?.length:undefined;}
 /** 🌘️ Original text headers quote independently from the allocation they retire. */
 closeDemand():RequestWorkDemand{return this.#bytes&&!this.#closed?{items:1,copyBytes:80,capacityBytes:0,releaseBytes:this.#bytes.length,depth:1}:{items:0,copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0};}
 close(grant:RequestWorkGrant):{ready:boolean;progress:RequestWorkProgress}{const stored=this.#bytes;if(!stored||this.#closed)return{ready:true,progress:emptyProgress()};const demand=this.closeDemand();if(this.#borrows>0||grant.maximumItems<demand.items||grant.maximumCopyBytes<demand.copyBytes||grant.maximumDepth<demand.depth||grant.maximumReleaseBytes<demand.releaseBytes)return{ready:false,progress:emptyProgress()};this.#closed=true;this.#bytes=undefined;return{ready:true,progress:{...emptyProgress(),copiedItems:1,copiedBytes:demand.copyBytes,releasedBytes:stored.length}};}
}
/** 🔎️ Snapshot positions derive from bounded original-byte equality rather than source-index identity. */
export class RequestSourceLookup{
 readonly #original:RetainedRequestText;readonly #cursor=new ExactKeyCursor(8);
 constructor(original:RetainedRequestText){if(original.byteLength()===undefined)throw Error("request-original-preparation-pending");this.#original=original;}
 step(candidate:Uint8Array|undefined,grant:RequestWorkGrant):{position:number;outcome:"searching"|"matched"|"missing";progress:RequestWorkProgress}{const receipt=this.#original.withBytes(bytes=>this.#cursor.advance(bytes,candidate,grant));return{position:this.#cursor.nextIndex,outcome:!this.#cursor.ready?"searching":this.#cursor.found===undefined?"missing":"matched",progress:{...emptyProgress(),...receipt}};}
}
/** 🔏️ Framed physical identities hash in bounded chunks through the first-party BLAKE3 authority. */
export class ContentRequestKeyCursor{
 #hash:Blake3Hasher|undefined=new Blake3Hasher();#lengths:Uint32Array|undefined;#prefix:Uint8Array|undefined=new Uint8Array(8);#segment=0;#offset=0;#key:Uint8Array|undefined;#finalizationBytes=0;
 private constructor(segments:readonly Uint8Array[]){this.#lengths=Uint32Array.from(segments,segment=>segment.length);}
 static birth(segments:readonly Uint8Array[],grant:RequestWorkGrant):{owner?:ContentRequestKeyCursor;progress:RequestWorkProgress}{if(segments.length<1||segments.length>32||segments.reduce((length,segment)=>length+8+segment.length,0)>131072)throw Error("request-selector-identity-limit");const bytes=2912+segments.length*4+8;if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCapacityBytes<bytes)return{progress:emptyProgress()};return{owner:new ContentRequestKeyCursor(segments),progress:{...emptyProgress(),copiedItems:1,retainedCapacityBytes:bytes}};}
 step(segments:readonly Uint8Array[],grant:RequestWorkGrant):{ready:boolean;progress:RequestWorkProgress}{const lengths=this.#lengths,prefix=this.#prefix,hash=this.#hash;if(!lengths||!prefix||!hash)throw Error("request-selector-closed");if(segments.length!==lengths.length||segments.some((segment,index)=>segment.length!==lengths[index]))throw Error("request-selector-input-changed");if(this.#key)return{ready:true,progress:emptyProgress()};if(this.#segment===lengths.length){if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCapacityBytes<this.#finalizationBytes||grant.maximumReleaseBytes<this.#finalizationBytes-32)return{ready:false,progress:emptyProgress()};this.#key=hash.digest();return{ready:true,progress:{...emptyProgress(),copiedItems:1,retainedCapacityBytes:this.#finalizationBytes,releasedBytes:this.#finalizationBytes-32}};}if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumCopyBytes<1)return{ready:false,progress:emptyProgress()};if(this.#offset===0)new DataView(prefix.buffer).setBigUint64(0,BigInt(lengths[this.#segment]!),true);const bytes=this.#offset<8?prefix.subarray(this.#offset):segments[this.#segment]!.subarray(this.#offset-8);const copied=Math.min(bytes.length,grant.maximumCopyBytes);hash.update(bytes.subarray(0,copied));this.#offset+=copied;if(this.#offset===8+lengths[this.#segment]!){this.#segment++;this.#offset=0;}if(this.#segment===lengths.length){let chunks=Math.floor((lengths.reduce((length,bytes)=>length+bytes+8,0)-1)/1024),depth=0;while(chunks>0){depth+=chunks&1;chunks>>>=1;}this.#finalizationBytes=224+depth*32;}return{ready:false,progress:{...emptyProgress(),copiedItems:1,copiedBytes:copied}};}
 key():Readonly<Uint8Array>|undefined{return this.#key;}
 close(grant:RequestWorkGrant):{ready:boolean;progress:RequestWorkProgress}{if(!this.#hash)return{ready:true,progress:emptyProgress()};const bytes=2912+(this.#lengths?.byteLength??0)+(this.#prefix?.byteLength??0)+(this.#key?.byteLength??0);if(grant.maximumItems<1||grant.maximumDepth<1||grant.maximumReleaseBytes<bytes)return{ready:false,progress:emptyProgress()};this.#hash=undefined;this.#lengths=undefined;this.#prefix=undefined;this.#key=undefined;return{ready:true,progress:{...emptyProgress(),copiedItems:1,releasedBytes:bytes}};}
}
/** 🌿️ Source authority remains exact after progress presentation is acknowledged. */
export type RequestSourceVersion={baseRevision:number;generation:number;canonicalRevision:string};
/** 🚦️ Compact terminal metadata lends no worker fault payload. */
export type RequestTerminalReceipt={source:RequestSourceVersion;job:number;kind:"validated"|"faulted"|"cancelled"};
/** 🎫️ Scoped complete identity admission shares work without replacing another consumer's request. */
export class FixedRequestOwners{
 readonly #owners:FixedKeySlots<string>;readonly #consumers:FixedKeySlots<string>;readonly #keys:Array<readonly[number,string]|undefined>;readonly #bindings:Array<FixedKeySlot|undefined>;readonly #references:number[];readonly #pending:Array<FixedKeySlot|undefined>;readonly #closing:boolean[];readonly #executions:Array<{source:RequestSourceVersion;job:number}|undefined>;readonly #terminals:Array<RequestTerminalReceipt|undefined>;
 constructor(capacity:number,consumers:number){this.#owners=new FixedKeySlots(capacity);this.#consumers=new FixedKeySlots(consumers);this.#keys=Array(capacity);this.#bindings=Array(consumers);this.#references=Array(capacity).fill(0);this.#pending=Array(consumers);this.#closing=Array(capacity).fill(false);this.#executions=Array(capacity);this.#terminals=Array(capacity);}
 #same(a:RequestSourceVersion,b:RequestSourceVersion):boolean{return a.baseRevision===b.baseRevision&&a.generation===b.generation&&a.canonicalRevision===b.canonicalRevision;}
 terminal(owner:FixedKeySlot,source:RequestSourceVersion):RequestTerminalReceipt|undefined{const terminal=this.#terminals[owner.index];return this.key(owner)&&terminal&&this.#same(terminal.source,source)?terminal:undefined;}
 beginExecution(owner:FixedKeySlot,source:RequestSourceVersion,job:number):boolean{if(!this.ready(owner)||this.#executions[owner.index]||this.terminal(owner,source))return false;this.#executions[owner.index]={source,job};return true;}
 recordTerminal(owner:FixedKeySlot,source:RequestSourceVersion,job:number,kind:RequestTerminalReceipt["kind"]):boolean{const original=this.#executions[owner.index];if(!this.key(owner)||!original||original.job!==job||!this.#same(original.source,source))return false;this.#terminals[owner.index]={source,job,kind};this.#executions[owner.index]=undefined;return true;}
 retry(owner:FixedKeySlot,source:RequestSourceVersion):boolean{if(!this.terminal(owner,source))return false;this.#terminals[owner.index]=undefined;return true;}
 #key(scope:number,key:string):string{return `${scope}:${key.length}:${key}`;}
 lookup(scope:number,key:string):FixedKeySlot|undefined{return this.#owners.lookup(this.#key(scope,key));}
 binding(scope:number,consumer:string):FixedKeySlot|undefined{const slot=this.#consumers.lookup(this.#key(scope,consumer));return slot?this.#pending[slot.index]??this.#bindings[slot.index]:undefined;}
 references(slot:FixedKeySlot):number{return this.key(slot)?this.#references[slot.index]!:0;}
 key(slot:FixedKeySlot):readonly[number,string]|undefined{const key=this.#keys[slot.index];if(!key)return;const live=this.lookup(...key);return live?.issuer===slot.issuer&&live.generation===slot.generation?key:undefined;}
 liveBinding(scope:number,consumer:string):FixedKeySlot|undefined{const slot=this.#consumers.lookup(this.#key(scope,consumer));return slot?this.#bindings[slot.index]:undefined;}
 ready(owner:FixedKeySlot):boolean{return!!this.key(owner)&&this.#bindings.some(binding=>this.#sameSlot(binding,owner));}
 #sameSlot(a:FixedKeySlot|undefined,b:FixedKeySlot):boolean{return a?.issuer===b.issuer&&a.index===b.index&&a.generation===b.generation;}
 #drop(owner:FixedKeySlot):FixedKeySlot|undefined{this.#references[owner.index]!--;if(this.#references[owner.index]===0){this.#closing[owner.index]=true;return owner;}}
 admit(scope:number,consumer:string,key:string):{owner:FixedKeySlot;fresh:boolean;retired?:FixedKeySlot}|undefined{return this.#bind(scope,consumer,key,false);}
 stage(scope:number,consumer:string,key:string):{owner:FixedKeySlot;fresh:boolean;retired?:FixedKeySlot}|undefined{return this.#bind(scope,consumer,key,true);}
 #bind(scope:number,consumer:string,key:string,pending:boolean):{owner:FixedKeySlot;fresh:boolean;retired?:FixedKeySlot}|undefined{
  if(!Number.isSafeInteger(scope)||scope<=0||!consumer||!key)return;const existing=this.lookup(scope,key);if(existing&&this.#closing[existing.index])return;
  const owner=this.#owners.claim(this.#key(scope,key));if(!owner)return;const fresh=!existing;const consumerSlot=this.#consumers.claim(this.#key(scope,consumer));if(!consumerSlot){if(fresh)this.#owners.release(this.#key(scope,key),owner);return;}
  const index=consumerSlot.index;if(fresh)this.#keys[owner.index]=[scope,key];
  if(pending){if(this.#sameSlot(this.#bindings[index],owner)){const previous=this.#pending[index];this.#pending[index]=undefined;return{owner,fresh,retired:previous?this.#drop(previous):undefined};}if(this.#sameSlot(this.#pending[index],owner))return{owner,fresh};const previous=this.#pending[index];this.#pending[index]=owner;this.#references[owner.index]!++;return{owner,fresh,retired:previous?this.#drop(previous):undefined};}
  const candidate=this.#pending[index],previous=this.#bindings[index];this.#pending[index]=undefined;this.#bindings[index]=owner;if(!this.#sameSlot(candidate,owner)&&!this.#sameSlot(previous,owner))this.#references[owner.index]!++;const retiredCandidate=candidate&&!this.#sameSlot(candidate,owner)?this.#drop(candidate):undefined;const retiredPrevious=previous&&!this.#sameSlot(previous,owner)?this.#drop(previous):undefined;return{owner,fresh,retired:retiredPrevious??retiredCandidate};
 }
 remove(scope:number,consumer:string):FixedKeySlot|undefined{const slot=this.#consumers.lookup(this.#key(scope,consumer));if(!slot)return;const previous=this.#bindings[slot.index],candidate=this.#pending[slot.index];this.#bindings[slot.index]=undefined;this.#pending[slot.index]=undefined;if(previous)this.#drop(previous);if(candidate)this.#drop(candidate);this.#consumers.release(this.#key(scope,consumer),slot);return previous??candidate;}

 release(slot:FixedKeySlot):boolean{const key=this.key(slot);if(!key||this.references(slot)!==0||!this.#owners.release(this.#key(...key),slot))return false;this.#keys[slot.index]=undefined;this.#closing[slot.index]=false;this.#executions[slot.index]=undefined;this.#terminals[slot.index]=undefined;return true;}
}
