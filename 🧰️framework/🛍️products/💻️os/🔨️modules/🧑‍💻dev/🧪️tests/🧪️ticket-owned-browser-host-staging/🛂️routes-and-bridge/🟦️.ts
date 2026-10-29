/** 🛂️ The descriptor route guard, the catalog state transition probe and the plugin component bridge source of the ticket-owned browser host staging suite. */
import { actorPageFixtureUrl, actorLifetimePatchFixtureUrl, actorLifetimeFixtureUrl } from "@semio-tech/framework-actor/testing/fixtures";
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerRouteAndBridgeTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { ACTOR_COMPONENT_EXPORTS, assertActorComponentExports, descriptorRouteDecision, finalizePluginDescriptor, join, mkdirSync, mkdtempSync, pluginComponentBridgeSource, readFileSync, repoRoot, rmSync, stateProbeCandidates, stateProbeChangedPaths, stateProbeSnapshot, tmpdir, writeFileSync } = dependencies;
  const { describe, expect, it } = vitest;
  const { itLong, generatedBridge, retainedGuestSource } = createTestKit(vitest, dependencies, source);
  const budget = { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 };

  type PackValue = import("@semio-tech/framework-os").PackValue;
  type ParityDump = import("../../../⚖️parity/🏗️structure/🟦️.ts").ParityDump;
  type ParityNode = import("../../../⚖️parity/🏗️structure/🟦️.ts").ParityNode;

  describe("descriptorRouteDecision", () => {
    it("admits an exact declared descriptor and rejects missing or undeclared descriptor paths before SPA fallback", () => {
      const root = mkdtempSync(join(tmpdir(), "semio-descriptor-route-"));
      try {
        mkdirSync(join(root, "🗒️note"), { recursive: true });
        writeFileSync(join(root, "🗒️note", "🔣️.json"), '{"manifest":{"pluginId":"note"}}\n');
        const specs = [{ route: "/🔌️plugin-modules", root, directoryNames: new Set(["🗒️note", "🪐️space"]) }];
        expect(descriptorRouteDecision("/🔌️plugin-modules/🗒️note/🔣️.json", specs)).toEqual({ kind: "pass" });
        expect(descriptorRouteDecision("/🔌️plugin-modules/🪐️space/🔣️.json?epoch=1", specs)).toEqual({ kind: "missing", moduleDirectory: "🪐️space" });
        expect(descriptorRouteDecision("/🔌️plugin-modules/unknown/🔣️.json", specs)).toEqual({ kind: "missing", moduleDirectory: "unknown" });
        expect(descriptorRouteDecision("/other/🪐️space/🔣️.json", specs)).toEqual({ kind: "pass" });
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

  describe("catalog state transition probe", () => {
    const node = (path: string, kind: string, selected = false): ParityNode => ({
      path,
      kind,
      rect: [0, 0, 20, 20],
      text: null,
      color: null,
      bg: null,
      fontSize: null,
      fontWeight: null,
      visible: true,
      state: { hovered: false, disabled: false, selected },
    });
    const dump = (nodes: readonly ParityNode[]): ParityDump => ({ viewport: { w: 100, h: 100, dpr: 1 }, focusPath: null, nodes });

    it("selects only common explicitly-id'd app controls in semantic priority order", () => {
      const react = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("stack[2]", "stack")]);
      const wgpu = dump([node("button[0]#run", "button"), node("toggle[1]#enabled", "toggle"), node("select[3]#mode", "select")]);
      expect(stateProbeCandidates(react, wgpu)).toEqual([
        { path: "toggle[1]#enabled", kind: "toggle" },
        { path: "button[0]#run", kind: "button" },
      ]);
    });

    it("records selected-state and topology changes but ignores focus-only movement", () => {
      const before = dump([node("toggle[0]#enabled", "toggle")]);
      const after = { ...dump([node("toggle[0]#enabled", "toggle", true), node("text[1]#status", "text")]), focusPath: "toggle[0]#enabled" };
      expect(stateProbeChangedPaths(before, after)).toEqual(["text[1]#status", "toggle[0]#enabled"]);
      expect(stateProbeSnapshot(before).digest).not.toBe(stateProbeSnapshot(after).digest);
    });
  });

  //#region 🔖️T-P8-tests
  describe("pluginComponentBridgeSource", () => {
    itLong("stages a command page and a cold-pair page through the guest exports before its single-event poll, like the native component host", async () => {
      const { createShardCommandIngressPages } = await import("@semio-tech/framework-actor/shard-client");
      const fixture = JSON.parse(readFileSync(actorPageFixtureUrl, "utf8"));
      const vectors = fixture.vectors.filter((row: { length: number }) => row.length !== 0);
      const bridge = await generatedBridge(retainedGuestSource({ extra: `stageCommandPage: async (cursor, bytes) => { globalThis.__bridgeStaged?.("command", cursor, bytes); }, stageColdPairPage: async (page) => { globalThis.__bridgeStaged?.("cold", page); },` }));
      const reached: unknown[][] = [];
      Object.assign(globalThis, { __bridgePolled: (events: unknown) => reached.push(["poll", events]), __bridgeStaged: (kind: string, ...args: unknown[]) => reached.push(["stage", kind, ...args]) });
      try {
        for (const [index, row] of vectors.entries() as Iterable<[number, { length: number }]>) {
          const command = Uint8Array.from({ length: row.length }, (_, byte) => (byte * fixture.bytePattern.multiplier + fixture.bytePattern.addend) % fixture.bytePattern.modulus);
          const page = createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: index, commandCount: vectors.length, instance: 13, seq: BigInt(index + 17), command })[0]!;
          const api = await bridge.activate(1n);
          reached.length = 0;
          await api.poll([], page, undefined, budget);
          expect(reached.map((step) => step[0])).toEqual(["stage", "poll"]);
          expect(reached[0]![1]).toBe("command");
          expect(reached[0]![2]).toBe(page.cursor);
          expect(reached[0]![3]).toBe(page.bytes);
          expect(reached[1]![1]).toEqual([]);
          expect(api.originalSource.producerRefusal).toBeUndefined();
          await api.poll([], undefined, undefined, budget);
        }
        expect(bridge.held).toHaveLength(0);
        const cold = { header: {}, pageIndex: 0, bytes: new Uint8Array([1]) };
        reached.length = 0;
        await (await bridge.activate(1n)).poll([], undefined, cold, budget);
        expect(reached.map((step) => step.slice(0, 2))).toEqual([["stage", "cold"], ["poll", []]]);
        expect(reached[0]![2]).toBe(cold);
      } finally {
        delete (globalThis as Record<string, unknown>).__bridgePolled;
        delete (globalThis as Record<string, unknown>).__bridgeStaged;
        bridge.dispose();
      }
    });

    itLong("maps issued UI patch receipts through the generated bridge and delivers every patch ACK or rejection as the poll's single event", async () => {
      const fixture = JSON.parse(readFileSync(actorLifetimePatchFixtureUrl, "utf8"));
      const convert = (value: { lifetime: { activationGeneration: string; instanceId: number; guestLifetime: string }; patchSequence: string }) => ({ lifetime: { activationGeneration: BigInt(value.lifetime.activationGeneration), instanceId: value.lifetime.instanceId, guestLifetime: BigInt(value.lifetime.guestLifetime) }, patchSequence: BigInt(value.patchSequence) });
      const result = (receipt: unknown, count: number) => ({ uiPatches: Array.from({ length: count }, () => ({})), uiPatchReceipt: receipt });
      const bridge = await generatedBridge(retainedGuestSource());
      const polled: unknown[] = [];
      (globalThis as Record<string, unknown>).__bridgePolled = (events: unknown) => polled.push(events);
      const answer = async (generation: bigint, next: Record<string, unknown>, events: readonly { kind: string; payload: unknown }[] = []) => {
        (globalThis as Record<string, unknown>).__bridgeNext = next;
        return (await bridge.activate(generation)).poll(events, undefined, undefined, budget);
      };
      try {
        for (const vector of fixture.vectors) {
          const receipt = convert(vector.value);
          const actual = await answer(receipt.lifetime.activationGeneration, result(receipt, 1));
          expect(actual.uiPatchReceipt instanceof Uint8Array ? Buffer.from(actual.uiPatchReceipt).toString("hex") : null).toBe(vector.hex);
          for (const kind of fixture.feedback.kinds) {
            const payload = { receipt, surface: { instance: receipt.lifetime.instanceId, surface: fixture.feedback.surface }, revision: BigInt(fixture.feedback.revision), ...(kind === "rejected" ? { reason: "fixture rejection" } : {}) };
            const before = polled.length;
            await answer(receipt.lifetime.activationGeneration, result(undefined, 0), [{ kind: "patch-" + kind, payload }]);
            expect(polled).toHaveLength(before + 1);
            expect(polled.at(-1)).toEqual([{ tag: "patch-" + kind, val: payload }]);
            expect(bridge.held).toHaveLength(0);
            await expect(answer(receipt.lifetime.activationGeneration, result(undefined, 0), [{ kind: "wake", payload: undefined }, { kind: "wake", payload: undefined }])).rejects.toThrow("a reactor turn admits at most one event");
          }
        }
        const receipt = convert(fixture.vectors[1].value);
        for (const row of fixture.pairing) {
          const outcome = await answer(receipt.lifetime.activationGeneration, result(row.hasReceipt ? receipt : undefined, row.patchCount)).then(() => true, () => false);
          expect(outcome, JSON.stringify(row)).toBe(row.accepted);
        }
        for (const raw of [result({ ...receipt, patchSequence: 0n }, 1), result({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: 42n } }, 1), { ...result(undefined, 0), uiPatches: undefined }]) {
          await expect(answer(receipt.lifetime.activationGeneration, raw)).rejects.toThrow();
        }
        expect(polled.length).toBeGreaterThan(0);
        expect(polled.every((events) => (events as unknown[]).length <= 1), "no guest poll carries more than one event").toBe(true);
      } finally {
        delete (globalThis as Record<string, unknown>).__bridgeNext;
        delete (globalThis as Record<string, unknown>).__bridgePolled;
        bridge.dispose();
      }
    });

    itLong("maps canonical lifecycle receipts through the real generated bridge and delivers every lifecycle request or ACK as the poll's single event", async () => {
      const fixture = JSON.parse(readFileSync(actorLifetimeFixtureUrl, "utf8"));
      const bigint = (value: unknown) => JSON.parse(JSON.stringify(value), (key, entry) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(entry) : entry);
      const body = (receipt: { lifetime: unknown; requestSequence: string; kind: string; closeGeneration?: bigint }) => ({ lifetime: receipt.lifetime, requestSequence: BigInt(receipt.requestSequence), ...(receipt.kind === "captured" ? {} : { closeGeneration: receipt.closeGeneration }) });
      const bridge = await generatedBridge(retainedGuestSource());
      const delivered: unknown[] = [];
      (globalThis as Record<string, unknown>).__bridgePolled = (events: unknown) => delivered.push(events);
      const answer = async (generation: bigint, next: Record<string, unknown>, events: readonly { kind: string; payload: unknown }[] = []) => {
        (globalThis as Record<string, unknown>).__bridgeNext = next;
        return (await bridge.activate(generation)).poll(events, undefined, undefined, budget);
      };
      try {
        for (const row of fixture.vectors) {
          const value = bigint(row.value);
          const receipt = value.kind === "ack" ? value.receipt : value;
          const generation = value.kind === "open" ? value.activationGeneration : receipt.lifetime.activationGeneration;
          if (["captured", "accepted", "retired"].includes(value.kind)) {
            const mapped = await answer(generation, { lifecycleReceipt: { tag: value.kind, val: body(value) } });
            expect(Buffer.from(mapped.lifecycleReceipt).toString("hex"), value.kind).toBe(row.hex);
            continue;
          }
          const kind = value.kind === "open" ? "instance-open" : value.kind === "close" ? "instance-close" : "instance-lifecycle-ack";
          const payload = value.kind === "open" ? { instance: value.instanceId, activationGeneration: value.activationGeneration, requestSequence: value.requestSequence, appId: "fixture", actor: "tester", config: [], assets: [], capabilities: [], quotas: [] } : value;
          const before = delivered.length;
          await answer(generation, {}, [{ kind, payload }]);
          expect(delivered, value.kind).toHaveLength(before + 1);
          expect(delivered.at(-1), value.kind).toMatchObject([{ tag: kind }]);
          expect(bridge.held, value.kind).toHaveLength(0);
        }
        for (const raw of [{ tag: "open", val: {} }, { tag: "captured", val: { lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: 9007199254740992n } }]) {
          await expect(answer(1n, { lifecycleReceipt: raw })).rejects.toThrow();
        }
      } finally {
        delete (globalThis as Record<string, unknown>).__bridgeNext;
        delete (globalThis as Record<string, unknown>).__bridgePolled;
        bridge.dispose();
      }
    });

    it("requires every actor export from plugin and extension component namespaces", () => {
      const actor = Object.fromEntries(Object.entries(ACTOR_COMPONENT_EXPORTS).map(([name, methods]) => [name, Object.fromEntries(methods.map((method) => [method, () => undefined]))]));
      expect(() => assertActorComponentExports(actor, ACTOR_COMPONENT_EXPORTS)).not.toThrow();
      expect(() => assertActorComponentExports({ _util: {} }, ACTOR_COMPONENT_EXPORTS)).toThrow(`Missing actor export reactor.${ACTOR_COMPONENT_EXPORTS.reactor[0]}`);
      for (const [name, methods] of Object.entries(ACTOR_COMPONENT_EXPORTS)) {
        for (const method of methods) {
          const incomplete = { ...actor, [name]: { ...actor[name], [method]: undefined } };
          expect(() => assertActorComponentExports(incomplete, ACTOR_COMPONENT_EXPORTS)).toThrow(`${name}.${method}`);
        }
      }
    });

    itLong("admits genuine semantic descriptor bytes exactly and refuses a foreign, failed or byte-bound one", async () => {
      const { clonePackValue, decodePackValue, encodePackValue, packValueToExactJson } = await import("@semio-tech/framework-os");
      const bytes = readFileSync(join(repoRoot, "🌎️hub/🧩️compositions/🏙️bim/🧩️component/🛂️.descriptor.semio"));
      const decoded = decodePackValue(bytes);
      const descriptor = decoded as unknown as { manifest: { pluginId: string } };
      const finalized = finalizePluginDescriptor(bytes, descriptor.manifest.pluginId);
      expect(Buffer.from(finalized.pack)).toEqual(bytes);
      expect(JSON.parse(finalized.json)).toEqual(packValueToExactJson(decoded));
      expect(() => finalizePluginDescriptor(bytes, "wrong-plugin")).toThrow("identity");
      const bound = clonePackValue(decoded) as unknown as Record<string, unknown>;
      bound.hashes = { wasmSha256: "a".repeat(64) };
      expect(() => finalizePluginDescriptor(encodePackValue(bound as unknown as PackValue), descriptor.manifest.pluginId)).toThrow("byte binding");
      descriptor.manifest.pluginId = "assembly-failed";
      expect(() => finalizePluginDescriptor(encodePackValue(decoded), "assembly-failed")).toThrow("assembly");
    });

    it("adapts the shard envelope into the canonical jco variant representation", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain('kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload })');
    });

    it("adapts the retained original turn and the actor grant into the reactor budget vocabulary", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("fuel: BigInt(turn.fuel), deadlineUs: turn.deadlineUs, maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: budget.maxFrames");
    });

    it("normalizes the scalar command-ingress wire record without relying on async variant discriminants", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("function normalizeCommandIngress(status)");
      expect(source).toContain('[0, "idle"], [1, "page-accepted"], [2, "backpressure"], [3, "command-pending"], [4, "command-complete"], [5, "fault"]');
      expect(source).toContain("commandIngress: normalizeCommandIngress(result.commandIngress)");
    });

    it("gives every actor a distinct component module while carrying the rebuild version", () => {
      const source = pluginComponentBridgeSource("plugin_component", "plugin_component.core.wasm");
      expect(source).toContain('componentUrl.searchParams.set("actor", actorId)');
      expect(source).toContain('componentUrl.searchParams.set("v", rebuildVersion)');
      expect(source).toContain("await import(componentUrl.href)");
      expect(source).not.toContain('await import("./🟦️")');
    });
  });
  //#endregion 🔖️T-P8-tests
}
