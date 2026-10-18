/** 📥️ The segmented download transport and actor id multiplexing tests. */
import { fixtureOriginalWorkStock } from "../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import { flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 📦 Registers the segmented download transport and the actor id multiplexing tests. */
export function registerSegmentedDownloadAndMultiplexingTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, SEGMENTED_DOWNLOAD_CONTRACT, SEGMENTED_DOWNLOAD_REFUSAL, admitSegmentedDownloadChunk, describe, expect, harness, it } = kit;
  describe("ShardClient segmented-download transport", () => {
    async function activatedHarness() {
      const state = harness(1);
      const activation = state.client.activate("actor-download", "https://x/plugin.js", [], BUDGET);
      await flushAdmission();
      const message = state.workers[0]!.sent[0] as { readonly requestId: string };
      state.workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await activation;
      return state;
    }

    it("preserves operation identity and last-Some then None ordering", async () => {
      const { client, workers } = await activatedHarness();
      for (const expected of [new Uint8Array([1, 2]), new Uint8Array([3]), undefined]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: string; readonly instanceId: number; readonly operationId: bigint };
        expect(message).toMatchObject({ kind: "takeSegmentedDownloadChunk", instanceId: 17, operationId: 91n });
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: expected });
        await expect(read).resolves.toEqual(expected);
      }
    });

    it("propagates unknown-operation errors without manufacturing a terminal None", async () => {
      const { client, workers } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 404n);
      const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: false, error: "interactive-job.unknown-segmented-download" });
      await expect(read).rejects.toThrow("interactive-job.unknown-segmented-download");
    });

    it("rejects oversized or empty response items", async () => {
      const { client, workers } = await activatedHarness();
      for (const invalid of [new Uint8Array(4_097), new Uint8Array(0)]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: invalid });
        await expect(read).rejects.toThrow(/segmented-download-chunk-(empty|over-cap)/);
      }
    });

    it("rejects an in-flight read when actor disposal cancels its transport ownership", async () => {
      const { client } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
      client.dispose("actor-download");
      await expect(read).rejects.toThrow("ShardClient actor disposed");
    });

    it("preserves the complete u64 operation authority through structured clone", async () => {
      const { client, workers } = await activatedHarness();
      for (const operationId of [(1n << 53n) + 1n, SEGMENTED_DOWNLOAD_CONTRACT.maximumOperationId]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, operationId);
        const message = structuredClone(workers[0]!.sent.at(-1)) as { readonly requestId: string; readonly operationId: bigint };
        expect(message.operationId).toBe(operationId);
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
        await expect(read).resolves.toBeUndefined();
      }
    });

    it("rejects zero and overflowing operation authorities", async () => {
      const { client } = await activatedHarness();
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 0n)).rejects.toThrow("segmented-download-authority-invalid");
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 1n << 64n)).rejects.toThrow("segmented-download-authority-invalid");
    });

    /** 📤️ THE 2026-09-12 segmented-export fault (ticket 26/09/02, battery `probe-2026-09-12T08-28-10.md`
     * on wasm #56): jco lifts `option<t>` as the TAGGED `{ tag, val }` variant — the generated bridge's
     * own `unwrapOption` docstring says exactly that — and `takeSegmentedDownloadChunk` was the ONE guest
     * export handed on raw, so the shard worker received `[object Object]`, refused chunk 0, and the
     * terminal `{ tag: "none" }` was not `undefined` either. Every host-side law of this lane stubbed the
     * worker's REPLY with an already-unwrapped `Uint8Array`, so the guest's real shape appeared in none of
     * them. Driven here through the REAL generated bridge bytes, against a stub guest module returning
     * exactly what jco's `_liftFlatVariant` builds. */
    it("unwraps the guest's tagged option in the generated bridge bytes, so no hop ever sees [object Object]", async () => {
      const { PLUGIN_HOST_SHIM_FILE } = await import("../../../🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts");
      const { pluginComponentBridgeSource } = await import("../../../🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts");
      const { APP_CHANNEL_VERSION } = await import("../../../../../🟦️.ts");
      const { mkdtempSync, writeFileSync } = await import("node:fs");
      const { tmpdir } = await import("node:os");
      const { join } = await import("node:path");
      const { pathToFileURL } = await import("node:url");
      const directory = mkdtempSync(join(tmpdir(), "semio-segmented-bridge-"));
      const chunk = new Uint8Array(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes).fill(7);
      (globalThis as Record<string, unknown>).__segmentedLifted = [{ tag: "some", val: chunk }, { tag: "none" }];
      writeFileSync(join(directory, "guest.js"), `export const reactor = { channelVersion: async () => ${APP_CHANNEL_VERSION} };\nexport const jobs = { takeSegmentedDownloadChunk: async () => globalThis.__segmentedLifted.shift() };\nexport const checkpoint = {};\nexport const describe = {};\n`);
      writeFileSync(join(directory, PLUGIN_HOST_SHIM_FILE), "export function __resolveEffect() {}\nexport function __rejectEffect() {}\nexport function __bindOriginalClock() {}\nexport function operationClockUs() { return 0n; }\n");
      writeFileSync(join(directory, "bridge.mjs"), pluginComponentBridgeSource("guest", "guest.wasm"));
      const { createActorApi } = (await import(pathToFileURL(join(directory, "bridge.mjs")).href)) as { createActorApi: (actorId: string, generation: bigint, originalAdmission: unknown, operationClockUs: () => bigint, originalReturn: { receipt(receipt: unknown): void; hold(call: unknown): void }) => Promise<{ takeSegmentedDownloadChunk: (instanceId: number, operationId: bigint) => Promise<unknown> }> };
      const api = await createActorApi("segmented-actor", 1n, await fixtureOriginalWorkStock().admit(), () => 0n, { receipt() {}, hold() {} });
      const first = await api.takeSegmentedDownloadChunk(1, 91n);
      expect(Object.prototype.toString.call(first)).toBe("[object Uint8Array]");
      expect(admitSegmentedDownloadChunk(first)).toBe(first);
      await expect(api.takeSegmentedDownloadChunk(1, 91n)).resolves.toBeUndefined();
      // 🔮️ Independent oracle for the same bytes: handed on RAW, that is precisely what every hop refuses —
      // which is why the fault read as an exceeded byte cap while nothing was over any cap at all.
      expect(() => admitSegmentedDownloadChunk({ tag: "some", val: chunk })).toThrow(SEGMENTED_DOWNLOAD_REFUSAL.chunkType);
      expect(() => admitSegmentedDownloadChunk({ tag: "none" })).toThrow(SEGMENTED_DOWNLOAD_REFUSAL.chunkType);
    });

    /** 📤️ The WORKER half of the one chunk contract, driven through the generated worker's own dispatcher:
     * a 145 714 B export (Nakagin) and a full `maximumTotalBytes - 1` payload both stream chunk by chunk,
     * and each violated bound is refused by ITS OWN name — never one "limit" standing for four different
     * violations, which is what made the live battery unreadable. */
    it("streams a 145 KB and a maximumTotalBytes-1 download through the generated worker and names each violated bound", async () => {
      const vm = await import("node:vm");
      const { SEGMENTED_DOWNLOAD_CHUNK_BYTES } = await import("../../../🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts");
      const { shardWorkerSource } = await import("../../../🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts");
      expect(SEGMENTED_DOWNLOAD_CHUNK_BYTES).toBe(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes);
      const { classicShardWorkerSource } = await import("../../✂️worker-vm-source/🟦️.ts");
      const source = classicShardWorkerSource(shardWorkerSource()).replace("await import(/* @vite-ignore */ moduleUrl)", "await __import(moduleUrl)");
      let remaining = 0;
      let poisoned: unknown;
      const results: { readonly ok: boolean; readonly value?: unknown; readonly error?: string }[] = [];
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      const api = {
        poll: async () => undefined,
        takeSegmentedDownloadChunk: async () => {
          if (poisoned !== undefined) {
            const value = poisoned;
            poisoned = undefined;
            return value;
          }
          if (remaining === 0) return undefined;
          const bytes = Math.min(remaining, SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes);
          remaining -= bytes;
          return new Uint8Array(bytes).fill(3);
        },
      };
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        URL,
        __import: async () => ({ createActorApi: async () => api }),
        setInterval: () => 1,
        clearInterval: () => undefined,
        console,
        self: {
          location: { href: "https://fixture.invalid/plugin-modules/_shard/shard-worker.js" },
          postMessage: (message: { kind: string; ok?: boolean; value?: unknown; error?: string }) => {
            if (message.kind === "result") results.push({ ok: message.ok === true, value: message.value, error: message.error });
          },
          addEventListener: (_kind: string, handler: typeof dispatch) => { dispatch = handler; },
        },
      });
      new vm.Script(source).runInContext(context);
      if (!dispatch) throw new Error("Missing generated worker dispatcher");
      const send = dispatch as (event: { data: Record<string, unknown> }) => Promise<void>;
      await send({ data: { kind: "activate", requestId: "a1", actorId: "a", activationGeneration: 1n, moduleUrl: "https://fixture.invalid/a.js", assets: [] } });
      const take = async (): Promise<{ readonly ok: boolean; readonly value?: unknown; readonly error?: string }> => {
        await send({ data: { kind: "takeSegmentedDownloadChunk", requestId: `t${results.length}`, actorId: "a", instanceId: 1, operationId: 91n } });
        return results.at(-1)!;
      };
      for (const total of [145_714, SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes - 1]) {
        remaining = total;
        let drained = 0;
        let taken = 0;
        for (;;) {
          const settled = await take();
          expect(settled.ok, `${total} B download must stream without a worker fault, got ${settled.error}`).toBe(true);
          if (settled.value === undefined) break;
          taken += 1;
          const page = settled.value as Uint8Array;
          expect(Object.prototype.toString.call(page)).toBe("[object Uint8Array]");
          expect(page.byteLength).toBeLessThanOrEqual(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes);
          drained += page.byteLength;
        }
        expect(drained).toBe(total);
        expect(taken).toBe(Math.ceil(total / SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes));
        expect(taken).toBeLessThanOrEqual(SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks);
      }
      for (const [value, code] of [
        [{ tag: "some", val: new Uint8Array(4) }, SEGMENTED_DOWNLOAD_REFUSAL.chunkType],
        [new Uint8Array(0), SEGMENTED_DOWNLOAD_REFUSAL.chunkEmpty],
        [new Uint8Array(SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes + 1), SEGMENTED_DOWNLOAD_REFUSAL.chunkOverCap],
      ] as const) {
        poisoned = value;
        const settled = await take();
        expect(settled.ok).toBe(false);
        expect(settled.error).toContain(code);
      }
    });
  });
  describe("ShardClient actor-id multiplexing", () => {
    it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
      const { client, workers } = harness(1);
      const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
      const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
      await flushAdmission();
      const [msgA, msgB] = workers[0]!.sent as { readonly requestId: string }[];
      // out-of-order reply: b's activate lands before a's
      workers[0]!.deliver({ kind: "result", requestId: msgB!.requestId, ok: true, value: undefined });
      workers[0]!.deliver({ kind: "result", requestId: msgA!.requestId, ok: true, value: undefined });
      await expect(p1).resolves.toBeUndefined();
      await expect(p2).resolves.toBeUndefined();
      expect(client.shardIndexFor("a")).toBe(0);
      expect(client.shardIndexFor("b")).toBe(0);
    });

    it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
      const { client } = harness(4, { exclusiveShardCount: 1 });
      const indices = ["a", "b", "c"].map((id) => {
        void client.activate(id, "https://x/y.js", [], BUDGET);
        return client.shardIndexFor(id);
      });
      expect(new Set(indices).has(3)).toBe(false); // shard 3 reserved exclusive
      expect(indices).toEqual([0, 1, 2]);
    });
  });
}
