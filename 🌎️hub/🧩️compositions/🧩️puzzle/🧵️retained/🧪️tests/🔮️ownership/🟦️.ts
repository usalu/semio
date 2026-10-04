import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🔮️ownership/🔣️.json";
import schema from "../../🧬️schema/🔮️ownership/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (text: string) => createHash("sha256").update(text).digest("hex");
const pointer = (value: unknown, path: string): unknown => path.slice(1).split("/").reduce((current, key) => current && typeof current === "object" ? (current as Record<string, unknown>)[key] : undefined, value);
const vectorIds = ["zero", "max", "maxPlusOne", "malformed", "staleGeneration", "wrongOperation", "abaGeneration", "cancelWirePage", "cancelWireByte", "cancelPreflight", "cancelWork", "cancelPublish", "faultWork", "retry", "close", "replay"];
const checkpointIds = ["checkpointEmpty", "checkpointSingle", "checkpointMax", "checkpointMaxPlusOne", "checkpointCorrupt", "checkpointInterruptedClose"];
const baseVectorSchema = { type: "object", required: ["id", "fingerprint"], properties: { id: { type: "string" }, fingerprint: { type: "string" } } };
const validateVector = new Ajv().compile(baseVectorSchema);

/** 🧫️ Projects the same owned outputs that the retained native fixture interface observes. */
function evaluate(source: string) {
  const data = JSON.parse(source);
  const vectors = data.vectors as { id: string; fingerprint?: string }[];
  if (vectors.slice(0, vectorIds.length).some((vector) => typeof vector.fingerprint !== "string")) throw new Error("missing base fingerprint");
  return { owner: data.owner, artifactSchema: data.documentSchema, payloadSchema: data.payloadSchema, tools: data.toolIds, evidenceTools: data.evidenceToolIds, capacities: ["rawBytes", "decodedItems", "workItems", "outputBytes", "stepMicros", "semanticUnitsPerGrant"].map((key) => data.capacities[key]), locales: Object.keys(data.locales).sort(), vectorIds: vectors.map(({ id }) => id), fingerprints: vectors.slice(0, vectorIds.length).map(({ fingerprint }) => fingerprint) };
}

describe("Puzzle retained laws belong to their actual selected owner", () => {
  test("closed neutral ownership fixture agrees with independent Ajv validation", () => {
    expect(validateJsonSchemaSubset(schema, contract)).toEqual([]);
    expect(new Ajv().validate(schema, contract)).toBe(true);
    expect(validateJsonSchemaSubset(schema, { ...contract, foreignOwner: "artifact" }).length).toBeGreaterThan(0);
    expect(new Ajv().validate(schema, { ...contract, foreignOwner: "artifact" })).toBe(false);
    expect(digest(contract.sharedOriginal)).toBe(contract.sharedSha256);
  });

  test("actual shared law source has no concrete artifact dependency and preserves every other assertion", () => {
    const source = read(contract.shared);
    expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🗿️artifacts"))).toEqual([]);
    expect(source).toBe(contract.sharedRewrites.reduce((text, rewrite) => text.replace(rewrite.previous, rewrite.current), contract.sharedOriginal));
    expect(source).toContain("for tool in &baseline.tool_ids");
    expect(source).toContain("assert_ne!(actual.tool_ids, baseline.tool_ids)");
  });

  for (const artifact of contract.artifacts) {
    test(`${artifact.name} local native wiring law retains its original assertions`, () => {
      expect(existsSync(resolve(root, artifact.leaf))).toBe(true);
      const source = read(artifact.leaf), editor = read(artifact.editor);
      expect(editor.endsWith(contract.leafMount)).toBe(true);
      expect(digest(editor.slice(0, -contract.leafMount.length))).toBe(artifact.editorSha256);
      expect(inspectRustCompileReferences(source).map(({ path }) => resolve(root, dirname(artifact.leaf), path))).toEqual([resolve(root, artifact.editor)]);
      for (const assertion of artifact.positive) expect(source).toContain(`assert!(source.contains(${JSON.stringify(assertion)}));`);
      for (const assertion of artifact.negative) expect(source).toContain(`assert!(!source.contains(${JSON.stringify(assertion)}));`);
    });

    test(`${artifact.name} retains its actual fixture, native catalog, mounted cohorts and full package route`, () => {
      for (const [path, hash] of [[artifact.fixture, artifact.fixtureSha256], [artifact.root, artifact.rootSha256], [artifact.manifest, artifact.manifestSha256], [artifact.script, artifact.scriptSha256]]) expect(digest(read(path)), path).toBe(hash);
      expect(read(artifact.root)).toContain("fn retained_command_test_catalog()");
      expect(read(artifact.root)).toContain("../../🎮️commands/🧵️retained/🦀️.rs");
      expect(read("🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/📜️script.ts")).toContain('"--features", "component-app-assembly"');
      const actual = evaluate(read(artifact.fixture));
      expect(actual.vectorIds.slice(0, vectorIds.length)).toEqual(vectorIds);
      expect(checkpointIds.every((id) => actual.vectorIds.includes(id))).toBe(true);
      expect(actual.locales).toEqual(["de", "en"]);
      expect(actual.fingerprints).toEqual(vectorIds.map((id) => id === "maxPlusOne" ? "8193:0:0:0:0:0" : id === "malformed" ? "1:0:0:0:0:0" : ["staleGeneration", "wrongOperation", "abaGeneration"].includes(id) ? "1:1:1:0:0:0" : "0:0:0:0:0:0"));
    });

    test(`${artifact.name} every original hostile category changes real bytes and semantic oracle output`, () => {
      const source = read(artifact.fixture), baseline = evaluate(source), parsed = JSON.parse(source);
      expect(parsed.vectors.slice(0, vectorIds.length).every((vector: unknown) => validateVector(vector))).toBe(true);
      for (const mutation of contract.mutations) {
        const mutated = source.replace(mutation.previous, mutation.current), value = JSON.parse(mutated);
        expect(mutated, mutation.id).not.toBe(source);
        expect(pointer(value, mutation.pointer), mutation.id).not.toEqual(pointer(parsed, mutation.pointer));
        if (mutation.id === "missing-fingerprint") {
          expect(() => evaluate(mutated)).toThrow("missing base fingerprint");
          expect(validateVector(value.vectors[0])).toBe(false);
        } else expect(evaluate(mutated), mutation.id).not.toEqual(baseline);
      }
      for (const tool of parsed.toolIds as string[]) {
        const mutated = source.replace(JSON.stringify(tool), '"missingTool"'), value = JSON.parse(mutated);
        expect(mutated, tool).not.toBe(source);
        expect(value.toolIds, tool).not.toEqual(parsed.toolIds);
        expect(evaluate(mutated), tool).not.toEqual(baseline);
      }
    });
  }
});
