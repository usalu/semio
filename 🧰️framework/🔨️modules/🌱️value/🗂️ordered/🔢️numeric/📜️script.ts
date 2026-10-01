#!/usr/bin/env bun
import { ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { TestScript } from "./🧪️tests/🧪️numeric-index-oracle/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });
