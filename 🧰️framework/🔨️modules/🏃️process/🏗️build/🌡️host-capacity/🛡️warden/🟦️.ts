import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const MESSAGES=schema.definitions.Messages.const;

/** 🎚️ The authored numbers of the host warden (schema `Policy`): the ladder lines, the per-tick maxima, the loop intervals and the file names of its directory. */
export type HostWardenPolicyV1=Readonly<typeof schema.definitions.Policy.const>;
/** 🧩️ One kind of subject (schema `Kinds`): the pattern over the command line, the cost rank, the actions the warden may take, the cancel signal and the scope a signal reaches. */
export type HostWardenKindV1=Readonly<{name:string;rank:number;pattern:string;actions:readonly("suspend"|"cancel")[];signal:"SIGKILL"|"SIGTERM";scope:"process"|"group"|"tree";source?:string}>;
/** 🗃️ The pattern tables classification reads: roots (repo-started by ancestry), identities (repo-owned by what the process is, whatever its ancestry), never (excluded whatever they hold) and the ordered kinds. */
export type HostWardenTablesV1=Readonly<{roots:readonly string[];identities?:readonly string[];never:readonly string[];kinds:readonly HostWardenKindV1[]}>;
/** 🌡️ The host numbers the ladder reads (schema `Host`): the memory counters, the free swap and the free space of the swap volume (null where unknown or where swap cannot grow). */
export type HostWardenHostV1=Readonly<{totalMegabytes:number;availableMegabytes:number;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical";swapFreeMegabytes:number|null;swapVolumeFreeMegabytes:number|null}>;
/** 🧾️ One row of the process table (schema `ProcessRow`). */
export type HostWardenRowV1=Readonly<{pid:number;parent:number;group:number;rssKilobytes:number;stopped:boolean;elapsedSeconds:number;start:string;command:string}>;
/** 🎯️ One process the warden may act on (schema `Subject`): the decision reads the first ten fields, the effects the rest. */
export type HostWardenSubjectV1=Readonly<{id:string;pid:number;kind:string;rank:number;elapsedSeconds:number;megabytes:number;suspended:boolean;suspendedSeconds:number;canSuspend:boolean;canCancel:boolean;parent?:number;start?:string;owner?:string;scope?:"process"|"group"|"tree";signal?:"SIGKILL"|"SIGTERM"}>;
/** 🔙️ What the previous tick left (schema `Previous`). */
export type HostWardenPreviousV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number}>;
/** ⚙️ One step of a decision (schema `Action`). */
export type HostWardenActionV1=Readonly<{action:"suspend"|"resume"|"cancel";id:string;reason:"guard"|"danger"|"overdue"|"calm"}>;
/** ⚖️ The verdict of one tick (schema `Decision`). */
export type HostWardenDecisionV1=Readonly<{level:"calm"|"guard"|"danger";calmTicks:number;hold:boolean;guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;actions:readonly HostWardenActionV1[]}>;
/** 🔔️ One line of `alarms.jsonl` (schema `Alarm`). */
export type HostWardenAlarmV1=Readonly<{version:1;atMs:number;observed:boolean;level:"calm"|"guard"|"danger";action:"suspend"|"resume"|"cancel";reason:"guard"|"danger"|"overdue"|"calm";id:string;pid:number;kind:string;owner:string;megabytes:number;host:Readonly<{availableMegabytes:number;guardMegabytes:number;dangerMegabytes:number;swapOutMegabytesPerSecond:number|null;pressure:"normal"|"warn"|"critical";headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number}>;text:Readonly<{en:string;de:string}>}>;
/** 🗂️ The cancel record written before a cancel signal (schema `Cancel`). */
export type HostWardenCancelV1=Readonly<{version:1;pid:number;start:string;kind:string;owner:string;command:string;megabytes:number;atMs:number;reason:"guard"|"danger"|"overdue"|"calm"}>;
/** 📋️ The state file of the running warden (schema `State`), written each tick: host numbers, level, subjects by kind, the suspended set, counters, the last alarms and the overhead of the loop. */
export type HostWardenStateV1=Readonly<{version:1;atMs:number;pid:number;mode:"observe"|"enforce";code:string;host:HostWardenHostV1;level:"calm"|"guard"|"danger";hold:boolean;guardMegabytes:number;dangerMegabytes:number;resumeMegabytes:number;swapRoomMegabytes:number|null;headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number;subjects:Readonly<{count:number;megabytes:number;byKind:Readonly<Record<string,Readonly<{count:number;megabytes:number}>>>}>;suspended:readonly Readonly<{id:string;pid:number;start:string;kind:string;sinceMs:number}>[];counters:Readonly<{suspended:number;resumed:number;cancelled:number;ticks:number}>;lastAlarms:readonly HostWardenAlarmV1[];overhead:Readonly<{tickMillisecondsP50:number;tickMillisecondsP95:number;cpuBasisPoints:number;residentMegabytes:number}>}>;
/** 🪞️ The memory record a worker mirrors to the group store (schema `Mirror`): the state of its warden as written, or the absent record. */
export type HostWardenMirrorV1=HostWardenStateV1|Readonly<{version:1;atMs:number;absent:true}>;
/** 🏷️ Names of the closed definitions a value can be required to match. */
export type HostWardenDefinitionV1="Host"|"ProcessRow"|"Subject"|"Previous"|"Action"|"Decision"|"Alarm"|"Cancel"|"Lock"|"Protect"|"State"|"Mode"|"Absent"|"Mirror";

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
  if(match)rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),group:Number(match[3]),rssKilobytes:Number(match[4]),stopped:match[5]!.startsWith("T"),elapsedSeconds:parseWardenElapsedV1(match[6]!),start:match[7]!,command:match[8]??""}));
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
export type HostWardenClassifyOptionsV1=Readonly<{protect?:readonly string[];within?:number;prefixes?:readonly string[];nowMs?:number;suspendedSince?:ReadonlyMap<string,number>;workspaces?:readonly string[]}>;

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

