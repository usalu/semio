/** 🔎️ Contract: without a journal the fact check costs one stat per distinct directory and file. A structural row under a directory whose stamp is unchanged is vouched for, a changed, new or racy directory and every unstamped row go to the precise check, a rows file edit is caught by its own stat, and the stamps of directories whose rows were stale are forgotten. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createDirectoryStampsV1} from "../../🗂️directory-stamps/🟨️.mjs";
import {createEpochDoubtV1,selectDoubtfulRowsV1} from "../../🔎️doubt/🟨️.mjs";

const old=(path:string)=>{const past=new Date(Date.now()-60_000);utimesSync(path,past,past);};
const fixture=()=>{
 const root=mkdtempSync(join(tmpdir(),"doubt-")),src=join(root,"src");mkdirSync(src);
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
