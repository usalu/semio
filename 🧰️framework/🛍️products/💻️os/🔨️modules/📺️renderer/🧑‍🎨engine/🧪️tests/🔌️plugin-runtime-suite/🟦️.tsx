/** 🔌️ Runs every PluginRuntime topic suite against the PluginRuntime module's own test dependencies: the module exports its dependencies and holds no test hook. */
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import * as vitest from "vitest";
import { pluginRuntimeTestDependenciesV1 } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { registerPluginRuntimeTests } from "../🔌️plugin-runtime/🟦️.tsx";

await registerPluginRuntimeTests(vitest, pluginRuntimeTestDependenciesV1(), { url: pathToFileURL(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx")).href });
