import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ResourceSchema } from "@modelcontextprotocol/sdk/types.js";
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, test } from "vitest";

const remoteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../🏠️workspace/🔗️remote");
const schema = JSON.parse(readFileSync(resolve(remoteRoot, "🧬️schema/🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️authenticated-hub-descriptor-index.json"), "utf8"));

describe("authenticated hub workspace fixture oracle", () => {
  test("AJV independently validates the neutral P4-A contract and fixed bounds", () => {
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.limits).toEqual({
      maxDocuments: 4096,
      maxTokenBytes: 4096,
      maxDiagnosticBytes: 4096,
      operationTimeoutMs: 10000,
    });
  });

  test("the MCP SDK accepts only the ready descriptor resources and no raw body resource", () => {
    const ready = fixture.cases.memberReady.expected.resourceUris as string[];
    const resources = ready.map((uri) => ({
      uri,
      name: uri.endsWith("/descriptor") ? "shared-doc descriptor" : uri.endsWith("/artifacts") ? "Workspace artifacts" : "Workspace",
      mimeType: "application/json",
    }));
    for (const resource of resources) expect(ResourceSchema.safeParse(resource).success).toBe(true);
    expect(ready).toEqual([
      "semio://workspace",
      "semio://workspace/artifacts",
      `semio://workspace/scopes/${encodeURIComponent("space-a")}/${encodeURIComponent("shared-doc")}/descriptor`,
    ]);
    expect(ready.some((uri) => uri === "semio://artifact/shared-doc" || uri.endsWith("/schema") || uri.endsWith("/validation"))).toBe(false);
    expect(Object.values(fixture.cases).filter((entry: any) => entry.expected.state !== "ready").every((entry: any) => entry.expected.resourceUris.length === 0)).toBe(true);
  });

  test("remote authorization stays distinct from local principal claims and bearer material", () => {
    const serialized = JSON.stringify(fixture);
    expect(serialized).not.toContain("localPolicyPrincipal");
    expect(serialized).not.toContain("Bearer ");
    expect(fixture.cases.publicWithoutMembership.expected.state).toBe("revoked");
    expect(fixture.cases.sameDocumentOtherSpace.expected.cacheAction).toBe("invalidate");
    expect(fixture.cases.memberRevoked.expected.state).toBe("revoked");
    expect(fixture.cases.streamReconnect.expected.state).toBe("refreshing");
  });
});
