import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { ResourceSchema } from "@modelcontextprotocol/sdk/types.js";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";

const remoteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../🏠️workspace/🔗️remote");
const schema = JSON.parse(readFileSync(resolve(remoteRoot, "🧬️schema/🔣️.json"), "utf8"));
const validateDescriptorIndex = (() => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  return ajv.getSchema(`${schema.$id}#/$defs/AuthenticatedHubDescriptorIndexV1`)!;
})();
const fixture = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️authenticated-hub-descriptor-index.json"), "utf8"));
const refusalCorpus = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️hub-unavailable-refusal.json"), "utf8"));

describe("authenticated hub workspace fixture oracle", () => {
  test("AJV independently validates the neutral P4-A contract and fixed bounds", () => {
    expect(validateDescriptorIndex(fixture), JSON.stringify(validateDescriptorIndex.errors)).toBe(true);
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

  test("a principal binds only at a role its account's member row admits: exactly it for a human, at most it for an agent", () => {
    const rank: Record<string, number> = { spectator: 0, author: 1 };
    const checked: string[] = [];
    for (const [name, entry] of Object.entries(fixture.cases) as [string, any][]) {
      const bodies = (entry.responses as any[]).map((response) => ({ text: response.canonicalBody as string | undefined, value: response.canonicalBody === undefined ? response.body : JSON.parse(response.canonicalBody) }));
      const session = bodies.find((body) => body.value?.schema === "semio.directory.session-authority.v1")?.value;
      const page = bodies.find((body) => body.value?.schema === "semio.directory.space-administration-page.v1");
      if (!session || !page || page.value.access === "public") continue;
      const unsigned = page.text!.replace(/,"receiptSha256":"[0-9a-f]{64}"\}$/u, "}");
      expect(createHash("sha256").update(unsigned).digest("hex"), name).toBe(page.value.receiptSha256);
      const row = page.value.members.rows.find((member: any) => member.userId === session.userId);
      const role = page.value.space.role;
      const admitted = row !== undefined && (role === row.role || (session.sessionKind === "agent" && rank[role]! < rank[row.role]!));
      expect(entry.expected.errorCode === "PERMISSION_DENIED", name).toBe(!admitted);
      checked.push(name);
    }
    expect(checked.sort()).toEqual(["agentBelowMembership", "humanBelowMembership", "memberReady", "principalAboveMembership", "sameDocumentOtherSpace"]);
    expect(fixture.cases.agentBelowMembership.expected.state).toBe("ready");
  });

  test("a hub-unavailable refusal keeps its typed cause bounded and bilingual", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const validateCorpus = ajv.getSchema(`${schema.$id}#/$defs/HubUnavailableRefusalCorpusV1`)!;
    expect(validateCorpus(refusalCorpus), JSON.stringify(validateCorpus.errors)).toBe(true);
    const typedCode = (body: string): string | null => {
      try {
        const code = JSON.parse(body)?.code;
        return typeof code === "string" ? [...code].slice(0, 64).join("") : null;
      } catch {
        return null;
      }
    };
    const kinds = new Set<string>();
    for (const entry of refusalCorpus.cases) {
      const fault = entry.fault;
      const cause =
        fault.kind === "transport"
          ? { kind: "transport", detail: [...fault.io].slice(0, refusalCorpus.detailMaxChars).join("") }
          : fault.kind === "http"
            ? { kind: "http", status: fault.status, code: typedCode(fault.body) }
            : { kind: "undecodable-response" };
      const said = cause.kind === "transport" ? `transport: ${cause.detail}` : cause.kind === "http" ? `HTTP ${cause.status}${cause.code === null ? "" : ` ${cause.code}`}` : "undecodable response";
      expect(entry.expected.details.cause, entry.id).toEqual(cause);
      expect(entry.expected.message, entry.id).toBe(`hub directory is temporarily unavailable (${said})`);
      expect(entry.expected.details.summary.en, entry.id).not.toBe(entry.expected.details.summary.de);
      if (cause.kind === "transport") for (const text of Object.values(entry.expected.details.summary)) expect(text, entry.id).toContain(cause.detail);
      kinds.add(cause.kind);
    }
    expect([...kinds].sort()).toEqual(["http", "transport", "undecodable-response"]);
    expect(refusalCorpus.cases.some((entry: any) => entry.fault.kind === "transport" && [...entry.fault.io].length > refusalCorpus.detailMaxChars)).toBe(true);
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
