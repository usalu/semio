import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import schema from "../../🧬️schema/🐳️remote-execution-service.json" with {type:"json"};
import contract from "../../../🔧️tools/🏗️bazel/🏃️execution/🧬️schema/🔣️.json" with {type:"json"};
import service from "../🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {parseRemoteExecutionContainersV1,readRemoteExecutionServiceV1,renderRemoteExecutionServiceV1,remoteExecutionServicePlatformV1,runRemoteExecutionCommandV1,parseRemoteExecutionFlagsV1,writeRemoteExecutionStackV1,type RemoteExecutionCommandContextV1,type RemoteExecutionComposeV1,type RemoteExecutionRuntimeV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),python=testPythonV1(),docker=Bun.which("docker");
const stack=fileURLToPath(new URL("../🐳️stack/",import.meta.url)),scratch:string[]=[];
const read=(path:string)=>JSON.parse(readFileSync(join(stack,path),"utf8"));
const reference=(image:{repository:string;tag:string;digest:string})=>`${image.repository}:${image.tag}@${image.digest}`;
const pipe=(script:string,input:unknown)=>{const run=spawnSync(python!,["-I","-c",script],{input:JSON.stringify(input),encoding:"utf8"});if(run.status!==0)throw Error(run.stderr);return JSON.parse(run.stdout);};
const context=(extra:Partial<RemoteExecutionCommandContextV1>={}):RemoteExecutionCommandContextV1&{lines:string[];stages:string[]}=>{
 const lines:string[]=[],stages:string[]=[];
 return{root:"/nowhere",environment:{},host:{platform:"darwin",arch:"arm64"},signal:new AbortController().signal,progress:async stage=>{stages.push(stage);},write:line=>{lines.push(line);},lines,stages,...extra};
};

test("the authored service, the schema and the packaged compose file agree with an independent JSON-schema implementation and Python",()=>{
 expect(validator("Service")(service),JSON.stringify(validator("Service").errors)).toBe(true);
 expect(readRemoteExecutionServiceV1()).toEqual(service);
 const compose=readFileSync(join(stack,"compose.yaml"),"utf8"),anchors=[...compose.matchAll(/^x-[a-z-]+-image: &[a-z-]+ (\S+)$/gmu)].map(match=>match[1]!).sort();
 expect(anchors).toEqual(service.images.map(reference).sort());
 for(const image of service.images)expect(image.digest).toMatch(/^sha256:[a-f0-9]{64}$/u);
 if(python){
  const verdict=pipe("import json,re,sys\nd=json.load(sys.stdin)\nanchors=sorted(re.findall(r'^x-[a-z-]+-image: &[a-z-]+ (\\S+)$',d['compose'],re.M))\nrefs=sorted(f\"{i['repository']}:{i['tag']}@{i['digest']}\" for i in d['service']['images'])\nprint(json.dumps({'same':anchors==refs,'pinned':all(re.fullmatch(r'sha256:[a-f0-9]{64}',i['digest']) for i in d['service']['images']),'unique':len({s for i in d['service']['images'] for s in i['services']})==sum(len(i['services']) for i in d['service']['images']),'loopback':d['service']['host']=='127.0.0.1'}))",{compose,service});
  expect(verdict).toEqual({same:true,pinned:true,unique:true,loopback:true});
 }
 for(const row of fixture.refusedServices){
  const copy=structuredClone(service) as Record<string,any>;let target=copy;
  for(const key of row.path.slice(0,-1))target=target[key as string];
  target[row.path.at(-1) as string]=row.value;
  expect(()=>readRemoteExecutionServiceV1(copy),row.name).toThrow("Closed remote execution service required");
  expect(validator("Service")(copy),row.name).toBe(false);
 }
});

test.skipIf(!docker)("docker compose itself resolves the stack to the pinned images, loopback ports and an isolated runner",()=>{
 const run=spawnSync(docker!,["compose","-f",join(stack,"compose.yaml"),"config","--format","json"],{encoding:"utf8",env:{...process.env,SEMIO_RBE_PLATFORM:"linux-arm64",SEMIO_RBE_DOCKER_PLATFORM:"linux/arm64"}});
 expect(run.status,run.stderr).toBe(0);
 const resolved=JSON.parse(run.stdout) as{name:string;services:Record<string,{image:string;network_mode?:string;ports?:{host_ip?:string;published:string;target:number}[];platform?:string}>};
 expect(resolved.name).toBe(service.project);
 const expected=Object.fromEntries(service.images.flatMap(image=>image.services.map(name=>[name,reference(image)])));
 expect(Object.fromEntries(Object.entries(resolved.services).map(([name,row])=>[name,row.image]))).toEqual(expected);
 for(const row of Object.values(resolved.services))for(const published of row.ports??[])expect(published.host_ip).toBe("127.0.0.1");
 expect(resolved.services.runner!.network_mode).toBe("none");
 expect(resolved.services.runner!.platform).toBe("linux/arm64");
 expect(resolved.services.frontend!.ports!.map(row=>[row.published,row.target]).sort()).toEqual([["8980",8980],["8981",80]]);
});

