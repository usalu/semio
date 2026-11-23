import{appendFileSync,closeSync,mkdirSync,openSync,readdirSync,readFileSync,readSync,statSync}from"node:fs";
import{join}from"node:path";
import{AsyncLocalStorage}from"node:async_hooks";
import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{machinePathV1}from"../../../🏗️build/🏠️machine-state/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧮️ Directory, thresholds and bounds of the phase ledger.
 * @see ./🧬️schema/🔣️.json */
export const PHASE_LEDGER_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});

/** 🧮️ One phase: `seconds` of exclusive time (or, when `within`, seconds summed over parallel workers), `count` occurrences, `bytes` when known. */
export type PhaseRowV1=Readonly<{phase:string;seconds:number;count:number;bytes:number|null;within:boolean}>;

/** 🧮️ A ledger: the exclusive rows plus `otherSeconds`, the time no phase claimed, add up to `totalSeconds`. */
export type PhaseTableV1=Readonly<{task:string;totalSeconds:number;otherSeconds:number;rows:readonly PhaseRowV1[]}>;

/** 🧮️ The recorder of one task process. */
export type PhaseLedgerV1=Readonly<{
 begin:(phase:string,count?:number)=>()=>void;
 run:<T>(phase:string,body:()=>Promise<T>,count?:number)=>Promise<T>;
 bytes:(phase:string,bytes:number)=>void;
 add:(row:PhaseRowV1)=>void;
 table:()=>PhaseTableV1;
 finish:()=>PhaseTableV1;
}>;

type Sink=(table:PhaseTableV1,final:boolean)=>void;

/** 🧮️ Records exclusive phases on a monotonic clock: a nested phase pauses its parent, so the rows never overlap and `otherSeconds` is exactly the time outside every phase. The sink sees the cumulative table at every moment no phase is open and at `finish`. */
export function createPhaseLedgerV1(options:Readonly<{task:string;now?:()=>number;sink?:Sink}>):PhaseLedgerV1{
 const now=options.now??(()=>performance.now()),first=now(),self=new Map<string,{ms:number;count:number;bytes:number|null}>(),within:PhaseRowV1[]=[],open:{phase:string}[]=[];
 let last=first,otherMs=0,closed=false;
 const entry=(phase:string)=>{let found=self.get(phase);if(!found){found={ms:0,count:0,bytes:null};self.set(phase,found);}return found;};
 const mark=()=>{const at=now(),delta=Math.max(0,at-last);last=at;const top=open.at(-1);if(top)entry(top.phase).ms+=delta;else otherMs+=delta;};
 const table=():PhaseTableV1=>Object.freeze({task:options.task,totalSeconds:(last-first)/1000,otherSeconds:otherMs/1000,rows:Object.freeze([...[...self].map(([phase,value])=>Object.freeze({phase,seconds:value.ms/1000,count:value.count,bytes:value.bytes,within:false})),...within])});
 const emit=(final:boolean)=>{try{options.sink?.(table(),final);}catch{}};
 const begin=(phase:string,count=1)=>{
  if(closed)return()=>undefined;
  mark();const frame={phase};open.push(frame);let done=false;
  return()=>{if(done||closed)return;done=true;mark();const at=open.indexOf(frame);if(at>=0)open.splice(at,1);entry(phase).count+=count;if(open.length===0)emit(false);};
 };
 return Object.freeze({
  begin,
  run:async<T>(phase:string,body:()=>Promise<T>,count=1)=>{const end=begin(phase,count);try{return await body();}finally{end();}},
  bytes:(phase:string,bytes:number)=>{const value=entry(phase);value.bytes=(value.bytes??0)+Math.max(0,Math.trunc(bytes));},
  add:row=>{within.push(Object.freeze({...row,within:true}));},
  table:()=>{mark();return table();},
  finish:()=>{mark();while(open.length)open.pop();closed=true;const result=table();try{options.sink?.(result,true);}catch{}return result;},
 });
}

