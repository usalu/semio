import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {appendFileSync,mkdirSync,readdirSync,readFileSync} from "node:fs";
import {homedir,tmpdir,userInfo} from "node:os";
import {dirname,isAbsolute,join} from "node:path";
import {readyForgetTreeV1,readyIdentityV1,readyKillPlanV1,readyReadTableV1,readyReadTreeV1,readyTaskkillArgumentsV1,readyTreeMembersV1,readyWriteTreeV1,READY_TREE_POLICY_V1,type ReadyKnownV1,type ReadyProcessV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The numbers and texts of the containment rules.
 * @see ./🧬️schema/🔣️.json */
export const READY_CONTAINMENT_POLICY_V1=schema.definitions.Policy.const;
const C=READY_CONTAINMENT_POLICY_V1,TEXTS=schema.definitions.Texts.const as Readonly<Record<"en"|"de",Readonly<Record<string,string>>>>;

/** 🪪️ The part of a lease registry entry the reaper reads: who registered (`pid`, `startedAtMs`, `command`), where, what it holds. */
export type ReadyRegistryEntryV1=Readonly<{pid:number;startedAtMs:number;updatedAtMs?:number;command:string;directory:string;session?:string;owner?:string;holds?:readonly Readonly<{resource:string;sinceMs?:number}>[]}>;
/** 🪟️ A stopped try of this worker: its stage, its start (which names its owner token) and its end in milliseconds since the epoch. */
export type ReadyWindowV1=Readonly<{stage:string;startedMs:number;endedMs:number}>;
/** 🧑️ The working window of a person: the process of the wrapper they started (pid, when known) and when. */
export type ReadyPersonV1=Readonly<{pid?:number;startedMs:number}>;
/** 🏚️ A process the reaper decided to stop: pid, start time, program name, the registered command and what it held. */
export type ReadyOrphanV1=Readonly<{pid:number;startMs:number;name:string;command:string;owner:string;held:readonly string[]}>;
export type ReadyReapKindV1="roster"|"registry"|"holder";
export type ReadyReapedV1=Readonly<{pid:number;name:string;kind:ReadyReapKindV1}>;
export type ReadyContainmentV1=Readonly<{stage:string;reaped:readonly ReadyReapedV1[];survivors:readonly Readonly<{pid:number;name:string}>[];en:string;de:string}>;
export type ReadyContainmentResultV1=Readonly<{reaped:readonly ReadyReapedV1[];survivors:readonly ReadyKnownV1[]}>;

/** ⏱️ The wait before the next look at the process table of a stage that has run for `elapsedMs`: `fastSampleMilliseconds` during the first `fastWindowMilliseconds` (a launcher that exits after it started a child is seen while it lives), `slowSampleMilliseconds` after that. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readySampleDelayV1(elapsedMs:number,policy:typeof C=C):number{return elapsedMs<policy.fastWindowMilliseconds?policy.fastSampleMilliseconds:policy.slowSampleMilliseconds;}

/** 🪟️ The windows of the stopped tries in the timing rows of a worker (`ready-timings.json`): the rows whose outcome is one of `stoppedOutcomes`. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyStoppedWindowsV1(runs:readonly Readonly<{stage:string;startedMs:number;endedMs:number;outcome:string}>[],policy:typeof C=C):readonly ReadyWindowV1[]{
 return runs.filter(run=>policy.stoppedOutcomes.includes(run.outcome)).map(run=>({stage:run.stage,startedMs:run.startedMs,endedMs:run.endedMs}));
}

/** 🧭️ A path as a comparison key: separators `/`, lower case (Windows paths differ in both). */
export function readyPathKeyV1(text:string):string{return text.replace(/\\/gu,"/").toLowerCase();}

/** 🏷️ The owner token of one try of a stage: `ready:<first 12 hex digits of the SHA-256 of the checkout path>:<stage>:<start of the try>`. The runner puts it into the environment of the stage (`ownerEnvironment`), every descendant inherits it and the lease registry writes it into the entry of every process that holds a lease, so a process is the stage's own by proof and by nothing else. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyOwnerTokenV1(root:string,stage:string,startedMs:number):string{return`ready:${createHash("sha256").update(readyPathKeyV1(root)).digest("hex").slice(0,12)}:${stage}:${startedMs}`;}

/** 🔎️ Why a registry entry is or is not an orphan: `orphan`, or the first rule that spared it (`protected`, `stage`, `not-ours`, `shared`, `not-alive`, `identity`, `person`, `sheltered`). */
export type ReadyVerdictV1="orphan"|"protected"|"stage"|"not-ours"|"shared"|"not-alive"|"identity"|"person"|"sheltered";

/** 🏚️ The verdict on every registry entry (first entry of a pid decides), by identity (pid and start time, never the pid alone) and by proof of ownership: the entry carries the owner token (`readyOwnerTokenV1`) of a stopped try in `windows`; a process without such a token, of another agent, checkout or person, is `not-ours`, never judged by the time it started. A resident service (`sharedCommandPattern`) is `shared` and spared. The process is alive in `table` with the registered start time; it did not start inside the working window of a person (`human`, `humanWindowMilliseconds`, `edgeToleranceMilliseconds` before their start) and is no descendant of their process; it is neither this process nor an ancestor of it, nor a member of the tree of the current stage (`stage`) nor below one. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyOrphanVerdictsV1(input:Readonly<{entries:readonly ReadyRegistryEntryV1[];table:readonly ReadyProcessV1[];windows:readonly ReadyWindowV1[];root:string;human:ReadyPersonV1|null;self:number;stage:Readonly<{root:ReadyKnownV1;known:readonly ReadyKnownV1[]}>|null;tolerance?:number;policy?:typeof C}>):ReadonlyMap<number,Readonly<{entry:ReadyRegistryEntryV1;verdict:ReadyVerdictV1}>>{
 const policy=input.policy??C,tolerance=input.tolerance??READY_TREE_POLICY_V1.toleranceMilliseconds,byPid=new Map(input.table.map(row=>[row.pid,row]));
 const protectedPids=new Set<number>(),owners=new Set(input.windows.map(window=>readyOwnerTokenV1(input.root,window.stage,window.startedMs)));
 for(let at:number|undefined=input.self,depth=0;at!==undefined&&depth<256&&!protectedPids.has(at);at=byPid.get(at)?.ppid,depth++)protectedPids.add(at);
 const inStage=new Set(input.stage===null?[]:readyTreeMembersV1(input.table,input.stage.root,input.stage.known,tolerance).map(row=>row.pid));
 const found=new Map<number,Readonly<{entry:ReadyRegistryEntryV1;verdict:ReadyVerdictV1}>>(),shared=new RegExp(policy.sharedCommandPattern,"u");
 const judge=(entry:ReadyRegistryEntryV1):ReadyVerdictV1=>{
  if(entry.pid<=READY_TREE_POLICY_V1.reservedPids||protectedPids.has(entry.pid))return"protected";
  if(inStage.has(entry.pid))return"stage";
  if(!(entry.startedAtMs>0)||entry.owner===undefined||!owners.has(entry.owner))return"not-ours";
  if(shared.test(entry.command))return"shared";
  const row=byPid.get(entry.pid);
  if(row===undefined)return"not-alive";
  if(readyIdentityV1({pid:entry.pid,startMs:entry.startedAtMs},input.table,tolerance)!=="same")return"identity";
  if(input.human!==null&&entry.startedAtMs>=input.human.startedMs-policy.edgeToleranceMilliseconds&&entry.startedAtMs<input.human.startedMs+policy.humanWindowMilliseconds)return"person";
  for(let at=byPid.get(row.ppid),depth=0;at!==undefined&&depth<256;at=byPid.get(at.ppid),depth++)if(at.startMs<=row.startMs+tolerance&&(inStage.has(at.pid)||input.human?.pid===at.pid))return"sheltered";
  return"orphan";
 };
 for(const entry of input.entries)if(!found.has(entry.pid))found.set(entry.pid,{entry,verdict:judge(entry)});
 return found;
}

/** 🏚️ The processes of the lease registry that are orphans of a stopped try and may be stopped (`readyOrphanVerdictsV1`), ordered by pid. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyOrphanPlanV1(input:Parameters<typeof readyOrphanVerdictsV1>[0]):readonly ReadyOrphanV1[]{
 const byPid=new Map(input.table.map(row=>[row.pid,row]));
 return[...readyOrphanVerdictsV1(input)].filter(([,row])=>row.verdict==="orphan").map(([pid,row])=>Object.freeze({pid,startMs:row.entry.startedAtMs,name:byPid.get(pid)?.name??"?",command:row.entry.command.slice(0,160),owner:row.entry.owner??"",held:Object.freeze((row.entry.holds??[]).map(hold=>hold.resource))})).sort((left,right)=>left.pid-right.pid);
}

/** 🤝️ The pids of the live processes that other agents use: registry entries whose process is alive with the registered start time and whose command is a resident service (`sharedCommandPattern`), and the live hosts named by their own descriptors (`readyHostPidsV1`). A stop never touches them or anything below them. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readySharedPidsV1(entries:readonly ReadyRegistryEntryV1[],table:readonly ReadyProcessV1[],policy:typeof C=C,tolerance:number=READY_TREE_POLICY_V1.toleranceMilliseconds,hostDirectories:readonly string[]=readyHostDirectoriesV1()):ReadonlySet<number>{
 const shared=new RegExp(policy.sharedCommandPattern,"u");
 return new Set([...entries.filter(entry=>shared.test(entry.command)&&readyIdentityV1({pid:entry.pid,startMs:entry.startedAtMs},table,tolerance)==="same").map(entry=>entry.pid),...readyHostPidsV1(table,hostDirectories,tolerance)]);
}

/** 🏡️ The directories that hold the descriptors of the resident hosts of this user (`host-<tag>.json`): `<tmp>/semio-repo/<uid>` and the short form `<tmp>/sr<uid>` (POSIX; Windows hosts speak over named pipes and are known by their registry entry alone). */
export function readyHostDirectoriesV1(base:string=tmpdir(),uid:string=(()=>{try{return String(userInfo().uid);}catch{return"0";}})(),platform:NodeJS.Platform=process.platform):readonly string[]{return platform==="win32"?[]:[join(base,"semio-repo",uid),join(base,"sr"+uid)];}

/** 🏠️ The pids of the live resident hosts named by their own descriptors, whether or not they publish a lease registry entry at this moment: a descriptor `{pid, startedAt, stopping?}` counts while its pid is in the table, started no later than the descriptor says and not more than `hostStartWindowMilliseconds` before it, and the host is not stopping. A live predicate of the service itself, so a try never stops the service that the next try or another agent needs; a damaged descriptor or an unreadable directory names nothing. A pure function of the table and the directories.
 * @see ./🧪️tests/🟦️.ts */
export function readyHostPidsV1(table:readonly ReadyProcessV1[],directories:readonly string[]=readyHostDirectoriesV1(),tolerance:number=READY_TREE_POLICY_V1.toleranceMilliseconds):ReadonlySet<number>{
 const found=new Set<number>(),byPid=new Map(table.map(row=>[row.pid,row]));
 for(const directory of directories){
  let names:string[];try{names=readdirSync(directory).filter(name=>/^host-[0-9a-f]+\.json$/u.test(name)).slice(0,C.registryEntryLimit);}catch{continue;}
  for(const name of names){
   try{
    const value=JSON.parse(readFileSync(join(directory,name),"utf8")) as {pid?:unknown;startedAt?:unknown;stopping?:unknown};
    if(!Number.isSafeInteger(value.pid)||!Number.isFinite(value.startedAt)||value.stopping===true)continue;
    const row=byPid.get(value.pid as number),startedAt=value.startedAt as number;
    if(row!==undefined&&row.startMs<=startedAt+tolerance&&startedAt-row.startMs<=C.hostStartWindowMilliseconds)found.add(row.pid);
   }catch{}
  }
 }
 return found;
}

/** 🗺️ The lease registry of this user, where every process that holds a lease publishes its entry: the same folder as the registry itself names, `SEMIO_LEASE_REGISTRY_DIRECTORY` first, then the cache of the platform. A law compares it with the registry's own answer on all three platforms.
 * @see ../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts */
export function readyRegistryRootV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment.SEMIO_LEASE_REGISTRY_DIRECTORY;
 if(override)return override;
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache");
 return join(base,"semio","lease-registry");
}

