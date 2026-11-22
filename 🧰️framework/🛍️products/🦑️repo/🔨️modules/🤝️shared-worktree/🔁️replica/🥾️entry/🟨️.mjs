#!/usr/bin/env bun
import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {promises as fs,chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,extname,join,posix,resolve,win32} from "node:path";
import {fileURLToPath} from "node:url";

/** 🔢️ Version of the installed boot program, which moves forward only after its own self-test. */
export const bootVersionV1=8;

/** 🧭️ Where the module lives below the repository root. */
export const moduleRelativeV1="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree";

const sha=data=>createHash("sha256").update(data).digest("hex");
const text=error=>error instanceof Error?error.message:String(error);
const readJson=file=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
const writeAtomic=(file,body)=>{mkdirSync(dirname(file),{recursive:true});const temporary=`${file}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`;try{writeFileSync(temporary,body);renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}};
const timed=(work,milliseconds)=>new Promise((done,fail)=>{const timer=setTimeout(()=>{fail(Error(`no answer within ${milliseconds} ms`));},milliseconds);work.then(value=>{clearTimeout(timer);done(value);},error=>{clearTimeout(timer);fail(error);});});
const pause=(milliseconds,signal)=>new Promise(done=>{if(signal.aborted)return done();const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();},timer=setTimeout(finish,milliseconds);signal.addEventListener("abort",finish,{once:true});});
const restartFile=root=>join(root,".cache","f","boot","loop-blocked-restart.json");
const installedNames=Object.freeze({".mjs":"boot.mjs",".sh":"boot.sh",".ps1":"boot.ps1"});

/** 📂️ The per-account directory of the installed boot program, outside every worktree and writable without privileges: `~/Library/Application Support/semio/fleet/entry` on macOS (not a folder that asks for consent, unlike Documents), `$XDG_DATA_HOME/semio/fleet/entry` (default `~/.local/share`) on Linux and `%LOCALAPPDATA%\semio\fleet\entry` on Windows. `SEMIO_FLEET_ENTRY_DIR` overrides it (tests). */
export function entryDirectoryV1(env=process.env,platform=process.platform,home=homedir()){
 const {join:combine}=platform==="win32"?win32:posix;
 return env.SEMIO_FLEET_ENTRY_DIR??(platform==="win32"?combine(env.LOCALAPPDATA??combine(home,"AppData","Local"),"semio","fleet","entry"):platform==="darwin"?combine(home,"Library","Application Support","semio","fleet","entry"):combine(env.XDG_DATA_HOME??combine(home,".local","share"),"semio","fleet","entry"));
}

/** ⏱️ The limits in milliseconds, each overridable by `SEMIO_FLEET_ENTRY_<NAME>`: a start is early while it has not reached its ready state or ran for less than `EARLY_MS`; a daemon not ready after `READY_MS` (which also bounds the boot and open phases), silent for `SILENT_MS` or, once it runs, blocked for `KILL_MS` is killed; a stopping daemon gets `GRACE_MS`; the watch polls every `POLL_MS`; with nothing runnable the program waits `BACKOFF_MS`, doubling up to `BACKOFF_MAX_MS`; the store answers within `STORE_MS`; a refused snapshot is skipped for `REFUSE_MS`. */
export function limitsV1(env=process.env){
 const number=(name,fallback)=>{const raw=env[`SEMIO_FLEET_ENTRY_${name}`],value=raw===undefined||raw===""?NaN:Number(raw);return Number.isFinite(value)&&value>=0?value:fallback;};
 return {earlyMs:number("EARLY_MS",60_000),readyMs:number("READY_MS",900_000),silentMs:number("SILENT_MS",30_000),killMs:number("KILL_MS",120_000),graceMs:number("GRACE_MS",20_000),pollMs:number("POLL_MS",1000),backoffMs:number("BACKOFF_MS",5000),backoffMaxMs:number("BACKOFF_MAX_MS",300_000),storeMs:number("STORE_MS",20_000),refuseMs:number("REFUSE_MS",3_600_000)};
}

function intact(directory){
 try{
  const seal=JSON.parse(readFileSync(join(directory,"sealed.json"),"utf8"));
  if(!Array.isArray(seal.files)||seal.files.length===0)return undefined;
  for(const file of seal.files)if(sha(readFileSync(join(directory,"files",...file.path.split("/"))))!==file.sha256)return undefined;
  return seal;
 }catch{return undefined;}
}