const nothing:PhaseLedgerV1=Object.freeze({begin:()=>()=>undefined,run:(_phase,body)=>body(),bytes:()=>undefined,add:()=>undefined,table:()=>Object.freeze({task:"-",totalSeconds:0,otherSeconds:0,rows:Object.freeze([])}),finish:()=>Object.freeze({task:"-",totalSeconds:0,otherSeconds:0,rows:Object.freeze([])})});
const current=new AsyncLocalStorage<PhaseLedgerV1>();

/** 🧮️ Runs `body` with `ledger` as the ledger of everything it awaits. */
export const withPhaseLedgerV1=<T>(ledger:PhaseLedgerV1,body:()=>Promise<T>):Promise<T>=>current.run(ledger,body);

/** 🧮️ The ledger of the running task, or one that records nothing. */
export const phaseLedgerV1=():PhaseLedgerV1=>current.getStore()??nothing;

/** 🧮️ The task name Nx gave this process (`project:target[:configuration]`), else `fallback`. */
export function phaseLedgerTaskV1(environment:Readonly<Record<string,string|undefined>>,fallback:string):string{
 const project=environment.NX_TASK_TARGET_PROJECT,target=environment.NX_TASK_TARGET_TARGET,configuration=environment.NX_TASK_TARGET_CONFIGURATION;
 return project&&target?project+":"+target+(configuration?":"+configuration:""):fallback;
}

const fixed=(value:number)=>value.toFixed(3);

/** 🧮️ The ledger as machine lines, one per phase: `[phase] task=… name=… seconds=… count=… [bytes=…]`, a head line and a closing line with the residual; rows `within` follow the sum. */
export function phaseTableLinesV1(table:PhaseTableV1):readonly string[]{
 const row=(value:PhaseRowV1)=>`[phase] task=${table.task} name=${value.phase} seconds=${fixed(value.seconds)} count=${value.count}${value.bytes===null?"":" bytes="+value.bytes}${value.within?" within=cargo":""}`;
 const exclusive=table.rows.filter(value=>!value.within),inner=table.rows.filter(value=>value.within),sum=exclusive.reduce((total,value)=>total+value.seconds,0)+table.otherSeconds;
 return[`[phase] task=${table.task} total=${fixed(table.totalSeconds)} phases=${fixed(sum-table.otherSeconds)} other=${fixed(table.otherSeconds)}`,...exclusive.map(row),...inner.map(row)];
}

/** 🧮️ The law of the sum: exclusive phases plus `other` equal `total` within the floating-point error of the sums. */
export function phaseTableBalancedV1(table:PhaseTableV1):boolean{
 const sum=table.rows.filter(value=>!value.within).reduce((total,value)=>total+value.seconds,0)+table.otherSeconds;
 return Math.abs(sum-table.totalSeconds)<=1e-6*Math.max(1,table.totalSeconds);
}

/** 🧮️ Whether the unclaimed time is small enough for the ledger to explain the task: below the absolute floor or the fraction of the total. */
export function phaseTableCoveredV1(table:PhaseTableV1):boolean{
 return table.otherSeconds<=Math.max(PHASE_LEDGER_POLICY_V1.residualToleranceSeconds,PHASE_LEDGER_POLICY_V1.residualToleranceFraction*table.totalSeconds);
}

type UnitDataV1=Readonly<{i:number;name:string;mode:string;start:number;duration:number;unblocked_units:readonly number[];unblocked_rmeta_units:readonly number[];sections?:readonly(readonly[string,Readonly<{start:number;end:number}>])[]}>;

/** 🧮️ What Cargo's own `--timings` report says about one run (see `cargoTimingsSummaryV1`). */
export type CargoTimingsV1=Readonly<{units:number;wallSeconds:number;startSeconds:number;frontendSeconds:number;codegenSeconds:number;unsplitSeconds:number;criticalUnits:number;criticalSeconds:number;longest:readonly Readonly<{name:string;seconds:number}>[];slow:readonly Readonly<{name:string;seconds:number}>[]}>;

