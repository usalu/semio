import {spawn} from "node:child_process";
import {join} from "node:path";
import {incrementalChildEnvironmentV1} from "../../../🏃️runner/🟦️.ts";
import {LATENCY_POLICY_V1,latencyArgumentsV1,latencyBudgetsV1,latencyObserveV1,latencyParseProfileV1,latencyResultV1,type LatencyBudgetsV1,type LatencyCommandV1,type LatencyReportV1,type LatencyResultV1,type LatencySampleV1} from "../🟦️.ts";

const both=(english:string,german:string)=>`${english}\n${german}`;

/** 🚀️ One `repo` run: the program, its arguments, the place and the environment. */
export type LatencyCallV1=Readonly<{command:string;args:readonly string[];cwd:string;env:Record<string,string>}>;
/** 🏁️ What one run left: the exit code (null when it was stopped), the wall clock and the error output the profile is read from. */
export type LatencyRunV1=Readonly<{exit:number|null;wallMs:number;stderr:string}>;
/** 📮️ What the latency run needs from the host: starting a run, the clock, progress, the cancellation check, the speed factor and the host numbers. */
export type LatencyPortsV1=Readonly<{
 spawn(call:LatencyCallV1,signal:AbortSignal):Promise<LatencyRunV1>;
 progress(stage:string):void|Promise<void>;
 check():void;
 factor:number;
 clock():string;
 host():Readonly<{cpus:number;memoryMb:number;loadPerCore:number|null}>;
 signal:AbortSignal;
}>;
export type LatencyRequestV1=Readonly<{root:string;executable:string;script:string;commands:readonly string[]|"all";runs:number;environment:Readonly<Record<string,string|undefined>>}>;

/** 🔌️ The command line of one measured run: the build door of the checkout, the neutral `repo` and the arguments, with the phase profile switched on in JSON and every inherited build or invocation variable dropped except the few the measured commands declare they need (the fleet, the remote cache, the host capacity declaration, the build budget). */
export function latencyCallV1(request:Pick<LatencyRequestV1,"root"|"executable"|"script"|"environment">,args:readonly string[]):LatencyCallV1{
 const env=incrementalChildEnvironmentV1(request.environment,{[LATENCY_POLICY_V1.profileVariable]:LATENCY_POLICY_V1.profileValue}),declared=LATENCY_POLICY_V1.environment;
 for(const [key,value] of Object.entries(request.environment))if(value!==undefined&&(declared.keep.includes(key)||declared.prefixes.some(prefix=>key.startsWith(prefix))))env[key]=value;
 return Object.freeze({command:request.executable,args:[join(request.root,request.script),"repo",...args],cwd:request.root,env});
}

/** ✂️ Picks the commands of the table by id; an unknown id is refused in both languages. */
export function latencySelectV1(set:LatencyBudgetsV1,ids:readonly string[]|"all"):readonly LatencyCommandV1[]{
 if(ids==="all")return set.commands;
 const known=new Set(set.commands.map(row=>row.id)),unknown=ids.filter(id=>!known.has(id));
 if(unknown.length)throw Error(both(`Unknown command: ${unknown.join(", ")} (${[...known].join("|")}|all)`,`Unbekannter Befehl: ${unknown.join(", ")} (${[...known].join("|")}|all)`));
 return set.commands.filter(row=>ids.includes(row.id));
}

/** 🏃️ Runs every selected command `runs` times on the fixed small targets with the profile on, reads the phase profile and round trips out of each run and judges the median against the budget; a run that fails or is cancelled is recorded, not hidden, and a cancellation stops after the run in flight. */
export async function runLatencyV1(request:LatencyRequestV1,ports:LatencyPortsV1,set:LatencyBudgetsV1=latencyBudgetsV1):Promise<LatencyReportV1>{
 const started=ports.clock(),results:LatencyResultV1[]=[],commands=latencySelectV1(set,request.commands),runs=Math.min(request.runs,LATENCY_POLICY_V1.maximumRuns);
 for(const command of commands){
  const samples:LatencySampleV1[]=[],call=latencyCallV1(request,latencyArgumentsV1(command,set.targets));
  let reason:string|undefined;
  for(let run=1;run<=runs;run++){
   ports.check();await ports.progress(`${command.id}:${run}/${runs}`);
   let done:LatencyRunV1;
   try{done=await ports.spawn(call,ports.signal);}catch(error){reason=(error as Error).message;break;}
   samples.push(Object.freeze({run,exit:done.exit,observation:latencyObserveV1(latencyParseProfileV1(done.stderr),done.wallMs)}));
   if(done.exit!==0){const tail=done.stderr.split("\n").map(line=>line.trim()).filter(line=>line&&!line.startsWith("{")&&!line.startsWith("[script]")).slice(-2).join(" | ").slice(0,300);reason=`exit ${done.exit}`+(tail?`: ${tail}`:"");break;}
  }
  results.push(latencyResultV1(command,samples,ports.factor,reason===undefined?{}:{reason}));
 }
 return Object.freeze({version:1,started,host:Object.freeze({...ports.host(),speedFactor:ports.factor}),results:Object.freeze(results)});
}

/** 🧵️ The real run of one call: the child is stopped with the signal and with the declared timeout, its error output is kept up to the declared bound and its wall clock is measured around the run. */
export function latencySpawnV1(call:LatencyCallV1,signal:AbortSignal):Promise<LatencyRunV1>{
 return new Promise((resolve,reject)=>{
  const began=performance.now(),child=spawn(call.command,[...call.args],{cwd:call.cwd,env:call.env,stdio:["ignore","ignore","pipe"],windowsHide:true}),chunks:Buffer[]=[];let size=0,stopped=false;
  const stop=()=>{stopped=true;try{child.kill("SIGTERM");}catch{}},timer=setTimeout(stop,LATENCY_POLICY_V1.timeoutMilliseconds);
  signal.addEventListener("abort",stop,{once:true});
  child.stderr.on("data",(chunk:Buffer)=>{if(size<LATENCY_POLICY_V1.maximumOutputBytes){chunks.push(chunk);size+=chunk.length;}});
  child.on("error",error=>{clearTimeout(timer);reject(error);});
  child.on("close",code=>{clearTimeout(timer);signal.removeEventListener("abort",stop);resolve(Object.freeze({exit:stopped?null:code,wallMs:performance.now()-began,stderr:Buffer.concat(chunks).toString("utf8")}));});
 });
}
