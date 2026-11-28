import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {UNIT_CEILING_POLICY_V1,runtimeCeilingOfCommandV1,type CompileBudgetPolicyV1} from "../../../📦️artifacts/🏗️native-build/🎟️admission/🏔️ceiling/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const MESSAGES=schema.definitions.Messages.const;

/** 🎚️ The authored numbers of the host warden (schema `Policy`): the ladder lines, the per-tick maxima, the loop intervals and the file names of its directory. */
export type HostWardenPolicyV1=Readonly<typeof schema.definitions.Policy.const>;
/** 🧩️ One kind of subject (schema `Kinds`): the pattern over the command line, the cost rank, the actions the warden may take, the cancel signal and the scope a signal reaches. */
export type HostWardenKindV1=Readonly<{name:string;rank:number;pattern:string;actions:readonly("suspend"|"cancel")[];signal:"SIGKILL"|"SIGTERM";scope:"process"|"group"|"tree";capMegabytes?:number;capCritical?:boolean;lastResort?:boolean;ceiling?:"unit";source?:string}>;
/** 🗃️ The pattern tables classification reads: roots (repo-started by ancestry), identities (repo-owned by what the process is, whatever its ancestry), never (excluded whatever they hold) and the ordered kinds. */
export type HostWardenTablesV1=Readonly<{roots:readonly string[];identities?:readonly string[];never:readonly string[];kinds:readonly HostWardenKindV1[]}>;
/** 🌡️ The host numbers the ladder reads (schema `Host`): the memory counters, the free swap and the free space of the swap volume (null where unknown or where swap cannot grow). */
export type HostWardenHostV1=Readonly<{totalMegabytes:number;availableMegabytes:number;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical";swapFreeMegabytes:number|null;swapVolumeFreeMegabytes:number|null;swapFileMegabytes?:number;intervalMilliseconds?:number}>;
/** 🏔️ The runtime ceiling of a compile unit (schema `Ceiling`): the unit (`<crate>|<target>`), its budget kind, the most its resident footprint may reach, whether it is on the carving list of the admission policy and, then, its measured peak. */
export type HostWardenCeilingV1=Readonly<{unit:string;kind:"library"|"final";megabytes:number;listed:boolean;measuredMegabytes?:number}>;
/** 🧾️ One row of the process table (schema `ProcessRow`). */
/** 🌲️ One process of the repository tree the warden published (schema `TreeRow`): the heaviest by CPU time, then memory, among the processes whose command line names a workspace root and everything below them; `megabytes` is what the kernel charges the process alone, `command` its first 240 characters. */
export type HostWardenTreeRowV1=Readonly<{pid:number;parent:number;elapsedSeconds:number;cpuSeconds?:number;megabytes:number;command:string}>;
export type HostWardenRowV1=Readonly<{pid:number;parent:number;group:number;memoryKilobytes:number;nonresidentKilobytes?:number;stopped:boolean;elapsedSeconds:number;cpuSeconds?:number;start:string;command:string}>;
/** 🎯️ One process the warden may act on (schema `Subject`): the decision reads the first ten fields, the effects the rest. */
export type HostWardenSubjectV1=Readonly<{id:string;pid:number;kind:string;rank:number;elapsedSeconds:number;megabytes:number;nonresidentMegabytes?:number|null;stoppedFootprintMegabytes?:number;suspended:boolean;suspendedSeconds:number;canSuspend:boolean;canCancel:boolean;parent?:number;start?:string;owner?:string;scope?:"process"|"group"|"tree";signal?:"SIGKILL"|"SIGTERM";capMegabytes?:number;capCritical?:boolean;lastResort?:boolean;keepsProgress?:boolean;ceiling?:HostWardenCeilingV1;suspensions?:number;resumedSeconds?:number;grantMegabytes?:number;held?:boolean}>;
/** 🎯️ What decided a level (schema `Trigger`): the available memory, the headroom, the swap in use, its share of a fixed swap, the compressed memory, the OS pressure (warn, critical) or the swap-out rate. */
export type HostWardenTriggerV1="available"|"headroom"|"swap-used"|"swap-share"|"compressed"|"pressure"|"critical"|"swap"|"swapping"|"cap"|"swap-free"|"grant";
/** 🔭️ One consumer the warden does not own (schema `Other`): the topmost process of a tree outside every subject, with the resident megabytes and the process count of the whole tree. */
export type HostWardenOtherV1=Readonly<{pid:number;megabytes:number;processes:number;command:string}>;
/** 🔙️ What the previous tick left (schema `Previous`). */
export type HostWardenPreviousV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number;swapMilliseconds?:number;resumedSeconds?:number}>;
/** ⚙️ One step of a decision (schema `Action`). */
export type HostWardenActionV1=Readonly<{action:"suspend"|"resume"|"cancel"|"report";id:string;reason:"guard"|"danger"|"overdue"|"limit"|"calm"|"cap"|"ceiling";capMegabytes?:number}>;
/** ⚖️ The verdict of one tick (schema `Decision`). */
export type HostWardenDecisionV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number;swapMilliseconds:number;hold:boolean;trigger:HostWardenTriggerV1|null;guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;actions:readonly HostWardenActionV1[]}>;
/** 🔔️ One line of `alarms.jsonl` (schema `Alarm`). */
export type HostWardenAlarmV1=Readonly<{version:1;atMs:number;observed:boolean;level:"calm"|"guard"|"danger";action:"suspend"|"resume"|"cancel"|"report";reason:"guard"|"danger"|"overdue"|"limit"|"calm"|"cap"|"ceiling";id:string;pid:number;kind:string;owner:string;megabytes:number;trigger?:HostWardenTriggerV1;capMegabytes?:number;ceiling?:HostWardenCeilingV1;grantMegabytes?:number;host:Readonly<{availableMegabytes:number;guardMegabytes:number;dangerMegabytes:number;swapOutMegabytesPerSecond:number|null;pressure:"normal"|"warn"|"critical";headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number}>;text:Readonly<{en:string;de:string}>}>;
/** 🗂️ The cancel record written before a cancel signal (schema `Cancel`). */
export type HostWardenCancelV1=Readonly<{version:1;pid:number;start:string;kind:string;owner:string;command:string;megabytes:number;atMs:number;reason:"guard"|"danger"|"overdue"|"limit"|"calm"|"cap"|"ceiling";ancestors:readonly number[];buildScriptOutput?:string;capMegabytes?:number;ceiling?:HostWardenCeilingV1;grantMegabytes?:number}>;
/** 📋️ The state file of the running warden (schema `State`), written each tick: host numbers, level, subjects by kind, the suspended set, counters, the last alarms and the overhead of the loop. */
export type HostWardenStateV1=Readonly<{version:1;atMs:number;pid:number;mode:"observe"|"enforce";code:string;host:HostWardenHostV1;level:"calm"|"guard"|"danger";hold:boolean;trigger?:HostWardenTriggerV1|null;others?:readonly HostWardenOtherV1[];foreign?:Readonly<{megabytes:number;processes:number;heavy:number}>;grants?:Readonly<{count:number;megabytes:number}>;guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;subjects:Readonly<{count:number;megabytes:number;cpuSeconds?:number;byKind:Readonly<Record<string,Readonly<{count:number;megabytes:number}>>>}>;suspended:readonly Readonly<{id:string;pid:number;start:string;kind:string;sinceMs:number}>[];counters:Readonly<{suspended:number;resumed:number;cancelled:number;ticks:number}>;lastAlarms:readonly HostWardenAlarmV1[];tree?:readonly HostWardenTreeRowV1[];overhead:Readonly<{tickMillisecondsP50:number;tickMillisecondsP95:number;cpuBasisPoints:number;residentMegabytes:number}>}>;
/** 🪞️ The memory record a worker mirrors to the group store (schema `Mirror`): the state of its warden as written, or the absent record. */
export type HostWardenMirrorV1=HostWardenStateV1|Readonly<{version:1;atMs:number;absent:true}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostWardenDefinitionV1="Host"|"ProcessRow"|"Subject"|"Previous"|"Action"|"Decision"|"Alarm"|"Ceiling"|"Claim"|"Cancel"|"Lock"|"Protect"|"State"|"Mode"|"Absent"|"Mirror";

/** 🎚️ The authored policy of the warden, frozen. */
export const HOST_WARDEN_POLICY_V1:HostWardenPolicyV1=Object.freeze({...schema.definitions.Policy.const});
/** 🗃️ The authored pattern tables of the warden, frozen. */
export const HOST_WARDEN_TABLES_V1:HostWardenTablesV1=Object.freeze({roots:Object.freeze([...schema.definitions.Roots.const]),identities:Object.freeze([...schema.definitions.Identities.const]),never:Object.freeze([...schema.definitions.Never.const]),kinds:Object.freeze(schema.definitions.Kinds.const.map(kind=>Object.freeze({...kind,actions:Object.freeze([...kind.actions])}))) as readonly HostWardenKindV1[]});
/** 🪟️ The platforms by what the warden may do there (schema `Platforms`). */
export const HOST_WARDEN_PLATFORMS_V1=schema.definitions.Platforms.const;
/** 🧭️ The anchors of the pattern tables (schema `Anchors`): how workspaces are found in command lines and where the repository browser runtime lives below one. */
export const HOST_WARDEN_ANCHORS_V1=schema.definitions.Anchors.const;
/** 💱️ How the swap of the machine is read (schema `Swap`). */
export const HOST_WARDEN_SWAP_V1=schema.definitions.Swap.const;
/** 🧰️ The process table command, its locale per platform and the command that reads one start text (schema `Table`). */
export const HOST_WARDEN_TABLE_V1=schema.definitions.Table.const;