/** 🧮️ Reads the unit table out of Cargo's `--timings` HTML (the JSON array `UNIT_DATA`, offline) and derives the wall time, the delay before the first unit starts (resolution and fingerprint checks), the sum of front end, code generation and unsectioned seconds (build scripts, links), the serial critical path through the dependency edges (a metadata edge is ready when the front end ends) and the units above `slowUnitFraction` of the wall. Returns `undefined` for a report it cannot read. */
export function cargoTimingsSummaryV1(html:string):CargoTimingsV1|undefined{
 const at=html.indexOf("const UNIT_DATA = ");if(at<0)return undefined;
 const from=html.indexOf("[",at),to=html.indexOf("\n];",from);if(from<0||to<0)return undefined;
 let data:UnitDataV1[];try{data=JSON.parse(html.slice(from,to+2));}catch{return undefined;}
 if(!Array.isArray(data)||data.length===0)return undefined;
 const end=(unit:UnitDataV1)=>unit.start+unit.duration,section=(unit:UnitDataV1,name:string)=>(unit.sections??[]).filter(([label])=>label===name).reduce((total,[,span])=>total+span.end-span.start,0);
 const rmetaReady=(unit:UnitDataV1)=>unit.start+((unit.sections??[]).find(([label])=>label==="frontend")?.[1].end??unit.duration);
 const wall=Math.max(...data.map(end)),start=Math.min(...data.map(unit=>unit.start)),byIndex=new Map(data.map(unit=>[unit.i,unit]));
 let current:UnitDataV1|undefined=data.reduce((best,unit)=>end(unit)>end(best)?unit:best);const chain:UnitDataV1[]=[];
 while(current&&!chain.includes(current)){
  chain.push(current);const target:UnitDataV1=current;
  const ready=data.flatMap(unit=>unit.unblocked_units.includes(target.i)?[[unit,end(unit)] as const]:unit.unblocked_rmeta_units.includes(target.i)?[[unit,rmetaReady(unit)] as const]:[]);
  current=ready.length?ready.reduce((best,row)=>row[1]>best[1]?row:best)[0]:undefined;
 }
 const named=(unit:UnitDataV1)=>Object.freeze({name:unit.name+(unit.mode==="todo"?"":":"+unit.mode),seconds:unit.duration}),sorted=[...data].sort((a,b)=>b.duration-a.duration);
 return Object.freeze({units:data.length,wallSeconds:wall,startSeconds:start,frontendSeconds:data.reduce((total,unit)=>total+section(unit,"frontend"),0),codegenSeconds:data.reduce((total,unit)=>total+section(unit,"codegen"),0),unsplitSeconds:data.filter(unit=>!(unit.sections??[]).length).reduce((total,unit)=>total+unit.duration,0),criticalUnits:chain.length,criticalSeconds:chain.reduce((total,unit)=>total+unit.duration,0),longest:Object.freeze(sorted.slice(0,PHASE_LEDGER_POLICY_V1.longestUnits).map(named)),slow:Object.freeze(sorted.filter(unit=>unit.duration>=PHASE_LEDGER_POLICY_V1.slowUnitFraction*wall).map(named))});
}

/** 🧮️ The `[cargo-timings]` machine lines of a summary; one line per slow unit, none when no unit is slow. */
export function cargoTimingsLinesV1(task:string,summary:CargoTimingsV1):readonly string[]{
 return[`[cargo-timings] task=${task} units=${summary.units} wall=${fixed(summary.wallSeconds)} start=${fixed(summary.startSeconds)} frontend=${fixed(summary.frontendSeconds)} codegen=${fixed(summary.codegenSeconds)} unsplit=${fixed(summary.unsplitSeconds)} critical-units=${summary.criticalUnits} critical=${fixed(summary.criticalSeconds)} longest=${summary.longest.map(unit=>unit.name+"@"+fixed(unit.seconds)).join(",")}`,...summary.slow.map(unit=>`[cargo-timings] task=${task} slow=${unit.name} seconds=${fixed(unit.seconds)}`)];
}

