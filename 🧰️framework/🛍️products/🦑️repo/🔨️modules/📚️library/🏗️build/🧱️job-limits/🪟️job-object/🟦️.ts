import {spawn,spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,renameSync} from "node:fs";
import {homedir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {JOB_LIMIT_RULES_V1,type JobBreachV1,type JobLimitPolicyV1,type JobLimiterV1,type LimitHandleV1} from "../🟦️.ts";

const SOURCE=new URL("./🦀️rust/🦀️.rs",import.meta.url);

/** 🔌️ How the limiter starts the helper that owns the job object; a test replaces it with a script that speaks the same frames. */
export type JobObjectPortsV1=Readonly<{start:(args:readonly string[])=>Readonly<{lines:AsyncIterable<string>;close:()=>void;done:Promise<void>}>}>;

/** 🔨️ Compiles the std-only helper once per source digest into the user cache and answers its path, undefined where no Rust compiler exists. */
export function ensureJobHelperV1(cache:string=join(process.env["LOCALAPPDATA"]??join(homedir(),".cache"),"semio","job-limit"),platform:NodeJS.Platform=process.platform):string|undefined{
 const digest=createHash("sha256").update(readFileSync(SOURCE)).digest("hex").slice(0,16),output=join(cache,`semio-job-limit-${digest}${platform==="win32"?".exe":""}`);
 if(existsSync(output))return output;
 mkdirSync(cache,{recursive:true});
 const staged=`${output}.${process.pid}.tmp`,done=spawnSync("rustc",["--edition","2021","--crate-name","semio_job_limit","-O","-o",staged,fileURLToPath(SOURCE)],{stdio:"ignore",windowsHide:true});
 if(done.status!==0)return undefined;
 renameSync(staged,output);
 return output;
}

/** 🚀️ Starts the real helper process. */
export function jobObjectProcessPortsV1(helper:string):JobObjectPortsV1{
 return Object.freeze({start:args=>{
  const child=spawn(helper,[...args],{stdio:["pipe","pipe","ignore"],windowsHide:true}),done=new Promise<void>(finish=>{child.once("close",()=>finish());child.once("error",()=>finish());});
  async function* lines():AsyncIterable<string>{let rest="";for await(const chunk of child.stdout){rest+=chunk.toString();let index:number;while((index=rest.indexOf("\n"))>=0){yield rest.slice(0,index);rest=rest.slice(index+1);}}}
  return {lines:lines(),close:()=>{child.stdin.end();},done};
 }});
}

/** 🪟️ The Windows limiter: a job object with a job memory limit, an active process limit and kill-on-close, owned by the std-only helper which terminates the job at the limit and reports the frames `ready`, `usage` and `breach`; no external runtime library. */
export function createJobObjectLimiterV1(ports:JobObjectPortsV1):JobLimiterV1{
 return Object.freeze({enforcement:"job-object" as const,async create(policy:JobLimitPolicyV1):Promise<LimitHandleV1>{
  const callbacks:((breach:JobBreachV1)=>void)[]=[];
  let peak=0,fired=false,session:ReturnType<JobObjectPortsV1["start"]>|undefined,reader:Promise<void>=Promise.resolve();
  return Object.freeze({enforcement:"job-object" as const,
   async attach(pid:number){
    session=ports.start(["--attach",String(pid),"--memory-mb",String(policy.memoryMegabytes),"--pids",String(policy.pids),"--poll-ms",String(JOB_LIMIT_RULES_V1.cgroupPollMilliseconds)]);
    reader=(async()=>{for await(const line of session!.lines){
     let frame:{event?:string;peakMegabytes?:number};try{frame=JSON.parse(line);}catch{continue;}
     if(typeof frame.peakMegabytes==="number")peak=Math.max(peak,frame.peakMegabytes);
     if(frame.event==="breach"&&!fired){fired=true;const breach={peakMegabytes:peak,limitMegabytes:policy.memoryMegabytes};for(const callback of callbacks)callback(breach);}
    }})().catch(()=>undefined);
   },
   onBreach(callback:(breach:JobBreachV1)=>void){callbacks.push(callback);},
   async usage(){return{rssMegabytes:peak,peakMegabytes:peak};},
   async release(){session?.close();await Promise.race([Promise.all([session?.done,reader]),new Promise(done=>setTimeout(done,JOB_LIMIT_RULES_V1.graceMilliseconds))]);return{peakMegabytes:peak,breached:fired};}});
 }});
}
