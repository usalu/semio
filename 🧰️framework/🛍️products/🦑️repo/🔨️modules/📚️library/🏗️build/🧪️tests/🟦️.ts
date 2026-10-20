import "./🎯️source-roster/🟦️.ts";
import "../⚙️operations/🧪️tests/🟦️.ts";
import "../🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts";
import "../📥️source/🛗️zero-touch/🧪️tests/🟦️.ts";
import "./🎟️phase/🟦️.ts";
import "../🧭️invocation/🧪️tests/🟦️.ts";
import "../📥️caller/🧪️tests/📥️birth/🟦️.ts";
import "../🧭️selection/👁️watch/🧪️tests/🟦️.ts";
import "../🕸️graph/📥️inference/🧪️tests/🟦️.ts";
import "../🕸️graph/📥️inference/🧪️tests/🔌️providers/🟦️.ts";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { build } from "esbuild";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { BuildScript } from "../📜️script.ts";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const root = findWorkspaceRoot(import.meta.dir);

test("workspace build entry loads without the audit monolith under the independent bundler", async () => {
  const compiled = await build({ absWorkingDir: root, entryPoints: [fixture.entry], bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  for (const input of Object.keys(compiled.metafile!.inputs)) for (const forbidden of fixture.forbiddenInputs) expect(forbidden.endsWith(".ts") ? input === forbidden : input.includes(forbidden), input).toBe(false);
  const project = JSON.parse(readFileSync(resolve(root, "📋️project.json"), "utf8"));
  for (const [name, args] of Object.entries(fixture.targets)) expect(project.targets[name].options.command).toBe(`bun ./${fixture.entry} ${args}`);
});

test("workspace build gate requires compiler intent and emits the completed graph mode", () => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "workspace-build-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
  const before = process.env.SEMIO_BUILD_MODE, messages: string[] = [], log = console.log;
  console.log = (...args: unknown[]) => { messages.push(args.join(" ")); };
  try {
    const script = new BuildScript(root, root, invocation);
    for (const mode of fixture.modes) {
      process.env.SEMIO_BUILD_MODE = mode;
      await script.run([]);
      expect(messages.at(-1)).toBe(`[build] Completed the ${mode} workspace build graph.`);
    }
    for (const mode of fixture.invalidModes) {
      if (mode === null) delete process.env.SEMIO_BUILD_MODE; else process.env.SEMIO_BUILD_MODE = mode;
      await expect(script.run([])).rejects.toThrow("prototype or release");
    }
    process.env.SEMIO_BUILD_MODE = "prototype";
    for (const args of fixture.invalidSelections) await expect(script.run(args)).rejects.toThrow();
  } finally {
    console.log = log;
    if (before === undefined) delete process.env.SEMIO_BUILD_MODE; else process.env.SEMIO_BUILD_MODE = before;
  }
}));

import "./🕸️graph/🌱️native-scope/🟦️.ts";

import "../../⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🚪️gate/🧪️tests/🟦️.ts";
