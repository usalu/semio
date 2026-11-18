/** ⏱️ Contract: a published wave tells where its seconds went below the coarse stages: the membership scan of a build that scanned and the three parts of the plan (index, facts, settle) of a wave that planned, and, once per process, the seconds from the start of the process to the first build, so the first wave after a start explains itself. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GENERATION_POLICY_V1} from "../../🟨️.mjs";
import {createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const BASE=GENERATION_POLICY_V1.ownerWave,COUNT=BASE*2+7,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-stages-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const staged=${at};\n`);};
type Info={phases?:Record<string,number>};

test("the first build of the process names its start, the first wave of a delta names its plan stages, later waves of the same read name neither",async()=>{
 const dir=scratch(),told:Info[][]=[[],[]];
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh({published:(_generation:number,info:Info)=>{told[0]!.push(info);}});
  expect(told[0]!.length).toBe(1);
  expect(Number.isSafeInteger(told[0]![0]!.phases!.start)).toBe(true);
  expect(told[0]![0]!.phases!.start).toBeGreaterThanOrEqual(0);
  expect(Number.isSafeInteger(told[0]![0]!.phases!.membership)).toBe(true);
  expect(told[0]![0]!.phases!["plan.facts"]).toBeUndefined();
  await edit(dir,COUNT);
  await world.refresh({published:(_generation:number,info:Info)=>{told[1]!.push(info);},wave:(info:unknown)=>{told[1]!.push(info as Info);}});
  expect(told[1]!.length).toBe(3);
  const [first,second,third]=told[1]!.map(info=>info.phases!);
  for(const name of ["plan.index","plan.facts","plan.settle"])expect(Number.isSafeInteger(first![name]),name).toBe(true);
  for(const wave of [first!,second!,third!])expect(wave.start).toBeUndefined();
  for(const wave of [second!,third!])for(const name of ["plan.index","plan.facts","plan.settle","membership"])expect(wave[name],name).toBeUndefined();
  for(const wave of [first!,second!])for(const name of ["prime","plan","compute","fragments"])expect(Number.isSafeInteger(wave[name]),name).toBe(true);
  for(const name of ["prime","plan","compute","fragments","patch","pack","publish"])expect(Number.isSafeInteger(third![name]),name).toBe(true);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
