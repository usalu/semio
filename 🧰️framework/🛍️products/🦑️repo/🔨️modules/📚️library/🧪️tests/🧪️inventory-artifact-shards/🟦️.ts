import { expect, test } from "bun:test";
import { createHash, webcrypto } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import stringify from "fast-json-stable-stringify";
import fastGlob from "fast-glob";
import { parse as parseJsonc } from "jsonc-parser";
import { inventoryTaxonomy } from "../../🧹️normalization/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const ticketRoot = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️🗿️inventory-artifact-shards/🔣️.json"), "utf8"));
type Violation = { path: string; code: string; severity: "error" | "warning"; message: string };
const byteCompare = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));
const sha = (value: string | Uint8Array) => createHash("sha256").update(value).digest("hex");
const compareViolation = (left: Violation, right: Violation) => vector.contract.identityFields.reduce((result: number, key: keyof Violation) => result || byteCompare(left[key], right[key]), 0);

function inventory(rows: Violation[], topLevel = rows) {
  const paths = [...new Set(rows.map((row) => row.path))].sort(byteCompare);
  return {
    schemaVersion: 1,
    taxonomySchemaVersion: 7,
    repoRoot: "/fixture",
    taxonomyPath: "/fixture/🔣️taxonomy.json",
    sourceTreeDigest: "a".repeat(64),
    inventoryDigest: "b".repeat(64),
    pathExclusions: ["compose", "temp/compose"],
    activePathExclusions: [],
    captureAuthority: { baseline: "9f449b10659b95148c8bcb3f91ce583bf7446973", bytePreimage: "AAECA/7/" },
    entries: paths.map((path) => ({ sourcePath: path, normalizedPath: path, ownerId: "fixture", areaId: "fixture", nodeKind: "file", fileKind: "plain-text", semanticStem: null, contentHash: sha(path), mode: 0o640, size: 37, referencesIn: [], referencesOut: [], violations: rows.filter((row) => row.path === path).sort(compareViolation) })),
    violations: structuredClone(topLevel),
  };
}

