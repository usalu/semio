import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import micromatch from "micromatch";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import fixture from "./🔣️.json";

const projectRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
let workspace = projectRoot;
while (!existsSync(resolve(workspace, "📋️build-owners.json"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw new Error("Repository owner registry required");
  workspace = parent;
}
const read = (path: string): any => JSON.parse(readFileSync(resolve(workspace, path), "utf8"));
const project = read("✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📋️project.json");
const schema = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json");
const expand = (path: string): string => path.replace("{workspaceRoot}", workspace).replace("{projectRoot}", projectRoot);

/** 📦️ Portable declarations keep test modules executable and all consumed artifact bytes fresh. */
describe("Remodel TypeScript test declaration", () => {
  it("declares exact runtime entries under the owned source contract", () => {
    const semio = project.targets.test.metadata.semio;
    expect(semio.execution).toEqual(fixture.execution);
    expect(semio.sourceInputObservation).toBe(fixture.observation);
    expect(semio.runtimeSourceEntries).toEqual(fixture.runtimeEntries);
    expect(semio.closure.resources).toEqual(fixture.resourceEntries);
    const ajv = new Ajv({ strict: true });
    expect(ajv.compile(schema.$defs.RuntimeSourceEntries)(semio.runtimeSourceEntries)).toBe(true);
    expect(ajv.compile(schema.$defs.SourceResourceEntries)(semio.closure.resources)).toBe(true);
    for (const entry of [...semio.runtimeSourceEntries, ...semio.closure.resources]) {
      expect(expand(entry).length).toBeLessThanOrEqual(256);
      expect(statSync(expand(entry)).isFile()).toBe(true);
    }
  });
  it("hashes artifact code, schemas, vectors, example assets and native case evidence", () => {
    expect(project.targets.test.inputs).toContain("default");
    expect(project.namedInputs.default).toContain(fixture.artifactPattern);
    const pattern = fixture.artifactPattern.replace("{workspaceRoot}/", "");
    for (const file of fixture.evidence) {
      expect(existsSync(resolve(workspace, file)), file).toBe(true);
      expect(micromatch.isMatch(file, pattern), file).toBe(true);
    }
    for (const suffix of ["ts", "tsx", "json", "semio", "png", "absent", "rs"]) {
      expect(micromatch.isMatch("✏️s/🔌️plugins/📸️remodel/🗿️artifacts/example." + suffix, pattern)).toBe(true);
    }
    expect(micromatch.isMatch("🌎️hub/unrelated.json", pattern)).toBe(false);
  });
});
