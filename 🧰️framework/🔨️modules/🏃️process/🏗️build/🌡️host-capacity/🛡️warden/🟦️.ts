import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {UNIT_CEILING_POLICY_V1,runtimeCeilingOfCommandV1,type CompileBudgetPolicyV1} from "../../../📦️artifacts/🏗️native-build/🎟️admission/🏔️ceiling/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const MESSAGES=schema.definitions.Messages.const;

/** 🎚️ The authored numbers of the host warden (schema `Policy`): the ladder lines, the per-tick maxima, the loop intervals and the file names of its directory. */
export type HostWardenPolicyV1=Readonly<typeof schema.definitions.Policy.const>;
/** 🧩️ One kind of subject (schema `Kinds`): the pattern over the command line, the cost rank, the actions the warden may take, the cancel signal and the scope a signal reaches. */
export type HostWardenKindV1=Readonly<{name:string;rank:number;pattern:string;actions:readonly("suspend"|"cancel")[];signal:"SIGKILL"|"SIGTERM";scope:"process"|"group"|"tree";capMegabytes?:number;capCritical?:boolean;ceiling?:"unit";source?:string}>;
/** 🗃️ The pattern tables classification reads: roots (repo-started by ancestry), identities (repo-owned by what the process is, whatever its ancestry), never (excluded whatever they hold) and the ordered kinds. */
export type HostWardenTablesV1=Readonly<{roots:readonly string[];identities?:readonly string[];never:readonly string[];kinds:readonly HostWardenKindV1[]}>;
/** 🌡️ The host numbers the ladder reads (schema `Host`): the memory counters, the free swap and the free space of the swap volume (null where unknown or where swap cannot grow). */
export type HostWardenHostV1=Readonly<{totalMegabytes:number;availableMegabytes:number;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical";swapFreeMegabytes:number|null;swapVolumeFreeMegabytes:number|null}>;
/** 🏔️ The runtime ceiling of a compile unit (schema `Ceiling`): the unit (`<crate>|<target>`), its budget kind, the most its resident footprint may reach, whether it is on the carving list of the admission policy and, then, its measured peak. */
export type HostWardenCeilingV1=Readonly<{unit:string;kind:"library"|"final";megabytes:number;listed:boolean;measuredMegabytes?:number}>;
/** 🧾️ One row of the process table (schema `ProcessRow`). */
export type HostWardenRowV1=Readonly<{pid:number;parent:number;group:number;memoryKilobytes:number;stopped:boolean;elapsedSeconds:number;start:string;command:string}>;
/** 🎯️ One process the warden may act on (schema `Subject`): the decision reads the first ten fields, the effects the rest. */
export type HostWardenSubjectV1=Readonly<{id:string;pid:number;kind:string;rank:number;elapsedSeconds:number;megabytes:number;suspended:boolean;suspendedSeconds:number;canSuspend:boolean;canCancel:boolean;parent?:number;start?:string;owner?:string;scope?:"process"|"group"|"tree";signal?:"SIGKILL"|"SIGTERM";capMegabytes?:number;capCritical?:boolean;ceiling?:HostWardenCeilingV1;suspensions?:number;resumedSeconds?:number}>;
/** 🎯️ What decided a level (schema `Trigger`): the available memory, the headroom, the swap in use, its share of a fixed swap, the compressed memory, the OS pressure (warn, critical) or the swap-out rate. */
export type HostWardenTriggerV1="available"|"headroom"|"swap-used"|"swap-share"|"compressed"|"pressure"|"critical"|"swap"|"cap";
/** 🔭️ One consumer the warden does not own (schema `Other`): the topmost process of a tree outside every subject, with the resident megabytes and the process count of the whole tree. */
export type HostWardenOtherV1=Readonly<{pid:number;megabytes:number;processes:number;command:string}>;
/** 🔙️ What the previous tick left (schema `Previous`). */
export type HostWardenPreviousV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number}>;
/** ⚙️ One step of a decision (schema `Action`). */
export type HostWardenActionV1=Readonly<{action:"suspend"|"resume"|"cancel"|"report";id:string;reason:"guard"|"danger"|"overdue"|"calm"|"cap"|"ceiling";capMegabytes?:number}>;
/** ⚖️ The verdict of one tick (schema `Decision`). */
export type HostWardenDecisionV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number;hold:boolean;trigger:HostWardenTriggerV1|null;guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;actions:readonly HostWardenActionV1[]}>;
/** 🔔️ One line of `alarms.jsonl` (schema `Alarm`). */
export type HostWardenAlarmV1=Readonly<{version:1;atMs:number;observed:boolean;level:"calm"|"guard"|"danger";action:"suspend"|"resume"|"cancel"|"report";reason:"guard"|"danger"|"overdue"|"calm"|"cap"|"ceiling";id:string;pid:number;kind:string;owner:string;megabytes:number;trigger?:HostWardenTriggerV1;capMegabytes?:number;ceiling?:HostWardenCeilingV1;host:Readonly<{availableMegabytes:number;guardMegabytes:number;dangerMegabytes:number;swapOutMegabytesPerSecond:number|null;pressure:"normal"|"warn"|"critical";headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number}>;text:Readonly<{en:string;de:string}>}>;
/** 🗂️ The cancel record written before a cancel signal (schema `Cancel`). */
export type HostWardenCancelV1=Readonly<{version:1;pid:number;start:string;kind:string;owner:string;command:string;megabytes:number;atMs:number;reason:"guard"|"danger"|"overdue"|"calm"|"cap"|"ceiling";ancestors:readonly number[];buildScriptOutput?:string;capMegabytes?:number;ceiling?:HostWardenCeilingV1}>;
/** 📋️ The state file of the running warden (schema `State`), written each tick: host numbers, level, subjects by kind, the suspended set, counters, the last alarms and the overhead of the loop. */
export type HostWardenStateV1=Readonly<{version:1;atMs:number;pid:number;mode:"observe"|"enforce";code:string;host:HostWardenHostV1;level:"calm"|"guard"|"danger";hold:boolean;trigger?:HostWardenTriggerV1|null;others?:readonly HostWardenOtherV1[];guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;subjects:Readonly<{count:number;megabytes:number;byKind:Readonly<Record<string,Readonly<{count:number;megabytes:number}>>>}>;suspended:readonly Readonly<{id:string;pid:number;start:string;kind:string;sinceMs:number}>[];counters:Readonly<{suspended:number;resumed:number;cancelled:number;ticks:number}>;lastAlarms:readonly HostWardenAlarmV1[];overhead:Readonly<{tickMillisecondsP50:number;tickMillisecondsP95:number;cpuBasisPoints:number;residentMegabytes:number}>}>;
/** 🪞️ The memory record a worker mirrors to the group store (schema `Mirror`): the state of its warden as written, or the absent record. */
export type HostWardenMirrorV1=HostWardenStateV1|Readonly<{version:1;atMs:number;absent:true}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostWardenDefinitionV1="Host"|"ProcessRow"|"Subject"|"Previous"|"Action"|"Decision"|"Alarm"|"Ceiling"|"Cancel"|"Lock"|"Protect"|"State"|"Mode"|"Absent"|"Mirror";

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

