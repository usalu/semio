/** ♻️ Contract: the declared data files of the graph implementation are facts named by the digest of their canonical JSON without the sections nothing of the graph reads. The digest equals the independent Python model, a row carries the sections it ignores, an edit of an ignored section keeps the row valid while an edit of any other section, a new section or a broken document stales it, and the declaration names files the revision covers. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{existsSync,readFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{environmentRowHoldsV1,sectionalDigestV1}from"../../../🕰️generations/🌍️environment/🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs") as any;
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py"),library=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../");
const declared=schema.definitions.Sections.const.files as Record<string,string[]>;

test("the declaration follows its schema and names files that exist",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Declaration",definitions:schema.definitions},schema.definitions.Sections.const)).toBe(true);
 for(const[local,ignore]of Object.entries(declared)){
  const path=join(library,local);expect(existsSync(path),local).toBe(true);
  const document=JSON.parse(readFileSync(path,"utf8"));
  for(const key of ignore)expect(Object.hasOwn(document,key),local+" has the section "+key).toBe(true);
 }
});

test("the digest equals the independent Python model",()=>{
 const cases=[
  {text:'{"a":1,"b":{"z":[1,2,{"y":null}],"a":"ü😀"},"c":false}',ignore:["c"]},
  {text:'{"b":1,"a":2}',ignore:[]},
  {text:'{"b":1,"a":2}',ignore:["a","b"]},
  {text:'{"k":0.5,"big":19326304256,"neg":-3}',ignore:["k"]},
  {text:"[1,2]",ignore:["a"]},{text:"not json",ignore:[]},{text:"null",ignore:[]},
  ...Object.entries(declared).map(([local,ignore])=>({text:readFileSync(join(library,local),"utf8"),ignore})),
 ];
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases}))});
 expect(run.exitCode).toBe(0);
 expect(cases.map(row=>sectionalDigestV1(row.text,row.ignore))).toEqual(JSON.parse(run.stdout.toString()).digests);
});

test("the rows of the loaded implementation carry the ignored sections and their digest",()=>{
 const rows=inference.loadedImplementationRowsV1 as {kind:string;path:string;fact:{sha256:string;ignore?:string[]}}[];
 for(const[local,ignore]of Object.entries(declared)){
  const row=rows.find(entry=>entry.path===join(library,local));
  expect(row,local).toBeDefined();expect(row!.fact.ignore,local).toEqual(ignore);
  expect(row!.fact.sha256,local).toBe(sectionalDigestV1(readFileSync(join(library,local),"utf8"),ignore));
 }
});

test("an edit of an ignored section keeps the row valid, any other edit stales it",async()=>{
 const local="🏗️build/🔣️policy.json",ignore=declared[local]!,text=readFileSync(join(library,local),"utf8"),document=JSON.parse(text);
 const row={kind:"digest",path:local,fact:{sha256:sectionalDigestV1(text,ignore),size:-1,mtimeMs:-1,ignore}};
 const holds=async(edited:unknown)=>environmentRowHoldsV1(row as never,{read:async()=>typeof edited==="string"?edited:JSON.stringify(edited,null,2),stat:async()=>({size:7,mtimeMs:9})});
 expect(await holds(document)).toBe(true);
 expect(await holds({...document,[ignore[0]!]:{changed:true}})).toBe(true);
 const dropped={...document};delete dropped[ignore[1]!];expect(await holds(dropped)).toBe(true);
 const read=Object.keys(document).find(key=>!ignore.includes(key)&&key!=="version")!;
 expect(await holds({...document,[read]:"edited"})).toBe(false);
 expect(await holds({...document,addedSection:1})).toBe(false);
 expect(await holds("{ broken")).toBe(false);
 const whole={kind:"digest",path:local,fact:{sha256:"0".repeat(64),size:-1,mtimeMs:-1}};
 expect(await environmentRowHoldsV1(whole as never,{read:async()=>text,stat:async()=>({size:7,mtimeMs:9})})).toBe(false);
});