/** 🛂️ Requires a value to match one closed definition of the warden schema. */
export function requireHostWardenV1<T>(definition:HostWardenDefinitionV1,value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(`warden ${definition}: ${String(errors[0])}`);
 return value as T;
}

/** 🔍️ Answers whether a value matches one closed definition of the warden schema without throwing. */
export function isHostWardenV1(definition:HostWardenDefinitionV1,value:unknown):boolean{
 return validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value).length===0;
}

/** ⏲️ Turns a `ps` elapsed time `[[dd-]hh:]mm:ss` into seconds; a text of another shape is 0. */
export function parseWardenElapsedV1(text:string):number{
 const match=/^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)$/u.exec(text);
 return match?((Number(match[1]??0)*24+Number(match[2]??0))*60+Number(match[3]))*60+Number(match[4]):0;
}

/** ⏱️ Turns a `ps` CPU time (`[dd-]hh:mm:ss` on Linux, `mmm:ss.cc` or `h:mm:ss.cc` on macOS) into seconds with fractions; a text of another shape is 0. */
export function parseWardenCpuV1(text:string):number{
 const match=/^(?:(\d+)-)?((?:\d+:)?\d+):(\d+(?:[.,]\d+)?)$/u.exec(text);
 if(!match)return 0;
 const head=match[2]!.split(":").map(Number),seconds=Number(match[3]!.replace(",","."));
 const [hours,minutes]=head.length===2?[head[0]!,head[1]!]:[0,head[0]!];
 return Number(match[1]??0)*86400+hours*3600+minutes*60+seconds;
}

const ROW=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\S+)\s+((?:\d+-)?(?:\d+:)?\d+:\d+)(?:\s+((?:\d+-)?(?:\d+:)?\d+:\d+(?:[.,]\d+)?))?\s+(\S+\s+\S+\s+\S+\s+\S+\s+\S+?)(?:\s+(.*))?$/u;
/** 🌳️ Parses `ps -A -o pid=,ppid=,pgid=,rss=,state=,etime=,lstart=,args=` (macOS and Linux; without the `args=` column the commands are empty) into rows: a stopped process is one whose state starts with `T`, the start is the five-token `lstart` text; a line that is no row (the continuation of a multi-line command) is skipped. */
export function parseWardenProcessTableV1(text:string):readonly HostWardenRowV1[]{
 const rows:HostWardenRowV1[]=[];
 for(const line of text.split("\n")){
  const match=ROW.exec(line);
  if(match)rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),group:Number(match[3]),memoryKilobytes:Number(match[4]),stopped:match[5]!.startsWith("T"),elapsedSeconds:parseWardenElapsedV1(match[6]!),...match[7]===undefined?{}:{cpuSeconds:parseWardenCpuV1(match[7])},start:match[8]!,command:match[9]??""}));
 }
 return rows;
}

const compiled=new Map<string,RegExp>();
const pattern=(source:string):RegExp=>{let regex=compiled.get(source);if(regex===undefined){regex=new RegExp(source,"u");compiled.set(source,regex);}return regex;};
const matches=(sources:readonly string[],command:string):boolean=>sources.some(source=>pattern(source).test(command));

/** 🆔️ The id of a subject: its pid and the first eight hex digits of the sha256 of its start text, so a reused pid is another subject. */
export function wardenSubjectIdV1(pid:number,start:string):string{
 return `${pid}.${createHash("sha256").update(start).digest("hex").slice(0,8)}`;
}

/** 🏷️ The short label of a root process: its executable cut to the base name, the rest of the command with every prefix (workspace, home) cut, at most 96 characters. */
export function wardenOwnerLabelV1(command:string,prefixes:readonly string[]=[]):string{
 const space=command.indexOf(" "),executable=space<0?command:command.slice(0,space),rest=space<0?"":command.slice(space);
 let text=executable.slice(executable.lastIndexOf("/")+1)+rest;
 for(const prefix of [...prefixes].filter(Boolean).sort((a,b)=>b.length-a.length))text=text.split(prefix).join("");
 return [...text].slice(0,96).join("");
}

/** 🔭️ What classification may be told besides the table: protected path fragments, a pid the subjects must descend from, the prefixes cut from owner labels, the suspension times of earlier ticks and the workspace roots the anchors of the patterns stand for. */
export type HostWardenClassifyOptionsV1=Readonly<{owned?:Map<string,Readonly<{cpuSeconds:number;elapsedSeconds:number}>>;protect?:readonly string[];within?:number;prefixes?:readonly string[];nowMs?:number;suspendedSince?:ReadonlyMap<string,number>;workspaces?:readonly string[];ceilings?:CompileBudgetPolicyV1}>;

const escaped=(text:string):string=>text.replace(/[\\^$.*+?()[\]{}|]/gu,"\\$&");
const anchorMarks=new RegExp(`<(${HOST_WARDEN_ANCHORS_V1.names.join("|")})>`,"gu");

/** 🧭️ The directories of the repository browser runtime below the given workspaces (schema `Anchors.browsers`). */
export function wardenBrowsersV1(workspaces:readonly string[]):readonly string[]{
 return HOST_WARDEN_ANCHORS_V1.browsers.flatMap(template=>workspaces.map(workspace=>template.split("<workspace>").join(workspace)));
}

/** 🏠️ Finds the workspace roots in a process table: every command line that runs from the pinned cache of a workspace names its root (schema `Anchors.workspace`); `seed` roots are kept. Sorted, unique. */
export function discoverWardenWorkspacesV1(rows:readonly HostWardenRowV1[],seed:readonly string[]=[]):readonly string[]{
 const found=new Set(seed),regex=pattern(HOST_WARDEN_ANCHORS_V1.workspace);
 for(const row of rows){const match=regex.exec(row.command);if(match)found.add(match[1]!);}
 return[...found].sort();
}

/** 🧷️ Replaces the anchors of a pattern by the alternation of the roots they stand for; null when an anchor has no root, so the pattern is inactive. */
export function expandWardenPatternV1(source:string,workspaces:readonly string[]):string|null{
 const anchors:Readonly<Record<string,readonly string[]>>={workspace:workspaces,browsers:wardenBrowsersV1(workspaces)};
 let empty=false;
 const text=source.replace(anchorMarks,(_,name:string)=>{const list=anchors[name]??[];if(list.length===0){empty=true;return"";}return`(?:${list.map(escaped).join("|")})`;});
 return empty?null:text;
}

/** 🎯️ Finds the subjects in a process table: a process is a subject when it or an ancestor matches a root, or when it matches an identity (so processes started from a shell are seen), neither it nor an ancestor contains a protected path, it matches no never pattern, the first kind that matches its command names its kind, and, when `within` is given, it descends from that pid. The megabytes of a subject are its own resident megabytes plus those of every process whose nearest subject is it. A subject of a kind that runs compile units (`ceiling: "unit"`) carries the runtime ceiling its command line names (`options.ceilings`, default the compile-memory policy of the admission schema). Sorted by pid. */
export function classifyWardenSubjectsV1(rows:readonly HostWardenRowV1[],tables:HostWardenTablesV1=HOST_WARDEN_TABLES_V1,options:HostWardenClassifyOptionsV1={}):readonly HostWardenSubjectV1[]{
 const byPid=new Map(rows.map(row=>[row.pid,row] as const)),protect=options.protect??[],workspaces=options.workspaces??[];
 const identities=(tables.identities??[]).flatMap(source=>{const text=expandWardenPatternV1(source,workspaces);return text===null?[]:[text];});
 const kinds=tables.kinds.flatMap(kind=>{const text=expandWardenPatternV1(kind.pattern,workspaces);return text===null?[]:[{kind,text}];});
 const root=new Map<number,number>(),blocked=new Map<number,boolean>(),inside=new Map<number,boolean>();
 const chain=(row:HostWardenRowV1):HostWardenRowV1[]=>{const seen=new Set<number>(),path:HostWardenRowV1[]=[];for(let at:HostWardenRowV1|undefined=row;at!==undefined&&!seen.has(at.pid)&&path.length<256;at=byPid.get(at.parent)){seen.add(at.pid);path.push(at);}return path;};
 const rootOf=(row:HostWardenRowV1):number=>{
  const known=root.get(row.pid);if(known!==undefined)return known;
  const found=chain(row).find(link=>matches(tables.roots,link.command))?.pid??0;
  root.set(row.pid,found);return found;
 };
 const blockedOf=(row:HostWardenRowV1):boolean=>{
  if(protect.length===0)return false;
  const known=blocked.get(row.pid);if(known!==undefined)return known;
  const found=chain(row).some(link=>protect.some(fragment=>link.command.includes(fragment)));
  blocked.set(row.pid,found);return found;
 };
 const insideOf=(row:HostWardenRowV1):boolean=>{
  if(options.within===undefined)return true;
  const known=inside.get(row.pid);if(known!==undefined)return known;
  const found=chain(row).slice(1).some(link=>link.pid===options.within);
  inside.set(row.pid,found);return found;
 };
 const found=new Map<number,{row:HostWardenRowV1;kind:HostWardenKindV1}>();
 for(const row of rows){
  if((rootOf(row)===0&&!matches(identities,row.command))||blockedOf(row)||!insideOf(row)||matches(tables.never,row.command))continue;
  const kind=kinds.find(candidate=>pattern(candidate.text).test(row.command))?.kind;
  if(kind!==undefined)found.set(row.pid,{row,kind});
 }
 const kilobytes=new Map<number,number>(),home=(row:HostWardenRowV1):number|undefined=>chain(row).find(link=>found.has(link.pid))?.pid;
 for(const row of rows){const owner=home(row);if(owner!==undefined){kilobytes.set(owner,(kilobytes.get(owner)??0)+row.memoryKilobytes);if(row.cpuSeconds!==undefined)options.owned?.set(`${row.pid}.${row.start}`,{cpuSeconds:row.cpuSeconds,elapsedSeconds:row.elapsedSeconds});}}
 const ceilingOf=(kind:HostWardenKindV1,row:HostWardenRowV1):Readonly<{ceiling?:HostWardenCeilingV1}>=>{const ceiling=kind.ceiling==="unit"?runtimeCeilingOfCommandV1(options.ceilings??UNIT_CEILING_POLICY_V1,row.command):null;return ceiling===null?{}:{ceiling};};
 return [...found.values()].sort((left,right)=>left.row.pid-right.row.pid).map(({row,kind})=>{
  const id=wardenSubjectIdV1(row.pid,row.start),since=options.suspendedSince?.get(id),seconds=row.stopped&&since!==undefined&&options.nowMs!==undefined?Math.max(0,Math.floor((options.nowMs-since)/1000)):0,top=byPid.get(rootOf(row))??row;
  return Object.freeze({id,pid:row.pid,parent:row.parent,kind:kind.name,rank:kind.rank,elapsedSeconds:row.elapsedSeconds,megabytes:Math.floor((kilobytes.get(row.pid)??0)/1024),suspended:row.stopped,suspendedSeconds:seconds,canSuspend:kind.actions.includes("suspend"),canCancel:kind.actions.includes("cancel"),start:row.start,owner:wardenOwnerLabelV1(top.command,options.prefixes),scope:kind.scope,signal:kind.signal,...kind.capMegabytes===undefined?{}:{capMegabytes:kind.capMegabytes},...kind.capCritical===true?{capCritical:true}:{},...kind.lastResort===true?{lastResort:true}:{},...ceilingOf(kind,row)});
 });
}

