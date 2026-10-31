import {RetainedWorkLedger,RetainedWorkTurnCounter,type RetainedWorkTurn,type RetainedWorkDemand} from "../../../📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {NumericPages,NUMERIC_NODE_BYTES,type NumericPageProgress} from './📚️pages/🟦️.ts';
import type {NumericReserveGrant,NumericReserveDemand,NumericReserveReceipt} from './🎟️reservation/🟦️.ts';
export type {NumericReserveGrant,NumericReserveDemand,NumericReserveReceipt} from './🎟️reservation/🟦️.ts';
/** 🔢️ Mutable fixed-key AVL scratch storage retains spent slots through logical resets. */
export class NumericIndex<V> {
  private nodes=new NumericPages<V>();
 private workBorrowed=false;
 private available():void{if(this.workBorrowed)throw Error("Numeric index is borrowed by an original work cursor");}
  private node(index:number){const node=this.nodes.get(index);if(!node)throw Error('numeric.index.node');return node;}
  private root: number | undefined;
  private generation = 1;
  private height(index: number | undefined): number { return index === undefined ? 0 : this.node(index).height; }
  private count(index: number | undefined): number { return index === undefined || this.node(index).countGeneration !== this.generation ? 0 : this.node(index).count; }
  private find(key: number): number | undefined { let current = this.root; while (current !== undefined) { const node = this.node(current); if (key === node.key) return current; current = key < node.key ? node.left : node.right; } return undefined; }
  private update(index: number): void { const node = this.node(index); node.height = 1 + Math.max(this.height(node.left), this.height(node.right)); node.count = this.count(node.left) + this.count(node.right) + Number(node.generation === this.generation && node.value !== undefined); node.countGeneration = this.generation; }
  private replace(parent: number | undefined, before: number, after: number): void { if (parent === undefined) this.root = after; else if (this.node(parent).left === before) this.node(parent).left = after; else this.node(parent).right = after; this.node(after).parent = parent; }
  private rotate(index: number, left: boolean): number { const node = this.node(index); const child = (left ? node.right : node.left)!; const middle = left ? this.node(child).left : this.node(child).right; this.replace(node.parent, index, child); if (left) { node.right = middle; this.node(child).left = index; } else { node.left = middle; this.node(child).right = index; } if (middle !== undefined) this.node(middle).parent = index; node.parent = child; this.update(index); this.update(child); return child; }
  private repair(current: number | undefined): void { while (current !== undefined) { this.update(current); const node = this.node(current); const balance = this.height(node.left) - this.height(node.right); let top = current; if (balance > 1) { const child = node.left!; if (this.height(this.node(child).left) < this.height(this.node(child).right)) this.rotate(child, true); top = this.rotate(current, false); } else if (balance < -1) { const child = node.right!; if (this.height(this.node(child).right) < this.height(this.node(child).left)) this.rotate(child, false); top = this.rotate(current, true); } current = this.node(top).parent; } }
  get size(): number { this.available();return this.count(this.root); }
  get storedKeys(): number { return this.nodes.length; }
  get depth(): number { return this.height(this.root); }
  get(key: number): V | undefined { this.available();const index = this.find(key); return index === undefined || this.node(index).generation !== this.generation ? undefined : this.node(index).value; }
  get hasReservedSlot():boolean{return this.nodes.hasReservedSlot;}
  get allocatedBytes():number{return this.nodes.allocatedBytes;}
  nextAllocationBytes():number{return this.nodes.nextAllocationBytes();}
  nextReserveDemand():NumericReserveDemand{return{copyBytes:this.nodes.nextReserveCopyByteDemand(),capacityBytes:this.nodes.nextAllocationBytes(),releaseBytes:0,depth:this.nodes.nextReserveDepthDemand()};}
  reserveOneFunded(grant:NumericReserveGrant):NumericReserveReceipt{this.available();for(const value of Object.values(grant))if(!Number.isSafeInteger(value)||value<0)throw RangeError('numeric.index.grant');const d=this.nextReserveDemand();if(grant.maximumItems===0||grant.maximumCopyBytes<d.copyBytes||grant.maximumCapacityBytes<d.capacityBytes||grant.maximumDepth<d.depth)return{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};const p=this.nodes.reserveOne(grant.maximumCapacityBytes);return{copiedItems:Number(p.progressed),copiedBytes:p.progressed?d.copyBytes:0,retainedCapacityBytes:p.allocatedBytes,releasedBytes:0};}
  reserveOne(maximumBytes:number):NumericPageProgress{this.available();return this.nodes.reserveOne(maximumBytes);}
  terminalIsEmpty():boolean{return this.nodes.terminalIsEmpty();}
  nextCloseReleaseByteDemand():number{return this.nodes.nextReleaseBytes();}
  /** 🧹️ Retires one scalar row or one original fixed block; values must have independent POD custody. */
  nextCloseCopyByteDemand():number{return this.nodes.length===0?0:NUMERIC_NODE_BYTES;}
  /** ❄️ Cold scalar disposal; retained callers use the independently funded close operation. */
  closeCopyStep(maximumItems:number,maximumReleaseBytes:number):NumericPageProgress{return this.closeCopyStepFunded(maximumItems,Number.MAX_SAFE_INTEGER,maximumReleaseBytes);}
  closeCopyStepFunded(maximumItems:number,maximumCopyBytes:number,maximumReleaseBytes:number):NumericPageProgress{this.available();
    if(!Number.isSafeInteger(maximumCopyBytes)||maximumCopyBytes<0)throw RangeError('numeric.index.copy');if(maximumCopyBytes<this.nextCloseCopyByteDemand())return{progressed:false,allocatedBytes:0,releasedAllocationBytes:0};
    if(!Number.isSafeInteger(maximumItems)||maximumItems<0)throw RangeError('numeric.index.items');if(maximumItems===0)return{progressed:false,allocatedBytes:0,releasedAllocationBytes:0};
    this.root=undefined;if(this.nodes.popCopy())return{progressed:true,allocatedBytes:0,releasedAllocationBytes:0};return this.nodes.releaseOne(maximumReleaseBytes);
  }
  set(key:number,value:V):V|undefined{this.available();if(!Number.isSafeInteger(key)||value===undefined)throw RangeError("Numeric index requires a fixed integer key and a present value");while(!this.hasReservedSlot&&this.find(key)===undefined)this.reserveOne(this.nextAllocationBytes());return this.setReserved(key,value);}
  setReserved(key: number, value: V): V | undefined {this.available();
    if (!Number.isSafeInteger(key) || value === undefined) throw new RangeError("Numeric index requires a fixed integer key and a present value");
    let parent: number | undefined; let current = this.root;
    while (current !== undefined) { const node = this.node(current); if (key === node.key) { const previous = node.generation === this.generation ? node.value : undefined; node.value = value; node.generation = this.generation; this.repair(current); return previous; } parent = current; current = key < node.key ? node.left : node.right; }
    const index = this.nodes.length; this.nodes.pushReserved({ key, value, generation: this.generation, countGeneration: this.generation, count: 1, height: 1, left: undefined, right: undefined, parent }); if (parent === undefined) this.root = index; else if (key < this.node(parent).key) this.node(parent).left = index; else this.node(parent).right = index; this.repair(parent); return undefined;
  }
  remove(key: number): V | undefined {this.available(); const index = this.find(key); if (index === undefined || this.node(index).generation !== this.generation) return undefined; const value = this.node(index).value; this.node(index).value = undefined; this.repair(index); return value; }
  popFirst(): [number, V] | undefined { let current = this.root; if (this.size === 0 || current === undefined) return undefined; for (;;) { const node = this.node(current); if (this.count(node.left) > 0) { current = node.left!; continue; } if (node.generation === this.generation && node.value !== undefined) return [node.key, this.remove(node.key)!]; current = node.right!; } }
  reset(): void {this.available(); if (this.generation === Number.MAX_SAFE_INTEGER) throw new RangeError("Numeric scratch generation exhausted"); this.generation++; }

