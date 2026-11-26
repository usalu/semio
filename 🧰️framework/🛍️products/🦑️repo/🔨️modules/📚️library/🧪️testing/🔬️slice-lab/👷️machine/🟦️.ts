import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname} from "node:path";
import {sliceLabStageEnvironmentV1,sliceLabWarmV1,type SliceLabWarmRowV1} from "../🏠️home/🟦️.ts";
import {createReadyPortsV1,runReadyStageV1,runReadyV1,type ReadyStageV1,type ReadyTimingV1} from "../../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";

/** 👷️ One machine of the slice lab: the real readiness loop over the real ports (stage runner, lease, status, timings) with the lab's stage plan, until the machine is ready or a stage fails once. It files `result.json` and ends; the failure it files is the first one, not the retry. */
type Config={origin:string;root:string;group:string;stages:readonly ReadyStageV1[];state:string;result:string;timeoutSeconds:number;warm?:{source:string;tools:readonly string[]|"all";prime:readonly string[]}};
const config=JSON.parse(readFileSync(process.argv.find(value=>value.startsWith("--config="))!.slice(9),"utf8")) as Config;
const timings:ReadyTimingV1[]=[],controller=new AbortController();
const file=(value:unknown)=>{mkdirSync(dirname(config.result),{recursive:true});const temporary=`${config.result}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,config.result);};
const ports=createReadyPortsV1({root:config.root,environment:process.env,origin:config.origin,group:config.group,home:process.env.HOME,stages:config.stages,say:line=>process.stdout.write(`${new Date().toISOString()} ${line}\n`)});
const timing=ports.timing;
if(process.env.SEMIO_LAB_POOL==="1"){
 const say=(line:string)=>process.stdout.write(`${new Date().toISOString()} ${line}\n`);
 const [{runPool},{readOwnerPoolStatusV1},{readHostAsksV1},{hostCapacityDirectoryV1}]=await Promise.all([import("../../../🏗️build/🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts"),import("../../../🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/📊️status/🟦️.ts"),import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🟦️.ts"),import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts")]);
 void runPool({root:config.root,environment:process.env,signal:controller.signal,write:line=>say(`pool: ${line}`)}).catch(error=>say(`pool failed: ${String(error)}`));
 const budget=setInterval(()=>{
  try{
   const asks=readHostAsksV1(hostCapacityDirectoryV1(process.env)),pool=readOwnerPoolStatusV1({store:process.env.SEMIO_FLEET_STORE_ROOT,origin:config.origin});
   say(`budget: asks ${asks.length===0?"none":asks.map(ask=>`${ask.key} ${ask.needMegabytes} MB${ask.alone?" alone":""}`).join(", ")}; pool ${pool===undefined?"not started":`${pool.state}, ${pool.children} children, ${pool.running} running, ${pool.reason??"no limit"}`}`);
  }catch{}
 },5000);
 controller.signal.addEventListener("abort",()=>clearInterval(budget),{once:true});
}
const warmed:SliceLabWarmRowV1[]=[];
if(config.warm!==undefined){
 const prime=await runReadyStageV1({root:config.root,environment:{...process.env,HTTPS_PROXY:"http://127.0.0.1:9",HTTP_PROXY:"http://127.0.0.1:9",NO_PROXY:"127.0.0.1,localhost"},stage:{name:"prime",kind:"repo",arguments:["tools","setup","toolchains",...config.warm.prime],timeoutMilliseconds:600000,inputs:[],bootstrapInput:false},signal:AbortSignal.timeout(600000),jobs:1,...process.env.HOME===undefined?{}:{home:process.env.HOME},onLine:line=>process.stdout.write(`${new Date().toISOString()} prime: ${line}\n`)});
 process.stdout.write(`${new Date().toISOString()} prime exit ${prime.exit}\n`);
 for(const stage of["dependencies","tools"])warmed.push(...sliceLabWarmV1({state:config.state,source:config.warm.source,stage,tools:config.warm.tools}));
}
const machineEnvironment={...process.env};
const wrapped={...ports,run:(async(stage:ReadyStageV1,options:Parameters<typeof ports.run>[1])=>{
 for(const name of Object.keys(process.env))if(!(name in machineEnvironment))delete process.env[name];
 const staged=sliceLabStageEnvironmentV1(stage.name,machineEnvironment);
 for(const[name,value]of Object.entries(staged)){if(value===undefined)delete process.env[name];else process.env[name]=value;}
 for(const name of Object.keys(machineEnvironment))if(!(name in staged))delete process.env[name];
 try{return await ports.run(stage,options);}finally{for(const[name,value]of Object.entries(machineEnvironment)){if(value===undefined)delete process.env[name];else process.env[name]=value;}for(const name of Object.keys(process.env))if(!(name in machineEnvironment))delete process.env[name];}
}) as typeof ports.run,timing:(run:ReadyTimingV1)=>{timings.push(run);timing?.(run);}};
const finish=(ready:boolean)=>{
 const failure=ready?null:ports.state.read().failure;
 file({warm:warmed,ready,failure:failure===null?null:{stage:failure.stage,code:failure.code,exit:failure.exit,tail:failure.tail},timings:timings.map(row=>({stage:row.stage,seconds:row.seconds,outcome:row.outcome}))});
 controller.abort();
};
const watch=setInterval(()=>{
 if(ports.status.current?.().ready===true)return finish(true);
 if(ports.state.read().failure!==null)return finish(false);
},500);
process.on("SIGTERM",()=>{clearInterval(watch);controller.abort();});
await runReadyV1(wrapped,controller.signal);
clearInterval(watch);
process.exit(0);