const ROW=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\S+)\s+((?:\d+-)?(?:\d+:)?\d+:\d+)\s+(\S+\s+\S+\s+\S+\s+\S+\s+\S+?)(?:\s+(.*))?$/u;
/** 🌳️ Parses `ps -A -o pid=,ppid=,pgid=,rss=,state=,etime=,lstart=,args=` (macOS and Linux; without the `args=` column the commands are empty) into rows: a stopped process is one whose state starts with `T`, the start is the five-token `lstart` text; a line that is no row (the continuation of a multi-line command) is skipped. */
export function parseWardenProcessTableV1(text:string):readonly HostWardenRowV1[]{
 const rows:HostWardenRowV1[]=[];
 for(const line of text.split("\n")){
  const match=ROW.exec(line);
  if(match)rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),group:Number(match[3]),memoryKilobytes:Number(match[4]),stopped:match[5]!.startsWith("T"),elapsedSeconds:parseWardenElapsedV1(match[6]!),start:match[7]!,command:match[8]??""}));
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
export type HostWardenClassifyOptionsV1=Readonly<{protect?:readonly string[];within?:number;prefixes?:readonly string[];nowMs?:number;suspendedSince?:ReadonlyMap<string,number>;workspaces?:readonly string[];ceilings?:CompileBudgetPolicyV1}>;

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
 for(const row of rows){const owner=home(row);if(owner!==undefined)kilobytes.set(owner,(kilobytes.get(owner)??0)+row.memoryKilobytes);}
 const ceilingOf=(kind:HostWardenKindV1,row:HostWardenRowV1):Readonly<{ceiling?:HostWardenCeilingV1}>=>{const ceiling=kind.ceiling==="unit"?runtimeCeilingOfCommandV1(options.ceilings??UNIT_CEILING_POLICY_V1,row.command):null;return ceiling===null?{}:{ceiling};};
 return [...found.values()].sort((left,right)=>left.row.pid-right.row.pid).map(({row,kind})=>{
  const id=wardenSubjectIdV1(row.pid,row.start),since=options.suspendedSince?.get(id),seconds=row.stopped&&since!==undefined&&options.nowMs!==undefined?Math.max(0,Math.floor((options.nowMs-since)/1000)):0,top=byPid.get(rootOf(row))??row;
  return Object.freeze({id,pid:row.pid,parent:row.parent,kind:kind.name,rank:kind.rank,elapsedSeconds:row.elapsedSeconds,megabytes:Math.floor((kilobytes.get(row.pid)??0)/1024),suspended:row.stopped,suspendedSeconds:seconds,canSuspend:kind.actions.includes("suspend"),canCancel:kind.actions.includes("cancel"),start:row.start,owner:wardenOwnerLabelV1(top.command,options.prefixes),scope:kind.scope,signal:kind.signal,...kind.capMegabytes===undefined?{}:{capMegabytes:kind.capMegabytes},...kind.capCritical===true?{capCritical:true}:{},...ceilingOf(kind,row)});
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

/** 🪙️ The order of the victims, cheapest to lose first: whatever is not build-critical before what is, then the least work lost per megabyte freed (wall seconds the subject ran divided by its megabytes, compared by cross multiplication so a young unit or pool child goes before a long running final), then the cost rank, then the pid. The last of a list is the dearest to lose. */
const order=(left:HostWardenSubjectV1,right:HostWardenSubjectV1):number=>Number(left.capCritical===true)-Number(right.capCritical===true)||left.elapsedSeconds*Math.max(1,right.megabytes)-right.elapsedSeconds*Math.max(1,left.megabytes)||left.rank-right.rank||left.pid-right.pid;
const clamp=(value:number,low:number,high:number):number=>Math.min(high,Math.max(low,value));

/** 🌊️ How deep the swap and the compressed memory are (schema `Policy`): 2 at or above a danger line, 1 at or above a guard line, 0 below. The swap in use is read as a share of the physical memory and, when swap occupancy is known, also as a share of the existing swap plus the disk space available to grow it above the reserve; the compressed memory as a share of the physical memory. The source is the first of swap-used, swap-share and compressed that reaches the depth.
 * @see ./🧬️schema/🔣️.json */
export function swapDepthHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1):Readonly<{depth:0|1|2;compressed:0|1|2;source:"swap-used"|"swap-share"|"compressed"|null}>{
 const total=host.totalMegabytes,used=host.swapUsedMegabytes,free=host.swapFreeMegabytes,compressed=host.compressedMegabytes;
 const growth = Math.max(0, (host.swapVolumeFreeMegabytes ?? 0) - policy.swapDiskReserveMegabytes);
 const swapCapacity = used === null || free === null ? 0 : used + free + growth;
 const depthOf=(value:number|null,whole:number,guard:number,danger:number):0|1|2=>value===null?0:value*10000>=whole*danger?2:value*10000>=whole*guard?1:0;
 const lines=[
  ["swap-used",depthOf(used,total,policy.swapUsedGuardBasisPoints,policy.swapUsedDangerBasisPoints)],
  ["swap-share",swapCapacity>0?depthOf(used,swapCapacity,policy.swapShareGuardBasisPoints,policy.swapShareDangerBasisPoints):0],
  ["compressed",depthOf(compressed,total,policy.compressedGuardBasisPoints,policy.compressedDangerBasisPoints)],
 ] as const,depth=Math.max(...lines.map(line=>line[1])) as 0|1|2;
 return{depth,compressed:lines[2][1],source:depth===0?null:lines.find(line=>line[1]===depth)![0]};
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
export function parseWardenSwapV1(platform:string,texts:Readonly<{usage?:string|null;disk?:string|null;meminfo?:string|null}>):Readonly<{swapFreeMegabytes:number|null;swapVolumeFreeMegabytes:number|null}>{
 if(platform==="darwin"){
  const free=/free = ([\d.]+)([KMGT])/u.exec(texts.usage??""),scale:Record<string,number>={K:1/1024,M:1,G:1024,T:1048576},lines=(texts.disk??"").trim().split("\n"),columns=lines.length>1?lines.at(-1)!.trim().split(/\s+/u):[],available=Number(columns[3]);
  return{swapFreeMegabytes:free?Math.floor(Number(free[1])*scale[free[2]!]!):null,swapVolumeFreeMegabytes:Number.isFinite(available)&&columns.length>3?Math.floor(available/1024):null};
 }
 if(platform==="linux"){
  const kilobytes=(name:string):number|null=>{const match=new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(texts.meminfo??"");return match?Number(match[1]):null;},total=kilobytes("SwapTotal"),free=kilobytes("SwapFree");
  return{swapFreeMegabytes:total===null||free===null||total===0?null:Math.floor(free/1024),swapVolumeFreeMegabytes:null};
 }
 return{swapFreeMegabytes:null,swapVolumeFreeMegabytes:null};
}

/** ⚖️ Decides one tick of the warden from the host numbers, the subjects and what the previous tick left; pure, no clock and no I/O. The law: guard, danger and resume lines are shares of the physical memory between their bounds. The level is `danger` below the danger line, with a headroom (available memory plus the swap room, bounded by the free disk of the swap volume, see {@link headroomHostWardenV1}) below the headroom danger line, with the swap or compressed memory at danger depth ({@link swapDepthHostWardenV1}) and the available memory below the guard line, or during active swap-out with danger-depth compression or a continuing OS warning and guard-depth compression (these are severe), or, whatever the available memory, with the OS pressure critical or the compressed memory at danger depth; `guard` below the guard line, with a headroom below the headroom guard line, with the OS pressure warn, with the compressed memory at guard depth, or with the swap worked or the swap or compressed memory at guard depth and the available memory below the resume line; else `calm`. So the OS pressure alone is at least tight (new work is held and the cheapest subjects are suspended) and critical alone is critical; only a severe danger cancels (suspended subjects first, until the freed megabytes cover the deficit), a danger from the pressure or the compressed memory alone suspends and cancels what stays suspended too long. The deficit is the distance to the guard line (to the resume line with a deep swap), never less than `pressureDeficitMegabytes` while acute compression and swap-out, or pressure or compressed memory without a severe shortage, holds the level. A subject of a build-critical kind (`capCritical`: the compile units, the graph pool children, the slice workers) that crosses its cap is not killed first: the level is at least tight (trigger `cap`), the other subjects are suspended until the overage is covered (the critical ones keep running), the crossing is reported as an alarm of action `report`, and the subject is cancelled only when the level is danger, when it stays suspended too long (see below) or when it is over its ceiling. When nothing but the compressed or swapped memory holds the level (pressure normal, available memory at or above the guard line, headroom above its guard line, swap not worked) the level only holds new admissions: nothing is suspended or cancelled for it, and one suspended subject, the most expensive, is resumed per tick, because compressed memory stays high long after the load is gone. A subject over the cap of its kind (`capMegabytes` of the kind, at most `capMaximumBasisPoints` of the physical memory) is cancelled at any level. A subject whose runtime ceiling (`ceiling`, set for compile units) is below its megabytes is cancelled at any level at once, before every cap and every shedding, build-critical or not (a unit does not outgrow the budget its kind or its carving entry declares); the cancel counts as freed memory. The level is quiet when no measured signal is at or below its line: the available memory at or above the guard line, the headroom at or above its guard line, the swap-out rate below `swapActiveMegabytesPerSecond` and the OS pressure not critical; the OS pressure `warn` and the compressed or swapped memory alone are only felt by the kernel, so a quiet level holds new admissions and suspends and cancels nothing (a suspended subject is resumed, one per tick, the dearest first). A suspended subject that stays suspended for `suspendMaximumSeconds` is overdue; it is cancelled only while a measured signal is at or below its line (not quiet) and not calm, and resumed when it cannot be cancelled, when the level is quiet or calm. Victims, suspended or cancelled, go in the order of `order`: not build-critical first, then the least work lost per megabyte freed (elapsed wall seconds per megabyte), then the cost rank. A subject suspended `flapMaximumSuspensions` times within `flapWindowSeconds`, or resumed less than `resumeHoldSeconds` ago, is not suspended again (damped) unless the level is severe danger. Overdue suspended subjects are cancelled (not calm and not quiet) or resumed (cannot cancel, quiet or calm); in guard and danger the cheapest running subjects are suspended until the suspended megabytes cover the deficit, the most expensive `keepRunning` keeps running unless compression and active swap-out are acute; after `calmTicks` calm ticks above the resume line one suspended subject, the most expensive, is resumed per tick.
 * @see ./🧬️schema/🔣️.json */
export function decideHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1,subjects:readonly HostWardenSubjectV1[],previous:HostWardenPreviousV1):HostWardenDecisionV1{
 const total=host.totalMegabytes,available=host.availableMegabytes;
 const guard=clamp(Math.floor(total*policy.guardBasisPoints/10000),policy.guardMinimumMegabytes,policy.guardMaximumMegabytes),danger=clamp(Math.floor(total*policy.dangerBasisPoints/10000),policy.dangerMinimumMegabytes,policy.dangerMaximumMegabytes),resume=Math.max(guard,Math.floor(total*policy.resumeBasisPoints/10000));
 const swapWorked=host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.swapActiveMegabytesPerSecond;
 const room=headroomHostWardenV1(policy,host),headroom=room.headroomMegabytes,deep=swapDepthHostWardenV1(policy,host),pressure=host.pressure==="critical"?2:host.pressure==="warn"?1:0;
 const critical=(subject:HostWardenSubjectV1):boolean=>subject.capCritical===true,capOf=(subject:HostWardenSubjectV1):number=>Math.min(subject.capMegabytes!,Math.floor(total*policy.capMaximumBasisPoints/10000));
 const over=(subject:HostWardenSubjectV1):boolean=>subject.ceiling!==undefined&&subject.canCancel&&subject.megabytes>subject.ceiling.megabytes;
 const crossed=subjects.filter(subject=>subject.canCancel&&(over(subject)||subject.capMegabytes!==undefined&&critical(subject)&&subject.megabytes>capOf(subject))),overage=crossed.reduce((sum,subject)=>sum+subject.megabytes-(over(subject)?subject.ceiling!.megabytes:capOf(subject)),0);
 const sustainedWarning = previous.level !== "calm" && pressure >= 1 && deep.compressed >= 1;
 const acute = swapWorked && (deep.compressed === 2 || sustainedWarning);
 const severe=available<danger||(headroom!==null&&headroom<room.headroomDangerMegabytes)||(deep.depth===2&&available<guard)||acute;
 const base:HostWardenDecisionV1["level"]=severe||pressure===2||deep.compressed===2?"danger":available<guard||(headroom!==null&&headroom<room.headroomGuardMegabytes)||pressure>=1||deep.compressed>=1||((swapWorked||deep.depth>=1)&&available<resume)?"guard":"calm",level:HostWardenDecisionV1["level"]=base==="calm"&&crossed.length>0?"guard":base;
 const trigger:HostWardenTriggerV1|null=level==="calm"?null:base==="calm"?"cap":available<(level==="danger"?danger:guard)?"available":headroom!==null&&headroom<(level==="danger"?room.headroomDangerMegabytes:room.headroomGuardMegabytes)?"headroom":level==="danger"?deep.depth===2&&available<guard?deep.source!:deep.compressed===2?"compressed":"pressure":pressure===1?"pressure":deep.compressed>=1?"compressed":deep.depth>=1&&available<resume&&deep.source!==null?deep.source:"swap";
 const quiet=pressure<2&&available>=guard&&!(headroom!==null&&headroom<room.headroomGuardMegabytes)&&!swapWorked;
 const held=(acute||!severe&&(pressure>=1||deep.compressed>=1))?policy.pressureDeficitMegabytes:0,target=deep.depth>=1?resume:guard,deficitOf=():number=>Math.max(target-available,headroom===null?0:room.headroomGuardMegabytes-headroom,held,overage,1);
 const damped=(subject:HostWardenSubjectV1):boolean=>!(level==="danger"&&severe)&&((subject.suspensions??0)>=policy.flapMaximumSuspensions||(subject.resumedSeconds!==undefined&&subject.resumedSeconds<policy.resumeHoldSeconds));
 const actions:HostWardenActionV1[]=[],acted=new Set<string>(),capLimit=Math.floor(total*policy.capMaximumBasisPoints/10000);
 let cancels=0,suspends=0,capped=0;
 const cancel=(subject:HostWardenSubjectV1,reason:"danger"|"overdue"):void=>{actions.push({action:"cancel",id:subject.id,reason});acted.add(subject.id);cancels++;};
 for(const subject of [...subjects].sort(order)){
  if(!over(subject))continue;
  if(level!=="danger"){actions.push({action:"report",id:subject.id,reason:"ceiling"});continue;}
  actions.push({action:"cancel",id:subject.id,reason:"ceiling"});acted.add(subject.id);capped+=subject.megabytes;
 }
 for(const subject of [...subjects].sort(order)){
  if(subject.capMegabytes===undefined||!subject.canCancel||acted.has(subject.id)||over(subject))continue;
  const cap=Math.min(subject.capMegabytes,capLimit);
  if(subject.megabytes<=cap)continue;
  if(critical(subject)&&level!=="danger"){actions.push({action:"report",id:subject.id,reason:"cap",capMegabytes:cap});continue;}
  actions.push({action:"cancel",id:subject.id,reason:"cap",capMegabytes:cap});acted.add(subject.id);capped+=subject.megabytes;
 }
 const overdue=subjects.filter(subject=>subject.suspended&&subject.suspendedSeconds>=policy.suspendMaximumSeconds&&!acted.has(subject.id)).sort(order);
 const cancelled:HostWardenSubjectV1[]=[];
 if(level!=="calm"&&!quiet)for(const subject of overdue)if(subject.canCancel&&(!critical(subject)||severe)&&cancels<policy.maximumCancelsPerTick){cancel(subject,"overdue");cancelled.push(subject);}
 for(const subject of overdue)if(!subject.canCancel||level!=="calm"&&quiet||critical(subject)&&!severe){actions.push({action:"resume",id:subject.id,reason:"overdue"});acted.add(subject.id);}
 if(level==="danger"&&severe){
  const deficit=deficitOf();
  let freed=capped+cancelled.reduce((sum,subject)=>sum+subject.megabytes,0);
  const pausable=subjects.filter(subject=>subject.canSuspend),paused=(subject:HostWardenSubjectV1):boolean=>subject.suspended&&subject.suspendedSeconds>=policy.pauseDeadlineSeconds,pausingFailed=pausable.every(paused);
  const candidates=subjects.filter(subject=>subject.canCancel&&!acted.has(subject.id)&&(subject.canSuspend?paused(subject):pausingFailed)).sort((left,right)=>Number(right.suspended)-Number(left.suspended)||order(left,right));
  for(const subject of candidates){
   if(freed>=deficit||cancels>=policy.maximumCancelsPerTick)break;
   cancel(subject,"danger");freed+=subject.megabytes;
  }
 }
 if(level!=="calm"&&(!quiet||crossed.length>0)){
  const deficit=deficitOf(),every=subjects.filter(subject=>!subject.suspended&&!(quiet&&critical(subject))).sort((left,right)=>Number(critical(left))-Number(critical(right))||order(left,right)),kept=new Set(!acute&&policy.keepRunning>0?every.slice(-policy.keepRunning).map(subject=>subject.id):[]);
  let covered=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).reduce((sum,subject)=>sum+subject.megabytes,0);
  for(const subject of every.filter(candidate=>!acted.has(candidate.id))){
   if(covered>=deficit||suspends>=policy.maximumSuspendsPerTick)break;
   if(kept.has(subject.id)||!subject.canSuspend||damped(subject))continue;
   actions.push({action:"suspend",id:subject.id,reason:level});acted.add(subject.id);suspends++;covered+=subject.megabytes;
  }
 }
 const calmTicks=level==="calm"?(previous.level==="calm"?previous.calmTicks:0)+1:0;
 if(level!=="calm"&&quiet&&crossed.length===0){
  const waiting=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).sort(order).at(-1);
  if(waiting!==undefined)actions.push({action:"resume",id:waiting.id,reason:"calm"});
 }
 if(level==="calm"&&calmTicks>=policy.calmTicks&&available>=resume){
  const waiting=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).sort(order).at(-1);
  if(waiting!==undefined)actions.push({action:"resume",id:waiting.id,reason:"calm"});
 }
 return Object.freeze({level,calmTicks,hold:level!=="calm",trigger,guardMegabytes:guard,dangerMegabytes:danger,resumeMegabytes:resume,...room,actions:Object.freeze(actions)});
}

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-zA-Z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
const ceilingReason=(language:"en"|"de",ceiling:HostWardenCeilingV1,peakMegabytes:number):string=>{const texts=MESSAGES[language];return fill(ceiling.listed?texts.reasons.ceilingListed:texts.reasons.ceiling,{unit:ceiling.unit,peak:peakMegabytes,ceilingKind:texts.ceilingKinds[ceiling.kind],ceiling:ceiling.megabytes,measured:ceiling.measuredMegabytes??ceiling.megabytes});};