/** 🎯️ Finds the subjects in a process table: a process is a subject when it or an ancestor matches a root, or when it matches an identity (so processes started from a shell are seen), neither it nor an ancestor contains a protected path, it matches no never pattern, the first kind that matches its command names its kind, and, when `within` is given, it descends from that pid. The megabytes of a subject are its own resident megabytes plus those of every process whose nearest subject is it. Sorted by pid. */
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
 for(const row of rows){const owner=home(row);if(owner!==undefined)kilobytes.set(owner,(kilobytes.get(owner)??0)+row.rssKilobytes);}
 return [...found.values()].sort((left,right)=>left.row.pid-right.row.pid).map(({row,kind})=>{
  const id=wardenSubjectIdV1(row.pid,row.start),since=options.suspendedSince?.get(id),seconds=row.stopped&&since!==undefined&&options.nowMs!==undefined?Math.max(0,Math.floor((options.nowMs-since)/1000)):0,top=byPid.get(rootOf(row))??row;
  return Object.freeze({id,pid:row.pid,parent:row.parent,kind:kind.name,rank:kind.rank,elapsedSeconds:row.elapsedSeconds,megabytes:Math.floor((kilobytes.get(row.pid)??0)/1024),suspended:row.stopped,suspendedSeconds:seconds,canSuspend:kind.actions.includes("suspend"),canCancel:kind.actions.includes("cancel"),start:row.start,owner:wardenOwnerLabelV1(top.command,options.prefixes),scope:kind.scope,signal:kind.signal});
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

const order=(left:HostWardenSubjectV1,right:HostWardenSubjectV1):number=>left.rank-right.rank||left.elapsedSeconds-right.elapsedSeconds||right.megabytes-left.megabytes||left.pid-right.pid;
const clamp=(value:number,low:number,high:number):number=>Math.min(high,Math.max(low,value));

/** 🎯 The trigger that decided a hold level, in the precedence of the classification of {@link decideHostWardenV1}: the available memory below the line, else the headroom (available memory plus the swap that can still grow) below its line, else the memory pressure, else the swap-out rate. It only words the alarm; the level itself is classified by the decision, and the tests pin that both agree.
 * @see ./🧬️schema/🔣️.json */
export function triggerHostWardenV1(level:"guard"|"danger",numbers:Readonly<{availableMegabytes:number;guardMegabytes:number;dangerMegabytes:number;pressure:HostWardenHostV1["pressure"];headroomMegabytes:number|null;headroomGuardMegabytes:number;headroomDangerMegabytes:number}>):"available"|"headroom"|"pressure"|"swap"{
 if(numbers.availableMegabytes<(level==="danger"?numbers.dangerMegabytes:numbers.guardMegabytes))return"available";
 if(numbers.headroomMegabytes!==null&&numbers.headroomMegabytes<(level==="danger"?numbers.headroomDangerMegabytes:numbers.headroomGuardMegabytes))return"headroom";
 return level==="danger"||numbers.pressure==="warn"?"pressure":"swap";
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

/** ⚖️ Decides one tick of the warden from the host numbers, the subjects and what the previous tick left; pure, no clock and no I/O. The law: guard, danger and resume lines are shares of the physical memory between their bounds; the level is `danger` below the danger line, at critical pressure or with a headroom (available memory plus the swap room, bounded by the free disk of the swap volume, see {@link headroomHostWardenV1}) below the headroom danger line, `guard` below the guard line, with a headroom below the headroom guard line, or (swap worked, or pressure warn) below the resume line, else `calm`; overdue suspended subjects are cancelled (not calm) or resumed (cannot cancel); in danger the cheapest cancellable subjects, suspended ones first, are cancelled until the freed megabytes cover the deficit; in guard and danger the cheapest running subjects are suspended until the suspended megabytes cover the deficit, the most expensive `keepRunning` always keeps running; after `calmTicks` calm ticks above the resume line one suspended subject, the most expensive, is resumed per tick.
 * @see ./🧬️schema/🔣️.json */
export function decideHostWardenV1(policy:HostWardenPolicyV1,host:HostWardenHostV1,subjects:readonly HostWardenSubjectV1[],previous:HostWardenPreviousV1):HostWardenDecisionV1{
 const total=host.totalMegabytes,available=host.availableMegabytes;
 const guard=clamp(Math.floor(total*policy.guardBasisPoints/10000),policy.guardMinimumMegabytes,policy.guardMaximumMegabytes),danger=clamp(Math.floor(total*policy.dangerBasisPoints/10000),policy.dangerMinimumMegabytes,policy.dangerMaximumMegabytes),resume=Math.max(guard,Math.floor(total*policy.resumeBasisPoints/10000));
 const swapWorked=host.swapOutMegabytesPerSecond!==null&&host.swapOutMegabytesPerSecond>=policy.swapActiveMegabytesPerSecond;
 const room=headroomHostWardenV1(policy,host),headroom=room.headroomMegabytes;
 const level:HostWardenDecisionV1["level"]=available<danger||host.pressure==="critical"||(headroom!==null&&headroom<room.headroomDangerMegabytes)?"danger":available<guard||(headroom!==null&&headroom<room.headroomGuardMegabytes)||((swapWorked||host.pressure==="warn")&&available<resume)?"guard":"calm";
 const deficitOf=():number=>Math.max(guard-available,headroom===null?0:room.headroomGuardMegabytes-headroom,1);
 const actions:HostWardenActionV1[]=[],acted=new Set<string>();
 let cancels=0,suspends=0;
 const cancel=(subject:HostWardenSubjectV1,reason:"danger"|"overdue"):void=>{actions.push({action:"cancel",id:subject.id,reason});acted.add(subject.id);cancels++;};
 const overdue=subjects.filter(subject=>subject.suspended&&subject.suspendedSeconds>=policy.suspendMaximumSeconds).sort(order);
 const cancelled:HostWardenSubjectV1[]=[];
 if(level!=="calm")for(const subject of overdue)if(subject.canCancel&&cancels<policy.maximumCancelsPerTick){cancel(subject,"overdue");cancelled.push(subject);}
 for(const subject of overdue)if(!subject.canCancel){actions.push({action:"resume",id:subject.id,reason:"overdue"});acted.add(subject.id);}
 if(level==="danger"){
  const deficit=deficitOf();
  let freed=cancelled.reduce((sum,subject)=>sum+subject.megabytes,0);
  const candidates=subjects.filter(subject=>subject.canCancel&&!acted.has(subject.id)).sort((left,right)=>Number(right.suspended)-Number(left.suspended)||order(left,right));
  for(const subject of candidates){
   if(freed>=deficit||cancels>=policy.maximumCancelsPerTick)break;
   cancel(subject,"danger");freed+=subject.megabytes;
  }
 }
 if(level!=="calm"){
  const deficit=deficitOf(),every=subjects.filter(subject=>!subject.suspended).sort(order),kept=new Set(policy.keepRunning>0?every.slice(-policy.keepRunning).map(subject=>subject.id):[]);
  let covered=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).reduce((sum,subject)=>sum+subject.megabytes,0);
  for(const subject of every.filter(candidate=>!acted.has(candidate.id))){
   if(covered>=deficit||suspends>=policy.maximumSuspendsPerTick)break;
   if(kept.has(subject.id)||!subject.canSuspend)continue;
   actions.push({action:"suspend",id:subject.id,reason:level});acted.add(subject.id);suspends++;covered+=subject.megabytes;
  }
 }
 const calmTicks=level==="calm"?(previous.level==="calm"?previous.calmTicks:0)+1:0;
 if(level==="calm"&&calmTicks>=policy.calmTicks&&available>=resume){
  const waiting=subjects.filter(subject=>subject.suspended&&!acted.has(subject.id)).sort(order).at(-1);
  if(waiting!==undefined)actions.push({action:"resume",id:waiting.id,reason:"calm"});
 }
 return Object.freeze({level,calmTicks,hold:level!=="calm",guardMegabytes:guard,dangerMegabytes:danger,resumeMegabytes:resume,...room,actions:Object.freeze(actions)});
}

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-zA-Z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);

/** 🗣️ Words one alarm in a language: the action, the process (pid, kind, owner, megabytes), the reason and the host numbers (available, guard, danger, swap, pressure). */
export function formatHostWardenAlarmV1(language:"en"|"de",alarm:Pick<HostWardenAlarmV1,"action"|"reason"|"pid"|"kind"|"owner"|"megabytes"|"host"|"observed">):string{
 const texts=MESSAGES[language],swap=alarm.host.swapOutMegabytesPerSecond;
 return fill(texts.alarm,{action:texts.actions[alarm.action],kind:alarm.kind,pid:alarm.pid,owner:alarm.owner,megabytes:alarm.megabytes,reason:alarm.reason==="guard"||alarm.reason==="danger"?texts.triggers[alarm.reason][triggerHostWardenV1(alarm.reason,alarm.host) as never]:texts.reasons[alarm.reason],available:alarm.host.availableMegabytes,guard:alarm.host.guardMegabytes,danger:alarm.host.dangerMegabytes,swap:swap===null?texts.swapUnknown:fill(texts.swapRate,{rate:swap}),pressure:texts.pressures[alarm.host.pressure],headroom:alarm.host.headroomMegabytes===null?"":fill(texts.headroom,{headroom:alarm.host.headroomMegabytes,headroomGuard:alarm.host.headroomGuardMegabytes,headroomDanger:alarm.host.headroomDangerMegabytes}),observed:alarm.observed?texts.observed:""});
}

/** 🔔️ Builds the alarm of one action with its English and German text. */
export function buildHostWardenAlarmV1(input:Readonly<{atMs:number;observed:boolean;level:HostWardenDecisionV1["level"];action:HostWardenActionV1;subject:Pick<HostWardenSubjectV1,"pid"|"kind"|"megabytes"|"owner">;host:HostWardenHostV1;decision:Pick<HostWardenDecisionV1,"guardMegabytes"|"dangerMegabytes"|"headroomMegabytes"|"headroomGuardMegabytes"|"headroomDangerMegabytes">}>):HostWardenAlarmV1{
 const base={action:input.action.action,reason:input.action.reason,pid:input.subject.pid,kind:input.subject.kind,owner:input.subject.owner??"",megabytes:input.subject.megabytes,observed:input.observed,host:{availableMegabytes:input.host.availableMegabytes,guardMegabytes:input.decision.guardMegabytes,dangerMegabytes:input.decision.dangerMegabytes,swapOutMegabytesPerSecond:input.host.swapOutMegabytesPerSecond,pressure:input.host.pressure,headroomMegabytes:input.decision.headroomMegabytes,headroomGuardMegabytes:input.decision.headroomGuardMegabytes,headroomDangerMegabytes:input.decision.headroomDangerMegabytes}} as const;
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
