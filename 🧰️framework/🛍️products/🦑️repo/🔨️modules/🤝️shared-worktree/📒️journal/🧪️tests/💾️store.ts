import {afterAll,describe,expect,mock,test} from "bun:test";
import * as real from "node:fs/promises";
import {spawnSync} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const syncs={count:0,on:false},originals={...real};
mock.module("node:fs/promises",()=>({...originals,open:async(...args:unknown[])=>{
 const handle=await (originals.open as unknown as (...inner:unknown[])=>Promise<{sync:()=>Promise<void>}>)(...args);
 if(!syncs.on)return handle;
 const sync=handle.sync.bind(handle);
 handle.sync=()=>{syncs.count++;return sync();};
 return handle;
}}));
const journal=await import("../🟦️.ts");
const {appendEventsV1,blobPathV1,copyHashedV1,copyVerifiedV1,formatHlcV1,isEventV1,isHeadV1,isManifestEntryV1,isPathV1,largeBlobBytesV1,publishFlatHeadV1,readEventSpansV1,readFlatHeadsV1,readHeadV1,requireEventV1,storeBlobV1,storePathsV1,writeBlobV1}=journal;

const roots:string[]=[];
const temporary=():string=>{const root=mkdtempSync(join(tmpdir(),"journal-store-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sha=(data:string|Buffer):string=>createHash("sha256").update(data).digest("hex");
const put=(origin:string,counter:number,path:string,content="x") => ({version:1 as const,hlc:formatHlcV1({ms:1788000000000,counter},origin),operation:"put" as const,path,sha256:sha(content),bytes:content.length,executable:false,base:null});
const python=requireTestToolV1("python"),ajv=new Ajv({strict:true,allowUnionTypes:true});
const ajvOf=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});
const generic=(definition:string)=>compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);

let seed=20261113;
const random=():number=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/2**32;};
const pick=<T>(items:readonly T[]):T=>items[Math.floor(random()*items.length)]!;
const hexa=sha("a"),hlcs=["1788000000000-000000-alpha","0000000000000-000000-a","1788000000000-000001-a0-b-"+"c".repeat(36),"1788000000000-000000-"+"c".repeat(40)];
const pathPool=["a.txt","dir/sub/file.txt","🧬️/é.txt","x".repeat(1024),"😀".repeat(512),"a b/c d"];
const junk:unknown[]=[null,true,false,0,1,2,-1,1.5,9007199254740991,9007199254740992,1e21,"","a",".","..","a//b","/a","a/","a/.","./a","a/..","a\\b","a\u0000b","é","x".repeat(1025),"😀".repeat(513),"\ud800",hexa.toUpperCase(),hexa.slice(1),hexa+"0","0".repeat(64),"1788000000000-000000-Alpha","1788000000000-00000-a","1788000000000-000000-","1788000000000-000000-"+"c".repeat(41),"put","delete","link","1",[],{},[1],{a:1},"t".repeat(4096),"t".repeat(4097),"😀".repeat(2048),"😀".repeat(2049)];
const makeEvent=():Record<string,unknown>=>{
 const base={version:1,hlc:pick(hlcs),path:pick(pathPool),base:random()<0.5?null:hexa},kind=pick(["put","delete","link"]);
 if(kind==="put")return {...base,operation:"put",sha256:hexa,bytes:pick([0,1,12345,9007199254740991]),executable:random()<0.5};
 if(kind==="link")return {...base,operation:"link",target:pick(["t","../x/y","t".repeat(4096)])};
 return {...base,operation:"delete"};
};
const makeEntry=():Record<string,unknown>=>{
 const kind=pick(["put","delete","link"]);
 if(kind==="put")return {hlc:pick(hlcs),operation:"put",sha256:hexa,bytes:pick([0,7]),executable:random()<0.5};
 if(kind==="link")return {hlc:pick(hlcs),operation:"link",target:"t"};
 return {hlc:pick(hlcs),operation:"delete"};
};
const makeHead=():Record<string,unknown>=>({version:1,segment:pick([0,3]),bytes:pick([0,99]),events:pick([0,5]),hlc:pick(hlcs)});
const mutate=(value:Record<string,unknown>):Record<string,unknown>=>{
 const copy={...value};
 for(let round=Math.floor(random()*4);round>0;round--){
  const keys=Object.keys(copy),action=random();
  if(action<0.55&&keys.length>0)copy[pick(keys)]=pick(junk);
  else if(action<0.8&&keys.length>0)delete copy[pick(keys)];
  else if(action<0.9)copy[pick(["extra","sha256","target","bytes","executable","base","operation"])]=pick(junk);
  else copy.operation=pick(["put","delete","link","move",undefined]);
 }
 return copy;
};
const ndjson=(items:readonly unknown[]):string=>items.map(item=>JSON.stringify(item)).join("\n")+"\n";
const accepts=(run:()=>unknown):boolean=>{try{run();return true;}catch{return false;}};

