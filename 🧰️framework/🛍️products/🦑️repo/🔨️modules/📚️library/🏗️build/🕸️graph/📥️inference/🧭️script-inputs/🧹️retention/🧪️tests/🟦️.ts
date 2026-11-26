/** 🧹️ The closure store keeps what its newest generation names, not every generation. */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync,mkdirSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";
import{createClosureStoreV1}from"../../💾️entries/🟨️.mjs";
import{collectClosureStoreV1,planClosureRetentionV1}from"../🟨️.mjs";

const GRACE=1000,tmp=()=>mkdtempSync(join(realpathSync(tmpdir()),"closure-retention-"));
const recipe=(slot:number)=>({fn:"target-sources",projectRoot:"p"+slot,target:{options:{command:"c"+slot,cwd:"."}}});
const names=(directory:string,kind:string)=>readdirSync(join(directory,kind)).sort();

test("the plan equals an independent Python model on 300 seeded stores",()=>{
 let seed=7;const random=(limit:number)=>(seed=(seed*1103515245+12345)&0x7fffffff)%limit;
 const cases=Array.from({length:300},()=>{
  const digests=Array.from({length:12},(_,index)=>"d"+index),now=100000,grace=1000;
  return{now,grace,
   entries:Array.from({length:random(14)},(_,index)=>({name:"e"+index,mtimeMs:now-random(3000),record:random(6)===0?null:{recipe:recipe(random(4)),layers:[digests[random(12)],digests[random(12)]],valueDigest:digests[random(12)]}})),
   layers:digests.map(name=>({name,mtimeMs:now-random(3000)})),values:digests.map(name=>({name,mtimeMs:now-random(3000)}))};
 });
 const out=Bun.spawnSync(["python3",join(import.meta.dir,"../🐍️oracle/🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify(cases))});
 const model=JSON.parse(out.stdout.toString());
 cases.forEach((value,index)=>{const got=planClosureRetentionV1(value);expect({entries:[...got.entries].sort(),layers:[...got.layers].sort(),values:[...got.values].sort()}).toEqual(model[index]);});
});

test("after eight generations of twenty slots the store holds what the last generation names, not eight times that",async()=>{
 const directory=tmp();
 try{
  const limits={...INFERENCE_LIMITS_V1,closureStoreGraceMilliseconds:GRACE,closureStoreCollectIntervalMilliseconds:0},store=createClosureStoreV1({directory,limits});
  for(let generation=0;generation<8;generation++)for(let slot=0;slot<20;slot++)await store.save("id-"+slot+"-g"+generation,{recipe:recipe(slot),value:["v",slot,generation],rows:[],layers:[[{kind:"file",path:"/l/"+slot+"/"+generation,fact:{size:generation}}]]});
  expect(names(directory,"l").length).toBe(160);expect(names(directory,"e").length).toBe(160);
  const dry=await collectClosureStoreV1({directory,graceMilliseconds:GRACE,now:Date.now()+10*GRACE,dryRun:true});
  expect(dry.removed?.entries).toBe(140);expect(names(directory,"e").length).toBe(160);
  const done=await collectClosureStoreV1({directory,graceMilliseconds:GRACE,now:Date.now()+10*GRACE});
  expect(done.kept).toEqual({entries:20,layers:20,values:20});expect(done.removed).toEqual({entries:140,layers:140,values:140});
  for(let slot=0;slot<20;slot++)expect(await store.load("id-"+slot+"-g7")).toBeDefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("an entry overwritten in place orphans its layers, which go after the grace and not before; a reused layer is refreshed and stays",async()=>{
 const directory=tmp();
 try{
  const limits={...INFERENCE_LIMITS_V1,closureStoreGraceMilliseconds:GRACE,closureStoreCollectIntervalMilliseconds:0},store=createClosureStoreV1({directory,limits}),layer=(n:number)=>[{kind:"file",path:"/x/"+n,fact:{size:n}}];
  await store.save("same",{recipe:recipe(1),value:[1],rows:[],layers:[layer(1)]});
  await store.save("same",{recipe:recipe(1),value:[2],rows:[],layers:[layer(2)]});
  expect(names(directory,"l").length).toBe(2);
  expect((await collectClosureStoreV1({directory,graceMilliseconds:GRACE})).removed?.layers).toBe(0);
  const later=Date.now()+10*GRACE;
  expect((await collectClosureStoreV1({directory,graceMilliseconds:GRACE,now:later})).removed).toEqual({entries:0,layers:1,values:1});
  const left=names(directory,"l");expect(left.length).toBe(1);
  const old=new Date(Date.now()-10*GRACE);utimesSync(join(directory,"l",left[0]!),old,old);
  const fresh=createClosureStoreV1({directory,limits});await fresh.save("other",{recipe:recipe(2),value:[3],rows:[],layers:[layer(2)]});
  expect(Date.now()-require("node:fs").statSync(join(directory,"l",left[0]!)).mtimeMs).toBeLessThan(GRACE);
  expect((await collectClosureStoreV1({directory,graceMilliseconds:GRACE,now:Date.now()+GRACE/2})).removed?.layers).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a damaged entry and a leftover temporary file go after the grace, a younger one stays",async()=>{
 const directory=tmp();
 try{
  for(const folder of["e","l","v"])mkdirSync(join(directory,folder));
  writeFileSync(join(directory,"e","bad.json.br"),"nope");writeFileSync(join(directory,"l","x.json.br.1.abc.tmp"),"half");
  expect((await collectClosureStoreV1({directory,graceMilliseconds:GRACE})).removed).toEqual({entries:0,layers:0,values:0});
  expect((await collectClosureStoreV1({directory,graceMilliseconds:GRACE,now:Date.now()+10*GRACE})).removed).toEqual({entries:1,layers:1,values:0});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a write that meets a full image prunes what is superseded or unreferenced first, tries once more, and only then fails",async()=>{
 const directory=tmp();
 try{
  const limits={...INFERENCE_LIMITS_V1,closureStoreGraceMilliseconds:GRACE,closureStoreEmergencyGraceMilliseconds:5000,closureStoreCollectIntervalMilliseconds:1e12},layer=(n:number)=>[{kind:"file",path:"/x/"+n,fact:{size:n}}];
  const seed=createClosureStoreV1({directory,limits});
  await seed.save("same",{recipe:recipe(1),value:[1],rows:[],layers:[layer(1)]});await seed.save("same",{recipe:recipe(1),value:[2],rows:[],layers:[layer(2)]});
  expect(names(directory,"l").length).toBe(2);
  const old=new Date(Date.now()-60000);for(const folder of["e","l","v"])for(const name of names(directory,folder))utimesSync(join(directory,folder,name),old,old);
  let full=true,attempts=0;
  const write=async(path:string,bytes:Uint8Array)=>{attempts++;if(full&&path.includes("/l/")&&names(directory,"l").length>=2)throw Object.assign(new Error("no space left on device"),{code:"ENOSPC"});writeFileSync(path,bytes);};
  const store=createClosureStoreV1({directory,limits,write});
  await store.save("other",{recipe:recipe(2),value:[3],rows:[],layers:[layer(3)]});
  expect(names(directory,"l").length).toBe(2);expect(attempts).toBeGreaterThan(1);
  const refused=createClosureStoreV1({directory,limits:{...limits,closureStoreEmergencyGraceMilliseconds:1e12},write:async()=>{throw Object.assign(new Error("no space left on device"),{code:"ENOSPC"});}});
  await expect(refused.save("third",{recipe:recipe(3),value:[4],rows:[],layers:[layer(4)]})).rejects.toThrow("no space");
  expect(readdirSync(join(directory,"l")).filter(name=>name.endsWith(".tmp")).length).toBe(0);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
