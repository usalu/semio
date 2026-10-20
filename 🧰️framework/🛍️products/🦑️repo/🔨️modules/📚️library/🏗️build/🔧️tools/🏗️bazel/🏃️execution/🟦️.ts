import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{checkResourceLeaseControl,withResourceLeases,type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{requireNativeStoragePathV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type{RepositoryToolingStoreV1}from"../../../../⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{bunArchiveExecutable,bunDistribution}from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🟦️.ts";
import bunManifest from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json" with {type:"json"};
import hermeticSchema from"../../../🏃️execution/🧬️schema/🔒️hermetic.json" with {type:"json"};
import schema from"./🧬️schema/🔣️.json" with {type:"json"};
import type{BazelTaskV1}from"../📋️task-plan/🟦️.ts";
import{createHash}from"node:crypto";
import{chmod,lstat,mkdir,mkdtemp,readFile,readlink,rename,rm,symlink,writeFile}from"node:fs/promises";
import{dirname,join}from"node:path";
export{BAZEL_HERMETIC_RUNNER}from"./🏭️runner/🟦️.ts";

const contract=schema.definitions.Contract.const,audit=schema.definitions.Audit.const,reasons=schema.definitions.Reasons.const,bun=bunManifest.definitions.Release.const;
if(validateJsonSchemaSubset(schema.definitions.ContractShape,contract,schema).length)throw new Error("Closed Bazel execution contract required");
if(validateJsonSchemaSubset(schema.definitions.AuditShape,audit,schema).length)throw new Error("Closed Bazel hermeticity audit required");

/** 🖥️ One execution platform of the contract: Node's platform and arch names, the Bazel constraint values and the properties a remote worker pool matches. */
export type BazelExecutionPlatformV1={readonly id:string;readonly platform:string;readonly arch:string;readonly os:string;readonly cpu:string;readonly properties:Readonly<Record<string,string>>};
/** 🧾️ Why a target stays out of the hermetic class: every reason is authored in the schema. */
export type BazelHermeticReasonV1=keyof typeof reasons;
/** ⚖️ Whether a target is a hermetic action or keeps host state, with the closed reason. */
export type BazelHermeticDecisionV1={hermetic:true}|{hermetic:false;reason:BazelHermeticReasonV1};
/** 🌍️ The parsed remote execution switch: endpoint, routed platform, extra worker properties and the concurrency bound. */
export interface BazelRemoteExecutionV1{endpoint:string;platform:string;properties:Readonly<Record<string,string>>;jobs:number}
/** 🧭️ The platforms one generated workspace declares: the host, an optional remote pool and the provisioned bun executable of each. */
export interface BazelExecutionPlanV1{host:string;remote?:{platform:string;properties:Readonly<Record<string,string>>};tools:Readonly<Record<string,string>>}

/** 🖥️ Selects the contract platform for Node's platform and arch names. */
export function bazelExecutionPlatformV1(platform:string,arch:string):BazelExecutionPlatformV1{
 const row=contract.platforms.find(row=>row.platform===platform&&row.arch===arch);
 if(!row)throw new Error(`Unsupported Bazel execution platform: ${platform}/${arch}`);
 return row;
}

/** 🖥️ Selects the contract platform by its id. */
export function bazelExecutionPlatformByIdV1(id:string):BazelExecutionPlatformV1{
 const row=contract.platforms.find(row=>row.id===id);
 if(!row)throw new Error(`Unknown Bazel execution platform: ${id}`);
 return row;
}

/** 📛️ Names a platform's bun executable file: Windows executables carry `.exe`. */
export function bazelBunExecutableNameV1(platform:BazelExecutionPlatformV1):string{return contract.toolchain.executable+(platform.platform==="win32"?".exe":"");}

/** 🌍️ Reads the optional remote execution switch from the environment; unset or empty keeps every action local and a malformed value fails closed. */
export function bazelRemoteExecutionV1(environment:Readonly<Record<string,string|undefined>>):BazelRemoteExecutionV1|undefined{
 const names=contract.environment,selected:Record<string,string>={};
 for(const name of Object.values(names)){const value=environment[name];if(value!==undefined&&value!=="")selected[name]=value;}
 if(validateJsonSchemaSubset(schema.definitions.Environment,selected,schema).length)throw new Error("Closed Bazel remote execution environment required");
 const endpoint=selected[names.endpoint];if(endpoint===undefined)return undefined;
 const platform=bazelExecutionPlatformByIdV1(selected[names.platform]??contract.remote.defaultPlatform),properties:Record<string,string>={...platform.properties};
 for(const pair of (selected[names.properties]??"").split(",").filter(Boolean)){const cut=pair.indexOf("=");properties[pair.slice(0,cut)]=pair.slice(cut+1);}
 const jobs=Math.min(contract.remote.maximumJobs,selected[names.jobs]===undefined?contract.remote.defaultJobs:Number(selected[names.jobs]));
 return{endpoint,platform:platform.id,properties:Object.freeze(Object.fromEntries(Object.entries(properties).sort(([left],[right])=>left<right?-1:left>right?1:0))),jobs};
}

/** 🧭️ Derives the closed Bazel flags that route hermetic actions: the platforms and toolchains to register and, only with a remote executor, the executor and no local fallback. Cache flags, including the output download mode that the snapshot restore needs, stay with the remote cache contract. */
export function bazelExecutionFlagsV1(options:Readonly<{host:{platform:string;arch:string};hermetic:boolean;jobs:number;environment?:Readonly<Record<string,string|undefined>>}>):Readonly<{command:readonly string[];remote:boolean;platform:string;jobs:number}>{
 const host=bazelExecutionPlatformV1(options.host.platform,options.host.arch);
 if(!options.hermetic)return Object.freeze({command:Object.freeze([]),remote:false,platform:host.id,jobs:options.jobs});
 const remote=bazelRemoteExecutionV1(options.environment??{}),ids=[...new Set([host.id,...(remote?[remote.platform]:[])])],command=[`--extra_execution_platforms=${[`//:exec_${host.id.replace("-","_")}`,...(remote?[`//:exec_remote_${remote.platform.replace("-","_")}`]:[])].join(",")}`,`--extra_toolchains=${ids.map(id=>`//:bun_toolchain_${id.replace("-","_")}`).join(",")}`];
 if(remote)command.push(`--remote_executor=${remote.endpoint}`,"--noremote_local_fallback");
 const flags={command,remote:remote!==undefined,platform:remote?remote.platform:host.id,jobs:remote?remote.jobs:options.jobs};
 if(validateJsonSchemaSubset(schema.definitions.Flags,flags,schema).length)throw new Error("Closed Bazel execution flags required");
 return Object.freeze({...flags,command:Object.freeze(command)});
}

/** 🛡️ Decides whether a cacheable task is a hermetic action: it must carry the neutral declaration, a command the runner interprets, declared outputs and file inputs only; anything else keeps host state with its reason. */
export function bazelHermeticDecisionV1(task:BazelTaskV1,cache:{cache:boolean},inputs:readonly unknown[]|(()=>readonly unknown[])):BazelHermeticDecisionV1{
 const semio=(task.definition as {metadata?:{semio?:{hermetic?:unknown;execution?:{kind?:string}}}}).metadata?.semio,execution=task.definition.execution;
 if(task.definition.continuous)return{hermetic:false,reason:"continuous"};
 if(semio?.execution?.kind==="orchestration")return{hermetic:false,reason:"orchestration"};
 if(semio?.hermetic===undefined)return{hermetic:false,reason:"undeclared"};
 if(validateJsonSchemaSubset(hermeticSchema,semio.hermetic,hermeticSchema).length)throw new Error(`Closed hermetic declaration required: ${task.id}`);
 const commands=execution.commands??(execution.command===undefined?[]:[execution.command]),text=JSON.stringify([commands,execution.cwd,execution.env,task.definition.outputs]);
 if(execution.kind!=="command"||execution.program!==undefined||execution.ownerArguments!==undefined||execution.readyWhen!==undefined||!commands.length||commands.some(command=>typeof command!=="string")||/\{options\.|\{env\./u.test(text)||Object.values(execution.env??{}).some(value=>typeof value!=="string"))return{hermetic:false,reason:"unsupported-execution"};
 if((typeof inputs==="function"?inputs():inputs).some(input=>typeof input==="object"&&input!==null&&("runtime"in input||"env"in input)))return{hermetic:false,reason:"host-inputs"};
 if(!cache.cache||!Array.isArray(task.definition.outputs))return{hermetic:false,reason:"uncached"};
 return{hermetic:true};
}

/** 🗝️ Builds the closed, content-stable manifest the hermetic runner interprets; it holds no path, environment or implementation identity. */
export function bazelHermeticManifestV1(task:BazelTaskV1,args:readonly string[]):Record<string,unknown>{
 const execution=task.definition.execution,commands=(execution.commands??(execution.command===undefined?[]:[execution.command])) as string[],manifest={version:1,id:task.id,project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{}),root:task.root,...(execution.cwd===undefined?{}:{cwd:execution.cwd}),args:[...args],...(execution.forwardAllArgs===undefined?{}:{forwardAllArgs:execution.forwardAllArgs}),...(execution.parallel===undefined?{}:{parallel:execution.parallel}),commands,...(execution.env===undefined?{}:{env:{...execution.env}}),outputs:[...(task.definition.outputs??[])]};
 if(validateJsonSchemaSubset(schema.definitions.Manifest,manifest,schema).length)throw new Error(`Closed hermetic task manifest required: ${task.id}`);
 return manifest;
}

/** 🔬️ One spawn of Bazel's JSON execution log: the fields the hermeticity law inspects. */
export interface BazelExecutionSpawnV1{mnemonic:string;runner:string;commandArgs:readonly string[];environmentVariables:readonly {name:string;value:string}[];inputs:readonly {path:string;isTool?:boolean}[];platform?:{properties?:readonly {name:string;value:string}[]};targetLabel:string}

/** 🔬️ Judges one logged hermetic spawn against what its action declared: an isolated runner, exactly the declared and tool inputs plus its dependency snapshots, the strict environment, no absolute argument and the platform's properties. Returns every violation; an empty list is a hermetic spawn. */
export function bazelHermeticViolationsV1(spawn:BazelExecutionSpawnV1,expected:Readonly<{inputs:readonly string[];tool:string;properties:Readonly<Record<string,string>>}>):string[]{
 const found:string[]=[],paths=spawn.inputs.map(input=>input.path),allowed=new Set([...expected.inputs,expected.tool]),tools=spawn.inputs.filter(input=>input.isTool).map(input=>input.path),pairs=(rows:readonly {name:string;value:string}[])=>JSON.stringify(rows.map(row=>[row.name,row.value]).sort());
 if(!/sandbox|remote/u.test(spawn.runner))found.push(`runner: ${spawn.runner}`);
 for(const path of paths)if(!allowed.has(path)&&!/\.snapshot\/(snapshot\.json|files\/[a-f0-9]{32})$/u.test(path))found.push(`undeclared input: ${path}`);
 for(const path of expected.inputs)if(!paths.includes(path))found.push(`missing input: ${path}`);
 if(JSON.stringify(tools)!==JSON.stringify([expected.tool]))found.push(`tool inputs: ${tools.join(",")||"none"}`);
 if(pairs(spawn.environmentVariables)!==pairs(Object.entries(contract.actionEnvironment).map(([name,value])=>({name,value}))))found.push(`environment: ${spawn.environmentVariables.map(row=>row.name).join(",")||"empty"}`);
 for(const argument of spawn.commandArgs)if(/^(\/|[A-Za-z]:[\\/])/u.test(argument))found.push(`absolute argument: ${argument}`);
 if(pairs(Object.entries(spawn.platform?.properties?.length?Object.fromEntries(spawn.platform.properties.map(row=>[row.name,row.value])):{}).map(([name,value])=>({name,value})))!==pairs(Object.entries(expected.properties).map(([name,value])=>({name,value}))))found.push("platform properties");
 return found;
}

/** 📑 Names the package-relative file a platform's bun executable is linked at. */
export function bazelBunToolPathV1(platform:BazelExecutionPlatformV1):string{return `${contract.toolchain.directory}/${platform.id}/${bazelBunExecutableNameV1(platform)}`;}

/** 📑 Renders the constraint universe, the platforms with their worker properties, the bun toolchain type and one toolchain per provisioned platform; the remote platforms carry the `remote` pool so hermetic targets route to them while host actions stay local. */
export function renderBazelExecutionBuildV1(plan:BazelExecutionPlanV1):string{
 const quote=(value:string)=>JSON.stringify(value),order=(left:string,right:string)=>left<right?-1:left>right?1:0,name=(id:string)=>id.replace("-","_"),settings=contract.settings,lines:string[]=[];
 const dictionary=(row:Readonly<Record<string,string>>)=>"{\n"+Object.entries(row).sort(([left],[right])=>order(left,right)).map(([key,value])=>`        ${quote(key)}: ${quote(value)},`).join("\n")+"\n    }";
 const values=(key:"os"|"cpu")=>[...new Set(contract.platforms.map(row=>row[key]))].sort(order);
 for(const[key,setting]of [["os",settings.os],["cpu",settings.cpu]] as const){lines.push(`constraint_setting(name = ${quote(setting)})`,"");for(const value of values(key))lines.push("constraint_value(",`    name = ${quote(`${setting}_${value}`)},`,`    constraint_setting = ${quote(`:${setting}`)},`,")","");}
 lines.push(`constraint_setting(name = "exec_pool")`,"");for(const pool of ["local","remote"])lines.push("constraint_value(",`    name = "exec_pool_${pool}",`,`    constraint_setting = ":exec_pool",`,")","");
 const platform=(row:BazelExecutionPlatformV1,pool:"local"|"remote",properties:Readonly<Record<string,string>>,label:string)=>lines.push("platform(",`    name = ${quote(label)},`,`    constraint_values = [":${settings.os}_${row.os}", ":${settings.cpu}_${row.cpu}", ":exec_pool_${pool}"],`,`    exec_properties = ${dictionary(properties)},`,")","");
 const host=bazelExecutionPlatformByIdV1(plan.host);platform(host,"local",host.properties,`exec_${name(host.id)}`);
 if(plan.remote){const row=bazelExecutionPlatformByIdV1(plan.remote.platform);platform(row,"remote",plan.remote.properties,`exec_remote_${name(row.id)}`);}
 lines.push(`toolchain_type(name = ${quote(contract.toolchain.type)})`,"");
 for(const id of Object.keys(plan.tools).sort(order)){
  const row=bazelExecutionPlatformByIdV1(id),pin=bunDistribution(bun.version,row.platform,row.arch,false).sha256;
  lines.push("repo_bun_toolchain(",`    name = "bun_tool_${name(id)}",`,`    executable = ${quote(bazelBunToolPathV1(row))},`,`    pin = ${quote(pin)},`,")","","toolchain(",`    name = "bun_toolchain_${name(id)}",`,`    exec_compatible_with = [":${settings.os}_${row.os}", ":${settings.cpu}_${row.cpu}"],`,`    toolchain = ":bun_tool_${name(id)}",`,`    toolchain_type = ":${contract.toolchain.type}",`,")","");
 }
 return lines.join("\n");
}

/** 🔗️ Links each provisioned bun executable into the generated package at its toolchain path, replacing a link that points elsewhere. */
export async function linkBazelToolchainsV1(workspace:string,tools:Readonly<Record<string,string>>):Promise<void>{
 for(const[id,target]of Object.entries(tools)){
  const path=join(workspace,bazelBunToolPathV1(bazelExecutionPlatformByIdV1(id)));await mkdir(dirname(path),{recursive:true});
  try{if(await readlink(path)===target)continue;await rm(path,{force:true});}catch(error){if(!["ENOENT","EINVAL"].includes((error as NodeJS.ErrnoException).code??""))throw error;await rm(path,{force:true});}
  await symlink(target,path);
 }
}

const environmentLines=Object.entries(contract.actionEnvironment).map(([key,value])=>`${JSON.stringify(key)}: ${JSON.stringify(value)}`).join(", ");
/** 📜️ The Starlark rules of the hermetic class: a toolchain rule exposing the declared bun file and a task rule that runs the runner under it with an explicit minimal environment, no network, and no host-state tag, so Bazel may sandbox it or send it to a remote executor. */
export const BAZEL_HERMETIC_STARLARK=`
def _bun_toolchain_impl(ctx):
    return [platform_common.ToolchainInfo(bun = ctx.file.executable)]

repo_bun_toolchain = rule(implementation = _bun_toolchain_impl, attrs = {"executable": attr.label(allow_single_file = True, executable = True, cfg = "exec", mandatory = True), "pin": attr.string(mandatory = True)})

def _hermetic_impl(ctx):
    bun = ctx.toolchains["//:${contract.toolchain.type}"].bun
    output = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    snapshots = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    ctx.actions.run(executable = bun, tools = [bun], arguments = [ctx.file.runner.path, ctx.file.manifest.path, output.path] + [snapshot.path for snapshot in snapshots], inputs = depset([ctx.file.manifest, ctx.file.runner] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "${contract.toolchain.mnemonic}", progress_message = ctx.attr.identity, env = {${environmentLines}}, use_default_shell_env = False, execution_requirements = {"block-network": "1"}, toolchain = "//:${contract.toolchain.type}")
    return [DefaultInfo(files = depset([output]))]

repo_hermetic_task = rule(implementation = _hermetic_impl, toolchains = ["//:${contract.toolchain.type}"], attrs = {"manifest": attr.label(allow_single_file = True), "runner": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "identity": attr.string(mandatory = True)})
`;

/** 🔐️ Reads the pinned bun release row for one platform id. */
export function bazelBunDistributionV1(id:string){const row=bazelExecutionPlatformByIdV1(id);return bunDistribution(bun.version,row.platform,row.arch,false);}

/** 📍️ Names where a platform's verified bun executable lives inside the tools partition. */
export function bazelBunStorePathV1(toolsRoot:string,id:string):string{
 const row=bazelExecutionPlatformByIdV1(id),pin=bazelBunDistributionV1(id);
 return join(requireNativeStoragePathV1(toolsRoot,join(toolsRoot,"bazel/bun")),pin.sha256.slice(0,16),bazelBunExecutableNameV1(row));
}

/** 🔐️ Verifies a downloaded archive against its pin before a byte is extracted and returns the executable's bytes. */
export function verifyBazelBunArchiveV1(archive:Uint8Array,pin:Readonly<{bytes:number;sha256:string}>,member:string):Buffer{
 if(archive.length!==pin.bytes||createHash("sha256").update(archive).digest("hex")!==pin.sha256)throw new Error("Bun archive checksum mismatch");
 return bunArchiveExecutable(Buffer.from(archive),member);
}

/** 📊️ Progress of one bun acquisition: waiting for the lease, verifying an existing executable, downloading the pinned archive, ready. */
export type BazelBunAcquisitionProgressV1={readonly phase:"waiting"|"verifying"|"downloading"|"ready";readonly bytes:number;readonly total:number};

/** 📥️ Acquires one platform's bun executable under its release pin: the archive size and SHA-256 are verified before a byte is extracted, the executable is published atomically and an existing one is accepted only when its checksum still matches the extraction receipt. */
export async function prepareBazelBunV1(workspace:string,store:RepositoryToolingStoreV1,id:string,phaseControl:ResourceLeaseControl,options:Readonly<{offline?:boolean;fetchArchive?:(url:string,signal:AbortSignal)=>Promise<Response>;progress?:(event:BazelBunAcquisitionProgressV1)=>void|Promise<void>}>={}):Promise<string>{
 checkResourceLeaseControl(phaseControl);await store.recheck();checkResourceLeaseControl(phaseControl);
 const row=bazelExecutionPlatformByIdV1(id),pin=bazelBunDistributionV1(id),executable=bazelBunStorePathV1(store.root,id),directory=dirname(executable),signal=phaseControl.signal,receipt=join(directory,".pin"),digest=async(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),progress=options.progress??(()=>undefined);
 const verified=async():Promise<boolean>=>{try{const info=await lstat(executable);if(!info.isFile()||info.isSymbolicLink())return false;return (await readFile(receipt,"utf8")).trim()===await digest(await readFile(executable));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}};
 await progress({phase:"waiting",bytes:0,total:pin.bytes});
 return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),signal,control:phaseControl,resources:[{resource:`bazel-bun-${pin.sha256}`,mode:"exclusive"}]},async()=>{
  checkResourceLeaseControl(phaseControl);await progress({phase:"verifying",bytes:0,total:pin.bytes});if(await verified()){await progress({phase:"ready",bytes:pin.bytes,total:pin.bytes});return executable;}
  if(options.offline)throw new Error(`Pinned Bun ${bun.version} for ${id} is not prepared for the original offline invocation`);
  await mkdir(dirname(directory),{recursive:true});const staging=await mkdtemp(join(dirname(directory),".prepare-"));
  try{
   const response=await (options.fetchArchive??((url,signalIn)=>fetch(url,{signal:signalIn})))(`${bun.release}/${pin.archive}`,signal);checkResourceLeaseControl(phaseControl);
   if(!response.ok||!response.body)throw new Error(`Bun download failed: ${response.status}`);
   const chunks:Buffer[]=[],reader=response.body.getReader();let bytes=0;
   try{for(let read=await reader.read();!read.done;read=await reader.read()){checkResourceLeaseControl(phaseControl);bytes+=read.value.length;if(bytes>pin.bytes)throw new Error("Bun archive exceeds its pinned size");const chunk=Buffer.from(read.value);chunks.push(chunk);await progress({phase:"downloading",bytes,total:pin.bytes});}}catch(error){await reader.cancel().catch(()=>undefined);throw error;}finally{reader.releaseLock();}
   const payload=verifyBazelBunArchiveV1(Buffer.concat(chunks),pin,`${pin.archive.slice(0,-4)}/${bazelBunExecutableNameV1(row)}`),staged=join(staging,bazelBunExecutableNameV1(row));
   await writeFile(staged,payload,{flag:"wx"});await chmod(staged,0o755);await writeFile(join(staging,".pin"),await digest(payload));
   await rm(directory,{recursive:true,force:true});checkResourceLeaseControl(phaseControl);await rename(staging,directory);await progress({phase:"ready",bytes,total:pin.bytes});return executable;
  }finally{await rm(staging,{recursive:true,force:true});}
 });
}
