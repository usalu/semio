/** 🤝️ Contract: a nested command that answers to an Original run reads exactly the generation that run served (the pin), never queues for the builder custody, never verifies the live tree and never plans, even when peers sealed newer generations meanwhile; it refuses in both languages when the pinned generation lacks an owner of its closure, was retired or belongs to another identity; a command without a bound pin behaves as before. */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{loadGenerationGraphV1}from"../../../🔌️load/🟨️.mjs";
import{readGenerationPointerV1}from"../../../🟨️.mjs";
import{applyOps,createMockHost,seed}from"../../../🧪️tests/🏃️world/🟦️.ts";
import{encodeGenerationPinV1,generationPinStateV1,readGenerationPinV1}from"../../🟨️.mjs";

const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-pin-"));
const chain=async(dir:string)=>{const files:Record<string,string>={};for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";await seed(dir,files);};
const edit=(dir:string,name:string,value:number)=>applyOps(dir,[{op:"write",path:`pkg/${name}/src/index.ts`,content:`export const v=${value};\n`}]);
const pointer=(dir:string)=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json")));
type Sealed={generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number};
const refusal=async(body:Promise<unknown>)=>{try{await body;}catch(error){return error as Error&{code?:string;reason?:string};}throw Error("refusal expected");};
const dead=()=>new Promise<void>(()=>{});

async function parentOf(dir:string,closure=["a"]){
 await chain(dir);const parent=createMockHost(dir,{stamp:true}),sealed:Sealed[]=[];(parent.host as any).sealed=(record:Sealed)=>{sealed.push(record);};
 const graph=await loadGenerationGraphV1(parent.host,{closure});
 const record=sealed.at(-1)!,pin=readGenerationPinV1(encodeGenerationPinV1({owner:process.pid,generation:record.generation,manifest:record.manifest,owners:record.owners,covered:record.covered,atMs:record.atMs}));
 return{parent,graph,record,pin};
}

test("the load tells which generation it served and which owners: the pin of its closure",async()=>{
 const dir=scratch();
 try{
  const{record,parent}=await parentOf(dir);
  expect(parent.log).toHaveLength(1);
  expect(record.generation).toBe(pointer(dir).generation);expect(record.manifest).toBe(pointer(dir).manifest!);
  expect([...record.covered!].sort()).toEqual(["a","b","c"]);expect(record.owners).toEqual(["a","b","c","d","e"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a nested command reads exactly the generation of its Original while peers sealed newer ones: no custody, no hold, no computation",async()=>{
 const dir=scratch();
 try{
  const{graph,pin,record}=await parentOf(dir);
  await edit(dir,"c",2);const peer=createMockHost(dir,{stamp:true});await loadGenerationGraphV1(peer.host,{closure:["a"]});
  expect(peer.log).toHaveLength(1);expect(pointer(dir).generation).toBeGreaterThan(record.generation);
  const nested=createMockHost(dir,{custody:{tail:dead()}}),reseal:Sealed[]=[];(nested.host as any).sealed=(row:Sealed)=>{reseal.push(row);};
  const started=Date.now(),served=await loadGenerationGraphV1(nested.host,{closure:["a"],pin});
  expect(Date.now()-started).toBeLessThan(5000);
  expect(nested.log).toHaveLength(0);expect(nested.holdsLog).toHaveLength(0);expect(nested.events.filter(event=>event.startsWith("builder"))).toEqual([]);
  expect(served).toEqual(graph);
  const newest=createMockHost(dir,{stamp:true}),current=await loadGenerationGraphV1(newest.host,{closure:["a"]});
  expect(current).not.toEqual(graph);
  expect(nested.said.map(row=>row.key)).toEqual(["pinned"]);expect(nested.said[0]!.text).toContain("pinned by the invoking build");expect(nested.said[0]!.text).toContain(`generation ${record.generation}`);
  expect(reseal.at(-1)).toMatchObject({generation:record.generation,manifest:record.manifest,atMs:pin.covered.atMs});
  expect([...reseal.at(-1)!.covered!].sort()).toEqual(["a","b","c"]);
  const german=createMockHost(dir,{language:"de",custody:{tail:dead()}});await loadGenerationGraphV1(german.host,{closure:["b"],pin});
  expect(german.said[0]!.text).toContain("gepinnte Generation");
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a nested command whose closure needs an owner the pinned generation did not serve refuses in both languages and plans nothing",async()=>{
 const dir=scratch();
 try{
  const{pin,record}=await parentOf(dir);
  for(const[closure,count,names]of[[["d"],2,"d, e"],[["a","d"],2,"d, e"],[["e"],1,"e"]] as [string[],number,string][]){
   const nested=createMockHost(dir,{custody:{tail:dead()}}),error=await refusal(loadGenerationGraphV1(nested.host,{closure,pin}));
   expect(error.code).toBe("GENERATION_PIN_REFUSED");expect(error.reason).toBe("uncovered");
   expect(error.message).toContain(`generation ${record.generation} does not cover ${count} of the projects it needs (${names})`);expect(error.message).toContain("deckt");expect(error.message).toContain("plant keinen eigenen Graphen");
   expect(nested.log).toHaveLength(0);expect(nested.holdsLog).toHaveLength(0);expect(nested.events.filter(event=>event.startsWith("builder"))).toEqual([]);
  }
  const unknown=createMockHost(dir,{custody:{tail:dead()}}),error=await refusal(loadGenerationGraphV1(unknown.host,{closure:["zz"],pin}));
  expect(error.reason).toBe("uncovered");expect(error.message).toContain("(zz)");
  const everything=createMockHost(dir,{custody:{tail:dead()}});
  expect((await refusal(loadGenerationGraphV1(everything.host,{pin}))).message).toContain("does not cover 2 of the projects it needs (d, e)");
  expect(everything.log).toHaveLength(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a pin of the whole graph serves every closure and the whole graph",async()=>{
 const dir=scratch();
 try{
  await chain(dir);const parent=createMockHost(dir,{stamp:true}),sealed:Sealed[]=[];(parent.host as any).sealed=(row:Sealed)=>{sealed.push(row);};
  const graph=await loadGenerationGraphV1(parent.host),record=sealed.at(-1)!;expect(record.covered).toBeUndefined();
  const pin=readGenerationPinV1(encodeGenerationPinV1({owner:process.pid,generation:record.generation,manifest:record.manifest,owners:record.owners,covered:record.covered,atMs:record.atMs}));
  await edit(dir,"a",9);const peer=createMockHost(dir,{stamp:true});await loadGenerationGraphV1(peer.host);
  const nested=createMockHost(dir,{custody:{tail:dead()}});
  expect(await loadGenerationGraphV1(nested.host,{pin})).toEqual(graph);expect(await loadGenerationGraphV1(createMockHost(dir,{custody:{tail:dead()}}).host,{closure:["d"],pin})).toBeDefined();
  expect(nested.log).toHaveLength(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a retired generation, another repository identity and a forged manifest refuse instead of falling back to a plan",async()=>{
 const dir=scratch();
 try{
  const{pin,record}=await parentOf(dir);
  const gone=readGenerationPinV1(encodeGenerationPinV1({owner:process.pid,generation:record.generation,manifest:"f".repeat(64),owners:record.owners,covered:record.covered,atMs:record.atMs}));
  const retired=createMockHost(dir,{custody:{tail:dead()}}),first=await refusal(loadGenerationGraphV1(retired.host,{closure:["a"],pin:gone}));
  expect(first.reason).toBe("retired");expect(first.message).toContain("is no longer kept");expect(first.message).toContain("wird nicht mehr aufbewahrt");
  const stranger=createMockHost(dir,{identity:"0".repeat(64),custody:{tail:dead()}}),second=await refusal(loadGenerationGraphV1(stranger.host,{closure:["a"],pin}));
  expect(second.reason).toBe("identity");expect(second.message).toContain("another repository identity");
  const renumbered=readGenerationPinV1(encodeGenerationPinV1({owner:process.pid,generation:record.generation+7,manifest:record.manifest,owners:record.owners,covered:record.covered,atMs:record.atMs}));
  expect((await refusal(loadGenerationGraphV1(createMockHost(dir,{custody:{tail:dead()}}).host,{closure:["a"],pin:renumbered}))).reason).toBe("identity");
  for(const host of[retired,stranger])expect(host.log).toHaveLength(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("without a bound pin a nested command behaves as before: it verifies the live tree and builds what changed",async()=>{
 const dir=scratch();
 try{
  const{graph,pin,record}=await parentOf(dir);
  await edit(dir,"c",3);
  const orphan=generationPinStateV1({SEMIO_BUILD_GENERATION_PIN:JSON.stringify(pin),SEMIO_BUILD_INVOCATION_PID:String(process.pid+1)});expect(orphan.state).toBe("foreign");
  const plain=createMockHost(dir,{stamp:true}),served=await loadGenerationGraphV1(plain.host,{closure:["a"]});
  expect(plain.log.map(row=>row.full)).toEqual([["c"]]);expect(served).not.toEqual(graph);expect(pointer(dir).generation).toBeGreaterThan(record.generation);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("the slice of a selected task closure is served from the pin and refuses a project the pin did not serve",async()=>{
 const dir=scratch();
 try{
  await chain(dir);const parent=createMockHost(dir,{stamp:true}),sealed:Sealed[]=[];(parent.host as any).sealed=(row:Sealed)=>{sealed.push(row);};
  const slice=(project:string)=>({selection:{kind:"tasks",tasks:[{project,target:"build"}]},roots:[project]});
  const graph=await loadGenerationGraphV1(parent.host,{slice:slice("a")}),record=sealed.at(-1)!;
  expect([...record.covered!].sort()).toEqual(["a","b","c"]);
  const pin=readGenerationPinV1(encodeGenerationPinV1({owner:process.pid,generation:record.generation,manifest:record.manifest,owners:record.owners,covered:record.covered,atMs:record.atMs}));
  await edit(dir,"b",5);const peer=createMockHost(dir,{stamp:true});await loadGenerationGraphV1(peer.host,{slice:slice("a")});
  const nested=createMockHost(dir,{custody:{tail:dead()}});
  expect(await loadGenerationGraphV1(nested.host,{slice:slice("b"),pin})).toBeDefined();expect(nested.log).toHaveLength(0);
  const again=createMockHost(dir,{custody:{tail:dead()}});expect(await loadGenerationGraphV1(again.host,{slice:slice("a"),pin})).toEqual(graph);
  const outside=createMockHost(dir,{custody:{tail:dead()}}),error=await refusal(loadGenerationGraphV1(outside.host,{slice:slice("d"),pin}));
  expect(error.reason).toBe("uncovered");expect(error.message).toContain("(d, e)");expect(outside.log).toHaveLength(0);
  const unknown=createMockHost(dir,{custody:{tail:dead()}});expect((await refusal(loadGenerationGraphV1(unknown.host,{slice:slice("zz"),pin}))).message).toContain("(zz)");
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