/** 👪️ The processes of `table` that join the roster because their parent was a roster member (`previous`) that has exited since: the pid of a dead parent stays in its child on Windows, so a launcher that exits after it started a detached child does not lose the child. The child must have started after its parent and no earlier than the previous sample (`sinceMs`, within the tolerance); a parent whose pid another process holds now is not adopted from. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyHeirsV1(input:Readonly<{table:readonly ReadyProcessV1[];previous:readonly ReadyKnownV1[];sinceMs:number;tolerance?:number}>):readonly ReadyKnownV1[]{
 const tolerance=input.tolerance??READY_TREE_POLICY_V1.toleranceMilliseconds,parents=new Map(input.previous.map(member=>[member.pid,member]));
 return input.table.filter(row=>{
  const parent=parents.get(row.ppid);
  return parent!==undefined&&!parents.has(row.pid)&&readyIdentityV1(parent,input.table,tolerance)==="gone"&&row.startMs>=parent.startMs-tolerance&&row.startMs>=input.sinceMs-tolerance;
 }).map(row=>row.name===undefined?{pid:row.pid,startMs:row.startMs}:{pid:row.pid,startMs:row.startMs,name:row.name});
}

/** 📖️ Every readable lease registry entry below `root` (`<root>/<folder>/<pid>-<startedAtMs>.json`), without any freshness rule: an orphan that still holds a lease is the case, a heartbeat that stopped is no reason to forget it. At most `registryEntryLimit` files; a damaged or foreign file is skipped. */
export function readyRegistryEntriesV1(root:string,policy:typeof C=C):readonly ReadyRegistryEntryV1[]{
 const name=new RegExp(policy.registryFileName,"u"),rows:ReadyRegistryEntryV1[]=[];
 let folders:string[];try{folders=readdirSync(root);}catch{return rows;}
 for(const folder of folders){
  let names:string[];try{names=readdirSync(join(root,folder)).filter(each=>name.test(each));}catch{continue;}
  for(const each of names){
   if(rows.length>=policy.registryEntryLimit)return rows;
   try{
    const value=JSON.parse(readFileSync(join(root,folder,each),"utf8")) as Partial<ReadyRegistryEntryV1>;
    if(Number.isSafeInteger(value.pid)&&Number.isFinite(value.startedAtMs)&&typeof value.command==="string"&&typeof value.directory==="string")rows.push(value as ReadyRegistryEntryV1);
   }catch{}
  }
 }
 return rows;
}

