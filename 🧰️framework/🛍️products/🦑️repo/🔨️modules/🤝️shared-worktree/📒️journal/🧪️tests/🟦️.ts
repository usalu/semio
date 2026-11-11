import {afterAll,describe,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {requireTestToolV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {appendEventsV1,blobPathV1,reclaimStagingV1,readEventSpansV1,streamMembersV1,foldEventsV1,formatHlcV1,listOriginsV1,mergeClockV1,parseHlcV1,readEventsV1,readHeadV1,readNewestSnapshotV1,requireEventV1,requireOriginIdV1,storePathsV1,tickClockV1,winsV1,writeBlobV1,writeSnapshotV1,type FleetEventV1,type FleetManifestEntryV1} from "../🟦️.ts";

type Clock={ms:number;counter:number};
const cases=fixture as unknown as {
 tick:{id:string;last:Clock;wallMs:number;expect:Clock|null}[];
 merge:{id:string;last:Clock;received:string;wallMs:number;expect:Clock|null}[];
 format:{id:string;clock:Clock;originId:string;hlc:string}[];
 parse:{id:string;hlc:string;expect:{ms:number;counter:number;originId:string}|null}[];
 wins:{id:string;candidate:string;current:string|null;expect:boolean}[];
 origins:{valid:string[];invalid:string[]};
 valid:{id:string;value:FleetEventV1}[];
 invalid:{id:string;value:unknown}[];
 fold:{id:string;initial:Record<string,FleetManifestEntryV1>;events:FleetEventV1[];manifest:Record<string,FleetManifestEntryV1>;winners:number[]}[];
};
const roots:string[]=[];
const temporary=():string=>{const root=mkdtempSync(join(tmpdir(),"journal-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sha=(text:string):string=>createHash("sha256").update(text).digest("hex");
const put=(origin:string,ms:number,counter:number,path:string,content="x"):FleetEventV1=>({version:1,hlc:formatHlcV1({ms,counter},origin),operation:"put",path,sha256:sha(content),bytes:content.length,executable:false,base:null});
const link=(origin:string,ms:number,counter:number,path:string,target:string):FleetEventV1=>({version:1,hlc:formatHlcV1({ms,counter},origin),operation:"link",path,target,base:null});
const attempt=(run:()=>unknown):unknown=>{try{return run();}catch{return null;}};
const walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?walk(join(directory,entry.name)):[join(directory,entry.name)]);

describe("journal fixture vectors",()=>{
 for(const row of cases.tick)test(`tick ${row.id}`,()=>expect(attempt(()=>tickClockV1(row.last,row.wallMs))).toEqual(row.expect));
 for(const row of cases.merge)test(`merge ${row.id}`,()=>expect(attempt(()=>mergeClockV1(row.last,row.received,row.wallMs))).toEqual(row.expect));
 for(const row of cases.format)test(`format ${row.id}`,()=>{
  expect(formatHlcV1(row.clock,row.originId)).toBe(row.hlc);
  expect(parseHlcV1(row.hlc)).toEqual({...row.clock,originId:row.originId});
 });
 for(const row of cases.parse)test(`parse ${row.id}`,()=>expect(attempt(()=>parseHlcV1(row.hlc))).toEqual(row.expect));
 for(const row of cases.wins)test(`wins ${row.id}`,()=>expect(winsV1(row.candidate,row.current??undefined)).toBe(row.expect));
 for(const origin of cases.origins.valid)test(`origin ${origin} is valid`,()=>expect(requireOriginIdV1(origin)).toBe(origin));
 for(const origin of cases.origins.invalid)test(`origin ${JSON.stringify(origin)} is refused`,()=>expect(()=>requireOriginIdV1(origin)).toThrow());
 for(const row of cases.valid)test(`event ${row.id} is admitted`,()=>expect(requireEventV1(row.value)).toEqual(row.value));
 for(const row of cases.invalid)test(`event ${row.id} is refused`,()=>expect(()=>requireEventV1(row.value)).toThrow());
 for(const row of cases.fold)test(`fold ${row.id}`,()=>{
  const manifest=new Map<string,FleetManifestEntryV1>(Object.entries(row.initial)),won=foldEventsV1(manifest,row.events);
  expect(Object.fromEntries(manifest)).toEqual(row.manifest);
  expect(won).toEqual(row.winners.map(index=>row.events[index]!));
 });
 test("fold is independent of the arrival order",()=>{
  for(const row of cases.fold){
   const forward=new Map<string,FleetManifestEntryV1>(Object.entries(row.initial)),backward=new Map<string,FleetManifestEntryV1>(Object.entries(row.initial));
   foldEventsV1(forward,row.events);foldEventsV1(backward,[...row.events].reverse());
   expect(Object.fromEntries(backward)).toEqual(Object.fromEntries(forward));
  }
 });
 test("fixture events and entries satisfy the closed schema",()=>{
  for(const row of cases.fold){
   for(const entry of Object.values(row.manifest))expect(validateJsonSchemaSubset({$ref:"#/definitions/ManifestEntry"},entry,schema)).toEqual([]);
  }
 });
 test("the python oracle agrees",()=>{
  const result=spawnSync(requireTestToolV1("python"),[join(import.meta.dir,"🐍️.py")],{encoding:"utf8"});
  expect(result.stderr+result.stdout).toContain("ok");
  expect(result.status).toBe(0);
 });
});

describe("journal store",()=>{
 test("a missing store reads as empty",async()=>{
  const root=join(temporary(),"absent");
  expect(await readHeadV1(root,"alpha")).toBeUndefined();
  expect(await listOriginsV1(root)).toEqual([]);
  expect(await readEventsV1(root,"alpha",undefined)).toEqual({events:[],cursor:{segment:0,bytes:0}});
  expect(await readNewestSnapshotV1(root)).toBeUndefined();
 });

 test("store paths and blob paths are fixed",()=>{
  const paths=storePathsV1("/s"),hash=sha("x");
  expect(paths).toEqual({root:join("/s","v1"),origins:join("/s","v1","origins"),blobs:join("/s","v1","blobs"),tmp:join("/s","v1","tmp"),snapshots:join("/s","v1","snapshots")});
  expect(blobPathV1("/s",hash)).toBe(join(paths.blobs,hash.slice(0,2),hash));
  expect(()=>blobPathV1("/s","ABC")).toThrow();
 });

 test("blobs are written once and verified",async()=>{
  const root=temporary(),source=join(root,"source.txt");
  writeFileSync(source,"hello");
  const hash=sha("hello"),store=join(root,"store");
  expect(await writeBlobV1(store,"alpha",hash,source)).toBe(true);
  expect(await writeBlobV1(store,"beta",hash,source)).toBe(false);
  expect(readFileSync(blobPathV1(store,hash),"utf8")).toBe("hello");
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
  await expect(writeBlobV1(store,"alpha",sha("other"),source)).rejects.toThrow();
  expect(readdirSync(storePathsV1(store).tmp)).toEqual([]);
 });

 test("append and read round trip with cursor resume",async()=>{
  const store=join(temporary(),"store"),first=[put("alpha",1788000000000,0,"a.txt"),put("alpha",1788000000000,1,"b.txt","y")];
  const head=await appendEventsV1(store,"alpha",first);
  expect(head).toEqual({version:1,segment:0,bytes:head.bytes,events:2,hlc:first[1]!.hlc});
  expect(head.bytes).toBe(readFileSync(join(storePathsV1(store).origins,"alpha","segments","000000000000.jsonl")).length);
  expect(await readHeadV1(store,"alpha")).toEqual(head);
  const all=await readEventsV1(store,"alpha",undefined);
  expect(all.events).toEqual(first);
  expect(all.cursor).toEqual({segment:0,bytes:head.bytes});
  expect((await readEventsV1(store,"alpha",all.cursor)).events).toEqual([]);
  const second=[link("alpha",1788000000001,0,"l","a.txt")],next=await appendEventsV1(store,"alpha",second);
  expect(next.events).toBe(3);
  const resumed=await readEventsV1(store,"alpha",all.cursor);
  expect(resumed.events).toEqual(second);
  expect(resumed.cursor).toEqual({segment:0,bytes:next.bytes});
  expect(await listOriginsV1(store)).toEqual(["alpha"]);
 });

 test("append refuses invalid events and foreign origins",async()=>{
  const store=join(temporary(),"store");
  await expect(appendEventsV1(store,"alpha",[put("beta",1788000000000,0,"a.txt")])).rejects.toThrow();
  await expect(appendEventsV1(store,"alpha",[{...put("alpha",1788000000000,0,"a.txt"),path:"../a"}])).rejects.toThrow();
  await expect(appendEventsV1(store,"alpha",[])).rejects.toThrow();
  await expect(appendEventsV1(store,"Alpha",[])).rejects.toThrow();
  expect(await readHeadV1(store,"alpha")).toBeUndefined();
 });

 test("segments seal at 4096 events and reads cross segments from any cursor",async()=>{
  const store=join(temporary(),"store"),events=Array.from({length:4100},(_,index)=>put("alpha",1788000000000,index,`f/${index}.txt`));
  const head=await appendEventsV1(store,"alpha",events);
  expect(head.segment).toBe(1);
  expect(head.events).toBe(4);
  const directory=join(storePathsV1(store).origins,"alpha","segments");
  expect(readdirSync(directory).sort()).toEqual(["000000000000.jsonl","000000000001.jsonl"]);
  expect(readFileSync(join(directory,"000000000000.jsonl"),"utf8").split("\n").length-1).toBe(4096);
  const all=await readEventsV1(store,"alpha",undefined);
  expect(all.events).toEqual(events);
  expect(all.cursor).toEqual({segment:1,bytes:head.bytes});
  const middle=await readEventsV1(store,"alpha",{segment:0,bytes:0});
  expect(middle.events.length).toBe(4100);
  const tail=await readEventsV1(store,"alpha",{segment:1,bytes:0});
  expect(tail.events).toEqual(events.slice(4096));
  const more=[put("alpha",1788000000001,0,"later.txt")];
  await appendEventsV1(store,"alpha",more);
  expect((await readEventsV1(store,"alpha",all.cursor)).events).toEqual(more);
  const old=await readEventsV1(store,"alpha",{segment:0,bytes:readFileSync(join(directory,"000000000000.jsonl")).length});
  expect(old.events.length).toBe(5);
 });

 test("segments seal after 4 MiB",async()=>{
  const store=join(temporary(),"store"),target="t".repeat(4000),events=Array.from({length:1100},(_,index)=>link("alpha",1788000000000,index,`l${index}`,target));
  const head=await appendEventsV1(store,"alpha",events);
  expect(head.segment).toBe(1);
  const first=statSync(join(storePathsV1(store).origins,"alpha","segments","000000000000.jsonl")).size;
  expect(first).toBeGreaterThan(3*1024*1024);
  expect(first).toBeLessThanOrEqual(4*1024*1024+8192);
  expect((await readEventsV1(store,"alpha",undefined)).events.length).toBe(1100);
 });

 test("bytes beyond head.bytes are ignored and overwritten",async()=>{
  const store=join(temporary(),"store"),first=[put("alpha",1788000000000,0,"a.txt")],directory=join(storePathsV1(store).origins,"alpha","segments"),file=join(directory,"000000000000.jsonl");
  const head=await appendEventsV1(store,"alpha",first);
  appendFileSync(file,`${JSON.stringify(put("alpha",1788000000000,1,"unpublished.txt"))}\n{"version":1,"hl`);
  const seen=await readEventsV1(store,"alpha",undefined);
  expect(seen.events).toEqual(first);
  expect(seen.cursor).toEqual({segment:0,bytes:head.bytes});
  const second=[put("alpha",1788000000001,0,"b.txt")],next=await appendEventsV1(store,"alpha",second);
  expect(readFileSync(file,"utf8")).toBe([...first,...second].map(event=>`${JSON.stringify(event)}\n`).join(""));
  expect((await readEventsV1(store,"alpha",seen.cursor)).events).toEqual(second);
  expect(next.events).toBe(2);
 });

 test("a torn line inside the published bytes is a complete-line boundary",async()=>{
  const store=join(temporary(),"store"),events=[put("alpha",1788000000000,0,"a.txt"),put("alpha",1788000000000,1,"b.txt")];
  const head=await appendEventsV1(store,"alpha",events);
  const file=join(storePathsV1(store).origins,"alpha","segments","000000000000.jsonl");
  const full=readFileSync(file);
  writeFileSync(file,full.subarray(0,full.length-5));
  writeFileSync(join(storePathsV1(store).origins,"alpha","head.json"),JSON.stringify(head));
  const seen=await readEventsV1(store,"alpha",undefined);
  expect(seen.events).toEqual([events[0]!]);
  expect(seen.cursor.bytes).toBe(`${JSON.stringify(events[0])}\n`.length);
 });

 test("a corrupt published line is reported",async()=>{
  const store=join(temporary(),"store");
  await appendEventsV1(store,"alpha",[put("alpha",1788000000000,0,"a.txt")]);
  const origin=join(storePathsV1(store).origins,"alpha"),file=join(origin,"segments","000000000000.jsonl"),text="not json\n";
  writeFileSync(file,text);
  writeFileSync(join(origin,"head.json"),JSON.stringify({version:1,segment:0,bytes:text.length,events:1,hlc:formatHlcV1({ms:1788000000000,counter:0},"alpha")}));
  await expect(readEventsV1(store,"alpha",undefined)).rejects.toThrow();
 });

 test("two origins interleave into one deterministic state",async()=>{
  const store=join(temporary(),"store"),ms=1788000000000;
  await appendEventsV1(store,"alpha",[put("alpha",ms,0,"shared.txt","from alpha"),put("alpha",ms+5,0,"only-alpha.txt")]);
  await appendEventsV1(store,"beta",[put("beta",ms,0,"shared.txt","from beta")]);
  await appendEventsV1(store,"alpha",[put("alpha",ms+10,0,"shared.txt","alpha again")]);
  await appendEventsV1(store,"beta",[put("beta",ms+10,0,"shared.txt","beta again")]);
  expect(await listOriginsV1(store)).toEqual(["alpha","beta"]);
  const gathered:FleetEventV1[]=[];
  for(const origin of await listOriginsV1(store))gathered.push(...(await readEventsV1(store,origin,undefined)).events);
  const forward=new Map<string,FleetManifestEntryV1>(),backward=new Map<string,FleetManifestEntryV1>();
  foldEventsV1(forward,gathered);foldEventsV1(backward,[...gathered].reverse());
  expect(Object.fromEntries(backward)).toEqual(Object.fromEntries(forward));
  expect(forward.get("shared.txt")?.sha256).toBe(sha("beta again"));
  expect(forward.get("shared.txt")?.hlc).toBe(formatHlcV1({ms:ms+10,counter:0},"beta"));
  expect(forward.get("only-alpha.txt")?.operation).toBe("put");
 });

 test("snapshots round trip and the newest across origins wins",async()=>{
  const store=join(temporary(),"store"),manifest=new Map<string,FleetManifestEntryV1>();
  foldEventsV1(manifest,[put("alpha",1788000000000,0,"a.txt"),link("alpha",1788000000001,0,"l","a.txt"),{version:1,hlc:formatHlcV1({ms:1788000000002,counter:0},"alpha"),operation:"delete",path:"gone.txt",base:null},put("alpha",1788000000003,0,"é/ü.txt")]);
  const cursors={alpha:{segment:0,bytes:10},beta:{segment:2,bytes:0}};
  const older=formatHlcV1({ms:1788000000100,counter:0},"alpha"),newer=formatHlcV1({ms:1788000000100,counter:1},"beta");
  const file=await writeSnapshotV1(store,"alpha",older,manifest,cursors);
  expect(file).toBe(join(storePathsV1(store).snapshots,"alpha",`${older}.json`));
  const one=await readNewestSnapshotV1(store);
  expect(one?.hlc).toBe(older);
  expect(Object.fromEntries(one!.manifest)).toEqual(Object.fromEntries(manifest));
  expect(one?.cursors).toEqual(cursors);
  const smaller=new Map(manifest);smaller.delete("a.txt");
  await writeSnapshotV1(store,"beta",newer,smaller,{});
  const two=await readNewestSnapshotV1(store);
  expect(two?.hlc).toBe(newer);
  expect(two?.manifest.has("a.txt")).toBe(false);
  expect(two?.cursors).toEqual({});
  const origin=join(storePathsV1(store).snapshots,"beta");
  writeFileSync(join(origin,`${formatHlcV1({ms:1788000000200,counter:0},"beta")}.json`),"{broken");
  expect((await readNewestSnapshotV1(store))?.hlc).toBe(newer);
 });

 test("a big document streams member by member across chunk boundaries and any formatting",async()=>{
  const root=temporary(),file=join(root,"big.json"),hostile=(index:number):string=>`dir/${"é🧬️".repeat(index%7)}f${index}"\\{[,}]`;
  const members=Object.fromEntries(Array.from({length:3000},(_,index)=>[hostile(index),{hlc:formatHlcV1({ms:1788000000000+index,counter:0},"alpha"),operation:"link",target:`a,b}{${index}\\"]`}]));
  const document={version:1,hlc:"h",manifest:members,cursors:{alpha:{segment:1,bytes:2}},empty:{}};
  for(const text of [JSON.stringify(document),JSON.stringify(document,null,2),JSON.stringify(document,null,"\t").replace(/\n/gu,"\r\n")]){
   writeFileSync(file,text);
   for(const chunk of [7,4096,1<<20]){
    const seen:Record<string,unknown>={},top:Record<string,unknown>={},opened:string[]=[];
    await streamMembersV1(file,new Set(["manifest","empty"]),(outer,inner,piece)=>{if(inner===undefined)top[outer]=JSON.parse(piece);else seen[inner]=JSON.parse(piece);},outer=>opened.push(outer),chunk);
    expect(seen).toEqual(members);
    expect(top).toEqual({version:1,hlc:"h",cursors:{alpha:{segment:1,bytes:2}}});
    expect(opened).toEqual(["manifest","empty"]);
   }
  }
  for(const broken of ["[1]","{","{\"a\" 1}","{\"a\":1} x","{\"a\":{\"b\":1}","",'{"a":1,}x'])
   {writeFileSync(file,broken);await expect(streamMembersV1(file,new Set(["a"]),()=>{})).rejects.toThrow("Closed JSON object required");}
 });

 test("a long journal is read one segment at a time and the cursors chain without gaps",async()=>{
  const store=join(temporary(),"store"),events=Array.from({length:9000},(_,index)=>put("alpha",1788000000000+index,0,`f/${index}.txt`));
  await appendEventsV1(store,"alpha",events);
  const head=(await readHeadV1(store,"alpha"))!;
  expect(head.segment).toBeGreaterThanOrEqual(2);
  const seen:FleetEventV1[]=[];
  let cursor:{segment:number;bytes:number}|undefined,calls=0;
  for(;;){
   const read=await readEventSpansV1(store,"alpha",cursor,1);
   seen.push(...read.events);calls++;
   if(cursor!==undefined&&read.cursor.segment===cursor.segment&&read.cursor.bytes===cursor.bytes)break;
   cursor=read.cursor;
  }
  expect(seen.map(event=>event.path)).toEqual(events.map(event=>event.path));
  expect(cursor).toEqual({segment:head.segment,bytes:head.bytes});
  expect(calls).toBeGreaterThanOrEqual(head.segment+1);
 });

 test("every file name in a store is ASCII",async()=>{
  const root=temporary(),store=join(root,"store"),source=join(root,"é-🧬️.txt");
  writeFileSync(source,"z");
  await writeBlobV1(store,"alpha",sha("z"),source);
  await appendEventsV1(store,"alpha",[put("alpha",1788000000000,0,"🧬️/é.txt","z")]);
  await writeSnapshotV1(store,"alpha",formatHlcV1({ms:1788000000001,counter:0},"alpha"),new Map(),{});
  const names=walk(store).map(file=>file.slice(store.length+1));
  expect(names.length).toBeGreaterThanOrEqual(4);
  for(const name of names)expect(/^[\x20-\x7e]+$/u.test(name)).toBe(true);
 });
 test("a store root that was seen is never rebuilt as a local directory after the share vanished",async()=>{
  const root=temporary(),store=join(root,"store"),source=join(root,"a.txt");
  writeFileSync(source,"z");
  await writeBlobV1(store,"alpha",sha("z"),source);
  renameSync(join(store,"v1"),join(store,"away"));
  await expect(writeBlobV1(store,"alpha",sha("z"),source)).rejects.toThrow();
  await expect(appendEventsV1(store,"alpha",[put("alpha",1788000000000,0,"a.txt","z")])).rejects.toThrow();
  await expect(writeSnapshotV1(store,"alpha",formatHlcV1({ms:1788000000001,counter:0},"alpha"),new Map(),{})).rejects.toThrow();
  expect(existsSync(join(store,"v1"))).toBe(false);
  renameSync(join(store,"away"),join(store,"v1"));
  await appendEventsV1(store,"alpha",[put("alpha",1788000000000,0,"a.txt","z")]);
  expect((await readHeadV1(store,"alpha"))?.events).toBe(1);
 });
 test("a cached origin list notices a new origin directory at once and a removed one",async()=>{
  const root=temporary(),store=join(root,"store"),source=join(root,"a.txt");
  writeFileSync(source,"z");
  await writeBlobV1(store,"alpha",sha("z"),source);
  await appendEventsV1(store,"alpha",[put("alpha",1788000000000,0,"a.txt","z")]);
  expect(await listOriginsV1(store)).toEqual(["alpha"]);
  expect(await listOriginsV1(store)).toEqual(["alpha"]);
  await appendEventsV1(store,"beta",[put("beta",1788000000001,0,"b.txt","z")]);
  expect(await listOriginsV1(store)).toEqual(["alpha","beta"]);
  rmSync(join(store,"v1","origins","beta"),{recursive:true});
  expect(await listOriginsV1(store)).toEqual(["alpha"]);
 });
});

describe("staging files that a killed process left in the store",()=>{
 const hex="0123456789abcdef";
 test("the own origin's leftovers of dead processes and old names go at once, those of a live process, of other origins and of the relay stay",async()=>{
  const store=temporary(),{tmp}=storePathsV1(store),child=spawnSync(process.execPath,["--version"]),dead=child.pid!;
  mkdirSync(tmp,{recursive:true});
  const files=[`a-${dead}-${hex}`,`a-${hex}`,`a-${process.pid}-${hex}`,`b-${dead}-${hex}`,`relay-${hex}`,`a-bootstrap-${hex}`];
  for(const name of files)writeFileSync(join(tmp,name),"half");
  expect(await reclaimStagingV1(store,{originId:"a"})).toEqual({files:2,bytes:8});
  expect(readdirSync(tmp).sort()).toEqual(files.slice(2).sort());
 });
 test("by age every file goes whose newest timestamp is older than the limit, except placeholders of the NFS client, and a store without tmp stays untouched",async()=>{
  const store=temporary(),{tmp}=storePathsV1(store);
  expect(await reclaimStagingV1(store,{olderThanMilliseconds:1})).toEqual({files:0,bytes:0});expect(existsSync(tmp)).toBe(false);
  mkdirSync(tmp,{recursive:true});
  for(const name of [`relay-${hex}`,`c-1-${hex}`,".nfs00ff"])writeFileSync(join(tmp,name),"half");
  expect(await reclaimStagingV1(store,{olderThanMilliseconds:3_600_000,now:Date.now()+1000})).toEqual({files:0,bytes:0});
  expect(await reclaimStagingV1(store,{olderThanMilliseconds:3_600_000,now:Date.now()+2*3_600_000})).toEqual({files:2,bytes:8});
  expect(readdirSync(tmp)).toEqual([".nfs00ff"]);
 });
});