describe("hand validators have the verdict of the schema",()=>{
 const eventAjv=ajvOf("Event"),eventGeneric=generic("Event"),entryAjv=ajvOf("ManifestEntry"),entryGeneric=generic("ManifestEntry"),headAjv=ajvOf("Head"),headGeneric=generic("Head"),pathAjv=ajvOf("Path"),pathGeneric=generic("Path");
 const cases=[...Array.from({length:30000},()=>random()<0.1?makeEvent():mutate(makeEvent())),...(fixture as unknown as {valid:{value:unknown}[];invalid:{value:unknown}[]}).valid.map(row=>row.value),...(fixture as unknown as {invalid:{value:unknown}[]}).invalid.map(row=>row.value),null,[],"x",1,true] as unknown[];
 test("an event is judged like the generic validator and like Ajv, on 30 000 fuzzed values and the fixture",()=>{
  let valid=0,invalid=0;
  for(const value of cases){
   const hand=isEventV1(value),walker=eventGeneric.validate(value).length===0,library=eventAjv(value)===true;
   if(hand!==walker||hand!==library)throw Error(`verdict differs for ${JSON.stringify(value)}: hand ${hand}, generic ${walker}, ajv ${library}`);
   if(hand)valid++;else invalid++;
  }
  expect(valid).toBeGreaterThan(2000);expect(invalid).toBeGreaterThan(2000);
 });
 test("a manifest entry, a head and a path are judged like the generic validator and like Ajv",()=>{
  for(let index=0;index<10000;index++){
   const entry=random()<0.15?makeEntry():mutate(makeEntry()),head=random()<0.15?makeHead():mutate(makeHead()),path=pick([...pathPool,...junk]);
   expect({entry:isManifestEntryV1(entry),head:isHeadV1(head),path:isPathV1(path)}).toEqual({entry:entryGeneric.validate(entry).length===0,head:headGeneric.validate(head).length===0,path:pathGeneric.validate(path).length===0});
   expect({entry:isManifestEntryV1(entry),head:isHeadV1(head),path:isPathV1(path)}).toEqual({entry:entryAjv(entry)===true,head:headAjv(head)===true,path:pathAjv(path)===true});
  }
 });
 test("admission (schema and relative path) equals the Python oracle on 6 000 fuzzed events",()=>{
  const events=Array.from({length:6000},()=>random()<0.2?makeEvent():mutate(makeEvent())).filter(event=>{try{return JSON.parse(JSON.stringify(event))!==undefined;}catch{return false;}});
  const file=join(temporary(),"events.ndjson");
  writeFileSync(file,ndjson(events));
  const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),"events",file],{encoding:"utf8",maxBuffer:1<<24}),verdicts=result.stdout.trim().split("\n").map(Number);
  expect(result.status).toBe(0);
  expect(verdicts.length).toBe(events.length);
  const mine=events.map(event=>accepts(()=>requireEventV1(JSON.parse(JSON.stringify(event)))));
  const wrong=mine.map((verdict,index)=>verdict===(verdicts[index]===1)?-1:index).filter(index=>index>=0).slice(0,3).map(index=>JSON.stringify(events[index]));
  expect(wrong).toEqual([]);
  expect(mine.filter(Boolean).length).toBeGreaterThan(500);
 });
 test("a refused event keeps the message of the schema error",()=>{
  expect(()=>requireEventV1({...put("alpha",0,"a"),path:"a//b"})).toThrow("Closed fleet event required");
  expect(()=>requireEventV1({...put("alpha",0,"a"),path:"a/../b"})).toThrow("Closed fleet event required: path must stay relative");
 });
});

