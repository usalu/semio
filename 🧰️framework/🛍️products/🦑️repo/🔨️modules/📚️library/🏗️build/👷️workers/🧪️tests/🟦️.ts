import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import buildPolicy from "../../🔣️policy.json" with {type:"json"};
import {parseBuildInvocationV1} from "../../🧭️invocation/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {WORKER_ACTION_KINDS_V1,planWorkersV1,readWorkerPolicyV1,readWorkerSwitchV1,requireWorkerPlanV1,workerActionKindV1,workerContractFieldsV1,type WorkerPlanV1,type WorkerPolicyV1} from "../🟦️.ts";
import {WorkerModuleCacheV1,WorkerProtocolErrorV1,encodeWorkResponseV1,installWorkerOutputCaptureV1,parseWorkRequestV1,serveWorkerV1,type WorkHandlerV1} from "../📨️protocol/🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),policy=readWorkerPolicyV1(buildPolicy.workers);

/** 🐍️ Evaluates the same fixture rows with Python's regular expressions and arithmetic, an implementation of the rules that shares nothing with the module under test. */
function python(program:string,input:unknown):any{
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",program],{input:JSON.stringify(input),encoding:"utf8"});
 if(run.error||run.status!==0)throw new Error("python3 oracle: "+(run.error?.message??run.stderr));
 return JSON.parse(run.stdout);
}

async function* chunks(rows:readonly string[],pause=0):AsyncGenerator<string>{for(const row of rows){if(pause)await Bun.sleep(pause);yield row;}}

test("the authored worker policy is valid under an independent validator and every broken variant is refused by both validators",()=>{
 const validate=validator("Policy");
 expect(validate(buildPolicy.workers),JSON.stringify(validate.errors)).toBe(true);
 expect(readWorkerPolicyV1(buildPolicy.workers).version).toBe(1);
 for(const row of fixture.invalidPolicies){
  const broken=structuredClone(buildPolicy.workers) as Record<string,any>;
  for(const[key,value]of Object.entries((row as any).set??{}))broken[key]=value;
  for(const[path,value]of Object.entries((row as any).setIn??{})){const parts=path.split(".");let at=broken;for(const part of parts.slice(0,-1))at=at[part];at[parts.at(-1)!]=value;}
  expect(validate(broken),row.name).toBe(false);
  expect(()=>readWorkerPolicyV1(broken),row.name).toThrow();
 }
 const pattern=structuredClone(buildPolicy.workers) as any;pattern.classification[1].pattern="(";expect(()=>readWorkerPolicyV1(pattern)).toThrow();
});

test("every task is classified as Python's regular expressions classify it",()=>{
 const rows=fixture.classification;
 for(const row of rows)expect(workerActionKindV1(policy,row.target,row.continuous),row.target).toBe(row.kind);
 const oracle=python(`
import json,re,sys
policy=json.load(sys.stdin)
def classify(target,continuous):
    for rule in policy['rules']:
        if 'continuous' in rule:
            if rule['continuous']==continuous: return rule['kind']
        elif re.search(rule['pattern'],target): return rule['kind']
    return policy['default']
print(json.dumps([classify(r['target'],r['continuous']) for r in policy['rows']]))`,{rules:buildPolicy.workers.classification,default:buildPolicy.workers.defaultKind,rows});
 expect(oracle).toEqual(rows.map(row=>row.kind));
 expect(new Set(rows.map(row=>row.kind))).toEqual(new Set(WORKER_ACTION_KINDS_V1));
});

test("a plan is the policy resolved for the host: scalar budgets against an SQL oracle, kinds against the fixture",()=>{
 const validate=validator("Plan"),database=new Database(":memory:");
 try{
  for(const row of fixture.plans){
   const plan=planWorkersV1(policy,row.host,row.overrides);
   expect(plan,row.name).toEqual(row.expected as WorkerPlanV1);
   expect(validate(plan),row.name+JSON.stringify(validate.errors)).toBe(true);
   const sql=database.query("SELECT MAX(64,CAST(?1/1048576.0*?2/10000 AS INTEGER)) AS total").get(row.host.memoryBytes,policy.memory.hostShareBasisPoints) as {total:number},instances=database.query("SELECT MIN(1024,MAX(1,?1/?2)) AS instances").get(sql.total,policy.memory.perWorkerMegabytes) as {instances:number};
   expect(plan.memory.totalMegabytes,row.name).toBe(sql.total);
   expect(plan.memory.instances,row.name).toBe(instances.instances);
   expect(plan.localJobs,row.name).toBe(row.overrides.jobs??Math.max(1,Math.min(row.host.cores,policy.localJobs.maximum)));
   const used=WORKER_ACTION_KINDS_V1.reduce((sum,kind)=>sum+(plan.kinds[kind].workers?plan.kinds[kind].maxInstances:0),0);
   expect(used,row.name).toBeLessThanOrEqual(Math.max(plan.memory.instances,WORKER_ACTION_KINDS_V1.filter(kind=>plan.kinds[kind].workers).length));
   if(row.overrides.workers===false)expect(WORKER_ACTION_KINDS_V1.some(kind=>plan.kinds[kind].workers)).toBe(false);
   expect(plan.kinds.service.workers).toBe(false);
  }
 }finally{database.close();}
 expect(()=>requireWorkerPlanV1({...planWorkersV1(policy,{cores:2,memoryBytes:2**33}),localJobs:0})).toThrow();
 expect(()=>planWorkersV1(policy,{cores:2,memoryBytes:2**33},{jobs:0})).toThrow("positive integer");
});

