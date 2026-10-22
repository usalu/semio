import installBase from "../🧬️schema/📥️install.json" with {type:"json"};
import generationBase from "../../🧾️generation/🧬️schema/🔣️.json" with {type:"json"};
import {materializeSchemaV1,schemaViewV1} from "../../🔌️managers/🧩️materialize/🟦️.ts";
import {currentHostV1,selectPackageManagerV1} from "../../🔌️managers/📚️catalog/🟦️.ts";

const managerV1=selectPackageManagerV1("javascript"),viewV1=schemaViewV1(managerV1.descriptor);
let runtimeV1:string|undefined;
const withManagerV1=(schema:any):any=>{
 const protocol=schema.$defs.ManagerProtocol;
 Object.defineProperty(protocol,"manager",{value:managerV1,enumerable:false});
 Object.defineProperty(protocol.const,"runtime",{enumerable:false,get:()=>runtimeV1??=managerV1.runtimeIdentity(currentHostV1())});
 return schema;
};
/** 📥️ The install schema of the selected JavaScript manager: the neutral base with every manager value filled in from its descriptor, plus the live manager under `$defs.ManagerProtocol.manager` and its runtime identity under `.const.runtime`. */
export const javascriptInstallSchemaV1:any=withManagerV1(materializeSchemaV1(installBase,viewV1));
/** 🐑️ Clones an install schema together with its live manager, which a structured clone cannot carry. */
export const cloneJavascriptInstallSchemaV1=(schema:any):any=>withManagerV1(structuredClone(schema));
/** 🧾️ The generation schema of the selected JavaScript manager, materialized the same way (without the live manager, which only the install schema carries). */
export const javascriptGenerationSchemaV1:any=materializeSchemaV1(generationBase,viewV1);
