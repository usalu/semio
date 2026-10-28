/** 🔂️ Contract: inside a validation round the verdict on the stored facts of a memoized computation is reached once per fact, at its first check, so a computation that thousands of consumers reuse is validated once per round and not once per use; without a round every use checks again. The verdicts of both shapes equal an independent Python model on every scenario, an edit made during a round is seen by the next round, and the round is much cheaper. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{memoizedObservedComputationV1,observationRoundV1,observedFs,withObservationRoundV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type Op=["hit"]|["edit",number]|["round"];
type Scenario={name?:string;files:number;ops:Op[]};
const model=(scenarios:readonly Scenario[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({scenarios}))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).answers as {plain:string[];round:string[]}[];
};

const play=(scenario:Scenario,rounds:boolean)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"validation-round-"))),paths=Array.from({length:scenario.files},(_,at)=>join(root,"f"+at)),text=paths.map(()=> "0"),cache=new Map<string,unknown>(),verdicts:string[]=[];
 const write=(at:number)=>writeFileSync(paths[at]!,text[at]+"#".repeat(text[at]!.length));
 paths.forEach((_,at)=>write(at));
 let computes=0;
 const hit=()=>{const before=computes;memoizedObservedComputationV1(cache,"k",()=>{computes++;return paths.map(path=>observedFs.readFileSync(path,"utf8").length);});verdicts.push(computes>before?"recompute":"reuse");};
 try{
  let batch:(()=>void)[]=[];
  const flush=()=>{const run=batch;batch=[];if(rounds)withObservationRoundV1(()=>{for(const step of run)step();});else for(const step of run)step();};
  let edits=0;
  for(const op of scenario.ops){
   if(op[0]==="hit")batch.push(hit);
   else if(op[0]==="edit"){const at=op[1];batch.push(()=>{edits++;text[at]=String(Number(text[at])+edits);write(at);});}
   else flush();
  }
  flush();
 }finally{rmSync(root,{recursive:true,force:true});}
 return verdicts;
};

test("outside a round there is none, inside one the open round is the one used",()=>{
 expect(observationRoundV1()).toBeUndefined();
 const outer=withObservationRoundV1(()=>[observationRoundV1(),withObservationRoundV1(()=>observationRoundV1())]);
 expect(outer[0]).toBeDefined();expect(outer[1]).toBe(outer[0]);
 expect(observationRoundV1()).toBeUndefined();
});

test("the verdicts of both shapes equal the independent model on the authored scenarios",()=>{
 const answers=model(fixture.scenarios as Scenario[]);
 for(const[at,scenario]of fixture.scenarios.entries()){
  expect(play(scenario as Scenario,false),scenario.name+" (plain)").toEqual(answers[at]!.plain);
  expect(play(scenario as Scenario,true),scenario.name+" (round)").toEqual(answers[at]!.round);
 }
 expect(answers.some(answer=>JSON.stringify(answer.plain)!==JSON.stringify(answer.round))).toBe(true);
});

test("60 seeded scenarios: the verdicts equal the model in both shapes",()=>{
 let seed=26102026;const random=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
 const scenarios:Scenario[]=Array.from({length:60},()=>{const files=1+Math.floor(random()*70);return{files,ops:Array.from({length:4+Math.floor(random()*14)},()=>{const pick=random();return(pick<0.5?["hit"]:pick<0.75?["edit",Math.floor(random()*files)]:["round"]) as Op;})};});
 const answers=model(scenarios);
 for(const[at,scenario]of scenarios.entries()){
  expect(play(scenario,false),"plain "+at).toEqual(answers[at]!.plain);
  expect(play(scenario,true),"round "+at).toEqual(answers[at]!.round);
 }
});

test("a round makes the reuse of a computation that 400 files back cheap: one validation instead of one per use",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"validation-round-cost-"))),paths=Array.from({length:400},(_,at)=>join(root,"f"+at)),cache=new Map<string,unknown>();
 try{
  for(const path of paths)writeFileSync(path,"x".repeat(2000));
  const use=()=>memoizedObservedComputationV1(cache,"k",()=>paths.map(path=>observedFs.readFileSync(path,"utf8").length));
  use();
  const time=(run:()=>void)=>{const started=performance.now();run();return performance.now()-started;};
  const plain=time(()=>{for(let at=0;at<40;at++)use();}),round=time(()=>withObservationRoundV1(()=>{for(let at=0;at<40;at++)use();}));
  expect(round*4,"round "+round.toFixed(1)+" ms, plain "+plain.toFixed(1)+" ms").toBeLessThan(plain);
 }finally{rmSync(root,{recursive:true,force:true});}
});
