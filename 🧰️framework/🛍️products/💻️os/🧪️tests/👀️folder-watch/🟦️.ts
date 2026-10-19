/** 👀️ Stream-primary folder watch: the coalesced poll, its suppression while the stream is open, one stream per folder and abort on close. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, WORKER_LAW_ACTOR_IDENTITY_GRANT } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactActorConfig = import("../../🟦️.ts").ArtifactActorConfig;
  type StreamMuxEndpointV1 = import("@semio-tech/framework/io/stream-mux").StreamMuxEndpointV1;
  type StreamMuxHandlersV1 = import("@semio-tech/framework/io/stream-mux").StreamMuxHandlersV1;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerFolderWatchTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { installStreamMuxEndpoint, openArtifact, artifactState, closeArtifact, SANITY_POLL_MIN_MS } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    /** 🔀️ The page's stream channel as the worker sees it: every open is a recorded stream whose handlers the test drives. */
    class FakeStreamEndpoint implements StreamMuxEndpointV1 {
      readonly streams: { readonly route: string; readonly key: string; readonly handlers: StreamMuxHandlersV1; closed: boolean }[] = [];
      open(route: string, key: string, handlers: StreamMuxHandlersV1, options: { readonly signal?: AbortSignal } = {}): { close(): void } {
        const stream = { route, key, handlers, closed: false };
        this.streams.push(stream);
        options.signal?.addEventListener("abort", () => (stream.closed = true), { once: true });
        return { close: () => (stream.closed = true) };
      }
    }

    function folderOnlyConfig(documentId: string): ArtifactActorConfig {
      return { documentId, schema: "demo/v1", bindings: [{ kind: "folder", dataClass: "persistedLocalOnly", path: `/tmp/${documentId}` }], actor: "actor-1", actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT };
    }

    function notFoundResponse() {
      return { ok: false, status: 404, statusText: "not found", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
    }

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    it("poll never overlaps itself: concurrent revalidateFolder() calls collapse into one coalesced follow-up", async () => {
      installStreamMuxEndpoint(new FakeStreamEndpoint());
      let fetchCalls = 0;
      const gates: Array<() => void> = [];
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        await new Promise<void>((resolve) => gates.push(resolve));
        return notFoundResponse();
      };

      try {
        openArtifact(folderOnlyConfig("doc-overlap"));
        const state = artifactState("doc-overlap")!;
        await flushMicrotasks();
        expect(fetchCalls).toBe(1);

        const call2 = state.revalidateFolder();
        const call3 = state.revalidateFolder();
        const call4 = state.revalidateFolder();
        await flushMicrotasks();
        expect(fetchCalls).toBe(1);

        gates.shift()!();
        await flushMicrotasks();
        expect(fetchCalls).toBe(2);

        gates.shift()!();
        await Promise.all([call2, call3, call4]);
        expect(fetchCalls).toBe(2);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-overlap");
        installStreamMuxEndpoint(null);
      }
    });

    it("poll is suppressed while the folder's change stream is open and resumes once it ends", async () => {
      const endpoint = new FakeStreamEndpoint();
      installStreamMuxEndpoint(endpoint);
      let fetchCalls = 0;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        return notFoundResponse();
      };
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0);

      try {
        openArtifact(folderOnlyConfig("doc-sanity"));
        const state = artifactState("doc-sanity")!;
        await vi.advanceTimersByTimeAsync(0);
        expect(fetchCalls).toBe(1);

        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2);

        const stream = endpoint.streams.at(-1)!;
        stream.handlers.opened?.("resumed");
        expect(state.watchHealthy).toBe(true);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2);

        stream.handlers.end?.("failed", "route-removed");
        expect(state.watchHealthy).toBe(false);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sanity");
        installStreamMuxEndpoint(null);
      }
    });

    it("opens ONE backbone.folder stream per folder document: fresh opens and notices revalidate, closing cancels it", async () => {
      const endpoint = new FakeStreamEndpoint();
      installStreamMuxEndpoint(endpoint);
      let fetchCalls = 0;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        return notFoundResponse();
      };
      try {
        openArtifact(folderOnlyConfig("doc-stream"));
        await flushMicrotasks();
        expect(endpoint.streams.map((stream) => stream.route)).toEqual(["backbone.folder"]);
        expect(endpoint.streams[0]!.key.startsWith("folder://")).toBe(true);
        expect(fetchCalls).toBe(1);
        const stream = endpoint.streams[0]!;
        stream.handlers.opened?.("resumed");
        await flushMicrotasks();
        expect(fetchCalls).toBe(1);
        stream.handlers.opened?.("fresh");
        await flushMicrotasks();
        expect(fetchCalls).toBe(2);
        await stream.handlers.data?.("changed");
        expect(fetchCalls).toBe(3);
        expect(stream.closed).toBe(false);
        closeArtifact("doc-stream");
        expect(stream.closed).toBe(true);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-stream");
        installStreamMuxEndpoint(null);
      }
    });

    it("abort on close cancels an in-flight folder fetch", async () => {
      let capturedSignal: AbortSignal | undefined;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = (_url: string, init?: RequestInit) => {
        capturedSignal = init?.signal ?? undefined;
        return new Promise(() => {});
      };
      installStreamMuxEndpoint(null);

      try {
        openArtifact(folderOnlyConfig("doc-abort"));
        await Promise.resolve();
        expect(capturedSignal).toBeDefined();
        expect(capturedSignal?.aborted).toBe(false);

        closeArtifact("doc-abort");
        expect(capturedSignal?.aborted).toBe(true);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
