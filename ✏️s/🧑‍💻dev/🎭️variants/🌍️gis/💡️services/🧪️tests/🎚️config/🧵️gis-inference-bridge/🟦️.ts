import {mergeConfig} from "vitest/config";
import {resolve} from "node:path";
import base from "@semio-tech/framework-os-mcp/testing/config";
export default mergeConfig(base,{test:{include:[resolve(import.meta.dir,"../../🧵️gis-inference-bridge/🟦️.ts")],includeSource:[]}});
