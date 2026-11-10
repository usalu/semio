import {spawn,spawnSync} from "node:child_process";
import {cpus,freemem,loadavg,platform,totalmem} from "node:os";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {acceptanceLineV1,acceptanceMutateV1,acceptanceRestoreV1} from "../../🛂️acceptance/🟦️.ts";
import {planningClosuresV1,planningObserveV1,planningOomKillsV1,planningScenariosV1,type PlanningLineV1,type PlanningReportV1,type PlanningResultV1,type PlanningSampleV1,type PlanningScenarioV1} from "../🟦️.ts";

export type PlanningHostV1=Readonly<{availableMb:number|null;swapUsedMb:number|null;repoRssMb:number|null}>;
const repoProcess=/\brepo\b|runtimes\/bun|bazel|\bnx\b|cargo|rustc/u;

/** 🌡️ Reads the resident memory of every repo-related process (summed), the memory still available and the swap in use; each value is null where the host cannot tell. */
export function planningHostV1():PlanningHostV1{
 let availableMb:number|null=Math.round(freemem()/1048576),swapUsedMb:number|null=null,repoRssMb:number|null=null;
 if(platform()==="linux")try{
  const info=new Map(readFileSync("/proc/meminfo","utf8").split("\n").map(line=>line.match(/^(\w+):\s+(\d+)/u)).filter(Boolean).map(hit=>[hit![1]!,Number(hit![2])] as const));
  availableMb=Math.round((info.get("MemAvailable")??0)/1024);swapUsedMb=Math.round(((info.get("SwapTotal")??0)-(info.get("SwapFree")??0))/1024);
 }catch{}
 if(platform()!=="win32"){
  const ps=spawnSync("ps",["-axo","rss=,args="],{encoding:"utf8",maxBuffer:1<<26});
  if(ps.status===0)repoRssMb=Math.round(ps.stdout.split("\n").filter(line=>repoProcess.test(line)).reduce((sum,line)=>sum+(Number(line.trim().split(/\s+/u)[0])||0),0)/1024);
 }
 return {availableMb,swapUsedMb,repoRssMb};
}

/** 💾️ The megabytes a folder holds on disk, null when it does not exist or the host has no `du`. */
export function planningStoreMbV1(root:string,folder:string):number|null{
 const path=resolve(root,folder);
 if(!existsSync(path))return null;
 const du=spawnSync("du",["-sk",path],{encoding:"utf8"});
 return du.status===0?Math.round(Number(du.stdout.split(/\s/u)[0])/1024):null;
}

export const planningOomSinceV1=(sinceIso:string)=>{
 if(platform()!=="linux")return null;
 const out=spawnSync("journalctl",["-k","--no-pager","--since",sinceIso],{encoding:"utf8"});
 return out.status===0?planningOomKillsV1(out.stdout):null;
};

/** 🪫️ Watches the whole host while a plan runs: the peak resident memory of all repo-related processes, the least available memory, the peak swap in use, the OOM kills the kernel journal reports since the start and the disk use of the store homes; every value is left out where the host cannot tell. */
export function planningResourcesV1(root:string,stores:readonly string[]=planningScenariosV1.stores,intervalMs=500):Readonly<{end():Record<string,number>}>{
 const since=new Date().toISOString().slice(0,19).replace("T"," ");
 let rss:number|null=null,swap:number|null=null,available:number|null=null;
 const read=()=>{const host=planningHostV1();if(host.repoRssMb!==null)rss=Math.max(rss??0,host.repoRssMb);if(host.swapUsedMb!==null)swap=Math.max(swap??0,host.swapUsedMb);if(host.availableMb!==null)available=Math.min(available??Infinity,host.availableMb);};
 read();
 const timer=setInterval(read,intervalMs);
 return Object.freeze({end(){
  clearInterval(timer);read();
  const mb=1048576,out:Record<string,number>={},oom=planningOomSinceV1(since),store=stores.map(folder=>planningStoreMbV1(root,folder)).reduce<number|null>((sum,value)=>value===null?sum:(sum??0)+value,null);
  if(rss!==null)out.peakRssBytes=rss*mb;
  if(swap!==null)out.swapBytes=swap*mb;
  if(available!==null)out.availableBytes=available*mb;
  if(oom!==null)out.oomKills=oom;
  if(store!==null)out.storeBytes=store*mb;
  return out;
 }});
}

type PlanRunV1=Readonly<{exit:number|null;wallMs:number;lines:readonly PlanningLineV1[];peakRssMb:number|null;peakSwapMb:number|null;minAvailableMb:number|null}>;

const plan=async(root:string,target:string,intervalMs:number,signal:AbortSignal):Promise<PlanRunV1>=>{
 const lines:PlanningLineV1[]=[],begin=Date.now();
 let origin=begin,peakRss:number|null=null,peakSwap:number|null=null,minAvail:number|null=null;
 const child=spawn("repo",["run",target,"--dry-run","--output=static","--lang=en"],{cwd:root,stdio:["ignore","pipe","pipe"]});
 const push=(text:string)=>{if(/^\[slot\] acquired/u.test(text))origin=Date.now();lines.push({atMs:Date.now()-origin,text});};
 {let tail="";for(const stream of [child.stdout!,child.stderr!])stream.on("data",(chunk:Buffer)=>{const parts=(tail+chunk.toString("utf8")).split("\n");tail=parts.pop()!;parts.forEach(push);});}
 const timer=setInterval(()=>{
  const host=planningHostV1();
  if(host.repoRssMb!==null)peakRss=Math.max(peakRss??0,host.repoRssMb);
  if(host.swapUsedMb!==null)peakSwap=Math.max(peakSwap??0,host.swapUsedMb);
  if(host.availableMb!==null)minAvail=Math.min(minAvail??Infinity,host.availableMb);
 },intervalMs);
 signal.addEventListener("abort",()=>child.kill("SIGTERM"),{once:true});
 const exit=await new Promise<number|null>(done=>child.on("close",code=>done(code)));
 clearInterval(timer);
 return {exit,wallMs:Date.now()-origin,lines,peakRssMb:peakRss,peakSwapMb:peakSwap,minAvailableMb:minAvail};
};

