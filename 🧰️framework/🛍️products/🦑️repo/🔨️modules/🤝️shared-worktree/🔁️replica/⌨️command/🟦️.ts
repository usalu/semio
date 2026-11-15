import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {readFile,stat} from "node:fs/promises";
import {join} from "node:path";
import {isAbsolute,resolve} from "node:path";
import {groupV1,peerStorePathV1,platformV1,readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {compactV1,dryOpenReplicaV1,fleetCompactV1,homedOriginsV1,joinV1,JoinRefusedV1,OriginTakenV1,relayOnceV1,relayV1,ReplicaBusyV1,requestRepairV1,requireReplicaShapeV1,runV1,seedV1,setSyncEnabledV1,snapshotV1,statusV1,smokeBootV1,storeReachableV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type StatusReportV1} from "../🟦️.ts";
import {attestPathV1,codeDigestV1,fleetReportV1,freshSecondsV1,readAttestationV1,readRetirementV1,requireAttestShapeV1,viewDiffV1,writeRetirementV1,type FleetReportV1} from "../../🩺️attest/🟦️.ts";
import {formatHlcV1,listOriginsV1,parseHlcV1} from "../../📒️journal/🟦️.ts";
import {installBootStageV1,readFallbackV1,selfCheckV1} from "../../🛡️resilience/🟦️.ts";
import type {LatencyReportV1} from "../../🎣️tiers/🟦️.ts";
import {refreshBootstrapV1,runBootstrapCommandV1} from "../🥾️bootstrap/🟦️.ts";
import {refreshKitV1} from "../../🗺️topology/🥾️provision/🟦️.ts";
import {consolidateGitV1,refreshConsolidationV1} from "../🧱️consolidate/🟦️.ts";
import {requestRebaseV1} from "../../🌿️git/🟦️.ts";
import {normaliseHintPathV1,parseHintTextV1,parseHookInputV1,writeHintV1} from "../../🎣️tiers/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import adoptTexts from "../🧲️adopt/📖️texts/🔣️.json" with {type:"json"};
import {adoptV1,AdoptRefusedV1,type AdoptCategoryV1,type AdoptPlanV1} from "../🧲️adopt/🟦️.ts";
import {relocateV1,RelocateRefusedV1,type RelocatePhaseV1} from "../📍️relocate/🟦️.ts";
import {listAccessV1} from "../../🔐️access/🟦️.ts";

/** 🎛️ Carries the issued phase into the command: where the repository is, the environment, cancellation, the stdout line sink (NDJSON events and reports) and the stderr line sink (human messages). */
export type ReplicaCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;write:(line:string)=>void;notice:(line:string)=>void;readStdin?:()=>Promise<string>}>;

/** 🏁️ How a command ended: the process exit code. */
export type ReplicaCommandResultV1=Readonly<{exitCode:number}>;

/** 🗣️ The languages of all messages; there is no default among them. */
export type ReplicaLanguageV1="en"|"de";

type Operation="seed"|"join"|"run"|"status"|"snapshot"|"relay"|"compact"|"compaction"|"consolidate"|"verify"|"bootstrap-archive"|"bootstrap-settle"|"sync"|"fleet"|"rebase"|"repair"|"retire"|"self-check"|"touch"|"relocate";
type Variables=Readonly<Record<string,string|number>>;

const LANGUAGES=texts.languages as readonly ReplicaLanguageV1[],
 IDENTITY=["--worktree","--store","--group","--origin","--lang"],
 OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
  seed:{valued:IDENTITY,plain:[]},
  join:{valued:[...IDENTITY,"--sample"],plain:["--adopt","--dry-run","--force"]},
  relocate:{valued:["--worktree","--to","--phase","--wait","--sample","--lang"],plain:["--dry-run","--force"]},
  run:{valued:["--worktree","--store","--poll","--scan","--debounce","--lang"],plain:[]},
  status:{valued:["--worktree","--store","--lang"],plain:["--json","--deep"]},
  snapshot:{valued:["--worktree","--store","--lang"],plain:[]},
  relay:{valued:["--store","--group","--worktree","--peers","--poll","--bootstrap-minutes","--consolidate-minutes","--concurrency","--lang"],plain:["--once"]},
  compact:{valued:["--worktree","--store","--retention-days","--lang"],plain:["--collect-blobs"]},
  compaction:{valued:["--worktree","--store","--lang"],plain:["--plan","--apply","--no-blobs"]},
  consolidate:{valued:["--worktree","--store","--lang"],plain:["--apply"]},
  verify:{valued:["--worktree","--store","--lang"],plain:["--json"]},
  "bootstrap-archive":{valued:["--worktree","--store","--lang"],plain:["--force"]},
  "bootstrap-settle":{valued:["--worktree","--store","--lang"],plain:[]},
  sync:{valued:["--worktree","--lang"],plain:["--on","--off"]},
  rebase:{valued:["--worktree","--lang"],plain:[]},
  repair:{valued:["--worktree","--lang"],plain:[]},
  fleet:{valued:["--worktree","--store","--lang","--wait"],plain:["--json","--diff"]},
  retire:{valued:["--worktree","--store","--lang","--reason"],plain:["--undo","--force"]},
  touch:{valued:["--worktree","--lang"],plain:["--stdin","--hook"]},
  "self-check":{valued:["--lang"],plain:[]}
 };

