import{availableParallelism,totalmem}from"node:os";
import{validateJsonSchemaSubset}from"../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{requireHostCapacityV1,resolveHostCapacityLanguageV1,hostCapacityErrorV1,type HostPlanV1}from"../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import{hostProfileV1}from"../../🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import hostSchema from"../../🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import table from"./🔣️.json"with{type:"json"};

const protocol=schema.definitions.Protocol.const,messages=schema.definitions.Messages.const;

/** 🏷️ Names a test runner whose parallelism the policy bounds. */
export type TestRunnerIdV1="bun-test"|"vitest"|"cargo-test"|"playwright"|"e2e-stacks";
/** 👷️ Declares one runner: its override variable, what its process costs without workers and per worker (measured peak RSS), how many workers it asks for by default (`cores` = one per core of the host), its hard maximum and how many workers one admitted core may carry. */
export type TestRunnerV1=Readonly<{id:TestRunnerIdV1;override:string;baseMegabytes:number;workerMegabytes:number;wanted:number|"cores";maximum:number;oversubscription:number}>;
/** 📋️ The closed worker policy: the share of an unadmitted host a runner may take and the runners. */
export type TestWorkerPolicyV1=Readonly<{version:1;hostShareBasisPoints:number;runners:readonly TestRunnerV1[]}>;
/** 🌡️ What a runner may count on: the cores and megabytes of its share of the admitted grant (one job), or of the host when nothing admitted it. */
export type TestWorkerFactsV1=Readonly<{source:"grant"|"host";cores:number;hostCores:number;memoryMegabytes:number}>;
/** 🎯️ What decided the number of workers. */
export type TestWorkerBoundV1="override"|"request"|"wanted"|"maximum"|"cores"|"memory";
/** 🧾️ The answer: the workers to start, what the runner wanted, what bound it and the sizes the decision rested on (`declaredMegabytes` is what the runner is declared to hold with these workers, the default size an observed peak is compared with). */
export type TestWorkerPlanV1=Readonly<{runner:TestRunnerIdV1;workers:number;wanted:number;bound:TestWorkerBoundV1;source:"grant"|"host";cores:number;memoryMegabytes:number;workerMegabytes:number;declaredMegabytes:number;ratioBasisPoints:number}>;
/** ✋️ What a caller adds: a count it asks for, the learned size ratio of its class and an override text. */
export type TestWorkerOptionsV1=Readonly<{request?:number;ratioBasisPoints?:number;override?:string}>;

const fill=(text:string,values:Readonly<Record<string,string|number>>)=>text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
const refuse=(name:keyof typeof messages.en.errors,values:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>)=>Error(fill(messages[resolveHostCapacityLanguageV1(environment)].errors[name],values));

/** 📖️ Reads the worker policy, refusing a document that breaks the schema, repeats a runner or wants more than it may have. */
export function readTestWorkerPolicyV1(value:unknown=table):TestWorkerPolicyV1{
 const findings=validateJsonSchemaSubset(schema.definitions.Policy,value,schema);
 if(findings.length)throw refuse("policy",{detail:String(findings[0])},process.env);
 const policy=value as TestWorkerPolicyV1,ids=policy.runners.map(row=>row.id);
 if(new Set(ids).size!==ids.length)throw refuse("policy",{detail:"runner repeated"},process.env);
 for(const row of policy.runners)if(typeof row.wanted==="number"&&row.wanted>row.maximum)throw refuse("policy",{detail:row.id+" wants more than its maximum"},process.env);
 return policy;
}

/** 🔎️ Reads what the process may count on: its share (one job) of the grant its environment carries, else the host share of the policy; a plan that is not a plan is refused. */
export function testWorkerFactsV1(environment:Readonly<Record<string,string|undefined>>,policy:TestWorkerPolicyV1=readTestWorkerPolicyV1(),host:Readonly<{cores:number;totalMegabytes:number}>=hostProfileV1(environment,{cores:availableParallelism(),totalMegabytes:Math.floor(totalmem()/1048576),declared:false})):TestWorkerFactsV1{
 const text=environment[hostSchema.definitions.Protocol.const.planEnvironment];
 if(text===undefined||text==="")return Object.freeze({source:"host",cores:host.cores,hostCores:host.cores,memoryMegabytes:Math.floor(host.totalMegabytes*policy.hostShareBasisPoints/10000)});
 let value:unknown;
 try{value=JSON.parse(text);}catch{throw hostCapacityErrorV1("planRequired",{},environment);}
 const plan=requireHostCapacityV1<HostPlanV1>("Plan",value),jobs=Math.max(1,plan.jobs);
 return Object.freeze({source:"grant",cores:Math.max(1,Math.floor(plan.cpu/jobs)),hostCores:host.cores,memoryMegabytes:plan.memoryMegabytes>0?Math.floor(plan.memoryMegabytes/jobs):plan.fixedMegabytes});
}

