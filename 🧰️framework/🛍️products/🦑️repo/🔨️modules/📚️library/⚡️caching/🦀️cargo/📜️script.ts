#!/usr/bin/env bun
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { NativeScript } from "../📦️artifacts/📋️native-orchestration/🟦️.ts";

const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("native", NativeScript);
if (import.meta.main) await router.run(process.argv.slice(2));
