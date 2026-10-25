/** 🤝️ Contract: requests queued for the graph builder before its observation began are served by the generation it seals, without a second verification or computation; a closure only partly covered computes just the open part; a request that began after the observation builds; a crashed holder leaves no held custody; a waiter reports its wait in both languages. The expected counts come from the independent Python state machine of the same queue. */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{loadGenerationGraphV1}from"../../../🔌️load/🟨️.mjs";
import{readGenerationPointerV1}from"../../../🟨️.mjs";
import{decodeVerifiedScopeV1}from"../../🟨️.mjs";
import{applyOps,createMockHost,seed}from"../../../🧪️tests/🏃️world/🟦️.ts";
import fixture from"./🔣️.json";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url));
const model=Object.fromEntries(Bun.spawnSync([python!,"-I",join(here,"🐍️model/🐍️.py")],{stdin:Buffer.from(JSON.stringify(fixture))}).stdout.toString().trim().split("\n").map(line=>{const row=JSON.parse(line);return[row.id,row];}));
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-queue-"));
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const chain=async(dir:string)=>{const files:Record<string,string>={};for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";await seed(dir,files);};
const edit=(dir:string,name:string,value:number)=>applyOps(dir,[{op:"write",path:`pkg/${name}/src/index.ts`,content:`export const v=${value};\n`}]);
const stamp=(dir:string)=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json"))).verified;
const gate=()=>{let open!:()=>void;const wait=new Promise<void>(resolve=>{open=resolve;});return{wait:()=>wait,open};};
const builds=(...hosts:{log:unknown[]}[])=>hosts.reduce((sum,host)=>sum+host.log.length,0);

async function queue(stamped:boolean,dir:string){
 await chain(dir);const custody={tail:Promise.resolve()},cold=createMockHost(dir,{stamp:stamped,custody});await loadGenerationGraphV1(cold.host);
 await edit(dir,"c",2);
 const door=gate(),first=createMockHost(dir,{stamp:stamped,custody,gate:door.wait,ownerDelayMs:120}),second=createMockHost(dir,{stamp:stamped,custody}),third=createMockHost(dir,{stamp:stamped,custody}),fourth=createMockHost(dir,{stamp:stamped,custody});
 const one=loadGenerationGraphV1(first.host,{closure:["a"]});await sleep(40);
 const rest=[loadGenerationGraphV1(second.host,{closure:["a"]}),loadGenerationGraphV1(third.host,{closure:["a"]}),loadGenerationGraphV1(fourth.host,{closure:["b"]})];await sleep(60);
 door.open();while(first.log.length===0)await sleep(2);await edit(dir,"b",3);
 const graphs=await Promise.all([one,...rest]);
 return{first,second,third,fourth,graphs,custody};
}

test("requests queued before the builder observed are served by its sealed generation: one computation instead of one per waiter",async()=>{
 const dir=scratch();
 try{
  const{first,second,third,fourth,graphs,custody}=await queue(true,dir),expected=model["same-closure-queue"];
  expect(first.log.map(row=>row.full)).toEqual([["c"]]);
  expect(builds(second,third,fourth)).toBe(0);expect(second.holdsLog.length+third.holdsLog.length+fourth.holdsLog.length).toBe(0);
  expect(second.said.some(row=>row.key==="covered"&&row.text.includes("already covers it"))).toBe(true);
  expect(builds(first,second,third,fourth)).toBe(expected.builds);
  expect(Object.values(expected.served).filter(kind=>kind==="covered")).toHaveLength(2);
  expect((graphs[1] as any).projects.a).toEqual((graphs[0] as any).projects.a);
  const sealed=stamp(dir)!;expect(sealed).toBeDefined();expect([...decodeVerifiedScopeV1(["a","b","c","d","e"],sealed)].sort()).toEqual(["a","b","c"]);
  const later=createMockHost(dir,{stamp:true,custody});await loadGenerationGraphV1(later.host,{closure:["a"]});
  expect(later.log.map(row=>row.full)).toEqual([["b"]]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("without verified stamps every woken waiter verifies the live tree again and computes the edit that arrived meanwhile",async()=>{
 const dir=scratch();
 try{
  const{first,second,third,fourth}=await queue(false,dir);
  expect(builds(first,second,third,fourth)).toBe(2);
  expect(stamp(dir)).toBeUndefined();
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a closure that is only partly covered computes just the open owners and the stamp keeps the older time",async()=>{
 const dir=scratch();
 try{
  await chain(dir);const custody={tail:Promise.resolve()};await loadGenerationGraphV1(createMockHost(dir,{stamp:true,custody}).host);
  await edit(dir,"c",2);await edit(dir,"e",2);
  const door=gate(),course=createMockHost(dir,{stamp:true,custody,gate:door.wait}),print=createMockHost(dir,{stamp:true,custody}),dash=createMockHost(dir,{stamp:true,custody});
  const one=loadGenerationGraphV1(course.host,{closure:["a"]});await sleep(40);
  const two=loadGenerationGraphV1(print.host,{closure:["a","d"]}),three=loadGenerationGraphV1(dash.host,{closure:["d"]});await sleep(60);door.open();
  await Promise.all([one,two,three]);
  const expected=model["mixed-closures-partial-coverage"];
  expect(course.log.map(row=>row.full)).toEqual([["c"]]);expect([...print.log,...dash.log].map(row=>row.full)).toEqual([["e"]]);expect(print.said.concat(dash.said).some(row=>row.key==="covered")).toBe(true);
  expect(builds(course,print,dash)).toBe(expected.builds);expect(Object.values(expected.served).filter(kind=>kind==="covered")).toHaveLength(1);
  expect([...decodeVerifiedScopeV1(["a","b","c","d","e"],stamp(dir)!)].sort()).toEqual(["a","b","c","d","e"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a holder that fails releases the custody; the next waiter builds, nothing was stamped by the failed one",async()=>{
 const dir=scratch();
 try{
  await chain(dir);const custody={tail:Promise.resolve()};await loadGenerationGraphV1(createMockHost(dir,{stamp:true,custody}).host);
  await edit(dir,"c",2);
  const door=gate(),crashed=createMockHost(dir,{stamp:true,custody,gate:async()=>{await door.wait();throw Error("crashed holder");}}),next=createMockHost(dir,{stamp:true,custody});
  const failing=loadGenerationGraphV1(crashed.host,{closure:["a"]}).then(()=>"served",error=>String(error.message));await sleep(40);
  const waiting=loadGenerationGraphV1(next.host,{closure:["a"]});await sleep(40);door.open();
  expect(await failing).toBe("crashed holder");await waiting;
  expect(crashed.events.at(-1)).toBe("builder:released");expect(next.log.map(row=>row.full)).toEqual([["c"]]);
  const expected=model["crash-of-the-holder"];expect(expected.served.r1).toBe("crashed");expect(builds(crashed,next)).toBe(expected.builds);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a killed process holding the builder lease leaves nothing held",async()=>{
 const dir=scratch(),leases=join(dir,"leases");
 try{
  const{createFileLeasePortV1}=await import("../../../🟨️.mjs"),module=join(dirname(dirname(here)),"..","🟨️.mjs");
  const child=Bun.spawn([process.execPath,"-e",`const{createFileLeasePortV1}=await import(${JSON.stringify(module)});await createFileLeasePortV1(${JSON.stringify(leases)}).exclusive("builder",async()=>{console.log("held");await new Promise(()=>{});});`],{stdout:"pipe",stderr:"ignore"});
  const reader=child.stdout.getReader();await reader.read();child.kill(9);await child.exited;
  const started=Date.now();let acquired=false;await createFileLeasePortV1(leases).exclusive("builder",async()=>{acquired=true;},{timeoutMs:5000});
  expect(acquired).toBe(true);expect(Date.now()-started).toBeLessThan(5000);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

for(const language of["en","de"] as const)test("a waiter reports its wait and the newest generation while it queues: "+language,async()=>{
 const dir=scratch();
 try{
  await chain(dir);const custody={tail:Promise.resolve()};await loadGenerationGraphV1(createMockHost(dir,{stamp:true,custody}).host);
  await edit(dir,"c",2);
  const door=gate(),holder=createMockHost(dir,{stamp:true,custody,gate:door.wait}),waiter=createMockHost(dir,{stamp:true,custody,language,waitNoticeMilliseconds:25});
  const one=loadGenerationGraphV1(holder.host,{closure:["a"]});await sleep(40);const two=loadGenerationGraphV1(waiter.host,{closure:["a"]});while(!waiter.events.includes("builder:wait"))await sleep(2);await sleep(150);
  const notices=waiter.said.filter(row=>row.key==="waiting");expect(notices.length).toBeGreaterThanOrEqual(2);
  expect(notices[0]!.text).toContain(language==="en"?"waiting for the graph build of a peer":"wartet auf den Graph-Aufbau");expect(notices[0]!.text.toLowerCase()).toContain("generation");
  door.open();await Promise.all([one,two]);const after=waiter.said.length;await sleep(80);expect(waiter.said.length).toBe(after);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("requests that need no custody are served verified exactly like the model",async()=>{
 const dir=scratch();
 try{
  await chain(dir);const custody={tail:Promise.resolve()},cold=createMockHost(dir,{stamp:true,custody});await loadGenerationGraphV1(cold.host);
  const clean=createMockHost(dir,{stamp:true,custody});await loadGenerationGraphV1(clean.host,{closure:["a"]});await loadGenerationGraphV1(clean.host,{closure:["d"]});
  expect(clean.events.filter(event=>event==="builder:held")).toEqual([]);expect(Object.values(model["clean-needs-no-custody"].served)).toEqual(["verified","verified"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
