import Ajv from "ajv";
import { sha256 } from "@noble/hashes/sha2.js";
import ts from "typescript";
import { createHash } from "node:crypto";
import { open } from "node:fs/promises";
import { mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { expect, it } from "vitest";
import schema from "../../../../🛂️proctor/🏗️bootstrap/🧬️schema/🔣️.json" with { type: "json" };
import fixtures from "../../../../🛂️proctor/🏗️bootstrap/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { buildProctor, proctorBuildMode, proctorBuildPlan, proctorSourceRoots, TEACHING_WORKSPACE, type ProctorArtifact, type ProctorExecutable } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";
import { pinExecutableArtifact } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { withResourceLeases, checkResourceLeaseControl } from "../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope, createIssuedResourceControlV1, createRequestedIssuedResourceControlV1, requireIssuedScriptInvocationV1, requireIssuedResourceControlInvocationV1, requireIssuedResourceControlRefinementV1, scriptInvocationEnvironment, withRequestedIssuedResourceControlV1, type ScriptInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment } from "../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { requireNativeStoragePathV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const validate = new Ajv().compile(schema);
const validateMode = new Ajv().compile(schema.definitions.ModeSelection);
for (const row of fixtures.modes) it(`preserves the original native build intent ${row.id}`, () => {
  expect(validateMode(row.request)).toBe(row.mode !== null);
  if (row.mode === null) expect(() => proctorBuildMode(row.request)).toThrow();
  else expect(proctorBuildMode(row.request)).toBe(row.mode);
});
for (const row of fixtures.cases) it(`admits the portable proctor build intent ${row.id}`, () => {
  expect(validate(row.request)).toBe(row.arguments !== null);
  if (row.arguments === null) expect(() => proctorBuildPlan(row.request)).toThrow();
  else {
    const expected = ["--package", "teaching-proctor", "--bin", "proctor", "--profile", row.request.mode];
    expect(proctorBuildPlan(row.request).arguments).toEqual(expected);
    expect(proctorBuildPlan(row.request).arguments).toEqual(row.arguments);
  }
});

for (const row of fixtures.authority) it(`refuses ${row.id} authority before opening compiler storage or launching Cargo`, async () => {
  await expect(buildProctor(resolve("."), row.invocation as never)).rejects.toThrow();
});

it("follows the prepared build graph's local owners once, excluding dev dependencies and nested roots", () => {
  const root = resolve("."), metadata = { ...fixtures.sources, packages: fixtures.sources.packages.map(row => ({ ...row, manifest_path: resolve(root, row.manifest_path) })) };
  const owners = proctorSourceRoots(metadata), expected = fixtures.sources.owners.map(owner => resolve(root, owner)).sort();
  const oracle = new Ajv().compile({ type: "array", uniqueItems: true, minItems: expected.length, maxItems: expected.length, items: { enum: expected } });
  expect(oracle(owners)).toBe(true);
  expect(owners).toEqual(expected);
});

it("publishes identical rebuild bytes in a new generation while the old immutable reader remains held", async () => {
  const root = mkdtempSync(join(realpathSync(tmpdir()), "course-pin-")), source = join(root, "proctor"), directory = join(root, "leases");
  writeFileSync(source, fixtures.runtime.bytes);
  const digest = Buffer.from(sha256(Buffer.from(fixtures.runtime.bytes))).toString("hex");
  try {
    await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "proctor:pinning-law", maximumElapsedMilliseconds: 5000 }, {}, Date.now()), async invocation => {
      const control = createIssuedResourceControlV1(invocation, "proctor:pinning-law", 0), signal = control.signal;
      const old = await pinExecutableArtifact(source, join(root, fixtures.runtime.generations[0]!), "proctor:test", { leaseDirectory: directory, signal, control });
      await withResourceLeases({ directory, signal, control, resources: [{ resource: "artifact:" + dirname(old), mode: "shared" }] }, async () => {
        const next = await pinExecutableArtifact(source, join(root, fixtures.runtime.generations[1]!), "proctor:test", { leaseDirectory: directory, signal, control });
        expect(old).not.toBe(next);
        expect(dirname(old).endsWith(digest)).toBe(true);
        expect(dirname(next).endsWith(digest)).toBe(true);
        expect(readFileSync(old)).toEqual(readFileSync(next));
        expect(readFileSync(next, "utf8")).toBe(fixtures.runtime.bytes);
        console.log("[DEBUG] Identical proctor rebuild retained its old reader through unique-generation publication");
      });
    });
  } finally { rmSync(root, { recursive: true, force: true }); }
});

