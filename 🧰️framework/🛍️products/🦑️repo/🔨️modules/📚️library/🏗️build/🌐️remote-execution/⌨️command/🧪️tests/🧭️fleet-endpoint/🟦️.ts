import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../../../../🏃️execution/🧬️schema/🌐️remote-execution.json";
import {fleetCacheLocalPathV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {eventuallyV1,startClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {remoteExecutionStatusV1,type RemoteExecutionCommandContextV1} from "../../🟦️.ts";

const scratch:string[]=[],ajv=new Ajv({strict:true,allErrors:true,validateFormats:false});
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const checkout=(execution?:{origin:string;port:number;protocolVersion:number},selectedExecution:string|null=null):string=>{
 const root=mkdtempSync(join(tmpdir(),"rexec-status-"));scratch.push(root);
 mkdirSync(join(root,".cache","f"),{recursive:true});
 const facts={version:1,group:"g1",generation:7,publishedAtMs:1,publisher:{origin:"g1-server",directory:"/var/lib/semio-remote-cache",port:6090},cache:{instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]},...execution?{execution}:{}};
 writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:null,candidates:[],selectedExecution,facts}));
 return root;
};
const context=(root:string,environment:Record<string,string|undefined>={}):RemoteExecutionCommandContextV1=>({root,environment:{PATH:process.env.PATH,...environment},signal:new AbortController().signal,progress:async()=>{},write:()=>{},spawnWorker:()=>{throw Error("no worker");}});
const valid=(value:unknown):boolean=>ajv.compile({$ref:"#/definitions/Status",definitions:schema.definitions as never} as never)(value) as boolean;

test("with fleet facts and no variable, status is configured, says the endpoint came from the fleet facts, probes the capabilities and reads the scheduler stats on the declared worker port",async()=>{
 const cluster=await startClusterV1({workers:1});
 try{
  await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,15000,"worker");
  const grpcPort=new URL(cluster.server.grpcUrl.replace(/^grpc/u,"http")).port,workerPort=new URL(cluster.server.workerUrl).port,endpoint=`http://127.0.0.1:${grpcPort}`;
  const root=checkout({origin:"g1-server",port:Number(grpcPort),protocolVersion:1},endpoint),status=await remoteExecutionStatusV1([],context(root,{SEMIO_REMOTE_EXECUTION_PORTS:`${grpcPort},${workerPort}`})) as any;
  expect(valid(status),JSON.stringify(status).slice(0,300)).toBe(true);
  expect(status).toMatchObject({configured:true,contract:{endpoint,instance:"main"},scheduler:`http://127.0.0.1:${workerPort}`,grpc:{capabilities:true,execution:true,probe:{reachable:true}},server:{service:"semio-remote-execution",workers:[{name:"worker-1"}]},resolution:{endpointFrom:"fleet-facts",schedulerFrom:"derived",factsGeneration:7}});
  const named=await remoteExecutionStatusV1([],context(root,{SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_EXECUTION_SCHEDULER:cluster.server.workerUrl})) as any;
  expect(named).toMatchObject({configured:true,contract:{endpoint:cluster.server.grpcUrl},resolution:{endpointFrom:"environment",schedulerFrom:"environment",factsGeneration:7}});
  expect(valid(named)).toBe(true);
 }finally{await cluster.stop();}
});

test("without a variable and without a published scheduler status stays unconfigured with no resolution, and a dead published scheduler is reported, not thrown",async()=>{
 const none=await remoteExecutionStatusV1([],context(checkout())) as any;
 expect(none).toMatchObject({configured:false,contract:null,scheduler:null,grpc:null,server:null,resolution:null});expect(valid(none)).toBe(true);
 const dead=await remoteExecutionStatusV1([],context(checkout({origin:"g1-server",port:9,protocolVersion:1},"http://127.0.0.1:9"))) as any;
 expect(dead).toMatchObject({configured:true,grpc:{capabilities:false,probe:{reachable:false}},server:null,resolution:{endpointFrom:"fleet-facts"}});expect(valid(dead)).toBe(true);
});
