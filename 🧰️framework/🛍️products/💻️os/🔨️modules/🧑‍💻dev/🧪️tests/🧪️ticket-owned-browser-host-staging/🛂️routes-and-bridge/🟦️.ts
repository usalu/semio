/** 🛂️ The descriptor route guard, the catalog state transition probe and the plugin component bridge source of the ticket-owned browser host staging suite. */
import { actorPageFixtureUrl, actorLifetimePatchFixtureUrl, actorLifetimeFixtureUrl } from "@semio-tech/framework-actor/testing/fixtures";
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerRouteAndBridgeTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { ACTOR_COMPONENT_EXPORTS, assertActorComponentExports, descriptorRouteDecision, finalizePluginDescriptor, join, mkdirSync, mkdtempSync, pluginComponentBridgeSource, readFileSync, repoRoot, rmSync, stateProbeCandidates, stateProbeChangedPaths, stateProbeSnapshot, tmpdir, writeFileSync } = dependencies;
  const { describe, expect, it } = vitest;
  const { itLong } = createTestKit(vitest, dependencies, source);

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
    itLong("stages canonical command-page bytes unchanged and leaves poll carrying no page", async () => {
      const { execFileSync } = await import("node:child_process");
      const { createShardCommandIngressPages } = await import("@semio-tech/framework-actor/shard-client");
      const fixture = JSON.parse(readFileSync(actorPageFixtureUrl, "utf8"));
      const vectors = fixture.vectors.filter((row: { length: number }) => row.length !== 0);
      const inputs = vectors.map((row: { length: number }, index: number) => {
        const command = Uint8Array.from({ length: row.length }, (_, byte) => (byte * fixture.bytePattern.multiplier + fixture.bytePattern.addend) % fixture.bytePattern.modulus);
        return { hex: Buffer.from(command).toString("hex"), page: createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: index, commandCount: vectors.length, instance: 13, seq: BigInt(index + 17), command })[0] };
      });
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        let received;
        let pollArity;
        const context = createContext({ URL, Uint8Array, record: (cursor, bytes) => { received = { cursor, bytes }; }, recordPollArity: count => { pollArity = count; } });
        const component = new SourceTextModule("export const reactor = { stageCommandPage: async (cursor, bytes) => { record(cursor, bytes); }, poll: async (...args) => { recordPollArity(args.length); return { uiPatches: [], status: { tag: 'idle' }, commandIngress: { kind: 0 } }; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const api = await bridge.namespace.createActorApi("paged", 1n);
        const rows = [];
        for (const entry of input.inputs) {
          const page = entry.page;
          for (const key of ["owner", "generation", "seq"]) page.cursor[key] = BigInt(page.cursor[key]);
          page.bytes = Uint8Array.from(Object.values(page.bytes));
          await api.poll([], page, undefined, { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 });
          rows.push({ sameCursor: received.cursor === page.cursor, sameBytes: received.bytes === page.bytes, length: received.bytes.length, hex: Buffer.from(received.bytes).toString("hex"), pollArity });
        }
        console.log(JSON.stringify(rows));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), inputs }, (_key, value) => typeof value === "bigint" ? value.toString() : value), encoding: "utf8", timeout: 10_000 });
      expect(JSON.parse(output)).toEqual(inputs.map((entry: { hex: string }, index: number) => ({ sameCursor: true, sameBytes: true, length: vectors[index].length, hex: entry.hex, pollArity: 2 })));
    });

    itLong("maps issued UI patch receipts and exact ACK or rejection through the generated bridge", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixture = JSON.parse(readFileSync(actorLifetimePatchFixtureUrl, "utf8"));
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const observed = [];
        const context = createContext({ URL, Uint8Array, nextResult: null, record: events => observed.push(events) });
        const component = new SourceTextModule("export const reactor = { poll: async events => { record(events); return nextResult; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const budget = { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 };
        const result = (receipt, count) => ({ uiPatches: Array.from({ length: count }, () => ({})), uiPatchReceipt: receipt, status: { tag: "idle" }, commandIngress: { kind: 0 } });
        const convert = value => ({ lifetime: { activationGeneration: BigInt(value.lifetime.activationGeneration), instanceId: value.lifetime.instanceId, guestLifetime: BigInt(value.lifetime.guestLifetime) }, patchSequence: BigInt(value.patchSequence) });
        const rows = [];
        for (const vector of input.fixture.vectors) {
          const receipt = convert(vector.value);
          const api = await bridge.namespace.createActorApi("same", receipt.lifetime.activationGeneration);
          context.nextResult = result(receipt, 1);
          const actual = await api.poll([], undefined, undefined, budget);
          const row = { hex: actual.uiPatchReceipt instanceof Uint8Array ? Buffer.from(actual.uiPatchReceipt).toString("hex") : null, feedback: [] };
          context.nextResult = result(undefined, 0);
          for (const kind of input.fixture.feedback.kinds) {
            const payload = { receipt, surface: { instance: receipt.lifetime.instanceId, surface: input.fixture.feedback.surface }, revision: BigInt(input.fixture.feedback.revision), ...(kind === "rejected" ? { reason: "fixture rejection" } : {}) };
            await api.poll([{ kind: "patch-" + kind, payload }], undefined, undefined, budget);
            row.feedback.push(observed.at(-1)[0]);
          }
          rows.push(row);
        }
        const receipt = convert(input.fixture.vectors[1].value);
        const api = await bridge.namespace.createActorApi("same", receipt.lifetime.activationGeneration);
        const pairings = [];
        for (const row of input.fixture.pairing) {
          context.nextResult = result(row.hasReceipt ? receipt : undefined, row.patchCount);
          try { await api.poll([], undefined, undefined, budget); pairings.push(true); } catch { pairings.push(false); }
        }
        const malformedResults = [result({ ...receipt, patchSequence: 0n }, 1), result({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: 42n } }, 1), { ...result(undefined, 0), uiPatches: undefined }];
        const refusedResults = [];
        for (const raw of malformedResults) { context.nextResult = raw; try { await api.poll([], undefined, undefined, budget); refusedResults.push(false); } catch { refusedResults.push(true); } }
        context.nextResult = result(undefined, 0);
        const refusedEvents = [];
        for (const kind of input.fixture.feedback.kinds) {
          for (const wrong of [undefined, { ...receipt, patchSequence: 0n }, { ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: 42n } }]) {
            const before = observed.length;
            try { await api.poll([{ kind: "patch-" + kind, payload: { receipt: wrong, surface: { instance: receipt.lifetime.instanceId, surface: input.fixture.feedback.surface }, revision: 1n, reason: "fixture" } }], undefined, undefined, budget); refusedEvents.push(false); } catch { refusedEvents.push(observed.length === before); }
          }
        }
        console.log(JSON.stringify({ rows, pairings, refusedResults, refusedEvents }, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), fixture }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output);
      expect(result.rows.map((row: { hex: string }) => row.hex)).toEqual(fixture.vectors.map((row: { hex: string }) => row.hex));
      expect(result.pairings).toEqual(fixture.pairing.map((row: { accepted: boolean }) => row.accepted));
      expect(result.refusedResults).toEqual([true, true, true]);
      expect(result.refusedEvents).toEqual([true, true, true, true, true, true]);
      for (let index = 0; index < fixture.vectors.length; index += 1) {
        for (let kindIndex = 0; kindIndex < fixture.feedback.kinds.length; kindIndex += 1) {
          const kind = fixture.feedback.kinds[kindIndex];
          expect(result.rows[index].feedback[kindIndex]).toEqual({ tag: "patch-" + kind, val: { receipt: fixture.vectors[index].value, surface: { instance: fixture.vectors[index].value.lifetime.instanceId, surface: fixture.feedback.surface }, revision: fixture.feedback.revision, ...(kind === "rejected" ? { reason: "fixture rejection" } : {}) } });
        }
      }
    });

    itLong("maps canonical lifecycle requests and receipts through the real generated bridge", async () => {
      const { execFileSync } = await import("node:child_process");
      const fixture = JSON.parse(readFileSync(actorLifetimeFixtureUrl, "utf8"));
      const output = execFileSync("node", ["--experimental-vm-modules", "--input-type=module", "--eval", `
        import { SourceTextModule, createContext } from "node:vm";
        import { readFileSync } from "node:fs";
        const input = JSON.parse(readFileSync(0, "utf8"));
        const observed = [];
        const context = createContext({ URL, Uint8Array, nextReceipt: undefined, record: events => observed.push(events) });
        const component = new SourceTextModule("export const reactor = { poll: async events => { record(events); return { uiPatches: [], lifecycleReceipt: nextReceipt, status: { tag: 'idle' }, commandIngress: { kind: 0 } }; } }; export const jobs = {}; export const checkpoint = {}; export const describe = {};", { context });
        const host = new SourceTextModule("export const __resolveEffect = () => {}; export const __rejectEffect = () => {};", { context });
        for (const module of [component, host]) { await module.link(() => { throw new Error("unexpected import"); }); await module.evaluate(); }
        const bridge = new SourceTextModule(input.source, { context, identifier: "https://fixture.invalid/bridge.js", initializeImportMeta: meta => { meta.url = "https://fixture.invalid/bridge.js"; }, importModuleDynamically: async specifier => specifier.includes("host-shim") ? host : component });
        await bridge.link(() => { throw new Error("unexpected static import"); }); await bridge.evaluate();
        const budget = { fuel: 1, wallMs: 4, maxEffects: 1, maxPatchBytes: 4096 };
        const rows = [];
        const body = receipt => ({ lifetime: receipt.lifetime, requestSequence: BigInt(receipt.requestSequence), ...(receipt.kind === "captured" ? {} : { closeGeneration: receipt.closeGeneration }) });
        for (const row of input.vectors) {
          const value = JSON.parse(JSON.stringify(row.value), (key, value) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(value) : value);
          const receipt = value.kind === "ack" ? value.receipt : value;
          const generation = value.kind === "open" ? value.activationGeneration : receipt.lifetime.activationGeneration;
          const api = await bridge.namespace.createActorApi("same", generation);
          context.nextReceipt = undefined;
          if (["captured", "accepted", "retired"].includes(value.kind)) {
            context.nextReceipt = { tag: value.kind, val: body(value) };
            const result = await api.poll([], undefined, undefined, budget);
            rows.push({ kind: value.kind, hex: Buffer.from(result.lifecycleReceipt).toString("hex") });
          } else {
            const kind = value.kind === "open" ? "instance-open" : value.kind === "close" ? "instance-close" : "instance-lifecycle-ack";
            const payload = value.kind === "open" ? { instance: value.instanceId, activationGeneration: value.activationGeneration, requestSequence: value.requestSequence, appId: "fixture", actor: "tester", config: [], assets: [], capabilities: [], quotas: [] } : value;
            await api.poll([{ kind, payload }], undefined, undefined, budget);
            rows.push({ kind: value.kind, event: observed.at(-1)[0] });
          }
        }
        const api = await bridge.namespace.createActorApi("same", 1n);
        const errors = [];
        for (const receipt of [{ tag: "open", val: {} }, { tag: "captured", val: { lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: 9007199254740992n } }]) {
          context.nextReceipt = receipt;
          try { await api.poll([], undefined, undefined, budget); errors.push(false); } catch { errors.push(true); }
        }
        console.log(JSON.stringify({ rows, errors }, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      `], { input: JSON.stringify({ source: pluginComponentBridgeSource("component", "component.core.wasm"), vectors: fixture.vectors }), encoding: "utf8", timeout: 10_000 });
      const result = JSON.parse(output);
      expect(result.errors).toEqual([true, true]);
      for (let index = 0; index < fixture.vectors.length; index += 1) {
        const value = fixture.vectors[index].value;
        const actual = result.rows[index];
        if (["captured", "accepted", "retired"].includes(value.kind)) expect(actual.hex).toBe(fixture.vectors[index].hex);
        else if (value.kind === "open") expect(actual.event).toMatchObject({ tag: "instance-open", val: { instance: value.instanceId, activationGeneration: value.activationGeneration, requestSequence: String(value.requestSequence) } });
        else if (value.kind === "close") expect(actual.event).toEqual({ tag: "instance-close", val: { lifetime: value.lifetime, requestSequence: String(value.requestSequence) } });
        else expect(actual.event).toEqual({ tag: "instance-lifecycle-ack", val: { tag: value.receipt.kind, val: { lifetime: value.receipt.lifetime, requestSequence: String(value.receipt.requestSequence), ...(value.receipt.kind === "captured" ? {} : { closeGeneration: value.receipt.closeGeneration }) } } });
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

    it("adapts the actor grant into the reactor budget vocabulary", () => {
      const source = pluginComponentBridgeSource("plugin", "plugin.core.wasm");
      expect(source).toContain("fuel: BigInt(budget.fuel), deadlineMs: budget.wallMs");
      expect(source).toContain("maxFrames: 8");
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
