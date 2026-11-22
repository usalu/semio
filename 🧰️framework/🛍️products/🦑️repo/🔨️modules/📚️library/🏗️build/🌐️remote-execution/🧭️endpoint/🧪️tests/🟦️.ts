import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetCacheLocalPathV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {resolveRemoteExecutionEndpointV1,resolveRemoteExecutionSchedulerV1} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const facts=(execution?:{endpoint:string;protocolVersion:number})=>({version:1,group:"g1",generation:7,publishedAtMs:1,publisher:{directory:"/var/lib/semio-remote-cache",port:6090,host:"h1"},cache:{endpoints:["http://10.0.0.5:6090"],instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]},...execution?{execution}:{}});
const checkout=(execution?:{endpoint:string;protocolVersion:number}):string=>{
 const root=mkdtempSync(join(tmpdir(),"rexec-endpoint-"));scratch.push(root);
 mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:null,facts:facts(execution)}));
 return root;
};

test("the endpoint comes from the explicit variable first, then from the fleet facts the scheduler publishes, and is absent otherwise",()=>{
 const published={endpoint:"http://10.0.0.5:8980",protocolVersion:1},root=checkout(published);
 expect(resolveRemoteExecutionEndpointV1({environment:{},root})).toEqual({endpoint:"http://10.0.0.5:8980",endpointFrom:"fleet-facts",protocolVersion:1,factsGeneration:7});
 expect(resolveRemoteExecutionEndpointV1({environment:{SEMIO_WORKSPACE_ROOT:root}})?.endpointFrom,"the root may come from the environment").toBe("fleet-facts");
 expect(resolveRemoteExecutionEndpointV1({environment:{SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1"},root})).toEqual({endpoint:"grpc://h:1",endpointFrom:"environment",protocolVersion:null,factsGeneration:7});
 expect(resolveRemoteExecutionEndpointV1({environment:{SEMIO_REMOTE_EXECUTION_ENDPOINT:""},root})?.endpointFrom,"an empty variable is unset").toBe("fleet-facts");
 expect(resolveRemoteExecutionEndpointV1({environment:{},root:checkout()}),"facts without a scheduler").toBeUndefined();
 expect(resolveRemoteExecutionEndpointV1({environment:{}}),"no root, no facts: a tool outside a build never reads the facts of the machine it runs on").toBeUndefined();
 expect(resolveRemoteExecutionEndpointV1({environment:{},root:join(tmpdir(),"rexec-endpoint-missing-"+process.pid)})).toBeUndefined();
});

test("the scheduler API is the explicit variable, else the endpoint host on the declared worker port (the environment ports win over the defaults)",async()=>{
 const root=checkout({endpoint:"http://10.0.0.5:8980",protocolVersion:1});
 expect(await resolveRemoteExecutionSchedulerV1({environment:{SEMIO_REMOTE_EXECUTION_SCHEDULER:"http://s:1"},root},"http://10.0.0.5:8980")).toEqual({url:"http://s:1",schedulerFrom:"environment"});
 expect(await resolveRemoteExecutionSchedulerV1({environment:{},root},"http://10.0.0.5:8980")).toEqual({url:"http://10.0.0.5:8981",schedulerFrom:"derived"});
 expect(await resolveRemoteExecutionSchedulerV1({environment:{SEMIO_REMOTE_EXECUTION_PORTS:"7000,7001"},root},"grpc://[::1]:7000")).toEqual({url:"http://[::1]:7001",schedulerFrom:"derived"});
 expect(await resolveRemoteExecutionSchedulerV1({environment:{},root},"not a url")).toBeUndefined();
});