/** 🗣️ Words one alarm in a language: the action, the process (pid, kind, owner, megabytes), the reason and the host numbers (available, guard, danger, swap, pressure). */
export function formatHostWardenAlarmV1(language:"en"|"de",alarm:Pick<HostWardenAlarmV1,"action"|"reason"|"pid"|"kind"|"owner"|"megabytes"|"host"|"observed">&Partial<Pick<HostWardenAlarmV1,"trigger"|"capMegabytes"|"ceiling">>):string{
 const texts=MESSAGES[language],swap=alarm.host.swapOutMegabytesPerSecond;
 return fill(texts.alarm,{action:texts.actions[alarm.action],kind:alarm.kind,pid:alarm.pid,owner:alarm.owner,megabytes:alarm.megabytes,reason:alarm.reason==="ceiling"&&alarm.ceiling!==undefined?ceilingReason(language,alarm.ceiling,alarm.megabytes):alarm.reason==="cap"?fill(alarm.action==="report"?texts.reasons.capCrossed:texts.reasons.cap,{cap:alarm.capMegabytes??0,kind:alarm.kind}):alarm.reason==="guard"||alarm.reason==="danger"?texts.triggers[alarm.reason][(alarm.trigger??triggerHostWardenV1(alarm.reason,alarm.host)) as never]:texts.reasons[alarm.reason],available:alarm.host.availableMegabytes,guard:alarm.host.guardMegabytes,danger:alarm.host.dangerMegabytes,swap:swap===null?texts.swapUnknown:fill(texts.swapRate,{rate:swap}),pressure:texts.pressures[alarm.host.pressure],headroom:alarm.host.headroomMegabytes===null?"":fill(texts.headroom,{headroom:alarm.host.headroomMegabytes,headroomGuard:alarm.host.headroomGuardMegabytes,headroomDanger:alarm.host.headroomDangerMegabytes}),observed:alarm.observed?texts.observed:""});
}