/** 🧮️ The newest timing report below a target directory's `cargo-timings` folder with the HTML text, when one was written at or after `sinceMs`. */
export function readCargoTimingsV1(targetDirectory:string,sinceMs:number):CargoTimingsV1|undefined{
 try{
  const directory=join(targetDirectory,"cargo-timings"),files=readdirSync(directory).filter(name=>/^cargo-timing-.*\.html$/u.test(name)).map(name=>({name,at:statSync(join(directory,name)).mtimeMs})).filter(file=>file.at>=sinceMs).sort((a,b)=>b.at-a.at);
  return files.length?cargoTimingsSummaryV1(readFileSync(join(directory,files[0]!.name),"utf8")):undefined;
 }catch{return undefined;}
}

type UnitEventRowV1=Readonly<{atMs:number;lane:string;kind:string;milliseconds:number;waitedMs:number}>;

/** 🧮️ Rows `within` the Cargo phase from the unit event log of one lane between two wall-clock times: restores from this machine (`hit`) and from the fleet (`remote-hit`, the whole obtain including the transfer), fleet misses answered (`remote-miss`), compiles (`miss`), publications (`publish`) and waits (slot, lock, remote claim). Seconds are summed over the workers, so they can exceed the Cargo wall. */
export function unitEventRowsV1(events:readonly UnitEventRowV1[],lane:string,fromMs:number,toMs:number):readonly PhaseRowV1[]{
 const groups:Readonly<Record<string,{phase:string;wait?:boolean}>>={"hit":{phase:"unit:restore-local"},"remote-hit":{phase:"unit:restore-fleet"},"remote-miss":{phase:"unit:lookup-fleet-miss"},"miss":{phase:"unit:compile"},"publish":{phase:"unit:publish"},"slot-wait":{phase:"unit:wait",wait:true},"lock-wait":{phase:"unit:wait",wait:true},"remote-wait":{phase:"unit:wait",wait:true}};
 const sums=new Map<string,{ms:number;count:number}>();
 for(const event of events){
  if(event.lane!==lane||event.atMs<fromMs||event.atMs>toMs)continue;
  const group=groups[event.kind];if(!group)continue;
  const sum=sums.get(group.phase)??{ms:0,count:0};sum.ms+=group.wait?event.waitedMs:event.milliseconds;sum.count++;sums.set(group.phase,sum);
 }
 return[...sums].map(([phase,sum])=>Object.freeze({phase,seconds:sum.ms/1000,count:sum.count,bytes:null,within:true}));
}

/** 🧮️ Reads the event rows of every file of a unit cache root's event directory, skipping lines it cannot parse. */
export function readUnitEventsV1(unitCacheRoot:string):readonly UnitEventRowV1[]{
 const rows:UnitEventRowV1[]=[];
 try{
  const directory=join(unitCacheRoot,PHASE_LEDGER_POLICY_V1.eventsDirectory);
  for(const name of readdirSync(directory))if(name.endsWith(".jsonl"))for(const line of readFileSync(join(directory,name),"utf8").split("\n")){
   if(!line)continue;try{const row=JSON.parse(line);if(typeof row.atMs==="number"&&typeof row.kind==="string")rows.push({atMs:row.atMs,lane:String(row.lane),kind:row.kind,milliseconds:Number(row.milliseconds)||0,waitedMs:Number(row.waitedMs)||0});}catch{}
  }
 }catch{}
 return rows;
}

