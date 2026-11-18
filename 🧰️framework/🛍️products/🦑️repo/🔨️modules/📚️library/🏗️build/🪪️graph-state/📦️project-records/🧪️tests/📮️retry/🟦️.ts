import Ajv from "ajv";
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,rmSync,statSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {graphFleetClientV1,type GraphFleetClientV1} from "../../../🛰️exchange/🟦️.ts";
import {unsentListV1} from "../../📮️unsent/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {createHash} from "node:crypto";
import {mkdirSync,writeFileSync} from "node:fs";
import {bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1} from "../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import {createClosureStoreV1} from "../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
import {createProjectFleetPortV1,projectClosurePrefixV1,projectPublishWaitV1,type ProjectRecordInfoV1} from "../../🟦️.ts";

/** 📮️ Contract: a finished record is never thrown away for a cause that may pass. Against a real cache server behind a client whose requests are made to time out or to be refused on a schedule: a record that meets no answer is sent again after the back-off of the policy (and after the breaker of the machine), one that still is not stored is kept on disk and stored by a later `resend`, a refusal that every machine would meet (too large, malformed) is reported as deterministic and not retried, a divergence counts as stored, and every refusal says its true reason, status and words of the cache, in English and German. */
const scratch:string[]=[],ajv=new Ajv({strict:false}),temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`retry-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the pure rules equal the fixture and the independent Python oracle",()=>{
 for(const row of fixture.prefix)expect(projectClosurePrefixV1(row.sizes,row.budget),row.name).toBe(row.kept);
 for(const row of fixture.wait)expect(projectPublishWaitV1(row.attempt,row.breaker,row.unit),row.name).toBe(row.milliseconds);
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {prefix:number[];wait:number[]};
 expect(oracle.prefix).toEqual(fixture.prefix.map(row=>projectClosurePrefixV1(row.sizes,row.budget)));
 expect(oracle.wait).toEqual(fixture.wait.map(row=>projectPublishWaitV1(row.attempt,row.breaker,row.unit)));
 expect(ajv.validate({$ref:"#/definitions/Publish",definitions:schema.definitions},schema.definitions.Publish.const)).toBe(true);
});

type Fault=Readonly<{when:RegExp;answer:"timeout"|"network"|number;body?:string;times:number}>;
async function world(faults:Fault[]){
 const store=temp("store"),state=temp("state"),root=temp("root"),server=await startRemoteCacheServerV1({directory:store,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"complete"});
 const left=faults.map(fault=>fault.times),seen:string[]=[];
 const fetcher=(async(url:string|URL|Request,init?:RequestInit)=>{
  const key=`${init?.method??"GET"} ${new URL(String(url)).pathname}`;seen.push(key);
  for(const [index,fault] of faults.entries())if(left[index]!>0&&fault.when.test(key)){
   left[index]!--;
   if(fault.answer==="timeout")throw Object.assign(new Error("timed out"),{name:"TimeoutError"});
   if(fault.answer==="network")throw new TypeError("connection refused");
   return new Response(fault.body??"",{status:fault.answer});
  }
  return fetch(url,init);
 }) as typeof fetch;
 const client=graphFleetClientV1({endpoint:server.url,instance:"main",token:"w",upload:true,stateDirectory:state},{fetch:fetcher});
 const sleeps:number[]=[],age=(milliseconds:number)=>{for(const name of["down","denied"]){const path=join(state,name);try{const at=new Date(statSync(path).mtimeMs-milliseconds);utimesSync(path,at,at);}catch{}}};
 const sleep=async(milliseconds:number)=>{sleeps.push(milliseconds);age(milliseconds);},port=(extra:Partial<Parameters<typeof createProjectFleetPortV1>[0]>={})=>createProjectFleetPortV1({root,environment:{},client,owner:"m:1",sleep,random:()=>0,...extra})!;
 return{server,client,root,state,sleeps,seen,port,sleep};
}
const image=JSON.stringify({name:"alpha"});
const infoOf=(root:string,extra:Partial<ProjectRecordInfoV1>={}):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/alpha",{name:"alpha"}]]),exportRecord:()=>({digest:"a".repeat(64),authority:[],observations:[{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(){return true;},...extra});
const claimed=async(port:ReturnType<typeof createProjectFleetPortV1>&{},info:ProjectRecordInfoV1)=>{const answer=await port.claim(info);if(answer.state!=="granted")throw Error("not granted: "+answer.state);return answer;};

test("a record that meets no answer twice is stored by the third try after the back-off of the policy, and the claim is held throughout",async()=>{
 const w=await world([{when:/^PUT \/main\/cas\//u,answer:"timeout",times:2}]);
 try{
  const port=w.port(),claim=await claimed(port,infoOf(w.root));
  expect(await claim.publish()).toBe(true);
  expect(w.sleeps.length).toBe(2);for(const waited of w.sleeps){expect(waited).toBeGreaterThan(14000);expect(waited).toBeLessThanOrEqual(15000);}
  expect(port.stats().published).toBe(1);expect(claim.deterministic()).toBe(false);
  expect(w.seen.filter(key=>key.startsWith("PUT /main/ac/")).length).toBe(1);
  const again=await w.port().claim(infoOf(w.root));expect(again.state).toBe("stored");
 }finally{await w.server.stop();}
});

test("a record that is still not stored after every try says why in both languages, is kept on disk, and a later resend stores it",async()=>{
 const w=await world([{when:/^PUT \/main\/ac\//u,answer:507,body:"Insufficient storage",times:4}]);
 try{
  const port=w.port(),claim=await claimed(port,infoOf(w.root));
  expect(await claim.publish()).toBe(false);
  expect(w.sleeps.length).toBe(3);expect(claim.code()).toBe("capacity");expect(claim.deterministic()).toBe(false);
  expect(claim.why()).toContain("HTTP 507");expect(claim.why()).toContain("Insufficient storage");expect(claim.why()).toContain("der Flottenspeicher hat keinen Platz mehr");expect(claim.why()).toContain("auf dieser Maschine");
  expect(port.stats().kept).toBe(1);
  const directory=join(w.root,".cache","f","owner-pool-unsent");expect(unsentListV1(directory).length).toBe(1);
  expect(await port.resend({force:true})).toBe(1);
  expect(unsentListV1(directory).length).toBe(0);expect(port.stats().resent).toBe(1);
  expect((await w.port().claim(infoOf(w.root))).state).toBe("stored");
 }finally{await w.server.stop();}
});

test("a refusal every machine would meet is deterministic, is sent once and is not kept",async()=>{
 const w=await world([{when:/^PUT \/main\/ac\//u,answer:413,body:"Entry exceeds the maximum entry size",times:9}]);
 try{
  const port=w.port(),claim=await claimed(port,infoOf(w.root));
  expect(await claim.publish()).toBe(false);
  expect(claim.deterministic()).toBe(true);expect(claim.code()).toBe("too-large");expect(w.sleeps).toEqual([]);expect(port.stats().kept).toBe(0);
  expect(w.seen.filter(key=>key.startsWith("PUT /main/ac/")).length).toBe(1);expect(claim.why()).toContain("HTTP 413");
 }finally{await w.server.stop();}
});

test("a divergence means another machine stored the key: success for the owner, counted, not retried",async()=>{
 const w=await world([{when:/^PUT \/main\/ac\//u,answer:409,body:"Conflict: another origin stored a different result for this key",times:9}]);
 try{
  const port=w.port(),claim=await claimed(port,infoOf(w.root));
  expect(await claim.publish()).toBe(true);expect(port.stats().diverged).toBe(1);expect(port.stats().published).toBe(0);expect(w.sleeps).toEqual([]);expect(claim.code()).toBe("diverged");
 }finally{await w.server.stop();}
});

test("the wait before a try covers the rest of the open breaker of the machine",async()=>{
 const w=await world([{when:/^PUT \/main\/cas\//u,answer:"network",times:1}]);
 try{
  const patched={...w.client,breakerRemainingMilliseconds:()=>15000} as GraphFleetClientV1,port=createProjectFleetPortV1({root:w.root,environment:{},client:patched,owner:"m:2",sleep:w.sleep,random:()=>0})!;
  const claim=await claimed(port,infoOf(w.root));
  expect(await claim.publish()).toBe(true);expect(w.sleeps).toEqual([15000]);
 }finally{await w.server.stop();}
});

test("a record another machine stored during the back-off is not a failure: the second send replaces nothing that matters and the owner counts as stored",async()=>{
 const w=await world([{when:/^PUT \/main\/cas\//u,answer:"timeout",times:1}]);
 try{
  const first=w.port(),second=w.port({owner:"m:other"}),claim=await claimed(first,infoOf(w.root));
  const hook=async(milliseconds:number)=>{await w.sleep(milliseconds);};
  expect(await claim.publish()).toBe(true);
  expect(w.sleeps.length).toBe(1);expect((await second.claim(infoOf(w.root))).state).toBe("stored");
  expect(existsSync(join(w.root,".cache","f","owner-pool-unsent"))).toBe(false);void hook;
 }finally{await w.server.stop();}
});

const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),commandRecipe={fn:"native-command"};
test("closure documents are taken in the order of their keys until the budget is used, the rest is left out and counted, and the record stays valid",async()=>{
 const w=await world([]);
 try{
  const root=w.root;mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"e.ts"),"export const e=1;\n");
  const row={kind:"file",path:`${root}/e.ts`,fact:{sha256:sha("e"),size:20,mtimeMs:1,ctimeMs:1,ino:1}},store=createClosureStoreV1({directory:join(root,"ce")}),recipes:Record<string,unknown>={};
  for(const name of["a","b","c"]){const recipe={fn:"native-command",name},key=closureRecipeKeyV1(recipe);recipes[key]=recipe;await store.save(`${root}\0${key}`,{recipe,value:["{workspaceRoot}/e.ts"],rows:[row],layers:[[row]],sidecar:{facts:[null],members:[null]}});}
  bindClosureStoreV1(root,join(root,"ce"),true);
  const wide=w.port(),info=infoOf(root,{closures:closureTravelV1,exportRecord:()=>({digest:"a".repeat(64),authority:[],observations:[row],image:JSON.stringify({name:"alpha",closureRecipes:recipes})})});
  const full=await claimed(wide,info);expect(await full.publish()).toBe(true);expect(wide.stats().shed).toBe(0);
  const tight=createProjectFleetPortV1({root:w.root,environment:{},client:w.client,owner:"m:tight",sleep:w.sleep,random:()=>0,policy:{...schema.definitions.Publish.const,closureBudgetBytes:1}})!;
  const other=infoOf(root,{keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/beta",{name:"beta"}]]),closures:closureTravelV1,exportRecord:()=>({digest:"b".repeat(64),authority:[],observations:[row],image:JSON.stringify({name:"beta",closureRecipes:recipes})})});
  const part=await claimed(tight,other);expect(await part.publish()).toBe(true);expect(tight.stats().shed).toBe(3);expect(tight.stats().published).toBe(1);
  void commandRecipe;
 }finally{await w.server.stop();}
});
