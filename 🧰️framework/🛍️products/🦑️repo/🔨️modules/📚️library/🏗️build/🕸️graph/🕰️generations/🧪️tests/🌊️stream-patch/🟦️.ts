/** 🌊️ Contract: a fact index patched in its packed bytes (`🌊️stream-patch`) is exactly the index the in-memory patch (`🧮️fact-rows` `patchFactIndexV1`) gives, in canonical bytes (decoding and encoding it again returns the same bytes), for seeded random patches with new, shared and renamed rows, nested facts, digests and non-ASCII paths; the consumers of every row equal the independent Python model; rows read in batches equal rows given as arrays; and the peak memory of patching many owners stays within a few bytes per owner-row incidence because the rows of an owner are read where they are consumed. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createFactIndexV1,factConsumersV1,patchFactIndexV1}from"../../🟨️.mjs";
import{decodeFactIndexV1,encodeFactIndexV1,openFactIndexV1}from"../../🧮️fact-codec/🟨️.mjs";
import{emptyFactIndexViewV1,patchFactIndexBytesV1}from"../../🌊️stream-patch/🟨️.mjs";

type Row={kind:string;path:string;fact:unknown};
type Patch={drop:string[];upserts:Map<string,Row[]>;append:Map<string,Row[]>};
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),LIMIT=1<<28;

const lcg=(seed:number)=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const FACTS:unknown[]=[{size:1},{size:2,digest:"a".repeat(64)},"text",null,[1,2,{key:"value"}],{nested:{list:[true,false,-3,1.5]}},42,{wide:"é€𝄞"}];

function pool(count:number):Row[]{return Array.from({length:count},(_,at)=>({kind:["file","exists","analysis"][at%3]!,path:"p/"+at+(at%7===0?"é𝄞":""),fact:at%4===0?FACTS[at%FACTS.length]:{n:at%13}}));}

const consumers=(index:{owners:string[];rows:Row[];classes:{r:number[];o:number[]}[]})=>Object.fromEntries(factConsumersV1(index as never).filter(([,names])=>names.length).map(([key,names])=>[JSON.parse(key)[1] as string,names]));

async function viaStream(raw:Buffer,patch:Patch):Promise<Buffer>{return patchFactIndexBytesV1(openFactIndexV1(raw,{maximumBytes:LIMIT}),patch,{maximumBytes:LIMIT});}

test("seeded random patches give the canonical bytes of the in-memory patch, and its consumers equal the Python model",async()=>{
 const random=lcg(11);
 for(let world=0;world<60;world++){
  const owners=1+Math.floor(random()*10),rows=pool(40),pick=(share:number)=>rows.filter(()=>random()<share),table=new Map<string,Row[]>(),script:{owners:Record<string,string[]>;steps:unknown[]}={owners:{},steps:[]};
  for(let owner=0;owner<owners;owner++){table.set("o"+owner,pick(0.3));script.owners["o"+owner]=table.get("o"+owner)!.map(row=>row.path);}
  let raw=encodeFactIndexV1(createFactIndexV1(table),{maximumBytes:LIMIT}),reference=decodeFactIndexV1(raw);
  for(let step=0;step<7;step++){
   const kind=random(),name="o"+Math.floor(random()*(owners+2)),patch:Patch={drop:[],upserts:new Map(),append:new Map()},entry:{drop?:string[];upsert?:Record<string,string[]>;append?:Record<string,string[]>}={};
   if(kind<0.45){const picked=pick(0.3);table.set(name,picked);patch.upserts.set(name,picked);entry.upsert={[name]:picked.map(row=>row.path)};}
   else if(kind<0.7){table.delete(name);patch.drop.push(name);entry.drop=[name];}
   else if(kind<0.9&&table.has(name)){const extra=pick(0.2);table.set(name,[...table.get(name)!,...extra]);patch.append.set(name,extra);entry.append={[name]:extra.map(row=>row.path)};}
   reference=patchFactIndexV1(reference as never,patch) as never;
   raw=await viaStream(raw,patch);
   script.steps.push(entry);
   const decoded=decodeFactIndexV1(raw,{maximumBytes:LIMIT});
   expect(Buffer.compare(encodeFactIndexV1(decoded,{maximumBytes:LIMIT}),raw),"canonical bytes in world "+world+" step "+step).toBe(0);
   expect(consumers(decoded),"consumers in world "+world+" step "+step).toEqual(consumers(reference));
   expect(new Set(decoded.owners)).toEqual(new Set(reference.owners));
   expect(decoded.rows.length).toBe(reference.rows.length);
  }
  if(python&&world%6===0){
   const directory=mkdtempSync(join(tmpdir(),"stream-patch-"));
   try{const path=join(directory,"script.json");writeFileSync(path,JSON.stringify(script));const done=Bun.spawnSync([python,"-I",oracle,"consumers",path]);expect(done.exitCode).toBe(0);const model=JSON.parse(done.stdout.toString()) as {owners:string[];consumers:Record<string,string[]>};const decoded=decodeFactIndexV1(raw,{maximumBytes:LIMIT});expect(new Set(decoded.owners)).toEqual(new Set(model.owners));expect(consumers(decoded)).toEqual(model.consumers);}
   finally{rmSync(directory,{recursive:true,force:true});}
  }
 }
},120000);

test("a cold build is the patch of the empty index and rows read in batches equal rows given as arrays",async()=>{
 const rows=pool(300),random=lcg(5),table=new Map<string,Row[]>();
 for(let owner=0;owner<40;owner++)table.set("o"+String(owner).padStart(2,"0"),rows.filter(()=>random()<(owner%5===0?0.8:0.1)));
 const reference=encodeFactIndexV1(createFactIndexV1(table),{maximumBytes:LIMIT}),cold=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:table},{maximumBytes:LIMIT});
 expect(consumers(decodeFactIndexV1(cold,{maximumBytes:LIMIT}))).toEqual(consumers(decodeFactIndexV1(reference,{maximumBytes:LIMIT})));
 const batched=new Map([...table].map(([name,list])=>[name,{length:list.length,*[Symbol.iterator](){yield*list;},async*batches(){for(let at=0;at<list.length;at+=7)yield list.slice(at,at+7);}}]));
 const lazy=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:batched as never},{maximumBytes:LIMIT});
 expect(Buffer.compare(lazy,cold)).toBe(0);
 const next=await patchFactIndexBytesV1(openFactIndexV1(cold,{maximumBytes:LIMIT}),{drop:["o01"],upserts:new Map([["o02",rows.slice(0,50)]])},{maximumBytes:LIMIT});
 expect(decodeFactIndexV1(next,{maximumBytes:LIMIT}).owners).not.toContain("o01");
},60000);

test("patching refines an index whose classes outgrow its owners, like the in-memory refinement",async()=>{
 const rows=pool(2400),table=new Map<string,Row[]>(),random=lcg(3);
 for(let owner=0;owner<30;owner++)table.set("o"+String(owner).padStart(2,"0"),rows.filter(()=>random()<0.02));
 let raw=await patchFactIndexBytesV1(emptyFactIndexViewV1(),{upserts:table},{maximumBytes:LIMIT});
 for(let round=0;round<25;round++){
  const name="o"+String(round%30).padStart(2,"0"),picked=rows.filter(()=>random()<0.02);table.set(name,picked);
  raw=await viaStream(raw,{drop:[],upserts:new Map([[name,picked]]),append:new Map()});
 }
 const decoded=decodeFactIndexV1(raw,{maximumBytes:LIMIT}),naive=new Map<string,string[]>();
 for(const[name,list]of table)for(const row of list)naive.set(row.path,[...(naive.get(row.path)??[]),name]);
 expect(consumers(decoded)).toEqual(Object.fromEntries([...naive].map(([path,names])=>[path,[...new Set(names)].sort()])));
 expect(decoded.classes.length).toBeLessThanOrEqual(decoded.owners.length*2+1024+decoded.rows.length);
},60000);

test("patching owners whose rows arrive in batches holds a few bytes per owner-row incidence, not a list per owner",()=>{
 const owners=400,pooled=60000,per=15000,script=`
  const codec=await import(${JSON.stringify(new URL("../../🧮️fact-codec/🟨️.mjs",import.meta.url).href)}),patch=await import(${JSON.stringify(new URL("../../🌊️stream-patch/🟨️.mjs",import.meta.url).href)});
  const rows=Array.from({length:${pooled}},(_,at)=>({kind:"file",path:"p/"+at,fact:{size:at%97}}));
  const base=new Map([["base",rows.slice(0,100)]]),empty=await patch.patchFactIndexBytesV1(patch.emptyFactIndexViewV1(),{upserts:base},{maximumBytes:2**28});
  const view=codec.openFactIndexV1(empty,{maximumBytes:2**28}),upserts=new Map();
  for(let owner=0;owner<${owners};owner++){const start=(owner%40)*${pooled/40};upserts.set("o"+String(owner).padStart(4,"0"),{length:${per},async*batches(){for(let at=0;at<${per};at+=2000)yield rows.slice(start+at%${pooled-per},start+at%${pooled-per}+Math.min(2000,${per}-at));}});}
  Bun.gc(true);const before=process.resourceUsage().maxRSS,out=await patch.patchFactIndexBytesV1(view,{upserts},{maximumBytes:2**28});
  console.log(JSON.stringify({growth:(process.resourceUsage().maxRSS-before)/1024,incidences:${owners}*${per},bytes:out.length}));`;
 const result=Bun.spawnSync([process.execPath,"-e",script]),found=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {growth:number;incidences:number;bytes:number};
 expect(result.exitCode).toBe(0);
 expect(found.growth*1024/found.incidences).toBeLessThan(2);
},120000);
