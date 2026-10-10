import {dirname,resolve} from "node:path";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));
const key="SEMIO_OWNER_ARGUMENTS";
const executorSchema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️executor.json",import.meta.url),"utf8"));
const nativeCallerSchema=JSON.parse(readFileSync(new URL("./🧬️schema/📥️native-caller.json",import.meta.url),"utf8"));
const nativeSchema=JSON.parse(readFileSync(new URL("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",import.meta.url),"utf8"));

/** 🧬️ Admits the closed native caller and command contracts in the Node executor. */
function conforms(contract,value){
 if(contract.const!==undefined&&value!==contract.const)return false;
 if(contract.type==="object"){if(value===null||typeof value!=="object"||Array.isArray(value))return false;const keys=Object.keys(value);if(contract.required?.some(key=>!Object.hasOwn(value,key)))return false;if(contract.additionalProperties===false&&keys.some(key=>!Object.hasOwn(contract.properties??{},key)))return false;return Object.entries(contract.properties??{}).every(([key,rule])=>!Object.hasOwn(value,key)||conforms(rule,value[key]));}
 if(contract.type==="array")return Array.isArray(value)&&(contract.minItems===undefined||value.length>=contract.minItems)&&(contract.maxItems===undefined||value.length<=contract.maxItems)&&value.every(item=>conforms(contract.items,item));
 if(contract.type==="integer")return Number.isSafeInteger(value)&&(contract.minimum===undefined||value>=contract.minimum)&&(contract.maximum===undefined||value<=contract.maximum);
 if(contract.type==="boolean")return typeof value==="boolean";
 if(contract.type==="string")return typeof value==="string"&&(contract.minLength===undefined||value.length>=contract.minLength)&&(contract.maxLength===undefined||value.length<=contract.maxLength)&&(!contract.pattern||new RegExp(contract.pattern,"u").test(value));
 return Object.hasOwn(contract,"const");
}

