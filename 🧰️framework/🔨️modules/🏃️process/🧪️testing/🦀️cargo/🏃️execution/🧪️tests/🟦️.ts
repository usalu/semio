import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";
import {executeCargoRequestV1,readCargoExecutionContractV1,selectCargoExecutionOperationV1,type CargoExecutionRequestV1,type CargoExecutionDeclarationV1} from "../🟦️.ts";
import type {ScriptInvocation} from "../../../../🧭️routing/📥️invocation/🟦️.ts";
import {mkdirSync,mkdtempSync,writeFileSync,existsSync} from "node:fs";
import {join,resolve} from "node:path";
import cargoCorpus from "../../🧪️tests/🧫️fixtures/🔣️.json";
import {parseCommandArgumentsV1,resolveCommandConfigurationV1} from "../../../../🧭️routing/🎛️command/⚙️configuration/🟦️.ts";
import {executeCommandV1} from "../../../../🧭️routing/🎛️command/🟦️.ts";
import {readCargoTestPolicyV1} from "../../🟦️.ts";
import {captureOwnedProcess} from "../../../../📥️capture/🟦️.ts";

test("Cargo execution portable contracts agree with independent strict Ajv",()=>{
  const ajv=new Ajv({strict:true}).addSchema(schema);
  for(const row of corpus.cases){
    expect(ajv.compile({$ref:schema.$id+"#/$defs/CargoExecutionRequestV1"})(row.value),row.id).toBe(row.accepted);
    let accepted=true;try{readCargoExecutionContractV1(row.definition as "request",row.value);}catch{accepted=false;}
    expect(accepted,row.id).toBe(row.accepted);
  }
});
test("Cargo execution command delegation retains original resources and refuses foreign selections",async()=>{
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned Cargo execution fixture directory required");mkdirSync(output,{recursive:true});
  const root=mkdtempSync(join(resolve(output),"delegation-")),owner=join(root,"kernel"),configuration=join(root,"configuration.json");mkdirSync(owner);mkdirSync(join(root,"other"));
  writeFileSync(join(root,"Cargo.toml"),'[workspace]\nmembers=["kernel"]\n');writeFileSync(join(owner,"Cargo.toml"),'[package]\nname="kernel"\nversion="0.0.0"\nworkspace=".."\n');writeFileSync(configuration,JSON.stringify(corpus.configuration));writeFileSync(join(root,"nextest.toml"),"[profile.default]\n");
  const source={file:{path:join(owner,"Cargo.toml"),device:"1",inode:"2"},ancestors:[{path:root,device:"1",inode:"1"}],sha256:"0".repeat(64),bytes:10},backend={version:1,program:"runtime",arguments:["endpoint","execute"],cwd:root,sources:[source,source],request:corpus.request,continuation:{program:process.execPath,cwd:owner,arguments:["script","test",...corpus.request.arguments]}};
  const originalPolicy=readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify({...cargoCorpus.policies[0],manifestPath:join(owner,"Cargo.toml"),targetDirectory:join(root,"issued-target"),buildDirectory:join(root,"issued-build"),leaseDirectory:join(root,"issued-leases"),artifactDirectory:join(root,"captures")})});
  const environment:Record<string,string|undefined>={SEMIO_BUILD_MODE:"prototype",CARGO_TARGET_DIR:originalPolicy.targetDirectory,CARGO_BUILD_BUILD_DIR:originalPolicy.buildDirectory,SEMIO_CARGO_TEST_POLICY:JSON.stringify(originalPolicy),SEMIO_TEST_ARTIFACT_DIR:join(root,"captures")};
  const invocation:ScriptInvocation={policy:{version:1,owner:"neutral-command-delegation",maximumElapsedMilliseconds:1000},capabilities:{cargoExecutionBackend:backend},control:{signal:new AbortController().signal,remainingMilliseconds:()=>1000,async publish(){},async yieldContinuation(){}}};
  const selected=parseCommandArgumentsV1(["--config",configuration,"--cwd","kernel","--manifest","kernel/Cargo.toml","--",process.execPath,"script","test",...corpus.request.arguments]);
  for(const row of corpus.commandCases){
    const args=structuredClone(selected),env={...environment};
    if(row.id==="foreign-manifest")Object.assign(args,{manifests:["other/Cargo.toml"]});
    if(row.id==="foreign-cwd")Object.assign(args,{cwd:"other"});
    if(row.id==="foreign-arguments")Object.assign(args,{args:["script","test","quick"]});
    if(row.id==="foreign-program"||row.id==="empty-tail-foreign-program")Object.assign(args,{command:"foreign-program"});
    if(row.id==="foreign-prefix")Object.assign(args,{args:["foreign-script","test",...corpus.request.arguments]});
    if(row.id==="empty-tail-foreign-program")backend.request={...corpus.request,arguments:[]};
    if(row.id==="foreign-policy-target")env.SEMIO_CARGO_TEST_POLICY=JSON.stringify({...originalPolicy,targetDirectory:join(root,"foreign")});
    if(row.id==="foreign-policy-mode")env.SEMIO_CARGO_TEST_POLICY=JSON.stringify({...originalPolicy,mode:"release"});
    if(row.id==="missing-policy")delete env.SEMIO_CARGO_TEST_POLICY;
    let accepted=true;try{const plan=resolveCommandConfigurationV1(args,env,invocation);expect(plan.environment.CARGO_TARGET_DIR).toBe(environment.CARGO_TARGET_DIR);expect(plan.environment.CARGO_BUILD_BUILD_DIR).toBe(environment.CARGO_BUILD_BUILD_DIR);expect(plan.environment.SEMIO_CARGO_TEST_POLICY).toBe(environment.SEMIO_CARGO_TEST_POLICY);expect(plan.policy.cargoPolicies).toEqual([originalPolicy]);expect(plan.policy.cargoArtifactPolicy).toBeNull();}catch{accepted=false;}
    expect(accepted,row.id).toBe(row.accepted);backend.request=corpus.request;
  }
  const plan=resolveCommandConfigurationV1(selected,environment,invocation);
  await expect(executeCommandV1(plan.request,plan.policy,{invocation,environment:plan.environment,onProgress(){Object.assign(invocation,{control:{...invocation.control,signal:new AbortController().signal}});}})).rejects.toThrow("Original command custody changed");
});
test("Cargo execution capture refuses replaced original control before acquisition",async()=>{
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned capture fixture directory required");
  for(const mutation of corpus.captureMutations){
    const args:string[]=[],env:Record<string,string>={};
    const invocation:ScriptInvocation={policy:{version:1,owner:"neutral-capture-delegation",maximumElapsedMilliseconds:1000},capabilities:{},control:{signal:new AbortController().signal,remainingMilliseconds:()=>1000,async publish(){
      if(mutation==="control")Object.assign(invocation,{control:{...invocation.control,signal:new AbortController().signal,remainingMilliseconds:()=>1000}});
      if(mutation==="arguments")args.push("foreign");if(mutation==="budget")options.budgetMs=10000;if(mutation==="output")options.maxOutputBytes=100000;if(mutation==="environment")env.FOREIGN="true";
    },async yieldContinuation(){}}};
    const options={invocation,cwd:resolve(output),env,budgetMs:1000,maxOutputBytes:1024,stdoutPath:join(output,"unacquired-stdout"),stderrPath:join(output,"unacquired-stderr")};
    await expect(captureOwnedProcess("unacquired-command",args,options)).rejects.toThrow("Original capture invocation changed");expect(existsSync(options.stdoutPath)).toBe(false);expect(existsSync(options.stderrPath)).toBe(false);
  }
});
test("Declared command tails bind the complete original test level and arguments",()=>{
  const declaration=corpus.declaration as CargoExecutionDeclarationV1,command={program:"runtime",cwd:"kernel",arguments:["script","test",...corpus.request.arguments]};
  expect(selectCargoExecutionOperationV1(declaration,command,"/mount")).toEqual({request:corpus.request as CargoExecutionRequestV1,continuation:null});
  for(const changed of [{...command,program:"foreign"},{...command,cwd:"foreign"},{...command,arguments:["script","other"]}])expect(selectCargoExecutionOperationV1(declaration,changed,"/mount")).toBeUndefined();
  expect(()=>selectCargoExecutionOperationV1({...declaration,operations:[...declaration.operations,...declaration.operations]},command,"/mount")).toThrow("Ambiguous");
  const rooted={...declaration,operations:declaration.operations.map(row=>({...row,command:{...row.command,arguments:["{workspaceRoot}/kernel/script",...row.command.arguments.slice(1)]}}))};
  expect(selectCargoExecutionOperationV1(rooted,{...command,arguments:["/mount/kernel/script",...command.arguments.slice(1)]},"/mount")).toEqual({request:corpus.request as CargoExecutionRequestV1,continuation:null});
});
for(const row of corpus.controlCases)test("Cargo execution retains original caller: "+row.id,async()=>{
  const controller=new AbortController(),request=structuredClone(corpus.request) as CargoExecutionRequestV1,identity={path:"/mount/kernel/Cargo.toml",device:"1",inode:"2"},source={file:identity,ancestors:[{path:"/mount",device:"1",inode:"1"}],sha256:"0".repeat(64),bytes:10};
  const backend={version:1,program:"runtime",arguments:["endpoint","execute"],cwd:"/mount",sources:[source,{...source,file:{...identity,path:"/mount/kernel/execution.json",inode:"3"}}],request:structuredClone(request),continuation:null};
  const capabilities:{cargoExecutionBackend?:typeof backend;transport:{maximumBytes:number;maximumLines:number};child:{maximumElapsedMilliseconds:number}}={cargoExecutionBackend:backend,transport:{maximumBytes:65536,maximumLines:100},child:{maximumElapsedMilliseconds:1000}};
  let remaining=1000,commands=0;
  const invocation:ScriptInvocation={policy:{version:1,owner:"neutral-cargo-port",maximumElapsedMilliseconds:1000},capabilities,control:{signal:controller.signal,remainingMilliseconds:()=>remaining,async publish(){
    if(row.id==="deadline-after-publication")remaining=0;
    if(row.id==="descriptor-after-publication")backend.arguments.push("foreign");
    if(row.id==="policy-after-publication")Object.assign(invocation,{policy:{...invocation.policy,owner:"foreign"}});
    if(row.id==="control-after-publication")Object.assign(invocation,{control:{...invocation.control,signal:new AbortController().signal,remainingMilliseconds:()=>1000}});
    if(row.id==="signal-after-publication")Object.assign(invocation.control,{signal:new AbortController().signal});
    if(row.id==="clock-after-publication")Object.assign(invocation.control,{remainingMilliseconds:()=>1000});
  },async yieldContinuation(){}}};
  if(row.id==="foreign-manifest")Object.assign(request,{manifest:"foreign/Cargo.toml"});
  if(row.id==="foreign-action")Object.assign(request,{kind:"exact-laws"});
  if(row.id==="foreign-package")Object.assign(request,{packages:["foreign"]});
  if(row.id==="missing-port")delete capabilities.cargoExecutionBackend;
  if(row.id==="cancel-before")controller.abort();
  let refused=false;try{await executeCargoRequestV1(request,invocation,async(command,args,cwd,_label,budget,options)=>{
    commands++;expect(command).toBe("runtime");expect(args).toEqual(["endpoint","execute"]);expect(cwd).toBe("/mount");expect(budget).toBeGreaterThan(0);expect(budget).toBeLessThanOrEqual(1000);expect(options!.signal).toBe(controller.signal);expect(options!.input).toBe(JSON.stringify(corpus.request));
    const wire=JSON.parse(options!.env!.SEMIO_SCRIPT_PROCESS_INVOCATION!);expect(wire.capabilities).toEqual(invocation.capabilities);expect(wire.policy).toEqual(invocation.policy);expect(wire.deadlineEpochMilliseconds).toBeLessThanOrEqual(Date.now()+1000);
    if(row.id==="cancel-during-execution")controller.abort();
  },{});}catch{refused=true;}
  expect(refused).toBe(row.refused);expect(commands).toBe(row.commands);
});