 /** 🎟️ The original index remains exclusively borrowed until the funded insertion result is claimed. */
 beginWorkInsert(key:number,value:V):NumericWorkInsertCursor<V>{return this.beginWorkEdit(key,value,false);}
 beginWorkRemove(key:number):NumericWorkRemoveCursor<V>{return this.beginWorkEdit(key,undefined,true);}
 private beginWorkEdit(key:number,value:V|undefined,remove:boolean):NumericWorkEditCursor<V>{
  this.available();if(!Number.isSafeInteger(key)||(!remove&&value===undefined))throw RangeError("Numeric work insertion requires a fixed key and present value");this.workBorrowed=true;
  const counter=new RetainedWorkTurnCounter();
  let phase:"root"|"search"|"reserve"|"link"|"replace"|"remove"|"repair"|"complete"="root",current:number|undefined,parent:number|undefined,previous:V|undefined,inserted=false,claimed=false,cancelled=false,turns=0,last=0,lastSemantic=0,semantic=false;
  const complete=()=>phase==="complete";
  const rowDemand=():RetainedWorkDemand=>({copyBytes:NUMERIC_NODE_BYTES,capacityBytes:0,releaseBytes:0,depth:2});
  const reserveDemand=():RetainedWorkDemand=>{if(this.nodes.hasReservedSlot)return rowDemand();const quote=this.nextReserveDemand();return{...quote,depth:quote.depth+1};};
  const demand=():RetainedWorkDemand=>{if(phase==="complete"||(phase==="repair"&&current===undefined)||(phase==="search"&&current===undefined&&remove))return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:0};if(phase==="reserve"||(phase==="search"&&current===undefined&&!remove))return reserveDemand();return phase==="link"?rowDemand():{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};};
  const step=(work:RetainedWorkLedger,turn:RetainedWorkTurn):void=>{
   semantic=false;
   if(cancelled&&!inserted&&phase!=="repair"){phase="complete";return;}
   if(phase==="root"){current=this.root;turn.commitActualWork();semantic=true;phase="search";return;}
   if(phase==="search"){if(current===undefined){phase=remove?"complete":"reserve";return;}const row=this.node(current);turn.commitActualWork();semantic=true;if(key===row.key){previous=row.generation===this.generation?row.value:undefined;phase=remove?(previous===undefined?"complete":"remove"):"replace";return;}parent=current;current=key<row.key?row.left:row.right;return;}
   if(phase==="reserve"){if(this.nodes.hasReservedSlot){phase="link";return;}const quote=this.nextReserveDemand(),grant=work.childGrant;if(grant.maximumItems===0||grant.maximumCopyBytes<quote.copyBytes||grant.maximumCapacityBytes<quote.capacityBytes||grant.maximumDepth<quote.depth){work.requireChild(quote);return;}const actual=this.nodes.reserveOne(grant.maximumCapacityBytes);if(actual.progressed){turn.commitActualWork();work.notePhysicalEffect();}work.consumeChild({copiedItems:Number(actual.progressed),copiedBytes:actual.progressed?quote.copyBytes:0,retainedCapacityBytes:actual.allocatedBytes,releasedBytes:actual.releasedAllocationBytes});return;}
   if(phase==="link"){const quote=demand();if(!work.copyBytes(quote.copyBytes))return;const at=this.nodes.length;this.nodes.pushReserved({key,value:value!,generation:this.generation,countGeneration:this.generation,count:1,height:1,left:undefined,right:undefined,parent});work.notePhysicalEffect();turn.commitActualWork();semantic=true;if(parent===undefined)this.root=at;else if(key<this.node(parent).key)this.node(parent).left=at;else this.node(parent).right=at;inserted=true;current=parent;phase="repair";return;}
   if(phase==="replace"){const row=this.node(current!);row.value=value!;work.notePhysicalEffect();turn.commitActualWork();semantic=true;row.generation=this.generation;inserted=true;phase="repair";return;}
   if(phase==="remove"){this.node(current!).value=undefined;work.notePhysicalEffect();turn.commitActualWork();semantic=true;inserted=true;phase="repair";return;}
   if(phase==="repair"){if(current===undefined){phase="complete";return;}const at=current;this.update(at);turn.commitActualWork();semantic=true;const row=this.node(at),balance=this.height(row.left)-this.height(row.right);let top=at;if(balance>1){const child=row.left!;if(this.height(this.node(child).left)<this.height(this.node(child).right))this.rotate(child,true);top=this.rotate(at,false);}else if(balance< -1){const child=row.right!;if(this.height(this.node(child).right)<this.height(this.node(child).left))this.rotate(child,false);top=this.rotate(at,true);}current=this.node(top).parent;}
  };
  const cursor:NumericWorkEditCursor<V>=Object.freeze({nextWorkDemand:()=>({...demand()}),lastTurnWork:()=>last,lastTurnSemanticWork:()=>lastSemantic,cancel:()=>{if(claimed)throw Error("Numeric insertion already claimed");cancelled=true;},advance:(items:number,work:RetainedWorkLedger)=>{last=0;lastSemantic=0;const turn=counter.beginTurn();let entered:boolean|null=null;try{if(claimed)throw Error("Numeric insertion already claimed");if(!Number.isSafeInteger(items)||items<1)throw RangeError("Numeric insertion item quantum");if(complete())return{done:true,work:turns};entered=work.enter();while(entered!==null&&turn.actualWork()<items&&!complete()&&work.remaining.maximumItems>0){const before=turn.actualWork(),beforePhase=phase;try{step(work,turn);}finally{if(semantic){turns++;lastSemantic++;}}if(turn.actualWork()>before)work.noteSource(entered);if(work.blocked||(before===turn.actualWork()&&beforePhase===phase))break;}}finally{if(entered!==null)work.exit();last=counter.endTurn(turn);}return{done:complete(),work:turns};},takeResult:()=>{if(claimed||phase!=="complete")throw Error("Numeric insertion result is unavailable");claimed=true;this.workBorrowed=false;return{previous:inserted?previous:undefined,input:inserted?undefined:value,committed:inserted,cancelled};}});return cursor;
 }


 /** 🧹️ Each original value closes before its row is cleared and its actual page backing is released. */
 beginWorkClose(owner:NumericWorkValueClose<V>):NumericWorkCloseCursor{
  this.available();if(!owner||typeof owner.close!=="function"||typeof owner.nextWorkDemand!=="function")throw TypeError("Numeric value close ownership is required");this.workBorrowed=true;
  const counter=new RetainedWorkTurnCounter();
  let phase:"root"|"value"|"row"|"release"|"complete"="root",turns=0,last=0;
  const complete=()=>phase==="complete";
  const demand=():RetainedWorkDemand=>{if(phase==="root")return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};if(phase==="row")return{copyBytes:NUMERIC_NODE_BYTES,capacityBytes:0,releaseBytes:0,depth:2};if(phase==="value"&&this.nodes.length){const value=this.node(this.nodes.length-1).value;if(value!==undefined){const child=owner.nextWorkDemand(value);return{...child,depth:child.depth+1};}}if(phase==="release"||(phase==="value"&&this.nodes.length===0))return{copyBytes:0,capacityBytes:0,releaseBytes:this.nodes.nextReleaseBytes(),depth:this.nodes.nextReleaseBytes()?2:1};return{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:phase==="complete"?0:1};};
  const step=(work:RetainedWorkLedger,turn:RetainedWorkTurn):void=>{if(phase==="root"){if(this.root!==undefined){this.root=undefined;turn.commitActualWork();}phase="value";return;}if(phase==="value"){if(this.nodes.length===0){phase="release";return;}const row=this.node(this.nodes.length-1);if(row.value!==undefined&&!owner.close(row.value,work,turn))return;phase="row";return;}if(phase==="row"){if(!work.copyBytes(NUMERIC_NODE_BYTES))return;if(!this.nodes.popCopy())throw Error("Numeric original row disappeared");work.notePhysicalEffect();turn.commitActualWork();phase="value";return;}if(phase==="release"){const bytes=this.nodes.nextReleaseBytes();if(bytes===0){if(!this.nodes.terminalIsEmpty())throw Error("Numeric original page release quote missing");phase="complete";this.workBorrowed=false;return;}if(!work.releaseBytes(bytes))return;const actual=this.nodes.releaseOne(bytes);if(actual.releasedAllocationBytes!==bytes)throw Error("Numeric original release differs from quote");work.notePhysicalEffect();turn.commitActualWork();}};
  return Object.freeze({nextWorkDemand:()=>({...demand()}),lastTurnWork:()=>last,terminalIsEmpty:()=>phase==="complete",advance:(items:number,work:RetainedWorkLedger)=>{last=0;const turn=counter.beginTurn();let entered:boolean|null=null;try{if(!Number.isSafeInteger(items)||items<1)throw RangeError("Numeric close item quantum");if(complete())return{done:true,work:turns};entered=work.enter();while(entered!==null&&turn.actualWork()<items&&!complete()&&work.remaining.maximumItems>0){const before=turn.actualWork(),beforePhase=phase;step(work,turn);if(turn.actualWork()>before)work.noteSource(entered);if(work.blocked||(before===turn.actualWork()&&beforePhase===phase))break;}}finally{if(entered!==null)work.exit();last=counter.endTurn(turn);turns+=last;}return{done:complete(),work:turns};}});
 }

}

/** 🧭️ Scalar cursor state retains the original input and displaced value until explicit result custody. */
export interface NumericWorkEditCursor<V>{nextWorkDemand():RetainedWorkDemand;lastTurnWork():number;lastTurnSemanticWork():number;advance(items:number,work:RetainedWorkLedger):{done:boolean;work:number};cancel():void;takeResult():{previous:V|undefined;input:V|undefined;committed:boolean;cancelled:boolean}}

export interface NumericWorkInsertCursor<V> extends NumericWorkEditCursor<V>{}
export interface NumericWorkRemoveCursor<V> extends NumericWorkEditCursor<V>{}
export interface NumericWorkValueClose<V>{nextWorkDemand(value:V):RetainedWorkDemand;close(value:V,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean}
export interface NumericWorkCloseCursor{nextWorkDemand():RetainedWorkDemand;lastTurnWork():number;terminalIsEmpty():boolean;advance(items:number,work:RetainedWorkLedger):{done:boolean;work:number}}