const sample=(run:number,result:PlanRunV1,scenario:PlanningScenarioV1,oom:number|null,storeMb:number|null):PlanningSampleV1=>{
 const observation=planningObserveV1(result.lines,result.wallMs);
 return {run,exit:result.exit,wallMs:result.wallMs,phaseMs:observation.phaseMs,closures:planningClosuresV1(observation),peakRssMb:result.peakRssMb,swapMb:result.peakSwapMb,oomKills:oom,minAvailableMb:result.minAvailableMb,storeMb};
};

/** 🏁️ Runs the selected planning scenarios against the checkout  and returns the report; edits are appended as one comment line and restored byte for byte, every run is cancellable through the signal. */
export async function planningRunV1(options:Readonly<{root:string;ids:readonly string[];outDir:string;signal:AbortSignal;note(line:string):void}>):Promise<PlanningReportV1>{
 const set=planningScenariosV1,started=new Date().toISOString(),chosen=set.scenarios.filter(row=>options.ids.includes("all")||options.ids.includes(row.id)),results:PlanningResultV1[]=[];
 for(const scenario of chosen){
  const samples:PlanningSampleV1[]=[],intervalMs=scenario.intervalMs??500;
  const oomStart=new Date().toISOString().slice(0,19).replace("T"," ");
  options.note(`[bench] ${scenario.id}`);
  if(scenario.kind==="store"){
   const mb=set.stores.map(folder=>planningStoreMbV1(options.root,folder)).reduce<number|null>((sum,value)=>value===null?sum:(sum??0)+value,null);
   samples.push({run:1,exit:0,wallMs:0,phaseMs:{facts:0,verification:0,inference:0,planning:0},closures:{verified:null,reused:null,changed:null},peakRssMb:null,swapMb:null,oomKills:null,minAvailableMb:null,storeMb:mb});
  }else if(scenario.kind==="host"){
   const host=planningHostV1();
   samples.push({run:1,exit:0,wallMs:0,phaseMs:{facts:0,verification:0,inference:0,planning:0},closures:{verified:null,reused:null,changed:null},peakRssMb:host.repoRssMb,swapMb:host.swapUsedMb,oomKills:planningOomSinceV1(oomStart),minAvailableMb:host.availableMb,storeMb:null});
  }else{
   const file=scenario.edit?resolve(options.root,scenario.edit.file):null,original=file&&existsSync(file)?readFileSync(file):null;
   const total=(scenario.primes??0)+scenario.runs;
   try{
    for(let index=0;index<total&&!options.signal.aborted;index++){
     const measured=index>=(scenario.primes??0);
     let edited:ReturnType<typeof acceptanceMutateV1>|null=null,line="";
     if(measured&&file&&original){line=acceptanceLineV1(scenario.edit!.comment,Math.random().toString(36).slice(2,10));edited=acceptanceMutateV1(original,line);writeFileSync(file,edited.bytes);}
     try{
      const runs=await Promise.all(scenario.projects.slice(0,scenario.kind==="concurrent"?scenario.projects.length:1).map((target,at)=>plan(options.root,target,intervalMs,options.signal)));
      const worst=runs.reduce((a,b)=>b.wallMs>a.wallMs?b:a);
      if(measured)samples.push(sample(samples.length+1,{...worst,peakRssMb:Math.max(...runs.map(x=>x.peakRssMb??0)),peakSwapMb:Math.max(...runs.map(x=>x.peakSwapMb??0)),exit:runs.every(x=>x.exit===0)?0:runs.find(x=>x.exit!==0)!.exit},scenario,planningOomSinceV1(oomStart),null));
     }finally{
      if(edited&&file&&original){const back=acceptanceRestoreV1(original,edited,line,readFileSync(file));if(back.bytes)writeFileSync(file,back.bytes);}
     }
    }
   }finally{if(file&&original&&!readFileSync(file).equals(original)&&!scenario.edit)writeFileSync(file,original);}
  }
  results.push({scenario:scenario.id,status:options.signal.aborted?"failed":samples.some(x=>x.exit!==0)?"failed":"ran",...(options.signal.aborted?{reason:"aborted"}:{}),samples});
 }
 const now=planningHostV1(),host={cpus:cpus().length,memoryMb:Math.round(totalmem()/1048576),swapMb:now.swapUsedMb,loadPerCore:Math.round(loadavg()[0]!/cpus().length*100)/100};
 const report:PlanningReportV1={version:1,started,host,results};
 mkdirSync(options.outDir,{recursive:true});
 writeFileSync(join(options.outDir,`report-${started.replace(/[:.]/gu,"-")}.json`),JSON.stringify(report,null,1));
 return report;
}

if(import.meta.main){
 const args=process.argv.slice(2),flag=(name:string)=>args.find(a=>a.startsWith(`--${name}=`))?.slice(name.length+3)??null;
 const root=resolve(flag("root")??process.cwd()),out=flag("out");
 if(!out){console.error("usage: bun run runner.ts --out=<dir> [--scenario=<ids|all>] [--root=<checkout>]");process.exit(2);}
 const controller=new AbortController();process.on("SIGINT",()=>controller.abort());
 const report=await planningRunV1({root,ids:(flag("scenario")??"all").split(","),outDir:resolve(out),signal:controller.signal,note:line=>console.error(line)});
 console.log(JSON.stringify(report));
}
