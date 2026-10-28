/** 🔀️ Contract: with the retention rule no provenance is lost under any interleaving of lane acquisition, compilation, publication, release and a trimmer; the same model with the lane released before its outputs are published, or with a trimmer that deletes what it planned instead of what nobody holds, does lose provenance (the negative controls prove the model can see the loss). The state machine is explored exhaustively twice: here against the production pool rules (`planPoolAcquireV1`, `planPoolTrimV1`) and in the independent Python model `./🐍️.py`, which must reach the same state counts.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {planPoolAcquireV1,planPoolTrimV1,type PoolBoundsV1,type PoolLaneV1} from "../../../🏊️pool/🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

type Regime="claim"|"unguarded";
type Order="retain-first"|"release-first";
type Lane={id:number;holder:number;key:string;used:number;pending:number[]};
type Proc={pc:"read"|"claim"|"run"|"retain"|"release"|"done";kind:string|null;id:number|null};
type Trim={pc:"off"|"read"|"delete"|"done";victims:number[]};
type State={lanes:Lane[];procs:Proc[];trim:Trim;tick:number;fresh:number;shared:number[];lost:number[]};
type Totals={configs:number;states:number;lostStates:number;lostFinal:number;stuck:number};

const explore=(config:{maximumLanes:number;maximumLiveLanes:number;regime:Regime;order:Order;lanes:string[];keys:string[]}):Omit<Totals,"configs">=>{
 const bounds:PoolBoundsV1={maximumLanes:config.maximumLanes,maximumLiveLanes:config.maximumLiveLanes,quotaBytes:Number.MAX_SAFE_INTEGER},n=config.keys.length;
 const view=(lanes:Lane[]):PoolLaneV1[]=>lanes.map(lane=>({id:String(lane.id),claimed:lane.holder>=0,lastUsedMs:lane.used,crateKey:lane.key,bytes:null}));
 const sorted=(lanes:Lane[])=>[...lanes].sort((left,right)=>left.id-right.id),encode=(state:State)=>JSON.stringify(state);
 const afterRun:Proc["pc"]=config.order==="retain-first"?"retain":"release";
 const start:State={lanes:config.lanes.map((key,id)=>({id,holder:-1,key,used:0,pending:[]})),procs:config.keys.map(()=>({pc:"read",kind:null,id:null})),trim:{pc:"read",victims:[]},tick:0,fresh:config.lanes.length,shared:[],lost:[]};
 const finished=(state:State)=>state.procs.every(proc=>proc.pc==="done")&&(state.trim.pc==="off"||state.trim.pc==="done");
 const moves=(state:State):State[]=>{
  const out:State[]=[],{lanes,procs,trim,tick,fresh,shared,lost}=state;
  const put=(index:number,proc:Proc,patch:Partial<State>={}):State=>{const next={lanes,procs:procs.map((row,at)=>at===index?proc:row),trim,tick,fresh,shared,lost,...patch};return{...next,lanes:sorted(next.lanes),shared:[...next.shared].sort((a,b)=>a-b),lost:[...next.lost].sort((a,b)=>a-b)};};
  for(let index=0;index<n;index++){
   const proc=procs[index]!;
   if(proc.pc==="read"){const choice=planPoolAcquireV1(view(lanes),{crateKey:config.keys[index]!},bounds);if(choice.kind!=="busy")out.push(put(index,{pc:"claim",kind:choice.kind,id:choice.id===undefined?null:Number(choice.id)}));}
   else if(proc.pc==="claim"){
    if(proc.kind==="new")out.push(put(index,{pc:"run",kind:"new",id:fresh},{lanes:[...lanes,{id:fresh,holder:index,key:config.keys[index]!,used:tick,pending:[]}],fresh:fresh+1}));
    else{const lane=lanes.find(row=>row.id===proc.id);if(lane&&lane.holder<0)out.push(put(index,{pc:"run",kind:proc.kind,id:proc.id},{lanes:lanes.map(row=>row.id===proc.id?{...row,holder:index}:row)}));else out.push(put(index,{pc:"read",kind:null,id:null}));}
   }
   else if(proc.pc==="run"){
    const lane=lanes.find(row=>row.id===proc.id);
    if(!lane)out.push(put(index,{...proc,pc:afterRun},{lost:[...lost,index]}));
    else out.push(put(index,{...proc,pc:afterRun},{lanes:lanes.map(row=>row.id===proc.id?{...row,pending:[...row.pending,index]}:row)}));
   }
   else if(proc.pc==="retain"){
    const lane=lanes.find(row=>row.id===proc.id),moved=lane?lane.pending:[];
    out.push(put(index,{...proc,pc:config.order==="retain-first"?"release":"done"},{lanes:lanes.map(row=>row.id===proc.id?{...row,pending:[]}:row),shared:[...shared,...moved]}));
   }
   else if(proc.pc==="release")out.push(put(index,{...proc,pc:config.order==="retain-first"?"done":"retain"},{lanes:lanes.map(row=>row.id===proc.id?{...row,holder:-1,key:config.keys[index]!,used:tick+1}:row),tick:tick+1}));
  }
  if(trim.pc==="read")out.push({...state,trim:{pc:"delete",victims:planPoolTrimV1(view(lanes),bounds).map(Number)}});
  else if(trim.pc==="delete"){
   if(trim.victims.length===0)out.push({...state,trim:{pc:"done",victims:[]}});
   else{
    const [head,...rest]=trim.victims,lane=lanes.find(row=>row.id===head),next={pc:"delete" as const,victims:rest};
    if(!lane||(config.regime==="claim"&&lane.holder>=0))out.push({...state,trim:next});
    else out.push({...state,lanes:lanes.filter(row=>row.id!==head),trim:next,lost:[...lost,...lane.pending].sort((a,b)=>a-b)});
   }
  }
  return out;
 };
 const seen=new Set<string>([encode(start)]),stack=[start];let lostStates=0,lostFinal=0,stuck=0;
 const everything=Array.from({length:n},(_,index)=>index);
 while(stack.length){
  const state=stack.pop()!;
  if(state.lost.length)lostStates++;
  const next=moves(state);
  if(finished(state)){if(state.lost.length||JSON.stringify([...new Set(state.shared)])!==JSON.stringify(everything))lostFinal++;}
  else if(!next.length)stuck++;
  for(const row of next){const code=encode(row);if(!seen.has(code)){seen.add(code);stack.push(row);}}
 }
 return{states:seen.size,lostStates,lostFinal,stuck};
};

const sweep=(regime:Regime,order:Order):Totals=>{
 const totals={configs:0,states:0,lostStates:0,lostFinal:0,stuck:0};
 for(const keys of [["a"],["a","a"],["a","b"],["a","b","a"]])for(const lanes of [[],["a"],["a","b"]])for(const maximumLanes of [1,2]){
  const result=explore({maximumLanes,maximumLiveLanes:maximumLanes+1,regime,order,lanes,keys});
  totals.configs++;totals.states+=result.states;totals.lostStates+=result.lostStates;totals.lostFinal+=result.lostFinal;totals.stuck+=result.stuck;
 }
 return totals;
};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 for(const mutate of [(d:any)=>{d.extra=1;},(d:any)=>{d.version=2;},(d:any)=>{delete d.unguarded.states;},(d:any)=>{d.rule.stuck=-1;}]){const copy=JSON.parse(JSON.stringify(corpus));mutate(copy);expect(ajv.validate(corpusSchema,copy)).toBe(false);}
});

test("with retention before release and a claim-guarded trimmer no receipt is ever lost, in the production model and in the Python model",()=>{
 const production=sweep("claim","retain-first");
 expect(production).toEqual(corpus.rule);
 expect(production.lostStates).toBe(0);expect(production.lostFinal).toBe(0);expect(production.stuck).toBe(0);
 const python=requireTestToolV1("python"),run=spawnSync(python,["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),"sweep"],{encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual({rule:corpus.rule,releaseFirst:corpus.releaseFirst,unguarded:corpus.unguarded});
},120000);

test("negative controls: releasing the lane before publishing, or a trimmer that deletes what it planned, loses receipts",()=>{
 const releaseFirst=sweep("claim","release-first"),unguarded=sweep("unguarded","retain-first");
 expect(releaseFirst).toEqual(corpus.releaseFirst);expect(unguarded).toEqual(corpus.unguarded);
 expect(releaseFirst.lostFinal).toBeGreaterThan(0);expect(unguarded.lostFinal).toBeGreaterThan(0);
 expect(releaseFirst.stuck).toBe(0);expect(unguarded.stuck).toBe(0);
},120000);
