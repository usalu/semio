/** 🔓️ Contract: Cargo's fine-grain unit locks deadlock two processes of one build directory whose freshness views cross, a custody per directory, a private directory per process (a lane) and a custody per unit with an exact dirty estimate never do, and only the last two let different crates compile at the same time; an estimate that misses a dirty unit brings the deadlock back. The model is written twice (this reference and the Python oracle in `./🐍️.py`), both agree with the language-agnostic corpus `./🔣️.json`, and its `free` verdict is the one the real pinned Cargo gives in `../🔐️custody/🟦️.ts`. The production decision that follows: every compiler runs in its own lane over the content-addressed unit cache, a shared directory keeps its whole-directory custody, and a per-unit custody is admissible only with an estimate that never misses a dirty unit. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

type Process={graph:string[];dirty:string[];mightDirty?:string[]};
type Case={id:string;regime:"free"|"directory"|"lanes"|"units";processes:Process[]};
type Step={kind:"custody"|"lock"|"unlock"|"enter"|"leave"|"release";key?:string;mode?:"s"|"x"};

const program=(regime:Case["regime"],processes:Process[],index:number):Step[]=>{
 const mine=processes[index]!,dirty=new Set(mine.dirty),might=new Set(mine.mightDirty??mine.dirty),name=(unit:string)=>regime==="lanes"?`${index}:${unit}`:`0:${unit}`,steps:Step[]=[];
 if(regime==="directory")steps.push({kind:"custody",key:"dir",mode:"x"});
 if(regime==="units")for(const unit of [...mine.graph].sort())steps.push({kind:"custody",key:"unit:"+unit,mode:might.has(unit)?"x":"s"});
 steps.push({kind:"enter"});
 for(const unit of mine.graph){steps.push({kind:"lock",key:name(unit),mode:"s"});if(dirty.has(unit))steps.push({kind:"unlock",key:name(unit),mode:"s"});}
 for(const unit of mine.graph)if(dirty.has(unit)){steps.push({kind:"lock",key:name(unit),mode:"x"});steps.push({kind:"unlock",key:name(unit),mode:"x"});}
 for(const unit of mine.graph)if(!dirty.has(unit))steps.push({kind:"unlock",key:name(unit),mode:"s"});
 steps.push({kind:"leave"},{kind:"release"});
 return steps;
};

const explore=(item:Case):{deadlock:boolean;overlap:boolean}=>{
 const programs=item.processes.map((_,index)=>program(item.regime,item.processes,index)),count=programs.length;
 type State={at:number[];held:Set<string>[]};
 const encode=(state:State)=>JSON.stringify([state.at,state.held.map(set=>[...set].sort())]);
 const conflicts=(state:State,index:number,step:Step):boolean=>{
  const space=step.kind==="custody"?"c":"f";
  for(let other=0;other<count;other++)if(other!==index)for(const row of state.held[other]!){
   const [rowSpace,mode,key]=[row[0]!,row[1]!,row.slice(2)];
   if(rowSpace===space&&key===step.key&&(mode==="x"||step.mode==="x"))return true;
  }
  return false;
 };
 const start:State={at:programs.map(()=>0),held:programs.map(()=>new Set<string>())},seen=new Set([encode(start)]),stack=[start];
 let deadlock=false,overlap=false;
 while(stack.length){
  const state=stack.pop()!;
  const inside=programs.filter((steps,index)=>state.at[index]!>steps.findIndex(step=>step.kind==="enter")&&state.at[index]!<=steps.findIndex(step=>step.kind==="leave")).length;
  if(inside>1)overlap=true;
  let moved=false;
  for(let index=0;index<count;index++){
   const step=programs[index]![state.at[index]!];
   if(!step)continue;
   if((step.kind==="custody"||step.kind==="lock")&&conflicts(state,index,step))continue;
   moved=true;
   const next:State={at:[...state.at],held:state.held.map(set=>new Set(set))};
   next.at[index]!++;
   const space=step.kind==="custody"?"c":"f",token=`${space}${step.mode}${step.key}`;
   if(step.kind==="custody"||step.kind==="lock")next.held[index]!.add(token);
   else if(step.kind==="unlock")next.held[index]!.delete(token);
   else if(step.kind==="release")for(const row of [...next.held[index]!])if(row[0]==="c")next.held[index]!.delete(row);
   const id=encode(next);
   if(!seen.has(id)){seen.add(id);stack.push(next);}
  }
  if(!moved&&programs.some((steps,index)=>state.at[index]!<steps.length))deadlock=true;
 }
 return {deadlock,overlap};
};

const python=requireTestToolV1("python"),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
});

test("the reference model and the Python oracle reach the verdict of every corpus case",()=>{
 const scenarios=corpus.cases.map(item=>({id:item.id,regime:item.regime,processes:item.processes}));
 const run=spawnSync(python,["-I",oracle,"/dev/stdin"],{input:JSON.stringify(scenarios),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout) as {id:string;deadlock:boolean;overlap:boolean}[];
 for(const [index,item] of corpus.cases.entries()){
  const reference=explore(item as Case);
  expect({deadlock:reference.deadlock,overlap:reference.overlap},item.id+" reference").toEqual({deadlock:item.deadlock,overlap:item.overlap});
  expect({deadlock:verdicts[index]!.deadlock,overlap:verdicts[index]!.overlap},item.id+" oracle").toEqual({deadlock:item.deadlock,overlap:item.overlap});
 }
});

test("the exhaustive sweep over every pair of process shapes on three units reproduces the counts of the corpus",()=>{
 const run=spawnSync(python,["-I",oracle,"--sweep"],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const counts=JSON.parse(run.stdout) as typeof corpus.sweep;
 expect(counts).toEqual(corpus.sweep);
 expect(counts.free.deadlocks).toBeGreaterThan(0);
 expect(counts["units-under"].deadlocks).toBe(counts.free.deadlocks);
 for(const regime of ["directory","lanes","units"] as const)expect(counts[regime].deadlocks,regime).toBe(0);
 expect(counts.directory.overlaps).toBe(0);
 expect(counts.lanes.overlaps).toBe(counts.lanes.pairs);
 expect(counts.units.overlaps).toBeGreaterThan(counts.directory.overlaps);
 expect(counts.units.overlaps).toBeLessThan(counts.lanes.overlaps);
});
