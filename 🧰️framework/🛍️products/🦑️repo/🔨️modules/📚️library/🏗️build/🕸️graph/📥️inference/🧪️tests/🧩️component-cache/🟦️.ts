/** 🧩️ The `component` target of every plugin and extension crate replays from the task cache: exact inputs, one store receipt as its output, serial, classed deterministic. The fixture in `./🔣️.json` is language-agnostic; the expectations come from the policy and receipt schema files read directly, and the strict independent validators (Ajv) admit the decision and the receipt target. */
import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import { dirname, join, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import cacheSchema from "../../../../🏃️execution/🧬️schema/💾️cache.json" with { type: "json" };
import receiptSchema from "../../../../🧾️store-receipt/🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };

const require = createRequire(import.meta.url);
const plugin = await import("../../🟨️.mjs");
await plugin.libraryBootstrap;
const api = plugin.cacheInternals;
const library = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
const policy = JSON.parse(await readFile(join(library, "⚡️caching/🔣️policy.json"), "utf8")), receipt = receiptSchema.definitions.Policy.const;
const ajv = new Ajv({ strict: true, allErrors: true });
const posixPath = (path: string) => path.split(sep).join(posix.sep);

test("a component crate replays from one store receipt over its exact inputs, one Cargo build at a time", async () => {
  const root = await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir()), "cc-")), toml = require("@iarna/toml");
  try {
    for (const row of corpus.crates) {
      const manifest = { package: { name: "crate-" + row.id, version: "0.0.0", metadata: { component: { package: "semio:" + row.id }, semio: { "component-kind": row.kind } } }, lib: { "crate-type": row.crateTypes } };
      await mkdir(join(root, row.root), { recursive: true });
      await writeFile(join(root, row.root, "Cargo.toml"), toml.stringify(manifest));
      if (row.expect === "refused") { await expect(api.componentBuildTargets(row.root, root), row.id).rejects.toThrow("cdylib-only component crate"); continue; }
      const targets = await api.componentBuildTargets(row.root, root);
      if (row.expect === null) { expect(targets, row.id).toEqual({}); continue; }
      expect(Object.keys(targets), row.id).toEqual([corpus.target]);
      const target = targets[corpus.target];
      expect(target.cache, row.id).toBe(true);
      expect(target.parallelism, row.id).toBe(false);
      expect(target.dependsOn, row.id).toEqual(corpus.dependsOn);
      expect(target.metadata.semio.execution, row.id).toEqual({ version: 1, kind: "orchestration" });
      expect([target.metadata.semio.cache.class, target.metadata.semio.cache.reason], row.id).toEqual([corpus.cache.class, corpus.cache.reason]);
      expect(ajv.validate(cacheSchema, target.metadata.semio.cache), JSON.stringify(ajv.errors)).toBe(true);
      expect(new RegExp(receiptSchema.definitions.Target.pattern, "u").test(corpus.target), row.id).toBe(true);
      expect(target.outputs, row.id).toEqual([`${receipt.projectRootToken}/${receipt.directory}/${corpus.target}${receipt.extension}`]);
      const strings = target.inputs.filter((input: unknown) => typeof input === "string") as string[], objects = target.inputs.filter((input: unknown) => typeof input === "object") as Record<string, unknown>[];
      expect(strings.filter((input) => !input.startsWith("{workspaceRoot}/")), row.id).toEqual(corpus.named);
      const files = new Set(strings.filter((input) => input.startsWith("{workspaceRoot}/")).map((input) => input.slice("{workspaceRoot}/".length)));
      for (const name of (row.expect as {descriptorOwner: string; descriptorFiles: string[]}).descriptorFiles) expect(files.has(`${(row.expect as {descriptorOwner: string}).descriptorOwner}/${name}`), `${row.id}: ${name}`).toBe(true);
      expect(files.has(posixPath(relative(root, join(library, corpus.binaryenManifest)))), `${row.id}: pinned Binaryen manifest`).toBe(true);
      for (const file of policy.toolchains.cargo.files) expect(files.has(file), `${row.id}: ${file}`).toBe(true);
      expect(objects.filter((input) => "env" in input).map((input) => input.env), row.id).toEqual(policy.toolchains.cargo.environment);
      expect(objects.filter((input) => "runtime" in input).map((input) => input.runtime).filter((command) => policy.toolchains.cargo.commands.includes(command)), row.id).toEqual(policy.toolchains.cargo.commands);
      expect(objects, row.id).toContainEqual({ externalDependencies: corpus.externalDependencies });
      expect(objects, row.id).toContainEqual({ dependentTasksOutputFiles: "**/*" });
      expect(target.options.command, row.id).toBe(`repo ${JSON.stringify("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts")} component --manifest ${JSON.stringify(posixPath(relative(root, join(root, row.root, "Cargo.toml"))))}`);
    }
  } finally { await rm(root, { recursive: true, force: true }); }
}, 30000);