test("each worker serves exactly the properties of its execution platform in the remote execution contract and the service instance",()=>{
 const platforms=(contract.definitions.Contract.const as{platforms:{id:string;properties:Record<string,string>}[]}).platforms;
 const rows=service.platforms.map(id=>{
  const worker=read(`config/worker-${id}.json`),runner=worker.buildDirectories[0].runners[0];
  return{id,properties:Object.fromEntries(runner.platform.properties.map((row:{name:string;value:string})=>[row.name,row.value])),instance:runner.instanceNamePrefix,expected:platforms.find(row=>row.id===id)!.properties};
 });
 for(const row of rows){expect(row.properties,row.id).toEqual(row.expected);expect(row.instance,row.id).toBe(service.instance);}
 if(python)expect(pipe("import json,sys\nrows=json.load(sys.stdin)\nprint(json.dumps(all(r['properties']==r['expected'] and r['instance']=='main' for r in rows)))",rows)).toBe(true);
 for(const name of ["frontend","scheduler","storage","runner"])expect(existsSync(join(stack,`config/${name}.json`)),name).toBe(true);
 expect(readdirSync(join(stack,"config")).sort()).toEqual(["frontend.json","runner.json","scheduler.json","storage.json",...service.platforms.map(id=>`worker-${id}.json`)].sort());
});

test("rendering rows resolve the platform, ports, compose variables and the client variables of the remote execution contract",async()=>{
 for(const row of fixture.rendering){
  const flags=parseRemoteExecutionFlagsV1(row.args),platform=remoteExecutionServicePlatformV1(service,row.host,flags["--platform"]),rendering=renderRemoteExecutionServiceV1(service,{platform,...(flags["--port"]?{port:Number(flags["--port"])}:{}),...(flags["--diagnostics-port"]?{diagnosticsPort:Number(flags["--diagnostics-port"])}:{})});
  expect(platform,row.name).toBe(row.platform);
  expect(rendering.compose.environment,row.name).toEqual(row.compose);
  expect(rendering.client,row.name).toEqual(row.client);
  expect(validator("Rendering")(rendering),row.name+JSON.stringify(validator("Rendering").errors)).toBe(true);
  const names=(contract.definitions.Contract.const as{environment:Record<string,string>}).environment;
  for(const name of Object.keys(rendering.client))expect(Object.values(names),name).toContain(name);
 }
 for(const row of fixture.refusedFlags)await expect(runRemoteExecutionCommandV1(["env",...row.args],context()),row.name).rejects.toThrow(row.message);
});

test("env prints the client variables and an unknown subcommand is refused",async()=>{
 const run=context();
 await runRemoteExecutionCommandV1(["env"],run);
 expect(run.lines).toEqual(["SEMIO_REMOTE_EXECUTION_ENDPOINT=grpc://127.0.0.1:8980","SEMIO_REMOTE_EXECUTION_PLATFORM=linux-arm64","SEMIO_REMOTE_EXECUTION_INSTANCE=main"]);
 await expect(runRemoteExecutionCommandV1(["bounce"],context())).rejects.toThrow("Unknown remote-execution command: bounce");
});

test("container rows parse the compose listing in both output shapes",()=>{
 for(const row of fixture.containers)expect(parseRemoteExecutionContainersV1(row.output),row.name).toEqual(row.expected);
});

const double=(options:{exitUp?:number}={}):{runtime:RemoteExecutionRuntimeV1;calls:{args:string[];plan:RemoteExecutionComposeV1}[]}=>{
 const calls:{args:string[];plan:RemoteExecutionComposeV1}[]=[];
 return{calls,runtime:{
  probe:async()=>({available:true,version:"29.0.0"}),
  compose:async(plan,args,run)=>{
   calls.push({args:[...args],plan});
   if(args[0]!=="up")return{code:0,output:""};
   run.onLine?.("storage-1  | started");
   if(options.exitUp!==undefined)return{code:options.exitUp,output:""};
   await new Promise<void>(resolve=>run.signal?.addEventListener("abort",()=>resolve(),{once:true}));
   return{code:143,output:""};
  }
 }};
};

