import {createHash} from "node:crypto";
import {readFile,realpath} from "node:fs/promises";
import {delimiter,dirname,join} from "node:path";
import {taskFamilyV1} from "../../../🏃️execution/🟦️.ts";
import {nativeToolBinaryDirectoryV1,toolchainHostV1,toolchainRootsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import componentSchema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=componentSchema.definitions.Policy.const,hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");

/** 🐹️ The closed identity a Go task is keyed by besides its sources: hashes of the toolchain version file and binary, the host and the values of every variable that changes what `go` compiles. */
export interface BazelGoIdentityV1{version:string|null;binary:string|null;host:string|null;environment:Record<string,string>}

/** 🧮️ True for a task of the Go family, decided by the same ordered rule table that classifies every task for coverage, so Bazel and the neutral contract agree on what is Go. */
export function bazelGoClassV1(task:{project?:string;target:string;root:string;definition:{execution:{kind:string;command?:string}}}):boolean{
 const execution=task.definition.execution;
 return taskFamilyV1({...(task.project!==undefined?{project:task.project}:{}),target:task.target,root:task.root,kind:execution.kind==="noop"?"noop":"command",...(execution.command!==undefined?{command:execution.command}:{}),cache:{cache:true}})==="go";
}

/** 🧊️ Resolves the `go` binary like a shell would, without spawning it: `GOROOT` first, then the `PATH` entries in order; the first existing candidate wins and its links are resolved. */
export async function bazelGoBinaryV1(environment:Readonly<Record<string,string|undefined>>,platform:string):Promise<string|undefined>{
 const name=platform.startsWith("win32")?policy.binary.windows:policy.binary.posix,[system,architecture]=platform.split("-"),pinned=environment.SEMIO_WORKSPACE_ROOT===undefined?undefined:join(nativeToolBinaryDirectoryV1("go",toolchainHostV1(system,architecture),toolchainRootsV1(environment.SEMIO_WORKSPACE_ROOT).tools),name),candidates=pinned!==undefined?[pinned]:[...(environment.GOROOT?[join(environment.GOROOT,"bin",name)]:[]),...(environment.PATH??"").split(delimiter).filter(Boolean).map(directory=>join(directory,name))];
 for(const candidate of candidates)try{return await realpath(candidate);}catch(error){if(!["ENOENT","ENOTDIR","EACCES"].includes((error as NodeJS.ErrnoException).code??""))throw error;}
 return undefined;
}

/** 🔐️ Reads the toolchain identity without a compiler process: the `VERSION` file of its root and the binary itself, so an upgrade or a reinstall changes it while a moved checkout does not. */
export async function bazelGoIdentityV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform+"-"+process.arch):Promise<BazelGoIdentityV1>{
 const read=async(path:string)=>{try{return await readFile(path);}catch(error){if(["ENOENT","EISDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
 const binary=await bazelGoBinaryV1(environment,platform),bytes=binary?await read(binary):undefined,version=binary?await read(join(dirname(dirname(binary)),policy.versionFile)):undefined;
 return{version:version?hash(version):null,binary:bytes?hash(bytes):null,host:(policy.hosts as Record<string,string>)[platform]??null,environment:Object.fromEntries(policy.environment.flatMap(name=>environment[name]===undefined?[]:[[name,environment[name]!]]))};
}

/** 🔑️ Names the identity by one digest that re-keys every Go action when the toolchain, the host or a compile-relevant variable changes. */
export async function bazelGoToolchainDigestV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform+"-"+process.arch):Promise<string>{
 return hash(JSON.stringify(await bazelGoIdentityV1(environment,platform)));
}
