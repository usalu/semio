import { afterAll, describe, expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import ts from "typescript";
import { computeWorkspaces, getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const fixturePath = join(import.meta.dir, "../../🧫️fixtures/🧱️wasm-package-wrappers/🔣️.json");
const schemaPath = join(import.meta.dir, "../../🧬️schema/🧱️wasm-package-wrappers/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as any;
const suppliedArtifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!suppliedArtifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(suppliedArtifactRoot, { recursive: true });
const artifactRoot = mkdtempSync(join(suppliedArtifactRoot, "run-"));
afterAll(() => rmSync(artifactRoot, { recursive: true, force: true }));

function json(path: string): any {
  return JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
}

function installedPackage(row: any, root: string): string {
  const directory = join(root, "node_modules", ...row.packageName.split("/"));
  mkdirSync(join(directory, "pkg"), { recursive: true });
  writeFileSync(join(directory, "package.json"), readFileSync(join(repoRoot, row.ownerPath)));
  for (const name of [row.module, row.types, row.wasm, row.wasmTypes]) cpSync(join(repoRoot, dirname(row.payloadManifestPath), name), join(directory, "pkg", name));
  return directory;
}

describe("stable wasm package wrappers", () => {
  test("allocates and cleans only one owned child of the caller artifact root", () => {
    expect(dirname(artifactRoot)).toBe(suppliedArtifactRoot);
    expect(artifactRoot.startsWith(join(suppliedArtifactRoot, "run-"))).toBe(true);
  });

  test("validates the exact language-agnostic wrapper contract", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")));
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.wrappers.map((row: any) => row.id)).toEqual(["actor", "puzzle"]);
    expect(new Set(fixture.wrappers.map((row: any) => row.ownerPath)).size).toBe(2);
    for (const invalid of [{ ...fixture, schemaVersion: 2 }, { ...fixture, extra: true }, { ...fixture, wrappers: fixture.wrappers.slice(1) }]) expect(validate(invalid)).toBe(false);
  });

  test("binds each stable owner to its compiler payload and existing Nx producer", () => {
    for (const row of fixture.wrappers) {
      expect(existsSync(join(repoRoot, row.ownerPath)), row.ownerPath).toBe(true);
      expect(json(row.ownerPath), row.ownerPath).toEqual(row.manifest);
      const payload = json(row.payloadManifestPath);
      expect(payload.name, row.id).toBe(row.packageName);
      expect(payload.type, row.id).toBe("module");
      expect(payload.main, row.id).toBe(row.module);
      expect(payload.module, row.id).toBe(row.module);
      expect(payload.types, row.id).toBe(row.types);
      expect(payload.files, row.id).toEqual([row.wasm, row.module, row.types, row.wasmTypes]);
      const project = json(row.projectPath);
      expect(project.name, row.id).toBe(row.projectName);
      expect(project.targets[row.producerTarget].options.command, row.id).toBe(row.producerCommand);
      expect(project.targets[row.producerTarget].outputs, row.id).toEqual([row.producerOutput]);
    }
  });

  test("preserves exact wasm-bindgen companion identities and native module links", () => {
    for (const row of fixture.wrappers) {
      const payloadRoot = join(repoRoot, dirname(row.payloadManifestPath));
      const bytes = readFileSync(join(payloadRoot, row.wasm));
      expect([...bytes.subarray(0, 8)], row.id).toEqual([0, 97, 115, 109, 1, 0, 0, 0]);
      const moduleSource = readFileSync(join(payloadRoot, row.module), "utf8");
      expect(moduleSource, row.id).toContain(`new URL('${row.wasm}', import.meta.url)`);
      expect(readFileSync(join(payloadRoot, row.types), "utf8").length, row.id).toBeGreaterThan(0);
      expect(readFileSync(join(payloadRoot, row.wasmTypes), "utf8").length, row.id).toBeGreaterThan(0);
    }
  });

  test("resolves installed root, module and wasm exports through Bun, Node and TypeScript", () => {
    const root = join(artifactRoot, "installed-resolution");
    mkdirSync(root, { recursive: true });
    writeFileSync(join(root, "package.json"), '{"private":true,"type":"module"}\n');
    const expectedNode: string[] = [];
    for (const row of fixture.wrappers) {
      const packageRoot = installedPackage(row, root);
      const rootSpecifier = row.packageName;
      const moduleSpecifier = `${row.packageName}/${row.module}`;
      const wasmSpecifier = `${row.packageName}/${row.wasm}`;
      expect(Bun.resolveSync(rootSpecifier, root), row.id).toBe(join(packageRoot, "pkg", row.module));
      expect(Bun.resolveSync(moduleSpecifier, root), row.id).toBe(join(packageRoot, "pkg", row.module));
      expect(Bun.resolveSync(wasmSpecifier, root), row.id).toBe(join(packageRoot, "pkg", row.wasm));
      const resolved = ts.resolveModuleName(rootSpecifier, join(root, "🟦️.ts"), { module: ts.ModuleKind.NodeNext, moduleResolution: ts.ModuleResolutionKind.NodeNext }, ts.sys).resolvedModule;
      expect(resolved?.resolvedFileName, row.id).toBe(join(packageRoot, "pkg", row.types));
      expectedNode.push(join(packageRoot, "pkg", row.module), join(packageRoot, "pkg", row.module), join(packageRoot, "pkg", row.wasm));
    }
    const probe = join(root, "🟨️.mjs");
    const specifiers = fixture.wrappers.flatMap((row: any) => [row.packageName, `${row.packageName}/${row.module}`, `${row.packageName}/${row.wasm}`]);
    writeFileSync(probe, `for (const specifier of ${JSON.stringify(specifiers)}) console.log(new URL(import.meta.resolve(specifier)).pathname);\n`);
    const result = spawnSync(process.env.SEMIO_NODE_PATH ?? "node", [probe], { cwd: root, encoding: "utf8" });
    expect(result.status, result.stderr).toBe(0);
    expect(result.stdout.trim().split(/\r?\n/u).map(decodeURIComponent)).toEqual(expectedNode);
  });

  test("keeps generated child manifests as payloads of one stable workspace member", () => {
    for (const row of fixture.wrappers) {
      const root = join(artifactRoot, `membership-${row.id}`);
      const owner = join(root, "members", row.id);
      mkdirSync(join(owner, "pkg"), { recursive: true });
      writeFileSync(join(root, "package.json"), '{"name":"workspace","private":true}\n');
      writeFileSync(join(owner, "package.json"), JSON.stringify(row.manifest));
      writeFileSync(join(owner, "pkg", "package.json"), JSON.stringify({ name: row.packageName }));
      writeFileSync(join(owner, "pkg", row.module), "export {};\n");
      expect(computeWorkspaces(root), row.id).toEqual([relative(root, owner).replaceAll("\\", "/")]);
    }
  });

  test("discovers each current stable owner and suppresses its generated payload identity", () => {
    const current = computeWorkspaces(repoRoot);
    for (const row of fixture.wrappers) {
      expect(current, row.id).toContain(dirname(row.ownerPath));
      expect(current, row.id).not.toContain(dirname(row.payloadManifestPath));
    }
  }, 30_000);

  test("registers one exact cached Nx and launch route with complete inputs", async () => {
    const registration = fixture.registration;
    const project = json(registration.projectPath);
    const manifest = json(registration.packagePath);
    expect(project.namedInputs.wasmPackageWrapperSources).toEqual(fixture.inputs);
    expect(project.targets[registration.target]).toMatchObject({ cache: true, inputs: ["wasmPackageWrapperSources"], outputs: [], options: { command: "bun ./📜️script.ts test wasm-package-wrappers" } });
    expect(manifest.scripts[registration.packageScript]).toBe(`nx run @semio-tech/repo-lib:${registration.target}`);
    const jsonc = await import("jsonc-parser");
    for (const path of [registration.launchSeedPath, registration.launchPath]) {
      const launch = jsonc.parse(readFileSync(join(repoRoot, path), "utf8"));
      expect(launch.configurations.filter((row: any) => row.name === registration.launchName && row.command === registration.launchCommand)).toHaveLength(1);
    }
  });
});
