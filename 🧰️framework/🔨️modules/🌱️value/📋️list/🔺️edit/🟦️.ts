import {HeapSortCursor} from "../../📶️sort/🟦️.ts";

export type IndexedEditEntry={idx:number;ordinal:number};
export type IndexedEditFault="remove"|"modify"|"add";
type EditKind=0|1|2;

/** 🧮️ Index-only admission borrows immutable authored coordinates and advances one action per step. */
export class IndexedEditPlan{
 private readonly orders:[IndexedEditEntry[],IndexedEditEntry[],IndexedEditEntry[]]=[[],[],[]];
 private kind:EditKind=0;
 private phase:"fill"|"sort"|"validate"|"settled"="fill";
 private cursor=0;
 private removal=0;
 private sort=new HeapSortCursor(0);
 private closing=false;
 fault:IndexedEditFault|null=null;
 readonly sources:readonly [readonly number[],readonly number[],readonly number[]];
 constructor(readonly baseLength:number,removed:readonly number[],modified:readonly number[],added:readonly number[]){
  if(!Number.isSafeInteger(baseLength)||baseLength<0)throw new RangeError("indexed base length must be exact and nonnegative");
  this.sources=[removed,modified,added];
 }
 get settled():boolean{return this.phase==="settled";}
 get ready():boolean{return this.settled&&this.fault===null&&!this.closing;}
 get outputLength():number{if(!this.ready)throw new Error("indexed plan is not admitted");return this.baseLength-this.orders[0].length+this.orders[2].length;}
 entries(kind:EditKind):readonly IndexedEditEntry[]{if(!this.ready)throw new Error("indexed plan is not admitted");return this.orders[kind];}
 step():void{
  if(this.closing)throw new Error("indexed owner is closing");
  if(this.settled)return;
  const rows=this.orders[this.kind],source=this.sources[this.kind];
  if(this.phase==="fill"){
   if(this.cursor<source.length){const idx=source[this.cursor]!;if(!Number.isSafeInteger(idx)||idx<0){this.refuse();return;}rows.push({idx,ordinal:this.cursor++});}
   else{this.sort=new HeapSortCursor(rows.length);this.phase="sort";}
  }else if(this.phase==="sort"){
   const work=this.sort.next();
   if(work.kind==="complete"){this.cursor=0;this.phase="validate";}
   else if(work.kind==="compare"){const left=rows[work.left]!,right=rows[work.right]!,order=left.idx-right.idx||left.ordinal-right.ordinal;this.sort.advance(order<0?-1:order>0?1:0);}
   else{if(work.kind==="swap")[rows[work.left],rows[work.right]]=[rows[work.right]!,rows[work.left]!];this.sort.advance();}
  }else{
   if(this.cursor===rows.length){if(this.kind===2){if(!Number.isSafeInteger(this.baseLength-this.orders[0].length+rows.length))throw new RangeError("indexed final cardinality overflow");this.phase="settled";}else{this.kind=(this.kind+1)as EditKind;this.cursor=0;this.phase="fill";}return;}
   const row=rows[this.cursor]!;
   if(this.kind===1&&this.removal<this.orders[0].length&&this.orders[0][this.removal]!.idx<row.idx){this.removal++;return;}
   const invalid=this.cursor>0&&rows[this.cursor-1]!.idx===row.idx||this.kind<2&&row.idx>=this.baseLength||this.kind===1&&this.orders[0][this.removal]?.idx===row.idx||this.kind===2&&row.idx>this.baseLength-this.orders[0].length+this.cursor;
   if(invalid)this.refuse();else this.cursor++;
  }
 }
 private refuse():void{this.fault=["remove","modify","add"][this.kind]as IndexedEditFault;this.phase="settled";}
 beginClose():void{this.closing=true;}
 get terminalIsEmpty():boolean{return this.closing&&this.orders.every(rows=>rows.length===0);}
 closeStep():void{if(!this.closing)throw new Error("indexed closure is not admitted");this.orders.find(rows=>rows.length>0)?.pop();}
}

export type IndexedOutputWork={kind:"base"|"removed";index:number}|{kind:"added";ordinal:number}|{kind:"complete"};

/** 🚚️ An admitted collection transfers original or authored headers without shifting any survivors. */
export class IndexedOutputCursor{
 private base=0;
 private output=0;
 private removed=0;
 private added=0;
 next(plan:IndexedEditPlan):IndexedOutputWork{
  if(!plan.ready)throw new Error("indexed output requires admitted preflight");
  if(plan.entries(0)[this.removed]?.idx===this.base)return{kind:"removed",index:this.base};
  const addition=plan.entries(2)[this.added];
  if(addition?.idx===this.output)return{kind:"added",ordinal:addition.ordinal};
  if(this.base<plan.baseLength)return{kind:"base",index:this.base};
  if(this.output!==plan.outputLength||addition)throw new Error("indexed output disagrees with admitted cardinality");
  return{kind:"complete"};
 }
 advance(plan:IndexedEditPlan):void{
  switch(this.next(plan).kind){
   case"base":this.base++;this.output++;break;
   case"removed":this.base++;this.removed++;break;
   case"added":this.output++;this.added++;break;
   case"complete":throw new Error("indexed output is complete");
  }
 }
}
