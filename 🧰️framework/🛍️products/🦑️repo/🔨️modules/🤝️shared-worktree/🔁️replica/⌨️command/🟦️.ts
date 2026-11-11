import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {readFile} from "node:fs/promises";
import {isAbsolute,resolve} from "node:path";
import {groupV1,platformV1,readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {compactV1,homedOriginsV1,joinV1,JoinRefusedV1,relayOnceV1,relayV1,ReplicaBusyV1,requireReplicaShapeV1,runV1,seedV1,snapshotV1,statusV1,verifyV1,type ReplicaEventV1,type ReplicaOptionsV1,type StatusReportV1} from "../🟦️.ts";
import {refreshBootstrapV1,runBootstrapCommandV1} from "../🥾️bootstrap/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};

/** 🎛️ Carries the issued phase into the command: where the repository is, the environment, cancellation, the stdout line sink (NDJSON events and reports) and the stderr line sink (human messages). */
export type ReplicaCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;write:(line:string)=>void;notice:(line:string)=>void}>;

/** 🏁️ How a command ended: the process exit code. */
export type ReplicaCommandResultV1=Readonly<{exitCode:number}>;

/** 🗣️ The languages of all messages; there is no default among them. */
export type ReplicaLanguageV1="en"|"de";

type Operation="seed"|"join"|"run"|"status"|"snapshot"|"relay"|"compact"|"verify"|"bootstrap-archive"|"bootstrap-settle";
type Variables=Readonly<Record<string,string|number>>;

const LANGUAGES=texts.languages as readonly ReplicaLanguageV1[],
 IDENTITY=["--worktree","--store","--group","--origin","--lang"],
 OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
  seed:{valued:IDENTITY,plain:[]},
  join:{valued:IDENTITY,plain:[]},
  run:{valued:["--worktree","--store","--poll","--scan","--debounce","--lang"],plain:[]},
  status:{valued:["--worktree","--store","--lang"],plain:["--json","--deep"]},
  snapshot:{valued:["--worktree","--store","--lang"],plain:[]},
  relay:{valued:["--store","--group","--worktree","--peers","--poll","--bootstrap-minutes","--concurrency","--lang"],plain:["--once"]},
  compact:{valued:["--worktree","--store","--retention-days","--lang"],plain:["--collect-blobs"]},
  verify:{valued:["--worktree","--store","--lang"],plain:["--json"]},
  "bootstrap-archive":{valued:["--worktree","--store","--lang"],plain:[]},
  "bootstrap-settle":{valued:["--worktree","--store","--lang"],plain:[]}
 };

/** 💬️ Renders one catalogue message in one language. */
export function replicaTextV1(language:ReplicaLanguageV1,key:string,variables:Variables={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<ReplicaLanguageV1,string>>>>)[key];
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

/** 📝️ The closed arguments of one replica operation: valued options by name and the plain flags that were given. */
export type ReplicaArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>}>;

