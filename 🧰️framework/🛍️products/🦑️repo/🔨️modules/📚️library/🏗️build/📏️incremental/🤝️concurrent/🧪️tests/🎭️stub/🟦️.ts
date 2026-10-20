import {appendFileSync,existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";

/** 🎭️ A stand-in for the build CLI of a checkout copy, driven by `stub.json` in the working directory: it takes the real lease module's locks (so waits are real contention between processes), prints the CLI's own progress rows, Bazel's wait line and optionally the lease trace, executes and restores tasks against a content-addressed cache with or without deduplication, and reports what it executed through the task report file. */
type SpecV1={leases?:{resource:string;mode:"shared"|"exclusive";holdMs:number}[];bazelWaitMs?:number;tasks?:{id:string;inputs:string[];ms:number}[];stdout?:string;counter?:boolean;exit?:number;workMs?:number};
const root=process.cwd(),config=JSON.parse(readFileSync(join(root,"stub.json"),"utf8")) as {live:string;lease:string;dedupe:boolean;trace?:boolean;commands:Record<string,SpecV1>;default:SpecV1};
const rest=process.argv.slice(3).filter(argument=>!argument.startsWith("--tool=")),key=rest[0]==="run"?`run:${rest[1]}`:rest.join(" "),spec=config.commands[key]??config.default;
const lease=await import(join(config.live,config.lease)) as {acquireResourceLease(options:{directory:string;resource:string;mode:"shared"|"exclusive";signal:AbortSignal;onWait(progress:{resource:string;mode:"shared"|"exclusive";elapsedMs:number}):void}):Promise<{release():void}>};
if(!config.trace)delete process.env.SEMIO_LEASE_TRACE;
const controller=new AbortController(),sleep=(milliseconds:number)=>new Promise<void>(accept=>setTimeout(accept,milliseconds));
const row=(resource:string,mode:string,elapsedMs:number)=>process.stderr.write(`[script] ${JSON.stringify({owner:"repository:build",command:"repository:command",stage:"running",activity:{resource,mode,elapsedMs}})}\n`);
const hold=async<T>(resource:string,mode:"shared"|"exclusive",body:()=>Promise<T>):Promise<T>=>{
 mkdirSync(join(root,"leases"),{recursive:true});
 const startedMs=Date.now(),taken=await lease.acquireResourceLease({directory:join(root,"leases"),resource,mode,signal:controller.signal,onWait:progress=>row(progress.resource,progress.mode,progress.elapsedMs)}),acquiredMs=Date.now();
 if(config.trace&&process.env.SEMIO_LEASE_TRACE&&acquiredMs-startedMs>=20)appendFileSync(process.env.SEMIO_LEASE_TRACE,JSON.stringify({version:1,resource,mode,startedMs,acquiredMs,pid:process.pid})+"\n");
 try{return await body();}finally{taken.release();}
};
const executed:string[]=[],restored:string[]=[],cache=join(root,"cache");mkdirSync(cache,{recursive:true});
for(const wanted of spec.leases??[])await hold(wanted.resource,wanted.mode,()=>sleep(wanted.holdMs));
if(spec.bazelWaitMs){process.stderr.write("Another command (pid=4242) is running. Waiting for it to complete on the server (server_pid=77)...\n");await sleep(spec.bazelWaitMs);process.stderr.write("INFO: Invocation ID: stub\n");}
for(const task of spec.tasks??[]){
 const hash=createHash("sha256");for(const input of task.inputs)hash.update(existsSync(join(root,input))?readFileSync(join(root,input)):"missing");
 const entry=join(cache,`${task.id.replace(/[^a-z0-9]/gu,"_")}-${hash.digest("hex").slice(0,16)}`),work=async()=>{if(existsSync(entry)){restored.push(task.id);return;}await sleep(task.ms);writeFileSync(entry,"done");executed.push(task.id);};
 if(config.dedupe)await hold(`task:${task.id}`,"exclusive",work);else await work();
}
if(spec.workMs)await sleep(spec.workMs);
const report=process.env.SEMIO_BUILD_TASK_REPORT;
if(report)writeFileSync(report,JSON.stringify({executed,restored}));
let counter=0;
if(spec.counter){const file=join(root,"counter.txt");counter=existsSync(file)?Number(readFileSync(file,"utf8"))+1:1;writeFileSync(file,String(counter));}
appendFileSync(join(root,"calls.log"),`${process.pid} ${key}\n`);
process.stdout.write((spec.stdout??"ok")+(spec.counter?` ${counter}`:"")+"\n");
process.exit(spec.exit??0);
