import{expect,test}from"bun:test";
import{appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createLayerIndexV1}from"../🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";

import generations from "./dictionary-generation.json";
import { Database } from "bun:sqlite";
import { closureLayerListV1 } from "../../🟨️.mjs";

const root=()=>mkdtempSync(join(realpathSync(tmpdir()),"layer-index-"));
const rows=(count:number,from:number)=>Array.from({length:count},(_all,at)=>({kind:"lstat",path:"/tree/"+(from+at),fact:at%3===0?"file":"directory"}));
const digest=(letter:string)=>letter.repeat(64);
const canonical=(row:any)=>JSON.stringify([row.kind,row.path,row.fact]);

/** 🧪️ The ids of a layer name its rows in order, read back by a reader that shares nothing with the writer but the files. */
test("a later reader gets the rows of a layer back from its ids",()=>{
 const directory=root();
 try{
  const layer=rows(40,0),writer=createLayerIndexV1({directory});
  expect(writer.indexSync(digest("a"),layer)?.length).toBe(40);
  const reader=createLayerIndexV1({directory}),ids=reader.idsSync(digest("a"));
  expect(ids?.length).toBe(40);
  expect(Array.from(ids!,id=>reader.rowSync(id))).toEqual(layer);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ Rows that several layers name are written once; the count of distinct rows is the one an independent set of canonical texts finds. */
test("the dictionary holds each distinct row once however many layers name it",()=>{
 const directory=root();
 try{
  const layers=[rows(60,0),rows(60,30),rows(60,10),rows(60,0)],writer=createLayerIndexV1({directory});
  layers.forEach((layer,at)=>writer.indexSync(digest("abcd"[at]!),layer));
  const distinct=new Set(layers.flat().map(canonical)).size,reader=createLayerIndexV1({directory}),seen=new Set<number>();
  for(const letter of "abcd")for(const id of reader.idsSync(digest(letter))!)seen.add(id);
  expect(seen.size).toBe(distinct);
  expect(reader.idsSync(digest("a"))).toEqual(reader.idsSync(digest("d")));
  const extra=rows(60,20),another=createLayerIndexV1({directory});another.indexSync(digest("e"),extra);
  const after=new Set<number>();for(const letter of "abcde")for(const id of another.idsSync(digest(letter))!)after.add(id);
  expect(after.size).toBe(new Set([...layers.flat(),...extra].map(canonical)).size);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ A damaged list, a torn dictionary tail and a replaced dictionary are refused or repaired, never answered with a wrong row. */
test("a damaged list, a torn tail and a replaced dictionary never give a wrong row",()=>{
 const directory=root();
 try{
  const writer=createLayerIndexV1({directory}),layer=rows(30,0);writer.indexSync(digest("a"),layer);
  const listing=join(directory,"x",digest("a")+".ix"),bytes=readFileSync(listing);bytes[40]^=1;writeFileSync(listing,bytes);
  expect(createLayerIndexV1({directory}).idsSync(digest("a"))).toBeUndefined();
  rmSync(listing);
  const dictionary=join(directory,"x","rows.dict");appendFileSync(dictionary,Buffer.from([82,87,66,49,200,0,0,0,1,2,3]));
  const repaired=createLayerIndexV1({directory}),ids=repaired.indexSync(digest("b"),rows(30,0));
  expect(Array.from(ids!,id=>repaired.rowSync(id))).toEqual(layer);
  const reader=createLayerIndexV1({directory});
  expect(Array.from(reader.idsSync(digest("b"))!,id=>reader.rowSync(id))).toEqual(layer);
  rmSync(dictionary);
  const fresh=createLayerIndexV1({directory});fresh.indexSync(digest("c"),rows(30,1000));
  expect(fresh.idsSync(digest("b"))).toBeUndefined();
  expect(Array.from(reader.idsSync(digest("b"))!,id=>reader.rowSync(id))).toEqual(layer);
  expect(Array.from(fresh.idsSync(digest("c"))!,id=>fresh.rowSync(id))).toEqual(rows(30,1000));
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ A writer that finds the dictionary locked gives up within its wait and parses on; a lock that its owner abandoned is taken over. */
test("a held lock is skipped within the wait and a stale lock is taken over",()=>{
 const directory=root();
 try{
  const lock=join(directory,"x","lock");mkdirSync(lock,{recursive:true});
  const limits={...INFERENCE_LIMITS_V1,layerIndexLockWaitMilliseconds:50},index=createLayerIndexV1({directory,limits}),started=Date.now();
  expect(index.indexSync(digest("a"),rows(30,0))).toBeUndefined();
  expect(Date.now()-started).toBeLessThan(2000);
  const old=new Date(Date.now()-INFERENCE_LIMITS_V1.layerIndexLockStaleMilliseconds-1000);utimesSync(lock,old,old);
  expect(index.indexSync(digest("a"),rows(30,0))?.length).toBe(30);
  expect(existsSync(lock)).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ A list of a layer that is gone from the store is removed, an oversized dictionary resets the whole index under a new identity. */
test("the sweep removes lists of vanished layers and resets an oversized dictionary",()=>{
 const directory=root();
 try{
  const layers=join(directory,"l");mkdirSync(layers,{recursive:true});writeFileSync(join(layers,digest("a")+".json.br"),"");
  const index=createLayerIndexV1({directory});index.indexSync(digest("a"),rows(30,0));index.indexSync(digest("b"),rows(30,100));
  expect(index.sweepSync(layers)).toEqual({removed:1,reset:false});
  writeFileSync(join(directory,"x","stray.ids"),"");
  expect(index.sweepSync(layers)).toEqual({removed:1,reset:false});
  expect(readdirSync(join(directory,"x")).sort()).toEqual([digest("a")+".ix","rows.dict"]);
  const small=createLayerIndexV1({directory,limits:{...INFERENCE_LIMITS_V1,layerIndexDictionaryMaximumBytes:statSync(join(directory,"x","rows.dict")).size}});
  expect(small.sweepSync(layers).reset).toBe(true);
  expect(readdirSync(join(directory,"x"))).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ A row whose bytes changed in the dictionary does not match the fingerprint its record carries and is refused. */
test("a row that was damaged in the dictionary is refused, its neighbours are still read",()=>{
 const directory=root();
 try{
  const writer=createLayerIndexV1({directory}),layer=rows(30,0),ids=writer.indexSync(digest("a"),layer)!;
  const dictionary=join(directory,"x","rows.dict"),bytes=readFileSync(dictionary),victim=ids[7]!;
  bytes[victim+14+12]^=2;writeFileSync(dictionary,bytes);
  const reader=createLayerIndexV1({directory}),read=reader.idsSync(digest("a"))!;
  expect(reader.rowSync(read[7]!)).toBeUndefined();
  expect(reader.rowSync(read[8]!)).toEqual(layer[8]);
  expect(reader.stats().refused).toBe(1);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("row offsets belong to the dictionary generation in which they were issued",()=>{
 const directory=root(),oracle=new Database(":memory:");
 try{
  oracle.run("CREATE TABLE rows (generation INTEGER, ordinal INTEGER, path TEXT)");
  const reader=createLayerIndexV1({directory}),actual:string[]=[];
  const log={id(row:{path:string}){return actual.push(row.path)-1;},builder(){let ids:number[]=[];return {addIds(batch:Uint32Array){ids=Array.from(batch);},finish(){return ids;}};}};
  const store={layerListingSync:(key:string)=>reader.listingSync(key)};
  let original:ReturnType<typeof reader.listingSync>|undefined;
  for(const [generation,start] of [generations.first,generations.second].entries()){
   if(generation)rmSync(join(directory,"x","rows.dict"));
   const layer=rows(generations.count,start),key=digest("ab"[generation]!);
   createLayerIndexV1({directory}).indexSync(key,layer);
   layer.forEach((row,at)=>oracle.run("INSERT INTO rows VALUES (?,?,?)",[generation,at,row.path]));
   const ids=closureLayerListV1(log,{key,digest:key,store});
   const expected=oracle.query("SELECT path FROM rows WHERE generation=? ORDER BY ordinal").all(generation) as {path:string}[];
   expect(Array.from(ids,(id:number)=>actual[id])).toEqual(expected.map(row=>row.path));
   if(!generation)original=reader.listingSync(key);
  }
  expect(original).toBeDefined();
  expect(original!.rowSync(original!.ids[0]!)).toBeUndefined();
 }finally{oracle.close();rmSync(directory,{recursive:true,force:true});}
});

/** 🧪️ The descriptor this process keeps open for reads is closed by `release`, so no file of the image stays open between operations. */
test("release closes the dictionary descriptor: a listing handed out before answers nothing, the next read opens it again",()=>{
 const directory=root();
 try{
  const layer=rows(20,0),index=createLayerIndexV1({directory});
  index.indexSync(digest("a"),layer);
  const listing=index.listingSync(digest("a"))!;
  expect(listing.rowSync(listing.ids[0]!)).toEqual(layer[0]);
  index.release();
  expect(listing.rowSync(listing.ids[0]!)).toBeUndefined();
  const again=index.listingSync(digest("a"))!;
  expect(again.rowSync(again.ids[0]!)).toEqual(layer[0]);
  index.release();index.release();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
