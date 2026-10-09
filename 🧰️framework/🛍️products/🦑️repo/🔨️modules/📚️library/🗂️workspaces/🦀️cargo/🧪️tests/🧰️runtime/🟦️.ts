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