describe("flat heads",()=>{
 test("every append leaves exactly one name that holds the head, and reads see the later of two",async()=>{
  const store=join(temporary(),"store"),heads=storePathsV1(store).heads;
  const first=await appendEventsV1(store,"alpha",[put("alpha",0,"a.txt")]);
  expect(readdirSync(heads)).toEqual([`alpha_${first.segment}_${first.bytes}_${first.events}_${first.hlc}`]);
  expect((await readFlatHeadsV1(store)).get("alpha")).toEqual(first);
  const second=await appendEventsV1(store,"alpha",[put("alpha",1,"b.txt")]);
  expect(readdirSync(heads)).toEqual([`alpha_${second.segment}_${second.bytes}_${second.events}_${second.hlc}`]);
  expect((await readFlatHeadsV1(store)).get("alpha")).toEqual(second);
  writeFileSync(join(heads,`alpha_0_1_1_${first.hlc}`),"");writeFileSync(join(heads,"README"),"");writeFileSync(join(heads,"alpha_x_1_1_nothing"),"");
  await Bun.sleep(5);
  const merged=await readFlatHeadsV1(store);
  expect([...merged.keys()]).toEqual(["alpha"]);expect(merged.get("alpha")).toEqual(second);
  expect(await readFlatHeadsV1(store)).toBeDefined();
 });
 test("a store nobody writes flat heads into has none, and an origin without one stays readable through head.json",async()=>{
  const store=join(temporary(),"store");
  expect((await readFlatHeadsV1(store)).size).toBe(0);
  await appendEventsV1(store,"alpha",[put("alpha",0,"a.txt")]);
  rmSync(storePathsV1(store).heads,{recursive:true});
  await Bun.sleep(5);
  expect((await readFlatHeadsV1(store)).size).toBe(0);
  expect((await readHeadV1(store,"alpha"))?.events).toBe(1);
 });
 test("an unchanged directory is answered from the cache and a change is noticed",async()=>{
  const store=join(temporary(),"store");
  await appendEventsV1(store,"alpha",[put("alpha",0,"a.txt")]);
  const first=await readFlatHeadsV1(store);
  expect(await readFlatHeadsV1(store)).toBe(first);
  await appendEventsV1(store,"beta",[put("beta",0,"b.txt")]);
  const next=await readFlatHeadsV1(store);
  expect([...next.keys()]).toEqual(["alpha","beta"]);
 });
 test("a flat head that cannot be written does not fail the append",async()=>{
  const store=join(temporary(),"store");
  mkdirSync(storePathsV1(store).root,{recursive:true});writeFileSync(storePathsV1(store).heads,"not a directory");
  const head=await appendEventsV1(store,"alpha",[put("alpha",0,"a.txt")]);
  expect(await readHeadV1(store,"alpha")).toEqual(head);
  expect((await readFlatHeadsV1(store).catch(()=>new Map())).size).toBe(0);
 });
 test("the origin's leftover names from an earlier process are removed by its next publish",async()=>{
  const store=join(temporary(),"store"),heads=storePathsV1(store).heads;
  await appendEventsV1(store,"beta",[put("beta",0,"b.txt")]);
  const stale=[`gamma_0_1_1_${formatHlcV1({ms:1788000000000,counter:9},"gamma")}`,`gamma_0_5_3_${formatHlcV1({ms:1788000000000,counter:7},"gamma")}`];
  for(const name of stale)writeFileSync(join(heads,name),"");
  const head=await appendEventsV1(store,"gamma",[put("gamma",1,"g.txt")]);
  expect(readdirSync(heads).filter(name=>name.startsWith("gamma_"))).toEqual([`gamma_${head.segment}_${head.bytes}_${head.events}_${head.hlc}`]);
  expect(readdirSync(heads).filter(name=>name.startsWith("beta_"))).toHaveLength(1);
  await publishFlatHeadV1(store,"gamma",head);
 });
});