/** 🔢️ Reads the override variable of a runner: an integer from 1 to the maximum of the runner, or nothing when the variable is not set. */
export function readTestWorkerOverrideV1(runner:TestRunnerV1,value:string|undefined,environment:Readonly<Record<string,string|undefined>>=process.env):number|undefined{
 if(value===undefined)return undefined;
 const count=/^[1-9][0-9]{0,2}$/u.test(value)?Number(value):0;
 if(count<1||count>runner.maximum)throw refuse("override",{variable:runner.override,maximum:runner.maximum,value},environment);
 return count;
}

/** 🧮️ Decides how many workers a runner starts: what it wants, bounded by the memory of its share (a worker costs the declared size scaled by the learned ratio), by the cores of its share times the oversubscription the runner carries, and by its maximum; never fewer than one, never more than an override asks for within those cores. */
export function planTestWorkersV1(policy:TestWorkerPolicyV1,id:TestRunnerIdV1,facts:TestWorkerFactsV1,options:TestWorkerOptionsV1={},environment:Readonly<Record<string,string|undefined>>=process.env):TestWorkerPlanV1{
 const runner=policy.runners.find(row=>row.id===id);
 if(!runner)throw refuse("runner",{runner:id},environment);
 const ratio=options.ratioBasisPoints??10000,scaled=(megabytes:number)=>Math.ceil(megabytes*ratio/10000),base=scaled(runner.baseMegabytes),each=Math.max(1,scaled(runner.workerMegabytes));
 const asked=options.request??(runner.wanted==="cores"?facts.hostCores:runner.wanted),wanted=Math.max(1,Math.min(asked,runner.maximum)),clipped:TestWorkerBoundV1=asked>runner.maximum?"maximum":options.request!==undefined?"request":"wanted";
 const memoryCap=facts.memoryMegabytes>=base+each?Math.floor((facts.memoryMegabytes-base)/each):0,coreCap=facts.cores*runner.oversubscription,forced=readTestWorkerOverrideV1(runner,options.override,environment);
 const workers=forced!==undefined?Math.max(1,Math.min(forced,runner.maximum,coreCap)):Math.max(1,Math.min(wanted,memoryCap,coreCap));
 const bound:TestWorkerBoundV1=forced!==undefined?workers===forced?"override":"cores":memoryCap<wanted&&memoryCap<=coreCap?"memory":coreCap<wanted?"cores":clipped;
 return Object.freeze({runner:id,workers,wanted,bound,source:facts.source,cores:facts.cores,memoryMegabytes:facts.memoryMegabytes,workerMegabytes:each,declaredMegabytes:runner.baseMegabytes+workers*runner.workerMegabytes,ratioBasisPoints:ratio});
}

/** 🛠️ Plans the workers of one runner for the process that asks: policy, facts of its environment and the override variable the policy declares for the runner. */
export function testWorkersV1(id:TestRunnerIdV1,environment:Readonly<Record<string,string|undefined>>=process.env,options:Omit<TestWorkerOptionsV1,"override">={},policy:TestWorkerPolicyV1=readTestWorkerPolicyV1(),host?:Readonly<{cores:number;totalMegabytes:number}>):TestWorkerPlanV1{
 const runner=policy.runners.find(row=>row.id===id);
 return planTestWorkersV1(policy,id,testWorkerFactsV1(environment,policy,host),{...options,override:runner===undefined?undefined:environment[runner.override]},environment);
}

/** 🗣️ Words a plan in the language of the environment, only when the runner gets fewer workers than it wanted (nothing to say when it gets what it asked for). */
export function describeTestWorkersV1(plan:TestWorkerPlanV1,environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 if(plan.workers>=plan.wanted)return undefined;
 const words:(typeof messages)["en"]=messages[resolveHostCapacityLanguageV1(environment)];
 return fill(words.reduced,{runner:words.runners[plan.runner],workers:plan.workers,wanted:plan.wanted,bound:words.bounds[plan.bound],memory:plan.memoryMegabytes,cores:plan.cores,source:words.sources[plan.source],each:plan.workerMegabytes});
}

/** 📣️ Writes the description of a reduced plan; a plan that got what it wanted writes nothing. */
export function announceTestWorkersV1(plan:TestWorkerPlanV1,environment:Readonly<Record<string,string|undefined>>=process.env,write:(line:string)=>void=line=>process.stderr.write(line+"\n")):TestWorkerPlanV1{
 const line=describeTestWorkersV1(plan,environment);
 if(line!==undefined)write(line);
 return plan;
}
