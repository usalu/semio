import {createScriptProcessEnvelope,scriptProcessEnvironment,receiveScriptProcessInvocation,findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import {lstatSync} from "node:fs";
import {isAbsolute} from "node:path";
import {fileURLToPath} from "node:url";

const api=fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),custody=fileURLToPath(new URL("../../🛠️preparation/🧾️custody/🟦️.ts",import.meta.url));
/** 🧰️ Binds the actual tracked preparation owner to one independently authored physical fixture root. */
export function cargoPreparationRuntimeV1(root:string):string {
 if(!isAbsolute(root)||root.length>256)throw Error("Preparation fixture root refused");const stat=lstatSync(root);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Preparation fixture root must be physical");return api;
}
/** 🧾️ Makes neutral fixture producers declare actual source custody without copying defining machinery. */
export function observedCargoPreparationV1(root:string,source:string):string {
 cargoPreparationRuntimeV1(root);return `import {observeCargoPreparationSourceV1,completeCargoPreparationObservationV1} from ${JSON.stringify(custody)};observeCargoPreparationSourceV1(import.meta.path);\n${source}\ncompleteCargoPreparationObservationV1();\n`;
}

/** 🎟️ Authors one genuine finite fixture operation for the independently created physical root. */
export function cargoPreparationFixtureEnvironmentV1(root:string):Record<string,string|undefined>{
 cargoPreparationRuntimeV1(root);const original=fixture.originalProcess,policy={version:1 as const,owner:"physical-cargo-fixture",maximumElapsedMilliseconds:original.maximumElapsedMilliseconds},capabilities={version:1,buildMode:original.buildMode,repositoryRoot:root,artifactDirectory:process.env.SEMIO_TEST_ARTIFACT_DIR,command:{kind:"native-owner-command",manifest:"Cargo.toml",workingDirectory:".",program:process.execPath,arguments:["prepare","--manifest","Cargo.toml"]},transport:original.transport,child:{maximumElapsedMilliseconds:original.maximumElapsedMilliseconds},network:{offline:true}};
 return scriptProcessEnvironment(createScriptProcessEnvelope(policy,capabilities,Date.now()),{...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(capabilities)});
}

import {prepareCargoWorkspaceInvocation} from "../../🟦️.ts";
import type {CargoPreparationStorageV1} from "../../🛠️preparation/📦️storage/🟦️.ts";
import {createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {nativeCallerPhaseV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
/** 🎛️ Awaits the real preparation executable through the independently authored original fixture grant. */
export async function prepareCargoFixtureV1(storage:CargoPreparationStorageV1,root:string,args:readonly string[],cwd:string,environment:Record<string,string|undefined>):Promise<void>{await receiveScriptProcessInvocation(environment,original=>prepareCargoWorkspaceInvocation(storage,root,args,cwd,environment,nativeCallerPhaseV1(original,root,createIssuedResourceControlV1(original,"native:cargo-fixture-entry",0)).leaseControl));}

import {withRepositoryCargoPreparationV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🦀️ Runs the independent Cargo oracle only while authentic backing and original fixture custody remain live. */
export async function captureCargoFixtureV1(root:string,args:readonly string[]):Promise<string>{const environment=cargoPreparationFixtureEnvironmentV1(root);return receiveScriptProcessInvocation(environment,original=>withRepositoryCargoPreparationV1(findWorkspaceRoot(import.meta.dir),root,original,createIssuedResourceControlV1(original,"native:cargo-fixture-entry",0),environment,async(bounded,phase)=>{let output="";await runOwnedCommand("cargo",[...args],root,"cargo:fixture-oracle",phase.budget(),{...phase.options(bounded),stdout:"stderr",onLine:(line,stream)=>{if(stream==="stdout")output+=line+"\n";}});return output;}));}