/** 🧭️ Reads `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle` with its options (`--name value` or `--name=value`); every unknown, misplaced or repeated option is refused in both languages. */
export function parseReplicaArgumentsV1(args:readonly string[]):ReplicaArgumentsV1{
 const operation=args[0];
 if(operation===undefined||!Object.hasOwn(OPTIONS,operation))throw Error(both("error.syntax"));
 const allowed=OPTIONS[operation as Operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>();
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  if(allowed.valued.includes(name)){
   const value=cut<0?args[++index]:argument.slice(cut+1);
   if(value===undefined||value===""||value.startsWith("--"))throw Error(both("error.value",{option:name}));
   values[name]=value;
  }else if(allowed.plain.includes(name)&&cut<0)flags.add(name);
  else throw Error(both("error.option",{operation,option:argument}));
  seen.add(name);
 }
 return Object.freeze({operation:operation as Operation,values:Object.freeze(values),flags});
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
 return {worktree,storeRoot,options:{worktree,storeRoot,originId,groupId,pollMilliseconds:limits("--poll",defaults.pollMilliseconds,10,60000),scanSeconds:limits("--scan",defaults.scanSeconds,1,3600),debounceMilliseconds:limits("--debounce",defaults.debounceMilliseconds,0,10000),retentionDays:limits("--retention-days",defaults.retentionDays,0,3650)}};
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

const stream=(context:ReplicaCommandContextV1)=>(event:ReplicaEventV1):void=>{context.write(JSON.stringify(event));};

function showStatus(report:StatusReportV1,say:(key:string,variables?:Variables)=>void):void{
 say("status.header",{origin:report.originId,group:report.groupId,store:report.storeRoot,state:report.reachable?"@status.reachable":"@status.unreachable"});
 if(report.daemon.running)say("status.daemon.running",{pid:report.daemon.pid??0});else say("status.daemon.stopped");
 say("status.queue",{files:report.files,incoming:report.queue.incoming,outgoing:report.queue.outgoing??"?",lost:report.lost});
 if(report.refused!==null&&report.refused>0)say("status.refused",{refused:report.refused});
 for(const lag of report.origins){
  if(lag.rejoinRequired)say("status.origin.rejoin",{origin:lag.origin});
  else say("status.origin",{origin:lag.origin,events:lag.behindEvents,bytes:formatReplicaBytesV1(lag.behindBytes),age:lag.ageSeconds===null?"-":`${lag.ageSeconds}s`});
 }
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
  const minutes=values["--bootstrap-minutes"]===undefined?topology.replica.bootstrapMinutes:whole("--bootstrap-minutes",values["--bootstrap-minutes"],1,10080);
  await Promise.all([relayV1({ownStore,peers,pollMilliseconds:poll,concurrency},signal,sink),machine===undefined?Promise.resolve():refreshBootstrapV1({worktree:where(context.root,values["--worktree"]??context.root),storeRoot:ownStore,originId:machine.originId,minutes},signal,sink)]);
  say("relay.stopped");
  return {exitCode:0};
 }
 if(parsed.operation==="bootstrap-archive"||parsed.operation==="bootstrap-settle")return {exitCode:await runBootstrapCommandV1(parsed.operation,values,{root:context.root,signal,write:context.write,say,format:formatReplicaBytesV1})};
 const resolved=await resolveOptions(parsed,context),{options,storeRoot}=resolved;
 if(parsed.operation==="seed"){
  say("seed.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  const result=await seedV1(options,signal,sink);
  say("seed.done",{files:result.published,bytes:formatReplicaBytesV1(result.bytes)});
  return {exitCode:0};
 }
 if(parsed.operation==="join"){
  say("join.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  try{
   const result=await joinV1(options,signal,sink);
   say("join.done",{files:result.files,adopted:result.adopted,queued:result.queued});
   return {exitCode:result.queued>0?1:0};
  }catch(error){
   if(error instanceof JoinRefusedV1){say("error.refused",{count:error.conflicts.length,paths:error.conflicts.slice(0,5).join(", ")});return {exitCode:1};}
   throw error;
  }
 }
 if(parsed.operation==="run"){
  say("run.start",{worktree:resolved.worktree,store:storeRoot,origin:options.originId,group:options.groupId});
  await runV1(options,signal,sink);
  say("run.stopped");
  return {exitCode:0};
 }
 if(parsed.operation==="status"){
  const report=await statusV1(options,parsed.flags.has("--deep"),signal);
  if(parsed.flags.has("--json"))context.write(JSON.stringify(report));else showStatus(report,say);
  return {exitCode:0};
 }
 if(parsed.operation==="snapshot"){
  const result=await snapshotV1(storeRoot,options.originId,signal,sink);
  if(result===undefined)say("snapshot.empty");else say("snapshot.done",{entries:result.entries,file:result.file,state:result.created?"@snapshot.created":"@snapshot.existing"});
  return {exitCode:0};
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

/** 🎬️ Routes `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle`;`args` are the words after the script name. Streaming operations write NDJSON events to stdout and human messages to stderr; `status --json` and `verify --json` write one document; `verify` exits non-zero on drift. */
export async function runReplicaCommandV1(args:readonly string[],context:ReplicaCommandContextV1):Promise<ReplicaCommandResultV1>{
 const parsed=parseReplicaArgumentsV1(args),say=reporter(context,replicaLanguagesV1(parsed.values["--lang"],context.environment));
 try{return await execute(parsed,context,say);}
 catch(error){
  if(error instanceof ReplicaBusyV1){say("error.busy",{pid:error.pid});return {exitCode:1};}
  if(context.signal.aborted)return {exitCode:130};
  throw error;
 }
}
