/** 📯 Typed-operation completion drain, delivery, ceilings and retired instances. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { TurnOutcome } from "@semio-tech/framework-kernel";
import { type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerTypedOperationCompletionTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { adaptPluginHandle, createTurnOutcomeBroadcast, decodePackValue, drainTypedOperationTurns, encodePackValue, pendingCompletionEffects, pendingTurnEffects, PLUGIN_OPERATION_DRAIN_BUDGET, yieldPluginUiContinuation } = dependencies;
  describe("typed-operation completion drain", () => {
    const idle = { tag: "idle" } as const;
    const moreWork = { tag: "moreWork" } as const;

    it("stops at the first idle settle instead of polling forever", async () => {
      const statuses: unknown[] = [moreWork, moreWork, idle, moreWork];
      let polls = 0;
      const outcome = await drainTypedOperationTurns(PLUGIN_OPERATION_DRAIN_BUDGET, () => true, async () => ({ status: statuses[polls++], nextWake: null }), async () => {});
      expect(outcome).toEqual({ polls: 3, stopped: "idle", nextWake: null });
      expect(polls).toBe(3);
    });

    it("never exceeds its poll budget when the actor never goes idle", async () => {
      let polls = 0;
      const outcome = await drainTypedOperationTurns(8, () => true, async () => { polls += 1; return { status: moreWork, nextWake: null }; }, async () => {});
      expect(outcome).toEqual({ polls: 8, stopped: "budget", nextWake: null });
      expect(polls).toBe(8);
    });

    it("stops the moment its instance stops being live, without settling again", async () => {
      let polls = 0;
      const outcome = await drainTypedOperationTurns(16, () => polls < 2, async () => { polls += 1; return { status: moreWork, nextWake: null }; }, async () => {});
      expect(outcome).toEqual({ polls: 2, stopped: "closed", nextWake: null });
      expect(polls).toBe(2);
    });

    it("surfaces the last settle's next-wake so an idle actor can be re-armed", async () => {
      const outcome = await drainTypedOperationTurns(4, () => true, async () => ({ status: idle, nextWake: 42 }), async () => {});
      expect(outcome).toEqual({ polls: 1, stopped: "idle", nextWake: 42 });
    });

    /** ⏱️ Counts every wall-clock primitive a drain could reach for, so "no backoff" is asserted rather
     * than read off the source. `requestAnimationFrame`/`requestIdleCallback` are in the list because
     * neither fires in a hidden tab: evaluation must keep converging when nobody is looking. */
    const countTimerPrimitives = <T,>(run: () => Promise<T>): Promise<{ result: T; timers: number; elapsedMs: number }> => {
      const host = globalThis as unknown as Record<string, unknown>;
      const names = ["setTimeout", "setInterval", "requestAnimationFrame", "requestIdleCallback"] as const;
      const originals = names.map((name) => [name, host[name]] as const);
      let timers = 0;
      for (const [name, original] of originals) {
        if (typeof original !== "function") continue;
        host[name] = (...args: unknown[]) => { timers += 1; return (original as (...rest: unknown[]) => unknown)(...args); };
      }
      const started = Date.now();
      return run().then(
        (result) => { for (const [name, original] of originals) host[name] = original; return { result, timers, elapsedMs: Date.now() - started }; },
        (error) => { for (const [name, original] of originals) host[name] = original; throw error; },
      );
    };

    it("re-polls every more-work answer on a continuation, never on a wall clock", async () => {
      const answers = 64;
      let polls = 0;
      const { result, timers, elapsedMs } = await countTimerPrimitives(async () =>
        drainTypedOperationTurns(answers + 8, () => true, async () => { polls += 1; return { status: polls < answers ? moreWork : idle, nextWake: null }; }, yieldPluginUiContinuation));
      expect(result).toEqual({ polls: answers, stopped: "idle", nextWake: null });
      expect(polls).toBe(answers);
      expect(timers).toBe(0);
      expect(elapsedMs).toBeLessThan(1_000);
    });

    it("yields the thread to a contended peer between polls instead of sleeping on it", async () => {
      let polls = 0;
      let peerTurns = 0;
      let peerLive = true;
      const peer = (async () => { while (peerLive) { peerTurns += 1; await yieldPluginUiContinuation(); } })();
      const { result, timers, elapsedMs } = await countTimerPrimitives(async () =>
        drainTypedOperationTurns(48, () => true, async () => { polls += 1; return { status: polls < 32 ? moreWork : idle, nextWake: null }; }, yieldPluginUiContinuation));
      peerLive = false;
      await peer;
      expect(result.stopped).toBe("idle");
      expect(result.polls).toBe(32);
      expect(peerTurns).toBeGreaterThanOrEqual(result.polls);
      expect(timers).toBe(0);
      expect(elapsedMs).toBeLessThan(1_000);
    });
  });

  describe("typed-operation completion delivery", () => {
    it("hands one completion its own effects exactly once and never the invocation's", async () => {
      const instanceId = 4242;
      let push: ((outcome: TurnOutcome) => void) | null = null;
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      push = (outcome) => broadcast.push(outcome);
      const handle = {
        manifest: { id: "fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
        createApp: async () => instanceId,
        destroyApp: async () => {},
        takeSegmentedDownloadChunk: async () => undefined,
        enqueue: () => {},
        outcomes: broadcast.stream,
        dispose: async () => {},
      } as unknown as Parameters<typeof adaptPluginHandle>[1]["handle"];
      const adapted = await adaptPluginHandle("fixture", { handle, release: async () => {} });
      await adapted.createApp("app.demo");
      const seen: unknown[] = [];
      const alsoSeen: unknown[] = [];
      const unsubscribe = adapted.subscribeOperationCompletions(instanceId, (completion) => seen.push(completion));
      // 🏁️ A SECOND subscriber on the SAME instance is the real shell (the visible session's completion
      // pass and Home's directory-bootstrap receipt settle both bind `session.instanceId`). One channel
      // registration fans out to both: two registrations would race for the leftover map, and whichever
      // drained it first would leave the other a completion with no effects and no terminal output.
      const unsubscribeSecond = adapted.subscribeOperationCompletions(instanceId, (completion) => alsoSeen.push(completion));
      pendingCompletionEffects.set(instanceId, [
        { tag: "notify", val: { message: "done" } },
        { tag: "typed-operation-terminal-output", val: { schema: "semio.space.home.directory-projection-receipt.v1", throughSeqInclusive: 8 } },
      ]);
      pendingTurnEffects.set(instanceId, [{ tag: "notify", val: { message: "invocation" } }]);
      push({ instanceId, frames: [encodeAppFrame({ OperationCompleted: { operation: 9n, revision: 3n, ui_scope: [], history_patch: Array.from(encodePackValue({ cursor: 2, upserts: [] })) } })] });
      for (let tick = 0; tick < 8 && seen.length === 0; tick += 1) await Promise.resolve();
      expect(seen).toHaveLength(1);
      expect(seen[0]).toMatchObject({ instanceId, operation: 9n, revision: 3n, requestedEffects: [{ notify: { message: "done" } }] });
      // 🧾️ The terminal value the operation published is the completion's own carrier — a job-routed
      // verb never drains it inside a host call, so without this field it was decoded and then dropped.
      expect(seen[0]).toMatchObject({ terminalOutput: { schema: "semio.space.home.directory-projection-receipt.v1", throughSeqInclusive: 8 } });
      expect(alsoSeen).toEqual(seen);
      expect(pendingCompletionEffects.has(instanceId)).toBe(false);
      expect(pendingTurnEffects.get(instanceId)).toEqual([{ tag: "notify", val: { message: "invocation" } }]);
      unsubscribe();
      unsubscribeSecond();
      pendingTurnEffects.delete(instanceId);
    });
  });

  it("bounds one command's drain by a ceiling derived from its own page count, never by a chosen number", async () => {
    const { commandIngressContinuationCeilingV1 } = dependencies;
    const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/command-ingress-drain.json");
    for (const row of fixture.ceilings) {
      expect(commandIngressContinuationCeilingV1(row.pages), row.id).toBe(row.ceiling);
      expect(commandIngressContinuationCeilingV1(row.pages), `${row.id} is the derivation, not a literal`).toBe(Math.max(1, row.pages) * fixture.turnsPerPage * fixture.continuationBatchSize);
    }
    expect(commandIngressContinuationCeilingV1(2), "a two-page command may never buy less than a one-page command").toBeGreaterThan(commandIngressContinuationCeilingV1(1));
  });

  it("calls a command unowned on the FIRST idle turn instead of spending a whole ceiling on it", async () => {
    const { commandIngressUnownedV1, commandIngressContinuationCeilingV1 } = dependencies;
    const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/command-ingress-drain.json");
    for (const row of fixture.unowned) {
      expect(commandIngressUnownedV1(row.ingress ?? undefined, row.actor ?? undefined), row.id).toBe(row.unowned);
    }
    // 🧾️ The drain's own loop, replayed: a reactor that owns nothing answers `idle` forever, and the
    // shape this law pins is that the host asks exactly ONCE more, not `ceiling` times.
    const ceiling = commandIngressContinuationCeilingV1(1);
    let asked = 0;
    let terminal: string | undefined = "idle";
    let actor: string | undefined = "idle";
    for (let continuation = 0; terminal !== "command-complete" && continuation < ceiling; continuation += 1) {
      if (commandIngressUnownedV1(terminal, actor)) break;
      asked += 1;
      terminal = "idle";
      actor = "idle";
    }
    expect(asked, "an unowned command costs no further crossing at all").toBe(0);
  });

  it("refuses a retired instance by REJECTION, never by a synchronous throw past the caller's catch", async () => {
    const { adaptPluginHandle, encodePackValue, isPluginInstanceRetiredV1 } = dependencies;
    const lease = {
      handle: {
        manifest: { pluginId: "reject-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
        createApp: async () => 55,
        destroyApp: async () => {},
        enqueue: () => {},
        takeSegmentedDownloadChunk: async () => undefined,
        outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { return { next: () => new Promise(() => {}), return: async () => ({ done: true, value: undefined }) }; } },
        dispose: async () => {},
      },
      release: async () => {},
    };
    const handle = await adaptPluginHandle("reject-fixture", lease);
    const instance = await handle.createApp("fixture");
    await handle.destroyApp(instance);

    // 🧯️ The shape `ShellHost.onAction` uses: build the promise, attach `.catch`, never wrap in try.
    // A synchronous throw escapes this entirely and reaches React's error boundary.
    for (const [what, call] of [
      ["handleAction", () => handle.handleAction(instance, JSON.stringify({ controllerId: "c", action: "a" }), {} as never)],
      ["handleCommand", () => handle.handleCommand!(instance, JSON.stringify({ controllerId: "c", action: "a" }), {} as never)],
      ["contextMenu", () => handle.contextMenu!(instance, { surfaceId: "s" } as never, {} as never)],
    ] as const) {
      let synchronous: unknown = null;
      let rejected: unknown = null;
      try {
        await call().catch((error: unknown) => { rejected = error; });
      } catch (error) {
        synchronous = error;
      }
      expect(synchronous, `${what} must not throw synchronously past the caller's catch`).toBe(null);
      expect(isPluginInstanceRetiredV1(rejected), `${what} rejects with the typed retirement`).toBe(true);
    }
  });

  it("remembers a destroyed instance across a hot swap that replaces the handle", async () => {
    const { adaptPluginHandle, encodePackValue, isPluginInstanceRetiredV1, pluginInstanceWasRetiredV1 } = dependencies;
    const makeLease = (nextInstance: number) => ({
      handle: {
        manifest: { pluginId: "swap-fixture", apps: [] } as unknown as import("@semio-tech/framework").PluginManifest,
        createApp: async () => nextInstance,
        destroyApp: async () => {},
        enqueue: () => {},
        takeSegmentedDownloadChunk: async () => undefined,
        outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { return { next: () => new Promise(() => {}), return: async () => ({ done: true, value: undefined }) }; } },
        dispose: async () => {},
      },
      release: async () => {},
    });
    const before = await adaptPluginHandle("swap-fixture", makeLease(41));
    const instance = await before.createApp("fixture");
    await before.destroyApp(instance);
    expect(pluginInstanceWasRetiredV1("swap-fixture", instance), "the handle that destroyed it says so").toBe(true);

    const after = await adaptPluginHandle("swap-fixture", makeLease(42));
    let raised: unknown = null;
    try { await after.readHistory(instance); } catch (error) { raised = error; }
    expect(String(raised)).toContain(`no channel for instance ${instance}`);
    expect(isPluginInstanceRetiredV1(raised), "the REPLACEMENT handle still knows the instance was retired, so a late lane drops instead of throwing").toBe(true);

    let foreign: unknown = null;
    try { await after.readHistory(9_001); } catch (error) { foreign = error; }
    expect(isPluginInstanceRetiredV1(foreign), "and an id nothing ever created stays a loud failure").toBe(false);

    const reviving = await adaptPluginHandle("swap-fixture", makeLease(instance));
    const revived = await reviving.createApp("fixture");
    expect(revived, "the fixture mints the same id again").toBe(instance);
    expect(pluginInstanceWasRetiredV1("swap-fixture", revived), "a live instance under that id unsays the retirement").toBe(false);
  });

  it("keeps reserved command ingress Interactive ahead of catalog Background and stamps a reply when none arrives", async () => {
    const { commandIngressLaneForActionV1, commandIngressNeedsReplyStampV1 } = dependencies;
    const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧪️tests/🧫️fixtures/command-ingress-lane.json");
    for (const row of fixture.lanes) {
      expect(commandIngressLaneForActionV1(row.actionId), row.actionId).toBe(row.lane);
    }
    for (const row of fixture.stamps) {
      expect(commandIngressNeedsReplyStampV1(row.replySequences, row.seq), row.id).toBe(row.stamp);
    }
  });

  it("packs completion-result.fault the same way the guest decodes it", async () => {
    const { default: fixture } = await import("@semio-tech/framework-plugin/reactor/testing/fixtures/extension-result-fault-pack.json");
    const packed = encodePackValue(fixture.fault);
    expect(() => JSON.parse(new TextDecoder().decode(packed))).toThrow();
    const decoded = decodePackValue(packed) as { code: string; message: string };
    expect(decoded.code).toBe("extension.missing");
    expect(decoded.message).toBe("no such extension");
  });
}
