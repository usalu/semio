import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {startRemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {fleetFetchContextV1} from "../../🟦️.ts";
import {claimFleetWorkV1,workClaimKeyV1,WORK_CLAIM_POLICY_V1,type WorkClaimEventV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const scratch:string[]=[],stops:Array<()=>Promise<void>>=[],TOKEN="write-token";
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`work-claim-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stops)await stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const serve=async()=>{const server=await startRemoteCacheServerV1({directory:directory("store"),host:"127.0.0.1",port:0,maximumBytes:2**27,maximumEntryBytes:2**25,maximumAgeDays:30,defaultInstance:"main",tokens:{write:TOKEN}});stops.push(()=>server.stop());return server;};
const machine=(url:string,authenticated=true)=>fleetFetchContextV1({},directory("root"),{storeRoot:undefined,remote:{endpoint:url,instance:"main",token:TOKEN,authenticated}});
const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("the key is the same for the same identity, differs per kind and identity and is closed by the schema (Ajv as the independent validator)",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$ref:"#/definitions/Key",definitions:schema.definitions});
 const one=workClaimKeyV1("blob","https://example.test/tool.tar.gz sha256:aa");
 expect(one).toBe("work-blob-"+createHash("sha256").update("https://example.test/tool.tar.gz sha256:aa").digest("hex"));
 expect(validate(one)).toBe(true);expect(validate("work-blob-short")).toBe(false);expect(validate(one.replace("blob","other"))).toBe(false);
 expect(workClaimKeyV1("crates","x")).not.toBe(workClaimKeyV1("uv","x"));expect(workClaimKeyV1("blob","x")).not.toBe(workClaimKeyV1("blob","y"));
 expect(WORK_CLAIM_POLICY_V1.ttlMilliseconds).toBeGreaterThan(WORK_CLAIM_POLICY_V1.renewMilliseconds*2);
 expect(()=>workClaimKeyV1("nope" as never,"x")).toThrow();
 expect(/^[a-z0-9_-]{1,128}$/u.test(one)).toBe(true);
});

test("nine machines ask for the same tool at once: exactly one does the work, the other eight are told it is ready and do nothing",async()=>{
 const server=await serve(),published=new Set<string>();let work=0;
 const run=async(index:number)=>{
  const context=machine(server.url),claim=await claimFleetWorkV1(context,{kind:"blob",identity:"tool-a",owner:`m${index}:1`,ready:async()=>published.has("tool-a")});
  if(claim.outcome==="granted"){work++;await sleep(400);published.add("tool-a");await claim.release();}
  return claim.outcome;
 };
 const outcomes=await Promise.all(Array.from({length:9},(_,index)=>run(index)));
 expect(work,"the work ran once").toBe(1);
 expect(outcomes.filter(outcome=>outcome==="granted")).toHaveLength(1);expect(outcomes.filter(outcome=>outcome==="ready")).toHaveLength(8);
});

test("a result that is already there is never claimed and never recomputed",async()=>{
 const server=await serve(),events:WorkClaimEventV1[]=[];
 const claim=await claimFleetWorkV1(machine(server.url),{kind:"uv",identity:"lock-a",ready:async()=>true,event:event=>events.push(event)});
 expect(claim.outcome).toBe("ready");expect(events.map(event=>event.stage)).toEqual(["ready"]);
 const status=await (await fetch(`${server.url}/status`,{headers:{authorization:`Bearer ${TOKEN}`}})).json() as {claims?:{active:number}};
 expect(status.claims?.active).toBe(0);
});

test("a holder that dies without releasing loses its lease and a waiter takes the work over",async()=>{
 const server=await serve(),key=workClaimKeyV1("crates","lock-dead");
 const dead=await fetch(`${server.url}/main/claim/nx/${key}`,{method:"POST",headers:{authorization:`Bearer ${TOKEN}`,"content-type":"application/json"},body:JSON.stringify({owner:"dead:1",ttlMilliseconds:1000})});
 expect((await dead.json() as {state:string}).state).toBe("granted");
 const started=Date.now(),claim=await claimFleetWorkV1(machine(server.url),{kind:"crates",identity:"lock-dead",owner:"alive:2",ready:async()=>false,waitMaximumMilliseconds:20000});
 expect(claim.outcome).toBe("granted");expect(Date.now()-started).toBeGreaterThanOrEqual(500);expect(Date.now()-started).toBeLessThan(15000);
 await claim.release();
});

test("a holder that stays alive but never finishes does not block longer than the wait maximum: the waiter works alone",async()=>{
 const server=await serve(),key=workClaimKeyV1("blob","lock-slow");
 await fetch(`${server.url}/main/claim/nx/${key}`,{method:"POST",headers:{authorization:`Bearer ${TOKEN}`,"content-type":"application/json"},body:JSON.stringify({owner:"slow:1",ttlMilliseconds:60000})});
 const started=Date.now(),claim=await claimFleetWorkV1(machine(server.url),{kind:"blob",identity:"lock-slow",owner:"waiter:2",ready:async()=>false,waitMaximumMilliseconds:1500});
 expect(claim.outcome).toBe("alone");expect(Date.now()-started).toBeLessThan(10000);
});

test("an unreachable server, a machine without a credential and a cancelled wait all end fast and fail open",async()=>{
 const dead=machine("http://127.0.0.1:1"),started=Date.now();
 expect((await claimFleetWorkV1(dead,{kind:"blob",identity:"x",ready:async()=>false})).outcome).toBe("alone");expect(Date.now()-started).toBeLessThan(20000);
 const server=await serve();
 expect((await claimFleetWorkV1(machine(server.url,false),{kind:"blob",identity:"x",ready:async()=>false})).outcome).toBe("alone");
 const key=workClaimKeyV1("uv","cancel");
 await fetch(`${server.url}/main/claim/nx/${key}`,{method:"POST",headers:{authorization:`Bearer ${TOKEN}`,"content-type":"application/json"},body:JSON.stringify({owner:"busy:1",ttlMilliseconds:60000})});
 let calls=0;
 await expect(claimFleetWorkV1(machine(server.url),{kind:"uv",identity:"cancel",ready:async()=>false,check:()=>{if(++calls>1)throw Error("cancelled");}})).rejects.toThrow("cancelled");
});

test("the claim of a finished piece of work is gone, so the next acquisition after a lost result works again",async()=>{
 const server=await serve(),first=await claimFleetWorkV1(machine(server.url),{kind:"blob",identity:"again",ready:async()=>false});
 expect(first.outcome).toBe("granted");await first.release();
 const second=await claimFleetWorkV1(machine(server.url),{kind:"blob",identity:"again",ready:async()=>false});
 expect(second.outcome).toBe("granted");await second.release();
});

test("holders that finish without publishing never serialize the fleet: after two such wakes the rest work alone in parallel",async()=>{
 const server=await serve(),granted:string[]=[],started=Date.now();
 const outcomes=await Promise.all(Array.from({length:6},async(_,index)=>{
  const claim=await claimFleetWorkV1(machine(server.url),{kind:"blob",identity:"unpublishable",owner:`m${index}:1`,ready:async()=>false,waitMaximumMilliseconds:60000});
  if(claim.outcome==="granted"){granted.push(`m${index}`);await sleep(300);await claim.release();}
  return claim.outcome;
 }));
 expect(granted.length).toBeLessThanOrEqual(3);expect(outcomes.filter(outcome=>outcome==="alone").length).toBeGreaterThanOrEqual(3);expect(Date.now()-started).toBeLessThan(20000);
});