/** 📜️ Binds one discovered literal native child command without shell evaluation. */
export function declaredNativeOwnerCommandV1(manifest,workingDirectory,command){
 const tokens=[];let cursor=0;
 while(cursor<command.length){while(/\s/u.test(command[cursor]??"")&&cursor<command.length)cursor++;if(cursor===command.length)break;const start=cursor;
  if(command[cursor]==='"'){cursor++;while(cursor<command.length){if(command[cursor]==="\\"){cursor+=2;continue;}if(command[cursor++]==='"')break;}tokens.push(JSON.parse(command.slice(start,cursor)));}
  else if(command[cursor]==="'"){cursor++;const end=command.indexOf("'",cursor);if(end<0)throw Error("Unclosed original native command");tokens.push(command.slice(cursor,end));cursor=end+1;}
  else{while(cursor<command.length&&!/\s/u.test(command[cursor]))cursor++;const token=command.slice(start,cursor);if(/[;&|<>`$"']/u.test(token))throw Error("Native command requires literal discovered arguments");tokens.push(token);}
  if(cursor<command.length&&!/\s/u.test(command[cursor]))throw Error("Native command requires distinct literal arguments");
 }
 const result={kind:"native-owner-command",manifest,workingDirectory,program:tokens[0],arguments:tokens.slice(1)};
 if(!conforms(nativeSchema.properties.command,result))throw Error("Original discovered native command refused");return result;
}

/** 📥️ Issues the exact discovered descendant with the original caller's unchanged deadline and resources. */
export function issueNativeOwnerInvocationV1(options,context,environment,now){
 const wire=environment.SEMIO_SCRIPT_PROCESS_INVOCATION;if(typeof wire!=="string")throw Error("Original Nx native parent invocation required");
 const original=JSON.parse(wire),policy=original?.policy,caller=original?.capabilities;
 if(original?.version!==1||Object.keys(original).length!==4||!policy||Object.keys(policy).length!==3||policy.version!==1||typeof policy.owner!=="string"||policy.owner.length===0||!Number.isSafeInteger(policy.maximumElapsedMilliseconds)||policy.maximumElapsedMilliseconds<0||!Number.isFinite(now)||(policy.maximumElapsedMilliseconds===0?original.deadlineEpochMilliseconds!==null:!Number.isSafeInteger(original.deadlineEpochMilliseconds)||original.deadlineEpochMilliseconds<=now||original.deadlineEpochMilliseconds-now>policy.maximumElapsedMilliseconds)||!conforms(nativeCallerSchema,caller))throw Error("Original Nx native authority refused");
 if(environment.SEMIO_SCRIPT_CAPABILITIES!==undefined&&JSON.stringify(JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES))!==JSON.stringify(caller))throw Error("Original Nx capabilities changed before native dispatch");
 if(resolve(caller.workspaceRoot)!==resolve(context.root)||caller.arguments[0]!=="nx"||caller.selection[0]!=="nx")throw Error("Original selected Nx command required");
 const graph=context.taskGraph;if(!graph?.tasks||!graph.dependencies||!Array.isArray(graph.roots))throw Error("Original Nx task graph required");
 let selectedTasks;
 if(caller.selection[1]==="run"){
  const parts=caller.selection[2].split(":"),selected={project:parts[0],target:parts[1],configuration:parts[2]};if(parts.length<2||parts.length>3||!selected.project||!selected.target)throw Error("Original selected Nx target refused");
  const configuration=selected.configuration??context.projectsConfigurations?.projects?.[selected.project]?.targets?.[selected.target]?.defaultConfiguration;
  selectedTasks=Object.keys(graph.tasks).filter(id=>{const target=graph.tasks[id]?.target;return target?.project===selected.project&&target?.target===selected.target&&(target.configuration??undefined)===configuration;});
  if(selectedTasks.length!==1)throw Error("Original selected Nx graph task refused");
 }else if(caller.selection[1]==="run-many"){
  if(!context.projectGraph?.nodes)throw Error("Original Nx project graph required");
  const require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json")),{withOverrides}=require(resolve(nxRoot,"dist/src/command-line/yargs-utils/shared-options.js")),{yargsRunManyCommand}=require(resolve(nxRoot,"dist/src/command-line/run-many/command-object.js")),{splitArgsIntoNxArgsAndOverrides}=require(resolve(nxRoot,"dist/src/utils/command-line-utils.js")),{projectsToRun}=require(resolve(nxRoot,"dist/src/command-line/run-many/run-many.js")),yargs=createRequire(resolve(nxRoot,"package.json"))("yargs/yargs");
  const parsed=withOverrides(yargsRunManyCommand.builder(yargs()).exitProcess(false).parseSync(caller.selection.slice(1))),{nxArgs}=splitArgsIntoNxArgsAndOverrides(parsed,"run-many",{printWarnings:false},context.nxJsonConfiguration??{}),projects=new Set(projectsToRun(nxArgs,context.projectGraph).map(project=>project.name));
  selectedTasks=Object.keys(graph.tasks).filter(id=>{const target=graph.tasks[id]?.target,configuration=nxArgs.configuration??context.projectsConfigurations?.projects?.[target?.project]?.targets?.[target?.target]?.defaultConfiguration;return projects.has(target?.project)&&nxArgs.targets.includes(target?.target)&&(target.configuration??undefined)===configuration;});
  if(selectedTasks.length===0)throw Error("Original selected Nx graph tasks refused");
 }else throw Error("Original selected Nx command required");
 const reachable=new Set(),pending=[...selectedTasks];while(pending.length){const id=pending.pop();if(reachable.has(id))continue;if(!graph.tasks[id]||!Array.isArray(graph.dependencies[id]))throw Error("Original Nx dependency graph incomplete");reachable.add(id);pending.push(...graph.dependencies[id]);}
 const admitted=[...reachable].filter(id=>{const target=graph.tasks[id].target;return target.project===context.projectName&&target.target===context.targetName&&(target.configuration??undefined)===(context.configurationName??undefined);});
 const declared=context.projectsConfigurations?.projects?.[context.projectName]?.targets?.[context.targetName],request=options.nativeOwnerCommand;
 if(admitted.length!==1||!declared||!conforms(nativeSchema.properties.command,request))throw Error("Original discovered native task command refused");
 const task=graph.tasks[admitted[0]],require=createRequire(import.meta.url),nxRoot=dirname(require.resolve("nx/package.json")),{combineOptionsForExecutor}=require(resolve(nxRoot,"dist/src/utils/params.js"));
 const expected=combineOptionsForExecutor(structuredClone(task.overrides),task.target.configuration,structuredClone(declared),executorSchema,context.projectName,context.cwd??".",false);
 if(expected.command!==options.command||JSON.stringify(expected.nativeOwnerCommand)!==JSON.stringify(request)||(expected.forwardAllArgs!==false)!==(options.forwardAllArgs!==false))throw Error("Original discovered native task command changed");
 const expectedArguments=expected.forwardAllArgs===false?[]:[...expected.args??[],...expected.__unparsed__??[]],actualArguments=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]];
 if(JSON.stringify(expectedArguments)!==JSON.stringify(actualArguments))throw Error("Original native task arguments changed");
 if(typeof environment.CARGO_TARGET_DIR!=="string"||resolve(context.root,caller.native.artifactDirectory)!==resolve(context.root,environment.CARGO_TARGET_DIR)||environment.CARGO_NET_OFFLINE!==String(caller.native.network.offline))throw Error("Original native artifact or network authority changed");
 const forwarded=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]],capabilities={version:1,repositoryRoot:context.root,artifactDirectory:resolve(context.root,caller.native.artifactDirectory),command:{...request,arguments:[...request.arguments,...forwarded]},transport:caller.native.transport,child:caller.native.child,network:caller.native.network};
 if(!conforms(nativeSchema,capabilities))throw Error("Issued original native capability refused");return {...original,capabilities};
}

/** 🛡️ Admits literal process arguments against their owned envelope authority. */
function admit(value){
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).length!==schema.required.length||value.version!==schema.properties.version.const||!Array.isArray(value.arguments)||value.arguments.length>schema.properties.arguments.maxItems||value.arguments.some(argument=>typeof argument!=="string"||argument.length>schema.properties.arguments.items.maxLength||argument.includes("\u0000")))throw Error("Invalid owner argument envelope");
 return [...value.arguments];
}

/** 📤️ Encodes raw argv as environment data independently of platform shell syntax. */
export function encodeOwnerArgumentsV1(arguments_){
 const value={version:1,arguments:arguments_};admit(value);return JSON.stringify(value);
}

/** 📥️ Removes the private envelope before handing unchanged arguments to the actual child owner. */
export function consumeOwnerArgumentsV1(environment){
 const next={...environment},encoded=next[key];delete next[key];
 if(encoded===undefined)return {arguments:[],environment:next};
 return {arguments:admit(JSON.parse(encoded)),environment:next};
}

/** 🎯️ Keeps Nx process handling while its static owner command receives no dynamic shell interpolation. */
export async function executeOwnerArgumentsV1(options,context,delegate){
 if(typeof options.command!=="string"||options.commands!==undefined||options.env?.[key]!==undefined||/\{args[.}]/u.test(options.command))throw Error("Owner executor requires one declared command and a private argv envelope");
 if(options.args!==undefined&&!Array.isArray(options.args))throw Error("Owner executor requires literal argument arrays");
 if(options.forwardAllArgs!==false&&!Array.isArray(options.__unparsed__))throw Error("Nx did not admit the original forwarded arguments");
 const arguments_=options.forwardAllArgs===false?[]:[...options.args??[],...options.__unparsed__??[]];
 const environment={...process.env,...options.env};
 if(options.nativeOwnerCommand!==undefined){environment.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(issueNativeOwnerInvocationV1(options,context,environment,Date.now()));delete environment.SEMIO_SCRIPT_CAPABILITIES;}
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