/** 🌿️ The pids a signal to a subject reaches, parent first: the pid for scope `process`, the pids of its process group for `group`, the pid and every descendant for `tree`; the pids in `exclude` (the warden itself and its ancestors) and every process that matches a never pattern are left out. */
export function planWardenSignalsV1(rows:readonly HostWardenRowV1[],subject:Readonly<{pid:number;scope?:"process"|"group"|"tree"}>,tables:HostWardenTablesV1=HOST_WARDEN_TABLES_V1,exclude:ReadonlySet<number>=new Set()):readonly number[]{
 const byPid=new Map(rows.map(row=>[row.pid,row] as const),),top=byPid.get(subject.pid);if(top===undefined)return[];
 const allowed=(row:HostWardenRowV1):boolean=>!exclude.has(row.pid)&&(row.pid===subject.pid||!matches(tables.never,row.command));
 if(subject.scope==="group")return rows.filter(row=>row.group===top.group&&allowed(row)).map(row=>row.pid).sort((a,b)=>a-b);
 if(subject.scope!=="tree")return allowed(top)?[top.pid]:[];
 const children=new Map<number,number[]>();for(const row of rows){const list=children.get(row.parent)??[];list.push(row.pid);children.set(row.parent,list);}
 const order:number[]=[],seen=new Set<number>(),queue=[top.pid];
 for(let pid=queue.shift();pid!==undefined;pid=queue.shift()){
  if(seen.has(pid))continue;seen.add(pid);
  const row=byPid.get(pid);if(row===undefined||!allowed(row))continue;
  order.push(pid);queue.push(...(children.get(pid)??[]));
 }
 return order;
}

/** 🏗️ Whether a subject is a declared final link: a compile unit whose ceiling is of the final kind and on the carving list of the admission policy. Such a unit is the single guarded heavy job of its machine (the law lets it run alone, serial back end, one slot) and a retry costs a whole link, so it is the dearest to lose among the compile units and waits longer suspended (`heavySuspendMaximumSeconds`). */
const heavy=(subject:HostWardenSubjectV1):boolean=>subject.ceiling!==undefined&&subject.ceiling.kind==="final"&&subject.ceiling.listed;

/** 🪙️ The order of the victims, cheapest to lose first: whatever is not build-critical before what is, then whatever is not a declared final link ({@link heavy}) before what is, then the least work lost per megabyte freed (wall seconds the subject ran divided by its megabytes, compared by cross multiplication so a young unit or pool child goes before a long running final), then the cost rank, then the pid. The last of a list is the dearest to lose. */
const order=(left:HostWardenSubjectV1,right:HostWardenSubjectV1):number=>Number(left.capCritical===true)-Number(right.capCritical===true)||Number(heavy(left))-Number(heavy(right))||left.elapsedSeconds*Math.max(1,right.megabytes)-right.elapsedSeconds*Math.max(1,left.megabytes)||left.rank-right.rank||left.pid-right.pid;
/** 🛟️ Whether the owner of a subject holds memory for it at the capacity gate (its claim) and the subject stays within it: such a subject is sheltered, the owner booked what it uses. */
const sheltered=(subject:HostWardenSubjectV1):boolean=>subject.grantMegabytes!==undefined&&subject.megabytes<=subject.grantMegabytes;
/** 📏️ How far a subject runs above the memory its owner holds for it (0 without a claim). */
const excess=(subject:HostWardenSubjectV1):number=>subject.grantMegabytes===undefined?0:Math.max(0,subject.megabytes-subject.grantMegabytes);

/** 🎯️ The order the danger cancel takes its victims, the first is cancelled first: a last-resort subject (the stage that owns a whole build tree: its own cancel frees next to nothing and costs the whole stage with a bare SIGTERM, while its workers are cancelled and requeued by their owner) after everything else, then whatever is not sheltered ({@link sheltered}, inside its grant) before what is, then the subject furthest above its grant, then whatever is not a declared final link ({@link heavy}), then the largest (a subject over its cap or ceiling is cancelled before this order applies; a small young step never goes before a large process, it frees too little and costs a retry), then a suspended one before a running one, then the cheapest to lose ({@link order}). */
const dearer=(left:HostWardenSubjectV1,right:HostWardenSubjectV1):number=>Number(left.lastResort===true)-Number(right.lastResort===true)||Number(sheltered(left))-Number(sheltered(right))||excess(right)-excess(left)||Number(heavy(left))-Number(heavy(right))||right.megabytes-left.megabytes||Number(right.suspended)-Number(left.suspended)||order(left,right);
const clamp=(value:number,low:number,high:number):number=>Math.min(high,Math.max(low,value));

/** 🌊️ How deep the swap and the compressed memory are (schema `Policy`): 2 at or above a danger line, 1 at or above a guard line, 0 below. The swap in use is read as a share of the physical memory and, when swap occupancy is known, also as a share of the existing swap plus the disk space available to grow it above the reserve; the compressed memory as a share of the physical memory. The source is the first of swap-used, swap-share and compressed that reaches the depth.
 * @see ./🧬️schema/🔣️.json */
export function swapDepthHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1):Readonly<{depth:0|1|2;compressed:0|1|2;source:"swap-used"|"swap-share"|"compressed"|"swap-free"|null;compressedSource:"compressed"|"swap-free"|null}>{
 const total=host.totalMegabytes,used=host.swapUsedMegabytes,free=host.swapFreeMegabytes,compressed=host.compressedMegabytes;
 const growth = Math.max(0, (host.swapVolumeFreeMegabytes ?? 0) - policy.swapDiskReserveMegabytes);
 const swapCapacity = used === null || free === null ? 0 : used + free + growth, allocated = used === null || free === null ? 0 : used + free, file = host.swapFileMegabytes ?? policy.swapFileDefaultMegabytes;
 const depthOf=(value:number|null,whole:number,guard:number,danger:number):0|1|2=>value===null?0:value*10000>=whole*danger?2:value*10000>=whole*guard?1:0;
 const freeDepth:0|1|2=free===null||used===null||allocated===0||used*10000<allocated*policy.swapFreeMinimumUsedBasisPoints?0:free*10000<file*policy.swapFreeDangerBasisPointsOfFile?2:free*10000<file*policy.swapFreeGuardBasisPointsOfFile?1:0;
 const guardLine=clamp(Math.floor(total*policy.guardBasisPoints/10000),policy.guardMinimumMegabytes,policy.guardMaximumMegabytes);
 const live=host.pressure!=="normal"||(host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.swapActiveMegabytesPerSecond)||freeDepth>0||host.availableMegabytes<guardLine;
 const lines=[
  ["swap-used",depthOf(used,total,policy.swapUsedGuardBasisPoints,policy.swapUsedDangerBasisPoints)],
  ["swap-share",swapCapacity>0?depthOf(used,swapCapacity,policy.swapShareGuardBasisPoints,policy.swapShareDangerBasisPoints):0],
  ["compressed",live?depthOf(compressed,total,policy.compressedGuardBasisPoints,policy.compressedDangerBasisPoints):0],
  ["swap-free",freeDepth],
 ] as const,depth=Math.max(...lines.map(line=>line[1])) as 0|1|2,exhaustion=Math.max(lines[2][1],lines[3][1]) as 0|1|2;
 return{depth,compressed:exhaustion,source:depth===0?null:lines.find(line=>line[1]===depth)![0],compressedSource:exhaustion===0?null:lines[2][1]===exhaustion?"compressed":"swap-free"};
}

