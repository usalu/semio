/** 🏊️ Contract: the lane pool never gives a lane to two live processes, never removes a lane a process holds, always lets every process finish, and lets different crate sets compile at the same time. The pool rules (`planPoolAcquireV1`, `planPoolTrimV1`, the quota) are written twice, in the production module and in the independent Python oracle `./🐍️.py`, and both agree with the language-agnostic corpus `./🔣️.json`; the exhaustive interleaving model of acquirers and a trimmer is written twice as well (this reference drives the production rules) and reproduces the counts of the corpus: the claim-guarded trimmer has no violation, the unguarded one (negative control) removes lanes under live processes.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {orderBy} from "lodash";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {planPoolAcquireV1,planPoolTrimV1,poolBoundsV1,poolQuotaBytesV1,type PoolBoundsV1,type PoolChoiceV1,type PoolLaneV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const python=requireTestToolV1("python"),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const ask=(args:string[],input?:unknown)=>{const run=spawnSync(python,["-I",oracle,...args],{input:input===undefined?undefined:JSON.stringify(input),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

type Explored={states:number;violations:number;stuck:number;unreachableDone:number;overlap:boolean};
type LaneState={id:number;holder:number;key:string;used:number};
type ProcState={pc:"read"|"claim"|"run"|"release"|"done";kind:string|null;id:number|null};
type TrimState={pc:"off"|"read"|"delete"|"done";victims:number[]};
type State={lanes:LaneState[];procs:ProcState[];trim:TrimState;tick:number;fresh:number};

const explore=(config:{maximumLanes:number;maximumLiveLanes:number;regime:"claim"|"unguarded";lanes:string[];keys:string[];trimmer:boolean}):Explored=>{
 const bounds:PoolBoundsV1={maximumLanes:config.maximumLanes,maximumLiveLanes:config.maximumLiveLanes,quotaBytes:Number.MAX_SAFE_INTEGER};
 const view=(lanes:LaneState[]):PoolLaneV1[]=>lanes.map(lane=>({id:String(lane.id),claimed:lane.holder>=0,lastUsedMs:lane.used,crateKey:lane.key,bytes:null}));
 const encode=(state:State)=>JSON.stringify(state),n=config.keys.length;
 const sorted=(lanes:LaneState[])=>[...lanes].sort((left,right)=>left.id-right.id);
 const start:State={lanes:config.lanes.map((key,id)=>({id,holder:-1,key,used:0})),procs:config.keys.map(()=>({pc:"read",kind:null,id:null})),trim:config.trimmer?{pc:"read",victims:[]}:{pc:"off",victims:[]},tick:0,fresh:config.lanes.length};
 const finished=(state:State)=>state.procs.every(proc=>proc.pc==="done")&&(state.trim.pc==="off"||state.trim.pc==="done");
 const moves=(state:State):State[]=>{
  const out:State[]=[],{lanes,procs,trim,tick,fresh}=state;
  const withProc=(index:number,proc:ProcState,patch:Partial<State>={}):State=>({lanes,procs:procs.map((row,at)=>at===index?proc:row),trim,tick,fresh,...patch});
  for(let index=0;index<n;index++){
   const proc=procs[index]!;
   if(proc.pc==="read"){const choice=planPoolAcquireV1(view(lanes),{crateKey:config.keys[index]!},bounds);if(choice.kind==="busy")continue;out.push(withProc(index,{pc:"claim",kind:choice.kind,id:choice.id===undefined?null:Number(choice.id)}));}
   else if(proc.pc==="claim"){
    if(proc.kind==="new")out.push(withProc(index,{pc:"run",kind:"new",id:fresh},{lanes:sorted([...lanes,{id:fresh,holder:index,key:config.keys[index]!,used:tick}]),fresh:fresh+1}));
    else{const lane=lanes.find(row=>row.id===proc.id);if(lane&&lane.holder<0)out.push(withProc(index,{pc:"run",kind:proc.kind,id:proc.id},{lanes:lanes.map(row=>row.id===proc.id?{...row,holder:index}:row)}));else out.push(withProc(index,{pc:"read",kind:null,id:null}));}
   }
   else if(proc.pc==="run")out.push(withProc(index,{...proc,pc:"release"}));
   else if(proc.pc==="release")out.push(withProc(index,{pc:"done",kind:null,id:null},{lanes:lanes.map(row=>row.id===proc.id?{...row,holder:-1,key:config.keys[index]!,used:tick+1}:row),tick:tick+1}));
  }
  if(trim.pc==="read")out.push({lanes,procs,trim:{pc:"delete",victims:planPoolTrimV1(view(lanes),bounds).map(Number)},tick,fresh});
  else if(trim.pc==="delete"){
   if(trim.victims.length===0)out.push({lanes,procs,trim:{pc:"done",victims:[]},tick,fresh});
   else{
    const [head,...rest]=trim.victims,lane=lanes.find(row=>row.id===head),next={pc:"delete" as const,victims:rest};
    if(!lane)out.push({lanes,procs,trim:next,tick,fresh});
    else if(config.regime==="claim")out.push({lanes:lane.holder<0?lanes.filter(row=>row.id!==head):lanes,procs,trim:next,tick,fresh});
    else out.push({lanes:lanes.filter(row=>row.id!==head),procs,trim:next,tick,fresh});
   }
  }
  return out;
 };
 const seen=new Map<string,State>([[encode(start),start]]),edges=new Map<string,string[]>(),stack=[start];
 let violations=0,stuck=0,overlap=false;
 while(stack.length){
  const state=stack.pop()!;
  if(state.procs.filter(proc=>proc.pc==="run").length>1)overlap=true;
  const held=new Map<number,number>();
  for(const proc of state.procs)if(proc.pc==="run"||proc.pc==="release")held.set(proc.id!,(held.get(proc.id!)??0)+1);
  for(const [id,count] of held){const lane=state.lanes.find(row=>row.id===id);if(!lane||count>1||lane.holder<0)violations++;}
  const next=moves(state),ids=next.map(encode);edges.set(encode(state),ids);
  if(!next.length&&!finished(state))stuck++;
  for(const [index,row] of next.entries())if(!seen.has(ids[index]!)){seen.set(ids[index]!,row);stack.push(row);}
 }
 const reverse=new Map<string,string[]>();
 for(const [from,tos] of edges)for(const to of tos)reverse.set(to,[...(reverse.get(to)??[]),from]);
 const good=new Set([...seen].filter(([,state])=>finished(state)).map(([id])=>id)),frontier=[...good];
 while(frontier.length){const id=frontier.pop()!;for(const from of reverse.get(id)??[])if(!good.has(from)){good.add(from);frontier.push(from);}}
 return {states:seen.size,violations,stuck,unreachableDone:[...seen.keys()].filter(id=>!good.has(id)).length,overlap};
};

const sweep=(regime:"claim"|"unguarded")=>{
 const totals={configs:0,states:0,violations:0,stuck:0,unreachableDone:0,overlaps:0};
 for(const keys of [[],["a"],["a","a"],["a","b"],["a","b","a"]])for(const lanes of [[],["a"],["a","b"],["b","b"]])for(const maximumLanes of [1,2])for(const trimmer of [false,true]){
  if(keys.length===0&&!trimmer)continue;
  const result=explore({maximumLanes,maximumLiveLanes:maximumLanes+1,regime,lanes,keys,trimmer});
  totals.configs++;totals.states+=result.states;totals.violations+=result.violations;totals.stuck+=result.stuck;totals.unreachableDone+=result.unreachableDone;totals.overlaps+=result.overlap?1:0;
 }
 return totals;
};

const random=(seed:number)=>{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};};
const lanesOf=(next:()=>number,count:number):PoolLaneV1[]=>Array.from({length:count},(_,index)=>({id:"l"+index,claimed:next()<0.4,lastUsedMs:Math.floor(next()*6),crateKey:["a","b","c","*",""][Math.floor(next()*5)]!,bytes:next()<0.3?null:Math.floor(next()*500)}));

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 for(const mutate of [(d:any)=>{d.extra=1;},(d:any)=>{d.version=2;},(d:any)=>{d.acquire[0].lanes=[{id:"X!",claimed:false,lastUsedMs:0,crateKey:"",bytes:null}];},(d:any)=>{d.acquire[0].expect.kind="other";},(d:any)=>{delete d.sweep.claim.states;},(d:any)=>{d.quota[0].laneShareDenominator=0;}]){
  const copy=JSON.parse(JSON.stringify(corpus));mutate(copy);expect(ajv.validate(corpusSchema,copy),JSON.stringify(copy).slice(0,80)).toBe(false);
 }
});

test("the production pool rules and the Python oracle reach the verdict of every corpus case",()=>{
 const verdicts=ask(["plan","/dev/stdin"],corpus);
 for(const [index,item] of corpus.acquire.entries()){
  expect(planPoolAcquireV1(item.lanes,{crateKey:item.crateKey},item.bounds),item.id+" production").toEqual(item.expect as PoolChoiceV1);
  expect(verdicts.acquire[index],item.id+" oracle").toEqual(item.expect);
 }
 for(const [index,item] of corpus.trim.entries()){
  expect(planPoolTrimV1(item.lanes,item.bounds),item.id+" production").toEqual(item.expect);
  expect(verdicts.trim[index],item.id+" oracle").toEqual(item.expect);
 }
 for(const [index,item] of corpus.quota.entries()){
  expect(item.laneShareNumerator/item.laneShareDenominator).toBe(UNIT_CACHE_POLICY_V1.gc.laneShare);
  expect(poolQuotaBytesV1(item.capacityBytes,item.machineQuotaBytes),item.id+" production").toBe(item.expect);
  expect(verdicts.quota[index],item.id+" oracle").toBe(item.expect);
 }
});

test("the production bounds are the authored pool policy",()=>{
 expect(poolBoundsV1(123)).toEqual({maximumLanes:UNIT_CACHE_POLICY_V1.pool.maximumLanes,maximumLiveLanes:UNIT_CACHE_POLICY_V1.pool.maximumLiveLanes,quotaBytes:123});
 expect(UNIT_CACHE_POLICY_V1.pool.maximumLiveLanes).toBeGreaterThanOrEqual(UNIT_CACHE_POLICY_V1.pool.maximumLanes);
});

test("on 600 seeded pools the production rules equal the oracle and keep the pool laws",()=>{
 const next=random(20261026),acquire:any[]=[],trim:any[]=[];
 for(let index=0;index<600;index++){
  const lanes=lanesOf(next,Math.floor(next()*9)),bounds={maximumLanes:1+Math.floor(next()*5),maximumLiveLanes:4+Math.floor(next()*5),quotaBytes:Math.floor(next()*1200)};
  acquire.push({lanes,crateKey:["a","b","c","*"][Math.floor(next()*4)],bounds});trim.push({lanes,bounds});
 }
 const verdicts=ask(["plan","/dev/stdin"],{acquire,trim,quota:[]});
 for(const [index,item] of acquire.entries()){
  const choice=planPoolAcquireV1(item.lanes,{crateKey:item.crateKey},item.bounds);
  expect(choice).toEqual(verdicts.acquire[index]);
  const byId=new Map<string,PoolLaneV1>(item.lanes.map((lane:PoolLaneV1)=>[lane.id,lane]));
  if(choice.id!==undefined)expect(byId.get(choice.id)!.claimed).toBe(false);
  const free=item.lanes.filter((lane:PoolLaneV1)=>!lane.claimed);
  if(free.some((lane:PoolLaneV1)=>lane.crateKey===item.crateKey))expect(choice.kind).toBe("warm");
  if(choice.kind==="busy"){expect(free.length).toBe(0);expect(item.lanes.length).toBeGreaterThanOrEqual(item.bounds.maximumLiveLanes);}
  if(choice.kind==="warm"){const best=orderBy(free.filter((lane:PoolLaneV1)=>lane.crateKey===item.crateKey),["lastUsedMs","id"],["desc","asc"])[0]!;expect(choice.id).toBe(best.id);}
  if(choice.kind==="lru"){const best=orderBy(free,["lastUsedMs","id"],["asc","asc"])[0]!;expect(choice.id).toBe(best.id);}
 }
 for(const [index,item] of trim.entries()){
  const victims=planPoolTrimV1(item.lanes,item.bounds);
  expect(victims).toEqual(verdicts.trim[index]);
  const idle=orderBy(item.lanes.filter((lane:PoolLaneV1)=>!lane.claimed),["lastUsedMs","id"],["asc","asc"]);
  expect(victims).toEqual(idle.slice(0,victims.length).map((lane:PoolLaneV1)=>lane.id));
  const left=item.lanes.filter((lane:PoolLaneV1)=>!victims.includes(lane.id));
  expect(planPoolTrimV1(left,item.bounds)).toEqual([]);
  const within=left.length<=item.bounds.maximumLanes&&left.reduce((sum:number,lane:PoolLaneV1)=>sum+(lane.bytes??0),0)<=item.bounds.quotaBytes;
  expect(within||left.every((lane:PoolLaneV1)=>lane.claimed)).toBe(true);
 }
});

test("the exhaustive interleaving of acquirers and a trimmer reproduces the counts of the corpus",()=>{
 const claim=sweep("claim"),unguarded=sweep("unguarded");
 expect({claim,unguarded}).toEqual(corpus.sweep);
 expect(ask(["sweep"])).toEqual(corpus.sweep);
 expect(claim.violations).toBe(0);expect(claim.stuck).toBe(0);expect(claim.unreachableDone).toBe(0);
 expect(unguarded.violations).toBeGreaterThan(0);
 expect(claim.overlaps).toBeGreaterThan(0);
});
