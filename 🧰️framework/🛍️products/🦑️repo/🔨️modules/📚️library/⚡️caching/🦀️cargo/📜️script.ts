#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CargoRelayScript, NativeScript } from "../📦️artifacts/📋️native-orchestration/🟦️.ts";

import {RustUnitScript} from "./📦️units/🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("native", NativeScript).register("relay", CargoRelayScript).register("unit",RustUnitScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));