/** 🎯 The trigger that decided a hold level, in the precedence of the classification of {@link decideHostWardenV1}: the available memory below the line, else the headroom (available memory plus the swap that can still grow) below its line, else the swap or compressed depth that raised the line, else the memory pressure, else the swap-out rate. It only words the alarm; the level itself is classified by the decision, and the tests pin that both agree.
 * @see ./🧬️schema/🔣️.json */
export function triggerHostWardenV1(level:"guard"|"danger",numbers:Readonly<{availableMegabytes:number;guardMegabytes:number;dangerMegabytes:number;pressure:HostWardenHostV1["pressure"];headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;swapDepth?:ReturnType<typeof swapDepthHostWardenV1>}>):HostWardenTriggerV1{
 if(numbers.availableMegabytes<(level==="danger"?numbers.dangerMegabytes:numbers.guardMegabytes))return"available";
 if(numbers.headroomMegabytes!==null&&numbers.headroomMegabytes<(level==="danger"?numbers.headroomDangerMegabytes:numbers.headroomGuardMegabytes))return"headroom";
 const deep=numbers.swapDepth;
 if(deep!==undefined&&deep.source!==null&&deep.depth>=(level==="danger"?2:1))return deep.source;
 return level==="danger"||numbers.pressure==="warn"?"pressure":numbers.pressure==="critical"?"critical":"swap";
}

/** 🛟️ The swap room and the headroom of a host sample: the swap room is the free swap plus the swap that can still grow (the free space of the swap volume minus `swapDiskReserveMegabytes`, never below 0, 0 where the volume is unknown), bounded by `swapRoomMaximumBasisPoints` of the physical memory; the headroom is the available memory plus the swap room. Both are null for a host that reports no swap, which keeps the available-memory law. The lines are shares of the physical memory between their bounds.
 * @see ./🧬️schema/🔣️.json */
export function headroomHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1):Readonly<{swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number}>{
 const total=host.totalMegabytes,free=host.swapFreeMegabytes??null,guard=clamp(Math.floor(total*policy.headroomGuardBasisPoints/10000),policy.headroomGuardMinimumMegabytes,policy.headroomGuardMaximumMegabytes),danger=clamp(Math.floor(total*policy.headroomDangerBasisPoints/10000),policy.headroomDangerMinimumMegabytes,policy.headroomDangerMaximumMegabytes);
 if(free===null)return{swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:guard,headroomDangerMegabytes:danger};
 const growable=Math.max(0,(host.swapVolumeFreeMegabytes??0)-policy.swapDiskReserveMegabytes),room=Math.min(free+growable,Math.floor(total*policy.swapRoomMaximumBasisPoints/10000));
 return{swapRoomMegabytes:room,headroomMegabytes:host.availableMegabytes+room,headroomGuardMegabytes:guard,headroomDangerMegabytes:danger};
}

/** 💱️ Reads the swap of a machine from the text of its platform commands (schema `Swap`): macOS `sysctl -n vm.swapusage` (free swap) and `df -k <volume>` (free space of the swap volume, the Available column of the last line); Linux `/proc/meminfo` (SwapFree, null when SwapTotal is 0; the swap does not grow, so the volume is null); every other platform reports null. Whole megabytes, rounded down; text that does not parse is null. */
export function parseWardenSwapV1(platform:string,texts:Readonly<{usage?:string|null;disk?:string|null;meminfo?:string|null;fileSize?:string|null}>):Readonly<{swapFreeMegabytes:number|null;swapVolumeFreeMegabytes:number|null;swapFileMegabytes?:number}>{
 if(platform==="darwin"){
  const free=/free = ([\d.]+)([KMGT])/u.exec(texts.usage??""),scale:Record<string,number>={K:1/1024,M:1,G:1024,T:1048576},lines=(texts.disk??"").trim().split("\n"),columns=lines.length>1?lines.at(-1)!.trim().split(/\s+/u):[],available=Number(columns[3]);
  const file=Math.floor(Number((texts.fileSize??"").trim())/1048576);
  return{swapFreeMegabytes:free?Math.floor(Number(free[1])*scale[free[2]!]!):null,swapVolumeFreeMegabytes:Number.isFinite(available)&&columns.length>3?Math.floor(available/1024):null,...Number.isFinite(file)&&file>0?{swapFileMegabytes:file}:{}};
 }
 if(platform==="linux"){
  const kilobytes=(name:string):number|null=>{const match=new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(texts.meminfo??"");return match?Number(match[1]):null;},total=kilobytes("SwapTotal"),free=kilobytes("SwapFree");
  return{swapFreeMegabytes:total===null||free===null||total===0?null:Math.floor(free/1024),swapVolumeFreeMegabytes:null};
 }
 return{swapFreeMegabytes:null,swapVolumeFreeMegabytes:null};
}

/** ⚖️ Decides one tick of the warden from the host numbers, the subjects and what the previous tick left; pure, no clock and no I/O. The law: guard, danger and resume lines are shares of the physical memory between their bounds. The level is `danger` below the danger line, with a headroom (available memory plus the swap room, bounded by the free disk of the swap volume, see {@link headroomHostWardenV1}) below the headroom danger line, with the swap or compressed memory at danger depth ({@link swapDepthHostWardenV1}) and the available memory below the guard line, or during acute swapping (the swap traffic at or above `swapActiveMegabytesPerSecond` for `swapSustainedSeconds` in a row, counted by a leaky timer that gains the sample interval on a busy sample and loses it on a quiet one, bounded by twice the span, together with the OS pressure at least warn and the available memory below the resume line; this is severe; a single burst, healthy paging with memory to spare and compressed memory, which the kernel gives back, never are), or, whatever the available memory, with the OS pressure critical or the compressed memory at danger depth; `guard` below the guard line, with a headroom below the headroom guard line, with the OS pressure warn, with the compressed memory at guard depth, or with the swap worked or the swap or compressed memory at guard depth and the available memory below the resume line; else `calm`. So the OS pressure alone is at least tight (new work is held and the cheapest subjects are suspended) and critical alone is critical; only a severe danger cancels (suspended subjects first, until the freed megabytes cover the deficit), a danger from the pressure or the compressed memory alone suspends and cancels what stays suspended too long. The deficit is the distance to the guard line (to the resume line with a deep swap), never less than `pressureDeficitMegabytes` while acute swapping, or pressure or compressed memory without a severe shortage, holds the level. A subject of a build-critical kind (`capCritical`: the compile units, the graph pool children, the slice workers) that crosses its cap is not killed first: the level is at least tight (trigger `cap`), the crossing is reported, and when the host itself has no shortage (every measured line is calm) nothing but a crossing non-critical subject (over its grant) is suspended: a subject within its grant is never paused for another subject's crossing; when the host does lack memory the crossers are suspended first, then the others until the overage is covered (the critical ones keep running while the host is quiet), the crossing is reported as an alarm of action `report`, and the subject is cancelled only when the level is danger, when it stays suspended too long (see below) or when it is over its ceiling. When nothing but the compressed or swapped memory holds the level (pressure normal, available memory at or above the guard line, headroom above its guard line, swap not worked) the level only holds new admissions: nothing is suspended or cancelled for it, and after the configured quiet recovery ticks one suspended subject, the most expensive, is resumed per tick, because compressed memory stays high long after the load is gone. A subject over the cap of its kind (`capMegabytes` of the kind, at most `capMaximumBasisPoints` of the physical memory) is cancelled at any level. A subject whose runtime ceiling (`ceiling`, set for compile units) is below its megabytes is cancelled at any level at once, before every cap and every shedding, build-critical or not (a unit does not outgrow the budget its kind or its carving entry declares); the cancel counts as freed memory. The level is quiet when no measured signal is at or below its line: the available memory at or above the guard line, the headroom at or above its guard line, the swap traffic not sustained above `swapActiveMegabytesPerSecond` and the OS pressure not critical; the OS pressure `warn` and the compressed or swapped memory alone are only felt by the kernel, so a quiet level holds new admissions and suspends and cancels nothing (a suspended subject is resumed after the quiet recovery ticks, one per tick, the dearest first). A suspended subject that stays suspended for `suspendMaximumSeconds` is overdue; it is cancelled only while a measured signal is at or below its line (not quiet) and not calm, and resumed when it cannot be cancelled or the level is quiet or calm, with quiet recovery ticks required before a quiet guard resumes. A subject within the memory its owner holds for it (`grantMegabytes`, see {@link applyWardenClaimsV1}) is never cancelled as overdue and is cancelled at danger only when the level is severe, after every subject not within a grant, the furthest above its grant first. A subject its owner holds stopped (`held`) is never a cancel candidate: it is stopped by its owner, not by a failed suspension, and its claim ends within `claimMaximumMilliseconds`. A sheltered subject is also suspended after the subjects not within a grant, and is not a cancel candidate while a subject not within a grant still waits for its pause deadline. A danger cancel is taken only after suspension failed (a pausable subject stays suspended for `pauseDeadlineSeconds`, a cancel-only subject only when every pausable one did) and then in the order of `dearer` (a subject over its cap or ceiling is cancelled before that, then the largest, so a small step is never cancelled before a large process). Victims, suspended, go in the order of `order`: not build-critical first, then the least work lost per megabyte freed (elapsed wall seconds per megabyte), then the cost rank. A subject suspended `flapMaximumSuspensions` times within `flapWindowSeconds`, or resumed less than `resumeHoldSeconds` ago, is not suspended again (damped) unless the level is severe danger. Overdue suspended subjects are cancelled (not calm and not quiet) or resumed (cannot cancel, quiet or calm); in guard and danger the cheapest running subjects are suspended until the suspended megabytes cover the deficit, the most expensive `keepRunning` keeps running unless the swapping is acute; after `calmTicks` calm ticks above the resume line one suspended subject is resumed per tick. A recovered host (quiet or calm, above the resume line, `calmTicks` ticks in a row, a hysteresis between the guard and the resume line; a build-critical subject that still runs over its cap does not delay it, it only holds admissions)  resumes one subject per tick and then waits `resumeSettleSeconds` (`previous.resumedSeconds`) before the next, in the order of the stuck ones (suspended for `suspendHardMaximumSeconds`) first, then the largest grant (the megabytes without a grant), then the longest suspended, then the dearest: the largest holds the most work and takes longest to page in, so it goes first while the host has the most room, and the small ones page in cheaply afterwards. A subject that is overdue is resumed on a recovered host whatever its page-in cost (a swapped-out subject can never be given its whole footprint as available memory; before it is overdue the page-in must fit the room above the guard line), except an unsheltered cancellable one that does not fit, which is cancelled. A subject suspended for `suspendHardMaximumSeconds` is never left: on a recovered host it is resumed first, otherwise it is cancelled (reason `limit`, bilingual, retryable) and a subject that cannot be cancelled is resumed.
 * @see ./🧬️schema/🔣️.json */
