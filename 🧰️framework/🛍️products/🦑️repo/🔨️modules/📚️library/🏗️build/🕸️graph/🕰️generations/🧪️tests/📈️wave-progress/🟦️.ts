/** 📈️ Contract: the build tells the host with every published wave how many owners it has refreshed in this build and how many are stale in all, so the host can word the progress of a refresh that runs for hours. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GENERATION_POLICY_V1} from "../../🟨️.mjs";
import {createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const COUNT=GENERATION_POLICY_V1.ownerWave*2+7,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-wave-progress-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);};

test("every published wave carries the owners done and the owners stale in all, the last one equal",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation,told:[number,unknown][]=[];
  await edit(dir,COUNT);
  const result=await world.refresh({published:(generation:number,info:unknown)=>{told.push([generation,info]);}});
  expect(result.published).toBe(true);
  const wave=GENERATION_POLICY_V1.ownerWave;
  expect(told.map(([generation,info])=>[generation,{done:(info as {done:number}).done,total:(info as {total:number}).total}])).toEqual([[before+1,{done:wave,total:COUNT}],[before+2,{done:wave*2,total:COUNT}],[before+3,{done:COUNT,total:COUNT}]]);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