test("canonical shard violation closure accepts every language-neutral order without losing metadata or entries", async () => {
  const { buildTaxonomyInventoryArtifactShards, taxonomyInventoryCanonicalChunks, taxonomyInventoryIncrementalCanonicalDigest, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const validate = new Ajv().compile(vector.violationSchema);
  for (const row of vector.cases) {
    const expected = row.publishedOrder.map((index: number) => row.rows[index]);
    expect([...row.rows].sort(compareViolation)).toEqual(expected);
    expect(row.rows.every((item: unknown) => validate(item))).toBe(true);
    let previous: string | undefined;
    for (const order of [row.rows, [...row.rows].reverse()]) {
      const source = inventory(row.rows, order), before = JSON.stringify(source);
      const built = buildTaxonomyInventoryArtifactShards(source);
      const restored = validateTaxonomyInventoryArtifactShards(built).inventory;
      expect(restored.violations).toEqual(expected);
      expect(restored.entries).toEqual(source.entries);
      expect(built.manifest.inventoryMetadata).toEqual(Object.fromEntries(Object.entries(source).filter(([key]) => key !== "entries" && key !== "violations")));
      expect(JSON.stringify(source)).toBe(before);
      expect(built.manifest.violationsDigest).toBe(sha(stringify(expected)));
      expect(built.manifest.inventoryCanonicalDigest).toBe(sha(stringify(restored)));
      expect([...taxonomyInventoryCanonicalChunks(restored)].join("")).toBe(stringify(restored));
      expect(taxonomyInventoryIncrementalCanonicalDigest(restored)).toBe(Buffer.from(await webcrypto.subtle.digest("SHA-256", Buffer.from(stringify(restored)))).toString("hex"));
      for (const shard of built.shards) expect(shard.content).toBe(`${stringify(JSON.parse(shard.content))}\n`);
      if (previous) expect(built.manifestContent).toBe(previous);
      previous = built.manifestContent;
    }
  }
});

test("canonical shard closure rejects changed missing extra and duplicate top-level records", async () => {
  const { buildTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const rows = vector.cases[0].publishedOrder.map((index: number) => vector.cases[0].rows[index]);
  for (const kind of vector.rejections) {
    const source = inventory(rows), changed = structuredClone(source.violations);
    if (kind === "missing") changed.pop();
    else if (kind === "extra") changed.push({ ...rows[0], path: "fixture/extra" });
    else if (kind === "duplicate") changed.push({ ...rows[0] });
    else if (kind === "changed-severity") changed[0].severity = "warning";
    else changed[0][kind.slice("changed-".length) as Exclude<keyof Violation, "severity">] += "-drift";
    source.violations = changed;
    expect(() => buildTaxonomyInventoryArtifactShards(source), kind).toThrow("Inventory top-level violations must equal");
  }
  const source = inventory(rows);
  source.entries.push({ ...source.entries[0], sourcePath: "fixture/shared-owner" });
  expect(validateTaxonomyInventoryArtifactShards(buildTaxonomyInventoryArtifactShards(source)).violationCount).toBe(rows.length);
});

test("shard descriptor order is explicit and cannot be hidden by co-reordering payloads", async () => {
  const { buildTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const source = inventory(vector.cases[0].rows);
  source.entries.forEach((entry, index) => { entry.ownerId = vector.orderedOwners[index]; });
  const built = buildTaxonomyInventoryArtifactShards(source);
  expect(built.manifest.shards.map((row) => row.ownerId)).toEqual(vector.orderedOwners);
  const reordered = { ...built, manifest: { ...built.manifest, shards: [...built.manifest.shards].reverse() }, shards: [...built.shards].reverse() };
  expect(() => validateTaxonomyInventoryArtifactShards(reordered)).toThrow();
  reordered.manifest.shardLedgerDigest = sha(stringify(reordered.manifest.shards));
  reordered.manifestContent = `${stringify(reordered.manifest)}\n`;
  expect(() => validateTaxonomyInventoryArtifactShards(reordered)).toThrow("not in byte-sorted owner/part order");
  expect(JSON.parse(built.manifestContent).shards.map((row: { ownerId: string }) => row.ownerId)).toEqual(vector.orderedOwners);
  expect(built.manifest.shardLedgerDigest).toBe(sha(stringify(built.manifest.shards)));
});

test("serialized shard reconstruction preserves nested metadata and entry array order", async () => {
  const { buildTaxonomyInventoryArtifactShards, taxonomyInventoryCanonicalChunks, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const source = { ...inventory(vector.cases[0].rows), coordinateHistory: structuredClone(vector.orderedCoordinateHistory) };
  const entries = source.entries.map((entry) => ({ ...entry, coordinateHistory: structuredClone(vector.orderedCoordinateHistory) }));
  const input = { ...source, entries }, before = JSON.stringify(input);
  const built = buildTaxonomyInventoryArtifactShards(input), manifest = JSON.parse(built.manifestContent);
  const restored = validateTaxonomyInventoryArtifactShards({ ...built, manifest }).inventory;
  expect(restored.coordinateHistory).toEqual(vector.orderedCoordinateHistory);
  expect(restored.entries).toEqual(entries);
  expect(built.manifest.inventoryCanonicalDigest).toBe(sha(stringify(restored)));
  expect([...taxonomyInventoryCanonicalChunks(restored)].join("")).toBe(stringify(restored));
  expect(JSON.stringify(input)).toBe(before);
});

test("canonical artifact validation rejects every language-neutral preimage corruption", async () => {
  const { buildTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const source = inventory(vector.cases[0].rows), built = buildTaxonomyInventoryArtifactShards(source);
  for (const kind of vector.artifactRejections) {
    if (kind === "duplicate-source") {
      expect(() => buildTaxonomyInventoryArtifactShards({ ...source, entries: [...source.entries, source.entries[0]] }), kind).toThrow("Duplicate inventory sourcePath");
      continue;
    }
    const changed = structuredClone(built) as { manifest: Record<string, any>; manifestContent: string; shards: { descriptor: Record<string, any>; content: string }[] };
    let available = changed.shards.map((shard) => shard.descriptor.path);
    if (kind === "missing-payload") available = [];
    else if (kind === "extra-payload") available.push("📊️shards/extra/🔣️.json");
    else if (kind === "changed-entry") changed.shards[0].content = changed.shards[0].content.replace('"size":37', '"size":38');
    else if (kind === "changed-metadata") changed.manifest.inventoryMetadata.captureAuthority.bytePreimage = "changed";
    else if (kind === "duplicate-descriptor") { changed.manifest.shards.push(structuredClone(changed.manifest.shards[0])); changed.shards.push(structuredClone(changed.shards[0])); }
    else if (kind === "wrong-count") changed.manifest.entryCount += 1;
    else if (kind === "wrong-ledger") changed.manifest.shardLedgerDigest = "0".repeat(64);
    changed.manifestContent = `${stringify(changed.manifest)}${kind === "noncanonical-manifest" ? " " : ""}\n`;
    expect(() => validateTaxonomyInventoryArtifactShards(changed as typeof built, available), kind).toThrow();
  }
});

test("the real inventory producer publishes locale-different violation order losslessly", async () => {
  const { buildTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const root = mkdtempSync(join(ticketRoot, "🧪️inventory-producer-order-"));
  const put = (path: string, bytes: string | Buffer) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), bytes); };
  put(`${library}/🔣️taxonomy.json`, readFileSync(join(repoRoot, library, "🔣️taxonomy.json")));
  put("🧪️fixture/a/source.ts", "export const a = 1;\n");
  put("🧪️fixture/B/source.ts", "export const b = 2;\n");
  const initialized = Bun.spawnSync(["git", "init", "-q"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  expect(initialized.exitCode, initialized.stderr.toString()).toBe(0);
  const source = inventoryTaxonomy({ repoRoot: root, scope: "🧪️fixture", workers: 1 });
  expect(source.violations.length).toBeGreaterThan(0);
  expect(source.violations).not.toEqual([...source.violations].sort(compareViolation));
  const before = JSON.stringify(source), built = buildTaxonomyInventoryArtifactShards(source);
  const restored = validateTaxonomyInventoryArtifactShards(built).inventory;
  expect(restored.entries).toEqual([...source.entries].sort((left, right) => byteCompare(left.sourcePath, right.sourcePath)));
  expect(restored.violations).toEqual([...source.violations].sort(compareViolation));
  expect(built.manifest.inventoryCanonicalDigest).toBe(sha(stringify(restored)));
  expect(restored.sourceTreeDigest).toBe(source.sourceTreeDigest);
  expect(restored.inventoryDigest).toBe(source.inventoryDigest);
  expect(JSON.stringify(source)).toBe(before);
  console.log("[DEBUG] Real inventory violation-order publication verified", source.entries.length, source.violations.length);
});

test("publication verifies canonical violations and rejects inconsistent replacement before writing", async () => {
  const { publishTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards } = await import("../../../../../../../📜️script.ts");
  const root = mkdtempSync(join(ticketRoot, "🧪️inventory-order-publication-")), dataRoot = join(root, "📊️inventory");
  const source = inventory(vector.cases[0].rows), inputPath = join(root, "../🧪️🗿️inventory-artifact-shards/🔣️.json");
  writeFileSync(inputPath, JSON.stringify(source));
  const manifest = publishTaxonomyInventoryArtifactShards(dataRoot, source);
  const path = join(dataRoot, "📊️shards/🔣️.json"), before = readFileSync(path);
  const available = (await fastGlob("📊️shards/🔖️*/🔣️.json", { cwd: dataRoot, onlyFiles: true, followSymbolicLinks: false })).sort(byteCompare);
  expect(available).toEqual(manifest.shards.map((row) => row.path).sort(byteCompare));
  const built = { manifest, manifestContent: before.toString(), shards: manifest.shards.map((descriptor) => ({ descriptor, content: readFileSync(join(dataRoot, descriptor.path), "utf8") })) };
  expect(validateTaxonomyInventoryArtifactShards(built, available).violationCount).toBe(2);
  expect(before.toString()).toBe(`${stringify(manifest)}\n`);
  const bad = structuredClone(source);
  bad.violations[0].message += "-drift";
  expect(() => publishTaxonomyInventoryArtifactShards(dataRoot, bad)).toThrow("Inventory top-level violations must equal");
  expect(readFileSync(path)).toEqual(before);
  expect(readdirSync(join(dataRoot, "📊️shards")).some((name) => name.endsWith(".staging"))).toBe(false);
  expect(readFileSync(inputPath, "utf8")).toBe(JSON.stringify(source));
  rmSync(join(dataRoot, "📊️shards"), { recursive: true });
  expect(existsSync(inputPath)).toBe(true);
});

test("inventory shard consistency gate is registered through Nx and both launch catalogs", () => {
  const project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[vector.execution.target]?.options.command).toBe(vector.execution.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const entries = parseJsonc(readFileSync(join(repoRoot, path), "utf8")).configurations.filter((row: { name: string }) => row.name === vector.execution.launchName);
    expect(entries).toHaveLength(1);
    expect(entries[0].command).toBe(vector.execution.launchCommand);
    expect(entries[0].presentation).toEqual({ group: vector.execution.group, order: vector.execution.order });
  }
});