export function decideHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1,subjects:readonly HostWardenSubjectV1[],previous:HostWardenPreviousV1):HostWardenDecisionV1{
 const total=host.totalMegabytes,available=host.availableMegabytes;
 const guard=clamp(Math.floor(total*policy.guardBasisPoints/10000),policy.guardMinimumMegabytes,policy.guardMaximumMegabytes),danger=clamp(Math.floor(total*policy.dangerBasisPoints/10000),policy.dangerMinimumMegabytes,policy.dangerMaximumMegabytes),resume=Math.max(guard,Math.floor(total*policy.resumeBasisPoints/10000));
 const swapBusy=host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.swapActiveMegabytesPerSecond,sustainedMilliseconds=policy.swapSustainedSeconds*1000,elapsedMilliseconds=host.intervalMilliseconds??policy.tightMilliseconds;
 const swapMilliseconds=clamp((previous.swapMilliseconds??0)+(swapBusy?elapsedMilliseconds:-elapsedMilliseconds),0,2*sustainedMilliseconds),swapWorked=swapMilliseconds>=sustainedMilliseconds;
 const room=headroomHostWardenV1(policy,host),headroom=room.headroomMegabytes,deep=swapDepthHostWardenV1(policy,host),pressure=host.pressure==="critical"?2:host.pressure==="warn"?1:0;
 const kills=(subject:HostWardenSubjectV1):boolean=>subject.canCancel&&subject.keepsProgress!==true,critical=(subject:HostWardenSubjectV1):boolean=>subject.capCritical===true,canPause=(subject:HostWardenSubjectV1):boolean=>subject.canSuspend&&subject.canCancel,capOf=(subject:HostWardenSubjectV1):number=>subject.grantMegabytes===undefined?Math.min(subject.capMegabytes!,Math.floor(total*policy.capMaximumBasisPoints/10000)):Math.floor(total*policy.capMaximumBasisPoints/10000);
 const over=(subject:HostWardenSubjectV1):boolean=>subject.ceiling!==undefined&&subject.canCancel&&subject.megabytes>subject.ceiling.megabytes;
 const capCrossed=(subject:HostWardenSubjectV1):boolean=>subject.canCancel&&(over(subject)||subject.capMegabytes!==undefined&&critical(subject)&&subject.megabytes>capOf(subject));
 const spare=available>=resume&&(headroom===null||headroom>=room.headroomGuardMegabytes)&&deep.depth===0&&pressure===0;
 const grantOver=(subject:HostWardenSubjectV1):boolean=>subject.canCancel&&subject.grantMegabytes!==undefined&&!spare&&excess(subject)>=Math.max(policy.grantOverMinimumMegabytes,Math.floor(subject.grantMegabytes*policy.grantOverBasisPoints/10000));
 const crossed=subjects.filter(subject=>capCrossed(subject)||grantOver(subject)),crossedIds=new Set(crossed.map(subject=>subject.id)),overage=crossed.reduce((sum,subject)=>sum+(capCrossed(subject)?subject.megabytes-(over(subject)?subject.ceiling!.megabytes:capOf(subject)):excess(subject)),0);
 const acute=swapWorked&&pressure>=1&&available<resume;
 const roomy=available>=guard&&(headroom===null||headroom>=room.headroomGuardMegabytes)&&deep.depth<2,severe=available<danger||(headroom!==null&&headroom<room.headroomDangerMegabytes)||(deep.depth===2&&available<guard)||acute;
 const base:HostWardenDecisionV1["level"]=severe||pressure===2||deep.compressed===2?"danger":available<guard||(headroom!==null&&headroom<room.headroomGuardMegabytes)||pressure>=1||deep.compressed>=1||((swapWorked||deep.depth>=1)&&available<resume)?"guard":"calm",level:HostWardenDecisionV1["level"]=base==="calm"&&crossed.length>0?"guard":base;
 const trigger:HostWardenTriggerV1|null=level==="calm"?null:base==="calm"?crossed.some(capCrossed)?"cap":"grant":available<(level==="danger"?danger:guard)?"available":headroom!==null&&headroom<(level==="danger"?room.headroomDangerMegabytes:room.headroomGuardMegabytes)?"headroom":level==="danger"?deep.depth===2&&available<guard?deep.source!:deep.compressed===2?deep.compressedSource!:pressure===2?"pressure":"swapping":pressure===1?"pressure":deep.compressed>=1?deep.compressedSource!:deep.depth>=1&&available<resume&&deep.source!==null?deep.source:"swap";
 const quiet=pressure<2&&available>=guard&&!(headroom!==null&&headroom<room.headroomGuardMegabytes)&&!swapWorked;
 const calmTicks=level==="calm"?(previous.level==="calm"||previous.calmTicks>0?previous.calmTicks:0)+1:quiet&&available>=resume?previous.calmTicks+1:0,quietReady=quiet&&calmTicks>=policy.calmTicks&&available>=resume;
 const held=(acute||!severe&&(pressure>=1||deep.compressed>=1))?policy.pressureDeficitMegabytes:0,target=deep.depth>=1?resume:guard,deficitOf=():number=>Math.max(target-available,headroom===null?0:room.headroomGuardMegabytes-headroom,held,overage,1);
 const damped=(subject:HostWardenSubjectV1):boolean=>!(level==="danger"&&severe)&&((subject.suspensions??0)>=policy.flapMaximumSuspensions||(subject.resumedSeconds!==undefined&&subject.resumedSeconds<policy.resumeHoldSeconds));
 const actions:HostWardenActionV1[]=[],acted=new Set<string>();
 let cancels=0,suspends=0,capped=0;
 const cancel=(subject:HostWardenSubjectV1,reason:"danger"|"overdue"|"limit"):void=>{actions.push({action:"cancel",id:subject.id,reason});acted.add(subject.id);cancels++;};
 const resumeRoom=quiet&&available>=resume?available-guard:0;
 let remainingResumeRoom=resumeRoom,woken=0;
 const recovered=quietReady||(level==="calm"&&calmTicks>=policy.calmTicks&&available>=resume),settled=previous.resumedSeconds===undefined||previous.resumedSeconds>=policy.resumeSettleSeconds;
 const stuck=(subject:HostWardenSubjectV1):boolean=>subject.suspended&&subject.suspendedSeconds>=policy.suspendHardMaximumSeconds;
 const resumeOrder=(left:HostWardenSubjectV1,right:HostWardenSubjectV1):number=>Number(stuck(right))-Number(stuck(left))||(right.grantMegabytes??right.megabytes)-(left.grantMegabytes??left.megabytes)||right.suspendedSeconds-left.suspendedSeconds||Number(critical(right))-Number(critical(left))||Number(heavy(right))-Number(heavy(left))||right.elapsedSeconds*Math.max(1,left.megabytes)-left.elapsedSeconds*Math.max(1,right.megabytes)||right.rank-left.rank||left.pid-right.pid;
 const mayWake=(subject:HostWardenSubjectV1):boolean=>recovered?settled&&woken===0:pageIn(subject)<=remainingResumeRoom;
 const pageIn=(subject:HostWardenSubjectV1):number=>{
  if(subject.nonresidentMegabytes!==null)return subject.nonresidentMegabytes??0;
  return base==="calm"&&pressure===0?(subject.stoppedFootprintMegabytes??subject.megabytes):Infinity;
 };
 const wake=(subject:HostWardenSubjectV1,reason:"calm"|"overdue"):void=>{
  remainingResumeRoom-=pageIn(subject);woken++;
  actions.push({action:"resume",id:subject.id,reason});
  acted.add(subject.id);
 };
 for(const subject of [...subjects].sort(order)){
  if(!over(subject))continue;
  if(level!=="danger"){actions.push({action:"report",id:subject.id,reason:"ceiling"});continue;}
  actions.push({action:"cancel",id:subject.id,reason:"ceiling"});acted.add(subject.id);capped+=subject.megabytes;
 }
 for(const subject of [...subjects].sort(order)){
  if(subject.capMegabytes===undefined||!subject.canCancel||acted.has(subject.id)||over(subject))continue;
  const cap=capOf(subject);
  if(subject.megabytes<=cap)continue;
  if(critical(subject)&&level!=="danger"){actions.push({action:"report",id:subject.id,reason:"cap",capMegabytes:cap});continue;}
  actions.push({action:"cancel",id:subject.id,reason:"cap",capMegabytes:cap});acted.add(subject.id);capped+=subject.megabytes;
 }
 const overdue=subjects.filter(subject=>subject.suspended&&subject.suspendedSeconds>=(heavy(subject)?policy.heavySuspendMaximumSeconds:policy.suspendMaximumSeconds)&&!acted.has(subject.id)).sort(order);
 const cancelled:HostWardenSubjectV1[]=[];
 for(const subject of overdue)if(stuck(subject)&&kills(subject)&&!recovered&&cancels<policy.maximumCancelsPerTick){cancel(subject,"limit");cancelled.push(subject);}
 for(const subject of overdue)if(!acted.has(subject.id)&&kills(subject)&&(!sheltered(subject)||level==="danger"&&severe&&!roomy)&&(pageIn(subject)>resumeRoom||level!=="calm"&&!quiet&&(!critical(subject)||severe))&&cancels<policy.maximumCancelsPerTick){cancel(subject,"overdue");cancelled.push(subject);}
 for(const subject of recovered?[...overdue].sort(resumeOrder):overdue)if(!acted.has(subject.id)&&mayWake(subject)&&(!quiet||quietReady||level==="calm")&&(!kills(subject)||level!=="calm"&&quiet||critical(subject)&&!severe||recovered))wake(subject,"overdue");
 for(const subject of overdue.filter(stuck).sort(resumeOrder))if(!acted.has(subject.id)&&!kills(subject)&&woken===0)wake(subject,"overdue");
 if(level==="danger"&&severe){
  const deficit=deficitOf();
  let freed=capped+cancelled.reduce((sum,subject)=>sum+subject.megabytes,0);
  const pausable=subjects.filter(canPause),paused=(subject:HostWardenSubjectV1):boolean=>subject.suspended&&subject.suspendedSeconds>=policy.pauseDeadlineSeconds,pausingFailed=pausable.every(paused);
  const eligible=(subject:HostWardenSubjectV1):boolean=>subject.held!==true&&(subject.canSuspend?paused(subject):pausingFailed),unshelteredWaiting=subjects.some(subject=>!sheltered(subject)&&kills(subject)&&!acted.has(subject.id)&&!eligible(subject));
  const candidates=subjects.filter(subject=>kills(subject)&&!acted.has(subject.id)&&eligible(subject)&&!(roomy&&sheltered(subject))&&!(sheltered(subject)&&unshelteredWaiting)).sort(dearer);
  for(const subject of candidates){
   if(freed>=deficit||cancels>=policy.maximumCancelsPerTick)break;
   cancel(subject,"danger");freed+=subject.megabytes;
  }
 }
 if(level!=="calm"&&(!quiet||crossed.length>0)){
  const deficit=deficitOf(),every=subjects.filter(subject=>!subject.suspended&&!(quiet&&critical(subject))&&(base!=="calm"||crossedIds.has(subject.id))).sort((left,right)=>Number(!crossedIds.has(left.id))-Number(!crossedIds.has(right.id))||Number(critical(left))-Number(critical(right))||Number(sheltered(left))-Number(sheltered(right))||order(left,right)),kept=new Set(!acute&&policy.keepRunning>0?every.filter(subject=>!grantOver(subject)).slice(-policy.keepRunning).map(subject=>subject.id):[]);
  let covered=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).reduce((sum,subject)=>sum+subject.megabytes,0);
  for(const subject of every.filter(candidate=>!acted.has(candidate.id))){
   if(covered>=deficit||suspends>=policy.maximumSuspendsPerTick)break;
   if(kept.has(subject.id)||!canPause(subject)||damped(subject))continue;
   actions.push({action:"suspend",id:subject.id,reason:level});acted.add(subject.id);suspends++;covered+=subject.megabytes;
  }
 }
 if(recovered&&settled&&woken===0){
  const waiting=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)&&!crossedIds.has(subject.id)&&pageIn(subject)<=remainingResumeRoom).sort(resumeOrder).at(0);
  if(waiting!==undefined)wake(waiting,"calm");
 }
 const mayCancel=available<guard||swapBusy;
 if(!mayCancel)for(let at=actions.length-1;at>=0;at--){const action=actions[at]!;if(action.action==="cancel"&&(action.reason==="danger"||action.reason==="overdue"||action.reason==="limit"))actions.splice(at,1);}
 return Object.freeze({level,calmTicks,swapMilliseconds,hold:level!=="calm",trigger,guardMegabytes:guard,dangerMegabytes:danger,resumeMegabytes:resume,...room,actions:Object.freeze(actions)});
}

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-zA-Z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
/** 🗣️ The words that tell whether a process held its megabytes within or above the grant its owner held for it; nothing without a grant. */
const grantNote=(language:"en"|"de",megabytes:number,grant:number|undefined):string=>grant===undefined?"":fill(MESSAGES[language].grant[megabytes<=grant?"within":"over"],{megabytes,grant,excess:Math.max(0,megabytes-grant)});

