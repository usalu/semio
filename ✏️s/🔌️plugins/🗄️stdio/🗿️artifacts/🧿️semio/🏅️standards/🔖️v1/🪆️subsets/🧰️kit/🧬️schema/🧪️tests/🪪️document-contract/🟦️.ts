import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import fg from "fast-glob";
import { applyPatch } from "fast-json-patch";
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));

function referenceIntegrity(value: any): boolean {
  const typeIds = new Set(value.types.map((entry: any) => entry.id));
  if (typeIds.size !== value.types.length || new Set(value.designs.map((entry: any) => entry.id)).size !== value.designs.length) return false;
  for (const design of value.designs) {
    const pieceIds = new Set(design.pieces.map((entry: any) => entry.id));
    if (pieceIds.size !== design.pieces.length || !design.pieces.every((piece: any) => typeIds.has(piece.typeId))) return false;
    if (!design.connections.every((connection: any) => pieceIds.has(connection.connectingPieceId) && pieceIds.has(connection.connectedPieceId))) return false;
  }
  return value.representations.every((entry: any) => typeIds.has(entry.role));
}

function childIdentity(value: any): boolean {
  const lists = [["objects", "object"], ["models", "model"]] as const;
  for (const [field, subset] of lists) for (const child of value[field] ?? []) {
    if (child.childId !== child.target.artifactId || child.target.dialect.artifactKind !== "s.stdio.semio" || child.target.dialect.standard !== "v1" || child.target.dialect.subset !== subset) return false;
  }
  const child = value.properties;
  return !child || child.childId === child.target.artifactId && child.target.dialect.artifactKind === "s.stdio.semio" && child.target.dialect.standard === "v1" && child.target.dialect.subset === "value";
}

/** 🧪️ Kit catalog records, child identities and shared links agree with independent validators. */
export function testSemioKitDocumentContract(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const key of ["x-semio-state", "x-semio-child", "x-semio-link"]) ajv.addKeyword(key);
  for (const path of [
    "../../../../../../../../../../../..//🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json",
    "../../../../../../../../../../../..//🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🔣️.json",
    "../../../../../../../../../../../..//🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json",
    "../../../../../../../../../../../..//🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json",
    "../../../../✉️base/🧬️schema/🪆️child/🔣️.json",
    "../../../../✉️base/🧬️schema/🧮️geometry/🔣️.json",
  ]) ajv.addSchema(read(path));
  const fixtures = read("./🧫️fixtures/🔣️.json");
  const artifactSchema = ajv.compile(read("../../🔣️.json"));
  const snapshotSchema = ajv.compile(read("../../📸️snapshot/🔣️.json"));
  const diffSchema = ajv.compile(read("../../🔺️diff/🔣️.json"));
  const parseArtifact = (artifact as Record<string, (input: unknown) => unknown>).parseSemioKitArtifact!;
  const parseSnapshot = (snapshot as Record<string, (input: unknown) => unknown>).parseSemioKitSnapshot!;
  const parseDiff = (diff as Record<string, (input: unknown) => unknown>).parseSemioKitDiff!;
  for (const entry of fixtures.snapshotCases) {
    const admitted = artifactSchema(entry.input) && snapshotSchema(entry.input) && childIdentity(entry.input) && referenceIntegrity(entry.input);
    assert.equal(admitted, entry.valid, "independent artifact/snapshot oracle");
    if (entry.valid) {
      assert.deepEqual(parseArtifact(entry.input), entry.input);
      assert.deepEqual(parseSnapshot(entry.input), entry.input);
    } else {
      assert.throws(() => parseArtifact(entry.input), JSON.stringify(entry.input));
      assert.throws(() => parseSnapshot(entry.input), JSON.stringify(entry.input));
    }
  }
  for (const entry of fixtures.diffCases) {
    assert.equal(diffSchema(entry.input) && childIdentity({ types: [], designs: [], objects: entry.input.objects?.values ?? [], models: entry.input.models?.values ?? [], properties: entry.input.properties, representations: [] }), entry.valid, "independent diff oracle");
    if (entry.valid) assert.deepEqual(parseDiff(entry.input), entry.input);
    else assert.throws(() => parseDiff(entry.input), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.patchCases) {
    const expected = applyPatch(structuredClone(entry.before), entry.patch, true).newDocument;
    assert.deepEqual(expected, entry.after, "independent patch oracle");
    assert.deepEqual(diff.applySemioKitDiff(parseArtifact(entry.before), parseDiff(entry.diff)), expected, "Kit parent edit");
  }

  const mutationRoot = fileURLToPath(new URL("../../🧬️mutations/", import.meta.url));
  for (const file of fg.sync("*/🧬️schema/🔣️.json", { cwd: mutationRoot, absolute: true })) ajv.addSchema(JSON.parse(readFileSync(file, "utf8")));
  const mutationSchema = ajv.compile(read("../../🧬️mutations/🔣️.json"));
  let snapshots = 0, diffs = 0, mutations = 0;
  const corpusRoot = fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations/", import.meta.url));
  for (const file of fg.sync("**/🔣️.json", { cwd: corpusRoot, absolute: true })) {
    const value = JSON.parse(readFileSync(file, "utf8"));
    if (file.includes("/📸️snapshot/")) {
      assert(snapshotSchema(value) && childIdentity(value) && referenceIntegrity(value), file + ": snapshot oracle");
      assert.deepEqual(parseSnapshot(value), value, file);
      snapshots++;
    } else if (file.includes("/🔺️diff/")) {
      assert(diffSchema(value), file + ": diff oracle");
      assert.deepEqual(parseDiff(value), value, file);
      diffs++;
    } else if (file.includes("/🦠️mutation/")) {
      assert(mutationSchema(value), file + ": mutation schema oracle");
      const payload = Object.values(value)[0] as Record<string, any>;
      if (payload.child_id && payload.target) assert.equal(payload.child_id, payload.target.artifactId, file + ": mutation child identity");
      mutations++;
    }
  }
  assert.equal(snapshots, 30, "every committed Kit snapshot");
  assert.equal(diffs, 15, "every committed Kit diff");
  assert.equal(mutations, 15, "every committed Kit mutation");
  console.log("[DEBUG] Stdio Kit exact contracts: " + fixtures.snapshotCases.length + " snapshot vectors, " + fixtures.diffCases.length + " diff vectors, " + snapshots + " committed snapshots, " + diffs + " committed diffs, " + mutations + " mutations");
}
