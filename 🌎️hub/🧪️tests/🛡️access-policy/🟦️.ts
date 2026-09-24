// #region Header
/**
 * 🛡️ Third-party oracle for the hub's declared access policy: Ajv validates the declared document
 * and every truth-table vector against `schema://hub.auth`, and the TypeScript twin decides every
 * vector exactly as the Rust authority does (`🔐️auth/🛡️access-policy/🧪️tests`).
 * @see ../../🔐️auth/🛡️access-policy/🦀️.rs
 */
// #endregion Header

import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { hubSchemaExport } from "../../🤝️integration-harness/🟦️.ts";
import { hubAccessPermits, type HubAccessActionV1, type HubAccessPolicyV1, type HubAccessRoleV1 } from "../../🔐️auth/🛡️access-policy/🟦️.ts";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..", "..");
const read = (path: string) => JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
const policy = read("🌎️hub/🔐️auth/🛡️access-policy/🔣️.json") as HubAccessPolicyV1;
const vectors = read("🌎️hub/🔐️auth/🛡️access-policy/🧫️fixtures/🔣️.json").vectors as Array<{ name: string; roles: HubAccessRoleV1[]; action: HubAccessActionV1; spaceKind?: string; permitted: boolean }>;

describe("declared hub access policy", () => {
  it("is a valid HubAccessPolicyV1 and every vector a valid decision vector", () => {
    expect(hubSchemaExport(repoRoot, "schema://hub.auth/HubAccessPolicyV1")(policy)).toBe(true);
    const vector = hubSchemaExport(repoRoot, "schema://hub.auth/HubAccessDecisionVectorV1");
    for (const row of vectors) expect(vector(row), row.name).toBe(true);
  });

  it("decides every truth-table vector exactly as declared", () => {
    for (const row of vectors) expect(hubAccessPermits(policy, row.roles, row.action, row.spaceKind), row.name).toBe(row.permitted);
  });
});
