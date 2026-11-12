import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createFactIndexV1}from"../../🧮️fact-rows/🟨️.mjs";
import{canonicalJsonV1,decodeFactIndexV1,encodeFactIndexV1}from"../../🧮️fact-codec/🟨️.mjs";
import{emptyFactIndexViewV1,patchFactIndexBytesV1}from"../../🌊️stream-patch/🟨️.mjs";
import{createRowLogV1}from"../../🗃️row-log/🟨️.mjs";
import{shapeComputeResultV1}from"../../🔌️load/🟨️.mjs";
import{derivedOwnerNameV1,derivedUpsertsV1}from"../../✂️cutoff/🟨️.mjs";

type Row={kind:string;path:string;fact:unknown};
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),LIMIT=1<<28;
const lcg=(seed:number)=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const keyOf=(row:Row)=>canonicalJsonV1([row.kind,row.path,row.fact]);
const pool=(count:number):Row[]=>Array.from({length:count},(_,at)=>({kind:["file","analysis","exists"][at%3]!,path:"p/"+at+(at%5===0?"é𝄞":""),fact:at%4===0?{b:at,a:{z:[1,2,at],y:"x".repeat(at%9)}}:{n:at%13}}));
const worlds=(owners:number,count:number,seed:number)=>{const random=lcg(seed),rows=pool(count);return Object.fromEntries(Array.from({length:owners},(_,owner)=>["o"+String(owner).padStart(3,"0"),Array.from({length:Math.floor(random()*count*0.6)},()=>({...rows[Math.floor(random()*count)]!}))]));};
const read=async(list:{batches():AsyncIterable<readonly Row[]>})=>{const out:Row[]=[];for await(const batch of list.batches())out.push(...batch);return out;};

test("the rows of every owner come back from the log in first-seen order without duplicates, the log holds each distinct row once, and the Python model agrees",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-")),owners=worlds(25,400,7),log=createRowLogV1<Row>({path:join(directory,"rl","a.rows"),policy:{flushBytes:4096,batchRows:37,gapBytes:512,spanBytes:8192,staleSeconds:3600}});
 try{
  const lists=new Map<string,Awaited<ReturnType<typeof log.builder>>|ReturnType<typeof log.append>>();
  for(const[name,rows]of Object.entries(owners)){const builder=log.builder();for(let at=0;at<rows.length;at+=53)builder.add(rows.slice(at,at+53));lists.set(name,builder.finish());}
  const fixturePath=join(directory,"fixture.json"),expected=(()=>{writeFileSync(fixturePath,JSON.stringify({owners}));const done=python?Bun.spawnSync([python,"-I",oracle,"distinct",fixturePath]):undefined;return done?(expect(done.exitCode).toBe(0),JSON.parse(done.stdout.toString()) as {distinct:number;owners:Record<string,string[]>}):undefined;})();
  const distinctKeys=new Set<string>();
  for(const[name,rows]of Object.entries(owners)){
   const list=lists.get(name) as ReturnType<typeof log.append>,back=await read(list),want=[...new Set(rows.map(keyOf))];
   expect(back.map(keyOf),name).toEqual(want);expect([...list.keys()],name).toEqual(want);expect(list.length,name).toBe(want.length);expect([...list].map(keyOf),name+" sync").toEqual(want);
   if(expected)expect(expected.owners[name],name).toEqual(want);
   for(const key of want)distinctKeys.add(key);
  }
  expect(log.stats().rows).toBe(distinctKeys.size);
  if(expected)expect(expected.distinct).toBe(distinctKeys.size);
  if(python){
   const first=lists.get("o000") as ReturnType<typeof log.append>;await read(first);
   const done=Bun.spawnSync([python,"-I",oracle,"log",log.path]);expect(done.exitCode).toBe(0);
   const held=JSON.parse(done.stdout.toString()) as {lines:number;keys:string[]};expect(held.lines).toBe(distinctKeys.size);expect(held.keys).toEqual([...distinctKeys].sort());
  }
 }finally{log.close();rmSync(directory,{recursive:true,force:true});}
});

