import {buildOrchestrationSourceIdentityV1,buildOrchestrationContextV1,requireBuildOrchestrationContextV1,projectBuildOrchestrationCapabilitiesV1} from "../../../../🏗️build/📥️caller/📤️orchestration/🟨️.mjs";
import {dirname,resolve} from "node:path";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
import {mkdir,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
const key="SEMIO_OWNER_ARGUMENTS",readinessMarker=JSON.parse(readFileSync(new URL("../../../../🏗️build/🧬️schema/🔣️.json",import.meta.url),"utf8")).$defs.ReadinessPolicy.const.marker;
const taskAuthorityPaths=[new URL(import.meta.url),...['🔣️executor.json','📥️native-caller.json','📥️source.json'].map(path=>new URL('./🧬️schema/'+path,import.meta.url))],taskAuthorityBytes=taskAuthorityPaths.map(path=>readFileSync(path)),executorSchema=JSON.parse(taskAuthorityBytes[1].toString("utf8")),nativeCallerSchema=JSON.parse(taskAuthorityBytes[2].toString("utf8")),sourceSchema=JSON.parse(taskAuthorityBytes[3].toString("utf8"));
const nativeSchema=JSON.parse(readFileSync(new URL("../../../📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",import.meta.url),"utf8"));

import {encodeOwnerArgumentsV1,conformsOwnerContractV1 as conforms} from "../../../../🏗️build/📤️arguments/🟨️.mjs";
import {readPrivateNxBuildGraphV1} from "../📌️graph-pin/🟨️.mjs";
import {nxBuildDependencySourceIdentityV1} from "../🔏️implementation/🟨️.mjs";
import {issueStoreReceiptV1} from "../../../../🏗️build/🧾️store-receipt/🟨️.mjs";

/** 🧾️ Starts a task that declares a store receipt from no receipt and names the receipt to its handler. */
function storeReceiptEnvironmentV1(context){const project=context.projectsConfigurations?.projects?.[context.projectName];return issueStoreReceiptV1({workspaceRoot:context.root,projectRoot:project?.root??".",target:context.targetName,outputs:project?.targets?.[context.targetName]?.outputs});}

const taskAuthority=(captured=false)=>createHash('sha256').update(JSON.stringify([buildOrchestrationSourceIdentityV1(),...taskAuthorityPaths.map((path,index)=>createHash('sha256').update(captured?taskAuthorityBytes[index]:readFileSync(path)).digest('hex'))])).digest('hex'),loadedTaskAuthority=taskAuthority(true);
/** 🔏️ Refuses executor code or delegation contracts changed after actual module evaluation. */
export function nxTaskArgumentsSourceIdentityV1(){if(taskAuthority()!==loadedTaskAuthority)throw Error('Loaded private Nx task Source changed');return loadedTaskAuthority;}

/** 🐚️ Retains Nx process lifecycle while Bun's portable shell receives exact literal arguments. */
export async function executeLiteralBuildArgumentsV1(options,context,delegate){
 for(const key of ["args","__unparsed__"])if(options[key]!==undefined&&(!Array.isArray(options[key])||options[key].some(value=>typeof value!=="string")))throw Error("Literal build argument arrays required");
 if(["SEMIO_NX_COMMAND_RUNNER","SEMIO_NX_COMMAND_DIRECTORY"].some(key=>options.env?.[key]!==undefined))throw Error("Private Nx command runner environment reserved");
 const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(typeof wire!=="string"||options.env?.SEMIO_SCRIPT_PROCESS_INVOCATION!==undefined&&options.env.SEMIO_SCRIPT_PROCESS_INVOCATION!==wire)throw Error("Original Nx parent invocation required");
 const directory=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??resolve(context.root,".🧬semio/🦑️repo/⚡️cache/build/nx-commands")),invocationWire=(await issueNxBuildOrchestrationV1(options,context,{...process.env,...options.env},Date.now())),invocation=createHash("sha256").update(invocationWire?JSON.stringify(invocationWire):wire).digest("hex");await mkdir(directory,{recursive:true,mode:0o700});
 if(typeof options.command!=="string"&&!Array.isArray(options.commands)||options.command!==undefined&&options.commands!==undefined)throw Error("One build command form required");
 const orchestration=invocationWire;
 const target=context.projectsConfigurations?.projects?.[context.projectName]?.targets?.[context.targetName],ready=target?.continuous===true&&options.readyWhen===undefined?target.metadata?.semio?.dashboard?.ready:undefined;
 const args=[...options.args??[],...options.__unparsed__??[]],quote=value=>"'"+value.replaceAll("'","'\\''")+"'",wrap=async operation=>{
  const command=typeof operation==="string"?operation:operation.command,forward=typeof operation==="string"?options.forwardAllArgs:operation.forwardAllArgs??options.forwardAllArgs;
  if(typeof command!=="string")throw Error("Build command string required");
  const source={version:1,invocation,command:command+(forward!==false&&args.length?" "+args.map(quote).join(" "):""),...(ready===undefined?{}:{ready})};
  if(!conforms(sourceSchema,source))throw Error("Private Nx command source refused");
  const image=JSON.stringify(source),digest=createHash("sha256").update(image).digest("hex");try{await writeFile(resolve(directory,digest+".json"),image,{flag:"wx",mode:0o600});}catch(error){if(error.code!=="EEXIST")throw error;}
  const bridge=(process.platform==="win32"?'repo "%SEMIO_NX_COMMAND_RUNNER%"':'repo "$SEMIO_NX_COMMAND_RUNNER"')+" "+digest;
  return typeof operation==="string"?bridge:{...operation,command:bridge,forwardAllArgs:false};
 };
 const plan={...options,...(ready===undefined?{}:{readyWhen:readinessMarker}),args:undefined,__unparsed__:[],forwardAllArgs:false,env:{...process.env,...options.env,SEMIO_BUILD_TOOL:"nx",SEMIO_WORKSPACE_ROOT:context.root,SEMIO_TASK_PROJECT:context.projectName,SEMIO_TASK_TARGET:context.targetName,...storeReceiptEnvironmentV1(context),...(context.configurationName?{SEMIO_TASK_CONFIGURATION:context.configurationName}:{}),SEMIO_NX_COMMAND_DIRECTORY:directory,SEMIO_NX_COMMAND_RUNNER:fileURLToPath(new URL("./📜️script.ts",import.meta.url)),...(orchestration?{SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(orchestration),SEMIO_SCRIPT_CAPABILITIES:undefined}:{})}};
 if(options.command!==undefined)plan.command=await wrap(options.command);else{plan.commands=[];for(const command of options.commands)plan.commands.push(await wrap(command));}
 return delegate(plan,context);
}

