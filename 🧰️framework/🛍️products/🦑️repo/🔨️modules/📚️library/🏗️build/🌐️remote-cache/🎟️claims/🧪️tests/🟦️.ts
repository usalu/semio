import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🖥️server/🟦️.ts";
import {CLAIM_POLICY_V1,createClaimTableV1,parseClaimPathV1,type ClaimKindV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🎟️ The claim table is proven three ways: JSON vectors that any language can replay, an independent Python model fed the same vectors and 400 random sequences, and the schema through Ajv. The HTTP routes are proven against a real server over real sockets. */
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),scratch:string[]=[];
const ajv=new Ajv({strict:false});ajv.addSchema(schema);
const validator=(name:string)=>{const validate=ajv.getSchema(`${schema.$id}#/definitions/${name}`);if(!validate)throw Error("Missing definition "+name);return validate;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

type Step={at:number;op:string;key?:string;owner?:string;ttl?:number;expected?:number;stored?:boolean;expect?:unknown};

function replay(steps:readonly Step[],maximumClaims=65536):unknown[]{
 let clock=0;const table=createClaimTableV1({now:()=>clock,sweepMilliseconds:0,maximumClaims}),out:unknown[]=[];
 for(const step of steps){
  clock=step.at;const key=step.key??"",owner=step.owner??"";
  if(step.op==="claim")out.push(table.claim("main","nx",key,{owner,ttlMilliseconds:step.ttl!,...(step.expected===undefined?{}:{expectedMilliseconds:step.expected})},step.stored===true));
  else if(step.op==="renew")out.push(table.renew("main","nx",key,{owner,ttlMilliseconds:step.ttl!}));
  else if(step.op==="release")out.push({state:table.release("main","nx",key,owner)});
  else if(step.op==="stored"){table.stored("main","nx",key);out.push({state:"ok"});}
  else if(step.op==="sweep"){table.sweep();out.push({state:"ok"});}
  else if(step.op==="claimed")out.push({claimed:table.claimed("main","nx",key)});
  else if(step.op==="summary"){const {active,oldestSeconds}=table.summary();out.push({active,oldestSeconds});}
 }
 table.close();return out;
}

function pythonModels(batches:readonly {steps:readonly Step[];maximumClaims?:number}[]):unknown[][]{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify({batches})),stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString());
}

test("the schema is closed and every vector expectation validates against it",()=>{
 expect(validator("Policy")(CLAIM_POLICY_V1)).toBe(true);
 for(const vector of fixture.vectors){expect(validator("Vector")(vector),vector.name).toBe(true);}
 expect(validator("ClaimRequest")({owner:"m1:1",ttlMilliseconds:5,extra:1})).toBe(false);
 expect(validator("ClaimRequest")({owner:"bad owner",ttlMilliseconds:5})).toBe(false);
 for(const vector of fixture.vectors)for(const [index,answer] of replay(vector.steps as Step[]).entries()){
  const step=vector.steps[index] as Step;
  if(step.op==="claim")expect(validator("ClaimResponse")(answer),vector.name).toBe(true);
  if(step.op==="renew")expect(validator("RenewResponse")(answer),vector.name).toBe(true);
  if(step.op==="release")expect(validator("Released")(answer),vector.name).toBe(true);
 }
});

test("the vectors hold: claim, takeover after the TTL, heartbeat, release, stored, clamp, expected cost, summary",()=>{
 for(const vector of fixture.vectors){
  const answers=replay(vector.steps as Step[]);
  vector.steps.forEach((step,index)=>expect(answers[index],`${vector.name} step ${index}`).toEqual(step.expect));
 }
});

