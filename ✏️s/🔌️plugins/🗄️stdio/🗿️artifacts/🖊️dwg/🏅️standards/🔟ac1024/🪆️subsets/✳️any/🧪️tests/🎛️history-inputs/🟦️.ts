import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const standards = resolve(import.meta.dir, "../../../../../"), read = (path: string) => JSON.parse(readFileSync(path, "utf8"));

test("DWG version-info edits are genuine committed inputs for each native version", () => {
  const leaf = read(resolve(standards, "🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-version-info/🧬️schema/🔣️.json"));
  const validate = new Ajv({ strict: false }).compile(leaf);
  for (const [directory, version, codepage] of [["4️⃣ac1018", "AC1018", 30], ["🔟ac1024", "AC1024", 28]] as const) {
    const owner = resolve(standards, directory, "🪆️subsets/✳️any");
    const { before, after, mutation } = read(resolve(owner, "🧫️fixtures/🧬️history-edits/version-info/🦠️mutation/🔣️.json"));
    expect(before.version).toBe(version);
    expect(mutation).toEqual({ mutation: "setVersionInfo", version: "AC1024", maintenanceVersion: before.maintenanceVersion, codepage });
    expect(validate(mutation)).toBe(true);
    expect(validate({ ...mutation, version: "ACX" })).toBe(false);
    expect(after).toEqual({ ...before, version: "AC1024", codepage });
    console.log(`[DEBUG] DWG committed version info originalVersion=${version} version=${after.version} codepage=${after.codepage} unchangedOtherFields=true`);
  }
});
