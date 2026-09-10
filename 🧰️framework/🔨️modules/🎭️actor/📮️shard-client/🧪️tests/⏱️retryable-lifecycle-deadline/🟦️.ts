type TestSource = { readonly directory: string; readonly url: string };

/** ⏱️ LAW: a RETRYABLE `plugin.reactor-turn-deadline` on a turn carrying at most one lifecycle event
 * is a YIELD the client replays — never a worker fault, and never `onActorTrap`. Driven from the very
 * fixture the Rust host law reads (`🔌️plugin/🖥️host/🔁️lifecycle/🧫️fixtures/🔣️.json`), so the browser
 * verdict and `retryable_lifecycle_turn`'s verdict can never diverge.
 * @see 🔌️plugin/🖥️host/🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs */
export async function registerRetryableLifecycleDeadlineTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: import("../../🟦️.ts").ShardClientRetryableLifecycleTestDependenciesV1,
  testSource: TestSource,
): Promise<void> {
  const { RETRYABLE_LIFECYCLE_TURN_ATTEMPTS, graftWorkerStack, isRetryableLifecycleTurn } = dependencies;
  const { describe, it, expect } = vitest;

  const lifecycleEventEnvelope = (kind: string): Record<string, unknown> | null => {
    switch (kind) {
      case "open":
        return { kind: "instance-open", payload: { instance: 7, activationGeneration: 41n, requestSequence: 8, appId: "s.test.synthetic@1/*#editor", actor: "fixture", config: [], assets: [], capabilities: [], quotas: {} } };
      case "close":
        return { kind: "instance-close", payload: { kind: "close", lifetime: { activationGeneration: 41n, instanceId: 7, guestLifetime: 3n }, requestSequence: 9 } };
      case "ack":
        return { kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: { kind: "captured", lifetime: { activationGeneration: 41n, instanceId: 7, guestLifetime: 3n }, requestSequence: 8 } } };
      case "wake":
        return { kind: "wake" };
      default:
        return null;
    }
  };

  describe("RetryableLifecycleTurnDeadline", () => {
    it("classifies every host-fixture case identically in the generated worker and never traps an eligible one", async () => {
      const { default: fixture } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️lifecycle/🧫️fixtures/🔣️.json");
      const vm = await import("node:vm");
      const { shardWorkerSource } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
      const posted: Record<string, unknown>[] = [];
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      let fault: unknown = null;
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        TextDecoder,
        console: { log: () => {}, error: () => {} },
        self: { postMessage: (message: Record<string, unknown>) => posted.push(message), addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } },
        api: { poll: async () => { throw fault; } },
      });
      new vm.Script(shardWorkerSource()).runInContext(context);
      new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(context);
      const send = dispatch as unknown as ((event: { data: Record<string, unknown> }) => Promise<void>) | null;
      if (!send) throw new Error("Missing generated worker dispatcher");
      const observed: string[] = [];
      for (const [index, row] of fixture.cases.entries()) {
        posted.length = 0;
        fault = { code: row.code, message: "retained lifecycle fixture", origin: "framework", retryable: row.retryable };
        const events = row.events.map(lifecycleEventEnvelope).filter((event): event is Record<string, unknown> => event !== null);
        const commandPage = row.events.includes("page") ? { cursor: {}, bytes: new Uint8Array() } : undefined;
        await send({ data: { kind: "turn", requestId: `r${index}`, actorId: "a", activationGeneration: 1n, events, commandPage, budget: {} } });
        const result = posted.find((message) => message.kind === "result" && message.requestId === `r${index}`);
        const trapped = posted.some((message) => message.kind === "worker-fault");
        expect(result, row.id).toBeTruthy();
        expect(result?.ok, row.id).toBe(false);
        expect(result?.retryableLifecycle, row.id).toBe(row.eligible);
        expect(trapped, row.id).toBe(!row.eligible);
        observed.push(row.id);
      }
      expect(observed).toEqual(fixture.cases.map((row) => row.id));
      console.log(`[DEBUG] retryable lifecycle deadline: generated-worker verdicts replayed cases=${observed.length}`);
    });

    it("carries the retryable verdict onto the grafted main-thread rejection and nowhere else", () => {
      const message = "guest lifecycle turn exceeded strict time authority; receipt retained";
      const marked = graftWorkerStack("procedural#1", message, undefined, "Object", 0, true);
      const plain = graftWorkerStack("procedural#1", message, undefined, "Object", 0, false);
      expect(isRetryableLifecycleTurn(marked)).toBe(true);
      expect(isRetryableLifecycleTurn(plain)).toBe(false);
      expect(isRetryableLifecycleTurn(graftWorkerStack("procedural#1", "other", undefined, undefined, undefined))).toBe(false);
      expect(isRetryableLifecycleTurn(new Error("other"))).toBe(false);
      expect(isRetryableLifecycleTurn(null)).toBe(false);
      expect(JSON.stringify({ ...marked })).toBe("{}");
      expect(RETRYABLE_LIFECYCLE_TURN_ATTEMPTS).toBeGreaterThan(1);
    });

    it("replays the exact same lifecycle events on a marked rejection, bounded by the shared attempt budget", async () => {
      const { readFileSync } = await import("node:fs");
      const source = readFileSync(new URL(testSource.url), "utf8");
      const start = source.indexOf("private async sendInstanceLifecycle(");
      expect(start).toBeGreaterThan(0);
      const body = source.slice(start, source.indexOf("\n  private recordInstanceTurn(", start));
      expect(body).toContain("isRetryableLifecycleTurn(error)");
      expect(body).toContain("RETRYABLE_LIFECYCLE_TURN_ATTEMPTS");
      expect(body.indexOf("isRetryableLifecycleTurn(error)")).toBeLessThan(body.indexOf("this.recordInstanceTurn(owner, result)"));
    });
  });
}
