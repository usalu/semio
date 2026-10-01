#!/usr/bin/env bun
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { CargoRelayScript, NativeScript } from "../📦️artifacts/📋️native-orchestration/🟦️.ts";

const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("native", NativeScript).register("relay", CargoRelayScript);
if (import.meta.main) await router.run(process.argv.slice(2));
