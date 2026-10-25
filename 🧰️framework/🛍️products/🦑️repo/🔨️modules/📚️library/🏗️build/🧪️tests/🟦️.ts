import "../📥️inputs/🎯️selected/🦀️rust/🧪️tests/📌️publication/🟦️.ts";
import "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/📋️profiles/🟦️.ts";
import "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧪️tests/🟦️.ts";
import "../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📥️data/🟦️.ts";
import "../📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts";
import "../📥️source/🔏️closure/🧾️diagnostics/🧪️tests/🟦️.ts";
import "../🚀️bootstrap/🧪️tests/🟦️.ts";
import "../../../🎛️dashboard/🎛️control-policy/🧩️products/🧪️tests/🟦️.ts";
import "../📥️source/🔏️closure/🚪️command/🧪️tests/🟦️.ts";
import "../⌨️command/🧪️tests/🟦️.ts";
import "../../⚡️caching/🚀️bootstrap/📦️dependencies/⌨️commands/🧪️tests/🟦️.ts";
import "../📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts";
import "../📥️source/🔏️closure/🏗️compiler/🧪️tests/🟦️.ts";
import "../../🗂️workspaces/🟦️bun/🧪️tests/♻️refresh/🟦️.ts";
import "../📥️source/🔏️closure/🧪️tests/🔏️selection/🟦️.ts";
import "../../../🔗️graphql/🔤️types/🧪️tests/🔒️producer/🟦️.ts";
import "../../⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🔁️lifecycle/🪝️automatic-hooks/🟦️.ts";
import "../🎯️targets/🧪️tests/🟦️.ts";
import "../🕸️graph/🕰️generations/🧪️tests/📦️document-size/🟦️.ts";
import "../../../../../../🔨️modules/🌱️value/📦️packages/🟦️typescript/♻️retirement/🌀️support/🧪️tests/🟦️.ts";
import "../../🧪️testing/🐹️go-plan/💾️overlay/🧪️tests/🟦️.ts";
import {ticketScopeVectorLawV1} from "../../../🔗️graphql/🧪️tests/🌿️tickets/📁️scope/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "./../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1} from "@semio-tech/repo-test/storage";
import {testResourceLeaseAdmissionTimeoutV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧪️tests/🚪️admission/🟦️.ts";
import "../🕸️graph/🚪️admission/🧪️tests/🟦️.ts";
import "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests/🟦️.ts";
import "../📥️inputs/💾️storage/🧪️tests/🟦️.ts";
import "../📥️inputs/💾️storage/🧪️tests/💧️headroom/🟦️.ts";
import "../🕸️graph/📥️inference/🔎️owner-discovery/🧪️tests/🔁️observation/🟦️.ts";
import "../📥️inputs/💾️storage/🧪️tests/🔒️lifetime/🟦️.ts";
import "../📥️inputs/💾️storage/🧪️tests/📏️quota/🔎️observation/🟦️.ts";
import "../📥️inputs/💾️storage/🧪️tests/📒️ledger/🔎️observation/🟦️.ts";
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
import "../🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts";
import "../🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🟦️.ts";
import "../🧷️format-stamps/🧪️tests/🟦️.ts";
import "../📥️source/🔏️closure/🧪️tests/🧭️sites/🟦️.ts";
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
  for (const [name, args] of Object.entries(fixture.targets)) expect(project.targets[name].options.command).toBe(`repo ./${fixture.entry} ${args}`);
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

test("actual lease admission refusal retains bounded context and genuine continuation",async()=>withBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(root,original,createIssuedResourceControlV1(original,"test:artifact-entry",0),async()=>testResourceLeaseAdmissionTimeoutV1(repositoryTestArtifactRootV1(root),scriptCompilerAdmissionControlV1(original,"test:resource-leases",30000)))));

test("rooted ticket lifecycle neutral vectors preserve independent repository projections",()=>ticketScopeVectorLawV1());

import "../../⚡️caching/🧹️pruning/🌐️workspace/🧪️tests/🔗️imports/🟦️.ts";
import "../🚀️bootstrap/📦️parsers/🧪️tests/📥️resolution/🟦️.ts";