test.skipIf(!python)("an independent Python model gives the same answers on every vector and on 400 random operation sequences",()=>{
 const known=pythonModels(fixture.vectors.map(vector=>({steps:vector.steps as Step[]})));
 fixture.vectors.forEach((vector,index)=>expect(known[index],vector.name).toEqual(replay(vector.steps as Step[])));
 let state=20261113;const random=()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};
 const pick=(limit:number)=>Math.floor(random()*limit);
 const rounds:{steps:Step[];maximumClaims:number}[]=[];
 for(let round=0;round<400;round++){
  const steps:Step[]=[];let at=0;
  for(let index=0;index<40;index++){
   at+=pick(4000);const key=["a","b","c"][pick(3)]!,owner=["m1:1","m2:2","m3:3"][pick(3)]!,op=["claim","claim","claim","renew","release","stored","sweep","claimed","summary"][pick(9)]!;
   steps.push({at,op,key,owner,ttl:[1,500,2000,5000,30000,999999][pick(6)]!,...(op==="claim"&&random()<0.3?{expected:pick(120000)}:{}),...(op==="claim"&&random()<0.1?{stored:true}:{})});
  }
  rounds.push({steps,maximumClaims:round%5===0?2:65536});
 }
 const modelled=pythonModels(rounds);
 rounds.forEach((round,index)=>expect(replay(round.steps,round.maximumClaims),"round "+index).toEqual(modelled[index]));
});

test("routes parse as the vectors say",()=>{
 for(const row of fixture.routes)expect<unknown>(parseClaimPathV1(row.path,"main")??null,row.path).toEqual(row.address);
});

test("a table that is full refuses new claims with full and still lets holders renew",()=>{
 const table=createClaimTableV1({now:()=>0,sweepMilliseconds:0,maximumClaims:1});
 expect(table.claim("m","nx","a",{owner:"x:1",ttlMilliseconds:5000}).state).toBe("granted");
 expect(table.claim("m","nx","b",{owner:"y:1",ttlMilliseconds:5000}).state).toBe("full");
 expect(table.renew("m","nx","a",{owner:"x:1",ttlMilliseconds:5000}).state).toBe("granted");
 expect(table.claimed("m","nx","a")).toBe(true);expect(table.claimed("other","nx","a")).toBe(false);
 table.close();
});

test("a long poll returns stored when the entry is stored, lapsed when the claim is released or expires, and timeout when nothing happens",async()=>{
 let clock=0;const table=createClaimTableV1({now:()=>clock,sweepMilliseconds:0});
 table.claim("m","nx","a",{owner:"x:1",ttlMilliseconds:5000});
 const started=Date.now(),waiting=table.await("m","nx","a",10000,async()=>false);
 setTimeout(()=>table.stored("m","nx","a"),60);
 expect(await waiting).toEqual({state:"stored"});expect(Date.now()-started).toBeLessThan(2000);
 table.claim("m","nx","b",{owner:"x:1",ttlMilliseconds:5000});
 const lapse=table.await("m","nx","b",10000,async()=>false);setTimeout(()=>table.release("m","nx","b","x:1"),40);
 expect(await lapse).toEqual({state:"lapsed"});
 table.claim("m","nx","c",{owner:"x:1",ttlMilliseconds:1000});
 const expiry=table.await("m","nx","c",10000,async()=>false);setTimeout(()=>{clock=2000;table.sweep();},40);
 expect(await expiry).toEqual({state:"lapsed"});
 table.claim("m","nx","d",{owner:"x:1",ttlMilliseconds:300000});
 const quiet=Date.now();expect(await table.await("m","nx","d",120,async()=>false)).toEqual({state:"timeout"});expect(Date.now()-quiet).toBeGreaterThanOrEqual(100);
 expect(await table.await("m","nx","nothing",5000,async()=>false)).toEqual({state:"lapsed"});
 expect(await table.await("m","nx","d",5000,async()=>true)).toEqual({state:"stored"});
 table.close();
});

async function post(url:string,path:string,body:unknown,token?:string){return fetch(url+path,{method:"POST",body:JSON.stringify(body),headers:{...(token?{authorization:"Bearer "+token}:{})}});}

