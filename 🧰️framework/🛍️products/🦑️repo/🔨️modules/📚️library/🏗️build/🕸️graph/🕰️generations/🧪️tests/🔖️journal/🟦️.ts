import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,realpathSync,rmSync}from"node:fs";
import{writeFile}from"node:fs/promises";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import journalSchema from"../../🔖️journal/🧬️schema/🔣️.json";
import{BUILD_JOURNAL_V1,buildJournalKeyV1,decodeJournalRowsV1,encodeJournalRowsV1,readBuildJournalV1}from"../../🔖️journal/🟨️.mjs";
import{GENERATION_POLICY_V1,canonicalJsonV1}from"../../🟨️.mjs";
import{createWorld,ringFiles,seed}from"../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const COUNT=GENERATION_POLICY_V1.ownerWave*2+7,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-journal-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);};
const hex=(seed:string)=>buildJournalKeyV1(seed);
const journalFiles=(dir:string)=>{try{return readdirSync(join(dir,".gen","j"));}catch{return[];}};
const killAt=(step:string,seen:string[])=>(done:string)=>{seen.push(done);if(done===step)throw new Error("killed after "+step);};

test("the journal header is closed under its schema and its rows files round-trip, long lists in several lines",()=>{
 const ajv=new Ajv({strict:false}),header={version:1,key:hex("k"),state:"waves",parent:null,generation:0,identity:hex("i"),implementation:hex("m"),steps:[{step:"storing",inputs:hex("a"),output:hex("b")}],names:["a","b"],done:1,owners:{a:{root:"a"}},dropped:[],returned:["a"],derived:[],rows:[{file:hex("k")+".0.rows",sha256:hex("r"),bytes:12}],env:{digest:hex("e")}};
 expect(ajv.validate(journalSchema,header),JSON.stringify(ajv.errors?.slice(0,2))).toBe(true);
 expect(readBuildJournalV1(Buffer.from(JSON.stringify(header))).done).toBe(1);
 expect(ajv.validate(journalSchema,{...header,state:"half"})).toBe(false);
 expect(()=>readBuildJournalV1(Buffer.from(JSON.stringify({...header,state:"half"})))).toThrow();
 expect(()=>readBuildJournalV1(Buffer.from(JSON.stringify({...header,key:"short"})))).toThrow();
 const rows=Array.from({length:BUILD_JOURNAL_V1.rowsPerLine*2+5},(_unused,at)=>({kind:"file",path:"p/"+at,fact:{size:at}}));
 const bytes=encodeJournalRowsV1([{name:"o",mode:"u",rows},{name:"e",mode:"a",rows:[]},{name:"o2",mode:"u",rows:rows.slice(0,3)}]);
 expect(bytes.toString("utf8").split("\n").filter(Boolean).length).toBe(5);
 expect(decodeJournalRowsV1(bytes)).toEqual([{name:"o",mode:"u",rows},{name:"e",mode:"a",rows:[]},{name:"o2",mode:"u",rows:rows.slice(0,3)}]);
 expect(()=>decodeJournalRowsV1(Buffer.from('{"o":"x","m":"z","r":[]}\n'))).toThrow();
});

test("a build killed after the index was kept is published by the next try without deriving, storing or indexing again, and the tree is checked against it",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT));
  const first:string[]=[],killed=createWorld(dir,{stepped:killAt("indexing",first)});
  await expect(killed.refresh()).rejects.toThrow("killed after indexing");
  expect(first).toEqual(["storing","indexing"]);
  expect((await killed.store.read())?.manifest).toBeUndefined();
  expect(journalFiles(dir).length).toBe(1);
  const second:string[]=[],again=createWorld(dir,{stepped:step=>{second.push(step);}}),result=await again.refresh();
  expect(second).toEqual(["sealing"]);
  expect(again.log.length).toBe(0);
  expect(result.published).toBe(true);expect(result.served).toBe("built");
  expect((await again.store.read())!.pointer.generation).toBe(0);
  expect(journalFiles(dir)).toEqual([]);
  const{generation:_g,membership:_m,...rest}=await again.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a sealed build whose tree changed meanwhile is published and then checked: the changed owner is derived again",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT));
  const killed=createWorld(dir,{stepped:killAt("indexing",[])});
  await expect(killed.refresh()).rejects.toThrow("killed after indexing");
  await edit(dir,1);
  const again=createWorld(dir),result=await again.refresh();
  expect(result.published).toBe(true);
  expect(again.log.flatMap(entry=>entry.full)).toEqual(["p1"]);
  const{generation:_g,membership:_m,...rest}=await again.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a read publishes one generation however many waves it derives, tells the host of every wave but the last, and equals the oracle",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);
  world.log.length=0;
  const told:number[]=[],published:number[]=[];
  const result=await world.refresh({published:generation=>{published.push(generation);},wave:info=>{told.push(info.done);}});
  expect(result.published).toBe(true);expect(result.computed).toBe(COUNT);
  expect((await world.store.read())!.pointer.generation-before).toBe(1);
  expect(world.log.map(entry=>entry.full.length)).toEqual([GENERATION_POLICY_V1.ownerWave,GENERATION_POLICY_V1.ownerWave,7]);
  expect(told).toEqual([GENERATION_POLICY_V1.ownerWave,GENERATION_POLICY_V1.ownerWave*2]);
  expect(published).toEqual([before+1]);
  expect(journalFiles(dir)).toEqual([]);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a build cut off after a wave derives only the rest on the next try, publishes one generation and equals the oracle; a build of other owners starts over",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const controller=new AbortController();let calls=0;
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{if(++calls===2)controller.abort(new Error("memory pressure"));return compute(spec,signal);}});
  await world.refresh();calls=0;
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);
  world.log.length=0;
  await expect(world.refresh({signal:controller.signal})).rejects.toThrow();
  expect((await world.store.read())!.pointer.generation).toBe(before);
  const finished=new Set(world.log.flatMap(entry=>entry.full).slice(0,GENERATION_POLICY_V1.ownerWave));
  expect(journalFiles(dir).length).toBe(2);
  world.log.length=0;
  const resumed=await world.refresh();
  expect(resumed.published).toBe(true);expect(resumed.computed).toBe(COUNT-GENERATION_POLICY_V1.ownerWave);
  expect(world.log.flatMap(entry=>entry.full).some(name=>finished.has(name))).toBe(false);
  expect((await world.store.read())!.pointer.generation-before).toBe(1);
  expect(journalFiles(dir)).toEqual([]);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(resumed.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a journal of a build with other owners is not taken up",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const controller=new AbortController();let calls=0;
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{if(++calls===2)controller.abort(new Error("memory pressure"));return compute(spec,signal);}});
  await world.refresh();calls=0;
  await edit(dir,COUNT);
  await expect(world.refresh({signal:controller.signal})).rejects.toThrow();
  await edit(dir,COUNT+1);
  world.log.length=0;
  const again=await world.refresh();
  expect(again.computed).toBe(COUNT+1);
  expect(world.log.flatMap(entry=>entry.full).length).toBe(COUNT+1);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
