/** 🧮️ Contract: the binary fact index round-trips the logical index exactly, equals the independent Python encoder byte for byte, refuses every malformed document the Python decoder refuses, scans its strings without building rows, and stays a fraction of the canonical JSON of the same graph. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv2020 from"ajv";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{FACT_CODEC_POLICY_V1,canonicalJsonV1,decodeFactIndexV1,encodeFactIndexV1,openFactIndexV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py"),limit=1<<26,decode=(hex:string)=>decodeFactIndexV1(Buffer.from(hex,"hex"),{maximumBytes:limit});
const model=(command:string,input:string)=>Bun.spawnSync([python!,"-I",oracle,command],{stdin:Buffer.from(input)});

test("the fixture and the policy follow the schema file",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(FACT_CODEC_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof FACT_CODEC_POLICY_V1);
 expect(Buffer.from(FACT_CODEC_POLICY_V1.magic,"latin1")).toHaveLength(4);
});

for(const{id,index}of fixture.indexes)test("binary fact index round trip and Python model: "+id,()=>{
 const bytes=encodeFactIndexV1(index as never,{maximumBytes:limit}),again=encodeFactIndexV1(decodeFactIndexV1(bytes,{maximumBytes:limit}),{maximumBytes:limit});
 expect(JSON.parse(canonicalJsonV1(decodeFactIndexV1(bytes,{maximumBytes:limit})))).toEqual(JSON.parse(canonicalJsonV1(index)));
 expect(Buffer.compare(bytes,again)).toBe(0);
 const scan=decodeFactIndexV1(bytes,{maximumBytes:limit,build:false}).strings,strict=[...scan].every((text,at)=>at===0||Buffer.compare(Buffer.from(scan[at-1]!),Buffer.from(text))<0);
 expect(strict).toBe(true);
 if(python){
  const encoded=model("encode",JSON.stringify(index));expect(encoded.exitCode).toBe(0);expect(encoded.stdout.toString()).toBe(bytes.toString("hex"));
  const decoded=model("decode",bytes.toString("hex"));expect(decoded.exitCode).toBe(0);expect(JSON.parse(decoded.stdout.toString())).toEqual(JSON.parse(canonicalJsonV1(index)));
  const strings=model("strings",bytes.toString("hex"));expect(JSON.parse(strings.stdout.toString())).toEqual(scan);
 }
});

for(const{id,hex}of fixture.refusals)test("malformed binary fact index is refused by both decoders: "+id,()=>{
 expect(()=>decode(hex)).toThrow("Closed binary fact index required");
 expect(()=>decodeFactIndexV1(Buffer.from(hex,"hex"),{maximumBytes:limit,build:false})).toThrow("Closed binary fact index required");
 if(python)expect(model("decode",hex).exitCode).toBe(3);
});

test("a document beyond the finite byte ceiling is refused on both sides before anything is built",()=>{
 const index={version:1 as const,owners:["a"],rows:[{kind:"file",path:"x".repeat(2000),fact:true}],classes:[{r:[0],o:[0]}]},bytes=encodeFactIndexV1(index,{maximumBytes:limit});
 expect(()=>encodeFactIndexV1(index,{maximumBytes:bytes.length-1})).toThrow("Stored document exceeds its finite limit");
 expect(()=>decodeFactIndexV1(bytes,{maximumBytes:bytes.length-1})).toThrow("Bounded fact index bytes required");
 expect(()=>encodeFactIndexV1({...index,rows:[{kind:"",path:"p",fact:0}]},{maximumBytes:limit})).toThrow("Closed fact row required");
 expect(()=>encodeFactIndexV1({...index,rows:[{kind:"k",path:"\ud800",fact:0}]},{maximumBytes:limit})).toThrow("Well formed text required");
});

function random(seed:number){return()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};}
const hexOf=(next:()=>number)=>Array.from({length:64},()=>"0123456789abcdef"[Math.floor(next()*16)]).join("");

/** 🏗️ A graph shaped like the real workspace: owners share directory listings and existence probes, every analysis fact carries two digests and three stamps. */
function workspace(owners:number,perOwner:number){
 const next=random(7),root="/Users/dev/Documents/semio/",segments=["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🕸️graph","🧪️tests","📦️packages"],rows:{kind:string;path:string;fact:unknown}[]=[],classes:{r:number[];o:number[]}[]=[],names:string[]=[];
 const dirs=Array.from({length:Math.max(1,owners*2)},(_,i)=>root+segments.slice(0,2+i%7).join("/")+"/d"+i);
 for(let owner=0;owner<owners;owner++){
  names.push("@semio-tech/p"+String(owner).padStart(5,"0"));const mine:number[]=[];
  for(let item=0;item<perOwner;item++){
   const pick=next(),dir=dirs[Math.floor(next()*dirs.length)]!,at=rows.length;
   if(pick<0.4)rows.push({kind:"exists",path:dir+"/f"+item,fact:next()<0.5});
   else if(pick<0.6)rows.push({kind:"readdir",path:dir,fact:hexOf(()=>0.25+0.5*((owner+item)%2))});
   else rows.push({kind:"analysis",path:dir+"/s"+owner+"_"+item+".ts",fact:{nestedObservations:[],provider:"build:script-requirements:v1:"+"a".repeat(64),resultSha256:hexOf(next),revision:"b".repeat(64),source:{ctimeMs:1.79e12+next()*1e6,ino:Math.floor(next()*1e9),mtimeMs:1.79e12+next()*1e6,sha256:hexOf(next),size:Math.floor(next()*5e4)}}});
   mine.push(at);
  }
  classes.push({r:mine,o:[owner]});
 }
 return{version:1 as const,owners:names,rows,classes};
}

