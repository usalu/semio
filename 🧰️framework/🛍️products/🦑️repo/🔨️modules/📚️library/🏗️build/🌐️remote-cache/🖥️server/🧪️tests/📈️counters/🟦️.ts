/** 📈️ Contract: the counters of the cache server survive a restart: `GET /status` answers the cumulative count (the persisted ledger plus this run), `session` the run alone, and the ledger file in the data directory is valid against its schema (checked with Ajv).
 * @see ../../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import Ajv from"ajv";
import{createHash}from"node:crypto";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../🟦️.ts";
import schema from"../../../../📈️counter-ledger/🧬️schema/🔣️.json"with{type:"json"};

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
const serve=(directory:string)=>startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{}});
const status=async(url:string)=>await (await fetch(url+"/status")).json() as {counters:{ac:{misses:number};cas:{hits:number;misses:number;puts:number}};session?:{cas:{hits:number;puts:number}};ledger?:{sinceMs:number;restarts:number}};

test("a restarted server answers the counts of its earlier runs and its own",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"rcache-counters-"));scratch.push(directory);
 const body="counted",key=sha(body);
 const first=await serve(directory);
 expect((await fetch(`${first.url}/main/cas/${key}`,{method:"PUT",body})).ok).toBe(true);
 for(let index=0;index<3;index++)expect((await fetch(`${first.url}/main/cas/${key}`)).status).toBe(200);
 expect((await fetch(`${first.url}/main/cas/${sha("absent")}`)).status).toBe(404);expect((await fetch(`${first.url}/main/ac/${sha("absent")}`)).status).toBe(404);
 const before=await status(first.url);
 expect([before.counters.cas.puts,before.counters.cas.hits,before.counters.cas.misses,before.counters.ac.misses]).toEqual([1,3,1,1]);
 expect(before.session?.cas.hits).toBe(3);expect(before.ledger?.restarts).toBe(1);
 await first.stop();
 const written=JSON.parse(readFileSync(join(directory,"v1","counters.json"),"utf8")) as {groups:{cas:{hits:number}};sinceMs:number};
 const ajv=new Ajv({strict:true,allErrors:true}),valid=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Ledger"});
 expect(valid(written),JSON.stringify(valid.errors)).toBe(true);expect(written.groups.cas.hits).toBe(3);
 const second=await serve(directory);
 try{
  const fresh=await status(second.url);
  expect([fresh.counters.cas.puts,fresh.counters.cas.hits,fresh.counters.cas.misses,fresh.counters.ac.misses]).toEqual([1,3,1,1]);
  expect(fresh.session?.cas.hits).toBe(0);expect(fresh.ledger?.restarts).toBe(2);expect(fresh.ledger?.sinceMs).toBe(written.sinceMs);
  expect((await fetch(`${second.url}/main/cas/${key}`)).status).toBe(200);
  const after=await status(second.url);
  expect(after.counters.cas.hits).toBe(4);expect(after.session?.cas.hits).toBe(1);
 }finally{await second.stop();}
});
