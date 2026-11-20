import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {createNativeStorageVerificationTimingsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/⏱️verification/🟦️.ts";

const stages=Object.keys(schema.definitions.ConsumerTimings.properties),bodyStages=Object.keys(schema.definitions.BodyTimings.properties);
const limit=schema.definitions.TimingNumber.maximum;

/** ⏱️ Counts elapsed batch milliseconds by phase, including active work. @see ../🧬️schema/🔣️.json */
export function createInferenceInventoryTimingsV1(clock=()=>performance.now()){
 const rows=names=>new Map(names.map(stage=>[stage,{started:0,active:0,elapsed:0,at:0}])),counters=rows(stages),operations=rows(bodyStages);
 const before=createNativeStorageVerificationTimingsV1(clock),after=createNativeStorageVerificationTimingsV1(clock),native=Object.freeze({preVerify:before.observe,postVerify:after.observe});
 const elapsed=(row,now)=>Math.min(limit,row.elapsed+(now-row.at)*row.active);
 const move=(row,now,delta)=>{row.elapsed=elapsed(row,now);row.at=now;row.active+=delta;if(delta>0)row.started=Math.min(limit,row.started+1);};
 const batch=()=>{
  let previous;
  return stage=>{
   const now=clock(),next=counters.get(stage);
   if(previous)move(previous,now,-1);
   if(next)move(next,now,1);
   previous=next;
  };
 };
 const body=(operation)=>{
  const row=operations.get(operation);move(row,clock(),1);
  return ()=>move(row,clock(),-1);
 };
 const snapshot=(rows,onlyStarted)=>{
  const now=clock();
  return Object.fromEntries([...rows].filter(([,row])=>!onlyStarted||row.started>0).map(([stage,row])=>[stage,{started:row.started,active:Math.min(limit,row.active),milliseconds:Math.floor(elapsed(row,now))}]));
 };
 const stats=()=>snapshot(counters,false);
 const bodyOperations=()=>{const rows=snapshot(operations,true);return Object.keys(rows).length?rows:undefined;};
 const verification=()=>{const preVerify=before.stats(),postVerify=after.stats();return Object.keys(preVerify).length||Object.keys(postVerify).length?{preVerify,postVerify}:undefined;};
 return {batch,stats,body,bodyOperations,native,verification};
}