test("the binary index of a workspace shaped graph stays a fraction of its canonical JSON",()=>{
 const index=workspace(fixture.sizes.owners,fixture.sizes.rowsPerOwner),bytes=encodeFactIndexV1(index,{maximumBytes:limit}),text=canonicalJsonV1(index),database=new Database(":memory:");
 let jsonBytes:number;try{jsonBytes=(database.query("SELECT length(CAST(json(?) AS BLOB)) AS bytes").get(text) as {bytes:number}).bytes;}finally{database.close();}
 expect(jsonBytes).toBe(Buffer.byteLength(text));
 expect(bytes.length).toBeLessThanOrEqual(fixture.sizes.maximumExpandedBytes);
 expect(bytes.length/jsonBytes).toBeLessThan(fixture.sizes.maximumRatio);
 expect(JSON.parse(canonicalJsonV1(decodeFactIndexV1(bytes,{maximumBytes:limit})))).toEqual(JSON.parse(text));
 if(python){const encoded=model("encode",JSON.stringify(index));expect(encoded.stdout.toString()).toBe(bytes.toString("hex"));}
});

for(const{id,index}of fixture.indexes)test("a lazily opened fact index equals the decoded one and reads its rows on demand: "+id,()=>{
 const bytes=encodeFactIndexV1(index as never,{maximumBytes:limit}),lazy=openFactIndexV1(bytes,{maximumBytes:limit}),full=decodeFactIndexV1(bytes,{maximumBytes:limit});
 expect(lazy.rowsRead()).toBe(0);
 expect(lazy.owners).toEqual(full.owners);expect(lazy.classes).toEqual(full.classes);expect(lazy.rows.length).toBe(full.rows.length);
 for(let row=full.rows.length-1;row>=0;row--){expect(lazy.rowKindOf(row)).toBe(full.rows[row]!.kind);expect(lazy.rows[row]).toEqual(full.rows[row]);}
 expect(lazy.rowsRead()).toBe(full.rows.length);
 expect(lazy.rows.map(row=>row.path)).toEqual(full.rows.map(row=>row.path));
});

for(const{id,hex}of fixture.refusals)test("a malformed binary fact index is refused by the lazy opener as well: "+id,()=>{
 expect(()=>openFactIndexV1(Buffer.from(hex,"hex"),{maximumBytes:limit})).toThrow("Closed binary fact index required");
});

test("a lazily opened index of a workspace shaped graph materialises only the rows that are read",()=>{
 const index=workspace(fixture.sizes.owners,fixture.sizes.rowsPerOwner),bytes=encodeFactIndexV1(index,{maximumBytes:limit}),lazy=openFactIndexV1(bytes,{maximumBytes:limit});
 const picks=[0,7,Math.floor(index.rows.length/2),index.rows.length-1];
 for(const row of picks)expect(JSON.parse(canonicalJsonV1(lazy.rows[row]))).toEqual(JSON.parse(canonicalJsonV1(index.rows[row])));
 expect(lazy.rowsRead()).toBe(picks.length);
});