const ceilingReason=(language:"en"|"de",ceiling:HostWardenCeilingV1,peakMegabytes:number):string=>{const texts=MESSAGES[language];return fill(ceiling.listed?texts.reasons.ceilingListed:texts.reasons.ceiling,{unit:ceiling.unit,peak:peakMegabytes,ceilingKind:texts.ceilingKinds[ceiling.kind],ceiling:ceiling.megabytes,measured:ceiling.measuredMegabytes??ceiling.megabytes});};

/** 🗣️ Words one alarm in a language: the action, the process (pid, kind, owner, megabytes), the reason and the host numbers (available, guard, danger, swap, pressure). */
export function formatHostWardenAlarmV1(language:"en"|"de",alarm:Pick<HostWardenAlarmV1,"action"|"reason"|"pid"|"kind"|"owner"|"megabytes"|"host"|"observed">&Partial<Pick<HostWardenAlarmV1,"trigger"|"capMegabytes"|"ceiling"|"grantMegabytes">>):string{
 const texts=MESSAGES[language],swap=alarm.host.swapOutMegabytesPerSecond;
 return fill(texts.alarm,{action:texts.actions[alarm.action],kind:alarm.kind,pid:alarm.pid,owner:alarm.owner,megabytes:alarm.megabytes,reason:(alarm.reason==="ceiling"&&alarm.ceiling!==undefined?ceilingReason(language,alarm.ceiling,alarm.megabytes):alarm.reason==="cap"?fill(alarm.action==="report"?texts.reasons.capCrossed:texts.reasons.cap,{cap:alarm.capMegabytes??0,kind:alarm.kind}):alarm.reason==="guard"||alarm.reason==="danger"?texts.triggers[alarm.reason][(alarm.trigger??triggerHostWardenV1(alarm.reason,alarm.host)) as never]:texts.reasons[alarm.reason])+grantNote(language,alarm.megabytes,alarm.grantMegabytes),available:alarm.host.availableMegabytes,guard:alarm.host.guardMegabytes,danger:alarm.host.dangerMegabytes,swap:swap===null?texts.swapUnknown:fill(texts.swapRate,{rate:swap}),pressure:texts.pressures[alarm.host.pressure],headroom:alarm.host.headroomMegabytes===null?"":fill(texts.headroom,{headroom:alarm.host.headroomMegabytes,headroomGuard:alarm.host.headroomGuardMegabytes,headroomDanger:alarm.host.headroomDangerMegabytes}),observed:alarm.observed?texts.observed:""});
}