const refusedOf=(entry,now)=>{const value=readJson(join(entry,"refused.json"));return new Set(Object.entries(value?.digests??{}).filter(([,until])=>typeof until==="number"&&until>now).map(([digest])=>digest));};
function refuse(entry,digest,now,limits){
 const value=readJson(join(entry,"refused.json")),digests=Object.fromEntries(Object.entries(value?.digests??{}).filter(([,until])=>typeof until==="number"&&until>now));
 digests[digest]=now+limits.refuseMs;
 try{writeAtomic(join(entry,"refused.json"),JSON.stringify({version:1,digests}));}catch{}
}

function supervisorOf(base,moduleRel){
 const path=join(base,...moduleRel.split("/"),"🔁️replica","🛟️supervisor","🟦️.ts");
 return existsSync(path)?path:undefined;
}

function snapshotsOf(directories,entry,now){
 const refused=refusedOf(entry,now),found=new Map();
 for(const base of directories)try{
  for(const item of readdirSync(base,{withFileTypes:true})){
   if(!item.isDirectory()||!/^[0-9a-f]{64}$/u.test(item.name)||refused.has(item.name)||found.has(item.name))continue;
   const seal=readJson(join(base,item.name,"sealed.json"));
   if(seal!==undefined)found.set(item.name,{digest:item.name,createdMs:Number(seal.createdMs)||0,directory:join(base,item.name)});
  }
 }catch{}
 return [...found.values()].sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

async function storeOffers(store,entry,limits,now){
 try{
  const refused=refusedOf(entry,now),base=join(store,"v1","bootstrap","code"),attested=new Map(),offers=[];
  const names=(await timed(fs.readdir(base),limits.storeMs)).filter(name=>/^[0-9a-f]{64}$/u.test(name)&&!refused.has(name));
  for(const origin of await timed(fs.readdir(join(store,"v1","origins")),limits.storeMs).catch(()=>[])){
   try{const value=JSON.parse(await timed(fs.readFile(join(store,"v1","origins",origin,"attest.json"),"utf8"),limits.storeMs));if(typeof value.code==="string"&&typeof value.wallMs==="number"&&now-value.wallMs<600_000)attested.set(value.code,(attested.get(value.code)??0)+1);}catch{}
  }
  for(const name of names)try{const seal=JSON.parse(await timed(fs.readFile(join(base,name,"sealed.json"),"utf8"),limits.storeMs));if(seal.digest===name)offers.push({digest:name,createdMs:Number(seal.createdMs)||0,directory:join(base,name)});}catch{}
  return offers.sort((left,right)=>(attested.get(right.digest)??0)-(attested.get(left.digest)??0)||right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
 }catch{return [];}
}

async function fetchOffer(entry,offer,limits){
 const target=join(entry,"code",offer.digest);
 if(intact(target)!==undefined)return target;
 const temporary=join(entry,"code",`.tmp-${randomBytes(8).toString("hex")}`);
 try{
  const sealText=await timed(fs.readFile(join(offer.directory,"sealed.json"),"utf8"),limits.storeMs),seal=JSON.parse(sealText);
  if(seal.digest!==offer.digest||!Array.isArray(seal.files)||seal.files.length===0)throw Error("bad seal");
  for(const file of seal.files){
   if(file.path.split("/").some(part=>part===".."||part===""))throw Error(`bad path ${file.path}`);
   const data=await timed(fs.readFile(join(offer.directory,"files",...file.path.split("/"))),limits.storeMs);
   if(data.length!==file.bytes||sha(data)!==file.sha256)throw Error(`changed ${file.path}`);
   const destination=join(temporary,"files",...file.path.split("/"));
   await fs.mkdir(dirname(destination),{recursive:true});await fs.writeFile(destination,data);
  }
  await fs.writeFile(join(temporary,"sealed.json"),sealText);
  await fs.rm(target,{recursive:true,force:true});
  await fs.rename(temporary,target);
  return target;
 }catch{await fs.rm(temporary,{recursive:true,force:true}).catch(()=>{});return undefined;}
}

function pruneSnapshots(entry,keep){
 try{
  const base=join(entry,"code"),all=readdirSync(base,{withFileTypes:true}).filter(item=>item.isDirectory()&&/^[0-9a-f]{64}$/u.test(item.name)).map(item=>({name:item.name,time:statSync(join(base,item.name)).mtimeMs})).sort((left,right)=>right.time-left.time);
  for(const old of all.slice(keep))rmSync(join(base,old.name),{recursive:true,force:true});
  for(const item of readdirSync(base,{withFileTypes:true}))if(item.name.startsWith(".tmp-")&&Date.now()-statSync(join(base,item.name)).mtimeMs>3_600_000)rmSync(join(base,item.name),{recursive:true,force:true});
 }catch{}
}

function sampled(pid,file,milliseconds){
 return new Promise(done=>{
  if(process.platform!=="darwin"){done(false);return;}
  let finished=false,timer,proc;
  const finish=ok=>{if(finished)return;finished=true;clearTimeout(timer);done(ok);};
  try{proc=spawn("sample",[String(pid),"3","-file",file],{stdio:"ignore",windowsHide:true});}catch{done(false);return;}
  timer=setTimeout(()=>{try{proc.kill("SIGKILL");}catch{}finish(false);},milliseconds);
  proc.on("error",()=>{finish(false);});
  proc.on("close",code=>{finish(code===0&&existsSync(file));});
 });
}

async function recordBlock(root,pid,beat,reason){
 const boot=join(root,".cache","f","boot"),previous=readJson(restartFile(root)),count=(typeof previous?.count==="number"?previous.count:0)+1,name=`loop-blocked-${pid}.sample.txt`;
 const write=sample=>{try{writeAtomic(restartFile(root),JSON.stringify({version:1,count,pid,wallMs:Date.now(),lagMs:beat.lagMs??0,phase:beat.phase??"run",reason,activity:beat.activity??null,sample}));}catch{}};
 write(null);
 if(await sampled(pid,join(boot,name),10_000))write(name);
 try{for(const stale of readdirSync(boot).filter(item=>/^loop-blocked-\d+\.sample\.txt$/u.test(item)&&item!==name).sort().slice(0,-2))rmSync(join(boot,stale),{force:true});}catch{}
}

function launch(spec,context){
 const {root,moduleRel,environment,limits,signal,state,say}=context;
 return new Promise(done=>{
  const boot=join(root,".cache","f","boot"),beatFile=join(boot,"beat.json"),readyFile=join(boot,"ready.json"),startedAt=Date.now(),replica=join(root,...moduleRel.split("/"),"🔁️replica"),reloadToken=randomBytes(16).toString("hex");
  let ready=false,waiting=false,stopping=false,finished=false,recording=false,killed="",termTimer,ownedBeat,ownedBeatAt=startedAt;
  const proc=spawn(process.execPath,[spec.script,"--boot-child",...context.args],{cwd:existsSync(replica)?replica:root,env:{...environment,SEMIO_FLEET_ENTRY:"boot",SEMIO_FLEET_BOOT_ROOT:root,SEMIO_FLEET_BOOT_BASE:spec.base,SEMIO_FLEET_BOOT_SOURCE:spec.source,SEMIO_FLEET_BOOT_REASON:spec.reason,SEMIO_FLEET_BOOT_FORCE:spec.force?"1":"",SEMIO_FLEET_BOOT_RELOAD_TOKEN:reloadToken,...spec.watch===undefined?{}:{SEMIO_FLEET_CODE_WATCH:spec.watch}},stdio:"inherit",windowsHide:true});
  const pid=proc.pid??0,kill=(why,beat,reason)=>{
   killed=killed||why;
   if(beat!==undefined&&reason!==undefined&&beat.phase==="run"){
    if(recording)return;
    recording=true;
    recordBlock(root,pid,beat,reason).catch(()=>{}).finally(()=>{if(!finished&&!stopping)try{proc.kill("SIGKILL");}catch{}});
    return;
   }
   try{proc.kill("SIGKILL");}catch{}
  };
  const watch=setInterval(()=>{
   const seen=readJson(beatFile),marker=readJson(readyFile),now=Date.now();
   if(marker?.pid===pid){ready=marker.waiting!==true;waiting=marker.waiting===true;}
   if(seen?.pid===pid&&Number.isFinite(seen.wallMs)&&seen.wallMs>=ownedBeatAt&&seen.wallMs<=now){ownedBeat=seen;ownedBeatAt=seen.wallMs;}
   const beat=ownedBeat;
   if(beat!==undefined){
    if(beat.phase==="run")ready=true;
    if(!stopping&&now-ownedBeatAt>limits.silentMs)kill("the watchdog of the daemon fell silent",beat,"silent");
    else if(!stopping&&beat.phase==="run"&&(beat.lagMs??0)>=limits.killMs)kill(`the event loop was blocked for ${Math.round((beat.lagMs??0)/1000)} s${beat.activity===undefined||beat.activity==="idle"?"":` in ${beat.activity}`}`,beat,"blocked");
   }
   if((ready||waiting)&&state.get().phase==="starting")state.set({phase:waiting?"waiting":"running"});
   if(!ready&&!waiting&&now-startedAt>limits.readyMs&&!stopping)kill(`the daemon was not ready after ${Math.round(limits.readyMs/1000)} s`);
  },Math.max(10,Math.min(1000,limits.pollMs)));
  const stop=()=>{
   if(stopping||finished)return;
   stopping=true;
   try{proc.kill("SIGTERM");}catch{}
   termTimer=setTimeout(()=>{kill("did not stop within the grace period");},limits.graceMs);
  };
  if(signal.aborted)stop();else signal.addEventListener("abort",stop,{once:true});
  const end=outcome=>{if(finished)return;finished=true;clearInterval(watch);clearTimeout(termTimer);signal.removeEventListener("abort",stop);rmSync(beatFile,{force:true});rmSync(readyFile,{force:true});done(outcome);};
  proc.on("error",error=>{end({kind:"early",code:1,reason:`could not start: ${error.message}`});});
  proc.on("exit",(code,name)=>{
   const status=code??(name===null?1:128+({SIGTERM:15,SIGINT:2,SIGKILL:9,SIGHUP:1}[name]??0)||1),uptime=Date.now()-startedAt;
   if(stopping)return end({kind:"exit",code:killed===""?code??0:status});
   if(code===0){const receipt=readJson(readyFile);if(receipt?.pid===pid&&receipt.waiting===false&&receipt.reloadToken===reloadToken&&Object.keys(receipt).sort().join(",")==="pid,reloadToken,waiting")return end({kind:"reload"});}
   if(code===0||code===75)return end({kind:"exit",code});
   if(!ready||uptime<limits.earlyMs)return end({kind:"early",code:status,reason:killed===""?`exit ${status}`:killed,uptime});
   if(killed!=="")say(`The daemon was ended (${killed}) / Der Daemon wurde beendet (${killed})`);
   return end({kind:"exit",code:status,killed});
  });
 });
}

function stateOf(entry,startedMs){
 let value={version:1,pid:process.pid,startedMs,bootVersion:bootVersionV1,phase:"starting",source:null,digest:null,base:null,reason:"",attempts:[],nextMs:null,wallMs:Date.now()};
 const write=()=>{try{writeAtomic(join(entry,"state.json"),JSON.stringify(value));}catch{}};
 return {get:()=>value,set:patch=>{value={...value,...patch,wallMs:Date.now()};write();},attempt:record=>{value={...value,attempts:[...value.attempts,{...record,atMs:Date.now()}].slice(-8),wallMs:Date.now()};write();}};
}

async function cycle(context){
 const {entry,root,moduleRel,limits,signal,say,state}=context,module=join(root,...moduleRel.split("/")),now=()=>Date.now();
 if(!existsSync(root)){state.set({phase:"waiting",source:null,digest:null,base:null,reason:`the worktree ${root} does not exist`});say(`The worktree ${root} does not exist, waiting / Das Arbeitsverzeichnis ${root} existiert nicht, es wird gewartet`);return {kind:"none"};}
 let rejected=false,why="";
 const run=async(spec)=>{
  state.set({phase:"starting",source:spec.source,digest:spec.digest??null,base:spec.base,reason:spec.reason});
  const outcome=await launch(spec,context);
  state.attempt({source:spec.source,digest:spec.digest??null,outcome:outcome.kind==="exit"?`exit ${outcome.code}${outcome.killed?` (${outcome.killed})`:""}`:outcome.kind==="reload"?"graceful reload":`early: ${outcome.reason}`});
  return outcome;
 };
 const worktreeScript=supervisorOf(root,moduleRel);
 if(worktreeScript!==undefined){
  const first=await run({script:worktreeScript,base:root,source:"worktree",reason:"",force:false});
  if(first.kind!=="early")return first;
  rejected=first.code===71;why=first.reason;
  say(`The worktree code does not start (${first.reason}) / Der Code des Arbeitsverzeichnisses startet nicht (${first.reason})`);
 }else{why="the replica script is missing";say("The worktree has no replica script / Das Arbeitsverzeichnis hat kein Replica-Skript");}
 const reason=rejected?"the code on disk ran before and was superseded":`the worktree code does not start: ${why}`,tried=new Set();
 const fallback=async(directory,digest,source)=>{
  tried.add(digest);
  const script=supervisorOf(join(directory,"files"),moduleRel);
  if(script===undefined)return undefined;
  say(`Running the sealed code ${directory} until the worktree code starts again / Der versiegelte Code ${directory} läuft, bis der Code des Arbeitsverzeichnisses wieder startet`);
  const outcome=await run({script,base:join(directory,"files"),source,reason,force:false,watch:module,digest});
  if(outcome.kind!=="early")return outcome;
  refuse(entry,digest,now(),limits);
  return undefined;
 };
 for(const snapshot of snapshotsOf([join(root,".cache","f","code"),join(entry,"code")],entry,now())){
  if(signal.aborted)return {kind:"exit",code:143};
  if(intact(snapshot.directory)===undefined){say(`The sealed code ${snapshot.directory} is damaged and skipped / Der versiegelte Code ${snapshot.directory} ist beschädigt und wird übersprungen`);continue;}
  const outcome=await fallback(snapshot.directory,snapshot.digest,"snapshot");
  if(outcome!==undefined)return outcome;
 }
 const store=context.storeRoot();
 if(store!==undefined)for(const offer of await storeOffers(store,entry,limits,now())){
  if(signal.aborted)return {kind:"exit",code:143};
  if(tried.has(offer.digest))continue;
  const directory=await fetchOffer(entry,offer,limits);
  if(directory===undefined){say(`The code ${offer.digest.slice(0,12)} of the store could not be fetched or its hashes differ / Der Code ${offer.digest.slice(0,12)} des Stores konnte nicht geholt werden oder seine Hashes weichen ab`);continue;}
  const outcome=await fallback(directory,offer.digest,"store");
  if(outcome!==undefined){pruneSnapshots(entry,3);return outcome;}
 }
 if(rejected&&worktreeScript!==undefined){
  say("No sealed code starts either, running the worktree code after all / Auch kein versiegelter Code startet, der Code des Arbeitsverzeichnisses läuft trotzdem");
  const last=await run({script:worktreeScript,base:root,source:"worktree",reason:"",force:true});
  if(last.kind!=="early")return last;
 }
 state.set({reason:store===undefined&&!existsSync(join(root,".cache","f","machine.json"))?`${why}; the store is unknown`:why});
 return {kind:"none"};
}

/** 🥾️ The boot loop: it runs the daemon from the first code that starts, in this order, and never ends by itself. (1) The code of the worktree, as the child process of its supervisor file. (2) The newest sealed snapshot below `<worktree>/.cache/f/code` or `<entry>/code` that is intact (every file hash checked) and not refused. (3) A snapshot of `<store>/v1/bootstrap/code/<digest>/`, ranked by fresh attestations, fetched with a per-file sha256 check into `<entry>/code`. (4) The worktree code once more, forced, when it was refused only as a revert. A start that ends early (not ready or under `earlyMs`) with an error moves on to the next code and refuses a failing snapshot for `refuseMs`; a verified graceful reload starts a new worker under the same boot owner. Unmarked exit 0, exit 75 and actual stop end the owner, while a late crash is restarted with bounded delay. With nothing runnable the loop writes `state.json` (phase `exhausted`, reasons, attempts, the next try) and polls again after a doubling backoff. A missing worktree is the phase `waiting`. `context`: `entry`, `worktree`, `moduleRel`, `storeRoot` (function), `environment`, `limits`, `args`, `say`. */
export async function runBootV1(context,signal){
 const startedMs=Date.now(),state=stateOf(context.entry,startedMs),full={...context,root:context.worktree,signal,state};
 const beat=setInterval(()=>{state.set({});},10_000);beat.unref?.();
 try{
  let wait=context.limits.backoffMs,crashes=[];
  for(;;){
   if(signal.aborted)return 143;
   const result=await cycle(full);
   if(result.kind==="reload"){
    const delay=context.limits.backoffMs;
    state.set({phase:"starting",reason:"daemon requested a graceful reload",nextMs:Date.now()+delay});
    context.say("The daemon requested a graceful reload and will restart inside the running boot stage / Der Daemon hat ein geordnetes Neuladen angefordert und startet innerhalb der laufenden Boot-Stufe neu");
    await pause(delay,signal);
    continue;
   }
   if(result.kind==="exit"){
    if(result.code===0||result.code===75||signal.aborted)return result.code;
    const now=Date.now();
    crashes=[...crashes.filter(at=>now-at<600_000),now];
    const delay=Math.min(context.limits.backoffMs*2**(crashes.length-1),context.limits.backoffMaxMs);
    state.set({phase:"starting",reason:`daemon exit ${result.code}`,nextMs:now+delay});
    context.say(`The daemon ended with ${result.code}; the boot stage starts it again in ${Math.round(delay/1000)} s, the service manager sees no exit / Der Daemon endete mit ${result.code}; die Boot-Stufe startet ihn in ${Math.round(delay/1000)} s neu, der Dienstverwalter sieht kein Ende`);
    await pause(delay,signal);
    continue;
   }
   if(signal.aborted)return 143;
   const waiting=state.get().phase==="waiting"&&!existsSync(context.worktree);
   state.set({phase:waiting?"waiting":"exhausted",nextMs:Date.now()+wait});
   context.say(`${waiting?"Waiting for the worktree":"No replica code starts"}, trying again in ${Math.round(wait/1000)} s / ${waiting?"Warten auf das Arbeitsverzeichnis":"Kein Replica-Code startet"}, nächster Versuch in ${Math.round(wait/1000)} s`);
   await pause(wait,signal);
   wait=Math.min(wait*2,context.limits.backoffMaxMs);
  }
 }finally{clearInterval(beat);}
}

/** 🧪️ Proves this boot program on throwaway directories before it may be installed: (A) a worktree whose supervisor starts is run, found ready and stopped by an abort with code 0; (B) a worktree whose supervisor fails with 70 falls back to a sealed snapshot in the entry directory, which starts. Resolves to `undefined` when both hold, else to the reason. */
export async function selfTestV1(){
 const base=mkdtempSync(join(tmpdir(),"semio-entry-selftest-"));
 try{
  const relative="m",limits={...limitsV1({}),earlyMs:3000,readyMs:8000,graceMs:3000,pollMs:20,backoffMs:20,backoffMaxMs:40,refuseMs:60_000};
  const working=`import {mkdirSync,writeFileSync} from "node:fs";\nimport {join} from "node:path";\nconst root=process.env.SEMIO_FLEET_BOOT_ROOT;\nmkdirSync(join(root,".cache","f","boot"),{recursive:true});\nwriteFileSync(join(root,"ran.txt"),process.env.SEMIO_FLEET_BOOT_SOURCE);\nwriteFileSync(join(root,".cache","f","boot","ready.json"),JSON.stringify({pid:process.pid,waiting:false}));\nprocess.on("SIGTERM",()=>{process.exit(0);});\nsetInterval(()=>{},1000);\n`;
  const supervisor=(root,body)=>{const file=join(root,relative,"🔁️replica","🛟️supervisor","🟦️.ts");mkdirSync(dirname(file),{recursive:true});writeFileSync(file,body);return file;};
  for(const scenario of ["worktree","snapshot"]){
   const root=join(base,scenario,"wt"),entry=join(base,scenario,"entry");
   mkdirSync(root,{recursive:true});mkdirSync(entry,{recursive:true});
   supervisor(root,scenario==="worktree"?working:"process.exit(70);\n");
   if(scenario==="snapshot"){
    const body=working,digest=sha("snapshot"),directory=join(entry,"code",digest);
    supervisor(join(directory,"files"),body);
    writeFileSync(join(directory,"sealed.json"),JSON.stringify({version:1,digest,createdMs:1,moduleRoot:relative,files:[{path:`${relative}/🔁️replica/🛟️supervisor/🟦️.ts`,bytes:Buffer.byteLength(body),sha256:sha(body)}]}));
   }
   const controller=new AbortController(),lines=[],context={entry,worktree:root,moduleRel:relative,storeRoot:()=>undefined,environment:{...process.env},limits,args:["run"],say:line=>lines.push(line)};
   const finished=runBootV1(context,controller.signal);
   const end=Date.now()+20_000;
   while(!existsSync(join(root,"ran.txt"))&&Date.now()<end)await new Promise(done=>setTimeout(done,20));
   const ranWhere=existsSync(join(root,"ran.txt"))?readFileSync(join(root,"ran.txt"),"utf8"):"";
   while(readJson(join(entry,"state.json"))?.phase!=="running"&&Date.now()<end)await new Promise(done=>setTimeout(done,20));
   const phase=readJson(join(entry,"state.json"))?.phase;
   controller.abort();
   const code=await Promise.race([finished,new Promise(done=>setTimeout(()=>done("timeout"),15_000))]);
   if(ranWhere!==scenario)return `scenario ${scenario}: the code ran from "${ranWhere}"`;
   if(phase!=="running")return `scenario ${scenario}: the state was "${phase}"`;
   if(code!==0&&code!==143)return `scenario ${scenario}: the loop ended with ${code}`;
  }
  return undefined;
 }catch(error){return text(error);}
 finally{rmSync(base,{recursive:true,force:true});}
}

/** 🔍️ Checks the installed files against `entry.json`: returns the list of problems (empty when the installation is intact). */
export function verifyV1(entry){
 const manifest=readJson(join(entry,"entry.json")),problems=[];
 if(manifest===undefined||typeof manifest.files!=="object"||manifest.files===null)return ["entry.json is missing or unreadable"];
 for(const [name,expected] of Object.entries(manifest.files)){
  try{if(sha(readFileSync(join(entry,name)))!==expected.sha256)problems.push(`${name} differs`);}catch{problems.push(`${name} is missing`);}
 }
 return problems;
}

/** 📥️ Installs the boot program that lives in this file's directory into `entry`: the three files `boot.mjs`, `boot.sh`, `boot.ps1` (this directory's `.mjs`, `.sh` and `.ps1`), the one-line file `worktree` and, last, `entry.json` with the version, the sha256 and byte count of every file, the worktree, the module path and the store. It never installs a version below the installed one, and not the same version again unless the installation is damaged; it installs nothing that failed its own self-test (`selfTestV1`); the previous `boot.mjs` stays as `boot.prev.mjs`; every file is replaced by a rename. A program installed for another worktree that still holds a machine identity is not taken over unless `takeOver` is set (provisioning and relocation set it), so a scratch worktree of a test or a second checkout never redirects the service of the machine. Result `action`: `installed`, `current`, `refused-downgrade`, `refused-same-version`, `refused-selftest`, `refused-foreign` or `refused-incomplete`. */
export async function installV1(options){
 const {entry,worktree,store=null,moduleRel=moduleRelativeV1,takeOver=false,from=dirname(fileURLToPath(import.meta.url))}=options,now=options.now??Date.now();
 const sources={};
 for(const name of readdirSync(from).sort()){const target=installedNames[extname(name)];if(target!==undefined&&!(target in sources))sources[target]=readFileSync(join(from,name));}
 if(Object.keys(sources).length!==3)return {action:"refused-incomplete",version:bootVersionV1,reason:`the source ${from} lacks one of boot.mjs, boot.sh, boot.ps1`};
 if(sha(sources["boot.mjs"])!==sha(readFileSync(fileURLToPath(import.meta.url))))return {action:"refused-incomplete",version:bootVersionV1,reason:"the program that installs is not the one of the source directory"};
 const manifest=readJson(join(entry,"entry.json")),problems=manifest===undefined?["no manifest"]:verifyV1(entry),hashes=Object.fromEntries(Object.entries(sources).map(([name,data])=>[name,{sha256:sha(data),bytes:data.length}]));
 const metadata=()=>({worktree,module:moduleRel,store});
 if(manifest!==undefined){
  if(!takeOver&&typeof manifest.worktree==="string"&&resolve(manifest.worktree)!==resolve(worktree)&&existsSync(join(manifest.worktree,".cache","f","machine.json")))return {action:"refused-foreign",version:bootVersionV1,reason:`the boot program belongs to the worktree ${manifest.worktree}`};
  if(manifest.bootVersion>bootVersionV1)return {action:"refused-downgrade",version:bootVersionV1,installed:manifest.bootVersion};
  if(manifest.bootVersion===bootVersionV1&&problems.length===0){
   const identical=Object.entries(hashes).every(([name,item])=>manifest.files?.[name]?.sha256===item.sha256);
   if(!identical)return {action:"refused-same-version",version:bootVersionV1,reason:"the source differs from the installed program of the same version, raise bootVersionV1"};
   const changed=JSON.stringify(metadata())!==JSON.stringify({worktree:manifest.worktree,module:manifest.module,store:manifest.store});
   if(changed){writeAtomic(join(entry,"worktree"),`${worktree}\n`);writeAtomic(join(entry,"entry.json"),JSON.stringify({...manifest,...metadata(),installedMs:now}));}
   return {action:"current",version:bootVersionV1,changed};
  }
 }
 const failure=await selfTestV1();
 if(failure!==undefined)return {action:"refused-selftest",version:bootVersionV1,reason:failure};
 mkdirSync(entry,{recursive:true});
 const temporary=join(entry,`.tmp-${randomBytes(6).toString("hex")}`);
 try{
  mkdirSync(temporary,{recursive:true});
  for(const [name,data] of Object.entries(sources)){writeFileSync(join(temporary,name),data);if(name==="boot.sh")chmodSync(join(temporary,name),0o755);if(sha(readFileSync(join(temporary,name)))!==hashes[name].sha256)throw Error(`${name} was not written completely`);}
  const previous=existsSync(join(entry,"boot.mjs"))&&manifest?.files?.["boot.mjs"]!==undefined&&sha(readFileSync(join(entry,"boot.mjs")))===manifest.files["boot.mjs"].sha256?readFileSync(join(entry,"boot.mjs")):undefined;
  if(previous!==undefined)writeAtomic(join(entry,"boot.prev.mjs"),previous);
  for(const name of ["boot.sh","boot.ps1","boot.mjs"])renameSync(join(temporary,name),join(entry,name));
  writeAtomic(join(entry,"worktree"),`${worktree}\n`);
  writeAtomic(join(entry,"entry.json"),JSON.stringify({version:1,bootVersion:bootVersionV1,files:hashes,...previous===undefined?{}:{previous:{sha256:sha(previous)}},...metadata(),installedMs:now}));
  return {action:"installed",version:bootVersionV1};
 }finally{rmSync(temporary,{recursive:true,force:true});}
}

function option(argv,name){const index=argv.indexOf(`--${name}`);return index<0?undefined:argv[index+1];}

async function main(argv){
 const [command="run"]=argv,environment=process.env,entry=entryDirectoryV1(environment);
 if(command==="self-test"){const failure=await selfTestV1();process.stdout.write(failure===undefined?`boot-self-test-ok ${bootVersionV1}\n`:`boot-self-test-failed ${failure}\n`);return failure===undefined?0:1;}
 if(command==="verify"){const problems=verifyV1(entry);process.stdout.write(`${JSON.stringify({ok:problems.length===0,problems})}\n`);return problems.length===0?0:1;}
 if(command==="state"){process.stdout.write(`${JSON.stringify(readJson(join(entry,"state.json"))??null)}\n`);return 0;}
 if(command==="install"){
  const worktree=option(argv,"worktree");
  if(worktree===undefined){process.stderr.write("install needs --worktree <path>\n");return 2;}
  const result=await installV1({entry,worktree:resolve(worktree),store:option(argv,"store")??null,moduleRel:option(argv,"module")??moduleRelativeV1,takeOver:argv.includes("--take-over")});
  process.stdout.write(`${JSON.stringify(result)}\n`);
  return result.action==="installed"||result.action==="current"?0:3;
 }
 if(command!=="run"){process.stderr.write(`unknown command ${command}\n`);return 2;}
 const controller=new AbortController(),stop=()=>{controller.abort();},parent=process.ppid;
 for(const name of process.platform==="win32"?["SIGINT","SIGTERM","SIGBREAK","SIGHUP"]:["SIGINT","SIGTERM"])process.once(name,stop);
 if(parent>1)setInterval(()=>{try{process.kill(parent,0);}catch(error){if(error.code!=="EPERM")stop();}},2000).unref();
 const manifest=()=>readJson(join(entry,"entry.json")),worktree=environment.SEMIO_FLEET_BOOT_ROOT??manifest()?.worktree??(()=>{try{return readFileSync(join(entry,"worktree"),"utf8").trim();}catch{return undefined;}})();
 const say=line=>{process.stdout.write(`${line}\n`);};
 if(worktree===undefined||worktree===""){say(`No worktree is configured in ${entry} / In ${entry} ist kein Arbeitsverzeichnis eingerichtet`);return 2;}
 const storeRoot=()=>readJson(join(worktree,".cache","f","machine.json"))?.storeRoot??manifest()?.store??undefined;
 return runBootV1({entry,worktree,moduleRel:manifest()?.module??moduleRelativeV1,storeRoot,environment,limits:limitsV1(environment),args:["run"],say},controller.signal);
}

const entryFile=process.argv[1];
if(entryFile!==undefined&&entryFile!==""&&(()=>{try{return realpathSync(entryFile)===realpathSync(fileURLToPath(import.meta.url));}catch{return false;}})()){
 process.exitCode=await main(process.argv.slice(2));
 setTimeout(()=>{process.exit(process.exitCode);},3000).unref();
}
