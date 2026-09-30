import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import policy from "../../🔨️modules/📇️directory/🛡️access-policy/🔣️.json";
import schema from "../../🔨️modules/📇️directory/🛡️access-policy/🧬️schema/🔣️.json";
import fixture from "../../🔨️modules/📇️directory/🛡️access-policy/🧫️fixtures/🔣️.json";
import { directoryAccessPermits, parseDirectoryAccessPolicyV1, type DirectoryAccessActionV1, type DirectoryAccessRoleV1 } from "../../🔨️modules/📇️directory/🛡️access-policy/🧬️schema/🟦️.ts";

describe("shared directory access policy", () => {
  const validate = new Ajv({ strict: true }).compile(schema);

  it("matches the schema oracle for every language-neutral policy", () => {
    for (const row of [{ name: "declared-policy", value: policy, valid: true }, ...fixture.policies]) {
      expect(validate(row.value), row.name).toBe(row.valid);
      if (row.valid) expect(parseDirectoryAccessPolicyV1(row.value), row.name).toEqual(row.value);
      else expect(() => parseDirectoryAccessPolicyV1(row.value), row.name).toThrow();
    }
  });

  it("decides every neutral role, action and space-kind vector", () => {
    const declared = parseDirectoryAccessPolicyV1(policy);
    for (const row of fixture.vectors) expect(directoryAccessPermits(declared, row.roles as DirectoryAccessRoleV1[], row.action as DirectoryAccessActionV1, "spaceKind" in row ? row.spaceKind : undefined), row.name).toBe(row.permitted);
    console.log(`[access-policy] policies=${fixture.policies.length + 1} decisions=${fixture.vectors.length}`);
  });

  it("keeps deny precedence independent of declaration order", () => {
    const declared = parseDirectoryAccessPolicyV1(policy);
    const reverse = { ...declared, grants: [...declared.grants].reverse() };
    for (const row of fixture.vectors) expect(directoryAccessPermits(reverse, row.roles as DirectoryAccessRoleV1[], row.action as DirectoryAccessActionV1, "spaceKind" in row ? row.spaceKind : undefined), row.name).toBe(row.permitted);
  });
});
