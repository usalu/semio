import{spawn}from"node:child_process";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{loadavg,tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{refuseBuildV1}from"../../../../📖️texts/🟦️.ts";
import{RepoErrorV1,readRepoErrorDataV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import{hostProfileV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import{MACHINE_SPEED_POLICY_V1,localMachineSpeedFactorV1,processCpuMillisecondsV1}from"../../../../🐢️speed/🟨️.mjs";
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

const BEAT=/^\u0001alive(?: (\S+))?(?: (\d+))?$/u;
const questionOfV1=request=>request.prime?"prime":request.preflight?"preflight":String(request.row?.kind??"derived")+" "+String(request.row?.path??"").slice(0,120);
const lanes={questions:Promise.resolve(),prime:Promise.resolve()};
const inLane=(name,task)=>{const run=lanes[name].then(task,task);lanes[name]=run.then(()=>undefined,()=>undefined);return run;};
const running=new Set();
process.once("exit",()=>{for(const child of running)try{child.kill("SIGKILL");}catch{}});

const answerIn=(request,workspaceRoot,signal,policy=HEAVY_QUESTION_POLICY_V1,script=SCRIPT,ports={})=>new Promise((resolve,reject)=>{
 signal?.throwIfAborted();
 const directory=mkdtempSync(join(tmpdir(),"semio-heavy-question-")),output=join(directory,"answer.json");
 const{scale=heavyQuestionScaleV1,cpuOf=processCpuMillisecondsV1}=ports,factor=scale(policy),window=Math.ceil(policy.silenceMilliseconds*factor),look=Math.min(policy.livenessMilliseconds,window>>1),question=questionOfV1(request);
 let silent=false,phase="boot",lag=0,cpuSeen,partial="",tail="",lastLife=Date.now(),lookedAt=lastLife;
 const done=(error,value)=>{signal?.removeEventListener("abort",abort);clearTimeout(timer);clearInterval(watch);running.delete(child);rmSync(directory,{recursive:true,force:true});error?reject(error):resolve(value);};
 const child=spawn(process.execPath,[script,workspaceRoot,output],{stdio:["pipe","ignore","pipe"],windowsHide:true,env:{...process.env,[policy.environment]:String(policy.memoryLimitMegabytes)}});
 running.add(child);
 child.stderr.on("data",chunk=>{
  lastLife=Date.now();cpuSeen=undefined;partial+=chunk;
  const lines=partial.split("\n");partial=lines.pop();
  for(const line of lines){const beat=BEAT.exec(line);if(beat===null){tail=(tail+line+"\n").slice(-2000);continue;}phase=beat[1]??phase;lag=Math.max(lag,Number(beat[2]??0));}
 });
 const abort=()=>{child.kill();};signal?.addEventListener("abort",abort,{once:true});
 const timer=setTimeout(()=>child.kill(),Math.ceil(policy.timeoutMilliseconds*factor));
 const watch=setInterval(()=>{
  const now=Date.now();
  if(now-lastLife>=look&&now-lookedAt>=look){
   lookedAt=now;const spent=cpuOf(child.pid);
   if(spent!==undefined){if(cpuSeen!==undefined&&spent-cpuSeen>=MACHINE_SPEED_POLICY_V1.livenessCpuMilliseconds)lastLife=now;cpuSeen=spent;}
  }
  if(now-lastLife>=window){silent=true;child.kill("SIGKILL");}
 },Math.max(10,Math.min(1000,window>>2)));
 child.once("error",error=>done(error));
 child.once("close",code=>{
  tail+=partial;
  if(signal?.aborted)return done(signal.reason);
  if(silent)return done(refuseBuildV1("failed","graph.heavy-question-silent",{seconds:String(Math.round((Date.now()-lastLife)/1000)),question,phase,lag:String(Math.round(lag/1000)),detail:tail.trim().split("\n").slice(-2).join(" | ").slice(-300)||"-"}));
  if(code!==0)return done(failureOfV1(code,tail,output));
  try{done(undefined,JSON.parse(readFileSync(output,"utf8")));}catch(error){done(error);}
 });
 child.stdin.end(JSON.stringify(request));
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