test("lists extend with further rows, an owner of no rows is empty, and closing the log removes its file",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-")),rows=pool(30),log=createRowLogV1<Row>({path:join(directory,"b.rows")});
 try{
  const base=log.append(rows.slice(0,10)),more=base.concat(rows.slice(8,14));
  expect(more.length).toBe(16);expect((await read(more)).map(keyOf)).toEqual([...rows.slice(0,10),...rows.slice(8,14)].map(keyOf));
  expect([...more.keys()]).toEqual([...rows.slice(0,10),...rows.slice(8,14)].map(keyOf));
  expect(log.empty.length).toBe(0);expect(await read(log.empty)).toEqual([]);
  const builder=log.builder();builder.addList(base);builder.add(rows.slice(5,12));expect([...builder.finish().keys()]).toEqual(rows.slice(0,12).map(keyOf));
  expect(existsSync(log.path)).toBe(true);
 }finally{log.close();}
 expect(existsSync(log.path)).toBe(false);expect(()=>log.append([])).toThrow();
 rmSync(directory,{recursive:true,force:true});
});

test("the patch of an index from log-backed owners is byte for byte the patch from arrays",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-")),owners=worlds(40,300,3),log=createRowLogV1<Row>({path:join(directory,"c.rows"),policy:{flushBytes:2048,batchRows:19,gapBytes:256,spanBytes:4096,staleSeconds:3600}});
 try{
  const arrays=new Map(Object.entries(owners).map(([name,rows])=>[name,[...new Map(rows.map(row=>[keyOf(row),row])).values()]])),lazy=new Map(Object.entries(owners).map(([name,rows])=>[name,log.append(rows)]));
  const reference=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:arrays as never},{maximumBytes:LIMIT}),streamed=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:lazy as never},{maximumBytes:LIMIT});
  expect(Buffer.compare(streamed,reference)).toBe(0);
  expect(Buffer.compare(encodeFactIndexV1(decodeFactIndexV1(streamed,{maximumBytes:LIMIT}),{maximumBytes:LIMIT}),streamed)).toBe(0);
  expect(createFactIndexV1(arrays as never).rows.length).toBe(log.stats().rows);
 }finally{log.close();rmSync(directory,{recursive:true,force:true});}
});

test("shaping a compute result claims the same rows from log-backed owners as from arrays",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-")),log=createRowLogV1<Row>({path:join(directory,"d.rows")}),rows=pool(60).map(row=>({...row,path:"/w/"+row.path}));
 try{
  const inside=rows.map((row,at)=>({...row,path:"/w/a/"+at})),own=inside.slice(0,6),part=(nodeRows:unknown)=>({owners:[{name:"a",root:"a",node:{targets:{}},nodeRows},{name:"b",root:"b",node:{targets:{}},nodeRows:[]}],edgeOnly:[],edges:[],externalNodes:{},parseUnavailable:[],rows:[...inside,...rows.slice(40)],env:{digest:"e",rows:[]},replaceTags:[]});
  const shapedArrays=shapeComputeResultV1({workspaceRoot:"/w",part:part(own) as never,stubs:{}}),shapedLazy=shapeComputeResultV1({workspaceRoot:"/w",part:part(log.append(own)) as never,stubs:{}});
  expect(shapedLazy.owners.map(owner=>[...(owner.rows as Iterable<Row>)].map(keyOf).sort())).toEqual(shapedArrays.owners.map(owner=>[...(owner.rows as Iterable<Row>)].map(keyOf).sort()));
  expect(shapedLazy.env.rows.map(keyOf)).toEqual(shapedArrays.env.rows.map(keyOf));
  expect((shapedLazy.owners[0]!.rows as Iterable<Row>)&&shapedLazy.owners[0]!.rows.length).toBe(inside.length);
 }finally{log.close();rmSync(directory,{recursive:true,force:true});}
});

