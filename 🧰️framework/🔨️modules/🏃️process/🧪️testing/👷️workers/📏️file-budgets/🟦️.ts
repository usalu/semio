import{readdir}from"node:fs/promises";
import{platform as osPlatform}from"node:os";
import{relative,sep}from"node:path";
import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{resolveHostCapacityLanguageV1}from"../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import{parseProcessTableV1,readHostTextV1,readLinuxProcessTableV1,runHostCommandV1,treeMegabytesV1,WINDOWS_PROCESS_TABLE_COMMAND}from"../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import authored from"./🔣️.json"with{type:"json"};

const protocol=schema.definitions.Protocol.const,messages=schema.definitions.Messages.const;
const fill=(text:string,values:Readonly<Record<string,string|number>>)=>text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);

/** 📏️ One test file that holds more than a typical file process: its workspace-relative path with forward slashes, the declared peak megabytes of its process tree and why it holds them. */
export type TestFileBudgetEntryV1=Readonly<{path:string;megabytes:number;reason:string}>;
/** 🗒️ The authored budgets: the share of a declared budget a file may exceed before it is stopped, the ceiling of a file nobody declared and the declared files in path order. */
export type TestFileBudgetsV1=Readonly<{version:1;ceilingBasisPoints:number;unlistedCeilingMegabytes:number;files:readonly TestFileBudgetEntryV1[]}>;
/** 🎚️ What one file may hold: the megabytes it is planned with, the ceiling above which it is stopped and whether its budget is declared. */
export type TestFileLimitsV1=Readonly<{budgetMegabytes:number;ceilingMegabytes:number;declared:boolean}>;
/** 🌳️ The rows of a process table: pid, parent and resident kilobytes. */
export type TestFileProcessRowsV1=readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[];
/** 🎟️ A place of the gate held by one running file; `raise` tells the gate the file holds more than it was admitted with. */
export type TestFileSlotV1=Readonly<{raise:(megabytes:number)=>void;release:()=>void}>;
/** 🚪️ Admits files one after another in the order they ask, while the megabytes held and the budget of the next fit the share. */
export type TestFileGateV1=Readonly<{enter:(megabytes:number)=>Promise<TestFileSlotV1>}>;
/** 👁️ Follows the process trees of running files with ONE process table read per tick; `watch` answers the function that stops following and reports the peak. */
export type TestFileWatchV1=Readonly<{watch:(pid:number,ceilingMegabytes:number,exceeded:(megabytes:number)=>void,progress?:(megabytes:number)=>void)=>()=>Readonly<{peakMegabytes:number;exceeded:boolean}>}>;
/** 🧠️ What a runner of test files asks of its children: the limits of a file, the gate and the watch; `SEMIO_TEST_FILE_CEILING=off` lifts every ceiling (the gate and the planner stay) for a run that must see a file finish whatever it holds. */
export type TestFileMemoryV1=Readonly<{shareMegabytes:number;limits:(file:string)=>TestFileLimitsV1;gate:TestFileGateV1;watch:TestFileWatchV1;environment:Readonly<Record<string,string|undefined>>}>;

/** 📖️ Reads the budgets, refusing a document that breaks the schema, repeats a path, is not in path order or names a path that is not workspace-relative with forward slashes. */
export function readTestFileBudgetsV1(value:unknown=authored,environment:Readonly<Record<string,string|undefined>>=process.env):TestFileBudgetsV1{
 const refuse=(detail:string)=>Error(fill(messages[resolveHostCapacityLanguageV1(environment)].errors.table,{detail})),findings=validateJsonSchemaSubset(schema.definitions.Table,value,schema);
 if(findings.length)throw refuse(String(findings[0]));
 const table=value as TestFileBudgetsV1,paths=table.files.map(row=>row.path);
 if(new Set(paths).size!==paths.length)throw refuse("path repeated");
 if(paths.some((path,index)=>index>0&&paths[index-1]!>=path))throw refuse("paths are not in order");
 const bad=paths.find(path=>path.startsWith("/")||path.includes("\\")||path.split("/").some(part=>part===""||part==="."||part===".."));
 if(bad!==undefined)throw refuse("not workspace-relative: "+bad);
 return table;
}

/** 🔖️ The limits of one file: a declared file is planned with its budget and stopped above the budget times the ceiling share; a file nobody declared is planned with the default of its runner and stopped above the unlisted ceiling (never below the default times the share). */
export function testFileLimitsV1(budgets:TestFileBudgetsV1,path:string,defaultMegabytes:number):TestFileLimitsV1{
 const row=budgets.files.find(entry=>entry.path===path),share=(megabytes:number)=>Math.ceil(megabytes*budgets.ceilingBasisPoints/10000);
 return row===undefined?Object.freeze({budgetMegabytes:defaultMegabytes,ceilingMegabytes:Math.max(budgets.unlistedCeilingMegabytes,share(defaultMegabytes)),declared:false}):Object.freeze({budgetMegabytes:row.megabytes,ceilingMegabytes:share(row.megabytes),declared:true});
}

/** 📈️ The size of the biggest file of a cohort against the default size of one file process, in basis points (10000 = the default); the planner prices a worker with it so a cohort that holds a big file gets fewer workers. */
export function testFileRatioBasisPointsV1(budgets:TestFileBudgetsV1,paths:readonly string[],defaultMegabytes:number):number{
 return Math.ceil(Math.max(...paths.map(path=>testFileLimitsV1(budgets,path,defaultMegabytes).budgetMegabytes),defaultMegabytes)*10000/defaultMegabytes);
}

