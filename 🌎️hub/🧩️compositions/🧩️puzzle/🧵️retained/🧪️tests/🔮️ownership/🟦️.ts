import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import contract from "../🧫️fixtures/🔮️ownership/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const pointer = (value: unknown, path: string): unknown => path.slice(1).split("/").reduce((current, key) => current && typeof current === "object" ? (current as Record<string, unknown>)[key] : undefined, value);
const vectorIds = ["zero", "max", "maxPlusOne", "malformed", "staleGeneration", "wrongOperation", "abaGeneration", "cancelWirePage", "cancelWireByte", "cancelPreflight", "cancelWork", "cancelPublish", "faultWork", "retry", "close", "replay"];
const checkpointIds = ["checkpointEmpty", "checkpointSingle", "checkpointMax", "checkpointMaxPlusOne", "checkpointCorrupt", "checkpointInterruptedClose"];

/** 🧫️ Projects the same owned outputs that the retained native fixture interface observes. */
function evaluate(source: string) {
  const data = JSON.parse(source);
  const vectors = data.vectors as { id: string; fingerprint?: string }[];
  if (vectors.slice(0, vectorIds.length).some((vector) => typeof vector.fingerprint !== "string")) throw new Error("missing base fingerprint");
  return { owner: data.owner, artifactSchema: data.documentSchema, payloadSchema: data.payloadSchema, tools: data.toolIds, evidenceTools: data.evidenceToolIds, capacities: ["rawBytes", "decodedItems", "workItems", "outputBytes", "stepMicros", "semanticUnitsPerGrant"].map((key) => data.capacities[key]), locales: Object.keys(data.locales).sort(), vectorIds: vectors.map(({ id }) => id), fingerprints: vectors.slice(0, vectorIds.length).map(({ fingerprint }) => fingerprint) };
}

describe("Puzzle retained laws belong to their actual selected owner", () => {
  for (const artifact of contract.artifacts) {
    test(`${artifact.name} local native wiring law retains its original assertions`, () => {
      expect(existsSync(resolve(root, artifact.leaf))).toBe(true);
      const source = read(artifact.leaf);
      for (const assertion of artifact.positive) expect(source).toContain(`assert!(source.contains(${JSON.stringify(assertion)}));`);
      for (const assertion of artifact.negative) expect(source).toContain(`assert!(!source.contains(${JSON.stringify(assertion)}));`);
    });

    test(`${artifact.name} retains its actual fixture and full package route`, () => {
      expect(read(artifact.script)).not.toContain('"--features"');
      expect(read("🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/📜️script.ts")).not.toContain('"--features"');
      const actual = evaluate(read(artifact.fixture));
      expect(actual.vectorIds.slice(0, vectorIds.length)).toEqual(vectorIds);
      expect(checkpointIds.every((id) => actual.vectorIds.includes(id))).toBe(true);
      expect(actual.locales).toEqual(["de", "en"]);
      expect(actual.fingerprints).toEqual(vectorIds.map((id) => id === "maxPlusOne" ? "8193:0:0:0:0:0" : id === "malformed" ? "1:0:0:0:0:0" : ["staleGeneration", "wrongOperation", "abaGeneration"].includes(id) ? "1:1:1:0:0:0" : "0:0:0:0:0:0"));
    });

    test(`${artifact.name} every original hostile category changes real bytes and semantic oracle output`, () => {
      const source = read(artifact.fixture), baseline = evaluate(source), parsed = JSON.parse(source);
      for (const mutation of contract.mutations) {
        const mutated = source.replace(mutation.previous, mutation.current), value = JSON.parse(mutated);
        expect(mutated, mutation.id).not.toBe(source);
        expect(pointer(value, mutation.pointer), mutation.id).not.toEqual(pointer(parsed, mutation.pointer));
        if (mutation.id === "missing-fingerprint") {
          expect(() => evaluate(mutated)).toThrow("missing base fingerprint");
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