/** 📥️ Issues the exact discovered descendant with the original caller's unchanged deadline and resources. */
async function readNxTaskAuthorityV1(options,context,environment,now){
 const wire=environment.SEMIO_SCRIPT_PROCESS_INVOCATION;if(typeof wire!=="string")throw Error("Original Nx native parent invocation required");
 const original=JSON.parse(wire),policy=original?.policy,caller=original?.capabilities;
 if(original?.version!==1||Object.keys(original).length!==4||!policy||Object.keys(policy).length!==3||policy.version!==1||typeof policy.owner!=="string"||policy.owner.length===0||!Number.isSafeInteger(policy.maximumElapsedMilliseconds)||policy.maximumElapsedMilliseconds<0||!Number.isFinite(now)||(policy.maximumElapsedMilliseconds===0?original.deadlineEpochMilliseconds!==null:!Number.isSafeInteger(original.deadlineEpochMilliseconds)||original.deadlineEpochMilliseconds<=now||original.deadlineEpochMilliseconds-now>policy.maximumElapsedMilliseconds)||!conforms(nativeCallerSchema,caller))throw Error("Original Nx native authority refused");
 if(environment.SEMIO_SCRIPT_CAPABILITIES!==undefined&&JSON.stringify(JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES))!==JSON.stringify(caller))throw Error("Original Nx capabilities changed before native dispatch");
 if(resolve(caller.workspaceRoot)!==resolve(context.root)||caller.arguments[0]!=="nx"||caller.selection[0]!=="nx")throw Error("Original selected Nx command required");
 const graph=context.taskGraph;if(!graph?.tasks||!graph.dependencies||!Array.isArray(graph.roots))throw Error("Original Nx task graph required");
 let selectedTasks;
 if(caller.selection[1]==="run"){
  const parts=caller.selection[2].split(":"),selected={project:parts[0],target:parts[1],configuration:parts[2]};if(parts.length<2||parts.length>3||!selected.project||!selected.target)throw Error("Original selected Nx target refused");
  const values=selected.configuration===undefined?[]:[selected.configuration],arguments_=caller.selection.slice(3),boundary=arguments_.indexOf("--"),flags=arguments_.slice(0,boundary<0?undefined:boundary);
  for(let index=0;index<flags.length;index++){const flag=flags[index];if(flag==="--configuration"||flag==="-c")values.push(flags[++index]);else if(/^(?:--configuration|-c)=/u.test(flag))values.push(flag.slice(flag.indexOf("=")+1));}
  if(values.some(value=>typeof value!=="string"||!value||value!==values[0]))throw Error("Original selected Nx configuration refused");
  const declared=context.projectsConfigurations?.projects?.[selected.project]?.targets?.[selected.target],policy=declared?.metadata?.semio?.buildMode,configuration=values[0]??(policy?(policy.fixed??caller.buildMode):undefined)??declared?.defaultConfiguration;
  selectedTasks=Object.keys(graph.tasks).filter(id=>{const target=graph.tasks[id]?.target;return target?.project===selected.project&&target?.target===selected.target&&(target.configuration??undefined)===configuration;});
  if(selectedTasks.length!==1)throw Error("Original selected Nx graph task refused");
 }else if(["run-many","affected"].includes(caller.selection[1])){
  if(!context.projectGraph?.nodes)throw Error("Original Nx project graph required");
  const require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json")),{withOverrides}=require(resolve(nxRoot,"dist/src/command-line/yargs-utils/shared-options.js")),{yargsRunManyCommand}=require(resolve(nxRoot,"dist/src/command-line/run-many/command-object.js")),{splitArgsIntoNxArgsAndOverrides}=require(resolve(nxRoot,"dist/src/utils/command-line-utils.js")),{projectsToRun}=require(resolve(nxRoot,"dist/src/command-line/run-many/run-many.js")),yargs=createRequire(resolve(nxRoot,"package.json"))("yargs/yargs");
  const operation=caller.selection[1],command=operation==="run-many"?yargsRunManyCommand:require(resolve(nxRoot,"dist/src/command-line/affected/command-object.js")).yargsAffectedCommand;
  const parsed=withOverrides(command.builder(yargs()).exitProcess(false).parseSync(caller.selection.slice(1))),{nxArgs}=splitArgsIntoNxArgsAndOverrides(parsed,operation,{printWarnings:false},context.nxJsonConfiguration??{});
  const selected=operation==="run-many"?projectsToRun(nxArgs,context.projectGraph):await require(resolve(nxRoot,"dist/src/command-line/affected/affected.js")).getAffectedGraphNodes(nxArgs,context.projectGraph),projects=new Set(selected.map(project=>project.name));
  selectedTasks=Object.keys(graph.tasks).filter(id=>{const target=graph.tasks[id]?.target,declared=context.projectsConfigurations?.projects?.[target?.project]?.targets?.[target?.target],policy=declared?.metadata?.semio?.buildMode,configuration=nxArgs.configuration??(policy?(policy.fixed??caller.buildMode):undefined)??declared?.defaultConfiguration;return projects.has(target?.project)&&nxArgs.targets.includes(target?.target)&&(target.configuration??undefined)===configuration;});
  if(selectedTasks.length===0)throw Error("Original selected Nx graph tasks refused");
 }else throw Error("Original selected Nx command required");
 const reachable=new Set(),pending=[...selectedTasks];while(pending.length){const id=pending.pop();if(reachable.has(id))continue;if(!graph.tasks[id]||!Array.isArray(graph.dependencies[id]))throw Error("Original Nx dependency graph incomplete");reachable.add(id);pending.push(...graph.dependencies[id]);}
 const admitted=[...reachable].filter(id=>{const target=graph.tasks[id].target;return target.project===context.projectName&&target.target===context.targetName&&(target.configuration??undefined)===(context.configurationName??undefined);});
 const declared=context.projectsConfigurations?.projects?.[context.projectName]?.targets?.[context.targetName];
 if(admitted.length!==1)throw Error(`Original native task reachability refused: ${context.projectName}:${context.targetName}:${context.configurationName??""} admitted=${admitted.length}`);
 if(!declared)throw Error("Original discovered native target declaration required");
 const task=graph.tasks[admitted[0]],require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json")),{combineOptionsForExecutor}=require(resolve(nxRoot,"dist/src/utils/params.js"));
 const expected=combineOptionsForExecutor(structuredClone(task.overrides),task.target.configuration,structuredClone(declared),executorSchema,context.projectName,context.cwd??".",false);
 return {original,caller,declared,task,expected};
}