/** 🔌️ What a stop touches outside its decisions: the process table, the end of one process (on Windows the `taskkill` command with its arguments), the wait. */
export type ReadyStopPortsV1=Readonly<{platform?:NodeJS.Platform;table?:()=>Promise<readonly ReadyProcessV1[]>;kill?:(pid:number,signal:"SIGTERM"|"SIGKILL")=>Promise<void>|void;grace?:number;spare?:(table:readonly ReadyProcessV1[])=>ReadonlySet<number>;record?:(row:ReadyProcessV1,signal:"SIGTERM"|"SIGKILL")=>void;run?:(command:string,args:readonly string[])=>Promise<void>;sleep?:(milliseconds:number)=>Promise<void>;self?:number;policy?:typeof C}>;

const runCommand=(command:string,args:readonly string[],timeoutMilliseconds:number):Promise<void>=>new Promise<void>(done=>{
 let child:ReturnType<typeof spawn>;
 try{child=spawn(command,[...args],{windowsHide:true,stdio:"ignore"});}catch{return done();}
 const timer=setTimeout(()=>{try{child.kill();}catch{}done();},timeoutMilliseconds);
 child.on("error",()=>{clearTimeout(timer);done();});child.on("close",()=>{clearTimeout(timer);done();});
});

/** 🛑️ Stops a root and everything recorded with it and proves it: each pass reads the table, takes the plan (the root if it is still the same process, everything below, every `known` member and every member an earlier pass ended that is still there with its recorded start, so a child that outlived its killed parent is not forgotten; never this process or its ancestors, never `spare` and what is below it), ends every member (Windows `taskkill /PID <pid> /T /F`, POSIX `SIGKILL`, and the process group `group` when given), waits `settleMilliseconds` and looks again, at most `stopAttempts` passes; the members still alive at the end are the survivors, named by their program. With `grace` (POSIX) the first pass only asks with SIGTERM and polls the table for up to `grace` milliseconds for the members to end by themselves; SIGKILL follows only for those that stay. `record` hears about every signal before it is sent. An empty table (the command failed) leaves the answer unverified and no survivor is claimed. */
export async function readyStopV1(input:ReadyStopPortsV1&Readonly<{root:ReadyKnownV1;known?:readonly ReadyKnownV1[];group?:number}>):Promise<Readonly<{stopped:readonly ReadyKnownV1[];survivors:readonly ReadyKnownV1[];verified:boolean;passes:number}>>{
 const policy=input.policy??C,platform=input.platform??process.platform,table=input.table??(()=>readyReadTableV1(20000,platform)),self=input.self??process.pid;
 const run=input.run??((command:string,args:readonly string[])=>runCommand(command,args,policy.killTimeoutMilliseconds));
 const kill=input.kill??((pid:number,signal:"SIGTERM"|"SIGKILL")=>platform==="win32"?run("taskkill",readyTaskkillArgumentsV1(pid)):(()=>{try{process.kill(pid,signal);}catch{}})());
 const sleep=input.sleep??((milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds)));
 const stopped=new Map<number,ReadyKnownV1>();
 let survivors:readonly ReadyKnownV1[]=[],verified=false,passes=0,asked=!(input.grace!==undefined&&input.grace>0&&platform!=="win32");
 const plan=(rows:readonly ReadyProcessV1[])=>{const base={table:rows,root:input.root,known:[...input.known??[],...stopped.values()],self},spare=input.spare?.(rows),ids=readyKillPlanV1({...base,...spare===undefined?{}:{spare}});return{ids,members:rows.filter(row=>ids.includes(row.pid)),whole:spare===undefined||spare.size===0||readyKillPlanV1(base).length===ids.length};};
 for(let pass=0;pass<=policy.stopAttempts;pass++){
  const rows=await table();
  if(rows.length===0)break;
  const {ids,members:found,whole}=plan(rows),members=found.map((row):ReadyKnownV1=>row.name===undefined?{pid:row.pid,startMs:row.startMs}:{pid:row.pid,startMs:row.startMs,name:row.name});
  if(members.length===0){verified=true;survivors=[];break;}
  if(pass===policy.stopAttempts){verified=true;survivors=members;break;}
  for(const member of members)stopped.set(member.pid,member);
  passes++;
  const signal=asked?"SIGKILL":"SIGTERM";
  for(const row of found)try{input.record?.(row,signal);}catch{}
  await Promise.all(ids.map(async pid=>{try{await kill(pid,signal);}catch{}}));
  if(input.group!==undefined&&platform!=="win32"&&whole)try{process.kill(-input.group,signal);}catch{}
  if(asked){await sleep(policy.settleMilliseconds);continue;}
  asked=true;
  for(let waited=0;waited<input.grace!;waited+=policy.terminatePollMilliseconds){await sleep(policy.terminatePollMilliseconds);const now=await table();if(now.length===0||plan(now).members.length===0)break;}
 }
 return Object.freeze({stopped:[...stopped.values()],survivors,verified,passes});
}

