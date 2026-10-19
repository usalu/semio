import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import base from "@semio-tech/framework-os-mcp/testing/config";
export default {...base,test:{...base.test,include:[resolve(dirname(fileURLToPath(import.meta.url)),"../../🧵️gis-inference-bridge/🟦️.ts")],includeSource:[]}};