test("the contract names every field a tool maps, once",()=>{
 const fields=workerContractFieldsV1();
 expect(new Set(fields).size).toBe(fields.length);
 expect(fields.length).toBe(7+WORKER_ACTION_KINDS_V1.length*4);
});

test("the worker switch is on, off or absent and the build entry reads it as a global flag like the tool",()=>{
 expect(readWorkerSwitchV1(undefined)).toBeUndefined();expect(readWorkerSwitchV1("")).toBeUndefined();expect(readWorkerSwitchV1("on")).toBe(true);expect(readWorkerSwitchV1("off")).toBe(false);
 for(const value of fixture.invalidSwitches)expect(()=>readWorkerSwitchV1(value),value).toThrow("on or off");
 for(const row of fixture.invocation){
  const parsed=parseBuildInvocationV1(row.args,row.environment as NodeJS.ProcessEnv);
  expect((parsed as {workers?:string}).workers,JSON.stringify(row.args)).toBe(row.workers??undefined);
  expect(parsed.args,JSON.stringify(row.args)).toEqual(row.rest??row.args);
 }
 expect(()=>parseBuildInvocationV1(["run","a:b","--workers=maybe"],{})).toThrow("on or off");
});

test("requests are parsed and responses encoded as Python's json module reads and writes them",()=>{
 const request=validator("WorkRequest"),response=validator("WorkResponse");
 for(const row of fixture.frames.valid){
  expect(parseWorkRequestV1(row.line),row.name).toEqual(row.expected as never);
  expect(request(JSON.parse(row.line)),row.name).toBe(true);
 }
 const oracle=python(`
import json,sys
print(json.dumps([json.loads(line) for line in json.load(sys.stdin)],ensure_ascii=False))`,fixture.frames.valid.map(row=>row.line));
 oracle.forEach((value:any,index:number)=>{const parsed=parseWorkRequestV1(fixture.frames.valid[index]!.line);expect(parsed.arguments,String(index)).toEqual(value.arguments??[]);expect(parsed.inputs.map(input=>[input.path,input.digest]),String(index)).toEqual((value.inputs??[]).map((input:any)=>[input.path,input.digest]));expect(parsed.requestId,String(index)).toBe(value.requestId??0);});
 for(const row of fixture.frames.invalid)expect(()=>parseWorkRequestV1(row.line),row.name).toThrow(WorkerProtocolErrorV1);
 for(const row of fixture.frames.responses){
  const line=encodeWorkResponseV1(row.response);
  expect(line,row.name).toBe(row.line);expect(line.endsWith("\n")&&line.indexOf("\n")===line.length-1,row.name).toBe(true);
  expect(response(JSON.parse(line)),row.name).toBe(true);
  expect(python(`import json,sys\nprint(json.dumps(json.loads(json.load(sys.stdin)),ensure_ascii=False))`,line)).toEqual(row.response);
 }
 const cut=fixture.frames.truncation,encoded=JSON.parse(encodeWorkResponseV1({exitCode:0,output:cut.output,requestId:0},cut.maximumOutputBytes));
 expect(Buffer.byteLength(encoded.output)).toBeLessThanOrEqual(cut.maximumOutputBytes);expect(encoded.output.endsWith(cut.marker)).toBe(true);expect(encoded.output.includes("�")).toBe(false);
});

