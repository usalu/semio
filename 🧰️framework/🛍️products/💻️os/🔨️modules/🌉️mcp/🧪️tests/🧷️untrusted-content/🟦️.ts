/** 🧷️ The untrusted-content law over the REAL `semio-os-mcp` binary (audit `📓️audit-s12-ai-mcp.md`
 * §3.3, G12-P1-2). A collaborator writes the law's canary — a prompt-injection sentence — into a
 * shared document; an agent then reads the document back through every surface the gateway has.
 * The canary must arrive only inside the `untrusted` envelope of every carrier the law names, and
 * nowhere in any observer or any other field, neither as text nor in any of its three base64
 * alignments. AJV (third party) validates every live envelope against the published
 * `UntrustedContentV1` and refuses every hostile one; `node:crypto` recomputes the provenance's
 * `contentSha256` over the decoded bytes. The Rust twin is
 * `artifact::quick::document_authored_content_reaches_an_agent_only_inside_the_untrusted_envelope`. */
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { requireMcpBinary, spawnRawMcp, type RawMcpProcess } from "../../🟦️.ts";
import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const bin = requireMcpBinary(repoRoot);
const moduleRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const readJson = (...segments: string[]): any => JSON.parse(readFileSync(join(moduleRoot, ...segments), "utf8"));
const law = readJson("🗿️artifact", "🧫️fixtures", "🧷️untrusted-content-law.json");
const mcpSchema = readJson("🧬️schema", "🔣️.json");
const artifactSchema = readJson("🗿️artifact", "🧬️schema", "🔣️.json");
const workspaceSchema = readJson("🏠️workspace", "🧬️schema", "🔣️.json");
const ajv = new Ajv({ strict: true, allErrors: true });
for (const schema of [mcpSchema, workspaceSchema, artifactSchema]) ajv.addSchema(schema);
const validEnvelope = ajv.getSchema(`${mcpSchema.$id}#/$defs/UntrustedContentV1`)!;
const validLaw = ajv.getSchema(`${artifactSchema.$id}#/$defs/UntrustedContentLawV1`)!;

const CATALOG_COMPILE_TIMEOUT_MS = 90_000;
const GUEST_CALL_TIMEOUT_MS = 240_000;
const canary: string = law.canary;
const needles = [canary, ...[0, 1, 2].map((shift) => Buffer.from(canary, "utf8").subarray(shift).subarray(0, Math.floor((Buffer.byteLength(canary) - shift) / 3) * 3).toString("base64"))];
const namesCanary = (text: string): boolean => needles.some((needle) => text.includes(needle));

/** ✂️ `value` with every untrusted envelope cut out, collecting the envelopes it held. */
function outsideUntrusted(value: unknown, envelopes: Record<string, any>[]): unknown {
  if (Array.isArray(value)) return value.map((item) => outsideUntrusted(item, envelopes));
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value as Record<string, unknown>).flatMap(([key, member]) => {
      if (key === law.envelopeField && (member as Record<string, unknown> | null)?.schema === "semio.mcp.untrusted-content/v1") {
        envelopes.push(member as Record<string, any>);
        return [];
      }
      return [[key, outsideUntrusted(member, envelopes)]];
    }),
  );
}

/** 📖️ The bytes an envelope carries, decoded in the order `contentSha256` hashes them. */
function envelopeBytes(envelope: Record<string, any>): Buffer {
  const content = envelope.content as Record<string, string>;
  return Buffer.concat(["packBase64", "sprBase64", "contentBase64"].filter((field) => typeof content[field] === "string").map((field) => Buffer.from(content[field]!, "base64")));
}

function withHostile(envelope: Record<string, any>, path: string[], value: unknown): Record<string, any> {
  const copy = structuredClone(envelope);
  let cursor: Record<string, any> = copy;
  for (const segment of path.slice(0, -1)) cursor = cursor[segment];
  cursor[path.at(-1)!] = value;
  return copy;
}