export async function issueNativeOwnerInvocationV1(options,context,environment,now){
 const {original,caller,declared,task,expected}=await readNxTaskAuthorityV1(options,context,environment,now),request=options.nativeOwnerCommand;if(!conforms(nativeSchema.properties.command,request))throw Error("Original discovered native command schema refused");
 if(expected.command!==options.command||JSON.stringify(expected.nativeOwnerCommand)!==JSON.stringify(request)||(expected.forwardAllArgs!==false)!==(options.forwardAllArgs!==false))throw Error("Original discovered native task command changed");
 const expectedArguments=expected.forwardAllArgs===false?[]:[...expected.args??[],...expected.__unparsed__??[]],actualArguments=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]];
 if(JSON.stringify(expectedArguments)!==JSON.stringify(actualArguments))throw Error("Original native task arguments changed");
 if(typeof environment.CARGO_TARGET_DIR!=="string"||resolve(context.root,caller.native.artifactDirectory)!==resolve(context.root,environment.CARGO_TARGET_DIR)||environment.CARGO_NET_OFFLINE!==String(caller.native.network.offline))throw Error("Original native artifact or network authority changed");
 const forwarded=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]],capabilities={version:1,buildMode:caller.buildMode,repositoryRoot:context.root,artifactDirectory:resolve(context.root,caller.native.artifactDirectory),command:{...request,arguments:[...request.arguments,...forwarded]},transport:caller.native.transport,child:caller.native.child,network:caller.native.network};
 if(!conforms(nativeSchema,capabilities))throw Error("Issued original native capability refused");return {...original,capabilities};
}