const fill=(language:"en"|"de",kind:string,fields:Readonly<Record<string,string|number>>):string=>TEXTS[language][kind]!.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));

/** 🗣️ One line of the log in English and German, `<English> / <German>`. */
export function readyContainmentLineV1(kind:"reaped-roster"|"reaped-registry"|"reaped-holder"|"yielded-reader"|"survivor",fields:Readonly<Record<string,string|number>>):string{return`${fill("en",kind,fields)} / ${fill("de",kind,fields)}`;}

const reasonOf=(language:"en"|"de",reason:ReadyKillReasonV1,seconds:number):string=>fill(language,`reason-${reason}`,{seconds});

/** 🪪️ Why the stage runner stops a process: `orphan`, `holder`, `roster`, `stall`, `overdue` (a step of the read ran far past its own bound with nothing else moving), `restart`, `yield` (a background graph read gives way to the stage that waits behind it), `timeout`, `cancel` or `settle`. */
export type ReadyKillReasonV1="orphan"|"holder"|"roster"|"stall"|"overdue"|"restart"|"yield"|"timeout"|"cancel"|"settle";

/** 🗣️ The record of one signal in English and German, `<English> / <German>`: who (the runner process `killer`), which stage and owner token, why. */
export function readyKillRecordLineV1(input:Readonly<{pid:number;name:string;signal:"SIGTERM"|"SIGKILL";killer:number;stage:string;owner:string;reason:ReadyKillReasonV1;seconds?:number}>):string{
 const fields={pid:input.pid,name:input.name,signal:input.signal,killer:input.killer,stage:input.stage,owner:input.owner};
 return`${fill("en","kill-record",{...fields,reason:reasonOf("en",input.reason,input.seconds??0)})} / ${fill("de","kill-record",{...fields,reason:reasonOf("de",input.reason,input.seconds??0)})}`;
}

