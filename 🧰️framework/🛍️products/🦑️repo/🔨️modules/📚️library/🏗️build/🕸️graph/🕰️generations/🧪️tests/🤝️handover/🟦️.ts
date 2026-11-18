/** 🤝️ Contract: a build that runs on other code than the library (or made no progress) while a conflicting read waits stops at its next wave boundary with the restart error, keeps every finished wave, and the read that starts again derives only the owners the waves did not: no finished owner is computed twice and the final graph equals the independent Python oracle. A build with nobody waiting runs on. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1,createNodeFilePortV1} from "../../🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../🧩️claims/🟨️.mjs";
import {createSliceCustodyV1} from "../../🔐️custody/🟨️.mjs";
import {STALE_IMPLEMENTATION_CODES_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {createCompute,createMembership,createWorld,IDENTITY,IMPLEMENTATION,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const COUNT=GENERATION_POLICY_V1.ownerWave*2+7,sha="a".repeat(64),other="b".repeat(64),scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-handover-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const handed=${at};\n`);};
const custodyOf=(dir:string)=>createSliceCustodyV1({root:join(dir,".custody"),fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(dir,".claims"))});
const holder=(digest:string)=>({pid:process.pid,command:"script.ts repo show projects",startedMs:1,digest});

test("a build on stale code hands over at its next wave while a conflicting read waits; the request that starts again computes only what the waves did not",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);await world.refresh();
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);world.log.length=0;
  const custody=custodyOf(dir),membership=createMembership(dir),last=Array.from({length:COUNT},(_,at)=>"p"+(at+1)).sort().at(-1)!;let waiter:Promise<unknown>|undefined;
  const base=createCompute(dir,world.log),compute=async(spec:never,signal?:AbortSignal)=>{waiter??=custody.hold({owners:[last],holder:holder(other)});await sleep(30);return(base as never as (spec:unknown,signal?:AbortSignal)=>Promise<unknown>)(spec,signal);};
  const first=await custody.hold({owners:[],holder:holder(sha),currentDigest:async()=>other});
  let thrown:(Error&{code?:string;handover?:{reason:string;done:number}})|undefined;
  try{await world.store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute:compute as never,builder:body=>body(first)});}catch(error){thrown=error as never;}finally{await first.release();}
  expect(thrown?.code).toBe("SEMIO_STALE_IMPLEMENTATION");expect(STALE_IMPLEMENTATION_CODES_V1.has(thrown!.code!)).toBe(true);
  expect(thrown?.handover).toMatchObject({reason:"stale-code",done:GENERATION_POLICY_V1.ownerWave});
  expect(thrown!.message).toContain("hands over after");expect(thrown!.message).toContain("übergibt nach");
  expect((await world.store.read())!.pointer.generation).toBe(before);
  const finished=new Set(world.log.flatMap(entry=>entry.full));expect(finished.size).toBe(GENERATION_POLICY_V1.ownerWave);
  const held=await waiter;await (held as {release():Promise<void>}).release();
  world.log.length=0;
  const again=await custody.hold({owners:[],holder:holder(other),currentDigest:async()=>other});
  const resumed=await world.store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute:base as never,builder:body=>body(again)});await again.release();
  expect(resumed.published).toBe(true);expect(resumed.computed).toBe(COUNT-GENERATION_POLICY_V1.ownerWave);
  expect(world.log.flatMap(entry=>entry.full).some(name=>finished.has(name))).toBe(false);
  expect(new Set([...finished,...world.log.flatMap(entry=>entry.full)]).size).toBe(COUNT);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(resumed.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("with nobody waiting a build on stale code runs on to its generation; finished waves release their owners to the custody as they go",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);await world.refresh();await edit(dir,COUNT);
  const custody=custodyOf(dir),claim=await custody.hold({owners:[],holder:holder(sha),currentDigest:async()=>other}),sizes:number[]=[];
  const result=await world.store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership:createMembership(dir),compute:createCompute(dir,world.log),wave:()=>{sizes.push(claim.owners.length);},builder:body=>body(claim)});
  await claim.release();
  expect(result.published).toBe(true);expect(result.computed).toBe(COUNT);
  expect(sizes).toEqual([COUNT-GENERATION_POLICY_V1.ownerWave,7]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
