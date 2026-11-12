/** 🌊️ Contract: the shared row layers of a stored closure entry are streamed from the store, never cached parsed: a layer comes back in batches equal to the rows that were saved, a damaged or missing layer is a typed loss (never an empty layer), and reading all layers of an entry repeatedly holds one layer at a time, not all of them. */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClosureStoreV1}from"../../🧭️script-inputs/💾️entries/🟨️.mjs";

type Row={kind:string;path:string;fact:unknown};
const layer=(at:number,count:number):Row[]=>Array.from({length:count},(_,row)=>({kind:"analysis",path:"/w/l"+at+"/f"+row,fact:{digest:String(at*100000+row).padEnd(64,"d"),size:row}}));

test("a stored layer streams back in batches of the rows that were saved, and a damaged or missing layer is a typed loss",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"layer-stream-")),store=createClosureStoreV1({directory});
 try{
  const layers=[layer(0,10),layer(1,1),layer(2,0)],saved=await store.save("recipe",{recipe:{fn:"x"},value:[],rows:[],layers});
  expect(saved.digests?.length).toBe(3);
  for(const[at,digest]of saved.digests!.entries()){
   const batches=[...store.layerBatchesSync(digest,4)];
   expect(batches.flat()).toEqual(layers[at]);expect(batches.every(batch=>batch.length<=4)).toBe(true);
  }
  const file=join(directory,"l",readdirSync(join(directory,"l")).find(name=>name.startsWith(saved.digests![0]!))!);
  writeFileSync(file,"damaged");
  expect(()=>[...store.layerBatchesSync(saved.digests![0]!,4)]).toThrow(expect.objectContaining({code:"CLOSURE_LAYER_LOST"}) as never);
  expect(()=>[...store.layerBatchesSync("0".repeat(64),4)]).toThrow(expect.objectContaining({code:"CLOSURE_LAYER_LOST"}) as never);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("reading the 24 layers of an entry three times holds one layer at a time: the heap a warm reader retains after three rounds stays below a tenth of the bytes read",()=>{
 const module=JSON.stringify(new URL("../../🧭️script-inputs/💾️entries/🟨️.mjs",import.meta.url).href),directory=mkdtempSync(join(tmpdir(),"layer-stream-memory-"));
 try{
  const writer=`
   const{createClosureStoreV1}=await import(${module});
   const store=createClosureStoreV1({directory:${JSON.stringify(directory)}});
   const layer=(at,count)=>Array.from({length:count},(_,row)=>({kind:"analysis",path:"/w/l"+at+"/f"+row,fact:{digest:String(at*1000000+row).padEnd(600,"d"),size:row,nested:{list:[row,row+1,row+2]}}}));
   const layers=Array.from({length:24},(_,at)=>layer(at,6000)),saved=await store.save("recipe",{recipe:{fn:"x"},value:[],rows:[],layers});
   let text=0;for(const one of layers)text+=JSON.stringify(one).length;
   console.log(JSON.stringify({text,digests:saved.digests}));`;
  const written=Bun.spawnSync([process.execPath,"-e",writer]);expect(written.exitCode,written.stderr.toString()).toBe(0);
  const{text,digests}=JSON.parse(written.stdout.toString().trim().split("\n").at(-1)!) as {text:number;digests:string[]};
  const reader=`
   const{createClosureStoreV1}=await import(${module});
   const store=createClosureStoreV1({directory:${JSON.stringify(directory)}}),digests=${JSON.stringify(digests)};
   const{heapStats}=await import("bun:jsc"),held=()=>{const stats=heapStats();return stats.heapSize+stats.extraMemorySize;};
   for(const digest of digests.slice(0,2))for(const batch of store.layerBatchesSync(digest,2048))batch.length;
   Bun.gc(true);const before=held();let read=0;
   for(let round=0;round<3;round++)for(const digest of digests)for(const batch of store.layerBatchesSync(digest,2048))read+=batch.length;
   Bun.gc(true);console.log(JSON.stringify({growth:held()-before,read}));`;
  const result=Bun.spawnSync([process.execPath,"-e",reader]),found=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {growth:number;read:number};
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  expect(found.read).toBe(3*24*6000);expect(text).toBeGreaterThan(80*1024*1024);
  expect(found.growth).toBeLessThan(text/10);
 }finally{rmSync(directory,{recursive:true,force:true});}
},180000);

test("entries that name equal values store that value once and share one parsed copy, and a damaged value is a miss",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"layer-stream-values-")),store=createClosureStoreV1({directory});
 try{
  const value=Array.from({length:5000},(_,at)=>"{workspaceRoot}/🧰️framework/📦️pkg/"+at+"/🟦️.ts");
  for(let at=0;at<200;at++)await store.save("recipe"+at,{recipe:{fn:"x",at},value:[...value],rows:[],layers:[]});
  expect(readdirSync(join(directory,"v")).length).toBe(1);
  expect(readdirSync(join(directory,"e")).length).toBe(200);
  const loaded=await Promise.all(Array.from({length:200},(_,at)=>store.load("recipe"+at)));
  expect(loaded.every(entry=>entry!==undefined&&entry.value===loaded[0]!.value)).toBe(true);
  expect(loaded[0]!.value).toEqual(value);
  const other=await store.save("other",{recipe:{fn:"y"},value:{closure:value.slice(0,10),generatedDependencies:[]},rows:[],layers:[]});
  expect(other.bytes).toBeGreaterThan(0);
  expect(readdirSync(join(directory,"v")).length).toBe(2);
  expect((await store.load("other"))?.value).toEqual({closure:value.slice(0,10),generatedDependencies:[]});
  writeFileSync(join(directory,"v",readdirSync(join(directory,"v"))[0]!),"damaged");
  const fresh=createClosureStoreV1({directory});
  expect(await Promise.all(["recipe0","other"].map(id=>fresh.load(id)))).toContain(undefined);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
