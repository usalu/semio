#!/usr/bin/env bun
import { ScriptRouter, runBundleScriptMain } from "@semio-tech/repo-lib";
import { TestScript } from "./🧪️tests/🧪️resident-oracle/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
