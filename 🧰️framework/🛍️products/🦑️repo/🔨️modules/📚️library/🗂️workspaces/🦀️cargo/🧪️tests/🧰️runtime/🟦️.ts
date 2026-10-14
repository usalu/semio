import {createScriptProcessEnvelope,scriptProcessEnvironment} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
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
