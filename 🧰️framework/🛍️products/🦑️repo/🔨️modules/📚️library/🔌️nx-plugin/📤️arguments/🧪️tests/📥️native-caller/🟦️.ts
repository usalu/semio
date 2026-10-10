/** 📥️ Proves actual Nx dispatch issues exact original native child authority. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import nativeSchema from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json";
const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/📥️native-caller.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/📥️native-caller.json",import.meta.url),"utf8"));
const api=await import(new URL("../../🟨️.mjs",import.meta.url).href);
const require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json"));
const {createTaskGraph}=require(resolve(nxRoot,"dist/src/tasks-runner/create-task-graph.js"));
function inputs(rawArguments:string[]=[],forward=false){
 const started=Date.now(),capabilities={workspaceRoot:"/repo",arguments:["nx","run",fixture.selected,...(rawArguments.length?["--",...rawArguments]:[])],native:structuredClone(fixture.resources),selection:[] as string[]};
 capabilities.selection=[...capabilities.arguments];
 const original={version:1,policy:{version:1,owner:"original-nx-child-law",maximumElapsedMilliseconds:10000},deadlineEpochMilliseconds:started+10000,capabilities};
 const projects:any={app:{root:"app",targets:{test:{options:{command:"bun app"}}}}},tasks:any={[fixture.selected]:{id:fixture.selected,target:{project:"app",target:"test"}}};
 for(const row of fixture.commands){const [project,target]=row.task.split(":");const nativeOwnerCommand={kind:"native-owner-command",manifest:row.manifest,workingDirectory:row.workingDirectory,program:row.program,arguments:row.arguments};const command=`bun wrapper native owner-command --manifest "${row.manifest}" --cwd "${row.workingDirectory}" -- bun "${row.arguments[0]}" generate`;projects[project]={root:project,targets:{[target]:{options:{command,nativeOwnerCommand,forwardAllArgs:forward}}}};tasks[row.task]={id:row.task,target:{project,target}};}
 for(const project of Object.values(projects) as any[])for(const target of Object.values(project.targets) as any[])target.executor="@semio-tech/repo-lib:owner-command";
 projects.app.targets.test.dependsOn=[{target:"generate",projects:["value","ui"],params:"forward"}];
 const projectGraph={nodes:Object.fromEntries(Object.entries(projects).map(([name,data]:[string,any])=>[name,{name,type:"lib",data:{...data,name}}])),dependencies:Object.fromEntries(Object.keys(projects).map(name=>[name,[]]))};
 const taskGraph=createTaskGraph(projectGraph,{},["app"],["test"],undefined,{__overrides_unparsed__:rawArguments},false);
 const context:any={root:"/repo",taskGraph,projectGraph,projectsConfigurations:{projects}};
 return {original,context};
}
function optionsFor(input:any,row:any){const [project,target]=row.task.split(":");input.context.projectName=project;input.context.targetName=target;return {...input.context.projectsConfigurations.projects[project].targets[target].options,__unparsed__:input.context.taskGraph.tasks[row.task].overrides.__overrides_unparsed__,env:{SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(input.original),SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(input.original.capabilities),CARGO_TARGET_DIR:"/repo/artifacts",CARGO_NET_OFFLINE:"true"}};}
test("actual Nx graph uses dependency leaves and preserves original selected task edges",()=>{
 const {context}=inputs();expect(context.taskGraph.roots).toEqual(fixture.graphRoots);expect(context.taskGraph.dependencies[fixture.selected]).toEqual(fixture.dependencies);
});
test("literal native command kind agrees with independent const admission",()=>{
 const admit=new Ajv({strict:true}).compile(nativeSchema.properties.command);
 for(const row of fixture.commands){
  const command={kind:"native-owner-command",manifest:row.manifest,workingDirectory:row.workingDirectory,program:row.program,arguments:row.arguments};
  expect(admit(command)).toBe(true);
  expect(admit({...command,kind:"alternate-native-owner"})).toBe(false);
  const text=[row.program,...row.arguments.map((argument:string)=>JSON.stringify(argument))].join(" ");
  expect(api.declaredNativeOwnerCommandV1(row.manifest,row.workingDirectory,text)).toEqual(command);
 }
});
test("closed original Nx caller authority matches independent Ajv admission",()=>{
 const {original}=inputs(),admit=new Ajv({strict:true}).compile(schema);
 for(const value of [original.capabilities,{...original.capabilities,native:undefined},{...original.capabilities,allowAll:true}])expect(validateJsonSchemaSubset(schema,value).length===0).toBe(admit(value));
 expect(admit(original.capabilities)).toBe(true);
});
test("native path authority refuses NUL while preserving literal platform paths",()=>{
 const nativePath=nativeSchema.properties.command.properties.program,admit=new Ajv({strict:true}).compile(nativePath);
 for(const row of fixture.pathCases){expect(validateJsonSchemaSubset(nativePath,row.input).length===0).toBe(row.accepted);expect(admit(row.input)).toBe(row.accepted);}
});
test("actual dispatcher issues two distinct selected dependencies without rebasing original authority",async()=>{
 const input=inputs(),before=JSON.stringify(input.original),validate=new Ajv({strict:true}).compile(nativeSchema),observed:any[]=[];
 for(const row of fixture.commands){let plan:any;await api.executeOwnerArgumentsV1(optionsFor(input,row),input.context,async(value:any)=>{plan=value;return {success:true};});const issued=JSON.parse(plan.env.SEMIO_SCRIPT_PROCESS_INVOCATION);expect(issued.policy).toEqual(input.original.policy);expect(issued.deadlineEpochMilliseconds).toBe(input.original.deadlineEpochMilliseconds);expect(validate(issued.capabilities),JSON.stringify(validate.errors)).toBe(true);expect(issued.capabilities.transport).toEqual(input.original.capabilities.native.transport);expect(issued.capabilities.child).toEqual(input.original.capabilities.native.child);expect(issued.capabilities.network).toEqual(input.original.capabilities.native.network);expect(issued.capabilities.artifactDirectory).toBe("/repo/artifacts");const command=issued.capabilities.command;observed.push([command.manifest,command.workingDirectory,command.program,command.arguments]);}
 expect(observed).toEqual(fixture.expectedCommands);expect(JSON.stringify(input.original)).toBe(before);
 console.log("[DEBUG] actual Nx dispatcher original dependency commands=2 parentDeadline=unchanged capabilities=closed schemaOracle=Ajv");
});
test("actual Nx dependency forwards only its original literal arguments",async()=>{
 const input=inputs(fixture.forwardedArguments,true),row=fixture.commands[0];let observed:any;
 await api.executeOwnerArgumentsV1(optionsFor(input,row),input.context,async(plan:any)=>{observed=JSON.parse(plan.env.SEMIO_SCRIPT_PROCESS_INVOCATION);return {success:true};});
 expect(observed.capabilities.command.arguments).toEqual([...row.arguments,...fixture.forwardedArguments]);expect(api.consumeOwnerArgumentsV1(optionsFor(input,row).env).arguments).toEqual([]);
});
test("actual dispatcher refuses foreign or altered native authority before delegation",async()=>{
 for(const refusal of fixture.refusals){const input=inputs(),options=optionsFor(input,fixture.commands[0]);let calls=0;
 if(refusal==="foreign-task")input.context.taskGraph.dependencies[fixture.selected]=[];
 if(refusal==="changed-command")options.command+=" changed";
 if(refusal==="changed-forwarded-arguments"){options.forwardAllArgs=true;options.__unparsed__=["--foreign-original-argument"];}
 if(refusal==="changed-request")options.nativeOwnerCommand={...options.nativeOwnerCommand,manifest:"foreign/Cargo.toml"};
 if(refusal==="missing-native"){delete input.original.capabilities.native;options.env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(input.original);}
 if(refusal==="changed-artifacts")options.env.CARGO_TARGET_DIR="/repo/foreign";
 if(refusal==="expired"){input.original.deadlineEpochMilliseconds=Date.now()-1;options.env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(input.original);}
 if(refusal==="capability-override")options.env.SEMIO_SCRIPT_CAPABILITIES=JSON.stringify({allowAll:true});
 await expect(api.executeOwnerArgumentsV1(options,input.context,async()=>{calls++;return {success:true};})).rejects.toThrow();expect(calls,refusal).toBe(0);
 }
 console.log(`[DEBUG] actual Nx dispatcher original refusals=${fixture.refusals.length} delegated=0`);
});

test("native dispatch preserves authentic zero and nullable original parent authority",async()=>{
 const validate=new Ajv({strict:true}).compile(nativeSchema);
 for(const row of fixture.parentCases){
  const input=inputs(),options=optionsFor(input,fixture.commands[0]);
  input.original.policy.maximumElapsedMilliseconds=row.maximumElapsedMilliseconds;
  input.original.deadlineEpochMilliseconds=row.remainingMilliseconds===null?null:Date.now()+row.remainingMilliseconds;
  options.env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(input.original);
  const before=JSON.stringify(input.original);let issued:any,calls=0;
  const invoke=()=>api.executeOwnerArgumentsV1(options,input.context,async(plan:any)=>{calls++;issued=JSON.parse(plan.env.SEMIO_SCRIPT_PROCESS_INVOCATION);return {success:true};});
  if(row.accepted){await invoke();expect(calls).toBe(1);expect(issued.policy).toEqual(input.original.policy);expect(issued.deadlineEpochMilliseconds).toBe(input.original.deadlineEpochMilliseconds);expect(validate(issued.capabilities)).toBe(true);expect(issued.capabilities.child).toEqual(fixture.resources.child);}
  else{await expect(invoke()).rejects.toThrow();expect(calls).toBe(0);}
  expect(JSON.stringify(input.original)).toBe(before);
 }
 console.log("[DEBUG] Original zero/null and finite parent policies preserved exactly; independently issued native child remains finite; mixed authority refused before delegation");
});

test("production native caller binds canonical artifacts and actual selected argv without changing the parent",async()=>{
 const producer=await import(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts",import.meta.url).href);
 const policy=JSON.parse(readFileSync(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/🔣️policy.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:true}).compile(nativeSchema.$defs.NativeOwnerExecutionPolicyV1)(policy)).toBe(true);
 for(const row of fixture.producerCases){
  const original={version:1,policy:{version:1,owner:"actual-nx-parent",maximumElapsedMilliseconds:0},deadlineEpochMilliseconds:null,capabilities:{workspaceRoot:"/repo",arguments:row.original,selection:["nx",...row.selected],native:structuredClone(row.native)}};
  const before=JSON.stringify(original),environment={...row.environment,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(original)};
  const observed=producer.nativeNxChildEnvironmentV1("/repo",original,row.selected,environment),issued=JSON.parse(observed.SEMIO_SCRIPT_PROCESS_INVOCATION);
  expect(issued.policy).toEqual(original.policy);expect(issued.deadlineEpochMilliseconds).toBeNull();expect(issued.capabilities.arguments).toEqual(row.original);expect(issued.capabilities.selection).toEqual(["nx",...row.selected]);
  expect(issued.capabilities.native.transport).toEqual(row.native.transport);expect(issued.capabilities.native.child).toEqual(row.native.child);expect(issued.capabilities.native.artifactDirectory).toBe(observed.CARGO_TARGET_DIR);expect(observed.CARGO_BUILD_BUILD_DIR).toBe(observed.CARGO_TARGET_DIR);
  expect(observed.CARGO_NET_OFFLINE).toBe(String(issued.capabilities.native.network.offline));expect(issued.capabilities.native.network.offline).toBe(row.offline);expect(new Ajv({strict:true}).compile(schema)(issued.capabilities)).toBe(true);
  const oracle=Bun.spawnSync(["node","-e","process.stdout.write(JSON.stringify(require('node:path').resolve(process.argv[1],process.argv[2])))","--","/repo",row.artifact]);
  expect(oracle.exitCode).toBe(0);expect(issued.capabilities.native.artifactDirectory).toBe(JSON.parse(new TextDecoder().decode(oracle.stdout)));
  expect(JSON.stringify(original)).toBe(before);expect(environment.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(before);expect(observed.SEMIO_SCRIPT_CAPABILITIES).toBeUndefined();
 }
 console.log("[DEBUG] Original fixture owns native child resources and exact selected argv; original zero/null parent remains unchanged, independent Ajv/Node path oracle");
});
test("actual run-many dispatcher retains the installed Nx project and target selection",async()=>{
 const input=inputs();input.original.capabilities.arguments=["nx","run-many","--target=test","--projects=app"];input.original.capabilities.selection=[...input.original.capabilities.arguments];
 for(const row of fixture.commands){let calls=0;await api.executeOwnerArgumentsV1(optionsFor(input,row),input.context,async()=>{calls++;return {success:true};});expect(calls).toBe(1);}
 console.log("[DEBUG] Installed Nx run-many selection admits only actual selected root dependencies");
});

test("original Nx launch issues authority for each actual provision user and watcher child",async()=>{
 const ts=(await import("typescript")).default,bootstrapPath=new URL("../../../../⚡️caching/🚀️bootstrap/📜️script.ts",import.meta.url),text=readFileSync(bootstrapPath,"utf8"),tree=ts.createSourceFile(bootstrapPath.pathname,text,ts.ScriptTarget.Latest,true);
 let source="";const find=(node:any)=>{if(ts.isVariableDeclaration(node)&&node.name.getText(tree)==="launch"&&node.initializer)source=node.initializer.getText(tree);ts.forEachChild(node,find);};find(tree);expect(source.length).toBeGreaterThan(0);
 const bootstrap=await import(bootstrapPath.href),producer=await import(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts",import.meta.url).href),processApi=await import(new URL("../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts",import.meta.url).href);
 const original=processApi.createScriptProcessEnvelope({version:1,owner:"original-launch-parent",maximumElapsedMilliseconds:0},{workspaceRoot:"/repo",arguments:["nx","run","workspace:dev"]},Date.now()),environment={SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(original)};
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText]){
  let observed:any;const children:any[]=[];
  const spawn=(program:string,args:string[],options:any)=>{observed={program,args,options};return {pid:1};};
  const execute=new Function("spawnNxProcess","nxChildEnvironment","nativeNxChildEnvironmentV1","issueSelectedNxCallerEnvelopeV1","readScriptProcessEnvelope","SCRIPT_PROCESS_INVOCATION_ENV","nxCli","children","invocation","env","exports",compile("function retainOriginalLaunch(){return "+source+";}")+";return retainOriginalLaunch.call(this);").call({root:"/repo"},spawn,bootstrap.nxChildEnvironment,producer.nativeNxChildEnvironmentV1,producer.issueSelectedNxCallerEnvelopeV1,processApi.readScriptProcessEnvelope,processApi.SCRIPT_PROCESS_INVOCATION_ENV,"nx-cli",children,{watch:"app:activate"},environment,bootstrap);
  for(const selected of fixture.launchCases){const caller=processApi.createScriptProcessEnvelope(original.policy,{...original.capabilities,selection:["nx",...selected],native:structuredClone(fixture.launchResources)},Date.now()),declaredEnvironment={...environment,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(caller)};execute.call({root:"/repo"},selected,false,declaredEnvironment);const issued=JSON.parse(observed.options.env.SEMIO_SCRIPT_PROCESS_INVOCATION);expect(new Ajv({strict:true}).compile(schema)(issued.capabilities)).toBe(true);expect(issued.capabilities.selection).toEqual(["nx",...selected]);expect(issued.capabilities.arguments).toEqual(original.capabilities.arguments);expect(issued.policy).toEqual(original.policy);expect(issued.deadlineEpochMilliseconds).toBeNull();expect(observed.args).toEqual(["nx-cli",...selected]);}
  expect(children.length).toBe(fixture.launchCases.length);
 }
 console.log("[DEBUG] Original launch body through Bun and independent TypeScript compilers retains parent while issuing each actual provision, alias and watcher selected argv; real child verification remains separate");
});

test("actual caller never fabricates native authority or selects another original command",async()=>{
 const producer=await import(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts",import.meta.url).href);
 const originalArguments=["nx","run","app:test"],validate=new Ajv({strict:true}).compile(schema);
 for(const law of fixture.callerReceivingCases){
  const resources={...structuredClone(fixture.resources),artifactDirectory:"/repo/caller-artifacts"};
  const capabilities:any={workspaceRoot:"/repo",arguments:originalArguments,native:resources,selection:originalArguments};
  if(law.kind==="missingNative")delete capabilities.native;
  if(law.kind==="missingSelection")delete capabilities.selection;
  if(law.kind==="differentSelection")capabilities.selection=["nx","run","foreign:test"];
  const original={version:1,policy:{version:1,owner:"original-explicit-native",maximumElapsedMilliseconds:10000},deadlineEpochMilliseconds:Date.now()+10000,capabilities},before=JSON.stringify(original),environment={SEMIO_SCRIPT_PROCESS_INVOCATION:before,CARGO_TARGET_DIR:"/repo/caller-artifacts",CARGO_NET_OFFLINE:"true"};
  const invoke=()=>producer.nativeNxChildEnvironmentV1("/repo",original,["run","app:test"],environment);
  if(law.accepted){const output=invoke(),issued=JSON.parse(output.SEMIO_SCRIPT_PROCESS_INVOCATION!);expect(issued.capabilities.native!==undefined).toBe(law.nativeIssued);expect(issued.capabilities).toEqual(capabilities);expect(issued.policy).toEqual(original.policy);expect(issued.deadlineEpochMilliseconds).toBe(original.deadlineEpochMilliseconds);if(law.nativeIssued)expect(validate(issued.capabilities)).toBe(true);}
  else expect(invoke).toThrow();
  expect(JSON.stringify(original)).toBe(before);expect(environment.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(before);
 }
 console.log("[DEBUG] Actual explicit caller resources and selected argv preserve original provenance; non-native launch fabricates no native grant; missing/foreign selected authority refused");
});

test("actual normal Nx launch preserves ordinary original envelopes without native-only admission",async()=>{
 const producer=await import(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts",import.meta.url).href);
 const processSchema=JSON.parse(readFileSync(new URL("../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🔣️.json",import.meta.url),"utf8")),policySchema=JSON.parse(readFileSync(new URL("../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const validate=new Ajv({strict:true}).addSchema(policySchema).compile(processSchema);
 for(const capabilities of fixture.normalCallerCapabilities){
  const original={version:1,policy:{version:1,owner:"original-normal-nx",maximumElapsedMilliseconds:10000},deadlineEpochMilliseconds:Date.now()+10000,capabilities},before=JSON.stringify(original),environment={SEMIO_SCRIPT_PROCESS_INVOCATION:before,ORIGINAL_NORMAL_INPUT:"same-source"};
  expect(validate(original)).toBe(true);
  const output=producer.nativeNxChildEnvironmentV1("/repo",original,["exec","--projects=workspace","--excludeTaskDependencies","--skip-nx-cache","--","bun","original-input"],environment),issued=JSON.parse(output.SEMIO_SCRIPT_PROCESS_INVOCATION!);
  expect(validate(issued)).toBe(true);expect(issued).toEqual(original);expect(output.ORIGINAL_NORMAL_INPUT).toBe(environment.ORIGINAL_NORMAL_INPUT);expect(output.CARGO_TARGET_DIR).toBeUndefined();expect(output.CARGO_NET_OFFLINE).toBeUndefined();expect(issued.capabilities.native).toBeUndefined();expect(issued.capabilities.selection).toBeUndefined();expect(JSON.stringify(original)).toBe(before);expect(environment.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(before);
 }
 console.log("[DEBUG] Actual non-native Nx exec preserves ordinary original capability/policy/deadline/inputs; independent Ajv agrees; no native-only request or Cargo/native authority fabricated");
});

test("trusted outer Nx issuer owns production policy and declares child selections before handoff",async()=>{
 const producer=await import(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts",import.meta.url).href),processApi=await import(new URL("../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts",import.meta.url).href);
 const policy=JSON.parse(readFileSync(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/🔣️policy.json",import.meta.url),"utf8")),admit=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.producerCases){
  const commands=[row.selected,...fixture.launchCases],capabilities=producer.createNativeNxCallerCapabilitiesV1("/repo",row.original,row.selected,row.environment,commands),parent=processApi.createScriptProcessEnvelope({version:1,owner:"trusted-outer-launch",maximumElapsedMilliseconds:0},capabilities,Date.now()),before=JSON.stringify(parent);
  expect(capabilities.native.transport).toEqual(policy.transport);expect(capabilities.native.child).toEqual(policy.child);expect(capabilities.native.network.offline).toBe(row.offline);expect(capabilities.launches).toEqual(commands.map((args:string[])=>["nx",...args]));expect(admit(capabilities),JSON.stringify(admit.errors)).toBe(true);
  for(const selected of commands){const child=producer.issueSelectedNxCallerEnvelopeV1("/repo",parent,selected);expect(child.policy).toBe(parent.policy);expect(child.deadlineEpochMilliseconds).toBeNull();expect(child.capabilities.native).toBe(capabilities.native);expect(child.capabilities.arguments).toBe(capabilities.arguments);expect(child.capabilities.selection).toEqual(["nx",...selected]);expect(admit(child.capabilities)).toBe(true);}
  expect(()=>producer.issueSelectedNxCallerEnvelopeV1("/repo",parent,["run","undeclared:foreign"])).toThrow();expect(JSON.stringify(parent)).toBe(before);
  const bound=processApi.createScriptProcessEnvelope(parent.policy,{workspaceRoot:capabilities.workspaceRoot,arguments:capabilities.arguments,native:capabilities.native,selection:capabilities.selection},Date.now());expect(producer.issueSelectedNxCallerEnvelopeV1("/repo",bound,row.selected)).toBe(bound);expect(()=>producer.issueSelectedNxCallerEnvelopeV1("/repo",bound,["run","undeclared:foreign"])).toThrow();
  const oracle=Bun.spawnSync(["node","-e","process.stdout.write(JSON.stringify(require('node:path').resolve(process.argv[1],process.argv[2])))","--","/repo",row.artifact]);expect(oracle.exitCode).toBe(0);expect(capabilities.native.artifactDirectory).toBe(JSON.parse(new TextDecoder().decode(oracle.stdout)));
 }
 console.log("[DEBUG] Trusted outer issuer reads genuine production policy before original envelope; explicit child command list narrows declared selections without fabricating or rebasing caller authority; Ajv and Node independently agree");
});

/** 🧵️ The private native argument handoff reaches a real process without shell interpolation. */
test("actual native private argument transport preserves selected child argv",async()=>{
 const input=inputs(fixture.forwardedArguments,true),row=fixture.commands[0];let originalPlan:any;
 await api.executeOwnerArgumentsV1(optionsFor(input,row),input.context,async(plan:any)=>{originalPlan=plan;return {success:true};});
 const before=originalPlan.env.SEMIO_OWNER_ARGUMENTS,transported=api.consumeOwnerArgumentsV1(originalPlan.env),issued=JSON.parse(originalPlan.env.SEMIO_SCRIPT_PROCESS_INVOCATION);
 expect(transported.arguments).toEqual(fixture.forwardedArguments);expect(transported.environment.SEMIO_OWNER_ARGUMENTS).toBeUndefined();expect(originalPlan.env.SEMIO_OWNER_ARGUMENTS).toBe(before);
 const arguments_=[...row.arguments,...transported.arguments],marker="original-native-argv-boundary";
 expect(arguments_).toEqual(issued.capabilities.command.arguments);
 const code="process.stdout.write(JSON.stringify(process.argv.slice(process.argv.indexOf('original-native-argv-boundary')+1)))";
 const actual=Bun.spawnSync([process.execPath,"-e",code,"--",marker,...arguments_],{env:transported.environment});
 const oracle=Bun.spawnSync(["node","-e",code,"--",marker,...arguments_],{env:transported.environment});
 expect(actual.exitCode,new TextDecoder().decode(actual.stderr)).toBe(0);expect(oracle.exitCode,new TextDecoder().decode(oracle.stderr)).toBe(0);
 expect(JSON.parse(new TextDecoder().decode(actual.stdout))).toEqual(arguments_);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(arguments_);
 console.log("[DEBUG] Native private envelope preserves literal selected arguments in actual Bun and independent Node child argv; outer wrapper command remains static");
});
