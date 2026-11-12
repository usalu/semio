import {homedir} from "node:os";
import {adoptCasV1,casPlacesOfWorkspaceV1,type CasAdoptionV1} from "../🧲️adopt/🟦️.ts";
import {casAreasOfWorkspaceV1} from "../📂️areas/🟦️.ts";
import {collectCasV1,formatCasCollectionV1} from "../🧹️collector/🟦️.ts";
import {casPartitionUsedFractionV1,measureCasPartitionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
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
 return casTextV1("adopted",language,{linked:result.linked,objects:result.objects,savedMib:Math.round(result.savedBytes/1048576*10)/10,failed:result.failed,partial:result.partial?casTextV1("partial",language):""});
}

/** ⌨️ `repo cache cas status|adopt|gc [--dry-run] [--json] [--lang=en|de]`: reports the host store and its partition, makes the finished files of the declared places share host objects, or collects it together with the declared areas (closures, graph memo) down to the low watermark of the cap and of the partition; progress and cancellation come from the runner ports. The collection works on a full disk: it deletes before it writes anything. */
export async function runCasCommandV1(args:readonly string[],context:CasCommandContextV1,ports:CasCommandPortsV1):Promise<number>{
 const operation=args[0],flags=args.slice(1),language=(flags.find(flag=>flag.startsWith("--lang="))?.slice(7)??"en") as "en"|"de",json=flags.includes("--json"),dryRun=flags.includes("--dry-run");
 if((operation!=="status"&&operation!=="gc"&&operation!=="adopt")||flags.some(flag=>!["--json","--dry-run"].includes(flag)&&!flag.startsWith("--lang=")))throw Error("Usage / Aufruf: cache cas status|adopt|gc [--dry-run] [--json] [--lang=en|de]");
 if(language!=="en"&&language!=="de")throw Error("Language en or de required / Sprache en oder de erforderlich");
 const home=context.home??casHomeOfV1(context.environment)??homedir(),root=casRootV1(context.environment,process.platform,home),policy=CAS_POLICY_V1,gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100;
 if(operation==="status"){
  const usage=await casUsageV1(root),partition=(context.partition??measureCasPartitionV1)(root),percent=(fraction:number):number=>Math.round(fraction*1000)/10;
  const partitionLine=partition?casTextV1("statusPartition",language,{partition:partition.partition,usedPercent:percent(casPartitionUsedFractionV1(partition)),freeGib:gib(partition.freeBytes),highPercent:percent(policy.partition.highUsedFraction),lowPercent:percent(policy.partition.lowUsedFraction),criticalPercent:percent(policy.partition.criticalUsedFraction)}):undefined;
  context.write(json?JSON.stringify({version:1,root,capBytes:policy.capBytes,...usage,...partition?{partition:{...partition,usedFraction:casPartitionUsedFractionV1(partition)}}:{}},null,2):[casTextV1("status",language,{root,objects:usage.objects,usedGib:gib(usage.bytes),capGib:gib(policy.capBytes)}),...partitionLine?[partitionLine]:[]].join("\n"));
  return 0;
 }
 if(operation==="adopt"){
  const rows:Record<string,CasAdoptionV1>={};let scanned=0;
  for(const place of casPlacesOfWorkspaceV1(context.workspace)){
   const row=await adoptCasV1({root,places:[place],leave:casPlacesOfWorkspaceV1(context.workspace).map(other=>other.directory),signal:ports.signal,dryRun,progress:event=>ports.progress(casTextV1("adopting",language,{entries:scanned+event.entries,savedMib:Math.round(event.savedBytes/1048576*10)/10}))});
   rows[place.id]=row;scanned+=row.scanned;
  }
  const total=Object.values(rows).reduce((sum,row)=>({partial:sum.partial||row.partial,dryRun,scanned:sum.scanned+row.scanned,linked:sum.linked+row.linked,objects:sum.objects+row.objects,savedBytes:sum.savedBytes+row.savedBytes,failed:sum.failed+row.failed,skipped:Object.fromEntries(Object.entries(sum.skipped).map(([key,value])=>[key,value+row.skipped[key as keyof typeof row.skipped]])) as CasAdoptionV1["skipped"]}),{partial:false,dryRun,scanned:0,linked:0,objects:0,savedBytes:0,failed:0,skipped:{young:0,small:0,volatile:0,linked:0,device:0,mode:0,changed:0,clone:0}} as CasAdoptionV1);
  context.write(json?JSON.stringify({version:1,places:rows,total},null,2):[...Object.entries(rows).map(([id,row])=>id+": "+formatCasAdoptionV1(row,language)),formatCasAdoptionV1(total,language)].join("\n"));
  return 0;
 }
 const areas=await casAreasOfWorkspaceV1(context.workspace,home),result=await collectCasV1({root,policy,areas,externalBytes:context.externalBytes,partition:(context.partition??measureCasPartitionV1)(root),signal:ports.signal,dryRun,progress:event=>ports.progress(casTextV1("progress",language,{entries:event.entries,usedGib:gib(event.usedBytes),capGib:gib(policy.capBytes),freedMib:Math.round(event.freedBytes/1048576*10)/10}))});
 context.write(json?JSON.stringify({version:1,...result},null,2):formatCasCollectionV1(result,policy,language));
 return result.status==="locked"?1:0;
}