/** 📝️ The recorder of a stage runner: every signal it sends is said (`say`, the log of the stage) and appended with its time to `<directory>/<killRecordName>`, so the log of a victim and the worker say who stopped it. */
export function readyKillRecorderV1(input:Readonly<{directory:string;stage:string;owner:string;reason:ReadyKillReasonV1;seconds?:number;killer?:number;say?:(line:string)=>void;now?:()=>number;policy?:typeof C}>):(row:ReadyProcessV1,signal:"SIGTERM"|"SIGKILL")=>void{
 return(row,signal)=>{
  const line=readyKillRecordLineV1({pid:row.pid,name:row.name??"?",signal,killer:input.killer??process.pid,stage:input.stage,owner:input.owner,reason:input.reason,...input.seconds===undefined?{}:{seconds:input.seconds}});
  input.say?.(line);
  try{const file=join(input.directory,(input.policy??C).killRecordName);mkdirSync(dirname(file),{recursive:true});appendFileSync(file,`${new Date((input.now??Date.now)()).toISOString()} ${line}\n`);}catch{}
 };
}

const names=(rows:readonly Readonly<{pid:number;name:string}>[]):string=>rows.map(row=>`${row.name} (${row.pid})`).join(", ");

/** 📋️ What `ready.json` says about one stage run (`containment`): what was reaped and what survived the stop, with both texts; `null` when there is nothing to say. */
export function readyContainmentStatusV1(input:Readonly<{stage:string;reaped:readonly ReadyReapedV1[];survivors:readonly ReadyKnownV1[]}>):ReadyContainmentV1|null{
 if(input.reaped.length===0&&input.survivors.length===0)return null;
 const survivors=input.survivors.map(row=>({pid:row.pid,name:row.name??"?"})),parts=(language:"en"|"de"):string=>[
  ...input.reaped.length===0?[]:[fill(language,"summary-reaped",{count:input.reaped.length,names:names(input.reaped)})],
  ...survivors.length===0?[]:[fill(language,"summary-survivors",{stage:input.stage,count:survivors.length,names:names(survivors)})],
 ].join(" ");
 return Object.freeze({stage:input.stage,reaped:input.reaped.map(row=>({pid:row.pid,name:row.name,kind:row.kind})),survivors,en:parts("en"),de:parts("de")});
}

