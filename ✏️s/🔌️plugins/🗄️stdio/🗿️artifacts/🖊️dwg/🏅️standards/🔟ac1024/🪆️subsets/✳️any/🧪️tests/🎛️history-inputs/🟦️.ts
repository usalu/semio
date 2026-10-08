import { expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const standards = resolve(import.meta.dir, "../../../../../"), read = (path: string) => JSON.parse(readFileSync(path, "utf8"));

test("DWG summary titles are genuine committed inputs for each native version", () => {
  const artifact = read(resolve(standards, "🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔣️.json"));
  const expectedRef = artifact.$id + "#/$defs/DwgSummaryInfo";
  for (const [directory, version, example] of [["4️⃣ac1018", "AC1018", "🔢️bumps"], ["🔟ac1024", "AC1024", "✏️retitles"]]) {
    const owner = resolve(standards, directory, "🪆️subsets/✳️any");
    const before = read(resolve(owner, "🧫️fixtures/🧬️mutations/📸️set-snapshot", example!, "📸️snapshot/⬅️before/🔣️.json"));
    expect(before.version).toBe(version);
    expect(typeof before.summary.title).toBe("string");
    const after = applyPatch(structuredClone(before), [{ op: "replace", path: "/summary/title", value: "History title" }], true, true).newDocument;
    expect(after.version).toBe(version);
    expect(after.summary.title).toBe("History title");
    expect(applyPatch(structuredClone(after), [{ op: "replace", path: "/summary/title", value: before.summary.title }], true, true).newDocument).toEqual(before);
    const snapshot = read(resolve(owner, "🧬️schema/📸️snapshot/🔣️.json"));
    expect(snapshot.properties.summary.$ref).toBe(expectedRef);
    expect(artifact.$defs.DwgSummaryInfo.properties.title.type).toBe("string");
    const ajv = new Ajv({ strict: false }); ajv.addSchema(artifact);
    const validate = ajv.compile({ $ref: expectedRef });
    expect(validate(before.summary)).toBe(true);
    expect(validate(after.summary)).toBe(true);
    expect(validate({ ...before.summary, title: 7 })).toBe(false);
    const record = read(resolve(owner, "🧫️fixtures/🧬️history-edits/summary-title/🦠️mutation/🔣️.json"));
    expect(record).toEqual({ mutation: { mutation: "patchSnapshot", patch: { op: "set", path: "/summary/title", value: "History title" } }, before, after });
    console.log(`[DEBUG] DWG committed summary title version=${version} originalVersion=${after.version} unchangedOtherFields=true`);
  }
});
