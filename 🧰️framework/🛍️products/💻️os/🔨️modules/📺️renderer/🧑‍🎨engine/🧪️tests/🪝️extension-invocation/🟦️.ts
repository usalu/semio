/** 🪝 Extension invocation completion ownership and public invocation paging. */
import { dispatchInvokeExtensionEffect, runInvokeExtensionEffect } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import type { LoadedProgramState } from "../../🧱️elements/🐚️Shell/📦️program-sessions/🟦️.ts";
import extensionInvocationFixture from "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/🔣️extension-invocation.json";
import { extensionInvocationWireFixture } from "../../../../🌊️flow/🧪️tests/🧫️fixtures/🟦️.ts";
import { describe, expect, it, vi } from "vitest";
import { manifestPublicInvocationSchema as publicInvocationSchema } from "../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🟦️.ts";
import { PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_DEPTH, PUBLIC_INVOCATION_STRING_BYTES, publicInvocationCharCost, publicInvocationStringPages } from "@semio-tech/framework";
import { rendererExport } from "../🧰️engine-contract-helpers/🟦️.ts";


//#region 🔁️ExtensionInvocation
/** 📦️ What a capability actually answers across the door: BYTES in the capability's own encoding
 * (`evaluate_invoke_json` emits JSON text), never a JS string — `PluginWasmHandle.invoke` returns the
 * guest's `respond` OK pack verbatim and the shell re-encodes it once, on the completion. */
const extensionAnswerBytes = (answer: unknown): Uint8Array => new TextEncoder().encode(JSON.stringify(answer));

