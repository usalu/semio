import {buildSync} from "esbuild";
import {copyFileSync,mkdirSync} from "node:fs";
import {dirname,join,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {getWorkspaceRoot} from "../../../🟦️.ts";

const sourceRoot=getWorkspaceRoot(),entry=fileURLToPath(new URL("../../🛠️preparation/📜️script.ts",import.meta.url)),api=fileURLToPath(new URL("../../🟦️.ts",import.meta.url));
const sources=[...Object.keys(buildSync({absWorkingDir:sourceRoot,entryPoints:[entry],bundle:true,write:false,metafile:true,platform:"node",packages:"external",format:"esm",logLevel:"silent"}).metafile!.inputs),... ["../../../📦️payload/🟨️.cjs","../../../🟦️bun/🟨️.cjs","../../🧬️schema/🛠️preparation/🔣️.json"].map(path=>relative(sourceRoot,fileURLToPath(new URL(path,import.meta.url))))];
/** 🧰️ Installs the actual preparation machinery inside an independently authored test repository. */
export function cargoPreparationRuntimeV1(root:string):string {
 for(const source of sources){const target=join(root,source);mkdirSync(dirname(target),{recursive:true});copyFileSync(join(sourceRoot,source),target);}return join(root,relative(sourceRoot,api));
}
/** 🧾️ Makes neutral test producers declare their current source before completing custody. */
export function observedCargoPreparationV1(root:string,source:string):string {
 const custody=fileURLToPath(new URL("../../🛠️preparation/🧾️custody/🟦️.ts",import.meta.url));
 return `import {observeCargoPreparationSourceV1,completeCargoPreparationObservationV1} from ${JSON.stringify(join(root,relative(sourceRoot,custody)))};observeCargoPreparationSourceV1(import.meta.path);\n${source}\ncompleteCargoPreparationObservationV1();\n`;
}
