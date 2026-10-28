import {freemem,totalmem} from "node:os";
import {resolve} from "node:path";
import {isMainModuleV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {CI_SUITE_POLICY_V1,CI_SUITE_ROOT_V1,ciSuiteTextV1,refuseCiSuiteV1,planCiMustRunV1,planCiShardV1,readCiSuiteTablesV1,renderCiSuiteReportV1,runCiSuitesV1,type CiPlanV1,type CiSampleV1,type CiSuiteHostV1,type CiSuiteLanguageV1,type CiSuitePolicyV1,type CiSuiteRunPortsV1,type CiSuiteTableIdV1,type CiViolationV1} from "./🟦️.ts";

const flags:Readonly<Record<string,readonly string[]>>={plan:["shard","must-run","host","lang"],run:["shard","must-run","host","tool","out","lang"]};
const HOSTS=["darwin","linux","win32"] as const;

function parse(command:string,args:readonly string[]):Map<string,string>{
 const allowed=flags[command]!,values=new Map<string,string>();
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,name=argument.slice(2);
  if(!argument.startsWith("--")||!allowed.includes(name))throw refuseCiSuiteV1("usage","refusal-option",{option:argument});
  if(name==="must-run"){values.set(name,"true");continue;}
  const value=args[++index];
  if(value===undefined||value.startsWith("--"))throw refuseCiSuiteV1("usage","refusal-option",{option:argument});
  values.set(name,value);
 }
 return values;
}

/** 🧭️ Reads `<table>:<index>/<count>` of the policy. */
export function parseCiShardV1(value:string,policy:CiSuitePolicyV1=CI_SUITE_POLICY_V1):Readonly<{table:CiSuiteTableIdV1;index:number;count:number}>{
 const match=/^(process|caching|build):([1-9][0-9]?)\/([1-9][0-9]?)$/u.exec(value),row=match===null?undefined:policy.tables.find(item=>item.id===match[1]);
 if(match===null||row===undefined||Number(match[3])!==row.shards||Number(match[2])>row.shards)throw refuseCiSuiteV1("usage","refusal-shard",{value});
 return {table:match[1] as CiSuiteTableIdV1,index:Number(match[2]),count:Number(match[3])};
}

function select(values:Map<string,string>,host:CiSuiteHostV1):Readonly<{plan:CiPlanV1;enforce:boolean;undeclared:readonly CiViolationV1[]}>{
 const tables=readCiSuiteTablesV1();
 if(values.has("must-run")===values.has("shard"))throw refuseCiSuiteV1("usage","refusal-selection");
 if(values.has("must-run")){
  const wanted=[...CI_SUITE_POLICY_V1.mustRun.all,...CI_SUITE_POLICY_V1.mustRun[host]],undeclared:CiViolationV1[]=wanted.filter(row=>!tables.find(table=>table.id===row.table)!.table.suites.some(suite=>suite.id===row.suite)).map(row=>({table:row.table,suite:row.suite,reason:"not-declared"}));
  return {plan:planCiMustRunV1(tables,CI_SUITE_POLICY_V1,host),enforce:true,undeclared};
 }
 const shard=parseCiShardV1(values.get("shard")!);
 return {plan:planCiShardV1(tables,host,shard.table,shard.index,shard.count),enforce:false,undeclared:[]};
}

