import {homedir} from "node:os";
import {adoptCasV1,casPlacesOfWorkspaceV1,type CasAdoptionV1} from "../🧲️adopt/🟦️.ts";
import {casAreasOfWorkspaceV1} from "../📂️areas/🟦️.ts";
import {collectCasV1,formatCasCollectionV1,readCasUsageHintV1} from "../🧹️collector/🟦️.ts";
import {readCasTransientsV1} from "../📌️pins/🟦️.ts";
import {casHoldersUsageV1} from "../🫂️holders/🟦️.ts";
import {casEffectiveCapBytesV1,casPartitionUsedFractionV1,measureCasPartitionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {CAS_POLICY_V1,casTextV1} from "../🧬️schema/🟦️.ts";
import {casRootV1} from "../🗃️store/🟦️.ts";
import {casUsageV1} from "../🗃️store/📏️usage/🟦️.ts";

/** 🔌️ What `cache cas` needs from its host: workspace, environment, output and the runner's cancellation and progress. */
export type CasCommandContextV1=Readonly<{workspace:string;environment:Readonly<Record<string,string|undefined>>;write(line:string):void;home?:string;externalBytes?:number;partition?(root:string):CasPartitionV1|undefined}>;
/** 🔌️ The cancellation and progress ports of the runner this command shares with `cache gc`. */
export type CasCommandPortsV1=Readonly<{signal:AbortSignal;progress(stage:string):Promise<void>}>;

/** 🏠️ The user home an environment declares (`HOME`, else `USERPROFILE`); none means the environment is not a real user session and the host store is left alone. */
export const casHomeOfV1=(environment:Readonly<Record<string,string|undefined>>):string|undefined=>environment.HOME||environment.USERPROFILE||undefined;

/** 📣️ The one-line report of an adoption in the requested language. */
export function formatCasAdoptionV1(result:CasAdoptionV1,language:"en"|"de"):string{
 return casTextV1(result.dryRun?"previewAdoption":"adopted",language,{linked:result.linked,objects:result.objects,savedMib:Math.round(result.savedBytes/1048576*10)/10,failed:result.failed,partial:result.partial?casTextV1("partial",language):""});
}

/** ⌨️ `repo cache cas status|adopt|gc [--dry-run] [--unreferenced] [--json] [--lang=en|de]` reports the host store and partition, shares eligible files, or collects host objects and graph memo files toward the low watermarks; `gc --unreferenced` removes only the objects no holder, pin, lease or hard link references and that are older than the in-flight grace, never a held entry. Source inventories count toward the cap. Progress and cancellation come from the runner ports. The collection works on a full disk by deleting before it writes. */
export async function runCasCommandV1(args:readonly string[],context:CasCommandContextV1,ports:CasCommandPortsV1):Promise<number>{
 const operation=args[0],flags=args.slice(1),language=(flags.find(flag=>flag.startsWith("--lang="))?.slice(7)??"en") as "en"|"de",json=flags.includes("--json"),dryRun=flags.includes("--dry-run"),unreferenced=flags.includes("--unreferenced");
 if((operation!=="status"&&operation!=="gc"&&operation!=="adopt")||flags.some(flag=>!["--json","--dry-run","--unreferenced"].includes(flag)&&!flag.startsWith("--lang=")))throw Error("Usage / Aufruf: cache cas status|adopt|gc [--dry-run] [--unreferenced] [--json] [--lang=en|de]");
 if(unreferenced&&operation!=="gc")throw Error("--unreferenced belongs to gc / --unreferenced gehört zu gc");
 if(language!=="en"&&language!=="de")throw Error("Language en or de required / Sprache en oder de erforderlich");
 const home=context.home??casHomeOfV1(context.environment)??homedir(),root=casRootV1(context.environment,process.platform,home),policy=CAS_POLICY_V1,gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100;
 if(operation==="status"){
  const usage=await casUsageV1(root),partition=(context.partition??measureCasPartitionV1)(root),percent=(fraction:number):number=>Math.round(fraction*1000)/10;
  const partitionLine=partition?casTextV1("statusPartition",language,{partition:partition.partition,usedPercent:percent(casPartitionUsedFractionV1(partition)),freeGib:gib(partition.freeBytes),highPercent:percent(policy.partition.highUsedFraction),lowPercent:percent(policy.partition.lowUsedFraction),criticalPercent:percent(policy.partition.criticalUsedFraction)}):undefined;
  const effective=partition?casEffectiveCapBytesV1(policy,partition,usage.bytes):policy.capBytes,leases=[...(await readCasTransientsV1(root)).values()],at=Date.now(),transient={leased:leases.filter(until=>until>at).length,expired:leases.filter(until=>until<=at).length};
  const holders=await casHoldersUsageV1(root,policy,at,ports.signal),unreferencedBytes=Math.max(0,usage.bytes-holders.referencedBytes);
  const holderLines=[casTextV1("statusHolders",language,{holders:holders.rows.length,referenced:gib(holders.referencedBytes),unreferenced:gib(unreferencedBytes)}),...holders.rows.map(row=>casTextV1("statusHolder",language,{kind:row.kind,label:row.label,entries:row.entries,digests:row.digests,gib:gib(row.bytes),note:""}))];
  const hint=await readCasUsageHintV1(root),collectionLine=hint===undefined?casTextV1("statusNoCollection",language):casTextV1("statusCollection",language,{age:Math.max(0,Math.round((at-hint.atMs)/60000)),gib:gib(hint.bytes)});
  const capLine=casTextV1("statusCap",language,{role:partition?.role??policy.sizing.defaultRole,effectiveGib:gib(effective),capGib:gib(policy.capBytes),highGib:gib(effective*policy.highWatermark),lowGib:gib(effective*policy.lowWatermark),incrementGib:gib(policy.incrementBytes),transient:transient.leased+transient.expired,leaseMinutes:Math.round(policy.transientLeaseMs/60000)});
  context.write(json?JSON.stringify({version:1,root,role:partition?.role??policy.sizing.defaultRole,capBytes:policy.capBytes,effectiveCapBytes:effective,highWatermarkBytes:Math.floor(effective*policy.highWatermark),lowWatermarkBytes:Math.floor(effective*policy.lowWatermark),incrementBytes:policy.incrementBytes,transientLeaseMs:policy.transientLeaseMs,transient,lastCollection:hint===undefined?null:{atMs:hint.atMs,bytes:hint.bytes},holders:holders.rows,referencedBytes:holders.referencedBytes,unreferencedBytes,...usage,...partition?{partition:{...partition,usedFraction:casPartitionUsedFractionV1(partition)}}:{}},null,2):[casTextV1("status",language,{root,objects:usage.objects,usedGib:gib(usage.bytes),capGib:gib(policy.capBytes)}),capLine,...holderLines,collectionLine,...partitionLine?[partitionLine]:[]].join("\n"));
  return 0;
 }
 if(operation==="adopt"){
  const rows:Record<string,CasAdoptionV1>={};let scanned=0;
  for(const place of casPlacesOfWorkspaceV1(context.workspace)){
   const row=await adoptCasV1({root,places:[place],leave:casPlacesOfWorkspaceV1(context.workspace).map(other=>other.directory),signal:ports.signal,dryRun,progress:event=>ports.progress(casTextV1(dryRun?"previewAdopting":"adopting",language,{entries:scanned+event.entries,savedMib:Math.round(event.savedBytes/1048576*10)/10}))});
   rows[place.id]=row;scanned+=row.scanned;
  }
  const total=Object.values(rows).reduce((sum,row)=>({partial:sum.partial||row.partial,dryRun,scanned:sum.scanned+row.scanned,linked:sum.linked+row.linked,objects:sum.objects+row.objects,savedBytes:sum.savedBytes+row.savedBytes,failed:sum.failed+row.failed,skipped:Object.fromEntries(Object.entries(sum.skipped).map(([key,value])=>[key,value+row.skipped[key as keyof typeof row.skipped]])) as CasAdoptionV1["skipped"]}),{partial:false,dryRun,scanned:0,linked:0,objects:0,savedBytes:0,failed:0,skipped:{young:0,small:0,volatile:0,linked:0,device:0,mode:0,changed:0,clone:0}} as CasAdoptionV1);
  context.write(json?JSON.stringify({version:1,places:rows,total},null,2):[...Object.entries(rows).map(([id,row])=>id+": "+formatCasAdoptionV1(row,language)),formatCasAdoptionV1(total,language)].join("\n"));
  return 0;
 }
 const areas=unreferenced?[]:await casAreasOfWorkspaceV1(context.workspace,home),result=await collectCasV1({root,policy,areas,role:"operator",waitMs:policy.lockWaitMs,unreferencedOnly:unreferenced,externalBytes:unreferenced?undefined:context.externalBytes,partition:(context.partition??measureCasPartitionV1)(root),signal:ports.signal,dryRun,progress:event=>ports.progress(casTextV1(dryRun?"previewProgress":"progress",language,{entries:event.entries,usedGib:gib(event.usedBytes),capGib:gib(policy.capBytes),freedMib:Math.round(event.freedBytes/1048576*10)/10}))});
 context.write(json?JSON.stringify({version:1,...result},null,2):formatCasCollectionV1(result,policy,language));
 return result.status==="locked"?1:0;
}