describe("single-pass blobs",()=>{
 const source=(root:string,data:string|Buffer,name="source.bin"):string=>{const file=join(root,name);writeFileSync(file,data);return file;};
 test("a blob is named by the bytes that were written, read once, fsynced once and published once",async()=>{
  const root=temporary(),file=source(root,"hello"),store=join(root,"store");
  const first=await storeBlobV1(store,"alpha",file);
  expect(first).toEqual({sha256:sha("hello"),bytes:5,stable:true,created:true});
  expect(readFileSync(blobPathV1(store,first.sha256),"utf8")).toBe("hello");
  expect(await storeBlobV1(store,"beta",file)).toEqual({sha256:sha("hello"),bytes:5,stable:true,created:false});
  expect(await storeBlobV1(store,"beta",file,{known:{sha256:sha("hello"),bytes:5}})).toEqual({sha256:sha("hello"),bytes:5,stable:true,created:false});
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 });
 test("a source whose bytes differ from the hash the caller holds is reported unstable and nothing is stored",async()=>{
  const root=temporary(),file=source(root,"actual"),store=join(root,"store");
  const result=await storeBlobV1(store,"alpha",file,{known:{sha256:sha("promised"),bytes:8}});
  expect(result).toMatchObject({sha256:sha("actual"),stable:false,created:false});
  expect(existsSync(blobPathV1(store,sha("promised")))).toBe(false);expect(existsSync(blobPathV1(store,sha("actual")))).toBe(false);
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
  await expect(writeBlobV1(store,"alpha",sha("promised"),file)).rejects.toThrow("Fleet blob source changed");
 });
 test("a source that changes while it is copied is never published",async()=>{
  const root=temporary(),data=randomBytes(3<<20),file=source(root,data),store=join(root,"store");
  let touched=false;
  const result=await storeBlobV1(store,"alpha",file,{progress:done=>{if(!touched&&done>0){touched=true;appendFileSync(file,"more");}}});
  expect(touched).toBe(true);expect(result.stable).toBe(false);
  expect(existsSync(join(storePathsV1(store).blobs))?readdirSync(storePathsV1(store).blobs):[]).toEqual([]);
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 });
 test("a blob whose content does not match its name is never applied: the copy is refused and removed",async()=>{
  const root=temporary(),store=join(root,"store"),wanted=sha("original"),target=join(root,"copy");
  mkdirSync(join(store,"v1","blobs",wanted.slice(0,2)),{recursive:true});
  writeFileSync(blobPathV1(store,wanted),"tampered");
  expect(await copyVerifiedV1(blobPathV1(store,wanted),target,wanted)).toBe("corrupt");
  expect(existsSync(target)).toBe(false);
  writeFileSync(blobPathV1(store,wanted),"original");
  expect(await copyVerifiedV1(blobPathV1(store,wanted),target,wanted,{mode:0o755})).toBe("ok");
  expect(readFileSync(target,"utf8")).toBe("original");
  if(process.platform!=="win32")expect(statSync(target).mode&0o777).toBe(0o755);
  await expect(copyVerifiedV1(join(root,"missing"),join(root,"never"),wanted)).rejects.toMatchObject({code:"ENOENT"});
  expect(existsSync(join(root,"never"))).toBe(false);
 });
 test("a copy honours cancellation between chunks and leaves no file",async()=>{
  const root=temporary(),file=source(root,randomBytes(5<<20)),target=join(root,"copy"),controller=new AbortController();
  let reports=0;
  await expect(copyHashedV1(file,target,{signal:controller.signal,progress:(done,total)=>{reports++;expect(total).toBe(5<<20);if(done>=1<<20)controller.abort();}})).rejects.toThrow();
  expect(reports).toBeGreaterThan(0);expect(reports).toBeLessThan(5);
  expect(existsSync(target)).toBe(false);
 });
 test("a file of 64 MiB or more is copied by clone or by stream with the sha256 of an independent tool, in bounded memory, and is stable",async()=>{
  const root=temporary(),data=Buffer.alloc(largeBlobBytesV1+12345,7);
  data.writeUInt32LE(0xabcdef,100);
  const file=source(root,data),expected=spawnSync(python,["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",file],{encoding:"utf8"}).stdout.trim();
  expect(expected).toBe(sha(data));
  for(const clone of [false,true]){
   const target=join(root,`copy-${clone}`),before=process.memoryUsage().rss,reports:number[]=[];
   const result=await copyHashedV1(file,target,{clone,progress:done=>{reports.push(done);}}),grown=process.memoryUsage().rss-before;
   expect(grown).toBeLessThan(data.length);
   expect(result).toEqual({sha256:expected,bytes:data.length,stable:true});
   expect(statSync(target).size).toBe(data.length);expect(readFileSync(target).equals(data)).toBe(true);
   expect(reports.length).toBeGreaterThan(10);expect(reports[reports.length-1]).toBe(data.length);
  }
 });
 test("fsync counts: one per new blob, one per journal append and one per head, none for a blob that exists and none for a plain copy",async()=>{
  const root=temporary(),store=join(root,"store"),files=["a","b","c"].map(name=>source(root,`content ${name}`,name));
  await storeBlobV1(store,"alpha",files[0]!);
  syncs.count=0;syncs.on=true;
  try{
   for(const file of files)await storeBlobV1(store,"alpha",file);
   expect(syncs.count).toBe(2);
   syncs.count=0;
   await appendEventsV1(store,"alpha",files.map((_,index)=>put("alpha",index,`f${index}.txt`)));
   expect(syncs.count).toBe(2);
   syncs.count=0;
   expect(await copyVerifiedV1(files[0]!,join(root,"plain"),sha("content a"))).toBe("ok");
   expect(syncs.count).toBe(0);
   expect(await copyVerifiedV1(files[0]!,join(root,"synced"),sha("content a"),{sync:true})).toBe("ok");
   expect(syncs.count).toBe(1);
  }finally{syncs.on=false;}
 });
});