/** 🔔️ Builds the alarm of one action with its English and German text. */
export function buildHostWardenAlarmV1(input:Readonly<{atMs:number;observed:boolean;level:HostWardenDecisionV1["level"];action:HostWardenActionV1;subject:Pick<HostWardenSubjectV1,"pid"|"kind"|"megabytes"|"owner"|"ceiling">;host:HostWardenHostV1;decision:Pick<HostWardenDecisionV1,"guardMegabytes"|"dangerMegabytes"|"headroomMegabytes"|"headroomGuardMegabytes"|"headroomDangerMegabytes">&Partial<Pick<HostWardenDecisionV1,"trigger">>}>):HostWardenAlarmV1{
 const base={action:input.action.action,reason:input.action.reason,pid:input.subject.pid,kind:input.subject.kind,owner:input.subject.owner??"",megabytes:input.subject.megabytes,observed:input.observed,...input.action.reason==="guard"||input.action.reason==="danger"?input.decision.trigger===null||input.decision.trigger===undefined?{}:{trigger:input.decision.trigger}:{},...input.action.capMegabytes===undefined?{}:{capMegabytes:input.action.capMegabytes},...input.action.reason==="ceiling"&&input.subject.ceiling!==undefined?{ceiling:input.subject.ceiling}:{},host:{availableMegabytes:input.host.availableMegabytes,guardMegabytes:input.decision.guardMegabytes,dangerMegabytes:input.decision.dangerMegabytes,swapOutMegabytesPerSecond:input.host.swapOutMegabytesPerSecond,pressure:input.host.pressure,headroomMegabytes:input.decision.headroomMegabytes,headroomGuardMegabytes:input.decision.headroomGuardMegabytes,headroomDangerMegabytes:input.decision.headroomDangerMegabytes}} as const;
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
export function formatHostWardenCancelReasonV1(language:"en"|"de",record:Pick<HostWardenCancelV1,"reason"|"kind"|"megabytes"|"capMegabytes"|"ceiling">):string{
 const texts=MESSAGES[language];
 return record.reason==="ceiling"&&record.ceiling!==undefined?ceilingReason(language,record.ceiling,record.megabytes):record.reason==="cap"?fill(texts.reasons.cap,{cap:record.capMegabytes??record.megabytes,kind:record.kind}):texts.reasons[record.reason];
}

/** 🧯️ The one line a cancelled process leaves where its owner looks (`cancelLine` of the catalogue): its kind, pid, megabytes and the typed reason of the cancel. */
export function formatHostWardenCancelLineV1(language:"en"|"de",record:Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"megabytes"|"capMegabytes"|"ceiling">):string{
 return fill(MESSAGES[language].cancelLine,{kind:record.kind,pid:record.pid,megabytes:record.megabytes,reason:formatHostWardenCancelReasonV1(language,record)});
}

/** 🧯️ The message that ends a build whose processes the warden cancelled (`failure` of the catalogue): how many, then one line per cancel with its kind, pid, owner, megabytes and reason, then that nothing is retried. */
export function formatHostWardenCancelFailureV1(language:"en"|"de",records:readonly Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"owner"|"megabytes"|"capMegabytes"|"ceiling">[]):string{
 const texts=MESSAGES[language].failure;
 return[fill(texts.head,{count:records.length}),...records.map(record=>fill(texts.line,{kind:record.kind,pid:record.pid,owner:record.owner,megabytes:record.megabytes,reason:formatHostWardenCancelReasonV1(language,record)})),texts.tail].join("\n");
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

/** 🚧️ Whether new work waits for the warden: its fresh state says hold (the level is not calm) and the work needs at least `holdMinimumMegabytes` or is an operation of `holdOperations` (the builds and runs, whatever they declare), so a status command or a small query never waits behind the memory it would report.
 * @see ./🧬️schema/🔣️.json (`Policy`) */
export function hostWardenHoldsV1(policy:HostWardenPolicyV1,state:Pick<HostWardenStateV1,"hold"|"atMs">|null,needMegabytes:number,nowMs:number,operation?:string):boolean{
 return state!==null&&state.hold&&nowMs-state.atMs<=policy.staleMilliseconds&&(needMegabytes>=policy.holdMinimumMegabytes||operation!==undefined&&(policy.holdOperations as readonly string[]).includes(operation));
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