/** 🧹️ Stops the roster of every stage that an earlier try left behind (`<stage>.tree.json` of the directory): each is stopped and verified; a roster whose members are all gone is forgotten, one with survivors keeps only them. Returns what it stopped and what survived. */
export async function readySweepRostersV1(input:ReadyStopPortsV1&Readonly<{directory:string;owner?:(stage:string,startedMs:number)=>string;say?:(line:string)=>void;now?:()=>number}>):Promise<ReadyContainmentResultV1>{
 const suffix=`.${READY_TREE_POLICY_V1.treeName}`,reaped:ReadyReapedV1[]=[],survivors:ReadyKnownV1[]=[];
 let files:string[];try{files=readdirSync(input.directory).filter(name=>name.endsWith(suffix));}catch{return{reaped,survivors};}
 for(const file of files.sort()){
  const stage=file.slice(0,-suffix.length),left=readyReadTreeV1(input.directory,stage);
  if(left===undefined)continue;
  const result=await readyStopV1({...input,grace:input.grace??(input.policy??C).terminateGraceMilliseconds,root:left.root,known:left.processes,...input.record===undefined&&input.owner!==undefined?{record:readyKillRecorderV1({directory:input.directory,stage,owner:input.owner(stage,left.root.startMs),reason:"roster",...input.say===undefined?{}:{say:input.say},policy:input.policy??C})}:{}});
  for(const member of result.stopped){const name=member.name??"?";reaped.push({pid:member.pid,name,kind:"roster"});input.say?.(readyContainmentLineV1("reaped-roster",{pid:member.pid,name,stage}));}
  if(result.survivors.length===0){if(result.verified)readyForgetTreeV1(input.directory,stage);}
  else{survivors.push(...result.survivors);readyWriteTreeV1(input.directory,stage,left.root,result.survivors,(input.now??Date.now)());}
 }
 return{reaped,survivors};
}

