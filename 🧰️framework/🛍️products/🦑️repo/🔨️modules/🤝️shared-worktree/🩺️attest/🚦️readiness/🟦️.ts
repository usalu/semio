import {readFile,readdir} from "node:fs/promises";
import {join} from "node:path";
import {compileJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import readySchema from "../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json" with {type:"json"};
import treeSchema from "../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧬️schema/🔣️.json" with {type:"json"};

type ReadinessMessageV1=Readonly<{en:string;de:string}>;
type ReadinessReceiptV1=Readonly<{origin:string;state:string;ready:boolean;digest:string;stage:string|null;stageIndex:number;stageCount:number;attempts:number;exit:number|null;peakMegabytes:number|null;refusal:ReadinessMessageV1|null;reason:ReadinessMessageV1|null;tail:readonly string[];updatedMs:number}>;

/** 🚦️ Readiness evidence from a worker's latest valid receipt. Current readiness expires with the Ready sidecar's foreground freshness window. */
export type ReadinessRowV1=Readonly<{originId:string;kind:"worker"|"server";freshness:"fresh"|"stale"|"future"|"missing";currentReady:boolean|null;lastReport:ReadinessReceiptV1|null;ageSeconds:number|null}>;

const policy=readySchema.definitions.Policy.const;
const freshnessMilliseconds=treeSchema.definitions.Policy.const.foregroundFreshMilliseconds;

/** 📋️ Reads the origins in name order and reports only current evidence from valid, origin-bound receipts. The group server has no readiness sidecar. */
export async function readinessReportV1(storeRoot:string,now?:number):Promise<readonly ReadinessRowV1[]>{
 const base=join(storeRoot,"v1","origins");
 let names:string[];
 try{names=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.isDirectory()).map(entry=>entry.name).sort();}catch{return[];}
 const schema=compileJsonSchemaSubset(readySchema.definitions.Status,readySchema);
 return Promise.all(names.map(async originId=>{
  const kind=originId.endsWith("-server")?"server":"worker";
  const empty:ReadinessRowV1={originId,kind,freshness:"missing",currentReady:null,lastReport:null,ageSeconds:null};
  if(kind==="server")return empty;
  let value:unknown;
  try{value=JSON.parse(await readFile(join(base,originId,policy.statusName),"utf8"));}catch{return empty;}
  if(schema.validate(value).length!==0)return empty;
  const lastReport=value as ReadinessReceiptV1;
  if(lastReport.origin!==originId)return empty;
  const age=(now??Date.now())-lastReport.updatedMs;
  const freshness=age<0?"future":age>freshnessMilliseconds?"stale":"fresh";
  const currentReady=freshness==="fresh"?lastReport.ready&&lastReport.state==="ready"&&lastReport.stage===null&&lastReport.refusal===null:null;
  return {originId,kind,freshness,currentReady,lastReport,ageSeconds:age<0?null:Math.round(age/1000)};
 }));
}