/** 🧭️ Projects only a reachable immutable orchestration task before its owned script process is born. */
export async function issueNxBuildOrchestrationV1(options,context,environment,now){
 nxTaskArgumentsSourceIdentityV1();
 const declaration=context.projectsConfigurations?.projects?.[context.projectName]?.targets?.[context.targetName]?.metadata?.semio?.execution;if(declaration?.kind!=="orchestration"||declaration.children===undefined)return;
 const {original,declared,task,expected}=await readNxTaskAuthorityV1(options,context,environment,now);if(options.nativeOwnerCommand!==undefined||expected.command!==options.command||JSON.stringify(expected.commands)!==JSON.stringify(options.commands)||JSON.stringify(expected.args??[])!==JSON.stringify(options.args??[])||JSON.stringify(expected.__unparsed__??[])!==JSON.stringify(options.__unparsed__??[]))throw Error("Original orchestration task command changed");
 const portable=await readPrivateNxBuildGraphV1(context.root,environment,()=>{nxTaskArgumentsSourceIdentityV1();nxBuildDependencySourceIdentityV1();}),selection={project:context.projectName,target:context.targetName,...(context.configurationName?{configuration:context.configurationName}:{})},bound=buildOrchestrationContextV1(portable.projects,selection);
 if(!bound||JSON.stringify(declaration)!==JSON.stringify(portable.projects[selection.project]?.targets?.[selection.target]?.metadata?.semio?.execution))throw Error("Immutable orchestration task declaration changed");
 const current=buildOrchestrationContextV1(context.projectsConfigurations.projects,{project:selection.project,target:selection.target});requireBuildOrchestrationContextV1(portable.projects,{...current,task:selection});
 if(original.deadlineEpochMilliseconds!==null&&original.deadlineEpochMilliseconds<=Date.now())throw Error("Original orchestration deadline exhausted");const capabilities=projectBuildOrchestrationCapabilitiesV1(original,context.root,"nx",bound,Date.now());return {...original,capabilities};
}

/** 🎯️ Keeps Nx process handling while its static owner command receives no dynamic shell interpolation. */
export async function executeOwnerArgumentsV1(options,context,delegate){
 if(typeof options.command!=="string"||options.commands!==undefined||options.env?.[key]!==undefined||/\{args[.}]/u.test(options.command))throw Error("Owner executor requires one declared command and a private argv envelope");
 if(options.args!==undefined&&!Array.isArray(options.args))throw Error("Owner executor requires literal argument arrays");
 if(options.forwardAllArgs!==false&&!Array.isArray(options.__unparsed__))throw Error("Nx did not admit the original forwarded arguments");
 const arguments_=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]];
 const environment={...process.env,...options.env,SEMIO_BUILD_TOOL:"nx",SEMIO_WORKSPACE_ROOT:context.root,SEMIO_TASK_PROJECT:context.projectName,SEMIO_TASK_TARGET:context.targetName,...storeReceiptEnvironmentV1(context)};
 const orchestration=await issueNxBuildOrchestrationV1(options,context,environment,Date.now());if(orchestration){environment.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(orchestration);delete environment.SEMIO_SCRIPT_CAPABILITIES;}
 if(options.nativeOwnerCommand!==undefined){environment.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(await issueNativeOwnerInvocationV1(options,context,environment,Date.now()));delete environment.SEMIO_SCRIPT_CAPABILITIES;}
 else if(/ native owner-command /u.test(options.command))throw Error("Original discovered native request required");
 const plan={...options,args:undefined,__unparsed__:[],forwardAllArgs:false,env:{...environment,[key]:encodeOwnerArgumentsV1(arguments_)}};
 return delegate(plan,context);
}

/** 🚀️ Delegates the registered native/process owner route through the current Nx lifecycle. */
export default async function executor(options,context){
 const require=createRequire(import.meta.url),registryPath=require.resolve("nx/executors.json"),registry=JSON.parse(readFileSync(registryPath,"utf8")),implementation=registry.executors?.["run-commands"]?.implementation;
 if(typeof implementation!=="string")throw Error("Nx has no registered process executor");
 const delegate=require(resolve(dirname(registryPath),implementation)).default;
 return executeOwnerArgumentsV1(options,context,delegate);
}
