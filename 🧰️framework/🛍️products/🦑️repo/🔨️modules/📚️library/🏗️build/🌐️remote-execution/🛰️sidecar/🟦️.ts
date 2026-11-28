import {readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {createSidecarClosureWatchV1,createSidecarRestartDebounceV1,type SidecarClosureMemoryV1,type SidecarRestartPolicyV1} from "./🧮️closure/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {applyServicePriorityV1,servicePriorityTextV1} from "./🪜️priority/🟦️.ts";
import {startSidecarIdleCompactionV1} from "../../🧹️cache-gc/🛌️idle/⏲️loop/🟦️.ts";
import type {RemoteExecutionSidecarContextV1,RemoteExecutionSidecarRoleV1,SidecarWorkV1} from "./🧰️shared/🟦️.ts";
import {sidecarHandsOverV1} from "./🧰️shared/🟦️.ts";
import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import {createSidecarCodeRecorderV1} from "./📇️code-record/🟦️.ts";
import {readySidecarSilenceV1,readySilenceChildNoticeV1,readySilenceChildWatchV1,readySilencePolicyV1,readySilenceStatusFileV1} from "./🚦️ready/💔️silence/🟦️.ts";

export {remoteExecutionPortsV1,remoteExecutionWorkerApiV1,sidecarHandsOverV1,settleSidecarSuccessorV1} from "./🧰️shared/🟦️.ts";
export type {RemoteExecutionSidecarContextV1,RemoteExecutionSidecarRoleV1,SidecarWorkV1} from "./🧰️shared/🟦️.ts";

const ROLE_MODULES:Readonly<Record<RemoteExecutionSidecarRoleV1,string>>={agent:"👷️agent",scheduler:"🗓️scheduler",cache:"🗄️cache",graph:"🕸️graph",pool:"🏊️pool",ready:"🚦️ready",spool:"📮️spool"};

/** 🕰️ The measured numbers of a restart: how often the closure is looked at, the quiet period, the maximum deferral and the grace for the work in flight. See `./🧬️schema/🔣️.json`. */
export const sidecarRestartPolicyV1=Object.freeze({...schema.definitions.Restart.const});

/** ♻️ The restart numbers of one role: the common ones, overridden by the role's own entry (`pool` restarts within seconds, `ready` never defers for its stage in flight because the next child adopts it). */
export function sidecarRestartPolicyForV1(role:string):typeof sidecarRestartPolicyV1&Readonly<{serveDeferralMilliseconds:number;drainMaximumMilliseconds?:number;drainPollMilliseconds?:number}>{
 return role==="pool"?{...sidecarRestartPolicyV1,...sidecarRestartPolicyV1.pool}:role==="ready"?{...sidecarRestartPolicyV1,...sidecarRestartPolicyV1.ready}:role==="agent"?{...sidecarRestartPolicyV1,...sidecarRestartPolicyV1.agent}:role==="cache"?{...sidecarRestartPolicyV1,...sidecarRestartPolicyV1.cache}:sidecarRestartPolicyV1;
}

const REASON_KEYS:Readonly<Record<string,"reasonQuiet"|"reasonMaximum"|"reasonServed"|"reasonServeMaximum">>={quiet:"reasonQuiet",maximum:"reasonMaximum",served:"reasonServed",serveMaximum:"reasonServeMaximum"};

/** 🔔️ One message of the restart texts, English and German in one line (the reason is named in both languages). */
export function sidecarRestartTextV1(key:keyof typeof schema.definitions.Texts.const.en,fields:Readonly<Record<string,string|number>>={}):string{
 const fill=(language:"en"|"de"):string=>schema.definitions.Texts.const[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>name==="reason"?schema.definitions.Texts.const[language][REASON_KEYS[String(fields.reason)]??"reasonQuiet"]:String(fields[name]??""));
 return `${fill("en")} / ${fill("de")}`;
}

/** ♻️ Everything the restart of a sidecar needs: the watch over its closure, the policy, what to do when the restart is due (`begin`, then `drain` and a later `abort` for the agent, `abort` at once for the others) and where to say it. */
export type SidecarRestartInputV1=Readonly<{role:RemoteExecutionSidecarRoleV1;watch:Pick<ReturnType<typeof createSidecarClosureWatchV1>,"baseline"|"closure"|"changes">;policy:SidecarRestartPolicyV1&Readonly<{graceMilliseconds:number;serveDeferralMilliseconds?:number;progressStallMilliseconds?:number;minimumIntervalMilliseconds?:number;drainMaximumMilliseconds?:number;drainPollMilliseconds?:number}>;work?:SidecarWorkV1;begin():void;drain():void;abort():void;write(line:string):void;now?:()=>number;after?:(milliseconds:number,run:()=>void)=>void}>;

/** ♻️ Decides, one `tick` at a time, when a sidecar restarts for new code: the closure must differ from the one it started with, stay quiet for the policy's time (a closure edited without a pause restarts after the maximum deferral), then the agent stops taking new work (`drain`) and is aborted when the work in flight is done; after the grace it is aborted only if it serves nothing for other agents, otherwise it waits for those actions to end by their own deadline (`drainMaximumMilliseconds` bounds a leaked flag, never a running action's own timeout), the other roles are aborted at once. The graph role, which passes `work`, is different: a code change is not urgent for a process in the middle of a long computation whose results are keyed by the implementation that produced them (the read itself runs under a detached runner and survives any restart; the wait only keeps the server from claiming new work meanwhile), so while `work.busy` the restart waits (`work.restartPending` tells the role to take no new work), for `serveDeferralMilliseconds` at the latest counted from the first change, and happens at once when the role calls `work.wake` after the work ended. It says once that a restart is waiting, with the files, and once why and after how long it restarts. A graph read in flight is not touched by any of this: it runs under a detached runner (`./🕸️graph/🏃️runner`) and the restarted graph server attaches to it again. */
export function createSidecarRestartV1(input:SidecarRestartInputV1):Readonly<{tick():void}>{
 const now=input.now??Date.now,debounce=createSidecarRestartDebounceV1(input.policy,now),after=input.after??((milliseconds,run)=>{setTimeout(run,milliseconds).unref();});
 const work=input.role==="graph"||input.role==="ready"||input.role==="cache"?input.work:undefined,serveMaximum=input.policy.serveDeferralMilliseconds??sidecarRestartPolicyV1.serveDeferralMilliseconds;
 const created=now(),stall=input.policy.progressStallMilliseconds??sidecarRestartPolicyV1.progressStallMilliseconds;let announced=false,finished=false,waiting=false;
 const holding=(deferred:number):boolean=>{const at=work?.progressAtMs?.();return at===undefined||at===null?deferred<serveMaximum:now()-at<stall;};
 const files=():string=>{const names=input.watch.changes();return names.slice(0,6).join(", ")+(names.length>6?`, +${names.length-6}`:"");};
 const wait=():void=>{waiting=true;work!.restartPending=true;input.write(sidecarRestartTextV1("waitingForServe",{role:input.role,files:files(),minutes:Math.round(serveMaximum/60000)}));};
 const tick=():void=>{
  if(finished)return;
  const verdict=debounce.observe(input.watch.closure().digest,input.watch.baseline);
  if(verdict.state==="same"){announced=false;waiting=false;if(work!==undefined)work.restartPending=false;return;}
  const over=waiting&&!work!.busy;
  if(verdict.state==="deferred"&&!over){
   if(!announced){
    announced=true;
    if(work?.busy===true&&serveMaximum>0)wait();
    else if(work!==undefined){work.restartPending=true;input.write(sidecarRestartTextV1("deferred",{role:input.role,files:files(),quiet:Math.round(input.policy.quietMilliseconds/1000),maximum:Math.round(input.policy.maximumDeferralMilliseconds/1000)}));}
    else input.write(sidecarRestartTextV1("deferred",{role:input.role,files:files(),quiet:Math.round(input.policy.quietMilliseconds/1000),maximum:Math.round(input.policy.maximumDeferralMilliseconds/1000)}));
   }
   return;
  }
  if(verdict.state==="due"&&(input.policy.minimumIntervalMilliseconds??0)>now()-created)return;
  if(verdict.state==="due"&&work?.busy===true&&holding(verdict.deferredMilliseconds)){if(!waiting)wait();return;}
  finished=true;
  const reason=work!==undefined&&serveMaximum>0&&(waiting||work.busy)?work.busy?"serveMaximum":"served":verdict.reason!;
  input.write(sidecarRestartTextV1("restarting",{role:input.role,files:files(),deferred:Math.round(verdict.deferredMilliseconds/1000),reason}));
  input.begin();
  if(input.role!=="agent"){input.abort();return;}
  input.write(sidecarRestartTextV1("draining",{role:input.role,grace:Math.round(input.policy.graceMilliseconds/1000)}));
  input.drain();
  const served=input.work,bound=now()+(input.policy.drainMaximumMilliseconds??sidecarRestartPolicyV1.agent.drainMaximumMilliseconds),poll=input.policy.drainPollMilliseconds??sidecarRestartPolicyV1.agent.drainPollMilliseconds;
  let told=false;
  const retire=():void=>{
   if(served?.busy===true&&now()<bound){
    if(!told){told=true;input.write(sidecarRestartTextV1("drainingServed",{role:input.role,running:served.running??"?",minutes:Math.round((bound-now())/60000)}));}
    after(poll,retire);return;
   }
   if(served?.busy===true)input.write(sidecarRestartTextV1("drainBounded",{role:input.role,running:served!.running??"?",minutes:Math.round((input.policy.drainMaximumMilliseconds??sidecarRestartPolicyV1.agent.drainMaximumMilliseconds)/60000)}));
   input.abort();
  };
  after(input.policy.graceMilliseconds,retire);
 };
 if(work!==undefined)work.wake=tick;
 return Object.freeze({tick});
}

/** 🛰️ Runs one role of the build sidecars until the signal aborts; a role that cannot start (no scheduler published, no credential yet, port taken) throws, and the replica daemon restarts it with back-off. Only the role asked for is loaded, so a process holds (and is restarted for) the code of its own role. */
export async function runRemoteExecutionSidecarV1(role:RemoteExecutionSidecarRoleV1,context:RemoteExecutionSidecarContextV1):Promise<void>{
 if(role==="agent")return (await import("./👷️agent/🟦️.ts")).runAgent(context);
 if(role==="cache")return (await import("./🗄️cache/🟦️.ts")).runCache(context);
 if(role==="graph")return (await import("./🕸️graph/🟦️.ts")).superviseGraphServerV1(context);
 if(role==="pool")return (await import("./🏊️pool/🟦️.ts")).runPool(context);
 if(role==="ready")return (await import("./🚦️ready/🟦️.ts")).runReady(context);
 if(role==="spool")return (await import("./📮️spool/🟦️.ts")).runSpool(context);
 return (await import("./🗓️scheduler/🟦️.ts")).runScheduler(context);
}

/** 🧭️ The files that make up the code of one role: this module (static imports only) and the role's own module with everything it imports. */
export function remoteExecutionSidecarEntriesV1(role:RemoteExecutionSidecarRoleV1):Readonly<{entries:readonly string[];staticOnly:readonly string[]}>{
 const self=join(import.meta.dir,"🟦️.ts");
 return {entries:[self,join(import.meta.dir,ROLE_MODULES[role],"🟦️.ts")],staticOnly:[self]};
}

/** 🧬️ A watch that says when the code this role runs changed (closure digest over file contents), not when some file of the two modules was touched. */
export function remoteExecutionSidecarWatchV1(role:RemoteExecutionSidecarRoleV1,memory?:SidecarClosureMemoryV1):ReturnType<typeof createSidecarClosureWatchV1>{
 const {entries,staticOnly}=remoteExecutionSidecarEntriesV1(role);
 return createSidecarClosureWatchV1(entries,staticOnly,join(import.meta.dir,"..","..",".."),memory);
}

/** ⏱️ How long the launcher waits before it starts a crashed child again: it doubles from `initialMilliseconds` to `maximumMilliseconds` and starts again from the beginning once a child ran for `stableMilliseconds`. */
export const REMOTE_EXECUTION_SIDECAR_RESTART_V1=Object.freeze({initialMilliseconds:1000,maximumMilliseconds:15_000,stableMilliseconds:60_000,handoverMilliseconds:120_000,handoverAttempts:3,handoverRetryMilliseconds:2000});

/** 🔁️ The process the replica daemon supervises: a small launcher that runs the role in a child process and starts the child again by itself, a moment later and with the new code, when the child ends because the code it runs changed (exit status 75). A code landing therefore never depends on the daemon's loop to bring the service back (a daemon that is busy for minutes used to leave the cache and the scheduler down after they exited for new code). A child that crashes (a module that is missing for a moment while another landing replaces a folder is the usual cause) is started again by the launcher after a back-off of 1 s doubling to 15 s, with a log line that names the status: the role is back, with its worker, within seconds of the cause going away, instead of after the daemon's own back-off of up to a minute. The launcher ends only when it is told to (SIGTERM) or when the lease of its daemon (`SEMIO_FLEET_SIDECAR_LEASE`, renewed by the daemon every 5 s) was not renewed for `SEMIO_FLEET_SIDECAR_LEASE_MS` (2 h: a Windows daemon needs 10 to 50 minutes to open its tree and restarts for a blocked loop, so a shorter lease ended the readiness supervisor of the worker during every such restart); a daemon that restarts meanwhile adopts the launcher, which proves that it lives and runs the same entry script by its record (`SEMIO_FLEET_SIDECAR_RECORD`, rewritten every 2 s). It forwards SIGTERM and keeps no other state. */
export async function runRemoteExecutionSidecarLauncherV1(role:RemoteExecutionSidecarRoleV1,entry:string,environment:Readonly<Record<string,string|undefined>>,restart:typeof REMOTE_EXECUTION_SIDECAR_RESTART_V1=REMOTE_EXECUTION_SIDECAR_RESTART_V1):Promise<never>{
 const base=environment.SEMIO_FLEET_SIDECAR_LEASE,record=environment.SEMIO_FLEET_SIDECAR_RECORD,protocol=await import("../../../../🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts"),startedMs=Date.now(),lease=base===undefined||base===""?undefined:protocol.sidecarLeaseOfV1(base,process.pid);
 type Child=ReturnType<typeof Bun.spawn>;
 let stopping=false,alive=false,child:Child|undefined,successor:Child|undefined,launchedMs=Date.now();
 const stop=():void=>{stopping=true;if(!alive)process.exit(0);try{successor?.kill("SIGTERM");}catch{}try{child?.kill("SIGTERM");}catch{}};
 const launch=(extra:Readonly<Record<string,string>>={}):Child=>{
  launchedMs=Date.now();
  const created:Child=Bun.spawn([process.execPath,entry,role],{windowsHide:true,stdin:"ignore",stdout:"inherit",stderr:"inherit",env:{...environment,SEMIO_SIDECAR_CHILD:"1",...extra},ipc:(message:unknown)=>{hear(created,message);}});
  return created;
 };
 const say=(line:string):void=>{process.stderr.write(`${new Date().toISOString()} ${role} handover: ${line}\n`);};
 const hear=(from:Child,message:unknown):void=>{
  const type=(message as {type?:string}|null)?.type;
  if(type==="successor"&&from===child&&successor===undefined&&!stopping){
   const old=from,deadline=Date.now()+(restart.handoverMilliseconds??120_000),attempts=restart.handoverAttempts??3;
   const attempt=(number:number):void=>{
    if(stopping||child!==old){return;}
    const next=launch({SEMIO_SIDECAR_SUCCESSOR:"1"});successor=next;say(`successor ${number} of ${attempts} started (pid ${next.pid})`);
    const giveUp=setTimeout(()=>{if(successor===next){successor=undefined;say("the successor did not listen in time, the old process leaves");try{next.kill("SIGTERM");}catch{}try{old.kill("SIGTERM");}catch{}}},Math.max(1000,deadline-Date.now()));giveUp.unref();
    void next.exited.then(code=>{clearTimeout(giveUp);if(successor!==next)return;successor=undefined;say(`successor ${number} ended with status ${code} before it listened`);if(number<attempts&&Date.now()<deadline&&!stopping){setTimeout(()=>{attempt(number+1);},restart.handoverRetryMilliseconds??2000).unref();return;}say("no successor, the old process leaves");try{old.kill("SIGTERM");}catch{}});
   };
   attempt(1);
  }
  if(type==="listening"&&from===successor&&from!==undefined){const old=child;child=from;successor=undefined;say(`successor (pid ${from.pid}) listens, the old process (pid ${old?.pid}) drains`);try{old?.kill("SIGTERM");}catch{}}
 };
 for(const name of ["SIGTERM","SIGINT"] as const)process.on(name,stop);
 if(lease!==undefined&&lease!==""){
  const expire=Number(environment.SEMIO_FLEET_SIDECAR_LEASE_MS)||protocol.sidecarProtocolV1.leaseMilliseconds;
  setInterval(()=>{if(protocol.sidecarLeaseExpiredV1(lease,Date.now(),expire,startedMs)){process.stderr.write(`${new Date().toISOString()} ${role} lease not renewed for ${Math.round(expire/1000)} s, leaving\n`);stop();}},2000).unref();
 }
 if(record!==undefined&&record!==""){
  const entrySha256=protocol.sidecarEntryHashV1(entry)??"0".repeat(64),name=environment.SEMIO_FLEET_SIDECAR??role,beat=():void=>{try{const other=stopping?undefined:protocol.readSidecarRecordV1(record);if(other!==undefined&&other.pid!==process.pid&&other.startedMs>startedMs&&protocol.sidecarRecordLivesV1(other)){process.stderr.write(`${new Date().toISOString()} ${role} superseded by launcher ${other.pid} (started later), leaving / ersetzt durch Starter ${other.pid} (später gestartet), beendet sich\n`);stop();return;}protocol.writeSidecarRecordV1(record,{version:1,name,...ownProcessIdentityV1(),startedMs,wallMs:Date.now(),entrySha256});}catch{}};
  beat();setInterval(beat,protocol.sidecarProtocolV1.recordMilliseconds).unref();
  process.on("exit",()=>{try{if(protocol.readSidecarRecordV1(record)?.pid===process.pid)rmSync(record,{force:true});}catch{}});
 }
 const silenceFile=role==="ready"?readySilenceStatusFileV1(environment):undefined;
 if(silenceFile!==undefined){
  const pick=(name:string,fallback:number):number=>Number(environment[name])||fallback,watch=readySilenceChildWatchV1({...readySilencePolicyV1,silenceChildGraceMilliseconds:pick("SEMIO_SIDECAR_SILENCE_GRACE_MILLISECONDS",readySilencePolicyV1.silenceChildGraceMilliseconds)});
  setInterval(()=>{
   if(stopping||!alive||child===undefined)return;
   try{
    const status=JSON.parse(readFileSync(silenceFile,"utf8")),now=Date.now();
    if(watch.observe(status,now,launchedMs)!=="restart")return;
    process.stderr.write(`${new Date().toISOString()} ${role} ${readySilenceChildNoticeV1(readySidecarSilenceV1({status,nowMs:now}))}\n`);
    child.kill("SIGKILL");
   }catch{}
  },pick("SEMIO_SIDECAR_SILENCE_POLL_MILLISECONDS",5000)).unref();
 }
 let delay:number=restart.initialMilliseconds;
 for(;;){
  const started=Date.now();
  let code=1;
  try{child=launch();alive=true;for(;;){const waiting:Child|undefined=child;if(waiting===undefined)break;code=await waiting.exited;if(waiting===child)break;}}catch{}
  alive=false;
  if(stopping)process.exit(code===75?0:code);
  if(code===75){delay=restart.initialMilliseconds;await Bun.sleep(500);continue;}
  if(Date.now()-started>=restart.stableMilliseconds)delay=restart.initialMilliseconds;
  process.stderr.write(`${new Date().toISOString()} ${role} ended with status ${code}, starting again in ${(delay/1000).toFixed(delay<1000?2:0)} s\n`);
  await Bun.sleep(delay);
  delay=Math.min(restart.maximumMilliseconds,delay*2);
 }
}

if(import.meta.main){
 const role=process.argv[2];
 if(role!=="agent"&&role!=="scheduler"&&role!=="cache"&&role!=="graph"&&role!=="pool"&&role!=="ready"&&role!=="spool"){process.stderr.write("usage: sidecar agent|scheduler|cache|graph|pool|ready|spool\n");process.exit(2);}
 if(process.env.SEMIO_SIDECAR_CHILD!=="1")await runRemoteExecutionSidecarLauncherV1(role,import.meta.path,process.env);
 const controller=new AbortController(),launcher=process.ppid,watch=remoteExecutionSidecarWatchV1(role),work:SidecarWorkV1={busy:false,restartPending:false};
 let changed=false;
 const drain=new AbortController(),override=(name:string,fallback:number):number=>Number(process.env[name])||fallback,restart=sidecarRestartPolicyForV1(role);
 const restarting=createSidecarRestartV1({role,watch,work,policy:{quietMilliseconds:override("SEMIO_SIDECAR_QUIET_MILLISECONDS",restart.quietMilliseconds),maximumDeferralMilliseconds:override("SEMIO_SIDECAR_MAXIMUM_DEFERRAL_MILLISECONDS",restart.maximumDeferralMilliseconds),graceMilliseconds:override("SEMIO_SIDECAR_GRACE_MILLISECONDS",restart.graceMilliseconds),serveDeferralMilliseconds:override("SEMIO_SIDECAR_SERVE_DEFERRAL_MILLISECONDS",restart.serveDeferralMilliseconds),progressStallMilliseconds:override("SEMIO_SIDECAR_PROGRESS_STALL_MILLISECONDS",restart.progressStallMilliseconds),minimumIntervalMilliseconds:override("SEMIO_SIDECAR_MINIMUM_INTERVAL_MILLISECONDS",restart.minimumIntervalMilliseconds),drainMaximumMilliseconds:override("SEMIO_SIDECAR_DRAIN_MAXIMUM_MILLISECONDS",sidecarRestartPolicyV1.agent.drainMaximumMilliseconds),drainPollMilliseconds:override("SEMIO_SIDECAR_DRAIN_POLL_MILLISECONDS",sidecarRestartPolicyV1.agent.drainPollMilliseconds)},begin:()=>{changed=true;},drain:()=>{drain.abort();},abort:()=>{if(sidecarHandsOverV1(role,process.send!==undefined,process.env)){try{process.send?.({type:"successor"});setTimeout(()=>{controller.abort();},REMOTE_EXECUTION_SIDECAR_RESTART_V1.handoverMilliseconds+30_000).unref();return;}catch{}}controller.abort();},write:line=>{process.stderr.write(`${new Date().toISOString()} ${line}\n`);}});
 setInterval(()=>{restarting.tick();},override("SEMIO_SIDECAR_WATCH_MILLISECONDS",restart.watchMilliseconds)).unref();
 if(role!=="pool")for(const name of ["SIGTERM","SIGINT"] as const)process.on(name,()=>{controller.abort();});
 setInterval(()=>{if(process.ppid!==launcher)role==="pool"?process.exit(0):controller.abort();},2000).unref();
 const root=process.env.SEMIO_FLEET_ROOT??process.cwd();
 const recorder=createSidecarCodeRecorderV1({root,role}),publish=():void=>{try{recorder.record(watch.baseline,watch.closure().digest);}catch{}};
 publish();setInterval(publish,override("SEMIO_SIDECAR_WATCH_MILLISECONDS",restart.watchMilliseconds)).unref();
 process.stderr.write(`${new Date().toISOString()} ${servicePriorityTextV1(applyServicePriorityV1(role))}\n`);
 startSidecarIdleCompactionV1({role,root,environment:process.env,signal:controller.signal,write:line=>{process.stderr.write(`${new Date().toISOString()} ${line}\n`);}});
 try{
  await runRemoteExecutionSidecarV1(role,{root,environment:process.env,signal:controller.signal,drain:drain.signal,work,write:line=>{process.stderr.write(`${new Date().toISOString()} ${line}\n`);}});
  process.exit(changed?75:0);
 }catch(error){
  process.stderr.write(`${new Date().toISOString()} ${role} stopped: ${(error as Error).message}\n`);
  process.exit(1);
 }
}
