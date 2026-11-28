/** 🛡️ Contract: over randomized interleavings of record publication, eviction (its references stay retained until the retirement settles) and retirement passes, the closure plan never selects a leaf that any live record or unsettled retirement intent references, and no record or intent is ever left dangling; an independent Python model replays the same trace and must agree on every selection.
 * @see ../../🪟️policy/🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {resolve} from "node:path";
import {planBuildGraphClosureRetirementV1} from "../../🪟️policy/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";

const identityOf=(at:number)=>(at%2?"source:":"lock:")+at.toString(16).padStart(64,"0");
const random=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/2**32;};
const trace=(seed:number)=>{
 const next=random(seed),pick=(count:number)=>Math.floor(next()*count),steps:Record<string,unknown>[]=[],inventory=new Map<string,number>(),live=new Map<string,string[]>(),intents=new Map<string,string[]>();
 let leaves=0,records=0;
 const leaf=()=>{const identity=identityOf(leaves++),allocatedBytes=4096*(1+pick(8));inventory.set(identity,allocatedBytes);steps.push({op:"leaf",identity,allocatedBytes});};
 for(let at=0;at<12;at++)leaf();
 for(let at=0;at<80;at++){
  const roll=pick(10),referencedBy=()=>new Set([...live.values(),...intents.values()].flat());
  if(roll<2)leaf();
  else if(roll<5&&inventory.size){const all=[...inventory.keys()],references=Array.from({length:1+pick(3)},()=>all[pick(all.length)]!),record="r"+records++;live.set(record,[...new Set(references)]);steps.push({op:"publish",record,references:[...new Set(references)]});}
  else if(roll<6&&live.size){const record=[...live.keys()][pick(live.size)]!;intents.set(record,live.get(record)!);live.delete(record);steps.push({op:"evict",record});}
  else if(roll<7&&intents.size){const record=[...intents.keys()][pick(intents.size)]!;intents.delete(record);steps.push({op:"settle",record});}
  else{
   const references=[...referencedBy()],inventories=[...inventory].map(([identity,allocatedBytes])=>({identity,allocatedBytes,modified:0})),availableBytes=pick(3)*4096,minimumAvailableBytes=availableBytes+4096*(1+pick(40)),selected=planBuildGraphClosureRetirementV1({availableBytes,minimumAvailableBytes,references,inventories}) as string[];
   for(const identity of selected){expect(referencedBy().has(identity)).toBe(false);inventory.delete(identity);}
   steps.push({op:"retire",availableBytes,minimumAvailableBytes,selected:[...selected]});
  }
 }
 return{steps};
};

test("no referenced leaf is ever selected for deletion over randomized interleavings, and an independent Python model agrees",()=>{
 for(let seed=1;seed<=60;seed++){
  const model=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py")],{encoding:"utf8",input:JSON.stringify(trace(seed))});
  expect(model.status).toBe(0);
  expect(JSON.parse(model.stdout).violations).toEqual([]);
 }
});