test("serve renders the stack, waits for the frontend, prints the client variables and removes the cluster when cancelled",async()=>{
 const root=mkdtempSync(join(tmpdir(),"rbe-")),controller=new AbortController(),run=context({root,signal:controller.signal}),{runtime,calls}=double();scratch.push(root);
 let probes=0;
 const fetchDouble=(async()=>new Response("ok",{status:++probes<3?503:200})) as unknown as typeof fetch;
 const write=run.write;
 const served=runRemoteExecutionCommandV1(["serve","--port","9200","--diagnostics-port","9201"],{...run,runtime,fetch:fetchDouble,write:line=>{write(line);if(line.startsWith("[rbe] listening"))controller.abort();}});
 await served;
 expect(calls.map(call=>call.args)).toEqual([["up","--remove-orphans"],["down","--volumes","--remove-orphans"]]);
 expect(calls[0]!.plan.environment).toEqual({SEMIO_RBE_PLATFORM:"linux-arm64",SEMIO_RBE_DOCKER_PLATFORM:"linux/arm64",SEMIO_RBE_PORT:"9200",SEMIO_RBE_DIAGNOSTICS_PORT:"9201"});
 expect(probes).toBe(3);
 expect(run.stages).toEqual(["remote-execution:preflight","remote-execution:render","remote-execution:up","remote-execution:serving","remote-execution:down"]);
 expect(run.lines).toContain("[rbe] listening grpc://127.0.0.1:9200 (linux-arm64, instance main)");
 expect(run.lines).toContain("SEMIO_REMOTE_EXECUTION_ENDPOINT=grpc://127.0.0.1:9200");
 expect(run.lines.at(-1)).toBe("[rbe] stopped");
 expect(existsSync(join(root,".cache/remote-execution/compose.yaml"))).toBe(true);
});

test("serve refuses without a container runtime and fails when the cluster exits before it serves",async()=>{
 const missing:RemoteExecutionRuntimeV1={probe:async()=>({available:false,reason:"Cannot connect to the Docker daemon"}),compose:async()=>({code:0,output:""})};
 await expect(runRemoteExecutionCommandV1(["serve"],{...context(),runtime:missing})).rejects.toThrow("Container runtime unavailable: Cannot connect to the Docker daemon");
 const root=mkdtempSync(join(tmpdir(),"rbe-"));scratch.push(root);
 const{runtime,calls}=double({exitUp:1}),run=context({root});
 await expect(runRemoteExecutionCommandV1(["serve"],{...run,runtime,fetch:(async()=>new Response("",{status:503})) as unknown as typeof fetch})).rejects.toThrow("exited with 1 before it was ready");
 expect(calls.map(call=>call.args[0])).toEqual(["up","down"]);
});

test("status reports the runtime, the containers and the endpoint without starting anything",async()=>{
 const{runtime}=double(),listing:RemoteExecutionRuntimeV1={probe:runtime.probe,compose:async()=>({code:0,output:"{\"Service\":\"worker\",\"State\":\"running\"}\n"})};
 const status=await runRemoteExecutionCommandV1(["status"],{...context(),runtime:listing,fetch:(async()=>new Response("ok")) as unknown as typeof fetch}) as Record<string,unknown>;
 expect(validator("Status")(status),JSON.stringify(validator("Status").errors)).toBe(true);
 expect(status).toEqual({version:1,service:"semio-rbe",runtime:{available:true,version:"29.0.0"},containers:[{service:"worker",state:"running"}],endpoint:{url:"grpc://127.0.0.1:8980",answers:true},client:{SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:8980",SEMIO_REMOTE_EXECUTION_PLATFORM:"linux-arm64",SEMIO_REMOTE_EXECUTION_INSTANCE:"main"}});
 const down=await runRemoteExecutionCommandV1(["status"],{...context(),runtime:{probe:async()=>({available:false,reason:"no daemon"}),compose:async()=>{throw Error("must not run");}},fetch:(async()=>{throw Error("refused");}) as unknown as typeof fetch}) as {runtime:object;containers:unknown[];endpoint:{answers:boolean}};
 expect(down.runtime).toEqual({available:false,reason:"no daemon"});expect(down.containers).toEqual([]);expect(down.endpoint.answers).toBe(false);
});

test("the packaged stack is copied byte for byte to an ASCII-only runtime directory",async()=>{
 const root=mkdtempSync(join(tmpdir(),"rbe-"));scratch.push(root);
 const plan=await writeRemoteExecutionStackV1(root,service,renderRemoteExecutionServiceV1(service,{platform:"linux-x64"}));
 expect(plan.directory).toBe(join(root,".cache/remote-execution"));expect(plan.directory).toMatch(/^[\x20-\x7e]+$/u);
 for(const path of ["compose.yaml","config/storage.json","config/worker-linux-x64.json"])expect(readFileSync(join(plan.directory,path)).equals(readFileSync(join(stack,path))),path).toBe(true);
 expect(plan.environment.SEMIO_RBE_DOCKER_PLATFORM).toBe("linux/amd64");
});

test("cleanup",()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
