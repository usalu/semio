import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,readdirSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync,mkdirSync} from "node:fs";
import {mkdir,readdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join,relative,resolve} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {isMainThread,Worker,workerData} from "node:worker_threads";
import {ownProcessIdentityV1,processHolderAliveV1,readProcessStartV1,type ProcessStartReadV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

type Check=(root:string,signal?:AbortSignal)=>Promise<{ok:boolean;error:string|null}>;
type Reject=(root:string,digest:string)=>Promise<string|null>;
type Digest=(root:string)=>Promise<string>;
type Sink=(line:string)=>void;
type Context=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;write:Sink;notice:Sink;readStdin?:()=>Promise<string>}>;
type Run=(args:readonly string[],context:Context)=>Promise<{exitCode:number}>;
type Seal=Readonly<{digest:string;createdMs:number;files:readonly Readonly<{path:string;bytes:number;sha256:string}>[]}>;
type Snapshot=Readonly<{digest:string;createdMs:number;directory:string}>;
type Offer=Snapshot&Readonly<{healthy:number}>;
const provenOriginsV1=2;
type Outcome=Readonly<{kind:"exit";code:number}>|Readonly<{kind:"early";code:number;reason:string}>;
type Watch=Readonly<{shared:SharedArrayBuffer;beat:string;health:string;parent:number;parentStart:ProcessStartReadV1;startedMs:number;base:string;source:string;blockMs:number;beatMs:number}>;
type Beat=Readonly<{pid?:number;wallMs?:number;phase?:string;lagMs?:number;activity?:string;longestMs?:number;longestActivity?:string}>;

/** 📐️ The limits of the boot stage in milliseconds: a start counts as early while it has not reached its ready state or ran for less than `earlyMs`; a daemon that is not ready after `readyMs` is killed; a block of the event loop is recorded as a failure after `blockMs` and ends the process after `killMs` (120 s) once the daemon runs (the boot and open phases are bounded by `readyMs` instead), with a `loop-blocked-restart` record and a stack sample taken first; a stopping daemon gets `graceMs` before it is killed; the watchdog writes its beat every `beatMs`; fetching from the store gives up after `storeMs`. `SEMIO_FLEET_BOOT_<NAME>` overrides each one (`EARLY_MS`, `READY_MS`, `BLOCK_MS`, `KILL_MS`, `GRACE_MS`, `BEAT_MS`, `STORE_MS`). */
export type BootLimitsV1=Readonly<{earlyMs:number;readyMs:number;blockMs:number;killMs:number;graceMs:number;beatMs:number;storeMs:number}>;

/** 🧭️ Where the module lives below the repository root; the stable default of every worktree. */
export const bootModuleRelativeV1="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";

const phases=["boot","open","run"] as const;
const activities=["idle","open","follow","publish","walk","hot","sweep","file","attest","sidecars","health","tend","maintain","compact","git","drift","hash","journal","persist","settle"] as const;
const restartFile=(root:string):string=>join(bootDirectory(root),"loop-blocked-restart.json");
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const text=(error:unknown):string=>error instanceof Error?error.message:String(error);
const bootDirectory=(root:string):string=>join(root,".cache","f","boot");
const codeDirectory=(root:string):string=>join(root,".cache","f","code");
const readJson=(file:string):unknown=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
const writeAtomic=(file:string,body:string):void=>{mkdirSync(dirname(file),{recursive:true});const temporary=`${file}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`;try{writeFileSync(temporary,body);renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}};
const parentLives=(pid:number,start:ProcessStartReadV1):boolean=>start==="gone"?false:start==="unknown"?true:processHolderAliveV1({pid,startMs:start});
const timed=<T>(work:Promise<T>,milliseconds:number):Promise<T>=>new Promise<T>((done,fail)=>{const timer=setTimeout(()=>{fail(Error(`store did not answer within ${milliseconds} ms`));},milliseconds);work.then(value=>{clearTimeout(timer);done(value);},error=>{clearTimeout(timer);fail(error);});});

/** ⏱️ Reads the limits from the environment. */
export function bootLimitsV1(environment:Readonly<Record<string,string|undefined>>):BootLimitsV1{
 const number=(name:string,fallback:number):number=>{const raw=environment[`SEMIO_FLEET_BOOT_${name}`],value=raw===undefined||raw===""?NaN:Number(raw);return Number.isFinite(value)&&value>=0?value:fallback;};
 return {earlyMs:number("EARLY_MS",60_000),readyMs:number("READY_MS",900_000),blockMs:number("BLOCK_MS",60_000),killMs:number("KILL_MS",120_000),graceMs:number("GRACE_MS",20_000),beatMs:number("BEAT_MS",1000),storeMs:number("STORE_MS",20_000)};
}

function refusedOf(root:string):Set<string>{
 const value=readJson(join(codeDirectory(root),"history.json")) as {refused?:unknown}|undefined;
 return new Set(Array.isArray(value?.refused)?value.refused.filter((item):item is string=>typeof item==="string"):[]);
}

