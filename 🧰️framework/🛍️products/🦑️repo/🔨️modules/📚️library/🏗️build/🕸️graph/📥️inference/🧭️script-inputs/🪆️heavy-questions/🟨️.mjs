import{spawn}from"node:child_process";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{loadavg,tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";
import{RepoErrorV1,readRepoErrorDataV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import{hostProfileV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import{MACHINE_SPEED_POLICY_V1,localMachineSpeedFactorV1,processCpuMillisecondsV1}from"../../../../🐢️speed/🟨️.mjs";
import{fleetHostReadingV1}from"../../../../🧠️memory/🟨️.mjs";
import{wantedPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{soloGrantV1}from"../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{PROVIDER_CLOSURE_POLICY_V1}from"../🔌️provider-closures/🟨️.mjs";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import table from"./🔣️.json"with{type:"json"};

if(validateJsonSchemaSubset(schema,table).length)throw refuseBuildV1("invalid","closure.walk-views-required");

/** 🪆️ The declared policy of the heavy derived questions (schema `./🧬️schema/🔣️.json`). */
export const HEAVY_QUESTION_POLICY_V1=Object.freeze(structuredClone(table));
const SCRIPT=fileURLToPath(new URL("./📜️script.ts",import.meta.url));

/** 🪆️ Whether a derived row asks a question that must be answered outside the heap of this process: a TypeScript program or a derived walk, and this process is not already the child that answers it. */
export function isHeavyQuestionV1(row,environment=process.env){
 if(environment[HEAVY_QUESTION_POLICY_V1.environment]!==undefined)return false;
 let question;try{question=JSON.parse(row.path);}catch{return false;}
 return HEAVY_QUESTION_POLICY_V1.kinds.some(kind=>question?.[kind]===1);
}

/** 🚑️ The refusal a failed child leaves: its own refusal (code, key and values, so every language renders it) or, for any other failure, the bilingual generic one that names the exit status and the error line, never the stack frames below it. */
const failureOfV1=(code,tail,output)=>{
 try{
  const row=JSON.parse(readFileSync(output,"utf8"));
  if(row.refusal){const data=readRepoErrorDataV1(row.refusal);return new RepoErrorV1(data.code,data.key,data.values);}
  if(typeof row.failure==="string")return refuseBuildV1("failed","graph.heavy-question-failed",{exit:String(code),detail:row.failure.slice(-600)});
 }catch{}
 const lines=tail.trim().split("\n").map(line=>line.trim()).filter(Boolean),named=lines.filter(line=>/^(error|[A-Za-z]*Error)\b/.test(line));
 return refuseBuildV1("failed","graph.heavy-question-failed",{exit:String(code),detail:(named.length?named:lines.slice(-3)).join(" | ").slice(-600)||"-"});
};

/** 🐢️ How many times longer than on the reference machine a child of a heavy question may show no sign of life before it counts as stuck: the speed factor of the machine times the one-minute load per core where that is above one, never above `maximumScale`; a machine at load 36 on 10 cores waits 3.6 times as long, not the same minute as an idle one.
 * @see ../../../../🐢️speed/🟨️.mjs `localMachineSpeedFactorV1` */
export function heavyQuestionScaleV1(policy=HEAVY_QUESTION_POLICY_V1,environment=process.env,load=loadavg()[0]){
 const cores=Math.max(1,hostProfileV1(environment).cores);
 return Math.min(policy.maximumScale,Math.max(1,localMachineSpeedFactorV1(environment))*Math.max(1,load/cores));
}

/** 🎫️ The resident memory one heavy child is admitted with on this machine, by the grant machinery of the graph reads: the solo grant (`soloGrantV1`: the memory of the host less the parent and the system reserve, and the available memory less the available floor, so free memory and reserve both bind) shared by `concurrentChildren` heavy children, never below `memoryFloorMegabytes` and never above `memoryCeilingMegabytes`. Returns the cap with the numbers it was derived from, which the refusal of a child that outgrows it names.
 * @see ../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts `soloGrantV1` */
export function heavyQuestionCapV1(policy=HEAVY_QUESTION_POLICY_V1,reading=fleetHostReadingV1()){
 const solo=soloGrantV1(reading,wantedPolicyV1).footprintMegabytes,shared=Math.floor(solo/policy.concurrentChildren),megabytes=Math.min(policy.memoryCeilingMegabytes,Math.max(policy.memoryFloorMegabytes,shared));
 return Object.freeze({megabytes,host:reading.hostMegabytes,available:reading.availableMegabytes??null,children:policy.concurrentChildren,floor:policy.memoryFloorMegabytes,ceiling:policy.memoryCeilingMegabytes});
}

/** 🛡️ The cancel record the host warden wrote for a pid, or null: a child that ended by SIGKILL with such a record was cancelled to protect the machine, not crashed.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts `readHostWardenCancelV1` */
const wardenCancelOfV1=async pid=>(await import("../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")).readHostWardenCancelV1(pid);

const BEAT=/^\u0001alive(?: (\S+))?(?: (\d+))?$/u;
const questionOfV1=request=>request.prime?"prime":request.preflight?"preflight":String(request.row?.kind??"derived")+" "+String(request.row?.path??"").slice(0,120);
const lanes={questions:Promise.resolve(),prime:Promise.resolve()};
const inLane=(name,task)=>{const run=lanes[name].then(task,task);lanes[name]=run.then(()=>undefined,()=>undefined);return run;};
const running=new Set();
process.once("exit",()=>{for(const child of running)try{child.kill("SIGKILL");}catch{}});

const wallAfterV1=(task,milliseconds)=>{const handle=setTimeout(task,milliseconds);return()=>clearTimeout(handle);};
const wallEveryV1=(task,milliseconds)=>{const handle=setInterval(task,milliseconds);return()=>clearInterval(handle);};
const answerIn=(request,workspaceRoot,signal,policy=HEAVY_QUESTION_POLICY_V1,script=SCRIPT,ports={})=>new Promise((resolve,reject)=>{
 signal?.throwIfAborted();
 const directory=mkdtempSync(join(tmpdir(),"semio-heavy-question-")),output=join(directory,"answer.json");
 const{scale=heavyQuestionScaleV1,cpuOf=processCpuMillisecondsV1,cancelOf=wardenCancelOfV1,now=Date.now,after=wallAfterV1,every=wallEveryV1,start=spawn,host=fleetHostReadingV1}=ports,spawnedAt=now(),cap=heavyQuestionCapV1(policy,host()),factor=scale(policy),window=Math.ceil(policy.silenceMilliseconds*factor),look=Math.min(policy.livenessMilliseconds,window>>1),question=questionOfV1(request);
 let silent=false,phase="boot",lag=0,cpuSeen,partial="",tail="",lastLife=now(),lookedAt=lastLife;
 const done=(error,value)=>{signal?.removeEventListener("abort",abort);stopTimer();stopWatch();running.delete(child);rmSync(directory,{recursive:true,force:true});error?reject(error):resolve(value);};
 const child=start(process.execPath,[script,workspaceRoot,output],{stdio:["pipe","ignore","pipe"],windowsHide:true,env:{...process.env,[policy.environment]:String(cap.megabytes)}});
 running.add(child);
 child.stderr.on("data",chunk=>{
  lastLife=now();cpuSeen=undefined;partial+=chunk;
  const lines=partial.split("\n");partial=lines.pop();
  for(const line of lines){const beat=BEAT.exec(line);if(beat===null){tail=(tail+line+"\n").slice(-2000);continue;}phase=beat[1]??phase;lag=Math.max(lag,Number(beat[2]??0));}
 });
 const abort=()=>{child.kill();};signal?.addEventListener("abort",abort,{once:true});
 const stopTimer=after(()=>child.kill(),Math.ceil(policy.timeoutMilliseconds*factor));
 const stopWatch=every(()=>{
  const at=now();
  if(at-lastLife>=look&&at-lookedAt>=look){
   lookedAt=at;const spent=cpuOf(child.pid);
   if(spent!==undefined){if(cpuSeen!==undefined&&spent-cpuSeen>=MACHINE_SPEED_POLICY_V1.livenessCpuMilliseconds)lastLife=at;cpuSeen=spent;}
  }
  if(at-lastLife>=window){silent=true;child.kill("SIGKILL");}
 },Math.max(10,Math.min(1000,window>>2)));
 child.once("error",error=>done(error));
 child.once("close",(code,killed)=>{
  tail+=partial;
  if(signal?.aborted)return done(signal.reason);
  if(code===null&&killed==="SIGKILL"&&!silent){
   const pid=child.pid;
   void Promise.resolve().then(()=>cancelOf(pid)).catch(()=>null).then(record=>done(record!=null&&record.atMs>=spawnedAt?refuseBuildV1("failed","graph.heavy-question-cancelled",{pid:String(pid),kind:record.kind,megabytes:String(record.megabytes),reason:record.reason,question}):failureOfV1(code,tail,output)));
   return;
  }
  if(silent)return done(refuseBuildV1("failed","graph.heavy-question-silent",{seconds:String(Math.round((now()-lastLife)/1000)),question,phase,lag:String(Math.round(lag/1000)),detail:tail.trim().split("\n").slice(-2).join(" | ").slice(-300)||"-"}));
  const grown=/resident memory cap of (\d+) MB exceeded \(resident (\d+) MB\)/u.exec(tail);
  if(grown!==null)return done(refuseBuildV1("failed","graph.heavy-question-memory",{limit:grown[1],rss:grown[2],host:String(cap.host),available:cap.available===null?"?":String(cap.available),children:String(cap.children),floor:String(cap.floor),ceiling:String(cap.ceiling),question,phase}));
  if(code!==0)return done(failureOfV1(code,tail,output));
  try{done(undefined,JSON.parse(readFileSync(output,"utf8")));}catch(error){done(error);}
 });
 child.stdin.once("error",()=>{});child.stdin.end(JSON.stringify(request));
});

/** 🪆️ Asks one heavy derived question in a child process, one at a time (its own lane: a long prime never holds a question back), and returns `{fact,rows}` as the in-process deriver would. A child that shows no sign of life (no line on its standard error, no processor time) for `silenceMilliseconds` times the scale of the machine is stopped with a refusal that names the question, its last phase and the silent seconds; a child that computes without printing, or waits on a lease with its loop turning, is never stopped. */
export function askHeavyQuestionV1(row,{workspaceRoot,signal,policy,script,ports}){return inLane("questions",()=>answerIn({row},workspaceRoot,signal,policy,script,ports));}

/** 🧰️ Derives the shared closure entries of a workspace in a child process capped at its own resident memory, in the lane of the prime (the heavy questions run beside it), and stores them durably below `storeDirectory`; the answer counts what the child derived, loaded and found stale. The transient of the walks never touches the heap of this process, which only reads the stored entries afterwards. */
export function primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal,policy,script,ports}){return inLane("prime",()=>answerIn({prime:{storeDirectory}},workspaceRoot,signal,policy,script,ports));}

/** 🛃️ The refusal of a preflight: the refusal itself when one closure is refused, otherwise one refusal that names the count and every tool with its key and values. */
const refusedV1=rows=>rows.length===1?new RepoErrorV1(rows[0].code,rows[0].key,rows[0].values):refuseBuildV1("invalid","closure.tool-closures-refused",{count:String(rows.length),details:rows.map(row=>row.name+": "+row.key+" "+JSON.stringify(row.values)).join("; ").slice(0,1500)});
const preflights=new Map();

/** 🛃️ Walks every declared tool closure and provider port in the capped child before a read derives anything, once per process and policy, and refuses with what the child found: a closure that can never be derived is known in seconds, not after the work of the read. */
export const refusedToolClosuresV1=refusedV1;
export function preflightToolClosuresInChildV1({workspaceRoot,signal,ports}){
 const key=JSON.stringify([workspaceRoot,PROVIDER_CLOSURE_POLICY_V1.tools]);
 let known=preflights.get(key);
 if(known===undefined){
  const run=inLane("prime",()=>answerIn({preflight:true},workspaceRoot,signal,undefined,undefined,ports));
  known=run.then(({checked,refused})=>{if(refused.length)throw refusedV1(refused);return{checked};});
  preflights.set(key,known);known.catch(()=>preflights.delete(key));
 }
 return known;
}
