import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as execution from "../../../../🔧️tools/🏗️bazel/🏃️execution/🟦️.ts";
import {fleetCacheLocalPathV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";

const host={platform:process.platform,arch:process.arch};
const flags=(environment:Record<string,string>)=>execution.bazelExecutionFlagsV1({host,hermetic:true,jobs:3,environment});

test("with an executor the Bazel flags name its instance, switch compression off and keep the strategy list that includes remote; without one nothing remote is emitted",()=>{
 const local=execution.bazelExecutionFlagsV1({host,hermetic:true,jobs:3});
 expect(local.remote).toBe(false);expect(local.command.some(flag=>/remote_/u.test(flag))).toBe(false);
 const remote=flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:8980"});
 expect(remote.remote).toBe(true);expect(remote.command).toContain("--remote_executor=grpc://127.0.0.1:8980");expect(remote.command.includes("--noremote_local_fallback"),"a default pool of another platform cannot run the action here").toBe(execution.bazelExecutionPlatformV1(host.platform,host.arch).id!=="linux-x64");expect(remote.command).toContain("--remote_instance_name=main");expect(remote.command).toContain("--noremote_cache_compression");
 expect(flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpcs://build.example:443",SEMIO_REMOTE_EXECUTION_INSTANCE:"team-a"}).command).toContain("--remote_instance_name=team-a");
 expect(execution.bazelRemoteExecutionV1({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1"})?.instance).toBe("main");
 expect(()=>flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1",SEMIO_REMOTE_EXECUTION_INSTANCE:"Bad Name"})).toThrow();
 expect(new Set(remote.command).size).toBe(remote.command.length);
});

test("an HTTP cache cannot stay beside a gRPC executor: the cache endpoint is dropped, the credential helper moves to the executor host and every other cache flag survives",()=>{
 const cache=["--remote_cache=http://cache.example:6090","--remote_instance_name=main","--credential_helper=cache.example=/work/helper.sh","--remote_upload_local_results","--remote_download_outputs=toplevel","--remote_timeout=30s"];
 expect(execution.bazelExecutorCacheFlagsV1(cache,"grpc://127.0.0.1:8980")).toEqual(["--remote_instance_name=main","--credential_helper=127.0.0.1=/work/helper.sh","--remote_upload_local_results","--remote_download_outputs=toplevel","--remote_timeout=30s"]);
 expect(execution.bazelExecutorCacheFlagsV1(cache,"grpcs://[::1]:8980")).toContain("--credential_helper=::1=/work/helper.sh");
 expect(execution.bazelExecutorCacheFlagsV1(cache,"unix:/run/exec.sock")).toContain("--credential_helper=localhost=/work/helper.sh");
 expect(execution.bazelExecutorCacheFlagsV1(["--remote_cache=grpc://other:1","--remote_cache=grpcs://other:2"],"grpc://h:1")).toEqual(["--remote_cache=grpc://other:1","--remote_cache=grpcs://other:2"]);
 expect(execution.bazelExecutorCacheFlagsV1([],"grpc://h:1")).toEqual([]);
});

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const checkout=(endpoint?:string):string=>{
 const root=mkdtempSync(join(tmpdir(),"rexec-bazel-"));scratch.push(root);mkdirSync(join(root,".cache","f"),{recursive:true});
 const facts={version:1,group:"g1",generation:3,publishedAtMs:1,publisher:{directory:"/var/lib/semio-remote-cache",port:6090,host:"h1"},cache:{endpoints:["http://10.0.0.5:6090"],instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]},...endpoint===undefined?{}:{execution:{endpoint,protocolVersion:1}}};
 writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:null,facts}));
 return root;
};
const hostOf=execution.bazelExecutionPlatformV1(host.platform,host.arch);

test("the local fallback stays forbidden only where the local toolchain could not run the action: a pool of the host's platform falls back to local, another platform does not",()=>{
 const same=flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1",SEMIO_REMOTE_EXECUTION_PLATFORM:hostOf.id}),other=flags({SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://h:1",SEMIO_REMOTE_EXECUTION_PLATFORM:hostOf.id==="linux-x64"?"linux-arm64":"linux-x64"});
 expect(same.command).not.toContain("--noremote_local_fallback");expect(same.command).toContain("--remote_executor=grpc://h:1");
 expect(other.command).toContain("--noremote_local_fallback");
});

test("the Bazel executor comes from the same resolver as status and the offload: the explicit variable, else the endpoint the scheduler publishes in the fleet facts, routed to the host's own platform so that a failing scheduler falls back to local",()=>{
 const published=checkout("http://10.0.0.5:8980"),environment={SEMIO_WORKSPACE_ROOT:published,SEMIO_REMOTE_CACHE_TOKEN:"bearer-secret"};
 const fleet=flags(environment);
 expect(fleet).toMatchObject({remote:true,platform:hostOf.id});expect(fleet.command).toContain("--remote_executor=grpc://10.0.0.5:8980");expect(fleet.command).not.toContain("--noremote_local_fallback");
 expect(execution.bazelRemoteExecutionV1(environment)).toMatchObject({endpoint:"grpc://10.0.0.5:8980",platform:hostOf.id,instance:"main"});
 expect(flags({...environment,SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpcs://own.example:443"}).command).toContain("--remote_executor=grpcs://own.example:443");
 expect(flags({...environment,SEMIO_REMOTE_EXECUTION_PLATFORM:"linux-x64"}).platform,"an explicit platform still wins").toBe("linux-x64");
 expect(execution.bazelRemoteExecutionV1({SEMIO_WORKSPACE_ROOT:checkout("https://sched.example:8980"),SEMIO_REMOTE_CACHE_TOKEN:"t"})?.endpoint).toBe("grpcs://sched.example:8980");
 expect(execution.bazelRemoteExecutionV1({SEMIO_WORKSPACE_ROOT:published}),"the fleet scheduler accepts signed requests only: without a bearer token Bazel stays local and keeps its HTTP cache").toBeUndefined();
 expect(execution.bazelRemoteExecutionV1({SEMIO_WORKSPACE_ROOT:checkout(),SEMIO_REMOTE_CACHE_TOKEN:"t"}),"facts without a scheduler").toBeUndefined();
 expect(execution.bazelRemoteExecutionV1({}),"no root, no facts").toBeUndefined();
 expect(execution.bazelExecutionFlagsV1({host,hermetic:true,jobs:3,environment:{}}).remote).toBe(false);
});