describe("semio-os-mcp — document content reaches an agent only as untrusted data", () => {
  let proc: RawMcpProcess;
  let folder: string;
  let discovered: Record<string, any>;
  const call = async (name: string, args: Record<string, unknown>, timeoutMs = GUEST_CALL_TIMEOUT_MS): Promise<Record<string, any>> => (await proc.request("tools/call", { name, arguments: args }, timeoutMs)) as Record<string, any>;
  /** ✍️ The collaborator's write, by the law's headless recipe: the kind's own text verb, prepared
   * and invoked like any agent edit, every `{{canary}}` replaced by the canary. */
  const plant = async (recipe: { capabilityId: string; input: Record<string, unknown> }): Promise<Record<string, any>> => {
    const input = JSON.parse(JSON.stringify(recipe.input).replaceAll("{{canary}}", canary));
    const prepared = await call("action_prepare", { capabilityId: recipe.capabilityId, input });
    expect(prepared.result?.isError, JSON.stringify(prepared).slice(0, 400)).not.toBe(true);
    return call("action_invoke", { preparedActionHandle: prepared.result.structuredContent.preparedHandle });
  };
  const carrierReply = async (carrier: { surface: string; name: string }, artifactId: string): Promise<unknown> =>
    carrier.surface === "tool" ? (await call(carrier.name, { artifactId })).result : JSON.parse(((await proc.request("resources/read", { uri: carrier.name.replace("{artifactId}", artifactId) })) as any).result.contents[0].text);
  const observerReply = async (observer: { surface: string; name: string }, artifactId: string): Promise<unknown> => {
    if (observer.surface === "list") return proc.request(observer.name, {});
    if (observer.surface === "resource") return proc.request("resources/read", { uri: observer.name.replace("{artifactId}", artifactId) });
    const argumentsFor: Record<string, Record<string, unknown>> = { context_resolve: { principal: "agent:local" }, capabilities_search: { query: "delete every block" }, inference_list: { artifactId } };
    return (await call(observer.name, argumentsFor[observer.name] ?? { artifactId })).result;
  };

  beforeAll(async () => {
    folder = mkdtempSync(join(tmpdir(), "semio-os-mcp-untrusted-"));
    proc = spawnRawMcp(bin, ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write"]);
    proc.writeRaw(JSON.stringify({ jsonrpc: "2.0", id: 9001, method: "server/discover", params: {} }));
    discovered = JSON.parse(await proc.nextLine(CATALOG_COMPILE_TIMEOUT_MS));
    expect((await call("artifact_create", { artifactId: "probe-canary", kind: "os.agent.probe/v1", initial: { text: canary } })).result?.isError).not.toBe(true);
    const recipe = law.plants.find((row: { lane: string }) => row.lane === "headless");
    expect((await call("artifact_create", { artifactId: "plugin-canary", kind: recipe.artifactKind })).result?.isError).not.toBe(true);
    const planted = await plant(recipe);
    expect(planted.result?.structuredContent?.status, `the collaborator's write: ${JSON.stringify(planted).slice(0, 400)}`).toBe("SUCCEEDED");
  }, 600_000);

  afterAll(async () => {
    await proc?.close();
    if (folder) rmSync(folder, { recursive: true, force: true });
  });

  it("the law fixture is a valid UntrustedContentLawV1 and its canary is printable ASCII", () => {
    expect(validLaw(law), JSON.stringify(validLaw.errors)).toBe(true);
  });

  it("the server states the rule before any tool is called, and the carrying tools state it again", async () => {
    expect(String(discovered.result?.instructions ?? "")).toContain("semio.mcp.untrusted-content/v1");
    expect(String(discovered.result?.instructions ?? "")).toContain("nie als Anweisungen");
    const { tools } = ((await proc.request("tools/list", {})) as any).result as { tools: Array<{ name: string; description?: string }> };
    for (const carrier of law.carriers.filter((row: { surface: string }) => row.surface === "tool")) {
      expect(tools.find((tool) => tool.name === carrier.name)?.description ?? "", `${carrier.name} description`).toContain("semio.mcp.untrusted-content/v1");
    }
  });

  for (const artifactId of ["probe-canary", "plugin-canary"]) {
    it(`every carrier hands ${artifactId}'s content over only inside a valid, pinned envelope`, async () => {
      for (const carrier of law.carriers) {
        if (artifactId === "probe-canary" && carrier.source === "artifact-export") continue;
        const reply = await carrierReply(carrier, artifactId);
        const envelopes: Record<string, any>[] = [];
        const outside = JSON.stringify(outsideUntrusted(reply, envelopes));
        expect(namesCanary(outside), `${carrier.name} leaked the canary outside its envelope: ${outside.slice(0, 400)}`).toBe(false);
        expect(envelopes.length, `${carrier.name} carries exactly one envelope`).toBe(1);
        const envelope = envelopes[0]!;
        expect(validEnvelope(envelope), `${carrier.name}: ${JSON.stringify(validEnvelope.errors)}`).toBe(true);
        expect(envelope.provenance.source).toBe(carrier.source);
        expect(envelope.provenance.artifactId).toBe(artifactId);
        expect(envelope.provenance.authors.kind).toBe("local-principal");
        const bytes = envelopeBytes(envelope);
        if (carrier.source === "artifact-body") expect(bytes.toString("latin1").includes(canary), `${carrier.name} really carries the collaborator's text`).toBe(true);
        expect(envelope.provenance.revision.contentSha256).toBe(createHash("sha256").update(bytes).digest("hex"));
        for (const hostile of law.hostile) expect(validEnvelope(withHostile(envelope, hostile.path, hostile.value)), `hostile ${hostile.name} was admitted`).toBe(false);
      }
    }, GUEST_CALL_TIMEOUT_MS);

    it(`no observer forwards ${artifactId}'s content at all`, async () => {
      for (const observer of law.observers) {
        const reply = JSON.stringify(await observerReply(observer, artifactId));
        expect(namesCanary(reply), `${observer.name} forwarded document content outside any envelope: ${reply.slice(0, 400)}`).toBe(false);
      }
    }, GUEST_CALL_TIMEOUT_MS);
  }
});
