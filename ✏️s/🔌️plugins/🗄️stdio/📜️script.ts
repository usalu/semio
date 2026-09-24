#!/usr/bin/env bun
/** 🗄️ Stdio plugin router: artifact package contract and graph proofs. */
import { ScriptRouter, runBundleScriptMain } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { StdioArtifactPackageContractScript, StdioArtifactPackageGraphScript } from "./🗿️artifacts/🏃️commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir).register("package-contract", StdioArtifactPackageContractScript).register("package-graph", StdioArtifactPackageGraphScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "package-contract" });
