/** 🏗️ The worker's space-artifact creation owner: catalog publication, status polling, cancellation and conflict handling over the exact retained creation owner. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type FetchTimeoutResponse = import("@semio-tech/framework/io/fetch-timeout").FetchTimeoutResponse;
type HubSpaceArtifactCreationStatusV1 = import("../../🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts").SpaceArtifactCreationStatusV1;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerSpaceArtifactCreationTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, handleTsRequest, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations } = dependencies;
  describe("space artifact creation owner", () => {
    beforeEach(resetWorkerLaw);
    const requestId = "1".repeat(32);
    const catalogGenerationId = "3".repeat(64);
    const response = (phase: HubSpaceArtifactCreationStatusV1["phase"], ready?: HubSpaceArtifactCreationStatusV1["ready"], generation = catalogGenerationId): FetchTimeoutResponse => {
      const body = JSON.stringify({ schema: "semio.hub.space-artifact-creation-status/v1", requestId, spaceId: "space-a", catalogGenerationId: generation, phase, ...(ready === undefined ? {} : { ready }) });
      return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
    };

    it("publishes only the canonical selected-current catalog for the exact Space", async () => {
      const body = JSON.stringify({
        schema: "semio.hub.space-artifact-creation-catalog/v1",
        spaceId: "space-a",
        catalogGenerationId,
        kinds: [{ kindId: "s.demo.demo", schema: "s.demo.demo", dialect: { artifactKind: "s.demo.demo", standard: "1", subset: "any" }, label: { en: "Demo Map", de: "Demo-Karte" } }],
      });
      const replies: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        expect([path, init.method]).toEqual(["/spaces/space-a/artifact-creations", "GET"]);
        return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
      };
      testSeams.workerPostTestSink = (message) => replies.push(message);
      const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
      handleTsRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" });
      await vi.waitFor(() => expect(replies.at(-1)).toEqual({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "ready" }));
      expect(replies).toEqual([
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "loading" },
        { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId: "space-a", catalogGenerationId, kinds: JSON.parse(body).kinds },
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "ready" },
      ]);
      expect(spaceArtifactCreationCatalogOperations).toHaveLength(0);

      testSeams.spaceArtifactCreationTestFetch = async () => new Response(body.replace('"spaceId":"space-a"', '"spaceId":"space-b"'), { status: 200 });
      handleTsRequest({ kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" });
      await vi.waitFor(() => expect(spaceArtifactCreationCatalogOperations).toHaveLength(0));
      expect(replies.slice(-2)).toEqual([
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "loading" },
        { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "unavailable" },
      ]);
    });

    it("retains the selected catalog generation through rotated and missing statuses until exact ready", async () => {
      const calls: Array<readonly [string, string]> = [];
      const bodies: string[] = [];
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      const missingGeneration = JSON.stringify({ schema: "semio.hub.space-artifact-creation-status/v1", requestId, spaceId: "space-a", phase: "accepted" });
      const replies: readonly FetchTimeoutResponse[] = [
        response("accepted", undefined, "4".repeat(64)),
        new Response(missingGeneration, { status: 200, headers: { "content-length": String(new TextEncoder().encode(missingGeneration).byteLength) } }),
        response("accepted"),
        response("ready", { artifactId: `artifact-${"2".repeat(32)}`, kindId: "s.demo.demo", artifactSchema: "s.demo.demo", parentDialect: { artifactKind: "s.demo.demo", standard: "1", subset: "any" } }),
      ];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        if (typeof init.body === "string") bodies.push(init.body);
        return replies[calls.length - 1]!;
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("ready"), { timeout: 2_000 });
      expect(calls).toEqual([
        ["/spaces/space-a/artifact-creations", "POST"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
        [`/spaces/space-a/artifact-creations/${requestId}`, "GET"],
      ]);
      expect(JSON.parse(bodies[0]!)).toEqual({ schema: "semio.hub.space-artifact-create/v1", requestId, expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      expect(statuses.every((status) => status.catalogGenerationId === catalogGenerationId)).toBe(true);
      expect(statuses.filter((status) => status.ready !== undefined)).toHaveLength(1);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("follows a creation the hub keeps accepted far past the old client deadline, and concludes only when the hub stops answering", async () => {
      const polling = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🏪️store/👷️worker/🌱️creation-polling/🔣️.json", source.url), "utf8")) as { pollMaxMs: number; unreachableBoundMs: number };
      vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "Date"] });
      try {
        let answering = true;
        let readyAfterMs = 10 * 60_000;
        const startedAtMs = Date.now();
        const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
        let gets = 0;
        testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
          if ((init.method ?? "GET") === "GET") gets += 1;
          if (!answering && (init.method ?? "GET") === "GET") throw new Error("hub unreachable");
          if (Date.now() - startedAtMs >= readyAfterMs) return response("ready", { artifactId: `artifact-${"2".repeat(32)}`, kindId: "s.demo.demo", artifactSchema: "s.demo.demo", parentDialect: { artifactKind: "s.demo.demo", standard: "1", subset: "any" } });
          return response("accepted");
        };
        testSeams.workerPostTestSink = (message) => {
          if (message.kind === "space-artifact-creation-status") statuses.push(message);
        };
        handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Slow Map" });
        await vi.advanceTimersByTimeAsync(9 * 60_000);
        expect(statuses.some((status) => status.phase === "indeterminate"), "a hub that still answers is never concluded").toBe(false);
        expect(spaceArtifactCreationOperations).toHaveLength(1);
        expect(gets, "polls back off to the contract's maximum interval").toBeLessThanOrEqual(Math.ceil((9 * 60_000) / polling.pollMaxMs) + 16);
        await vi.advanceTimersByTimeAsync(2 * 60_000);
        expect(statuses.at(-1)?.phase).toBe("ready");
        expect(spaceArtifactCreationOperations).toHaveLength(0);

        statuses.length = 0;
        readyAfterMs = Number.POSITIVE_INFINITY;
        const secondRequestId = "5".repeat(32);
        testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
          if (!answering && (init.method ?? "GET") === "GET") throw new Error("hub unreachable");
          const body = JSON.stringify({ schema: "semio.hub.space-artifact-creation-status/v1", requestId: secondRequestId, spaceId: "space-a", catalogGenerationId, phase: "accepted" });
          return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
        };
        handleTsRequest({ kind: "space-artifact-create", requestId: secondRequestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Lost Map" });
        await vi.advanceTimersByTimeAsync(5 * 60_000);
        expect(statuses.some((status) => status.phase === "indeterminate")).toBe(false);
        answering = false;
        await vi.advanceTimersByTimeAsync(polling.unreachableBoundMs - polling.pollMaxMs * 2);
        expect(statuses.some((status) => status.phase === "indeterminate"), "not before the unreachable bound").toBe(false);
        await vi.advanceTimersByTimeAsync(polling.pollMaxMs * 4);
        expect(statuses.at(-1)?.phase, "concluded only once the hub stopped answering for the bound").toBe("indeterminate");
        expect(spaceArtifactCreationOperations).toHaveLength(0);
      } finally {
        vi.useRealTimers();
      }
    });

    it("routes cancellation through the exact retained owner without manufacturing a ready tuple", async () => {
      const calls: Array<readonly [string, string]> = [];
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        return response(path.endsWith("/cancel") ? "cancelled" : "preparing");
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.some((status) => status.phase === "preparing")).toBe(true));
      handleTsRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("cancelled"), { timeout: 2_000 });
      expect(calls.at(-1)).toEqual([`/spaces/space-a/artifact-creations/${requestId}/cancel`, "POST"]);
      expect(statuses.every((status) => status.ready === undefined)).toBe(true);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("never fabricates cancelled from a rejected cancel request", async () => {
      const statuses: Array<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>> = [];
      testSeams.spaceArtifactCreationTestFetch = async (path) => {
        if (path.endsWith("/cancel")) return new Response(null, { status: 409 });
        return response("preparing");
      };
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      await vi.waitFor(() => expect(statuses.some((status) => status.phase === "preparing")).toBe(true));
      handleTsRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
      await vi.waitFor(() => expect(statuses.at(-1)?.phase).toBe("failed"), { timeout: 2_000 });
      expect(statuses.some((status) => status.phase === "cancelled")).toBe(false);
      expect(spaceArtifactCreationOperations).toHaveLength(0);
    });

    it("requires a catalog refresh before failing one exact initial POST conflict without resubmission", async () => {
      const calls: Array<readonly [string, string]> = [];
      const statuses: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = async (path, init) => {
        calls.push([path, init.method ?? "GET"]);
        return new Response(null, { status: 409 });
      };
      testSeams.workerPostTestSink = (message) => statuses.push(message);
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      await vi.waitFor(() => expect(spaceArtifactCreationOperations).toHaveLength(0));
      expect(calls).toEqual([["/spaces/space-a/artifact-creations", "POST"]]);
      expect(statuses).toEqual([
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "accepted" },
        { kind: "space-artifact-creation-catalog-refresh-required", requestId, spaceId: "space-a", catalogGenerationId },
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "failed" },
      ]);
    });

    it("suppresses a late initial POST conflict after the exact creation owner retires", async () => {
      let resolveResponse!: (response: FetchTimeoutResponse) => void;
      const statuses: BackboneWorkerResponse[] = [];
      testSeams.spaceArtifactCreationTestFetch = () => new Promise((resolve) => {
        resolveResponse = resolve;
      });
      testSeams.workerPostTestSink = (message) => {
        if (message.kind === "space-artifact-creation-status" || message.kind === "space-artifact-creation-catalog-refresh-required") statuses.push(message);
      };
      handleTsRequest({ kind: "space-artifact-create", requestId, spaceId: "space-a", expectedCatalogGenerationId: catalogGenerationId, kindId: "s.demo.demo", name: "Shared Map" });
      await vi.waitFor(() => expect(spaceArtifactCreationOperations).toHaveLength(1));
      const operation = spaceArtifactCreationOperations.get(requestId)!;
      spaceArtifactCreationOperations.delete(requestId);
      operation.abort.abort(new Error("space artifact creation: replaced"));
      resolveResponse(new Response(null, { status: 409 }));
      await Promise.resolve();
      await Promise.resolve();
      expect(statuses).toEqual([
        { kind: "space-artifact-creation-status", requestId, spaceId: "space-a", catalogGenerationId, phase: "accepted" },
      ]);
    });
  });
}
