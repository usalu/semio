import{PersistentListBuilder,type BuiltPersistentList}from"../../../../../🔗️persistent/📥️build/🟦️.ts";
import{NamedKeyIndexPreparation,NamedPersistentRoot,persistentNamedKeySource,type AdmittedNamedKeyIndex}from"../../🟦️.ts";
import type{NamedEditDemand,NamedEditGrant,NamedEditReceipt}from"../../../🟦️.ts";
/** 🫴️ Required typed original child owners expose their own funded work and explicit transfer. */
export interface NativeNamedRowReceiving<T>{readonly ready:boolean;demand(body:number):NamedEditDemand;advance(grant:NamedEditGrant):NamedEditReceipt;take(grant:NamedEditGrant):T|undefined;close():void;}
const zero=():NamedEditReceipt=>({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0});
/** 🛬️ The existing page builder and named admission owner compose behind one original typed source. */
export class NativeNamedRootReceiving<T>{
 #original:unknown;#rows:readonly unknown[]|undefined;#childFactory:(row:unknown)=>NativeNamedRowReceiving<T>;#key:(row:T)=>Uint8Array;#builder:PersistentListBuilder<T>|undefined;#child:NativeNamedRowReceiving<T>|undefined;#row:T|undefined;#root:BuiltPersistentList<T>|undefined;#admission:NamedKeyIndexPreparation|undefined;#index:AdmittedNamedKeyIndex|undefined;#output:NamedPersistentRoot<T>|undefined;#ordinal=0;#admissions=0;#phase:"bind"|"birth"|"receive"|"takeRow"|"adopt"|"build"|"finish"|"takeList"|"admissionBirth"|"admission"|"admissionTake"|"publish"|"ready"|"refused"|"spent"|"closed"="bind";
 constructor(original:unknown,child:(row:unknown)=>NativeNamedRowReceiving<T>,key:(row:T)=>Uint8Array){this.#original=original;this.#childFactory=child;this.#key=key;}
 get ready():boolean{return this.#phase==="ready";}
 get settled():boolean{return["ready","refused","spent","closed"].includes(this.#phase);}
 get baseAdmissions():number{return this.#admissions;}
 get empty():boolean{return this.#phase==="closed"&&this.#original===undefined&&this.#builder===undefined&&this.#child===undefined&&this.#root===undefined&&this.#admission===undefined&&this.#index===undefined&&this.#output===undefined;}
 demand(body:number):NamedEditDemand{if(this.settled)throw Error("original named receiving is settled");if(this.#phase==="receive"){const d=this.#child!.demand(Math.max(0,body-128));return{...d,copyBytes:d.copyBytes+128,depth:d.depth+1};}if(this.#phase==="admission"){const d=this.#admission!.demand(Math.max(0,body-128));return{...d,copyBytes:d.copyBytes+128,depth:d.depth+1};}return{copyBytes:256,capacityBytes:0,depth:1};}
 advance(g:NamedEditGrant):NamedEditReceipt{const d=this.demand(g.maximumCopyBytes);if(g.maximumItems<1||g.maximumCopyBytes<d.copyBytes||g.maximumCapacityBytes<d.capacityBytes||g.maximumDepth<d.depth)return zero();const p={copiedItems:1,copiedBytes:d.copyBytes,retainedCapacityBytes:0};switch(this.#phase){
  case"bind":if(!Array.isArray(this.#original))throw Error("original named source is not an array");this.#rows=this.#original;this.#builder=new PersistentListBuilder<T>(this.#rows.length);this.#phase=this.#rows.length?"birth":"finish";break;
  case"birth":this.#child=this.#childFactory(this.#rows![this.#ordinal]);this.#phase="receive";break;
  case"receive":{const c=this.#child!.advance({...g,maximumCopyBytes:g.maximumCopyBytes-128,maximumDepth:g.maximumDepth-1});if(this.#child!.ready)this.#phase="takeRow";return{...c,copiedBytes:c.copiedBytes+128};}
  case"takeRow":this.#row=this.#child!.take(g);if(this.#row===undefined)throw Error("funded original row omitted output");this.#child!.close();this.#child=undefined;this.#phase="adopt";break;
  case"adopt":this.#builder!.adopt(this.#row!);this.#row=undefined;this.#ordinal++;this.#phase=!this.#builder!.acceptsRow?"build":this.#ordinal<this.#rows!.length?"birth":"finish";break;
  case"build":this.#builder!.advance();if(this.#builder!.ready)this.#phase="takeList";else if(this.#builder!.acceptsRow)this.#phase=this.#ordinal<this.#rows!.length?"birth":"finish";break;
  case"finish":this.#builder!.finish();this.#phase="build";break;
  case"takeList":this.#root=this.#builder!.take();this.#builder!.close();this.#builder=undefined;this.#phase="admissionBirth";break;
  case"admissionBirth":this.#admission=new NamedKeyIndexPreparation(persistentNamedKeySource(this.#root!,this.#key));this.#admissions++;this.#phase="admission";break;
  case"admission":{const c=this.#admission!.advance({...g,maximumCopyBytes:g.maximumCopyBytes-128,maximumDepth:g.maximumDepth-1});if(this.#admission!.settled)this.#phase="admissionTake";return{...c,copiedBytes:c.copiedBytes+128};}
  case"admissionTake":this.#index=this.#admission!.take(g);this.#admission=undefined;this.#phase=this.#index?"publish":"refused";break;
  case"publish":this.#output=new NamedPersistentRoot(this.#root!,this.#index!,this.#key);this.#root=undefined;this.#index=undefined;this.#phase="ready";break;
  default:throw Error("original named receiving phase is invalid");
 }return p;}
 take():NamedPersistentRoot<T>{if(!this.ready||!this.#output)throw Error("original named output is not ready");const root=this.#output;this.#output=undefined;this.#phase="spent";return root;}
 close():void{this.#child?.close();this.#builder?.close();this.#root?.close();this.#output?.close();this.#original=undefined;this.#rows=undefined;this.#child=undefined;this.#builder=undefined;this.#row=undefined;this.#root=undefined;this.#admission=undefined;this.#index=undefined;this.#output=undefined;this.#phase="closed";}
}
