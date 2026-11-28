import {createTakenMemoV1,type TakenMemoV1} from "../../📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs";
import {observationRowsHoldV1,retainObservedRowsV1} from "../../../👁️observation/🟨️.mjs";
import {SLICE_RECYCLE_POLICY_V1,sliceRecycleCarriedV1,sliceRecycleDecisionV1,sliceRecycleErrorV1} from "../../../🧠️memory/♻️recycle/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

type ReusePortsV1={readonly environment?:Readonly<Record<string,string|undefined>>;readonly heapBytes?:()=>number;readonly say?:(line:string)=>void;readonly memo?:TakenMemoV1};

const fill=(template:string,values:Readonly<Record<string,string|number>>)=>template.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]));

const noticeSinksV1=new Set<(notice:Readonly<{round:number;moved:number;evicted:number;heap:number}>)=>void>();

/** 📡️ Registers a listener of the pass boundaries and registry follow-ups of this process (a slice worker reports them to its service, which prints them in both languages: a read that starts a pass again says why). Returns the function that removes it.
 * @see ../🧭️registry-follow/🟦️.ts */
export function onGraphPassNoticeV1(listener:(notice:Readonly<{round:number;moved:number;evicted:number;heap:number}>)=>void):()=>void{noticeSinksV1.add(listener);return()=>{noticeSinksV1.delete(listener);};}

/** 📣️ Hands one pass boundary to every listener; a failing listener never fails the read. */
export function emitGraphPassNoticeV1(notice:Readonly<{round:number;moved:number;evicted:number;heap:number}>):void{for(const sink of noticeSinksV1)try{sink(notice);}catch{}}

/** 🫀 The live heap of this process in bytes, measured after a collection (the garbage a pass leaves is not what slows the next one). */
function liveHeapBytesV1():number{(globalThis as {Bun?:{gc?:(force:boolean)=>void}}).Bun?.gc?.(true);return process.memoryUsage().heapUsed;}

/** 🧠️ The taken memo of one read: created once by the read and handed to every attempt of it, so a registry follow that restarts the attempt keeps the owners the earlier passes took (a pass answers an owner from it only while its job is unchanged and every row it consumed still holds).
 * @see ../🧭️registry-follow/🟦️.ts */
export function createReadTakenMemoV1():TakenMemoV1{return createTakenMemoV1({holds:rows=>observationRowsHoldV1(rows as never),retain:rows=>retainObservedRowsV1(rows as never)});}

/** ♻️ What one read keeps from one pass over the owners to the next, and when the worker that runs it hands it over: the memo of the owners the read took (a pass answers an owner from it only while its job is unchanged and every row it consumed still holds), and the recycle of the worker at the boundary between two passes when its live heap is above the declared threshold, carrying the counters of the read to the fresh worker (what the read finished is stored on disk and is taken again from there in a fraction of the time).
 * @see ../../📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs
 * @see ../../../🧠️memory/♻️recycle/🟨️.mjs
 * @see ./🧪️tests/🟦️.ts */
export function createPassReuseV1(ports:ReusePortsV1={}){
  const environment=ports.environment??process.env,carried=sliceRecycleCarriedV1(environment),memo=ports.memo??createReadTakenMemoV1();
  const say=ports.say??((line:string)=>{process.stderr.write(line);}),measure=ports.heapBytes??liveHeapBytesV1,enabled=Number(environment[SLICE_RECYCLE_POLICY_V1.heapEnvironment])>0;
  let passes=0;
  if(ports.memo!==undefined)(memo as Partial<{distrust():void}>).distrust?.();
  return Object.freeze({
    memo,carried,
    boundary(input:{readonly round:number;readonly moved:readonly string[]}):void{
      if(passes++===0)return;
      const before=memo.size,evicted=memo.evictReferencing(input.moved),kept=before-evicted,heapBytes=enabled?measure():Number.NaN,heap=Number.isFinite(heapBytes)?Math.round(heapBytes/1048576):0;
      const values={pass:passes,round:input.round,moved:input.moved.length,evicted,entries:memo.size,heap},template=schema.$defs.Texts.const.boundary;
      emitGraphPassNoticeV1({round:input.round,moved:input.moved.length,evicted,heap});
      say("[graph-reuse] "+JSON.stringify({version:1,...values,recycles:carried.recycles,text:{en:fill(template.en,values),de:fill(template.de,values)}})+"\n");
      const decision=enabled?sliceRecycleDecisionV1({environment,heapBytes,kept,evicted,carried}):undefined;
      if(decision!==undefined)throw sliceRecycleErrorV1({decision,round:input.round,passes:passes-1,owners:memo.size,carried});
    },
  });
}
