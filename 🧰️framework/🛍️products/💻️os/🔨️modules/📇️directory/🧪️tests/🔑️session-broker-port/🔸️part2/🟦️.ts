/** 🔸️ The part2 phase of registerSessionBrokerPortTests. */
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { WORKER_LAW_CAPABILITY, WORKER_LAW_CAPABILITY_TWO } from "../../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { useBeforeState } from "../🔶️before/🟦️.ts";
import type { Vitest } from "../🟢️shared/🟦️.ts";

/** 🔹 Runs the effects and registrations of registerSessionBrokerPortTests. */
export function buildPart2({ vitest, resetWorkerLaw, source, clearHubSessionCapability, testSeams }: { readonly vitest: Vitest; readonly source: TestSource } & Pick<ReturnType<typeof useBeforeState>, "resetWorkerLaw" | "clearHubSessionCapability" | "testSeams">) {
  vitest.it("closes the private broker proof before or during a session-authority read", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { setImmediate: immediate } = await import("node:timers/promises");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-port/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const { HubSessionPortClientV1 } = await import("@semio-tech/framework/directory/session-refresh/session-port");
    const originalFetch = globalThis.fetch;
    try {
      for (const row of fixture.close.cases) {
        clearHubSessionCapability();
        const channel = new MessageChannel();
        testSeams.attachHubSessionPort(channel.port2);
        let readStarted = false;
        let requestSignal: AbortSignal | undefined;
        let resolveStarted!: () => void;
        const started = new Promise<void>((resolve) => { resolveStarted = resolve; });
        globalThis.fetch = stubFetch(vitest.vi.fn(async (_input: RequestInfo | URL, init?: RequestInit) => {
          readStarted = true;
          requestSignal = init?.signal ?? undefined;
          resolveStarted();
          return new Promise<Response>((_resolve, reject) => requestSignal?.addEventListener("abort", () => reject(new Error("cancelled")), { once: true }));
        }));
        const client = new HubSessionPortClientV1(channel.port1, row.id === "before-read" ? WORKER_LAW_CAPABILITY : WORKER_LAW_CAPABILITY_TWO);
        try {
          if (row.id === "before-read") {
            client.close();
            await vitest.expect(client.me()).rejects.toThrow("hub session unavailable");
          } else {
            await immediate();
            const pending = client.me();
            await started;
            client.close();
            await vitest.expect(pending).rejects.toThrow("hub session closed");
          }
          await immediate();
          await immediate();
          vitest.expect(equal({ id: row.id, readStarted }, row)).toBe(true);
          vitest.expect(testSeams.browserSessionAuthority).toBe(null);
          if (row.readStarted) vitest.expect(requestSignal?.aborted).toBe(true);
        } finally {
          client.close();
          testSeams.detachHubSessionPort();
          channel.port1.close();
          channel.port2.close();
        }
      }
    } finally {
      clearHubSessionCapability();
      globalThis.fetch = originalFetch;
    }
  });
}
