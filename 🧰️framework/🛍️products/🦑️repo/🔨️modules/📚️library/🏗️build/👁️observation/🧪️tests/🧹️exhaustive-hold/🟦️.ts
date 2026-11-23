/** 🧹️ Contract: a complete look (`exhaustive`) verifies every row after the first stale one and reports each changed row, so the caller learns the whole set of changed paths of one pass; the default look stops at the first stale row. Rows that contradict each other never hold, also when each of them is checked.
 * @see ../../🟨️.mjs */
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{afterAll,expect,test}from"bun:test";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{observationsHold}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type Scenario={name:string;entries:string[];rows:{name:string;fact:boolean}[]};
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const model=(scenarios:readonly Scenario[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({scenarios}))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).answers as {stale:number[];holds:boolean}[];
};
const build=(entries:readonly string[])=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"exhaustive-hold-")));scratch.push(base);mkdirSync(base,{recursive:true});
 for(const name of entries)writeFileSync(join(base,name),"x\n");
 return base;
};
const look=async(scenario:Scenario,exhaustive:boolean)=>{
 const base=build(scenario.entries),reported:number[]=[],rows=scenario.rows.map(row=>({kind:"exists",path:join(base,row.name),fact:row.fact}));
 const ok=await observationsHold(rows as never,4,new Set(),new Set(),undefined,row=>{reported.push(rows.findIndex(each=>each.path===row.path&&each.fact===row.fact));},undefined,exhaustive);
 return{ok,reported:[...new Set(reported)].sort((a,b)=>a-b)};
};

test("a complete look reports exactly the rows the model calls stale and holds exactly when the model says so",async()=>{
 const scenarios=fixture.scenarios as Scenario[],answers=model(scenarios);
 for(const[at,scenario]of scenarios.entries()){
  const complete=await look(scenario,true),expected=answers[at]!;
  expect(complete.ok,scenario.name).toBe(expected.holds);
  expect(complete.reported.filter(index=>expected.stale.includes(index)),scenario.name).toEqual(expected.stale);
 }
});

test("the default look stops at the first stale row and reports only a few rows of a hundred changed ones",async()=>{
 const scenario=(fixture.scenarios as Scenario[]).find(each=>each.rows.length===100)!,answer=model([scenario])[0]!;
 const partial=await look(scenario,false),complete=await look(scenario,true);
 expect(partial.ok).toBe(false);expect(partial.reported.length).toBeGreaterThan(0);expect(partial.reported.length).toBeLessThan(answer.stale.length);
 expect(complete.reported).toEqual(answer.stale);
});
