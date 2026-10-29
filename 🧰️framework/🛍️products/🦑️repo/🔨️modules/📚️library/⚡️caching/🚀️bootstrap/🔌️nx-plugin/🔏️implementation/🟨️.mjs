import{fileURLToPath}from"node:url";
import{REVISION_POLICY_V1,observeCurrentSourceRevisionV1}from"../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {readFileSync} from "node:fs";
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

/** 🧬️ Pins the engine named input to the exact schema bytes this implementation evaluated. */
export const nxEngineNamedInputV1=engineSchema.definitions.NamedInput.const;

/** 🦀️ Pins the engine named input of commands that compile Rust, which also keys the scheduling code that builds Cargo arguments, to the same schema bytes. */
export const nxEngineNativeNamedInputV1=engineSchema.definitions.NativeNamedInput.const;

/** 🔏️ Refuses the plugin root, its private leaves, schemas or contract owners changed after actual module evaluation. */
export function nxBuildDependencySourceIdentityV1(){if(dependencySourceIdentity()!==loadedDependencySourceIdentity)throw Error("Loaded private Nx dependency Source changed");return loadedDependencySourceIdentity;}
