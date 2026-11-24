import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{hostOfRootV1,neutralRecordV1,localRecordV1,type PortableRecordV1,type ObservationRowV1}from"../../🟦️.ts";

/** 🖥️ Law: two machines with other checkout roots, other cache roots, other sealed tool snapshot locations, other dependency generations, other analysis provider hashes and other file stamps record the same owner as byte-identical neutral records (every row is a repo-relative path with a content digest or a declared tool identity), and each accepts the record of the other as the record it would have made itself. Whatever lies in a cache or a tool snapshot is machine-local by name and is no row. The neutral bytes equal the independent Python oracle.
 * @see ./🐍️.py */
type Machine={name:string;root:string;cache:string;snapshot:string;generation:string;provider:string;record:PortableRecordV1&{machineLocal:ObservationRowV1[]}};
const here=import.meta.dir,schema=join(here,"..","..","🧬️schema","🔣️.json"),fixturePath=join(here,"🔣️.json");
const machines=(JSON.parse(readFileSync(fixturePath,"utf8")) as {machines:Machine[]}).machines;
const plain=(record:PortableRecordV1)=>({digest:record.digest,authority:record.authority,observations:record.observations,image:record.image});
const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical((value as Record<string,unknown>)[key])])):value;
const form=(record:PortableRecordV1)=>JSON.stringify(canonical({...record,image:JSON.parse(record.image)}));
const neutralOf=(machine:Machine)=>neutralRecordV1(plain(machine.record),hostOfRootV1(machine.root)).record;

test("what a machine keeps in its caches and its sealed tool snapshot lies outside the repo-relative rows and is no row of a record, on every machine",()=>{
 expect(machines.length).toBe(2);
 for(const machine of machines){
  expect(machine.record.machineLocal.length).toBeGreaterThan(1);
  for(const row of machine.record.observations){
   const text=JSON.stringify(row);
   expect(text.includes(machine.cache),machine.name+" "+row.path).toBe(false);
   expect(text.includes(machine.snapshot),machine.name+" "+row.path).toBe(false);
  }
 }
});

test("the two machines write byte-identical neutral records that hold no trace of a root, a cache, a snapshot, a generation, a provider hash or a stamp",()=>{
 const[a,b]=machines.map(neutralOf);
 expect(form(a!)).toBe(form(b!));
 const text=JSON.stringify(a);
 for(const machine of machines)for(const trace of[machine.root,machine.cache,machine.snapshot,machine.generation,machine.provider,"1700000000","1800000000"])expect(text.includes(trace),machine.name+" "+trace).toBe(false);
});

test("the neutral bytes equal the independent Python oracle",()=>{
 const run=spawnSync("python3",["-I",join(here,"🐍️.py"),"neutral",schema,fixturePath],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const oracle=JSON.parse(run.stdout) as PortableRecordV1[];
 machines.forEach((machine,at)=>expect(form(neutralOf(machine)),machine.name).toBe(form(oracle[at]!)));
});

test("each machine accepts the record of the other: the record localized for it equals the record it made itself, up to the rows above its root and the stamps",()=>{
 const stampless=(rows:readonly ObservationRowV1[])=>JSON.stringify(canonical(rows.map(row=>JSON.parse(JSON.stringify(row,(key,value)=>["mtimeMs","ctimeMs","ino"].includes(key)&&typeof value==="number"?0:value)))));
 for(const[from,to]of[[machines[0]!,machines[1]!],[machines[1]!,machines[0]!]]){
  const made=localRecordV1(neutralOf(from),hostOfRootV1(to.root),{providers:["build:command-imports:v1:"+to.provider],generation:to.generation}).record,own=to.record;
  const kept=own.observations.filter(row=>!(row.kind==="lstat"&&row.fact==="directory"));
  expect(stampless(made.observations),from.name+" -> "+to.name).toBe(stampless(kept));
  expect(JSON.stringify(canonical(JSON.parse(made.image)))).toBe(JSON.stringify(canonical(JSON.parse(own.image))));
 }
});