/** 🔔️ Builds the alarm of one action with its English and German text. */
export function buildHostWardenAlarmV1(input:Readonly<{atMs:number;observed:boolean;level:HostWardenDecisionV1["level"];action:HostWardenActionV1;subject:Pick<HostWardenSubjectV1,"pid"|"kind"|"megabytes"|"owner"|"ceiling"|"grantMegabytes">;host:HostWardenHostV1;decision:Pick<HostWardenDecisionV1,"guardMegabytes"|"dangerMegabytes"|"headroomMegabytes"|"headroomGuardMegabytes"|"headroomDangerMegabytes">&Partial<Pick<HostWardenDecisionV1,"trigger">>}>):HostWardenAlarmV1{
 const base={action:input.action.action,reason:input.action.reason,pid:input.subject.pid,kind:input.subject.kind,owner:input.subject.owner??"",megabytes:input.subject.megabytes,observed:input.observed,...input.action.reason==="guard"||input.action.reason==="danger"?input.decision.trigger===null||input.decision.trigger===undefined?{}:{trigger:input.decision.trigger}:{},...input.action.capMegabytes===undefined?{}:{capMegabytes:input.action.capMegabytes},...input.subject.grantMegabytes===undefined||input.action.action!=="cancel"&&!(input.action.action==="suspend"&&input.subject.megabytes>input.subject.grantMegabytes)?{}:{grantMegabytes:input.subject.grantMegabytes},...input.action.reason==="ceiling"&&input.subject.ceiling!==undefined?{ceiling:input.subject.ceiling}:{},host:{availableMegabytes:input.host.availableMegabytes,guardMegabytes:input.decision.guardMegabytes,dangerMegabytes:input.decision.dangerMegabytes,swapOutMegabytesPerSecond:input.host.swapOutMegabytesPerSecond,pressure:input.host.pressure,headroomMegabytes:input.decision.headroomMegabytes,headroomGuardMegabytes:input.decision.headroomGuardMegabytes,headroomDangerMegabytes:input.decision.headroomDangerMegabytes}} as const;
 return Object.freeze({version:1,atMs:input.atMs,level:input.level,id:input.action.id,...base,text:{en:formatHostWardenAlarmV1("en",base),de:formatHostWardenAlarmV1("de",base)}});
}

/** 🗣️ A plain message of the catalogue in a language (`started`, `stopped`, `locked`, `left`, `observeOnly`). */
export function formatHostWardenNoticeV1(language:"en"|"de",name:"started"|"stopped"|"locked"|"left"|"observeOnly",values:Readonly<Record<string,string|number>>={}):string{
 return fill(MESSAGES[language][name],values);
}

/** 🗣️ The text of a reason the warden stopped (`idle`, `code`, `signal`, `locked`, `limit`). */
export function formatHostWardenStopReasonV1(language:"en"|"de",reason:"idle"|"code"|"signal"|"locked"|"limit",values:Readonly<Record<string,string|number>>={}):string{
 return fill(MESSAGES[language].reasonsStopped[reason],values);
}

/** 🧯️ The words of one cancel record in a language: the typed reason of the cancel (the unit, its peak and its ceiling for a ceiling cancel, the cap, the state of the host) without the host numbers of the alarm. */
export function formatHostWardenCancelReasonV1(language:"en"|"de",record:Pick<HostWardenCancelV1,"reason"|"kind"|"megabytes"|"capMegabytes"|"ceiling"|"grantMegabytes">):string{
 const texts=MESSAGES[language];
 return(record.reason==="ceiling"&&record.ceiling!==undefined?ceilingReason(language,record.ceiling,record.megabytes):record.reason==="cap"?fill(texts.reasons.cap,{cap:record.capMegabytes??record.megabytes,kind:record.kind}):texts.reasons[record.reason])+grantNote(language,record.megabytes,record.grantMegabytes);
}

/** 🧯️ The one line a cancelled process leaves where its owner looks (`cancelLine` of the catalogue): its kind, pid, megabytes and the typed reason of the cancel. */
export function formatHostWardenCancelLineV1(language:"en"|"de",record:Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"megabytes"|"capMegabytes"|"ceiling"|"grantMegabytes">):string{
 return fill(MESSAGES[language].cancelLine,{kind:record.kind,pid:record.pid,megabytes:record.megabytes,reason:formatHostWardenCancelReasonV1(language,record)});
}

/** 🧯️ The message that ends a build whose processes the warden cancelled (`failure` of the catalogue): how many, then one line per cancel with its kind, pid, owner, megabytes and reason, then that nothing is retried. */
export function formatHostWardenCancelFailureV1(language:"en"|"de",records:readonly Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"owner"|"megabytes"|"capMegabytes"|"ceiling"|"grantMegabytes">[],retries:number=0):string{
 const texts=MESSAGES[language].failure;
 return[fill(texts.head,{count:records.length}),...records.map(record=>fill(texts.line,{kind:record.kind,pid:record.pid,owner:record.owner,megabytes:record.megabytes,reason:formatHostWardenCancelReasonV1(language,record)})),retries>0?fill(texts.exhausted,{retries}):texts.tail].join("\n");
}

/** 🔁️ Whether the warden cancelled a process for the memory the host lacked (`danger`, `overdue`, `guard`, `calm`), which a calm host cures, and not for a cap or a ceiling of its own, which a retry reproduces. */
export function hostWardenCancelRetryableV1(records:readonly Pick<HostWardenCancelV1,"reason">[]):boolean{
 return records.length>0&&records.every(record=>record.reason!=="cap"&&record.reason!=="ceiling");
}

/** 🔁️ The one line (English and German) that tells a build its processes were cancelled for memory and that it is queued again (`wait`), is still waiting for a calm host (`waiting`) or is admitted again (`admit`). */
export function formatHostWardenRequeueV1(name:"wait"|"waiting"|"admit",values:Readonly<Record<string,string|number>>):string{
 return(["en","de"] as const).map(language=>fill(MESSAGES[language].requeue[name],values)).join(" / ");
}

/** 🔭️ The largest consumers the warden does not own: the rows outside every subject and its descendants are grouped by their topmost ancestor below the init process, the groups are ranked by resident megabytes (largest first, then pid) and the first `count` are named. Reported, never signalled.
 * @see ./🧬️schema/🔣️.json (`Other`) */
export function topWardenOthersV1(rows:readonly HostWardenRowV1[],subjects:readonly Pick<HostWardenSubjectV1,"pid">[],count:number,prefixes:readonly string[]=[]):readonly HostWardenOtherV1[]{
 const byPid=new Map(rows.map(row=>[row.pid,row] as const)),owned=new Set(subjects.map(subject=>subject.pid)),groups=new Map<number,{root:HostWardenRowV1;kilobytes:number;processes:number}>();
 const owns=(row:HostWardenRowV1):boolean=>{for(let at:HostWardenRowV1|undefined=row,steps=0;at!==undefined&&steps<256;at=byPid.get(at.parent),steps++)if(owned.has(at.pid))return true;return false;};
 const top=(row:HostWardenRowV1):HostWardenRowV1=>{let at=row;for(let steps=0;steps<256;steps++){const parent=byPid.get(at.parent);if(parent===undefined||parent.pid<=1)break;at=parent;}return at;};
 for(const row of rows){
  if(row.pid<=1||owns(row))continue;
  const root=top(row),group=groups.get(root.pid)??{root,kilobytes:0,processes:0};
  group.kilobytes+=row.memoryKilobytes;group.processes++;groups.set(root.pid,group);
 }
 return [...groups.values()].sort((left,right)=>right.kilobytes-left.kilobytes||left.root.pid-right.root.pid).slice(0,Math.max(0,count)).map(group=>Object.freeze({pid:group.root.pid,megabytes:Math.floor(group.kilobytes/1024),processes:group.processes,command:wardenOwnerLabelV1(group.root.command,prefixes)}));
}

/** 🎟️ One live grant of the capacity gate as the warden reads it: the process that holds it and the megabytes the gate reserved for it. */
export type HostWardenGrantV1=Readonly<{id:string;pid:number;reservedMegabytes:number;startMs?:number}>;

/** 🛟️ Gives every live grant a subject, so a grant is enforced whatever its process runs: a subject at the pid of a grant carries the sum of the reservations booked on it; a grant whose pid is no subject (a test runner, a shell, an editor job started by the owner) becomes the subject `granted` of its whole process tree (everything below the pid, including other subjects), which the danger cancel takes after every other candidate (`lastResort`). A grant whose pid is not in the table, or whose start time (`startMs`, tolerance `startToleranceMilliseconds`) is another process, is ignored. Pure.
 * @see ./🧬️schema/🔣️.json (`grantOverMinimumMegabytes`, `grantOverBasisPoints`) */
export function grantedWardenSubjectsV1(rows:readonly HostWardenRowV1[],subjects:readonly HostWardenSubjectV1[],grants:readonly HostWardenGrantV1[],nowMs:number,startToleranceMilliseconds:number=5000,suspendedSince:ReadonlyMap<string,number>=new Map()):readonly HostWardenSubjectV1[]{
 const byPid=new Map(rows.map(row=>[row.pid,row] as const)),booked=new Map<number,number>();
 for(const grant of grants){
  const row=byPid.get(grant.pid);
  if(row===undefined||grant.startMs!==undefined&&Math.abs(nowMs-row.elapsedSeconds*1000-grant.startMs)>startToleranceMilliseconds)continue;
  booked.set(grant.pid,(booked.get(grant.pid)??0)+grant.reservedMegabytes);
 }
 const children=new Map<number,number[]>();for(const row of rows){const list=children.get(row.parent)??[];list.push(row.pid);children.set(row.parent,list);}
 const tree=(pid:number):number=>{let kilobytes=0;const seen=new Set<number>();for(const stack=[pid];stack.length>0;){const at=stack.pop()!;if(seen.has(at))continue;seen.add(at);const row=byPid.get(at);if(row===undefined)continue;kilobytes+=row.memoryKilobytes;stack.push(...children.get(at)??[]);}return Math.floor(kilobytes/1024);};
 const present=new Set(subjects.map(subject=>subject.pid)),carried=subjects.map(subject=>booked.has(subject.pid)&&subject.grantMegabytes===undefined?{...subject,grantMegabytes:booked.get(subject.pid)!}:subject);
 const added=[...booked].filter(([pid])=>!present.has(pid)).sort((left,right)=>left[0]-right[0]).map(([pid,grantMegabytes])=>{const row=byPid.get(pid)!,id=wardenSubjectIdV1(pid,row.start),since=suspendedSince.get(id);return Object.freeze({id,pid,parent:row.parent,kind:"granted",rank:70,elapsedSeconds:row.elapsedSeconds,megabytes:tree(pid),suspended:row.stopped,suspendedSeconds:row.stopped&&since!==undefined?Math.max(0,Math.floor((nowMs-since)/1000)):0,canSuspend:true,canCancel:true,start:row.start,owner:wardenOwnerLabelV1(row.command),scope:"tree" as const,signal:"SIGTERM" as const,lastResort:true,keepsProgress:true,grantMegabytes});});
 return Object.freeze([...carried,...added]);
}