/** 🧮️ Whether the next file may start: nothing runs, or the megabytes held plus the budget of the next fit the share. */
export function admitTestFileV1(held:readonly number[],next:number,share:number):boolean{
 return held.length===0||held.reduce((total,megabytes)=>total+megabytes,0)+next<=share;
}

/** 🛂️ A gate over the megabytes of one share; files enter in the order they ask and a file that was admitted with its budget but holds more raises what the gate counts for it. */
export function createTestFileGateV1(shareMegabytes:number):TestFileGateV1{
 const held=new Map<symbol,number>(),waiting:{megabytes:number;start:(slot:TestFileSlotV1)=>void}[]=[];
 const slot=(megabytes:number):TestFileSlotV1=>{
  const key=Symbol("file");held.set(key,megabytes);
  return Object.freeze({raise:(more:number)=>{if(held.has(key))held.set(key,Math.max(held.get(key)!,more));},release:()=>{if(held.delete(key))pump();}});
 };
 const pump=()=>{for(;waiting.length&&admitTestFileV1([...held.values()],waiting[0]!.megabytes,shareMegabytes);){const next=waiting.shift()!;next.start(slot(next.megabytes));}};
 return Object.freeze({enter:(megabytes:number)=>new Promise<TestFileSlotV1>(start=>{waiting.push({megabytes,start});pump();})});
}

/** 🗂️ Reads the process table of the host: `/proc` on Linux, `ps` on macOS, one PowerShell table on Windows. */
export async function readTestFileProcessRowsV1(platform:string=osPlatform(),run=runHostCommandV1,proc:Readonly<{read:typeof readHostTextV1;list:()=>Promise<readonly string[]>}>={read:readHostTextV1,list:()=>readdir("/proc")}):Promise<TestFileProcessRowsV1>{
 if(platform==="win32")return parseProcessTableV1(await run("powershell.exe",["-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_TABLE_COMMAND]));
 if(platform==="linux")return readLinuxProcessTableV1(proc.read,proc.list);
 return parseProcessTableV1(await run("ps",["-A","-o","pid=,ppid=,rss="]));
}

/** 🔭️ Follows many process trees with one timer and one process table read per tick; a tick that cannot read the table is skipped (the file then runs unwatched, as the gate learns nothing on a host that cannot observe a tree), the timer runs only while something is watched. */
export function createTestFileWatchV1(options:Readonly<{intervalMilliseconds?:number;rows?:()=>Promise<TestFileProcessRowsV1>}>={}):TestFileWatchV1{
 const interval=options.intervalMilliseconds??protocol.observeMilliseconds,rows=options.rows??readTestFileProcessRowsV1,entries=new Map<number,{ceiling:number;peak:number;exceeded:boolean;hit:(megabytes:number)=>void;progress?:(megabytes:number)=>void}>();
 let timer:ReturnType<typeof setInterval>|undefined,reading=false;
 const tick=async()=>{
  if(reading||!entries.size)return;reading=true;
  try{
   const table=await rows();
   for(const[pid,entry]of[...entries]){const megabytes=treeMegabytesV1(table,pid);if(megabytes===null)continue;entry.peak=Math.max(entry.peak,megabytes);entry.progress?.(megabytes);if(megabytes>entry.ceiling&&!entry.exceeded){entry.exceeded=true;entry.hit(megabytes);}}
  }catch{}finally{reading=false;}
 };
 return Object.freeze({watch:(pid:number,ceiling:number,exceeded:(megabytes:number)=>void,progress?:(megabytes:number)=>void)=>{
  const entry={ceiling,peak:0,exceeded:false,hit:exceeded,...(progress?{progress}:{})};entries.set(pid,entry);
  if(timer===undefined){timer=setInterval(()=>{void tick();},interval);timer.unref();}
  void tick();
  return()=>{entries.delete(pid);if(!entries.size&&timer!==undefined){clearInterval(timer);timer=undefined;}return Object.freeze({peakMegabytes:entry.peak,exceeded:entry.exceeded});};
 }});
}

/** 🏗️ Builds what a runner of test files needs for one cohort: the limits of a file by its absolute path (relative to the checkout), a gate over the share it was admitted and a watch. */
export function createTestFileMemoryV1(input:Readonly<{budgets:TestFileBudgetsV1;checkout:string;defaultMegabytes:number;shareMegabytes:number;environment?:Readonly<Record<string,string|undefined>>;rows?:()=>Promise<TestFileProcessRowsV1>;intervalMilliseconds?:number}>):TestFileMemoryV1{
 return Object.freeze({shareMegabytes:input.shareMegabytes,environment:input.environment??process.env,
  limits:(file:string)=>{const limits=testFileLimitsV1(input.budgets,relative(input.checkout,file).split(sep).join("/"),input.defaultMegabytes);return(input.environment??process.env)[protocol.ceilingSwitch]==="off"?Object.freeze({...limits,ceilingMegabytes:Number.MAX_SAFE_INTEGER}):limits;},
  gate:createTestFileGateV1(input.shareMegabytes),
  watch:createTestFileWatchV1({...(input.rows?{rows:input.rows}:{}),...(input.intervalMilliseconds?{intervalMilliseconds:input.intervalMilliseconds}:{})})});
}

/** 🗣️ Words the stop of a file in the language of the environment. */
export function describeTestFileExceededV1(file:string,peakMegabytes:number,limits:TestFileLimitsV1,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const words=messages[resolveHostCapacityLanguageV1(environment)];
 return fill(words.exceeded,{file,peak:peakMegabytes,ceiling:limits.ceilingMegabytes,budget:fill(limits.declared?words.budgets.declared:words.budgets.unlisted,{budget:limits.budgetMegabytes})});
}