test("the serve loop answers the requests Bazel 8.4.2 recorded exactly like the worker that recorded them",async()=>{
 const captured=fixture.bazel.captures,byId=(left:{requestId:number},right:{requestId:number})=>left.requestId-right.requestId;
 for(const name of ["singleplex","multiplex"] as const){
  const requests=captured[name].stdin.join("").split("\n").filter(Boolean).map(row=>parseWorkRequestV1(row)),expected=requests.map(request=>({exitCode:0,output:"ok "+request.arguments[1],requestId:request.requestId})).sort(byId);
  const out:string[]=[],handler:WorkHandlerV1=async request=>({exitCode:0,output:"ok "+request.arguments[1]});
  const result=await serveWorkerV1({input:chunks(captured[name].stdin),write:text=>{out.push(text);},handler,concurrency:name==="multiplex"?3:1});
  expect(result.requests,name).toBe(requests.length);
  expect(out.map(line=>JSON.parse(line)).sort(byId),name).toEqual(expected);
  expect([...captured[name].responses].sort(byId),name).toEqual(expected);
 }
 const out:string[]=[],handler:WorkHandlerV1=(request,{signal})=>new Promise(accept=>{signal.addEventListener("abort",()=>accept({exitCode:0,output:"cancelled"}),{once:true});setTimeout(()=>accept({exitCode:0,output:"too late"}),2000);});
 const result=await serveWorkerV1({input:chunks(captured.cancel.stdin),write:text=>{out.push(text);},handler});
 expect(out.map(line=>JSON.parse(line))).toEqual(captured.cancel.responses);
 expect(result.cancelled).toBe(1);
});

test("the serve loop frames lines across chunks, runs requests concurrently up to its bound, cancels queued and running requests and reports handler failures",async()=>{
 const line=(id:number,extra:object={})=>JSON.stringify({arguments:["t"+id],requestId:id,...extra})+"\n",whole=line(1)+line(2)+line(3),split=[whole.slice(0,25),whole.slice(25,70),whole.slice(70)];
 let running=0,peak=0;const gates=new Map<number,()=>void>(),order:number[]=[];
 const handler:WorkHandlerV1=async request=>{running++;peak=Math.max(peak,running);await new Promise<void>(accept=>gates.set(request.requestId,accept));running--;order.push(request.requestId);return{exitCode:request.requestId===2?7:0,output:"r"+request.requestId};};
 const out:string[]=[],input=(async function*(){yield split[0]!;yield split[1]!;yield split[2]!;await Bun.sleep(80);gates.get(2)!();await Bun.sleep(40);gates.get(1)!();await Bun.sleep(40);gates.get(3)!();})();
 const result=await serveWorkerV1({input,write:text=>{out.push(text);},handler,concurrency:2});
 expect(peak).toBe(2);expect(order).toEqual([2,1,3]);expect(out.map(row=>JSON.parse(row))).toEqual([{exitCode:7,output:"r2",requestId:2},{exitCode:0,output:"r1",requestId:1},{exitCode:0,output:"r3",requestId:3}]);expect(result.requests).toBe(3);
 const cancelled:string[]=[],slow:WorkHandlerV1=(request,{signal})=>new Promise(accept=>{signal.addEventListener("abort",()=>accept({exitCode:0,output:"stopped "+request.requestId}),{once:true});});
 await serveWorkerV1({input:chunks([line(1)+line(2)+line(2,{cancel:true,arguments:[]})+line(1,{cancel:true,arguments:[]}).replace('"arguments":[],','')]),write:text=>{cancelled.push(text);},handler:slow});
 expect(cancelled.map(row=>JSON.parse(row)).sort((left,right)=>left.requestId-right.requestId)).toEqual([{exitCode:0,output:"stopped 1",requestId:1,wasCancelled:true},{exitCode:0,output:"",requestId:2,wasCancelled:true}]);
 const failures:string[]=[];
 await serveWorkerV1({input:chunks([line(5)+'{"arguments":["tail"],"requestId":6}']),write:text=>{failures.push(text);},handler:async request=>{if(request.requestId===5)throw new Error("boom");return{exitCode:0,output:"fine"};}});
 expect(failures.map(row=>JSON.parse(row))).toEqual([{exitCode:1,output:"boom",requestId:5},{exitCode:0,output:"fine",requestId:6}]);
 const crlf:string[]=[];
 await serveWorkerV1({input:chunks(['{"arguments":["a"],"requestId":1}\r\n']),write:text=>{crlf.push(text);},handler:async()=>({exitCode:0,output:"crlf"})});
 expect(crlf.length).toBe(1);
 await expect(serveWorkerV1({input:chunks(["not json\n"]),write:()=>{},handler:async()=>({exitCode:0,output:""})})).rejects.toBeInstanceOf(WorkerProtocolErrorV1);
 await expect(serveWorkerV1({input:chunks(["x".repeat(64)]),write:()=>{},handler:async()=>({exitCode:0,output:""}),maximumRequestBytes:32})).rejects.toBeInstanceOf(WorkerProtocolErrorV1);
});