function refuse(root:string,digest:string):void{
 const file=join(codeDirectory(root),"history.json"),value=readJson(file) as {runs?:unknown;refused?:unknown}|undefined,refused=[...refusedOf(root)];
 if(refused.includes(digest))return;
 try{writeAtomic(file,JSON.stringify({version:1,runs:Array.isArray(value?.runs)?value.runs:[],refused:[...refused,digest].slice(-64)}));}catch{}
}

function intact(directory:string):Seal|undefined{
 try{
  const seal=JSON.parse(readFileSync(join(directory,"sealed.json"),"utf8")) as Seal;
  if(!Array.isArray(seal.files)||seal.files.length===0)return undefined;
  for(const file of seal.files)if(sha(readFileSync(join(directory,"files",...file.path.split("/"))))!==file.sha256)return undefined;
  return seal;
 }catch{return undefined;}
}

function snapshotsOf(root:string):Snapshot[]{
 const base=codeDirectory(root),refused=refusedOf(root),found:Snapshot[]=[];
 try{for(const entry of readdirSync(base,{withFileTypes:true})){
  if(!entry.isDirectory()||!/^[0-9a-f]{64}$/u.test(entry.name)||refused.has(entry.name))continue;
  const seal=readJson(join(base,entry.name,"sealed.json")) as {createdMs?:unknown}|undefined;
  if(seal!==undefined)found.push({digest:entry.name,createdMs:Number(seal.createdMs)||0,directory:join(base,entry.name)});
 }}catch{}
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

async function storeOffers(root:string,limits:BootLimitsV1,now:number):Promise<Readonly<{store:string;offers:Offer[]}>|undefined>{
 const machine=readJson(join(root,".cache","f","machine.json")) as {storeRoot?:unknown}|undefined;
 if(typeof machine?.storeRoot!=="string")return undefined;
 const store=machine.storeRoot,refused=refusedOf(root);
 try{
  const names=(await timed(readdir(join(store,"v1","bootstrap","code")),limits.storeMs)).filter(name=>/^[0-9a-f]{64}$/u.test(name)&&!refused.has(name)),offers:Offer[]=[],attested=new Map<string,number>();
  for(const origin of await timed(readdir(join(store,"v1","origins")),limits.storeMs).catch(()=>[] as string[])){
   try{const value=JSON.parse(await timed(readFile(join(store,"v1","origins",origin,"attest.json"),"utf8"),limits.storeMs)) as {code?:unknown;wallMs?:unknown};if(typeof value.code==="string"&&typeof value.wallMs==="number"&&now-value.wallMs<600_000)attested.set(value.code,(attested.get(value.code)??0)+1);}catch{}
  }
  for(const name of names){
   try{const seal=JSON.parse(await timed(readFile(join(store,"v1","bootstrap","code",name,"sealed.json"),"utf8"),limits.storeMs)) as Seal;if(seal.digest===name)offers.push({digest:name,createdMs:Number(seal.createdMs)||0,directory:join(store,"v1","bootstrap","code",name),healthy:attested.get(name)??0});}catch{}
  }
  return {store,offers:offers.sort((left,right)=>(attested.get(right.digest)??0)-(attested.get(left.digest)??0)||right.createdMs-left.createdMs||(left.digest<right.digest?-1:1))};
 }catch{return undefined;}
}

async function install(root:string,offer:Snapshot,limits:BootLimitsV1):Promise<string|undefined>{
 const target=join(codeDirectory(root),offer.digest);
 if(intact(target)!==undefined)return target;
 const temporary=join(codeDirectory(root),`.tmp-${randomBytes(8).toString("hex")}`);
 try{
  const sealText=await timed(readFile(join(offer.directory,"sealed.json"),"utf8"),limits.storeMs),seal=JSON.parse(sealText) as Seal;
  if(seal.digest!==offer.digest||!Array.isArray(seal.files)||seal.files.length===0)throw Error("bad seal");
  for(const file of seal.files){
   if(file.path.split("/").some((part:string)=>part===".."||part===""))throw Error(`bad path ${file.path}`);
   const data=await timed(readFile(join(offer.directory,"files",...file.path.split("/"))),limits.storeMs);
   if(data.length!==file.bytes||sha(data)!==file.sha256)throw Error(`changed ${file.path}`);
   const destination=join(temporary,"files",...file.path.split("/"));
   await mkdir(dirname(destination),{recursive:true});await writeFile(destination,data);
  }
  await writeFile(join(temporary,"sealed.json"),sealText);
  await rm(target,{recursive:true,force:true});
  await rename(temporary,target);
  return target;
 }catch{await rm(temporary,{recursive:true,force:true}).catch(()=>{});return undefined;}
}

const phaseIndex=(shared:Float64Array,phase:"boot"|"open"|"run"):void=>{shared[1]=phases.indexOf(phase);};

function watchdogLoop(config:Watch):void{
 const shared=new Float64Array(config.shared);
 let blocked=false,longest=0,longestActivity="idle";
 const record=():void=>{
  try{
   const value=JSON.parse(readFileSync(config.health,"utf8")) as {failures?:{kind:string;count:number;lastMs:number}[]};
   const failures=Array.isArray(value.failures)?value.failures:[],previous=failures.find(item=>item.kind==="loop-blocked");
   writeAtomic(config.health,JSON.stringify({...value,failures:[...failures.filter(item=>item.kind!=="loop-blocked"),{kind:"loop-blocked",count:(previous?.count??0)+1,lastMs:Date.now()}].slice(-8)}));
  }catch{}
 };
 const tick=():void=>{
  const now=Date.now(),lag=Math.max(0,now-shared[0]!),activity=activities[shared[2]!]??"idle";
  if(lag>longest){longest=lag;longestActivity=activity;}
  if(!parentLives(config.parent,config.parentStart))process.kill(process.pid,"SIGKILL");
  try{writeAtomic(config.beat,JSON.stringify({version:1,...ownProcessIdentityV1(),parent:config.parent,startedMs:config.startedMs,wallMs:now,phase:phases[shared[1]!]??"boot",base:config.base,source:config.source,lagMs:lag,activity,longestMs:Math.round(longest),longestActivity}));}catch{}
  if(lag>=config.blockMs&&!blocked){blocked=true;record();}
  else if(lag<2000)blocked=false;
 };
 setInterval(tick,config.beatMs);
 tick();
}

if(!isMainThread&&(workerData as {semioBoot?:Watch}|null)?.semioBoot!==undefined)watchdogLoop((workerData as {semioBoot:Watch}).semioBoot);

type Dog=Readonly<{phase:(phase:"boot"|"open"|"run")=>void}>;

function startWatchdog(root:string,base:string,source:string,limits:BootLimitsV1):Dog{
 const shared=new SharedArrayBuffer(24),view=new Float64Array(shared),record=Symbol.for("semio.fleet.blocked");
 view[0]=Date.now();
 const holder=globalThis as unknown as Record<symbol,unknown>,earlier=readJson(restartFile(root)) as {count?:unknown;wallMs?:unknown}|undefined;
 holder[Symbol.for("semio.fleet.activity")]=(name:string):void=>{const index=activities.indexOf(name as never);if(index>=0)view[2]=index;};
 if(typeof earlier?.count==="number"&&typeof earlier.wallMs==="number")holder[Symbol.for("semio.fleet.blockedRestart")]={count:earlier.count,lastMs:earlier.wallMs};
 const config:Watch={shared,beat:join(bootDirectory(root),"beat.json"),health:join(root,".cache","f","health.json"),parent:process.ppid,parentStart:readProcessStartV1(process.ppid),startedMs:Date.now(),base,source,blockMs:limits.blockMs,beatMs:limits.beatMs};
 try{const worker=new Worker(fileURLToPath(import.meta.url),{workerData:{semioBoot:config}});worker.unref();worker.on("error",()=>{});}catch{}
 let last=Date.now();
 setInterval(()=>{
  const now=Date.now(),late=now-last-250;
  last=now;view[0]=now;
  if(late>=limits.blockMs){const holder=globalThis as unknown as Record<symbol,{count:number;lastMs:number;maxMs:number}|undefined>,found=holder[record]??{count:0,lastMs:0,maxMs:0};holder[record]={count:found.count+1,lastMs:now,maxMs:Math.max(found.maxMs,late)};}
 },250).unref();
 return {phase:phase=>{phaseIndex(view,phase);}};
}

type Attempt=Readonly<{ran:true;code:number}>|Readonly<{ran:false;rejected:boolean;reason:string}>;

async function attempt(args:readonly string[],root:string,module:string,base:string,context:Context,options:Readonly<{worktree:boolean;force:boolean;phase?:(phase:"boot"|"open"|"run")=>void;watch?:string}>):Promise<Attempt>{
 const inside=relative(root,module);
 let run:Run;
 try{
  const directory=join(base,inside),load=async(...path:string[]):Promise<Record<string,unknown>>=>await import(pathToFileURL(join(directory,...path)).href) as Record<string,unknown>;
  run=(await load("🔁️replica","⌨️command","🟦️.ts")).runReplicaCommandV1 as Run;
  if(options.worktree){
   const report=await ((await load("🛡️resilience","🟦️.ts")).selfCheckV1 as Check)(directory,context.signal);
   if(!report.ok)throw Error(report.error??"self-check failed");
   if(args[0]!=="self-check"&&!options.force){
    const why=await ((await load("🛡️resilience","🟦️.ts")).startRejectionV1 as Reject)(root,await ((await load("🩺️attest","🟦️.ts")).codeDigestV1 as Digest)(directory));
    if(why!==null)return {ran:false,rejected:true,reason:`the code ${why}`};
   }
  }
 }catch(error){return {ran:false,rejected:false,reason:text(error)};}
 options.phase?.("open");
 const environment=options.watch===undefined?context.environment:{...context.environment,SEMIO_FLEET_CODE_WATCH:options.watch};
 try{return {ran:true,code:(await run(args,{...context,environment})).exitCode};}
 catch(error){context.notice(text(error));return {ran:true,code:1};}
}

async function inProcess(args:readonly string[],root:string,module:string,context:Context):Promise<number>{
 const strict=args[0]==="self-check";
 let first=true,lastResort:(()=>Promise<number>)|undefined;
 for(const snapshot of strict?[]:[undefined,...snapshotsOf(root)]){
  const base=snapshot===undefined?root:join(snapshot.directory,"files");
  if(snapshot!==undefined&&intact(snapshot.directory)===undefined)continue;
  const result=await attempt(args,root,module,base,context,{worktree:snapshot===undefined,force:false,watch:snapshot===undefined?undefined:module});
  if(result.ran){if(snapshot!==undefined)context.notice(`Ran the sealed code ${base} until the worktree code starts again / Der versiegelte Code ${base} lief, bis der Code des Arbeitsverzeichnisses wieder startet`);return result.code;}
  if(result.rejected){lastResort=async()=>{const again=await attempt(args,root,module,root,context,{worktree:false,force:true});return again.ran?again.code:1;};}
  context.notice(`${first?"The code of the worktree":`The sealed code ${base}`} does not start (${result.reason}) / Der Code ${first?"des Arbeitsverzeichnisses":`des Snapshots ${base}`} startet nicht (${result.reason})`);
  first=false;
 }
 if(strict){
  const result=await attempt(args,root,module,root,context,{worktree:true,force:true});
  if(result.ran)return result.code;
  context.notice(result.reason);return 1;
 }
 if(lastResort!==undefined){context.notice("No sealed code starts either, running the worktree code after all / Auch kein versiegelter Code startet, der Code des Arbeitsverzeichnisses läuft trotzdem");return lastResort();}
 context.notice("No replica code starts: neither the worktree nor a sealed snapshot / Kein Replica-Code startet: weder das Arbeitsverzeichnis noch ein versiegelter Snapshot");
 return 1;
}

async function child(args:readonly string[],root:string,module:string,context:Context):Promise<number>{
 const {SEMIO_FLEET_BOOT_RELOAD_TOKEN:token,...environment}=context.environment;
 delete process.env.SEMIO_FLEET_BOOT_RELOAD_TOKEN;
 const limits=bootLimitsV1(environment),base=environment.SEMIO_FLEET_BOOT_BASE??root,source=environment.SEMIO_FLEET_BOOT_SOURCE??"worktree",worktree=resolve(base)===resolve(root);
 const dog=startWatchdog(root,base,source,limits);
 let ready=false,reload=false,progressExit:ReturnType<typeof setTimeout>|undefined;
 const owned=globalThis as unknown as Record<symbol,unknown>,progress=Symbol.for("semio.fleet.gitProgressRestart"),parent=process.ppid,parentStart=readProcessStartV1(parent);
 const armProgressExit=():boolean=>{
  if(!ready||context.signal.aborted||progressExit!==undefined||process.ppid!==parent||!parentLives(parent,parentStart))return false;
  progressExit=setTimeout(()=>process.exit(1),limits.graceMs);
  return true;
 };
 if(typeof token==="string"&&/^[a-f0-9]{32}$/u.test(token))owned[progress]=armProgressExit;
 const write:Sink=line=>{
  const started=line.startsWith("{\"version\":1,\"event\":\"started\""),waiting=!started&&line.startsWith("{\"version\":1,\"event\":\"blocked\"")&&line.includes("worktree-access-denied");
  if(started){ready=true;dog.phase("run");}
  if(line==='{"version":1,"event":"restarting"}')reload=true;
  if(started||waiting){try{writeAtomic(join(bootDirectory(root),"ready.json"),JSON.stringify({pid:process.pid,waiting}));}catch{}}
  context.write(line);
 };
 const fallback=worktree?undefined:JSON.stringify({guard:"code-fallback",source,reason:(environment.SEMIO_FLEET_BOOT_REASON??"").slice(0,512)});
 const result=await attempt(args,root,module,base,{...context,write,environment:fallback===undefined?environment:{...environment,SEMIO_FLEET_CODE_FALLBACK:fallback}},{worktree,force:environment.SEMIO_FLEET_BOOT_FORCE==="1",phase:dog.phase,watch:worktree?undefined:module}).finally(()=>{
  clearTimeout(progressExit);if(owned[progress]===armProgressExit)delete owned[progress];
 });
 if(result.ran){
  if(result.code===0&&ready&&reload&&!context.signal.aborted&&typeof token==="string"&&/^[a-f0-9]{32}$/u.test(token)){
   try{writeAtomic(join(bootDirectory(root),"ready.json"),JSON.stringify({pid:process.pid,waiting:false,reloadToken:token}));}
   catch{context.notice("The daemon could not record its graceful reload receipt / Der Daemon konnte sein geordnetes Neuladen nicht belegen");}
  }
  return result.code;
 }
 context.notice(`The code ${worktree?"of the worktree":`of ${base}`} does not start (${result.reason}) / Der Code ${worktree?"des Arbeitsverzeichnisses":`von ${base}`} startet nicht (${result.reason})`);
 return result.rejected?71:70;
}

function sampled(pid:number,file:string,milliseconds:number):Promise<boolean>{
 return new Promise<boolean>(done=>{
  if(process.platform!=="darwin"){done(false);return;}
  let finished=false,timer:ReturnType<typeof setTimeout>|undefined,proc:ReturnType<typeof spawn>;
  const finish=(ok:boolean):void=>{if(finished)return;finished=true;clearTimeout(timer);done(ok);};
  try{proc=spawn("sample",[String(pid),"3","-file",file],{stdio:"ignore",windowsHide:true});}catch{done(false);return;}
  timer=setTimeout(()=>{try{proc.kill("SIGKILL");}catch{}finish(false);},milliseconds);
  proc.on("error",()=>{finish(false);});
  proc.on("close",code=>{finish(code===0&&existsSync(file));});
 });
}

async function recordBlock(root:string,pid:number,beat:Beat,reason:"blocked"|"silent"):Promise<void>{
 const previous=readJson(restartFile(root)) as {count?:unknown}|undefined,count=(typeof previous?.count==="number"?previous.count:0)+1,name=`loop-blocked-${pid}.sample.txt`;
 const write=(sample:string|null):void=>{try{writeAtomic(restartFile(root),JSON.stringify({version:1,count,pid,wallMs:Date.now(),lagMs:beat.lagMs??0,phase:beat.phase??"run",reason,activity:beat.activity??null,sample,...beat.longestMs===undefined?{}:{longestMs:beat.longestMs,longestActivity:beat.longestActivity??null}}));}catch{}};
 write(null);
 if(await sampled(pid,join(bootDirectory(root),name),10_000))write(name);
 try{for(const stale of readdirSync(bootDirectory(root)).filter(item=>/^loop-blocked-\d+\.sample\.txt$/u.test(item)&&item!==name).sort().slice(0,-2))rmSync(join(bootDirectory(root),stale),{force:true});}catch{}
}

type SilenceModuleV1={readySidecarSilenceV1(input:{status:unknown;nowMs:number}):{state:string;missedBeats:number;ageMs:number};readySilenceWatchV1():{observe(status:unknown,nowMs:number):"wait"|"restart"};readySilenceNoticeV1(silence:{state:"silent";missedBeats:number;ageMs:number}):string};
const silenceModuleRelativeV1=["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🌐️remote-execution","🛰️sidecar","🚦️ready","💔️silence","🟦️.ts"];
const priorityModuleRelativeV1=["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🌐️remote-execution","🛰️sidecar","🪜️priority","🟦️.ts"];

/** 🏎️ Puts this process (the boot stage or the daemon child, whose watchdog thread shares its priority) into the declared class of the `replica` service before it runs, so a service manager that starts it lower (a Windows task without `<Priority>` runs below normal) cannot starve its event loop and its beat behind other work on a saturated machine; reports the outcome on `notice` and never throws. See `../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🪜️priority/🟦️.ts`. */
async function applyReplicaPriorityV1(root:string,notice:Sink):Promise<void>{
 try{
  const module=await import(pathToFileURL(join(root,...priorityModuleRelativeV1)).href) as {applyServiceUnitPriorityV1(service:string):unknown;servicePriorityTextV1(result:unknown):string};
  notice(module.servicePriorityTextV1(module.applyServiceUnitPriorityV1("replica")));
 }catch(error){notice(`replica scheduling: not applied (${text(error)}) / replica Scheduling: nicht angewendet (${text(error)})`);}
}

/** 💔️ The `ready.json` of this machine's readiness sidecar (`<store>/v1/origins/<origin>/ready.json`), `undefined` without a fleet store. */
function readyStatusFileV1(root:string,environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const store=environment.SEMIO_FLEET_STORE_ROOT;
 if(store===undefined||store==="")return undefined;
 const explicit=environment.SEMIO_FLEET_ORIGIN,recorded=(readJson(join(root,".cache","f","machine.json")) as {originId?:unknown}|undefined)?.originId,origin=explicit!==undefined&&explicit!==""?explicit:typeof recorded==="string"?recorded:undefined;
 return origin===undefined||!/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(origin)?undefined:join(store,"v1","origins",origin,"ready.json");
}

function candidate(args:readonly string[],root:string,environment:Readonly<Record<string,string|undefined>>,limits:BootLimitsV1,signal:AbortSignal,notice:Sink,base:string,source:"worktree"|"snapshot"|"store",reason:string,force:boolean):Promise<Outcome>{
 return new Promise<Outcome>(done=>{
  const startedAt=Date.now(),beatFile=join(bootDirectory(root),"beat.json"),readyFile=join(bootDirectory(root),"ready.json");
  let ready=false,waiting=false,stopping=false,finished=false,killed="",termTimer:ReturnType<typeof setTimeout>|undefined;
  const proc=spawn(process.execPath,[fileURLToPath(import.meta.url),"--boot-child",...args],{env:{...environment,SEMIO_FLEET_BOOT_BASE:base,SEMIO_FLEET_BOOT_SOURCE:source,SEMIO_FLEET_BOOT_REASON:reason,SEMIO_FLEET_BOOT_FORCE:force?"1":""} as NodeJS.ProcessEnv,stdio:"inherit",windowsHide:true});
  let recording=false,ownedBeat:Beat|undefined,ownedBeatAt=startedAt,silenceRule:SilenceModuleV1|undefined,silenceWatch:ReturnType<SilenceModuleV1["readySilenceWatchV1"]>|undefined;
  const statusFile=readyStatusFileV1(root,environment);
  if(statusFile!==undefined)void import(pathToFileURL(join(root,...silenceModuleRelativeV1)).href).then(module=>{silenceRule=module as SilenceModuleV1;silenceWatch=silenceRule.readySilenceWatchV1();}).catch(()=>undefined);
  const pid=proc.pid??0,kill=(why:string,beat?:Beat,reason?:"blocked"|"silent"):void=>{
   killed=killed||why;
   if(beat!==undefined&&reason!==undefined&&beat.phase==="run"){
    if(recording)return;
    recording=true;
    void recordBlock(root,pid,beat,reason).catch(()=>{}).finally(()=>{if(!finished&&!stopping)try{proc.kill("SIGKILL");}catch{}});
    return;
   }
   try{proc.kill("SIGKILL");}catch{}
  };
  const watch=setInterval(()=>{
   const seen=readJson(beatFile) as Beat|undefined,now=Date.now();
   const marker=readJson(readyFile) as {pid?:number;waiting?:boolean}|undefined;
   if(marker?.pid===pid){ready=marker.waiting!==true;waiting=marker.waiting===true;}
   if(seen?.pid===pid&&typeof seen.wallMs==="number"&&Number.isFinite(seen.wallMs)&&seen.wallMs>=ownedBeatAt&&seen.wallMs<=now){ownedBeat=seen;ownedBeatAt=seen.wallMs;}
   const beat=ownedBeat;
   if(beat!==undefined){
    if(beat.phase==="run")ready=true;
    if(!stopping&&now-ownedBeatAt>30_000)kill("the watchdog of the daemon fell silent",beat,"silent");
    else if(!stopping&&beat.phase==="run"&&(beat.lagMs??0)>=limits.killMs)kill(`the event loop was blocked for ${Math.round((beat.lagMs??0)/1000)} s${beat.activity===undefined||beat.activity==="idle"?"":` in ${beat.activity}`}`,beat,"blocked");
   }
   if(statusFile!==undefined&&silenceRule!==undefined&&silenceWatch!==undefined&&ready&&!stopping){
    const status=readJson(statusFile);
    if(silenceWatch.observe(status,now)==="restart"){const silence=silenceRule.readySidecarSilenceV1({status,nowMs:now});notice(silenceRule.readySilenceNoticeV1({state:"silent",missedBeats:silence.missedBeats,ageMs:silence.ageMs}));kill("the readiness sidecar fell silent");}
   }
   if(!ready&&!waiting&&now-startedAt>limits.readyMs&&!stopping)kill(`the daemon was not ready after ${Math.round(limits.readyMs/1000)} s`);
  },Math.max(20,Math.min(1000,limits.beatMs)));
  const stop=():void=>{
   if(stopping||finished)return;
   stopping=true;
   try{proc.kill("SIGTERM");}catch{}
   termTimer=setTimeout(()=>{kill("did not stop within the grace period");},limits.graceMs);
  };
  if(signal.aborted)stop();else signal.addEventListener("abort",stop,{once:true});
  const end=(outcome:Outcome):void=>{if(finished)return;finished=true;clearInterval(watch);clearTimeout(termTimer);signal.removeEventListener("abort",stop);rmSync(beatFile,{force:true});rmSync(readyFile,{force:true});done(outcome);};
  proc.on("error",error=>{end({kind:"early",code:1,reason:`could not start: ${error.message}`});});
  proc.on("exit",(code,name)=>{
   const status=code??(name===null?1:128+({SIGTERM:15,SIGINT:2,SIGKILL:9,SIGHUP:1} as Record<string,number>)[name]!||1),uptime=Date.now()-startedAt;
   if(stopping)return end({kind:"exit",code:killed===""?code??0:status});
   if(code===0||code===75)return end({kind:"exit",code:code});
   if(!ready||uptime<limits.earlyMs){notice(`The daemon of ${source==="worktree"?"the worktree":base} ended after ${Math.round(uptime/1000)} s with ${name??`exit code ${code}`}${killed===""?"":` (${killed})`} / Der Daemon von ${source==="worktree"?"dem Arbeitsverzeichnis":base} endete nach ${Math.round(uptime/1000)} s mit ${name??`Exit-Code ${code}`}`);return end({kind:"early",code:status,reason:killed===""?`exit ${status}`:killed});}
   notice(`The daemon ended with ${name??`exit code ${code}`}${killed===""?"":` (${killed})`} / Der Daemon endete mit ${name??`Exit-Code ${code}`}`);
   return end({kind:"exit",code:status});
  });
 });
}

async function supervise(args:readonly string[],root:string,module:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,notice:Sink):Promise<number>{
 const limits=bootLimitsV1(environment);
 let rejected=false,reason="";
 const first=await candidate(args,root,environment,limits,signal,notice,root,"worktree","",false);
 if(first.kind==="exit")return first.code;
 rejected=first.code===71;reason=first.reason;
 const why=rejected?"the code on disk ran before and was superseded":`the worktree code does not start: ${reason}`;
 const tried=new Set<string>();
 const run=async(directory:string,digest:string,source:"snapshot"|"store"):Promise<number|undefined>=>{
  tried.add(digest);
  notice(`Running the sealed code ${directory} until the worktree code starts again / Der versiegelte Code ${directory} läuft, bis der Code des Arbeitsverzeichnisses wieder startet`);
  const outcome=await candidate(args,root,{...environment,SEMIO_FLEET_CODE_WATCH:module},limits,signal,notice,join(directory,"files"),source,why,false);
  if(outcome.kind==="exit")return outcome.code;
  refuse(root,digest);
  return undefined;
 };
 const offered=await storeOffers(root,limits,Date.now());
 const fetchOffer=async(offer:Offer):Promise<number|undefined>=>{
  if(signal.aborted)return 143;
  if(tried.has(offer.digest))return undefined;
  const directory=await install(root,offer,limits);
  if(directory===undefined){notice(`The code ${offer.digest.slice(0,12)} of the store could not be fetched / Der Code ${offer.digest.slice(0,12)} des Stores konnte nicht geholt werden`);return undefined;}
  return run(directory,offer.digest,"store");
 };
 if(offered!==undefined)for(const offer of offered.offers.filter(item=>item.healthy>=provenOriginsV1)){
  const code=await fetchOffer(offer);
  if(code!==undefined)return code;
 }
 for(const snapshot of snapshotsOf(root)){
  if(signal.aborted)return 143;
  if(tried.has(snapshot.digest))continue;
  if(intact(snapshot.directory)===undefined){notice(`The sealed code ${snapshot.directory} is damaged and skipped / Der versiegelte Code ${snapshot.directory} ist beschädigt und wird übersprungen`);continue;}
  const code=await run(snapshot.directory,snapshot.digest,"snapshot");
  if(code!==undefined)return code;
 }
 if(offered!==undefined)for(const offer of offered.offers){
  const code=await fetchOffer(offer);
  if(code!==undefined)return code;
 }
 if(rejected){
  notice("No sealed code starts either, running the worktree code after all / Auch kein versiegelter Code startet, der Code des Arbeitsverzeichnisses läuft trotzdem");
  const last=await candidate(args,root,environment,limits,signal,notice,root,"worktree","",true);
  return last.code;
 }
 notice("No replica code starts: neither the worktree, a sealed snapshot nor a snapshot of the store / Kein Replica-Code startet: weder das Arbeitsverzeichnis, ein versiegelter Snapshot noch ein Snapshot des Stores");
 await new Promise<void>(done=>{const timer=setTimeout(done,Math.min(5_000,limits.earlyMs));signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
 return 1;
}

const stageSignature=(root:string):string=>join(bootDirectory(root),"stage.json");

/** 🛟️ Runs one replica command against a repository root with explicit sinks: stdout lines and stderr lines; returns the process exit code. This file is the stable entry the service managers run, so it imports nothing but Node built-ins and keeps every other module behind a dynamic import. `run` is supervised: the daemon runs in a child process of the code of the worktree, and when that child ends within `earlyMs` (60 s) of its start or before it was ready (any import, self-check or state-open failure, a crash, a hang) the next code is tried: the newest sealed snapshot below `.cache/f/code/` that is intact and not refused, then a snapshot a healthy daemon published to `<store>/v1/bootstrap/code/` (fetched, hash-verified and sealed locally), and only as a last resort the worktree code that was refused as a revert. A machine on such a fallback code keeps replicating, reports the guard `code-fallback` and switches to the worktree code as soon as that passes the self-check. A watchdog thread in the child writes `.cache/f/boot/beat.json` and records an event loop blocked for `blockMs` as a failure; the supervisor ends a child blocked for `killMs`, forwards `SIGTERM` and kills a child that does not stop within `graceMs`. Every other command runs in this process, from the worktree code or the newest intact sealed snapshot that starts; `self-check` never falls back. */
export async function runReplicaScriptV1(args:readonly string[],root:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,write:Sink,notice:Sink,readStdin?:()=>Promise<string>,moduleRoot?:string):Promise<number>{
 const module=moduleRoot??(dirname(import.meta.dirname)===bootDirectory(root)||import.meta.dirname===bootDirectory(root)?join(root,...bootModuleRelativeV1.split("/")):resolve(import.meta.dirname,"..",".."));
 if(args[0]==="run")return supervise(args,root,module,environment,signal,notice);
 return inProcess(args,root,module,{root,environment,signal,write,notice,...readStdin===undefined?{}:{readStdin}});
}

/** 🪞️ The location of the verified copy of this file outside the module, which a healthy daemon refreshes (`.cache/f/boot/stage.ts`) and which the worktree file prefers: the copy ran on this machine before, the worktree file may have arrived a second ago as part of a mixed change set. */
export function bootCopyV1(root:string):Readonly<{script:string;signature:string}>{return {script:join(bootDirectory(root),"stage.ts"),signature:stageSignature(root)};}

async function delegate(root:string):Promise<((...parameters:Parameters<typeof runReplicaScriptV1>)=>Promise<number>)|undefined>{
 const {script,signature}=bootCopyV1(root);
 try{
  if(!existsSync(script)||import.meta.dirname===bootDirectory(root))return undefined;
  const expected=(readJson(signature) as {sha256?:unknown}|undefined)?.sha256;
  if(typeof expected!=="string"||sha(readFileSync(script))!==expected)return undefined;
  const loaded=await import(pathToFileURL(script).href) as {runReplicaScriptV1?:unknown};
  return typeof loaded.runReplicaScriptV1==="function"?loaded.runReplicaScriptV1 as never:undefined;
 }catch{return undefined;}
}

async function readStandardInput():Promise<string>{
 if(process.stdin.isTTY)return "";
 const chunks:Buffer[]=[];
 for await(const chunk of process.stdin)chunks.push(Buffer.from(chunk as Uint8Array));
 return Buffer.concat(chunks).toString("utf8");
}

const entry=process.argv[1],canonical=(path:string):string=>{const real=realpathSync(path);return process.platform==="win32"?real.toLowerCase():real;};

/** 🚪️ Runs the script `argv` (the arguments after the script name) with the process signals wired to an abort and the process streams as sinks, and returns the exit code. `📜️script.ts` (the trampoline the service managers run) calls it when no installed boot program takes over; the daemon child (`--boot-child`) and the copy in `.cache/f/boot/stage.ts` reach it through the same path. */
export async function scriptMainV1(argv:readonly string[]):Promise<number>{
 const controller=new AbortController(),stop=():void=>{controller.abort();},signals:NodeJS.Signals[]=process.platform==="win32"?["SIGINT","SIGTERM","SIGBREAK","SIGHUP"]:["SIGINT","SIGTERM"];
 for(const name of signals)process.once(name,stop);
 const staged=import.meta.dirname.endsWith(join(".cache","f","boot")),worktreeRoot=process.env.SEMIO_FLEET_BOOT_ROOT??(staged?resolve(import.meta.dirname,"../../.."):resolve(import.meta.dirname,"../../../../../../..")),out:Sink=line=>{process.stdout.write(`${line}\n`);},err:Sink=line=>{process.stderr.write(`${line}\n`);};
 const childMode=argv[0]==="--boot-child",args=argv.slice(childMode?1:0);
 if(args[0]==="run")await applyReplicaPriorityV1(worktreeRoot,err);
 const base=process.env.SEMIO_FLEET_BOOT_BASE,module=staged||(base!==undefined&&base!==""&&resolve(base)!==resolve(worktreeRoot))?join(worktreeRoot,...bootModuleRelativeV1.split("/")):resolve(import.meta.dirname,"..","..");
 let code:number;
 if(childMode){
  code=await child(args,worktreeRoot,module,{root:worktreeRoot,environment:process.env,signal:controller.signal,write:out,notice:err,readStdin:readStandardInput});
  setTimeout(()=>{process.exit(code);},3000).unref();
 }else{
  const run=args[0]==="run"?await delegate(worktreeRoot):undefined;
  code=await (run??runReplicaScriptV1)(args,worktreeRoot,process.env,controller.signal,out,err,readStandardInput,run===undefined?module:undefined);
 }
 for(const name of signals)process.removeListener(name,stop);
 return code;
}

if(isMainThread&&entry!==undefined&&entry!==""&&(()=>{try{return canonical(entry)===canonical(fileURLToPath(import.meta.url));}catch{return false;}})())process.exitCode=await scriptMainV1(process.argv.slice(2));
