import {existsSync,readdirSync} from "node:fs";
import type {DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../📋️contract/🟦️.ts";

type SessionRow=Readonly<{state:"live"|"idle"|"gone"}>;
type SessionStore=Readonly<{directory:string}>;
type SessionApi=Readonly<{
 agentSessionRegistryRootV1(environment:Readonly<Record<string,string|undefined>>):string;
 agentSessionWorktreeKeyV1(worktree:string|undefined):string;
 agentSessionStoreV1(root:string,worktree:string):SessionStore;
 listAgentSessionsV1(store:SessionStore,ports:Readonly<{alive?:(pid:number)=>boolean;now?:number}>):readonly SessionRow[];
 collectAgentSessionsV1(store:SessionStore,reclaim:unknown,options:Readonly<{alive?:(pid:number)=>boolean;now?:number;dryRun?:boolean;reason?:string}>):readonly Readonly<{removed:boolean}>[];
}>;
type SessionBuildApi=Readonly<{agentSessionReclaimV1(options?:Readonly<{alive?:(pid:number)=>boolean}>):unknown}>;

/** 🪪️ Opens the session store of this worktree, or reports that the session module is not installed. */
async function open(ports:DoctorPortsV1):Promise<Readonly<{api:SessionApi;store:SessionStore}>|undefined>{
 const api=await ports.optional("sessions-core") as SessionApi|undefined;
 if(!api||typeof api.listAgentSessionsV1!=="function")return undefined;
 return {api,store:api.agentSessionStoreV1(api.agentSessionRegistryRootV1(ports.environment),api.agentSessionWorktreeKeyV1(ports.root))};
}

/** 🧹️ Collects the sessions whose agent is gone through the session owner, which reclaims their resources first. */
async function repair(ports:DoctorPortsV1,dryRun:boolean):Promise<DoctorRepairRunV1>{
 const opened=await open(ports),build=await ports.optional("sessions-build") as SessionBuildApi|undefined;
 if(!opened||!build)return {removed:0,bytes:0,status:"unavailable",reason:"The agent session module is not installed in this checkout."};
 const rows=opened.api.collectAgentSessionsV1(opened.store,build.agentSessionReclaimV1({alive:ports.alive}),{alive:ports.alive,now:ports.now(),dryRun,reason:"doctor"}),removed=dryRun?rows.length:rows.filter(row=>row.removed).length;
 return removed===0?{removed:0,bytes:0,status:"nothing"}:{removed,bytes:0};
}

/** 👥️ The agent sessions registered for this worktree: how many live, idle and gone, and how many records are unreadable. */
export const sessionsProbeV1:DoctorProbeV1=Object.freeze({
 area:"sessions",phase:1,
 repairs:[Object.freeze({id:"agent-sessions",area:"sessions" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repair(ports,dryRun)})],
 async read(ports){
  const opened=await open(ports);
  if(!opened)return {unavailable:"The agent session module is not installed in this checkout"};
  if(!existsSync(opened.store.directory))return {sections:{sessions:{live:0,idle:0,gone:0,damaged:0}}};
  const rows=opened.api.listAgentSessionsV1(opened.store,{alive:ports.alive,now:ports.now()}),records=readdirSync(opened.store.directory).filter(name=>name.endsWith(".json")).length;
  return {sections:{sessions:{live:rows.filter(row=>row.state==="live").length,idle:rows.filter(row=>row.state==="idle").length,gone:rows.filter(row=>row.state==="gone").length,damaged:Math.max(0,records-rows.length)}}};
 },
});
