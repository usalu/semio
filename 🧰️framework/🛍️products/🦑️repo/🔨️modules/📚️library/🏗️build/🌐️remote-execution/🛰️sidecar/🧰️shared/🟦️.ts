import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";

/** 🎭️ The two roles the replica daemon supervises: the agent (build worker and load reporter) and the graph server of every machine and, on the group server, the scheduler and the cache. */
export type RemoteExecutionSidecarRoleV1="agent"|"scheduler"|"cache"|"graph"|"pool"|"ready"|"spool";

/** 🧰️ What a sidecar needs: the worktree, the environment the replica daemon hands over (`SEMIO_FLEET_ORIGIN`, `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_STORE_ROOT`), cancellation and a log sink. */
export type RemoteExecutionSidecarContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;drain?:AbortSignal;write:(line:string)=>void;work?:SidecarWorkV1}>;

/** 🧵️ What a role that holds a long computation tells its restart and hears back: `busy` while it serves (the restart of a code change waits for the end of it), `restartPending` while the restart waits (the role takes no new work) and `running`, how many actions the agent serves for other agents, and `wake`, which the role calls when `busy` turns false so that the restart happens at once. */
export type SidecarWorkV1={busy:boolean;running?:number;restartPending:boolean;wake?:()=>void;progressAtMs?:()=>number|null};

/** 🔔️ What a server tells its restart about its load: `onActivity(active)` keeps `work.busy` and wakes the restart check (a scan of the whole code closure) only when the last request ended while a restart is waiting for it, and at most once per `minimumMilliseconds`, so a server that answers hundreds of requests a second never scans its code on every one of them. */
export function createActivityWakeV1(work:SidecarWorkV1|undefined,options:Readonly<{minimumMilliseconds?:number;now?:()=>number}>={}):(active:number)=>void{
 const now=options.now??Date.now,minimum=options.minimumMilliseconds??1000;let woken=-Infinity;
 return active=>{
  if(work===undefined)return;
  const was=work.busy;work.busy=active>0;
  if(!was||work.busy||!work.restartPending)return;
  const at=now();if(at-woken<minimum)return;
  woken=at;work.wake?.();
 };
}

/** ⏱️ Notices when the event loop of a service did not come back for a long time: `tick` is called every `intervalMilliseconds` by a timer; when the time since the last tick exceeds the interval by more than `thresholdMilliseconds` the service was stalled (a synchronous job, a starved file table, a stopped process) and `report` says for how long and how many files were open. With `profile` (see `createStallProfilerV1`) it also says what the loop was busy in; a lateness of `attributeMilliseconds` or more, below the stall threshold, is reported that way too, at most once per `attributeEveryMilliseconds`. */
export function createLoopLagWatchV1(input:Readonly<{intervalMilliseconds:number;thresholdMilliseconds:number;now?:()=>number;openFiles?:()=>number|undefined;profile?:()=>()=>string;attributeMilliseconds?:number;attributeEveryMilliseconds?:number;report:(line:string)=>void}>):Readonly<{tick():void}>{
 const now=input.now??Date.now,attributeAt=input.attributeMilliseconds??input.thresholdMilliseconds,every=input.attributeEveryMilliseconds??5000;let last=now(),reported=-Infinity;
 return Object.freeze({tick():void{
  const at=now(),lag=at-last-input.intervalMilliseconds;last=at;
  const describe=input.profile?.();
  if(lag>input.thresholdMilliseconds){
   const files=input.openFiles?.(),busy=describe?.();
   input.report(`event loop stalled for ${Math.round(lag)} ms${files===undefined?"":`, ${files} files open`}${busy?`, busy in ${busy}`:""} / Ereignisschleife stand ${Math.round(lag)} ms still${files===undefined?"":`, ${files} Dateien offen`}${busy?`, beschäftigt in ${busy}`:""}`);reported=at;
  }else if(describe!==undefined&&lag>=attributeAt&&at-reported>=every){
   const busy=describe();reported=at;
   input.report(`event loop was ${Math.round(lag)} ms late, busy in ${busy} / Ereignisschleife kam ${Math.round(lag)} ms zu spät zurück, beschäftigt in ${busy}`);
  }
 }});
}

