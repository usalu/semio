/** 🎁️ Contract: a component pass inside a parent memory grant is started when the grant carries it, refused at once when nothing can ever carry it (an exact grant that nobody grows, a need beyond what the machine could give), and carried by a supplementary reservation of the difference when the gate grants one: it never waits for ever for a grant that cannot grow. The expected outcomes come from the independent Python oracle.
 * @see ./🐍️oracle/🐍️.py
 * @see ./🧫️fixtures/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import * as jobs from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const python=Bun.which("python3")??Bun.which("python")??"python3",oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as Record<string,{outcome:string;acquired:number;allowance:number}>;})();
const sample=async()=>({availableMegabytes:8000,ownedMegabytes:0,pressure:"normal" as const});

test("the cases satisfy their schema",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Fixture,fixture,schema)).toEqual([]);
});

for(const row of fixture.cases)test(`case ${row.name}: the pass is ${expected[row.name]!.outcome} exactly as the oracle says`,async()=>{
 let acquired=0,released=0,clock=0;
 const growth:jobs.ParentComponentGrowthV1={ceilingMegabytes:row.ceilingMegabytes,waitMilliseconds:600000,refresh:()=>undefined,declare:()=>undefined,...row.growth==="none"?{}:{exact:row.growth==="exact"||row.growth==="exact-acquire-granted"},...row.growth.startsWith("acquire")||row.growth==="exact-acquire-granted"?{acquire:async(extra:number)=>{acquired+=extra;return row.growth==="acquire-refused"?undefined:{release:()=>{released++;}};}}:{}};
 const memory=new jobs.ParentComponentMemoryV1(row.grantMegabytes,row.baselineMegabytes,row.growth==="none"?undefined:growth);
 const started:string[]=[];
 const work=jobs.runPassesInJobSlotsV1([{target:"wasm32-wasip2"}],{jobs:1,check:()=>undefined,parentMemory:memory,local:{maximum:1,peak:()=>row.peakMegabytes,footprintMegabytes:row.footprintMegabytes,floorMegabytes:0,sample,pollMilliseconds:1,now:()=>clock+=1000},run:async(_pass,slot)=>{started.push(slot);}});
 const result=await work.then(()=>"started",error=>{expect(error).toMatchObject({name:"ParentComponentMemoryRequiredErrorV1"});return "refused";});
 expect({outcome:result,acquired,allowance:memory.allowanceMegabytes}).toEqual(expected[row.name]!);
 expect(started).toEqual(result==="started"?["local"]:[]);
 memory.releaseGrowth();
 expect(released).toBe(result==="started"&&acquired>0?1:0);
});

test("the refusal names the requirement and the grant in both languages",async()=>{
 const memory=new jobs.ParentComponentMemoryV1(3400,750,{ceilingMegabytes:14336,waitMilliseconds:600000,refresh:()=>undefined,declare:()=>undefined,exact:true});
 await expect(jobs.runPassesInJobSlotsV1([{target:"wasm32-wasip2"}],{jobs:1,check:()=>undefined,parentMemory:memory,local:{maximum:1,peak:()=>3409,floorMegabytes:0,sample,pollMilliseconds:1},run:async()=>undefined})).rejects.toThrow(/Component build needs 4159 MiB for the local pass, but the parent granted 3400 MiB.*Komponentenbau/su);
});
