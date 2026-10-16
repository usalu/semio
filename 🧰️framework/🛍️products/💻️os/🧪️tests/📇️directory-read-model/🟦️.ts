/** 📇️ Directory read model: folding the shared directory event fixtures, descriptor digests and the empty read model.
 *
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { descriptorDigestEncodingV1, descriptorDigestV1 } from "../../🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🟦️.ts";
import { emptyDirectoryReadModel, foldAll } from "@semio-tech/framework/directory";
import * as nodePath from "node:path";
import * as nodeUrl from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

const source: TestSource = { directory: nodePath.resolve(nodePath.dirname(nodeUrl.fileURLToPath(import.meta.url)), "../.."), url: new URL("../../🟦️.ts", import.meta.url).href };

type DirectoryEvent = import("@semio-tech/framework/directory/schema").DirectoryEvent;

type DirectoryReadModel = import("@semio-tech/framework/directory").DirectoryReadModel;

type DocumentDescriptor = import("@semio-tech/framework/directory/schema").DocumentDescriptor;

describe("@semio-tech/framework-os directory", () => {
  const loadFixtureEvents = async (): Promise<DirectoryEvent[]> => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const raw = readFileSync(join(here, "🧪️tests/🧫️fixtures", "📇️directory", "⚡️events.json"), "utf8");
    return (JSON.parse(raw) as { events: DirectoryEvent[] }).events;
  };

  it("folds the golden fixture into the expected projection (parity with the Rust twin)", async () => {
    const events = await loadFixtureEvents();
    const model: DirectoryReadModel = foldAll(emptyDirectoryReadModel(), events);

    expect(model.cursor).toBe(16);
    expect(model.spaces.size).toBe(1);
    expect(model.spaces.has("sp-atelier-amara")).toBe(false);

    const studio = model.spaces.get("sp-studio-fabrication");
    expect(studio?.view.name).toBe("Fabrication Studio");
    expect(studio?.view.visibility).toBe("public");
    expect(studio?.view.kind).toBe("archive");
    expect(studio?.view.memberCount).toBe(2);

    const roles = (studio?.members ?? []).map((member) => [member.userId, member.role]).sort();
    expect(roles).toEqual([
      ["u-amara", "spectator"],
      ["u-devon", "spectator"],
    ]);

    const devon = studio?.members.find((member) => member.userId === "u-devon");
    expect(devon?.email).toBe("devon@semio.dev");
  });

  it("is idempotent on replay", async () => {
    const events = await loadFixtureEvents();
    const once = foldAll(emptyDirectoryReadModel(), events);
    const twice = foldAll(once, events);
    expect(twice).toEqual(once);
  });

  it("preserves the language-neutral document descriptor fixture canonically", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const fixture = JSON.parse(readFileSync(join(here, "🧪️tests/🧫️fixtures", "📇️directory", "🪪️document-descriptor.json"), "utf8")) as { valid: import("@semio-tech/framework/directory/schema").DocumentDescriptor; canonical: string };
    expect(JSON.stringify(fixture.valid)).toBe(fixture.canonical);
  });

  it("derives the domain-separated descriptor digest independently of JSON", async () => {
    const { createHash } = await import("node:crypto");
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const fixture = JSON.parse(readFileSync(join(here, "🧪️tests/🧫️fixtures", "📇️directory", "🛡️artifact-authority.json"), "utf8")) as { descriptor: DocumentDescriptor; descriptorEncodingHex: string; descriptorDigestV1: number[] };
    const encoded = descriptorDigestEncodingV1(fixture.descriptor);
    expect(Buffer.from(encoded).toString("hex")).toBe(fixture.descriptorEncodingHex);
    expect(Array.from(await descriptorDigestV1(fixture.descriptor))).toEqual(fixture.descriptorDigestV1);
    expect(Array.from(createHash("sha256").update(encoded).digest())).toEqual(fixture.descriptorDigestV1);
    expect(Buffer.from(encoded).includes(Buffer.from(JSON.stringify(fixture.descriptor)))).toBe(false);
    expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, spaceId: "" })).toThrow();
    expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, packSchemaHash: "AA".repeat(32) })).toThrow();
    expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, bootstrapFrontier: { ...fixture.descriptor.bootstrapFrontier, headSeq: Number.MAX_SAFE_INTEGER + 1 } })).toThrow();
  });

  it("projects document.announced without allowing replay to redefine it", async () => {
    const { readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const fixture = JSON.parse(readFileSync(join(here, "🧪️tests/🧫️fixtures", "📇️directory", "🪪️document-descriptor.json"), "utf8")) as { valid: import("@semio-tech/framework/directory/schema").DocumentDescriptor; conflictingSchemaHash: import("@semio-tech/framework/directory/schema").DocumentDescriptor };
    const created: DirectoryEvent = { seq: 1, id: "space", hlc: { physicalMs: 1, logical: 0 }, actor: { kind: "system", id: "system:test" }, spaceId: fixture.valid.spaceId, body: { kind: "space.created", spaceId: fixture.valid.spaceId, name: "Fixture", spaceKind: "studio", visibility: "private", ownerUserId: "user-owner" }, recordedAtMs: 1 };
    const announced: DirectoryEvent = { seq: 2, id: "document", hlc: { physicalMs: 2, logical: 0 }, actor: { kind: "user", id: "user:user-owner#test" }, spaceId: fixture.valid.spaceId, body: { kind: "document.announced", descriptor: fixture.valid }, recordedAtMs: 2 };
    const conflictReplay: DirectoryEvent = { ...announced, seq: 3, id: "conflict", body: { kind: "document.announced", descriptor: fixture.conflictingSchemaHash } };
    const model = foldAll(emptyDirectoryReadModel(), [created, announced, conflictReplay]);

    expect(model.spaces.get(fixture.valid.spaceId)?.documents).toEqual([fixture.valid]);
    expect(model.spaces.get(fixture.valid.spaceId)?.view.documentCount).toBe(1);
  });
});