/** 🔬️ Samples the JavaScript stacks of this process through the runtime's own sampling profiler (`bun:jsc`) so that a late event loop names what it was busy in: the returned function drains the samples since its last call (call it once per tick) and answers a function that formats the four most frequent stacks (`count x frame < caller < ...`). `undefined` where the runtime has no such profiler. */
export async function createStallProfilerV1():Promise<(()=>()=>string)|undefined>{
 try{
  const specifier="bun:jsc",jsc=await import(specifier) as {startSamplingProfiler():void;samplingProfilerStackTraces():unknown};
  jsc.startSamplingProfiler();
  return ()=>{
   const traces=(jsc.samplingProfilerStackTraces() as {traces?:readonly {frames:readonly {name?:string;sourceURL?:string;line?:number}[]}[]}).traces??[];
   return ()=>{
    const counts=new Map<string,number>();
    for(const trace of traces){const key=trace.frames.slice(0,6).map(frame=>`${frame.name??"?"}@${String(frame.sourceURL??"").split("/").slice(-2).join("/")}:${frame.line??0}`).join(" < ");counts.set(key,(counts.get(key)??0)+1);}
    return [...counts].sort((left,right)=>right[1]-left[1]).slice(0,4).map(([key,count])=>`${count}x ${key}`).join(" | ")||"native code";
   };
  };
 }catch{return undefined;}
}

export const sleep=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise<void>(done=>{
 if(signal.aborted)return done();
 const timer=setTimeout(done,milliseconds);
 signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});
});

/** 🔌️ The ports of the scheduler: `SEMIO_REMOTE_EXECUTION_PORTS=<grpc>,<worker>` when set (tests and machines where the ports are taken), else the ports the topology row of the group declares (`declared`: what every machine of a fleet uses), else the defaults of the standalone `repo remote-execution serve` in the policy. */
export function remoteExecutionPortsV1(environment:Readonly<Record<string,string|undefined>>,declared?:Readonly<{scheduler:number;worker:number}>):Readonly<{grpc:number;worker:number}>{
 const server=readRemoteExecutionPolicyV1().server,named=environment.SEMIO_REMOTE_EXECUTION_PORTS?.split(",").map(Number);
 if(named!==undefined&&named.length===2&&named.every(port=>Number.isInteger(port)&&port>=0&&port<=65535))return {grpc:named[0]!,worker:named[1]!};
 return declared===undefined?{grpc:server.port,worker:server.workerPort}:{grpc:declared.scheduler,worker:declared.worker};
}

/** 🔗️ The worker API (HTTP) of the scheduler that a gRPC endpoint names: the same host on the policy's worker port. */
export function remoteExecutionWorkerApiV1(endpoint:string,workerPort:number):string{
 const url=new URL(endpoint.replace(/^grpc(s?):/u,"http$1:"));
 return `${url.protocol}//${url.hostname.includes(":")&&!url.hostname.startsWith("[")?`[${url.hostname}]`:url.hostname}:${workerPort}`;
}

/** 🤝️ Whether a role that must restart asks its launcher for a successor that binds the same port before it leaves: the cache does, over the launcher's channel, unless it is itself a successor that has not listened yet. */
export const sidecarHandsOverV1=(role:RemoteExecutionSidecarRoleV1,channel:boolean,environment:Readonly<Record<string,string|undefined>>):boolean=>role==="cache"&&channel&&environment.SEMIO_SIDECAR_SUCCESSOR!=="1";

/** 🤝️ Marks a successor as promoted once it listens: it is the serving generation from then on and hands over to its own successor at the next restart (a flag that outlived the promotion made every second restart a gap of seconds in which the port had no listener). */
export const settleSidecarSuccessorV1=(environment:Record<string,string|undefined>):void=>{delete environment.SEMIO_SIDECAR_SUCCESSOR;};
