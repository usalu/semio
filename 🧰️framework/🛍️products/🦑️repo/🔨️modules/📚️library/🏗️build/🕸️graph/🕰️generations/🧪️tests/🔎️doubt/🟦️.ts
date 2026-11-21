/** 🔎️ Contract: without a journal the fact check costs one stat per distinct directory and file. A structural row under a directory whose stamp is unchanged is vouched for, a changed, new or racy directory and every unstamped row go to the precise check, a rows file edit is caught by its own stat, and the stamps of directories whose rows were stale are forgotten. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import packedStampFixture from "./🪶️packed-stamps.json";
import {createFactIndexV1} from "../../🟨️.mjs";
import {encodeFactIndexV1,openFactIndexV1} from "../../🧮️fact-codec/🟨️.mjs";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
import {createDirectoryStampsV1} from "../../🗂️directory-stamps/🟨️.mjs";
import {createEpochDoubtV1,rowDependenciesV1,selectDoubtfulRowsV1} from "../../🔎️doubt/🟨️.mjs";

const old=(path:string)=>{const past=new Date(Date.now()-60_000);utimesSync(path,past,past);};
const fixture=()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"doubt-")),src=join(root,"src");mkdirSync(src);
 writeFileSync(join(src,"a.ts"),"a");writeFileSync(join(src,"b.ts"),"b");old(join(src,"a.ts"));old(join(src,"b.ts"));old(src);old(root);
 const status=(path:string)=>statSync(path),fileFact=(path:string)=>{const s=status(path);return{size:s.size,mtimeMs:s.mtimeMs,ctimeMs:s.ctimeMs,ino:s.ino,sha256:"0".repeat(64)};};
 const rows=[{kind:"file",path:join(src,"a.ts"),fact:fileFact(join(src,"a.ts"))},{kind:"readdir",path:src,fact:["a.ts","b.ts"]},{kind:"fileExists",path:join(src,"missing.ts"),fact:false},{kind:"exists",path:join(src,"a.ts"),fact:true},{kind:"resolve",path:"[]",fact:"x"}];
 return{root,src,rows,fileFact};
};
const pass=async(rows:readonly unknown[],file:string,stale:ReadonlySet<number>=new Set())=>{
 const doubt=createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}),flags=await doubt.classify(rows as never);
 await doubt.settled({rows:rows as never,ids:rows.map((_,id)=>id),stale});
 return Array.from(flags);
};

test("the first pass doubts every row, the next pass vouches for the unchanged structural and file rows and doubts the rest",async()=>{
 const{root,rows}=fixture(),file=join(root,"stamps.json");
 try{
  expect(await pass(rows,file),"nothing is stamped yet").toEqual([0,1,1,1,1]);
  expect(await pass(rows,file),"stamped: the file by its stat, the listing, the absence and the plain existing entry by their directory; a bare resolution stays doubtful").toEqual([0,0,0,0,1]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a new entry, an edit and a racy directory send their rows to the precise check; stale rows forget their stamps",async()=>{
 const{root,src,rows,fileFact}=fixture(),file=join(root,"stamps.json");
 try{
  await pass(rows,file);
  writeFileSync(join(src,"c.ts"),"c");
  expect(await pass(rows,file),"the directory changed: its listing and absence rows are doubtful").toEqual([0,1,1,1,1]);
  expect(await pass(rows,file),"a directory touched just now is racy and never stamped").toEqual([0,1,1,1,1]);
  old(src);old(join(src,"c.ts"));
  await pass(rows,file);
  writeFileSync(join(src,"a.ts"),"edited");
  const edited=[{...rows[0],fact:rows[0]!.fact},...rows.slice(1)];
  expect((await pass(edited,file))[0],"the edit of a file is its own stat").toBe(1);
  expect(fileFact(join(src,"a.ts")).size).toBe(6);
  old(src);old(join(src,"a.ts"));
  await pass(rows,file,new Set([1]));
  expect(await pass(rows,file),"a stale listing forgot the stamp of its directory").toEqual([1,1,1,1,1]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an analysis is vouched for only with its nested observations, and only doubtful rows are selected in chunks",async()=>{
 const{root,src,rows}=fixture(),file=join(root,"stamps.json"),a=statSync(join(src,"a.ts"));
 try{
  const analysis={kind:"analysis",path:join(src,"a.ts"),fact:{source:{size:a.size,mtimeMs:a.mtimeMs,ctimeMs:a.ctimeMs,ino:a.ino},nestedObservations:[{kind:"readdir",path:src,fact:["a.ts","b.ts"]}]}},all=[...rows,analysis];
  await pass(all,file);
  expect((await pass(all,file))[5],"stat and nested listing hold").toBe(0);
  writeFileSync(join(src,"c.ts"),"c");
  expect((await pass(all,file))[5],"the nested listing changed").toBe(1);
  const doubt=createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}),kept=await selectDoubtfulRowsV1({rows:all} as never,all.map((_,id)=>id),doubt as never,{chunk:2});
  expect(kept.every(id=>id>=0&&id<all.length)).toBe(true);expect(kept.length).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a long synchronous verification gives the event loop turns, so heartbeats and cancellation flow",async()=>{
 const{staleFactRowsV1}=await import("../../🟨️.mjs");
 let ticks=0;const timer=setInterval(()=>{ticks++;},20);
 const busy=(milliseconds:number)=>{const until=performance.now()+milliseconds;while(performance.now()<until);};
 const index={rows:Array.from({length:6},(_,id)=>({id}))};
 const{stale}=await staleFactRowsV1(index as never,[0,1,2,3,4,5],async()=>{busy(300);return true;},{chunk:1,parallel:1});
 clearInterval(timer);
 expect(stale.size).toBe(0);expect(ticks).toBeGreaterThanOrEqual(3);
});

test("a published generation stamps the untouched directories, so a fresh process vouches for them at once, and never one touched since the facts were observed",async()=>{
 const{root,src,rows}=fixture(),file=join(root,"stamps.json");
 try{
  const publisher=createEpochDoubtV1({stamps:createDirectoryStampsV1({file})});
  await publisher.published({rows:rows as never,observedAt:Date.now()-120_000});
  expect(Array.from(await createEpochDoubtV1({stamps:createDirectoryStampsV1({file})}).classify(rows as never)),"touched after the observation: nothing stamped").toEqual([0,1,1,1,1]);
  expect(await publisher.published({rows:rows as never,observedAt:Date.now()})).toBe(true);
  const fresh=createEpochDoubtV1({stamps:createDirectoryStampsV1({file})});
  expect(Array.from(await fresh.classify(rows as never)),"vouched at the first pass of a new process").toEqual([0,0,0,0,1]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a verdict by id keeps the same ids as a verdict by rows and decodes no row for them",async()=>{
 const rows=[{kind:"file",path:"a"},{kind:"readdir",path:"b"},{kind:"file",path:"c"},{kind:"exists",path:"d"},{kind:"file",path:"e"}],read={count:0},index={get rows(){return new Proxy(rows,{get:(target,key)=>{if(typeof key==="string"&&/^\d+$/u.test(key))read.count++;return Reflect.get(target,key);}});},rowPathIdOf:(id:number)=>id};
 const odd=(id:number)=>id%2===1?1:0,byRows={classify:async(part:readonly {path:string}[])=>Uint8Array.from(part,row=>odd("abcde".indexOf(row.path)))},byIds={classify:async()=>{throw Error("rows are not wanted");},classifyIds:async(_index:unknown,part:readonly number[])=>Uint8Array.from(part,odd)};
 const ids=[0,1,2,3,4],expected=await selectDoubtfulRowsV1(index as never,ids,byRows as never,{chunk:2}),actual=await selectDoubtfulRowsV1(index as never,ids,byIds as never,{chunk:2});
 expect(actual).toEqual(expected);expect(actual).toEqual([1,3]);
 read.count=0;await selectDoubtfulRowsV1(index as never,ids,byIds as never,{chunk:2});expect(read.count).toBe(0);
});

test("an unchanged stamp table is not written again, a changed one is",async()=>{
 const{root,src,rows}=fixture(),file=join(root,"stamps.json");
 try{
  await pass(rows,file);
  const first=statSync(file),text=readFileSync(file,"utf8");
  await new Promise(resolve=>setTimeout(resolve,25));
  await pass(rows,file);
  expect(statSync(file).ino,"the same file").toBe(first.ino);expect(statSync(file).mtimeMs).toBe(first.mtimeMs);expect(readFileSync(file,"utf8")).toBe(text);
  writeFileSync(join(src,"c.ts"),"c");old(join(src,"c.ts"));old(src);
  await pass(rows,file);
  expect(readFileSync(file,"utf8"),"a moved directory stamp is written").not.toBe(text);
  const second=readFileSync(file,"utf8");await pass(rows,file);
  expect(readFileSync(file,"utf8")).toBe(second);
  rmSync(file);await pass(rows,file);
  expect(statSync(file).isFile(),"a missing file is written even when no stamp changed").toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("settling forgets exactly the directories the full row walk forgot, over random rows, verified sets and stale sets",async()=>{
 const{root,src,rows}=fixture(),next=(seed=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;})(7);
 try{
  const more=[...rows,{kind:"readdir",path:root,fact:["src"]},{kind:"exists",path:join(src,"a.ts"),fact:true},{kind:"lstat",path:join(src,"b.ts"),fact:"file"},{kind:"derived",path:"x",fact:1}];
  for(let round=0;round<40;round++){
   const ids=more.map((_,id)=>id).filter(()=>next()<0.7),stale=new Set(more.map((_,id)=>id).filter(()=>next()<0.2));
   const real=createDirectoryStampsV1({file:join(root,"s"+round+".json")});let forgotten:Set<string>|undefined;
   const doubt=createEpochDoubtV1({stamps:{classify:(...arguments_:Parameters<typeof real.classify>)=>real.classify(...arguments_),save:async(measured:Map<string,unknown>,exclude:Set<string>)=>{forgotten=new Set(exclude);return real.save(measured as never,exclude);}} as never});
   await doubt.classify(more as never);await doubt.settled({rows:more as never,ids,stale});
   const selected=new Uint8Array(more.length),expected=new Set<string>();for(const id of ids)selected[id]=1;
   for(let id=0;id<more.length;id++){const own=new Set<string>();rowDependenciesV1(more[id] as never,own,new Set());if(own.size&&(selected[id]!==1||stale.has(id)))for(const path of own)expected.add(path);}
   expect([...forgotten!].sort(),"round "+round).toEqual([...expected].sort());
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🪶️ Checks that the real binary view decodes only rows that can affect directory stamps. */
test("directory stamp passes skip packed content-only rows without losing structural dependencies",async()=>{
 const root=join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"packed-rows"),at=(scope:string,name?:string)=>name?join(root,scope,name):join(root,scope),rows=[];
 for(let id=0;id<packedStampFixture.files;id++)rows.push({kind:"file",path:at("contents","f"+id),fact:{size:id,mtimeMs:7,ctimeMs:8,ino:id,sha256:"0".repeat(64)}});
 for(let id=0;id<packedStampFixture.digests;id++)rows.push({kind:"digest",path:at("contents","d"+id),fact:{size:id,mtimeMs:7,sha256:"a".repeat(64)}});
 rows.push(
  {kind:"readdir",path:at("listing"),fact:["one"]},
  {kind:"fileExists",path:at("missing","one"),fact:false},
  {kind:"exists",path:at("present","one"),fact:true},
  {kind:"analysis",path:at("contents","analyzed"),fact:{nestedObservations:[{kind:"lstat",path:at("nested","one"),fact:"file"}]}},
  {kind:"resolve",path:JSON.stringify([at("resolved","request.ts"),"./provider"]),fact:at("resolved","provider.ts")}
 );
 const bytes=encodeFactIndexV1(createFactIndexV1(new Map([["owner",rows]])),{maximumBytes:packedStampFixture.maximumBytes}),published=openFactIndexV1(bytes),settled=openFactIndexV1(bytes);
 const database=new Database(":memory:"),names=(paths:readonly string[])=>paths.map(path=>relative(root,path)).sort();
 let checked:string[]|undefined,excluded:Set<string>|undefined;
 try{
  const lookup=(values:readonly string[])=>database.query("select distinct value from json_each(?) order by value").all(JSON.stringify(values)).map(row=>(row as {value:string}).value);
  const doubt=createEpochDoubtV1({stamps:{async classify(paths:string[]){checked=paths;return{measured:new Map()};},async save(_measured:Map<string,unknown>,exclude?:Set<string>){excluded=exclude;return true;}} as never});
  expect(await doubt.published({rows:published.rows,index:published,observedAt:1} as never)).toBe(true);
  expect(names(checked!)).toEqual(lookup(packedStampFixture.published));
  expect(published.rowsRead()).toBe(packedStampFixture.publishedReads);
  const stable=Array.from({length:settled.rows.length},(_,id)=>id).find(id=>settled.rowKindOf(id)==="readdir")!;
  expect(await doubt.settled({rows:settled.rows,index:settled,ids:[stable],stale:new Set()} as never)).toBe(true);
  expect(names([...excluded!])).toEqual(lookup(packedStampFixture.excluded));
  expect(settled.rowsRead()).toBe(packedStampFixture.settledReads);
 }finally{database.close();}
});

/** 🛑️ Keeps a cancelled optional stamp pass from saving a partial observation. */
test("publication stamp measurement propagates cancellation before save",async()=>{
 const controller=new AbortController();
 let saved=0,signal:AbortSignal|undefined;
 const doubt=createEpochDoubtV1({stamps:{async classify(_paths:string[],options:{signal?:AbortSignal}){signal=options.signal;controller.abort(Error("stamp measurement cancelled"));return{measured:new Map()};},async save(){saved++;return true;}} as never});
 await expect(doubt.published({rows:[{kind:"readdir",path:".",fact:["one"]}],observedAt:1,signal:controller.signal} as never)).rejects.toThrow("stamp measurement cancelled");
 expect(signal).toBe(controller.signal);
 expect(saved).toBe(0);
});
