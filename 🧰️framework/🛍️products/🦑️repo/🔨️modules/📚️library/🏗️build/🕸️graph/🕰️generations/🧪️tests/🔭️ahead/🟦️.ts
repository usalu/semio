/** 🔭️ Contract: a wave names the next stale owners of its read to the compute (`ahead`, in the order of the read, at most `ownerAhead`, never an owner of the wave itself), so the fleet pool can infer them while the wave runs; the same owners are the waves that follow, in the same order.
 * @see ../../🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GENERATION_POLICY_V1,type GenerationComputeSpecV1} from "../../🟨️.mjs";
import {createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const BASE=GENERATION_POLICY_V1.ownerWave,AHEAD=GENERATION_POLICY_V1.ownerAhead,COUNT=BASE*3+7,sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));

test("the policy declares how many next owners a wave names",()=>{
 expect(AHEAD).toBe(480);expect(AHEAD).toBeGreaterThanOrEqual(BASE);
});

test("every wave names the owners of the waves that follow it, in order, and the last wave names none",async()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-ahead-")),specs:GenerationComputeSpecV1[]=[];
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{specs.push(spec);return compute(spec,signal);}});
  await world.refresh();
  await sleep(20);
  for(let at=1;at<=COUNT;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const ahead=${at};\n`);
  specs.length=0;
  await world.refresh();
  const waves=specs.filter(spec=>!spec.cold),read=waves.flatMap(spec=>spec.owners);
  expect(read.length).toBe(COUNT);
  let offset=0;
  for(const spec of waves){
   const rest=read.slice(offset+spec.owners.length);
   expect(spec.ahead??[],"the owners that follow, in the order of the read").toEqual(rest.slice(0,AHEAD));
   expect((spec.ahead??[]).some(name=>spec.owners.includes(name)),"never an owner of the wave itself").toBe(false);
   offset+=spec.owners.length;
  }
  expect(waves.at(-1)!.ahead).toBeUndefined();
  expect(waves[0]!.ahead!.length).toBe(Math.min(AHEAD,COUNT-waves[0]!.owners.length));
 }finally{rmSync(dir,{recursive:true,force:true});}
});