describe("extension invocation completion ownership", () => {
  const fixture = extensionInvocationFixture;
  const entry = (handle: Record<string, unknown>) => ({ handle, manifest: {} }) as unknown as LoadedProgramState;
  const completionHandle = (complete: (instanceId: number, req: bigint, outcome: unknown) => Promise<unknown>) => ({
    captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete: (outcome: unknown) => complete(instanceId, req, outcome) }),
  });
  const call = (requester: LoadedProgramState, extension?: LoadedProgramState) => runInvokeExtensionEffect(requester, extension, fixture.instanceId, fixture.extensionId, fixture.capability, JSON.stringify(fixture.request), BigInt(fixture.requestId));

  it("validates the language-neutral request and fault vectors with the schema oracle", () => {
    expect(rendererExport("ExtensionInvocationCompletionV1")(fixture.completion)).toBe(true);
    expect(fixture.completionFailures.every((row) => rendererExport("ExtensionCompletionRefusalV1")(row))).toBe(true);
    expect(fixture.missing.every((row) => rendererExport("ExtensionInvocationMissV1")(row))).toBe(true);
    expect(rendererExport("ExtensionInvocationFaultV1")(fixture.fault)).toBe(true);
    expect(rendererExport("ExtensionInvocationCompletionV1")({ ...fixture.completion, revision: 0 })).toBe(false);
  });

  it.each(fixture.missing)("faults the exact requester when $phase is unavailable", async ({ phase, code }) => {
    const { decodePackValue, decodeFaultFromWire } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    await call(entry(completionHandle(complete)), phase === "extension" ? undefined : entry({}));
    expect(complete).toHaveBeenCalledOnce();
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { fault: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(outcome).toHaveProperty("fault");
    const fault = decodeFaultFromWire(Array.from(outcome.fault), decodePackValue);
    expect(fault?.code).toBe(code);
    expect(fault?.origin).toBe("os");
  });

  it("refuses a missing completion capability before executing the extension", async () => {
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    await expect(call(entry({}), entry({ invoke }))).rejects.toThrow("extension.completion-unavailable");
    expect(invoke).not.toHaveBeenCalled();
  });

  it("preserves a typed extension fault and sends exactly one completion", async () => {
    const { SemioFaultError } = await import("@semio-tech/framework");
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    const invoke = vi.fn(async () => {
      throw new SemioFaultError(fixture.fault as ConstructorParameters<typeof SemioFaultError>[0]);
    });
    await call(entry(completionHandle(complete)), entry({ invoke }));
    expect(complete).toHaveBeenCalledOnce();
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { fault: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(decodePackValue(outcome.fault)).toEqual(fixture.fault);
  });

  it("propagates a failed completion without retrying it as a second fault completion", async () => {
    const failure = new Error("completion transport refused");
    const complete = vi.fn(async () => {
      throw failure;
    });
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    await expect(call(entry(completionHandle(complete)), entry({ invoke }))).rejects.toBe(failure);
    expect(invoke).toHaveBeenCalledOnce();
    expect(complete).toHaveBeenCalledOnce();
  });

  it("returns the requesting actor's completion publication to its host owner", async () => {
    const response = {
      output: fixture.response,
      mutations: [],
      inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
      requestedEffects: [{ notify: { message: fixture.completion.notification } }],
      uiScope: fixture.completion.uiScope,
      historyPatch: fixture.completion.historyPatch,
    };
    const complete = vi.fn(async () => response);
    expect(await call(entry(completionHandle(complete)), entry({ invoke: async () => extensionAnswerBytes(fixture.response) }))).toBe(response);
    expect(complete).toHaveBeenCalledOnce();
  });


  // 🔁️ The Rust law over the SAME fixture is `evaluate_invoke_json`'s
  // `the_evaluate_wire_answers_every_fixture_row` (`🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🦀️.rs`):
  // it produces the answer bytes, this twin proves the shell classifies and packs exactly those.
  it.each(extensionInvocationWireFixture.rows)("classifies the $name evaluate answer the way the extension SDK produced it", async (row) => {
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const request = JSON.parse(row.requestJson) as Record<string, unknown>;
    const present = extensionInvocationWireFixture.requestFields.filter((field) => request[field] !== undefined);
    expect(present.length === extensionInvocationWireFixture.requestFields.length).toBe(row.outcome === "ok");
    const answer = row.outcome === "ok" ? Object.fromEntries((row.outputKeys ?? []).map((key) => [key, row.outputEmpty ? {} : { value: 1 }])) : undefined;
    const complete = vi.fn(async () => {});
    const invoke = vi.fn(async () => {
      if (answer === undefined) throw new Error(row.fault);
      return extensionAnswerBytes(answer);
    });
    await runInvokeExtensionEffect(entry(completionHandle(complete)), entry({ invoke }), fixture.instanceId, fixture.extensionId, extensionInvocationWireFixture.capability, row.requestJson, BigInt(fixture.requestId));
    // 🛑️ The shell hands every extension request an `AbortSignal` so a surface's declared cancel
    // can retire it from outside the per-actor queue (`📓️preview-eval-cancellation-2026-09-12.md`).
    expect(invoke).toHaveBeenCalledExactlyOnceWith(extensionInvocationWireFixture.capability, row.requestJson, { originInstanceId: fixture.instanceId, signal: expect.any(AbortSignal) });
    const [, , outcome] = complete.mock.calls[0] as unknown as [number, bigint, { ok?: Uint8Array; fault?: Uint8Array }];
    expect("ok" in outcome ? "ok" : "fault").toBe(row.outcome);
    if (row.outcome === "ok") expect(decodePackValue(outcome.ok!)).toEqual(answer);
    else expect(decodePackValue(outcome.fault!)).toMatchObject({ code: "extension.invoke-failed", message: row.fault });
  });

  it.each(fixture.genericRequests)("dispatches $capability without requiring unrelated domain fields", async ({ capability, request }) => {
    const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
    const extension = entry({ pluginId: fixture.extensionId, invoke });
    const requestJson = JSON.stringify(request);
    const publish = vi.fn(async () => {});
    await dispatchInvokeExtensionEffect([requester, extension], { pluginId: "requester", instanceId: fixture.instanceId }, { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability, requestJson }, publish);
    // 🛑️ The shell hands every extension request an `AbortSignal` so a surface's declared cancel
    // can retire it from outside the per-actor queue (`📓️preview-eval-cancellation-2026-09-12.md`).
    expect(invoke).toHaveBeenCalledExactlyOnceWith(capability, requestJson, { originInstanceId: fixture.instanceId, signal: expect.any(AbortSignal) });
    expect(complete).toHaveBeenCalledOnce();
    expect(publish).toHaveBeenCalledExactlyOnceWith(requester, await complete.mock.results[0]!.value);
  });

  // 🪪️ LAW: an invocation addresses an extension by the CONTRIBUTING PLUGIN's id. A producer whose
  // own domain names the extension differently (flow calls the text kernel `text`, its plugin is
  // `flow-extension-text`) must translate before it emits — this shell knows no topic vocabulary and
  // resolves nothing but `handle.pluginId`. Regression guard: the removed fallback scanned
  // `manifest.contributions` for an `extensionId` field no manifest carries, so the miss was silent
  // and every flow evaluation stalled (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  it("resolves the extension actor by plugin id and faults a flow-domain address", async () => {
    const { decodePackValue, decodeFaultFromWire } = await import("@semio-tech/framework-os");
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const address = async (extensionId: string) => {
      const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
      const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
      const extension = { handle: { pluginId: fixture.extensionId, invoke }, manifest: { topicContributions: [{ topic: "flow.extension", payload: { extensionId: fixture.foreignAddress } }] } } as unknown as LoadedProgramState;
      await dispatchInvokeExtensionEffect(
        [requester, extension],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {},
      );
      return complete.mock.calls[0] as unknown as [number, bigint, { ok?: Uint8Array; fault?: Uint8Array }];
    };
    const [, , resolved] = await address(fixture.extensionId);
    expect(invoke).toHaveBeenCalledOnce();
    expect(decodePackValue(resolved.ok!)).toEqual(fixture.response);
    const [, , missed] = await address(fixture.foreignAddress);
    expect(invoke).toHaveBeenCalledOnce();
    expect(decodeFaultFromWire(Array.from(missed.fault!), decodePackValue)?.code).toBe("extension.missing");
  });

  it("rejects a missing originating plugin rather than silently discarding its request", async () => {
    await expect(
      dispatchInvokeExtensionEffect(
        [],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {},
      ),
    ).rejects.toThrow("extension.requester-unavailable");
  });

  it("propagates publication refusal after delivering only one completion", async () => {
    const refusal = new Error("extension.requester-retired");
    const complete = vi.fn(async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }));
    const requester = entry({ pluginId: "requester", ...completionHandle(complete) });
    const extension = entry({ pluginId: fixture.extensionId, invoke: async () => extensionAnswerBytes(fixture.response) });
    await expect(
      dispatchInvokeExtensionEffect(
        [requester, extension],
        { pluginId: "requester", instanceId: fixture.instanceId },
        { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
        async () => {
          throw refusal;
        },
      ),
    ).rejects.toBe(refusal);
    expect(complete).toHaveBeenCalledOnce();
  });

  it("decodes a real response without losing the owning method receiver", async () => {
    const { decodePackValue } = await import("@semio-tech/framework-os");
    const complete = vi.fn(async () => {});
    const handle = {
      response: fixture.response,
      invoke: async function () {
        return extensionAnswerBytes(this.response);
      },
    };
    await call(entry(completionHandle(complete)), entry(handle));
    const [instance, req, outcome] = complete.mock.calls[0] as unknown as [number, bigint, { ok: Uint8Array }];
    expect([instance, req]).toEqual([fixture.instanceId, BigInt(fixture.requestId)]);
    expect(decodePackValue(outcome.ok)).toEqual(fixture.response);
  });

  it("captures the exact completion authority before executing extension evaluation", async () => {
    const events: string[] = [];
    const complete = vi.fn(async () => {
      events.push("complete");
      return { output: null };
    });
    const capture = vi.fn((instanceId: number, req: bigint) => {
      events.push("capture");
      return { instanceId, req, assertActive: () => {}, complete };
    });
    const invoke = vi.fn(async () => {
      events.push("invoke");
      return extensionAnswerBytes(fixture.response);
    });
    await call(entry({ captureExtensionCompletion: capture }), entry({ invoke }));
    expect(events).toEqual(["capture", "invoke", "complete"]);
    expect(capture).toHaveBeenCalledExactlyOnceWith(fixture.instanceId, BigInt(fixture.requestId));
    expect(complete).toHaveBeenCalledOnce();
  });

  it("does not evaluate when initial activation capture is refused", async () => {
    const failure = new Error("actor-activation.revoked");
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const complete = vi.fn(async () => ({ output: null }));
    await expect(
      call(
        entry({
          captureExtensionCompletion: () => {
            throw failure;
          },
        }),
        entry({ invoke }),
      ),
    ).rejects.toBe(failure);
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
  });

  it.each(fixture.completionFailures)("refuses $kind before extension execution", async ({ kind, code }) => {
    const complete = vi.fn(async () => ({ output: null }));
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const capture = (instanceId: number, req: bigint) =>
      kind === "missing-lease"
        ? null
        : {
            instanceId: kind === "foreign-instance" ? instanceId + 1 : instanceId,
            req: kind === "foreign-request" ? req + 1n : req,
            assertActive: kind === "missing-guard" ? undefined : () => {},
            complete: kind === "missing-complete" ? undefined : complete,
          };
    await expect(call(entry({ captureExtensionCompletion: capture }), entry({ invoke }))).rejects.toThrow(code);
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
  });

  it("does not deliver a result to an activation replaced during extension evaluation", async () => {
    const entered = Promise.withResolvers<void>();
    const release = Promise.withResolvers<Uint8Array>();
    let generation = 1;
    const complete = vi.fn(async () => ({ output: null }));
    const capture = (instanceId: number, req: bigint) => {
      const captured = generation;
      return {
        instanceId,
        req,
        assertActive: () => {
          if (captured !== generation) throw new Error("actor-activation.revoked");
        },
        complete,
      };
    };
    const pending = call(
      entry({ captureExtensionCompletion: capture }),
      entry({
        invoke: async () => {
          entered.resolve();
          return release.promise;
        },
      }),
    );
    const observed = expect(pending).rejects.toThrow("actor-activation.revoked");
    await entered.promise;
    generation += 1;
    release.resolve(extensionAnswerBytes(fixture.response));
    await observed;
    expect(complete).not.toHaveBeenCalled();
  });

  it("captures before the host queue and cancels queued evaluation after replacement", async () => {
    const { serializePerActor } = await import("../../🧱️elements/🔌️PluginRuntime/🟦️.tsx");
    const entered = Promise.withResolvers<void>();
    const release = Promise.withResolvers<void>();
    let generation = 1;
    const complete = vi.fn(async () => ({ output: null }));
    const capture = vi.fn((instanceId: number, req: bigint) => {
      const captured = generation;
      return {
        instanceId,
        req,
        assertActive: () => {
          if (captured !== generation) throw new Error("actor-activation.revoked");
        },
        complete,
      };
    });
    const requester = entry({ pluginId: "requester", captureExtensionCompletion: capture });
    const invoke = vi.fn(async () => extensionAnswerBytes(fixture.response));
    const held = serializePerActor(`requester:${fixture.instanceId}`, async () => {
      entered.resolve();
      await release.promise;
    });
    await entered.promise;
    const publish = vi.fn(async () => {});
    const pending = dispatchInvokeExtensionEffect(
      [requester, entry({ pluginId: fixture.extensionId, invoke })],
      { pluginId: "requester", instanceId: fixture.instanceId },
      { req: BigInt(fixture.requestId), extensionId: fixture.extensionId, capability: fixture.capability, requestJson: JSON.stringify(fixture.request) },
      publish,
    );
    const observed = expect(pending).rejects.toThrow("actor-activation.revoked");
    const capturedBeforeRelease = capture.mock.calls.length;
    generation += 1;
    release.resolve();
    await held;
    await observed;
    expect(capturedBeforeRelease).toBe(1);
    expect(capture).toHaveBeenCalledOnce();
    expect(invoke).not.toHaveBeenCalled();
    expect(complete).not.toHaveBeenCalled();
    expect(publish).not.toHaveBeenCalled();
  });
});
//#endregion 🔁️ExtensionInvocation