/** 💬️ Renders one catalogue message in one language. */
export function replicaTextV1(language:ReplicaLanguageV1,key:string,variables:Variables={}):string{
 const message=({...texts.messages,...adoptTexts.messages} as Readonly<Record<string,Readonly<Record<ReplicaLanguageV1,string>>>>)[key];
 if(!message)throw Error(`Declared replica message required: ${key}`);
 return message[language].replace(/\{([a-z0-9]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Replica message variable missing: ${name}`);return String(value);});
}

function both(key:string,variables:Variables={}):string{return LANGUAGES.map(language=>replicaTextV1(language,key,variables)).join("\n");}

/** ☁️ Picks the output languages: the explicit request, else the one the language protocol chooses, else all of them, because no language is the default. */
export function replicaLanguagesV1(requested:string|undefined,environment:Readonly<Record<string,string|undefined>>):readonly ReplicaLanguageV1[]{
 if(requested!==undefined){
  if(!LANGUAGES.includes(requested as ReplicaLanguageV1))throw Error(both("error.language",{language:requested}));
  return [requested as ReplicaLanguageV1];
 }
 return noticeLanguagesV1(LANGUAGES,environment);
}

/** 🔠️ Spells a byte count in binary units, deterministically. */
export function formatReplicaBytesV1(bytes:number):string{
 if(bytes<1024)return `${bytes} B`;
 const units=["KiB","MiB","GiB","TiB"];let value=bytes,index=-1;
 do{value/=1024;index++;}while(value>=1024&&index<units.length-1);
 return `${value.toFixed(2)} ${units[index]}`;
}

/** 📝️ The closed arguments of one replica operation: valued options by name, the plain flags that were given, for `retire` the origin named as the first argument and for `touch` every argument that is not an option as a path. */
export type ReplicaArgumentsV1=Readonly<{operation:Operation;target:string|undefined;paths:readonly string[];values:Readonly<Record<string,string>>;flags:ReadonlySet<string>}>;

/** 🧭️ Reads `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle|sync|fleet|rebase|repair|retire <origin>|touch <path>…` with its options (`--name value` or `--name=value`); every unknown, misplaced or repeated option is refused in both languages. */
export function parseReplicaArgumentsV1(args:readonly string[]):ReplicaArgumentsV1{
 const operation=args[0];
 if(operation===undefined||!Object.hasOwn(OPTIONS,operation))throw Error(both("error.syntax"));
 const allowed=OPTIONS[operation as Operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),paths:string[]=[];
 const target=operation==="retire"&&args[1]!==undefined&&!args[1].startsWith("--")?args[1]:undefined;
 for(let index=target===undefined?1:2;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(operation==="touch"&&!argument.startsWith("--")){paths.push(argument);continue;}
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  if(allowed.valued.includes(name)){
   const value=cut<0?args[++index]:argument.slice(cut+1);
   if(value===undefined||value===""||value.startsWith("--"))throw Error(both("error.value",{option:name}));
   values[name]=value;
  }else if(allowed.plain.includes(name)&&cut<0)flags.add(name);
  else throw Error(both("error.option",{operation,option:argument}));
  seen.add(name);
 }
 return Object.freeze({operation:operation as Operation,target,paths:Object.freeze(paths),values:Object.freeze(values),flags});
}

const whole=(name:string,value:string,minimum:number,maximum:number):number=>{
 const number=Number(value);
 if(!/^[0-9]+$/u.test(value)||!Number.isSafeInteger(number)||number<minimum||number>maximum)throw Error(both("error.number",{option:name,minimum,maximum}));
 return number;
};

const where=(root:string,path:string):string=>isAbsolute(path)?path:resolve(root,path);

const peersOf=(root:string,text:string):Readonly<Record<string,string>>=>{
 const peers:Record<string,string>={};
 for(const item of text.split(",")){
  const cut=item.indexOf("="),id=item.slice(0,cut),path=item.slice(cut+1);
  if(cut<1||path===""||Object.hasOwn(peers,id))throw Error(both("error.peers",{option:"--peers"}));
  peers[id]=where(root,path);
 }
 return peers;
};

async function machineOf(worktree:string):Promise<Readonly<{originId:string;groupId:string;storeRoot:string}>|undefined>{
 try{
  const value=JSON.parse(await readFile(resolve(worktree,".cache","f","machine.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Machine",value,"Closed replica machine required");
  return value as {originId:string;groupId:string;storeRoot:string};
 }catch(error){if((error as {code?:string}).code==="ENOENT")return undefined;throw error;}
}

type Resolved=Readonly<{options:ReplicaOptionsV1;worktree:string;storeRoot:string}>;

async function resolveOptions(parsed:ReplicaArgumentsV1,context:ReplicaCommandContextV1):Promise<Resolved>{
 const topology=readTopologyV1(),worktree=where(context.root,parsed.values["--worktree"]??context.root),machine=await machineOf(worktree);
 const groupId=parsed.values["--group"]??machine?.groupId,originId=parsed.values["--origin"]??machine?.originId;
 if(groupId===undefined||originId===undefined)throw Error(both("error.identity"));
 groupV1(topology,groupId);
 const given=parsed.values["--store"],storeRoot=given!==undefined?where(context.root,given):machine?.storeRoot??topology.storeMount[platformV1()],defaults=topology.replica,limits=(name:string,fallback:number,minimum:number,maximum:number):number=>parsed.values[name]===undefined?fallback:whole(name,parsed.values[name]!,minimum,maximum);
 return {worktree,storeRoot,options:{worktree,storeRoot,originId,groupId,pollMilliseconds:limits("--poll",defaults.pollMilliseconds,10,60000),scanSeconds:limits("--scan",defaults.scanSeconds,1,3600),debounceMilliseconds:limits("--debounce",defaults.debounceMilliseconds,0,10000),retentionDays:limits("--retention-days",defaults.retentionDays,0,3650),compaction:defaults.compaction,...topology.sidecars===undefined?{}:{sidecars:topology.sidecars}}};
}

function reporter(context:ReplicaCommandContextV1,languages:readonly ReplicaLanguageV1[]):(key:string,variables?:Variables)=>void{
 return (key,variables)=>{
  for(const language of languages){
   const resolved:Record<string,string|number>={};
   for(const [name,value] of Object.entries(variables??{}))resolved[name]=typeof value=="string"&&value.startsWith("@")?replicaTextV1(language,value.slice(1)):value;
   context.notice(replicaTextV1(language,key,resolved));
  }
 };
}

const idle=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise<void>(done=>{
 const finish=():void=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();},timer=setTimeout(finish,milliseconds);
 signal.addEventListener("abort",finish,{once:true});
});

const stream=(context:ReplicaCommandContextV1)=>(event:ReplicaEventV1):void=>{context.write(JSON.stringify(event));};

const accessHint=(platform:string):string=>`@access.hint.${platform==="darwin"||platform==="macos"?"macos":"other"}`;

function showStatus(report:StatusReportV1,say:(key:string,variables?:Variables)=>void):void{
 if(report.access!==undefined)say(`status.access.${report.access.state==="denied"?"denied":"waiting-for-consent"}`,{worktree:report.access.worktree,platform:report.access.platform,hint:accessHint(report.access.platform)});
 say("status.header",{origin:report.originId,group:report.groupId,store:report.storeRoot,state:report.reachable?"@status.reachable":"@status.unreachable"});
 if(report.sync)say("status.sync.on");else say("status.sync.off");
 if(report.daemon.running)say("status.daemon.running",{pid:report.daemon.pid??0});else say("status.daemon.stopped");
 say("status.queue",{files:report.files,incoming:report.queue.incoming,outgoing:report.queue.outgoing??"?",lost:report.lost});
 if(report.refused!==null&&report.refused>0)say("status.refused",{refused:report.refused});
 if(report.retired!==null)say("status.retired",{origin:report.originId,by:report.retired.by,reason:report.retired.reason===""?"-":report.retired.reason});
 if(report.daemon.busySeconds!==undefined)say("status.daemon.busy",{seconds:report.daemon.busySeconds});
 if(report.code!==undefined&&report.code.source!=="worktree")say("status.code.fallback",{source:`@status.code.${report.code.source}`,phase:`@status.code.phase.${report.code.phase}`});
 if(report.health!==null){
  const seconds=(since:number):number=>Math.max(0,Math.floor((Date.now()-since)/1000));
  if(report.health.store!=="ok")say("status.health.store",{state:report.health.store,seconds:seconds(report.health.storeSinceMs)});
  if(report.health.pending!==null)say("status.health.pending",{digest:report.health.pending.code.slice(0,12),reason:report.health.pending.reason});
  if(report.health.trust.verifiedMs===null)say("status.health.trust.pending");
  else if(report.health.trust.lostMs!==null&&report.health.trust.verifiedMs<=report.health.trust.lostMs)say("status.health.trust.lost",{seconds:seconds(report.health.trust.lostMs),kind:report.health.trust.lostKind??"-"});
  for(const failure of report.health.failures)say("status.health.failure",{kind:failure.kind,count:failure.count,seconds:seconds(failure.lastMs)});
  for(const item of report.health.sidecars??[])say("status.health.sidecar",{name:item.name,state:`@status.sidecar.${item.state}`,pid:item.pid??"-",restarts:item.restarts,seconds:seconds(item.sinceMs),exit:item.lastExit??"-"});
 }
 if(report.store!==null){
  say("status.store.journal",{origins:report.store.journal.origins,segments:report.store.journal.segments,bytes:formatReplicaBytesV1(report.store.journal.bytes)});
  if(report.store.snapshot===null)say("status.store.nosnapshot");
  else say("status.store.snapshot",{origin:report.store.snapshot.origin,age:`${report.store.snapshot.ageSeconds}s`,bytes:formatReplicaBytesV1(report.store.snapshot.bytes)});
  say("status.store.reclaim",{segments:report.store.reclaimable.segments,bytes:formatReplicaBytesV1(report.store.reclaimable.bytes)});
  if(report.store.reclaimable.blobs!==null)say("status.store.blobs",{blobs:report.store.reclaimable.blobs.count,bytes:formatReplicaBytesV1(report.store.reclaimable.blobs.bytes),pending:report.store.reclaimable.blobs.pending,pendingbytes:formatReplicaBytesV1(report.store.reclaimable.blobs.pendingBytes)});
  if(report.store.last!==null)say("status.store.last",{age:`${Math.max(0,Math.floor((Date.now()-report.store.last.wallMs)/1000))}s`,blobs:report.store.last.blobs,bytes:formatReplicaBytesV1(report.store.last.blobBytes),pending:report.store.last.pending,pendingbytes:formatReplicaBytesV1(report.store.last.pendingBytes)});
 }
 say(`status.git.${report.git.reason}`,{tree:report.git.indexTree??"-"});
 if(report.latency!==null){const tier=(row:Readonly<{passes:number;lastMs:number|null;dutyPercent:number|null}>):string=>row.lastMs===null?"-":`${row.lastMs} ms ${row.dutyPercent}%`,rows=report.latency.tiers;say("status.tiers",{hot:tier(rows.hot),sweep:tier(rows.sweep),file:tier(rows.file),walk:tier(rows.walk),view:tier(rows.view)});}
 if(report.latency!==null)say("status.latency",{publish:report.latency.samples.publish,apply:report.latency.samples.apply,publishp50:report.latency.publishMs.p50??"-",publishp95:report.latency.publishMs.p95??"-",applyp50:report.latency.applyMs.p50??"-",applyp95:report.latency.applyMs.p95??"-"});
 for(const lag of report.origins){
  if(lag.retired)say("status.origin.retired",{origin:lag.origin});
  else if(lag.rejoinRequired)say("status.origin.rejoin",{origin:lag.origin});
  else say("status.origin",{origin:lag.origin,events:lag.behindEvents,bytes:formatReplicaBytesV1(lag.behindBytes),age:lag.ageSeconds===null?"-":`${lag.ageSeconds}s`});
 }
}

const spellQuantiles=(quantiles:Readonly<{p50:number|null;p95:number|null}>):string=>`${quantiles.p50??"-"}/${quantiles.p95??"-"}`;
const spellLatency=(latency:LatencyReportV1|null):string=>latency===null?"-":`${spellQuantiles(latency.publishMs)}|${spellQuantiles(latency.applyMs)}`;

const differsKey=(report:FleetReportV1,item:Readonly<{originId:string;field:string}>):string=>report.rows.find(row=>row.originId===item.originId)?.basis==="checkpoint"&&["manifest","head.commit","indexTree","view.digest"].includes(item.field)?"fleet.differs.checkpoint":"fleet.differs";

async function showPeerStores(groupId:string,originId:string,say:(key:string,variables?:Variables)=>void):Promise<void>{
 if(originId!==`${groupId}-server`)return;
 let peers:readonly (readonly [string,string])[];
 try{const topology=readTopologyV1(),group=groupV1(topology,groupId);peers=Object.keys(group.peers).sort().map(peer=>[peer,peerStorePathV1(topology,group,peer)] as const);}catch{return;}
 for(const [peer,path] of peers)if(!await storeReachableV1(path,3000))say("status.peer.unreachable",{group:peer,path,own:groupId});
}

function showFleet(report:FleetReportV1,say:(key:string,variables?:Variables)=>void,header:string):void{
 say(header);
 for(const row of report.rows)say("fleet.row",{origin:row.originId,platform:row.platform??"-",age:row.ageSeconds===null?"-":`${row.ageSeconds}s`,lag:row.behindOrigins===null?"-":`${row.behindOrigins}/${row.incoming}`,basis:row.basis===undefined?"now":`cp:${row.checkpointSeconds??"-"}${row.checkpointSeconds===null||row.checkpointSeconds===undefined?"":"s"}`,sync:row.sync===null?"-":row.sync?"@fleet.on":"@fleet.off",code:row.code??"-",view:row.view===null?"-":`${row.view.modified}/${row.view.deleted}/${row.view.untracked}`,latency:spellLatency(row.latency),verdict:`@fleet.verdict.${row.verdict}`});
 for(const item of report.warnings)say(`fleet.warning.${item.kind}`,{origin:item.originId,detail:item.detail});
 for(const item of report.differing)say(differsKey(report,item),{origin:item.originId,field:item.field,value:item.value?.slice(0,80)??"-",expected:item.expected?.slice(0,80)??"-"});
 say(report.ok?"fleet.ok":"fleet.failed",{origins:report.rows.length,fresh:report.freshSeconds});
}

const viewCap=40;

async function viewDiff(originId:string,storeRoot:string,signal:AbortSignal,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const report=await viewDiffV1(storeRoot,originId,signal);
 const fleet=await fleetReportV1(storeRoot,Date.now(),signal);
 for(const row of fleet.rows.filter(item=>item.verdict!=="ok"&&item.verdict!=="off"&&item.verdict!=="retired"))say("fleet.diff.row",{origin:row.originId,verdict:`@fleet.verdict.${row.verdict}`,fields:row.differs.join(", ")||"-"});
 for(const item of fleet.differing)say(differsKey(fleet,item),{origin:item.originId,field:item.field,value:item.value?.slice(0,80)??"-",expected:item.expected?.slice(0,80)??"-"});
 const diverged=fleet.rows.filter(row=>row.verdict==="diverged").length;
 if(report.here===null){say("fleet.diff.nohere",{origin:originId});return {exitCode:1};}
 for(const origin of report.unavailable)say("fleet.diff.unavailable",{origin});
 for(const item of report.differing){
  say("fleet.diff.origin",{origin:item.originId,there:item.onlyThere.length,here:item.onlyHere.length});
  for(const line of item.onlyThere.slice(0,viewCap))say("fleet.diff.there",{origin:item.originId,line});
  if(item.onlyThere.length>viewCap)say("fleet.diff.more",{count:item.onlyThere.length-viewCap});
  for(const line of item.onlyHere.slice(0,viewCap))say("fleet.diff.here",{origin:originId,line});
  if(item.onlyHere.length>viewCap)say("fleet.diff.more",{count:item.onlyHere.length-viewCap});
 }
 const bad=report.differing.length+report.unavailable.length;
 const checkpointed=fleet.rows.some(row=>row.basis==="checkpoint");
 if(checkpointed&&diverged===0&&bad>0)say("fleet.diff.moving",{count:bad});
 else say(bad===0?"fleet.diff.same":"fleet.diff.failed",{origin:originId,count:bad});
 return {exitCode:(checkpointed?diverged===0:bad===0)?0:1};
}

const nextHlc=(now:number,previous:string|undefined,originId:string):string=>{
 const last=previous===undefined?undefined:parseHlcV1(previous);
 return last!==undefined&&last.ms>=now?formatHlcV1({ms:last.ms,counter:last.counter+1},originId):formatHlcV1({ms:now,counter:0},originId);
};

async function retire(parsed:ReplicaArgumentsV1,options:ReplicaOptionsV1,storeRoot:string,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const origin=parsed.target,undo=parsed.flags.has("--undo"),force=parsed.flags.has("--force"),reason=parsed.values["--reason"];
 if(origin===undefined)throw Error(both("error.retire.origin"));
 if(undo&&(force||reason!==undefined))throw Error(both("error.retire.options"));
 if(origin===options.originId){say("error.retire.own",{origin});return {exitCode:1};}
 if(!await storeReachableV1(storeRoot)){say("error.retire.store",{store:storeRoot});return {exitCode:1};}
 if(!(await listOriginsV1(storeRoot)).includes(origin)){say("error.retire.unknown",{origin,store:storeRoot});return {exitCode:1};}
 const current=await readRetirementV1(storeRoot,origin),now=Date.now();
 if(undo&&current?.retired!==true){say("retire.notretired",{origin});return {exitCode:0};}
 if(!undo){
  if(current?.retired===true){say("retire.already",{origin,by:current.by});return {exitCode:0};}
  const attested=await readAttestationV1(storeRoot,origin),age=attested===undefined?undefined:Math.max(0,Math.floor((now-attested.wallMs)/1000));
  if(!force&&attested===undefined&&await stat(attestPathV1(storeRoot,origin)).then(()=>true,()=>false)){say("error.retire.unreadable",{origin});return {exitCode:1};}
  if(!force&&age!==undefined&&age<=freshSecondsV1){say("error.retire.alive",{origin,age});return {exitCode:1};}
 }
 const hlc=nextHlc(now,current?.hlc,options.originId);
 await writeRetirementV1(storeRoot,{version:1,originId:origin,by:options.originId,hlc,wallMs:parseHlcV1(hlc).ms,reason:undo?"":reason??"",retired:!undo});
 say(undo?"retire.undone":"retire.done",{origin,by:options.originId});
 return {exitCode:0};
}

const conflictLimit=50,workLimit=1000;

const showPlan=(plan:AdoptPlanV1,say:(key:string,variables?:Variables)=>void):void=>{
 if(plan.renamed.length>0){
  say("adopt.plan.renamed",{count:plan.renamed.length});
  for(const path of plan.renamed.slice(0,20))say("adopt.plan.renamed.path",{path});
  if(plan.renamed.length>20)say("adopt.plan.renamed.more",{count:plan.renamed.length-20});
 }
 for(const category of Object.keys(plan.counts) as AdoptCategoryV1[]){
  if(plan.counts[category]===0)continue;
  say("adopt.plan.count",{count:plan.counts[category],label:`@adopt.category.${category}`});
  for(const path of plan.samples[category])say("adopt.plan.sample",{path});
 }
 for(const category of ["conflict","stale","edit"] as const){
  const items=plan.decisions.filter(item=>item.category===category),shown=items.slice(0,category==="conflict"?1000:20);
  for(const item of shown)say(item.modeOnly?"adopt.plan.decision.mode":"adopt.plan.decision",{path:item.path,label:`@adopt.category.${item.category}`,reason:`@adopt.reason.${item.reason}`});
  if(items.length>shown.length)say("adopt.plan.decisions.more",{count:items.length-shown.length,label:`@adopt.category.${category}`});
 }
 for(const path of plan.nested)say("adopt.plan.nested",{path});
 say("adopt.plan.git",{fleet:plan.git.fleetTip?.slice(0,12)??"-",local:plan.git.localTip?.slice(0,12)??"-",relation:plan.git.relation,staged:plan.git.stagedPaths??"-"});
 say("adopt.plan.evidence",{events:plan.evidence.events,commits:plan.evidence.commits});
 say("adopt.plan.conflicts",{count:plan.counts.conflict,limit:conflictLimit});
 say("adopt.plan.work",{count:plan.counts.publish+plan.counts.edit,limit:workLimit});
};

async function adopt(parsed:ReplicaArgumentsV1,resolved:Readonly<{options:ReplicaOptionsV1;worktree:string;storeRoot:string}>,context:ReplicaCommandContextV1,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const {options,worktree,storeRoot}=resolved,dryRun=parsed.flags.has("--dry-run"),sample=parsed.values["--sample"]===undefined?20:whole("--sample",parsed.values["--sample"],0,1000);
 say(dryRun?"join.adopt.start.dry":"join.adopt.start",{worktree,store:storeRoot,origin:options.originId,group:options.groupId});
 try{
  const result=await adoptV1(options,{dryRun,force:parsed.flags.has("--force"),sample},context.signal,stream(context));
  showPlan(result.plan,say);
  if(dryRun){say("adopt.plan.done");return {exitCode:0};}
  const {counts,stamp}=result.report!;
  say("adopt.done",{same:counts.same,materialise:counts.materialise,publish:counts.publish,edit:counts.edit,stale:counts.stale,conflict:counts.conflict,queued:result.queued,report:`.cache/f/lost/adopt-${stamp}.md`});
  return {exitCode:result.queued>0?1:0};
 }catch(error){
  if(error instanceof AdoptRefusedV1){say(error.reason==="already-joined"?"error.adopt.joined":"error.adopt.history");return {exitCode:1};}
  throw error;
 }
}

async function relocate(parsed:ReplicaArgumentsV1,context:ReplicaCommandContextV1,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const values=parsed.values,target=values["--to"],phase=values["--phase"]??"all";
 if(target===undefined)throw Error(both("error.relocate.to"));
 if(!["prepare","move","finish","all"].includes(phase))throw Error(both("error.relocate.phase"));
 const from=where(context.root,values["--worktree"]??context.root),to=where(context.root,target),dryRun=parsed.flags.has("--dry-run");
 say("relocate.start",{from,to,phase});
 try{
  const result=await relocateV1({from,to,phase:phase as RelocatePhaseV1,dryRun,force:parsed.flags.has("--force"),sample:values["--sample"]===undefined?20:whole("--sample",values["--sample"],0,1000),waitSeconds:values["--wait"]===undefined?0:whole("--wait",values["--wait"],0,86400)},context.signal,stream(context));
  if(result.plan!==undefined){showPlan(result.plan,say);say("adopt.plan.done");return {exitCode:0};}
  if(result.verify!==undefined)say("relocate.verified",{checked:result.verify.checked});
  say("relocate.done",{step:result.step,origin:(await machineOf(to)??await machineOf(from))?.originId??"-",from,to});
  return {exitCode:0};
 }catch(error){
  if(error instanceof RelocateRefusedV1){say(`error.relocate.${error.reason}`);return {exitCode:1};}
  if(error instanceof AdoptRefusedV1){say(error.reason==="already-joined"?"error.adopt.joined":"error.adopt.history");return {exitCode:1};}
  throw error;
 }
}

async function touch(parsed:ReplicaArgumentsV1,context:ReplicaCommandContextV1,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const hook=parsed.flags.has("--hook"),stdin=parsed.flags.has("--stdin"),worktree=where(context.root,parsed.values["--worktree"]??context.root),read=context.readStdin??(async()=>"");
 if(hook){
  try{
   if(await machineOf(worktree)===undefined)return {exitCode:0};
   const accepted:string[]=[];
   for(const path of parseHookInputV1(await read()))if(await normaliseHintPathV1(worktree,path)!==undefined)accepted.push(path);
   if(accepted.length>0)await writeHintV1(worktree,accepted);
  }catch{}
  return {exitCode:0};
 }
 if((parsed.paths.length>0?1:0)+(stdin?1:0)!==1)throw Error(both("error.touch"));
 if(await machineOf(worktree)===undefined)throw Error(both("error.identity"));
 const accepted:string[]=[];
 for(const path of stdin?parseHintTextV1(await read()):parsed.paths)if(await normaliseHintPathV1(worktree,path)!==undefined)accepted.push(path);
 if(accepted.length===0){say("touch.none",{worktree});return {exitCode:1};}
 await writeHintV1(worktree,accepted);
 say("touch.done",{count:accepted.length,worktree});
 return {exitCode:0};
}

async function execute(parsed:ReplicaArgumentsV1,context:ReplicaCommandContextV1,say:(key:string,variables?:Variables)=>void):Promise<ReplicaCommandResultV1>{
 const values=parsed.values,sink=stream(context),signal=context.signal;
 if(parsed.operation==="relay"){
  const machine=await machineOf(where(context.root,values["--worktree"]??context.root)),groupId=values["--group"]??machine?.groupId;
  if(groupId===undefined)throw Error(both("error.group"));
  const topology=readTopologyV1(),group=groupV1(topology,groupId),given=values["--store"],ownStore=given!==undefined?where(context.root,given):machine?.storeRoot??topology.storeMount[platformV1()];
  const peers=values["--peers"]===undefined?group.peers as Readonly<Record<string,string>>:peersOf(context.root,values["--peers"]),poll=values["--poll"]===undefined?topology.replica.pollMilliseconds*4:whole("--poll",values["--poll"],10,3600000);
  const concurrency=values["--concurrency"]===undefined?undefined:whole("--concurrency",values["--concurrency"],1,64);
  say("relay.start",{group:groupId,peers:Object.keys(peers).length,store:ownStore});
  if(parsed.flags.has("--once")){
   let bytes=0;
   for(const [peerGroup,peerStore] of Object.entries(peers)){
    const result=await relayOnceV1(ownStore,peerStore,await homedOriginsV1(peerStore,peerGroup),signal,sink,true,concurrency);
    bytes+=result.bytes;
   }
   say("relay.done",{bytes:formatReplicaBytesV1(bytes)});
   return {exitCode:0};
  }
  const consolidateMinutes=values["--consolidate-minutes"]===undefined?1440:whole("--consolidate-minutes",values["--consolidate-minutes"],0,10080);
  const minutes=values["--bootstrap-minutes"]===undefined?topology.replica.bootstrapMinutes:whole("--bootstrap-minutes",values["--bootstrap-minutes"],1,10080);
  const relaySink=(event:Parameters<typeof sink>[0]):void=>{sink(event);if(event.event==="store-unreachable"&&event.groupId!==undefined)say("relay.peer.unreachable",{group:event.groupId,path:event.path??"-",own:groupId});else if(event.event==="store-reachable"&&event.groupId!==undefined)say("relay.peer.reachable",{group:event.groupId});};
  await Promise.all([relayV1({ownStore,peers,pollMilliseconds:poll,concurrency},signal,relaySink),machine===undefined||consolidateMinutes===0?Promise.resolve():refreshConsolidationV1({worktree:where(context.root,values["--worktree"]??context.root),storeRoot:ownStore,minutes:consolidateMinutes},signal,sink),machine===undefined?Promise.resolve():refreshBootstrapV1({worktree:where(context.root,values["--worktree"]??context.root),storeRoot:ownStore,originId:machine.originId,minutes},signal,sink),machine===undefined||machine.originId!==`${machine.groupId}-server`?Promise.resolve():refreshKitV1({root:where(context.root,values["--worktree"]??context.root),moduleRoot:resolve(import.meta.dirname,"..",".."),store:ownStore,originId:machine.originId,digestOf:codeDigestV1,attested:async()=>(await readAttestationV1(ownStore,machine.originId))?.code,minutes,sink},signal)]);
  say("relay.stopped");
  return {exitCode:0};
 }
 if(parsed.operation==="touch")return touch(parsed,context,say);
 if(parsed.operation==="bootstrap-archive"||parsed.operation==="bootstrap-settle")return {exitCode:await runBootstrapCommandV1(parsed.operation,parsed.flags.has("--force")?{...values,"--force":"1"}:values,{root:context.root,signal,write:context.write,say,format:formatReplicaBytesV1})};
 if(parsed.operation==="relocate")return relocate(parsed,context,say);
 if(parsed.operation==="self-check"){
  const root=resolve(import.meta.dirname,"..",".."),checked0=await selfCheckV1(root,signal),state=checked0.ok?await dryOpenReplicaV1(context.root):null,smoke=checked0.ok&&state===null?await smokeBootV1(signal):null,checked=state!==null?{...checked0,ok:false,error:`local state cannot be opened: ${state}`}:smoke!==null?{...checked0,ok:false,error:`the daemon does not boot: ${smoke}`}:checked0,report={...checked,digest:checked.ok?await codeDigestV1(root).catch(()=>null):null};
  context.write(JSON.stringify(report));
  if(report.ok)say("selfcheck.ok",{modules:report.modules,schemas:report.schemas,texts:report.texts});else say("selfcheck.failed",{error:report.error??"-"});
  return {exitCode:report.ok?0:1};
 }
 const resolved=await resolveOptions(parsed,context),{options,storeRoot}=resolved;
 if(parsed.operation==="sync"){
  const on=parsed.flags.has("--on"),off=parsed.flags.has("--off");
  if(on===off)throw Error(both("error.sync"));
  await setSyncEnabledV1(resolved.worktree,on);
  say(on?"sync.on":"sync.off",{worktree:resolved.worktree});
  return {exitCode:0};
 }
 if(parsed.operation==="repair"){
  await requestRepairV1(resolved.worktree);
  say("repair.requested",{worktree:resolved.worktree});
  return {exitCode:0};
 }
 if(parsed.operation==="rebase"){
  await requestRebaseV1(resolved.worktree);
  say("rebase.requested",{worktree:resolved.worktree});
  return {exitCode:0};
 }
 if(parsed.operation==="retire")return retire(parsed,options,storeRoot,say);
 if(parsed.operation==="fleet"){
  if(parsed.flags.has("--diff")){
   if(parsed.flags.has("--json")||values["--wait"]!==undefined)throw Error(both("error.fleet.diff"));
   return viewDiff(options.originId,storeRoot,signal,say);
  }
  const wait=values["--wait"]===undefined?0:whole("--wait",values["--wait"],0,86400),deadline=Date.now()+wait*1000;
  let report=await fleetReportV1(storeRoot,Date.now(),signal);
  while(!report.ok&&wait>0&&Date.now()<deadline&&!signal.aborted){
   say("fleet.waiting",{seconds:Math.max(0,Math.ceil((deadline-Date.now())/1000)),pending:report.rows.filter(row=>row.verdict!=="ok"&&row.verdict!=="off"&&row.verdict!=="retired").length});
   await idle(Math.min(1000,Math.max(0,deadline-Date.now())),signal);
   if(!signal.aborted)report=await fleetReportV1(storeRoot,Date.now(),signal);
  }
  if(signal.aborted)return {exitCode:130};
  if(parsed.flags.has("--json")){requireAttestShapeV1("Report",report,"Closed fleet report required");context.write(JSON.stringify(report));}
  else showFleet(report,say,"fleet.header");
  const blocked=await listAccessV1(storeRoot);
  for(const item of blocked)say("fleet.access",{origin:item.originId,worktree:item.worktree,state:item.state});
  if(!parsed.flags.has("--json"))await showPeerStores(options.groupId,options.originId,say);
  return {exitCode:report.ok&&blocked.length===0?0:1};
 }
 if(parsed.operation==="seed"){
  say("seed.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  const result=await seedV1(options,signal,sink);
  await installBootStageV1(resolved.worktree,join(import.meta.dirname,"..","🛟️supervisor","🟦️.ts")).catch(()=>{});
  say("seed.done",{files:result.published,bytes:formatReplicaBytesV1(result.bytes)});
  return {exitCode:0};
 }
 if(parsed.operation==="join"&&parsed.flags.has("--adopt"))return adopt(parsed,resolved,context,say);
 if(parsed.operation==="join"&&(parsed.flags.has("--dry-run")||parsed.flags.has("--force")||values["--sample"]!==undefined))throw Error(both("error.adopt.options"));
 if(parsed.operation==="join"){
  say("join.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  try{
   const result=await joinV1(options,signal,sink);
   await installBootStageV1(resolved.worktree,join(import.meta.dirname,"..","🛟️supervisor","🟦️.ts")).catch(()=>{});
   say("join.done",{files:result.files,adopted:result.adopted,queued:result.queued});
   return {exitCode:result.queued>0?1:0};
  }catch(error){
   if(error instanceof JoinRefusedV1){say("error.refused",{count:error.conflicts.length,paths:error.conflicts.slice(0,5).join(", ")});return {exitCode:1};}
   throw error;
  }
 }
 if(parsed.operation==="run"){
  say("run.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  let restarting=false;
  const fallback=readFallbackV1(context.environment);
  await runV1({...options,watchRoot:context.environment.SEMIO_FLEET_CODE_WATCH,...fallback===undefined?{}:{fallback}},signal,event=>{
   if(event.event==="restarting")restarting=true;
   if(event.event==="restart-refused")say("event.restart-refused",{digest:(event.reason??"").slice(0,12),reason:event.message??"-"});
   if(event.event==="code-reverted")say("event.code-reverted",{digest:(event.reason??"").slice(0,12),reason:event.message??"-"});
   if(event.event==="retired")say("event.retired",{origin:options.originId,by:event.origin??"-",reason:event.message===""?"-":event.message??"-"});
   if(event.event==="superseded")say("event.superseded",{path:event.path??"",winner:event.winner??"",reason:event.reason??"-"});
   if(event.event==="stale-republish")say("event.stale-republish",{path:event.path??""});
   if(event.event==="drift")say("event.drift",{fields:event.reason??"-",attempt:event.count??0});
   if(event.event==="linearised")say(event.reason==="duplicate"?"event.linearised.duplicate":(event.count??0)>0?"event.linearised.conflict":"event.linearised",{path:event.path??"",winner:(event.winner??"").slice(0,12),loser:(event.loser??"").slice(0,12),result:(event.result??"").slice(0,12),count:event.count??0});
   sink(event);
  });
  say(restarting?"run.restarting":"run.stopped");
  return {exitCode:0};
 }
 if(parsed.operation==="status"){
  const report=await statusV1(options,parsed.flags.has("--deep"),signal);
  if(parsed.flags.has("--json"))context.write(JSON.stringify(report));
  else{
   showStatus(report,say);
   if(report.reachable){const fleet=await fleetReportV1(storeRoot,Date.now(),signal).catch(()=>undefined);if(fleet!==undefined&&(fleet.rows.length>0||fleet.warnings.length>0))showFleet(fleet,say,"status.fleet.header");}
   await showPeerStores(options.groupId,options.originId,say);
  }
  return {exitCode:0};
 }
 if(parsed.operation==="snapshot"){
  const result=await snapshotV1(storeRoot,options.originId,signal,sink);
  if(result===undefined)say("snapshot.empty");else say("snapshot.done",{entries:result.entries,file:result.file,state:result.created?"@snapshot.created":"@snapshot.existing"});
  return {exitCode:0};
 }

 if(parsed.operation==="consolidate"){
  const result=await consolidateGitV1({worktree:options.worktree,apply:parsed.flags.has("--apply"),storeRoot,signal,sink:event=>sink(event)});
  say(`consolidate.${result.outcome}`,{packs:result.packsBefore,bytes:formatReplicaBytesV1(result.bytesBefore),retired:result.retired,freed:formatReplicaBytesV1(result.bytesFreed)});
  return {exitCode:["failed","unpublished","space","locked"].includes(result.outcome)?1:0};
 }
 if(parsed.operation==="compaction"){
  const plan=parsed.flags.has("--plan"),apply=parsed.flags.has("--apply");
  if(plan===apply)throw Error(both("error.compaction"));
  const result=await fleetCompactV1(storeRoot,options.originId,options.compaction!,apply?"apply":"plan",signal,sink,Date.now,{blobs:!parsed.flags.has("--no-blobs")});
  if(result.skipped==="locked"||result.skipped==="refused"||result.skipped==="empty"||result.skipped==="nosnapshot"){say(`compaction.${result.skipped}`);return {exitCode:result.skipped==="locked"||result.skipped==="refused"?1:0};}
  for(const [origin,why] of Object.entries(result.ignored))say("compaction.ignored",{origin,why:`@compaction.why.${why}`});
  say(apply?"compaction.done":"compaction.plan",{entries:result.entries,segments:result.segments,segmentbytes:formatReplicaBytesV1(result.segmentBytes),blobs:result.blobs,blobbytes:formatReplicaBytesV1(result.blobBytes),pending:result.pending,pendingbytes:formatReplicaBytesV1(result.pendingBytes),snapshots:result.snapshots});
  if(result.skipped==="aborted")say("compaction.aborted");
  return {exitCode:result.skipped==="aborted"?1:0};
 }
 if(parsed.operation==="compact"){
  const result=await compactV1(storeRoot,options.originId,options.retentionDays!,signal,sink,Date.now,{collectBlobs:parsed.flags.has("--collect-blobs")});
  say("compact.done",{segments:result.segments,blobs:result.blobs,snapshots:result.snapshots,staging:result.staging,bytes:formatReplicaBytesV1(result.bytes)});
  return {exitCode:0};
 }
 const report=await verifyV1(options,signal,sink);
 if(parsed.flags.has("--json")){requireReplicaShapeV1("VerifyReport",report,"Closed replica verify report required");context.write(JSON.stringify(report));return {exitCode:report.ok?0:1};}
 if(report.ok)say("verify.ok",{checked:report.checked});
 else{
  say("verify.drift",{count:report.total,checked:report.checked});
  for(const item of report.drift.slice(0,50))say("verify.item",{kind:`@verify.kind.${item.kind}`,path:item.path});
  if(report.total>50)say("verify.more",{count:report.total-50});
 }
 if(report.refusedTotal>0){
  say("verify.refused",{count:report.refusedTotal});
  for(const item of report.refused.slice(0,50))say("verify.refused.item",{reason:item.reason,path:item.path});
  if(report.refusedTotal>50)say("verify.more",{count:report.refusedTotal-50});
 }
 return {exitCode:report.ok?0:1};
}

/** 🎬️ Routes `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle|sync|fleet|rebase|repair|retire|touch|self-check`; `self-check` loads every module of the replica, compiles every embedded schema, validates every text catalogue, opens the local state on a private mirror with the checked code and boots the checked daemon once in a scratch worktree and store (it never writes the real state or the real store), prints one report line and exits 0 only when all is well (the daemon runs it in a fresh process on new code before it restarts); `args` are the words after the script name. Streaming operations write NDJSON events to stdout and human messages to stderr; `status --json` and `verify --json` write one document; `verify` exits non-zero on drift. `sync --off` stops publish and apply on this machine until `sync --on`. `retire <origin> [--reason text] [--force]` writes the retirement record of a silent origin into its store directory (refused for the own origin, an unknown origin, or one that attested within 60 s unless `--force`); `retire <origin> --undo` re-admits it. `fleet --diff` compares the working-tree view (`git status`, `origins/<origin>/view.txt`) of this machine with every other origin that attests one and prints, per differing origin, the lines only there and only here (40 per side, then `+N more`); exit 1 when any differs or a view is unavailable. `touch <path>…`, `touch --stdin` (paths separated by newlines or NUL) and `touch --hook` (a Claude Code `PostToolUse` payload on stdin: `tool_input.file_path` and `tool_input.notebook_path`) drop a hint file into `.cache/f/hints/` of the worktree, which the running replica consumes within about 50 ms and publishes at once; relative paths are relative to the worktree, paths outside it are ignored, `touch` exits 1 when nothing is left to hand over, and `touch --hook` exits 0 whatever it reads or finds. */
export async function runReplicaCommandV1(args:readonly string[],context:ReplicaCommandContextV1):Promise<ReplicaCommandResultV1>{
 const parsed=parseReplicaArgumentsV1(args),say=reporter(context,replicaLanguagesV1(parsed.values["--lang"],context.environment));
 try{return await execute(parsed,context,say);}
 catch(error){
  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:parsed.operation==="run"?75:1};}
  if(error instanceof OriginTakenV1){say("error.origin.taken",{origin:error.originId});return {exitCode:parsed.operation==="run"?75:1};}
  if(context.signal.aborted)return {exitCode:130};
  throw error;
 }
}