test("closing the input lets every accepted request finish while a protocol failure aborts the running ones",async()=>{
 const out:string[]=[],hooks:number[]=[];
 await serveWorkerV1({input:chunks(['{"requestId":1}\n{"requestId":2}\n']),write:text=>{out.push(text);},handler:async request=>{await Bun.sleep(5);return{exitCode:0,output:"done "+request.requestId};},afterRequest:served=>{hooks.push(served);}});
 expect(out.map(row=>JSON.parse(row).output)).toEqual(["done 1","done 2"]);expect(hooks).toEqual([1,2]);
 let aborted=false;
 const handler:WorkHandlerV1=(_request,{signal})=>new Promise(accept=>{signal.addEventListener("abort",()=>{aborted=true;accept({exitCode:1,output:"stopped"});},{once:true});});
 await expect(serveWorkerV1({input:chunks(['{"requestId":3}\n','not json\n']),write:()=>{},handler})).rejects.toBeInstanceOf(WorkerProtocolErrorV1);
 expect(aborted).toBe(true);
});

test("standard output and error writes of a request become its output while the protocol stream stays clean",async()=>{
 const written:string[]=[],stream=(name:string)=>({name,write:(chunk:unknown,encoding?:unknown,callback?:unknown)=>{written.push(name+":"+String(chunk));const done=typeof encoding==="function"?encoding:callback;if(typeof done==="function")(done as () => void)();return true;}}),stdout=stream("out"),stderr=stream("err"),originalOut=stdout.write,originalErr=stderr.write;
 const capture=installWorkerOutputCaptureV1(stdout as never,stderr as never);
 try{
  const result=await capture.run(async()=>{stdout.write("a\n");stderr.write(Buffer.from("b\n"));await Bun.sleep(1);stdout.write("c",()=>{});return 7;});
  expect(result).toEqual({value:7,output:"a\nb\nc"});
  stdout.write("outside");expect(written.at(-1)).toBe("err:outside");
  await capture.writeProtocol('{"x":1}\n');expect(written.at(-1)).toBe('out:{"x":1}\n');
 }finally{capture.restore();}
 expect(stdout.write).toBe(originalOut);expect(stderr.write).toBe(originalErr);
});

test("loaded modules are reused until a declared input changes, then every module the cache loaded is dropped and nothing else",async()=>{
 const root=mkdtempSync(join(tmpdir(),"worker-modules-")),state=globalThis as unknown as {__workerLoads?:number};
 try{
  mkdirSync(join(root,"node_modules","vendor"),{recursive:true});
  writeFileSync(join(root,"node_modules","vendor","index.js"),"globalThis.__workerVendorLoads=(globalThis.__workerVendorLoads??0)+1;export const vendor=1;");
  const write=(value:number)=>writeFileSync(join(root,"dep.ts"),`globalThis.__workerLoads=(globalThis.__workerLoads??0)+1;export const value=${value};`);
  write(1);writeFileSync(join(root,"entry.ts"),`import {value} from "./dep.ts";import {vendor} from "vendor";export const run=()=>value+vendor;`);
  state.__workerLoads=0;
  const cache=new WorkerModuleCacheV1(root),entry=join(root,"entry.ts"),inputs=(digest:string)=>[{path:"dep.ts",digest},{path:"entry.ts",digest:"e"}];
  const first=await cache.load<{run():number}>(entry,inputs("d1"));expect(first.module.run()).toBe(2);expect(first.reset).toBe(false);
  const second=await cache.load<{run():number}>(entry,inputs("d1"));expect(second.reset).toBe(false);expect(state.__workerLoads).toBe(1);
  write(5);
  const stale=await cache.load<{run():number}>(entry,inputs("d1"));expect(stale.reset).toBe(false);expect(stale.module.run()).toBe(2);
  const third=await cache.load<{run():number}>(entry,inputs("d2"));expect(third.reset).toBe(true);expect(third.module.run()).toBe(6);expect(state.__workerLoads).toBe(2);
  expect((globalThis as unknown as {__workerVendorLoads?:number}).__workerVendorLoads).toBe(1);
  expect(cache.reset()).toBeGreaterThan(0);
  const undeclared=await cache.load<{run():number}>(entry,[]);expect(undeclared.module.run()).toBe(6);
  await Bun.sleep(15);write(9);
  const touched=await cache.load<{run():number}>(entry,[]);expect(touched.reset).toBe(true);expect(touched.module.run()).toBe(10);
 }finally{rmSync(root,{recursive:true,force:true});}
});