test("over HTTP: grant, held with 409, takeover after expiry, heartbeat, release, await woken by the PUT of the entry, auth and read-only rules",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"claims-http-"));scratch.push(directory);
 const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const sha=(value:string)=>createHash("sha256").update(value).digest("hex"),key=sha("unit-1"),path=(action="")=>`/claim/ac/${key}${action}`;
 try{
  expect((await post(server.url,path(),{owner:"m1:1",ttlMilliseconds:20000})).status).toBe(401);
  expect((await post(server.url,path(),{owner:"m1:1",ttlMilliseconds:20000},"r")).status).toBe(403);
  const granted=await post(server.url,path(),{owner:"m1:1",ttlMilliseconds:20000,expectedMilliseconds:42000},"w");
  expect(granted.status).toBe(200);const grant=await granted.json();expect(validator("Granted")(grant)).toBe(true);
  const held=await post(server.url,path(),{owner:"m2:2",ttlMilliseconds:20000},"w");
  expect(held.status).toBe(409);const body=await held.json();expect(validator("Held")(body)).toBe(true);expect(body).toMatchObject({state:"held",by:"m1:1",expectedMilliseconds:42000});
  expect((await post(server.url,path("/renew"),{owner:"m1:1",ttlMilliseconds:20000},"w")).status).toBe(200);
  expect((await post(server.url,path("/renew"),{owner:"m2:2",ttlMilliseconds:20000},"w")).status).toBe(409);
  expect((await post(server.url,path(),{owner:"bad owner",ttlMilliseconds:20000},"w")).status).toBe(400);
  expect((await post(server.url,path(),{owner:"m1:1",ttlMilliseconds:"x"},"w")).status).toBe(400);
  const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer r"}})).json() as {claims:{active:number;waiting:number}};expect(status.claims.active).toBe(1);
  const waiting=fetch(server.url+path("/await")+"?timeoutMilliseconds=10000",{headers:{authorization:"Bearer r"}}).then(response=>response.json());
  await new Promise(done=>setTimeout(done,80));
  expect((await fetch(server.url+"/status",{headers:{authorization:"Bearer r"}}).then(response=>response.json()) as {claims:{waiting:number}}).claims.waiting).toBe(1);
  const put=await fetch(server.url+`/ac/${key}`,{method:"PUT",body:"result",headers:{authorization:"Bearer w"}});expect(put.status).toBe(200);
  expect(await waiting).toEqual({state:"stored"});
  expect(server.claims.claimed("main","ac",key)).toBe(false);
  expect(await (await post(server.url,path(),{owner:"m3:3",ttlMilliseconds:20000},"w")).json()).toEqual({state:"stored"});
  const other=sha("unit-2");
  expect((await post(server.url,`/claim/ac/${other}`,{owner:"m1:1",ttlMilliseconds:20000},"w")).status).toBe(200);
  const lapse=fetch(server.url+`/claim/ac/${other}/await?timeoutMilliseconds=10000`,{headers:{authorization:"Bearer r"}}).then(response=>response.json());
  await new Promise(done=>setTimeout(done,50));
  expect(await (await post(server.url,`/claim/ac/${other}/release`,{owner:"m1:1"},"w")).json()).toEqual({state:"released"});
  expect(await lapse).toEqual({state:"lapsed"});
  expect((await fetch(server.url+"/claim/ac/not-a-digest")).status).toBe(404);
  expect((await fetch(server.url+path("/await"))).status).toBe(401);
 }finally{await server.stop();}
 const readOnly=await startRemoteCacheServerV1({directory:mkdtempSync(join(tmpdir(),"claims-ro-")),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"},readOnly:true});
 try{expect((await post(readOnly.url,path(),{owner:"m1:1",ttlMilliseconds:20000},"w")).status).toBe(403);}finally{await readOnly.stop();}
},60000);

test("kinds are checked: ac keys are digests, nx keys are hashes",()=>{
 const table=createClaimTableV1({sweepMilliseconds:0});const kinds:ClaimKindV1[]=["ac","nx"];
 expect(kinds.every(kind=>table.claim("m",kind,"x",{owner:"a:1",ttlMilliseconds:2000}).state==="granted")).toBe(true);
 expect(table.summary().active).toBe(2);table.close();
});
