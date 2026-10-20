/** 🔶️ The round trips scoped directory tests of the DirectoryClient.stream suite. */
import { expect, it } from "vitest";
import { BACKBONE_WORKER_WIRE_MAGIC, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { encodePackValue } from "@semio-tech/framework/packs";
import { parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import { source, type BackboneWorkerRequest } from "../../🧮️directory-client-helpers/🟦️.ts";
import type { BackboneWorkerResponse, BrowserActorUiMountedV1, DirectoryCommand } from "../🟢️shared/🟦️.ts";

/** 🔷 Registers the round trips scoped directory tests of the DirectoryClient.stream suite. */
export function registerRoundTripsScopedDirectoryTests(): void {
  it("round trips scoped directory worker ownership without flattening scope", () => {
    const scope = { spaceId: "space/a", documentId: "document b" };
    const request: BackboneWorkerRequest = { kind: "directory-scope-open", baseUrl: "http://hub.test", scope, since: 7 };
    const response: BackboneWorkerResponse = { kind: "directory-scope-revoked", scope };
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
  });
  it("carries every directory command across the worker wire back into its declaration order, so the worker can seal it", async () => {
    const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import("@semio-tech/framework/directory/io/text");
    const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🧪️tests/🧫️fixtures/📇️directory/🧾️command-receipt-v1.json", source.url), "utf8")) as { readonly requests: readonly { readonly name: string; readonly requestId: string; readonly command: DirectoryCommand; readonly canonical: string }[] };
    const reordered = corpus.requests.filter((row) => Object.keys(row.command).join(",") !== Object.keys(row.command).sort().join(","));
    expect(reordered.map((row) => row.command.kind)).toEqual(expect.arrayContaining(["upsert-member", "rename-space", "create-invite"]));
    for (const row of corpus.requests) {
      const decoded = decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "directory-command", requestId: row.requestId, command: row.command }));
      expect(decoded, row.name).toEqual({ kind: "directory-command", requestId: row.requestId, command: row.command });
      if (decoded.kind !== "directory-command") throw new Error(row.name);
      expect(Object.keys(decoded.command), row.name).toEqual(Object.keys(row.command));
      expect(directoryCommandRequestJson(sealDirectoryCommandRequestV1(decoded.requestId, decoded.command)), row.name).toBe(row.canonical);
    }
    expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "directory-command", requestId: "0".repeat(32), command: corpus.requests[0]!.command }))).toThrow("invalid directory command request id");
    expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "directory-command", requestId: corpus.requests[0]!.requestId, command: { ...corpus.requests[0]!.command, extra: 1 } as unknown as DirectoryCommand }))).toThrow();
  });
  it("document opening attempt stays outer-wire-owned without widening the browser patch contract", async () => {
    const clientInstanceId = "33333333-3333-4333-8333-333333333333";
    const requests: readonly BackboneWorkerRequest[] = [
      { kind: "open", actorIdentityGrant: { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 }, documentId: "same-document", clientInstanceId, schema: "fixture.counter", actor: "caller", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "https://hub.test", spaceId: "space-a" }] },
      { kind: "send", documentId: "same-document", spaceId: "space-a", clientInstanceId, message: { kind: "externalChanged" } },
      { kind: "close", documentId: "same-document", spaceId: "space-a", clientInstanceId },
    ];
    for (const request of requests) expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);

    const event: BackboneWorkerResponse = { kind: "event", documentId: "same-document", clientInstanceId, scope: { spaceId: "space-a", documentId: "same-document" }, event: { kind: "status", persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } } };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(event))).toEqual(event);
    const fixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const offer = parseBrowserActorUiPatchOfferV1(fixture.offer);
    const result = parseBrowserActorUiPatchResultV1(fixture.acknowledged);
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...offer, clientInstanceId }))).toEqual({ ...offer, clientInstanceId });
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...result, clientInstanceId }))).toEqual({ ...result, clientInstanceId });
    expect(Object.hasOwn(parseBrowserActorUiPatchOfferV1(fixture.offer), "clientInstanceId")).toBe(false);
    expect(Object.hasOwn(parseBrowserActorUiPatchResultV1(fixture.acknowledged), "clientInstanceId")).toBe(false);
    const mounted: BrowserActorUiMountedV1 = {
      kind: "browser-actor-ui-mounted",
      scope: { spaceId: "space-a", documentId: "same-document" },
      clientInstanceId,
      activationGeneration: "41",
      instanceId: 0,
      verifiedSurfaceId: "fixture.neutral-host-fixture.counter@1/viewer",
      windowKindId: "fixture-counter-main",
      catalogGenerationId: "1".repeat(64),
      componentSha256: "2".repeat(64),
      descriptorSha256: "3".repeat(64),
      browserActorSha256: "4".repeat(64),
      activeCheckpointId: "5".repeat(64),
      descriptorDigestV1: "6".repeat(64),
      frontier: { documentId: "same-document", headEditOrdinal: 7, headEditId: "edit-7", lastCommitSeq: 7, chainHash: new Array(32).fill(7) },
      uiRevision: 7,
    };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(mounted))).toEqual(mounted);
    const mountedUnknown = encodePackValue({ ...mounted, grant: "forbidden" });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mountedUnknown]))).toThrow("invalid mounted UI fields");
    const { windowKindId: _windowKindId, ...mountedWindowless } = mounted;
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue(mountedWindowless)]))).toThrow("invalid mounted UI fields");
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ ...mounted, windowKindId: "" })]))).toThrow("invalid mounted UI owner");
    const mountedForeignFrontier = encodePackValue({ ...mounted, frontier: { ...mounted.frontier, documentId: "foreign" } });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mountedForeignFrontier]))).toThrow("invalid mounted UI identity");

    const missing = new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "event", documentId: "same-document", event: event.event })]);
    expect(() => decodeBackboneWorkerResponse(missing)).toThrow("invalid client instance id");
    const malformed = new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "close", documentId: "same-document", clientInstanceId: "not-an-owner" })]);
    expect(() => decodeBackboneWorkerRequest(malformed)).toThrow("invalid client instance id");
  });
  it("withholds creation authority and document coordinates until an exact ready status", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🌱️space-artifact-creation-generation-v1.json", source.url), "utf8"));
    const requestId = "1".repeat(32);
    const catalogGenerationId = "3".repeat(64);
    const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
    const catalogRequest: BackboneWorkerRequest = { kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" };
    const catalog: BackboneWorkerResponse = {
      kind: "space-artifact-creation-catalog",
      clientInstanceId,
      spaceId: "space-a",
      catalogGenerationId,
      kinds: [{ kindId: "fixture.neutral-host-fixture.counter", schema: "fixture.neutral-host-fixture.counter", dialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "any" }, label: { en: "Counter", de: "Zähler" } }],
    };
    const request: BackboneWorkerRequest = { kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "fixture.neutral-host-fixture.counter", name: "Shared Counter" };
    const ready: BackboneWorkerResponse = {
      kind: "space-artifact-creation-status",
      requestId,
      spaceId: "space-a",
      catalogGenerationId,
      phase: "ready",
      ready: { artifactId: `artifact-${"2".repeat(32)}`, kindId: "fixture.neutral-host-fixture.counter", artifactSchema: "fixture.neutral-host-fixture.counter", parentDialect: { artifactKind: "fixture.neutral-host-fixture.counter", standard: "1", subset: "any" } },
    };
    expect(equal(fixture.request, request)).toBe(true);
    expect(equal(fixture.ready, ready)).toBe(true);
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(catalogRequest))).toEqual(catalogRequest);
    for (const phase of ["loading", "ready", "unavailable"] as const) {
      const status: BackboneWorkerResponse = { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase };
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(status))).toEqual(status);
    }
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(catalog))).toEqual(catalog);
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
    expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" }))).toEqual({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(ready))).toEqual(ready);
    const refreshRequired: BackboneWorkerResponse = { kind: "space-artifact-creation-catalog-refresh-required", requestId, spaceId: "space-a", catalogGenerationId };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(refreshRequired))).toEqual(refreshRequired);
    expect(fixture.initialPostConflict).toEqual({
      httpStatus: 409,
      workerResponseKind: refreshRequired.kind,
      terminalPhase: "failed",
      emissionOrder: ["catalog-refresh-required", "failed"],
      catalogRefresh: true,
      autoResubmit: false,
    });
    const leaked = encodePackValue({ ...ready, phase: "preparing" });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...leaked]))).toThrow("invalid fields");
    const forgedCatalog = encodePackValue({ ...catalog, kinds: [{ ...catalog.kinds[0], descriptor: "forbidden" }] });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...forgedCatalog]))).toThrow("invalid kind fields");
    const forgedCatalogStatus = encodePackValue({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "empty" });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...forgedCatalogStatus]))).toThrow("invalid owner");
    const catalogNamed: BackboneWorkerResponse = { ...ready, ready: { ...ready.ready!, kindId: "2d.drawing", artifactSchema: "drawing.document", parentDialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" } } };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(catalogNamed)), "a creation kind need not equal the dialect that opens it").toEqual(catalogNamed);
    const drawingCatalog: BackboneWorkerResponse = { ...catalog, kinds: [{ kindId: "2d.drawing", schema: "drawing.document", dialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" }, label: { en: "Editor", de: "Editor" } }] };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(drawingCatalog))).toEqual(drawingCatalog);
    const spaceEvent = (seq: number, spaceId = "space-a") => ({ seq, id: `event-${seq}`, hlc: { physicalMs: 1_790_000_000_000 + seq, logical: 0 }, actor: { kind: "user" as const, id: "user:u1" }, spaceId, body: { kind: "space.renamed" as const, spaceId, name: `Space ${seq}` }, recordedAtMs: 1_790_000_000_000 + seq });
    for (const request of [{ kind: "directory-space-open", baseUrl: "http://127.0.0.1:8040", spaceId: "space-a" }, { kind: "directory-space-close", spaceId: "space-a" }] as const) expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "directory-space-open", spaceId: "space-a" })]))).toThrow("invalid space directory fields");
    const spaceEvents: BackboneWorkerResponse = { kind: "directory-space-events", spaceId: "space-a", events: [spaceEvent(3), spaceEvent(7)] };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(spaceEvents))).toEqual(spaceEvents);
    for (const events of [[spaceEvent(7), spaceEvent(3)], [spaceEvent(3), spaceEvent(3)], [spaceEvent(3, "space-b")], []]) {
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "directory-space-events", spaceId: "space-a", events })])), JSON.stringify(events.map((event) => [event.seq, event.spaceId]))).toThrow(/invalid space directory/u);
    }
    const mismatched = encodePackValue({ ...ready, ready: { ...ready.ready!, parentDialect: { ...ready.ready!.parentDialect, artifactKind: "" } } });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mismatched]))).toThrow("invalid ready identity");
    const overposted = encodePackValue({ ...request, descriptor: "forbidden" });
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...overposted]))).toThrow("invalid space artifact creation fields");
    const missingGeneration = encodePackValue({ kind: request.kind, requestId, spaceId: request.spaceId, kindId: request.kindId, name: request.name });
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...missingGeneration]))).toThrow("invalid space artifact creation fields");
    const zeroGeneration = encodePackValue({ ...request, expectedCatalogGenerationId: "0".repeat(64) });
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...zeroGeneration]))).toThrow("invalid space artifact creation intent");
    const missingStatusGeneration = encodePackValue({ kind: ready.kind, requestId, spaceId: ready.spaceId, phase: "accepted" });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...missingStatusGeneration]))).toThrow("invalid owner");
    for (const invalidRefresh of [
      { ...refreshRequired, catalogGenerationId: "0".repeat(64) },
      { kind: refreshRequired.kind, requestId, spaceId: "space-a" },
      { ...refreshRequired, retry: true },
    ]) {
      const wire = encodePackValue(invalidRefresh);
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...wire]))).toThrow("space artifact creation catalog refresh: invalid");
    }
  });
}
