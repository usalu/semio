type TestSource = { readonly directory: string; readonly url: string };

/** 🧬️ LAW: one codec call reaches a component's `codec` interface through every browser hop with the
 * same answer — the zero-import arm (`actorCodecAnswer`), the GENERATED bridge (`pluginComponentBridgeSource`,
 * imported as a real ES module beside a stand-in jco component) and the GENERATED worker
 * (`shardWorkerSource`'s `codec` case, which also refuses an overlapping turn and a stale activation).
 * Driven from the language-agnostic `🧬️component-codec/🧫️fixtures/🔣️.json`, validated against its schema
 * by Ajv (third-party), so the declaration and both generated halves cannot drift.
 * @see 🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts
 * @see 🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs (the native twin) */
export async function registerComponentCodecReplyTests(vitest: NonNullable<ImportMeta["vitest"]>, _testSource: TestSource): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: fixture } = await import("../../🧬️component-codec/🧫️fixtures/🔣️.json");
  const { default: schema } = await import("../../🧬️component-codec/🧬️schema/🔣️.json");
  const { actorCodecAnswer, ACTOR_CODEC_REFUSAL } = await import("../../🧬️component-codec/🟦️.ts");
  type Case = (typeof fixture.cases)[number];
  type Pair = { readonly packHex: string; readonly sprHex: string };

  const bytes = (hex: string): Uint8Array => Uint8Array.from(hex.match(/../g) ?? [], (pair) => Number.parseInt(pair, 16));
  const hex = (value: unknown): string => (value instanceof Uint8Array ? Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("") : `<${Object.prototype.toString.call(value)}>`);
  const pair = (value: Pair) => ({ pack: bytes(value.packHex), spr: bytes(value.sprHex) });
  const request = (row: Case): Record<string, unknown> => ("pair" in row.request && row.request.pair ? { ...row.request, pair: pair(row.request.pair as Pair) } : { ...row.request });
  const componentFault = (value: unknown): Error => Object.defineProperty(new Error(`${String(value)} (see error.payload)`), "payload", { value, enumerable: true });
  const codecFor = (row: Case) => {
    const behaviour = row.component as Record<string, unknown>;
    if (behaviour.exported === false) return undefined;
    const answer = async (): Promise<unknown> => {
      if (typeof behaviour.throws === "string") throw new Error(behaviour.throws);
      if (typeof behaviour.faultHex === "string") throw componentFault({ tag: "fault", val: bytes(behaviour.faultHex) });
      if (typeof behaviour.bytesHex === "string") return bytes(behaviour.bytesHex);
      if (behaviour.pair) return pair(behaviour.pair as Pair);
      return behaviour.mirror;
    };
    return { packSchemaHash: answer, genesis: answer, printMirror: answer } as never;
  };
  const verdict = async (call: () => Promise<unknown>): Promise<Record<string, unknown>> => {
    try {
      const answer = (await call()) as Record<string, unknown>;
      if ("fault" in answer) return { faultHex: hex(answer.fault) };
      const ok = answer.ok as Record<string, unknown> | Uint8Array | readonly string[];
      if (ok instanceof Uint8Array) return { okHex: hex(ok) };
      if (Array.isArray(ok)) return { okMirror: [...ok] };
      const value = ok as Record<string, unknown>;
      return { okPair: { packHex: hex(value.pack), sprHex: hex(value.spr) } };
    } catch (error) {
      return { refusal: error instanceof Error ? error.message : String(error) };
    }
  };

  describe("ShardComponentCodecLane", () => {
    it("owns a fixture its schema admits and names the refusals the leaf declares", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.refusals).toEqual({ ...ACTOR_CODEC_REFUSAL });
    });

    it("answers every fixture case identically in the leaf arm and in the generated bridge", async () => {
      const { mkdtempSync, writeFileSync, rmSync } = await import("node:fs");
      const { tmpdir } = await import("node:os");
      const { join } = await import("node:path");
      const { pathToFileURL } = await import("node:url");
      const { pluginComponentBridgeSource } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
      const directory = mkdtempSync(join(tmpdir(), "semio-codec-bridge-"));
      try {
        writeFileSync(join(directory, "🌉️bridge.js"), pluginComponentBridgeSource("fixture_component", "fixture.wasm"));
        writeFileSync(join(directory, "🟨️.js"), "export const __resolveEffect = () => {}; export const __rejectEffect = () => {};\n");
        writeFileSync(join(directory, "fixture_component.js"), "export const reactor = {}; export const jobs = {}; export const checkpoint = {}; export const describe = {}; export const codec = globalThis.__semioCodecFixture;\n");
        const observed: Record<string, unknown>[] = [];
        for (const [index, row] of fixture.cases.entries()) {
          const codec = codecFor(row);
          const leaf = await verdict(() => actorCodecAnswer(codec, request(row) as never));
          (globalThis as { __semioCodecFixture?: unknown }).__semioCodecFixture = codec;
          const bridge = await import(`${pathToFileURL(join(directory, "🌉️bridge.js")).href}?case=${index}`);
          const api = await bridge.createActorApi(`a${index}`, 1n);
          const generated = await verdict(() => api.codec(request(row)));
          expect(leaf, `${row.id} (leaf)`).toEqual(row.expected);
          expect(generated, `${row.id} (generated bridge)`).toEqual(row.expected);
          observed.push({ id: row.id, ...generated });
        }
        console.log(`shard-component-codec.bridge ${JSON.stringify(observed)}`);
      } finally {
        delete (globalThis as { __semioCodecFixture?: unknown }).__semioCodecFixture;
        rmSync(directory, { recursive: true, force: true });
      }
    });

    it("answers a codec request from the generated worker with a result and a beat, and refuses an overlapping turn or a stale activation", async () => {
      const vm = await import("node:vm");
      const { shardWorkerSource } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
      const posted: Record<string, unknown>[] = [];
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      let row = fixture.cases[0]!;
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        TextDecoder,
        console: { log: () => {}, error: () => {}, warn: () => {} },
        setInterval: () => null,
        clearInterval: () => {},
        self: { postMessage: (message: Record<string, unknown>) => posted.push(message), addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } },
        api: { codec: (codecRequest: never) => actorCodecAnswer(codecFor(row), codecRequest) },
      });
      new vm.Script(shardWorkerSource()).runInContext(context);
      new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(context);
      const send = dispatch as unknown as ((event: { data: Record<string, unknown> }) => Promise<void>) | null;
      if (!send) throw new Error("Missing generated worker dispatcher");
      const answered = (requestId: string) => posted.find((message) => message.kind === "result" && message.requestId === requestId) as { ok?: boolean; value?: unknown; error?: string; beat?: { turnSeq?: number } } | undefined;
      for (const [index, current] of fixture.cases.entries()) {
        row = current;
        await send({ data: { kind: "codec", requestId: `c${index}`, actorId: "a", activationGeneration: 1n, request: request(current) } });
        const result = answered(`c${index}`);
        expect(result, `${current.id}: a codec request the worker never answers is an outstanding request the watchdog kills the shard over`).toBeTruthy();
        const worker = result?.ok ? await verdict(async () => result.value) : { refusal: result?.error?.replace(/^Error: /, "") };
        expect(worker, `${current.id} (generated worker)`).toEqual(current.expected);
        if (result?.ok) expect(typeof result.beat?.turnSeq, "a codec call is a guest step and proves liveness like a job step").toBe("number");
      }
      row = fixture.cases[0]!;
      new vm.Script('inFlightTurnActors.add("a");').runInContext(context);
      await send({ data: { kind: "codec", requestId: "overlap", actorId: "a", activationGeneration: 1n, request: request(row) } });
      expect(answered("overlap")?.ok, "a codec call never re-enters a component a turn of the same actor is suspended in").toBe(false);
      expect(answered("overlap")?.error).toContain("already has a turn in flight");
      new vm.Script('inFlightTurnActors.delete("a");').runInContext(context);
      await send({ data: { kind: "codec", requestId: "stale", actorId: "a", activationGeneration: 2n, request: request(row) } });
      expect(answered("stale")?.ok).toBe(false);
      expect(answered("stale")?.error).toContain("actor-lifecycle.activation-mismatch");
    });
  });
}
