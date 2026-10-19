/** 🛰 Framework plugin runtime contract of the renderer. */
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { describe, expect, it } from "vitest";
import { type UiNodeRecord, createMemoryStoragePort, createTurnOutcomeBroadcast, type TurnOutcome } from "@semio-tech/framework";
import { adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime, applyUiPatchToRetained, decodeWirePatchOps, serializeCommandIngressForActor, serializePerActor, initialShellState, selectOpenConflicts, selectQuarantinedConflicts, shellReducer } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework plugin runtime", () => {
  function exchangeStyleChannel(respond: (instanceId: number, frames: Uint8Array[]) => Uint8Array[] | Promise<Uint8Array[]>): {
    readonly enqueue: (instanceId: number, events: readonly Uint8Array[]) => void;
    readonly outcomes: AsyncIterable<TurnOutcome>;
  } {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    return {
      enqueue: (instanceId, events) => {
        void (async () => {
          try {
            const frames = await respond(instanceId, [...events]);
            broadcast.push({ instanceId, frames });
          } catch (error) {
            broadcast.push({ instanceId, error });
          }
        })();
      },
      outcomes: broadcast.stream,
    };
  }

  it("adaptPluginHandle's own refreshUi is an honest empty result — window-body refresh now lives in loadPluginModule's ActivationRegistry/ShardClient turn loop, which a bare no-command handle has no access to", async () => {
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-refresh", label: "Mock Refresh", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 7,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel(() => {
        throw new Error("adaptPluginHandle.refreshUi must not call enqueue() — there is no AppCommand for it anymore");
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-refresh", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    await expect(handle.refreshUi(instanceId, { viewState: {}, windows: [{ key: "overview", bodyKey: "overview" }] })).resolves.toEqual({});
  });

  it("fetchDescriptorManifest refuses a missing descriptor and surfaces a published one", async () => {
    const originalFetch = globalThis.fetch;
    try {
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 404 }));
      await expect(fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js")).rejects.toThrow("plugin.descriptor-unavailable");

      globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] } }), { headers: { "content-type": "application/json" } }));
      const real = await fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js");
      expect(real).toEqual({ pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] });
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("resolves the descriptor before initializing the shard runtime", async () => {
    const order: string[] = [];
    const resolved = await resolveDescriptorBeforeRuntime(
      async () => {
        order.push("descriptor-start");
        await Promise.resolve();
        order.push("descriptor-ready");
        return "manifest";
      },
      () => {
        order.push("runtime");
        return "shards";
      },
    );
    expect(order).toEqual(["descriptor-start", "descriptor-ready", "runtime"]);
    expect(resolved).toEqual({ manifest: "manifest", runtime: "shards" });
  });

  describe("applyUiPatchToRetained", () => {
    const leaf = (id: number, value: string): UiNodeRecord => ({
      id,
      key: `leaf-${id}`,
      component: { type: "text", value, emphasize: null, dataAttributes: null },
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
      bindings: [],
      menu: null,
      children: [],
    });

    it("applies the semantic upsert and set-root operations emitted by the actor WIT boundary", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: 0, key: "leaf-0", component: leaf(0, "a").component, layout: leaf(0, "a").layout, style: {}, activity: "idle", accessibility: {} })),
          },
        },
        { tag: "set-root", val: 0n },
      ]);
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(1);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "a", emphasize: null, dataAttributes: null });
      expect(result.surface?.nodes.get(0)?.children).toEqual([]);
    });

    it("projects lossless pack integer carriers onto exact node ids, children and revisions", async () => {
      const { encodePackValue, packUInt } = await import("@semio-tech/framework-os");
      const ops = decodeWirePatchOps([
        {
          tag: "upsert",
          val: {
            node: Array.from(encodePackValue({ id: packUInt(1n), key: "leaf-1", component: leaf(1, "a").component, layout: leaf(1, "a").layout, style: {}, activity: "idle", accessibility: {}, children: [packUInt(3n)] })),
          },
        },
        { tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(3n), key: "leaf-3", component: leaf(3, "b").component, layout: leaf(3, "b").layout, style: {}, activity: "idle", accessibility: {} })) } },
        { tag: "set-children", val: { node: 1n, children: [3n] } },
        { tag: "set-root", val: 1n },
      ]);
      expect(ops.map((op) => op.type)).toEqual(["upsert", "upsert", "setChildren", "setRoot"]);
      expect(ops[0]).toMatchObject({ type: "upsert", id: 1, children: [3] });
      expect(ops[1]).toMatchObject({ type: "upsert", id: 3 });
      expect(ops[3]).toEqual({ type: "setRoot", id: 1 });
      const result = applyUiPatchToRetained(null, { surface: "s", revision: 1n, baseRevision: 0n, ops });
      expect(result.desynced).toBe(false);
      expect(result.surface?.nodes.get(1)?.children).toEqual([3]);
      expect(result.surface?.nodes.get(3)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
      expect(() => decodeWirePatchOps([{ tag: "upsert", val: { node: Array.from(encodePackValue({ id: packUInt(2n ** 60n), key: "huge" })) } }])).toThrow(/upsert\.node\.id/u);
    });

    it("applies incremental semantic field updates with a matching base revision", () => {
      const first = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(first.surface, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(false);
      expect(result.surface?.revision).toBe(2);
      expect(result.surface?.nodes.get(0)?.component).toEqual({ type: "text", value: "b", emphasize: null, dataAttributes: null });
    });

    it("keeps the previous body when a semantic patch has a stale base revision", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 0, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "b", emphasize: null, dataAttributes: null } }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });

    it("keeps the previous body when a patch violates the retained document graph", () => {
      const { surface: previous } = applyUiPatchToRetained(null, {
        surface: "s",
        revision: 1,
        baseRevision: 0,
        ops: [
          { type: "upsert", ...leaf(0, "a") },
          { type: "setRoot", id: 0 },
        ],
      });
      const result = applyUiPatchToRetained(previous, { surface: "s", revision: 2, baseRevision: 1, ops: [{ type: "setChildren", id: 0, children: [99] }] });
      expect(result.desynced).toBe(true);
      expect(result.surface).toBe(previous);
    });
  });

  it("parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    const response = parseInvocationResponse(
      JSON.stringify({
        output: null,
        mutations: [{ diff: { payload: { schemaId: "draw.operation", document: { id: "forest" } } } }],
        inverseGroup: { invocationId: "setActiveExample:1:0", mutations: [], inverseMutations: [] },
        requestedEffects: [{ navigate: { uri: "/spaces/forest" } }],
      }),
    );
    expect(response.mutations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/spaces/forest" } }]);
  });

  it("falls back to an empty InvocationResponse for malformed handle-action JSON", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework");
    expect(parseInvocationResponse("not json")).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
    expect(parseInvocationResponse(JSON.stringify({ output: null }))).toEqual({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } });
  });

  it("serializePerActor queues concurrent turns for the same actor one at a time, never overlapping", async () => {
    let inFlight = 0;
    let maxInFlight = 0;
    const runOne = () =>
      serializePerActor("actor-1", async () => {
        inFlight += 1;
        maxInFlight = Math.max(maxInFlight, inFlight);
        await new Promise((resolve) => setTimeout(resolve, 5));
        inFlight -= 1;
        return "done";
      });
    const results = await Promise.all([runOne(), runOne(), runOne()]);
    expect(maxInFlight).toBe(1);
    expect(results).toEqual(["done", "done", "done"]);
  });

  it("serializePerActor keys independently per actor — different actors run concurrently, not queued behind each other", async () => {
    let concurrentAcrossActors = 0;
    let maxConcurrentAcrossActors = 0;
    const runOn = (actorId: string) =>
      serializePerActor(actorId, async () => {
        concurrentAcrossActors += 1;
        maxConcurrentAcrossActors = Math.max(maxConcurrentAcrossActors, concurrentAcrossActors);
        await new Promise((resolve) => setTimeout(resolve, 5));
        concurrentAcrossActors -= 1;
      });
    await Promise.all([runOn("actor-a"), runOn("actor-b")]);
    expect(maxConcurrentAcrossActors).toBe(2);
  });

  it("serializePerActor keeps queuing subsequent turns after an earlier one rejects", async () => {
    const order: string[] = [];
    const failing = serializePerActor("actor-2", async () => {
      order.push("first");
      throw new Error("turn faulted");
    });
    const succeeding = serializePerActor("actor-2", async () => {
      order.push("second");
      return "ok";
    });
    await expect(failing).rejects.toThrow("turn faulted");
    await expect(succeeding).resolves.toBe("ok");
    expect(order).toEqual(["first", "second"]);
  });

  it("serializes complete multi-turn command ingress sequences for one actor", async () => {
    const order: string[] = [];
    let releaseFirst!: () => void;
    let markFirstStarted!: () => void;
    const firstStarted = new Promise<void>((resolve) => {
      markFirstStarted = resolve;
    });
    const firstGate = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const first = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("first-page");
      markFirstStarted();
      await firstGate;
      order.push("first-terminal");
    });
    await firstStarted;
    const second = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("second-page");
      await Promise.resolve();
      order.push("second-terminal");
    });
    await Promise.resolve();
    expect(order).toEqual(["first-page"]);
    releaseFirst();
    await Promise.all([first, second]);
    expect(order).toEqual(["first-page", "first-terminal", "second-page", "second-terminal"]);
  });

  it("serializeCommandIngressForActor dequeues queued calls by ascending order — a later call with a smaller order overtakes a larger one behind the held call", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-causal-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const callA = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("A");
      },
      "Interactive",
      5,
    );
    const callB = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("B");
      },
      "Interactive",
      7,
    );
    const callC = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("C");
      },
      "Interactive",
      6,
    );
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, callA, callB, callC]);
    expect(ran).toEqual(["held", "A", "C", "B"]);
  });

  it("serializeCommandIngressForActor keeps plain FIFO for calls without order", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const first = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("first");
    });
    const second = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("second");
    });
    const third = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("third");
    });
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, first, second, third]);
    expect(ran).toEqual(["held", "first", "second", "third"]);
  });

  it("adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, decodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-action", label: "Mock Action", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 3,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("Command" in command)) throw new Error("expected a Command");
        const invocation = decodePackValue(new Uint8Array(command.Command.command));
        return [
          encodeAppFrame({
            Invocation: {
              in_reply_to: command.Command.seq,
              output: Array.from(encodePackValue({ echo: invocation })),
              diagnostics: Array.from(encodePackValue([])),
              ui_scope: Array.from(encodePackValue({ kind: "partial", windowBodies: ["graph"], utilities: false })),
              history_patch: Array.from(encodePackValue({ cursor: 1, upserts: [] })),
              messages: [],
              mutations: Array.from(encodePackValue([])),
              inverse_group: Array.from(encodePackValue({ invocationId: "", mutations: [], inverseMutations: [] })),
            },
          }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-action", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    const invocation = { address: { pluginId: "mock-action", appId: "main", modeId: "edit", windowKindId: "main", windowInstanceId: "main", actionId: "addShot" }, arguments: { format: "png" } };
    const response = await handle.handleAction(instanceId, JSON.stringify(invocation), { locale: "en", terminology: "native" });
    expect(response.output).toEqual({ echo: invocation });
    expect(response.requestedEffects).toEqual([]);
    expect(response.uiScope).toEqual({ kind: "partial", windowBodies: ["graph"], utilities: false });
    expect(response.historyPatch).toEqual({ cursor: 1, upserts: [] });
  });

  it("adaptPluginHandle exposes setMergePolicy/resolveConflict/readConflicts and sends the real AppCommand wire frames — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane K2: the merge-policy Settings control and Conflicts panel Accept/Discard used to call `plugin.handle.setMergePolicy?.(…)` against a handle that never had the method, so the optional call silently no-opped — this asserts the method genuinely exists AND that calling it round-trips through the real `AppCommand`/`AppFrame` codecs, not an internal spy", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue } = await import("@semio-tech/framework-os");
    const sentCommands: unknown[] = [];
    const fakeHandle = {
      manifest: { pluginId: "mock-merge", label: "Mock Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 9,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        sentCommands.push(command);
        if (command && typeof command === "object" && "setMergePolicy" in command) return [encodeAppFrame({ Done: { in_reply_to: command.setMergePolicy.seq } })];
        if (command && typeof command === "object" && "resolveConflict" in command) {
          const seq = command.resolveConflict.seq;
          return [
            encodeAppFrame({ MergeReport: { in_reply_to: seq, report: Array.from(encodePackValue({ policy: "Normal", accepted: true, insertionIndex: 0, replayed: [], worst: null, conflict: null })) } }),
            encodeAppFrame({ Conflicts: { in_reply_to: seq, conflicts: Array.from(encodePackValue([])) } }),
          ];
        }
        if (command && typeof command === "object" && "readConflicts" in command) {
          return [encodeAppFrame({ Conflicts: { in_reply_to: command.readConflicts.seq, conflicts: Array.from(encodePackValue([])) } })];
        }
        throw new Error(`unexpected command ${JSON.stringify(command)}`);
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    expect(typeof handle.setMergePolicy).toBe("function");
    expect(typeof handle.resolveConflict).toBe("function");
    expect(typeof handle.readConflicts).toBe("function");
    const instanceId = await handle.createApp("main");

    await handle.setMergePolicy(instanceId, "Vigilant");
    expect(sentCommands[0]).toEqual({ setMergePolicy: { seq: 1, policy: 2 } });

    const resolved = await handle.resolveConflict(instanceId, "conflict-abc", "accept");
    expect(sentCommands[1]).toEqual({ resolveConflict: { seq: 2, conflict_id: "conflict-abc", resolution: 0 } });
    expect(resolved.mergeReport?.accepted).toBe(true);
    expect(resolved.conflicts).toEqual([]);

    await handle.readConflicts(instanceId);
    expect(sentCommands[2]).toEqual({ readConflicts: { seq: 3 } });
  });

  it("adaptPluginHandle.applyMutations decodes an unsolicited MergeReport/Conflicts reply and it reaches ShellState — ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS lane L1 gap 1: a peer's ApplyEnvelopes ingest batches MergeReport/Conflicts frames alongside it (contract freeze §C6/§C9 'pushed unsolicited after every ingest'), but `applyMutations` used to only look for an Error frame and silently drop everything else — this asserts the guest's real roster survives the decode AND that dispatching it through `shellReducer`'s SET_CONFLICTS (what ShellHost's `applyRemoteMerge` does) lands a remote-origin quarantined conflict in both `selectOpenConflicts` and `selectQuarantinedConflicts`, exactly the panel/badge lane K2 wired", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, encodeMutationEnvelopesPack } = await import("@semio-tech/framework-os");
    const remoteConflict = {
      id: "conflict-remote-1",
      kind: { kind: "quarantined", envelopes: [] },
      status: "open",
      messages: [{ level: "error", code: "mutation.target-missing", message: "peer deleted the renamed node" }],
      actors: ["peer-actor"],
      timestamp: { actor: 7, physical_ms: 1000, logical: 1 },
    };
    const fakeHandle = {
      manifest: { pluginId: "mock-remote-merge", label: "Mock Remote Merge", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 11,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("ApplyEnvelopes" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
        const seq = command.ApplyEnvelopes.seq;
        return [
          encodeAppFrame({ Done: { in_reply_to: seq } }),
          encodeAppFrame({ MergeReport: { in_reply_to: null, report: Array.from(encodePackValue({ policy: "Normal", accepted: false, insertionIndex: 0, replayed: [], worst: "error", conflict: remoteConflict.id })) } }),
          encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: Array.from(encodePackValue([remoteConflict])) } }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-remote-merge", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");

    if (!handle.applyMutations) throw new Error("Mutation fixture requires its owned mutation handler");
    const result = await handle.applyMutations(instanceId, encodeMutationEnvelopesPack([]));
    expect(result.mergeReport?.accepted).toBe(false);
    expect(result.mergeReport?.worst).toBe("error");
    expect(result.conflicts).toEqual([remoteConflict]);

    const state = shellReducer(initialShellState({ locale: "en", plugins: [], storage: createMemoryStoragePort() }), { type: "SET_CONFLICTS", value: result.conflicts ?? [] });
    expect(selectOpenConflicts(state)).toEqual([remoteConflict]);
    expect(selectQuarantinedConflicts(state)).toEqual([remoteConflict]);
  });
});
