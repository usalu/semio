/** 🧹️ Contract: the cache prune plan deletes by age, then evicts oldest-first over budget without touching held or guarded units, exactly like an independent Python oracle over the hand-written scenarios and 150 seeded pseudo-random scenarios of `./🔣️.json`, upholds its invariants on every one of them, and the size formatter rounds half up like the Python exact decimal rendering. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {formatBytes,planCachePrune,type CacheAreaInput} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(request:object):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const summarize=(areas:readonly CacheAreaInput[]) =>planCachePrune(areas,corpus.nowMs,corpus.guardAgeMs).areas.map(row=>({name:row.name,totalBytes:row.totalBytes,unitCount:row.unitCount,age:row.ageDeletions.map(unit=>unit.path),budget:row.budgetDeletions.map(unit=>unit.path),retainedBytes:row.retainedBytes,guarded:row.guardedOverBudgetBytes}));

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,bytes:[-1]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,authored:[{...corpus.authored[0],areas:[{name:"a"}]}]})).toBe(false);
});

test("the hand-written scenarios give their authored plans and the Python plans",()=>{
 const rows=corpus.authored,answers=ask({op:"plan",nowMs:corpus.nowMs,guardAgeMs:corpus.guardAgeMs,rows});
 for(const[index,row]of rows.entries()){const actual=summarize(row.areas as never);expect(actual,row.name).toEqual(row.expect);expect(answers[index],row.name).toEqual(row.expect);}
});

test("the seeded pseudo-random scenarios match the Python plan and uphold every invariant of the policy",()=>{
 const answers=ask({op:"plan",nowMs:corpus.nowMs,guardAgeMs:corpus.guardAgeMs,rows:corpus.random});let aged=0,budgeted=0,guarded=0;
 for(const[index,row]of corpus.random.entries()){
  const plan=planCachePrune(row.areas as never,corpus.nowMs,corpus.guardAgeMs);expect(summarize(row.areas as never),row.name).toEqual(answers[index]);
  expect(plan.generatedAtMs).toBe(corpus.nowMs);
  for(const[position,areaPlan]of plan.areas.entries()){
   const input=row.areas[position]!,label=row.name+" "+areaPlan.name,deleted=[...areaPlan.ageDeletions,...areaPlan.budgetDeletions];
   expect(deleted.every(unit=>!unit.lockHeld),label).toBe(true);
   expect(new Set(deleted.map(unit=>unit.path)).size,label).toBe(deleted.length);
   expect(areaPlan.retainedBytes,label).toBe(areaPlan.totalBytes-deleted.reduce((sum,unit)=>sum+unit.bytes,0));
   expect(areaPlan.ageDeletions.every(unit=>unit.recencyMs<=corpus.nowMs-input.unusedAgeMs),label).toBe(true);
   const guard=corpus.nowMs-((input as {guardAgeMs?:number}).guardAgeMs??corpus.guardAgeMs);
   expect(areaPlan.budgetDeletions.every(unit=>unit.recencyMs<=guard),label).toBe(true);
   const order=areaPlan.budgetDeletions.map(unit=>[unit.recencyMs,unit.path] as const);expect(order,label).toEqual([...order].sort((left,right)=>left[0]-right[0]||(left[1]<right[1]?-1:left[1]>right[1]?1:0)));
   if(input.budgetBytes===null)expect(areaPlan.budgetDeletions.length,label).toBe(0);else if(areaPlan.guardedOverBudgetBytes===0)expect(areaPlan.retainedBytes<=input.budgetBytes||areaPlan.budgetDeletions.length+areaPlan.ageDeletions.length===input.units.length,label).toBe(true);
   if(areaPlan.ageDeletions.length)aged++;if(areaPlan.budgetDeletions.length)budgeted++;if(areaPlan.guardedOverBudgetBytes)guarded++;
  }
 }
 expect(aged).toBeGreaterThan(30);expect(budgeted).toBeGreaterThan(20);expect(guarded).toBeGreaterThan(10);
});

test("the plan never mutates its input and is a pure function of it",()=>{
 const input=structuredClone(corpus.random[0]!.areas) as CacheAreaInput[],again=structuredClone(input);
 const first=planCachePrune(input,corpus.nowMs,corpus.guardAgeMs);expect(input).toEqual(again);expect(planCachePrune(input,corpus.nowMs,corpus.guardAgeMs)).toEqual(first);
 expect(planCachePrune([],corpus.nowMs,corpus.guardAgeMs)).toEqual({generatedAtMs:corpus.nowMs,areas:[]});
});

test("byte counts are rendered like the exact round-half-up decimal rendering, binary units capped at TiB",()=>{
 const answers=ask({op:"format",bytes:corpus.bytes});
 for(const[index,count]of corpus.bytes.entries())expect(formatBytes(count),String(count)).toBe(answers[index]);
 expect(formatBytes(1152)).toBe("1.13 KiB");expect(formatBytes(0)).toBe("0 B");expect(formatBytes(1023)).toBe("1023 B");expect(formatBytes(1048576)).toBe("1.00 MiB");expect(formatBytes(2**50*2)).toBe("2048.00 TiB");
});
