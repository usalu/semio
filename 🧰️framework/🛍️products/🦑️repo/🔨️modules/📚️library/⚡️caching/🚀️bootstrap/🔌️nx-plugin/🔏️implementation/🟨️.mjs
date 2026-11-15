import{fileURLToPath}from"node:url";
import{REVISION_POLICY_V1,observeCurrentSourceRevisionV1}from"../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {ownerArgumentsSourceIdentityV1,ownerArgumentsSourceRevisionsV1} from "../../../../🏗️build/📤️arguments/🟨️.mjs";
const library=new URL("../../../../",import.meta.url),read=(path,captured)=>captured??readFileSync(new URL(path,library)),sha=bytes=>createHash("sha256").update(bytes).digest("hex");
const projectionSources=["./🟨️.mjs","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/🔏️implementation/🟨️.mjs","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📌️graph-pin/🟨️.mjs","./⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"];
const publicationSchemaSource=read("./🏗️build/🧬️schema/📥️pin.json");
const engineSchemaSource=read("./🏗️build/🧬️schema/📥️engine.json"),engineSchema=JSON.parse(engineSchemaSource.toString("utf8"));
const privateTaskSources=["./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/🔣️executor.json","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json","./⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️source.json","./🏗️build/📥️caller/📤️orchestration/🟨️.mjs","./🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json","./🏗️build/🧬️schema/📥️caller.json"],privateTaskBytes=privateTaskSources.map(path=>read(path));
const dependencySourceIdentity=(captured=false)=>sha(JSON.stringify([...projectionSources.map(path=>sha(read(path))),sha(read("./🏗️build/🧬️schema/📥️pin.json",captured?publicationSchemaSource:undefined)),sha(read("./🏗️build/🧬️schema/📥️engine.json",captured?engineSchemaSource:undefined)),ownerArgumentsSourceIdentityV1(),...privateTaskSources.map((path,index)=>sha(read(path,captured?privateTaskBytes[index]:undefined)))])),loadedDependencySourceIdentity=dependencySourceIdentity(true);

const loadedSourceRevisions=import.meta.semioFrozenAuthorizers?undefined:Object.freeze([...new Map([...projectionSources,"./🏗️build/🧬️schema/📥️pin.json","./🏗️build/🧬️schema/📥️engine.json",...privateTaskSources].map(path=>{const absolute=fileURLToPath(new URL(path,library));return[absolute,Object.freeze({path:absolute,sha256:sha(read(path))})];}).concat(ownerArgumentsSourceRevisionsV1().map(row=>[row.path,row]))).values()]);
/** 👁️ Observes current physical dependency Source DATA before delegating to a child that loads mutable workspace modules. */
export function nxBuildDependencySourceChangesV1(){if(!loadedSourceRevisions)throw Error("Live private Nx dependency Source revisions required");return Object.freeze(loadedSourceRevisions.filter(row=>observeCurrentSourceRevisionV1(row.path,REVISION_POLICY_V1.maximumSourceBytes).sha256!==row.sha256).map(row=>row.path));}

/** 🧾️ The revisions of the sources this implementation evaluated, as plain rows a fresh process can report and the Original can compare with the disk later. */
export function nxBuildDependencySourceRevisionsV1(){if(!loadedSourceRevisions)throw Error("Live private Nx dependency Source revisions required");return Object.freeze(loadedSourceRevisions.map(row=>Object.freeze({path:row.path,sha256:row.sha256})));}

/** 👁️ Observes which of the given revisions no longer equal the physical Source DATA, in the order given. */
export function nxBuildDependencySourceChangesFromV1(revisions){return Object.freeze(revisions.filter(row=>observeCurrentSourceRevisionV1(row.path,REVISION_POLICY_V1.maximumSourceBytes).sha256!==row.sha256).map(row=>row.path));}

const probeCode='import(process.env.SEMIO_NX_PROJECTION_PROBE).then(module=>process.stdout.write(JSON.stringify({identity:module.nxBuildDependencyImplementationV1(),revisions:module.nxBuildDependencyImplementationRevisionsV1()}))).catch(error=>{process.stderr.write(String(error?.message??error).split("\\n")[0]);process.exit(1);});';
/** 🌱️ Lets a fresh process evaluate the current projection sources and report their identity and revisions: the process is the admission of the current code, the Original adopts what it reports. A process that fails, times out or answers more than the bound is an error. */
export function nxBuildDependencySourceProbeV1({execPath=process.execPath,timeoutMilliseconds=120000,maximumBytes=1048576,run=spawnSync,root=fileURLToPath(new URL("./🟨️.mjs",library))}={}){
 const result=run(execPath,["--eval",probeCode],{encoding:"utf8",timeout:timeoutMilliseconds,maxBuffer:maximumBytes,env:{...process.env,SEMIO_NX_PROJECTION_PROBE:root},stdio:["ignore","pipe","pipe"],windowsHide:true});
 if(result.error)throw result.error;
 if(result.status!==0)throw Error("Projection probe ended with status "+result.status+(result.stderr?": "+String(result.stderr).split("\n")[0].slice(0,200):""));
 const value=JSON.parse(result.stdout);
 if(typeof value?.identity!=="string"||!/^[a-f0-9]{64}$/u.test(value.identity)||!Array.isArray(value.revisions)||!value.revisions.length||value.revisions.some(row=>typeof row?.path!=="string"||typeof row?.sha256!=="string"))throw Error("Projection probe answered no identity");
 return Object.freeze({identity:value.identity,revisions:Object.freeze(value.revisions.map(row=>Object.freeze({path:row.path,sha256:row.sha256})))});
}

/** 🧬️ Pins the engine named input to the exact schema bytes this implementation evaluated. */
export const nxEngineNamedInputV1=engineSchema.definitions.NamedInput.const;

/** 🦀️ Pins the engine named input of commands that compile Rust, which also keys the scheduling code that builds Cargo arguments, to the same schema bytes. */
export const nxEngineNativeNamedInputV1=engineSchema.definitions.NativeNamedInput.const;

/** 🔏️ Refuses the plugin root, its private leaves, schemas or contract owners changed after actual module evaluation. */
export function nxBuildDependencySourceIdentityV1(){if(dependencySourceIdentity()!==loadedDependencySourceIdentity)throw Error("Loaded private Nx dependency Source changed");return loadedDependencySourceIdentity;}