/** 🏚️ Reaps the orphans of the lease registry (`readyOrphanPlanV1`) one by one with a verified stop, saying each in both languages; `kind` is `registry` at the start of a stage and `holder` when a stage that waits on a named holder found it to be an orphan. A table that cannot be read reaps nothing. */
export async function readyReapOrphansV1(input:ReadyStopPortsV1&Readonly<{registryRoot:string;root:string;windows:readonly ReadyWindowV1[];human:ReadyPersonV1|null;stage:Readonly<{root:ReadyKnownV1;known:readonly ReadyKnownV1[]}>|null;stageName:string;kind:"registry"|"holder";directory?:string;say?:(line:string)=>void;explain?:(lines:readonly string[])=>void}>):Promise<ReadyContainmentResultV1>{
 const reaped:ReadyReapedV1[]=[],survivors:ReadyKnownV1[]=[],platform=input.platform??process.platform;
 if(input.windows.length===0)return{reaped,survivors};
 const entries=readyRegistryEntriesV1(input.registryRoot,input.policy??C);
 if(entries.length===0)return{reaped,survivors};
 const table=await(input.table??(()=>readyReadTableV1(20000,platform)))();
 if(table.length===0)return{reaped,survivors};
 const judged={entries,table,windows:input.windows,root:input.root,human:input.human,self:input.self??process.pid,stage:input.stage,...input.policy===undefined?{}:{policy:input.policy}};
 const verdicts=readyOrphanVerdictsV1(judged),counts=new Map<string,number>();
 for(const row of verdicts.values())counts.set(row.verdict,(counts.get(row.verdict)??0)+1);
 input.explain?.([`${new Date().toISOString()} registry ${input.registryRoot}: ${entries.length} entries, ${[...counts].map(([name,count])=>`${name} ${count}`).join(", ")}; windows ${input.windows.length}, person ${input.human===null?"none":"yes"}`,...[...verdicts].filter(([pid,row])=>row.verdict!=="orphan"&&row.verdict!=="not-alive"&&row.verdict!=="protected"&&table.some(each=>each.pid===pid)).slice(0,8).map(([pid,row])=>`${pid} ${row.verdict} started ${new Date(row.entry.startedAtMs).toISOString()} directory ${row.entry.directory.slice(-90)} command ${row.entry.command.slice(0,70)}`)]);
 for(const orphan of readyOrphanPlanV1(judged)){
  const result=await readyStopV1({...input,grace:input.grace??(input.policy??C).terminateGraceMilliseconds,spare:input.spare??(rows=>readySharedPidsV1(entries,rows,input.policy??C)),root:{pid:orphan.pid,startMs:orphan.startMs},...input.directory===undefined?{}:{record:readyKillRecorderV1({directory:input.directory,stage:input.stageName,owner:orphan.owner,reason:input.kind==="holder"?"holder":"orphan",...input.say===undefined?{}:{say:input.say},policy:input.policy??C})}});
  reaped.push({pid:orphan.pid,name:orphan.name,kind:input.kind});survivors.push(...result.survivors);
  input.say?.(readyContainmentLineV1(input.kind==="holder"?"reaped-holder":"reaped-registry",{pid:orphan.pid,name:orphan.name,stage:input.stageName,started:new Date(orphan.startMs).toISOString(),owner:orphan.owner}));
  for(const left of result.survivors)input.say?.(readyContainmentLineV1("survivor",{pid:left.pid,name:left.name??"?",passes:result.passes}));
 }
 return{reaped,survivors};
}