/** ♻️ Counts the units one lane served from a cache since the tally was created, read incrementally from the end of the lane's event log: restores from this machine (`hit`, `script-hit`) and from the fleet (`remote-hit`). A partial last line waits for its newline; a log that shrank restarts from its beginning. */
export function createUnitReuseTallyV1(unitCacheRoot:string,lane:string):Readonly<{read:()=>Readonly<{local:number;fleet:number}>}>{
 const file=join(unitCacheRoot,PHASE_LEDGER_POLICY_V1.eventsDirectory,lane+".jsonl");
 let offset=0,pending="",local=0,fleet=0;
 try{offset=statSync(file).size;}catch{}
 return Object.freeze({read:()=>{
  try{
   const size=statSync(file).size;
   if(size<offset){offset=0;pending="";}
   if(size>offset){
    const descriptor=openSync(file,"r");
    try{const buffer=Buffer.allocUnsafe(Math.min(size-offset,1<<24)),bytes=readSync(descriptor,buffer,0,buffer.length,offset);offset+=bytes;pending+=buffer.toString("utf8",0,bytes);}finally{closeSync(descriptor);}
    const lines=pending.split("\n");pending=lines.pop()!;
    for(const line of lines){if(!line)continue;try{const kind=JSON.parse(line).kind;if(kind==="hit"||kind==="script-hit")local++;else if(kind==="remote-hit")fleet++;}catch{}}
   }
  }catch{}
  return{local,fleet};
 }});
}

const ledgerDirectory=(workspace:string)=>machinePathV1(workspace,".cache","f",...PHASE_LEDGER_POLICY_V1.directory);

/** 🧮️ A sink that appends the table as one JSON line to `<machine>/.cache/f/phase-ledger/<pid>.jsonl`, so a killed task still leaves its last cumulative table; the front of the run prints the lines (a successful Nx task prints nothing). */
export function phaseLedgerFileSinkV1(workspace:string,pid=process.pid):Sink{
 return(table,final)=>{
  const line=JSON.stringify({version:1,task:table.task,pid,atMs:Date.now(),final,totalSeconds:table.totalSeconds,otherSeconds:table.otherSeconds,rows:table.rows})+"\n";
  if(line.length>PHASE_LEDGER_POLICY_V1.maximumRecordBytes)return;
  const directory=ledgerDirectory(workspace);try{appendFileSync(join(directory,pid+".jsonl"),line);}catch{mkdirSync(directory,{recursive:true});appendFileSync(join(directory,pid+".jsonl"),line);}
 };
}

/** 🧮️ The newest table of every ledger file written at or after `sinceMs`, validated against the schema; unreadable files and lines are skipped. */
export function readPhaseLedgersV1(workspace:string,sinceMs:number):readonly PhaseTableV1[]{
 const found:PhaseTableV1[]=[];
 try{
  const directory=ledgerDirectory(workspace);
  for(const name of readdirSync(directory)){
   if(!name.endsWith(".jsonl"))continue;
   try{
    if(statSync(join(directory,name)).mtimeMs<sinceMs)continue;
    let newest:PhaseTableV1|undefined;
    for(const line of readFileSync(join(directory,name),"utf8").split("\n")){
     if(!line)continue;
     try{const row=JSON.parse(line);if(row.atMs>=sinceMs&&validateJsonSchemaSubset(schema,row,schema).length===0)newest={task:row.task,totalSeconds:row.totalSeconds,otherSeconds:row.otherSeconds,rows:row.rows};}catch{}
    }
    if(newest)found.push(newest);
   }catch{}
  }
 }catch{}
 return found;
}

/** 🧮️ The lines the front of an Nx run prints after the run: the ledger of every task that wrote one since the run began, and the part of the run wall outside the longest task. */
export function phaseLedgerRunLinesV1(workspace:string,sinceMs:number,runSeconds:number):readonly string[]{
 const tables=readPhaseLedgersV1(workspace,sinceMs);if(!tables.length)return[];
 const longest=Math.max(...tables.map(table=>table.totalSeconds));
 return[...tables.flatMap(table=>phaseTableLinesV1(table)),`[phase] run total=${fixed(runSeconds)} longest-task=${fixed(longest)} outside-task=${fixed(Math.max(0,runSeconds-longest))}`];
}
