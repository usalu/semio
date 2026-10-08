#!/usr/bin/env bun
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { MaterializeScript, SupportScript } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
const router = new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register("support", SupportScript).register("materialize", MaterializeScript);
if (import.meta.main && process.argv[2] === "test") {
  const selected = process.argv[3];
  const root = resolve(import.meta.dir, "../../../../../../.."), { runOwnedCommand } = await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
  if (selected === "catalog-composition") await runOwnedCommand(process.execPath, ["test", fileURLToPath(new URL("../../📇️registry/🧪️tests/🧩️composition/🟦️.ts", import.meta.url))], root, "catalog-composition", 300000, { env: process.env });
  else if (selected === "catalog-receivers") await runOwnedCommand(process.execPath, [resolve(root, "node_modules/vitest/vitest.mjs"), "run", "--config", fileURLToPath(new URL("../../📇️registry/🧪️tests/🧩️composition/🎚️config/🟦️.ts", import.meta.url))], root, "catalog-receivers", 300000, { env: { ...process.env, SEMIO_TEST_LEVEL: "full" } });
  else if (selected === "mounted-owner") await runOwnedCommand(process.execPath, ["test", fileURLToPath(new URL("../../🏇️mounted-owner/🧪️tests/🟦️.ts", import.meta.url))], root, "mounted-owner", 300000, { env: process.env });
  else throw new Error("Unknown plugin test selection");
} else if (import.meta.main) await runScriptMain(router);