test("the supports of derived facts stay lists of the log and join the fact row without being read",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-")),log=createRowLogV1<Row>({path:join(directory,"e.rows")}),support=pool(12),fact:Row={kind:"derived",path:"walk:q",fact:{walk:1}};
 try{
  const lazy=derivedUpsertsV1([{row:fact,support:log.append(support)}]),plain=derivedUpsertsV1([{row:fact,support}]),name=derivedOwnerNameV1(fact);
  expect((await read(lazy.get(name) as never)).map(keyOf).sort()).toEqual([...plain.get(name)!].map(keyOf).sort());
  expect(log.stats().rows).toBe(support.length);
 }finally{log.close();rmSync(directory,{recursive:true,force:true});}
});

test("spilling the rows of 120 owners holds a few hundred bytes per distinct row, not the rows: the heap it retains stays below 512 bytes per distinct row and below half of the bytes the log holds",()=>{
 const script=`
  const{createRowLogV1}=await import(${JSON.stringify(new URL("../../🗃️row-log/🟨️.mjs",import.meta.url).href)});
  const{mkdtempSync,rmSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path");
  const directory=mkdtempSync(join(tmpdir(),"row-log-memory-")),log=createRowLogV1({path:join(directory,"m.rows")}),owners=120,per=2500,shared=100000;
  const make=at=>({kind:"analysis",path:"/w/p"+at,fact:{digest:"d".repeat(900),at,nested:{list:Array.from({length:8},(_,i)=>at+i)}}});
  const{heapStats}=await import("bun:jsc"),held=()=>{const stats=heapStats();return stats.heapSize+stats.extraMemorySize;};Bun.gc(true);const before=held();let total=0;
  const lists=[];
  for(let owner=0;owner<owners;owner++){const builder=log.builder(),start=(owner*997)%shared;for(let at=0;at<per;at+=500)builder.add(Array.from({length:500},(_,i)=>make((start+at+i)%shared)));lists.push(builder.finish());total+=per;}
  let read=0;for(const list of lists)for await(const batch of list.batches())read+=batch.length;
  Bun.gc(true);const growth=held()-before,stats=log.stats();
  console.log(JSON.stringify({growth,bytes:stats.bytes,rows:stats.rows,incidences:total,read}));log.close();rmSync(directory,{recursive:true,force:true});`;
 const result=Bun.spawnSync([process.execPath,"-e",script]),found=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {growth:number;bytes:number;rows:number;incidences:number;read:number};
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 expect(found.read).toBe(found.incidences);expect(found.bytes).toBeGreaterThan(90*1024*1024);
 expect(found.growth).toBeLessThan(found.rows*512);expect(found.growth).toBeLessThan(found.bytes/2);
},120000);

test("the patch learns the index row of a log row once: 300 owners that share a 3000-row closure read each distinct row from the log once, and the patched bytes equal the patch from arrays",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"row-log-shared-")),shared=pool(3000),log=createRowLogV1<Row>({path:join(directory,"rl","s.rows"),policy:{flushBytes:4096,batchRows:64,gapBytes:512,spanBytes:8192,staleSeconds:3600}});
 try{
  const lists=new Map<string,ReturnType<typeof log.append>>(),arrays=new Map<string,Row[]>();
  for(let owner=0;owner<300;owner++){const own:Row[]=[{kind:"file",path:"own/"+owner,fact:{owner}},...shared];arrays.set("o"+String(owner).padStart(3,"0"),own);lists.set("o"+String(owner).padStart(3,"0"),log.append(own));}
  const before=log.stats().reads,patched=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:lists as never}),afterReads=log.stats().reads,reference=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:arrays as never});
  expect(Buffer.compare(patched,reference)).toBe(0);
  expect(log.stats().rows).toBe(3300);
  expect(log.stats().readBytes).toBeLessThan(2*log.stats().bytes);
  expect(afterReads-before).toBeLessThan(3300);
 }finally{log.close();rmSync(directory,{recursive:true,force:true});}
},120000);