/** 🔌️ The ports of a real sweep: `repo run` of the official route through the owned command funnel, the host counters of the capacity gate (the free memory of the operating system when they fail), the real clock and the console, with GitHub log groups when running there. */
export function defaultCiSuitePortsV1(environment:Readonly<Record<string,string|undefined>>=process.env):CiSuiteRunPortsV1{
 const grouped=environment.GITHUB_ACTIONS==="true";
 let sampler:Promise<()=>Promise<CiSampleV1>>|undefined;
 const load=async()=>{
  const {currentHostProbeV1,observeTreeMegabytesV1,readHostCapacityPolicyV1,readHostSampleV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts"),{default:policyDocument}=await import("../../🔣️policy.json",{with:{type:"json"}}),policy=readHostCapacityPolicyV1((policyDocument as unknown as {hostCapacity:unknown}).hostCapacity),probe=currentHostProbeV1();
  return async():Promise<CiSampleV1>=>{
   const {host}=await readHostSampleV1(policy,probe),tree=await observeTreeMegabytesV1(process.pid).catch(()=>null);
   return {atMs:Date.now(),availableMegabytes:host.availableMegabytes,totalMegabytes:host.totalMegabytes,swapMegabytes:host.swapUsedMegabytes,treeMegabytes:tree===null?null:Math.round(tree),pressure:host.pressure};
  };
 };
 const system=():CiSampleV1=>({atMs:Date.now(),availableMegabytes:Math.floor(freemem()/1048576),totalMegabytes:Math.max(1,Math.floor(totalmem()/1048576)),swapMegabytes:null,treeMegabytes:null,pressure:null});
 return {
  start:async(row,tool,timeoutMs,onLine,signal)=>{
   const {OwnedCommandFailure,runOwnedCommand}=await import("../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
   try{
    await runOwnedCommand("repo",[`--tool=${tool}`,"run",`${row.project}:${row.target}`,"--skip-cache","--output=stream"],CI_SUITE_ROOT_V1,`${row.table}/${row.suite}`,timeoutMs,{onLine:line=>{onLine(line);},signal});
    return {exitCode:0,timedOut:false};
   }catch(error){
    if(error instanceof OwnedCommandFailure)return {exitCode:error.outcome.status,timedOut:false};
    return {exitCode:null,timedOut:/stopped: timeout/u.test(String((error as Error).message))};
   }
  },
  sample:async()=>{
   sampler??=load().catch(()=>async()=>system());
   try{return await (await sampler)();}catch{return system();}
  },
  now:()=>Date.now(),
  write:line=>{process.stdout.write(line+"\n");},
  group:title=>{if(grouped)process.stdout.write(title===null?"::endgroup::\n":`::group::${title}\n`);else if(title!==null)process.stdout.write(`== ${title}\n`);}
 };
}

/** 🚦️ Runs one command of the sweep; the exit code is 1 for a failed verdict and 2 for a usage problem. */
export async function runCiSuitesCommandV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>,write:(text:string)=>void,ports:CiSuiteRunPortsV1=defaultCiSuitePortsV1(environment),signal?:AbortSignal):Promise<number>{
 const command=argv[0],flag=argv.indexOf("--lang");
 let language:CiSuiteLanguageV1;
 try{language=resolveLanguageV1(CATALOG_LANGUAGES_V1,flag<0?[]:["--lang",argv[flag+1]??""],environment).language as CiSuiteLanguageV1;}catch(error){write(`${(error as Error).message}\n`);return 2;}
 if(command===undefined||!(command in flags)){write(ciSuiteTextV1(language,"usage")+"\n");return command===undefined||command==="--help"?0:2;}
 try{
  const values=parse(command,argv.slice(1)),host=(values.get("host")??process.platform) as CiSuiteHostV1;
  if(!HOSTS.includes(host))throw refuseCiSuiteV1("usage","refusal-option",{option:`--host ${host}`});
  const {plan,enforce,undeclared}=select(values,host);
  if(command==="plan"){write(JSON.stringify(plan,null,1)+"\n");return 0;}
  const tool=values.get("tool"),out=values.get("out");
  if(tool===undefined||out===undefined)throw refuseCiSuiteV1("usage","refusal-option",{option:tool===undefined?"--tool":"--out"});
  const report=await runCiSuitesV1({plan,policy:CI_SUITE_POLICY_V1,tool,out:resolve(out),language,enforce,ports,undeclared,...signal?{signal}:{}});
  write(renderCiSuiteReportV1(report,language,[...CI_SUITE_POLICY_V1.mustRun.all,...CI_SUITE_POLICY_V1.mustRun[host]]));
  if(!report.ok&&environment.GITHUB_ACTIONS==="true")write(`::error::${plan.label} failed\n`);
  return report.ok?0:1;
 }catch(error){write(`${isRepoErrorV1(error)?renderRepoErrorV1(error,language):(error as Error).message}\n`);return 2;}
}

if(isMainModuleV1(import.meta.url)){
 const controller=new AbortController(),stop=()=>controller.abort();
 process.once("SIGINT",stop);process.once("SIGTERM",stop);
 process.exit(await runCiSuitesCommandV1(process.argv.slice(2),process.env,text=>process.stdout.write(text),defaultCiSuitePortsV1(process.env),controller.signal));
}
