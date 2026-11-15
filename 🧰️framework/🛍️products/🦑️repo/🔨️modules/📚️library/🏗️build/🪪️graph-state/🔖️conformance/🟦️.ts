import{createHash}from"node:crypto";
import type{GraphStateV1}from"../🟦️.ts";
import{graphPlatformV1}from"../🟦️.ts";
import{fetchGraphEntryV1,publishGraphEntryV1,type GraphFleetClientV1}from"../🛰️exchange/🟦️.ts";

const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🔖️ The exchange state a conformance record lives under: one entry per inferrer, closure key and platform, so a machine of another platform never takes a record whose corpus it could answer differently. */
export function conformanceStateV1(id:string,key:string,platform:string=graphPlatformV1()):GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("inference-conformance"),implementation:key,platform,extra:Object.freeze({conformance:id})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["inference-conformance",id,key,platform]))}) as GraphStateV1;
}

/** 🛰️ The port that shares conformance records through the fleet cache: `get` answers the record of a closure key or undefined (absent, damaged, offline), `put` publishes one. Every call is bounded by the client's own timeouts and never throws; the caller checks what it gets against its own key before it trusts it. */
export function conformanceFleetPortV1(input:Readonly<{client:GraphFleetClientV1;owner:string;platform?:string}>){
 const platform=input.platform??graphPlatformV1();
 return Object.freeze({
  async get(id:string,key:string):Promise<unknown>{
   try{
    const found=await fetchGraphEntryV1(input.client,conformanceStateV1(id,key,platform));
    if(found===undefined||found==="fault")return undefined;
    const document=found.documents.find(row=>row.name==="record");
    return document===undefined?undefined:JSON.parse(Buffer.from(document.bytes).toString("utf8"));
   }catch{return undefined;}
  },
  async put(id:string,key:string,record:unknown):Promise<boolean>{
   try{return await publishGraphEntryV1({client:input.client,state:conformanceStateV1(id,key,platform),owner:input.owner,documents:[{name:"record",bytes:Buffer.from(JSON.stringify(record))}]});}catch{return false;}
  },
 });
}
