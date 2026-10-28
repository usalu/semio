/** 🗂️ Contract: the indexed relative import ownership answers exactly what the linear scan answers, for every path and every owner list, and what an independent Python model answers. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{buildRelativeImportProjectV1,createRelativeImportResolverV1}from"../../🟦️.ts";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py");
type World={owners:{name:string;root:string}[];paths:string[]};
const model=(worlds:readonly World[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({worlds}))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).answers as (string|null)[][];
};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};

for(const row of fixture.cases)test("case: "+row.name,()=>{
 const resolve=createRelativeImportResolverV1(row.owners);
 expect(row.paths.map(path=>resolve(path)??null)).toEqual(row.expected);
 expect(row.paths.map(path=>buildRelativeImportProjectV1(row.owners,path)??null)).toEqual(row.expected);
 expect(model([{owners:row.owners,paths:row.paths}])).toEqual([row.expected]);
});

test("120 seeded worlds: the index, the linear scan and the independent model agree on every path",()=>{
 const random=seeded(26102026),words=["a","b","ab","c","🧰️x","🔨️y","dir","dir2","m n"];
 const directory=()=>Array.from({length:1+Math.floor(random()*4)},()=>words[Math.floor(random()*words.length)]!).join("/");
 let resolved=0,none=0;const worlds:World[]=[],indexedAll:(string|null)[][]=[];
 for(let world=0;world<120;world++){
  const owners=Array.from({length:1+Math.floor(random()*30)},(_,at)=>({name:"p"+Math.floor(random()*40)+"-"+at%3,root:random()<0.05?".":directory()}));
  const paths=Array.from({length:60},()=>directory()+(random()<0.7?"/file"+Math.floor(random()*5)+".ts":""));
  const resolve=createRelativeImportResolverV1(owners),indexed=paths.map(path=>resolve(path)??null);
  expect(indexed).toEqual(paths.map(path=>buildRelativeImportProjectV1(owners,path)??null));
  worlds.push({owners,paths});indexedAll.push(indexed);
  for(const answer of indexed)answer===null?none++:resolved++;
 }
 expect(model(worlds)).toEqual(indexedAll);
 expect(resolved).toBeGreaterThan(500);expect(none).toBeGreaterThan(500);
});
