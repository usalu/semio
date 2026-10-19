import {IndexedOutputCursor,type IndexedEditPlan} from "../🟦️.ts";
/** 🚚️ Transfers one existing object or one authored addition through the admitted collection stream. */
export class IndexedCollectionCommit<T>{
 private stage:"begin"|"stream"|"install"|"done"|"closed"="begin";
 private original:T[]|null=null;
 private output:T[]|null=null;
 private remaining=0;
 private closing=false;
 private readonly cursor=new IndexedOutputCursor();
 get complete():boolean{return this.stage==="done";}
 step(plan:IndexedEditPlan,target:{value:T[]},added:(ordinal:number)=>T,retire:(value:T)=>void):void{
  if(!plan.ready||this.closing)throw new Error("collection transfer requires admitted preflight");
  if(this.stage==="done")return;
  if(this.stage==="begin"){
   if(target.value.length!==plan.baseLength)throw new Error("collection base cardinality changed");
   if(plan.entries(0).length===0&&plan.entries(2).length===0){this.stage="done";return;}
   this.original=target.value;target.value=[];this.output=[];this.stage="stream";return;
  }
  if(target.value.length!==0)throw new Error("unpublished collection ownership changed");
  if(this.stage==="install"){target.value=this.output!;this.output=null;this.stage="done";return;}
  const work=this.cursor.next(plan);
  if(work.kind==="complete"){this.stage="install";return;}
  if(work.kind==="added")this.output!.push(added(work.ordinal));
  else{if(work.index!==this.remaining)throw new Error("original collection order changed");const value=this.original![this.remaining] as T;delete this.original![this.remaining++];if(work.kind==="base")this.output!.push(value);else retire(value);}
  this.cursor.advance(plan);
 }
 beginClose():void{this.closing=true;}
 get terminalEmpty():boolean{return this.closing&&this.original===null&&this.output===null;}
 closeStep(retire:(value:T)=>void):void{
  if(!this.closing)throw new Error("collection close has not begun");
  if(this.original){if(this.remaining<this.original.length){const value=this.original[this.remaining]as T;delete this.original[this.remaining++];retire(value);}else this.original=null;return;}
  if(this.output){if(this.output.length)retire(this.output.pop()as T);else this.output=null;}
 }
}