it("carries the current compiler phase through capture and drains children and byte exports before closing storage", async () => {
  const root = mkdtempSync(join(realpathSync(tmpdir()), "course-build-")), source = join(root, process.platform === "win32" ? "proctor.exe" : "proctor"), phases: string[] = [];
  writeFileSync(source, fixtures.runtime.bytes);
  const document = ts.createSourceFile("bootstrap.ts", readFileSync(new URL("../../../../🛂️proctor/🏗️bootstrap/🟦️.ts", import.meta.url), "utf8"), ts.ScriptTarget.Latest, true);
  expect(ts.transpileModule(document.text, { reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).diagnostics).toEqual([]);
  const declaration = document.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "buildProctor");
  if (!declaration) throw Error("Current proctor compiler bridge required");
  const code = ts.transpileModule(declaration.getText(document).replace("export async function", "async function"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  let live = false, stopped = 0, retired = false;
  const storage = { recheck: async () => { expect(live).toBe(true); } };
  try {
    await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "proctor:lifecycle-law", maximumElapsedMilliseconds: 10000 }, {}, Date.now()), async invocation => {
      let storePhase: ReturnType<typeof createIssuedResourceControlV1>;
      const bindings = {
        requireIssuedScriptInvocationV1, proctorBuildMode, proctorBuildPlan, resolve, join, basename, dirname, TEACHING_WORKSPACE, scriptCompilerAdmissionControlV1, scriptInvocationEnvironment, buildModeEnvironment, requireNativeStoragePathV1, checkResourceLeaseControl, withResourceLeases, open, createHash,
        readNativeCallerAuthorityV1: () => ({ original: { buildMode: fixtures.lifecycle.mode }, transport: { maximumBytes: 65536 } }),
        repositoryCargoNetworkV1: (actual: ScriptInvocation) => { expect(actual).toBe(invocation); return { offline: true }; },
        repositoryCargoArtifactBuildPolicyV1: () => ({ leaseDirectory: join(root, "l") }),
        withRepositoryCompilerStoreV1: (_root: string, mode: string, control: typeof storePhase, _environment: unknown, body: Function) => withRequestedIssuedResourceControlV1(control, 0, async actual => {
          expect(mode).toBe(fixtures.lifecycle.mode); requireIssuedResourceControlInvocationV1(actual, invocation); requireIssuedResourceControlRefinementV1(control, actual); storePhase = actual; phases.push("store"); live = true;
          try { return await body({ root, buildDirectory: join(root, "b"), captureDirectory: join(root, "c"), environment: {}, storage, sourceBoundary: {} }, actual); }
          finally { live = false; retired = true; }
        }),
        randomUUID: () => "g", repositoryCargoPreparationStorageV1: () => ({}),
        cargoWorkspacePreparationInvocationV1: () => ({ command: "prepare", args: [], cwd: root, environment: {} }),
        captureOwnedCommandV1: async (_command: string, _args: unknown, _cwd: string, options: { control: typeof storePhase }) => { expect(options.control).toBe(storePhase); phases.push("preparation"); },
        proctorSourceDirectories: async () => { phases.push("sources"); return [root]; },
        CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1: 65536,
        repositoryCargoCompilerInputsV1: (options: { invocation: ScriptInvocation; control: typeof storePhase }) => { expect(options.invocation).toBe(invocation); expect(options.control).toBe(storePhase); return () => undefined; },
        buildCargoArtifacts: async (manifest: string, args: string[], _policy: unknown, options: { control: typeof storePhase; consume: Function }) => {
          expect(manifest).toBe(join(root, TEACHING_WORKSPACE, "🛂️proctor", "📦️packages", "🦀️rust", "Cargo.toml")); expect(args).toEqual(fixtures.lifecycle.arguments); expect(options.control).toBe(storePhase); phases.push("compiler"); await options.consume({});
        },
        withCapturedCargoArtifactsTaskV1: (_capture: unknown, body: Function) => { phases.push("capture"); return body({ files: new Map([[basename(source), source]]), control: createRequestedIssuedResourceControlV1(storePhase, 0), recheck: storage.recheck }); },
        pinExecutableArtifact: async (...args: Parameters<typeof pinExecutableArtifact>) => { phases.push("pin"); return pinExecutableArtifact(...args); },
        lstatSync: (await import("node:fs")).lstatSync, rmSync,
        launchProctorProcess: () => {
          let finish!: (code: number) => void;
          return { exited: new Promise<number>(accept => { finish = accept; }), stop: async () => { stopped++; finish(0); return 0; } };
        },
      };
      const build = new Function(...Object.keys(bindings), code + ";return buildProctor;")(...Object.values(bindings)) as typeof buildProctor;
      const artifact: ProctorArtifact = await build(root, invocation, {}, undefined, "release");
      expect(phases).toEqual(fixtures.lifecycle.phases);
      artifact.launch([]);
      let finish!: () => void, exported: ProctorExecutable | undefined, chunks = 0;
      const hold = new Promise<void>(accept => { finish = accept; }), consumer = artifact.withExecutable(async executable => {
        exported = executable;
        const bytes: Uint8Array[] = [];
        for await (const chunk of executable.chunks()) bytes.push(chunk);
        expect(Buffer.concat(bytes).toString()).toBe(fixtures.runtime.bytes); chunks++;
        await hold;
      });
      while (!chunks) await new Promise<void>(accept => setImmediate(accept));
      const closing = artifact.close();
      expect(live).toBe(true); expect(retired).toBe(false);
      expect(() => artifact.launch([])).toThrow("closing");
      finish(); await consumer; await closing;
      expect(stopped).toBe(fixtures.lifecycle.children); expect(chunks).toBe(fixtures.lifecycle.exports); expect(retired).toBe(true);
      await expect(exported!.recheck()).rejects.toThrow("retired");
      await expect(artifact.withExecutable(async () => undefined)).rejects.toThrow("closing");
      expect(new Ajv().compile({ const: fixtures.lifecycle.phases })(phases)).toBe(true);
      console.log("[DEBUG] Current proctor compiler phase, retained capture, child drain and byte export retirement verified");
    });
  } finally { rmSync(root, { recursive: true, force: true }); }
});