/** 🌐️ What every process outside the subjects holds, whoever started it (the desktop app and its git, browsers, editors, shells): the memory the kernel charges them (the footprint where it could be read, the resident set otherwise) as the sum, their count and how many of them hold at least `heavyMegabytes` on their own. The memory of the pids in `exclude` (the warden and the process of its tests) and the init process are left out. Admission counts the sum against the floor; the warden does not end while a heavy one runs. Pure.
 * @see ./🧬️schema/🔣️.json (`foreignHeavyMegabytes`) */
export function foreignWardenHoldersV1(rows:readonly HostWardenRowV1[],subjects:readonly Pick<HostWardenSubjectV1,"pid">[],heavyMegabytes:number,exclude:ReadonlySet<number>=new Set()):Readonly<{megabytes:number;processes:number;heavy:number}>{
 const byPid=new Map(rows.map(row=>[row.pid,row] as const)),owned=new Set(subjects.map(subject=>subject.pid));
 const owns=(row:HostWardenRowV1):boolean=>{for(let at:HostWardenRowV1|undefined=row,steps=0;at!==undefined&&steps<256;at=byPid.get(at.parent),steps++)if(owned.has(at.pid))return true;return false;};
 let kilobytes=0,processes=0,heavy=0;
 for(const row of rows){
  if(row.pid<=1||exclude.has(row.pid)||owns(row))continue;
  kilobytes+=row.memoryKilobytes;processes++;if(row.memoryKilobytes>=heavyMegabytes*1024)heavy++;
 }
 return Object.freeze({megabytes:Math.floor(kilobytes/1024),processes,heavy});
}

/** 🚧️ Whether new work waits for the warden: its fresh state says hold (the level is not calm) and the work needs at least `holdMinimumMegabytes` or is an operation of `holdOperations` (the builds and runs, whatever they declare), so a status command or a small query never waits behind the memory it would report. At level guard (never at danger) work whose need leaves the available memory at or above the guard line passes: a 256 MB unit does not wait while 4 GB are free.
 * @see ./🧬️schema/🔣️.json (`Policy`) */
export function hostWardenHoldsV1(policy:HostWardenPolicyV1,state:(Pick<HostWardenStateV1,"hold"|"atMs">&Partial<Pick<HostWardenStateV1,"level"|"guardMegabytes">>&Readonly<{host?:Pick<HostWardenHostV1,"availableMegabytes">}>)|null,needMegabytes:number,nowMs:number,operation?:string):boolean{
 const fits=state!==null&&state.level==="guard"&&state.host!==undefined&&state.guardMegabytes!==undefined&&state.host.availableMegabytes-needMegabytes>=state.guardMegabytes;
 return state!==null&&state.hold&&!fits&&nowMs-state.atMs<=policy.staleMilliseconds&&(needMegabytes>=policy.holdMinimumMegabytes||operation!==undefined&&(policy.holdOperations as readonly string[]).includes(operation));
}

const triggerWord=(language:"en"|"de",state:Pick<HostWardenStateV1,"level"|"trigger">):string=>{
 const texts=MESSAGES[language];
 return state.level==="calm"||state.trigger===null||state.trigger===undefined?texts.triggerNone:(texts.triggers[state.level] as Record<string,string>)[state.trigger]??texts.triggerNone;
};

/** 🗣️ The line of a command that waits because the warden holds: the operation, the level in the words of the language (tight, critical), what decided it and the memory numbers. */
export function formatHostWardenHoldV1(language:"en"|"de",operation:string,state:Pick<HostWardenStateV1,"level"|"trigger"|"host">):string{
 return fill(MESSAGES[language].held,{operation,level:MESSAGES[language].levels[state.level],trigger:triggerWord(language,state),available:state.host.availableMegabytes,swap:state.host.swapUsedMegabytes??"?",compressed:state.host.compressedMegabytes??"?"});
}

/** 📋️ The lines of the memory guard for a status command: the level, its trigger, the host numbers and the interventions so far, the last action in the language, the largest consumers the warden does not own; one line saying so where no fresh state exists. */
export function formatHostWardenStatusV1(language:"en"|"de",state:HostWardenStateV1|null):readonly string[]{
 const texts=MESSAGES[language];
 if(state===null)return[texts.statusAbsent];
 const lines=[fill(texts.status,{level:texts.levels[state.level],mode:state.mode,trigger:triggerWord(language,state),available:state.host.availableMegabytes,total:state.host.totalMegabytes,swap:state.host.swapUsedMegabytes??"?",compressed:state.host.compressedMegabytes??"?",pressure:texts.pressures[state.host.pressure],suspended:state.counters.suspended,resumed:state.counters.resumed,cancelled:state.counters.cancelled})];
 const last=state.lastAlarms.reduce<HostWardenAlarmV1|undefined>((newest,row)=>newest===undefined||row.atMs>=newest.atMs?row:newest,undefined);
 if(last!==undefined)lines.push(fill(texts.statusAlarm,{text:last.text[language]}));
 if(state.others!==undefined&&state.others.length>0)lines.push(fill(texts.statusOthers,{others:state.others.map(row=>fill(texts.statusOther,{command:row.command,pid:row.pid,processes:row.processes,megabytes:row.megabytes})).join("; ")}));
 return lines;
}

/** 🛟️ One claim of an owner on a subject (schema `Claim`). */
export type HostWardenClaimV1=Readonly<{version:1;pid:number;owner?:string;atMs:number;grantMegabytes?:number;stopped:boolean}>;

/** 🛟️ Applies the fresh claims (by pid, not older than `claimMaximumMilliseconds`) of the owners to the subjects: a claimed subject carries the grant its owner holds for it, and one the owner itself stopped while it waits for more is `held`, not suspended (so never overdue, never resumed by the warden) and not suspendable by the warden; pure. */
export function applyWardenClaimsV1(subjects:readonly HostWardenSubjectV1[],claims:ReadonlyMap<number,HostWardenClaimV1>,nowMs:number,policy:HostWardenPolicyV1=HOST_WARDEN_POLICY_V1):readonly HostWardenSubjectV1[]{
 return subjects.map(subject=>{
  const claim=claims.get(subject.pid);
  if(claim===undefined||nowMs-claim.atMs>policy.claimMaximumMilliseconds)return subject;
  const granted=claim.grantMegabytes===undefined?{}:{grantMegabytes:claim.grantMegabytes};
  return claim.stopped?{...subject,...granted,suspended:false,suspendedSeconds:0,canSuspend:false,held:true}:{...subject,...granted};
 });
}

/** 🌲️ The heaviest processes of the repository for the state file: a process counts when its command line names one of the `workspaces`, when it is one of the `roots` (the subjects the warden classified, which may be orphans of a dead command) or when it descends from one that does; the workspace root shows as `~` and the bun runtime path as `bun` in the command; at most `limit` rows, the most CPU time first, then the most memory, then the lowest pid, so the same table gives the same rows. Pure.
 * @see ./🧪️tests/🌲️tree/🐍️.py */
export function wardenTreeV1(rows:readonly HostWardenRowV1[],workspaces:readonly string[],limit:number=12,roots:readonly number[]=[]):readonly HostWardenTreeRowV1[]{
 const byPid=new Map(rows.map(row=>[row.pid,row])),inside=new Map<number,boolean>();
 const rooted=new Set(roots),named=(row:HostWardenRowV1)=>rooted.has(row.pid)||workspaces.some(root=>root!==""&&row.command.includes(root));
 const short=(command:string)=>workspaces.reduce((text,root)=>root===""?text:text.split(root).join("~"),command).replace(/\S*\/runtimes\/bun\/[0-9a-f]+\/bun /gu,"bun ");
 const decide=(row:HostWardenRowV1,seen:Set<number>):boolean=>{
  const known=inside.get(row.pid);if(known!==undefined)return known;
  if(seen.has(row.pid))return false;
  seen.add(row.pid);
  const parent=byPid.get(row.parent),value=named(row)||parent!==undefined&&parent.pid!==row.pid&&decide(parent,seen);
  inside.set(row.pid,value);return value;
 };
 return rows.filter(row=>decide(row,new Set())).map(row=>({pid:row.pid,parent:row.parent,elapsedSeconds:row.elapsedSeconds,...row.cpuSeconds===undefined?{}:{cpuSeconds:row.cpuSeconds},megabytes:Math.floor(row.memoryKilobytes/1024),command:short(row.command).slice(0,240)}))
  .sort((left,right)=>(right.cpuSeconds??-1)-(left.cpuSeconds??-1)||right.megabytes-left.megabytes||left.pid-right.pid).slice(0,limit);
}