describe("bounded reads",()=>{
 test("a read with a limit stops behind the last event it returned and the next read continues without gap or repeat",async()=>{
  const store=join(temporary(),"store"),events=Array.from({length:50},(_,index)=>put("alpha",index,`f${index}.txt`));
  await appendEventsV1(store,"alpha",events);
  const seen:string[]=[];
  let cursor:{segment:number;bytes:number}|undefined,calls=0;
  for(;;){
   const read=await readEventSpansV1(store,"alpha",cursor,1,undefined,7);
   calls++;
   expect(read.events.length).toBeLessThanOrEqual(7);
   for(const event of read.events)seen.push(event.path);
   if(read.events.length===0)break;
   cursor=read.cursor;
  }
  expect(seen).toEqual(events.map(event=>event.path));expect(calls).toBe(9);
 });
 test("a head handed in is used instead of head.json",async()=>{
  const store=join(temporary(),"store"),head=await appendEventsV1(store,"alpha",[put("alpha",0,"a.txt"),put("alpha",1,"b.txt")]);
  rmSync(join(storePathsV1(store).origins,"alpha","head.json"));
  expect((await readEventSpansV1(store,"alpha",undefined,undefined,head)).events).toHaveLength(2);
  expect((await readEventSpansV1(store,"alpha",undefined)).events).toHaveLength(0);
 });
});