//#region 📄️PublicInvocationPaging
describe("public invocation paging", () => {
  it("mirrors the language-neutral envelope schema both Rust and the shell read", () => {
    expect(PUBLIC_INVOCATION_BODY_BYTES).toBe(publicInvocationSchema.properties.maxBodyBytes.const);
    expect(PUBLIC_INVOCATION_STRING_BYTES).toBe(publicInvocationSchema.properties.maxStringBytes.const);
    expect(PUBLIC_INVOCATION_DEPTH).toBe(publicInvocationSchema.properties.maxDepth.const);
  });

  /** 📐️ The Node oracle for the cost function: what `JSON.stringify` actually writes for this
   * character, minus the leading backslash the guest's counter skips. */
  const oracleCost = (character: string): number => {
    const encoded = JSON.stringify(character).slice(1, -1);
    return encoded.length - (encoded.match(/\\/gu)?.length ?? 0);
  };

  it("never undercharges a character against the JSON.stringify oracle", () => {
    for (const character of ['"', "\\", "\n", "\r", "\t", "\u0001", "a", "\u00e4", "\u26f0", "\u{1f300}"]) {
      expect(publicInvocationCharCost(character)).toBeGreaterThanOrEqual(oracleCost(character));
    }
    expect(publicInvocationCharCost('"')).toBe(1);
    expect(publicInvocationCharCost("\u0001")).toBe(5);
    expect(publicInvocationCharCost("\u{1f300}")).toBe(10);
  });

  it("fills every page to the bound, splits only on code points and loses nothing", () => {
    const payload = '{"id":"brep.extrude","name":"Extrudieren \u26f0\ufe0f"},'.repeat(4_096);
    const pages = publicInvocationStringPages(payload);
    expect(pages.length).toBeGreaterThan(1);
    for (const page of pages) {
      let cost = 0;
      for (const character of page) cost += publicInvocationCharCost(character);
      expect(cost).toBeLessThanOrEqual(PUBLIC_INVOCATION_STRING_BYTES);
      const encoded = JSON.stringify(page).slice(1, -1);
      expect(encoded.length - (encoded.match(/\\/gu)?.length ?? 0)).toBeLessThanOrEqual(PUBLIC_INVOCATION_STRING_BYTES);
    }
    expect(pages.join("")).toBe(payload);
  });

  it("yields one empty page for an empty payload, so a producer always sends an addressed page", () => {
    expect(publicInvocationStringPages("")).toEqual([""]);
  });
});
//#endregion 📄️PublicInvocationPaging
